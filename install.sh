#!/bin/sh

BIN_DIR="$HOME/bin"
BIN_NAME="goi"
URL="https://github.com/thefactlab-org/goi/releases/download/v0.1.0/goi_darwin_arm64"

mkdir -p "$BIN_DIR"
curl -fsSL "$URL" -o "$BIN_DIR/$BIN_NAME"
chmod +x "$BIN_DIR/$BIN_NAME"

if ! grep -q 'export PATH="$HOME/bin:$PATH"' "$HOME/.zshrc" 2>/dev/null; then
    echo 'export PATH="$HOME/bin:$PATH"' >> "$HOME/.zshrc"
fi

export PATH="$HOME/bin:$PATH"

echo "Installed: $BIN_DIR/$BIN_NAME"
echo ""
echo "👉 Please run: source ~/.zshrc"
echo "Usage: goi"
