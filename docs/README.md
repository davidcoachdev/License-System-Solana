# 📚 Documentation - License System on Solana

Documentación completa del proyecto organizada por fases de desarrollo.

---

## 🗂️ Estructura de Documentación

```
docs/
├── README.md           # Este archivo (índice principal)
└── phases/
    ├── phase-1/        # Core Program + DevContainer + Tests
    ├── phase-2/        # SDK + TUI + Integration
    └── phase-3/        # Backend + Frontend + Production
```

---

## 📋 Roadmap General

### ✅ Phase 1: Core Program + DevContainer + Tests (95% completa)
**Objetivo**: Anchor program funcional con security fixes, devcontainer production-ready, tests de integración.

**Componentes**:
- ✅ Anchor program (4 instrucciones, 6 security fixes)
- ✅ DevContainer (audit completo, security hardening)
- ✅ Tests TypeScript (5 test cases)
- ⏳ Deploy a devnet (pendiente)

**Documentación**: [📖 Phase 1 README](./phases/phase-1/README.md)

---

### 🚧 Phase 2: SDK + TUI + Integration (30% completa)
**Objetivo**: SDK en Rust para abstraer interacción con Solana, TUI funcional con Ratatui, integración completa end-to-end.

**Componentes**:
- 🔲 SDK en Rust (pendiente)
- ✅ TUI con Ratatui (creada, compilando)
- 🔲 Wallet Manager (pendiente)
- 🔲 Integración end-to-end (pendiente)

**Documentación**: [📖 Phase 2 README](./phases/phase-2/README.md)

---

### ⏸️ Phase 3: Backend + Frontend + Production (0% completa)
**Objetivo**: Backend API, frontend web, licencias firmadas offline, PostgreSQL indexer, monitoring.

**Componentes**:
- ⏸️ Backend API (Rust/Axum)
- ⏸️ PostgreSQL Indexer
- ⏸️ Frontend Web (Next.js)
- ⏸️ Licencias Firmadas (offline validation)
- ⏸️ Monitoring & Analytics

**Documentación**: [📖 Phase 3 README](./phases/phase-3/README.md)

---

## 🎯 Estado Actual del Proyecto

### ✅ Completado
- Anchor program con 6 security fixes
- DevContainer production-ready (HEALTHCHECK, Trivy CI, security hardening)
- Tests TypeScript de integración
- TUI con Ratatui (código completo)
- Deploy a localnet exitoso

### 🚧 En Progreso
- Compilación de TUI (dependencias pesadas)
- Deploy a devnet (bloqueado por airdrop rate limit)

### 🔲 Pendiente
- SDK en Rust
- Integración TUI + SDK
- Wallet Manager
- Backend API
- Frontend Web
- PostgreSQL Indexer

---

## 📊 Métricas Globales

| Fase | Completitud | Status |
|------|-------------|--------|
| Phase 1 | 95% | ✅ Casi completa |
| Phase 2 | 30% | 🚧 En progreso |
| Phase 3 | 0% | ⏸️ No iniciada |
| **Total** | **42%** | 🚧 En desarrollo |

---

## 🚀 Quick Start

### Requisitos
- Rust 1.89+
- Solana CLI 3.1+
- Anchor 0.32+
- Node.js 20+
- Docker (para devcontainer)

### Setup Rápido

```bash
# 1. Clonar repo
git clone <repo-url>
cd License-System-on-Solana

# 2. Abrir en DevContainer (VS Code)
# Ctrl+Shift+P → Dev Containers: Reopen in Container

# 3. Compilar program
cd license-system
anchor build

# 4. Iniciar validador local
solana-test-validator --reset

# 5. Deploy
anchor deploy

# 6. Ejecutar tests
export ANCHOR_WALLET=~/.config/solana/id.json
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/license-system.ts

# 7. Ejecutar TUI (cuando esté compilada)
cd ../crates/tui
cargo run --release
```

---

## 📁 Estructura del Proyecto

```
License-System-on-Solana/
├── .devcontainer/          # DevContainer config (production-ready)
├── .github/                # CI/CD workflows (Trivy security scanning)
├── license-system/         # Anchor workspace
│   ├── programs/           # Solana programs
│   │   └── license-system/ # Core program (4 instrucciones)
│   ├── tests/              # Integration tests (TypeScript)
│   └── target/             # Build artifacts
├── crates/                 # Rust workspace
│   ├── sdk/                # SDK (pendiente)
│   ├── tui/                # TUI con Ratatui (creada)
│   ├── cli/                # CLI (pendiente)
│   └── backend/            # Backend API (pendiente)
├── apps/                   # Applications
│   └── web/                # Frontend (pendiente)
└── docs/                   # Documentación por fases
    └── phases/
        ├── phase-1/        # ✅ 95% completa
        ├── phase-2/        # 🚧 30% completa
        └── phase-3/        # ⏸️ 0% completa
```

---

## 🔧 Comandos Útiles por Fase

### Phase 1: Program + Tests
```bash
# Compilar program
cd license-system && anchor build

# Deploy a localnet
solana-test-validator --reset
anchor deploy

# Ejecutar tests
export ANCHOR_WALLET=~/.config/solana/id.json
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/license-system.ts
```

### Phase 2: SDK + TUI
```bash
# Compilar SDK (cuando exista)
cd crates/sdk && cargo build --release

# Compilar TUI
cd crates/tui && cargo build --release

# Ejecutar TUI
cd crates/tui && cargo run --release
```

### Phase 3: Backend + Frontend
```bash
# Backend
cd crates/backend && cargo run --release

# Frontend
cd apps/web && npm run dev
```

---

## 🐛 Issues Conocidos

### Global
1. **Program ID Mismatch** — Tests fallan, necesita recompilación
2. **getrandom Error** — Usar `anchor build` en lugar de `cargo build-sbf`
3. **Devnet Deploy Bloqueado** — Airdrop rate limit

Ver detalles en cada fase:
- [Phase 1 Issues](./phases/phase-1/README.md#-issues-conocidos)
- [Phase 2 Issues](./phases/phase-2/README.md#-issues-conocidos)

---

## 📝 Convenciones del Proyecto

### Git Commits
Usamos **Conventional Commits**:
```
feat(scope): descripción corta
fix(scope): descripción del bug
docs(scope): cambios en documentación
test(scope): agregar/modificar tests
refactor(scope): refactoring sin cambio funcional
```

### Branches
- `main` — producción
- `develop` — desarrollo
- `feature/*` — nuevas features
- `fix/*` — bug fixes

### Testing
- **Unit tests**: Rust (dentro del program)
- **Integration tests**: TypeScript (Anchor)
- **E2E tests**: TUI manual testing

---

## 🎯 Próximos Pasos Inmediatos

1. ✅ **Documentar fases** (completado)
2. 🔲 **Compilar TUI** (en progreso)
3. 🔲 **Crear SDK en Rust** (próximo)
4. 🔲 **Integrar TUI + SDK**
5. 🔲 **Probar end-to-end en localnet**
6. 🔲 **Deploy a devnet con wallet del usuario**

---

## 📚 Recursos Adicionales

### Solana
- [Solana Docs](https://docs.solana.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)

### Rust
- [Rust Book](https://doc.rust-lang.org/book/)
- [Ratatui Tutorial](https://ratatui.rs/tutorials/)

### DevContainer
- [Dev Containers Spec](https://containers.dev/)
- [Skill DevContainer](../.config/opencode/skills/skill-devcontainer/SKILL.md)

---

## 👥 Equipo

**Autor**: License System Team  
**Última actualización**: 2026-04-26  
**Versión**: 0.1.0 (Phase 1 casi completa)

---

## 📄 Licencia

[Agregar licencia aquí]

---

**¿Preguntas?** Consulta la documentación de cada fase o abre un issue.
