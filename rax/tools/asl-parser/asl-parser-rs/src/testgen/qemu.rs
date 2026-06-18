//! QEMU differential testing harness.
//!
//! This module provides functionality to:
//! 1. Generate test binaries for AArch64 instructions
//! 2. Execute them under QEMU user mode
//! 3. Compare results with oracle expectations
//!
//! # Usage
//!
//! ```bash
//! # Generate QEMU test cases
//! cargo run -- test-gen arm_instrs.asl --output tests --format qemu
//!
//! # Run differential tests
//! ./run_qemu_tests.sh
//! ```

use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::testgen::types::*;

/// QEMU test case for a single instruction
#[derive(Debug, Clone)]
pub struct QemuTestCase {
    /// Test name
    pub name: String,
    /// Instruction encoding (4 bytes for A64)
    pub encoding: u32,
    /// Initial GP register values (reg -> value)
    pub initial_gp_regs: HashMap<u8, u64>,
    /// Initial SP value
    pub initial_sp: Option<u64>,
    /// Initial memory contents (address -> bytes)
    pub initial_memory: HashMap<u64, Vec<u8>>,
    /// Expected GP register values after execution
    pub expected_gp_regs: HashMap<u8, u64>,
    /// Expected NZCV flags
    pub expected_nzcv: Option<u8>,
    /// Expected memory contents
    pub expected_memory: HashMap<u64, Vec<u8>>,
}

/// Result of running a QEMU test
#[derive(Debug)]
pub struct QemuTestResult {
    /// Test name
    pub name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Actual GP register values
    pub actual_gp_regs: HashMap<u8, u64>,
    /// Actual NZCV flags
    pub actual_nzcv: u8,
    /// Mismatches found
    pub mismatches: Vec<String>,
    /// QEMU stdout/stderr
    pub output: String,
}

/// QEMU test harness
pub struct QemuHarness {
    /// Path to qemu-aarch64 binary
    qemu_path: String,
    /// Temporary directory for test binaries
    temp_dir: String,
}

impl QemuHarness {
    /// Create a new QEMU harness
    pub fn new() -> Self {
        Self {
            qemu_path: "qemu-aarch64".to_string(),
            temp_dir: "/tmp/asl-qemu-tests".to_string(),
        }
    }

    /// Create with custom QEMU path
    pub fn with_qemu_path(qemu_path: &str) -> Self {
        Self {
            qemu_path: qemu_path.to_string(),
            temp_dir: "/tmp/asl-qemu-tests".to_string(),
        }
    }

    /// Check if QEMU is available
    pub fn is_available(&self) -> bool {
        Command::new(&self.qemu_path)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    /// Generate a minimal ELF binary for testing a single instruction
    pub fn generate_test_binary(&self, test: &QemuTestCase) -> Vec<u8> {
        let mut code = Vec::new();

        // Prologue: Set up registers from test case
        for (reg, value) in &test.initial_gp_regs {
            if *reg < 31 {
                // MOVZ Xreg, #(value & 0xFFFF)
                // MOVK Xreg, #((value >> 16) & 0xFFFF), LSL #16
                // MOVK Xreg, #((value >> 32) & 0xFFFF), LSL #32
                // MOVK Xreg, #((value >> 48) & 0xFFFF), LSL #48
                code.extend_from_slice(&self.encode_mov_imm(*reg, *value));
            }
        }

        // Set up SP if needed
        if let Some(sp) = test.initial_sp {
            // MOV SP, X28 (use X28 as temp)
            code.extend_from_slice(&self.encode_mov_imm(28, sp));
            code.extend_from_slice(&0x9100039F_u32.to_le_bytes()); // MOV SP, X28
        }

        // The instruction under test
        code.extend_from_slice(&test.encoding.to_le_bytes());

        // Epilogue: Exit syscall (Linux aarch64)
        // MOV X8, #93 (exit syscall number)
        // MOV X0, #0 (exit code)
        // SVC #0
        code.extend_from_slice(&0xD2800BA8_u32.to_le_bytes()); // MOVZ X8, #93
        code.extend_from_slice(&0xD2800000_u32.to_le_bytes()); // MOVZ X0, #0
        code.extend_from_slice(&0xD4000001_u32.to_le_bytes()); // SVC #0

        // Build minimal ELF
        self.build_elf(&code)
    }

    /// Encode a 64-bit immediate load using MOVZ/MOVK sequence
    fn encode_mov_imm(&self, reg: u8, value: u64) -> Vec<u8> {
        let mut code = Vec::new();
        let reg = reg as u32;

        // MOVZ Xreg, #(value & 0xFFFF)
        let movz = 0xD2800000 | (reg) | (((value as u32) & 0xFFFF) << 5);
        code.extend_from_slice(&movz.to_le_bytes());

        if value > 0xFFFF {
            // MOVK Xreg, #((value >> 16) & 0xFFFF), LSL #16
            let movk16 = 0xF2A00000 | (reg) | ((((value >> 16) as u32) & 0xFFFF) << 5);
            code.extend_from_slice(&movk16.to_le_bytes());
        }

        if value > 0xFFFF_FFFF {
            // MOVK Xreg, #((value >> 32) & 0xFFFF), LSL #32
            let movk32 = 0xF2C00000 | (reg) | ((((value >> 32) as u32) & 0xFFFF) << 5);
            code.extend_from_slice(&movk32.to_le_bytes());
        }

        if value > 0xFFFF_FFFF_FFFF {
            // MOVK Xreg, #((value >> 48) & 0xFFFF), LSL #48
            let movk48 = 0xF2E00000 | (reg) | ((((value >> 48) as u32) & 0xFFFF) << 5);
            code.extend_from_slice(&movk48.to_le_bytes());
        }

        code
    }

    /// Build a minimal ELF binary from code
    fn build_elf(&self, code: &[u8]) -> Vec<u8> {
        let mut elf = Vec::new();

        // ELF header (64 bytes)
        let entry = 0x400000u64;
        let phoff = 64u64;
        let phnum = 1u16;

        // ELF magic
        elf.extend_from_slice(&[0x7f, b'E', b'L', b'F']);
        // Class (64-bit), Little endian, Version
        elf.extend_from_slice(&[2, 1, 1, 0]);
        // Padding
        elf.extend_from_slice(&[0; 8]);
        // Type (EXEC), Machine (AArch64)
        elf.extend_from_slice(&2u16.to_le_bytes());
        elf.extend_from_slice(&0xB7u16.to_le_bytes());
        // Version
        elf.extend_from_slice(&1u32.to_le_bytes());
        // Entry point
        elf.extend_from_slice(&entry.to_le_bytes());
        // Program header offset
        elf.extend_from_slice(&phoff.to_le_bytes());
        // Section header offset (0 = none)
        elf.extend_from_slice(&0u64.to_le_bytes());
        // Flags
        elf.extend_from_slice(&0u32.to_le_bytes());
        // ELF header size
        elf.extend_from_slice(&64u16.to_le_bytes());
        // Program header entry size
        elf.extend_from_slice(&56u16.to_le_bytes());
        // Number of program headers
        elf.extend_from_slice(&phnum.to_le_bytes());
        // Section header entry size
        elf.extend_from_slice(&0u16.to_le_bytes());
        // Number of section headers
        elf.extend_from_slice(&0u16.to_le_bytes());
        // Section name string table index
        elf.extend_from_slice(&0u16.to_le_bytes());

        // Program header (56 bytes)
        let file_offset: u64 = 64 + 56; // After ELF header and one program header
        let code_size = code.len() as u64;

        // Type (PT_LOAD)
        elf.extend_from_slice(&1u32.to_le_bytes());
        // Flags (PF_R | PF_X)
        elf.extend_from_slice(&5u32.to_le_bytes());
        // Offset in file
        elf.extend_from_slice(&file_offset.to_le_bytes());
        // Virtual address
        elf.extend_from_slice(&entry.to_le_bytes());
        // Physical address
        elf.extend_from_slice(&entry.to_le_bytes());
        // File size
        elf.extend_from_slice(&code_size.to_le_bytes());
        // Memory size
        elf.extend_from_slice(&code_size.to_le_bytes());
        // Alignment
        elf.extend_from_slice(&0x1000u64.to_le_bytes());

        // Code
        elf.extend_from_slice(code);

        elf
    }

    /// Run a single test under QEMU
    pub fn run_test(&self, test: &QemuTestCase) -> Result<QemuTestResult, String> {
        // Create temp directory if needed
        std::fs::create_dir_all(&self.temp_dir).map_err(|e| e.to_string())?;

        // Generate test binary
        let binary = self.generate_test_binary(test);
        let binary_path = format!("{}/test_{}.elf", self.temp_dir, test.name);

        let mut file = std::fs::File::create(&binary_path).map_err(|e| e.to_string())?;
        file.write_all(&binary).map_err(|e| e.to_string())?;

        // Make executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&binary_path)
                .map_err(|e| e.to_string())?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&binary_path, perms).map_err(|e| e.to_string())?;
        }

        // Run under QEMU with register dump
        let output = Command::new(&self.qemu_path)
            .arg("-d")
            .arg("cpu")
            .arg("-D")
            .arg("/dev/stderr")
            .arg(&binary_path)
            .output()
            .map_err(|e| e.to_string())?;

        // Parse output
        let stderr = String::from_utf8_lossy(&output.stderr);
        let actual_gp_regs = self.parse_register_dump(&stderr);
        let actual_nzcv = self.parse_nzcv(&stderr);

        // Compare with expected
        let mut mismatches = Vec::new();

        for (reg, expected) in &test.expected_gp_regs {
            if let Some(actual) = actual_gp_regs.get(reg) {
                if *actual != *expected {
                    mismatches.push(format!(
                        "X{}: expected 0x{:016X}, got 0x{:016X}",
                        reg, expected, actual
                    ));
                }
            } else {
                mismatches.push(format!("X{}: no value in output", reg));
            }
        }

        if let Some(expected_nzcv) = test.expected_nzcv {
            if actual_nzcv != expected_nzcv {
                mismatches.push(format!(
                    "NZCV: expected 0x{:X}, got 0x{:X}",
                    expected_nzcv, actual_nzcv
                ));
            }
        }

        Ok(QemuTestResult {
            name: test.name.clone(),
            passed: mismatches.is_empty(),
            actual_gp_regs,
            actual_nzcv,
            mismatches,
            output: stderr.to_string(),
        })
    }

    /// Parse register values from QEMU CPU dump
    fn parse_register_dump(&self, output: &str) -> HashMap<u8, u64> {
        let mut regs = HashMap::new();

        // QEMU format: X00=0000000000000000 X01=...
        for line in output.lines() {
            for part in line.split_whitespace() {
                if part.starts_with('X') || part.starts_with('x') {
                    if let Some((name, value)) = part.split_once('=') {
                        if let Ok(reg_num) = name[1..].parse::<u8>() {
                            if let Ok(val) = u64::from_str_radix(value, 16) {
                                regs.insert(reg_num, val);
                            }
                        }
                    }
                }
            }
        }

        regs
    }

    /// Parse NZCV from QEMU output
    fn parse_nzcv(&self, output: &str) -> u8 {
        // Look for PSTATE or NZCV in output
        for line in output.lines() {
            if line.contains("PSTATE") || line.contains("NZCV") {
                // Parse flags
                let n = if line.contains("N") { 8 } else { 0 };
                let z = if line.contains("Z") { 4 } else { 0 };
                let c = if line.contains("C") { 2 } else { 0 };
                let v = if line.contains("V") { 1 } else { 0 };
                return n | z | c | v;
            }
        }
        0
    }
}

impl Default for QemuHarness {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate QEMU test script from execution tests
pub fn generate_qemu_test_script(tests: &[ExecutionTest]) -> String {
    let mut script = String::new();

    script.push_str("#!/bin/bash\n");
    script.push_str("# QEMU differential test runner\n");
    script.push_str("# Generated by asl-parser test-gen\n\n");

    script.push_str("set -e\n\n");

    script.push_str("QEMU=${QEMU:-qemu-aarch64}\n");
    script.push_str("PASSED=0\n");
    script.push_str("FAILED=0\n\n");

    script.push_str("# Check QEMU is available\n");
    script.push_str("if ! command -v $QEMU &> /dev/null; then\n");
    script.push_str("    echo \"Error: $QEMU not found. Install qemu-user.\"\n");
    script.push_str("    exit 1\n");
    script.push_str("fi\n\n");

    script.push_str("echo \"Running QEMU differential tests...\"\n\n");

    for (idx, test) in tests.iter().enumerate() {
        script.push_str(&format!("# Test {}: {}\n", idx, test.description));
        script.push_str(&format!("echo -n \"Test {}: {}... \"\n", idx, test.id));

        // Generate assembly file content
        script.push_str(&format!("cat > /tmp/test_{}.s << 'EOF'\n", idx));
        script.push_str(".global _start\n");
        script.push_str("_start:\n");

        // Set up initial registers
        for (reg, value) in &test.initial_state.gp_regs {
            if *reg < 31 {
                script.push_str(&format!("    ldr x{}, =0x{:X}\n", reg, value));
            }
        }

        // The instruction under test (as raw bytes)
        script.push_str(&format!(
            "    .word 0x{:08X}  // instruction under test\n",
            test.encoding as u32
        ));

        // Exit
        script.push_str("    mov x8, #93\n");
        script.push_str("    mov x0, #0\n");
        script.push_str("    svc #0\n");
        script.push_str("EOF\n\n");

        // Assemble and link
        script.push_str(&format!(
            "aarch64-linux-gnu-as -o /tmp/test_{}.o /tmp/test_{}.s 2>/dev/null\n",
            idx, idx
        ));
        script.push_str(&format!(
            "aarch64-linux-gnu-ld -o /tmp/test_{} /tmp/test_{}.o 2>/dev/null\n",
            idx, idx
        ));

        // Run and check
        script.push_str(&format!(
            "OUTPUT=$($QEMU -d cpu -D /dev/stderr /tmp/test_{} 2>&1 || true)\n",
            idx
        ));

        // Check expected values
        for assertion in &test.assertions {
            match &assertion.check {
                AssertionCheck::GpReg(reg) => {
                    if let AssertionValue::U64(expected) = &assertion.expected {
                        script.push_str(&format!(
                            "if echo \"$OUTPUT\" | grep -q 'X{:02}={:016x}'; then\n",
                            reg,
                            expected.to_be() // QEMU might show big-endian
                        ));
                        script.push_str("    PASSED=$((PASSED + 1))\n");
                        script.push_str("else\n");
                        script.push_str("    FAILED=$((FAILED + 1))\n");
                        script.push_str(&format!("    echo \"FAIL: {}\"\n", assertion.message));
                        script.push_str("fi\n");
                    }
                }
                _ => {}
            }
        }

        script.push_str("echo \"done\"\n\n");
    }

    script.push_str("echo \"\"\n");
    script.push_str("echo \"Results: $PASSED passed, $FAILED failed\"\n");
    script.push_str("if [ $FAILED -gt 0 ]; then\n");
    script.push_str("    exit 1\n");
    script.push_str("fi\n");

    script
}

/// Convert ExecutionTest to QemuTestCase
pub fn execution_test_to_qemu(test: &ExecutionTest) -> QemuTestCase {
    QemuTestCase {
        name: test.id.clone(),
        encoding: test.encoding as u32,
        initial_gp_regs: test.initial_state.gp_regs.clone(),
        initial_sp: test.initial_state.sp,
        initial_memory: test.initial_state.memory.clone(),
        expected_gp_regs: test.expected_state.gp_regs.clone(),
        expected_nzcv: test.expected_state.nzcv,
        expected_memory: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mov_imm_encoding() {
        let harness = QemuHarness::new();
        let code = harness.encode_mov_imm(0, 0x1234);
        assert!(!code.is_empty());
    }

    #[test]
    fn test_build_elf() {
        let harness = QemuHarness::new();
        let code = vec![0xD2800000u32.to_le_bytes()].concat(); // MOVZ X0, #0
        let elf = harness.build_elf(&code);

        // Check ELF magic
        assert_eq!(&elf[0..4], &[0x7f, b'E', b'L', b'F']);
    }
}
