#!/bin/bash

# Script para abrir la TUI en una ventana flotante de tmux

cd /home/dcdebian/Proyects/License-System-on-Solana/crates/tui

export ANCHOR_WALLET=/home/dcdebian/.config/solana/id.json

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  🚀 Launching License System TUI                           ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📍 Configuration:"
echo "  • Wallet: $ANCHOR_WALLET"
echo "  • Cluster: localnet (http://localhost:8899)"
echo "  • TUI Binary: ./target/release/license-tui (1.2M)"
echo ""
echo "⌨️  Controls:"
echo "  • ↑↓ or 1-6: Navigate menu"
echo "  • Enter: Confirm action"
echo "  • ESC: Return to main menu"
echo "  • q or 6: Exit"
echo ""
echo "📝 Options:"
echo "  1. Issue License    - Format: owner_pubkey,product_id,days"
echo "  2. Extend License   - Format: owner_pubkey,additional_days"
echo "  3. Validate License - Format: owner_pubkey,product_id"
echo "  4. Revoke License   - Format: owner_pubkey"
echo "  5. List Licenses    - Format: owner_pubkey"
echo "  6. Exit"
echo ""
echo "🎮 Starting TUI in 3 seconds..."
sleep 3

./target/release/license-tui
