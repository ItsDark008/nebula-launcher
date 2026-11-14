# Nebula Launcher

Nebula Launcher is a Linux desktop launcher for Minecraft Java Edition. The development window includes navigation and Microsoft account sign-in.

## Development on CachyOS

The bootstrap was verified on CachyOS x86_64 with niri on Wayland, GTK 3.24.52, and WebKitGTK 2.54.2. Use Node.js 26.13.1, npm 12.2.2, and the Rust 1.100.0 toolchain from `rust-toolchain.toml`. Install the native build tools and WebKitGTK 4.1 development dependencies from the [Tauri prerequisites for Arch Linux](https://v2.tauri.app/start/prerequisites/). Ensure Cargo is on PATH.

```sh
npm ci
cargo test --locked -p nebula-core
NEBULA_MICROSOFT_CLIENT_ID="<approved-public-client-id>" npm run tauri dev
```

The public client ID identifies the owner-approved Microsoft Entra application and is not a client secret. Supply it through the environment; do not put a client secret, password, token, or authorization code in the repository. The registration must support personal Microsoft accounts and the desktop `http://localhost` redirect. The Linux desktop session needs an available, unlocked Secret Service collection (for example GNOME Keyring). Sign-in opens the system browser, and account refresh credentials are stored in that collection. If the collection is unavailable or locked, unlock it or start a Secret Service provider before retrying.

The development server uses `127.0.0.1:1420`. Open Accounts to sign in or refresh status. To observe the status reply and event separately, set development WebView debugger breakpoints in the status client at invoke resolution and the event callback.

Open `http://127.0.0.1:1420/src/fixtures/status-retry.html` while the development server is running to exercise an initial failure and a successful keyboard retry. The fixture uses synthetic data.

```sh
cargo fmt --all -- --check
cargo check --locked -p nebula-launcher
cargo test --locked --workspace
npm run test -- --run
npm run build
```

The owner registration checklist, review prerequisites, and current access evidence are in [Minecraft Game Service access](docs/minecraft-access.md).
