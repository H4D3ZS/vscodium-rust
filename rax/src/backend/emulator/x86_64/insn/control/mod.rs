//! Control flow instructions: JMP, CALL, RET, Jcc, SETcc, CMOVcc, LOOP, INT.

mod call;
mod cmov;
mod groups;
mod int;
mod jump;
mod loop_;
mod setcc;
mod xlat;

// Re-export all instruction functions
pub use call::*;
pub use cmov::*;
pub use groups::*;
pub use int::*;
pub use jump::*;
pub use loop_::*;
pub use setcc::*;
pub use xlat::*;
