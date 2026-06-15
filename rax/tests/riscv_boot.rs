//! End-to-end integration test: a bare-metal RISC-V program runs through the
//! VMM emulator backend (`RiscVVcpu`), producing UART console output via MMIO
//! and halting via `ecall`. This exercises the full wiring of the
//! `rax::riscv` interpreter into rax as a bootable architecture.

use std::sync::Arc;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use rax::backend::emulator::riscv::RiscVVcpu;
use rax::cpu::{CpuState, RiscVRegisters, VCpu, VcpuExit};

const CODE_ADDR: u64 = 0x1000;
const UART_BASE: u64 = 0x1000_0000;

// RV64I instruction encoders.
fn lui(rd: u32, imm20: u32) -> u32 {
    (imm20 << 12) | (rd << 7) | 0x37
}
fn addi(rd: u32, rs1: u32, imm: i32) -> u32 {
    (((imm as u32) & 0xfff) << 20) | (rs1 << 15) | (rd << 7) | 0x13
}
fn sb(rs2: u32, rs1: u32, imm: i32) -> u32 {
    let u = (imm as u32) & 0xfff;
    ((u >> 5) << 25) | (rs2 << 20) | (rs1 << 15) | (0 << 12) | ((u & 0x1f) << 7) | 0x23
}
const ECALL: u32 = 0x0000_0073;

fn run_program(code: &[u32]) -> (Vec<u8>, RiscVRegisters) {
    let mem =
        Arc::new(GuestMemoryMmap::<()>::from_ranges(&[(GuestAddress(0), 64 * 1024)]).unwrap());
    let bytes: Vec<u8> = code.iter().flat_map(|w| w.to_le_bytes()).collect();
    mem.write_slice(&bytes, GuestAddress(CODE_ADDR)).unwrap();

    let mut vcpu = RiscVVcpu::new(0, mem.clone());
    let mut regs = RiscVRegisters::default();
    regs.pc = CODE_ADDR;
    vcpu.set_state(&CpuState::riscv(regs)).unwrap();

    let mut output = Vec::new();
    loop {
        match vcpu.run().unwrap() {
            VcpuExit::Shutdown => break,
            VcpuExit::MmioWrite { addr, data } => {
                // Writes to the 16550 THR (UART base + 0) are console bytes.
                if addr == UART_BASE {
                    output.extend_from_slice(&data);
                }
            }
            VcpuExit::Hlt => break,
            other => panic!("unexpected vcpu exit: {other:?}"),
        }
    }

    let state = match vcpu.get_state().unwrap() {
        CpuState::RiscV(s) => s.regs,
        _ => panic!("expected riscv state"),
    };
    (output, state)
}

#[test]
fn boots_and_prints_via_uart() {
    // x10 = UART base; write 'R','V'; compute x13 = 7 + 35 = 42; ecall.
    let prog = [
        lui(10, UART_BASE as u32 >> 12), // x10 = 0x10000000
        addi(11, 0, b'R' as i32),
        sb(11, 10, 0),
        addi(11, 0, b'V' as i32),
        sb(11, 10, 0),
        addi(12, 0, 7),
        addi(13, 12, 35),
        ECALL,
    ];
    let (output, regs) = run_program(&prog);
    assert_eq!(output, b"RV", "UART console output");
    assert_eq!(regs.x[12], 7);
    assert_eq!(regs.x[13], 42, "x13 = 7 + 35");
    assert_eq!(regs.x[10], UART_BASE, "x10 holds UART base");
}

#[test]
fn loads_and_stores_to_ram() {
    // Store a doubleword to RAM and read it back into a register.
    // x5 = 0x2000; x6 = 0x1234; sd x6, 0(x5); ld x7, 0(x5).
    fn sd(rs2: u32, rs1: u32, imm: i32) -> u32 {
        let u = (imm as u32) & 0xfff;
        ((u >> 5) << 25) | (rs2 << 20) | (rs1 << 15) | (3 << 12) | ((u & 0x1f) << 7) | 0x23
    }
    fn ld(rd: u32, rs1: u32, imm: i32) -> u32 {
        (((imm as u32) & 0xfff) << 20) | (rs1 << 15) | (3 << 12) | (rd << 7) | 0x03
    }
    let prog = [
        lui(5, 2),         // x5 = 0x2000
        addi(6, 0, 0x123), // x6 = 0x123
        sd(6, 5, 0),
        ld(7, 5, 0),
        ECALL,
    ];
    let (_out, regs) = run_program(&prog);
    assert_eq!(regs.x[7], 0x123, "loaded value matches stored value");
}
