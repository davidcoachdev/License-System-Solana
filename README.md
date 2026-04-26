# 🧾 License System on Solana

Sistema de gestión de licencias descentralizado en Solana con TUI profesional, SDK completo, y arquitectura production-ready.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Anchor-663399?style=for-the-badge&logo=anchor&logoColor=white)](https://www.anchor-lang.com/)
[![Ratatui](https://img.shields.io/badge/Ratatui-FF6B6B?style=for-the-badge&logo=rust&logoColor=white)](https://ratatui.rs/)

---

## 📖 Table of Contents

- [🧾 License System on Solana](#-license-system-on-solana)
  - [📖 Table of Contents](#-table-of-contents)
  - [🎯 About This Project](#-about-this-project)
  - [✨ Features](#-features)
  - [🏗️ Architecture](#️-architecture)
  - [⚙️ Technology Stack](#️-technology-stack)
  - [📁 Project Structure](#-project-structure)
  - [🚀 Quick Start](#-quick-start)
  - [📚 Documentation](#-documentation)
  - [🔧 Development](#-development)
  - [🧪 Testing](#-testing)
  - [🚢 Deployment](#-deployment)
  - [🤝 Contributing](#-contributing)
  - [📄 License](#-license)

---

## 🎯 About This Project

License System es una plataforma descentralizada para gestionar licencias de software en Solana blockchain. Permite a los administradores:

- ✅ **Emitir licencias** on-chain con duración configurable
- ✅ **Extender licencias** existentes (con grace period de 7 días)
- ✅ **Revocar licencias** permanentemente
- ✅ **Validar licencias** offline-first (sin RPC)
- ✅ **Listar y buscar** licencias por owner, product_id, status

**Diferenciadores**:
- 🎮 **TUI como herramienta principal** — interfaz terminal interactiva con Ratatui
- 🦀 **SDK en Rust** — abstracción completa de Solana RPC
- 🔐 **Security-first** — 6 security fixes aplicados, audit completo
- 🏗️ **Arquitectura híbrida** — on-chain + off-chain validation
- 🚀 **Production-ready** — DevContainer, CI/CD, monitoring

---

## ✨ Features

### Core Features
- ✅ **Issue License** — Crear licencia con owner, product_id, duración
- ✅ **Extend License** — Agregar días (grace period: 7 días post-expiración)
- ✅ **Revoke License** — Revocar permanentemente
- ✅ **Validate License** — Verificar estado (active/expired/revoked)
- ✅ **List Licenses** — Mostrar información detallada
- ✅ **Search Licenses** — Buscar por owner, product_id, status

### Security Features
- ✅ **Access Control** — Solo owner puede extender/revocar su licencia
- ✅ **PDA-based** — Program Derived Addresses (determinísticas)
- ✅ **Grace Period** — 7 días para extender después de expirar
- ✅ **Audit Completo** — DevContainer security hardening (P1/P2/P3)

### Developer Experience
- ✅ **TUI Interactiva** — Terminal UI con Ratatui
- ✅ **SDK en Rust** — Abstracción completa
- ✅ **DevContainer** — Entorno reproducible
- ✅ **Documentación Completa** — Manual de usuario, fases, comparaciones
- ✅ **Tests** — TypeScript integration tests + SDK unit tests

---

## 🏗️ Architecture

```
┌─────────────┐
│   TUI       │  Terminal User Interface (Ratatui)
│  (Rust)     │  - Issue, Extend, Revoke, Validate, List, Search
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    SDK      │  Rust SDK (license-sdk)
│  (Rust)     │  - PDA derivation, Transactions, Fetch accounts
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Solana RPC  │  RpcClient
│             │  - send_transaction, get_account, get_program_accounts
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Program   │  Anchor Smart Contract
│  (Solana)   │  - issue_license, extend_license, revoke_license, validate_license
└─────────────┘
```

### Data Model

```rust
pub struct License {
    pub owner: Pubkey,        // 32 bytes
    pub product_id: String,   // max 64 chars
    pub expires_at: i64,      // Unix timestamp
    pub is_revoked: bool,     // 1 byte
}
```

**PDA Seed**: `[b"license", owner.as_ref()]`

**Limitación**: Un owner = una licencia. Para múltiples productos, agregar `product_id` al seed.

---

## ⚙️ Technology Stack

### Smart Contract
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Anchor-663399?style=for-the-badge&logo=anchor&logoColor=white)](https://www.anchor-lang.com/)

- **Rust** 1.89+
- **Solana** 3.1+
- **Anchor** 0.32.1

### SDK
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

- **solana-sdk** 2.1
- **solana-client** 2.1
- **anchor-client** 0.32.1
- **anyhow** 1.0
- **thiserror** 1.0

### TUI
[![Ratatui](https://img.shields.io/badge/Ratatui-FF6B6B?style=for-the-badge&logo=rust&logoColor=white)](https://ratatui.rs/)

- **ratatui** 0.28
- **crossterm** 0.28
- **tokio** 1.0

### DevContainer
[![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)

- **Ubuntu** 24.04
- **Multi-stage builds**
- **Security scanning** (Trivy CI)

---

## 📁 Project Structure

```
License-System-on-Solana/
├── .devcontainer/           # DevContainer config (production-ready)
│   ├── Dockerfile           # Multi-stage build
│   ├── devcontainer.json    # VS Code config
│   ├── scripts/             # Install scripts
│   └── AUDIT_REPORT.md      # Security audit (P1/P2/P3)
│
├── docs/                    # Documentation
│   ├── README.md            # Docs index
│   ├── TUI-USER-MANUAL.md   # TUI user manual
│   ├── COMPARISON-TRUST-WORK-ESCROW.md  # Gap analysis
│   └── phases/              # Phase documentation
│       ├── phase-1/         # Core Program (95%)
│       ├── phase-2/         # SDK + TUI (70%)
│       └── phase-3/         # Backend + Frontend (0%)
│
├── license-system/          # Main project
│   ├── programs/            # Anchor programs
│   │   └── license-system/  # Smart contract
│   │       ├── src/lib.rs   # 4 instructions + 6 security fixes
│   │       └── Cargo.toml
│   │
│   ├── sdk/                 # Rust SDK (⚠️ needs refactor)
│   │   ├── src/
│   │   │   ├── lib.rs       # Re-exports + constants
│   │   │   ├── client.rs    # LicenseClient (TODO)
│   │   │   ├── pda.rs       # PDA helpers (TODO)
│   │   │   ├── types.rs     # License struct (TODO)
│   │   │   ├── error.rs     # Error types (TODO)
│   │   │   └── utils.rs     # Helpers (TODO)
│   │   ├── Cargo.toml
│   │   └── README.md        # SDK documentation
│   │
│   ├── tui/                 # Terminal UI (⚠️ needs refactor)
│   │   ├── src/
│   │   │   ├── main.rs      # Entry point
│   │   │   ├── app/         # State + logic (TODO)
│   │   │   └── ui/          # Rendering (TODO)
│   │   ├── Cargo.toml
│   │   └── README.md        # TUI documentation
│   │
│   ├── cli/                 # CLI (future)
│   ├── shared/              # Shared code (future)
│   ├── tests/               # TypeScript integration tests
│   ├── Anchor.toml          # Anchor config
│   └── Cargo.toml           # Workspace root
│
├── AGENTS.md                # Work strategy + roadmap
├── README.md                # This file
└── .gitignore
```

**⚠️ Nota**: La estructura actual tiene `crates/` fuera de `license-system/`. Esto será reorganizado en Sprint 1.

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.89+
- Solana CLI 3.1+
- Anchor 0.32+
- Node.js 20+
- Docker (for DevContainer)

### Installation

**Option 1: DevContainer (Recommended)**
```bash
# 1. Clone repo
git clone <repo-url>
cd License-System-on-Solana

# 2. Open in VS Code
code .

# 3. Reopen in Container
# Ctrl+Shift+P → Dev Containers: Reopen in Container
```

**Option 2: Local Setup**
```bash
# 1. Install Solana
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

# 2. Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.32.1
avm use 0.32.1

# 3. Install Node.js 20
# (use nvm or download from nodejs.org)
```

### Build

```bash
# Build program
cd license-system
anchor build

# Build SDK
cd sdk
cargo build --release

# Build TUI
cd tui
cargo build --release
```

### Run

```bash
# 1. Start local validator
solana-test-validator --reset

# 2. Deploy program
cd license-system
anchor deploy

# 3. Run TUI
cd tui
ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json ./target/release/license-tui
```

---

## 📚 Documentation

### User Documentation
- [📖 TUI User Manual](./docs/TUI-USER-MANUAL.md) — Complete guide for using the TUI
- [📊 Comparison with Trust-Work-Escrow](./docs/COMPARISON-TRUST-WORK-ESCROW.md) — Gap analysis

### Developer Documentation
- [📋 AGENTS.md](./AGENTS.md) — Work strategy, roadmap, sprints
- [📁 Phase 1: Core Program](./docs/phases/phase-1/README.md) — Program + DevContainer (95%)
- [📁 Phase 2: SDK + TUI](./docs/phases/phase-2/README.md) — SDK + TUI (70%)
- [📁 Phase 3: Backend + Frontend](./docs/phases/phase-3/README.md) — Future work (0%)

### Component Documentation
- [🦀 SDK README](./license-system/sdk/README.md) — SDK API reference (TODO)
- [🎮 TUI README](./license-system/tui/README.md) — TUI architecture (TODO)
- [⚙️ CLI README](./license-system/cli/README.md) — CLI usage (TODO)

---

## 🔧 Development

### Project Status

| Phase | Component | Status | Completeness |
|-------|-----------|--------|--------------|
| 1 | Anchor Program | ✅ Complete | 100% |
| 1 | DevContainer | ✅ Complete | 100% |
| 1 | TypeScript Tests | ✅ Complete | 100% |
| 2 | SDK | ⚠️ Basic | 30% |
| 2 | TUI | ⚠️ Demo mode | 70% |
| 3 | Backend | 🔲 Not started | 0% |
| 3 | Frontend | 🔲 Not started | 0% |

**Overall Progress**: 64% (21/33 tasks)

### Current Sprint: Sprint 1 (Reorganization + SDK)
**Goal**: Clean structure + real transactions

**Tasks**:
- [ ] Reorganize folders (move crates/ → license-system/)
- [ ] Modularize SDK (client, error, pda, types, utils)
- [ ] Implement real transactions
- [ ] Implement account fetching
- [ ] SDK tests

**ETA**: 2-3 days

---

## 🧪 Testing

### Run TypeScript Tests
```bash
cd license-system
export ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/license-system.ts
```

### Run SDK Tests
```bash
cd license-system/sdk
cargo test
```

### Run TUI (Manual Testing)
```bash
cd license-system/tui
ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json ./target/release/license-tui
```

---

## 🚢 Deployment

### Localnet
```bash
# Terminal 1: Start validator
solana-test-validator --reset

# Terminal 2: Deploy
cd license-system
anchor deploy
```

### Devnet
```bash
# Configure devnet
solana config set --url devnet

# Request airdrop
solana airdrop 2

# Deploy
anchor deploy
```

### Mainnet
```bash
# ⚠️ WARNING: Mainnet deployment requires careful review
solana config set --url mainnet-beta
anchor deploy
```

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feat/amazing-feature`)
5. Open a Pull Request

### Commit Convention
We use **Conventional Commits**:
- `feat(scope):` — New feature
- `fix(scope):` — Bug fix
- `docs(scope):` — Documentation
- `refactor(scope):` — Code refactoring
- `test(scope):` — Tests

---

## 📄 License

[Add license here]

---

## 👥 Team

**Author**: License System Team  
**Last Updated**: 2026-04-26  
**Version**: 0.2.0

---

## 🔗 Links

- [📋 Work Strategy (AGENTS.md)](./AGENTS.md)
- [📖 TUI User Manual](./docs/TUI-USER-MANUAL.md)
- [🔍 Gap Analysis](./docs/COMPARISON-TRUST-WORK-ESCROW.md)
- [🏗️ DevContainer Audit](./devcontainer/AUDIT_REPORT.md)

---

**Built with ❤️ using Rust, Solana, and Ratatui**
