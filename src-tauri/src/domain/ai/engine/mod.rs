//! AI engine: split from the 7.2K-LOC ai_engine.rs. One struct (`Sentient`),
//! impl blocks distributed across submodules.

pub mod autonomous;
pub mod prompt;
pub mod providers;
pub mod sentient;
pub mod streaming;
pub mod types;

pub use sentient::*;
pub use types::*;
