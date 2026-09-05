# Building VSCodium-Rust on macOS (Apple Silicon / M1)

The codebase is cross-platform; the Windows-only bits are gated, so the Mac build
just works through the standard Tauri flow.

## Prereqs (one time)
```bash
xcode-select --install                                  # Apple Command Line Tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # Rust
# Node 18+ (brew install node, or nvm). Bun only needed for the gpui-react clone.
rustc --version && node --version
```
M1 builds native **aarch64-apple-darwin** by default — no extra target needed.

## Get the code
```bash
git clone https://github.com/H4D3ZS/vscodium-rust-ide-saas.git vscodium-rust
cd vscodium-rust
git checkout main          # or mac_m1
npm install
```

## Dev run
```bash
npm run dev:tauri
```

## Production bundle (.app + .dmg)
```bash
npm run build && npx tauri build
# output: src-tauri/target/release/bundle/{macos/*.app, dmg/*.dmg}
```

## Mac specifics (vs Windows)
- **Renderer:** macOS uses the built-in **WKWebView** — no WebView2 to install/bundle. The `webviewInstallMode` config is under `bundle.windows`, so it's ignored on Mac. (Same Chromium-vs-WebKit caveat: WKWebView is lighter than Windows' Chromium, so memory is generally lower on Mac.)
- **Terminal:** `default_consumer_shell()` is `cfg(windows)`-gated for Git Bash; on macOS it uses `$SHELL` (zsh on modern macOS). No Git Bash bundling needed.
- **Local AI on M1:** Lemonade is AMD/x86-oriented and won't run local models well on Apple Silicon — on M1, point the model picker at a hosted model with your own API key, or run `llama.cpp` (Metal) directly and set it as an OpenAI-compatible endpoint.
- **Code signing / Gatekeeper:** `tauri.conf.json` sets `macOS.signingIdentity: "Apple Development"`. For local testing that's fine; if Gatekeeper blocks the `.app`, right-click → Open, or `xattr -dr com.apple.quarantine <App>.app`. For distribution you need a real **Developer ID** cert + notarization.
- **tree-sitter-typescript** is pinned to `=0.23.0` for a Windows/MSVC reason — it compiles fine on Mac; leave the pin.

## Native gpui app on Mac (bonus)
gpui's most mature platform is macOS, so the native shell should run cleanly:
```bash
cd gpui-ide && cargo run -- ~/path/to/project
```
Expect a ~12 MB native window — this is the lightweight future renderer.

## If the build fails
- Missing system lib → the first `tauri build` error names it; install via `brew`.
- Tauri/WebKit deps are bundled with macOS; no `libwebkit2gtk` needed (that's Linux only).
- Paste the first error and we fix the exact cause — the code itself is platform-gated.
