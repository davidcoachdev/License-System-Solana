use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub owner: Pubkey,
    pub product_id: String,
    pub expires_at: i64,
    pub is_revoked: bool,
}

impl License {
    pub fn is_active(&self, now: i64) -> bool {
        !self.is_revoked && self.expires_at > now
    }

    pub fn is_expired(&self, now: i64) -> bool {
        !self.is_revoked && self.expires_at <= now
    }

    pub fn can_extend(&self, now: i64, grace_period_days: i64) -> bool {
        if self.is_revoked {
            return false;
        }
        let grace_period = grace_period_days * 24 * 60 * 60;
        self.expires_at >= now - grace_period
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseStatus {
    Active,
    Expired,
    Revoked,
}

impl License {
    pub fn status(&self, now: i64) -> LicenseStatus {
        if self.is_revoked {
            LicenseStatus::Revoked
        } else if self.expires_at > now {
            LicenseStatus::Active
        } else {
            LicenseStatus::Expired
        }
    }
}
