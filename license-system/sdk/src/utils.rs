use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

pub fn make_rpc(url: &str) -> RpcClient {
    RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed())
}

pub fn send(rpc: &RpcClient, payer: &Keypair, ix: Instruction) -> Result<Signature> {
    let recent_blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    let signature = rpc.send_and_confirm_transaction(&tx)?;
    Ok(signature)
}

pub fn send_many(rpc: &RpcClient, payer: &Keypair, ixs: Vec<Instruction>) -> Result<Signature> {
    let recent_blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &ixs,
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    let signature = rpc.send_and_confirm_transaction(&tx)?;
    Ok(signature)
}

pub fn fetch_account(rpc: &RpcClient, pubkey: &Pubkey) -> Result<Vec<u8>> {
    let account = rpc.get_account(pubkey)?;
    Ok(account.data)
}
