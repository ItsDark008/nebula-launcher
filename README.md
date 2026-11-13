# Nebula Launcher

Nebula Launcher is a Linux desktop launcher for Minecraft Java Edition. The current development window shows the initial Minecraft Game Service access status.

## Development on CachyOS

The bootstrap was verified on CachyOS x86_64 with niri on Wayland, GTK 3.24.52, and WebKitGTK 2.54.2. Use Node.js 26.13.1, npm 12.2.2, and the Rust 1.100.0 toolchain from `rust-toolchain.toml`. Install the native build tools and WebKitGTK 4.1 development dependencies from the [Tauri prerequisites for Arch Linux](https://v2.tauri.app/start/prerequisites/). Ensure Cargo is on PATH.

```sh
npm ci
cargo test --locked -p nebula-core
npm run tauri dev
```

The development server uses `127.0.0.1:1420`. Open the launcher and use Tab and Enter to refresh the status. To observe the reply and event separately, set development WebView debugger breakpoints in the status client at invoke resolution and the event callback.

Open `http://127.0.0.1:1420/src/fixtures/status-retry.html` while the development server is running to exercise an initial failure and a successful keyboard retry. The fixture uses synthetic data.

```sh
cargo fmt --all -- --check
cargo check --locked -p nebula-launcher
cargo test --locked --workspace
npm run test -- --run
npm run build
```

The owner registration checklist, review prerequisites, and current access evidence are in [Minecraft Game Service access](docs/minecraft-access.md).
