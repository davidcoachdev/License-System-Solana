# 03 - Domain Model

## Core Entity: License

Una **License** representa el derecho de uso de un producto, emitido a una wallet específica.

### Attributes

| Campo | Tipo | Constraints | Descripción |
|-------|------|-------------|-------------|
| `id` | String | `"{owner}:{product_id}"` | Identificador único |
| `owner` | Pubkey | — | Wallet dueña de la licencia |
| `product_id` | String | max 32 bytes, alnum + `-_` | Producto asociado |
| `expires_at` | i64 | unix timestamp > now | Cuándo expira |
| `is_revoked` | bool | — | Si fue revocada |

### Relationships

```
User (wallet)
    │
    └── 1:N ─── License ─── N:1 ─── Product
```

Un **User** puede tener muchas licencias (una por producto).  
Un **Product** puede tener muchas licencias (una por user).

---

## Invariants

> **I1:** Licencia válida = `expires_at > now` AND `is_revoked == false`

> **I2:** Una vez revocada, NO se puede desproteger (no hay undo)

> **I3:** `owner` es **inmutable** — no existe operación de transfer

> **I4:** `expires_at` solo puede aumentar (extend), nunca diminue

> **I5:** No existen 2 licencias para el mismo `(owner, product_id)`

### Enforcement

| Invariant | enforced by |
|-----------|-----------|
| I1 | Runtime check en `validate()` |
| I2 | `revoke_license` es una via, no hay undo IX |
| I3 | No existe IX `transfer_license` |
| I4 | IX `extend_license` valida `new_expires_at > expires_at` |
| I5 | Unique constraint en PDA: `seeds = [owner, product_id]` |

---

## Lifecycle

```
DORMANT ──(issue_license)──▶ ACTIVE ──(extend_license)──▶ ACTIVE (extended)
                                          │
                                          └────────(revoke_license)──▶ REVOKED
                                          │
                                          └────────(expires_at passed)──▶ EXPIRED
```

Estados válidos para validación:

| Estado | `is_valid()` |
|--------|-------------|
| ACTIVE | ✅ true |
| EXPIRED | ❌ false |
| REVOKED | ❌ false |

---

## Domain Rules

### Rule 1: Solo admin puede emitir

Solo la wallet `ADMIN_WALLET` puede llamar `issue_license`.  
**Cómo se enforced:** La IX valida la firma del admin signer en el programa Anchor.

### Rule 2: Auto-expiry

No hay mantenimiento activo. Las licencias expiran por timestamp.  
**Validación:** Siempre checkear `expires_at > now` antes de otorgar acceso.

### Rule 3: Sin planes en blockchain

Los "planes" son metadata off-chain (en el producto/cliente).  
On-chain solo existe el timestamp de expiración.

### Rule 4: Offline validation usa firma

La licencia se firma con mensaje que incluye `owner + product_id + expires_at`.  
**Verificación:** El cliente firma un challenge y lo verifica contra el estado on-chain o cache local.

---

## Value Objects

### LicenseId

```rust
pub struct LicenseId(String); // formato: "{base58(owner)}:{product_id}"
```

- Unique por `(owner, product_id)`
- Usado como clave en PostgreSQL

### ExpirationTimestamp

```rust
pub struct ExpirationTimestamp(i64); // unix timestamp en segundos
```

- `0` = nunca expira (no usar en Fase 1)
- Máximo: `now() + 10 * 365 * 24 * 3600` (10 años)

### ProductId

```rust
pub struct ProductId(String); // alnum + `-_`, max 32 bytes
```

- Normalizado a lowercase antes de guardar

---

## Aggregate Root

**License** es el aggregate root. Todo acceso a datos de licencia va a través de esta entidad.

No hay sub-entidades en Fase 1 (no hay seat counts, features flags, etc.).

---

## Error Domain

| Código | Descripción |
|--------|-------------|
| `LICENSE_NOT_FOUND` | No existe licencia para `(owner, product_id)` |
| `LICENSE_EXPIRED` | `expires_at <= now` |
| `LICENSE_REVOKED` | `is_revoked == true` |
| `ALREADY_EXISTS` | Ya existe una licencia para `(owner, product_id)` |
| `UNAUTHORIZED_ISSUER` | Quien emite no es admin |
| `PRODUCT_ID_INVALID` | `product_id` tiene caracteres inválidos |
| `PRODUCT_ID_TOO_LONG` | `product_id` > 32 bytes |
| `EXPIRES_AT_IN_PAST` | `new_expires_at` es menor al actual |
| `EXTENSION_TOO_LARGE` | Extensión > 10 años desde ahora |

---

## Anti-corruption Layer (ACL)

El **Frontend/SDK** puede modelar la licencia con un enriched type:

```rust
pub struct EnrichedLicense {
    pub id: LicenseId,
    pub owner: Pubkey,
    pub product_id: ProductId,
    pub expires_at: ExpirationTimestamp,
    pub is_revoked: bool,
    pub is_valid: bool, // computado: I1
    pub days_remaining: i64, // computado
    pub status: LicenseStatus, // ACTIVE | EXPIRED | REVOKED
}

pub enum LicenseStatus {
    Active,
    Expired,
    Revoked,
}
```

El contrato on-chain solo persiste los campos base. La lógica derivada vive en el SDK.

---

##边界

License aggregate boundaries:
- **Entrada:** `issue_license`, `extend_license`, `revoke_license` IXs
- **Salida:** estado on-chain, eventos de programa
- **No sale:** relaciones con productos externos, billing, users

El dominio es deliberadamente estrecho para mantener el contrato on-chain mínimo.