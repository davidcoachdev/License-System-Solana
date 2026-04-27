use super::*;
use anchor_client::solana_sdk::{signature::Keypair, signer::Signer};

#[test]
fn test_program_id_parsing() {
    let program_id = PROGRAM_ID.parse::<Pubkey>();
    assert!(program_id.is_ok());
    assert_eq!(
        program_id.unwrap().to_string(),
        "5pXEX8z1aTSnm7jCKqvJCXezczKPVuPQif2BZh5u5Axq"
    );
}

#[test]
fn test_pda_derivation() {
    let keypair = Keypair::new();
    let client = LicenseClient::new_localnet(keypair).unwrap();
    
    let owner = Pubkey::new_unique();
    let (pda, bump) = client.derive_license_pda(&owner);
    
    assert!(bump <= 255);
    assert_ne!(pda, Pubkey::default());
    
    let (pda2, bump2) = client.derive_license_pda(&owner);
    assert_eq!(pda, pda2);
    assert_eq!(bump, bump2);
}

#[test]
fn test_pda_uniqueness() {
    let keypair = Keypair::new();
    let client = LicenseClient::new_localnet(keypair).unwrap();
    
    let owner1 = Pubkey::new_unique();
    let owner2 = Pubkey::new_unique();
    
    let (pda1, _) = client.derive_license_pda(&owner1);
    let (pda2, _) = client.derive_license_pda(&owner2);
    
    assert_ne!(pda1, pda2);
}

#[test]
fn test_client_creation_localnet() {
    let keypair = Keypair::new();
    let result = LicenseClient::new_localnet(keypair);
    assert!(result.is_ok());
}

#[test]
fn test_client_creation_devnet() {
    let keypair = Keypair::new();
    let result = LicenseClient::new_devnet(keypair);
    assert!(result.is_ok());
}

#[test]
fn test_payer_pubkey() {
    let keypair = Keypair::new();
    let expected_pubkey = keypair.pubkey();
    let client = LicenseClient::new_localnet(keypair).unwrap();
    
    assert_eq!(client.payer_pubkey(), expected_pubkey);
}
