#!/bin/bash
set -e

REPO="cipherunits/fusion-tool"

echo "Installing fusion-tool..."

OS="$(uname -s)"

case "$OS" in
    Linux*)
        SUFFIX="x86_64-unknown-linux-gnu"
        INSTALL_DIR="$HOME/.local/bin"
        ;;
    Darwin*)
        SUFFIX="x86_64-apple-darwin"
        INSTALL_DIR="/usr/local/bin"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

BINARY_NAME="fusion"

VERSION="${1:-latest}"

if [ "$VERSION" = "latest" ]; then
    TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"//;s/".*//')
    if [ -z "$TAG" ]; then
        echo "Error: Could not find the latest release of ${REPO}."
        echo "Make sure releases exist at: https://github.com/${REPO}/releases"
        exit 1
    fi
    VERSION="$TAG"
fi

URL="https://github.com/${REPO}/releases/download/${VERSION}/fusion-${SUFFIX}"

mkdir -p "$INSTALL_DIR"

echo "Downloading fusion-tool v${VERSION}..."

if command -v curl &>/dev/null; then
    curl -fsSL "$URL" -o "${INSTALL_DIR}/${BINARY_NAME}"
elif command -v wget &>/dev/null; then
    wget -q "$URL" -O "${INSTALL_DIR}/${BINARY_NAME}"
else
    echo "Error: Neither curl nor wget is available."
    exit 1
fi

chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

echo ""
echo "✔ fusion-tool v${VERSION} installed successfully!"
echo "  Location: ${INSTALL_DIR}/${BINARY_NAME}"
echo ""

if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
    echo "WARNING: ${INSTALL_DIR} is not in your PATH."
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"\${PATH}:${INSTALL_DIR}\""
fi