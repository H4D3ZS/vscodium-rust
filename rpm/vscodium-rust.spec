%global version 1.0.0
%global release 1

Name:           vscodium-rust
Version:        %{version}
Release:        %{release}%{?dist}
Summary:        Native GPU-accelerated code editor and security research IDE
License:        MIT
URL:            https://parrotsec.org
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo >= 0.70
BuildRequires:  rust >= 1.75
BuildRequires:  gcc
BuildRequires:  pkg-config
BuildRequires:  fontconfig-devel
BuildRequires:  libX11-devel
BuildRequires:  libxkbcommon-devel
BuildRequires:  wayland-devel
BuildRequires:  vulkan-loader-devel
BuildRequires:  openssl-devel

Requires:       libX11
Requires:       libxkbcommon
Requires:       fontconfig
Requires:       wayland
Requires:       vulkan-loader

Recommends:     git
Recommends:     python3
Recommends:     gdb

Suggests:       ghidra

%description
VSCodium Rust is an ultra-fast, native GPU-accelerated code editor and IDE
built with the GPUI framework and tailored for security researchers,
penetration testers, and reverse engineers.

Features include:
  * Pure native GPU-accelerated rendering with sub-millisecond typing latency
  * Zero web/Electron runtime memory footprint
  * Integrated mobile security auditing (MobSF, Frida instrumentation)
  * Dynamic MCP server integration (Ghidra, IDA Pro headless)
  * Hardware-accelerated terminal emulation and local zero-telemetry AI assistants

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release --manifest-path src-native/Cargo.toml

%install
install -Dm 755 target/release/vscodium-rust %{buildroot}%{_bindir}/vscodium-rust

# IP Panel wrapper script
install -Dm 755 /dev/stdin %{buildroot}%{_bindir}/vscodium-ip-panel << 'EOF'
#!/bin/sh
exec /usr/bin/vscodium-rust --device-window "$@"
EOF

# Desktop entry
install -Dm 644 debian/vscodium-rust.desktop %{buildroot}%{_datadir}/applications/vscodium-rust.desktop

# Icon
install -Dm 644 src-tauri/icons/icon.png %{buildroot}%{_datadir}/icons/hicolor/512x512/apps/vscodium-rust.png

# AppStream metainfo
install -Dm 644 debian/vscodium-rust.metainfo.xml %{buildroot}%{_datadir}/metainfo/vscodium-rust.metainfo.xml

%files
%license LICENSE
%doc README.md RELEASE_NOTES.md
%{_bindir}/vscodium-rust
%{_bindir}/vscodium-ip-panel
%{_datadir}/applications/vscodium-rust.desktop
%{_datadir}/icons/hicolor/512x512/apps/vscodium-rust.png
%{_datadir}/metainfo/vscodium-rust.metainfo.xml

%changelog
* Wed Sep 17 2026 H4D3ZS <hades@cyber-ifrit.org> - 1.0.0-1
- Initial RPM release
- Native GPU-accelerated code editor with built-in security auditing suites
