//! Tests for the IN instruction.
//!
//! IN - Input From Port
//!
//! Copies the value from the I/O port specified with the source operand to the
//! destination operand. The source operand can be a byte-immediate or the DX register;
//! the destination operand can be register AL, AX, or EAX.
//!
//! Flags affected: None
//!
//! Reference: docs/in.txt

use crate::common::*;
use rax::cpu::{Registers, VcpuExit};

// ============================================================================
// IN AL, imm8 (opcode E4 ib) - 8-bit input from immediate port
// ============================================================================

#[test]
fn test_in_al_imm8_port_0() {
    // IN AL, 0
    // E4 00 = IN AL, 0
    // F4 = HLT
    let code = [0xE4, 0x00, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    // Run until we get I/O exit
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0, "Port should be 0");
                assert_eq!(size, 1, "Size should be 1 byte");
                // Simulate reading value 0x42 from port
                vcpu.complete_io_in(&[0x42]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        regs.rax & 0xFF,
        0x42,
        "AL should contain value read from port"
    );
    assert_eq!(
        regs.rax & 0xFFFFFF00,
        0xDEADBE00,
        "High bytes should be preserved"
    );
}

#[test]
fn test_in_al_imm8_port_80h() {
    // IN AL, 0x80 (common POST code port)
    let code = [0xE4, 0x80, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x80, "Port should be 0x80");
                assert_eq!(size, 1, "Size should be 1 byte");
                vcpu.complete_io_in(&[0xAA]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0xAA);
}

#[test]
fn test_in_al_imm8_port_ff() {
    // IN AL, 0xFF (maximum immediate port)
    let code = [0xE4, 0xFF, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0x12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0xFF, "Port should be 0xFF");
                assert_eq!(size, 1, "Size should be 1 byte");
                vcpu.complete_io_in(&[0x55]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0x55);
}

#[test]
fn test_in_al_imm8_zero_value() {
    // Test reading zero from port
    let code = [0xE4, 0x20, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x20);
                assert_eq!(size, 1);
                vcpu.complete_io_in(&[0x00]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0x00, "AL should be zeroed");
}

#[test]
fn test_in_al_imm8_all_bits_set() {
    // Test reading 0xFF
    let code = [0xE4, 0x21, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x21);
                vcpu.complete_io_in(&[0xFF]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0xFF);
}

#[test]
fn test_in_al_imm8_multiple_reads() {
    // Multiple IN instructions
    // IN AL, 0x60; IN AL, 0x61
    let code = [0xE4, 0x60, 0xE4, 0x61, 0xF4];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut port_reads = vec![];

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                port_reads.push(port);
                match port {
                    0x60 => vcpu.complete_io_in(&[0x1A]),
                    0x61 => vcpu.complete_io_in(&[0x2B]),
                    _ => panic!("Unexpected port"),
                }
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(port_reads, vec![0x60, 0x61]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0x2B, "Should have last read value");
}

#[test]
fn test_in_al_imm8_preserves_flags() {
    // IN should not modify flags
    let code = [0xE4, 0x10, 0xF4];
    let mut regs = Registers::default();
    regs.rflags = 0x246; // Set some flags
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0x10);
                vcpu.complete_io_in(&[0x99]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rflags & 0xFFF, 0x246, "Flags should be preserved");
}

// ============================================================================
// IN AX, imm8 (opcode E5 ib with 66 prefix) - 16-bit input
// ============================================================================

#[test]
fn test_in_ax_imm8_basic() {
    // IN AX, 0x10
    // 66 E5 10 = IN AX, 0x10
    let code = [0x66, 0xE5, 0x10, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x10);
                assert_eq!(size, 2, "Size should be 2 bytes");
                vcpu.complete_io_in(&[0x34, 0x12]); // 0x1234 little-endian
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x1234, "AX should contain 0x1234");
    assert_eq!(
        regs.rax & 0xFFFF0000,
        0xDEAD0000,
        "High word should be preserved"
    );
}

#[test]
fn test_in_ax_imm8_port_0() {
    // IN AX, 0
    let code = [0x66, 0xE5, 0x00, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0);
                assert_eq!(size, 2);
                vcpu.complete_io_in(&[0xAB, 0xCD]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xCDAB);
}

#[test]
fn test_in_ax_imm8_port_ff() {
    // IN AX, 0xFF
    let code = [0x66, 0xE5, 0xFF, 0xF4];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0xFF);
                assert_eq!(size, 2);
                vcpu.complete_io_in(&[0xFF, 0xFF]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xFFFF);
}

#[test]
fn test_in_ax_imm8_zero_value() {
    // Reading zero
    let code = [0x66, 0xE5, 0x3F, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0x3F);
                vcpu.complete_io_in(&[0x00, 0x00]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x0000);
}

// ============================================================================
// IN EAX, imm8 (opcode E5 ib) - 32-bit input
// ============================================================================

#[test]
fn test_in_eax_imm8_basic() {
    // IN EAX, 0x40
    // E5 40 = IN EAX, 0x40
    let code = [0xE5, 0x40, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x40);
                assert_eq!(size, 4, "Size should be 4 bytes");
                vcpu.complete_io_in(&[0x78, 0x56, 0x34, 0x12]); // 0x12345678
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        regs.rax & 0xFFFFFFFF,
        0x12345678,
        "EAX should contain 0x12345678"
    );
    assert_eq!(
        regs.rax & 0xFFFFFFFF00000000,
        0,
        "High dword should be zeroed in 64-bit"
    );
}

#[test]
fn test_in_eax_imm8_port_0() {
    // IN EAX, 0
    let code = [0xE5, 0x00, 0xF4];
    let mut regs = Registers::default();
    regs.rax = 0xFFFFFFFF_FFFFFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0);
                assert_eq!(size, 4);
                vcpu.complete_io_in(&[0xEF, 0xBE, 0xAD, 0xDE]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0xDEADBEEF);
}

#[test]
fn test_in_eax_imm8_port_ff() {
    // IN EAX, 0xFF
    let code = [0xE5, 0xFF, 0xF4];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0xFF);
                assert_eq!(size, 4);
                vcpu.complete_io_in(&[0x11, 0x22, 0x33, 0x44]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0x44332211);
}

#[test]
fn test_in_eax_imm8_all_bits() {
    // Reading 0xFFFFFFFF
    let code = [0xE5, 0x7F, 0xF4];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                vcpu.complete_io_in(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFF);
}

// ============================================================================
// IN AL, DX (opcode EC) - 8-bit input from variable port
// ============================================================================

#[test]
fn test_in_al_dx_basic() {
    // IN AL, DX
    // EC = IN AL, DX
    let code = [0xEC, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x3F8; // COM1 port
    regs.rax = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x3F8, "Port should be from DX");
                assert_eq!(size, 1);
                vcpu.complete_io_in(&[0x41]); // 'A'
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0x41);
}

#[test]
fn test_in_al_dx_port_0() {
    // IN AL, DX with DX=0
    let code = [0xEC, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0);
                vcpu.complete_io_in(&[0xBB]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0xBB);
}

#[test]
fn test_in_al_dx_port_ffff() {
    // IN AL, DX with DX=0xFFFF (maximum port)
    let code = [0xEC, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0xFFFF);
                vcpu.complete_io_in(&[0x88]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0x88);
}

#[test]
fn test_in_al_dx_high_port() {
    // Test port > 255 (which can't be accessed with imm8 form)
    let code = [0xEC, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x1234;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0x1234);
                vcpu.complete_io_in(&[0xCC]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0xCC);
}

#[test]
fn test_in_al_dx_only_uses_dx_low_16bits() {
    // Only low 16 bits of RDX should be used
    let code = [0xEC, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFFFFFF_FFFF0042; // High bits set, port = 0x42
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0x42, "Only low 16 bits of DX used");
                vcpu.complete_io_in(&[0x77]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFF, 0x77);
}

// ============================================================================
// IN AX, DX (opcode ED with 66 prefix) - 16-bit input from variable port
// ============================================================================

#[test]
fn test_in_ax_dx_basic() {
    // IN AX, DX
    // 66 ED = IN AX, DX
    let code = [0x66, 0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F8; // COM2 port
    regs.rax = 0xDEADBEEF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x2F8);
                assert_eq!(size, 2);
                vcpu.complete_io_in(&[0xCD, 0xAB]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xABCD);
}

#[test]
fn test_in_ax_dx_port_0() {
    let code = [0x66, 0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                vcpu.complete_io_in(&[0x11, 0x22]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x2211);
}

#[test]
fn test_in_ax_dx_port_ffff() {
    let code = [0x66, 0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0xFFFF);
                vcpu.complete_io_in(&[0xEF, 0xBE]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0xBEEF);
}

#[test]
fn test_in_ax_dx_high_port() {
    let code = [0x66, 0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x8888;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0x8888);
                vcpu.complete_io_in(&[0x34, 0x12]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax & 0xFFFF, 0x1234);
}

// ============================================================================
// IN EAX, DX (opcode ED) - 32-bit input from variable port
// ============================================================================

#[test]
fn test_in_eax_dx_basic() {
    // IN EAX, DX
    // ED = IN EAX, DX
    let code = [0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF8; // PCI config address
    regs.rax = 0xDEADBEEF_12345678;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0xCF8);
                assert_eq!(size, 4);
                vcpu.complete_io_in(&[0x11, 0x22, 0x33, 0x44]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0x44332211);
}

#[test]
fn test_in_eax_dx_port_0() {
    let code = [0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                vcpu.complete_io_in(&[0xAA, 0xBB, 0xCC, 0xDD]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0xDDCCBBAA);
}

#[test]
fn test_in_eax_dx_port_ffff() {
    let code = [0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xFFFF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0xFFFF);
                vcpu.complete_io_in(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0xFFFFFFFF);
}

#[test]
fn test_in_eax_dx_high_port() {
    let code = [0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCFC; // PCI config data
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0xCFC);
                vcpu.complete_io_in(&[0x78, 0x56, 0x34, 0x12]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0x12345678);
}

#[test]
fn test_in_eax_dx_preserves_rdx() {
    // IN should not modify DX
    let code = [0xED, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xDEADBEEF_00000100;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0x100);
                vcpu.complete_io_in(&[0x00, 0x00, 0x00, 0x00]);
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
fn test_in_sequence_different_sizes() {
    // IN AL, 0x60; IN AX, DX; IN EAX, 0x40
    let code = [
        0xE4, 0x60, // IN AL, 0x60
        0x66, 0xED, // IN AX, DX
        0xE5, 0x40, // IN EAX, 0x40
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    regs.rdx = 0x64;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut io_count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                io_count += 1;
                match (port, size) {
                    (0x60, 1) => vcpu.complete_io_in(&[0xAA]),
                    (0x64, 2) => vcpu.complete_io_in(&[0x34, 0x12]),
                    (0x40, 4) => vcpu.complete_io_in(&[0x78, 0x56, 0x34, 0x12]),
                    _ => panic!("Unexpected I/O"),
                }
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_count, 3);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rax, 0x12345678, "Final value should be from last IN");
}

#[test]
fn test_in_loop_changing_port() {
    // Loop reading from incrementing ports
    // MOV DX, 0x100; IN AL, DX; INC DX; IN AL, DX; HLT
    let code = [
        0x66, 0xBA, 0x00, 0x01, // MOV DX, 0x100
        0xEC, // IN AL, DX
        0x66, 0xFF, 0xC2, // INC DX
        0xEC, // IN AL, DX
        0xF4, // HLT
    ];
    let mut regs = Registers::default();
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut ports = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                ports.push(port);
                vcpu.complete_io_in(&[port as u8]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(ports, vec![0x100, 0x101]);
}
