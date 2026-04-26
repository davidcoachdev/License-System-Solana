# Phase 2: SDK + TUI + Integration

**Status**: 🚧 EN PROGRESO (30%)

**Objetivo**: SDK en Rust para abstraer interacción con Solana, TUI funcional con Ratatui, integración completa end-to-end.

---

## 📋 Checklist

### 🚧 SDK en Rust
- [ ] Estructura del crate `crates/sdk/`
- [ ] Cliente RPC configurable (localnet/devnet/mainnet)
- [ ] Wrapper para las 4 instrucciones:
  - [ ] `issue_license(owner, product_id, duration_days)`
  - [ ] `extend_license(license_pda, additional_days)`
  - [ ] `revoke_license(license_pda)`
  - [ ] `validate_license(license_pda, product_id)`
- [ ] Derivación de PDAs (Program Derived Addresses)
- [ ] Manejo de keypairs y wallets
- [ ] Error handling robusto
- [ ] Tests unitarios del SDK
- [ ] Documentación con ejemplos

### ✅ TUI con Ratatui
- [x] Estructura del crate `crates/tui/`
- [x] Main menu con 5 opciones
- [x] Navegación por teclado (↑↓, 1-5, ESC, Enter)
- [x] Screens interactivos para cada operación
- [x] Status bar con feedback
- [x] Dependencias configuradas (ratatui 0.28, crossterm 0.28)
- [ ] Compilación completa (en progreso)
- [ ] Integración con SDK
- [ ] Conexión a Solana RPC
- [ ] Manejo de wallets
- [ ] Display de licencias existentes

### ⏸️ Wallet Manager
- [ ] Cargar keypair desde archivo
- [ ] Generar nuevo keypair
- [ ] Mostrar balance
- [ ] Airdrop en devnet (para testing)
- [ ] Cambiar wallet activa

---

## 📁 Estructura de Archivos

```
crates/
├── sdk/                    # SDK en Rust (pendiente)
│   ├── src/
│   │   ├── lib.rs         # API pública del SDK
│   │   ├── client.rs      # Cliente RPC
│   │   ├── instructions.rs # Wrappers de instrucciones
│   │   ├── pda.rs         # Derivación de PDAs
│   │   └── error.rs       # Error types
│   ├── Cargo.toml
│   └── README.md
│
└── tui/                    # TUI con Ratatui (creada)
    ├── src/
    │   └── main.rs        # 200+ líneas, 5 screens
    ├── Cargo.toml         # ratatui, crossterm, anchor-client
    └── README.md
```

---

## 🎯 Objetivos de Phase 2

### SDK en Rust
**Propósito**: Abstraer toda la complejidad de Solana para que la TUI/CLI solo llamen funciones simples.

**Ejemplo de uso**:
```rust
use license_sdk::LicenseClient;

let client = LicenseClient::new("http://localhost:8899", keypair)?;

// Emitir licencia
let signature = client.issue_license(
    owner_pubkey,
    "product-001",
    30, // días
).await?;

// Validar licencia
let is_valid = client.validate_license(license_pda, "product-001").await?;
```

### TUI con Ratatui
**Propósito**: Interfaz terminal interactiva para administradores del sistema.

**Features**:
- Main menu con navegación visual
- Input screens para cada operación
- Status bar con feedback en tiempo real
- Manejo de errores user-friendly
- Display de licencias activas

**Controles**:
- `↑↓` — navegar menú
- `1-5` — selección rápida
- `Enter` — confirmar
- `ESC` — volver
- `q` — salir

---

## 🔧 Comandos Útiles

### Compilar SDK
```bash
cd crates/sdk
cargo build --release
cargo test
```

### Compilar TUI
```bash
cd crates/tui
cargo build --release
```

### Ejecutar TUI
```bash
cd crates/tui
cargo run --release
```

### Ejecutar TUI con Validador Local
```bash
# Terminal 1: Iniciar validador
solana-test-validator --reset

# Terminal 2: Ejecutar TUI
cd crates/tui
ANCHOR_WALLET=~/.config/solana/id.json cargo run --release
```

---

## 🐛 Issues Conocidos

### 1. TUI Compilation Timeout
**Problema**: Compilación de TUI tarda 5-10 minutos (dependencias pesadas)

**Causa**: Primera compilación descarga y compila todas las dependencias de Solana/Anchor

**Solución**: Esperar a que termine. Compilaciones posteriores serán instantáneas.

**Status**: ⏳ En progreso

### 2. SDK No Implementado
**Problema**: TUI no puede conectarse a Solana porque el SDK no existe

**Causa**: SDK es el próximo paso a implementar

**Solución**: Crear `crates/sdk/` con las funciones necesarias

**Status**: 🔲 Pendiente

---

## 📊 Métricas

- **TUI Lines of Code**: ~200 líneas
- **TUI Screens**: 5 (Main, Issue, Extend, Validate, Revoke)
- **SDK Progress**: 0% (no iniciado)
- **Integration Progress**: 0% (bloqueado por SDK)

---

## 🎯 Criterios de Completitud

- [ ] SDK compila sin errores
- [ ] SDK tests pasan
- [ ] TUI compila sin errores
- [ ] TUI se ejecuta correctamente
- [ ] TUI conecta con SDK
- [ ] SDK conecta con Solana localnet
- [ ] End-to-end: Issue license desde TUI funciona
- [ ] End-to-end: Extend license desde TUI funciona
- [ ] End-to-end: Validate license desde TUI funciona
- [ ] End-to-end: Revoke license desde TUI funciona

**Completitud**: 2/10 (20%)

---

## 🚀 Plan de Implementación

### Paso 1: Crear SDK (Prioridad Alta)
1. Crear estructura `crates/sdk/`
2. Implementar `LicenseClient` con RPC connection
3. Implementar derivación de PDAs
4. Implementar wrappers de las 4 instrucciones
5. Agregar tests unitarios
6. Documentar API pública

**Tiempo estimado**: 2-3 horas

### Paso 2: Integrar TUI con SDK (Prioridad Alta)
1. Agregar SDK como dependencia en TUI
2. Implementar `execute_action()` con llamadas reales al SDK
3. Agregar manejo de errores
4. Agregar display de resultados
5. Agregar loading states

**Tiempo estimado**: 1-2 horas

### Paso 3: Wallet Manager (Prioridad Media)
1. Cargar keypair desde archivo
2. Mostrar balance actual
3. Airdrop en devnet (para testing)
4. Cambiar wallet activa

**Tiempo estimado**: 1 hora

### Paso 4: Testing End-to-End (Prioridad Alta)
1. Probar todas las operaciones en localnet
2. Verificar manejo de errores
3. Verificar UX (feedback, loading, etc.)
4. Documentar flujos de uso

**Tiempo estimado**: 1 hora

---

## 📝 Notas Técnicas

### Dependencias del SDK
```toml
[dependencies]
anchor-client = "0.32.1"
solana-sdk = "2.1"
solana-client = "2.1"
anyhow = "1.0"
thiserror = "1.0"
```

### Dependencias de la TUI
```toml
[dependencies]
ratatui = "0.28"
crossterm = "0.28"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
license-sdk = { path = "../sdk" }  # Cuando exista
```

### Arquitectura de Integración
```
TUI (Ratatui)
    ↓
SDK (Rust)
    ↓
Anchor Client
    ↓
Solana RPC
    ↓
Solana Program (on-chain)
```

---

## 🎨 TUI Mockup

```
┌─────────────────────────────────────────────────┐
│ License System on Solana - TUI                  │
└─────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────┐
│ Main Menu - Use ↑↓ or numbers to select        │
│                                                 │
│ → 1. Issue License                              │
│   2. Extend License                             │
│   3. Validate License                           │
│   4. Revoke License                             │
│   5. Exit                                       │
│                                                 │
└─────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────┐
│ Status: Welcome to License System TUI          │
└─────────────────────────────────────────────────┘
```

---

## 🚀 Próximos Pasos (Phase 3)

1. Backend API (opcional)
2. Licencias firmadas (offline validation)
3. Frontend web
4. PostgreSQL indexer
5. Monitoring y analytics

---

**Última actualización**: 2026-04-26
**Autor**: License System Team
