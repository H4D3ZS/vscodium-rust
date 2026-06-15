//! x87 FPU instruction implementations.

mod escape_d8;
mod escape_d9;
mod escape_da;
mod escape_db;
mod escape_dc;
mod escape_dd;
mod escape_de;
mod escape_df;
pub mod helpers;

// Re-export escape functions
pub use escape_d8::escape_d8;
pub use escape_d9::escape_d9;
pub use escape_da::escape_da;
pub use escape_db::escape_db;
pub use escape_dc::escape_dc;
pub use escape_dd::escape_dd;
pub use escape_de::escape_de;
pub use escape_df::escape_df;

// Re-export public helper functions
pub use helpers::{f64_to_f80_pub, f80_to_f64_pub};
