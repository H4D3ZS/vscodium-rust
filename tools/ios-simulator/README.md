# iOS Simulator helpers (macOS)

Vendored from [b-nnett/codex-plusplus-ios-simulator](https://github.com/b-nnett/codex-plusplus-ios-simulator) (MIT).

- `helpers/sim-capture.swift` — headless CoreSimulator IOSurface → JPEG stream
- `helpers/sim-input.m` — HID touch/button via SimulatorKit

Compiled on first use to `~/Library/Caches/com.hades.vscode-rust-app/ios-simulator/`.

Requirements: full Xcode, `xcode-select` pointing at Xcode.app, iOS simulator runtime installed.
