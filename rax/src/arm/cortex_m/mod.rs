//! Cortex-M processor implementation.
//!
//! This module provides complete Cortex-M emulation including:
//!
//! - **CPU Core**: Full Thumb/Thumb-2 instruction execution
//! - **NVIC**: Nested Vectored Interrupt Controller with priority handling
//! - **SysTick**: System timer for periodic interrupts
//! - **SCB**: System Control Block for configuration and fault handling
//!
//! # Supported Variants
//!
//! | Variant | Architecture | Features |
//! |---------|--------------|----------|
//! | Cortex-M0 | ARMv6-M | Basic Thumb |
//! | Cortex-M0+ | ARMv6-M | Basic Thumb, MTB |
//! | Cortex-M3 | ARMv7-M | Thumb-2, HW divide |
//! | Cortex-M4 | ARMv7E-M | DSP, optional FPU |
//! | Cortex-M7 | ARMv7E-M | DSP, DP FPU, cache |
//! | Cortex-M23 | ARMv8-M Baseline | TrustZone |
//! | Cortex-M33 | ARMv8-M Mainline | TrustZone, DSP, FPU |
//! | Cortex-M55 | ARMv8.1-M | MVE (Helium), TrustZone |
//! | Cortex-M85 | ARMv8.1-M | MVE, TrustZone, Pacbti |
//!
//! # Example
//!
//! ```ignore
//! use rax::arm::cortex_m::{CortexMCpu, CortexMVariant};
//! use rax::arm::memory::FlatMemory;
//!
//! // Create a Cortex-M4 with 64KB of RAM
//! let memory = Box::new(FlatMemory::new(0x2000_0000, 0x1_0000));
//! let mut cpu = CortexMCpu::new(CortexMVariant::CortexM4, memory);
//!
//! // Load program at vector table
//! // ... set up memory with code ...
//!
//! // Reset and run
//! cpu.reset();
//! loop {
//!     match cpu.step() {
//!         Ok(CpuExit::Continue) => continue,
//!         Ok(CpuExit::Halt) => break,
//!         Ok(exit) => println!("Exit: {:?}", exit),
//!         Err(e) => panic!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! # Memory Map
//!
//! The Cortex-M memory map follows the ARM architecture:
//!
//! | Region | Address Range | Description |
//! |--------|---------------|-------------|
//! | Code | 0x0000_0000 - 0x1FFF_FFFF | Flash/ROM |
//! | SRAM | 0x2000_0000 - 0x3FFF_FFFF | RAM |
//! | Peripheral | 0x4000_0000 - 0x5FFF_FFFF | On-chip peripherals |
//! | External RAM | 0x6000_0000 - 0x9FFF_FFFF | External memory |
//! | External Device | 0xA000_0000 - 0xDFFF_FFFF | External peripherals |
//! | System | 0xE000_0000 - 0xFFFF_FFFF | Private peripheral bus |
//!
//! ## System Region Details
//!
//! | Subregion | Address Range | Description |
//! |-----------|---------------|-------------|
//! | ITM | 0xE000_0000 - 0xE000_0FFF | Instrumentation Trace |
//! | DWT | 0xE000_1000 - 0xE000_1FFF | Data Watchpoint and Trace |
//! | FPB | 0xE000_2000 - 0xE000_2FFF | Flash Patch and Breakpoint |
//! | SCS | 0xE000_E000 - 0xE000_EFFF | System Control Space |
//! | TPIU | 0xE004_0000 - 0xE004_0FFF | Trace Port Interface |
//! | ETM | 0xE004_1000 - 0xE004_1FFF | Embedded Trace Macrocell |

pub mod cpu;
pub mod nvic;
pub mod scb;
pub mod systick;

pub use cpu::CortexMCpu;
pub use nvic::Nvic;
pub use scb::{CortexMVariant, Scb};
pub use systick::SysTick;

/// System Control Space (SCS) base address.
pub const SCS_BASE: u32 = 0xE000_E000;

/// SysTick register base offset from SCS.
pub const SYSTICK_OFFSET: u32 = 0x010;

/// NVIC register base offset from SCS.
pub const NVIC_OFFSET: u32 = 0x100;

/// SCB register base offset from SCS.
pub const SCB_OFFSET: u32 = 0xD00;

/// MPU register base offset from SCS.
pub const MPU_OFFSET: u32 = 0xD90;

/// FPU register base offset from SCS.
pub const FPU_OFFSET: u32 = 0xF30;

/// Debug register base offset from SCS.
pub const DEBUG_OFFSET: u32 = 0xDF0;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arm::cpu_trait::{ArmCpu, CpuExit};
    use crate::arm::memory::{ArmMemory, FlatMemory};

    #[test]
    fn test_cortex_m4_basic() {
        let mut memory = FlatMemory::new(0, 0x10000);

        // Set up vector table
        memory.write_u32(0, 0x8000).unwrap(); // Initial SP
        memory.write_u32(4, 0x101).unwrap(); // Reset handler (Thumb)

        // Simple program: MOV R0, #1; MOV R1, #2; ADD R2, R0, R1; WFI
        memory.write_u16(0x100, 0x2001).unwrap(); // MOV R0, #1
        memory.write_u16(0x102, 0x2102).unwrap(); // MOV R1, #2
        memory.write_u16(0x104, 0x1842).unwrap(); // ADD R2, R0, R1 (Rd=2, Rn=0, Rm=1)
        memory.write_u16(0x106, 0xBF30).unwrap(); // WFI

        let mut cpu = CortexMCpu::new(CortexMVariant::CortexM4, Box::new(memory));
        cpu.reset();

        // Execute instructions
        for _ in 0..3 {
            let exit = cpu.step().unwrap();
            assert!(matches!(exit, CpuExit::Continue));
        }

        // Check results
        assert_eq!(cpu.get_gpr(0), 1);
        assert_eq!(cpu.get_gpr(1), 2);
        assert_eq!(cpu.get_gpr(2), 3);

        // Should hit WFI
        let exit = cpu.step().unwrap();
        assert!(matches!(exit, CpuExit::Wfi));
    }

    #[test]
    fn test_cortex_m0_basic() {
        let mut memory = FlatMemory::new(0, 0x10000);

        memory.write_u32(0, 0x8000).unwrap();
        memory.write_u32(4, 0x101).unwrap();
        memory.write_u16(0x100, 0x2042).unwrap(); // MOV R0, #0x42

        let mut cpu = CortexMCpu::new(CortexMVariant::CortexM0, Box::new(memory));
        assert!(!cpu.has_fpu()); // M0 has no FPU

        cpu.reset();
        cpu.step().unwrap();

        assert_eq!(cpu.get_gpr(0), 0x42);
    }

    #[test]
    fn test_interrupt_injection() {
        let mut memory = FlatMemory::new(0, 0x10000);

        memory.write_u32(0, 0x8000).unwrap();
        memory.write_u32(4, 0x101).unwrap();

        // IRQ0 handler at exception 16
        memory.write_u32(64, 0x201).unwrap(); // Vector for IRQ0

        let mut cpu = CortexMCpu::new(CortexMVariant::CortexM4, Box::new(memory));
        cpu.reset();

        // Enable and trigger IRQ0
        cpu.nvic_mut().enable(0);
        cpu.nvic_mut().set_pending(0);

        // Should detect pending exception
        assert!(cpu.nvic().has_pending_exception());
    }

    #[test]
    fn test_systick_basic() {
        let memory = FlatMemory::new(0, 0x10000);
        let cpu = CortexMCpu::new(CortexMVariant::CortexM4, Box::new(memory));

        // SysTick should be available
        assert!(!cpu.systick().is_enabled());
    }

    #[test]
    fn test_scb_basic() {
        let memory = FlatMemory::new(0, 0x10000);
        let cpu = CortexMCpu::new(CortexMVariant::CortexM4, Box::new(memory));

        // Check CPUID
        assert_eq!(cpu.scb().variant(), CortexMVariant::CortexM4);
    }
}
