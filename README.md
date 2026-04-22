# 🧾 License System on Solana

Sistema de gestión de licencias basado en Solana usando Anchor (Rust), con arquitectura híbrida y enfoque en herramientas reales (CLI/TUI + SDK).

---

## 🧠 Visión

Este proyecto implementa un sistema de licencias moderno que combina:

* **Solana (Anchor)** como fuente de verdad
* **Backend** como orquestador
* **SDK en Rust** como capa de abstracción
* **TUI/CLI** como herramienta principal de administración
* **Frontend web** como interfaz de usuario

---

## 🎯 Objetivos

* Emitir, extender y revocar licencias on-chain
* Validación rápida (offline-first con firma)
* Arquitectura escalable y desacoplada
* Herramientas de administración reales (CLI/TUI)
* Base sólida para producto SaaS

---

## 🏗️ Arquitectura General

```
[Frontend] ───► [Backend] ───► [Solana Program]
     │                │
     ▼                ▼
 [Wallet]         [PostgreSQL]

[TUI/CLI] ─────► [SDK Rust] ─► [Solana Program]
```

---

## 🧱 Componentes

### 1. Program (Anchor - Solana)

Responsabilidad:

* Fuente de verdad
* Estado mínimo
* Operaciones críticas

#### Modelo de datos

```rust
pub struct License {
    pub owner: Pubkey,
    pub product_id: String,
    pub expires_at: i64,
    pub is_revoked: bool,
}
```

#### Instrucciones

* `issue_license`
* `extend_license`
* `revoke_license`

---

### 2. SDK (Rust)

Responsabilidad:

* Abstraer interacción con Solana
* Reutilizable por CLI, backend, etc.

```rust
pub struct LicenseClient {
    pub rpc_url: String,
    pub wallet: Keypair,
}
```

---

### 3. CLI / TUI

Responsabilidad:

* Herramienta principal de administración
* Interfaz para operadores del sistema

#### Funcionalidades

* Emitir licencia
* Extender licencia
* Revocar licencia
* Cambiar wallet
* Configurar RPC

---

### 4. Backend (Opcional - Fase 2)

Responsabilidad:

* Procesar pagos
* Indexar datos
* Exponer API

#### Ejemplo endpoints

```
POST /licenses
POST /licenses/extend
GET /licenses/:id
```

---

### 5. Frontend (Opcional)

Ubicación:

```
/apps/web
```

Responsabilidad:

* Conectar wallet del usuario
* Interfaz de compra/gestión

---

## 🗄️ Base de Datos (PostgreSQL)

```sql
CREATE TABLE licenses (
  id TEXT PRIMARY KEY,
  owner TEXT,
  product_id TEXT,
  expires_at BIGINT,
  is_revoked BOOLEAN
);
```

---

## 🔐 Modelo de Wallets

| Rol     | Wallet               |
| ------- | -------------------- |
| Usuario | Phantom / Web wallet |
| Admin   | CLI / TUI keypair    |
| Backend | Wallet del sistema   |

---

## 🔄 Flujos Principales

---

### 🛒 Compra de Licencia

1. Usuario conecta wallet (frontend)
2. Backend procesa pago
3. Backend llama a `issue_license`
4. Solana guarda estado
5. Backend responde al frontend

---

### 🔑 Validación

* Validación local (firma)
* Opcional: validación online

---

### 🧑‍💻 Administración (CLI)

1. Admin selecciona wallet
2. Ejecuta comando
3. SDK firma y envía transacción
4. Solana actualiza estado

---

## 🧠 Decisiones de Diseño

* ❌ No guardar planes en contrato
* ✅ Usar `expires_at`
* ✅ SDK en Rust
* ✅ Separación clara de capas
* ✅ Backend como orquestador
* ✅ PostgreSQL como read model

---

## 📁 Estructura del Proyecto

```
/license-system
│
├── program/        # Anchor program
├── crates/
│   ├── sdk/        # SDK Rust
│   ├── cli/        # CLI / TUI
│   └── backend/    # API (opcional)
│
├── apps/
│   └── web/        # Frontend
│
├── docs/
│   └── architecture.md
│
└── Cargo.toml      # Workspace
```

---

## 🚀 Roadmap

### Fase 1 (Bootcamp)

* [x] Anchor program
* [x] CLI básica
* [x] Deploy en Devnet

### Fase 2

* [ ] SDK completo
* [ ] TUI interactiva
* [ ] Wallet manager

### Fase 3

* [ ] Backend
* [ ] Licencias firmadas
* [ ] Frontend

---

## 🔥 Diferenciadores

* TUI como herramienta principal
* SDK en Rust (no duplicación)
* Arquitectura híbrida
* Validación offline
* Preparado para producción

---

## 📌 TL;DR

* Solana = fuente de verdad
* Backend = lógica
* SDK = integración
* TUI = operación
* Frontend = UX

---

## 💥 Autor

Proyecto diseñado como sistema real de licencias, no solo demo de blockchain.

---
