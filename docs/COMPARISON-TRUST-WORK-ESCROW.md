# 📊 Comparación: Trust-Work-Escrow vs License System

**Fecha**: 2026-04-26  
**Propósito**: Identificar gaps y mejorar License System basándose en Trust-Work-Escrow

---

## 🏗️ Estructura de Proyecto

### Trust-Work-Escrow (Referencia)
```
trust-escrow/
├── programs/trust-escrow/    # Anchor program
├── escrow-core/              # SDK con transacciones reales
├── tui/                      # TUI modular (app, ui, config)
├── cli/                      # CLI
├── app/                      # Frontend (opcional)
├── tests/                    # Tests TypeScript
└── Cargo.toml                # Workspace (exclude cli, tui, escrow-core)
```

**Características**:
- ✅ Monorepo con workspace
- ✅ SDK separado (`escrow-core`)
- ✅ TUI modular (3 archivos)
- ✅ CLI separado
- ✅ Frontend opcional

### License System (Actual)
```
License-System-on-Solana/
├── license-system/           # Anchor workspace
│   ├── programs/license-system/
│   └── tests/
├── crates/
│   ├── sdk/                  # SDK básico
│   └── tui/                  # TUI en 1 archivo
├── docs/                     # Documentación por fases
└── .devcontainer/            # DevContainer
```

**Características**:
- ✅ Estructura clara
- ⚠️ SDK incompleto (solo PDA derivation)
- ⚠️ TUI en 1 archivo (no modular)
- ❌ Sin CLI
- ❌ Sin frontend

---

## 🎨 TUI: Comparación Detallada

### Trust-Work-Escrow TUI

**Arquitectura**:
- `main.rs` — entry point, logger setup, main loop
- `app.rs` — state management, event handling, business logic (1318+ líneas)
- `ui.rs` — rendering, widgets, layout
- `config.rs` — settings, theme, wallet config

**Features**:
- ✅ Múltiples screens (20+): WalletSelect, RoleSelect, MainMenu, Forms, Result, Settings
- ✅ FormField system: text, select, readonly, masked, validation
- ✅ Screen stack navigation (push/pop)
- ✅ Role-based menus (Admin, Client, Freelancer, Arbiter, Treasury)
- ✅ Settings menu: Theme, Network, Wallets, Password
- ✅ Wallet manager: add, delete, switch, balance
- ✅ Job list con navegación
- ✅ Transaction history
- ✅ Logging a archivo (no rompe TUI)
- ✅ **Transacciones REALES** vía `escrow-core`

**Controles**:
- `↑↓` o `j/k` — navegar
- `Enter` — confirmar
- `ESC` — volver
- `Ctrl+C` — salir
- `d` — delete (en listas)
- `←→` — ciclar opciones (select fields)

### License System TUI (Actual)

**Arquitectura**:
- `main.rs` — todo en un archivo (373 líneas)

**Features**:
- ✅ 6 screens: Main, Issue, Extend, Validate, Revoke, List
- ✅ Input validation básica
- ✅ Status bar con feedback
- ❌ Solo modo demo (NO transacciones reales)
- ❌ Sin FormField system
- ❌ Sin screen stack
- ❌ Sin settings menu
- ❌ Sin wallet manager
- ❌ Sin logging
- ❌ Sin CRUD completo

**Controles**:
- `↑↓` o `1-6` — navegar
- `Enter` — confirmar
- `ESC` — volver
- `q` o `6` — salir

---

## 🔧 SDK: Comparación

### escrow-core (Referencia)

**Funciones**:
- ✅ `make_rpc()` — crear RpcClient
- ✅ `load_keypair()` — cargar keypair desde archivo
- ✅ `send()` — enviar transacción
- ✅ `send_many()` — enviar múltiples instrucciones
- ✅ `build_*_ix()` — construir instrucciones manualmente
- ✅ `op_*()` — operaciones de alto nivel (create_job, deposit, accept, etc.)
- ✅ `disc()` — calcular discriminator
- ✅ PDAs: `config_pda()`, `job_pda()`
- ✅ Fetch accounts: `get_job_info()`, `get_all_jobs()`
- ✅ Transaction history: `get_tx_history()`

**Tamaño**: 1318+ líneas (completo)

### license-sdk (Actual)

**Funciones**:
- ✅ `new_localnet()`, `new_devnet()` — crear cliente
- ✅ `derive_license_pda()` — derivar PDA
- ✅ `payer_pubkey()` — obtener pubkey del payer
- ✅ `program_id()` — obtener program ID
- ❌ Sin funciones de transacciones reales
- ❌ Sin `send()` helper
- ❌ Sin `build_*_ix()`
- ❌ Sin fetch de accounts
- ❌ Sin listado de licencias

**Tamaño**: 78 líneas (incompleto)

---

## ❌ Funcionalidades Faltantes en License System

### 1. CRUD Completo

**Actual**:
- ✅ Create (Issue License)
- ⚠️ Read (List Licenses) — solo muestra PDA, no datos reales
- ⚠️ Update (Extend License) — existe pero no ejecuta
- ❌ Delete — NO existe (debería ser Revoke, pero no ejecuta)

**Necesario**:
- ✅ Create — **implementar transacción real**
- ✅ Read — **fetch account data real**
- ✅ Update — **implementar transacción real**
- ✅ Delete (Revoke) — **implementar transacción real**
- ✅ **Search** — buscar por owner, product_id, status

### 2. Search/Filter

**Falta**:
- ❌ Buscar licencias por owner
- ❌ Buscar licencias por product_id
- ❌ Filtrar por status (active, expired, revoked)
- ❌ Listar TODAS las licencias (no solo una)

### 3. Transacciones Reales

**Falta en SDK**:
- ❌ `send()` helper
- ❌ `build_issue_license_ix()`
- ❌ `build_extend_license_ix()`
- ❌ `build_revoke_license_ix()`
- ❌ `op_issue_license()` — función de alto nivel
- ❌ `op_extend_license()`
- ❌ `op_revoke_license()`
- ❌ `op_validate_license()`

### 4. Fetch de Datos

**Falta en SDK**:
- ❌ `get_license()` — fetch account data
- ❌ `get_all_licenses()` — listar todas las licencias
- ❌ `get_licenses_by_owner()` — filtrar por owner
- ❌ `get_licenses_by_product()` — filtrar por product_id

### 5. TUI Modularización

**Falta**:
- ❌ Separar en módulos: `app.rs`, `ui.rs`, `config.rs`
- ❌ FormField system
- ❌ Screen stack navigation
- ❌ Settings menu
- ❌ Wallet manager
- ❌ Logging a archivo

---

## 🎯 Plan de Mejora (Priorizado)

### 🔴 Prioridad CRÍTICA (Bloqueadores)

#### 1. Implementar Transacciones Reales en SDK
**Tiempo estimado**: 2-3 horas

**Tareas**:
- [ ] Agregar `send()` helper
- [ ] Implementar `build_issue_license_ix()`
- [ ] Implementar `build_extend_license_ix()`
- [ ] Implementar `build_revoke_license_ix()`
- [ ] Implementar `op_issue_license()` (alto nivel)
- [ ] Implementar `op_extend_license()`
- [ ] Implementar `op_revoke_license()`
- [ ] Implementar `op_validate_license()`

**Bloqueador**: Sin esto, la TUI no puede hacer nada real.

#### 2. Implementar Fetch de Datos
**Tiempo estimado**: 1-2 horas

**Tareas**:
- [ ] Implementar `get_license()` — fetch account data
- [ ] Implementar `get_all_licenses()` — usar `getProgramAccounts`
- [ ] Agregar deserialización de License account

**Bloqueador**: Sin esto, no podemos listar ni buscar licencias.

### 🟡 Prioridad ALTA (Funcionalidad Core)

#### 3. CRUD Completo en TUI
**Tiempo estimado**: 2 horas

**Tareas**:
- [ ] Conectar Issue License con SDK real
- [ ] Conectar Extend License con SDK real
- [ ] Conectar Revoke License con SDK real
- [ ] Implementar List Licenses con fetch real
- [ ] Agregar Search screen (por owner, product_id, status)

#### 4. Search/Filter
**Tiempo estimado**: 1-2 horas

**Tareas**:
- [ ] Screen de búsqueda
- [ ] Filtros: owner, product_id, status (active/expired/revoked)
- [ ] Mostrar resultados en tabla
- [ ] Navegación por resultados

### 🟢 Prioridad MEDIA (UX Improvements)

#### 5. Modularizar TUI
**Tiempo estimado**: 2-3 horas

**Tareas**:
- [ ] Separar `app.rs` (state + logic)
- [ ] Separar `ui.rs` (rendering)
- [ ] Separar `config.rs` (settings)
- [ ] Implementar FormField system
- [ ] Implementar screen stack

#### 6. Settings Menu
**Tiempo estimado**: 1-2 horas

**Tareas**:
- [ ] Theme selector
- [ ] Network selector (localnet/devnet/mainnet)
- [ ] Wallet manager (add/delete/switch)
- [ ] Config persistence (JSON file)

### 🔵 Prioridad BAJA (Nice-to-have)

#### 7. Logging
**Tiempo estimado**: 30 min

**Tareas**:
- [ ] Agregar `tracing` + `tracing-appender`
- [ ] Log a archivo (no stderr)
- [ ] Log de operaciones exitosas/fallidas

#### 8. Transaction History
**Tiempo estimado**: 1 hora

**Tareas**:
- [ ] Fetch transaction history de wallet
- [ ] Mostrar en tabla
- [ ] Filtrar por tipo de operación

---

## 📐 Estructura Recomendada

### Opción 1: Monorepo Completo (como Trust-Work-Escrow)
```
License-System-on-Solana/
├── programs/license-system/  # Anchor program
├── license-core/             # SDK con transacciones reales
├── tui/                      # TUI modular
├── cli/                      # CLI (futuro)
├── backend/                  # Backend API (futuro)
├── app/                      # Frontend web (futuro)
├── tests/                    # Tests TypeScript
├── docs/                     # Documentación
├── .devcontainer/            # DevContainer
└── Cargo.toml                # Workspace root
```

### Opción 2: Híbrida (mantener estructura actual + mejorar)
```
License-System-on-Solana/
├── license-system/           # Anchor workspace (como está)
├── crates/
│   ├── license-core/         # Renombrar sdk → license-core, agregar transacciones
│   ├── tui/                  # Modularizar (app, ui, config)
│   └── cli/                  # CLI (futuro)
├── docs/                     # Documentación
└── .devcontainer/            # DevContainer
```

**Recomendación**: **Opción 2** (menos disruptivo, mantiene lo que funciona)

---

## 🚀 Roadmap de Implementación

### Sprint 1: Transacciones Reales (CRÍTICO)
**Duración**: 1 semana

1. Renombrar `crates/sdk` → `crates/license-core`
2. Implementar `send()` y `send_many()` helpers
3. Implementar `build_*_ix()` para las 4 instrucciones
4. Implementar `op_*()` de alto nivel
5. Implementar fetch de accounts
6. Tests unitarios del core

### Sprint 2: CRUD Completo en TUI
**Duración**: 3-4 días

1. Conectar TUI con license-core
2. Issue License → transacción real
3. Extend License → transacción real
4. Revoke License → transacción real
5. List Licenses → fetch real + mostrar datos
6. Agregar Search screen

### Sprint 3: Modularización TUI
**Duración**: 2-3 días

1. Separar `app.rs`, `ui.rs`, `config.rs`
2. Implementar FormField system
3. Implementar screen stack
4. Settings menu
5. Wallet manager

### Sprint 4: Features Avanzadas
**Duración**: 1 semana

1. Logging
2. Transaction history
3. Themes
4. Network switcher
5. Polish UX

---

## 📝 Funcionalidades Faltantes (Detallado)

### En el Program (Anchor)

**Actual**:
- ✅ issue_license
- ✅ extend_license
- ✅ revoke_license
- ✅ validate_license

**Falta**:
- ❌ `get_all_licenses` — no existe (se hace con `getProgramAccounts` desde el cliente)
- ❌ `search_licenses` — no existe (se hace con filters en `getProgramAccounts`)
- ❌ `delete_license` — ¿cerrar account y recuperar rent? (opcional)

**Nota**: El program está completo. El search/list se hace desde el cliente con RPC calls.

### En el SDK (license-core)

**Actual**:
- ✅ PDA derivation
- ✅ Error types
- ✅ Client creation

**Falta**:
- ❌ `send()` — enviar transacción
- ❌ `build_issue_license_ix()` — construir instrucción
- ❌ `build_extend_license_ix()`
- ❌ `build_revoke_license_ix()`
- ❌ `op_issue_license()` — función de alto nivel
- ❌ `op_extend_license()`
- ❌ `op_revoke_license()`
- ❌ `op_validate_license()`
- ❌ `get_license()` — fetch account data
- ❌ `get_all_licenses()` — listar todas
- ❌ `get_licenses_by_owner()` — filtrar por owner
- ❌ `get_licenses_by_product()` — filtrar por product_id
- ❌ `get_licenses_by_status()` — filtrar por status (active/expired/revoked)

### En la TUI

**Actual**:
- ✅ Main menu
- ✅ 6 opciones
- ✅ Input screens
- ✅ Status bar
- ✅ Navegación básica

**Falta**:
- ❌ Transacciones reales (solo modo demo)
- ❌ FormField system (validación avanzada)
- ❌ Screen stack (navegación con historial)
- ❌ Settings menu
- ❌ Wallet manager
- ❌ Search screen
- ❌ List screen con tabla (mostrar múltiples licencias)
- ❌ Logging
- ❌ Transaction history
- ❌ Themes
- ❌ Network switcher
- ❌ Modularización (app, ui, config)

---

## 🎯 Decisiones de Diseño

### ¿Renombrar `sdk` → `license-core`?

**Pros**:
- ✅ Consistente con Trust-Work-Escrow
- ✅ Nombre más descriptivo
- ✅ Indica que es el "core" del sistema

**Contras**:
- ⚠️ Requiere refactoring
- ⚠️ Cambiar imports en TUI

**Recomendación**: **SÍ, renombrar** — es más claro y profesional.

### ¿Modularizar TUI ahora o después?

**Opción A**: Modularizar ahora
- ✅ Más fácil agregar features después
- ✅ Código más mantenible
- ⚠️ Toma tiempo (2-3 días)

**Opción B**: Modularizar después
- ✅ Más rápido implementar transacciones reales
- ⚠️ Refactoring más difícil después

**Recomendación**: **Opción A** — modularizar ahora, vale la pena.

### ¿Usar anchor-client o RpcClient directo?

**anchor-client**:
- ✅ Más fácil (usa IDL)
- ⚠️ API compleja
- ⚠️ Errores de compilación

**RpcClient directo** (como Trust-Work-Escrow):
- ✅ Control total
- ✅ Sin errores de compilación
- ✅ Más flexible
- ⚠️ Más código (construir instrucciones manualmente)

**Recomendación**: **RpcClient directo** — más confiable y flexible.

---

## 📊 Comparación de Completitud

| Feature | Trust-Work-Escrow | License System | Gap |
|---------|-------------------|----------------|-----|
| **Program** | ✅ 100% | ✅ 100% | 0% |
| **SDK Transacciones** | ✅ 100% | ❌ 0% | **100%** |
| **SDK Fetch** | ✅ 100% | ❌ 0% | **100%** |
| **TUI Transacciones** | ✅ 100% | ❌ 0% | **100%** |
| **TUI CRUD** | ✅ 100% | ⚠️ 25% | **75%** |
| **TUI Search** | ✅ 100% | ❌ 0% | **100%** |
| **TUI Modular** | ✅ 100% | ❌ 0% | **100%** |
| **Settings** | ✅ 100% | ❌ 0% | **100%** |
| **Wallet Manager** | ✅ 100% | ❌ 0% | **100%** |
| **Logging** | ✅ 100% | ❌ 0% | **100%** |
| **Docs** | ⚠️ 50% | ✅ 100% | -50% |
| **DevContainer** | ❌ 0% | ✅ 100% | -100% |

**Promedio Trust-Work-Escrow**: 86%  
**Promedio License System**: 39%  
**Gap total**: **47%**

---

## 🚀 Próximos Pasos Inmediatos

### 1. Implementar Transacciones Reales (URGENTE)
Sin esto, la TUI es solo una calculadora de PDAs.

**Archivo**: `crates/sdk/src/lib.rs` (o renombrar a `license-core`)

**Agregar**:
```rust
pub fn send(rpc: &RpcClient, payer: &Keypair, ix: Instruction) -> Result<String> {
    let bh = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[payer], bh);
    let sig = rpc.send_and_confirm_transaction(&tx)?;
    Ok(sig.to_string())
}

pub fn op_issue_license(
    rpc: &RpcClient,
    payer: &Keypair,
    owner: &str,
    product_id: &str,
    duration_days: i64,
) -> Result<String> {
    let owner_pk = Pubkey::from_str(owner)?;
    let ix = build_issue_license_ix(&program_id()?, &owner_pk, product_id, duration_days)?;
    send(rpc, payer, ix)
}
```

### 2. Implementar Fetch de Licencias
**Archivo**: `crates/sdk/src/lib.rs`

**Agregar**:
```rust
pub fn get_license(rpc: &RpcClient, owner: &str) -> Result<License> {
    let owner_pk = Pubkey::from_str(owner)?;
    let (license_pda, _) = derive_license_pda(&owner_pk);
    let account = rpc.get_account(&license_pda)?;
    // Deserializar account.data
    Ok(license)
}

pub fn get_all_licenses(rpc: &RpcClient) -> Result<Vec<License>> {
    let program_id = program_id()?;
    let accounts = rpc.get_program_accounts(&program_id)?;
    // Deserializar cada account
    Ok(licenses)
}
```

### 3. Conectar TUI con SDK Real
**Archivo**: `crates/tui/src/main.rs`

**Modificar** `execute_action()` para llamar `op_*()` en lugar de solo mostrar info.

---

## 📚 Recursos

### Trust-Work-Escrow
- **Repo**: `/home/dcdebian/Proyects/Trust-Work-Escrow/trust-escrow`
- **TUI**: `tui/src/` (app.rs, ui.rs, config.rs)
- **SDK**: `escrow-core/src/lib.rs` (1318 líneas)

### License System
- **Repo**: `/home/dcdebian/Proyects/License-System-on-Solana`
- **TUI**: `crates/tui/src/main.rs` (373 líneas)
- **SDK**: `crates/sdk/src/lib.rs` (78 líneas)

---

**Última actualización**: 2026-04-26  
**Autor**: License System Team
