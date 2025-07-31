#!/usr/bin/env sh

set -eu

cleanup() {
    echo ""
    echo "Installation interrupted."
    exit 1
}

install_helix_ts_gen() {
    BINARY_PATH="$(dirname "$0")/helix-ts-gen"
    INSTALL_PATH="/usr/local/bin/helix-ts-gen"
    IS_UPDATE=false

    if [ ! -f "$BINARY_PATH" ]; then
        echo "Error: Expected binary not found at $BINARY_PATH."
        exit 1
    fi

    echo "Found binary at: $BINARY_PATH"

    # Check if it's an update
    if [ -f "$INSTALL_PATH" ]; then
        IS_UPDATE=true
        echo "helix-ts-gen is already installed. Proceeding with update..."
    fi

    # Make the binary executable
    chmod +x "$BINARY_PATH"

    # Move the binary to /usr/local/bin
    if [ "$IS_UPDATE" = true ]; then
        echo "Updating helix-ts-gen CLI at /usr/local/bin..."
    else
        echo "Installing helix-ts-gen CLI into /usr/local/bin..."
    fi

    sudo mv "$BINARY_PATH" "$INSTALL_PATH"

    # Remove macOS Gatekeeper quarantine flag
    if [ "$(uname)" = "Darwin" ]; then
        echo "Removing macOS quarantine flag..."
        sudo xattr -d com.apple.quarantine "$INSTALL_PATH" || true
    fi

    # Verify installation
    if command -v helix-ts-gen >/dev/null 2>&1; then
        if [ "$IS_UPDATE" = true ]; then
            echo ""
            echo "helix-ts-gen CLI has been successfully updated."
        else
            echo ""
            echo "helix-ts-gen CLI has been successfully installed."
        fi
    else
        echo "Error: Failed to install helix-ts-gen."
        exit 1
    fi

    # Check if /usr/local/bin is in the user's PATH
    if ! echo "$PATH" | grep -q "/usr/local/bin"; then
        echo ""
        echo "Note: /usr/local/bin is not in your PATH."
        echo "Add the following line to your shell profile (.zshrc, .bashrc, or .bash_profile):"
        echo "export PATH=\"/usr/local/bin:\$PATH\""
        echo "Then run: source ~/.zshrc (or restart your terminal)"
    fi
}

trap cleanup INT

install_helix_ts_gen