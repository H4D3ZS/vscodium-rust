//! ARM emulator backend.
//!
//! This module provides a software-based ARM CPU emulator that integrates
//! with the RAX VMM infrastructure.

use std::sync::Arc;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use crate::arm::{
    Armv7Cpu, ArmMemory, Decoder, Executor, ExecResult, ExceptionType,
    ExecutionState, MemoryError, Psr, ProcessorMode,
};
use crate::arm::cp15::Cp15State;
use crate::arm::vfp::VfpState;
use crate::cpu::{
    VCpu, VcpuExit, CpuState, 
    Aarch32CpuState, Aarch32Registers, Aarch32SystemRegisters,
};
use crate::error::{Error, Result};

/// ARM vCPU implementation for emulation.
pub struct ArmVcpu {
    /// vCPU ID.
    id: u32,
    /// Guest memory.
    mem: Arc<GuestMemoryMmap>,
    /// CPU state.
    cpu: Armv7Cpu,
    /// CP15 coprocessor state.
    cp15: Cp15State,
    /// VFP/NEON state.
    vfp: VfpState,
    /// Instruction decoder.
    decoder: Decoder,
    /// Total instructions executed.
    instructions_executed: u64,
    /// Pending I/O data for completion.
    pending_io: Option<Vec<u8>>,
}

impl ArmVcpu {
    /// Create a new ARM vCPU.
    pub fn new(id: u32, mem: Arc<GuestMemoryMmap>) -> Self {
        ArmVcpu {
            id,
            mem,
            cpu: Armv7Cpu::new(),
            cp15: Cp15State::new(),
            vfp: VfpState::new(),
            decoder: Decoder::new_aarch32(),
            instructions_executed: 0,
            pending_io: None,
        }
    }
    
    /// Get reference to CPU state.
    pub fn cpu(&self) -> &Armv7Cpu {
        &self.cpu
    }
    
    /// Get mutable reference to CPU state.
    pub fn cpu_mut(&mut self) -> &mut Armv7Cpu {
        &mut self.cpu
    }
    
    /// Get reference to CP15 state.
    pub fn cp15(&self) -> &Cp15State {
        &self.cp15
    }
    
    /// Get mutable reference to CP15 state.
    pub fn cp15_mut(&mut self) -> &mut Cp15State {
        &mut self.cp15
    }
    
    /// Get reference to VFP state.
    pub fn vfp(&self) -> &VfpState {
        &self.vfp
    }
    
    /// Get mutable reference to VFP state.
    pub fn vfp_mut(&mut self) -> &mut VfpState {
        &mut self.vfp
    }
    
    /// Convert internal state to CpuState.
    fn to_cpu_state(&self) -> Aarch32CpuState {
        let mut regs = Aarch32Registers::default();
        
        // Copy general-purpose registers R0-R12
        for i in 0..13 {
            regs.r[i] = self.cpu.regs[i];
        }
        regs.sp = self.cpu.regs[13];
        regs.lr = self.cpu.regs[14];
        regs.pc = self.cpu.regs[15];
        regs.cpsr = self.cpu.cpsr.to_u32();
        
        // Copy VFP state
        regs.fpscr = self.vfp.fpscr.bits();
        for i in 0..32 {
            regs.s[i] = self.vfp.read_s_bits(i as u8);
        }
        for i in 0..16 {
            regs.d_high[i] = self.vfp.read_d_bits((16 + i) as u8);
        }
        
        let sregs = Aarch32SystemRegisters {
            sctlr: self.cp15.sctlr.bits(),
            ttbr0: self.cp15.ttbr0 as u32,
            ttbr1: self.cp15.ttbr1 as u32,
            ttbcr: self.cp15.ttbcr,
            dacr: self.cp15.dacr,
            dfsr: self.cp15.dfsr,
            ifsr: self.cp15.ifsr,
            dfar: self.cp15.dfar,
            ifar: self.cp15.ifar,
            vbar: 0, // Vector base address not tracked in cp15 yet
            contextidr: self.cp15.contextidr,
            prrr: 0,
            nmrr: 0,
        };
        
        Aarch32CpuState { regs, sregs }
    }
    
    /// Apply CpuState to internal state.
    fn from_cpu_state(&mut self, state: &Aarch32CpuState) {
        // Copy general-purpose registers
        for i in 0..13 {
            self.cpu.regs[i] = state.regs.r[i];
        }
        self.cpu.regs[13] = state.regs.sp;
        self.cpu.regs[14] = state.regs.lr;
        self.cpu.regs[15] = state.regs.pc;
        self.cpu.cpsr = Psr::from_u32(state.regs.cpsr);
        
        // Copy VFP state
        self.vfp.fpscr = crate::arm::vfp::Fpscr::from_bits(state.regs.fpscr);
        for i in 0..32 {
            self.vfp.write_s_bits(i as u8, state.regs.s[i]);
        }
        for i in 0..16 {
            self.vfp.write_d_bits((16 + i) as u8, state.regs.d_high[i]);
        }
        
        // Copy CP15 state
        self.cp15.sctlr = crate::arm::cp15::Sctlr::from_bits(state.sregs.sctlr);
        self.cp15.ttbr0 = state.sregs.ttbr0 as u64;
        self.cp15.ttbr1 = state.sregs.ttbr1 as u64;
        self.cp15.ttbcr = state.sregs.ttbcr;
        self.cp15.dacr = state.sregs.dacr;
        self.cp15.dfsr = state.sregs.dfsr;
        self.cp15.ifsr = state.sregs.ifsr;
        self.cp15.dfar = state.sregs.dfar;
        self.cp15.ifar = state.sregs.ifar;
        self.cp15.contextidr = state.sregs.contextidr;
    }

    fn take_exception(&mut self, exception: ExceptionType) {
        let mut mem_wrapper = GuestMemoryWrapper { mem: &self.mem };
        let mut exec = Executor::new(&mut self.cpu, &mut mem_wrapper);
        exec.take_exception(exception);
    }
    
    /// Execute a single instruction.
    fn step(&mut self) -> Result<VcpuExit> {
        let pc = self.cpu.regs[15];
        let is_thumb = self.cpu.cpsr.t;
        
        // Fetch instruction
        let insn_bytes = if is_thumb {
            // Fetch 2 bytes first, may need 4 for Thumb-2
            let mut bytes = [0u8; 4];
            if self.mem.read(&mut bytes[0..2], GuestAddress(pc as u64)).is_err() {
                self.take_exception(ExceptionType::PrefetchAbort(pc));
                return Ok(VcpuExit::Hlt);
            }
            
            let hw1 = u16::from_le_bytes([bytes[0], bytes[1]]);
            
            // Check if this is a 32-bit Thumb-2 instruction
            if (hw1 >> 11) >= 0x1D {
                if self
                    .mem
                    .read(&mut bytes[2..4], GuestAddress((pc + 2) as u64))
                    .is_err()
                {
                    self.take_exception(ExceptionType::PrefetchAbort(pc + 2));
                    return Ok(VcpuExit::Hlt);
                }
                4
            } else {
                2
            }
        } else {
            4
        };
        
        // Fetch the instruction bytes
        let mut bytes = [0u8; 4];
        if self.mem.read(&mut bytes[0..insn_bytes], GuestAddress(pc as u64)).is_err() {
            self.take_exception(ExceptionType::PrefetchAbort(pc));
            return Ok(VcpuExit::Hlt);
        }
        
        // Decode
        let state = if is_thumb {
            ExecutionState::Thumb
        } else {
            ExecutionState::Aarch32
        };
        
        let insn = match self.decoder.decode_with_state(&bytes[0..insn_bytes], state) {
            Ok(insn) => insn,
            Err(_) => {
                self.take_exception(ExceptionType::UndefinedInstruction);
                return Ok(VcpuExit::Hlt);
            }
        };
        
        // Execute using a memory wrapper
        let mut mem_wrapper = GuestMemoryWrapper { mem: &self.mem };
        let mut exec = Executor::new(&mut self.cpu, &mut mem_wrapper);
        
        let result = exec.execute(&insn);
        self.instructions_executed += 1;
        
        // Handle result
        match result {
            ExecResult::Continue => {
                // Advance PC
                self.cpu.regs[15] = self.cpu.regs[15].wrapping_add(insn_bytes as u32);
                
                // Advance IT state if in IT block
                if self.cpu.cpsr.in_it_block() {
                    self.cpu.cpsr.advance_it_state();
                }
                
                Ok(VcpuExit::Hlt) // Continue is represented as Hlt for now
            }
            ExecResult::Branch(target) => {
                // Handle Thumb interworking
                if (target & 1) != 0 {
                    self.cpu.cpsr.t = true;
                    self.cpu.regs[15] = target & !1;
                } else {
                    self.cpu.cpsr.t = false;
                    self.cpu.regs[15] = target;
                }
                Ok(VcpuExit::Hlt)
            }
            ExecResult::Exception(exception) => {
                self.take_exception(exception);
                Ok(VcpuExit::Hlt)
            }
            ExecResult::Halt => {
                Ok(VcpuExit::Hlt)
            }
            ExecResult::Undefined => {
                self.take_exception(ExceptionType::UndefinedInstruction);
                Ok(VcpuExit::Hlt)
            }
            ExecResult::MemoryFault(e) => {
                self.take_exception(ExceptionType::DataAbort(memory_error_addr(&e)));
                Ok(VcpuExit::Hlt)
            }
        }
    }
}

fn memory_error_addr(error: &MemoryError) -> u32 {
    match error {
        MemoryError::Unaligned(addr)
        | MemoryError::OutOfBounds(addr)
        | MemoryError::PermissionDenied(addr)
        | MemoryError::BusError(addr) => *addr,
    }
}

impl VCpu for ArmVcpu {
    fn id(&self) -> u32 {
        self.id
    }
    
    fn run(&mut self) -> Result<VcpuExit> {
        // Run one instruction
        self.step()
    }
    
    fn get_state(&self) -> Result<CpuState> {
        Ok(CpuState::Aarch32(self.to_cpu_state()))
    }
    
    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        match state {
            CpuState::Aarch32(aarch32_state) => {
                self.from_cpu_state(aarch32_state);
                Ok(())
            }
            _ => Err(Error::InvalidConfig("Expected Aarch32 CPU state".to_string())),
        }
    }
    
    fn complete_io_in(&mut self, data: &[u8]) {
        self.pending_io = Some(data.to_vec());
    }
    
    fn inject_interrupt(&mut self, vector: u8) -> Result<bool> {
        // Check if IRQ is enabled
        if !self.cpu.cpsr.i {
            // Take IRQ exception
            let mut mem_wrapper = GuestMemoryWrapper { mem: &self.mem };
            let mut exec = Executor::new(&mut self.cpu, &mut mem_wrapper);
            exec.take_exception(ExceptionType::Irq);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    fn can_inject_interrupt(&self) -> bool {
        !self.cpu.cpsr.i
    }
}

/// Wrapper to adapt GuestMemoryMmap to ArmMemory trait.
struct GuestMemoryWrapper<'a> {
    mem: &'a Arc<GuestMemoryMmap>,
}

impl<'a> ArmMemory for GuestMemoryWrapper<'a> {
    fn read_byte(&self, addr: u32) -> std::result::Result<u8, MemoryError> {
        let mut buf = [0u8; 1];
        self.mem.read(&mut buf, GuestAddress(addr as u64))
            .map_err(|_| MemoryError::OutOfBounds(addr))?;
        Ok(buf[0])
    }
    
    fn write_byte(&mut self, addr: u32, value: u8) -> std::result::Result<(), MemoryError> {
        self.mem.write(&[value], GuestAddress(addr as u64))
            .map_err(|_| MemoryError::OutOfBounds(addr))?;
        Ok(())
    }
    
    fn read_halfword(&self, addr: u32) -> std::result::Result<u16, MemoryError> {
        let mut buf = [0u8; 2];
        self.mem.read(&mut buf, GuestAddress(addr as u64))
            .map_err(|_| MemoryError::OutOfBounds(addr))?;
        Ok(u16::from_le_bytes(buf))
    }
    
    fn write_halfword(&mut self, addr: u32, value: u16) -> std::result::Result<(), MemoryError> {
        self.mem.write(&value.to_le_bytes(), GuestAddress(addr as u64))
            .map_err(|_| MemoryError::OutOfBounds(addr))?;
        Ok(())
    }
    
    fn read_word(&self, addr: u32) -> std::result::Result<u32, MemoryError> {
        let mut buf = [0u8; 4];
        self.mem.read(&mut buf, GuestAddress(addr as u64))
            .map_err(|_| MemoryError::OutOfBounds(addr))?;
        Ok(u32::from_le_bytes(buf))
    }
    
    fn write_word(&mut self, addr: u32, value: u32) -> std::result::Result<(), MemoryError> {
        self.mem.write(&value.to_le_bytes(), GuestAddress(addr as u64))
            .map_err(|_| MemoryError::OutOfBounds(addr))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm_memory::{GuestMemoryMmap, GuestRegionMmap, MmapRegion};
    
    fn create_test_memory() -> Arc<GuestMemoryMmap> {
        let region = GuestRegionMmap::new(
            MmapRegion::new(0x10000).unwrap(),
            GuestAddress(0),
        ).unwrap();
        Arc::new(GuestMemoryMmap::from_regions(vec![region]).unwrap())
    }
    
    #[test]
    fn test_arm_vcpu_creation() {
        let mem = create_test_memory();
        let vcpu = ArmVcpu::new(0, mem);
        
        assert_eq!(vcpu.id, 0);
        assert_eq!(vcpu.cpu.regs[15], 0); // PC starts at 0
    }
    
    #[test]
    fn test_arm_vcpu_state_roundtrip() {
        let mem = create_test_memory();
        let mut vcpu = ArmVcpu::new(0, mem);
        
        // Set some state
        vcpu.cpu.regs[0] = 0x1234;
        vcpu.cpu.regs[13] = 0x2000; // SP
        vcpu.cpu.regs[15] = 0x1000; // PC
        vcpu.cpu.cpsr.n = true;
        vcpu.cpu.cpsr.z = false;
        
        // Get state
        let state = vcpu.get_state().unwrap();
        
        // Reset and restore
        vcpu.cpu.regs[0] = 0;
        vcpu.cpu.regs[13] = 0;
        vcpu.cpu.regs[15] = 0;
        vcpu.cpu.cpsr.n = false;
        
        vcpu.set_state(&state).unwrap();
        
        assert_eq!(vcpu.cpu.regs[0], 0x1234);
        assert_eq!(vcpu.cpu.regs[13], 0x2000);
        assert_eq!(vcpu.cpu.regs[15], 0x1000);
        assert!(vcpu.cpu.cpsr.n);
    }

    #[test]
    fn test_fetch_fault_takes_prefetch_abort() {
        let mem = create_test_memory();
        let mut vcpu = ArmVcpu::new(0, mem);
        vcpu.cpu.regs[15] = 0x10000;

        let exit = vcpu.run().unwrap();

        assert!(matches!(exit, VcpuExit::Hlt));
        assert_eq!(vcpu.cpu.cpsr.mode, ProcessorMode::Abort as u8);
        assert_eq!(vcpu.cpu.regs[15], 0x0c);
        assert_eq!(vcpu.cpu.regs[14], 0x10004);
    }

    #[test]
    fn test_data_fault_takes_data_abort() {
        let mem = create_test_memory();
        mem.write(&0xe590_0000u32.to_le_bytes(), GuestAddress(0))
            .unwrap(); // LDR r0, [r0]
        let mut vcpu = ArmVcpu::new(0, mem);
        vcpu.cpu.regs[0] = 0x10000;

        let exit = vcpu.run().unwrap();

        assert!(matches!(exit, VcpuExit::Hlt));
        assert_eq!(vcpu.cpu.cpsr.mode, ProcessorMode::Abort as u8);
        assert_eq!(vcpu.cpu.regs[15], 0x10);
        assert_eq!(vcpu.cpu.regs[14], 0x08);
    }
}
