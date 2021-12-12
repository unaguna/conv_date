//! Provide tables for time system conversion.
mod taiutc;
mod utctai;
pub use taiutc::{DiffTaiUtc, TaiUtcTable};
pub use utctai::{DiffUtcTai, UtcTaiTable};
