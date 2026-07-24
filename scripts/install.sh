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
        BINARY_NAME="fusion"
        ;;
    Darwin*)
        URL="https://github.com/${REPO}/releases/download/${VERSION}/fusion-x86_64-apple-darwin"
        INSTALL_DIR="/usr/local/bin"
        BINARY_NAME="fusion"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        URL="https://github.com/${REPO}/releases/download/${VERSION}/fusion-x86_64-pc-windows-msvc.exe"
        INSTALL_DIR="$APPDATA/Local/bin"
        BINARY_NAME="fusion.exe"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

mkdir -p "$INSTALL_DIR"

echo "Downloading fusion-tool from ${URL}..."

if command -v curl &>/dev/null; then
    curl -fsSL "$URL" -o "${INSTALL_DIR}/${BINARY_NAME}"
elif command -v wget &>/dev/null; then
    wget -q "$URL" -O "${INSTALL_DIR}/${BINARY_NAME}"
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

chmod +x "${INSTALL_DIR}/${BINARY_NAME}" 2>/dev/null || true

echo ""
echo "✔ fusion-tool installed successfully!"
echo "  Location: ${INSTALL_DIR}/${BINARY_NAME}"
echo ""

if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
    echo "WARNING: ${INSTALL_DIR} is not in your PATH."
    echo "Add the following to your shell profile (~/.bashrc or ~/.zshrc):"
    echo "  export PATH=\"\${PATH}:${INSTALL_DIR}\""
fi