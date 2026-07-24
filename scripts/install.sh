#!/bin/bash
set -e

REPO="cipherunits/fusion-tool"
VERSION="latest"

if [ "$1" != "" ]; then
    VERSION="$1"
fi

echo "Installing fusion-tool v${VERSION}..."

OS="$(uname -s)"

case "$OS" in
    Linux*)
        URL="https://github.com/${REPO}/releases/download/${VERSION}/fusion-x86_64-unknown-linux-gnu"
        INSTALL_DIR="$HOME/.local/bin"
        ;;
    Darwin*)
        URL="https://github.com/${REPO}/releases/download/${VERSION}/fusion-x86_64-apple-darwin"
        INSTALL_DIR="/usr/local/bin"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        URL="https://github.com/${REPO}/releases/download/${VERSION}/fusion-x86_64-pc-windows-msvc.exe"
        INSTALL_DIR="$APPDATA/Local/bin"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

mkdir -p "$INSTALL_DIR"

echo "Downloading fusion-tool from ${URL}..."
curl -fsSL "$URL" -o "${INSTALL_DIR}/fusion"

chmod +x "${INSTALL_DIR}/fusion"

echo ""
echo "✔ fusion-tool installed successfully!"
echo "  Location: ${INSTALL_DIR}/fusion"
echo ""
echo "Make sure ${INSTALL_DIR} is in your PATH."