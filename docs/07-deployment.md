# 07 - Deployment & Operations

## Environments

| Environment | Purpose | Solana | Config |
|------------|--------|--------|
| `local` | Desarrollo local | Dev keypair, test validator |
| `devnet` | testing | Programas de test, no real funds |
| `mainnet` | producción | Programa production, real SOL |

---

## Program Deployment

### Anchor Program IDs

| Environment | Program ID |
|------------|-----------|
| `local` | generado con `anchor test` |
| `devnet` | `dev1...` (registrar en `lib.rs`) |
| `mainnet` | `pro1...` (registrar en `lib.rs`) |

### Deployment Steps (Devnet)

```bash
# 1. Build
cargo build-bpf --program-name license_program

# 2. Deploy
solana program deploy \
  --keypair ~/.config/solana/id.json \
  --url devnet \
  target/deploy/license_program.so

# 3. Update PROGRAM_ID en lib.rs y rebuild SDK
# 4. Verify
solana program show <PROGRAM_ID> --url devnet
```

### Deployment Steps (Mainnet)

```bash
# 1. Audit completo (ver docs/06-testing-strategy.md)
# 2. Testnet first
solana program deploy \
  --keypair ~/.config/solana/id.json \
  --url testnet \
  target/deploy/license_program.so

# 3. Run full integration suite contra testnet
# 4. Approve by 2 reviewers
# 5. Mainnet deploy
solana program deploy \
  --keypair ~/.config/solana/admin-prod.json \
  --url mainnet \
  target/deploy/license_program.so

# 6. Verify
solana program show <PROGRAM_ID> --url mainnet
# 7. Announce deployment
```

---

## Backend Deployment (Fase 3)

### Container

```dockerfile
FROM rust:1.75-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p license-backend

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/license-backend /usr/local/bin/
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
EXPOSE 8080
CMD ["license-backend"]
```

### Kubernetes (optional)

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: license-backend
spec:
  replicas: 2
  template:
    spec:
      containers:
        - name: backend
          image: ghcr.io/user/license-backend:latest
          ports:
            - containerPort: 8080
          env:
            - name: RPC_URL
              valueFrom:
                secretKeyRef:
                  name: license-secrets
                  key: rpc-url
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: license-secrets
                  key: database-url
            - name: ADMIN_KEYPAIR
              valueFrom:
                secretKeyRef:
                  name: license-secrets
                  key: admin-keypair
```

---

## Database Migrations

### Tool

Usar `sqlx-cli` o `diesel` para migrations.

```bash
# Apply pending migrations
sqlx migrate run

# Create new migration
sqlx migrate add create_licenses_table

# Revert last
sqlx migrate revert
```

### Migration Files

```
migrations/
├── 000_create_licenses_table.sql
├── 001_add_tx_signature.sql
└── 002_add_issued_at.sql
```

---

## Secrets Management

### Local

`.env` file (nunca commitear):
```
RPC_URL=https://api.devnet.solana.com
PROGRAM_ID=dev1...
ADMIN_KEYPAIR=[base58 keypair]
DATABASE_URL=postgres://...
JWT_SECRET=...
```

### Production

Usar vault o secrets manager:
- **AWS:** Secrets Manager / Parameter Store
- **GCP:** Secret Manager
- **Self-hosted:** Vault

```bash
# Ejemplo: cargar secrets de vault
vault kv get -field=admin-keypair secret/license/prod/admin > admin_keypair.json
```

---

## Monitoring

### Health Checks

```
GET /health → 200 OK
{
  "status": "ok",
  "solana": "connected",
  "database": "connected",
  "version": "0.1.0"
}
```

### Metrics Endpoint

`GET /metrics` (Prometheus format)

### Alerts (ejemplos)

| Alert | Condition | Severity |
|-------|----------|---------|
| `solana_rpc_down` | RPC unreachable > 5min | P1 |
| `indexer_lag_high` | DB sync lag > 60s | P2 |
| `high_error_rate` | Error rate > 5% | P2 |
| `license_expiry_soon` | > 10 licenses expiring today | P3 |

### Dashboard (Grafana)

- **Solana:** TX confirmation rate, errors
- **Backend:** Request rate, latency p99
- **DB:** Connections, query latency
- **Indexer:** Events processed, lag

---

## Backup & Recovery

### Database Backup

```bash
# Backup diario (cronjob)
pg_dump -Fc license > backup_$(date +%Y%m%d).dump

# Retention: 30 days local, 90 days cold storage
```

### Recovery Plan

```
1. Detectar pérdida de DB
2. Parar indexer
3. Restaurar desde backup más reciente
4. Correr replay de eventos desde slot del backup
5. Verificar consistencia con Solana
6. Restart indexer
```

### Solana State Recovery

No hay forma de restaurar Solana. Si el programa tiene bugs:
1. Deploy fix con nuevo program ID
2. Migrar datos manualmente si es necesario (nuevo programa tiene su propio estado)

---

## Deployment Checklist

Pre-deploy:
- [ ] Tests pasan en CI
- [ ] Coverage es acceptable
- [ ] Code review aprobado
- [ ] Changelog actualizado
- [ ] Version bump en `Cargo.toml`
- [ ] Secrets configurados
- [ ] Health check responde
- [ ] Monitoring configurado
- [ ] Rollback plan documentado

Post-deploy:
- [ ] Verify deployment
- [ ] Smoke test pasa
- [ ] Monitor metrics
- [ ] Announce a stakeholders

---

## Versioning

### Semantic Versioning

```
major.minor.patch
  │      │     └── Bug fixes
  │      └── Features (backward compatible)
  └── Breaking changes
```

### Release Process

```bash
# Tag
git tag -a v0.1.0 -m "Release v0.1.0: initial program + CLI"
git push origin v0.1.0

# GitHub Release
gh release create v0.1.0 --title "v0.1.0" --notes "$(cat CHANGELOG.md)"
```

### API Versioning

Base path: `/api/v1/`  
Si hay breaking change: `/api/v2/`

No breaking changes en Fase 1 (API no existe aún).

---

## Rollback

### Backend

```bash
# Revert container
kubectl rollout undo deployment/license-backend

# Revert migrations
sqlx migrate revert
```

### Program (Solana)

Solana programs son inmutables post-deploy. Para rollback:
1. Desplegar programa anterior en nuevo ID
2. Migrar estado (si aplica)
3. Actualizar referencias

No hay hotfix rápido en Solana. Testing exhaustivo antes de deploy es crítico.

---

## Environments Config

### Devnet (desarrollo)

```toml
# .env.devnet
RPC_URL=https://api.devnet.solana.com
PROGRAM_ID=dev1...your-dev-program-id
ADMIN_KEYPAIR=<your keypair base58>
RUST_LOG=debug
```

### Mainnet (producción)

```toml
# .env.mainnet
RPC_URL=https://api.mainnet-beta.solana.com
PROGRAM_ID=pro1...your-production-id
DATABASE_URL=postgres://user:pass@rds.amazonaws.com/license
JWT_SECRET=<32 bytes hex>
RUST_LOG=info
HELIUS_WEBHOOK_SECRET=<secret>
```

---

## Logging

### Log Levels

| Level | Uso |
|-------|-----|
| `error` | Errores que requieren atención |
| `warn` | Condiciones inesperadas pero recovery ok |
| `info` | Operaciones normales |
| `debug` | Detalle para debugging (off en prod) |
| `trace` | Verbose debugging (off en prod) |

### Structured Log Format

```json
{
  "ts": "2024-01-01T00:00:00.000Z",
  "level": "INFO",
  "msg": "License issued",
  "owner": "7x4Jb...",
  "product_id": "pro-basic",
  "tx_sig": "abc...",
  "request_id": "uuid",
  "duration_ms": 450
}
```

---

## Incident Response

### Severity Levels

| Severity | Description | Response Time |
|----------|------------|-------------|
| P1 | Sistema down | 15 min |
| P2 | Funcionalidad degradada | 1 hora |
| P3 | Bug menor | 24 horas |
| P4 | Mejora | 1 semana |

### Runbook

**P1 - TXs no confirmando:**
1. Check RPC status (solana.com)
2. Check programa en explorer
3. Verificar admin wallet balance
4. Check recent program deployments
5. Engage Solana support si es RPC issue

**P1 - Indexer desincronizado:**
1. Detener indexer
2. Identificar último slot sincronizado
3. Resume desde slot conocido
4. Investigar causa (Helius issue, DB issue)