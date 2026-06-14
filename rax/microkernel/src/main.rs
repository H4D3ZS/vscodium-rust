//! Bare-metal microkernel example for rax emulator.
//!
//! Demonstrates:
//! - Memory allocation (bump allocator)
//! - Real-world physics simulation (n-body gravitational)
//! - Serial output via port 0xE9 (bare-metal) or syscall (usermode)
//! - Complex instruction coverage (arithmetic, SIMD, branching)
//!
//! Build modes:
//! - Bare-metal (default): cargo build --release
//! - Usermode (for Intel SDE): cargo build --release --features usermode --target x86_64-unknown-linux-gnu

#![cfg_attr(not(feature = "usermode"), no_std)]
#![cfg_attr(not(feature = "usermode"), no_main)]

use core::arch::asm;
#[cfg(not(feature = "usermode"))]
use core::arch::naked_asm;
use core::fmt;
#[cfg(not(feature = "usermode"))]
use core::panic::PanicInfo;
#[cfg(not(feature = "usermode"))]
use core::ptr::{self, addr_of_mut};

// =============================================================================
// Entry Point (bare-metal)
// =============================================================================

#[cfg(not(feature = "usermode"))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
extern "C" fn _start() -> ! {
    naked_asm!(
        // Set up stack
        "lea rsp, [rip + __stack_top]",
        // Clear BSS
        "lea rdi, [rip + __bss_start]",
        "lea rcx, [rip + __bss_end]",
        "sub rcx, rdi",
        "shr rcx, 3",
        "xor eax, eax",
        "rep stosq",
        // Call main
        "call {main}",
        // Shutdown via ACPI power-off (port 0x604)
        "mov dx, 0x604",
        "mov ax, 0x2000",
        "out dx, ax",
        // Fallback: halt
        "hlt",
        main = sym kernel_main,
    )
}

#[cfg(not(feature = "usermode"))]
unsafe extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
    static __stack_top: u8;
    static __heap_start: u8;
    static __heap_end: u8;
}

// =============================================================================
// Entry Point (usermode for Intel SDE)
// =============================================================================

#[cfg(feature = "usermode")]
fn main() {
    kernel_main();
    // Exit cleanly via syscall
    unsafe {
        asm!(
            "mov rax, 60",  // exit syscall
            "xor rdi, rdi", // exit code 0
            "syscall",
            options(noreturn)
        );
    }
}

// =============================================================================
// Serial Output
// =============================================================================

struct Serial;

impl Serial {
    #[cfg(not(feature = "usermode"))]
    fn write_byte(&self, b: u8) {
        // Bare-metal: use port 0xE9 (QEMU/Bochs debug port)
        unsafe {
            asm!(
                "out dx, al",
                in("dx") 0xE9u16,
                in("al") b,
                options(nostack, preserves_flags)
            );
        }
    }

    #[cfg(feature = "usermode")]
    fn write_byte(&self, b: u8) {
        // Usermode: use write() syscall to stdout
        let buf: [u8; 1] = [b];
        unsafe {
            asm!(
                "syscall",
                in("rax") 1u64,     // write syscall
                in("rdi") 1u64,     // fd = stdout
                in("rsi") buf.as_ptr(),
                in("rdx") 1u64,     // count = 1
                lateout("rax") _,
                lateout("rcx") _,
                lateout("r11") _,
            );
        }
    }

    /// Print a number manually without using fmt
    fn write_u64(&self, n: u64) {
        // Simple digit-by-digit output to avoid stack buffer issues
        if n == 0 {
            self.write_byte(b'0');
            return;
        }

        // Find the highest power of 10
        let mut divisor: u64 = 1;
        let mut temp = n;
        while temp >= 10 {
            divisor *= 10;
            temp /= 10;
        }

        // Output digits from most significant to least
        let mut remaining = n;
        loop {
            let digit = (remaining / divisor) as u8;
            self.write_byte(b'0' + digit);
            remaining %= divisor;
            if divisor == 1 {
                break;
            }
            divisor /= 10;
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }
        Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*) => {{
        let _ = core::fmt::write(&mut Serial, format_args!($($arg)*));
    }};
}

macro_rules! println {
    () => { print!("\n") };
    ($($arg:tt)*) => {{
        print!($($arg)*);
        print!("\n");
    }};
}

// =============================================================================
// Bump Allocator
// =============================================================================

#[cfg(feature = "usermode")]
const HEAP_SIZE: usize = 64 * 1024; // 64KB heap for usermode

#[cfg(feature = "usermode")]
static mut HEAP_BUFFER: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

struct BumpAllocator {
    next: *mut u8,
    end: *mut u8,
    allocated: usize,
}

impl BumpAllocator {
    const fn new() -> Self {
        Self {
            next: core::ptr::null_mut(),
            end: core::ptr::null_mut(),
            allocated: 0,
        }
    }

    #[cfg(not(feature = "usermode"))]
    unsafe fn init(&mut self) {
        unsafe {
            self.next = &__heap_start as *const u8 as *mut u8;
            self.end = &__heap_end as *const u8 as *mut u8;
        }
        self.allocated = 0;
    }

    #[cfg(feature = "usermode")]
    unsafe fn init(&mut self) {
        unsafe {
            let heap_ptr = core::ptr::addr_of_mut!(HEAP_BUFFER) as *mut u8;
            self.next = heap_ptr;
            self.end = heap_ptr.add(HEAP_SIZE);
        }
        self.allocated = 0;
    }

    fn alloc<T>(&mut self, count: usize) -> Option<*mut T> {
        let size = core::mem::size_of::<T>() * count;
        let align = core::mem::align_of::<T>();

        let aligned = (self.next as usize + align - 1) & !(align - 1);
        let new_next = aligned + size;

        if new_next > self.end as usize {
            return None;
        }

        let ptr = aligned as *mut T;
        self.next = new_next as *mut u8;
        self.allocated += size;
        Some(ptr)
    }

    fn allocated_bytes(&self) -> usize {
        self.allocated
    }
}

static mut ALLOCATOR: BumpAllocator = BumpAllocator::new();

/// Helper to access the allocator safely
#[inline(always)]
fn allocator() -> &'static mut BumpAllocator {
    unsafe { &mut *core::ptr::addr_of_mut!(ALLOCATOR) }
}

// =============================================================================
// Fixed-Point Math
// =============================================================================

#[derive(Clone, Copy, Default)]
struct Fixed(i64);

impl Fixed {
    const FRAC_BITS: u32 = 16;
    const SCALE: i64 = 1 << Self::FRAC_BITS;

    const fn from_int(n: i64) -> Self {
        Self(n << Self::FRAC_BITS)
    }

    const fn zero() -> Self {
        Self(0)
    }

    fn to_int(self) -> i64 {
        self.0 >> Self::FRAC_BITS
    }

    fn sqrt(self) -> Self {
        if self.0 <= 0 {
            return Self::zero();
        }
        let mut x = self;
        for _ in 0..16 {
            let x2 = x * x;
            let diff = self - x2;
            let two_x = Self(x.0 << 1);
            if two_x.0 != 0 {
                x = Self(x.0 + diff.0 / (two_x.0 >> Self::FRAC_BITS));
            }
        }
        x
    }
}

impl core::ops::Add for Fixed {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }
}

impl core::ops::Sub for Fixed {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }
}

impl core::ops::Mul for Fixed {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let result = (self.0 as i128 * rhs.0 as i128) >> Self::FRAC_BITS;
        Self(result as i64)
    }
}

impl core::ops::Div for Fixed {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            return Self(i64::MAX);
        }
        let result = ((self.0 as i128) << Self::FRAC_BITS) / rhs.0 as i128;
        Self(result as i64)
    }
}

impl core::ops::AddAssign for Fixed {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::SubAssign for Fixed {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// =============================================================================
// Vec3 and Body for simulation
// =============================================================================

#[derive(Clone, Copy, Default)]
struct Vec3 {
    x: Fixed,
    y: Fixed,
    z: Fixed,
}

impl Vec3 {
    const fn new(x: Fixed, y: Fixed, z: Fixed) -> Self {
        Self { x, y, z }
    }

    fn magnitude_squared(self) -> Fixed {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn magnitude(self) -> Fixed {
        self.magnitude_squared().sqrt()
    }
}

impl core::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl core::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl core::ops::Mul<Fixed> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: Fixed) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl core::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Clone, Copy)]
struct Body {
    pos: Vec3,
    vel: Vec3,
    mass: Fixed,
}

struct NBodySimulation {
    bodies: *mut Body,
    count: usize,
}

impl NBodySimulation {
    fn new(bodies: *mut Body, count: usize) -> Self {
        Self { bodies, count }
    }

    fn step(&mut self, dt: Fixed) {
        let g = Fixed::from_int(1);
        let softening = Fixed::from_int(1);

        for i in 0..self.count {
            let mut acc = Vec3::default();

            for j in 0..self.count {
                if i == j {
                    continue;
                }

                let bi = unsafe { *self.bodies.add(i) };
                let bj = unsafe { *self.bodies.add(j) };

                let r = bj.pos - bi.pos;
                let dist_sq = r.magnitude_squared() + softening;
                let dist = dist_sq.sqrt();
                let dist_cubed = dist_sq * dist;

                if dist_cubed.0 != 0 {
                    let force_mag = g * bj.mass / dist_cubed;
                    acc += r * force_mag;
                }
            }

            unsafe {
                (*self.bodies.add(i)).vel += acc * dt;
            }
        }

        for i in 0..self.count {
            unsafe {
                let body = &mut *self.bodies.add(i);
                body.pos += body.vel * dt;
            }
        }
    }

    fn total_energy(&self) -> Fixed {
        let mut kinetic = Fixed::zero();
        let mut potential = Fixed::zero();

        for i in 0..self.count {
            let bi = unsafe { *self.bodies.add(i) };
            let v_sq = bi.vel.magnitude_squared();
            kinetic += bi.mass * v_sq / Fixed::from_int(2);

            for j in (i + 1)..self.count {
                let bj = unsafe { *self.bodies.add(j) };
                let r = (bj.pos - bi.pos).magnitude();
                if r.0 != 0 {
                    potential -= bi.mass * bj.mass / r;
                }
            }
        }

        kinetic + potential
    }
}

// =============================================================================
// Instruction Coverage Tests
// =============================================================================

#[repr(align(16))]
struct Aligned16<T>(T);

#[repr(align(32))]
struct Aligned32<T>(T);

#[repr(align(64))]
struct Aligned64<T>(T);

fn sum_i32(values: &[i32]) -> u64 {
    let mut sum = 0u64;
    for value in values {
        sum = sum.wrapping_add(*value as i64 as u64);
    }
    sum
}

fn sum_f32_bits(values: &[f32]) -> u64 {
    let mut sum = 0u64;
    for value in values {
        sum = sum.wrapping_add(value.to_bits() as u64);
    }
    sum
}

fn test_arithmetic() -> u64 {
    let mut a: u64 = 12345;
    let mut b: u64 = 6789;
    let mut result: u64 = 0;

    unsafe {
        asm!(
            "add {0}, {1}",
            "sub {0}, 1000",
            "imul {0}, {0}, 3",
            "xchg {0}, {1}",
            "shl {0}, 4",
            "shr {0}, 2",
            "sar {1}, 1",
            "rol {0}, 3",
            "ror {1}, 5",
            "bsf {2}, {0}",
            "bsr {3}, {1}",
            "cmp {0}, {1}",
            "cmovg {2}, {3}",
            inout(reg) a,
            inout(reg) b,
            out(reg) result,
            out(reg) _,
            options(nostack)
        );
    }

    result
}

fn test_string_ops() -> u16 {
    let src: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut dst: [u8; 16] = [0; 16];

    unsafe {
        asm!(
            "cld",
            "rep movsb",
            in("rsi") src.as_ptr(),
            in("rdi") dst.as_mut_ptr(),
            in("rcx") 16usize,
            options(nostack)
        );
    }

    dst.iter().map(|&x| x as u16).sum()
}

fn test_simd() -> [i32; 4] {
    let a: [i32; 4] = [10, 20, 30, 40];
    let b: [i32; 4] = [1, 2, 3, 4];
    let mut result: [i32; 4] = [0; 4];

    unsafe {
        asm!(
            "movdqu xmm0, [{a}]",
            "movdqu xmm1, [{b}]",
            "paddd xmm0, xmm1",
            "pmulld xmm0, xmm1",
            "movdqu [{r}], xmm0",
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),
            r = in(reg) result.as_mut_ptr(),
            options(nostack)
        );
    }

    result
}

fn test_sse_extensions() -> u64 {
    let a = Aligned16([1i32, -2, 3, -4]);
    let b = Aligned16([5i32, 6, -7, 8]);
    let mut result = Aligned16([0i32; 4]);

    unsafe {
        asm!(
            "movdqa xmm0, [{a}]",
            "movdqa xmm1, [{b}]",
            "pabsd xmm0, xmm0",
            "paddd xmm0, xmm1",
            "pshufd xmm0, xmm0, 0x1B",
            "pmulld xmm0, xmm1",
            "movdqa [{r}], xmm0",
            a = in(reg) &a.0,
            b = in(reg) &b.0,
            r = in(reg) &mut result.0,
            options(nostack)
        );
    }

    sum_i32(&result.0)
}

#[target_feature(enable = "avx")]
unsafe fn test_avx128() -> u64 {
    let a = Aligned16([1.0f32, 2.0, 3.0, 4.0]);
    let b = Aligned16([5.0f32, 6.0, 7.0, 8.0]);
    let mut result = Aligned16([0f32; 4]);

    unsafe {
        asm!(
            "vmovaps xmm0, [{a}]",
            "vmovaps xmm1, [{b}]",
            "vaddps xmm0, xmm0, xmm1",
            "vmaxps xmm0, xmm0, xmm1",
            "vmulps xmm0, xmm0, xmm1",
            "vmovaps [{r}], xmm0",
            a = in(reg) &a.0,
            b = in(reg) &b.0,
            r = in(reg) &mut result.0,
            options(nostack)
        );
    }

    sum_f32_bits(&result.0)
}

#[target_feature(enable = "avx,avx2")]
unsafe fn test_avx256() -> u64 {
    let a = Aligned32([1i32, 2, 3, 4, 5, 6, 7, 8]);
    let b = Aligned32([2i32, 3, 4, 5, 6, 7, 8, 9]);
    let mut result = Aligned32([0i32; 8]);

    unsafe {
        asm!(
            "vmovdqa ymm0, [{a}]",
            "vmovdqa ymm1, [{b}]",
            "vpaddd ymm0, ymm0, ymm1",
            "vpmulld ymm0, ymm0, ymm1",
            "vpslld ymm0, ymm0, 1",
            "vmovdqa [{r}], ymm0",
            a = in(reg) &a.0,
            b = in(reg) &b.0,
            r = in(reg) &mut result.0,
            options(nostack)
        );
    }

    sum_i32(&result.0)
}

#[target_feature(enable = "avx512f")]
unsafe fn test_avx512() -> u64 {
    let a = Aligned64([
        1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ]);
    let b = Aligned64([
        16.0f32, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
    ]);
    let mut result = Aligned64([0f32; 16]);

    unsafe {
        asm!(
            "vmovaps zmm0, [{a}]",
            "vmovaps zmm1, [{b}]",
            "vaddps zmm0, zmm0, zmm1",
            "vsubps zmm0, zmm0, zmm1",
            "vmulps zmm0, zmm0, zmm1",
            "vmovaps [{r}], zmm0",
            a = in(reg) &a.0,
            b = in(reg) &b.0,
            r = in(reg) &mut result.0,
            options(nostack)
        );
    }

    sum_f32_bits(&result.0)
}

#[derive(Clone, Copy)]
struct CpuidRegs {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

fn cpuid(leaf: u32, subleaf: u32) -> CpuidRegs {
    let mut eax = leaf;
    let mut ecx = subleaf;
    let ebx: u32;
    let edx: u32;
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov {ebx_out:e}, ebx",
            "pop rbx",
            inout("eax") eax,
            inout("ecx") ecx,
            lateout("edx") edx,
            ebx_out = lateout(reg) ebx,
        );
    }
    CpuidRegs { eax, ebx, ecx, edx }
}

fn apx_cpuid_supported() -> bool {
    let max = cpuid(0, 0).eax;
    if max < 0x29 {
        return false;
    }

    let leaf1 = cpuid(1, 0);
    if leaf1.ecx & (1 << 26) == 0 {
        return false;
    }

    let leaf7_0 = cpuid(7, 0);
    if leaf7_0.eax < 1 {
        return false;
    }
    let leaf7_1 = cpuid(7, 1);
    if leaf7_1.edx & (1 << 21) == 0 {
        return false;
    }

    let xsave = cpuid(0xD, 0);
    if xsave.eax & (1 << 19) == 0 {
        return false;
    }

    let apx = cpuid(0x29, 0);
    apx.eax == 0 && apx.ebx & 1 != 0
}

#[cfg(not(feature = "usermode"))]
unsafe fn enable_apx_xcr0() -> bool {
    unsafe {
        asm!(
            "mov rax, cr4",
            "or rax, 0x40000",
            "mov cr4, rax",
            "xor ecx, ecx",
            "xgetbv",
            "or eax, 0x80007",
            "xor edx, edx",
            "xsetbv",
            out("rax") _,
            out("rcx") _,
            out("rdx") _,
            options(nostack)
        );
    }

    let leaf1 = cpuid(1, 0);
    leaf1.ecx & (1 << 27) != 0
}

#[cfg(feature = "usermode")]
unsafe fn enable_apx_xcr0() -> bool {
    false
}

unsafe fn test_apx_surface() -> u64 {
    let checksum: u64;
    unsafe {
        asm!(
            "push rbx",
            "mov rax, 0xf0f0f0f00f0f0f0f",
            "mov rbx, 0x0102030405060708",
            "mov rcx, 4",
            "mov rdx, 0",
            "mov r8, 0x1111222233334444",
            "mov r9, 0x5555666677778888",
            "mov r10, 0x9999aaaabbbbcccc",
            "mov r11, 0",
            ".byte 0xd5, 0x00, 0xa1",
            ".quad 2f",
            "mov r11, 0xbad",
            "2:",
            ".byte 0x62, 0xec, 0x74, 0x18, 0xff, 0xf0",
            ".byte 0x62, 0xec, 0x74, 0x18, 0x8f, 0xc0",
            ".byte 0x62, 0xec, 0xe4, 0x18, 0x01, 0xc8",
            ".byte 0x62, 0xec, 0xcc, 0x18, 0x31, 0xd0",
            ".byte 0x62, 0xec, 0xe4, 0x0c, 0xc1, 0xe8, 0x04",
            ".byte 0x62, 0xf4, 0x64, 0x1a, 0x01, 0xd9",
            ".byte 0x62, 0xf4, 0xbc, 0x1c, 0x24, 0xd8, 0x04",
            ".byte 0x62, 0x74, 0xfc, 0x0c, 0x88, 0xc0",
            ".byte 0x62, 0x74, 0xfc, 0x0c, 0xf5, 0xc0",
            ".byte 0x62, 0x74, 0xfc, 0x0c, 0xf4, 0xc0",
            ".byte 0x62, 0xd4, 0xfc, 0x08, 0x61, 0xc0",
            ".byte 0x62, 0xf4, 0xbc, 0x18, 0xaf, 0xc3",
            ".byte 0x62, 0xf4, 0xfc, 0x0c, 0xf7, 0xe3",
            ".byte 0x62, 0xf4, 0xe4, 0x0f, 0x83, 0xfb, 0x14",
            "stc",
            ".byte 0x62, 0xf4, 0xe4, 0x42, 0x85, 0xd8",
            ".byte 0x62, 0xf4, 0xe4, 0x44, 0x40, 0xc0",
            "xor rax, rax",
            "add rax, r8",
            "xor rax, r9",
            "add rax, r10",
            "xor rax, r11",
            "pop rbx",
            out("rax") checksum,
            out("rcx") _,
            out("rdx") _,
            out("r8") _,
            out("r9") _,
            out("r10") _,
            out("r11") _,
        );
    }
    checksum
}

// =============================================================================
// Kernel Main
// =============================================================================

fn kernel_main() {
    // Simple banner using direct byte output (no fmt)
    let serial = Serial;
    for c in b"=====================================\n" {
        serial.write_byte(*c);
    }
    for c in b"   RAX Microkernel Example v0.1.0\n" {
        serial.write_byte(*c);
    }
    for c in b"=====================================\n\n" {
        serial.write_byte(*c);
    }

    // Test number printing with our manual method
    for c in b"[TEST] Manual number print: " {
        serial.write_byte(*c);
    }
    serial.write_u64(12345);
    serial.write_byte(b'\n');

    // Now test fmt-based number printing
    for c in b"[TEST] fmt-based print: " {
        serial.write_byte(*c);
    }
    // This uses Rust's fmt machinery:
    println!("{}", 12345u64);

    // Initialize allocator
    unsafe {
        allocator().init();
    }

    for c in b"[INIT] Heap allocator initialized\n" {
        serial.write_byte(*c);
    }

    // Allocate bodies
    let body_count = 8usize;
    let bodies: *mut Body = allocator().alloc::<Body>(body_count).unwrap();

    for c in b"[ALLOC] Bodies allocated: " {
        serial.write_byte(*c);
    }
    serial.write_u64(body_count as u64);
    serial.write_byte(b'\n');

    // Initialize bodies
    for i in 0..body_count {
        let x_sign = if i < 4 { 1 } else { -1 };
        let y_sign = if i % 4 < 2 { 1 } else { -1 };
        let x = Fixed::from_int(((i % 4) as i64 + 1) * 25 * x_sign);
        let y = Fixed::from_int(((i % 4) as i64 + 1) * 25 * y_sign);

        unsafe {
            *bodies.add(i) = Body {
                pos: Vec3::new(x, y, Fixed::zero()),
                vel: Vec3::new(
                    Fixed::from_int(-y_sign),
                    Fixed::from_int(x_sign),
                    Fixed::zero(),
                ),
                mass: Fixed::from_int(10),
            };
        }
    }

    for c in b"[INIT] Bodies initialized\n\n" {
        serial.write_byte(*c);
    }

    // Run simulation
    for c in b"[SIM] Starting simulation...\n" {
        serial.write_byte(*c);
    }

    let mut sim = NBodySimulation::new(bodies, body_count);
    let dt = Fixed(Fixed::SCALE / 100);
    let steps = 50;

    for step in 0..steps {
        sim.step(dt);
        if step % 10 == 0 {
            let b0 = unsafe { *bodies };
            for c in b"[SIM] Step " {
                serial.write_byte(*c);
            }
            serial.write_u64(step as u64);
            for c in b": pos=(" {
                serial.write_byte(*c);
            }
            serial.write_u64(b0.pos.x.to_int().unsigned_abs());
            for c in b", " {
                serial.write_byte(*c);
            }
            serial.write_u64(b0.pos.y.to_int().unsigned_abs());
            for c in b")\n" {
                serial.write_byte(*c);
            }
        }
    }

    for c in b"[SIM] Complete!\n\n" {
        serial.write_byte(*c);
    }

    // Run instruction tests
    for c in b"=== Instruction Tests ===\n" {
        serial.write_byte(*c);
    }

    for c in b"[TEST] Arithmetic result: " {
        serial.write_byte(*c);
    }
    serial.write_u64(test_arithmetic());
    serial.write_byte(b'\n');

    for c in b"[TEST] String ops sum: " {
        serial.write_byte(*c);
    }
    serial.write_u64(test_string_ops() as u64);
    for c in b" (expected 136)\n" {
        serial.write_byte(*c);
    }

    let simd_result = test_simd();
    for c in b"[TEST] SIMD result: [" {
        serial.write_byte(*c);
    }
    for (i, v) in simd_result.iter().enumerate() {
        serial.write_u64(*v as u64);
        if i < 3 {
            for c in b", " {
                serial.write_byte(*c);
            }
        }
    }
    for c in b"]\n" {
        serial.write_byte(*c);
    }

    let sse_ext_sum = test_sse_extensions();
    println!("[TEST] SSE ext sum: {}", sse_ext_sum);

    let avx128_sum = unsafe { test_avx128() };
    println!("[TEST] AVX-128 sum: {}", avx128_sum);

    let avx256_sum = unsafe { test_avx256() };
    println!("[TEST] AVX2-256 sum: {}", avx256_sum);

    let avx512_sum = unsafe { test_avx512() };
    println!("[TEST] AVX-512 sum: {}", avx512_sum);

    if apx_cpuid_supported() {
        let enabled = unsafe { enable_apx_xcr0() };
        if enabled {
            let apx_sum = unsafe { test_apx_surface() };
            println!("[TEST] APX surface checksum: {}", apx_sum);
        } else {
            println!("[TEST] APX surface skipped: XCR0 enable unavailable");
        }
    } else {
        println!("[TEST] APX surface skipped: CPUID gate closed");
    }

    // Final stats
    for c in b"\n=== Final Statistics ===\n" {
        serial.write_byte(*c);
    }
    for c in b"[STAT] Heap used: " {
        serial.write_byte(*c);
    }
    let allocated = allocator().allocated_bytes();
    serial.write_u64(allocated as u64);
    for c in b" bytes\n" {
        serial.write_byte(*c);
    }

    for c in b"\n=====================================\n" {
        serial.write_byte(*c);
    }
    for c in b"   Microkernel execution complete!\n" {
        serial.write_byte(*c);
    }
    for c in b"=====================================\n" {
        serial.write_byte(*c);
    }
}

// =============================================================================
// Panic Handler (bare-metal only)
// =============================================================================

#[cfg(not(feature = "usermode"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let serial = Serial;
    for c in b"KERNEL PANIC: " {
        serial.write_byte(*c);
    }
    // Just print that we panicked, fmt might be broken
    for c in b"(see info)\n" {
        serial.write_byte(*c);
    }
    loop {
        unsafe {
            asm!("hlt", options(nostack, nomem));
        }
    }
}
