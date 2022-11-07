#!/usr/bin/env bash
set -e

echo Installing shdel...

SHDEL_DIR=${SHDEL_DIR-"$HOME/.shdel"}
SHDEL_BIN_DIR="$SHDEL_DIR/bin"

BIN_URL="https://github.com/xTeKc/shdel/releases/download/v0.1.0-ee241347/shdel-x86_64-unknown-linux-gnu"
BIN_PATH="$SHDEL_BIN_DIR/shdel"

# create .shdel bin dir and shdel bin if they don't exist
mkdir -p "$SHDEL_BIN_DIR"
curl -# -L "$BIN_URL" -o "$BIN_PATH"
chmod +x "$BIN_PATH"

# Store the correct profile file (i.e. .profile for Bash or .zshrc for ZSH).
case $SHELL in
*/zsh)
    PROFILE=$HOME/.zshrc
    PREF_SHELL=zsh
    ;;
*/bash)
    PROFILE=$HOME/.bashrc
    PREF_SHELL=bash
    ;;
*/fish)
    PROFILE=$HOME/.config/fish/config.fish
    PREF_SHELL=fish
    ;;
*)
    echo "shdel: could not detect shell, manually add ${SHDEL_BIN_DIR} to your PATH."
    exit 1
esac

# Only add shdel if it isn't already in PATH.
if [[ ":$PATH:" != *":${SHDEL_BIN_DIR}:"* ]]; then
    # Add the shdel directory to the path and ensure the old PATH variables remain.
    echo >> "$PROFILE" && echo "export PATH=\"\$PATH:$SHDEL_BIN_DIR\"" >> "$PROFILE"
fi

# Warn MacOS users that they may need to manually install libusb via Homebrew:
if [[ "$OSTYPE" =~ ^darwin && ! -f /usr/local/opt/libusb/lib/libusb-1.0.0.dylib ]]; then
    printf "\n" && printf "warning: libusb not found. 
    You may need to install it manually on MacOS via Homebrew (brew install libusb)."
fi

printf "\n" && printf "Detected your preferred shell is "$PREF_SHELL" and added shdel to PATH.
Run 'source "$PROFILE"' or start a new terminal session to use shdel."
