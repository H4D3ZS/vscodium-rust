//! Comprehensive integration tests for the Serial 16550 UART via x86 emulation.
//!
//! These tests verify that the Serial16550 correctly handles I/O port operations
//! when accessed through x86 IN/OUT instructions via the emulator.
//!
//! Serial I/O Ports (COM1 base 0x3F8):
//! - 0x3F8: RBR/THR (read=receive, write=transmit) or DLL when DLAB=1
//! - 0x3F9: IER or DLM when DLAB=1
//! - 0x3FA: IIR (read) / FCR (write)
//! - 0x3FB: LCR (Line Control Register)
//! - 0x3FC: MCR (Modem Control Register)
//! - 0x3FD: LSR (Line Status Register)
//! - 0x3FE: MSR (Modem Status Register)
//! - 0x3FF: SCR (Scratch Register)
//!
//! Tests validate against the UART 16550 specification and OpenCores Verilog reference.

use crate::common::*;
use rax::cpu::{Registers, VcpuExit};
use rax::devices::bus::IoDevice;
use rax::devices::serial::Serial16550;

// COM1 base port
const COM1_BASE: u16 = 0x3F8;
const DATA_REG: u16 = COM1_BASE; // 0x3F8
const IER_REG: u16 = COM1_BASE + 1; // 0x3F9
const IIR_REG: u16 = COM1_BASE + 2; // 0x3FA
const LCR_REG: u16 = COM1_BASE + 3; // 0x3FB
const MCR_REG: u16 = COM1_BASE + 4; // 0x3FC
const LSR_REG: u16 = COM1_BASE + 5; // 0x3FD
const MSR_REG: u16 = COM1_BASE + 6; // 0x3FE
const SCR_REG: u16 = COM1_BASE + 7; // 0x3FF
const FCR_REG: u16 = COM1_BASE + 2; // 0x3FA (same as IIR, write-only)

// LSR bit definitions (from datasheet)
const LSR_DATA_READY: u8 = 0x01; // bit 0: Data Ready
#[allow(dead_code)]
const LSR_OVERRUN: u8 = 0x02; // bit 1: Overrun Error
#[allow(dead_code)]
const LSR_PARITY: u8 = 0x04; // bit 2: Parity Error
#[allow(dead_code)]
const LSR_FRAMING: u8 = 0x08; // bit 3: Framing Error
#[allow(dead_code)]
const LSR_BREAK: u8 = 0x10; // bit 4: Break Interrupt
const LSR_THRE: u8 = 0x20; // bit 5: THR Empty
const LSR_TEMT: u8 = 0x40; // bit 6: Transmitter Empty
#[allow(dead_code)]
const LSR_FIFO_ERR: u8 = 0x80; // bit 7: FIFO data error

// LCR bit definitions
const LCR_DLAB: u8 = 0x80; // bit 7: Divisor Latch Access Bit

// IIR bit definitions
const IIR_NO_PENDING: u8 = 0x01; // bit 0: 1 = no interrupt pending
#[allow(dead_code)]
const IIR_RDA: u8 = 0x04; // Received Data Available (bits 3:1 = 010)
const IIR_THRE: u8 = 0x02; // THR Empty (bits 3:1 = 001)
#[allow(dead_code)]
const IIR_RLS: u8 = 0x06; // Receiver Line Status (bits 3:1 = 011)
#[allow(dead_code)]
const IIR_TIMEOUT: u8 = 0x0C; // Timeout (bits 3:1 = 110)
#[allow(dead_code)]
const IIR_MS: u8 = 0x00; // Modem Status (bits 3:1 = 000)

// IER bit definitions
#[allow(dead_code)]
const IER_RDA: u8 = 0x01; // bit 0: Received Data Available interrupt
#[allow(dead_code)]
const IER_THRE: u8 = 0x02; // bit 1: THR Empty interrupt
#[allow(dead_code)]
const IER_RLS: u8 = 0x04; // bit 2: Receiver Line Status interrupt
#[allow(dead_code)]
const IER_MS: u8 = 0x08; // bit 3: Modem Status interrupt

/// Test result tracking for serial operations
struct SerialTestResult {
    io_writes: Vec<(u16, Vec<u8>)>,
    io_reads: Vec<(u16, u8)>,
    final_regs: Registers,
}

/// Helper to run code with Serial device handling
fn run_with_serial(code: &[u8], regs: Registers) -> SerialTestResult {
    let (mut vcpu, _) = setup_vm(code, Some(regs));
    let mut serial = Serial16550::new(COM1_BASE);
    let mut io_writes: Vec<(u16, Vec<u8>)> = Vec::new();
    let mut io_reads: Vec<(u16, u8)> = Vec::new();

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                let mut data = vec![0u8; size as usize];
                // Route serial ports to the Serial device
                if port >= COM1_BASE && port <= SCR_REG {
                    for (i, byte) in data.iter_mut().enumerate() {
                        *byte = serial.read(port + i as u16);
                    }
                    io_reads.push((port, data[0]));
                }
                vcpu.complete_io_in(&data);
            }
            VcpuExit::IoOut { port, data } => {
                io_writes.push((port, data.clone()));
                // Route serial ports to the Serial device
                if port >= COM1_BASE && port <= SCR_REG {
                    for (i, byte) in data.iter().enumerate() {
                        serial.write(port + i as u16, *byte);
                    }
                }
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let final_regs = vcpu.get_regs().unwrap();
    SerialTestResult {
        io_writes,
        io_reads,
        final_regs,
    }
}

/// Helper to run code with Serial device and inject input
/// Note: Enables FIFO mode by default to allow multiple character buffering
fn run_with_serial_input(code: &[u8], regs: Registers, input: &[u8]) -> SerialTestResult {
    let (mut vcpu, _) = setup_vm(code, Some(regs));
    let mut serial = Serial16550::new(COM1_BASE);

    // Enable FIFO mode to allow buffering multiple characters
    IoDevice::write(&mut serial, FCR_REG, 0x01);

    // Inject input into the serial buffer
    serial.inject_input(input);

    let mut io_writes: Vec<(u16, Vec<u8>)> = Vec::new();
    let mut io_reads: Vec<(u16, u8)> = Vec::new();

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                let mut data = vec![0u8; size as usize];
                if port >= COM1_BASE && port <= SCR_REG {
                    for (i, byte) in data.iter_mut().enumerate() {
                        *byte = serial.read(port + i as u16);
                    }
                    io_reads.push((port, data[0]));
                }
                vcpu.complete_io_in(&data);
            }
            VcpuExit::IoOut { port, data } => {
                io_writes.push((port, data.clone()));
                if port >= COM1_BASE && port <= SCR_REG {
                    for (i, byte) in data.iter().enumerate() {
                        serial.write(port + i as u16, *byte);
                    }
                }
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let final_regs = vcpu.get_regs().unwrap();
    SerialTestResult {
        io_writes,
        io_reads,
        final_regs,
    }
}

// ============================================================================
// LSR (Line Status Register) Tests - Port 0x3FD
// ============================================================================

#[test]
fn test_serial_lsr_initial_state() {
    // Read LSR - should show THRE and TEMT set (transmitter empty)
    // LSR reset value per spec: 0x60 (bits 5 and 6 set)
    let code = [
        0xBA, 0xFD, 0x03, 0x00, 0x00, // MOV EDX, 0x3FD (LSR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // LSR should have THRE (0x20) and TEMT (0x40) set = 0x60
    let lsr = result.final_regs.rax as u8;
    assert_eq!(lsr & LSR_THRE, LSR_THRE, "THRE should be set");
    assert_eq!(lsr & LSR_TEMT, LSR_TEMT, "TEMT should be set");
    assert_eq!(
        lsr & LSR_DATA_READY,
        0,
        "Data Ready should be clear (no input)"
    );
}

#[test]
fn test_serial_lsr_data_ready_with_input() {
    // Inject input data and verify LSR shows Data Ready
    let code = [
        0xBA, 0xFD, 0x03, 0x00, 0x00, // MOV EDX, 0x3FD (LSR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial_input(&code, regs, b"A");

    let lsr = result.final_regs.rax as u8;
    assert_eq!(
        lsr & LSR_DATA_READY,
        LSR_DATA_READY,
        "Data Ready should be set"
    );
    assert_eq!(lsr & LSR_THRE, LSR_THRE, "THRE should still be set");
    assert_eq!(lsr & LSR_TEMT, LSR_TEMT, "TEMT should still be set");
}

#[test]
fn test_serial_lsr_write_is_ignored() {
    // Writes to LSR should be ignored (LSR is read-only per spec)
    let code = [
        0xB0, 0x00, // MOV AL, 0x00 (try to clear LSR)
        0xBA, 0xFD, 0x03, 0x00, 0x00, // MOV EDX, 0x3FD (LSR)
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX (read it back)
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // LSR should still show THRE and TEMT
    let lsr = result.final_regs.rax as u8;
    assert_eq!(lsr & (LSR_THRE | LSR_TEMT), LSR_THRE | LSR_TEMT);
}

// ============================================================================
// THR/RBR (Transmit/Receive) Tests - Port 0x3F8
// ============================================================================

#[test]
fn test_serial_write_thr_basic() {
    // Write a character to THR
    let code = [
        0xB0, 0x41, // MOV AL, 'A'
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (THR)
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.io_writes.len(), 1);
    assert_eq!(result.io_writes[0], (0x3F8, vec![0x41]));
}

#[test]
fn test_serial_write_multiple_chars() {
    // Write multiple characters to THR
    let code = [
        0xB0, 0x48, // MOV AL, 'H'
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (THR)
        0xEE, // OUT DX, AL
        0xB0, 0x69, // MOV AL, 'i'
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.io_writes.len(), 2);
    assert_eq!(result.io_writes[0], (0x3F8, vec![0x48]));
    assert_eq!(result.io_writes[1], (0x3F8, vec![0x69]));
}

#[test]
fn test_serial_read_rbr_with_input() {
    // Read a character from RBR when input is available
    let code = [
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (RBR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial_input(&code, regs, b"X");

    assert_eq!(result.final_regs.rax & 0xFF, 0x58); // 'X'
}

#[test]
fn test_serial_read_rbr_no_input() {
    // Read from RBR with no input should return 0
    let code = [
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (RBR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.final_regs.rax & 0xFF, 0);
}

#[test]
fn test_serial_read_multiple_chars() {
    // Read multiple characters from RBR
    // Note: run_with_serial_input enables FIFO mode automatically
    let code = [
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (RBR)
        0xEC, // IN AL, DX (first char)
        0x88, 0xC4, // MOV AH, AL
        0xEC, // IN AL, DX (second char)
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial_input(&code, regs, b"AB");

    // AH='A' (0x41), AL='B' (0x42)
    assert_eq!(result.final_regs.rax & 0xFF, 0x42); // AL = 'B'
    assert_eq!((result.final_regs.rax >> 8) & 0xFF, 0x41); // AH = 'A'
}

// ============================================================================
// Divisor Latch (DLAB) Tests - Ports 0x3F8/0x3F9 with LCR.DLAB=1
// ============================================================================

#[test]
fn test_serial_dlab_enable_and_set_divisor() {
    // Set DLAB, write divisor, clear DLAB
    // Divisor 0x000C = 12 for 9600 baud (with 1.8432 MHz clock)
    let code = [
        // Set DLAB
        0xB0, 0x80, // MOV AL, 0x80 (DLAB=1)
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        // Write DLL (low byte of divisor)
        0xB0, 0x0C, // MOV AL, 0x0C
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (DLL when DLAB=1)
        0xEE, // OUT DX, AL
        // Write DLM (high byte of divisor)
        0xB0, 0x00, // MOV AL, 0x00
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9 (DLM when DLAB=1)
        0xEE, // OUT DX, AL
        // Clear DLAB
        0xB0, 0x03, // MOV AL, 0x03 (8N1, DLAB=0)
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.io_writes.len(), 4);
    assert_eq!(result.io_writes[0], (0x3FB, vec![0x80])); // LCR with DLAB
    assert_eq!(result.io_writes[1], (0x3F8, vec![0x0C])); // DLL
    assert_eq!(result.io_writes[2], (0x3F9, vec![0x00])); // DLM
    assert_eq!(result.io_writes[3], (0x3FB, vec![0x03])); // LCR 8N1
}

#[test]
fn test_serial_dlab_read_divisor() {
    // Set DLAB, write divisor, read it back
    let code = [
        // Set DLAB and write divisor 0x1234
        0xB0, 0x80, // MOV AL, 0x80 (DLAB=1)
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        0xB0, 0x34, // MOV AL, 0x34 (DLL)
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8
        0xEE, // OUT DX, AL
        0xB0, 0x12, // MOV AL, 0x12 (DLM)
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9
        0xEE, // OUT DX, AL
        // Read back DLL
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8
        0xEC, // IN AL, DX
        0x88, 0xC4, // MOV AH, AL (save DLL)
        // Read back DLM
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9
        0xEC, // IN AL, DX
        0x86, 0xC4, // XCHG AL, AH (now AX = DLM:DLL)
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // AX should contain 0x1234
    assert_eq!(result.final_regs.rax & 0xFFFF, 0x1234);
}

// ============================================================================
// LCR (Line Control Register) Tests - Port 0x3FB
// ============================================================================

#[test]
fn test_serial_lcr_write_read() {
    // Write LCR and read it back
    let code = [
        0xB0, 0x1B, // MOV AL, 0x1B (7 bits, even parity, 2 stop)
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.final_regs.rax & 0xFF, 0x1B);
}

#[test]
fn test_serial_lcr_8n1_config() {
    // Set standard 8N1 configuration
    let code = [
        0xB0, 0x03, // MOV AL, 0x03 (8 bits, no parity, 1 stop)
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.final_regs.rax & 0xFF, 0x03);
}

// ============================================================================
// IER (Interrupt Enable Register) Tests - Port 0x3F9
// ============================================================================

#[test]
fn test_serial_ier_write_read() {
    // Write IER and read it back (DLAB must be 0)
    let code = [
        // First ensure DLAB is 0
        0xB0, 0x00, // MOV AL, 0x00
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        // Write IER
        0xB0, 0x03, // MOV AL, 0x03 (RDA + THRE interrupts)
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9 (IER)
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.final_regs.rax & 0xFF, 0x03);
}

#[test]
fn test_serial_ier_enable_all_standard() {
    // Enable all 4 standard interrupt sources
    let code = [
        0xB0, 0x00, // MOV AL, 0x00 (clear DLAB)
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        0xB0, 0x0F, // MOV AL, 0x0F (all 4 interrupts)
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9 (IER)
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // Note: bits 4-7 might be masked to 0 per spec
    assert_eq!(result.final_regs.rax & 0x0F, 0x0F);
}

// ============================================================================
// IIR (Interrupt Identification Register) Tests - Port 0x3FA
// ============================================================================

#[test]
fn test_serial_iir_no_interrupt() {
    // Read IIR with no interrupts pending
    let code = [
        0xBA, 0xFA, 0x03, 0x00, 0x00, // MOV EDX, 0x3FA (IIR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // Bit 0 should be 1 (no interrupt pending)
    assert_eq!(result.final_regs.rax & 0x01, 0x01);
}

#[test]
fn test_serial_iir_thre_interrupt() {
    // Enable THRE interrupt, then read IIR
    // THRE should be pending since transmitter is always empty
    // Note: MCR_OUT2 (0x08) must be set for interrupts to be reported
    let code = [
        // Ensure DLAB=0
        0xB0, 0x00, // MOV AL, 0x00
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        // Set MCR with OUT2 (global interrupt enable)
        0xB0, 0x08, // MOV AL, 0x08 (OUT2)
        0xBA, 0xFC, 0x03, 0x00, 0x00, // MOV EDX, 0x3FC (MCR)
        0xEE, // OUT DX, AL
        // Enable THRE interrupt
        0xB0, 0x02, // MOV AL, 0x02 (THRE interrupt)
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9 (IER)
        0xEE, // OUT DX, AL
        // Write to THR to trigger THRE interrupt
        0xB0, 0x41, // MOV AL, 'A'
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (THR)
        0xEE, // OUT DX, AL
        // Read IIR
        0xBA, 0xFA, 0x03, 0x00, 0x00, // MOV EDX, 0x3FA (IIR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // THRE interrupt: bit 0=0 (pending), bits 2:1=01, so IIR=0x02
    let iir = result.final_regs.rax & 0x0F;
    assert_eq!(iir, 0x02, "IIR should indicate THRE interrupt pending");
}

#[test]
fn test_serial_iir_read_clears_thre() {
    // Reading IIR should clear THRE interrupt
    let code = [
        // Set MCR with OUT2 (global interrupt enable)
        0xB0, 0x08, // MOV AL, 0x08 (OUT2)
        0xBA, 0xFC, 0x03, 0x00, 0x00, // MOV EDX, 0x3FC (MCR)
        0xEE, // OUT DX, AL
        // Enable THRE interrupt
        0xB0, 0x02, // MOV AL, 0x02
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9 (IER)
        0xEE, // OUT DX, AL
        // Write to THR to trigger THRE
        0xB0, 0x41, // MOV AL, 'A'
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (THR)
        0xEE, // OUT DX, AL
        // First IIR read
        0xBA, 0xFA, 0x03, 0x00, 0x00, // MOV EDX, 0x3FA (IIR)
        0xEC, // IN AL, DX
        0x88, 0xC4, // MOV AH, AL (save first read)
        // Second IIR read - should show no interrupt
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // First read: THRE pending (0x02)
    let first_iir = (result.final_regs.rax >> 8) & 0x0F;
    assert_eq!(first_iir, 0x02, "First IIR read should show THRE");

    // Second read: no interrupt pending (0x01)
    let second_iir = result.final_regs.rax & 0x0F;
    assert_eq!(second_iir, 0x01, "Second IIR read should show no interrupt");
}

// ============================================================================
// MCR (Modem Control Register) Tests - Port 0x3FC
// ============================================================================

#[test]
fn test_serial_mcr_write_read() {
    // Write MCR and read it back
    let code = [
        0xB0, 0x0B, // MOV AL, 0x0B (DTR, RTS, OUT2)
        0xBA, 0xFC, 0x03, 0x00, 0x00, // MOV EDX, 0x3FC (MCR)
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // Only bits 0-4 are writable per spec, bits 5-7 always 0
    assert_eq!(result.final_regs.rax & 0x1F, 0x0B);
}

#[test]
fn test_serial_mcr_loopback_bit() {
    // Set loopback mode (bit 4)
    let code = [
        0xB0, 0x10, // MOV AL, 0x10 (loopback)
        0xBA, 0xFC, 0x03, 0x00, 0x00, // MOV EDX, 0x3FC (MCR)
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.final_regs.rax & 0x10, 0x10);
}

// ============================================================================
// MSR (Modem Status Register) Tests - Port 0x3FE
// ============================================================================

#[test]
fn test_serial_msr_read() {
    // Read MSR
    let code = [
        0xBA, 0xFE, 0x03, 0x00, 0x00, // MOV EDX, 0x3FE (MSR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // Initial MSR value depends on modem input states
    // Just verify we can read it
    assert!(result.io_reads.len() >= 1);
}

// ============================================================================
// SCR (Scratch Register) Tests - Port 0x3FF
// ============================================================================

#[test]
fn test_serial_scratch_write_read() {
    // SCR should be a simple read/write register
    let code = [
        0xB0, 0xA5, // MOV AL, 0xA5
        0xBA, 0xFF, 0x03, 0x00, 0x00, // MOV EDX, 0x3FF (SCR)
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.final_regs.rax & 0xFF, 0xA5);
}

#[test]
fn test_serial_scratch_multiple_values() {
    // Write different values to scratch register
    let code = [
        0xBA, 0xFF, 0x03, 0x00, 0x00, // MOV EDX, 0x3FF (SCR)
        0xB0, 0x00, // MOV AL, 0x00
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0x88, 0xC4, // MOV AH, AL
        0xB0, 0xFF, // MOV AL, 0xFF
        0xEE, // OUT DX, AL
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // First value written was 0x00, second was 0xFF
    // AH should have 0x00, AL should have 0xFF
    assert_eq!((result.final_regs.rax >> 8) & 0xFF, 0x00);
    assert_eq!(result.final_regs.rax & 0xFF, 0xFF);
}

// ============================================================================
// FCR (FIFO Control Register) Tests - Port 0x3FA (write)
// ============================================================================

#[test]
fn test_serial_fcr_write() {
    // Write FCR (same address as IIR but write-only)
    // FCR 0xC7 = FIFOs enabled, clear both FIFOs, trigger level 14
    let code = [
        0xB0, 0xC7, // MOV AL, 0xC7
        0xBA, 0xFA, 0x03, 0x00, 0x00, // MOV EDX, 0x3FA (FCR)
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.io_writes.len(), 1);
    assert_eq!(result.io_writes[0], (0x3FA, vec![0xC7]));
}

// ============================================================================
// Typical OS Initialization Sequence Tests
// ============================================================================

#[test]
fn test_serial_typical_init_sequence() {
    // Standard serial port initialization as done by most OSes:
    // 1. Disable interrupts (IER = 0)
    // 2. Set DLAB to configure baud rate
    // 3. Set divisor for desired baud rate
    // 4. Clear DLAB, set line control (8N1)
    // 5. Enable FIFO
    // 6. Set MCR for DTR, RTS, OUT2
    // 7. Enable desired interrupts
    let code = [
        // 1. Disable interrupts
        0xB0, 0x00, // MOV AL, 0x00
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9 (IER)
        0xEE, // OUT DX, AL
        // 2. Set DLAB
        0xB0, 0x80, // MOV AL, 0x80
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        // 3. Set divisor for 115200 baud (divisor = 1)
        0xB0, 0x01, // MOV AL, 0x01 (DLL)
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8
        0xEE, // OUT DX, AL
        0xB0, 0x00, // MOV AL, 0x00 (DLM)
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9
        0xEE, // OUT DX, AL
        // 4. Set 8N1, clear DLAB
        0xB0, 0x03, // MOV AL, 0x03
        0xBA, 0xFB, 0x03, 0x00, 0x00, // MOV EDX, 0x3FB (LCR)
        0xEE, // OUT DX, AL
        // 5. Enable FIFO, clear, 14-byte trigger
        0xB0, 0xC7, // MOV AL, 0xC7
        0xBA, 0xFA, 0x03, 0x00, 0x00, // MOV EDX, 0x3FA (FCR)
        0xEE, // OUT DX, AL
        // 6. Set MCR (DTR, RTS, OUT2)
        0xB0, 0x0B, // MOV AL, 0x0B
        0xBA, 0xFC, 0x03, 0x00, 0x00, // MOV EDX, 0x3FC (MCR)
        0xEE, // OUT DX, AL
        // 7. Enable RDA and THRE interrupts
        0xB0, 0x03, // MOV AL, 0x03
        0xBA, 0xF9, 0x03, 0x00, 0x00, // MOV EDX, 0x3F9 (IER)
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // Verify all writes occurred in order
    assert!(result.io_writes.len() >= 8);
}

#[test]
fn test_serial_transmit_with_status_check() {
    // Real-world pattern: check LSR before transmitting
    let code = [
        // Check LSR for THRE
        0xBA, 0xFD, 0x03, 0x00, 0x00, // MOV EDX, 0x3FD (LSR)
        0xEC, // IN AL, DX
        // Write character (THRE should be set)
        0xB0, 0x48, // MOV AL, 'H'
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (THR)
        0xEE, // OUT DX, AL
        // Check LSR again
        0xBA, 0xFD, 0x03, 0x00, 0x00, // MOV EDX, 0x3FD (LSR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // Verify writes
    assert_eq!(result.io_writes.len(), 1);
    assert_eq!(result.io_writes[0], (0x3F8, vec![0x48]));

    // Both LSR reads should show THRE set (transmitter always empty)
    assert!(
        result
            .io_reads
            .iter()
            .all(|(port, val)| { *port != 0x3FD || (*val & LSR_THRE) != 0 })
    );
}

// ============================================================================
// Polling Loop Pattern Tests
// ============================================================================

#[test]
fn test_serial_receive_poll_pattern() {
    // Pattern: poll LSR until Data Ready, then read RBR
    // Note: This is a simplified version since we inject data beforehand
    let code = [
        // Read LSR to check for Data Ready
        0xBA, 0xFD, 0x03, 0x00, 0x00, // MOV EDX, 0x3FD (LSR)
        0xEC, // IN AL, DX
        // If bit 0 is set, read the data
        0xBA, 0xF8, 0x03, 0x00, 0x00, // MOV EDX, 0x3F8 (RBR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial_input(&code, regs, b"T");

    // Should have read the character 'T'
    assert_eq!(result.final_regs.rax & 0xFF, 0x54); // 'T'
}

// ============================================================================
// DX-based I/O (variable port) Tests
// ============================================================================

#[test]
fn test_serial_dx_based_io_sequence() {
    // Use DX-based I/O exclusively
    let code = [
        // Write to LCR using DX
        0x66, 0xBA, 0xFB, 0x03, // MOV DX, 0x3FB
        0xB0, 0x03, // MOV AL, 0x03
        0xEE, // OUT DX, AL
        // Write to THR using DX
        0x66, 0xBA, 0xF8, 0x03, // MOV DX, 0x3F8
        0xB0, 0x5A, // MOV AL, 'Z'
        0xEE, // OUT DX, AL
        // Read LSR using DX
        0x66, 0xBA, 0xFD, 0x03, // MOV DX, 0x3FD
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    assert_eq!(result.io_writes.len(), 2);
    assert_eq!(result.io_writes[0], (0x3FB, vec![0x03]));
    assert_eq!(result.io_writes[1], (0x3F8, vec![0x5A]));

    // LSR read should show THRE set
    assert_eq!(result.final_regs.rax & LSR_THRE as u64, LSR_THRE as u64);
}

// ============================================================================
// Edge Cases and Error Conditions
// ============================================================================

#[test]
fn test_serial_out_of_range_port() {
    // Access port outside serial range - should not affect serial
    let code = [
        0xB0, 0x42, // MOV AL, 0x42
        0xBA, 0x00, 0x04, 0x00, 0x00, // MOV EDX, 0x400 (outside range)
        0xEE, // OUT DX, AL
        // Read from valid serial port to verify it's unaffected
        0xBA, 0xFD, 0x03, 0x00, 0x00, // MOV EDX, 0x3FD (LSR)
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // LSR should be normal (0x60)
    assert_eq!(
        result.final_regs.rax & (LSR_THRE | LSR_TEMT) as u64,
        (LSR_THRE | LSR_TEMT) as u64
    );
}

#[test]
fn test_serial_immediate_port_form() {
    // Use immediate port form (E4/E6 with imm8)
    // Note: These only work for ports 0-255, so we can't test COM1 directly
    // This test verifies the instruction works for some port
    let code = [
        0xE4, 0x80, // IN AL, 0x80 (debug port - not serial)
        0xF4, // HLT
    ];
    let regs = Registers::default();
    let result = run_with_serial(&code, regs);

    // Just verify it runs without error
    assert!(result.io_reads.is_empty() || result.io_reads[0].0 != COM1_BASE);
}
