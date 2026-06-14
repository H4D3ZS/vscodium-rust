//! A32/T32/T16 test helpers.
//!
//! Common test infrastructure for AArch32 instruction tests.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use rax::arm::decoder::{Aarch32Decoder, ThumbDecoder};
use rax::arm::execution::{ArmMemory, FlatMemory};
pub use rax::arm::CpuExit;
use rax::arm::{Armv7Cpu, ExceptionType, ExecResult, Executor};

/// Processor state for flag assertions
#[derive(Debug, Clone, Copy, Default)]
pub struct PState {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
}

/// Test context combining CPU and memory
pub struct TestCpu {
    pub cpu: Armv7Cpu,
    pub mem: FlatMemory,
}

impl TestCpu {
    /// Execute one instruction
    pub fn step(&mut self) -> Result<CpuExit, String> {
        step(self)
    }

    /// Write bytes to memory (for compatibility with generated tests)
    pub fn write_memory(&mut self, addr: u64, bytes: &[u8]) -> Result<(), String> {
        for (i, &b) in bytes.iter().enumerate() {
            self.mem
                .write_byte((addr as u32) + i as u32, b)
                .map_err(|e| format!("write error: {:?}", e))?;
        }
        Ok(())
    }

    /// Read bytes from memory (for compatibility with generated tests)
    pub fn read_memory(&self, addr: u64, size: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(size);
        for i in 0..size {
            result.push(self.mem.read_byte((addr as u32) + i as u32).unwrap_or(0));
        }
        result
    }

    /// Get processor state flags (for compatibility with generated tests)
    pub fn get_pstate(&self) -> PState {
        PState {
            n: self.cpu.cpsr.n,
            z: self.cpu.cpsr.z,
            c: self.cpu.cpsr.c,
            v: self.cpu.cpsr.v,
        }
    }
}

/// Create a test CPU with default configuration (A32 mode)
pub fn create_test_cpu() -> TestCpu {
    let cpu = Armv7Cpu::new();
    let mem = FlatMemory::new(0x1000_0000, 0); // size, base
    TestCpu { cpu, mem }
}

/// Create a test CPU in Thumb mode
pub fn create_thumb_cpu() -> TestCpu {
    let mut cpu = Armv7Cpu::new();
    cpu.cpsr.t = true;
    let mem = FlatMemory::new(0x1000_0000, 0); // size, base
    TestCpu { cpu, mem }
}

/// Write a 32-bit instruction to memory
pub fn write_insn(ctx: &mut TestCpu, addr: u32, insn: u32) {
    ctx.mem.write_word(addr, insn).unwrap();
}

/// Write a 16-bit Thumb instruction to memory
pub fn write_insn16(ctx: &mut TestCpu, addr: u32, insn: u16) {
    ctx.mem.write_halfword(addr, insn).unwrap();
}

/// Write bytes to memory
pub fn write_memory(ctx: &mut TestCpu, addr: u32, bytes: &[u8]) {
    for (i, &b) in bytes.iter().enumerate() {
        ctx.mem.write_byte(addr + i as u32, b).unwrap();
    }
}

/// Read bytes from memory
pub fn read_memory(ctx: &TestCpu, addr: u32, size: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(size);
    for i in 0..size {
        result.push(ctx.mem.read_byte(addr + i as u32).unwrap());
    }
    result
}

/// Set a general purpose register (R0-R14)
pub fn set_w(ctx: &mut TestCpu, reg: u8, value: u32) {
    ctx.cpu.set_reg(reg as usize, value);
}

/// Get a general purpose register (R0-R14)
pub fn get_w(ctx: &TestCpu, reg: u8) -> u32 {
    ctx.cpu.reg(reg as usize)
}

/// Set the stack pointer (R13/SP)
pub fn set_sp(ctx: &mut TestCpu, value: u32) {
    ctx.cpu.set_reg(13, value);
}

/// Get the stack pointer (R13/SP)
pub fn get_sp(ctx: &TestCpu) -> u32 {
    ctx.cpu.reg(13)
}

/// Set the link register (R14/LR)
pub fn set_lr(ctx: &mut TestCpu, value: u32) {
    ctx.cpu.set_reg(14, value);
}

/// Get the link register (R14/LR)
pub fn get_lr(ctx: &TestCpu) -> u32 {
    ctx.cpu.reg(14)
}

/// Set the PC register
pub fn set_pc(ctx: &mut TestCpu, value: u32) {
    ctx.cpu.set_reg(15, value);
}

/// Get the PC register
pub fn get_pc(ctx: &TestCpu) -> u32 {
    ctx.cpu.get_pc()
}

/// Execute one instruction and return the exit status
pub fn step(ctx: &mut TestCpu) -> Result<CpuExit, String> {
    let pc = ctx.cpu.get_pc();
    let is_thumb = ctx.cpu.cpsr.t;

    // Fetch and decode instruction
    let decoded = if is_thumb {
        let hw = ctx
            .mem
            .read_halfword(pc)
            .map_err(|e| format!("fetch error: {:?}", e))?;
        // Check for 32-bit Thumb instruction
        if (hw >> 11) >= 0x1D {
            let hw2 = ctx
                .mem
                .read_halfword(pc + 2)
                .map_err(|e| format!("fetch error: {:?}", e))?;
            let insn32 = ((hw as u32) << 16) | (hw2 as u32);
            ThumbDecoder::decode_32bit(insn32).map_err(|e| format!("decode error: {:?}", e))?
        } else {
            ThumbDecoder::decode_16bit(hw).map_err(|e| format!("decode error: {:?}", e))?
        }
    } else {
        let insn = ctx
            .mem
            .read_word(pc)
            .map_err(|e| format!("fetch error: {:?}", e))?;
        Aarch32Decoder::decode(insn).map_err(|e| format!("decode error: {:?}", e))?
    };

    // Calculate instruction size for PC advance
    let insn_size = if is_thumb {
        let hw = ctx.mem.read_halfword(pc).unwrap();
        if (hw >> 11) >= 0x1D {
            4
        } else {
            2
        }
    } else {
        4
    };

    // Execute
    let mut exec = Executor::new(&mut ctx.cpu, &mut ctx.mem);
    match exec.execute(&decoded) {
        ExecResult::Continue => {
            // Advance PC
            ctx.cpu.set_reg(15, pc + insn_size);
            Ok(CpuExit::Continue)
        }
        ExecResult::Branch(addr) => {
            ctx.cpu.set_reg(15, addr);
            Ok(CpuExit::Continue)
        }
        ExecResult::Halt => Ok(CpuExit::Halt),
        ExecResult::Undefined => Ok(CpuExit::Undefined(0)),
        ExecResult::Exception(exc) => match exc {
            ExceptionType::SupervisorCall(imm) => Ok(CpuExit::Svc(imm)),
            ExceptionType::UndefinedInstruction => Ok(CpuExit::Undefined(0)),
            ExceptionType::Breakpoint(imm) => Ok(CpuExit::Breakpoint(imm as u32)),
            _ => Err(format!("exception: {:?}", exc)),
        },
        ExecResult::MemoryFault(e) => Err(format!("memory fault: {:?}", e)),
    }
}
