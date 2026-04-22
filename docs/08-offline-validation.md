# 08 - Offline Validation

> **Este documento describe el diferenciador principal del sistema: validación de licencias sin conexión a internet.**

---

## Concepto

El software cliente puede validar una licencia **localmente**, sin consultar a ningún servidor. La licencia se firma cryptográficamente con la clave privada del admin. El cliente verifica la firma y el timestamp.

---

## Por qué importa

| Validación online | Validación offline |
|------------------|-------------------|
| ✅ Siempre actual | ✅ Funciona sin internet |
| ✅ Siempre canonical | ✅ < 50ms latency |
| ⚠️ Depende de Solana RPC | ✅ Sin RPC, sin fees |
| ⚠️ RPC puede estar down | ✅ Resiliencia total |

**Trade-off:** Offline validation no detecta una revocación hasta que el cliente sincronice. El cliente puede usar online validation como fallback.

---

## Esquema de firma

### Qué se firma

El admin keypair firma un mensaje que contiene:

```
message = "{owner}:{product_id}:{expires_at}:{issued_at}:{tx_sig}"
```

### Formato del proof

```rust
/// Proof de licencia para validación offline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseProof {
    /// Wallet dueña de la licencia (base58 pubkey)
    pub owner: String,

    /// ID del producto
    pub product_id: String,

    /// Timestamp de expiración (unix seconds)
    pub expires_at: i64,

    /// Timestamp de emisión (unix seconds)
    pub issued_at: i64,

    /// Signature de la transacción original (base58 solana signature)
    /// Única para esta licencia específica
    pub tx_signature: String,

    /// Signature del admin firmando el proof (base58)
    /// Verificable contra admin pubkey
    pub proof_signature: String,
}
```

### El proof se genera en 2 pasos

**Paso 1 — Emisión:**
```
Admin wallet firma tx "issue_license"
    │
    ▼
TX se ejecuta en Solana
    │
    ▼
Transaction signature (tx_sig) queda en tx history
```

**Paso 2 — Generar proof:**
```
SDK toma tx_sig + license state
    │
    ▼
Genera LicenseProof {
    owner, product_id, expires_at, issued_at, tx_sig
}
    │
    ▼
Admin wallet firma este payload
    │
    ▼
proof_signature queda en el proof
```

### Verificación en cliente

```rust
pub fn verify_offline(proof: &LicenseProof) -> Result<ValidationResult> {
    // 1. Verificar que el proof no está expirado
    let now = Clock::get()?.unix_timestamp;
    if proof.expires_at <= now {
        return Ok(ValidationResult::Expired);
    }

    // 2. Recomponer mensaje original
    let message = format!(
        "{}:{}:{}:{}:{}",
        proof.owner,
        proof.product_id,
        proof.expires_at,
        proof.issued_at,
        proof.tx_signature
    );

    // 3. Verificar signature del admin
    let admin_pubkey = load_admin_pubkey()?;
    let signature = bs58::decode(&proof.proof_signature).unwrap();
    let message_bytes = message.as_bytes();

    if !admin_pubkey.verify(message_bytes, &signature) {
        return Err(SdkError::InvalidSignature);
    }

    // 4. Tudo ok
    Ok(ValidationResult::Valid {
        owner: proof.owner.clone(),
        product_id: proof.product_id.clone(),
        expires_at: proof.expires_at,
        days_remaining: (proof.expires_at - now) / 86400,
    })
}
```

---

## Flujo completo

```
┌─────────────┐          ┌─────────────┐          ┌─────────────┐
│   Admin     │          │   Solana   │          │  Client    │
└──────┬──────┘          └──────┬──────┘          └──────┬──────┘
       │                        │                        │
       │ issue_license          │                        │
       │ ─────────────────────▶│                        │
       │                        │                        │
       │              TX signature                       │
       │ ◀──────────────────────│                        │
       │                        │                        │
       │            Generar proof                         │
       │ sign(proof_payload)     │                        │
       │                        │                        │
       │              LicenseProof + proof_signature      │
       │ ─────────────────────────────────────────▶   │
       │                        │                        │
       │                        │              store locally
       │                        │              (file/DB)
       │                        │                        │
       │                        │              verify_offline()
       │                        │              ✓ access
       │                        │                        │
       │                        │              [offline forever]
       │
       │
       │ [opcional] revoke
       │ ─────────────────────▶│
       │                        │
       │              TX signature (revoke)
       │ ◀──────────────────────│
       │                        │
       │            [sync] — nueva proof revocada?
       │                        │
       │                        │              verify fails
       │                        │              ✗ access
```

---

## Almacenamiento en cliente

### Options

| Storage | Pros | Cons |
|---------|------|------|
| **File** (`~/.license/proof.json`) | Simple, portable | Puede borrarse |
| **OS Keychain** | Seguro | Depende de OS |
| **DB local** | Queryable | Más código |
| **Encrypted file** (elegida) | Simple + encryption | Encryption key management |

### Formato del archivo

`~/.license.d/active.json`:
```json
{
  "version": 1,
  "licenses": [
    {
      "owner": "7x4Jb...",
      "product_id": "pro-basic",
      "expires_at": 1735689600,
      "issued_at": 1704067200,
      "tx_signature": "abc...",
      "proof_signature": "xyz...",
      "created_at": 1704153600
    }
  ]
}
```

**Encryptado con AES-256-GCM.** Key derivada de `LICENSE_KEY` env var o keychain.

---

## Sincronización

El cliente necesita sync periodically para:
1. Detectar revocations (el proof puede revocarse offline)
2. Actualizar expirations
3. Manejar transfers (no aplica en Fase 1)

### Estrategia

```
On app startup (background):
    │
    ▼
POST /api/v1/sync { proof_signature: [...] }
    │
    ▼
Backend responde con estado actual + proof signature
    │
    ▼
Si remote state != local:
    ├── Revoked: remove local proof
    ├── Updated: replace local proof
    └── Same: nothing
```

### Sync endpoint

```
POST /api/v1/sync
Body: {
  "proofs": [
    {
      "owner": "...",
      "product_id": "...",
      "proof_signature": "..."
    }
  ]
}

Response 200: {
  "results": [
    { "owner": "...", "product_id": "...", "status": "valid" },
    { "owner": "...", "product_id": "...", "status": "revoked" },
    { "owner": "...", "product_id": "...", "status": "expired" }
  ],
  "server_time": 1704153600
}
```

---

## Edge Cases

### Clock skew

El cliente puede tener el reloj desincronizado. Para isso:
1. Usar NTP sync al arrancar
2. El proof incluye `server_time` del sync endpoint
3. Calcular drift y warn si > 5 min

### Admin key rotation

Si el admin key rota:
1. Todos los proofs existentes se invalidan
2. Admin debe re-emitir todas las licencias
3. **No es trivial** — documentar en runbook

### Replay attack

Un attacker no puede forgehear proofs porque:
1. `proof_signature` es verificable contra admin pubkey
2. `tx_signature` es único por licencia
3. Sin admin private key, no se puede firmar nuevo proof

### Offline por siempre

Si el cliente nunca sincroniza:
1. Proof expira naturalmente (expires_at)
2. Después de expires_at, acceso denegado
3. Sync online renueva acceso

**Mitigación:** Warn al usuario 30 días antes de expiración.

---

## SDK API

```rust
/// Build a license proof for offline use
pub async fn build_proof(
    &self,
    owner: Pubkey,
    product_id: &str,
) -> Result<LicenseProof> {
    let license = self.get_license(owner, product_id).await?
        .ok_or(SdkError::NotFound)?;

    // Get original tx signature
    let tx_sig = self.get_issue_transaction(owner, product_id).await?;

    // Build payload
    let payload = format!(
        "{}:{}:{}:{}:{}",
        owner,
        product_id,
        license.expires_at,
        license.issued_at,
        tx_sig
    );

    // Sign with admin keypair
    let signature = self.wallet.sign_message(&payload.as_bytes());

    Ok(LicenseProof {
        owner: owner.to_string(),
        product_id: product_id.to_string(),
        expires_at: license.expires_at,
        issued_at: license.issued_at,
        tx_signature: tx_sig,
        proof_signature: bs58::encode(signature).into_string(),
    })
}

/// Verify a license proof offline
pub fn verify_proof(proof: &LicenseProof) -> Result<ValidationResult> {
    // 1. Check expiry
    // 2. Verify signature against admin pubkey
    // 3. Return result
}
```

---

## Security Analysis

| Threat | Mitigation |
|--------|-----------|
| Tampering con archivo local | Encryption + signature check |
| Forgehear signature | Requiere admin private key |
| Clock manipulation | Server time en sync response |
| Replay old proof | Sync endpoint detecta revoke |
| Admin key compromise | Key rotation (con re-emisión) |
| Lossy storage (borrado) | Warning cuando proof falta |

---

## Client Integrations

### Mobile (iOS/Android)

- **iOS:** Keychain for proof storage, Background App Refresh for sync
- **Android:** EncryptedSharedPreferences, WorkManager for sync

### Desktop (macOS/Windows/Linux)

- **Storage:** System keychain (macOS), DPAPI (Windows), libsecret (Linux)
- **Sync:** Background service / daemon

### CLI tool (Rust, Go, Python)

- El SDK en Rust puede importarse como library
- Para Go/Python: wrapper sobre CLI o pre-built binary

### Web (JavaScript/TypeScript)

- **No SDK directo** — JS no puede acceder a admin keypair
- **Flujo:** Llamar CLI o backend para generar proof
- **Storage:** IndexedDB o service worker cache