//! Arithmetic instructions: ADD, SUB, ADC, SBB, CMP, IMUL, and more.

mod add;
mod bcd;
mod cmp;
mod extend;
mod group1;
mod mul;
mod sub;

// Re-export all instruction functions
pub use add::*;
pub use bcd::*;
pub use cmp::*;
pub use extend::*;
pub use group1::*;
pub use mul::*;
pub use sub::*;
