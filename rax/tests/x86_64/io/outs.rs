//! Tests for the OUTS/OUTSB/OUTSW/OUTSD instructions.
//!
//! OUTS/OUTSB/OUTSW/OUTSD - Output String to Port
//!
//! Copies data from the memory location specified by DS:(R/E)SI to the I/O port
//! specified in DX. After the transfer, (R/E)SI is incremented or decremented
//! based on the DF flag.
//!
//! Flags affected: None
//!
//! Reference: docs/outs:outsb:outsw:outsd.txt

use crate::common::*;
use rax::backend::emulator::x86_64::flags;
use rax::cpu::{Registers, VcpuExit};

// ============================================================================
// OUTSB (opcode 6E) - Output byte string
// ============================================================================

#[test]
fn test_outsb_basic() {
    // OUTSB - Output byte from [RSI] to port DX
    // 6E = OUTSB
    // F4 = HLT
    let code = [0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x60; // Keyboard command port
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF; // DF=0, increment
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data to memory
    write_mem_u8(&mem, 0x42);

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0x60);
                assert_eq!(data.len(), 1);
                captured_value = Some(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(captured_value, Some(0x42), "Should output byte from [RSI]");
    assert_eq!(regs.rsi, DATA_ADDR + 1, "RSI should increment by 1");
}

#[test]
fn test_outsb_df_clear_increment() {
    // OUTSB with DF=0 should increment RSI
    let code = [0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x70;
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF; // DF=0
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 0xAA);

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {}
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR + 1, "DF=0: RSI should increment");
}

#[test]
fn test_outsb_df_set_decrement() {
    // OUTSB with DF=1 should decrement RSI
    let code = [0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x71;
    regs.rsi = DATA_ADDR;
    regs.rflags |= flags::bits::DF; // DF=1
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 0xBB);

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {}
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR - 1, "DF=1: RSI should decrement");
}

#[test]
fn test_outsb_preserves_flags() {
    // OUTSB should not modify flags (except DF is used)
    let code = [0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x72;
    regs.rsi = DATA_ADDR;
    regs.rflags = 0x246; // Set some flags, DF=0
    let initial_flags = regs.rflags;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 0xCC);

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {}
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
fn test_outsb_multiple_sequential() {
    // Multiple OUTSB instructions
    // OUTSB; OUTSB; OUTSB
    let code = [0x6E, 0x6E, 0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x80;
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    write_mem_at_u8(&mem, DATA_ADDR, 0x11);
    write_mem_at_u8(&mem, DATA_ADDR + 1, 0x22);
    write_mem_at_u8(&mem, DATA_ADDR + 2, 0x33);

    let mut values = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                values.push(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(values, vec![0x11, 0x22, 0x33]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR + 3);
}

// ============================================================================
// OUTSW (opcode 6F with 66 prefix) - Output word string
// ============================================================================

#[test]
fn test_outsw_basic() {
    // OUTSW - Output word from [RSI] to port DX
    // 66 6F = OUTSW
    let code = [0x66, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x3F8;
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u16(&mem, 0x1234);

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0x3F8);
                assert_eq!(data.len(), 2);
                captured_value = Some(u16::from_le_bytes([data[0], data[1]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        captured_value,
        Some(0x1234),
        "Should output word from [RSI]"
    );
    assert_eq!(regs.rsi, DATA_ADDR + 2, "RSI should increment by 2");
}

#[test]
fn test_outsw_df_clear_increment() {
    // OUTSW with DF=0 should increment RSI by 2
    let code = [0x66, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F8;
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u16(&mem, 0xBBAA);

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {}
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR + 2, "DF=0: RSI should increment by 2");
}

#[test]
fn test_outsw_df_set_decrement() {
    // OUTSW with DF=1 should decrement RSI by 2
    let code = [0x66, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F9;
    regs.rsi = DATA_ADDR;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u16(&mem, 0xDDCC);

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {}
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR - 2, "DF=1: RSI should decrement by 2");
}

#[test]
fn test_outsw_multiple_sequential() {
    // Multiple OUTSW instructions
    let code = [0x66, 0x6F, 0x66, 0x6F, 0x66, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x90;
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    write_mem_at_u16(&mem, DATA_ADDR, 0x1111);
    write_mem_at_u16(&mem, DATA_ADDR + 2, 0x2222);
    write_mem_at_u16(&mem, DATA_ADDR + 4, 0x3333);

    let mut values = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                values.push(u16::from_le_bytes([data[0], data[1]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(values, vec![0x1111, 0x2222, 0x3333]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR + 6);
}

// ============================================================================
// OUTSD (opcode 6F) - Output dword string
// ============================================================================

#[test]
fn test_outsd_basic() {
    // OUTSD - Output dword from [RSI] to port DX
    // 6F = OUTSD
    let code = [0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCFC; // PCI config data
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u32(&mem, 0x12345678);

    let mut captured_value = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0xCFC);
                assert_eq!(data.len(), 4);
                captured_value = Some(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(
        captured_value,
        Some(0x12345678),
        "Should output dword from [RSI]"
    );
    assert_eq!(regs.rsi, DATA_ADDR + 4, "RSI should increment by 4");
}

#[test]
fn test_outsd_df_clear_increment() {
    // OUTSD with DF=0 should increment RSI by 4
    let code = [0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF8;
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u32(&mem, 0xDDCCBBAA);

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {}
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR + 4, "DF=0: RSI should increment by 4");
}

#[test]
fn test_outsd_df_set_decrement() {
    // OUTSD with DF=1 should decrement RSI by 4
    let code = [0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF9;
    regs.rsi = DATA_ADDR;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u32(&mem, 0x44332211);

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {}
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR - 4, "DF=1: RSI should decrement by 4");
}

#[test]
fn test_outsd_multiple_sequential() {
    // Multiple OUTSD instructions
    let code = [0x6F, 0x6F, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xA0;
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    write_mem_at_u32(&mem, DATA_ADDR, 0x11111111);
    write_mem_at_u32(&mem, DATA_ADDR + 4, 0x22222222);
    write_mem_at_u32(&mem, DATA_ADDR + 8, 0x33333333);

    let mut values = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                values.push(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(values, vec![0x11111111, 0x22222222, 0x33333333]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR + 12);
}

// ============================================================================
// REP OUTSB - Repeated byte output
// ============================================================================

#[test]
fn test_rep_outsb_basic() {
    // REP OUTSB - Output RCX bytes
    // F3 6E = REP OUTSB
    let code = [0xF3, 0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x60;
    regs.rsi = DATA_ADDR;
    regs.rcx = 5; // Write 5 bytes
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    for i in 0..5 {
        write_mem_at_u8(&mem, DATA_ADDR + i, (i + 1) as u8);
    }

    let mut values = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, data } => {
                assert_eq!(port, 0x60);
                assert_eq!(data.len(), 1);
                values.push(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(values, vec![1, 2, 3, 4, 5]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0, "RCX should be 0 after REP");
    assert_eq!(regs.rsi, DATA_ADDR + 5, "RSI should increment by 5");
}

#[test]
fn test_rep_outsb_zero_count() {
    // REP OUTSB with RCX=0 should not execute
    let code = [0xF3, 0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x60;
    regs.rsi = DATA_ADDR;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut io_count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {
                io_count += 1;
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_count, 0, "No I/O should occur with RCX=0");
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR, "RSI should not change");
}

#[test]
fn test_rep_outsb_df_set() {
    // REP OUTSB with DF=1 should decrement RSI
    let code = [0xF3, 0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x61;
    regs.rsi = DATA_ADDR + 100; // Start high
    regs.rcx = 3;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    write_mem_at_u8(&mem, DATA_ADDR + 100, 0xAA);
    write_mem_at_u8(&mem, DATA_ADDR + 99, 0xBB);
    write_mem_at_u8(&mem, DATA_ADDR + 98, 0xCC);

    let mut values = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                values.push(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(values, vec![0xAA, 0xBB, 0xCC]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, DATA_ADDR + 100 - 3, "RSI should decrement by 3");
}

// ============================================================================
// REP OUTSW - Repeated word output
// ============================================================================

#[test]
fn test_rep_outsw_basic() {
    // REP OUTSW - Output RCX words
    // F3 66 6F = REP OUTSW
    let code = [0xF3, 0x66, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x3F8;
    regs.rsi = DATA_ADDR;
    regs.rcx = 4; // Write 4 words
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    for i in 0..4 {
        write_mem_at_u16(&mem, DATA_ADDR + i * 2, (i + 1) as u16 * 0x1000);
    }

    let mut values = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                assert_eq!(data.len(), 2);
                values.push(u16::from_le_bytes([data[0], data[1]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(values, vec![0x1000, 0x2000, 0x3000, 0x4000]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(
        regs.rsi,
        DATA_ADDR + 8,
        "RSI should increment by 8 (4 words)"
    );
}

#[test]
fn test_rep_outsw_zero_count() {
    // REP OUTSW with RCX=0
    let code = [0xF3, 0x66, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F8;
    regs.rsi = DATA_ADDR;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut io_count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {
                io_count += 1;
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_count, 0);
}

#[test]
fn test_rep_outsw_df_set() {
    // REP OUTSW with DF=1
    let code = [0xF3, 0x66, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x2F9;
    regs.rsi = DATA_ADDR + 100;
    regs.rcx = 3;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    write_mem_at_u16(&mem, DATA_ADDR + 100, 0x1111);
    write_mem_at_u16(&mem, DATA_ADDR + 98, 0x2222);
    write_mem_at_u16(&mem, DATA_ADDR + 96, 0x3333);

    let mut count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {
                count += 1;
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 3);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(
        regs.rsi,
        DATA_ADDR + 100 - 6,
        "RSI should decrement by 6 (3 words)"
    );
}

// ============================================================================
// REP OUTSD - Repeated dword output
// ============================================================================

#[test]
fn test_rep_outsd_basic() {
    // REP OUTSD - Output RCX dwords
    // F3 6F = REP OUTSD
    let code = [0xF3, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCFC;
    regs.rsi = DATA_ADDR;
    regs.rcx = 3; // Write 3 dwords
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    write_mem_at_u32(&mem, DATA_ADDR, 0x10000000);
    write_mem_at_u32(&mem, DATA_ADDR + 4, 0x20000000);
    write_mem_at_u32(&mem, DATA_ADDR + 8, 0x30000000);

    let mut values = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                assert_eq!(data.len(), 4);
                values.push(u32::from_le_bytes([data[0], data[1], data[2], data[3]]));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(values, vec![0x10000000, 0x20000000, 0x30000000]);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(
        regs.rsi,
        DATA_ADDR + 12,
        "RSI should increment by 12 (3 dwords)"
    );
}

#[test]
fn test_rep_outsd_zero_count() {
    // REP OUTSD with RCX=0
    let code = [0xF3, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF8;
    regs.rsi = DATA_ADDR;
    regs.rcx = 0;
    let (mut vcpu, _) = setup_vm(&code, Some(regs));

    let mut io_count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {
                io_count += 1;
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_count, 0);
}

#[test]
fn test_rep_outsd_df_set() {
    // REP OUTSD with DF=1
    let code = [0xF3, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xCF9;
    regs.rsi = DATA_ADDR + 200;
    regs.rcx = 2;
    regs.rflags |= flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    write_mem_at_u32(&mem, DATA_ADDR + 200, 0x11111111);
    write_mem_at_u32(&mem, DATA_ADDR + 196, 0x22222222);

    let mut count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { .. } => {
                count += 1;
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 2);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(
        regs.rsi,
        DATA_ADDR + 200 - 8,
        "RSI should decrement by 8 (2 dwords)"
    );
}

// ============================================================================
// Edge cases and combinations
// ============================================================================

#[test]
fn test_outs_mixed_sizes() {
    // OUTSB, OUTSW, OUTSD in sequence
    let code = [0x6E, 0x66, 0x6F, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xB0;
    regs.rsi = DATA_ADDR;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    write_mem_at_u8(&mem, DATA_ADDR, 0xAA);
    write_mem_at_u16(&mem, DATA_ADDR + 1, 0xCCBB);
    write_mem_at_u32(&mem, DATA_ADDR + 3, 0x11FFEEDD);

    let mut io_ops = vec![];
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                io_ops.push((data.len(), data.clone()));
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_ops.len(), 3);
    assert_eq!(io_ops[0], (1, vec![0xAA]));
    assert_eq!(io_ops[1], (2, vec![0xBB, 0xCC]));
    assert_eq!(io_ops[2], (4, vec![0xDD, 0xEE, 0xFF, 0x11]));

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rsi, DATA_ADDR + 7, "RSI should increment by 1+2+4=7");
}

#[test]
fn test_rep_outsb_single_iteration() {
    // REP OUTSB with RCX=1
    let code = [0xF3, 0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x62;
    regs.rsi = DATA_ADDR;
    regs.rcx = 1;
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 0x99);

    let mut io_count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                io_count += 1;
                assert_eq!(data[0], 0x99);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(io_count, 1);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, DATA_ADDR + 1);
}

#[test]
fn test_outs_preserves_rdx() {
    // OUTS should not modify DX
    let code = [0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xDEADBEEF_00000080;
    regs.rsi = DATA_ADDR;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 0x42);

    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { port, .. } => {
                assert_eq!(port, 0x80, "Only low 16 bits of DX used");
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rdx, 0xDEADBEEF_00000080, "DX should not be modified");
}

#[test]
fn test_outs_reads_memory_not_registers() {
    // OUTS reads from memory, not registers
    let code = [0x6E, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0x80;
    regs.rsi = DATA_ADDR;
    regs.rax = 0xFFFFFFFF; // Set RAX to something else
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    write_mem_u8(&mem, 0x42); // Memory value

    let mut captured = None;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                captured = Some(data[0]);
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(captured, Some(0x42), "Should read from memory, not RAX");
}

#[test]
fn test_rep_outsd_large_transfer() {
    // REP OUTSD with larger count
    let code = [0xF3, 0x6F, 0xF4];
    let mut regs = Registers::default();
    regs.rdx = 0xD0;
    regs.rsi = DATA_ADDR;
    regs.rcx = 10; // Write 10 dwords
    regs.rflags &= !flags::bits::DF;
    let (mut vcpu, mem) = setup_vm(&code, Some(regs));

    // Write test data
    for i in 0..10 {
        write_mem_at_u32(&mem, DATA_ADDR + i * 4, (i as u32 + 1) * 0x10101010);
    }

    let mut count = 0;
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::IoOut { data, .. } => {
                assert_eq!(data.len(), 4);
                count += 1;
            }
            VcpuExit::Hlt => break,
            _ => continue,
        }
    }

    assert_eq!(count, 10);
    let regs = vcpu.get_regs().unwrap();
    assert_eq!(regs.rcx, 0);
    assert_eq!(regs.rsi, DATA_ADDR + 40, "RSI should increment by 40");
}
