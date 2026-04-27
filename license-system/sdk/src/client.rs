use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    system_program,
};
use std::str::FromStr;

use crate::error::SdkError;
use crate::pda::{derive_license_pda, program_id};
use crate::types::License;
use crate::utils::{fetch_account, make_rpc, send};

pub struct LicenseClient {
    rpc: RpcClient,
    payer: Keypair,
}

impl LicenseClient {
    pub fn new(rpc_url: &str, payer: Keypair) -> Self {
        let rpc = make_rpc(rpc_url);
        Self { rpc, payer }
    }

    pub fn new_localnet(payer: Keypair) -> Self {
        Self::new("http://127.0.0.1:8899", payer)
    }

    pub fn new_devnet(payer: Keypair) -> Self {
        Self::new("https://api.devnet.solana.com", payer)
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    pub fn derive_license_pda(&self, owner: &Pubkey) -> (Pubkey, u8) {
        derive_license_pda(owner)
    }

    pub fn program_id(&self) -> Pubkey {
        program_id().unwrap()
    }

    fn disc(name: &str) -> Vec<u8> {
        hash(format!("global:{}", name).as_bytes()).to_bytes()[..8].to_vec()
    }

    fn build_issue_license_ix(
        owner: &Pubkey,
        product_id: &str,
        duration_days: i64,
    ) -> Result<Instruction> {
        let pid = program_id()?;
        let (license_pda, _bump) = derive_license_pda(owner);
        
        let mut data = Self::disc("issue_license");
        data.extend_from_slice(&owner.to_bytes());
        let product_id_bytes = product_id.as_bytes();
        data.extend_from_slice(&(product_id_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(product_id_bytes);
        data.extend_from_slice(&duration_days.to_le_bytes());
        
        Ok(Instruction::new_with_bytes(
            pid,
            &data,
            vec![
                AccountMeta::new(license_pda, false),
                AccountMeta::new(owner.clone(), true),
                AccountMeta::new_readonly(owner.clone(), true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
        ))
    }

    fn build_extend_license_ix(
        owner: &Pubkey,
        additional_days: i64,
    ) -> Result<Instruction> {
        let pid = program_id()?;
        let (license_pda, _bump) = derive_license_pda(owner);
        
        let mut data = Self::disc("extend_license");
        data.extend_from_slice(&additional_days.to_le_bytes());
        
        Ok(Instruction::new_with_bytes(
            pid,
            &data,
            vec![
                AccountMeta::new(license_pda, false),
                AccountMeta::new_readonly(owner.clone(), true),
            ],
        ))
    }

    fn build_revoke_license_ix(owner: &Pubkey) -> Result<Instruction> {
        let pid = program_id()?;
        let (license_pda, _bump) = derive_license_pda(owner);
        
        let data = Self::disc("revoke_license");
        
        Ok(Instruction::new_with_bytes(
            pid,
            &data,
            vec![
                AccountMeta::new(license_pda, false),
                AccountMeta::new_readonly(owner.clone(), true),
            ],
        ))
    }

    pub fn op_issue_license(
        &self,
        owner: &str,
        product_id: &str,
        duration_days: i64,
    ) -> Result<String> {
        let owner_pk = Pubkey::from_str(owner)?;
        let ix = Self::build_issue_license_ix(&owner_pk, product_id, duration_days)?;
        let sig = send(&self.rpc, &self.payer, ix)?;
        Ok(sig.to_string())
    }

    pub fn op_extend_license(&self, owner: &str, additional_days: i64) -> Result<String> {
        let owner_pk = Pubkey::from_str(owner)?;
        let ix = Self::build_extend_license_ix(&owner_pk, additional_days)?;
        let sig = send(&self.rpc, &self.payer, ix)?;
        Ok(sig.to_string())
    }

    pub fn op_revoke_license(&self, owner: &str) -> Result<String> {
        let owner_pk = Pubkey::from_str(owner)?;
        let ix = Self::build_revoke_license_ix(&owner_pk)?;
        let sig = send(&self.rpc, &self.payer, ix)?;
        Ok(sig.to_string())
    }

    pub fn get_license(&self, owner: &str) -> Result<License> {
        let owner_pk = Pubkey::from_str(owner)?;
        let (license_pda, _bump) = derive_license_pda(&owner_pk);
        
        let data = fetch_account(&self.rpc, &license_pda)?;
        
        if data.len() < 8 {
            return Err(SdkError::DeserializationError("Account data too short".into()).into());
        }
        
        let owner_bytes: [u8; 32] = data[8..40].try_into()
            .map_err(|_| SdkError::DeserializationError("Invalid owner bytes".into()))?;
        let owner = Pubkey::new_from_array(owner_bytes);
        
        let product_id_len = u32::from_le_bytes(data[40..44].try_into().unwrap()) as usize;
        let product_id = String::from_utf8(data[44..44+product_id_len].to_vec())
            .map_err(|e| SdkError::DeserializationError(format!("Invalid product_id: {}", e)))?;
        
        let expires_at_offset = 44 + product_id_len;
        let expires_at = i64::from_le_bytes(
            data[expires_at_offset..expires_at_offset+8].try_into().unwrap()
        );
        
        let is_revoked = data[expires_at_offset+8] != 0;
        
        Ok(License {
            owner,
            product_id,
            expires_at,
            is_revoked,
        })
    }

    pub fn get_all_licenses(&self) -> Result<Vec<License>> {
        let pid = program_id()?;
        let accounts = self.rpc.get_program_accounts(&pid)?;
        
        let mut licenses = Vec::new();
        for (_pubkey, account) in accounts {
            if account.data.len() < 8 {
                continue;
            }
            
            if let Ok(license) = self.deserialize_license(&account.data) {
                licenses.push(license);
            }
        }
        
        Ok(licenses)
    }

    fn deserialize_license(&self, data: &[u8]) -> Result<License> {
        if data.len() < 8 {
            return Err(SdkError::DeserializationError("Account data too short".into()).into());
        }
        
        let owner_bytes: [u8; 32] = data[8..40].try_into()
            .map_err(|_| SdkError::DeserializationError("Invalid owner bytes".into()))?;
        let owner = Pubkey::new_from_array(owner_bytes);
        
        let product_id_len = u32::from_le_bytes(data[40..44].try_into().unwrap()) as usize;
        let product_id = String::from_utf8(data[44..44+product_id_len].to_vec())
            .map_err(|e| SdkError::DeserializationError(format!("Invalid product_id: {}", e)))?;
        
        let expires_at_offset = 44 + product_id_len;
        let expires_at = i64::from_le_bytes(
            data[expires_at_offset..expires_at_offset+8].try_into().unwrap()
        );
        
        let is_revoked = data[expires_at_offset+8] != 0;
        
        Ok(License {
            owner,
            product_id,
            expires_at,
            is_revoked,
        })
    }
}
