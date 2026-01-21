# VSCodium Rust Rewrite

A groundbreaking implementation of the VSCodium architecture, rewritten from the ground up using **Rust** and **Tauri**.

## Why was this created?

Electronic-based editors like VS Code have revolutionized development but often come at the cost of high memory usage and performance overhead. This project was born from a simple question: **Can we keep the Developer Experience (DX) of VS Code while shedding the weight of Electron?**

By leveraging Rust and Tauri, we have created an editor that:
- **Starts instantly.**
- **Uses a fraction of the RAM** (Verified < 100MB vs 500MB+).
- **Guarantees Zero Telemetry** by design, auditing every line of code.

This is not just a clone; it is a proof of concept that the future of desktop applications is native, efficient, and private.

## Architecture

- **Frontend**: Vanilla JS / HTML / CSS (No heavy frameworks), mirroring the complex VS Code workbench layout.
- **Backend**: Rust (Tauri), handling file I/O, git operations, and extension management.
- **Extension Host**: A custom Node.js-compatible layer that runs VS Code extensions natively.

## Credits

This project stands on the shoulders of giants:
- **[The VSCodium Team](https://vscodium.com/)**: For their tireless work in creating a binary distribution of VS Code without MS branding/telemetry/tracking.
- **Palinuro**: For pioneering privacy-first open source work and inspiring the "Soul" of this project—absolute user sovereignty and data privacy.

## License

MIT
