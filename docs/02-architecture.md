# 02 - Architecture

## Principles

1. **Solana es source of truth** — estado canonical en chain
2. **Backend como orquestador** — lógica de negocio, pagos, indexado
3. **SDK en Rust como integración** — single source of truth para CLI y backend
4. **Offline-first** — validación sin internet es el diferenciador
5. **Event-driven sync** — PostgreSQL como read model

---

## Component Map

```
┌─────────────────────────────────────────────────────────────────┐
│                      USERS                                 │
├──────────┬───────────┬──────────────┬───────────────────────┤
│ End User │   Admin   │  Developer  │      External          │
│          │           │             │ (payment, webhooks)   │
└────┬─────┴─────┬─────┴──────┬─────┴──────────┬──────────┘
     │           │            │                │
     ▼           ▼            ▼                ▼
┌────────┐  ┌────────┐  ┌─────────┐    ┌──────────┐
│Frontend│  │ TUI/CLI│  │ SDK App │    │ Webhooks │
│  (web) │  │(admin) │  │ (lib)   │    │ (out)    │
└───┬────┘  └────┬───┘  └────┬────┘    └────┬─────┘
    │             │          │               │
    │             │          │               │
    ▼             ▼          ▼               ▼
┌─────────┐  ┌─────────┐              ┌──────────┐
│ Backend │──│   SDK   │───────────────▶│ Solana   │
│   API   │◀│  Rust   │               │ Program  │
└────┬────┘  └─────────┘               └─────────┘
     │                                      ▲
     ▼                                      │
┌──────────┐  ┌───────────┐          ┌──────────┐
│PostgreSQL│◀─│ Indexer   │───────────▶│ (events) │
│(read    │  │ (async)   │          │          │
│model)   │  └───────────┘           └──────────┘
└─────────┘
```

---

## Components

### Anchor Program

**Responsabilidad:** Fuente de verdad, estado mínimo on-chain

**Account:** `License` (PDA)
```rust
pub struct License {
    pub owner: Pubkey,        // wallet dueña
    pub product_id: String,   // 32 bytes max
    pub expires_at: i64,      // unix timestamp
    pub is_revoked: bool,     // revocada?
    pub bump: u8,            // PDA bump
}
```

**Instructions:**
| IX | Params | Auth | Descripción |
|----|--------|------|-------------|
| `issue_license` | owner, product_id, expires_at | Admin signer | Crea PDA |
| `extend_license` | owner, product_id, new_expires_at | Admin signer | Extiende expires_at |
| `revoke_license` | owner, product_id | Admin signer | Marca revocada |

**Programa ID:** configurable via `PROGRAM_ID` constant

---

### SDK (Rust)

**Responsabilidad:** Abstraer interacción con Solana, reusable en CLI y backend

**Crate:** `license-sdk`

```rust
// API pública
pub struct LicenseClient {
    rpc_url: String,
    wallet: Keypair,
    program_id: Pubkey,
}

impl LicenseClient {
    pub fn issue(&self, owner: Pubkey, product_id: &str, days: u32) -> Result<Signature>
    pub fn extend(&self, owner: Pubkey, product_id: &str, days: u32) -> Result<Signature>
    pub fn revoke(&self, owner: Pubkey, product_id: &str) -> Result<Signature>
    pub fn get_license(&self, owner: Pubkey, product_id: &str) -> Result<Option<License>>
    pub fn validate(&self, owner: Pubkey, product_id: &str) -> Result<bool>
}
```

**Dependencias:** `anchor-client`, `solana-sdk`, `serde`, `thiserror`

---

### Backend API

**Responsabilidad:** Orquestar operaciones, procesar pagos, indexar, exponer API

**Stack:** Rust (Axum) o Go (Gin) — **TBD en Fase 3**

**Endpoints (Fase 3):**
| Method | Path | Auth | Descripción |
|--------|------|------|-------------|
| POST | `/api/v1/licenses` | Admin JWT | Emitir licencia |
| POST | `/api/v1/licenses/:id/extend` | Admin JWT | Extender |
| DELETE | `/api/v1/licenses/:id` | Admin JWT | Revocar |
| GET | `/api/v1/licenses/:id` | — | Consultar estado |
| POST | `/api/v1/validate` | — | Validar (online) |

**Auth:** JWT con expiry para sesiones admin; wallet signature para usuarios

---

### TUI/CLI

**Responsabilidad:** Herramienta principal de administración

**Stack:** Rust + Ratatui (TUI) + Clap (CLI)

**Comandos:**
```
license-cli issue <owner> <product_id> <days>
license-cli extend <owner> <product_id> <days>
license-cli revoke <owner> <product_id>
license-cli validate <owner> <product_id>
license-cli list
license-cli wallet switch <name>
license-cli config set rpc <url>
```

---

### Indexer

**Responsabilidad:** Sync Solana → PostgreSQL

**Estrategia:** Helius Webhooks (recomendado) o polling fallback

**Flujo:**
```
Solana tx → Helius → Backend endpoint → PostgreSQL upsert → Webhook dispatch
```

**Eventos:**
- `license.created`
- `license.extended`
- `license.revoked`

---

### PostgreSQL (Read Model)

**Tabla:** `licenses`
```sql
CREATE TABLE licenses (
  id TEXT PRIMARY KEY,          -- "{owner}:{product_id}"
  owner TEXT NOT NULL,
  product_id TEXT NOT NULL,
  expires_at BIGINT NOT NULL,
  is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
  tx_signature TEXT NOT NULL,
  updated_at BIGINT NOT NULL,
  created_at BIGINT NOT NULL,
  UNIQUE(owner, product_id)
);
```

---

## Data Flow

### Emisión de Licencia

```
Admin (TUI/CLI)
    │
    ▼
SDK.issue(owner, product_id, days)
    │
    ▼
Solana program: issue_license IX
    │
    ▼
Solana execution + PDA creation
    │
    ▼
[async] Helius webhook ──▶ Backend ──▶ PostgreSQL
                                   │
                                   ▼
                              Webhook dispatch
                                   │
                                   ▼
                              External services
```

### Validación Offline

```
Software client
    │
    ▼
SDK.validate(owner, product_id)
    │
    ▼
Verifica firma + timestamp localmente
    │
    ▼
✓ acceso │ ✗ denegado
```

### Validación Online

```
Software client
    │
    ▼
Backend POST /validate
    │
    ▼
SDK.get_license → Solana RPC
    │
    ▼
Verifica state + timestamp
    │
    ▼
✓ acceso │ ✗ denegado
```

---

## Wallet Model

| Rol | Wallet | Ubicación |
|-----|--------|----------|
| End User | Phantom / Solflare | Browser wallet |
| Admin | Keypair (file) | TUI config dir |
| Backend | Keypair (env/file) | Server secrets |

**Admin wallet** se configura en TUI y se carga desde archivo encriptado o env var.

---

## Deployment Topology

```
[Devnet]
├── Solana program
├── TUI (local)
└── Backend (local, Fase 3)

[Mainnet]
├── Solana program
├── Backend (server)
├── PostgreSQL ( RDS)
├── Indexer (Helius)
└── TUI (admin local)
```

---

## Trade-offs Documentados

| Decisión | Beneficiada | Costo |
|----------|------------|-------|
| Backend firma transacciones | UX simple | Menos descentralización |
| PostgreSQL como read model | Performance, queries | Consistencia eventual |
| SDK en Rust único | No duplicación | No usable directo en web |
| Indexer event-driven | Escalabilidad | Complejidad operacional |
| Offline validation | Resiliencia, speed | Requiere integración en cliente |