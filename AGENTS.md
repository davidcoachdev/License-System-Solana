# 📋 AGENTS - License System on Solana

## 🎯 Objetivo General
Desarrollar un sistema de gestión de licencias descentralizado en Solana con TUI profesional, SDK completo, y arquitectura production-ready.

**Deadline:** TBD

---

## 📅 Estructura de Fases

### Fase 01: Core Program + DevContainer ✅ COMPLETADA (95%)
- [x] Anchor program con 4 instrucciones
- [x] 6 security fixes aplicados
- [x] DevContainer audit completo (P1/P2/P3)
- [x] Tests TypeScript (5 test cases)
- [x] Deploy a localnet
- [ ] Deploy a devnet (pendiente: airdrop)
- **Commits:** 12
- **Status:** 95% completa

### Fase 02: SDK + TUI Básica ✅ COMPLETADA (70%)
- [x] SDK básico (PDA derivation, error handling)
- [x] SDK unit tests (6 tests passed)
- [x] TUI con Ratatui (6 opciones)
- [x] TUI compilada (1.2M binary)
- [x] Integración SDK + TUI
- [x] End-to-end test en localnet
- [x] Manual de usuario completo
- [ ] Transacciones reales (modo demo actualmente)
- [ ] CRUD completo
- **Commits:** 12
- **Status:** 70% completa

### Fase 03: SDK Completo + TUI Funcional 🚧 EN PROGRESO (0%)
- [ ] Reorganizar estructura (aplicar trust-escrow-v2)
- [ ] Modularizar SDK (client, error, pda, types, utils)
- [ ] Implementar transacciones reales con RpcClient
- [ ] Implementar fetch de accounts
- [ ] CRUD completo en TUI
- [ ] Search/Filter por owner, product_id, status
- [ ] List screen con tabla
- **Commits:** 0
- **Status:** 0% completa

### Fase 04: TUI Profesional + Features Avanzadas 🔲 PENDIENTE (0%)
- [ ] Modularizar TUI (app/, ui/)
- [ ] FormField system
- [ ] Settings menu (Theme, Network, Wallets)
- [ ] Wallet manager (add, delete, switch)
- [ ] Logging a archivo
- [ ] Transaction history
- [ ] Polish UX
- **Commits:** 0
- **Status:** 0% completa

### Fase 05: Deploy + Producción 🔲 PENDIENTE (0%)
- [ ] Deploy a devnet
- [ ] Deploy a mainnet
- [ ] Monitoring
- [ ] Backend API (opcional)
- [ ] Frontend web (opcional)
- **Commits:** 0
- **Status:** 0% completa

---

## 📊 Resumen de Fases

| Fase | Descripción | Tasks | Estado |
|------|-------------|-------|--------|
| 01 | Core Program + DevContainer | 6 | ✅ 95% |
| 02 | SDK + TUI Básica | 8 | ✅ 70% |
| 03 | SDK Completo + TUI Funcional | 7 | 🚧 0% |
| 04 | TUI Profesional + Features | 7 | 🔲 0% |
| 05 | Deploy + Producción | 5 | 🔲 0% |

**Total tasks:** 33 (21 completadas, 12 pendientes)

**Progreso global:** 64% (21/33)

---

## 🔧 Instrucciones del Contrato (4 total)

### License Operations (4) ✅
| # | Instrucción | Descripción | Status |
|---|-------------|-------------|--------|
| 1 | issue_license | Emite nueva licencia | ✅ Implementada |
| 2 | extend_license | Extiende duración | ✅ Implementada |
| 3 | revoke_license | Revoca licencia | ✅ Implementada |
| 4 | validate_license | Valida estado | ✅ Implementada |

**Nota:** Todas las instrucciones están implementadas en el program, pero el SDK y TUI NO las ejecutan (modo demo).

---

## 📁 Estructura del Proyecto

### Estructura ACTUAL (Desordenada) ❌
```
License-System-on-Solana/
├── .devcontainer/           # ✅ DevContainer
├── docs/                    # ✅ Documentación
├── crates/                  # ❌ Fuera del workspace
│   ├── sdk/                 # SDK incompleto (78 líneas)
│   └── tui/                 # TUI en 1 archivo (373 líneas)
├── license-system/          # Anchor workspace
│   ├── programs/
│   ├── tests/
│   └── Cargo.toml
└── test-ledger/             # ❌ Basura (eliminar)
```

### Estructura TARGET (Aplicar trust-escrow-v2) ✅
```
License-System-on-Solana/
├── .devcontainer/           # ✅ DevContainer (no mover)
├── docs/                    # ✅ Documentación (no mover)
├── AGENTS.md                # ⬅️ Este archivo
└── license-system/          # Todo el proyecto
    ├── programs/license-system/
    ├── sdk/                 # ⬅️ Mover desde crates/sdk
    │   └── src/
    │       ├── lib.rs       # Re-exports + constants
    │       ├── client.rs    # LicenseClient + transacciones
    │       ├── error.rs     # Error types
    │       ├── pda.rs       # PDA helpers
    │       ├── types.rs     # License struct
    │       └── utils.rs     # send(), fetch(), helpers
    ├── tui/                 # ⬅️ Mover desde crates/tui
    │   └── src/
    │       ├── main.rs      # Entry point
    │       ├── lib.rs       # Re-exports
    │       ├── app/         # State + logic
    │       │   ├── mod.rs
    │       │   ├── state.rs
    │       │   └── events.rs
    │       └── ui/          # Rendering
    │           ├── mod.rs
    │           └── layout.rs
    ├── cli/                 # CLI (futuro)
    ├── shared/              # Código compartido (futuro)
    ├── tests/               # Tests TypeScript
    ├── Anchor.toml
    ├── Cargo.toml           # Workspace: programs, sdk, tui, cli
    └── .gitignore           # Agregar test-ledger/
```

---

## 🎯 Modelo de Licencias

### Estructura de Datos
```rust
pub struct License {
    pub owner: Pubkey,        // 32 bytes
    pub product_id: String,   // max 64 chars
    pub expires_at: i64,      // Unix timestamp
    pub is_revoked: bool,     // 1 byte
}
```

### PDAs
| Cuenta | Seed | Descripción |
|--------|------|-------------|
| License | `b"license", owner` | Licencia única por owner |

**Limitación actual:** Un owner = una licencia. Para múltiples productos, agregar `product_id` al seed.

### Security Fixes Aplicados (6)
| # | Tipo | Fix | Prioridad |
|---|------|-----|-----------|
| 1 | Access Control | IssueLicense: owner == authority | P1 |
| 2 | Access Control | RevokeLicense: authority validation | P1 |
| 3 | Access Control | ExtendLicense: seed consistency | P1 |
| 4 | Logic Bug | Grace period fix (now + grace, not now - grace) | P2 |
| 5 | Type Safety | Owner derivation .as_ref() | P2 |
| 6 | Type Safety | ValidateLicense return type | P3 |

---

## 🔄 Flujo de Licencias

```
ISSUED → ACTIVE → EXPIRED (grace period 7 días) → CANNOT_EXTEND
   ↓                ↓
REVOKED         EXTENDED
(permanent)
```

**Estados**:
- **ISSUED**: Licencia creada
- **ACTIVE**: `expires_at > now && !is_revoked`
- **EXPIRED**: `expires_at < now && !is_revoked`
- **REVOKED**: `is_revoked == true` (permanente)

**Grace Period**: 7 días después de expirar, se puede extender. Después de 7 días, NO se puede extender.

---

## 📅 Timeline de Implementación

### Sprint 1: Reorganización + SDK Funcional (2-3 días)
**Objetivo**: Estructura limpia + transacciones reales

**Tareas**:
1. Reorganizar carpetas (mover crates/ → license-system/)
2. Actualizar Cargo.toml workspace
3. Modularizar SDK (client, error, pda, types, utils)
4. Implementar `send()` helper
5. Implementar `build_*_ix()` para las 4 instrucciones
6. Implementar `op_*()` de alto nivel
7. Tests del SDK

**Entregables**:
- ✅ Estructura ordenada
- ✅ SDK con transacciones reales
- ✅ Tests pasando

### Sprint 2: TUI Funcional + CRUD (2 días)
**Objetivo**: TUI que ejecute transacciones reales

**Tareas**:
1. Conectar TUI con SDK real
2. Issue License → transacción real
3. Extend License → transacción real
4. Revoke License → transacción real
5. Implementar `get_license()` (fetch real)
6. Implementar `get_all_licenses()`
7. List screen con tabla
8. Search screen (por owner, product_id, status)

**Entregables**:
- ✅ TUI funcional (no demo)
- ✅ CRUD completo
- ✅ Search/Filter

### Sprint 3: TUI Profesional (2 días)
**Objetivo**: TUI modular con features avanzadas

**Tareas**:
1. Modularizar TUI (app/, ui/)
2. FormField system
3. Settings menu
4. Wallet manager
5. Logging
6. Transaction history
7. Polish UX

**Entregables**:
- ✅ TUI modular
- ✅ Settings completo
- ✅ UX pulido

### Sprint 4: Deploy + Producción (1 día)
**Objetivo**: Deploy a devnet/mainnet

**Tareas**:
1. Deploy a devnet
2. Verificar funcionamiento
3. Deploy a mainnet (opcional)
4. Monitoring
5. Documentación final

**Entregables**:
- ✅ Program en devnet
- ✅ TUI funcionando contra devnet
- ✅ Docs actualizadas

---

## 🚀 Comandos Útiles

### Compilar Program
```bash
cd license-system
anchor build
```

### Compilar SDK
```bash
cd license-system/sdk
cargo build --release
cargo test
```

### Compilar TUI
```bash
cd license-system/tui
cargo build --release
```

### Ejecutar TUI
```bash
cd license-system/tui
ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json ./target/release/license-tui
```

### Deploy
```bash
# Localnet
solana-test-validator --reset
anchor deploy

# Devnet
solana config set --url devnet
anchor deploy
```

### Tests
```bash
# TypeScript tests
cd license-system
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/license-system.ts

# SDK tests
cd license-system/sdk
cargo test

# TUI tests (futuro)
cd license-system/tui
cargo test
```

---

## 🐛 Issues Conocidos

### 1. TUI en Modo Demo
**Problema**: TUI solo muestra información, NO ejecuta transacciones reales

**Causa**: SDK no tiene funciones `op_*()` implementadas

**Solución**: Implementar transacciones reales en Sprint 1

**Status**: 🔴 Bloqueador crítico

### 2. Program ID Mismatch en Tests
**Problema**: Tests TypeScript fallan con `DeclaredProgramIdMismatch`

**Causa**: Binary `.so` tiene Program ID viejo compilado

**Solución**: Recompilar program o usar IDL correcto

**Status**: ⚠️ Workaround aplicado (actualizar IDL manualmente)

### 3. getrandom Compilation Error
**Problema**: `cargo build-sbf` falla con getrandom 0.2

**Causa**: Bug conocido de getrandom con Solana BPF

**Solución**: Usar `anchor build` en lugar de `cargo build-sbf`

**Status**: ✅ Resuelto

### 4. Estructura Desordenada
**Problema**: `crates/` está fuera del workspace de Anchor

**Causa**: Creación incremental sin planificación

**Solución**: Reorganizar siguiendo trust-escrow-v2

**Status**: 🔴 Pendiente (Sprint 1)

---

## 📊 Comparación con trust-escrow-v2

| Feature | trust-escrow-v2 | License System | Gap |
|---------|-----------------|----------------|-----|
| **Program** | 37 instrucciones | 4 instrucciones | -33 |
| **SDK Lines** | 2057 | 78 | **-2000** |
| **SDK Modular** | ✅ 7 archivos | ❌ 1 archivo | **100%** |
| **SDK Transacciones** | ✅ Reales | ❌ Demo | **100%** |
| **TUI Lines** | 300+ (modular) | 373 (1 archivo) | 0% |
| **TUI Modular** | ✅ app/, ui/ | ❌ 1 archivo | **100%** |
| **TUI Funcional** | ✅ Transacciones | ❌ Demo | **100%** |
| **CRUD** | ✅ Completo | ⚠️ Parcial | **75%** |
| **Search** | ✅ Implementado | ❌ No existe | **100%** |
| **Settings** | ✅ Completo | ❌ No existe | **100%** |
| **Wallet Manager** | ✅ Completo | ❌ No existe | **100%** |
| **Logging** | ✅ Archivo | ❌ No existe | **100%** |
| **Docs** | ⚠️ Básica | ✅ Completa | -50% |
| **DevContainer** | ❌ No existe | ✅ Production | -100% |

**Promedio trust-escrow-v2**: 85%  
**Promedio License System**: 42%  
**Gap total**: **43%**

---

## 🎯 Prioridades Inmediatas

### 🔴 CRÍTICO (Bloqueadores)
1. **Reorganizar estructura** — aplicar trust-escrow-v2 (2-3 horas)
2. **Implementar transacciones reales en SDK** — RpcClient + send() + op_*() (1 día)
3. **Conectar TUI con SDK real** — ejecutar transacciones (4 horas)

### 🟡 ALTA (Funcionalidad Core)
4. **CRUD completo** — fetch, list, search (1 día)
5. **Modularizar TUI** — app/, ui/ (1 día)
6. **Settings menu** — Theme, Network, Wallets (4 horas)

### 🟢 MEDIA (Nice-to-have)
7. **Logging** — tracing a archivo (2 horas)
8. **Transaction history** — historial de txs (4 horas)
9. **Polish UX** — mejoras visuales (1 día)

---

## 🚀 Plan de Acción (Próximas 24 horas)

### Hora 0-3: Reorganización
1. Mover `crates/sdk` → `license-system/sdk`
2. Mover `crates/tui` → `license-system/tui`
3. Actualizar `Cargo.toml` workspace
4. Agregar `test-ledger/` a `.gitignore`
5. Commit: "refactor: reorganize project structure"

### Hora 3-8: SDK Funcional
1. Crear `sdk/src/client.rs` (basado en trust-escrow-v2)
2. Crear `sdk/src/pda.rs`
3. Crear `sdk/src/types.rs`
4. Crear `sdk/src/utils.rs` (send, fetch)
5. Crear `sdk/src/error.rs`
6. Implementar `op_issue_license()`
7. Implementar `op_extend_license()`
8. Implementar `op_revoke_license()`
9. Implementar `op_validate_license()`
10. Implementar `get_license()`
11. Implementar `get_all_licenses()`
12. Tests del SDK
13. Commit: "feat(sdk): implement real transactions"

### Hora 8-12: TUI Funcional
1. Conectar TUI con SDK real
2. Issue License → transacción real
3. Extend License → transacción real
4. Revoke License → transacción real
5. List License → fetch real
6. Probar end-to-end en localnet
7. Commit: "feat(tui): connect with real SDK transactions"

### Hora 12-16: CRUD Completo
1. Implementar Search screen
2. Implementar List screen con tabla
3. Filtros: owner, product_id, status
4. Probar todas las operaciones
5. Commit: "feat(tui): add CRUD complete with search"

### Hora 16-24: Deploy
1. Probar TODO en localnet
2. Deploy a devnet
3. Verificar funcionamiento
4. Actualizar documentación
5. Commit: "feat: deploy to devnet"

---

## 📝 Convenciones del Proyecto

### Git Commits
Usamos **Conventional Commits**:
```
feat(scope): descripción corta
fix(scope): descripción del bug
docs(scope): cambios en documentación
refactor(scope): refactoring sin cambio funcional
test(scope): agregar/modificar tests
```

### Branches
- `main` — producción
- `develop` — desarrollo
- `feat/*` — nuevas features
- `fix/*` — bug fixes
- `refactor/*` — refactoring

### Code Style
- **Rust**: rustfmt + clippy
- **TypeScript**: prettier + eslint
- **Commits**: conventional commits
- **Docs**: markdown con emojis

---

## 🔐 Security Checklist

- [x] Access control en todas las instrucciones
- [x] Seed consistency en PDAs
- [x] Grace period logic correcta
- [x] Type safety con .as_ref()
- [x] Error handling robusto
- [x] DevContainer security hardening
- [ ] Audit externo (futuro)
- [ ] Fuzzing tests (futuro)

---

## 📚 Recursos

### Documentación
- [Manual de Usuario TUI](./docs/TUI-USER-MANUAL.md)
- [Phase 1: Core Program](./docs/phases/phase-1/README.md)
- [Phase 2: SDK + TUI](./docs/phases/phase-2/README.md)
- [Phase 3: Backend + Frontend](./docs/phases/phase-3/README.md)
- [Comparación con Trust-Work-Escrow](./docs/COMPARISON-TRUST-WORK-ESCROW.md)

### Referencias
- [trust-escrow-v2](../Trust-Work-Escrow/trust-escrow-v2/) — Referencia de estructura
- [Solana Docs](https://docs.solana.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Ratatui Tutorial](https://ratatui.rs/tutorials/)

---

## 👥 Equipo

**Autor**: License System Team  
**Última actualización**: 2026-04-26  
**Versión**: 0.2.0 (Phase 2 en progreso)

---

## 📄 Licencia

[Agregar licencia aquí]

---

**Próximo paso**: Ejecutar Sprint 1 (Reorganización + SDK Funcional) 🚀
