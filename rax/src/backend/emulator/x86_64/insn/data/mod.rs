//! Data movement instructions: MOV, LEA, PUSH, POP, XCHG, MOVZX, MOVSX, XADD, CMPXCHG.

mod atomic;
mod bswap;
mod frame;
mod lea;
mod mov;
mod movdir;
mod movx;
mod stack;
mod xchg;

// Re-export GPR instruction functions
pub use atomic::*;
pub use bswap::*;
pub use frame::*;
pub use lea::*;
pub use mov::*;
pub use movdir::*;
pub use movx::*;
pub use stack::*;
pub use xchg::*;
