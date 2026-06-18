//! Simple test runner for x86_64 emulator validation.
//! Loads a flat binary at 0x100000 and runs it in 64-bit mode.

use std::env;
use std::fs;
use std::sync::Arc;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestRegionMmap, MmapRegion};

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::{Registers, SystemRegisters, VCpu, VcpuExit};

const LOAD_ADDR: u64 = 0x100000;
const MEM_SIZE: u64 = 16 * 1024 * 1024; // 16 MB

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <test.bin>", args[0]);
        std::process::exit(1);
    }

    let test_binary = &args[1];
    let code = fs::read(test_binary).expect("Failed to read test binary");
    println!("Loaded {} ({} bytes)", test_binary, code.len());

    // Create guest memory
    let region = MmapRegion::new(MEM_SIZE as usize).expect("Failed to create mmap region");
    let guest_region =
        GuestRegionMmap::new(region, GuestAddress(0)).expect("Failed to create guest region");
    let memory = Arc::new(
        GuestMemoryMmap::from_regions(vec![guest_region]).expect("Failed to create guest memory"),
    );

    // Load test binary at LOAD_ADDR
    memory
        .write_slice(&code, GuestAddress(LOAD_ADDR))
        .expect("Failed to write test code to memory");

    // Set up initial CPU state for 64-bit long mode
    let mut regs = Registers::default();
    regs.rip = LOAD_ADDR;
    regs.rsp = LOAD_ADDR + 0x10000; // Stack below code
    regs.rflags = 0x2; // Reserved bit must be 1

    let mut sregs = SystemRegisters::default();
    // Set up 64-bit long mode without paging
    sregs.cr0 = 0x21; // Just PE | NE (no paging)
    sregs.cr4 = 0x20; // PAE
    sregs.efer = 0x500; // LMA | LME (long mode active)

    // Set up code segment for 64-bit mode
    sregs.cs.base = 0;
    sregs.cs.limit = 0xFFFFFFFF;
    sregs.cs.selector = 0x8;
    sregs.cs.type_ = 0xB; // Execute/Read, accessed
    sregs.cs.present = true;
    sregs.cs.dpl = 0;
    sregs.cs.db = false;
    sregs.cs.s = true;
    sregs.cs.l = true; // 64-bit code segment
    sregs.cs.g = true;

    // Data segments
    sregs.ds.base = 0;
    sregs.ds.limit = 0xFFFFFFFF;
    sregs.ds.selector = 0x10;
    sregs.ds.type_ = 0x3; // Read/Write, accessed
    sregs.ds.present = true;
    sregs.ds.dpl = 0;
    sregs.ds.db = true;
    sregs.ds.s = true;
    sregs.ds.g = true;

    sregs.es = sregs.ds.clone();
    sregs.fs = sregs.ds.clone();
    sregs.gs = sregs.ds.clone();
    sregs.ss = sregs.ds.clone();

    // Create vCPU and set initial state
    let mut vcpu = X86_64Vcpu::new(0, memory);
    vcpu.set_regs(&regs).expect("Failed to set registers");
    vcpu.set_sregs(&sregs)
        .expect("Failed to set system registers");

    // Run with output capture
    let mut output = String::new();
    let mut insn_count: u64 = 0;
    let max_insns: u64 = 10_000_000;

    loop {
        match vcpu.run() {
            Ok(VcpuExit::Hlt) => {
                println!("\n--- Test Output ---");
                println!("{}", output);
                println!("--- End Output ---");
                println!("\nHalted after {} instructions", insn_count);
                break;
            }
            Ok(VcpuExit::IoOut { port, data }) => {
                if port == 0xE9 {
                    // Debug output port
                    for &b in &data {
                        output.push(b as char);
                    }
                }
            }
            Ok(exit) => {
                println!("\nUnhandled exit: {:?}", exit);
                break;
            }
            Err(e) => {
                println!("\n--- Test Output ---");
                println!("{}", output);
                println!("--- End Output ---");
                println!("\nError: {}", e);
                println!("Instruction count: {}", insn_count);

                // Get CPU state for debugging
                if let Ok(regs) = vcpu.get_regs() {
                    println!("\nCPU State:");
                    println!("  RIP: {:#x}", regs.rip);
                    println!("  RAX: {:#x}", regs.rax);
                    println!("  RBX: {:#x}", regs.rbx);
                    println!("  RCX: {:#x}", regs.rcx);
                    println!("  RDX: {:#x}", regs.rdx);
                    println!("  RSI: {:#x}", regs.rsi);
                    println!("  RDI: {:#x}", regs.rdi);
                    println!("  RSP: {:#x}", regs.rsp);
                    println!("  RBP: {:#x}", regs.rbp);
                }
                break;
            }
        }
        insn_count += 1;
        if insn_count >= max_insns {
            println!("Reached max instruction count ({})", max_insns);
            println!("Output so far:\n{}", output);
            break;
        }
    }

    // Count pass/fail
    let passes = output.matches("OK\n").count();
    let fails = output.matches("FAIL\n").count();
    println!("\n=== Summary ===");
    println!("Passed: {}", passes);
    println!("Failed: {}", fails);
    if fails > 0 {
        std::process::exit(1);
    }
}
