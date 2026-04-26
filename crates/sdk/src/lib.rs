use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        system_program,
    },
    Client, Cluster, Program,
};
use anyhow::Result;
use std::rc::Rc;

pub const PROGRAM_ID: &str = "46gJKBXChEaV3K4juvfcmAvNEXpCPd51yCDK6Cf4HoeX";

#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error("RPC connection failed: {0}")]
    RpcError(String),
    
    #[error("Transaction failed: {0}")]
    TransactionError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(String),
}

pub struct LicenseClient {
    program: Program<Rc<Keypair>>,
    payer: Rc<Keypair>,
}

impl LicenseClient {
    pub fn new(cluster: Cluster, payer: Keypair) -> Result<Self> {
        let payer = Rc::new(payer);
        let client = Client::new_with_options(
            cluster,
            payer.clone(),
            CommitmentConfig::confirmed(),
        );
        
        let program_id = PROGRAM_ID.parse::<Pubkey>()
            .map_err(|e| SdkError::InvalidInput(format!("Invalid program ID: {}", e)))?;
        
        let program = client.program(program_id)
            .map_err(|e| SdkError::RpcError(format!("Failed to load program: {}", e)))?;
        
        Ok(Self { program, payer })
    }

    pub fn new_localnet(payer: Keypair) -> Result<Self> {
        Self::new(Cluster::Localnet, payer)
    }

    pub fn new_devnet(payer: Keypair) -> Result<Self> {
        Self::new(Cluster::Devnet, payer)
    }

    pub fn derive_license_pda(&self, owner: &Pubkey) -> (Pubkey, u8) {
        let program_id = self.program.id();
        Pubkey::find_program_address(&[b"license", owner.as_ref()], &program_id)
    }

    pub async fn issue_license(
        &self,
        owner: Pubkey,
        product_id: String,
        duration_days: i64,
    ) -> Result<Signature> {
        let (license_pda, _bump) = self.derive_license_pda(&owner);
        
        let signature = self.program
            .request()
            .accounts(license_system::accounts::IssueLicense {
                license: license_pda,
                authority: self.payer.pubkey(),
                owner,
                system_program: system_program::ID,
            })
            .args(license_system::instruction::IssueLicense {
                owner,
                product_id,
                duration_days,
            })
            .signer(&*self.payer)
            .send()
            .map_err(|e| SdkError::TransactionError(format!("Issue license failed: {}", e)))?;
        
        Ok(signature)
    }

    pub async fn extend_license(
        &self,
        owner: Pubkey,
        additional_days: i64,
    ) -> Result<Signature> {
        let (license_pda, _bump) = self.derive_license_pda(&owner);
        
        let signature = self.program
            .request()
            .accounts(license_system::accounts::ExtendLicense {
                license: license_pda,
                authority: self.payer.pubkey(),
            })
            .args(license_system::instruction::ExtendLicense {
                additional_days,
            })
            .signer(&*self.payer)
            .send()
            .map_err(|e| SdkError::TransactionError(format!("Extend license failed: {}", e)))?;
        
        Ok(signature)
    }

    pub async fn revoke_license(&self, owner: Pubkey) -> Result<Signature> {
        let (license_pda, _bump) = self.derive_license_pda(&owner);
        
        let signature = self.program
            .request()
            .accounts(license_system::accounts::RevokeLicense {
                license: license_pda,
                authority: self.payer.pubkey(),
            })
            .args(license_system::instruction::RevokeLicense {})
            .signer(&*self.payer)
            .send()
            .map_err(|e| SdkError::TransactionError(format!("Revoke license failed: {}", e)))?;
        
        Ok(signature)
    }

    pub async fn validate_license(
        &self,
        owner: Pubkey,
        product_id: String,
    ) -> Result<bool> {
        let (license_pda, _bump) = self.derive_license_pda(&owner);
        
        let result: bool = self.program
            .request()
            .accounts(license_system::accounts::ValidateLicense {
                license: license_pda,
            })
            .args(license_system::instruction::ValidateLicense {
                product_id,
            })
            .view()
            .map_err(|e| SdkError::TransactionError(format!("Validate license failed: {}", e)))?;
        
        Ok(result)
    }

    pub async fn get_license(&self, owner: Pubkey) -> Result<License> {
        let (license_pda, _bump) = self.derive_license_pda(&owner);
        
        let account: license_system::License = self.program
            .account(license_pda)
            .map_err(|e| SdkError::AccountNotFound(format!("License not found: {}", e)))?;
        
        Ok(License {
            owner: account.owner,
            product_id: account.product_id,
            expires_at: account.expires_at,
            is_revoked: account.is_revoked,
        })
    }
}

#[derive(Debug, Clone)]
pub struct License {
    pub owner: Pubkey,
    pub product_id: String,
    pub expires_at: i64,
    pub is_revoked: bool,
}

mod license_system {
    use anchor_lang::prelude::*;

    declare_id!("46gJKBXChEaV3K4juvfcmAvNEXpCPd51yCDK6Cf4HoeX");

    pub mod accounts {
        use super::*;

        pub struct IssueLicense {
            pub license: Pubkey,
            pub authority: Pubkey,
            pub owner: Pubkey,
            pub system_program: Pubkey,
        }

        pub struct ExtendLicense {
            pub license: Pubkey,
            pub authority: Pubkey,
        }

        pub struct RevokeLicense {
            pub license: Pubkey,
            pub authority: Pubkey,
        }

        pub struct ValidateLicense {
            pub license: Pubkey,
        }
    }

    pub mod instruction {
        use super::*;

        #[derive(AnchorSerialize, AnchorDeserialize)]
        pub struct IssueLicense {
            pub owner: Pubkey,
            pub product_id: String,
            pub duration_days: i64,
        }

        #[derive(AnchorSerialize, AnchorDeserialize)]
        pub struct ExtendLicense {
            pub additional_days: i64,
        }

        #[derive(AnchorSerialize, AnchorDeserialize)]
        pub struct RevokeLicense {}

        #[derive(AnchorSerialize, AnchorDeserialize)]
        pub struct ValidateLicense {
            pub product_id: String,
        }
    }

    #[account]
    pub struct License {
        pub owner: Pubkey,
        pub product_id: String,
        pub expires_at: i64,
        pub is_revoked: bool,
    }
}
