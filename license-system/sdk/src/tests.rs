use super::*;
use solana_sdk::{signature::Keypair, signer::Signer};

#[test]
fn test_program_id_parsing() {
    let program_id = pda::program_id();
    assert!(program_id.is_ok());
    assert_eq!(
        program_id.unwrap().to_string(),
        "5pXEX8z1aTSnm7jCKqvJCXezczKPVuPQif2BZh5u5Axq"
    );
}

#[test]
fn test_pda_derivation() {
    let owner = solana_sdk::pubkey::Pubkey::new_unique();
    let (pda, bump) = pda::derive_license_pda(&owner);
    
    assert!(bump <= 255);
    assert_ne!(pda, solana_sdk::pubkey::Pubkey::default());
    
    let (pda2, bump2) = pda::derive_license_pda(&owner);
    assert_eq!(pda, pda2);
    assert_eq!(bump, bump2);
}

#[test]
fn test_pda_uniqueness() {
    let owner1 = solana_sdk::pubkey::Pubkey::new_unique();
    let owner2 = solana_sdk::pubkey::Pubkey::new_unique();
    
    let (pda1, _) = pda::derive_license_pda(&owner1);
    let (pda2, _) = pda::derive_license_pda(&owner2);
    
    assert_ne!(pda1, pda2);
}

#[test]
fn test_client_creation_localnet() {
    let keypair = Keypair::new();
    let client = LicenseClient::new_localnet(keypair);
    assert_eq!(client.payer_pubkey(), client.payer_pubkey());
}

#[test]
fn test_client_creation_devnet() {
    let keypair = Keypair::new();
    let client = LicenseClient::new_devnet(keypair);
    assert_eq!(client.payer_pubkey(), client.payer_pubkey());
}

#[test]
fn test_license_status() {
    use types::*;
    
    let now = 1000000i64;
    let owner = solana_sdk::pubkey::Pubkey::new_unique();
    
    let active_license = License {
        owner,
        product_id: "test".to_string(),
        expires_at: now + 1000,
        is_revoked: false,
    };
    assert_eq!(active_license.status(now), LicenseStatus::Active);
    assert!(active_license.is_active(now));
    
    let expired_license = License {
        owner,
        product_id: "test".to_string(),
        expires_at: now - 1000,
        is_revoked: false,
    };
    assert_eq!(expired_license.status(now), LicenseStatus::Expired);
    assert!(expired_license.is_expired(now));
    
    let revoked_license = License {
        owner,
        product_id: "test".to_string(),
        expires_at: now + 1000,
        is_revoked: true,
    };
    assert_eq!(revoked_license.status(now), LicenseStatus::Revoked);
}
