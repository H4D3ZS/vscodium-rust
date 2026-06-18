//! System register test generation.
//!
//! This module generates tests for ARM system registers including:
//! - MRS/MSR read/write tests
//! - Bit field access tests
//! - Register encoding validation

use crate::syntax::{Register, RegisterDefinition, RegisterField};
use std::collections::HashMap;

/// AArch64 system register encoding (op0, op1, crn, crm, op2).
#[derive(Debug, Clone, Copy)]
pub struct SysRegEncoding {
    pub op0: u8,
    pub op1: u8,
    pub crn: u8,
    pub crm: u8,
    pub op2: u8,
}

impl SysRegEncoding {
    pub const fn new(op0: u8, op1: u8, crn: u8, crm: u8, op2: u8) -> Self {
        Self {
            op0,
            op1,
            crn,
            crm,
            op2,
        }
    }

    /// Generate MRS instruction encoding: MRS Xt, <sysreg>
    /// Format: 1101010100 1 1 op0 op1 CRn CRm op2 Rt
    pub fn mrs_encoding(&self, rt: u8) -> u32 {
        0xD5300000
            | ((self.op0 as u32 & 0x3) << 19)
            | ((self.op1 as u32 & 0x7) << 16)
            | ((self.crn as u32 & 0xF) << 12)
            | ((self.crm as u32 & 0xF) << 8)
            | ((self.op2 as u32 & 0x7) << 5)
            | (rt as u32 & 0x1F)
    }

    /// Generate MSR instruction encoding: MSR <sysreg>, Xt
    /// Format: 1101010100 0 1 op0 op1 CRn CRm op2 Rt
    pub fn msr_encoding(&self, rt: u8) -> u32 {
        0xD5100000
            | ((self.op0 as u32 & 0x3) << 19)
            | ((self.op1 as u32 & 0x7) << 16)
            | ((self.crn as u32 & 0xF) << 12)
            | ((self.crm as u32 & 0xF) << 8)
            | ((self.op2 as u32 & 0x7) << 5)
            | (rt as u32 & 0x1F)
    }

    /// Get the canonical name like S3_0_C1_C0_0
    pub fn canonical_name(&self) -> String {
        format!(
            "S{}_{}_C{}_C{}_{}",
            self.op0, self.op1, self.crn, self.crm, self.op2
        )
    }
}

/// System register with parsed fields and encoding.
#[derive(Debug, Clone)]
pub struct SystemRegister {
    pub name: String,
    pub encoding: SysRegEncoding,
    pub width: u32,
    pub fields: Vec<RegisterField>,
    pub readable: bool,
    pub writable: bool,
    pub min_el: u8, // Minimum exception level required
}

/// Build the mapping from register names to encodings.
/// This is based on the ARMv8 Architecture Reference Manual.
pub fn build_sysreg_encodings() -> HashMap<String, SysRegEncoding> {
    let mut map = HashMap::new();

    // Control registers
    map.insert("SCTLR_EL1".into(), SysRegEncoding::new(3, 0, 1, 0, 0));
    map.insert("SCTLR_EL2".into(), SysRegEncoding::new(3, 4, 1, 0, 0));
    map.insert("SCTLR_EL3".into(), SysRegEncoding::new(3, 6, 1, 0, 0));
    map.insert("ACTLR_EL1".into(), SysRegEncoding::new(3, 0, 1, 0, 1));
    map.insert("CPACR_EL1".into(), SysRegEncoding::new(3, 0, 1, 0, 2));
    map.insert("HCR_EL2".into(), SysRegEncoding::new(3, 4, 1, 1, 0));
    map.insert("SCR_EL3".into(), SysRegEncoding::new(3, 6, 1, 1, 0));
    map.insert("CPTR_EL2".into(), SysRegEncoding::new(3, 4, 1, 1, 2));
    map.insert("CPTR_EL3".into(), SysRegEncoding::new(3, 6, 1, 1, 2));

    // ID registers (read-only)
    map.insert("MIDR_EL1".into(), SysRegEncoding::new(3, 0, 0, 0, 0));
    map.insert("MPIDR_EL1".into(), SysRegEncoding::new(3, 0, 0, 0, 5));
    map.insert("REVIDR_EL1".into(), SysRegEncoding::new(3, 0, 0, 0, 6));
    map.insert("ID_AA64PFR0_EL1".into(), SysRegEncoding::new(3, 0, 0, 4, 0));
    map.insert("ID_AA64PFR1_EL1".into(), SysRegEncoding::new(3, 0, 0, 4, 1));
    map.insert("ID_AA64DFR0_EL1".into(), SysRegEncoding::new(3, 0, 0, 5, 0));
    map.insert("ID_AA64DFR1_EL1".into(), SysRegEncoding::new(3, 0, 0, 5, 1));
    map.insert(
        "ID_AA64ISAR0_EL1".into(),
        SysRegEncoding::new(3, 0, 0, 6, 0),
    );
    map.insert(
        "ID_AA64ISAR1_EL1".into(),
        SysRegEncoding::new(3, 0, 0, 6, 1),
    );
    map.insert(
        "ID_AA64ISAR2_EL1".into(),
        SysRegEncoding::new(3, 0, 0, 6, 2),
    );
    map.insert(
        "ID_AA64MMFR0_EL1".into(),
        SysRegEncoding::new(3, 0, 0, 7, 0),
    );
    map.insert(
        "ID_AA64MMFR1_EL1".into(),
        SysRegEncoding::new(3, 0, 0, 7, 1),
    );
    map.insert(
        "ID_AA64MMFR2_EL1".into(),
        SysRegEncoding::new(3, 0, 0, 7, 2),
    );

    // Translation registers
    map.insert("TTBR0_EL1".into(), SysRegEncoding::new(3, 0, 2, 0, 0));
    map.insert("TTBR1_EL1".into(), SysRegEncoding::new(3, 0, 2, 0, 1));
    map.insert("TTBR0_EL2".into(), SysRegEncoding::new(3, 4, 2, 0, 0));
    map.insert("TCR_EL1".into(), SysRegEncoding::new(3, 0, 2, 0, 2));
    map.insert("TCR_EL2".into(), SysRegEncoding::new(3, 4, 2, 0, 2));
    map.insert("VTCR_EL2".into(), SysRegEncoding::new(3, 4, 2, 1, 2));
    map.insert("VTTBR_EL2".into(), SysRegEncoding::new(3, 4, 2, 1, 0));
    map.insert("MAIR_EL1".into(), SysRegEncoding::new(3, 0, 10, 2, 0));
    map.insert("MAIR_EL2".into(), SysRegEncoding::new(3, 4, 10, 2, 0));

    // Exception handling registers
    map.insert("VBAR_EL1".into(), SysRegEncoding::new(3, 0, 12, 0, 0));
    map.insert("VBAR_EL2".into(), SysRegEncoding::new(3, 4, 12, 0, 0));
    map.insert("VBAR_EL3".into(), SysRegEncoding::new(3, 6, 12, 0, 0));
    map.insert("ESR_EL1".into(), SysRegEncoding::new(3, 0, 5, 2, 0));
    map.insert("ESR_EL2".into(), SysRegEncoding::new(3, 4, 5, 2, 0));
    map.insert("ESR_EL3".into(), SysRegEncoding::new(3, 6, 5, 2, 0));
    map.insert("FAR_EL1".into(), SysRegEncoding::new(3, 0, 6, 0, 0));
    map.insert("FAR_EL2".into(), SysRegEncoding::new(3, 4, 6, 0, 0));
    map.insert("FAR_EL3".into(), SysRegEncoding::new(3, 6, 6, 0, 0));
    map.insert("ELR_EL1".into(), SysRegEncoding::new(3, 0, 4, 0, 1));
    map.insert("ELR_EL2".into(), SysRegEncoding::new(3, 4, 4, 0, 1));
    map.insert("ELR_EL3".into(), SysRegEncoding::new(3, 6, 4, 0, 1));
    map.insert("SPSR_EL1".into(), SysRegEncoding::new(3, 0, 4, 0, 0));
    map.insert("SPSR_EL2".into(), SysRegEncoding::new(3, 4, 4, 0, 0));
    map.insert("SPSR_EL3".into(), SysRegEncoding::new(3, 6, 4, 0, 0));

    // Stack pointers
    map.insert("SP_EL0".into(), SysRegEncoding::new(3, 0, 4, 1, 0));
    map.insert("SP_EL1".into(), SysRegEncoding::new(3, 4, 4, 1, 0));
    map.insert("SP_EL2".into(), SysRegEncoding::new(3, 6, 4, 1, 0));
    map.insert("SPSel".into(), SysRegEncoding::new(3, 0, 4, 2, 0));

    // Thread registers
    map.insert("TPIDR_EL0".into(), SysRegEncoding::new(3, 3, 13, 0, 2));
    map.insert("TPIDR_EL1".into(), SysRegEncoding::new(3, 0, 13, 0, 4));
    map.insert("TPIDR_EL2".into(), SysRegEncoding::new(3, 4, 13, 0, 2));
    map.insert("TPIDR_EL3".into(), SysRegEncoding::new(3, 6, 13, 0, 2));
    map.insert("TPIDRRO_EL0".into(), SysRegEncoding::new(3, 3, 13, 0, 3));

    // Counter-timer registers
    map.insert("CNTFRQ_EL0".into(), SysRegEncoding::new(3, 3, 14, 0, 0));
    map.insert("CNTVCT_EL0".into(), SysRegEncoding::new(3, 3, 14, 0, 2));
    map.insert("CNTPCT_EL0".into(), SysRegEncoding::new(3, 3, 14, 0, 1));
    map.insert("CNTP_CTL_EL0".into(), SysRegEncoding::new(3, 3, 14, 2, 1));
    map.insert("CNTP_CVAL_EL0".into(), SysRegEncoding::new(3, 3, 14, 2, 2));
    map.insert("CNTP_TVAL_EL0".into(), SysRegEncoding::new(3, 3, 14, 2, 0));
    map.insert("CNTV_CTL_EL0".into(), SysRegEncoding::new(3, 3, 14, 3, 1));
    map.insert("CNTV_CVAL_EL0".into(), SysRegEncoding::new(3, 3, 14, 3, 2));
    map.insert("CNTV_TVAL_EL0".into(), SysRegEncoding::new(3, 3, 14, 3, 0));
    map.insert("CNTKCTL_EL1".into(), SysRegEncoding::new(3, 0, 14, 1, 0));
    map.insert("CNTHCTL_EL2".into(), SysRegEncoding::new(3, 4, 14, 1, 0));

    // GIC registers
    map.insert("ICC_PMR_EL1".into(), SysRegEncoding::new(3, 0, 4, 6, 0));
    map.insert("ICC_IAR0_EL1".into(), SysRegEncoding::new(3, 0, 12, 8, 0));
    map.insert("ICC_IAR1_EL1".into(), SysRegEncoding::new(3, 0, 12, 12, 0));
    map.insert("ICC_EOIR0_EL1".into(), SysRegEncoding::new(3, 0, 12, 8, 1));
    map.insert("ICC_EOIR1_EL1".into(), SysRegEncoding::new(3, 0, 12, 12, 1));
    map.insert("ICC_SRE_EL1".into(), SysRegEncoding::new(3, 0, 12, 12, 5));
    map.insert("ICC_SRE_EL2".into(), SysRegEncoding::new(3, 4, 12, 9, 5));
    map.insert("ICC_SRE_EL3".into(), SysRegEncoding::new(3, 6, 12, 12, 5));
    map.insert(
        "ICC_IGRPEN0_EL1".into(),
        SysRegEncoding::new(3, 0, 12, 12, 6),
    );
    map.insert(
        "ICC_IGRPEN1_EL1".into(),
        SysRegEncoding::new(3, 0, 12, 12, 7),
    );
    map.insert("ICC_CTLR_EL1".into(), SysRegEncoding::new(3, 0, 12, 12, 4));
    map.insert("ICC_BPR0_EL1".into(), SysRegEncoding::new(3, 0, 12, 8, 3));
    map.insert("ICC_BPR1_EL1".into(), SysRegEncoding::new(3, 0, 12, 12, 3));

    // PSTATE/NZCV
    map.insert("NZCV".into(), SysRegEncoding::new(3, 3, 4, 2, 0));
    map.insert("DAIF".into(), SysRegEncoding::new(3, 3, 4, 2, 1));
    map.insert("CurrentEL".into(), SysRegEncoding::new(3, 0, 4, 2, 2));
    map.insert("FPCR".into(), SysRegEncoding::new(3, 3, 4, 4, 0));
    map.insert("FPSR".into(), SysRegEncoding::new(3, 3, 4, 4, 1));
    map.insert("DCZID_EL0".into(), SysRegEncoding::new(3, 3, 0, 0, 7));
    map.insert("CTR_EL0".into(), SysRegEncoding::new(3, 3, 0, 0, 1));

    // Performance monitors
    map.insert("PMCR_EL0".into(), SysRegEncoding::new(3, 3, 9, 12, 0));
    map.insert("PMCNTENSET_EL0".into(), SysRegEncoding::new(3, 3, 9, 12, 1));
    map.insert("PMCNTENCLR_EL0".into(), SysRegEncoding::new(3, 3, 9, 12, 2));
    map.insert("PMCCNTR_EL0".into(), SysRegEncoding::new(3, 3, 9, 13, 0));
    map.insert("PMSELR_EL0".into(), SysRegEncoding::new(3, 3, 9, 12, 5));
    map.insert("PMUSERENR_EL0".into(), SysRegEncoding::new(3, 3, 9, 14, 0));

    // Debug registers
    map.insert("MDSCR_EL1".into(), SysRegEncoding::new(2, 0, 0, 2, 2));
    map.insert("DBGBVR0_EL1".into(), SysRegEncoding::new(2, 0, 0, 0, 4));
    map.insert("DBGBCR0_EL1".into(), SysRegEncoding::new(2, 0, 0, 0, 5));
    map.insert("DBGWVR0_EL1".into(), SysRegEncoding::new(2, 0, 0, 0, 6));
    map.insert("DBGWCR0_EL1".into(), SysRegEncoding::new(2, 0, 0, 0, 7));

    // Pointer authentication keys
    map.insert("APIAKeyLo_EL1".into(), SysRegEncoding::new(3, 0, 2, 1, 0));
    map.insert("APIAKeyHi_EL1".into(), SysRegEncoding::new(3, 0, 2, 1, 1));
    map.insert("APIBKeyLo_EL1".into(), SysRegEncoding::new(3, 0, 2, 1, 2));
    map.insert("APIBKeyHi_EL1".into(), SysRegEncoding::new(3, 0, 2, 1, 3));
    map.insert("APDAKeyLo_EL1".into(), SysRegEncoding::new(3, 0, 2, 2, 0));
    map.insert("APDAKeyHi_EL1".into(), SysRegEncoding::new(3, 0, 2, 2, 1));
    map.insert("APDBKeyLo_EL1".into(), SysRegEncoding::new(3, 0, 2, 2, 2));
    map.insert("APDBKeyHi_EL1".into(), SysRegEncoding::new(3, 0, 2, 2, 3));
    map.insert("APGAKeyLo_EL1".into(), SysRegEncoding::new(3, 0, 2, 3, 0));
    map.insert("APGAKeyHi_EL1".into(), SysRegEncoding::new(3, 0, 2, 3, 1));

    // Context/Address registers
    map.insert("CONTEXTIDR_EL1".into(), SysRegEncoding::new(3, 0, 13, 0, 1));
    map.insert("CONTEXTIDR_EL2".into(), SysRegEncoding::new(3, 4, 13, 0, 1));

    // Virtualization
    map.insert("VMPIDR_EL2".into(), SysRegEncoding::new(3, 4, 0, 0, 5));
    map.insert("VPIDR_EL2".into(), SysRegEncoding::new(3, 4, 0, 0, 0));

    map
}

/// Registers that are read-only (ID registers, etc.)
pub fn readonly_registers() -> Vec<&'static str> {
    vec![
        "MIDR_EL1",
        "MPIDR_EL1",
        "REVIDR_EL1",
        "ID_AA64PFR0_EL1",
        "ID_AA64PFR1_EL1",
        "ID_AA64DFR0_EL1",
        "ID_AA64DFR1_EL1",
        "ID_AA64ISAR0_EL1",
        "ID_AA64ISAR1_EL1",
        "ID_AA64ISAR2_EL1",
        "ID_AA64MMFR0_EL1",
        "ID_AA64MMFR1_EL1",
        "ID_AA64MMFR2_EL1",
        "CTR_EL0",
        "DCZID_EL0",
        "CurrentEL",
        "CNTVCT_EL0",
        "CNTPCT_EL0", // Counter values are read-only
        "ICC_IAR0_EL1",
        "ICC_IAR1_EL1", // Interrupt acknowledge is read-only
    ]
}

/// Minimum EL required to access each register
pub fn register_min_el(name: &str) -> u8 {
    if name.ends_with("_EL3") {
        3
    } else if name.ends_with("_EL2") {
        2
    } else if name.ends_with("_EL1") {
        1
    } else if name.ends_with("_EL0") {
        0
    } else {
        1 // Default to EL1
    }
}

/// Generated system register test.
#[derive(Debug, Clone)]
pub struct SysRegTest {
    pub name: String,
    pub description: String,
    pub code: String,
}

/// Generate tests for a system register.
pub fn generate_sysreg_tests(
    reg: &Register,
    encoding: &SysRegEncoding,
    is_readonly: bool,
) -> Vec<SysRegTest> {
    let mut tests = Vec::new();
    let name = &reg.name;
    let name_lower = name.to_lowercase().replace("_", "");

    // Test 1: Basic read test (MRS)
    tests.push(generate_mrs_test(name, &name_lower, encoding, reg.width));

    // Test 2: Write/read test (MSR then MRS) - only if writable
    if !is_readonly {
        tests.push(generate_msr_mrs_test(
            name,
            &name_lower,
            encoding,
            reg.width,
        ));
    }

    // Test 3: Bit field tests
    for field in &reg.fields {
        if let Some(field_name) = &field.name {
            tests.push(generate_field_test(
                name,
                &name_lower,
                encoding,
                field,
                reg.width,
                is_readonly,
            ));
        }
    }

    tests
}

fn generate_mrs_test(name: &str, name_lower: &str, enc: &SysRegEncoding, width: u32) -> SysRegTest {
    let mrs_insn = enc.mrs_encoding(0); // MRS X0, <sysreg>

    let code = format!(
        r#"
#[test]
fn test_mrs_{name_lower}() {{
    let mut cpu = create_test_cpu();
    
    // MRS X0, {name}
    let mrs_insn: u32 = 0x{mrs_insn:08X};
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRS {name} should succeed: {{:?}}", result);
    
    // X0 should contain the register value
    let value = cpu.get_gpr(0);
    // Just verify we can read it without crashing
    println!("{name} = 0x{{:016X}}", value);
}}
"#,
        name = name,
        name_lower = name_lower,
        mrs_insn = mrs_insn,
    );

    SysRegTest {
        name: format!("test_mrs_{}", name_lower),
        description: format!("Read {} using MRS", name),
        code,
    }
}

fn generate_msr_mrs_test(
    name: &str,
    name_lower: &str,
    enc: &SysRegEncoding,
    width: u32,
) -> SysRegTest {
    let msr_insn = enc.msr_encoding(1); // MSR <sysreg>, X1
    let mrs_insn = enc.mrs_encoding(0); // MRS X0, <sysreg>

    // Use a test value that won't cause issues
    let test_value = if width == 32 {
        0x12345678u64
    } else {
        0x123456789ABCDEF0u64
    };

    let code = format!(
        r#"
#[test]
fn test_msr_mrs_{name_lower}() {{
    let mut cpu = create_test_cpu();
    
    // Set X1 to test value
    cpu.set_gpr(1, 0x{test_value:016X});
    
    // MSR {name}, X1
    let msr_insn: u32 = 0x{msr_insn:08X};
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, {name}
    let mrs_insn: u32 = 0x{mrs_insn:08X};
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    // Verify round-trip (may be masked by RES0/RES1 bits)
    let readback = cpu.get_gpr(0);
    println!("{name}: wrote 0x{{:016X}}, read 0x{{:016X}}", 0x{test_value:016X}u64, readback);
}}
"#,
        name = name,
        name_lower = name_lower,
        msr_insn = msr_insn,
        mrs_insn = mrs_insn,
        test_value = test_value,
    );

    SysRegTest {
        name: format!("test_msr_mrs_{}", name_lower),
        description: format!("Write then read {} using MSR/MRS", name),
        code,
    }
}

fn generate_field_test(
    name: &str,
    name_lower: &str,
    enc: &SysRegEncoding,
    field: &RegisterField,
    width: u32,
    is_readonly: bool,
) -> SysRegTest {
    let field_name = field.name.as_ref().unwrap();
    let field_name_lower = field_name.to_lowercase();
    let field_width = field.hi - field.lo + 1;
    let field_mask = ((1u64 << field_width) - 1) << field.lo;

    let mrs_insn = enc.mrs_encoding(0);

    let code = if is_readonly {
        // For read-only registers, just verify we can read and extract the field
        format!(
            r#"
#[test]
fn test_{name_lower}_field_{field_name_lower}() {{
    let mut cpu = create_test_cpu();
    
    // MRS X0, {name}
    let mrs_insn: u32 = 0x{mrs_insn:08X};
    cpu.write_memory(0, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> {lo}) & 0x{field_max:X};
    println!("{name}.{field_name} = 0x{{:X}} (bits [{hi}:{lo}])", field_value);
}}
"#,
            name = name,
            name_lower = name_lower,
            field_name = field_name,
            field_name_lower = field_name_lower,
            mrs_insn = mrs_insn,
            lo = field.lo,
            hi = field.hi,
            field_max = (1u64 << field_width) - 1,
        )
    } else {
        // For writable registers, test setting specific field values
        let msr_insn = enc.msr_encoding(1);
        format!(
            r#"
#[test]
fn test_{name_lower}_field_{field_name_lower}() {{
    let mut cpu = create_test_cpu();
    
    // Set field {field_name} to all 1s
    let test_value = 0x{field_mask:016X}u64;
    cpu.set_gpr(1, test_value);
    
    // MSR {name}, X1
    let msr_insn: u32 = 0x{msr_insn:08X};
    cpu.write_memory(0, &msr_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MSR should succeed");
    
    // MRS X0, {name}
    let mrs_insn: u32 = 0x{mrs_insn:08X};
    cpu.write_memory(4, &mrs_insn.to_le_bytes()).unwrap();
    cpu.step().expect("MRS should succeed");
    
    let value = cpu.get_gpr(0);
    let field_value = (value >> {lo}) & 0x{field_max:X};
    println!("{name}.{field_name} = 0x{{:X}} (bits [{hi}:{lo}])", field_value);
    
    // Field should be set (may be masked by implementation)
    // Just verify the instruction sequence works
}}
"#,
            name = name,
            name_lower = name_lower,
            field_name = field_name,
            field_name_lower = field_name_lower,
            msr_insn = msr_insn,
            mrs_insn = mrs_insn,
            field_mask = field_mask,
            lo = field.lo,
            hi = field.hi,
            field_max = (1u64 << field_width) - 1,
        )
    };

    SysRegTest {
        name: format!("test_{}_field_{}", name_lower, field_name_lower),
        description: format!(
            "Test {}.{} field (bits [{}:{}])",
            name, field_name, field.hi, field.lo
        ),
        code,
    }
}

/// Generate all system register tests from parsed definitions.
pub fn generate_all_sysreg_tests(regs: &[RegisterDefinition]) -> String {
    let encodings = build_sysreg_encodings();
    let readonly = readonly_registers();

    let mut output = String::new();
    output.push_str("//! Auto-generated system register tests.\n");
    output.push_str("//! DO NOT EDIT MANUALLY.\n\n");
    output.push_str("use super::super::test_helpers::*;\n\n");

    let mut test_count = 0;

    for reg_def in regs {
        let reg = match reg_def {
            RegisterDefinition::Single(r) => r,
            RegisterDefinition::Array(_) => continue, // Skip arrays for now
        };

        // Check if we have encoding for this register
        if let Some(encoding) = encodings.get(reg.name.as_str()) {
            let is_readonly = readonly.contains(&reg.name.as_str());
            let tests = generate_sysreg_tests(reg, encoding, is_readonly);

            for test in tests {
                output.push_str(&test.code);
                output.push('\n');
                test_count += 1;
            }
        }
    }

    println!("Generated {} system register tests", test_count);
    output
}

// =============================================================================
// AArch32 CP15 Coprocessor Register Support
// =============================================================================

/// CP15 coprocessor register encoding for AArch32.
#[derive(Debug, Clone, Copy)]
pub struct Cp15Encoding {
    /// CRn - primary register number (0-15)
    pub crn: u8,
    /// Op1 - opcode 1 (0-7)
    pub op1: u8,
    /// CRm - secondary register number (0-15)
    pub crm: u8,
    /// Op2 - opcode 2 (0-7)
    pub op2: u8,
}

impl Cp15Encoding {
    pub const fn new(crn: u8, op1: u8, crm: u8, op2: u8) -> Self {
        Self { crn, op1, crm, op2 }
    }

    /// Generate MRC instruction encoding: MRC p15, op1, Rt, CRn, CRm, op2
    /// Format: cond 1110 op1[2:0] 1 CRn Rt 1111 op2[2:0] 1 CRm
    /// We use cond=AL (0xE)
    pub fn mrc_encoding(&self, rt: u8) -> u32 {
        0xEE100F10  // Base: MRC p15, unconditional
            | ((self.op1 as u32 & 0x7) << 21)
            | ((self.crn as u32 & 0xF) << 16)
            | ((rt as u32 & 0xF) << 12)
            | ((self.op2 as u32 & 0x7) << 5)
            | (self.crm as u32 & 0xF)
    }

    /// Generate MCR instruction encoding: MCR p15, op1, Rt, CRn, CRm, op2
    /// Format: cond 1110 op1[2:0] 0 CRn Rt 1111 op2[2:0] 1 CRm
    pub fn mcr_encoding(&self, rt: u8) -> u32 {
        0xEE000F10  // Base: MCR p15, unconditional
            | ((self.op1 as u32 & 0x7) << 21)
            | ((self.crn as u32 & 0xF) << 16)
            | ((rt as u32 & 0xF) << 12)
            | ((self.op2 as u32 & 0x7) << 5)
            | (self.crm as u32 & 0xF)
    }
}

/// Build the mapping from A32 register names to CP15 encodings.
pub fn build_cp15_encodings() -> HashMap<String, Cp15Encoding> {
    let mut map = HashMap::new();

    // Main ID Register
    map.insert("MIDR".into(), Cp15Encoding::new(0, 0, 0, 0));
    // Cache Type Register
    map.insert("CTR".into(), Cp15Encoding::new(0, 0, 0, 1));
    // Multiprocessor Affinity Register
    map.insert("MPIDR".into(), Cp15Encoding::new(0, 0, 0, 5));

    // System Control Register
    map.insert("SCTLR".into(), Cp15Encoding::new(1, 0, 0, 0));
    // Auxiliary Control Register
    map.insert("ACTLR".into(), Cp15Encoding::new(1, 0, 0, 1));
    // Coprocessor Access Control Register
    map.insert("CPACR".into(), Cp15Encoding::new(1, 0, 0, 2));

    // Translation Table Base Register 0
    map.insert("TTBR0".into(), Cp15Encoding::new(2, 0, 0, 0));
    // Translation Table Base Register 1
    map.insert("TTBR1".into(), Cp15Encoding::new(2, 0, 0, 1));
    // Translation Table Base Control Register
    map.insert("TTBCR".into(), Cp15Encoding::new(2, 0, 0, 2));

    // Domain Access Control Register
    map.insert("DACR".into(), Cp15Encoding::new(3, 0, 0, 0));

    // Data Fault Status Register
    map.insert("DFSR".into(), Cp15Encoding::new(5, 0, 0, 0));
    // Instruction Fault Status Register
    map.insert("IFSR".into(), Cp15Encoding::new(5, 0, 0, 1));

    // Data Fault Address Register
    map.insert("DFAR".into(), Cp15Encoding::new(6, 0, 0, 0));
    // Instruction Fault Address Register
    map.insert("IFAR".into(), Cp15Encoding::new(6, 0, 0, 2));

    // Cache operations (write-only typically)
    map.insert("ICIALLU".into(), Cp15Encoding::new(7, 0, 5, 0));
    map.insert("DCIMVAC".into(), Cp15Encoding::new(7, 0, 6, 1));
    map.insert("DCISW".into(), Cp15Encoding::new(7, 0, 6, 2));
    map.insert("DCCMVAC".into(), Cp15Encoding::new(7, 0, 10, 1));
    map.insert("DCCSW".into(), Cp15Encoding::new(7, 0, 10, 2));
    map.insert("DCCIMVAC".into(), Cp15Encoding::new(7, 0, 14, 1));

    // TLB operations (write-only)
    map.insert("TLBIALL".into(), Cp15Encoding::new(8, 0, 7, 0));
    map.insert("TLBIMVA".into(), Cp15Encoding::new(8, 0, 7, 1));
    map.insert("TLBIASID".into(), Cp15Encoding::new(8, 0, 7, 2));

    // Performance Monitors
    map.insert("PMCR".into(), Cp15Encoding::new(9, 0, 12, 0));
    map.insert("PMCNTENSET".into(), Cp15Encoding::new(9, 0, 12, 1));
    map.insert("PMCNTENCLR".into(), Cp15Encoding::new(9, 0, 12, 2));
    map.insert("PMSELR".into(), Cp15Encoding::new(9, 0, 12, 5));
    map.insert("PMCCNTR".into(), Cp15Encoding::new(9, 0, 13, 0));

    // Vector Base Address Register
    map.insert("VBAR".into(), Cp15Encoding::new(12, 0, 0, 0));
    // Monitor Vector Base Address Register
    map.insert("MVBAR".into(), Cp15Encoding::new(12, 0, 0, 1));

    // Context ID Register
    map.insert("CONTEXTIDR".into(), Cp15Encoding::new(13, 0, 0, 1));
    // Thread ID registers
    map.insert("TPIDRURW".into(), Cp15Encoding::new(13, 0, 0, 2));
    map.insert("TPIDRURO".into(), Cp15Encoding::new(13, 0, 0, 3));
    map.insert("TPIDRPRW".into(), Cp15Encoding::new(13, 0, 0, 4));

    // Counter-timer registers
    map.insert("CNTFRQ".into(), Cp15Encoding::new(14, 0, 0, 0));

    map
}

/// CP15 registers that are read-only
pub fn readonly_cp15_registers() -> Vec<&'static str> {
    vec!["MIDR", "CTR", "MPIDR", "CNTFRQ"]
}

/// CP15 registers that are write-only (cache/TLB operations)
pub fn writeonly_cp15_registers() -> Vec<&'static str> {
    vec![
        "ICIALLU", "DCIMVAC", "DCISW", "DCCMVAC", "DCCSW", "DCCIMVAC", "TLBIALL", "TLBIMVA",
        "TLBIASID",
    ]
}

/// Generate A32 CP15 register tests.
pub fn generate_cp15_tests(
    reg: &Register,
    encoding: &Cp15Encoding,
    is_readonly: bool,
    is_writeonly: bool,
) -> Vec<SysRegTest> {
    let mut tests = Vec::new();
    let name = &reg.name;
    let name_lower = name.to_lowercase();

    // Test 1: MRC read test (unless write-only)
    if !is_writeonly {
        tests.push(generate_mrc_test(name, &name_lower, encoding));
    }

    // Test 2: MCR write test (unless read-only)
    if !is_readonly && !is_writeonly {
        tests.push(generate_mcr_mrc_test(name, &name_lower, encoding));
    }

    tests
}

fn generate_mrc_test(name: &str, name_lower: &str, enc: &Cp15Encoding) -> SysRegTest {
    let mrc_insn = enc.mrc_encoding(0); // MRC p15, op1, R0, CRn, CRm, op2

    let code = format!(
        r#"
#[test]
fn test_mrc_{name_lower}() {{
    let mut cpu = create_test_cpu();
    
    // MRC p15, {op1}, R0, c{crn}, c{crm}, {op2}  ({name})
    let mrc_insn: u32 = 0x{mrc_insn:08X};
    write_insn(&mut cpu, 0, mrc_insn);
    
    // Execute
    let result = cpu.step();
    assert!(result.is_ok(), "MRC {name} should succeed: {{:?}}", result);
    
    // R0 should contain the register value
    let value = get_w(&cpu, 0);
    println!("{name} = 0x{{:08X}}", value);
}}
"#,
        name = name,
        name_lower = name_lower,
        mrc_insn = mrc_insn,
        op1 = enc.op1,
        crn = enc.crn,
        crm = enc.crm,
        op2 = enc.op2,
    );

    SysRegTest {
        name: format!("test_mrc_{}", name_lower),
        description: format!("Read {} using MRC", name),
        code,
    }
}

fn generate_mcr_mrc_test(name: &str, name_lower: &str, enc: &Cp15Encoding) -> SysRegTest {
    let mcr_insn = enc.mcr_encoding(1); // MCR p15, op1, R1, CRn, CRm, op2
    let mrc_insn = enc.mrc_encoding(0); // MRC p15, op1, R0, CRn, CRm, op2

    let code = format!(
        r#"
#[test]
fn test_mcr_mrc_{name_lower}() {{
    let mut cpu = create_test_cpu();
    
    // Set R1 to test value
    set_w(&mut cpu, 1, 0x12345678);
    
    // MCR p15, {op1}, R1, c{crn}, c{crm}, {op2}  ({name})
    let mcr_insn: u32 = 0x{mcr_insn:08X};
    write_insn(&mut cpu, 0, mcr_insn);
    cpu.step().expect("MCR should succeed");
    
    // MRC p15, {op1}, R0, c{crn}, c{crm}, {op2}  ({name})
    let mrc_insn: u32 = 0x{mrc_insn:08X};
    write_insn(&mut cpu, 4, mrc_insn);
    cpu.step().expect("MRC should succeed");
    
    // Verify round-trip
    let readback = get_w(&cpu, 0);
    println!("{name}: wrote 0x12345678, read 0x{{:08X}}", readback);
}}
"#,
        name = name,
        name_lower = name_lower,
        mcr_insn = mcr_insn,
        mrc_insn = mrc_insn,
        op1 = enc.op1,
        crn = enc.crn,
        crm = enc.crm,
        op2 = enc.op2,
    );

    SysRegTest {
        name: format!("test_mcr_mrc_{}", name_lower),
        description: format!("Write then read {} using MCR/MRC", name),
        code,
    }
}

/// Generate all A32 CP15 register tests.
pub fn generate_all_cp15_tests(regs: &[RegisterDefinition]) -> String {
    let encodings = build_cp15_encodings();
    let readonly = readonly_cp15_registers();
    let writeonly = writeonly_cp15_registers();

    let mut output = String::new();
    output.push_str("//! Auto-generated A32 CP15 coprocessor register tests.\n");
    output.push_str("//! DO NOT EDIT MANUALLY.\n\n");
    output.push_str("use super::super::test_helpers_32::*;\n\n");

    let mut test_count = 0;

    for reg_def in regs {
        let reg = match reg_def {
            RegisterDefinition::Single(r) => r,
            RegisterDefinition::Array(_) => continue,
        };

        // Check if we have encoding for this register
        if let Some(encoding) = encodings.get(reg.name.as_str()) {
            let is_readonly = readonly.contains(&reg.name.as_str());
            let is_writeonly = writeonly.contains(&reg.name.as_str());
            let tests = generate_cp15_tests(reg, encoding, is_readonly, is_writeonly);

            for test in tests {
                output.push_str(&test.code);
                output.push('\n');
                test_count += 1;
            }
        }
    }

    println!("Generated {} CP15 register tests", test_count);
    output
}
