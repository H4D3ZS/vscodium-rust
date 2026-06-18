//! Logic instructions: AND, OR, XOR, TEST, NOT.

mod and;
mod group3;
mod or;
mod test;
mod xor;

// Re-export all instruction functions
pub use and::*;
pub use group3::*;
pub use or::*;
pub use test::*;
pub use xor::*;
