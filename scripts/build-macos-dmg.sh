#!/usr/bin/env bash
# build-macos-dmg.sh — Build universal macOS binary and package as .dmg
# Usage: ./scripts/build-macos-dmg.sh
#
# Requirements: Xcode Command Line Tools, Rust with both targets installed:
#   rustup target add x86_64-apple-darwin aarch64-apple-darwin

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION="${VERSION:-1.0.0}"
APP_NAME="VSCodium Rust"
BUNDLE_ID="org.parrotsec.vscodium-rust"

echo "═══════════════════════════════════════════════════════════════"
echo "  VSCodium Rust — macOS Universal DMG Builder"
echo "  Version: $VERSION"
echo "═══════════════════════════════════════════════════════════════"

DIST_DIR="$ROOT/dist"
mkdir -p "$DIST_DIR"

# ── Build for both architectures ──────────────────────────────────────────────
echo ""
echo "▶ Building for x86_64..."
cargo build --release --manifest-path "$ROOT/src-native/Cargo.toml" \
    --target x86_64-apple-darwin

echo ""
echo "▶ Building for aarch64 (Apple Silicon)..."
cargo build --release --manifest-path "$ROOT/src-native/Cargo.toml" \
    --target aarch64-apple-darwin

# ── Create universal binary via lipo ──────────────────────────────────────────
echo ""
echo "▶ Creating universal binary..."
UNIVERSAL_DIR="$ROOT/target/universal-release"
mkdir -p "$UNIVERSAL_DIR"

lipo -create \
    "$ROOT/target/x86_64-apple-darwin/release/vscodium-rust" \
    "$ROOT/target/aarch64-apple-darwin/release/vscodium-rust" \
    -output "$UNIVERSAL_DIR/vscodium-rust"

echo "✅ Universal binary created"

# ── Create .app bundle ────────────────────────────────────────────────────────
echo ""
echo "▶ Creating .app bundle..."

APP_DIR="$ROOT/target/${APP_NAME}.app"
rm -rf "$APP_DIR"

mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Copy binary
cp "$UNIVERSAL_DIR/vscodium-rust" "$APP_DIR/Contents/MacOS/vscodium-rust"
chmod +x "$APP_DIR/Contents/MacOS/vscodium-rust"

# Copy Info.plist
cp "$ROOT/macos/Info.plist" "$APP_DIR/Contents/Info.plist"
# Update version in Info.plist
sed -i.bak "s/1.0.0/$VERSION/g" "$APP_DIR/Contents/Info.plist"
rm -f "$APP_DIR/Contents/Info.plist.bak"

# Copy icon (convert PNG to icns if possible)
if [ -f "$ROOT/src-tauri/icons/icon.icns" ]; then
    cp "$ROOT/src-tauri/icons/icon.icns" "$APP_DIR/Contents/Resources/AppIcon.icns"
elif [ -f "$ROOT/src-tauri/icons/icon.png" ]; then
    # Create iconset from PNG
    ICONSET="$APP_DIR/Contents/Resources/AppIcon.iconset"
    mkdir -p "$ICONSET"
    for size in 16 32 64 128 256 512; do
        sips -z $size $size "$ROOT/src-tauri/icons/icon.png" --out "$ICONSET/icon_${size}x${size}.png" 2>/dev/null || true
        double=$((size * 2))
        sips -z $double $double "$ROOT/src-tauri/icons/icon.png" --out "$ICONSET/icon_${size}x${size}@2x.png" 2>/dev/null || true
    done
    iconutil -c icns "$ICONSET" -o "$APP_DIR/Contents/Resources/AppIcon.icns" 2>/dev/null || true
    rm -rf "$ICONSET"
fi

# Copy supplementary bundles into Resources
for dir in binaries ext-host; do
    if [ -d "$ROOT/$dir" ]; then
        cp -rf "$ROOT/$dir" "$APP_DIR/Contents/Resources/"
    fi
done

# Copy config
if [ -f "$ROOT/mcp_config.json" ]; then
    cp "$ROOT/mcp_config.json" "$APP_DIR/Contents/Resources/"
fi

echo "✅ .app bundle created"

# ── Create DMG ────────────────────────────────────────────────────────────────
echo ""
echo "▶ Creating DMG..."

DMG_NAME="VSCodium-Rust-${VERSION}-macos-universal.dmg"
DMG_PATH="$DIST_DIR/$DMG_NAME"
STAGING="$ROOT/target/dmg-staging"

rm -rf "$STAGING" "$DMG_PATH"
mkdir -p "$STAGING"

cp -R "$APP_DIR" "$STAGING/"
ln -s /Applications "$STAGING/Applications"

hdiutil create -volname "$APP_NAME" \
    -srcfolder "$STAGING" \
    -ov -format UDZO \
    "$DMG_PATH"

rm -rf "$STAGING"

echo "✅ DMG → $DMG_PATH"

# ── Checksums ─────────────────────────────────────────────────────────────────
echo ""
echo "▶ Checksum..."
cd "$DIST_DIR"
shasum -a 256 "$DMG_NAME" >> SHA256SUMS.txt 2>/dev/null || true

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  ✅ macOS DMG build complete!"
echo "  $DMG_PATH"
echo "═══════════════════════════════════════════════════════════════"
