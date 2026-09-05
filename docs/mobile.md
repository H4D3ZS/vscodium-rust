# Mobile Toolchain (Android + iPhone + vPhone)

Integrated in **Devices** panel (right sidebar → DEVICES).

## Tabs

| Tab | Purpose |
|-----|---------|
| **Android** | Launch AVD headless, embedded display via ADB screencap (`emulator:frame` events) |
| **iPhone** | acheron / Virtual-iPhone-Emulator (user-provided firmware) |
| **Toolchain** | vPhone phony-Xcode shims, `vphone-doctor`, AltStore deploy notes |

## Android emulator fix

The Devices panel now uses **`start_emulator_stream`** + **`emulator:frame`** (SDK-resolved `adb.exe`), not bare `adb` on PATH. Tap the canvas to send `input tap`.

## vPhone toolchain

Set `VPHONE_ROOT` to your `Virtual-iPhone-Emulator` folder, or place it beside the IDE workspace.

```powershell
cd Virtual-iPhone-Emulator\toolchain
install_doctor.bat
vphone-doctor.bat
```

Toolchain tab runs these from the IDE. Flutter/RN tooling sees phony Xcode + vPhone Bridge when doctor passes.

## AltStore

Not bundled (AGPL). Install [AltServer](https://github.com/altstoreio/AltStore) separately for `.ipa` sideload to physical devices.

---

## iPhone emulator — setup & required files

The iPhone-emulator **integration** ships with the IDE (`src-tauri/src/iphone_emulator.rs`
+ `src/components/IPhoneEmulatorPanel.tsx`). On **macOS** (Apple Silicon, Intel, Hackintosh
with Xcode), the **iPhone** tab uses a headless **Xcode Simulator mirror** adapted from
[codex-plusplus-ios-simulator](https://github.com/b-nnett/codex-plusplus-ios-simulator)
(`src-tauri/src/ios_simulator.rs` + `MacIOSSimulatorPanel.tsx`) — no `Simulator.app`
window, tap/keyboard/home via CoreSimulator helpers compiled on first use.

On **Windows/Linux**, the panel drives the **`acheron`** C++ hypervisor (`IPhoneAcheronPanel`),
streams the serial console + boot logs, captures the display, and feeds touch events back.
Use the **Legacy** button on macOS if you still want acheron there.

What is **NOT** in this repository (intentionally — too large / device firmware /
machine-specific) and must be provided locally:

| Artifact | Why it's excluded | Where the IDE looks for it |
|----------|-------------------|----------------------------|
| `acheron` hypervisor binary | Built per-platform from C++ source | `<project>/build/Release/acheron[.exe]`, `<project>/build/acheron[.exe]`, or on `PATH` |
| iOS firmware `.ipsw` | ~7–11 GB device firmware (Apple's) | You pass it to `acheron prepare`; output lands in `<project>/out/` |
| `Virtual-iPhone-Emulator/` app + ramdisk artifacts | Standalone/proprietary, large | Generated into `<project>/out/…` by `acheron prepare` |

> The repo's `.gitignore` excludes `Virtual-iPhone-Emulator/`, `*.ipsw`, and the build
> output. Cloning gives you a **buildable IDE**; the acheron feature stays idle until you
> provide these files. On macOS you can use the Xcode Simulator mirror without acheron.

---

## macOS — Xcode Simulator mirror (recommended)

Requirements (same as Codex++ tweak):

- Full **Xcode** installed (not Command Line Tools only)
- `sudo xcode-select -s /Applications/Xcode.app` if needed
- At least one iOS simulator runtime downloaded in Xcode

Open **Devices → iPhone**, pick a device, click **▶ Mirror**. Helpers compile once to
`~/Library/Caches/com.hades.vscode-rust-app/ios-simulator/`. Source helpers live in
`tools/ios-simulator/helpers/` (MIT, from codex-plusplus-ios-simulator).

---

## 1. Build the `acheron` hypervisor

`acheron` is a CMake C++ project. From its source directory:

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
```

This produces:
- Windows: `build/Release/acheron.exe`
- macOS/Linux: `build/acheron`

Put the binary on your `PATH`, or keep it under the project's `build/` so the IDE's
`find_acheron()` locates it automatically.

> If you don't have the `acheron` source, obtain it from your private
> Virtual-iPhone-Emulator repository — it is not redistributed here.

## 2. Get an iOS firmware (`.ipsw`)

Download the **official** restore image for the device/iOS version you want to emulate
(e.g. from Apple's signing servers / ipsw.me). Only use firmware you are licensed to use.

## 3. Prepare the firmware

`acheron prepare` extracts the real ramdisk + kernelcache + devicetree the emulator boots:

```bash
acheron prepare --ipsw /path/to/iPhone..._Restore.ipsw --out out
```

This writes (under `<project>/out/`):
- `out/raw/initrd.bin` — ramdisk (the IDE auto-fills this as the disk)
- kernelcache + devicetree (auto-loaded by `acheron run` via `rd=md0`)
- `out/diagnostics/` — runtime channel: `frame.raw` / `guest_frame.raw` (display),
  `touch_in.csv` (host→guest pointer)

## 4. Launch from the IDE

Open the **Emulator** panel → it calls `launch_iphone_emulator(project_path)` →
`acheron run …`, streams `emulator-console` (logs) and `emulator-frame` (display) events,
and sends touches via `send_iphone_touch`.

---

## Expected layout

```
<project>/
├── build/
│   └── Release/acheron.exe        # (or build/acheron on macOS/Linux)
├── out/
│   ├── raw/initrd.bin             # from `acheron prepare`
│   └── diagnostics/               # frame.raw, guest_frame.raw, touch_in.csv
└── (your .ipsw lives anywhere; pass its path to `acheron prepare`)
```

## Notes
- **macOS (Apple Silicon):** the IDE builds and runs without the emulator. To run the
  emulator you still need an `acheron` macOS build + a firmware you can use.
- These artifacts are user-provided for legal + size reasons; nothing here distributes
  Apple firmware or the proprietary emulator app.
