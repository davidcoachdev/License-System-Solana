# Phase 3: Backend + Frontend + Production

**Status**: ⏸️ NO INICIADA (0%)

**Objetivo**: Backend API, frontend web, licencias firmadas offline, PostgreSQL indexer, monitoring.

---

## 📋 Checklist

### ⏸️ Backend API (Rust/Axum)
- [ ] Estructura del crate `crates/backend/`
- [ ] API REST con Axum
- [ ] Endpoints:
  - [ ] `POST /licenses` — emitir licencia
  - [ ] `POST /licenses/:id/extend` — extender licencia
  - [ ] `POST /licenses/:id/revoke` — revocar licencia
  - [ ] `GET /licenses/:id` — obtener licencia
  - [ ] `GET /licenses` — listar licencias
  - [ ] `POST /licenses/:id/validate` — validar licencia
- [ ] Integración con SDK
- [ ] Autenticación (JWT)
- [ ] Rate limiting
- [ ] CORS configurado
- [ ] Documentación OpenAPI/Swagger

### ⏸️ PostgreSQL Indexer
- [ ] Schema de base de datos
- [ ] Indexer que escucha eventos del program
- [ ] Sincronización de estado on-chain → PostgreSQL
- [ ] Queries optimizadas
- [ ] Migraciones con sqlx

### ⏸️ Frontend Web (Next.js)
- [ ] Estructura del proyecto `apps/web/`
- [ ] Conexión de wallet (Phantom, Solflare)
- [ ] UI para comprar licencias
- [ ] UI para gestionar licencias
- [ ] Dashboard de admin
- [ ] Integración con backend API
- [ ] Responsive design

### ⏸️ Licencias Firmadas (Offline Validation)
- [ ] Generación de licencias firmadas
- [ ] Validación offline sin RPC
- [ ] Formato de licencia (JWT o custom)
- [ ] Revocation list
- [ ] SDK para validación offline

### ⏸️ Monitoring & Analytics
- [ ] Prometheus metrics
- [ ] Grafana dashboards
- [ ] Alerting (PagerDuty/Slack)
- [ ] Logs centralizados
- [ ] Error tracking (Sentry)

---

## 📁 Estructura de Archivos

```
crates/
├── backend/                # Backend API (Rust/Axum)
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes/
│   │   ├── models/
│   │   ├── db/
│   │   └── middleware/
│   ├── Cargo.toml
│   └── README.md
│
apps/
└── web/                    # Frontend (Next.js)
    ├── src/
    │   ├── app/
    │   ├── components/
    │   └── lib/
    ├── package.json
    └── README.md
```

---

## 🎯 Objetivos de Phase 3

### Backend API
**Propósito**: Orquestador entre frontend y Solana, procesar pagos, indexar datos.

**Stack**:
- Rust + Axum (framework web)
- PostgreSQL (read model)
- Redis (cache)
- JWT (autenticación)

### Frontend Web
**Propósito**: Interfaz para usuarios finales (comprar/gestionar licencias).

**Stack**:
- Next.js 14 (App Router)
- Tailwind CSS
- Wallet Adapter (@solana/wallet-adapter)
- React Query

### Licencias Firmadas
**Propósito**: Validación offline sin necesidad de RPC (para apps que no tienen internet).

**Formato**:
```json
{
  "license_id": "lic_123",
  "owner": "pubkey",
  "product_id": "product-001",
  "expires_at": 1735689600,
  "signature": "base64_signature"
}
```

---

## 🔧 Comandos Útiles

### Backend
```bash
cd crates/backend
cargo run --release

# Con hot reload
cargo watch -x run
```

### Frontend
```bash
cd apps/web
npm run dev
```

### PostgreSQL
```bash
# Iniciar con Docker
docker run -d \
  --name license-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  postgres:16

# Migraciones
sqlx migrate run
```

---

## 📊 Métricas

- **Backend Progress**: 0%
- **Frontend Progress**: 0%
- **Indexer Progress**: 0%
- **Offline Validation Progress**: 0%
- **Monitoring Progress**: 0%

---

## 🎯 Criterios de Completitud

- [ ] Backend API funcional
- [ ] PostgreSQL indexer sincronizando
- [ ] Frontend conecta wallet
- [ ] Frontend puede comprar licencias
- [ ] Frontend puede gestionar licencias
- [ ] Licencias firmadas funcionan offline
- [ ] Monitoring configurado
- [ ] Deploy a producción

**Completitud**: 0/8 (0%)

---

## 🚀 Plan de Implementación

### Paso 1: Backend API (Prioridad Alta)
1. Crear estructura `crates/backend/`
2. Configurar Axum + PostgreSQL
3. Implementar endpoints REST
4. Integrar con SDK
5. Agregar autenticación JWT
6. Tests de integración

**Tiempo estimado**: 1 semana

### Paso 2: PostgreSQL Indexer (Prioridad Alta)
1. Diseñar schema
2. Implementar listener de eventos
3. Sincronización inicial
4. Manejo de reorgs
5. Tests

**Tiempo estimado**: 3-4 días

### Paso 3: Frontend Web (Prioridad Media)
1. Setup Next.js
2. Integrar Wallet Adapter
3. UI para comprar licencias
4. UI para gestionar licencias
5. Dashboard de admin

**Tiempo estimado**: 1 semana

### Paso 4: Licencias Firmadas (Prioridad Baja)
1. Diseñar formato
2. Implementar generación
3. Implementar validación offline
4. SDK para clientes

**Tiempo estimado**: 2-3 días

### Paso 5: Monitoring (Prioridad Media)
1. Prometheus metrics
2. Grafana dashboards
3. Alerting
4. Error tracking

**Tiempo estimado**: 2 días

---

## 📝 Notas Técnicas

### Backend Stack
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
serde = { version = "1", features = ["derive"] }
jsonwebtoken = "9"
tower-http = { version = "0.5", features = ["cors"] }
license-sdk = { path = "../sdk" }
```

### Frontend Stack
```json
{
  "dependencies": {
    "next": "14.0",
    "react": "18.0",
    "@solana/wallet-adapter-react": "^0.15",
    "@solana/wallet-adapter-wallets": "^0.19",
    "@tanstack/react-query": "^5.0",
    "tailwindcss": "^3.0"
  }
}
```

### PostgreSQL Schema
```sql
CREATE TABLE licenses (
  id TEXT PRIMARY KEY,
  owner TEXT NOT NULL,
  product_id TEXT NOT NULL,
  expires_at BIGINT NOT NULL,
  is_revoked BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  signature TEXT,
  
  INDEX idx_owner (owner),
  INDEX idx_product (product_id),
  INDEX idx_expires (expires_at)
);

CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  signature TEXT UNIQUE NOT NULL,
  license_id TEXT REFERENCES licenses(id),
  instruction_type TEXT NOT NULL,
  timestamp TIMESTAMP DEFAULT NOW()
);
```

---

## 🎨 Frontend Mockup

```
┌─────────────────────────────────────────────────┐
│ License System                    [Connect Wallet]│
└─────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│ My Licenses                                     │
│                                                 │
│ ┌─────────────────────────────────────────────┐ │
│ │ Product: Premium Plan                       │ │
│ │ Status: ✅ Active                           │ │
│ │ Expires: 2026-05-26                         │ │
│ │ [Extend] [Revoke]                           │ │
│ └─────────────────────────────────────────────┘ │
│                                                 │
│ ┌─────────────────────────────────────────────┐ │
│ │ Product: Basic Plan                         │ │
│ │ Status: ❌ Expired                          │ │
│ │ Expires: 2026-03-15                         │ │
│ │ [Renew]                                     │ │
│ └─────────────────────────────────────────────┘ │
│                                                 │
│ [+ Buy New License]                             │
└─────────────────────────────────────────────────┘
```

---

## 🔐 Security Considerations

### Backend
- Rate limiting por IP
- JWT con refresh tokens
- Input validation estricta
- SQL injection prevention (sqlx)
- CORS configurado correctamente

### Frontend
- Wallet signature verification
- No exponer private keys
- HTTPS only
- CSP headers

### Licencias Firmadas
- Ed25519 signatures
- Revocation list actualizada
- Timestamp validation
- Replay attack prevention

---

## 🚀 Deployment

### Backend
```bash
# Docker
docker build -t license-backend .
docker run -p 8080:8080 license-backend

# Kubernetes
kubectl apply -f k8s/
```

### Frontend
```bash
# Vercel
vercel deploy --prod

# Docker
docker build -t license-frontend .
docker run -p 3000:3000 license-frontend
```

---

**Última actualización**: 2026-04-26
**Autor**: License System Team
**Nota**: Phase 3 se iniciará después de completar Phase 2
