//! Real-mode (16-bit) execution tests — the foundation for legacy/BIOS boot
//! (e.g. TempleOS via El-Torito). Built in increments; each test pins one piece
//! of real-mode behavior the TempleOS boot sector relies on.
#![cfg(target_arch = "x86_64")]

use std::sync::Arc;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestRegionMmap, MmapRegion};

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::{Registers, Segment, SystemRegisters, VCpu, VcpuExit};

const MEM: u64 = 4 * 1024 * 1024;

fn rm_seg(code: bool) -> Segment {
    Segment {
        base: 0,
        limit: 0xFFFF,
        selector: 0,
        type_: if code { 0x0B } else { 0x03 },
        present: true,
        dpl: 0,
        db: false, // 16-bit
        s: true,
        l: false,
        g: false,
        avl: false,
        unusable: false,
    }
}

/// Real-mode system registers: PE=0, no paging, 16-bit flat segments.
fn real_mode_sregs() -> SystemRegisters {
    let mut s = SystemRegisters::default();
    s.cs = rm_seg(true);
    s.ds = rm_seg(false);
    s.es = rm_seg(false);
    s.fs = rm_seg(false);
    s.gs = rm_seg(false);
    s.ss = rm_seg(false);
    s.idt.limit = 0x3FF;
    s
}

/// Build a real-mode vcpu with `code` at linear `load_linear`, CS base `cs_base`
/// (so IP = load_linear - cs_base), a small stack, and DS/ES/SS base 0. Returns
/// the guest memory too so tests can seed/inspect data.
fn rm_vcpu(code: &[u8], load_linear: u64, cs_base: u64) -> (X86_64Vcpu, Arc<GuestMemoryMmap>) {
    let region = MmapRegion::new(MEM as usize).unwrap();
    let gr = GuestRegionMmap::new(region, GuestAddress(0)).unwrap();
    let mem = Arc::new(GuestMemoryMmap::from_regions(vec![gr]).unwrap());
    mem.write_slice(code, GuestAddress(load_linear)).unwrap();

    let mut v = X86_64Vcpu::new(0, mem.clone());
    let mut s = real_mode_sregs();
    s.cs.base = cs_base;
    v.set_sregs(&s).unwrap();
    let mut r = Registers::default();
    r.rip = load_linear - cs_base;
    r.rsp = 0x2000;
    r.rflags = 0x2;
    v.set_regs(&r).unwrap();
    (v, mem)
}

fn run(v: &mut X86_64Vcpu, max: usize) {
    for _ in 0..max {
        match v.step() {
            Ok(Some(VcpuExit::Hlt)) => return,
            Ok(_) => {}
            Err(e) => panic!("step error: {e:?}"),
        }
    }
    panic!("no HLT within {max} steps");
}

// ── Increment 1: segment-register load sets base = selector<<4; fetch uses CS base.

#[test]
fn rm_segment_load_sets_base() {
    // mov ax, 0x9660 ; mov es, ax ; hlt   → real mode: es.base = 0x9660<<4
    let code = [0xB8, 0x60, 0x96, 0x8E, 0xC0, 0xF4];
    let (mut v, _m) = rm_vcpu(&code, 0x7C00, 0);
    run(&mut v, 10);
    assert_eq!(
        v.get_sregs().unwrap().es.base,
        0x9_6600,
        "real-mode segment load must set base = selector<<4"
    );
    assert_eq!(v.get_regs().unwrap().rax & 0xFFFF, 0x9660);
}

#[test]
fn rm_fetch_uses_cs_base() {
    // At linear 0x1100 (CS.base=0x1000, IP=0x100): mov ax,0xBEEF ; hlt
    let code = [0xB8, 0xEF, 0xBE, 0xF4];
    let (mut v, _m) = rm_vcpu(&code, 0x1100, 0x1000);
    run(&mut v, 10);
    assert_eq!(
        v.get_regs().unwrap().rax & 0xFFFF,
        0xBEEF,
        "instruction fetch must use CS.base + IP"
    );
}

// ── Increment 2: ModR/M + moffs data accesses add the segment base (DS default).

#[test]
fn rm_modrm_uses_ds_base() {
    // mov ax,0x200 ; mov ds,ax (ds.base=0x2000)
    // mov byte [dword 0x35],0x42 ; mov al,[dword 0x35] ; hlt
    // Both the write and the read must target DS.base+0x35 = 0x2035.
    let code = [
        0xB8, 0x00, 0x02, // mov ax, 0x200
        0x8E, 0xD8, // mov ds, ax
        0x67, 0xC6, 0x05, 0x35, 0x00, 0x00, 0x00, 0x42, // mov byte [0x35], 0x42
        0x67, 0x8A, 0x05, 0x35, 0x00, 0x00, 0x00, // mov al, [0x35]
        0xF4, // hlt
    ];
    let (mut v, m) = rm_vcpu(&code, 0x7C00, 0);
    run(&mut v, 20);
    assert_eq!(v.get_regs().unwrap().rax & 0xFF, 0x42, "read via DS.base");
    let mut b = [0u8; 1];
    m.read_slice(&mut b, GuestAddress(0x2035)).unwrap();
    assert_eq!(b[0], 0x42, "write must land at DS.base + 0x35 = 0x2035");
}

#[test]
fn rm_moffs_uses_ds_base() {
    // mov ax,0x300 ; mov ds,ax (ds.base=0x3000) ; mov ax,0xCAFE
    // mov [0x40],ax (moffs16 store, opcode A3) ; hlt — writes to DS.base+0x40.
    let code = [
        0xB8, 0x00, 0x03, // mov ax, 0x300
        0x8E, 0xD8, // mov ds, ax
        0xB8, 0xFE, 0xCA, // mov ax, 0xCAFE
        0xA3, 0x40, 0x00, // mov [0x0040], ax  (moffs16)
        0xF4,
    ];
    let (mut v, m) = rm_vcpu(&code, 0x7C00, 0);
    run(&mut v, 20);
    let mut b = [0u8; 2];
    m.read_slice(&mut b, GuestAddress(0x3040)).unwrap();
    assert_eq!(
        u16::from_le_bytes(b),
        0xCAFE,
        "moffs write must use DS.base + 0x40"
    );
}

// ── Increment 3: stack uses SS.base; string ops use DS/ES base; far jmp sets CS.base.

#[test]
fn rm_stack_uses_ss_base() {
    // mov ax,0x500 ; mov ss,ax (SS.base=0x5000) ; mov sp,0x100
    // mov bx,0x1234 ; push bx ; pop cx ; hlt
    let code = [
        0xB8, 0x00, 0x05, // mov ax, 0x500
        0x8E, 0xD0, // mov ss, ax
        0xBC, 0x00, 0x01, // mov sp, 0x100
        0xBB, 0x34, 0x12, // mov bx, 0x1234
        0x53, // push bx
        0x59, // pop cx
        0xF4,
    ];
    let (mut v, m) = rm_vcpu(&code, 0x7C00, 0);
    run(&mut v, 20);
    assert_eq!(
        v.get_regs().unwrap().rcx & 0xFFFF,
        0x1234,
        "pop must use SS.base"
    );
    let mut b = [0u8; 2];
    m.read_slice(&mut b, GuestAddress(0x50FE)).unwrap(); // ss.base + (sp-2)
    assert_eq!(
        u16::from_le_bytes(b),
        0x1234,
        "push must write to SS.base+SP"
    );
}

#[test]
fn rm_movs_uses_segment_bases() {
    // mov ds=0x600 (0x6000), es=0x700 (0x7000) ; si=di=0 ; cx=4 ; rep movsb
    let code = [
        0xB8, 0x00, 0x06, 0x8E, 0xD8, // mov ax,0x600 ; mov ds,ax
        0xB8, 0x00, 0x07, 0x8E, 0xC0, // mov ax,0x700 ; mov es,ax
        0x31, 0xF6, // xor si,si
        0x31, 0xFF, // xor di,di
        0xB9, 0x04, 0x00, // mov cx,4
        0xF3, 0xA4, // rep movsb
        0xF4,
    ];
    let (mut v, m) = rm_vcpu(&code, 0x7C00, 0);
    m.write_slice(&[0xDE, 0xAD, 0xBE, 0xEF], GuestAddress(0x6000))
        .unwrap();
    run(&mut v, 40);
    let mut b = [0u8; 4];
    m.read_slice(&mut b, GuestAddress(0x7000)).unwrap();
    assert_eq!(
        b,
        [0xDE, 0xAD, 0xBE, 0xEF],
        "rep movsb: DS.base+SI -> ES.base+DI"
    );
}

#[test]
fn rm_far_jmp_sets_cs_base() {
    // at 0x7C00: jmp 0x0900:0x0010 ; target at linear 0x9010: mov ax,0xF00D ; hlt
    let code = [0xEA, 0x10, 0x00, 0x00, 0x09]; // jmp ptr16:16 = 0x0900:0x0010
    let (mut v, m) = rm_vcpu(&code, 0x7C00, 0);
    m.write_slice(&[0xB8, 0x0D, 0xF0, 0xF4], GuestAddress(0x9010))
        .unwrap();
    run(&mut v, 10);
    assert_eq!(
        v.get_sregs().unwrap().cs.base,
        0x9000,
        "far jmp sets CS.base = sel<<4"
    );
    assert_eq!(
        v.get_regs().unwrap().rax & 0xFFFF,
        0xF00D,
        "fetch+exec at CS.base+IP"
    );
}

// ── Increment 4: the actual TempleOS boot prologue end-to-end (relocate + far jmp).

#[test]
fn rm_templeos_prologue_relocates_and_far_jumps() {
    // The verbatim TempleOS El-Torito boot prologue at 0x7C00: it sets ES=SS=0x9660,
    // copies its own 2KB to 0x96600, then far-jumps to 0x9660:0x00B0. We plant a
    // marker (mov ax,0xD00D ; hlt) at image offset 0xB0 so that, once relocated and
    // jumped to, it runs at linear 0x966B0.
    #[rustfmt::skip]
    let prologue: [u8; 0x35] = [
        0xFC,                   // cld
        0xB8, 0x60, 0x96,       // mov ax, 0x9660
        0x66, 0x8E, 0xC0,       // mov es, eax
        0xFA,                   // cli
        0x66, 0x8E, 0xD0,       // mov ss, eax
        0xBC, 0x00, 0x0A,       // mov sp, 0x0A00
        0xFB,                   // sti
        0xE8, 0x00, 0x00,       // call .next  (push IP=0x7C12)
        0x5B,                   // pop bx
        0x83, 0xEB, 0x12,       // sub bx, 0x12
        0xC1, 0xEB, 0x04,       // shr bx, 4
        0x66, 0x8C, 0xC8,       // mov eax, cs
        0x03, 0xC3,             // add ax, bx
        0x66, 0x8E, 0xD8,       // mov ds, eax
        0xB9, 0x00, 0x08,       // mov cx, 0x0800  (2048 bytes)
        0x33, 0xF6,             // xor si, si
        0x33, 0xFF,             // xor di, di
        0xF3, 0xA4,             // rep movsb       (relocate to ES:0 = 0x96600)
        0xB8, 0x60, 0x96,       // mov ax, 0x9660
        0x66, 0x8E, 0xD8,       // mov ds, eax
        0xEA, 0xB0, 0x00, 0x60, 0x96, // jmp 0x9660:0x00B0
    ];

    let mut image = vec![0u8; 2048];
    image[..prologue.len()].copy_from_slice(&prologue);
    // Marker at offset 0xB0: mov ax,0xD00D ; hlt
    image[0xB0..0xB4].copy_from_slice(&[0xB8, 0x0D, 0xD0, 0xF4]);

    let (mut v, m) = rm_vcpu(&image, 0x7C00, 0);
    run(&mut v, 5000);

    // Far-jumped into the relocated copy at 0x9660:0x00B0.
    let s = v.get_sregs().unwrap();
    assert_eq!(
        s.cs.base, 0x9_6600,
        "running in the relocated segment 0x9660"
    );
    assert_eq!(
        v.get_regs().unwrap().rax & 0xFFFF,
        0xD00D,
        "executed the relocated marker"
    );
    // The boot image was copied verbatim to 0x96600.
    let mut head = [0u8; 4];
    m.read_slice(&mut head, GuestAddress(0x9_6600)).unwrap();
    assert_eq!(&head, &prologue[..4], "2KB relocate landed at 0x96600");
}

// ── Increment 5: real-mode mini-BIOS — INT 13h AH=42h extended CD read.

#[test]
fn rm_bios_int13_extended_read() {
    use rax::backend::emulator::x86_64::bios;

    // Boot CD image: LBA 2 holds a recognizable ramp pattern.
    let mut cd = vec![0u8; 4 * 2048];
    for (i, b) in cd[2 * 2048..3 * 2048].iter_mut().enumerate() {
        *b = (i & 0xFF) as u8;
    }
    bios::install_cd(std::sync::Arc::new(cd));

    // mov ah,0x42 ; mov si,0x600 ; mov dl,0xE0 ; int 0x13 ; hlt
    let code = [
        0xB4, 0x42, // mov ah, 0x42
        0xBE, 0x00, 0x06, // mov si, 0x600  (DAP at DS:0x600 = linear 0x600)
        0xB2, 0xE0, // mov dl, 0xE0
        0xCD, 0x13, // int 0x13
        0xF4, // hlt
    ];
    let (mut v, m) = rm_vcpu(&code, 0x7C00, 0);
    // Disk Address Packet: size=0x10, count=1, buffer=0000:2000, LBA=2.
    let dap = [
        0x10, 0x00, // size
        0x01, 0x00, // sector count = 1
        0x00, 0x20, // buffer offset = 0x2000
        0x00, 0x00, // buffer segment = 0
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LBA = 2
    ];
    m.write_slice(&dap, GuestAddress(0x600)).unwrap();
    run(&mut v, 30);

    // CF clear (success), and the CD sector landed at the buffer 0x2000.
    assert_eq!(
        v.get_regs().unwrap().rflags & 1,
        0,
        "INT 13h AH=42h must clear CF"
    );
    let mut buf = [0u8; 2048];
    m.read_slice(&mut buf, GuestAddress(0x2000)).unwrap();
    for (i, b) in buf.iter().enumerate() {
        assert_eq!(*b, (i & 0xFF) as u8, "byte {i} of the read CD sector");
    }
}
