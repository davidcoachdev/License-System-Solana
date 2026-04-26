#!/bin/bash
set -euo pipefail

# ═══════════════════════════════════════════════════════════════════
# Instala Surfpool CLI
# ═══════════════════════════════════════════════════════════════════

echo "📦 Installing Surfpool CLI..."

# Ensure ~/.local/bin is in PATH
export PATH="$HOME/.local/bin:$PATH"
mkdir -p "$HOME/.local/bin"

# Clean up any existing surfpool installation to avoid symlink errors
rm -f "$HOME/.local/bin/surfpool"
rm -rf "$HOME/.local/share/surfpool"

# Download surfpool installer directly
INSTALLER_URL="https://run.surfpool.run/"
TAR_URL="http://txtx-public.s3.amazonaws.com/releases/surfpool-linux-x64.tar.gz"

echo "Downloading surfpool from ${TAR_URL}..."
curl -sSL -o /tmp/surfpool.tar.gz "$TAR_URL"

echo "Extracting surfpool..."
cd /tmp
if ! gzip -d surfpool.tar.gz; then
    echo "❌ Failed to decompress surfpool.tar.gz"
    exit 1
fi
if ! tar -xf surfpool.tar; then
    echo "❌ Failed to extract surfpool.tar"
    exit 1
fi

echo "Installing surfpool to ${HOME}/.local/bin..."
install -m 0755 surfpool "$HOME/.local/bin/surfpool"

# Verify installation
if command -v surfpool &> /dev/null; then
    echo "✅ Surfpool CLI installed: $(surfpool --version)"
else
    echo "❌ Surfpool CLI installation failed"
    exit 1
fi
