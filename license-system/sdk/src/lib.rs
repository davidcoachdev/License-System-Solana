pub mod client;
pub mod error;
pub mod pda;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

pub use client::LicenseClient;
pub use error::{Result, SdkError};
pub use pda::{derive_license_pda, program_id, PROGRAM_ID};
pub use types::{License, LicenseStatus};

pub const GRACE_PERIOD_DAYS: i64 = 7;
pub const MAX_PRODUCT_ID_LEN: usize = 64;
