//! Tests for the OUT instruction.
//!
//! OUT - Output to Port
//!
//! Copies the value from the source operand to the I/O port specified with the
//! destination operand. The source operand can be register AL, AX, or EAX;
//! the destination operand can be a byte-immediate or the DX register.
//!
//! Flags affected: None
//!
//! Reference: docs/out.txt

use crate::common::*;
use rax::cpu::{Registers, VcpuExit};

// ============================================================================
// OUT imm8, AL (opcode E6 ib) - 8-bit output to immediate port
// ============================================================================

#[test]
fn test_out_imm8_al_port_0() {
    // OUT 0, AL
    // E6 00 = OUT 0, AL
    // F4 = HLT
    let code = [0xE6, 0x00, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0, "Port should be 0");
                assert_eq!(data.len(), 1, "Size should be 1 byte");
                assert_eq!(data.len(), 1);
                captured_value = Some(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0x42), "Should output AL value");
}

#[test]
fn test_out_imm8_al_port_80h() {
    // OUT 0x80, AL (common POST code port)
    let code = [0xE6, 0x80, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0x80);
                assert_eq!(data.len(), 1);
                captured_value = Some(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0xEF), "Should output low byte of AL");
}

#[test]
fn test_out_imm8_al_port_ff() {
    // OUT 0xFF, AL (maximum immediate port)
    let code = [0xE6, 0xFF, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0xFF);
                captured_value = Some(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0xAA));
}

#[test]
fn test_out_imm8_al_zero_value() {
    // Output zero
    let code = [0xE6, 0x20, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                assert_eq!(port, 0x20);
                captured_value = Some(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0x00));
}

#[test]
fn test_out_imm8_al_all_bits() {
    // Output 0xFF
    let code = [0xE6, 0x21, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                captured_value = Some(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0xFF));
}

#[test]
fn test_out_imm8_al_multiple_writes() {
    // Multiple OUT instructions
    // OUT 0x60, AL; OUT 0x61, AL
    let code = [0xE6, 0x60, 0xE6, 0x61, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x55;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut port_writes = vec![];

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                port_writes.push((port, data[0]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(port_writes, vec![(0x60, 0x55), (0x61, 0x55)]);
}

#[test]
fn test_out_imm8_al_preserves_flags() {
    // OUT should not modify flags
    let code = [0xE6, 0x10, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x99;
    regs.rflags = 0x246; // Set some flags
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {}
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rflags & 0xFFF, 0x246, "Flags should be preserved");
}

#[test]
fn test_out_imm8_al_preserves_rax_high_bytes() {
    // OUT should not modify high bytes of RAX
    let code = [0xE6, 0x30, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                assert_eq!(data[0], 0x78);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0xDEADBEEF_12345678, "RAX should not be modified");
}

// ============================================================================
// OUT imm8, AX (opcode E7 ib with 66 prefix) - 16-bit output
// ============================================================================

#[test]
fn test_out_imm8_ax_basic() {
    // OUT 0x10, AX
    // 66 E7 10 = OUT 0x10, AX
    let code = [0x66, 0xE7, 0x10, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0x10);
                assert_eq!(data.len(), 2, "Size should be 2 bytes");
                assert_eq!(data.len(), 2);
                captured_value = Some(u16::from_le_bytes([data[0], data[1]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0x1234));
}

#[test]
fn test_out_imm8_ax_port_0() {
    // OUT 0, AX
    let code = [0x66, 0xE7, 0x00, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xCDAB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0);
                assert_eq!(data.len(), 2);
                captured_value = Some(u16::from_le_bytes([data[0], data[1]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0xCDAB));
}

#[test]
fn test_out_imm8_ax_port_ff() {
    // OUT 0xFF, AX
    let code = [0x66, 0xE7, 0xFF, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                assert_eq!(port, 0xFF);
                captured_value = Some(u16::from_le_bytes([data[0], data[1]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0xBEEF));
}

#[test]
fn test_out_imm8_ax_zero_value() {
    // Output zero
    let code = [0x66, 0xE7, 0x3F, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                captured_value = Some(u16::from_le_bytes([data[0], data[1]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0x0000));
}

#[test]
fn test_out_imm8_ax_all_bits() {
    // Output 0xFFFF
    let code = [0x66, 0xE7, 0x7F, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                captured_value = Some(u16::from_le_bytes([data[0], data[1]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0xFFFF));
}

// ============================================================================
// OUT imm8, EAX (opcode E7 ib) - 32-bit output
// ============================================================================

#[test]
fn test_out_imm8_eax_basic() {
    // OUT 0x40, EAX
    // E7 40 = OUT 0x40, EAX
    let code = [0xE7, 0x40, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0x40);
                assert_eq!(data.len(), 4, "Size should be 4 bytes");
                assert_eq!(data.len(), 4);
                captured_value = Some(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0x12345678));
}

#[test]
fn test_out_imm8_eax_port_0() {
    // OUT 0, EAX
    let code = [0xE7, 0x00, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                assert_eq!(port, 0);
                captured_value = Some(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0xDEADBEEF));
}

#[test]
fn test_out_imm8_eax_port_ff() {
    // OUT 0xFF, EAX
    let code = [0xE7, 0xFF, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x44332211;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                assert_eq!(port, 0xFF);
                captured_value = Some(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0x44332211));
}

#[test]
fn test_out_imm8_eax_zero_value() {
    // Output zero
    let code = [0xE7, 0x50, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                captured_value = Some(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0x00000000));
}

#[test]
fn test_out_imm8_eax_all_bits() {
    // Output 0xFFFFFFFF
    let code = [0xE7, 0x51, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                captured_value = Some(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured_value, Some(0xFFFFFFFF));
}

// ============================================================================
// OUT DX, AL (opcode EE) - 8-bit output to variable port
// ============================================================================

#[test]
fn test_out_dx_al_basic() {
    // OUT DX, AL
    // EE = OUT DX, AL
    let code = [0xEE, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x3F8; // COM1 port
    regs.rax = 0x41; // 'A'
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0x3F8, "Port should be from DX");
                assert_eq!(data.len(), 1);
                captured = Some((port, data[0]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0x3F8, 0x41)));
}

#[test]
fn test_out_dx_al_port_0() {
    // OUT DX, AL with DX=0
    let code = [0xEE, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0;
    regs.rax = 0xBB;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                captured = Some((port, data[0]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0, 0xBB)));
}

#[test]
fn test_out_dx_al_port_ffff() {
    // OUT DX, AL with DX=0xFFFF (maximum port)
    let code = [0xEE, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFF;
    regs.rax = 0x88;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                captured = Some((port, data[0]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0xFFFF, 0x88)));
}

#[test]
fn test_out_dx_al_high_port() {
    // Test port > 255 (which can't be accessed with imm8 form)
    let code = [0xEE, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x1234;
    regs.rax = 0xCC;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                captured = Some((port, data[0]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0x1234, 0xCC)));
}

#[test]
fn test_out_dx_al_only_uses_dx_low_16bits() {
    // Only low 16 bits of RDX should be used
    let code = [0xEE, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF_FFFF0042; // High bits set, port = 0x42
    regs.rax = 0x77;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                captured = Some((port, data[0]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0x42, 0x77)), "Only low 16 bits of DX used");
}

// ============================================================================
// OUT DX, AX (opcode EF with 66 prefix) - 16-bit output to variable port
// ============================================================================

#[test]
fn test_out_dx_ax_basic() {
    // OUT DX, AX
    // 66 EF = OUT DX, AX
    let code = [0x66, 0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F8; // COM2 port
    regs.rax = 0xABCD;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0x2F8);
                assert_eq!(data.len(), 2);
                let value = u16::from_le_bytes([data[0], data[1]]);
                captured = Some((port, value));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0x2F8, 0xABCD)));
}

#[test]
fn test_out_dx_ax_port_0() {
    let code = [0x66, 0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0;
    regs.rax = 0x2211;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                let value = u16::from_le_bytes([data[0], data[1]]);
                captured = Some((port, value));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0, 0x2211)));
}

#[test]
fn test_out_dx_ax_port_ffff() {
    let code = [0x66, 0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFF;
    regs.rax = 0xBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                let value = u16::from_le_bytes([data[0], data[1]]);
                captured = Some((port, value));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0xFFFF, 0xBEEF)));
}

#[test]
fn test_out_dx_ax_high_port() {
    let code = [0x66, 0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x8888;
    regs.rax = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                let value = u16::from_le_bytes([data[0], data[1]]);
                captured = Some((port, value));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0x8888, 0x1234)));
}

// ============================================================================
// OUT DX, EAX (opcode EF) - 32-bit output to variable port
// ============================================================================

#[test]
fn test_out_dx_eax_basic() {
    // OUT DX, EAX
    // EF = OUT DX, EAX
    let code = [0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF8; // PCI config address
    regs.rax = 0x44332211;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0xCF8);
                assert_eq!(data.len(), 4);
                let value = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                captured = Some((port, value));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0xCF8, 0x44332211)));
}

#[test]
fn test_out_dx_eax_port_0() {
    let code = [0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0;
    regs.rax = 0xDDCCBBAA;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                let value = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                captured = Some((port, value));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0, 0xDDCCBBAA)));
}

#[test]
fn test_out_dx_eax_port_ffff() {
    let code = [0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFF;
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                let value = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                captured = Some((port, value));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0xFFFF, 0xFFFFFFFF)));
}

#[test]
fn test_out_dx_eax_high_port() {
    let code = [0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCFC; // PCI config data
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                let value = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                captured = Some((port, value));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some((0xCFC, 0x12345678)));
}

#[test]
fn test_out_dx_eax_preserves_rdx() {
    // OUT should not modify DX
    let code = [0xEF, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xDEADBEEF_00000100;
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, .. } => {
                assert_eq!(port, 0x100);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdx, 0xDEADBEEF_00000100, "DX should not be modified");
}

// ============================================================================
// Edge cases and combinations
// ============================================================================

#[test]
fn test_out_sequence_different_sizes() {
    // OUT 0x60, AL; OUT DX, AX; OUT 0x40, EAX
    let code = [
        0xE6, 0x60, // OUT 0x60, AL
        0x66, 0xEF, // OUT DX, AX
        0xE7, 0x40, // OUT 0x40, EAX
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x64;
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut io_ops = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                io_ops.push((port, data.len(), data.clone()));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_ops.len(), 3);
    assert_eq!(io_ops[0], (0x60, 1, vec![0x78]));
    assert_eq!(io_ops[1], (0x64, 2, vec![0x78, 0x56]));
    assert_eq!(io_ops[2], (0x40, 4, vec![0x78, 0x56, 0x34, 0x12]));
}

#[test]
fn test_out_loop_changing_value() {
    // Loop writing incrementing values
    // MOV AL, 0; OUT 0x80, AL; INC AL; OUT 0x80, AL; HLT
    let code = [
        0xB0, 0x00, // MOV AL, 0
        0xE6, 0x80, // OUT 0x80, AL
        0xFE, 0xC0, // INC AL
        0xE6, 0x80, // OUT 0x80, AL
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut values = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                assert_eq!(port, 0x80);
                values.push(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(values, vec![0, 1]);
}

#[test]
fn test_out_loop_changing_port() {
    // Loop writing to incrementing ports
    // MOV DX, 0x100; OUT DX, AL; INC DX; OUT DX, AL; HLT
    let code = [
        0x66, 0xBA, 0x00, 0x01, // MOV DX, 0x100
        0xEE, // OUT DX, AL
        0x66, 0xFF, 0xC2, // INC DX
        0xEE, // OUT DX, AL
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rax = 0x42;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut ports = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data, .. } => {
                ports.push(port);
                assert_eq!(data[0], 0x42);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(ports, vec![0x100, 0x101]);
}
