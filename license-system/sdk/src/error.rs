use thiserror::Error;

pub type Result<T> = std::result::Result<T, SdkError>;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("RPC connection failed: {0}")]
    RpcError(String),
    
    #[error("Transaction failed: {0}")]
    TransactionError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    
    #[error("Deserialization failed: {0}")]
    DeserializationError(String),
    
    #[error("Program error: {0}")]
    ProgramError(String),
}

impl From<solana_client::client_error::ClientError> for SdkError {
    fn from(e: solana_client::client_error::ClientError) -> Self {
        SdkError::RpcError(e.to_string())
    }
}

impl From<solana_sdk::pubkey::ParsePubkeyError> for SdkError {
    fn from(e: solana_sdk::pubkey::ParsePubkeyError) -> Self {
        SdkError::InvalidInput(format!("Invalid pubkey: {}", e))
    }
}

impl From<std::io::Error> for SdkError {
    fn from(e: std::io::Error) -> Self {
        SdkError::RpcError(format!("IO error: {}", e))
    }
}
