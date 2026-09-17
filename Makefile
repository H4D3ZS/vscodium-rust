# VSCodium Rust — Cross-Platform Build & Package
# Usage:
#   make build            — cargo build --release (src-native)
#   make package-windows  — build + Inno Setup .exe
#   make package-deb      — build + dpkg-buildpackage
#   make package-rpm      — build + rpmbuild
#   make package-appimage — build + linuxdeploy AppImage
#   make package-macos    — build + universal DMG
#   make release          — build + all packages for current platform
#   make checksums        — sha256sum all dist/ artifacts
#   make clean            — remove build artifacts

VERSION    ?= 1.0.0
CARGO      := cargo
MANIFEST   := src-native/Cargo.toml
DIST       := dist
BUNDLE     := release-bundle

.PHONY: build package-windows package-deb package-rpm package-appimage package-macos release checksums clean

# ── Core Build ────────────────────────────────────────────────────────────────
build:
	$(CARGO) build --release --manifest-path $(MANIFEST)

# ── Windows (Inno Setup) ─────────────────────────────────────────────────────
package-windows: build
	@echo "▶ Building Windows installer..."
	powershell -ExecutionPolicy Bypass -File scripts/build-release.ps1 -SkipBuild

# ── Debian / Parrot (.deb) ────────────────────────────────────────────────────
package-deb: build
	@echo "▶ Building .deb package..."
	dpkg-buildpackage -us -uc -b --no-check-builddeps
	mkdir -p $(DIST)
	find .. -maxdepth 1 -name "vscodium-rust_*.deb" -exec mv {} $(DIST)/ \;
	@echo "✅ .deb → $(DIST)/"

# ── RPM (Fedora/RHEL) ────────────────────────────────────────────────────────
package-rpm: build
	@echo "▶ Building .rpm package..."
	mkdir -p rpmbuild/{SOURCES,SPECS,BUILD,RPMS,SRPMS}
	cp rpm/vscodium-rust.spec rpmbuild/SPECS/
	rpmbuild --define "_topdir $(CURDIR)/rpmbuild" \
	         --define "version $(VERSION)" \
	         -bb rpmbuild/SPECS/vscodium-rust.spec
	mkdir -p $(DIST)
	find rpmbuild/RPMS -name "*.rpm" -exec mv {} $(DIST)/ \;
	@echo "✅ .rpm → $(DIST)/"

# ── AppImage (universal Linux) ────────────────────────────────────────────────
package-appimage: build
	@echo "▶ Building AppImage..."
	bash scripts/build-release.sh --skip-build
	@echo "✅ AppImage → $(DIST)/"

# ── macOS (universal DMG) ────────────────────────────────────────────────────
package-macos:
	@echo "▶ Building macOS universal DMG..."
	VERSION=$(VERSION) bash scripts/build-macos-dmg.sh
	@echo "✅ DMG → $(DIST)/"

# ── Release (all packages for current OS) ─────────────────────────────────────
release: build
ifeq ($(OS),Windows_NT)
	$(MAKE) package-windows
else
	UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
	$(MAKE) package-deb package-rpm package-appimage
endif
ifeq ($(UNAME_S),Darwin)
	$(MAKE) package-macos
endif
endif
	$(MAKE) checksums

# ── Checksums ─────────────────────────────────────────────────────────────────
checksums:
	@mkdir -p $(DIST)
	@cd $(DIST) && sha256sum *.exe *.deb *.rpm *.AppImage *.dmg *.zip 2>/dev/null > SHA256SUMS.txt || true
	@echo "✅ SHA256SUMS.txt → $(DIST)/"

# ── Clean ─────────────────────────────────────────────────────────────────────
clean:
	$(CARGO) clean --manifest-path $(MANIFEST)
	rm -rf $(DIST) $(BUNDLE) rpmbuild AppDir target/universal-release
	rm -f *.AppImage
