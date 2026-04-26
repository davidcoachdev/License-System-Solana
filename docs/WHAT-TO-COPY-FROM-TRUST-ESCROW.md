# 📋 Qué ADAPTAR de Trust-Work-Escrow

**Basado en**: `/home/dcdebian/Proyects/Trust-Work-Escrow/`  
**Objetivo**: **ADAPTAR** (no copiar directo) las mejores prácticas de trust-escrow a License System

⚠️ **IMPORTANTE**: NO copiar código directamente. Usar como **REFERENCIA** para implementar funcionalidad similar adaptada a nuestro dominio (licencias vs escrow).

---

## ✅ **Ya Copiamos**

- [x] **rustfmt.toml** — code formatting rules
- [x] **clippy.toml** — lint configuration
- [x] **.gitignore** — archivos a ignorar
- [x] **Estructura de docs/** — fases, manual de usuario, comparaciones
- [x] **AGENTS.md** — estrategia de trabajo

---

## 🔴 **CRÍTICO - Copiar YA**

### 1. GitHub Actions Workflows

**Archivos a copiar**:
```
.github/
├── workflows/
│   ├── devnet-deploy.yml        # Auto-deploy a devnet
│   ├── mainnet-deploy.yml       # Deploy controlado a mainnet
│   └── rust-ci.yml              # CI: build, test, lint (CREAR)
├── dependabot.yml               # Actualizaciones automáticas
└── copilot-instructions.md      # Instrucciones para Copilot
```

**Beneficios**:
- ✅ Deploy automático a devnet en cada push a main
- ✅ Deploy controlado a mainnet (requiere aprobación)
- ✅ CI/CD: build, test, lint en cada PR
- ✅ Dependabot: actualizaciones automáticas de dependencias
- ✅ Copilot instructions: contexto para AI

**Acción**:
- [ ] Copiar `.github/workflows/devnet-deploy.yml`
- [ ] Copiar `.github/workflows/mainnet-deploy.yml`
- [ ] Crear `.github/workflows/rust-ci.yml`
- [ ] Copiar `.github/dependabot.yml`
- [ ] Adaptar `.github/copilot-instructions.md`

---

### 2. SDK Modular (Estructura Completa)

**trust-escrow-v2 SDK**:
```
sdk/src/
├── lib.rs           # 101 líneas - Re-exports + constants + tests
├── client.rs        # 2057 líneas - CofreClient + 37 operaciones
├── error.rs         # 11K - Error types + conversions
├── events.rs        # 10K - Event parsing + listener
├── pda.rs           # 13K - PDA helpers (7 PDAs)
├── types.rs         # 16K - Structs (Config, User, Job, Team, etc.)
└── utils.rs         # 13K - send(), fetch(), validation, conversion
```

**Qué copiar**:
- ✅ **Estructura modular** (7 archivos)
- ✅ **client.rs** — patrón de CofreClient con cache + retry
- ✅ **utils.rs** — helpers: `send()`, `make_rpc()`, `fetch_account()`
- ✅ **error.rs** — error types con conversions
- ✅ **pda.rs** — PDA helpers
- ✅ **types.rs** — structs con derives

**Acción**:
- [ ] Crear `sdk/src/client.rs` basado en trust-escrow-v2
- [ ] Crear `sdk/src/utils.rs` con send(), fetch()
- [ ] Crear `sdk/src/error.rs` con conversions
- [ ] Crear `sdk/src/pda.rs` con helpers
- [ ] Crear `sdk/src/types.rs` con License struct

---

### 3. TUI Modular (Estructura Completa)

**trust-escrow-v2 TUI**:
```
tui/src/
├── main.rs              # 22K - Entry point + logger
├── lib.rs               # 729B - Re-exports
├── app/
│   ├── mod.rs           # 3.7K - Module exports
│   ├── state.rs         # 75K - App state + logic
│   ├── events.rs        # 18K - Event handling
│   └── config.rs        # 6.8K - Settings + persistence
└── ui/
    ├── mod.rs           # 11K - Module exports
    ├── layout.rs        # 64K - UI rendering
    ├── navigation.rs    # 35K - Menu navigation
    └── async_integration.rs  # 25K - Async operations
```

**Qué copiar**:
- ✅ **Estructura modular** (app/, ui/)
- ✅ **Logger setup** — tracing a archivo (no stderr)
- ✅ **Config persistence** — load/save JSON
- ✅ **FormField system** — validación avanzada
- ✅ **Screen stack** — navegación con historial
- ✅ **Async integration** — tokio runtime en TUI

**Acción**:
- [ ] Crear `tui/src/app/` módulo
- [ ] Crear `tui/src/ui/` módulo
- [ ] Implementar logger (tracing + tracing-appender)
- [ ] Implementar config persistence
- [ ] Implementar FormField system
- [ ] Implementar screen stack

---

## 🟡 **ALTA PRIORIDAD - Copiar Pronto**

### 4. Scripts Útiles

**trust-escrow tiene**:
```
scripts/
├── deploy-devnet.sh     # Deploy a devnet con checks
├── deploy-mainnet.sh    # Deploy a mainnet con confirmación
├── backup.sh            # Backup de wallets y config
└── test-local.sh        # Test end-to-end local
```

**Qué copiar**:
- ✅ **demo.sh** — script de demostración
- ✅ **deploy-devnet.sh** — deploy con validaciones
- ✅ **test-local.sh** — test end-to-end

**Acción**:
- [ ] Crear `scripts/` directory
- [ ] Copiar y adaptar `demo.sh`
- [ ] Copiar y adaptar `deploy-devnet.sh`
- [ ] Copiar y adaptar `test-local.sh`

---

### 5. Documentación Estructurada

**trust-escrow-v2 docs/**:
```
docs/
├── README.md                    # Índice
├── planning/                    # Planificación
│   ├── PRD.md                   # Product Requirements
│   ├── TDD.md                   # Technical Design
│   ├── SDD.md                   # Software Design
│   └── requirements.md          # Requerimientos
├── architecture/                # Arquitectura
│   ├── SYSTEM_DESIGN.md        # Diseño de sistema
│   ├── DATABASE_SCHEMA.md      # Schema de DB
│   └── API_SPEC.md             # Especificación API
└── implementation/              # Implementación
    ├── SPEC_DRIVER.md          # Specs para IA
    └── IMPLEMENTATION_PLAN.md   # Plan de desarrollo
```

**Qué copiar**:
- ✅ **Estructura de carpetas** (planning/, architecture/, implementation/)
- ✅ **Templates** de PRD, TDD, SDD
- ✅ **SYSTEM_DESIGN.md** — diagrama de flujo
- ✅ **IMPLEMENTATION_PLAN.md** — plan por fases

**Acción**:
- [ ] Crear `docs/planning/` con PRD, TDD, requirements
- [ ] Crear `docs/architecture/` con SYSTEM_DESIGN
- [ ] Crear `docs/implementation/` con IMPLEMENTATION_PLAN

---

### 6. SDK Documentation

**trust-escrow-v2 sdk/docs/**:
```
sdk/docs/
├── getting-started.md           # Quick start guide
├── concepts/
│   ├── escrow-basics.md        # Conceptos básicos
│   └── pda-system.md           # Sistema de PDAs
└── api-reference.md            # API completa
```

**Qué copiar**:
- ✅ **Estructura de docs/** en SDK
- ✅ **getting-started.md** — tutorial paso a paso
- ✅ **concepts/** — explicación de conceptos
- ✅ **api-reference.md** — referencia completa

**Acción**:
- [ ] Crear `sdk/docs/getting-started.md`
- [ ] Crear `sdk/docs/concepts/license-basics.md`
- [ ] Crear `sdk/docs/concepts/pda-system.md`
- [ ] Crear `sdk/docs/api-reference.md`

---

## 🟢 **MEDIA PRIORIDAD - Copiar Después**

### 7. Copilot Instructions

**trust-escrow tiene**:
```markdown
# Trust Work Escrow — Copilot Instructions

## 🏗️ ¿Qué es este proyecto?
## 🧱 Herramientas disponibles
## 📁 Estructura del proyecto
## ✍️ Convenciones de código
## 🔐 Seguridad — Principios
## 🧪 Testing — Patrones
## 🔌 Puertos del entorno
## 🛠 Comandos del proyecto
## 📝 Commits
## 🌿 Ramas
## ⚠️ Pre-commit checklist
## 🚫 Nunca hacer
## 📌 Contexto del proyecto
```

**Qué copiar**:
- ✅ **Estructura completa** del copilot-instructions.md
- ✅ **Convenciones de código**
- ✅ **Principios de seguridad**
- ✅ **Pre-commit checklist**
- ✅ **Comandos útiles**

**Acción**:
- [ ] Crear `.github/copilot-instructions.md` adaptado a License System

---

### 8. LICENSE File

**trust-escrow tiene**:
```
MIT License

Copyright (c) 2026 Trust Work Escrow Team
...
```

**Qué copiar**:
- ✅ **MIT License** (estándar open source)

**Acción**:
- [ ] Crear `LICENSE` file con MIT License

---

### 9. env.example

**trust-escrow tiene**:
```bash
# Solana
SOLANA_RPC_URL=http://127.0.0.1:8899
SOLANA_WALLET_PATH=~/.config/solana/id.json

# Program
PROGRAM_ID=5gu5JCSpB8MKyJzhXpGaCt8SruAMnRD6cTPbwPX6JTYo

# Backend (opcional)
DATABASE_URL=postgresql://user:pass@localhost:5432/escrow
REDIS_URL=redis://localhost:6379
```

**Qué copiar**:
- ✅ **env.example** con variables de entorno

**Acción**:
- [ ] Crear `env.example` con variables necesarias

---

### 10. PROJECT-SUMMARY.md

**trust-escrow tiene**:
```markdown
# Project Summary

## Overview
## Features
## Architecture
## Tech Stack
## Roadmap
## Team
```

**Qué copiar**:
- ✅ **PROJECT-SUMMARY.md** — resumen ejecutivo

**Acción**:
- [ ] Crear `PROJECT-SUMMARY.md`

---

## 🔵 **BAJA PRIORIDAD - Copiar Opcional**

### 11. Shared Crate

**trust-escrow-v2 tiene**:
```
shared/
└── src/
    ├── lib.rs
    ├── constants.rs
    └── utils.rs
```

**Qué copiar**:
- ✅ **shared/** crate para código común

**Acción**:
- [ ] Crear `shared/` crate (futuro)

---

### 12. CLI

**trust-escrow tiene**:
```
cli/
└── src/
    ├── main.rs
    ├── commands/
    └── config.rs
```

**Qué copiar**:
- ✅ **CLI completo** con clap

**Acción**:
- [ ] Crear `cli/` crate (Phase 3)

---

## 📊 **Resumen de Archivos a Copiar**

### Configuración (5 archivos)
- [x] `.gitignore`
- [x] `rustfmt.toml`
- [x] `clippy.toml`
- [ ] `env.example`
- [ ] `LICENSE`

### GitHub (6 archivos)
- [ ] `.github/workflows/devnet-deploy.yml`
- [ ] `.github/workflows/mainnet-deploy.yml`
- [ ] `.github/workflows/rust-ci.yml` (crear)
- [ ] `.github/dependabot.yml`
- [ ] `.github/copilot-instructions.md`
- [x] `.github/workflows/docker-security.yml` (ya tenemos)

### Scripts (4 archivos)
- [ ] `scripts/demo.sh`
- [ ] `scripts/deploy-devnet.sh`
- [ ] `scripts/deploy-mainnet.sh`
- [ ] `scripts/test-local.sh`

### Documentación (8 archivos)
- [ ] `PROJECT-SUMMARY.md`
- [ ] `docs/planning/PRD.md`
- [ ] `docs/planning/TDD.md`
- [ ] `docs/planning/requirements.md`
- [ ] `docs/architecture/SYSTEM_DESIGN.md`
- [ ] `docs/implementation/IMPLEMENTATION_PLAN.md`
- [ ] `sdk/docs/getting-started.md`
- [ ] `sdk/docs/api-reference.md`

### Código (12 archivos)
- [ ] `sdk/src/client.rs` (basado en trust-escrow-v2)
- [ ] `sdk/src/error.rs`
- [ ] `sdk/src/pda.rs`
- [ ] `sdk/src/types.rs`
- [ ] `sdk/src/utils.rs`
- [ ] `tui/src/app/mod.rs`
- [ ] `tui/src/app/state.rs`
- [ ] `tui/src/app/events.rs`
- [ ] `tui/src/app/config.rs`
- [ ] `tui/src/ui/mod.rs`
- [ ] `tui/src/ui/layout.rs`
- [ ] `shared/src/lib.rs` (futuro)

**Total**: 35 archivos a copiar/crear

---

## 🎯 **Priorización**

### Sprint 1 (CRÍTICO) — 2-3 días
1. ✅ Linter configs (ya copiados)
2. [ ] GitHub Actions workflows (devnet-deploy, rust-ci)
3. [ ] SDK modular (client, error, pda, types, utils)
4. [ ] Transacciones reales en SDK

### Sprint 2 (ALTA) — 1-2 días
1. [ ] TUI modular (app/, ui/)
2. [ ] Copilot instructions
3. [ ] Scripts útiles (demo, deploy, test)
4. [ ] Dependabot

### Sprint 3 (MEDIA) — 1-2 días
1. [ ] Documentación estructurada (planning/, architecture/)
2. [ ] SDK docs (getting-started, api-reference)
3. [ ] PROJECT-SUMMARY.md
4. [ ] LICENSE file

### Sprint 4 (BAJA) — Futuro
1. [ ] Shared crate
2. [ ] CLI
3. [ ] env.example

---

## 📝 **Adaptaciones Necesarias**

Al copiar archivos, adaptar:

### Program IDs
```yaml
# trust-escrow
PROGRAM_ID: "5gu5JCSpB8MKyJzhXpGaCt8SruAMnRD6cTPbwPX6JTYo"

# license-system
PROGRAM_ID: "5pXEX8z1aTSnm7jCKqvJCXezczKPVuPQif2BZh5u5Axq"
```

### Nombres de Crates
```toml
# trust-escrow
trust_escrow_v2 = "..."

# license-system
license_system = "..."
```

### Paths
```yaml
# trust-escrow
directory: "/trust-escrow"

# license-system
directory: "/license-system"
```

### Instrucciones
```rust
// trust-escrow: 37 instrucciones
// license-system: 4 instrucciones

// Adaptar client.rs para solo 4 operaciones:
// - issue_license
// - extend_license
// - revoke_license
// - validate_license
```

---

## 🚀 **Plan de Implementación**

### Día 1: GitHub Actions + SDK Modular
**Tiempo**: 8 horas

1. Copiar `.github/workflows/devnet-deploy.yml` (1 hora)
2. Crear `.github/workflows/rust-ci.yml` (1 hora)
3. Copiar `.github/dependabot.yml` (30 min)
4. Crear `sdk/src/client.rs` (3 horas)
5. Crear `sdk/src/utils.rs` (1 hora)
6. Crear `sdk/src/error.rs`, `pda.rs`, `types.rs` (1.5 horas)

### Día 2: TUI Modular + Transacciones Reales
**Tiempo**: 8 horas

1. Modularizar TUI (app/, ui/) (3 horas)
2. Conectar TUI con SDK real (2 horas)
3. Implementar transacciones reales (2 horas)
4. Probar end-to-end (1 hora)

### Día 3: Scripts + Docs + Deploy
**Tiempo**: 8 horas

1. Crear scripts/ (demo, deploy, test) (2 horas)
2. Crear copilot-instructions.md (1 hora)
3. Crear docs estructurados (3 horas)
4. Deploy a devnet (1 hora)
5. Verificar funcionamiento (1 hora)

---

## 📋 **Checklist de Copiado**

### Configuración
- [x] `.gitignore`
- [x] `rustfmt.toml`
- [x] `clippy.toml`
- [ ] `env.example`
- [ ] `LICENSE`

### GitHub
- [ ] `.github/workflows/devnet-deploy.yml`
- [ ] `.github/workflows/mainnet-deploy.yml`
- [ ] `.github/workflows/rust-ci.yml`
- [ ] `.github/dependabot.yml`
- [ ] `.github/copilot-instructions.md`

### Scripts
- [ ] `scripts/demo.sh`
- [ ] `scripts/deploy-devnet.sh`
- [ ] `scripts/test-local.sh`

### SDK
- [ ] `sdk/src/client.rs`
- [ ] `sdk/src/error.rs`
- [ ] `sdk/src/pda.rs`
- [ ] `sdk/src/types.rs`
- [ ] `sdk/src/utils.rs`
- [ ] `sdk/docs/getting-started.md`
- [ ] `sdk/docs/api-reference.md`

### TUI
- [ ] `tui/src/app/mod.rs`
- [ ] `tui/src/app/state.rs`
- [ ] `tui/src/app/events.rs`
- [ ] `tui/src/app/config.rs`
- [ ] `tui/src/ui/mod.rs`
- [ ] `tui/src/ui/layout.rs`

### Docs
- [ ] `PROJECT-SUMMARY.md`
- [ ] `docs/planning/PRD.md`
- [ ] `docs/architecture/SYSTEM_DESIGN.md`
- [ ] `docs/implementation/IMPLEMENTATION_PLAN.md`

**Total**: 3/35 completados (9%)

---

## 🎯 **Beneficios de Copiar**

### GitHub Actions
- ✅ Deploy automático a devnet
- ✅ CI/CD en cada PR
- ✅ Dependabot mantiene deps actualizadas
- ✅ Verificación de binary (hash matching)

### SDK Modular
- ✅ Código mantenible (7 archivos vs 1)
- ✅ Transacciones reales (no demo)
- ✅ Cache + retry logic
- ✅ Event listener

### TUI Modular
- ✅ Código organizado (app/, ui/)
- ✅ Logging sin romper TUI
- ✅ Config persistence
- ✅ FormField system

### Scripts
- ✅ Deploy automatizado
- ✅ Testing simplificado
- ✅ Demo para presentaciones

---

## 📝 **Notas Importantes**

### Al Copiar GitHub Actions
1. Cambiar `PROGRAM_ID` en workflows
2. Configurar secrets en GitHub:
   - `DEVNET_DEPLOY_KEYPAIR`
   - `MAINNET_DEPLOY_KEYPAIR`
3. Crear environments en GitHub:
   - `devnet` (sin aprobación)
   - `mainnet` (con aprobación manual)

### Al Copiar SDK
1. Simplificar: trust-escrow tiene 37 instrucciones, nosotros solo 4
2. Mantener la estructura modular
3. Usar RpcClient directo (no anchor-client)

### Al Copiar TUI
1. Adaptar screens: trust-escrow tiene 20+, nosotros solo 6
2. Mantener la modularización (app/, ui/)
3. Implementar logger desde el inicio

---

**Última actualización**: 2026-04-26  
**Status**: 3/35 archivos copiados (9%)  
**Próximo paso**: Copiar GitHub Actions workflows
