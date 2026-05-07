#!/bin/sh
set -e

BIN_DIR="$HOME/bin"
BIN_NAME="goi"
URL="https://github.com/nidz-the-fact/goi/releases/download/v0.1.0/goi_darwin_arm64"

mkdir -p "$BIN_DIR"
curl -fsSL "$URL" -o "$BIN_DIR/$BIN_NAME"
chmod +x "$BIN_DIR/$BIN_NAME"

grep -q 'export PATH="$HOME/bin:$PATH"' "$HOME/.zshrc" 2>/dev/null || echo 'export PATH="$HOME/bin:$PATH"' >> "$HOME/.zshrc"
source "$HOME/.zshrc"

echo "Installed: $BIN_DIR/$BIN_NAME"
echo ""
echo "Usage: ${BINARY_NAME}"
