# 🎮 License System TUI

Terminal User Interface (TUI) for managing software licenses on Solana blockchain. Built with Ratatui.

---

## 📖 Overview

License System TUI is an interactive terminal application that provides a user-friendly interface for administrators to manage licenses on Solana. It features a clean, responsive UI with keyboard navigation and real-time feedback.

---

## ✨ Features

### Current Features (v0.1.0)
- ✅ **Main Menu** — 6 options with keyboard navigation
- ✅ **Issue License** — Create new license
- ✅ **Extend License** — Add more days
- ✅ **Validate License** — Check license status
- ✅ **Revoke License** — Permanently revoke
- ✅ **List Licenses** — Show license info
- ✅ **Input Validation** — Format checking and error messages
- ✅ **Status Bar** — Real-time feedback
- ⚠️ **Demo Mode** — Shows info but doesn't execute transactions

### Planned Features (v0.2.0+)
- [ ] **Real Transactions** — Execute on-chain operations
- [ ] **Search Screen** — Find licenses by owner, product, status
- [ ] **List Screen** — Table view of multiple licenses
- [ ] **Settings Menu** — Theme, Network, Wallets
- [ ] **Wallet Manager** — Add, delete, switch wallets
- [ ] **Transaction History** — View recent operations
- [ ] **Logging** — Debug logs to file
- [ ] **Themes** — Multiple color schemes

---

## 🚀 Quick Start

### Prerequisites
- Solana Test Validator running
- Wallet configured at `~/.config/solana/id.json`
- Program deployed to localnet

### Build

```bash
cd license-system/tui
cargo build --release
```

### Run

```bash
# Option 1: Using script
./run-tui.sh

# Option 2: Manual
export ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json
./target/release/license-tui
```

---

## ⌨️ Controls

| Key | Action |
|-----|--------|
| `↑` `↓` | Navigate menu |
| `1-6` | Quick select option |
| `Enter` | Confirm / Execute |
| `ESC` | Return to main menu |
| `q` or `6` | Exit application |
| `Backspace` | Delete character in input |

---

## 📋 Operations

### 1. Issue License

**Input Format**:
```
owner_pubkey,product_id,days
```

**Example**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,premium-plan,30
```

**What it does**:
- Derives License PDA from owner
- Creates license account on-chain
- Sets: owner, product_id, expires_at, is_revoked=false

---

### 2. Extend License

**Input Format**:
```
owner_pubkey,additional_days
```

**Example**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,15
```

**What it does**:
- Verifies license is not revoked
- Checks grace period (7 days)
- Adds days to expires_at

---

### 3. Validate License

**Input Format**:
```
owner_pubkey,product_id
```

**Example**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,premium-plan
```

**What it does**:
- Checks: !is_revoked && expires_at > now && product_id matches
- Returns true/false

---

### 4. Revoke License

**Input Format**:
```
owner_pubkey
```

**Example**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c
```

**What it does**:
- Sets is_revoked = true
- ⚠️ Permanent (cannot be undone)

---

### 5. List Licenses

**Input Format**:
```
owner_pubkey
```

**Example**:
```
3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c
```

**What it shows**:
- License PDA
- Bump seed
- Owner pubkey
- Payer pubkey
- Program ID

---

## 🏗️ Architecture

### Current Architecture (v0.1.0)
```
main.rs (373 lines)
├── App struct
│   ├── screen: Screen enum
│   ├── input: String
│   ├── status_message: String
│   └── sdk_client: Option<LicenseClient>
├── Screen enum (6 variants)
├── ui() function
└── main() function
```

**Status**: ⚠️ Monolithic (all in one file)

### Target Architecture (v0.2.0)
```
tui/src/
├── main.rs              # Entry point + main loop
├── lib.rs               # Re-exports
├── app/                 # State management
│   ├── mod.rs
│   ├── state.rs         # App struct
│   ├── events.rs        # Event handling
│   └── config.rs        # Settings
└── ui/                  # Rendering
    ├── mod.rs
    ├── layout.rs        # UI rendering
    ├── screens.rs       # Screen-specific UI
    └── widgets.rs       # Custom widgets
```

**Status**: 🔲 Planned (Sprint 3)

---

## 🎨 UI Components

### Main Menu
```
┌─────────────────────────────────────────────────┐
│ License System on Solana - TUI                  │
└─────────────────────────────────────────────────┘
┌ Main Menu - Use ↑↓ or numbers to select ────────┐
│ 1. Issue License                                │
│ 2. Extend License                               │
│ 3. Validate License                             │
│ 4. Revoke License                               │
│ 5. List Licenses                                │
│ 6. Exit                                         │
└─────────────────────────────────────────────────┘
┌ Status ──────────────────────────────────────────┐
│ Connected to Solana localnet                    │
└─────────────────────────────────────────────────┘
```

### Input Screen
```
┌─────────────────────────────────────────────────┐
│ License System on Solana - TUI                  │
└─────────────────────────────────────────────────┘
┌ IssueLicense - Press ESC to return ─────────────┐
│ 3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,p│
│                                                 │
└─────────────────────────────────────────────────┘
┌ Status ──────────────────────────────────────────┐
│ Issue License - Enter: owner_pubkey,product_id,d│
└─────────────────────────────────────────────────┘
```

---

## 🔧 Configuration

### Environment Variables

```bash
# Wallet path (required)
export ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json

# RPC URL (optional, defaults to localnet)
export ANCHOR_PROVIDER_URL=http://127.0.0.1:8899

# Log level (optional)
export RUST_LOG=debug
```

### Config File (Future)

```toml
# ~/.config/license-tui/config.toml
[network]
rpc_url = "http://127.0.0.1:8899"

[wallet]
path = "~/.config/solana/id.json"

[ui]
theme = "default"

[cache]
enabled = true
ttl_seconds = 30
```

---

## 🐛 Troubleshooting

### TUI doesn't start
**Error**: `Failed to initialize SDK: Failed to load keypair`

**Solution**:
```bash
# Check wallet exists
ls -la ~/.config/solana/id.json

# Set ANCHOR_WALLET
export ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json
```

### TUI shows "Demo mode"
**Cause**: SDK doesn't have real transaction functions

**Solution**: Wait for Sprint 1 (SDK refactor) or use TypeScript tests for real transactions

### Invalid pubkey error
**Cause**: Pubkey format is incorrect

**Solution**: Use valid base58 pubkey (43-44 characters)
```
Valid: 3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c
Invalid: abc123
```

---

## 🧪 Testing

### Manual Testing

```bash
# 1. Start validator
solana-test-validator --reset

# 2. Deploy program
cd ../
anchor deploy

# 3. Run TUI
cd tui
ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json ./target/release/license-tui
```

### Automated Testing (Future)

```bash
cargo test
```

---

## 📊 Performance

### Binary Size
- **Debug**: ~50MB
- **Release**: 1.2MB

### Startup Time
- **Cold start**: ~100ms
- **With cache**: ~50ms

### Memory Usage
- **Idle**: ~5MB
- **Active**: ~10MB

---

## 🚧 Roadmap

### v0.1.0 (Current) ✅
- [x] Basic UI with 6 options
- [x] Input validation
- [x] PDA derivation
- [x] Status bar
- [x] Demo mode

### v0.2.0 (Sprint 1-2)
- [ ] Real transactions
- [ ] Account fetching
- [ ] Search screen
- [ ] List screen with table
- [ ] CRUD complete

### v0.3.0 (Sprint 3)
- [ ] Modular architecture (app/, ui/)
- [ ] FormField system
- [ ] Settings menu
- [ ] Wallet manager
- [ ] Logging

### v1.0.0 (Production)
- [ ] Transaction history
- [ ] Themes
- [ ] Network switcher
- [ ] Polish UX
- [ ] Full documentation

---

## 🤝 Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

---

## 📄 License

[Add license here]

---

## 🔗 Links

- [📖 User Manual](../../docs/TUI-USER-MANUAL.md)
- [🦀 SDK Documentation](../sdk/README.md)
- [📋 Work Strategy](../../AGENTS.md)

---

**Last Updated**: 2026-04-26  
**Version**: 0.1.0  
**Status**: ⚠️ Demo mode (needs real transactions)
