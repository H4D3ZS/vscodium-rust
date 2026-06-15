use std::sync::Arc;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use super::decode::{
    AddrMode, CmpKind, CombineOperand, DecodedInsn, DecodedSub, ExtendKind, MemOpKind, MemOpSrc,
    MemSign, MemWidth, PredCond, ShiftKind, SpliceAmount, decode, decode_duplex, isa_supports_insn,
};
use super::opcode::Opcode;
use crate::config::{Endianness, HexagonIsa};
use crate::cpu::{CpuState, HexagonRegisters, VCpu, VcpuExit};
use crate::error::{Error, Result};

const SERIAL_MMIO_BASE: u64 = 0xf000_0000;
const SERIAL_MMIO_LEN: u64 = 8;
/// Debug-console MMIO port (single byte sink/source); see `crate::devices::map`.
const DEBUG_MMIO_BASE: u64 = crate::devices::map::HEXAGON_DEBUG_MMIO_BASE;
const DEBUG_MMIO_LEN: u64 = 8;
const MAX_RUN_ITERATIONS: u64 = 100_000;

/// 32-bit register-amount shift (`asl`/`asr`/`lsr`(Rs,Rt)).
///
/// The shift amount is the low 7 bits of `rt` sign-extended to a signed value
/// in `[-64, 63]` (`fSXTN(7,32,RtV)`); a negative amount reverses the shift
/// direction (`fBIDIR_*` macros). The shift is evaluated in 64 bits then
/// truncated, so amounts up to 63 are well defined and saturate to all-zero or
/// all-sign as the spec requires.
fn hex_reg_shift32(val: u32, rt: u32, kind: ShiftKind) -> u32 {
    let raw = rt & 0x7f;
    let shamt = ((raw as i32) << 25) >> 25; // sign-extend bit 6
    let result: i64 = match kind {
        // asl: signed source, left-biased (negative amount -> arithmetic right).
        ShiftKind::Lsl => {
            let s = val as i32 as i64;
            if shamt < 0 {
                (s >> ((-shamt) - 1)) >> 1
            } else {
                s << shamt
            }
        }
        // asr: signed source, right-biased (negative amount -> left).
        ShiftKind::Asr => {
            let s = val as i32 as i64;
            if shamt < 0 {
                (s << ((-shamt) - 1)) << 1
            } else {
                s >> shamt
            }
        }
        // lsr: unsigned source, right-biased (negative amount -> logical left).
        ShiftKind::Lsr => {
            let u = val as u64;
            (if shamt < 0 {
                (u << ((-shamt) - 1)) << 1
            } else {
                u >> shamt
            }) as i64
        }
    };
    result as u32
}

/// Bit-reverse the low 16 bits of `addr`, keeping the upper 16 bits intact
/// (`fbrev`/`fEA_BREVR`). Used by bit-reverse post-increment addressing, where
/// the FFT-style address is `brev(Rx)`.
fn hex_brev(addr: u32) -> u32 {
    let low = (addr & 0xffff) as u16;
    let rev = low.reverse_bits() as u32;
    (addr & 0xffff_0000) | rev
}

/// Hexagon circular-buffer post-increment (`fcirc_add`). `reg` is the current
/// pointer, `incr` the signed byte increment, `m` the modifier register
/// (M0/M1: bits 0..16 are the buffer length, bits 24..27 the K field), and
/// `cs` the circular-start register (CS0/CS1). The pointer wraps within
/// `[start, start+length)`. With `K==0` (the common case) `start` is exactly
/// `cs`; otherwise the start is derived from the masked pointer (matching
/// hardware behavior, which is only well-defined for K==0 / length>=4).
fn hex_circ_add(reg: u32, incr: i32, m: u32, cs: u32) -> u32 {
    let length = m & 0x0001_ffff;
    let k_const = (m >> 24) & 0xf;
    let new_ptr = reg.wrapping_add(incr as u32);
    let (start_addr, end_addr) = if k_const == 0 && length >= 4 {
        (cs, cs.wrapping_add(length))
    } else {
        let mask = (1u32 << (k_const + 2)).wrapping_sub(1);
        let start = reg & !mask;
        (start, start | (length & mask))
    };
    if new_ptr >= end_addr {
        new_ptr.wrapping_sub(length)
    } else if new_ptr < start_addr {
        new_ptr.wrapping_add(length)
    } else {
        new_ptr
    }
}

/// Read the I field of a modifier register (`fREAD_IREG`): an 11-bit signed
/// value packed as `((M & 0xf0000000) >> 21) | ((M >> 17) & 0x7f)`.
fn hex_read_ireg(m: u32) -> i32 {
    let packed = ((m & 0xf000_0000) >> 21) | ((m >> 17) & 0x7f);
    // Sign-extend the 11-bit value.
    ((packed << 21) as i32) >> 21
}

/// An HVX vector's 32 little-endian words as a flat 128-byte array.
fn vec_to_bytes(v: &[u32; 32]) -> [u8; 128] {
    let mut b = [0u8; 128];
    for i in 0..32 {
        b[i * 4..i * 4 + 4].copy_from_slice(&v[i].to_le_bytes());
    }
    b
}

/// Inverse of [`vec_to_bytes`].
fn bytes_to_vec(b: &[u8; 128]) -> [u32; 32] {
    let mut v = [0u32; 32];
    for i in 0..32 {
        v[i] = u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]]);
    }
    v
}

/// Snapshot of which GPRs currently have a buffered (in-flight) write.
fn producer_mask(new_r: &[Option<u32>; 32]) -> [bool; 32] {
    std::array::from_fn(|i| new_r[i].is_some())
}

/// Record the GPR an instruction just produced (the lowest newly-written
/// register) into the packet's producer list, for new-value resolution.
fn record_producer(new_r: &[Option<u32>; 32], before: [bool; 32], producers: &mut Vec<u8>) {
    for i in 0..32 {
        if new_r[i].is_some() && !before[i] {
            producers.push(i as u8);
            return;
        }
    }
}

/// Resolve a new-value store: `Nt8 >> 1` is the back-distance (1 = most recent)
/// among the packet's GPR producers; the selected producer's register is the
/// store data source (read as a `.new`/in-flight value). Non-`StoreNew` insns
/// pass through unchanged.
fn resolve_new_value(insn: DecodedInsn, producers: &[u8]) -> DecodedInsn {
    match insn {
        DecodedInsn::StoreNew {
            nt,
            addr,
            width,
            pred,
        } => {
            let back = (nt >> 1) as usize;
            let src = if back >= 1 && back <= producers.len() {
                producers[producers.len() - back]
            } else {
                0
            };
            DecodedInsn::Store {
                src,
                addr,
                width,
                pred,
                src_new: true,
                high_half: false,
            }
        }
        // New-value compound compare-and-jump (`_jumpnv`): `src1` is the `Ns8`
        // producer index (`Ns8 >> 1` = back-distance among the packet's GPR
        // producers, identical to a new-value store). Rewrite `src1` to the
        // resolved producer register and clear the `new_value` flag so the
        // executor reads the (already-committed-in-flight) GPR directly.
        DecodedInsn::CompoundCmpJump {
            kind,
            src1,
            src2,
            write_pred,
            sense,
            new_value: true,
            offset,
        } => {
            let back = (src1 >> 1) as usize;
            let resolved = if back >= 1 && back <= producers.len() {
                producers[producers.len() - back]
            } else {
                0
            };
            DecodedInsn::CompoundCmpJump {
                kind,
                src1: resolved,
                src2,
                write_pred,
                sense,
                new_value: false,
                offset,
            }
        }
        other => other,
    }
}

struct MmioPending {
    dst: u8,
    size: u8,
    signed: bool,
}

struct BranchTarget {
    target: u32,
    is_call: bool,
}

struct PacketState {
    packet_pc: u32,
    pc: u32,
    immext: Option<u32>,
    new_r: [Option<u32>; 32],
    new_p: [Option<u8>; 4],
    branch: Option<BranchTarget>,
    inst_index: usize,
    first_parse: Option<u32>,
    second_parse: Option<u32>,
    pending_subinsn: Option<DecodedSub>,
    pending_end: bool,
}

impl PacketState {
    fn new(pc: u32) -> Self {
        PacketState {
            packet_pc: pc,
            pc,
            immext: None,
            new_r: [None; 32],
            new_p: [None; 4],
            branch: None,
            inst_index: 0,
            first_parse: None,
            second_parse: None,
            pending_subinsn: None,
            pending_end: false,
        }
    }
}

pub struct HexagonVcpu {
    id: u32,
    regs: HexagonRegisters,
    mem: Arc<GuestMemoryMmap>,
    halted: bool,
    pending_mmio: Option<MmioPending>,
    pending_packet: Option<PacketState>,
    _isa: HexagonIsa,
    endian: Endianness,
    /// In-flight HVX vector writes for the current packet (committed at packet
    /// end so vector reads observe old values, matching VLIW semantics).
    new_v: [Option<[u32; 32]>; 32],
    new_q: [Option<[u32; 4]>; 4],
    /// `.tmp` HVX vector loads: forwarded to same-packet consumers (read via
    /// `vread_new`) but NEVER committed to the architectural register file —
    /// `.tmp` is scratch. Cleared each packet alongside `new_v`.
    new_v_tmp: [Option<[u32; 32]>; 32],
    /// Per-packet HVX gather scratch (`vtmp`, qemu `tmp_VRegs[0]`): the result of
    /// the packet's `vgather`, consumed by the paired `vmem(Rs+#k)=vtmp.new`
    /// store. A packet has at most one gather. Cleared each packet.
    ///
    /// `(data, valid)`: byte `b` of `data` is meaningful only where `valid[b]`.
    /// On hardware the gather temp is initialised from the destination memory and
    /// only in-range / predicate-on lanes are overwritten; the paired store thus
    /// writes only the `valid` bytes, leaving the rest of the destination intact.
    gather_tmp: Option<([u8; 128], [bool; 128])>,
    /// The most recently produced architectural HVX vector register in the current
    /// packet (in execution order). Resolves a new-value vector store
    /// (`vmem(...) = V.new`) to its in-packet producer. `None` when no vector has
    /// been produced yet (then a `vmem=vtmp.new` store falls back to `gather_tmp`).
    last_v_produced: Option<u8>,
    /// LL/SC reservation: the (word/dword-aligned) address reserved by the most
    /// recent load-locked, or `None`. A store-conditional succeeds (and clears
    /// it) only when this matches the SC address; otherwise the SC fails. Models
    /// the qemu-hexagon behaviour where a bare store-conditional fails.
    lock_addr: Option<u32>,
}

impl HexagonVcpu {
    pub fn new(id: u32, mem: Arc<GuestMemoryMmap>, isa: HexagonIsa, endian: Endianness) -> Self {
        HexagonVcpu {
            id,
            regs: HexagonRegisters::default(),
            mem,
            halted: false,
            pending_mmio: None,
            pending_packet: None,
            _isa: isa,
            endian,
            new_v: [None; 32],
            new_q: [None; 4],
            new_v_tmp: [None; 32],
            gather_tmp: None,
            last_v_produced: None,
            lock_addr: None,
        }
    }

    fn read_u8(&self, addr: u32) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.mem
            .read_slice(&mut buf, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(buf[0])
    }

    fn read_u16(&self, addr: u32) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.mem
            .read_slice(&mut buf, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(match self.endian {
            Endianness::Little => u16::from_le_bytes(buf),
            Endianness::Big => u16::from_be_bytes(buf),
        })
    }

    fn read_u32(&self, addr: u32) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.mem
            .read_slice(&mut buf, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(match self.endian {
            Endianness::Little => u32::from_le_bytes(buf),
            Endianness::Big => u32::from_be_bytes(buf),
        })
    }

    fn read_u64(&self, addr: u32) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.mem
            .read_slice(&mut buf, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(match self.endian {
            Endianness::Little => u64::from_le_bytes(buf),
            Endianness::Big => u64::from_be_bytes(buf),
        })
    }

    fn write_u8(&self, addr: u32, value: u8) -> Result<()> {
        self.mem
            .write_slice(&[value], GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(())
    }

    fn write_u16(&self, addr: u32, value: u16) -> Result<()> {
        let bytes = match self.endian {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.mem
            .write_slice(&bytes, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(())
    }

    fn write_u32(&self, addr: u32, value: u32) -> Result<()> {
        let bytes = match self.endian {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.mem
            .write_slice(&bytes, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(())
    }

    fn write_u64(&self, addr: u32, value: u64) -> Result<()> {
        let bytes = match self.endian {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        self.mem
            .write_slice(&bytes, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(())
    }

    /// Read a 128-byte HVX vector from memory as 32 little-endian words. HVX
    /// vmem is byte-transparent (the vector's byte order is memory order), so
    /// the words use the same LE packing as the `v[]` register storage.
    fn read_vec(&self, addr: u32) -> Result<[u32; 32]> {
        let mut buf = [0u8; 128];
        self.mem
            .read_slice(&mut buf, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        let mut v = [0u32; 32];
        for i in 0..32 {
            v[i] = u32::from_le_bytes([buf[i * 4], buf[i * 4 + 1], buf[i * 4 + 2], buf[i * 4 + 3]]);
        }
        Ok(v)
    }

    /// Write a 128-byte HVX vector to memory (32 little-endian words).
    fn write_vec(&self, addr: u32, vec: &[u32; 32]) -> Result<()> {
        let mut buf = [0u8; 128];
        for i in 0..32 {
            buf[i * 4..i * 4 + 4].copy_from_slice(&vec[i].to_le_bytes());
        }
        self.mem
            .write_slice(&buf, GuestAddress(addr as u64))
            .map_err(Error::from)?;
        Ok(())
    }

    /// Read element `idx` (0-based) of an HVX vector register `vreg` as an
    /// `esz`-byte little-endian value (`esz` = 2 or 4).
    fn velem(&self, vreg: u8, esz: u8, idx: usize) -> u32 {
        let bytes = vec_to_bytes(&self.regs.v[vreg as usize]);
        let off = idx * esz as usize;
        match esz {
            2 => u16::from_le_bytes([bytes[off], bytes[off + 1]]) as u32,
            _ => u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]]),
        }
    }

    /// Vector-byte predicate bit `i` (0..128) of vector predicate register `q`.
    /// Predicated scatter/gather mask per byte: byte `b` of element `k` is gated
    /// on bit `k*esz + b` (qemu's byte-granular scatter/gather predicate).
    fn qbit(&self, q: u8, i: usize) -> bool {
        let qv = self.regs.q[q as usize];
        (qv[i / 32] >> (i % 32)) & 1 != 0
    }

    /// HVX V65 scatter. For each element the (data) effective address is
    /// `EA = Rt + offset[i]`; the element is committed only if `Rt <= EA <= Rt+Mu`
    /// (and, for the `q` forms, its predicate bit is set). `_add` forms align the
    /// offset (`offset &= ~(esz-1)`) and accumulate (`*EA += data`); plain forms
    /// store (`*EA = data`). Elements are processed in order (later wins).
    #[allow(clippy::too_many_arguments)]
    fn exec_scatter(
        &mut self,
        base: u8,
        modsel: u8,
        offsets: u8,
        data: u8,
        esz: u8,
        off_pair: bool,
        add: bool,
        pred: Option<u8>,
    ) -> Result<()> {
        let mask = (esz as u32) - 1;
        let rt = self.regs.r[base as usize] & !mask;
        let mu = self.modifier(modsel) | mask;
        let rt_hi = rt.wrapping_add(mu);
        // hw (off_pair) forms: 64 halfword elements, each from a word offset in
        // the Vvv pair; element k's offset is word (k>>1) of vector (offsets+(k&1)).
        let nelem = if off_pair { 64 } else { 128 / esz as usize };
        let width = if esz == 2 {
            MemWidth::Half
        } else {
            MemWidth::Word
        };
        let esz = esz as usize;
        for k in 0..nelem {
            let mut off = if off_pair {
                self.velem(offsets + (k & 1) as u8, 4, k >> 1)
            } else {
                self.velem(offsets, esz as u8, k)
            };
            if add {
                off &= !mask;
            }
            let ea = rt.wrapping_add(off);
            if ea < rt || ea > rt_hi {
                continue;
            }
            let datum = self.velem(data, esz as u8, k);
            if add {
                // Accumulate forms are never predicated (no `q` variant); RMW the
                // whole element width, wrapping at that width.
                let cur = self.load_mem(ea, width, MemSign::Unsigned)?;
                let sum = match esz {
                    2 => (cur as u16).wrapping_add(datum as u16) as u32,
                    _ => cur.wrapping_add(datum),
                };
                self.store_mem(ea, width, sum)?;
            } else {
                // Plain store. The `q` forms mask PER BYTE: byte `b` of element
                // `k` is written iff predicate bit `k*esz + b` is set (matching
                // qemu's byte-granular scatter predicate).
                let bytes = datum.to_le_bytes();
                for b in 0..esz {
                    if let Some(q) = pred {
                        if !self.qbit(q, k * esz + b) {
                            continue;
                        }
                    }
                    self.store_mem(ea.wrapping_add(b as u32), MemWidth::Byte, bytes[b] as u32)?;
                }
            }
        }
        Ok(())
    }

    /// HVX V65 gather: read elements from `Rt + offset[i]` into a 128-byte vector,
    /// returning `(data, valid)`. Byte `b` is gathered (`valid[b] = true`) iff its
    /// element's effective address is in range and (for the `q` forms) its
    /// predicate bit is set; non-valid bytes are left for the paired store to
    /// preserve from the destination memory (matching hardware, where the gather
    /// temp is initialised from the destination and only valid lanes overwrite).
    fn exec_gather(
        &self,
        base: u8,
        modsel: u8,
        offsets: u8,
        esz: u8,
        off_pair: bool,
        pred: Option<u8>,
    ) -> Result<([u8; 128], [bool; 128])> {
        let mask = (esz as u32) - 1;
        let rt = self.regs.r[base as usize] & !mask;
        let mu = self.modifier(modsel) | mask;
        let rt_hi = rt.wrapping_add(mu);
        let nelem = if off_pair { 64 } else { 128 / esz as usize };
        let width = if esz == 2 {
            MemWidth::Half
        } else {
            MemWidth::Word
        };
        let esz = esz as usize;
        let mut out = [0u8; 128];
        let mut valid = [false; 128];
        for k in 0..nelem {
            let off = if off_pair {
                self.velem(offsets + (k & 1) as u8, 4, k >> 1)
            } else {
                self.velem(offsets, esz as u8, k)
            };
            let ea = rt.wrapping_add(off);
            if ea < rt || ea > rt_hi {
                continue;
            }
            let datum = self.load_mem(ea, width, MemSign::Unsigned)?;
            let bytes = datum.to_le_bytes();
            let dst = k * esz;
            // The `q` forms gather PER BYTE: byte `b` of element `k` is gathered
            // iff predicate bit `k*esz + b` is set.
            for b in 0..esz {
                if let Some(q) = pred {
                    if !self.qbit(q, k * esz + b) {
                        continue;
                    }
                }
                out[dst + b] = bytes[b];
                valid[dst + b] = true;
            }
        }
        Ok((out, valid))
    }

    fn fetch_word(&self, pc: u32) -> Result<u32> {
        self.read_u32(pc)
    }

    fn ensure_isa_supports(
        &self,
        word: u32,
        insn: &DecodedInsn,
        opcode: Option<Opcode>,
    ) -> Result<()> {
        if isa_supports_insn(self._isa, insn, opcode) {
            return Ok(());
        }
        Err(Error::Emulator(format!(
            "hexagon instruction 0x{word:08x} not supported by ISA {:?}",
            self._isa
        )))
    }

    fn is_mmio(addr: u32) -> bool {
        let addr = addr as u64;
        (addr >= SERIAL_MMIO_BASE && addr < SERIAL_MMIO_BASE + SERIAL_MMIO_LEN)
            || (addr >= DEBUG_MMIO_BASE && addr < DEBUG_MMIO_BASE + DEBUG_MMIO_LEN)
    }

    fn set_pc(&mut self, pc: u32) {
        self.regs.set_pc(pc);
    }

    /// USR.LPCFG (bits 9:8): software-pipelined loop config. Counts the number of
    /// loop iterations remaining before the pipeline predicate (P3) is set.
    fn get_lpcfg(&self) -> u32 {
        (self.regs.c[8] >> 8) & 0x3
    }

    fn set_lpcfg(&mut self, value: u32) {
        self.regs.c[8] = (self.regs.c[8] & !(0x3 << 8)) | ((value & 0x3) << 8);
    }

    fn load_mem(&self, addr: u32, width: MemWidth, sign: MemSign) -> Result<u32> {
        match width {
            MemWidth::Byte => {
                let val = self.read_u8(addr)?;
                Ok(match sign {
                    MemSign::Signed => (val as i8 as i32) as u32,
                    MemSign::Unsigned => val as u32,
                })
            }
            MemWidth::Half => {
                let val = self.read_u16(addr)?;
                Ok(match sign {
                    MemSign::Signed => (val as i16 as i32) as u32,
                    MemSign::Unsigned => val as u32,
                })
            }
            MemWidth::Word => self.read_u32(addr),
            MemWidth::Double => Err(Error::Emulator(
                "doubleword load not supported yet".to_string(),
            )),
        }
    }

    fn store_mem(&self, addr: u32, width: MemWidth, value: u32) -> Result<()> {
        match width {
            MemWidth::Byte => self.write_u8(addr, value as u8),
            MemWidth::Half => self.write_u16(addr, value as u16),
            MemWidth::Word => self.write_u32(addr, value),
            MemWidth::Double => Err(Error::Emulator(
                "doubleword store not supported yet".to_string(),
            )),
        }
    }

    fn commit_packet(&mut self, new_r: &[Option<u32>; 32], new_p: &[Option<u8>; 4]) {
        for (idx, val) in new_r.iter().enumerate() {
            if let Some(value) = val {
                self.regs.r[idx] = *value;
            }
        }
        for (idx, val) in new_p.iter().enumerate() {
            if let Some(value) = val {
                self.regs.set_predicate(idx, *value);
            }
        }
        // Commit in-flight HVX vector / vector-predicate writes. `.tmp` loads
        // (new_v_tmp) are scratch: discard them without writing the register file.
        for idx in 0..32 {
            if let Some(value) = self.new_v[idx].take() {
                self.regs.v[idx] = value;
            }
            self.new_v_tmp[idx] = None;
        }
        for idx in 0..4 {
            if let Some(value) = self.new_q[idx].take() {
                self.regs.q[idx] = value;
            }
        }
    }

    fn eval_pred(&self, cond: PredCond, new_p: &[Option<u8>; 4]) -> bool {
        let val = if cond.pred_new {
            new_p[cond.pred as usize].unwrap_or(self.regs.p[cond.pred as usize])
        } else {
            self.regs.p[cond.pred as usize]
        };
        // Conditional execution tests only the least-significant bit.
        let lsb = val & 1 != 0;
        if cond.sense { lsb } else { !lsb }
    }

    fn read_reg_with_new(&self, reg: u8, new_r: &[Option<u32>; 32]) -> u32 {
        new_r[reg as usize].unwrap_or(self.regs.r[reg as usize])
    }

    /// Modifier register value for `modsel` (0 -> M0/C6, 1 -> M1/C7).
    fn modifier(&self, modsel: u8) -> u32 {
        self.regs.control(if modsel & 1 == 0 { 6 } else { 7 })
    }

    /// Circular-start register for `modsel` (0 -> CS0/C12, 1 -> CS1/C13).
    fn circ_start(&self, modsel: u8) -> u32 {
        self.regs.control(if modsel & 1 == 0 { 12 } else { 13 })
    }

    /// Resolve a load/store `AddrMode` to `(effective_addr, base_update)`,
    /// where `base_update` is the post-increment write-back `(reg, value)`.
    fn resolve_addr(&self, addr: AddrMode) -> (u32, Option<(u8, u32)>) {
        match addr {
            AddrMode::Offset { base, offset } => {
                let base_val = self.regs.r[base as usize];
                (base_val.wrapping_add(offset as u32), None)
            }
            AddrMode::PostIncImm { base, offset } => {
                let base_val = self.regs.r[base as usize];
                let new_base = base_val.wrapping_add(offset as u32);
                (base_val, Some((base, new_base)))
            }
            AddrMode::GpOffset { offset } => {
                let gp = self.regs.control(11) & !0x3f; // GP low 6 bits hardwired zero
                (gp.wrapping_add(offset as u32), None)
            }
            AddrMode::Abs { addr } => (addr, None),
            AddrMode::PostIncReg { base, modsel } => {
                let base_val = self.regs.r[base as usize];
                let new_base = base_val.wrapping_add(self.modifier(modsel));
                (base_val, Some((base, new_base)))
            }
            AddrMode::PostIncBrev { base, modsel } => {
                let base_val = self.regs.r[base as usize];
                let ea = hex_brev(base_val);
                let new_base = base_val.wrapping_add(self.modifier(modsel));
                (ea, Some((base, new_base)))
            }
            AddrMode::PostIncCircImm { base, modsel, incr } => {
                let base_val = self.regs.r[base as usize];
                let new_base = hex_circ_add(
                    base_val,
                    incr,
                    self.modifier(modsel),
                    self.circ_start(modsel),
                );
                (base_val, Some((base, new_base)))
            }
            AddrMode::PostIncCircReg {
                base,
                modsel,
                shift,
            } => {
                let base_val = self.regs.r[base as usize];
                let m = self.modifier(modsel);
                let incr = hex_read_ireg(m).wrapping_shl(shift as u32);
                let new_base = hex_circ_add(base_val, incr, m, self.circ_start(modsel));
                (base_val, Some((base, new_base)))
            }
            AddrMode::RegScaled { base, index, shift } => {
                let base_val = self.regs.r[base as usize];
                let idx = self.regs.r[index as usize].wrapping_shl(shift as u32);
                (base_val.wrapping_add(idx), None)
            }
            // `memX(Re=##U6)`: EA is the absolute address; the address register
            // Re is also written with that same absolute (a base "update").
            AddrMode::AbsSet { areg, addr } => (addr, Some((areg, addr))),
            // `memX(Ru<<#u2+##U6)`: EA = abs + (Ru << shift); no register update.
            AddrMode::IndexAbs { index, shift, addr } => {
                let idx = self.regs.r[index as usize].wrapping_shl(shift as u32);
                (addr.wrapping_add(idx), None)
            }
        }
    }

    fn set_branch(
        &self,
        branch: &mut Option<BranchTarget>,
        target: u32,
        is_call: bool,
    ) -> Result<()> {
        if branch.is_some() {
            return Err(Error::Emulator("multiple branches in packet".to_string()));
        }
        *branch = Some(BranchTarget { target, is_call });
        Ok(())
    }

    fn execute_insn(
        &mut self,
        insn: DecodedInsn,
        packet_pc: u32,
        insn_pc: u32,
        immext: Option<u32>,
        new_r: &mut [Option<u32>; 32],
        new_p: &mut [Option<u8>; 4],
        branch: &mut Option<BranchTarget>,
    ) -> Result<Option<VcpuExit>> {
        let pcrel_base = if immext.is_some() { packet_pc } else { insn_pc };
        match insn {
            DecodedInsn::ImmExt { .. } => {
                return Err(Error::Emulator("unexpected immext in execute".to_string()));
            }
            DecodedInsn::Add { dst, src1, src2 } => {
                let val = self.regs.r[src1 as usize].wrapping_add(self.regs.r[src2 as usize]);
                new_r[dst as usize] = Some(val);
            }
            DecodedInsn::Sub { dst, src1, src2 } => {
                let val = self.regs.r[src1 as usize].wrapping_sub(self.regs.r[src2 as usize]);
                new_r[dst as usize] = Some(val);
            }
            DecodedInsn::And { dst, src1, src2 } => {
                new_r[dst as usize] = Some(self.regs.r[src1 as usize] & self.regs.r[src2 as usize]);
            }
            DecodedInsn::AndImm { dst, src, imm } => {
                new_r[dst as usize] = Some(self.regs.r[src as usize] & imm);
            }
            DecodedInsn::OrImm { dst, src, imm } => {
                new_r[dst as usize] = Some(self.regs.r[src as usize] | imm);
            }
            DecodedInsn::Or { dst, src1, src2 } => {
                new_r[dst as usize] = Some(self.regs.r[src1 as usize] | self.regs.r[src2 as usize]);
            }
            DecodedInsn::Xor { dst, src1, src2 } => {
                new_r[dst as usize] = Some(self.regs.r[src1 as usize] ^ self.regs.r[src2 as usize]);
            }
            DecodedInsn::AddImm { dst, src, imm } => {
                let val = self.regs.r[src as usize].wrapping_add(imm as u32);
                new_r[dst as usize] = Some(val);
            }
            // SubImmRev: Rd = #s - Rs (reversed subtract)
            DecodedInsn::SubImmRev { dst, src, imm } => {
                let val = (imm as u32).wrapping_sub(self.regs.r[src as usize]);
                new_r[dst as usize] = Some(val);
            }
            DecodedInsn::Mov { dst, src } => {
                new_r[dst as usize] = Some(self.regs.r[src as usize]);
            }
            DecodedInsn::MovImm { dst, imm } => {
                new_r[dst as usize] = Some(imm as u32);
            }
            DecodedInsn::ClearCond { dst, pred } => {
                if self.eval_pred(pred, new_p) {
                    new_r[dst as usize] = Some(0);
                }
            }
            DecodedInsn::Extend { dst, src, kind } => {
                let val = self.regs.r[src as usize];
                let result = match kind {
                    ExtendKind::Sxt8 => (val as i8 as i32) as u32,
                    ExtendKind::Sxt16 => (val as i16 as i32) as u32,
                    ExtendKind::Zxt8 => (val & 0xff) as u32,
                    ExtendKind::Zxt16 => (val & 0xffff) as u32,
                };
                new_r[dst as usize] = Some(result);
            }
            DecodedInsn::Combine { dst, high, low } => {
                let odd = dst.wrapping_add(1);
                if odd >= 32 {
                    return Err(Error::Emulator(
                        "invalid register pair for combine".to_string(),
                    ));
                }
                let low_val = match low {
                    CombineOperand::Reg(reg) => self.regs.r[reg as usize],
                    CombineOperand::Imm(val) => val,
                };
                let high_val = match high {
                    CombineOperand::Reg(reg) => self.regs.r[reg as usize],
                    CombineOperand::Imm(val) => val,
                };
                new_r[dst as usize] = Some(low_val);
                new_r[odd as usize] = Some(high_val);
            }
            DecodedInsn::Load {
                dst,
                addr,
                width,
                sign,
                pred,
            } => {
                if let Some(cond) = pred {
                    if !self.eval_pred(cond, new_p) {
                        return Ok(None);
                    }
                }

                let (addr, update) = self.resolve_addr(addr);

                if let Some((reg, value)) = update {
                    new_r[reg as usize] = Some(value);
                }

                if Self::is_mmio(addr) {
                    let size = match width {
                        MemWidth::Byte => 1,
                        MemWidth::Half => 2,
                        MemWidth::Word => 4,
                        MemWidth::Double => {
                            return Err(Error::Emulator(
                                "doubleword mmio load not supported".to_string(),
                            ));
                        }
                    };
                    let signed = matches!(sign, MemSign::Signed) && size < 4;
                    self.pending_mmio = Some(MmioPending { dst, size, signed });
                    return Ok(Some(VcpuExit::MmioRead {
                        addr: addr as u64,
                        size,
                    }));
                }

                if width == MemWidth::Double {
                    let even = dst & !1;
                    let odd = even.wrapping_add(1);
                    if odd >= 32 {
                        return Err(Error::Emulator(
                            "invalid register pair for doubleword load".to_string(),
                        ));
                    }
                    let val = self.read_u64(addr)?;
                    new_r[even as usize] = Some(val as u32);
                    new_r[odd as usize] = Some((val >> 32) as u32);
                } else {
                    let val = self.load_mem(addr, width, sign)?;
                    new_r[dst as usize] = Some(val);
                }
            }
            DecodedInsn::LoadAlign {
                dst_pair,
                addr,
                width,
                pred,
            } => {
                // Predicate CANCEL: no Ryy write and no post-increment.
                if let Some(cond) = pred {
                    if !self.eval_pred(cond, new_p) {
                        return Ok(None);
                    }
                }
                let (addr, update) = self.resolve_addr(addr);
                if let Some((reg, value)) = update {
                    new_r[reg as usize] = Some(value);
                }
                let even = dst_pair & !1;
                let odd = even.wrapping_add(1);
                if odd >= 32 {
                    return Err(Error::Emulator(
                        "invalid register pair for loadalign".to_string(),
                    ));
                }
                // Current Ryy (use the in-packet value if a producer set it).
                let lo = self.read_reg_with_new(even, new_r) as u64;
                let hi = self.read_reg_with_new(odd, new_r) as u64;
                let old = lo | (hi << 32);
                // `w` = access width in bits (8 byte / 16 half). Shift Ryy right
                // by w and insert the freshly-loaded value at the top:
                //   Ryy = (Ryy u>> w) | (zxt(load) << (64-w)).
                let (w, loaded) = match width {
                    MemWidth::Byte => (8u32, self.read_u8(addr)? as u64),
                    MemWidth::Half => (16u32, self.read_u16(addr)? as u64),
                    _ => {
                        return Err(Error::Emulator(
                            "loadalign supports only byte/half".to_string(),
                        ));
                    }
                };
                let result = (old >> w) | (loaded << (64 - w));
                new_r[even as usize] = Some(result as u32);
                new_r[odd as usize] = Some((result >> 32) as u32);
            }
            DecodedInsn::LoadUnpack {
                dst,
                addr,
                count,
                sign,
                pred,
            } => {
                // Predicate CANCEL (none of the bsw/bzw forms are predicated
                // today, but honour the field for forward-compatibility).
                if let Some(cond) = pred {
                    if !self.eval_pred(cond, new_p) {
                        return Ok(None);
                    }
                }
                let (addr, update) = self.resolve_addr(addr);
                if let Some((reg, value)) = update {
                    new_r[reg as usize] = Some(value);
                }
                // Load `count` contiguous bytes; unpack each into a halfword
                // lane, sign- (membh) or zero-extended (memubh).
                let mut result: u64 = 0;
                for i in 0..count as u32 {
                    let b = self.read_u8(addr.wrapping_add(i))?;
                    let lane = match sign {
                        MemSign::Signed => (b as i8 as i16) as u16,
                        MemSign::Unsigned => b as u16,
                    };
                    result |= (lane as u64) << (16 * i);
                }
                if count == 2 {
                    new_r[dst as usize] = Some(result as u32);
                } else {
                    let even = dst & !1;
                    let odd = even.wrapping_add(1);
                    if odd >= 32 {
                        return Err(Error::Emulator(
                            "invalid register pair for loadunpack".to_string(),
                        ));
                    }
                    new_r[even as usize] = Some(result as u32);
                    new_r[odd as usize] = Some((result >> 32) as u32);
                }
            }
            DecodedInsn::Store {
                src,
                addr,
                width,
                pred,
                src_new,
                high_half,
            } => {
                if let Some(cond) = pred {
                    if !self.eval_pred(cond, new_p) {
                        // Cancelled store: no memory write AND no post-increment.
                        return Ok(None);
                    }
                }

                let (addr, update) = self.resolve_addr(addr);

                if let Some((reg, value)) = update {
                    new_r[reg as usize] = Some(value);
                }

                if width == MemWidth::Double {
                    let even = src & !1;
                    let odd = even.wrapping_add(1);
                    if odd >= 32 {
                        return Err(Error::Emulator(
                            "invalid register pair for doubleword store".to_string(),
                        ));
                    }
                    if Self::is_mmio(addr) {
                        return Err(Error::Emulator(
                            "doubleword mmio store not supported".to_string(),
                        ));
                    }
                    let even_val = if src_new {
                        self.read_reg_with_new(even, new_r)
                    } else {
                        self.regs.r[even as usize]
                    };
                    let odd_val = if src_new {
                        self.read_reg_with_new(odd, new_r)
                    } else {
                        self.regs.r[odd as usize]
                    };
                    let combined = ((odd_val as u64) << 32) | even_val as u64;
                    self.write_u64(addr, combined)?;
                } else {
                    let mut val = if src_new {
                        self.read_reg_with_new(src, new_r)
                    } else {
                        self.regs.r[src as usize]
                    };
                    // `storerf` high-half store: the stored halfword is Rt[31:16].
                    if high_half {
                        val >>= 16;
                    }

                    if Self::is_mmio(addr) {
                        let data = match width {
                            MemWidth::Byte => vec![val as u8],
                            MemWidth::Half => match self.endian {
                                Endianness::Little => (val as u16).to_le_bytes().to_vec(),
                                Endianness::Big => (val as u16).to_be_bytes().to_vec(),
                            },
                            MemWidth::Word => match self.endian {
                                Endianness::Little => val.to_le_bytes().to_vec(),
                                Endianness::Big => val.to_be_bytes().to_vec(),
                            },
                            MemWidth::Double => {
                                return Err(Error::Emulator(
                                    "doubleword mmio store not supported".to_string(),
                                ));
                            }
                        };
                        return Ok(Some(VcpuExit::MmioWrite {
                            addr: addr as u64,
                            data,
                        }));
                    }
                    self.store_mem(addr, width, val)?;
                }
            }
            DecodedInsn::StoreImm {
                value,
                addr,
                width,
                pred,
            } => {
                if let Some(cond) = pred {
                    if !self.eval_pred(cond, new_p) {
                        return Ok(None);
                    }
                }
                let addr = match addr {
                    AddrMode::Offset { base, offset } => {
                        let base_val = self.regs.r[base as usize];
                        base_val.wrapping_add(offset as u32)
                    }
                    AddrMode::GpOffset { offset } => {
                        let gp = self.regs.control(11) & !0x3f; // GP low 6 bits are hardwired zero
                        gp.wrapping_add(offset as u32)
                    }
                    AddrMode::Abs { addr } => addr,
                    AddrMode::RegScaled { base, index, shift } => {
                        let base_val = self.regs.r[base as usize];
                        let idx = self.regs.r[index as usize].wrapping_shl(shift as u32);
                        base_val.wrapping_add(idx)
                    }
                    AddrMode::PostIncImm { .. }
                    | AddrMode::PostIncReg { .. }
                    | AddrMode::PostIncBrev { .. }
                    | AddrMode::PostIncCircImm { .. }
                    | AddrMode::PostIncCircReg { .. }
                    | AddrMode::AbsSet { .. }
                    | AddrMode::IndexAbs { .. } => {
                        return Err(Error::Emulator(
                            "post-increment store immediate not supported".to_string(),
                        ));
                    }
                };

                if Self::is_mmio(addr) {
                    let data = match width {
                        MemWidth::Byte => vec![value as u8],
                        MemWidth::Half => match self.endian {
                            Endianness::Little => (value as u16).to_le_bytes().to_vec(),
                            Endianness::Big => (value as u16).to_be_bytes().to_vec(),
                        },
                        MemWidth::Word => match self.endian {
                            Endianness::Little => value.to_le_bytes().to_vec(),
                            Endianness::Big => value.to_be_bytes().to_vec(),
                        },
                        MemWidth::Double => {
                            return Err(Error::Emulator(
                                "doubleword mmio store immediate not supported".to_string(),
                            ));
                        }
                    };
                    return Ok(Some(VcpuExit::MmioWrite {
                        addr: addr as u64,
                        data,
                    }));
                }
                self.store_mem(addr, width, value)?;
            }
            DecodedInsn::AllocFrame { base, size } => {
                let sp = self.regs.r[base as usize];
                let ea = sp.wrapping_sub(8);
                let fp = self.regs.r[30];
                let lr = self.regs.r[31];
                let value = ((lr as u64) << 32) | fp as u64;
                self.write_u64(ea, value)?;
                let new_sp = ea.wrapping_sub(size);
                new_r[base as usize] = Some(new_sp);
                new_r[30] = Some(ea);
            }
            DecodedInsn::DeallocFrame {
                base,
                dst,
                update_lr_fp,
            } => {
                let addr = self.regs.r[base as usize];
                let value = self.read_u64(addr)?;
                let fp = (value & 0xffff_ffff) as u32;
                let lr = (value >> 32) as u32;
                if let Some(dst) = dst {
                    let idx = dst as usize;
                    if idx < 32 {
                        new_r[idx] = Some(fp);
                    }
                    if idx + 1 < 32 {
                        new_r[idx + 1] = Some(lr);
                    }
                }
                if update_lr_fp {
                    new_r[30] = Some(fp);
                    new_r[31] = Some(lr);
                }
                new_r[base as usize] = Some(addr.wrapping_add(8));
            }
            DecodedInsn::DeallocReturn {
                base,
                dst,
                pred,
                update_lr_fp,
            } => {
                if let Some(cond) = pred {
                    if !self.eval_pred(cond, new_p) {
                        return Ok(None);
                    }
                }
                let addr = self.regs.r[base as usize];
                let value = self.read_u64(addr)?;
                let fp = (value & 0xffff_ffff) as u32;
                let lr = (value >> 32) as u32;
                if let Some(dst) = dst {
                    let idx = dst as usize;
                    if idx < 32 {
                        new_r[idx] = Some(fp);
                    }
                    if idx + 1 < 32 {
                        new_r[idx + 1] = Some(lr);
                    }
                }
                if update_lr_fp {
                    new_r[30] = Some(fp);
                    new_r[31] = Some(lr);
                }
                new_r[base as usize] = Some(addr.wrapping_add(8));
                self.set_branch(branch, lr & !0x3, false)?;
            }
            DecodedInsn::Jump { offset } => {
                let target = pcrel_base.wrapping_add(offset as u32) & !0x3;
                self.set_branch(branch, target, false)?;
            }
            DecodedInsn::JumpCond {
                offset,
                pred,
                sense,
                pred_new,
            } => {
                let cond = PredCond {
                    pred,
                    sense,
                    pred_new,
                };
                if self.eval_pred(cond, new_p) {
                    let target = pcrel_base.wrapping_add(offset as u32) & !0x3;
                    self.set_branch(branch, target, false)?;
                }
            }
            DecodedInsn::JumpReg { src } => {
                let target = self.regs.r[src as usize] & !0x3;
                self.set_branch(branch, target, false)?;
            }
            DecodedInsn::JumpRegCond {
                src,
                pred,
                sense,
                pred_new,
            } => {
                let cond = PredCond {
                    pred,
                    sense,
                    pred_new,
                };
                if self.eval_pred(cond, new_p) {
                    let target = self.regs.r[src as usize] & !0x3;
                    self.set_branch(branch, target, false)?;
                }
            }
            DecodedInsn::JumpRegZero { src, kind, offset } => {
                // `if (cmp.<kind>(Rs,#0)) jump #r13:2`: a DIRECT PC-relative jump
                // conditioned on a register compare-to-zero.
                let a = self.regs.r[src as usize] as i32;
                let take = match kind {
                    CmpKind::Eq => a == 0,
                    CmpKind::Ne => a != 0,
                    CmpKind::Gte => a >= 0,
                    CmpKind::Lte => a <= 0,
                    _ => false,
                };
                if take {
                    let target = pcrel_base.wrapping_add(offset as u32) & !0x3;
                    self.set_branch(branch, target, false)?;
                }
            }
            DecodedInsn::JumpSet { dst, value, offset } => {
                let val = match value {
                    crate::backend::emulator::hexagon::decode::JumpSetSrc::Reg(reg) => {
                        self.regs.r[reg as usize]
                    }
                    crate::backend::emulator::hexagon::decode::JumpSetSrc::Imm(imm) => imm,
                };
                new_r[dst as usize] = Some(val);
                let target = pcrel_base.wrapping_add(offset as u32) & !0x3;
                self.set_branch(branch, target, false)?;
            }
            DecodedInsn::Nop => {}
            DecodedInsn::CompoundCmpJump {
                kind,
                src1,
                src2,
                write_pred,
                sense,
                new_value: _,
                offset,
            } => {
                use crate::backend::emulator::hexagon::decode::CmpJumpKind;
                // `src1` is already resolved to a real GPR by resolve_new_value
                // for the `_jumpnv` forms; read it as an in-flight value so a
                // same-packet producer is seen.
                let a = self.read_reg_with_new(src1, new_r);
                let result = match kind {
                    CmpJumpKind::Eq => a == self.regs.r[src2 as usize],
                    CmpJumpKind::Gt => (a as i32) > (self.regs.r[src2 as usize] as i32),
                    CmpJumpKind::Gtu => a > self.regs.r[src2 as usize],
                    CmpJumpKind::Lt => (a as i32) < (self.regs.r[src2 as usize] as i32),
                    CmpJumpKind::Ltu => a < self.regs.r[src2 as usize],
                    CmpJumpKind::EqImm(imm) => (a as i32) == imm,
                    CmpJumpKind::GtImm(imm) => (a as i32) > imm,
                    CmpJumpKind::GtuImm(imm) => a > imm,
                    CmpJumpKind::EqN1 => (a as i32) == -1,
                    CmpJumpKind::GtN1 => (a as i32) > -1,
                    CmpJumpKind::TstBit0 => a & 1 != 0,
                };
                // The predicate-writing (`_jump`) forms write P0/P1 with the
                // compare result; the `_jumpnv` forms write no predicate.
                if let Some(p) = write_pred {
                    new_p[p as usize] = Some(if result { 0xff } else { 0 });
                }
                // Branch when (compare result == taken sense).
                if result == sense {
                    let target = pcrel_base.wrapping_add(offset as u32) & !0x3;
                    self.set_branch(branch, target, false)?;
                }
            }
            DecodedInsn::Call { offset, pred } => {
                let take = pred.map_or(true, |cond| self.eval_pred(cond, new_p));
                if take {
                    let target = pcrel_base.wrapping_add(offset as u32) & !0x3;
                    self.set_branch(branch, target, true)?;
                }
            }
            DecodedInsn::CallReg { src, pred } => {
                let take = pred.map_or(true, |cond| self.eval_pred(cond, new_p));
                if take {
                    let target = self.regs.r[src as usize] & !0x3;
                    self.set_branch(branch, target, true)?;
                }
            }
            DecodedInsn::Cmp {
                pred,
                src1,
                src2,
                kind,
            } => {
                let a = self.regs.r[src1 as usize];
                let b = self.regs.r[src2 as usize];
                let result = match kind {
                    CmpKind::Eq => a == b,
                    CmpKind::Gt => (a as i32) > (b as i32),
                    CmpKind::Gtu => a > b,
                    CmpKind::Ne => a != b,
                    CmpKind::Lte => (a as i32) <= (b as i32),
                    CmpKind::Lteu => a <= b,
                    CmpKind::Gte => (a as i32) >= (b as i32),
                };
                new_p[pred as usize] = Some(if result { 0xff } else { 0 });
            }
            DecodedInsn::CmpImm {
                pred,
                src,
                imm,
                kind,
                unsigned,
            } => {
                let a = self.regs.r[src as usize];
                let result = if unsigned {
                    let b = imm as u32;
                    match kind {
                        CmpKind::Eq => a == b,
                        CmpKind::Gt => (a as i32) > (b as i32),
                        CmpKind::Gtu => a > b,
                        CmpKind::Ne => a != b,
                        CmpKind::Lte => (a as i32) <= (b as i32),
                        CmpKind::Lteu => a <= b,
                        CmpKind::Gte => (a as i32) >= (b as i32),
                    }
                } else {
                    let b = imm as i32;
                    match kind {
                        CmpKind::Eq => (a as i32) == b,
                        CmpKind::Gt => (a as i32) > b,
                        CmpKind::Gtu => a > b as u32,
                        CmpKind::Ne => (a as i32) != b,
                        CmpKind::Lte => (a as i32) <= b,
                        CmpKind::Lteu => a <= b as u32,
                        CmpKind::Gte => (a as i32) >= b,
                    }
                };
                new_p[pred as usize] = Some(if result { 0xff } else { 0 });
            }
            DecodedInsn::Mul { dst, src1, src2 } => {
                let val = self.regs.r[src1 as usize].wrapping_mul(self.regs.r[src2 as usize]);
                new_r[dst as usize] = Some(val);
            }
            DecodedInsn::ShiftImm {
                dst,
                src,
                kind,
                amount,
            } => {
                let val = self.regs.r[src as usize];
                let shamt = (amount & 0x1f) as u32;
                let result = match kind {
                    ShiftKind::Lsl => val.wrapping_shl(shamt),
                    ShiftKind::Lsr => val.wrapping_shr(shamt),
                    ShiftKind::Asr => ((val as i32) >> shamt) as u32,
                };
                new_r[dst as usize] = Some(result);
            }
            DecodedInsn::ShiftReg {
                dst,
                src,
                amt,
                kind,
            } => {
                let val = self.regs.r[src as usize];
                let rt = self.regs.r[amt as usize];
                new_r[dst as usize] = Some(hex_reg_shift32(val, rt, kind));
            }
            DecodedInsn::TfrCrR { dst, src } => {
                new_r[dst as usize] = Some(self.regs.control(src as usize));
            }
            DecodedInsn::TfrRrCr { dst, src } => {
                let val = self.regs.r[src as usize];
                self.regs.set_control(dst as usize, val);
            }
            DecodedInsn::TfrCrRPair { dst, src } => {
                // Rdd = Css: read the even/odd control-register pair into a GPR pair.
                new_r[dst as usize] = Some(self.regs.control(src as usize));
                new_r[dst as usize + 1] = Some(self.regs.control(src as usize + 1));
            }
            DecodedInsn::TfrRrCrPair { dst, src } => {
                // Cdd = Rss: write a GPR pair into the control-register pair.
                let lo = self.regs.r[src as usize];
                let hi = self.regs.r[src as usize + 1];
                self.regs.set_control(dst as usize, lo);
                self.regs.set_control(dst as usize + 1, hi);
            }
            DecodedInsn::DcZero { base } => {
                // Zero the 32-byte cache line containing Rs (aligned down to 32).
                let ea = self.regs.r[base as usize] & !31;
                self.mem
                    .write_slice(&[0u8; 32], GuestAddress(ea as u64))
                    .map_err(|e| Error::Emulator(format!("dczeroa write: {e}")))?;
            }
            DecodedInsn::LoopStartReg {
                loop_id,
                start_offset,
                count_reg,
                lpcfg,
            } => {
                let count = self.regs.r[count_reg as usize];
                let sa = pcrel_base.wrapping_add(start_offset as u32) & !0x3;
                if loop_id == 0 {
                    self.regs.c[0] = sa;
                    self.regs.c[1] = count;
                } else {
                    self.regs.c[2] = sa;
                    self.regs.c[3] = count;
                }
                if let Some(n) = lpcfg {
                    self.set_lpcfg(n as u32);
                    new_p[3] = Some(0);
                }
            }
            DecodedInsn::LoopStartImm {
                loop_id,
                start_offset,
                count,
                lpcfg,
            } => {
                let sa = pcrel_base.wrapping_add(start_offset as u32) & !0x3;
                if loop_id == 0 {
                    self.regs.c[0] = sa;
                    self.regs.c[1] = count;
                } else {
                    self.regs.c[2] = sa;
                    self.regs.c[3] = count;
                }
                if let Some(n) = lpcfg {
                    self.set_lpcfg(n as u32);
                    new_p[3] = Some(0);
                }
            }
            DecodedInsn::Trap0 => {
                return Ok(Some(VcpuExit::Shutdown));
            }
            // Read-modify-write memory op: mem[Rs+#off] OP= (Rt | #imm | bit).
            DecodedInsn::MemOp {
                base,
                offset,
                width,
                op,
                src,
            } => {
                let ea = self.regs.r[base as usize].wrapping_add(offset as u32);
                let cur = self.load_mem(ea, width, MemSign::Unsigned)?;
                let srcval = match src {
                    MemOpSrc::Reg(reg) => self.regs.r[reg as usize],
                    MemOpSrc::Imm(value) => value,
                };
                let result = match op {
                    MemOpKind::Add => cur.wrapping_add(srcval),
                    MemOpKind::Sub => cur.wrapping_sub(srcval),
                    MemOpKind::And => cur & srcval,
                    MemOpKind::Or => cur | srcval,
                    MemOpKind::ClrBit => cur & !(1u32 << (srcval & 0x1f)),
                    MemOpKind::SetBit => cur | (1u32 << (srcval & 0x1f)),
                };
                self.store_mem(ea, width, result)?;
            }
            // HVX vector load: Vd = vmem(base + offset) (buffered to new_v).
            DecodedInsn::VLoad {
                dst,
                base,
                offset,
                post_inc,
                post_inc_mod,
                aligned,
                commit,
                pred,
            } => {
                // Scalar-predicated load: CANCEL (no register write, no
                // post-increment) when the predicate is false.
                let do_load = match pred {
                    Some(cond) => self.eval_pred(cond, new_p),
                    None => true,
                };
                if do_load {
                    let mut ea = self.regs.r[base as usize].wrapping_add(offset as u32);
                    if aligned {
                        ea &= !127;
                    }
                    // Read always (a `.tmp` load still faults on a bad address). A
                    // normal/`.cur` load buffers into `new_v` (committed at packet
                    // end); a `.tmp` load forwards into the scratch buffer so a
                    // same-packet consumer (e.g. vhist) sees it, but it is never
                    // committed architecturally.
                    let vec = self.read_vec(ea)?;
                    if commit {
                        self.new_v[dst as usize] = Some(vec);
                        self.last_v_produced = Some(dst);
                    } else {
                        self.new_v_tmp[dst as usize] = Some(vec);
                    }
                    // Post-increment: `pi` by the (vector-unit) immediate, `ppu` by
                    // the modifier register Mu (full byte value).
                    let inc = post_inc
                        .map(|i| i as u32)
                        .or_else(|| post_inc_mod.map(|m| self.modifier(m)));
                    if let Some(inc) = inc {
                        new_r[base as usize] = Some(self.regs.r[base as usize].wrapping_add(inc));
                    }
                }
            }
            // HVX vector store: vmem(base + offset) = Vs.
            DecodedInsn::VStore {
                src,
                base,
                offset,
                post_inc,
                post_inc_mod,
                aligned,
                pred,
                qmask,
                from_gather,
                srls,
            } => {
                // Scalar-predicated store: CANCEL (no write, no post-increment)
                // when the predicate is false. A byte-masked (qmask) store always
                // executes; it only masks which bytes change.
                let do_store = match pred {
                    Some(cond) => self.eval_pred(cond, new_p),
                    None => true,
                };
                if do_store {
                    let mut ea = self.regs.r[base as usize].wrapping_add(offset as u32);
                    if aligned {
                        ea &= !127;
                    }
                    // `srls` (scatter_release) is a barrier with no vector source:
                    // it performs no memory write (only the post-increment, below).
                    let mut bytes = if srls {
                        None
                    } else if from_gather {
                        // New-value vector store `vmem(...) = V.new`. Normally the
                        // source is the vector produced earlier in this packet
                        // (`new_v[last producer]`). For the gather idiom
                        // (`vmem=vtmp.new`, no architectural vector producer), it
                        // is the gather scratch: only the gathered (valid) bytes
                        // are written, the rest of memory is preserved (the gather
                        // temp is seeded from this memory on hardware).
                        if let Some(p) = self.last_v_produced {
                            let v = self.new_v[p as usize].unwrap_or(self.regs.v[p as usize]);
                            Some(vec_to_bytes(&v))
                        } else {
                            let (data, valid) =
                                self.gather_tmp.unwrap_or(([0u8; 128], [false; 128]));
                            let cur = vec_to_bytes(&self.read_vec(ea)?);
                            let mut b = cur;
                            for i in 0..128 {
                                if valid[i] {
                                    b[i] = data[i];
                                }
                            }
                            Some(b)
                        }
                    } else {
                        Some(vec_to_bytes(&self.regs.v[src as usize]))
                    };
                    if let (Some(bytes), Some((q, sense))) = (bytes.as_mut(), qmask) {
                        // Read-modify-write: keep existing bytes where the Q mask
                        // doesn't select. The Qv operand reads the OLD predicate
                        // (it is not a `.new` operand).
                        let qv = self.regs.q[q as usize];
                        let cur = vec_to_bytes(&self.read_vec(ea)?);
                        for i in 0..128 {
                            let bit = (qv[i / 32] >> (i % 32)) & 1 != 0;
                            if bit != sense {
                                bytes[i] = cur[i];
                            }
                        }
                    }
                    if let Some(bytes) = bytes {
                        self.write_vec(ea, &bytes_to_vec(&bytes))?;
                    }
                    // Post-increment: `pi` by the (vector-unit) immediate, `ppu` by
                    // the modifier register Mu. The `srls` scatter-release barrier
                    // is a pure ordering hint and does NOT write back the base, so
                    // its `_pi`/`_ppu` forms perform no post-increment (matching
                    // qemu-hexagon).
                    let inc = if srls {
                        None
                    } else {
                        post_inc
                            .map(|i| i as u32)
                            .or_else(|| post_inc_mod.map(|m| self.modifier(m)))
                    };
                    if let Some(inc) = inc {
                        new_r[base as usize] = Some(self.regs.r[base as usize].wrapping_add(inc));
                    }
                }
            }
            // HVX V65 scatter: store/accumulate elements into the region.
            DecodedInsn::VScatter {
                base,
                modsel,
                offsets,
                data,
                esz,
                off_pair,
                add,
                pred,
            } => {
                self.exec_scatter(base, modsel, offsets, data, esz, off_pair, add, pred)?;
            }
            // HVX V65 gather: build the gathered vector into the per-packet scratch
            // (`gather_tmp`), consumed by the paired `vmem=vtmp.new` store. The
            // gather is also pre-run at packet start (the assembler encodes it
            // AFTER its consuming store); re-running here is idempotent.
            DecodedInsn::VGather {
                base,
                modsel,
                offsets,
                esz,
                off_pair,
                pred,
            } => {
                self.gather_tmp =
                    Some(self.exec_gather(base, modsel, offsets, esz, off_pair, pred)?);
            }
            // Absolute value: Rd = |Rs|, with optional saturation
            DecodedInsn::Abs { dst, src, sat } => {
                let val = self.regs.r[src as usize] as i32;
                let result = if sat {
                    // Saturating absolute value
                    // Special case: 0x80000000 saturates to 0x7fffffff
                    if val == i32::MIN {
                        // Set overflow flag in USR (C8)
                        self.regs.c[8] |= 1; // OVF bit
                        i32::MAX as u32
                    } else {
                        val.abs() as u32
                    }
                } else {
                    // Non-saturating: wraps on MIN_VALUE
                    val.wrapping_abs() as u32
                };
                new_r[dst as usize] = Some(result);
            }
            // Saturating negation: Rd = sat(-Rs)
            DecodedInsn::NegSat { dst, src } => {
                let val = self.regs.r[src as usize] as i32;
                // Special case: -0x80000000 saturates to 0x7fffffff
                let result = if val == i32::MIN {
                    // Set overflow flag in USR (C8)
                    self.regs.c[8] |= 1; // OVF bit
                    i32::MAX as u32
                } else {
                    (-val) as u32
                };
                new_r[dst as usize] = Some(result);
            }
            // Signed maximum: Rd = max(Rs, Rt)
            DecodedInsn::Max { dst, src1, src2 } => {
                let a = self.regs.r[src1 as usize] as i32;
                let b = self.regs.r[src2 as usize] as i32;
                let result = if a > b { a } else { b };
                new_r[dst as usize] = Some(result as u32);
            }
            // Unsigned maximum: Rd = maxu(Rs, Rt)
            DecodedInsn::Maxu { dst, src1, src2 } => {
                let a = self.regs.r[src1 as usize];
                let b = self.regs.r[src2 as usize];
                let result = if a > b { a } else { b };
                new_r[dst as usize] = Some(result);
            }
            // Signed minimum: Rd = min(Rs, Rt)
            DecodedInsn::Min { dst, src1, src2 } => {
                let a = self.regs.r[src1 as usize] as i32;
                let b = self.regs.r[src2 as usize] as i32;
                let result = if a < b { a } else { b };
                new_r[dst as usize] = Some(result as u32);
            }
            // Unsigned minimum: Rd = minu(Rs, Rt)
            DecodedInsn::Minu { dst, src1, src2 } => {
                let a = self.regs.r[src1 as usize];
                let b = self.regs.r[src2 as usize];
                let result = if a < b { a } else { b };
                new_r[dst as usize] = Some(result);
            }
            DecodedInsn::StoreNew { .. } => {
                // New-value stores are resolved to a regular Store by the packet
                // driver before reaching here; seeing one is a logic error.
                return Err(Error::Emulator("unresolved new-value store".to_string()));
            }
            DecodedInsn::LoadLocked { dst, base, width } => {
                // Plain word/dword load at `Rs+0` that also sets the LL
                // reservation on the accessed address.
                let addr = self.regs.r[base as usize];
                if Self::is_mmio(addr) {
                    return Err(Error::Emulator(
                        "atomic load from mmio not supported".to_string(),
                    ));
                }
                if width == MemWidth::Double {
                    let even = dst & !1;
                    let odd = even.wrapping_add(1);
                    if odd >= 32 {
                        return Err(Error::Emulator(
                            "invalid register pair for doubleword load".to_string(),
                        ));
                    }
                    let val = self.read_u64(addr)?;
                    new_r[even as usize] = Some(val as u32);
                    new_r[odd as usize] = Some((val >> 32) as u32);
                } else {
                    new_r[dst as usize] = Some(self.read_u32(addr)?);
                }
                self.lock_addr = Some(addr);
            }
            DecodedInsn::StoreCond {
                src,
                base,
                width,
                success_pred,
            } => {
                // Store-conditional / store-release at `Rs+0`.
                let addr = self.regs.r[base as usize];
                if Self::is_mmio(addr) {
                    return Err(Error::Emulator(
                        "atomic store to mmio not supported".to_string(),
                    ));
                }
                // A `_locked` store-conditional succeeds only when the LL
                // reservation matches this address (a bare SC fails). Release
                // stores (`success_pred == None`) always store unconditionally.
                let succeed = match success_pred {
                    Some(_) => self.lock_addr == Some(addr),
                    None => true,
                };
                // Any store-conditional attempt clears the reservation.
                if success_pred.is_some() {
                    self.lock_addr = None;
                }
                if succeed {
                    if width == MemWidth::Double {
                        let even = src & !1;
                        let odd = even.wrapping_add(1);
                        if odd >= 32 {
                            return Err(Error::Emulator(
                                "invalid register pair for doubleword store".to_string(),
                            ));
                        }
                        let combined = ((self.regs.r[odd as usize] as u64) << 32)
                            | self.regs.r[even as usize] as u64;
                        self.write_u64(addr, combined)?;
                    } else {
                        self.store_mem(addr, width, self.regs.r[src as usize])?;
                    }
                }
                // The `_locked` forms set Pd to the success result (TRUE on a held
                // reservation, FALSE otherwise); release stores set no predicate.
                if let Some(pd) = success_pred {
                    new_p[pd as usize] = Some(if succeed { 0xff } else { 0 });
                }
            }
            DecodedInsn::Vsplice {
                dst,
                src_low,
                src_high,
                amount,
            } => {
                // Rdd = Rtt << N*8 | zxt(N*8)(Rss): low N bytes from Rss, high
                // (8-N) bytes from Rtt. N is #u3 or Pu&7 (range 0..=7, so the
                // shift is always < 64 and well-defined).
                let n = match amount {
                    SpliceAmount::Imm(v) => (v & 0x7) as u32,
                    SpliceAmount::Pred(p) => (self.regs.p[p as usize] & 0x7) as u32,
                };
                let bits = n * 8;
                let rss_even = src_low & !1;
                let rss_odd = rss_even.wrapping_add(1);
                let rtt_even = src_high & !1;
                let rtt_odd = rtt_even.wrapping_add(1);
                let rss = ((self.regs.r[rss_odd as usize] as u64) << 32)
                    | self.regs.r[rss_even as usize] as u64;
                let rtt = ((self.regs.r[rtt_odd as usize] as u64) << 32)
                    | self.regs.r[rtt_even as usize] as u64;
                let low_mask = if bits == 0 { 0 } else { (1u64 << bits) - 1 };
                let result = (rtt << bits) | (rss & low_mask);
                let even = dst & !1;
                let odd = even.wrapping_add(1);
                new_r[even as usize] = Some(result as u32);
                new_r[odd as usize] = Some((result >> 32) as u32);
            }
            DecodedInsn::Unknown(word) => {
                // Fall through to the direct opcode-dispatch semantic layer,
                // which handles instructions not modelled by the DecodedInsn IR.
                if !self.try_sem(word, immext, new_r, new_p)? {
                    return Err(Error::Emulator(format!(
                        "unknown hexagon instruction 0x{word:08x}"
                    )));
                }
            }
        }

        Ok(None)
    }

    /// Attempt to execute `word` via the direct opcode-dispatch semantic layer.
    /// Returns `Ok(true)` if a handler ran, `Ok(false)` if the opcode is not yet
    /// modelled there (caller treats it as a genuine decode failure).
    fn try_sem(
        &mut self,
        word: u32,
        immext: Option<u32>,
        new_r: &mut [Option<u32>; 32],
        new_p: &mut [Option<u8>; 4],
    ) -> Result<bool> {
        let dop = match super::opcode::decode_word(word) {
            Some(d) => d,
            None => return Ok(false),
        };
        let (usr_or, v_writes, q_writes) = {
            let mut ctx = super::sem::SemCtx {
                regs: &self.regs,
                new_r: &mut *new_r,
                new_p: &mut *new_p,
                vnew: &self.new_v,
                vtmp: &self.new_v_tmp,
                qnew: &self.new_q,
                v_writes: Vec::new(),
                q_writes: Vec::new(),
                immext,
                usr_or: 0,
            };
            if !super::sem::dispatch(&dop, &mut ctx) {
                return Ok(false);
            }
            (ctx.usr_or, ctx.v_writes, ctx.q_writes)
        };
        for (i, val) in v_writes {
            self.new_v[i as usize] = Some(val);
            // Track the in-packet vector producer for new-value vector stores.
            self.last_v_produced = Some(i);
        }
        for (i, val) in q_writes {
            self.new_q[i as usize] = Some(val);
        }
        if usr_or != 0 {
            self.regs.c[8] |= usr_or;
        }
        Ok(true)
    }

    fn finish_packet(
        &mut self,
        pc: u32,
        packet_pc: u32,
        new_r: [Option<u32>; 32],
        new_p: [Option<u8>; 4],
        branch: Option<BranchTarget>,
        first_parse: Option<u32>,
        second_parse: Option<u32>,
    ) {
        self.commit_packet(&new_r, &new_p);
        let packet_end = pc;

        let (loop0_end, loop1_end) = match (first_parse, second_parse) {
            (Some(first), Some(second)) => {
                let loop0 = first == 0b10 && second != 0;
                let loop1 = second == 0b10 && first != 0;
                (loop0, loop1)
            }
            _ => (false, false),
        };

        // Software-pipelined loop predicate (P3) maintenance: at every `endloop0`
        // (and the loop0 half of `endloop01`), if USR.LPCFG is non-zero then
        // when it reaches 1 P3 is set to 0xff, after which LPCFG is decremented.
        // This runs regardless of whether the back-edge is taken (`sp*loop0`
        // presets P3=0 at setup; the kernel predicate goes true on the final
        // prologue iterations).
        if loop0_end {
            let lpcfg = self.get_lpcfg();
            if lpcfg != 0 {
                if lpcfg == 1 {
                    self.regs.set_predicate(3, 0xff);
                }
                self.set_lpcfg(lpcfg - 1);
            }
        }

        if let Some(branch) = branch {
            if branch.is_call {
                self.regs.r[31] = packet_end;
            }
            self.set_pc(branch.target);
        } else if loop0_end && self.regs.c[1] > 1 {
            self.regs.c[1] = self.regs.c[1].wrapping_sub(1);
            self.set_pc(self.regs.c[0]);
        } else if loop1_end && self.regs.c[3] > 1 {
            self.regs.c[3] = self.regs.c[3].wrapping_sub(1);
            self.set_pc(self.regs.c[2]);
        } else {
            self.set_pc(packet_end);
        }
    }

    fn save_packet_state(
        &mut self,
        packet_pc: u32,
        pc: u32,
        immext: Option<u32>,
        new_r: [Option<u32>; 32],
        new_p: [Option<u8>; 4],
        branch: Option<BranchTarget>,
        inst_index: usize,
        first_parse: Option<u32>,
        second_parse: Option<u32>,
        pending_subinsn: Option<DecodedSub>,
        pending_end: bool,
    ) {
        self.pending_packet = Some(PacketState {
            packet_pc,
            pc,
            immext,
            new_r,
            new_p,
            branch,
            inst_index,
            first_parse,
            second_parse,
            pending_subinsn,
            pending_end,
        });
        self.set_pc(pc);
    }

    fn step_packet(&mut self) -> Result<Option<VcpuExit>> {
        let resuming = self.pending_packet.is_some();
        let mut state = self
            .pending_packet
            .take()
            .unwrap_or_else(|| PacketState::new(self.regs.pc()));
        // Fresh packet: clear the in-flight HVX vector write buffers (they
        // persist across an MMIO suspend/resume of the same packet).
        if !resuming {
            self.new_v = [None; 32];
            self.new_q = [None; 4];
            self.new_v_tmp = [None; 32];
            self.gather_tmp = None;
            self.last_v_produced = None;
            // Pre-execute vector `.tmp` loads so their data is visible to a
            // same-packet consumer (e.g. vhist, which the assembler encodes
            // BEFORE the producing `.tmp` load). Read-only here: the producing
            // load still runs normally in the main loop (post-increment, etc.).
            let mut scan_pc = state.pc;
            for _ in 0..8 {
                let w = match self.fetch_word(scan_pc) {
                    Ok(w) => w,
                    Err(_) => break,
                };
                let pb = (w >> 14) & 0x3;
                match decode(w, None, self._isa).insn {
                    DecodedInsn::VLoad {
                        dst,
                        base,
                        offset,
                        aligned,
                        commit: false,
                        ..
                    } => {
                        let mut ea = self.regs.r[base as usize].wrapping_add(offset as u32);
                        if aligned {
                            ea &= !127;
                        }
                        if let Ok(vec) = self.read_vec(ea) {
                            self.new_v_tmp[dst as usize] = Some(vec);
                        }
                    }
                    // Pre-run the gather: its result (`gather_tmp`) feeds the
                    // paired `vmem=vtmp.new` store, which the assembler encodes
                    // BEFORE the gather. The gather instruction itself still runs
                    // in the main loop (recomputing the same scratch).
                    DecodedInsn::VGather {
                        base,
                        modsel,
                        offsets,
                        esz,
                        off_pair,
                        pred,
                    } => {
                        if let Ok(vec) =
                            self.exec_gather(base, modsel, offsets, esz, off_pair, pred)
                        {
                            self.gather_tmp = Some(vec);
                        }
                    }
                    _ => {}
                }
                scan_pc = scan_pc.wrapping_add(4);
                if pb == 0b11 || pb == 0b00 {
                    break;
                }
            }
        }

        let mut pc = state.pc;
        let packet_pc = state.packet_pc;
        let mut immext = state.immext;
        let mut new_r = state.new_r;
        let mut new_p = state.new_p;
        let mut branch = state.branch;
        let mut inst_index = state.inst_index;
        let mut first_parse = state.first_parse;
        let mut second_parse = state.second_parse;
        let mut pending_subinsn = state.pending_subinsn;
        let mut pending_end = state.pending_end;
        // GPR producers (dest registers) of this packet's instructions, in
        // execution order — used to resolve new-value stores (`Nt8`). Local to
        // this call: a packet that suspends for MMIO mid-flight won't carry the
        // list across resume, but a new-value store after an MMIO access in the
        // same packet does not occur in practice.
        let mut producers: Vec<u8> = Vec::new();

        loop {
            if let Some(sub) = pending_subinsn.take() {
                if let Some(exit) = self.execute_insn(
                    sub.insn,
                    packet_pc,
                    packet_pc,
                    None,
                    &mut new_r,
                    &mut new_p,
                    &mut branch,
                )? {
                    if matches!(exit, VcpuExit::Shutdown) {
                        self.finish_packet(
                            pc,
                            packet_pc,
                            new_r,
                            new_p,
                            branch,
                            first_parse,
                            second_parse,
                        );
                        return Ok(Some(exit));
                    }

                    self.save_packet_state(
                        packet_pc,
                        pc,
                        immext,
                        new_r,
                        new_p,
                        branch,
                        inst_index,
                        first_parse,
                        second_parse,
                        None,
                        pending_end,
                    );
                    return Ok(Some(exit));
                }

                if pending_end {
                    self.finish_packet(
                        pc,
                        packet_pc,
                        new_r,
                        new_p,
                        branch,
                        first_parse,
                        second_parse,
                    );
                    return Ok(None);
                }

                continue;
            }

            let word = self.fetch_word(pc)?;
            let parse = (word >> 14) & 0x3;

            if inst_index == 0 {
                first_parse = Some(parse);
            } else if inst_index == 1 {
                second_parse = Some(parse);
            }
            inst_index += 1;

            if parse == 0 {
                let (slot1, slot0) = decode_duplex(word, self._isa).ok_or_else(|| {
                    Error::Emulator(format!(
                        "unknown duplex instruction 0x{word:08x} at pc=0x{pc:08x}"
                    ))
                })?;
                self.ensure_isa_supports(word, &slot1.insn, slot1.opcode)?;
                self.ensure_isa_supports(word, &slot0.insn, slot0.opcode)?;

                if let Some(exit) = self.execute_insn(
                    slot1.insn,
                    packet_pc,
                    pc,
                    None,
                    &mut new_r,
                    &mut new_p,
                    &mut branch,
                )? {
                    if matches!(exit, VcpuExit::Shutdown) {
                        self.finish_packet(
                            pc.wrapping_add(4),
                            packet_pc,
                            new_r,
                            new_p,
                            branch,
                            first_parse,
                            second_parse,
                        );
                        return Ok(Some(exit));
                    }

                    pc = pc.wrapping_add(4);
                    pending_end = true;
                    pending_subinsn = Some(slot0);
                    self.save_packet_state(
                        packet_pc,
                        pc,
                        immext,
                        new_r,
                        new_p,
                        branch,
                        inst_index,
                        first_parse,
                        second_parse,
                        pending_subinsn,
                        pending_end,
                    );
                    return Ok(Some(exit));
                }

                if let Some(exit) = self.execute_insn(
                    slot0.insn,
                    packet_pc,
                    pc,
                    None,
                    &mut new_r,
                    &mut new_p,
                    &mut branch,
                )? {
                    if matches!(exit, VcpuExit::Shutdown) {
                        self.finish_packet(
                            pc.wrapping_add(4),
                            packet_pc,
                            new_r,
                            new_p,
                            branch,
                            first_parse,
                            second_parse,
                        );
                        return Ok(Some(exit));
                    }

                    pc = pc.wrapping_add(4);
                    pending_end = true;
                    self.save_packet_state(
                        packet_pc,
                        pc,
                        immext,
                        new_r,
                        new_p,
                        branch,
                        inst_index,
                        first_parse,
                        second_parse,
                        None,
                        pending_end,
                    );
                    return Ok(Some(exit));
                }

                pc = pc.wrapping_add(4);
                pending_end = true;
                break;
            }

            let cur_immext = immext;
            let decoded = decode(word, immext, self._isa);
            immext = None;
            self.ensure_isa_supports(word, &decoded.insn, decoded.opcode)?;

            match decoded.insn {
                DecodedInsn::ImmExt { value } => {
                    immext = Some(value);
                    pc = pc.wrapping_add(4);
                    if parse == 0x3 {
                        self.finish_packet(
                            pc,
                            packet_pc,
                            new_r,
                            new_p,
                            branch,
                            first_parse,
                            second_parse,
                        );
                        return Ok(None);
                    }
                    continue;
                }
                insn => {
                    // Resolve a new-value store's `Nt8` against the packet's
                    // GPR producers before executing it.
                    let insn = resolve_new_value(insn, &producers);
                    let before = producer_mask(&new_r);
                    if let Some(exit) = self.execute_insn(
                        insn,
                        packet_pc,
                        pc,
                        cur_immext,
                        &mut new_r,
                        &mut new_p,
                        &mut branch,
                    )? {
                        if matches!(exit, VcpuExit::Shutdown) {
                            self.finish_packet(
                                pc.wrapping_add(4),
                                packet_pc,
                                new_r,
                                new_p,
                                branch,
                                first_parse,
                                second_parse,
                            );
                            return Ok(Some(exit));
                        }

                        pc = pc.wrapping_add(4);
                        pending_end = parse == 0x3;
                        self.save_packet_state(
                            packet_pc,
                            pc,
                            immext,
                            new_r,
                            new_p,
                            branch,
                            inst_index,
                            first_parse,
                            second_parse,
                            None,
                            pending_end,
                        );
                        return Ok(Some(exit));
                    }
                    record_producer(&new_r, before, &mut producers);
                }
            }

            pc = pc.wrapping_add(4);
            if parse == 0x3 {
                break;
            }
        }

        self.finish_packet(
            pc,
            packet_pc,
            new_r,
            new_p,
            branch,
            first_parse,
            second_parse,
        );
        Ok(None)
    }
}

impl VCpu for HexagonVcpu {
    fn run(&mut self) -> Result<VcpuExit> {
        if self.halted {
            return Ok(VcpuExit::Hlt);
        }
        let mut iterations = 0u64;
        loop {
            iterations += 1;
            if iterations > MAX_RUN_ITERATIONS {
                return Err(Error::Emulator(format!(
                    "exceeded {} iterations at pc=0x{:08x}",
                    MAX_RUN_ITERATIONS,
                    self.regs.pc()
                )));
            }

            if let Some(exit) = self.step_packet()? {
                return Ok(exit);
            }
        }
    }

    fn get_state(&self) -> Result<CpuState> {
        Ok(CpuState::hexagon(self.regs.clone()))
    }

    fn set_state(&mut self, state: &CpuState) -> Result<()> {
        let state = match state {
            CpuState::Hexagon(state) => state,
            _ => {
                return Err(Error::Emulator(
                    "expected hexagon state for hexagon vCPU".to_string(),
                ));
            }
        };
        self.regs = state.regs.clone();
        Ok(())
    }

    fn complete_io_in(&mut self, data: &[u8]) {
        if let Some(pending) = self.pending_mmio.take() {
            let val = match pending.size {
                1 if data.len() >= 1 => {
                    let raw = data[0] as u32;
                    if pending.signed {
                        (raw as i8 as i32) as u32
                    } else {
                        raw
                    }
                }
                2 if data.len() >= 2 => {
                    let raw = match self.endian {
                        Endianness::Little => u16::from_le_bytes([data[0], data[1]]) as u32,
                        Endianness::Big => u16::from_be_bytes([data[0], data[1]]) as u32,
                    };
                    if pending.signed {
                        (raw as i16 as i32) as u32
                    } else {
                        raw
                    }
                }
                4 if data.len() >= 4 => match self.endian {
                    Endianness::Little => u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
                    Endianness::Big => u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
                },
                _ => return,
            };

            if let Some(packet) = self.pending_packet.as_mut() {
                packet.new_r[pending.dst as usize] = Some(val);
            } else {
                self.regs.r[pending.dst as usize] = val;
            }
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
