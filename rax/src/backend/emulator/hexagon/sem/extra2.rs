//! Additional scalar register-only instructions that route through the sem
//! dispatch layer but were previously unimplemented:
//!   * `A5_ACS`         -- `Rxx,Pe = vacsh(Rss,Rtt)` (vector add-compare-select).
//!   * `S2_cabacdecbin` -- `Rdd = decbin(Rss,Rtt)` (CABAC regular-bin decode, +P0).
//!   * `A4_tlbmatch`    -- `Pd = tlbmatch(Rss,Rt)` (pure-function TLB-entry match).
//!
//! Semantics are taken verbatim from the Hexagon V68 spec
//! (`tools/hexagon/qemu/imported/{alu,shift,compare}.idef`) and verified against
//! the `qemu-hexagon` reference oracle (`tests/hexagon_diff.rs`).

use super::super::opcode::{DecodedOp, Opcode};
use super::{SemCtx, fld};

// --- H.264 / Hexagon CABAC decode tables (S2_cabacdecbin) ------------------
//
// These are the standard H.264/AVC arithmetic-decode tables used by the
// Hexagon `decbin` instruction:
//   * `R_LPS_TABLE_64X4` is `rangeTabLPS` (H.264 Table 9-46): for each of the
//     64 probability states, the LPS sub-range for the 4 quantized range
//     buckets `(range>>29)&3`.
//   * `AC_NEXT_STATE_MPS_64` is `transIdxMPS` (H.264 Table 9-45, MPS column).
//   * `AC_NEXT_STATE_LPS_64` is `transIdxLPS` (H.264 Table 9-45, LPS column).
// Verified cell-for-cell against the qemu-hexagon oracle by probing `decbin`
// (see `recover_decbin_tables`/`diff_decbin_exhaustive` in tests/hexagon_diff).
// R_LPS_TABLE is indexed rLPS_table[state][(range>>29)&3] and matches standard
// H.264 rangeTabLPS exactly.
#[rustfmt::skip]
const R_LPS_TABLE_64X4: [[u8; 4]; 64] = [
    [128, 176, 208, 240], [128, 167, 197, 227], [128, 158, 187, 216], [123, 150, 178, 205],
    [116, 142, 169, 195], [111, 135, 160, 185], [105, 128, 152, 175], [100, 122, 144, 166],
    [ 95, 116, 137, 158], [ 90, 110, 130, 150], [ 85, 104, 123, 142], [ 81,  99, 117, 135],
    [ 77,  94, 111, 128], [ 73,  89, 105, 122], [ 69,  85, 100, 116], [ 66,  80,  95, 110],
    [ 62,  76,  90, 104], [ 59,  72,  86,  99], [ 56,  69,  81,  94], [ 53,  65,  77,  89],
    [ 51,  62,  73,  85], [ 48,  59,  69,  80], [ 46,  56,  66,  76], [ 43,  53,  63,  72],
    [ 41,  50,  59,  69], [ 39,  48,  56,  65], [ 37,  45,  54,  62], [ 35,  43,  51,  59],
    [ 33,  41,  48,  56], [ 32,  39,  46,  53], [ 30,  37,  43,  50], [ 29,  35,  41,  48],
    [ 27,  33,  39,  45], [ 26,  31,  37,  43], [ 24,  30,  35,  41], [ 23,  28,  33,  39],
    [ 22,  27,  32,  37], [ 21,  26,  30,  35], [ 20,  24,  29,  33], [ 19,  23,  27,  31],
    [ 18,  22,  26,  30], [ 17,  21,  25,  28], [ 16,  20,  23,  27], [ 15,  19,  22,  25],
    [ 14,  18,  21,  24], [ 14,  17,  20,  23], [ 13,  16,  19,  22], [ 12,  15,  18,  21],
    [ 12,  14,  17,  20], [ 11,  14,  16,  19], [ 11,  13,  15,  18], [ 10,  12,  15,  17],
    [ 10,  12,  14,  16], [  9,  11,  13,  15], [  9,  11,  12,  14], [  8,  10,  12,  14],
    [  8,   9,  11,  13], [  7,   9,  11,  12], [  7,   9,  10,  12], [  7,   8,  10,  11],
    [  6,   8,   9,  11], [  6,   7,   9,  10], [  6,   7,   8,   9], [  2,   2,   2,   2],
];

#[rustfmt::skip]
const AC_NEXT_STATE_MPS_64: [u8; 64] = [
     1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 62, 63,
];

// Note: this is the Hexagon `decbin` LPS transition table, recovered cell-for-
// cell from the qemu-hexagon oracle. It differs from the textbook H.264
// transIdxLPS at index 28 (Hexagon = 22, textbook = 23); the rest match.
#[rustfmt::skip]
const AC_NEXT_STATE_LPS_64: [u8; 64] = [
     0,  0,  1,  2,  2,  4,  4,  5,  6,  7,  8,  9,  9, 11, 11, 12,
    13, 13, 15, 15, 16, 16, 18, 18, 19, 19, 21, 21, 22, 22, 23, 24,
    24, 25, 26, 26, 27, 27, 28, 29, 29, 30, 30, 30, 31, 32, 32, 33,
    33, 33, 34, 34, 35, 35, 35, 36, 36, 36, 37, 37, 37, 38, 38, 63,
];

/// Execute an "extra2" opcode. Returns `false` if `op` is not in this set.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    match op {
        // A5_ACS: Rxx32,Pe4 = vacsh(Rss32,Rtt32)
        //
        //   for i in 0..4:
        //     xv = (int)Rxx.h[i];  sv = (int)Rss.h[i];  tv = (int)Rtt.h[i];
        //     xv = xv + tv;     // 17-bit datapath, no saturation here
        //     sv = sv - tv;     // 17-bit datapath, no saturation here
        //     Pe[2i]   = Pe[2i+1] = (xv > sv);
        //     Rxx.h[i] = fSATH(fMAX(xv, sv));
        Opcode::A5_ACS => {
            let rxx = ctx.rp(fld(d, b'x'));
            let rss = ctx.rp(fld(d, b's'));
            let rtt = ctx.rp(fld(d, b't'));
            let mut out: u64 = 0;
            let mut pe: u8 = 0;
            for i in 0..4 {
                let xv = ((rxx >> (i * 16)) & 0xffff) as u16 as i16 as i32;
                let sv0 = ((rss >> (i * 16)) & 0xffff) as u16 as i16 as i32;
                let tv = ((rtt >> (i * 16)) & 0xffff) as u16 as i16 as i32;
                let xv = xv + tv;
                let sv = sv0 - tv;
                if xv > sv {
                    pe |= 0b11 << (i * 2);
                }
                let max = if xv > sv { xv } else { sv };
                // fSATH(16): saturating clamp that raises the sticky USR overflow
                // bit when it clamps (matches hardware USR:0 semantics).
                let h = ctx.sat_n(max as i64, 16) as u16 as u64;
                out |= h << (i * 16);
            }
            ctx.set_rp(fld(d, b'x'), out);
            ctx.set_p(fld(d, b'e'), pe);
        }

        // S2_cabacdecbin: Rdd32 = decbin(Rss32,Rtt32)  (also writes P0)
        Opcode::S2_cabacdecbin => {
            let rss = ctx.rp(fld(d, b's'));
            let rtt = ctx.rp(fld(d, b't'));

            let rtt_w1 = (rtt >> 32) as u32;
            let rtt_w0 = rtt as u32;
            let state = (rtt_w1 & 0x3f) as usize; // bits [5:0]
            let val_mps = (rtt_w1 >> 8) & 1; // bit [8]
            let bitpos = rtt_w0 & 0x1f; // bits [4:0]

            let mut range = rss as u32; // Rss.w0
            let mut offset = (rss >> 32) as u32; // Rss.w1

            range <<= bitpos;
            offset <<= bitpos;

            let r_lps = (R_LPS_TABLE_64X4[state][((range >> 29) & 3) as usize] as u32) << 23;
            let r_mps = (range & 0xff80_0000).wrapping_sub(r_lps);

            let mut rdd_w0: u32;
            let rdd_w1: u32;
            let p0: u8;

            if offset < r_mps {
                // most probable region
                rdd_w0 = AC_NEXT_STATE_MPS_64[state] as u32;
                rdd_w0 = insert_range(rdd_w0, 8, 8, val_mps);
                rdd_w0 = insert_range(rdd_w0, 31, 23, r_mps >> 23);
                rdd_w1 = offset;
                p0 = val_mps as u8;
            } else {
                // least probable region
                rdd_w0 = AC_NEXT_STATE_LPS_64[state] as u32;
                let mps_bit = if state == 0 { 1 - val_mps } else { val_mps };
                rdd_w0 = insert_range(rdd_w0, 8, 8, mps_bit);
                rdd_w0 = insert_range(rdd_w0, 31, 23, r_lps >> 23);
                rdd_w1 = offset.wrapping_sub(r_mps);
                p0 = (val_mps ^ 1) as u8;
            }

            let rdd = (rdd_w0 as u64) | ((rdd_w1 as u64) << 32);
            ctx.set_rp(fld(d, b'd'), rdd);
            ctx.set_p(0, p0);
        }

        // A4_tlbmatch: Pd4 = tlbmatch(Rss32,Rt32)
        //
        //   MASK  = 0x07ffffff
        //   TLBLO = Rss.w0;  TLBHI = Rss.w1
        //   SIZE  = min(6, cl1(~brev(TLBLO)))      // count leading ones of reverse(~TLBLO)
        //   MASK &= 0xffffffff << (2*SIZE)
        //   Pd = 8bitsof( bit31(TLBHI) && ((TLBHI & MASK) == (Rt & MASK)) )
        Opcode::A4_tlbmatch => {
            let rss = ctx.rp(fld(d, b's'));
            let rt = ctx.r(fld(d, b't'));
            let tlblo = rss as u32; // Rss.w0
            let tlbhi = (rss >> 32) as u32; // Rss.w1
            let mut mask: u32 = 0x07ff_ffff;
            // cl1_4(~brev_4(TLBLO)) = leading ones of bit-reversed (~TLBLO).
            let v = (!tlblo).reverse_bits();
            let size = v.leading_ones().min(6);
            mask &= 0xffff_ffffu32.wrapping_shl(2 * size);
            let valid = (tlbhi >> 31) & 1 != 0;
            let matched = valid && ((tlbhi & mask) == (rt & mask));
            ctx.set_p(fld(d, b'd'), if matched { 0xff } else { 0x00 });
        }

        _ => return false,
    }
    true
}

/// `fINSERT_RANGE(reg, hibit, lobit, val)`: replace bits `[hibit:lobit]` of
/// `reg` with the low `(hibit-lobit+1)` bits of `val`.
#[inline]
fn insert_range(reg: u32, hibit: u32, lobit: u32, val: u32) -> u32 {
    let width = hibit - lobit + 1;
    let field_mask = if width >= 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    (reg & !(field_mask << lobit)) | ((val & field_mask) << lobit)
}
