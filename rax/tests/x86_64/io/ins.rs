//! Tests for the INS/INSB/INSW/INSD instructions.
//!
//! INS/INSB/INSW/INSD - Input from Port to String
//!
//! Copies data from the I/O port specified in DX to the memory location
//! specified by ES:(R/E)DI. After the transfer, (R/E)DI is incremented or
//! decremented based on the DF flag.
//!
//! Flags affected: None
//!
//! Reference: docs/ins:insb:insw:insd.txt

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::{Registers, VcpuExit};

// ============================================================================
// INSB (opcode 6C) - Input byte string
// ============================================================================

#[test]
fn test_insb_basic() {
    // INSB - Input byte from port DX to [RDI]
    // 6C = INSB
    // F4 = HLT
    let code = [0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x60; // Keyboard data port
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF; // DF=0, increment
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x60);
                assert_eq!(size, 1);
                vcpu.complete_io_in(&[0x42]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(read_mem_u8(&mem), 0x42, "Byte should be written to [RDI]");
    assert_eq!(regs.rdi, DATA_ADDR + 1, "RDI should increment by 1");
}

#[test]
fn test_insb_df_clear_increment() {
    // INSB with DF=0 should increment RDI
    let code = [0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x70;
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF; // DF=0
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                vcpu.complete_io_in(&[0xAA]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR + 1, "DF=0: RDI should increment");
}

#[test]
fn test_insb_df_set_decrement() {
    // INSB with DF=1 should decrement RDI
    let code = [0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x71;
    regs.rdi = DATA_ADDR;
    regs.rflags |= flags::bits::DF; // DF=1
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                vcpu.complete_io_in(&[0xBB]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR - 1, "DF=1: RDI should decrement");
}

#[test]
fn test_insb_preserves_flags() {
    // INSB should not modify flags (except DF is used)
    let code = [0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x72;
    regs.rdi = DATA_ADDR;
    regs.rflags = 0x246; // Set some flags, DF=0
    let initial_flags = regs.rflags;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                vcpu.complete_io_in(&[0xCC]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        regs.rflags & 0xFFF,
        initial_flags & 0xFFF,
        "Flags should be preserved"
    );
}

#[test]
fn test_insb_multiple_sequential() {
    // Multiple INSB instructions
    // INSB; INSB; INSB
    let code = [0x6C, 0x6C, 0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x80;
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let mut count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                count += 1;
                vcpu.complete_io_in(&[count]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 3, "Should have 3 I/O operations");
    assert_eq!(read_mem_at_u8(&mem, DATA_ADDR), 1);
    assert_eq!(read_mem_at_u8(&mem, DATA_ADDR + 1), 2);
    assert_eq!(read_mem_at_u8(&mem, DATA_ADDR + 2), 3);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR + 3);
}

// ============================================================================
// INSW (opcode 6D with 66 prefix) - Input word string
// ============================================================================

#[test]
fn test_insw_basic() {
    // INSW - Input word from port DX to [RDI]
    // 66 6D = INSW
    let code = [0x66, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x3F8;
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x3F8);
                assert_eq!(size, 2);
                vcpu.complete_io_in(&[0x34, 0x12]); // 0x1234 little-endian
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        read_mem_u16(&mem),
        0x1234,
        "Word should be written to [RDI]"
    );
    assert_eq!(regs.rdi, DATA_ADDR + 2, "RDI should increment by 2");
}

#[test]
fn test_insw_df_clear_increment() {
    // INSW with DF=0 should increment RDI by 2
    let code = [0x66, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F8;
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                vcpu.complete_io_in(&[0xAA, 0xBB]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR + 2, "DF=0: RDI should increment by 2");
}

#[test]
fn test_insw_df_set_decrement() {
    // INSW with DF=1 should decrement RDI by 2
    let code = [0x66, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F9;
    regs.rdi = DATA_ADDR;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                vcpu.complete_io_in(&[0xCC, 0xDD]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR - 2, "DF=1: RDI should decrement by 2");
}

#[test]
fn test_insw_multiple_sequential() {
    // Multiple INSW instructions
    let code = [0x66, 0x6D, 0x66, 0x6D, 0x66, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x90;
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let mut count = 0u16;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                count += 1;
                let val = count * 0x1111;
                vcpu.complete_io_in(&val.to_le_bytes());
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 3);
    assert_eq!(read_mem_at_u16(&mem, DATA_ADDR), 0x1111);
    assert_eq!(read_mem_at_u16(&mem, DATA_ADDR + 2), 0x2222);
    assert_eq!(read_mem_at_u16(&mem, DATA_ADDR + 4), 0x3333);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR + 6);
}

// ============================================================================
// INSD (opcode 6D) - Input dword string
// ============================================================================

#[test]
fn test_insd_basic() {
    // INSD - Input dword from port DX to [RDI]
    // 6D = INSD
    let code = [0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCFC; // PCI config data
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0xCFC);
                assert_eq!(size, 4);
                vcpu.complete_io_in(&[0x78, 0x56, 0x34, 0x12]); // 0x12345678
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        read_mem_u32(&mem),
        0x12345678,
        "Dword should be written to [RDI]"
    );
    assert_eq!(regs.rdi, DATA_ADDR + 4, "RDI should increment by 4");
}

#[test]
fn test_insd_df_clear_increment() {
    // INSD with DF=0 should increment RDI by 4
    let code = [0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF8;
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                vcpu.complete_io_in(&[0xAA, 0xBB, 0xCC, 0xDD]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR + 4, "DF=0: RDI should increment by 4");
}

#[test]
fn test_insd_df_set_decrement() {
    // INSD with DF=1 should decrement RDI by 4
    let code = [0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF9;
    regs.rdi = DATA_ADDR;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                vcpu.complete_io_in(&[0x11, 0x22, 0x33, 0x44]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR - 4, "DF=1: RDI should decrement by 4");
}

#[test]
fn test_insd_multiple_sequential() {
    // Multiple INSD instructions
    let code = [0x6D, 0x6D, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xA0;
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let mut count = 0u32;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                count += 1;
                let val = count * 0x11111111;
                vcpu.complete_io_in(&val.to_le_bytes());
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 3);
    assert_eq!(read_mem_at_u32(&mem, DATA_ADDR), 0x11111111);
    assert_eq!(read_mem_at_u32(&mem, DATA_ADDR + 4), 0x22222222);
    assert_eq!(read_mem_at_u32(&mem, DATA_ADDR + 8), 0x33333333);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR + 12);
}

// ============================================================================
// REP INSB - Repeated byte input
// ============================================================================

#[test]
fn test_rep_insb_basic() {
    // REP INSB - Input RCX bytes
    // F3 6C = REP INSB
    let code = [0xF3, 0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x60;
    regs.rdi = DATA_ADDR;
    regs.rcx = 5; // Read 5 bytes
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let mut count = 0u8;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, size } => {
                assert_eq!(port, 0x60);
                assert_eq!(size, 1);
                count += 1;
                vcpu.complete_io_in(&[count]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 5, "Should have 5 I/O operations");
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0, "RCX should be 0 after REP");
    assert_eq!(regs.rdi, DATA_ADDR + 5, "RDI should increment by 5");

    // Verify memory
    for i in 0..5 {
        assert_eq!(read_mem_at_u8(&mem, DATA_ADDR + i), (i + 1) as u8);
    }
}

#[test]
fn test_rep_insb_zero_count() {
    // REP INSB with RCX=0 should not execute
    let code = [0xF3, 0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x60;
    regs.rdi = DATA_ADDR;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut io_count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                io_count += 1;
                vcpu.complete_io_in(&[0xFF]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_count, 0, "No I/O should occur with RCX=0");
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR, "RDI should not change");
}

#[test]
fn test_rep_insb_df_set() {
    // REP INSB with DF=1 should decrement RDI
    let code = [0xF3, 0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x61;
    regs.rdi = DATA_ADDR + 100; // Start high
    regs.rcx = 3;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                count += 1;
                vcpu.complete_io_in(&[count]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, DATA_ADDR + 100 - 3, "RDI should decrement by 3");
}

// ============================================================================
// REP INSW - Repeated word input
// ============================================================================

#[test]
fn test_rep_insw_basic() {
    // REP INSW - Input RCX words
    // F3 66 6D = REP INSW
    let code = [0xF3, 0x66, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x3F8;
    regs.rdi = DATA_ADDR;
    regs.rcx = 4; // Read 4 words
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let mut count = 0u16;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { size, .. } => {
                assert_eq!(size, 2);
                count += 1;
                let val = count * 0x1000;
                vcpu.complete_io_in(&val.to_le_bytes());
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 4);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(
        regs.rdi,
        DATA_ADDR + 8,
        "RDI should increment by 8 (4 words)"
    );

    // Verify memory
    assert_eq!(read_mem_at_u16(&mem, DATA_ADDR), 0x1000);
    assert_eq!(read_mem_at_u16(&mem, DATA_ADDR + 2), 0x2000);
    assert_eq!(read_mem_at_u16(&mem, DATA_ADDR + 4), 0x3000);
    assert_eq!(read_mem_at_u16(&mem, DATA_ADDR + 6), 0x4000);
}

#[test]
fn test_rep_insw_zero_count() {
    // REP INSW with RCX=0
    let code = [0xF3, 0x66, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F8;
    regs.rdi = DATA_ADDR;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut io_count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                io_count += 1;
                vcpu.complete_io_in(&[0, 0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_count, 0);
}

#[test]
fn test_rep_insw_df_set() {
    // REP INSW with DF=1
    let code = [0xF3, 0x66, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F9;
    regs.rdi = DATA_ADDR + 100;
    regs.rcx = 3;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                count += 1;
                vcpu.complete_io_in(&[0xAA, 0xBB]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(
        regs.rdi,
        DATA_ADDR + 100 - 6,
        "RDI should decrement by 6 (3 words)"
    );
}

// ============================================================================
// REP INSD - Repeated dword input
// ============================================================================

#[test]
fn test_rep_insd_basic() {
    // REP INSD - Input RCX dwords
    // F3 6D = REP INSD
    let code = [0xF3, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCFC;
    regs.rdi = DATA_ADDR;
    regs.rcx = 3; // Read 3 dwords
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let mut count = 0u32;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { size, .. } => {
                assert_eq!(size, 4);
                count += 1;
                let val = count * 0x10000000;
                vcpu.complete_io_in(&val.to_le_bytes());
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 3);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(
        regs.rdi,
        DATA_ADDR + 12,
        "RDI should increment by 12 (3 dwords)"
    );

    // Verify memory
    assert_eq!(read_mem_at_u32(&mem, DATA_ADDR), 0x10000000);
    assert_eq!(read_mem_at_u32(&mem, DATA_ADDR + 4), 0x20000000);
    assert_eq!(read_mem_at_u32(&mem, DATA_ADDR + 8), 0x30000000);
}

#[test]
fn test_rep_insd_zero_count() {
    // REP INSD with RCX=0
    let code = [0xF3, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF8;
    regs.rdi = DATA_ADDR;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut io_count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                io_count += 1;
                vcpu.complete_io_in(&[0, 0, 0, 0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_count, 0);
}

#[test]
fn test_rep_insd_df_set() {
    // REP INSD with DF=1
    let code = [0xF3, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF9;
    regs.rdi = DATA_ADDR + 200;
    regs.rcx = 2;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                count += 1;
                vcpu.complete_io_in(&[0x11, 0x22, 0x33, 0x44]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(
        regs.rdi,
        DATA_ADDR + 200 - 8,
        "RDI should decrement by 8 (2 dwords)"
    );
}

// ============================================================================
// Edge cases and combinations
// ============================================================================

#[test]
fn test_ins_mixed_sizes() {
    // INSB, INSW, INSD in sequence
    let code = [0x6C, 0x66, 0x6D, 0x6D, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xB0;
    regs.rdi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    let mut io_ops = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { size, .. } => {
                io_ops.push(size);
                match size {
                    1 => vcpu.complete_io_in(&[0xAA]),
                    2 => vcpu.complete_io_in(&[0xBB, 0xCC]),
                    4 => vcpu.complete_io_in(&[0xDD, 0xEE, 0xFF, 0x11]),
                    _ => panic!("Unexpected size"),
                }
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_ops, vec![1, 2, 4]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdi, DATA_ADDR + 7, "RDI should increment by 1+2+4=7");

    // Verify memory layout
    assert_eq!(read_mem_at_u8(&mem, DATA_ADDR), 0xAA);
    assert_eq!(read_mem_at_u16(&mem, DATA_ADDR + 1), 0xCCBB);
    assert_eq!(read_mem_at_u32(&mem, DATA_ADDR + 3), 0x11FFEEDD);
}

#[test]
fn test_rep_insb_single_iteration() {
    // REP INSB with RCX=1
    let code = [0xF3, 0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x62;
    regs.rdi = DATA_ADDR;
    regs.rcx = 1;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { .. } => {
                vcpu.complete_io_in(&[0x99]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rdi, DATA_ADDR + 1);
    assert_eq!(read_mem_u8(&mem), 0x99);
}

#[test]
fn test_ins_preserves_rdx() {
    // INS should not modify DX
    let code = [0x6C, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xDEADBEEF_00000080;
    regs.rdi = DATA_ADDR;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoIn { port, .. } => {
                assert_eq!(port, 0x80, "Only low 16 bits of DX used");
                vcpu.complete_io_in(&[0x42]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdx, 0xDEADBEEF_00000080, "DX should not be modified");
}
