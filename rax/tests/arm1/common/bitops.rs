//! Bit operation tests for ARM execution.
//!
//! Ported from arm-js-py/bitops.py
//! Copyright 2012, Ryota Ozaki
//! Copyright 2014, espes

use rax::arm::decoder::ShiftType;
use rax::arm::execution::*;

#[test]
fn test_clear_bit() {
    assert_eq!(0xffffffffu32 & !(1 << 0), 0xfffffffe);
    assert_eq!(0x13u32 & !(1u32 << 31), 0x13);
    assert_eq!(0x13u32 & !(1 << 0), 0x12);
}

#[test]
fn test_clear_bits() {
    fn clear_bits(val: u32, start: u32, end: u32) -> u32 {
        let mask = ((1u64 << (start + 1)) - 1) & !((1u64 << end) - 1);
        val & !(mask as u32)
    }
    assert_eq!(clear_bits(0xffffffff, 31, 0), 0);
    assert_eq!(clear_bits(0xffffffff, 31, 16), 0x0000ffff);
    assert_eq!(clear_bits(0xffffffff, 15, 0), 0xffff0000);
    assert_eq!(clear_bits(0xffffffff, 15, 12), 0xffff0fff);
    assert_eq!(clear_bits(0x0fffffff, 15, 12), 0x0fff0fff);
}

#[test]
fn test_xor() {
    assert_eq!(0xffffffffu32 ^ 0xffffffff, 0);
    assert_eq!(0x11111111u32 ^ 0x22222222, 0x33333333);
    assert_eq!(0xf0000000u32 ^ 0xf0000000, 0);
}

#[test]
fn test_xor64() {
    assert_eq!(0xffffffffu64 ^ 0xffffffff, 0);
    assert_eq!(0x11111111u64 ^ 0x22222222, 0x33333333);
    assert_eq!(0xf0000000u64 ^ 0xf0000000, 0);
    assert_eq!(0x1f0000000u64 ^ 0xf0000000, 0x100000000);
}

#[test]
fn test_not() {
    fn not32(x: u32) -> u32 {
        !x
    }
    assert_eq!(not32(0xffffffff), 0x00000000);
    assert_eq!(not32(0x00000000), 0xffffffff);
    assert_eq!(not32(0x00000001), 0xfffffffe);
    assert_eq!(not32(0x80000000), 0x7fffffff);
}

#[test]
fn test_or() {
    assert_eq!(0x11111111u32 | 0x22222222, 0x33333333);
    assert_eq!(0xffffffffu32 | 0x00000000, 0xffffffff);
    assert_eq!(0xffffffffu32 | 0xffffffff, 0xffffffff);
}

#[test]
fn test_or64() {
    assert_eq!(0x11111111u64 | 0x22222222, 0x33333333);
    assert_eq!(0xffffffffu64 | 0x00000000, 0xffffffff);
    assert_eq!(0xffffffffu64 | 0xffffffff, 0xffffffff);
    assert_eq!(0xf00000000u64 | 0x00000000, 0xf00000000);
    assert_eq!(0xf00000000u64 | 0x0000000f, 0xf0000000f);
}

#[test]
fn test_and() {
    assert_eq!(0x11111111u32 & 0x22222222, 0);
    assert_eq!(0xffffffffu32 & 0, 0);
}

#[test]
fn test_and64() {
    assert_eq!(0x11111111u64 & 0x22222222, 0);
    assert_eq!(0xffffffffu64 & 0, 0);
    assert_eq!(0xffffffffffffu64 & 0, 0);
    assert_eq!(0xffffffffffffu64 & 0xffffffff, 0xffffffff);
}

#[test]
fn test_get_bit() {
    fn get_bit(val: u32, pos: u32) -> u32 {
        (val >> pos) & 1
    }
    assert_eq!(get_bit(0xffffffff, 31), 1);
    assert_eq!(get_bit(0xffffffff, 0), 1);
    assert_eq!(get_bit(0x80000000, 31), 1);
    assert_eq!(get_bit(0, 31), 0);
    assert_eq!(get_bit(0, 0), 0);
    assert_eq!(get_bit(0x7fffffff, 31), 0);
    assert_eq!(get_bit(0x80000000, 31), 1);
}

#[test]
fn test_get_bit64() {
    fn get_bit64(val: u64, pos: u32) -> u64 {
        (val >> pos) & 1
    }
    assert_eq!(get_bit64(0xffffffff, 31), 1);
    assert_eq!(get_bit64(0xffffffff, 0), 1);
    assert_eq!(get_bit64(0x80000000, 31), 1);
    assert_eq!(get_bit64(0, 31), 0);
    assert_eq!(get_bit64(0, 0), 0);
    assert_eq!(get_bit64(0x7fffffff, 31), 0);
    assert_eq!(get_bit64(0xffffffffffff, 31), 1);
    assert_eq!(get_bit64(0xffffffffffff, 50), 0);
}

#[test]
fn test_get_bits() {
    fn get_bits(val: u32, start: u32, end: u32) -> u32 {
        (val >> end) & ((1u64 << (start - end + 1)) - 1) as u32
    }
    assert_eq!(get_bits(0xffffffff, 31, 0), 0xffffffff);
    assert_eq!(get_bits(0xffffffff, 31, 16), 0xffff);
    assert_eq!(get_bits(0, 31, 0), 0);
    assert_eq!(get_bits(0x13, 4, 0), 0x13);
    assert_eq!(get_bits(0xf0000000, 31, 27), 0x1e);
    assert_eq!(get_bits(0xc0000000, 31, 27), 0x18);
}

#[test]
fn test_set_bit() {
    fn set_bit(val: u32, pos: u32, bit: u32) -> u32 {
        if bit != 0 {
            val | (1 << pos)
        } else {
            val & !(1 << pos)
        }
    }
    assert_eq!(set_bit(0xffffffff, 0, 0), 0xfffffffe);
    assert_eq!(set_bit(0xffffffff, 31, 0), 0x7fffffff);
    assert_eq!(set_bit(0xffffffff, 31, 1), 0xffffffff);
    assert_eq!(set_bit(0x13, 31, 0), 0x13);
    assert_eq!(set_bit(0, 31, 1), 0x80000000);
    assert_eq!(set_bit(0, 0, 1), 1);
    assert_eq!(set_bit(0, 2, 1), 4);
}

#[test]
fn test_set_bits() {
    fn set_bits(val: u32, start: u32, end: u32, bits: u32) -> u32 {
        let mask = ((1u64 << (start + 1)) - 1) & !((1u64 << end) - 1);
        (val & !(mask as u32)) | ((bits << end) & mask as u32)
    }
    assert_eq!(set_bits(0xffffffff, 31, 0, 0), 0);
    assert_eq!(set_bits(0xffffffff, 15, 0, 0), 0xffff0000);
    assert_eq!(set_bits(0, 4, 0, 0x13), 0x13);
    assert_eq!(set_bits(0xf0000000, 31, 27, 0x1e), 0xf0000000);
    assert_eq!(set_bits(0x00000000, 31, 27, 0x1e), 0xf0000000);
    assert_eq!(set_bits(0xf0000000, 31, 27, 0x18), 0xc0000000);
}

#[test]
fn test_lsl() {
    assert_eq!(1u64 << 1, 2);
    assert_eq!(0xf0000000u64 << 1, 0x1e0000000);
    assert_eq!(0xffffffffu64 << 1, 0x1fffffffe);
    assert_eq!(0xf0f0f0f0u64 << 4, 0xf0f0f0f00);
    assert_eq!(0x100000000u64 << 1, 0x200000000);
}

#[test]
fn test_lsr() {
    fn lsr(val: u32, amount: u32) -> u32 {
        if amount >= 32 {
            0
        } else {
            val >> amount
        }
    }
    assert_eq!(lsr(1, 1), 0);
    assert_eq!(lsr(0xf0000000, 1), 0x78000000);
    assert_eq!(lsr(0xffffffff, 1), 0x7fffffff);
    assert_eq!(lsr(0xf0f0f0f0, 4), 0x0f0f0f0f);
    assert_eq!(lsr(0x80000000, 32), 0);
    assert_eq!(lsr(0x80000000, 1), 0x40000000);
}

#[test]
fn test_sint32() {
    assert_eq!(sint32(0x00000000), 0x00000000);
    assert_eq!(sint32(0x80000000), -2147483648);
    assert_eq!(sint32(0xffffffff), -1);
}

#[test]
fn test_uint32() {
    fn uint32(x: u64) -> u32 {
        (x & 0xffffffff) as u32
    }
    assert_eq!(uint32(0x00000000), 0x00000000);
    assert_eq!(uint32(0x80000000), 0x80000000);
    assert_eq!(uint32(0x100000000), 0x00000000);
    assert_eq!(uint32(0xffffffff), 0xffffffff);
    assert_eq!(uint32(0xfffffffff), 0xffffffff);
}

#[test]
fn test_sign_extend() {
    assert_eq!(sign_extend(0, 26), 0);
    assert_eq!(sign_extend(0, 1), 0);
    assert_eq!(sign_extend(1, 1), 0xffffffff);
    assert_eq!(sign_extend(0x0000ffff, 16), 0xffffffff);
    assert_eq!(sign_extend(0x00007fff, 16), 0x00007fff);
    assert_eq!(sign_extend(0xffffe3 << 2, 26), 0xffffff8c);
}

#[test]
fn test_copy_bits() {
    fn copy_bits(dest: u32, start: u32, end: u32, src: u32) -> u32 {
        let mask = ((1u64 << (start + 1)) - 1) & !((1u64 << end) - 1);
        let src_bits = (src >> end) & ((1 << (start - end + 1)) - 1);
        (dest & !(mask as u32)) | ((src_bits << end) & mask as u32)
    }
    assert_eq!(copy_bits(0xf0000000, 31, 27, 0), 0);
    assert_eq!(copy_bits(0xf0000000, 31, 27, 0xc0000000), 0xc0000000);
}

#[test]
fn test_copy_bit() {
    fn copy_bit(dest: u32, pos: u32, src: u32) -> u32 {
        let bit = (src >> pos) & 1;
        if bit != 0 {
            dest | (1 << pos)
        } else {
            dest & !(1 << pos)
        }
    }
    assert_eq!(copy_bit(0, 0, 1), 1);
    assert_eq!(copy_bit(1, 0, 0), 0);
    assert_eq!(copy_bit(0xffffffff, 0, 0), 0xfffffffe);
    assert_eq!(copy_bit(0xffffffff, 31, 0), 0x7fffffff);
}

#[test]
fn test_ror() {
    assert_eq!(ror(0x10000000, 1), 0x08000000);
    assert_eq!(ror(0x10000001, 1), 0x88000000);
    assert_eq!(ror(0xffffffff, 1), 0xffffffff);
    assert_eq!(ror(0x0000ffff, 16), 0xffff0000);
    assert_eq!(ror(0x000ffff0, 16), 0xfff0000f);
}

#[test]
fn test_count_leading_zeros() {
    assert_eq!(count_leading_zeros(0), 32);
    assert_eq!(count_leading_zeros(0x80000000), 0);
    assert_eq!(count_leading_zeros(0x00008000), 16);
}
