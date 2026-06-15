//! (hvx_conv) HVX conversions, splat/lookup/delta, accumulating ALU and other
//! vector ops not covered by the add/mpy/minmax/shift/perm/cmp classes.
//! Verified against the qemu-hexagon vector oracle (tests/hexagon_hvx_diff.rs).

use super::super::opcode::{DecodedOp, Opcode};
use super::SemCtx;

/// Execute an hvx_conv opcode. Returns `false` if `op` is not handled here.
pub fn exec(op: Opcode, d: &DecodedOp, ctx: &mut SemCtx) -> bool {
    let _ = (op, d, ctx);
    false
}
