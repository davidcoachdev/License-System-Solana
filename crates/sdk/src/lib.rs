use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
        system_program,
    },
    Client, Cluster,
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
    client: Client<Rc<Keypair>>,
    program_id: Pubkey,
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
        
        Ok(Self { client, program_id, payer })
    }

    pub fn new_localnet(payer: Keypair) -> Result<Self> {
        Self::new(Cluster::Localnet, payer)
    }

    pub fn new_devnet(payer: Keypair) -> Result<Self> {
        Self::new(Cluster::Devnet, payer)
    }

    pub fn derive_license_pda(&self, owner: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"license", owner.as_ref()], &self.program_id)
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }
}

#[derive(Debug, Clone)]
pub struct License {
    pub owner: Pubkey,
    pub product_id: String,
    pub expires_at: i64,
    pub is_revoked: bool,
}
