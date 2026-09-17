#!/usr/bin/env bash
# build-release.sh — Unified cross-platform build & package script
# Usage: ./scripts/build-release.sh [--package-only] [--skip-build]
#
# Detects host OS/arch, builds the native binary, assembles a release bundle,
# and invokes the platform-appropriate packager.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION="${VERSION:-1.0.0}"

# ── Detect platform ──────────────────────────────────────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)   PLATFORM="linux"  ;;
    Darwin*)  PLATFORM="macos"  ;;
    MINGW*|MSYS*|CYGWIN*) PLATFORM="windows" ;;
    *)        echo "❌ Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
    x86_64|amd64)   ARCH_LABEL="x64"    ;;
    aarch64|arm64)   ARCH_LABEL="arm64"  ;;
    *)               ARCH_LABEL="$ARCH"  ;;
esac

echo "═══════════════════════════════════════════════════════════════"
echo "  VSCodium Rust — Release Build"
echo "  Platform: $PLATFORM ($ARCH_LABEL)    Version: $VERSION"
echo "═══════════════════════════════════════════════════════════════"

SKIP_BUILD=false
PACKAGE_ONLY=false
for arg in "$@"; do
    case "$arg" in
        --skip-build)    SKIP_BUILD=true ;;
        --package-only)  PACKAGE_ONLY=true; SKIP_BUILD=true ;;
    esac
done

# ── Build ─────────────────────────────────────────────────────────────────────
if [ "$SKIP_BUILD" = false ]; then
    echo ""
    echo "▶ Building native binary (release)..."
    cargo build --release --manifest-path "$ROOT/src-native/Cargo.toml"
    echo "✅ Build complete"
fi

# ── Determine binary name ─────────────────────────────────────────────────────
if [ "$PLATFORM" = "windows" ]; then
    BIN_EXT=".exe"
else
    BIN_EXT=""
fi

RELEASE_DIR="$ROOT/target/release"
BINARY="$RELEASE_DIR/vscodium-rust${BIN_EXT}"

if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found at $BINARY"
    echo "   Run without --skip-build first."
    exit 1
fi

# ── Assemble release bundle ───────────────────────────────────────────────────
BUNDLE_DIR="$ROOT/release-bundle"
echo ""
echo "▶ Assembling release bundle → $BUNDLE_DIR"

mkdir -p "$BUNDLE_DIR"

# Core binaries
cp -f "$BINARY" "$BUNDLE_DIR/vscodium-rust${BIN_EXT}"
# The IP panel is the same binary launched with --device-window
cp -f "$BINARY" "$BUNDLE_DIR/vscodium-ip-panel${BIN_EXT}"

# Supplementary binaries (if they exist)
for item in ios${BIN_EXT} frida${BIN_EXT}; do
    if [ -f "$ROOT/$item" ]; then
        cp -f "$ROOT/$item" "$BUNDLE_DIR/$item"
    fi
done

# Supplementary directories
for dir in binaries ext-host devimages; do
    if [ -d "$ROOT/$dir" ]; then
        cp -rf "$ROOT/$dir" "$BUNDLE_DIR/"
    fi
done

# Config files
for cfg in mcp_config.json; do
    if [ -f "$ROOT/$cfg" ]; then
        cp -f "$ROOT/$cfg" "$BUNDLE_DIR/$cfg"
    fi
done

echo "✅ Release bundle assembled"

if [ "$PACKAGE_ONLY" = false ] && [ "$SKIP_BUILD" = true ]; then
    echo "Done (skip-build mode, no packaging)."
    exit 0
fi

# ── Package ───────────────────────────────────────────────────────────────────
echo ""
echo "▶ Packaging for $PLATFORM..."

DIST_DIR="$ROOT/dist"
mkdir -p "$DIST_DIR"

case "$PLATFORM" in
    linux)
        echo ""
        echo "  ── Debian (.deb) ──"
        if command -v dpkg-buildpackage &>/dev/null; then
            cd "$ROOT"
            dpkg-buildpackage -us -uc -b --no-check-builddeps 2>&1 | tail -5
            # Move .deb to dist/
            find "$ROOT/.." -maxdepth 1 -name "vscodium-rust_*.deb" -exec mv {} "$DIST_DIR/" \;
            echo "✅ .deb package → $DIST_DIR/"
        else
            echo "⚠ dpkg-buildpackage not found, skipping .deb"
        fi

        echo ""
        echo "  ── RPM (.rpm) ──"
        if command -v rpmbuild &>/dev/null; then
            RPMBUILD_DIR="$ROOT/rpmbuild"
            mkdir -p "$RPMBUILD_DIR"/{SOURCES,SPECS,BUILD,RPMS,SRPMS}
            cp "$ROOT/rpm/vscodium-rust.spec" "$RPMBUILD_DIR/SPECS/"
            # Create source tarball
            tar czf "$RPMBUILD_DIR/SOURCES/vscodium-rust-${VERSION}.tar.gz" \
                -C "$ROOT/.." --transform="s/^$(basename "$ROOT")/vscodium-rust-${VERSION}/" \
                "$(basename "$ROOT")/src-native" \
                "$(basename "$ROOT")/src-tauri" \
                "$(basename "$ROOT")/debian" \
                "$(basename "$ROOT")/Cargo.toml" 2>/dev/null || true
            rpmbuild --define "_topdir $RPMBUILD_DIR" \
                     --define "version $VERSION" \
                     -bb "$RPMBUILD_DIR/SPECS/vscodium-rust.spec" 2>&1 | tail -5
            find "$RPMBUILD_DIR/RPMS" -name "*.rpm" -exec mv {} "$DIST_DIR/" \;
            echo "✅ .rpm package → $DIST_DIR/"
        else
            echo "⚠ rpmbuild not found, skipping .rpm"
        fi

        echo ""
        echo "  ── AppImage ──"
        if command -v linuxdeploy &>/dev/null || [ -f "$ROOT/tools/linuxdeploy-x86_64.AppImage" ]; then
            LINUXDEPLOY="${ROOT}/tools/linuxdeploy-x86_64.AppImage"
            if ! [ -f "$LINUXDEPLOY" ]; then
                LINUXDEPLOY="$(command -v linuxdeploy)"
            fi
            APPDIR="$ROOT/AppDir"
            rm -rf "$APPDIR"
            mkdir -p "$APPDIR/usr/bin"
            mkdir -p "$APPDIR/usr/share/applications"
            mkdir -p "$APPDIR/usr/share/icons/hicolor/512x512/apps"

            cp -f "$BINARY" "$APPDIR/usr/bin/vscodium-rust"
            cp -f "$ROOT/appimage/vscodium-rust.desktop" "$APPDIR/usr/share/applications/"
            cp -f "$ROOT/src-tauri/icons/icon.png" "$APPDIR/usr/share/icons/hicolor/512x512/apps/vscodium-rust.png"
            # Copy supplementary bundles into AppDir
            for dir in binaries ext-host; do
                if [ -d "$ROOT/$dir" ]; then
                    cp -rf "$ROOT/$dir" "$APPDIR/usr/bin/"
                fi
            done

            OUTPUT="VSCodium_Rust-${VERSION}-${ARCH_LABEL}.AppImage" \
            "$LINUXDEPLOY" --appdir "$APPDIR" \
                --desktop-file "$APPDIR/usr/share/applications/vscodium-rust.desktop" \
                --icon-file "$APPDIR/usr/share/icons/hicolor/512x512/apps/vscodium-rust.png" \
                --output appimage 2>&1 | tail -5

            find "$ROOT" -maxdepth 1 -name "*.AppImage" -exec mv {} "$DIST_DIR/" \;
            echo "✅ AppImage → $DIST_DIR/"
        else
            echo "⚠ linuxdeploy not found, skipping AppImage"
            echo "  Download: https://github.com/linuxdeploy/linuxdeploy/releases"
        fi
        ;;

    macos)
        echo "  Delegating to build-macos-dmg.sh..."
        bash "$SCRIPT_DIR/build-macos-dmg.sh"
        ;;

    windows)
        echo "  Use PowerShell: scripts/build-release.ps1"
        echo "  Or run directly: ISCC.exe installer.iss"
        ;;
esac

# ── Checksums ─────────────────────────────────────────────────────────────────
echo ""
echo "▶ Generating checksums..."
cd "$DIST_DIR"
if ls *.exe *.deb *.rpm *.AppImage *.dmg *.zip 2>/dev/null | head -1 > /dev/null; then
    sha256sum *.exe *.deb *.rpm *.AppImage *.dmg *.zip 2>/dev/null > SHA256SUMS.txt || true
    echo "✅ SHA256SUMS.txt → $DIST_DIR/"
else
    echo "⚠ No artifacts found in $DIST_DIR for checksums"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  ✅ Release build complete!"
echo "  Artifacts: $DIST_DIR/"
echo "═══════════════════════════════════════════════════════════════"
