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
