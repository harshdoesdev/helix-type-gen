#!/usr/bin/env sh

set -eu

REPO="harshdoesdev/helix-type-gen"
INSTALL_PATH="/usr/local/bin/helix-ts-gen"
TMP_DIR="$(mktemp -d)"
ARCHIVE_NAME=""
ASSET_URL=""
IS_UPDATE=false

cleanup() {
    echo "Cleaning up temporary files..."
    rm -rf "$TMP_DIR"
}

on_interrupt() {
    echo ""
    echo "Installation interrupted."
    cleanup
    exit 1
}

trap on_interrupt INT TERM
trap cleanup EXIT

download_latest_release() {
    echo "Fetching latest release info..."
    API_RESPONSE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")

    TAG_NAME=$(echo "$API_RESPONSE" | grep -oE '"tag_name":\s*"[^"]+"' | cut -d'"' -f4)
    echo "Latest version: $TAG_NAME"

    if [ "$(uname)" = "Darwin" ]; then
        ARCHIVE_NAME="helix-ts-gen-${TAG_NAME}-aarch64-apple-darwin.zip"
    else
        echo "Error: Unsupported OS. Only macOS is supported by this installer."
        exit 1
    fi

    ASSET_URL=$(echo "$API_RESPONSE" | grep "browser_download_url" | grep "$ARCHIVE_NAME" | cut -d'"' -f4)

    if [ -z "$ASSET_URL" ]; then
        echo "Error: Could not find asset for $ARCHIVE_NAME in release $TAG_NAME."
        exit 1
    fi

    echo "Downloading: $ASSET_URL"
    curl -L "$ASSET_URL" -o "$TMP_DIR/$ARCHIVE_NAME"
}

install_binary() {
    echo "Extracting archive..."
    unzip -q "$TMP_DIR/$ARCHIVE_NAME" -d "$TMP_DIR"

    BINARY_PATH="$TMP_DIR/helix-ts-gen"
    if [ ! -f "$BINARY_PATH" ]; then
        echo "Error: Binary not found in archive."
        exit 1
    fi

    chmod +x "$BINARY_PATH"

    if [ -f "$INSTALL_PATH" ]; then
        IS_UPDATE=true
        echo "helix-ts-gen is already installed. Updating..."
    else
        echo "Installing helix-ts-gen..."
    fi

    sudo mv "$BINARY_PATH" "$INSTALL_PATH"

    if [ "$(uname)" = "Darwin" ]; then
        echo "Removing macOS quarantine flag (if present)..."
        sudo xattr -d com.apple.quarantine "$INSTALL_PATH" 2>/dev/null || true
    fi

    if command -v helix-ts-gen >/dev/null 2>&1; then
        echo ""
        if [ "$IS_UPDATE" = true ]; then
            echo "helix-ts-gen CLI has been successfully updated."
        else
            echo "helix-ts-gen CLI has been successfully installed."
        fi
    else
        echo "Error: Installation failed."
        exit 1
    fi

    if ! echo "$PATH" | grep -q "/usr/local/bin"; then
        echo ""
        echo "Note: /usr/local/bin is not in your PATH."
        echo "Add this to your shell profile:"
        echo "  export PATH=\"/usr/local/bin:\$PATH\""
        echo "Then run: source ~/.zshrc (or restart your terminal)"
    fi
}

download_latest_release
install_binary