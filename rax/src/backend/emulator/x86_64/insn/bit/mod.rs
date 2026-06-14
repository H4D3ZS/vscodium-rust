//! Bit manipulation instructions: BT, BTS, BTR, BTC, BSF, BSR, POPCNT, LZCNT, TZCNT.

mod count;
mod scan;
mod test;

// Re-export all instruction functions
pub use count::*;
pub use scan::*;
pub use test::*;
