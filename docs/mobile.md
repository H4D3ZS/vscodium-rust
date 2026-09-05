# Mobile toolchain (Android + iPhone)

Everything is driven from the **Devices** panel (right sidebar → DEVICES).

## Android

Launch a headless AVD; the panel embeds the display via `start_emulator_stream` +
`emulator:frame` events (using the SDK-resolved `adb.exe`, not bare `adb` on PATH).
Tap the canvas to send `input tap`. First-time SDK setup: `install-android-sdk.ps1`.

## iPhone

> **About our emulator.** VSCodium-Rust has its own iPhone emulator — a **Rust**
> project (`src-tauri/src/iphone_emulator.rs` + `IPhoneEmulatorPanel.tsx`),
> **still in development**. When it's finished it will be released publicly as its
> own open-source component. Until then, use one of the tools below — the Devices
> panel shells out to them.

### macOS, Apple Silicon (recommended)

On an M-series Mac with **16–32 GB RAM** you can run a real iOS VM without Xcode:

| Tool | What it gives you | License |
|------|-------------------|---------|
| **[vphone-cli](https://github.com/Lakr233/vphone-cli)** by [Lakr233](https://github.com/Lakr233) | A full iPhone emulator **with a UI** — the easiest path. | MIT |
| **[darwin-vm](https://github.com/jprx/darwin-vm)** by [jprx](https://github.com/jprx) | iOS/macOS in **QEMU** (iPhone 12–17, M1–M5). **Root shell + terminal only**, no touch UI — good for headless/CI and low-level work. | MIT |

Both are third-party MIT projects — install them yourself and point the Devices
panel at the binary. We don't vendor or redistribute them; all credit to their
authors.

### macOS, with full Xcode installed

The **iPhone** tab can drive a headless **Xcode Simulator mirror** (no
`Simulator.app` window) — `src-tauri/src/ios_simulator.rs` + `MacIOSSimulatorPanel`.
Requires:

- Full **Xcode** (not just Command Line Tools) — `sudo xcode-select -s /Applications/Xcode.app`
- At least one iOS simulator runtime downloaded

Open **Devices → iPhone → ▶ Mirror**. Helper binaries compile once into
`~/Library/Caches/com.hades.vscode-rust-app/ios-simulator/`; sources are in
`tools/ios-simulator/helpers/` (MIT, from
[codex-plusplus-ios-simulator](https://github.com/b-nnett/codex-plusplus-ios-simulator)).

### Windows / Linux

No hardware-virtualised iOS option exists on non-Apple hosts today. The Rust
emulator above is the planned path; for now, Windows/Linux users build and
**deploy to a physical iPhone** — see [Sideloading](#sideloading) and the iOS
build pipeline in the [README](../README.md#what-you-get).

## Sideloading to a physical device

- **[go-ios](https://github.com/danielpaulus/go-ios)** — install and launch `.ipa`
  builds from any OS, no Xcode.
- **[AltStore](https://github.com/altstoreio/AltStore)** — not bundled (AGPL);
  install AltServer separately for 7-day-cert sideload.

`iPhoneOS.sdk` is the one asset we can't redistribute — supply your own via
`SDKROOT` or the in-app **Import SDK**.
