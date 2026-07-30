#!/usr/bin/env bash

set -e

REPO="cipherunits/fusion-tool"

echo "Installing fusion-tool..."

VERSION="${FUSION_VERSION:-}"

if [ -z "$VERSION" ]; then
    LATEST_JSON="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" || true)"

    VERSION="$(printf '%s\n' "$LATEST_JSON" \
        | grep '"tag_name"' \
        | head -n 1 \
        | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"
fi

if [ -z "$VERSION" ]; then
    echo "Error: Could not determine the latest version."
    echo "Set FUSION_VERSION and retry, e.g. FUSION_VERSION=v1.0.2"
    exit 1
fi

echo "Version: ${VERSION}"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        TARGET="x86_64-unknown-linux-gnu"
        ;;

    Darwin)
        if [ "$ARCH" = "x86_64" ]; then
            TARGET="x86_64-apple-darwin"
        elif [ "$ARCH" = "arm64" ]; then
            echo "ARM64 macOS is not supported yet."
            exit 1
        else
            echo "Unsupported macOS architecture: $ARCH"
            exit 1
        fi
        ;;

    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

ARCHIVE="fusion-${VERSION}-${TARGET}.tar.gz"

URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE}"

TMP_DIR="$(mktemp -d)"

echo "Downloading ${ARCHIVE}..."

if ! curl -fL "$URL" -o "${TMP_DIR}/${ARCHIVE}"; then
    echo
    echo "Error: Failed to download fusion-tool ${VERSION}."
    echo
    echo "Make sure the release exists:"
    echo "https://github.com/${REPO}/releases/tag/${VERSION}"
    exit 1
fi

echo "Extracting..."

tar -xzf "${TMP_DIR}/${ARCHIVE}" -C "${TMP_DIR}"

INSTALL_DIR="${HOME}/.local/bin"

mkdir -p "$INSTALL_DIR"

mv "${TMP_DIR}/fusion" "${INSTALL_DIR}/fusion"

chmod +x "${INSTALL_DIR}/fusion"

echo
echo "✔ fusion-tool installed successfully!"
echo

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Add this directory to your PATH:"
    echo
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo
fi

echo "Run:"
echo
echo "  fusion --help"