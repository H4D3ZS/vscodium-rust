//! Throughput benchmark for the rax software x86_64 interpreter.
//!
//! Runs a tight register-only guest loop a fixed number of times and reports
//! sustained MIPS (million instructions per second). This is the apples-to-apples
//! metric for hot-path interpreter optimizations.
//!
//! Usage: cargo run --release --example bench_loop [iterations]
//!   iterations defaults to 0x1000_0000 (~1.34B guest instructions).

use std::sync::Arc;
use std::time::Instant;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestRegionMmap, MmapRegion};

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::{Registers, SystemRegisters, VCpu, VcpuExit};

const LOAD_ADDR: u64 = 0x10_0000;
const MEM_SIZE: u64 = 16 * 1024 * 1024;

fn main() {
    let iters: u32 = std::env::args()
        .nth(1)
        .and_then(|s| {
            let s = s.trim_start_matches("0x");
            u32::from_str_radix(s, 16)
                .ok()
                .or_else(|| s.parse::<u32>().ok())
        })
        .unwrap_or(0x1000_0000);

    // Guest program: a tight register-only loop.
    //   xor eax, eax              31 C0
    //   mov ecx, <iters>          B9 ii ii ii ii
    // loop:
    //   add eax, 3                83 C0 03
    //   xor edx, edx              31 D2
    //   sub eax, 1                83 E8 01
    //   dec ecx                   FF C9
    //   jnz loop                  75 F4   (rel8 = -12)
    //   hlt                       F4
    let mut code: Vec<u8> = vec![0x31, 0xC0];
    code.push(0xB9);
    code.extend_from_slice(&iters.to_le_bytes());
    code.extend_from_slice(&[0x83, 0xC0, 0x03]); // add eax,3
    code.extend_from_slice(&[0x31, 0xD2]); // xor edx,edx
    code.extend_from_slice(&[0x83, 0xE8, 0x01]); // sub eax,1
    code.extend_from_slice(&[0xFF, 0xC9]); // dec ecx
    code.extend_from_slice(&[0x75, 0xF4]); // jnz loop
    code.push(0xF4); // hlt

    // 5 instructions per iteration + 2 setup + final hlt.
    let expected_insns: u64 = (iters as u64) * 5 + 3;

    let region = MmapRegion::new(MEM_SIZE as usize).unwrap();
    let guest_region = GuestRegionMmap::new(region, GuestAddress(0)).unwrap();
    let memory = Arc::new(GuestMemoryMmap::from_regions(vec![guest_region]).unwrap());
    memory.write_slice(&code, GuestAddress(LOAD_ADDR)).unwrap();

    let mut regs = Registers::default();
    regs.rip = LOAD_ADDR;
    regs.rsp = 0x11_0000;
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

    // Warm up the decode cache for one iteration's worth of instructions.
    let mut executed: u64 = 0;
    let start = Instant::now();
    loop {
        match vcpu.step() {
            Ok(Some(VcpuExit::Hlt)) => {
                executed += 1;
                break;
            }
            Ok(Some(_other)) => break,
            Ok(None) => {
                executed += 1;
            }
            Err(e) => {
                eprintln!("[bench] error after {executed} insns: {e:?}");
                break;
            }
        }
    }
    let elapsed = start.elapsed();

    let secs = elapsed.as_secs_f64();
    let mips = (executed as f64) / secs / 1.0e6;
    eprintln!("[bench] iterations    : {iters} (0x{iters:x})");
    eprintln!("[bench] expected insns: {expected_insns}");
    eprintln!("[bench] executed insns: {executed}");
    eprintln!("[bench] elapsed       : {secs:.4} s");
    eprintln!("[bench] throughput    : {mips:.2} MIPS");
    let r = vcpu.get_regs().unwrap();
    eprintln!(
        "[bench] final eax={:#x} ecx={:#x}",
        r.rax & 0xffff_ffff,
        r.rcx & 0xffff_ffff
    );
}
