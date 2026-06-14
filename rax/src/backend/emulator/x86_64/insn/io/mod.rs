//! I/O instructions: IN, OUT, INSB, INSW, OUTSB, OUTSW.

mod port;
mod string;

// Re-export all instruction functions
pub use port::*;
pub use string::*;
