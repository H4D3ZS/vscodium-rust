//! Memory-access throughput benchmark for the rax software interpreter.
//!
//! Runs a tight guest loop doing a 64-bit load + store every iteration, so the
//! cost is dominated by the MMU physical-access path (read_phys/write_phys).
//! Complements bench_loop.rs (register-only). Reports MIPS + MB/s of guest
//! memory traffic.
//!
//! Usage: cargo run --release --example bench_mem [iterations]

use std::sync::Arc;
use std::time::Instant;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestRegionMmap, MmapRegion};

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::{Registers, SystemRegisters, VCpu, VcpuExit};

const LOAD_ADDR: u64 = 0x10_0000;
const BUF_ADDR: u64 = 0x20_0000;
const MEM_SIZE: u64 = 16 * 1024 * 1024;

fn main() {
    let iters: u32 = std::env::args()
        .nth(1)
        .and_then(|s| {
            let s = s.trim_start_matches("0x");
            u32::from_str_radix(s, 16).ok().or_else(|| s.parse().ok())
        })
        .unwrap_or(0x1000_0000);

    // Guest program:
    //   mov rsi, BUF_ADDR        48 BE <imm64>
    //   mov ecx, <iters>         B9 <imm32>
    // loop:
    //   mov rax, [rsi]           48 8B 06
    //   add rax, rcx             48 01 C8
    //   mov [rsi], rax           48 89 06
    //   dec rcx                  48 FF C9
    //   jnz loop                 75 F2   (rel8 = -14)
    //   hlt                      F4
    let mut code: Vec<u8> = vec![0x48, 0xBE];
    code.extend_from_slice(&BUF_ADDR.to_le_bytes());
    code.push(0xB9);
    code.extend_from_slice(&iters.to_le_bytes());
    code.extend_from_slice(&[0x48, 0x8B, 0x06]); // mov rax,[rsi]
    code.extend_from_slice(&[0x48, 0x01, 0xC8]); // add rax,rcx
    code.extend_from_slice(&[0x48, 0x89, 0x06]); // mov [rsi],rax
    code.extend_from_slice(&[0x48, 0xFF, 0xC9]); // dec rcx
    code.extend_from_slice(&[0x75, 0xF2]); // jnz loop
    code.push(0xF4); // hlt

    let region = MmapRegion::new(MEM_SIZE as usize).unwrap();
    let guest_region = GuestRegionMmap::new(region, GuestAddress(0)).unwrap();
    let memory = Arc::new(GuestMemoryMmap::from_regions(vec![guest_region]).unwrap());
    memory.write_slice(&code, GuestAddress(LOAD_ADDR)).unwrap();

    let mut regs = Registers::default();
    regs.rip = LOAD_ADDR;
    regs.rsp = 0x18_0000;
    regs.rflags = 0x2;

    let mut sregs = SystemRegisters::default();
    sregs.cr0 = 0x21;
    sregs.cr4 = 0x20;
    sregs.efer = 0x500;
    sregs.cs.base = 0;
    sregs.cs.limit = 0xFFFFFFFF;
    sregs.cs.selector = 0x8;
    sregs.cs.type_ = 0xB;
    sregs.cs.present = true;
    sregs.cs.s = true;
    sregs.cs.l = true;
    sregs.cs.g = true;
    sregs.ds.base = 0;
    sregs.ds.limit = 0xFFFFFFFF;
    sregs.ds.selector = 0x10;
    sregs.ds.type_ = 0x3;
    sregs.ds.present = true;
    sregs.ds.db = true;
    sregs.ds.s = true;
    sregs.ds.g = true;
    sregs.es = sregs.ds.clone();
    sregs.fs = sregs.ds.clone();
    sregs.gs = sregs.ds.clone();
    sregs.ss = sregs.ds.clone();

    let mut vcpu = X86_64Vcpu::new(0, memory);
    vcpu.set_regs(&regs).unwrap();
    vcpu.set_sregs(&sregs).unwrap();

    let mut executed: u64 = 0;
    let start = Instant::now();
    loop {
        match vcpu.step() {
            Ok(Some(VcpuExit::Hlt)) => {
                executed += 1;
                break;
            }
            Ok(Some(_)) => break,
            Ok(None) => executed += 1,
            Err(e) => {
                eprintln!("[bench] error after {executed} insns: {e:?}");
                break;
            }
        }
    }
    let secs = start.elapsed().as_secs_f64();
    let mips = (executed as f64) / secs / 1.0e6;
    // 16 bytes of guest memory traffic (one 8B load + one 8B store) per iteration.
    let mbps = (iters as f64) * 16.0 / secs / (1024.0 * 1024.0);
    eprintln!("[bench_mem] iterations : {iters} (0x{iters:x})");
    eprintln!("[bench_mem] executed   : {executed}");
    eprintln!("[bench_mem] elapsed    : {secs:.4} s");
    eprintln!("[bench_mem] throughput : {mips:.2} MIPS");
    eprintln!("[bench_mem] mem traffic: {mbps:.1} MB/s");
}
