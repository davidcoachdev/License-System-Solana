# 06 - Testing Strategy

## Principles

1. **Test the contract first** — el programa Anchor es la fuente de verdad
2. **No mocks en critical paths** — el SDK testa contra Solana real (devnet)
3. **Integration tests para flujos completos**
4. **E2E para flujos críticos de usuario**

---

## Test Pyramid

```
        ┌─────────────┐
        │    E2E    │  ← Full flow, 1-5 tests críticos
        ├─────────────┤
        │ Integration │  ← SDK + program, devnet
        ├─────────────┤
        │   Unit     │  ← SDK pure, program pure
        └─────────────┘
```

---

## Unit Tests

### Anchor Program

**Location:** `program/tests/` o inline `#[test]`

**Qué testar:**
- Cada IX con éxito
- Cada IX con error (casos negativos)
- Invariants (I1-I5 del dominio)
- PDA derivation correctness

```rust
#[derive(AnchorSerialize, AnchorDeserialize)]
struct IssueParams {
    product_id: String,
    expires_at: i64,
}

#[program]
mod license {
    #[test]
    fn test_issue_success() {
        // Setup program + issuer
        // Call issue_license
        // Assert license.created
        // Assert tx success
    }

    #[test]
    fn test_issue_duplicate_fails() {
        // Issue license
        // Try issue again
        // Assert AlreadyExists error
    }

    #[test]
    fn test_extend_requires_existing() {
        // Try extend non-existent
        // Assert NotFound error
    }

    #[test]
    fn test_extend_must_extend_not_shorten() {
        // Issue license with expires_at = T
        // Try extend with T-1
        // Assert error
    }

    #[test]
    fn test_revoke_is_permanent() {
        // Issue + revoke
        // Try extend
        // Assert Revoked error
    }

    #[test]
    fn test_pda_uniqueness() {
        // Same owner + product_id
        // Issue
        // Assert PDA is deterministic
    }

    #[test]
    fn test_product_id_validation() {
        // Try issue with "a/b/c"
        // Assert ProductIdInvalid
    }
}
```

### SDK (Rust)

**Location:** `crates/sdk/src/*.rs` → `crates/sdk/tests/`

**Qué testar:**
- `LicenseClient` methods (mock RPC o test validator)
- Error translation
- Offline validation logic
- Domain enrichment

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_issue() {
        // Setup test validator
        let client = LicenseClient::new(config, keypair);
        let sig = client.issue(owner, "pro", 30).await;
        assert!(sig.is_ok());
    }

    #[test]
    fn test_offline_validation() {
        let proof = LicenseProof { ... };
        assert!(verify_proof(&proof).is_ok());
    }

    #[test]
    fn test_offline_validation_tampered() {
        let mut proof = LicenseProof { ... };
        proof.expires_at -= 1000;
        assert!(verify_proof(&proof).is_err());
    }

    #[test]
    fn test_enrich_license() {
        let license = License { expires_at: now() + 86400, ... };
        let enriched = EnrichedLicense::from_license(&license);
        assert_eq!(enriched.status, Active);
        assert_eq!(enriched.days_remaining, 1);
    }

    #[test]
    fn test_enrich_expired() {
        let license = License { expires_at: now() - 86400, ... };
        let enriched = EnrichedLicense::from_license(&license);
        assert_eq!(enriched.status, Expired);
        assert!(!enriched.is_valid);
    }
}
```

---

## Integration Tests

**Location:** `tests/integration/` o `scripts/integration.ts`

**Qué testar:**
- SDK contra Solana devnet real
- Flujos completos: issue → extend → validate → revoke
- TX confirmation waits
- Error propagation from RPC

```rust
// tests/integration/license_flow.rs

#[tokio::test]
#[ignore] // Runs against devnet, skip in CI unless explicitly requested
async fn test_full_license_lifecycle() {
    // Setup: fund issuer wallet on devnet
    let client = test_client();

    // Issue
    let sig = client.issue(owner, "test-pro", 30).await.unwrap();
    client.wait_for_confirmation(sig).await;

    // Validate
    let result = client.validate(owner, "test-pro").await.unwrap();
    assert!(result.is_valid);

    // Extend
    let sig = client.extend(owner, "test-pro", 60).await.unwrap();
    client.wait_for_confirmation(sig).await;

    // Validate after extend
    let result = client.validate(owner, "test-pro").await.unwrap();
    assert_eq!(result.expires_at, original + 90 days);

    // Revoke
    let sig = client.revoke(owner, "test-pro").await.unwrap();
    client.wait_for_confirmation(sig).await;

    // Validate after revoke
    let result = client.validate(owner, "test-pro").await.unwrap();
    assert!(!result.is_valid);
}
```

### Running Integration Tests

```bash
# Against devnet (requires airdrop)
SOLANA_RPC_URL=https://api.devnet.solana.com \
cargo test --test integration -- --ignored

# Against test validator (local, faster)
cargo test --test integration -- --include-ignored
```

---

## E2E Tests (CLI + TUI)

**Location:** `tests/e2e/` → Rust o Playwright

**Quéistar:**
- CLI help/output
- TUI renders sin crash
- Full flow en terminal

```rust
#[test]
fn test_cli_issue_help() {
    let output = Command::new("./target/release/license-cli")
        .arg("issue")
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("OWNER"));
}

#[test]
fn test_cli_issue_and_validate() {
    // Start local test validator
    // Fund admin wallet
    // Run: license-cli issue <owner> test-pro 30
    // Assert signature output
    // Run: license-cli validate <owner> test-pro
    // Assert active
}
```

---

## Test Fixtures

### Test Wallets

Generar keypairs de test deterministic para reproducibilidad:

```rust
fn test_wallet() -> Keypair {
    let bytes = bs58::decode("4eGak...")
        .into_vec()
        .unwrap()
        .try_into()
        .unwrap();
    Keypair::from_bytes(&bytes).unwrap()
}
```

### Test Products

```
test-pro-basic    — producto básico de test
test-pro-plus    — producto premium de test
test-pro-expired  — para testar expiry
test-pro-revoked — para testar revocación
```

### Test Data Constants

```rust
const TEST_DAYS_30: u32 = 30;
const TEST_DAYS_365: u32 = 365;
const TEST_PRODUCT_BASIC: &str = "test-pro-basic";
const TEST_PRODUCT_PLUS: &str = "test-pro-plus";
const TEST_RPC_DEVNET: &str = "https://api.devnet.solana.com";
```

---

## Coverage

### Targets

| Layer | Target | Tool |
|-------|--------|------|
| Program | 90%+ branches | `cargo test` + anchor test |
| SDK | 85%+ lines | `cargo tarpaulin` |
| CLI | 80%+ | `cargo test` |
| TUI | 70%+ | `cargo test` |

### Enforcement

- Program coverage: **blocker** en PR (cannot merge < 90%)
- SDK coverage: **warning** en PR (advisory)
- CI corre coverage y falla si coverage bajó sin justificación

---

## CI Integration

```yaml
# .github/workflows/test.yml
name: Test
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Run program tests
        run: cargo test -p license-program
      - name: Run SDK unit tests
        run: cargo test -p license-sdk --lib
      - name: Run CLI tests
        run: cargo test -p license-cli
      - name: Check coverage
        run: cargo tarpaulin --out Xml

  integration:
    runs-on: ubuntu-latest
    if: github.event_name == 'schedule' || contains(github.event.head_commit.message, '[integration]')
    steps:
      - uses: actions/checkout@v4
      - name: Run integration tests (devnet)
        run: |
          SOLANA_RPC_URL=${{ secrets.DEVNET_RPC_URL }} \
          cargo test --test integration -- --ignored
```

---

## Testing Matrix

| Test | Devnet | Local validator | Mock |
|------|--------|--------------|------|
| Unit program | ❌ | ✅ | ❌ |
| Unit SDK | ❌ | ✅ | ✅ |
| Integration | ✅ | ❌ | ❌ |
| E2E CLI | ✅ | ✅ | ❌ |
| E2E TUI | ✅ | ✅ | ❌ |

**Local validator:** `solana-test-validator` via `cargo test-validator` o `solana-local-validator`

---

## Manual Testing

Para testing que no se automatiza (TUI manual, flujos ad-hoc):

```bash
# Quick manual test
./target/release/license-cli issue 7x4Jb... test-pro 7 --rpc https://api.devnet.solana.com --verbose

# Check on explorer
open https://explorer.solana.com/tx/<signature>?cluster=devnet

# Revoke
./target/release/license-cli revoke 7x4Jb... test-pro
```

### Test Validator Quick

```bash
# Start validator en background
solana-test-validator --limit-ledger-size 10000000 &
sleep 3

# Airdrop SOL
solana airdrop 2

# Run tests
cargo test -p license-cli

# Stop
pkill solana-test-validator
```