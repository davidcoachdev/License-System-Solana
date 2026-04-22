# 04 - Technical Design

## Stack

| Componente | Language | Framework | Notes |
|-----------|----------|-----------|-------|
| Program | Rust | Anchor 0.30+ | Solana |
| SDK | Rust | — | `no_std` compatible |
| CLI | Rust | Clap 4 | Wrapper sobre SDK |
| TUI | Rust | Ratatui | Wrapper sobre SDK |
| Backend | TBD | TBD | Fase 3 |
| Indexer | TBD | — | Helius o polling |
| DB | SQL | PostgreSQL | Read model |

---

## Program (Anchor)

### Account: License

```rust
#[account]
pub struct License {
    pub owner: Pubkey,        // 32 bytes: wallet dueña
    pub product_id: String,   // max 32 bytes
    pub expires_at: i64,       // unix timestamp
    pub is_revoked: bool,     // revocada?
    pub bump: u8,            // PDA bump
}
```

**PDA seeds:** `["license", owner.as_ref(), product_id.as_bytes()]`

### Instructions

#### 1. `issue_license`

**Accounts:**
| Account | Type | Signer | Description |
|---------|------|--------|-------------|
| license | License | ❌ | PDA, se crea |
| issuer | system:Account | ✅ | Admin wallet |
| system_program | Program | ❌ | System program |

**Params:** `product_id: String`, `expires_at: i64`

**Pre-conditions:**
- [ ] `expires_at > now()`
- [ ] `expires_at <= now() + 10 * 365 * 24 * 3600`
- [ ] `product_id.len() <= 32`
- [ ] `product_id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')`
- [ ] Solo `issuer` (admin) puede llamar

**Post-conditions:**
- [ ] `license.owner == issuer.key()`
- [ ] `license.product_id == product_id`
- [ ] `license.expires_at == expires_at`
- [ ] `license.is_revoked == false`
- [ ] License PDA creada con balance mínimo

**Errors:**
```rust
#[error_code]
pub enum LicenseError {
    #[msg("License already exists")]
    AlreadyExists,
    #[msg("Product ID too long")]
    ProductIdTooLong,
    #[msg("Invalid product ID characters")]
    ProductIdInvalid,
    #[msg("Expiration too far in future")]
    ExpirationTooFar,
    #[msg("Expiration in the past")]
    ExpirationInPast,
    #[msg("Unauthorized issuer")]
    UnauthorizedIssuer,
}
```

#### 2. `extend_license`

**Accounts:** igual que `issue_license`, pero `license` ya existe

**Params:** `new_expires_at: i64`

**Pre-conditions:**
- [ ] License existe y no está revocada
- [ ] `new_expires_at > license.expires_at`
- [ ] `new_expires_at <= now() + 10 * 365 * ...`

**Post-conditions:**
- [ ] `license.expires_at == new_expires_at`

**Errors:** `AlreadyExists`, `LicenseNotFound`, `LicenseRevoked`, `ExpirationNotExtended`, etc.

#### 3. `revoke_license`

**Accounts:** igual que `issue_license`

**Params:** ninguno

**Pre-conditions:**
- [ ] License existe
- [ ] No está ya revocada

**Post-conditions:**
- [ ] `license.is_revoked == true`

**Errors:** `LicenseNotFound`

#### 4. `validate` (opcional, view function)

**Returns:** `(bool, i64)` → `(is_valid, expires_at)` para el `(owner, product_id)` dado

---

## SDK (Rust)

### Crate structure

```
crates/
└── sdk/
    ├── Cargo.toml
    └── src/
        ├── lib.rs           ← re-exports
        ├── client.rs       ← LicenseClient
        ├── errors.rs       ← error types
        ├── types.rs       ← domain types
        └── instruction.rs  ← IX builders
```

### LicenseClient API

```rust
pub struct Config {
    pub rpc_url: String,
    pub commitment: CommitmentLevel,
    pub program_id: Pubkey,
}

pub struct LicenseClient {
    config: Config,
    wallet: Keypair,
}

pub enum IssueResult {
    Signature(Signature),
    AlreadyExists,
}

impl LicenseClient {
    /// Issue a new license
    pub fn issue(
        &self,
        owner: Pubkey,
        product_id: &str,
        days: u32,
    ) -> Result<Signature>

    /// Extend existing license
    pub fn extend(
        &self,
        owner: Pubkey,
        product_id: &str,
        days: u32,
    ) -> Result<Signature>

    /// Revoke license
    pub fn revoke(
        &self,
        owner: Pubkey,
        product_id: &str,
    ) -> Result<Signature>

    /// Get license state from Solana
    pub fn get_license(
        &self,
        owner: Pubkey,
        product_id: &str,
    ) -> Result<Option<License>>

    /// Validate license (online)
    pub fn validate(
        &self,
        owner: Pubkey,
        product_id: &str,
    ) -> Result<ValidationResult>

    /// Sign message for offline validation
    pub fn sign_license_message(
        &self,
        owner: Pubkey,
        product_id: &str,
    ) -> Result<Signature>
}
```

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("Transaction failed: {0}")]
    TxFailed(String),
    #[error("Account not found")]
    NotFound,
    #[error("Anchor IDL error: {0}")]
    Anchor(String),
    #[error("Invalid configuration: {0}")]
    Config(String),
}
```

### Offline Validation Support

```rust
/// Signed license proof for offline validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseProof {
    pub owner: String,        // base58 pubkey
    pub product_id: String,
    pub expires_at: i64,
    pub signature: String,   // base58 solana signature
    pub issued_at: i64,     // when issued
}

/// Build a license proof from on-chain data
pub fn build_proof(license: &License, tx_sig: &str) -> LicenseProof

/// Verify a license proof (no network needed)
pub fn verify_proof(proof: &LicenseProof) -> bool
```

---

## CLI / TUI

### CLI (Clap)

```rust
// Estructura de comandos
license-cli issue <OWNER> <PRODUCT_ID> <DAYS>
license-cli extend <OWNER> <PRODUCT_ID> <DAYS>
license-cli revoke <OWNER> <PRODUCT_ID>
license-cli validate <OWNER> <PRODUCT_ID>
license-cli list [--owner <OWNER>]
license-cli status <OWNER> <PRODUCT_ID>

// Flags globales
--rpc <URL>         // Override RPC URL
--wallet <NAME>      // Seleccionar wallet
--json             // Output JSON
--verbose          // Verbose
```

### TUI (Ratatui)

**Layout:**
```
┌─────────────────────────────────────┐
│ License System Admin    [wallet: xxx] │
├─────────────────────────────────────┤
│ [Lista de licencias]                 │
│  > licencia-1    ACTIVE   30d      │
│    licencia-2    EXPIRED  -2d     │
│    licencia-3    REVOKED  -      │
├─────────────────────────────────────┤
│ Actions:                          │
│  [I]ssue  [E]xtend  [R]evoke     │
│  [S]witch wallet  [C]onfig        │
├─────────────────────────────────────┤
│ > _                               │
└─────────────────────────────────────┘
```

**Keybindings:**
- `j/k` — navegar lista
- `i` — issue (shortcut)
- `e` — extend
- `r` — revoke
- `s` — switch wallet
- `q` — quit
- `/` — buscar

---

## Indexer (Fase 3)

### Helius Webhook Integration

```rust
// Endpoint: POST /api/v1/webhooks/solana
// Helius envía eventos de programa

#[derive(Deserialize)]
struct HeliusWebhookEvent {
    signature: String,
    slot: u64,
    logs: Vec<String>,
}

impl HeliusWebhookEvent {
    fn parse_license_ix(&self) -> Option<LicenseInstruction>
}
```

### Polling Fallback

```rust
// Fallback si Helius no está disponible
async fn poll_licenses(client: &LicenseClient) -> Result<Vec<License>> {
    let accounts = client.get_program_accounts().await?;
    // Filtrar accounts tipo License
    // Detectar cambios desde último poll
}
```

---

## Database Schema (PostgreSQL)

```sql
-- Schema: licenses

CREATE TABLE licenses (
  id TEXT PRIMARY KEY,              -- "{owner}:{product_id}"
  owner TEXT NOT NULL,
  product_id TEXT NOT NULL,
  expires_at BIGINT NOT NULL,
  is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
  tx_signature TEXT NOT NULL,
  issued_at BIGINT NOT NULL,
  updated_at BIGINT NOT NULL,
  created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT,
  CONSTRAINT unique_owner_product UNIQUE (owner, product_id)
);

CREATE INDEX idx_licenses_owner ON licenses(owner);
CREATE INDEX idx_licenses_product ON licenses(product_id);
CREATE INDEX idx_licenses_expires ON licenses(expires_at);
CREATE INDEX idx_licenses_active ON licenses(owner, product_id, is_revoked, expires_at);

-- Log de eventos para webhooks idempotentes
CREATE TABLE license_events (
  id SERIAL PRIMARY KEY,
  license_id TEXT NOT NULL REFERENCES licenses(id),
  event_type TEXT NOT NULL,         -- created | extended | revoked
  tx_signature TEXT NOT NULL UNIQUE,
  payload JSONB,
  processed_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())::BIGINT
);

CREATE INDEX idx_events_license ON license_events(license_id);
CREATE INDEX idx_events_unprocessed ON license_events(tx_signature)
  WHERE processed_at IS NULL;
```

---

## API Endpoints (Fase 3)

### Backend API

```
Base path: /api/v1

POST   /licenses              Body: {owner, product_id, days}   Auth: Admin JWT
POST   /licenses/:id/extend  Body: {days}                  Auth: Admin JWT
DELETE /licenses/:id         —                          Auth: Admin JWT
GET    /licenses/:id         —                          Auth: —
POST   /validate            Body: {owner, product_id}       Auth: —
GET    /health            —                          Auth: —
```

### Request / Response Examples

**POST /licenses**
```json
// Request
{
  "owner": "7x4Jb...",
  "product_id": "pro-plan-basic",
  "days": 365
}

// Response 201
{
  "id": "7x4Jb...:pro-plan-basic",
  "owner": "7x4Jb...",
  "product_id": "pro-plan-basic",
  "expires_at": 1234567890,
  "tx_signature": "abc..."
}

// Response 409 (conflict)
{
  "error": "LICENSE_ALREADY_EXISTS",
  "message": "License already exists for this owner and product"
}
```

**POST /validate**
```json
// Request
{
  "owner": "7x4Jb...",
  "product_id": "pro-plan-basic"
}

// Response 200
{
  "valid": true,
  "expires_at": 1234567890,
  "days_remaining": 300
}
```

---

## Configuration

### Environment Variables

| Variable | Required | Description |
|---------|----------|-------------|
| `RPC_URL` | ✅ | Solana RPC endpoint |
| `PROGRAM_ID` | ✅ | Anchor program ID |
| `ADMIN_WALLET_PATH` | ✅ (CLI) | Path a admin keypair |
| `ADMIN_KEYPAIR` | ✅ (CLI) | Base58 encoded keypair (alt) |
| `DATABASE_URL` | ✅ (Backend) | PostgreSQL connection |
| `HELIUS_WEBHOOK_SECRET` | — | Helius webhook verification |
| `JWT_SECRET` | ✅ (Backend) | JWT signing secret |
| `RUST_LOG` | — | Logging level |

### Config File (CLI)

`~/.config/license-cli/config.toml`:
```toml
[rpc]
default = "https://api.devnet.solana.com"
overrides = { prod = "https://api.mainnet-beta.solana.com" }

[wallet]
active = "admin-prod"
available = ["admin-dev", "admin-prod"]

[wallet.admin-dev]
type = "file"
path = "~/.config/license-cli/wallets/admin-dev.json"

[wallet.admin-prod]
type = "env"
key = "ADMIN_KEYPAIR"
```

---

## Error Handling

### Error Response Format

```json
{
  "error": "ERROR_CODE",
  "message": "Human readable message",
  "details": {
    "field": "additional context"
  },
  "request_id": "uuid"
}
```

### Error Codes

| Code | HTTP | Description |
|------|------|-------------|
| `LICENSE_NOT_FOUND` | 404 | License doesn't exist |
| `LICENSE_ALREADY_EXISTS` | 409 | License already issued |
| `LICENSE_EXPIRED` | 410 | License has expired |
| `LICENSE_REVOKED` | 410 | License was revoked |
| `UNAUTHORIZED` | 401 | Missing auth |
| `FORBIDDEN` | 403 | Not admin |
| `INVALID_REQUEST` | 400 | Bad params |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Observability

### Structured Logging

```rust
// Formato JSON para todos los logs
{
  "ts": "2024-01-01T00:00:00Z",
  "level": "INFO",
  "msg": "License issued",
  "owner": "7x4Jb...",
  "product_id": "pro-plan",
  "tx_sig": "abc...",
  "request_id": "uuid"
}
```

### Metrics (Prometheus)

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `licenses_issued_total` | Counter | `product_id` | Total issued |
| `licenses_revoked_total` | Counter | `product_id` | Total revoked |
| `validations_total` | Counter | `result` | Total validations |
| `tx_confirm_duration_ms` | Histogram | `ix` | Tx confirmation time |
| `indexer_lag_seconds` | Gauge | — | DB sync lag |

### Tracing

Trace completo: `request → SDK → Solana RPC → confirmation`

```rust
tracing::info!(
    parent: span,
    tx_sig = %signature,
    owner = %owner,
    "Transaction confirmed"
);
```