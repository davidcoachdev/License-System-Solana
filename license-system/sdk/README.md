# 🦀 License SDK

Rust SDK for interacting with the License System smart contract on Solana.

---

## 📖 Overview

The License SDK provides a type-safe, high-level interface for managing software licenses on Solana blockchain. It abstracts away the complexity of Solana RPC calls and provides convenient methods for all license operations.

---

## ✨ Features

- ✅ **Type-safe operations** — All functions use Rust types
- ✅ **PDA derivation** — Automatic Program Derived Address calculation
- ✅ **Error handling** — Custom error types with context
- ✅ **Async/await** — Non-blocking operations with tokio
- ✅ **Cluster support** — Localnet, Devnet, Mainnet
- ✅ **Account fetching** — Fetch and deserialize license data
- ✅ **Search/Filter** — Find licenses by owner, product_id, status

---

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
license-sdk = { path = "../sdk" }
solana-sdk = "2.1"
solana-client = "2.1"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use license_sdk::{LicenseClient, error::Result};
use solana_sdk::signature::Keypair;

#[tokio::main]
async fn main() -> Result<()> {
    // Create client
    let keypair = Keypair::new();
    let client = LicenseClient::new_localnet(keypair)?;
    
    // Issue a license
    let owner = Keypair::new().pubkey();
    let signature = client.issue_license(
        owner,
        "premium-plan".to_string(),
        30, // days
    ).await?;
    
    println!("License issued: {}", signature);
    
    Ok(())
}
```

---

## 📚 API Reference

### LicenseClient

#### Constructor Methods

```rust
// Create client for localnet
pub fn new_localnet(payer: Keypair) -> Result<Self>

// Create client for devnet
pub fn new_devnet(payer: Keypair) -> Result<Self>

// Create client for custom cluster
pub fn new(cluster: Cluster, payer: Keypair) -> Result<Self>
```

#### PDA Methods

```rust
// Derive license PDA for an owner
pub fn derive_license_pda(&self, owner: &Pubkey) -> (Pubkey, u8)

// Get payer pubkey
pub fn payer_pubkey(&self) -> Pubkey

// Get program ID
pub fn program_id(&self) -> Pubkey
```

#### Transaction Methods (TODO)

```rust
// Issue a new license
pub async fn issue_license(
    &self,
    owner: Pubkey,
    product_id: String,
    duration_days: i64,
) -> Result<Signature>

// Extend an existing license
pub async fn extend_license(
    &self,
    owner: Pubkey,
    additional_days: i64,
) -> Result<Signature>

// Revoke a license
pub async fn revoke_license(&self, owner: Pubkey) -> Result<Signature>

// Validate a license
pub async fn validate_license(
    &self,
    owner: Pubkey,
    product_id: String,
) -> Result<bool>
```

#### Fetch Methods (TODO)

```rust
// Get license data
pub async fn get_license(&self, owner: Pubkey) -> Result<License>

// Get all licenses
pub async fn get_all_licenses(&self) -> Result<Vec<License>>

// Get licenses by owner
pub async fn get_licenses_by_owner(&self, owner: Pubkey) -> Result<Vec<License>>

// Get licenses by product
pub async fn get_licenses_by_product(&self, product_id: &str) -> Result<Vec<License>>

// Get licenses by status
pub async fn get_licenses_by_status(&self, status: LicenseStatus) -> Result<Vec<License>>
```

---

## 🔧 Examples

### Issue License

```rust
use license_sdk::LicenseClient;
use solana_sdk::signature::Keypair;

let client = LicenseClient::new_localnet(Keypair::new())?;
let owner = Keypair::new().pubkey();

let signature = client.issue_license(
    owner,
    "premium-plan".to_string(),
    30, // days
).await?;

println!("License issued: {}", signature);
```

### Extend License

```rust
let signature = client.extend_license(
    owner,
    15, // additional days
).await?;

println!("License extended: {}", signature);
```

### Revoke License

```rust
let signature = client.revoke_license(owner).await?;
println!("License revoked: {}", signature);
```

### Validate License

```rust
let is_valid = client.validate_license(
    owner,
    "premium-plan".to_string(),
).await?;

if is_valid {
    println!("✅ License is valid");
} else {
    println!("❌ License is invalid");
}
```

### Get License Data

```rust
let license = client.get_license(owner).await?;

println!("Owner: {}", license.owner);
println!("Product: {}", license.product_id);
println!("Expires: {}", license.expires_at);
println!("Revoked: {}", license.is_revoked);
```

### List All Licenses

```rust
let licenses = client.get_all_licenses().await?;

for license in licenses {
    println!("License: {} - {}", license.owner, license.product_id);
}
```

---

## 🐛 Error Handling

The SDK uses custom error types:

```rust
pub enum SdkError {
    RpcError(String),           // RPC connection failed
    TransactionError(String),   // Transaction failed
    InvalidInput(String),       // Invalid input parameter
    AccountNotFound(String),    // Account doesn't exist
}
```

### Example

```rust
match client.issue_license(owner, product_id, days).await {
    Ok(signature) => println!("Success: {}", signature),
    Err(SdkError::RpcError(e)) => eprintln!("RPC error: {}", e),
    Err(SdkError::TransactionError(e)) => eprintln!("Transaction failed: {}", e),
    Err(SdkError::InvalidInput(e)) => eprintln!("Invalid input: {}", e),
    Err(SdkError::AccountNotFound(e)) => eprintln!("Account not found: {}", e),
}
```

---

## 🧪 Testing

### Run Unit Tests

```bash
cargo test
```

### Run Integration Tests

```bash
# Start local validator
solana-test-validator --reset

# Run tests
cargo test --test integration
```

---

## 📊 Performance

### Caching
The SDK includes an optional account data cache to reduce RPC calls:

```rust
let config = PerformanceConfig {
    enable_cache: true,
    cache_ttl: Duration::from_secs(30),
    max_cache_size: 1000,
    ..Default::default()
};

let client = LicenseClient::with_config(cluster, payer, config)?;
```

### Retry Logic
Automatic retry with exponential backoff for failed transactions:

```rust
let config = PerformanceConfig {
    retry_config: RetryConfig {
        max_retries: 3,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(5),
        backoff_multiplier: 2.0,
    },
    ..Default::default()
};
```

---

## 🔐 Security

### Access Control
- **Issue License**: Only when `owner == authority`
- **Extend License**: Only the owner can extend
- **Revoke License**: Only the owner can revoke
- **Validate License**: Read-only (anyone can validate)

### Grace Period
Licenses have a **7-day grace period** after expiration:
- ✅ Within grace period: can extend
- ❌ After grace period: cannot extend (must issue new license)

---

## 📝 Constants

```rust
pub const PROGRAM_ID: &str = "5pXEX8z1aTSnm7jCKqvJCXezczKPVuPQif2BZh5u5Axq";
pub const GRACE_PERIOD_DAYS: i64 = 7;
pub const MAX_PRODUCT_ID_LEN: usize = 64;
```

---

## 🚧 Roadmap

### v0.1.0 (Current)
- [x] PDA derivation
- [x] Error types
- [x] Client creation
- [x] Unit tests (6 tests)

### v0.2.0 (Sprint 1)
- [ ] Real transactions (issue, extend, revoke, validate)
- [ ] Account fetching
- [ ] Modular structure (client, error, pda, types, utils)
- [ ] Integration tests

### v0.3.0 (Sprint 2)
- [ ] Search/Filter
- [ ] Batch operations
- [ ] Event listener
- [ ] Performance optimizations (cache, retry)

### v1.0.0 (Production)
- [ ] Full documentation
- [ ] Examples
- [ ] Benchmarks
- [ ] Audit

---

## 🤝 Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

---

## 📄 License

[Add license here]

---

**Last Updated**: 2026-04-26  
**Version**: 0.1.0  
**Status**: ⚠️ Basic implementation (needs refactor)
