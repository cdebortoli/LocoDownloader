#!/bin/sh
set -e
DEST=/usr/local/bin/loco
echo "Installing loco to $DEST..."
cp "$(dirname "$0")/loco" "$DEST"
chmod +x "$DEST"
echo "Done. Run: loco"
