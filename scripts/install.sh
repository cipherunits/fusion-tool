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
        echo "Error: No releases found for ${REPO}."
        echo "Visit https://github.com/${REPO}/releases to create one."
        exit 1
    fi
    VERSION="$TAG"
fi

URL="https://github.com/${REPO}/releases/download/${VERSION}/fusion-${SUFFIX}"

mkdir -p "$INSTALL_DIR"

echo "Downloading fusion-tool ${VERSION}..."

DOWNLOAD_OK=true

if command -v curl &>/dev/null; then
    HTTP_CODE=$(curl -fsSL -w "%{http_code}" -o "${INSTALL_DIR}/${BINARY_NAME}" "$URL" 2>/dev/null) || DOWNLOAD_OK=false
elif command -v wget &>/dev/null; then
    wget -q "$URL" -O "${INSTALL_DIR}/${BINARY_NAME}" || DOWNLOAD_OK=false
else
    echo "Error: Neither curl nor wget is available."
    exit 1
fi

if [ "$DOWNLOAD_OK" = false ] || [ ! -f "${INSTALL_DIR}/${BINARY_NAME}" ] || [ ! -s "${INSTALL_DIR}/${BINARY_NAME}" ]; then
    rm -f "${INSTALL_DIR}/${BINARY_NAME}" 2>/dev/null || true
    echo ""
    echo "Error: Failed to download fusion-tool ${VERSION}."
    echo "The binary for this version may not be available yet."
    echo ""
    echo "Make sure the release exists and has assets uploaded at:"
    echo "  https://github.com/${REPO}/releases/tag/${VERSION}"
    echo ""
    echo "Alternatively, install from source:"
    echo "  git clone https://github.com/${REPO}.git"
    echo "  cd fusion-tool && cargo install --path ."
    exit 1
fi

chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

echo ""
echo "✔ fusion-tool ${VERSION} installed successfully!"
echo "  Location: ${INSTALL_DIR}/${BINARY_NAME}"
echo ""

if [[ ":${PATH}:" != *":${INSTALL_DIR}:"* ]]; then
    echo "WARNING: ${INSTALL_DIR} is not in your PATH."
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"\${PATH}:${INSTALL_DIR}\""
fi