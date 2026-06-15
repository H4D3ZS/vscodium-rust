//! RVC (compressed, 16-bit) instruction decoding.
//!
//! Each compressed parcel expands to exactly one base-ISA operation; the
//! expansion is performed here at decode time so the execution path is shared
//! with the 32-bit encodings (and inherits their verified semantics). Every
//! produced [`Insn`] carries `len = 2` so the PC advances by two bytes and
//! link registers receive `pc + 2`.
//!
//! Both RV32C and RV64C are handled; the few encodings that differ between
//! XLENs (C.LD/C.FLW, C.SD/C.FSW, C.ADDIW/C.JAL, the *SP loads/stores) are
//! selected on [`Xlen`].

use super::decode::{Insn, Op};
use super::{Isa, Xlen};

/// Construct a decoded compressed instruction (always 2 bytes).
fn mk(op: Op, rd: u8, rs1: u8, rs2: u8, imm: i64, half: u16) -> Insn {
    Insn {
        op,
        rd,
        rs1,
        rs2,
        rs3: 0,
        imm,
        funct3: 0,
        csr: 0,
        aq: false,
        rl: false,
        len: 2,
        raw: half as u32,
    }
}

#[inline]
fn ill(half: u16) -> Insn {
    Insn::illegal_compressed(half)
}

/// Extract bits `[hi:lo]` of `h` as a u32.
#[inline]
fn bits(h: u16, hi: u32, lo: u32) -> u32 {
    ((h as u32) >> lo) & ((1u32 << (hi - lo + 1)) - 1)
}
#[inline]
fn bit(h: u16, n: u32) -> u32 {
    ((h as u32) >> n) & 1
}
/// Compressed 3-bit register field -> x8..x15.
#[inline]
fn rvc_reg(r: u32) -> u8 {
    (r as u8 & 0x7) + 8
}

/// Sign-extend the low `n` bits of `v`.
#[inline]
fn sext(v: u32, n: u32) -> i64 {
    let shift = 32 - n;
    (((v << shift) as i32) >> shift) as i64
}

/// Decode a non-zero compressed parcel.
pub fn decode_rvc(half: u16, xlen: Xlen, isa: &Isa) -> Insn {
    let rv64 = xlen == Xlen::Rv64;
    let quadrant = half & 0x3;
    let funct3 = bits(half, 15, 13);
    match quadrant {
        0 => decode_q0(half, funct3, rv64, isa),
        1 => decode_q1(half, funct3, rv64, isa),
        2 => decode_q2(half, funct3, rv64, isa),
        _ => ill(half), // quadrant 3 is not compressed
    }
}

fn decode_q0(h: u16, funct3: u32, rv64: bool, isa: &Isa) -> Insn {
    let rd_ = rvc_reg(bits(h, 4, 2));
    let rs1_ = rvc_reg(bits(h, 9, 7));
    match funct3 {
        0b000 => {
            // C.ADDI4SPN -> addi rd', x2, nzuimm
            let nzuimm = (bits(h, 12, 11) << 4)
                | (bits(h, 10, 7) << 6)
                | (bit(h, 6) << 2)
                | (bit(h, 5) << 3);
            if nzuimm == 0 {
                return ill(h); // reserved
            }
            mk(Op::Addi, rd_, 2, 0, nzuimm as i64, h)
        }
        0b001 => {
            // C.FLD -> fld rd', off(rs1')  (RV32 & RV64; double)
            let off = (bits(h, 12, 10) << 3) | (bits(h, 6, 5) << 6);
            mk(Op::Fld, rd_, rs1_, 0, off as i64, h)
        }
        0b010 => {
            // C.LW -> lw rd', off(rs1')
            let off = (bits(h, 12, 10) << 3) | (bit(h, 6) << 2) | (bit(h, 5) << 6);
            mk(Op::Lw, rd_, rs1_, 0, off as i64, h)
        }
        0b011 => {
            let off_d = (bits(h, 12, 10) << 3) | (bits(h, 6, 5) << 6);
            if rv64 {
                // C.LD -> ld rd', off(rs1')
                mk(Op::Ld, rd_, rs1_, 0, off_d as i64, h)
            } else {
                // C.FLW -> flw rd', off(rs1')
                let off = (bits(h, 12, 10) << 3) | (bit(h, 6) << 2) | (bit(h, 5) << 6);
                mk(Op::Flw, rd_, rs1_, 0, off as i64, h)
            }
        }
        0b101 => {
            // C.FSD -> fsd rs2', off(rs1')
            let off = (bits(h, 12, 10) << 3) | (bits(h, 6, 5) << 6);
            mk(Op::Fsd, 0, rs1_, rvc_reg(bits(h, 4, 2)), off as i64, h)
        }
        0b110 => {
            // C.SW -> sw rs2', off(rs1')
            let off = (bits(h, 12, 10) << 3) | (bit(h, 6) << 2) | (bit(h, 5) << 6);
            mk(Op::Sw, 0, rs1_, rvc_reg(bits(h, 4, 2)), off as i64, h)
        }
        0b111 => {
            let off_d = (bits(h, 12, 10) << 3) | (bits(h, 6, 5) << 6);
            if rv64 {
                // C.SD -> sd rs2', off(rs1')
                mk(Op::Sd, 0, rs1_, rvc_reg(bits(h, 4, 2)), off_d as i64, h)
            } else {
                // C.FSW -> fsw rs2', off(rs1')
                let off = (bits(h, 12, 10) << 3) | (bit(h, 6) << 2) | (bit(h, 5) << 6);
                mk(Op::Fsw, 0, rs1_, rvc_reg(bits(h, 4, 2)), off as i64, h)
            }
        }
        0b100 if isa.zcb => decode_zcb_q0(h, rd_, rs1_),
        _ => ill(h),
    }
}

/// Zcb quadrant-0 byte/half loads and stores.
fn decode_zcb_q0(h: u16, rd_: u8, rs1_: u8) -> Insn {
    let rs2_ = rvc_reg(bits(h, 4, 2));
    match bits(h, 12, 10) {
        0b000 => {
            // c.lbu: uimm = {bit5, bit6}
            let uimm = (bit(h, 5) << 1) | bit(h, 6);
            mk(Op::Lbu, rd_, rs1_, 0, uimm as i64, h)
        }
        0b001 => {
            // c.lhu (bit6=0) / c.lh (bit6=1); uimm = bit5 << 1
            let uimm = (bit(h, 5) << 1) as i64;
            if bit(h, 6) == 1 {
                mk(Op::Lh, rd_, rs1_, 0, uimm, h)
            } else {
                mk(Op::Lhu, rd_, rs1_, 0, uimm, h)
            }
        }
        0b010 => {
            // c.sb: uimm = {bit5, bit6}
            let uimm = (bit(h, 5) << 1) | bit(h, 6);
            mk(Op::Sb, 0, rs1_, rs2_, uimm as i64, h)
        }
        0b011 => {
            // c.sh (bit6 must be 0); uimm = bit5 << 1
            if bit(h, 6) != 0 {
                return ill(h);
            }
            mk(Op::Sh, 0, rs1_, rs2_, (bit(h, 5) << 1) as i64, h)
        }
        _ => ill(h),
    }
}

fn decode_q1(h: u16, funct3: u32, rv64: bool, isa: &Isa) -> Insn {
    let rd = bits(h, 11, 7) as u8;
    match funct3 {
        0b000 => {
            // C.ADDI -> addi rd, rd, nzimm (rd==0 -> C.NOP; both still addi)
            let imm = sext((bit(h, 12) << 5) | bits(h, 6, 2), 6);
            mk(Op::Addi, rd, rd, 0, imm, h)
        }
        0b001 => {
            let imm = sext((bit(h, 12) << 5) | bits(h, 6, 2), 6);
            if rv64 {
                // C.ADDIW -> addiw rd, rd, imm (rd==0 reserved)
                if rd == 0 {
                    return ill(h);
                }
                mk(Op::Addiw, rd, rd, 0, imm, h)
            } else {
                // C.JAL -> jal x1, offset
                let off = cj_offset(h);
                mk(Op::Jal, 1, 0, 0, off, h)
            }
        }
        0b010 => {
            // C.LI -> addi rd, x0, imm
            let imm = sext((bit(h, 12) << 5) | bits(h, 6, 2), 6);
            mk(Op::Addi, rd, 0, 0, imm, h)
        }
        0b011 => {
            if rd == 2 {
                // C.ADDI16SP -> addi x2, x2, nzimm
                let v = (bit(h, 12) << 9)
                    | (bits(h, 4, 3) << 7)
                    | (bit(h, 5) << 6)
                    | (bit(h, 2) << 5)
                    | (bit(h, 6) << 4);
                if v == 0 {
                    return ill(h);
                }
                mk(Op::Addi, 2, 2, 0, sext(v, 10), h)
            } else {
                // C.LUI -> lui rd, nzimm (value already sign-extended << 12)
                let v = (bit(h, 12) << 17) | (bits(h, 6, 2) << 12);
                if v == 0 || rd == 0 {
                    return ill(h);
                }
                mk(Op::Lui, rd, 0, 0, sext(v, 18), h)
            }
        }
        0b100 => decode_q1_alu(h, rv64, isa),
        0b101 => {
            // C.J -> jal x0, offset
            mk(Op::Jal, 0, 0, 0, cj_offset(h), h)
        }
        0b110 => {
            // C.BEQZ -> beq rs1', x0, offset
            mk(Op::Beq, 0, rvc_reg(bits(h, 9, 7)), 0, cb_offset(h), h)
        }
        0b111 => {
            // C.BNEZ -> bne rs1', x0, offset
            mk(Op::Bne, 0, rvc_reg(bits(h, 9, 7)), 0, cb_offset(h), h)
        }
        _ => ill(h),
    }
}

fn decode_q1_alu(h: u16, rv64: bool, isa: &Isa) -> Insn {
    let rd_ = rvc_reg(bits(h, 9, 7));
    let funct2 = bits(h, 11, 10);
    match funct2 {
        0b00 => {
            // C.SRLI -> srli rd', rd', shamt
            let shamt = (bit(h, 12) << 5) | bits(h, 6, 2);
            mk(Op::Srli, rd_, rd_, 0, shamt as i64, h)
        }
        0b01 => {
            // C.SRAI -> srai rd', rd', shamt
            let shamt = (bit(h, 12) << 5) | bits(h, 6, 2);
            mk(Op::Srai, rd_, rd_, 0, shamt as i64, h)
        }
        0b10 => {
            // C.ANDI -> andi rd', rd', imm
            let imm = sext((bit(h, 12) << 5) | bits(h, 6, 2), 6);
            mk(Op::Andi, rd_, rd_, 0, imm, h)
        }
        0b11 => {
            let rs2_ = rvc_reg(bits(h, 4, 2));
            match (bit(h, 12), bits(h, 6, 5)) {
                (0, 0b00) => mk(Op::Sub, rd_, rd_, rs2_, 0, h),
                (0, 0b01) => mk(Op::Xor, rd_, rd_, rs2_, 0, h),
                (0, 0b10) => mk(Op::Or, rd_, rd_, rs2_, 0, h),
                (0, 0b11) => mk(Op::And, rd_, rd_, rs2_, 0, h),
                (1, 0b00) if rv64 => mk(Op::Subw, rd_, rd_, rs2_, 0, h),
                (1, 0b01) if rv64 => mk(Op::Addw, rd_, rd_, rs2_, 0, h),
                // Zcb: c.mul (10) and the zext/sext/not unary ops (11).
                (1, 0b10) if isa.zcb => mk(Op::Mul, rd_, rd_, rs2_, 0, h),
                (1, 0b11) if isa.zcb => match bits(h, 4, 2) {
                    0b000 => mk(Op::Andi, rd_, rd_, 0, 0xff, h), // c.zext.b
                    0b001 => mk(Op::SextB, rd_, rd_, 0, 0, h),   // c.sext.b
                    0b010 => mk(Op::ZextH, rd_, rd_, 0, 0, h),   // c.zext.h
                    0b011 => mk(Op::SextH, rd_, rd_, 0, 0, h),   // c.sext.h
                    0b100 => mk(Op::AddUw, rd_, rd_, 0, 0, h),   // c.zext.w (add.uw rd',rd',x0)
                    0b101 => mk(Op::Xori, rd_, rd_, 0, -1, h),   // c.not
                    _ => ill(h),
                },
                _ => ill(h),
            }
        }
        _ => ill(h),
    }
}

fn decode_q2(h: u16, funct3: u32, rv64: bool, _isa: &Isa) -> Insn {
    let rd = bits(h, 11, 7) as u8;
    match funct3 {
        0b000 => {
            // C.SLLI -> slli rd, rd, shamt (rd==0 hint)
            let shamt = (bit(h, 12) << 5) | bits(h, 6, 2);
            mk(Op::Slli, rd, rd, 0, shamt as i64, h)
        }
        0b001 => {
            // C.FLDSP -> fld rd, off(x2)
            let off = (bit(h, 12) << 5) | (bits(h, 6, 5) << 3) | (bits(h, 4, 2) << 6);
            mk(Op::Fld, rd, 2, 0, off as i64, h)
        }
        0b010 => {
            // C.LWSP -> lw rd, off(x2) (rd==0 reserved)
            if rd == 0 {
                return ill(h);
            }
            let off = (bit(h, 12) << 5) | (bits(h, 6, 4) << 2) | (bits(h, 3, 2) << 6);
            mk(Op::Lw, rd, 2, 0, off as i64, h)
        }
        0b011 => {
            if rv64 {
                // C.LDSP -> ld rd, off(x2) (rd==0 reserved)
                if rd == 0 {
                    return ill(h);
                }
                let off = (bit(h, 12) << 5) | (bits(h, 6, 5) << 3) | (bits(h, 4, 2) << 6);
                mk(Op::Ld, rd, 2, 0, off as i64, h)
            } else {
                // C.FLWSP -> flw rd, off(x2)
                let off = (bit(h, 12) << 5) | (bits(h, 6, 4) << 2) | (bits(h, 3, 2) << 6);
                mk(Op::Flw, rd, 2, 0, off as i64, h)
            }
        }
        0b100 => {
            let rs2 = bits(h, 6, 2) as u8;
            if bit(h, 12) == 0 {
                if rs2 == 0 {
                    // C.JR -> jalr x0, 0(rs1) (rs1==0 reserved)
                    if rd == 0 {
                        return ill(h);
                    }
                    mk(Op::Jalr, 0, rd, 0, 0, h)
                } else {
                    // C.MV -> add rd, x0, rs2
                    mk(Op::Add, rd, 0, rs2, 0, h)
                }
            } else if rs2 == 0 {
                if rd == 0 {
                    // C.EBREAK
                    mk(Op::Ebreak, 0, 0, 0, 0, h)
                } else {
                    // C.JALR -> jalr x1, 0(rs1)
                    mk(Op::Jalr, 1, rd, 0, 0, h)
                }
            } else {
                // C.ADD -> add rd, rd, rs2
                mk(Op::Add, rd, rd, rs2, 0, h)
            }
        }
        0b101 => {
            // C.FSDSP -> fsd rs2, off(x2)
            let off = (bits(h, 12, 10) << 3) | (bits(h, 9, 7) << 6);
            mk(Op::Fsd, 0, 2, bits(h, 6, 2) as u8, off as i64, h)
        }
        0b110 => {
            // C.SWSP -> sw rs2, off(x2)
            let off = (bits(h, 12, 9) << 2) | (bits(h, 8, 7) << 6);
            mk(Op::Sw, 0, 2, bits(h, 6, 2) as u8, off as i64, h)
        }
        0b111 => {
            if rv64 {
                // C.SDSP -> sd rs2, off(x2)
                let off = (bits(h, 12, 10) << 3) | (bits(h, 9, 7) << 6);
                mk(Op::Sd, 0, 2, bits(h, 6, 2) as u8, off as i64, h)
            } else {
                // C.FSWSP -> fsw rs2, off(x2)
                let off = (bits(h, 12, 9) << 2) | (bits(h, 8, 7) << 6);
                mk(Op::Fsw, 0, 2, bits(h, 6, 2) as u8, off as i64, h)
            }
        }
        _ => ill(h),
    }
}

/// C.J / C.JAL jump offset (sign-extended, even).
fn cj_offset(h: u16) -> i64 {
    let v = (bit(h, 12) << 11)
        | (bit(h, 11) << 4)
        | (bits(h, 10, 9) << 8)
        | (bit(h, 8) << 10)
        | (bit(h, 7) << 6)
        | (bit(h, 6) << 7)
        | (bits(h, 5, 3) << 1)
        | (bit(h, 2) << 5);
    sext(v, 12)
}

/// C.BEQZ / C.BNEZ branch offset (sign-extended, even).
fn cb_offset(h: u16) -> i64 {
    let v = (bit(h, 12) << 8)
        | (bits(h, 11, 10) << 3)
        | (bits(h, 6, 5) << 6)
        | (bits(h, 4, 3) << 1)
        | (bit(h, 2) << 5);
    sext(v, 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(h: u16) -> Insn {
        decode_rvc(h, Xlen::Rv64, &Isa::rv64gc())
    }

    #[test]
    fn c_addi() {
        // c.addi x8, 1 : funct3=000, rd=8, imm=1 -> 0x0405? build by fields
        // [15:13]=000, [12]=imm5=0, [11:7]=rd=8, [6:2]=imm[4:0]=1, [1:0]=01
        let h = (0 << 13) | (8 << 7) | (1 << 2) | 0b01;
        let i = dec(h as u16);
        assert_eq!(i.op, Op::Addi);
        assert_eq!(i.rd, 8);
        assert_eq!(i.rs1, 8);
        assert_eq!(i.imm, 1);
        assert_eq!(i.len, 2);
    }

    #[test]
    fn c_li() {
        // c.li x10, -1 : funct3=010, rd=10, imm=-1 (all imm bits set)
        let h = (0b010 << 13) | (1 << 12) | (10 << 7) | (0x1f << 2) | 0b01;
        let i = dec(h as u16);
        assert_eq!(i.op, Op::Addi);
        assert_eq!(i.rd, 10);
        assert_eq!(i.rs1, 0);
        assert_eq!(i.imm, -1);
    }

    #[test]
    fn c_mv_add() {
        // c.mv x10, x11 : funct3=100, bit12=0, rd=10, rs2=11
        let h = (0b100 << 13) | (0 << 12) | (10 << 7) | (11 << 2) | 0b10;
        let i = dec(h as u16);
        assert_eq!(i.op, Op::Add);
        assert_eq!(i.rd, 10);
        assert_eq!(i.rs1, 0);
        assert_eq!(i.rs2, 11);
    }

    #[test]
    fn c_ebreak() {
        let h = (0b100 << 13) | (1 << 12) | 0b10;
        assert_eq!(dec(h as u16).op, Op::Ebreak);
    }
}
