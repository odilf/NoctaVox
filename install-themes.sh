#!/usr/bin/env sh

SCRIPT_DIR="$(dirname "$0")"
THEME_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/noctavox/themes"

mkdir -p "$THEME_DIR"
cp "$SCRIPT_DIR/docs/theme_examples/"*.toml "$THEME_DIR"

echo "Installed themes to $THEME_DIR"
