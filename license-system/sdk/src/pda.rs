use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::error::Result;

pub const PROGRAM_ID: &str = "5pXEX8z1aTSnm7jCKqvJCXezczKPVuPQif2BZh5u5Axq";

pub fn program_id() -> Result<Pubkey> {
    Pubkey::from_str(PROGRAM_ID).map_err(|e| crate::error::SdkError::InvalidInput(format!("Invalid program ID: {}", e)).into())
}

pub fn derive_license_pda(owner: &Pubkey) -> (Pubkey, u8) {
    let pid = program_id().unwrap();
    Pubkey::find_program_address(&[b"license", owner.as_ref()], &pid)
}
