# Stack Research

**Project:** Nebula Launcher  
**Researched:** 2026-11-12  
**Confidence:** MEDIUM — official documentation and registry snapshots support the choices, but the complete combination has not been installed, resolved, built, or tested. Version listings may change between research and implementation.

## Recommendation

Use Tauri 2 with a standalone Rust core, Tokio for asynchronous work, SQLite for local non-secret state, and a React/TypeScript frontend built with Vite, Tailwind CSS, and shadcn/ui. This follows the project brief and keeps launch, network, filesystem, authentication, and data logic testable without the desktop shell. Run version resolution and compatibility checks before committing the first lockfiles; the observations below are candidates, not installed or tested selections.

## Version candidates

| Component | Observed release or line on 2026-11-12 | Proposed selection and reason |
|---|---|---|
| Tauri | Rust crate **2.12.5**, according to [docs.rs](https://docs.rs/tauri/latest/tauri/) | Tauri 2; resolve mutually compatible Rust crate, CLI and JS API releases, not necessarily identical patch numbers. Native Linux WebKit and narrow command surface fit the desktop target. |
| Rust | **1.99.1**, announced 2026-10-15 in the [release index](https://blog.rust-lang.org/releases/); the stable notes and forge pages can lag | Stable toolchain, pinned after a clean build; verify release-day updates and dependency MSRV. |
| Tokio | **1.54.1** on [crate documentation](https://docs.rs/tokio/latest/tokio/); another cached docs page still shows 1.53.1 | Tokio 1.x; confirm crates.io at resolution time. Async transfers and process supervision stay in the Rust core. |
| SQLite | **3.54.1**, 2026-10-16, per [release chronology](https://www.sqlite.org/chronology.html) | SQLite through `rusqlite` 0.42 candidate; bundled mode offers reproducibility, but inspect the actual linked SQLite version and licensing. The engine's latest upstream version is not necessarily the bundled version. |
| React / React DOM | npm lists React **19.3.1** ([registry](https://www.npmjs.com/package/react?activeTab=versions)) | React 19 with a matching React DOM version; confirm component peer dependencies. |
| TypeScript | npm lists **7.0.2** ([registry](https://www.npmjs.com/package/typescript?activeTab=versions)) | Test 7.0.x against the actual Vite, React types, and lint tooling before selection; use a supported earlier release if necessary. |
| Vite | npm lists **8.3.8** ([registry](https://www.npmjs.com/package/vite?activeTab=versions)); [support policy](https://vite.dev/releases) lists 8.3 as current | Vite 8.3.x with a compatible React plugin. |
| Tailwind | npm lists **4.3.5** ([registry](https://www.npmjs.com/package/tailwindcss?activeTab=versions)) | Tailwind 4 with matching `@tailwindcss/vite`; follow the [shadcn Vite guide](https://ui.shadcn.com/docs/installation/vite). |
| shadcn/ui | Source-distributed components and CLI, not a single runtime package version | Generate only needed components, review and own the resulting source, and pin its actual dependencies. Verify keyboard, focus, and dark-theme behavior. |
| Node.js | [Node release schedule](https://nodejs.org/en/about/previous-releases) lists 24 as LTS; listing may lag | Node 24 LTS candidate. Vite's [guide](https://vite.dev/guide/) lists 20.19+ or 22.12+; Node 20 is EOL. Recheck engines and peer dependencies during lockfile resolution. |

Registry pages and documentation were accessed on the research date; crawl snapshots have differing ages, so none establishes a tested compatibility matrix. The compiler's release on the research date may have advanced beyond the latest release listing observed.

## Supporting choices

| Need | Recommendation | Boundary / decision to verify |
|---|---|---|
| HTTP and verified downloads | `reqwest` with explicit TLS configuration, streaming, timeouts and bounded concurrency; `sha1`/`sha2` only as required by the trusted provider metadata | Pin versions and TLS backend after inspecting dependency features; compare size and digest before atomic promotion, and never treat an untrusted supplied digest alone as authenticity. |
| Serialization and errors | `serde`, `serde_json`, `thiserror` | Strong typed contracts and structured, redacted diagnostics; do not serialize credentials to the UI. |
| Persistence | [`rusqlite`](https://docs.rs/crate/rusqlite/latest), migrations, bounded blocking database work | Keep SQLite off async executor threads; ensure instance data has a recovery and migration strategy. No tokens in SQLite. |
| Credentials | OS Secret Service via a reviewed Rust keyring backend; [`keyring` 4.2.0](https://docs.rs/crate/keyring/latest) is a candidate | Explicitly select a persistent Linux Secret Service backend and test locked, absent, unavailable, and restart cases. Do not silently fall back to plaintext or assume kernel key storage has persistence. |
| Archive extraction | Maintained ZIP handling library with a constrained extraction adapter | Reject absolute and traversal paths, unsafe links, collisions and resource exhaustion. Inspect archive-specific format and metadata for each provider. |
| IPC | Tauri commands and events, with a reviewed generated or shared typed contract | Restrict capabilities, validate every Rust-side input, and keep secrets and arbitrary command execution out of the frontend. |

These supporting library versions are deliberately not locked before a build and security review; candidate availability does not establish compatibility or security suitability.

## Linux prerequisites and packaging

Tauri's [Linux prerequisites](https://v2.tauri.app/start/prerequisites/) list for Arch: `webkit2gtk-4.1`, `base-devel`, `curl`, `wget`, `file`, `openssl`, `appmenu-gtk-module`, `libappindicator-gtk3`, `librsvg`, and `xdotool`, plus Rust and Node. CachyOS is the development target; confirm package availability against the configured repositories and the actual features used. No installation was performed.

Test the actual system WebKitGTK, not just a browser: [Tauri's WebView documentation](https://v2.tauri.app/reference/webview-versions/) says Linux uses WebKitGTK and varies by distribution. Check CSS and JavaScript support against the oldest supported runtime. For Linux x86_64, start with a reproducible native package and choose distributable formats only after testing runtime dependencies, desktop integration and the target baseline. [Tauri's Debian documentation](https://v2.tauri.app/distribute/debian/) describes WebKitGTK and GTK dependencies; [AppImage documentation](https://v2.tauri.app/distribute/appimage/) describes its separate bundling considerations. Test any AppImage against relevant FUSE, graphics, process execution and system library environments; its portability is not guaranteed. Do not publish without explicit authorization.

## Local Java and distribution constraints

Detect local executables and permit an explicit per-instance path. Inspect version and architecture, compare to the actual game's version metadata, and provide actionable errors for missing or incompatible Java. Do not assume one Java release works for all game versions or loaders. As a concrete example, Minecraft **1.20.5 requires Java 21 and a 64-bit OS**, according to the [official 2024-04-23 release notes](https://www.minecraft.net/en-us/article/minecraft-java-edition-1-20-5). Loader-specific requirements need separate verification and real launch tests. Automatic Java distribution is outside the MVP.

The app must authenticate a licensed user and fetch authorized artifacts using official metadata; do not bundle or redistribute game assets, loader installers, Java distributions, or third-party modpacks without reviewing the applicable terms and permissions. Packaging a launcher does not itself establish redistribution rights.

## Alternatives and exclusions

| Choice | Alternative | Tradeoff |
|---|---|---|
| Tauri 2 | Electron | Electron can standardize the browser engine but adds a bundled runtime; choose it only if WebKitGTK limitations prove blocking. |
| SQLite with Rust-owned access | JSON-only files | JSON is simple for small settings, but transactional migrations and concurrent state become harder. |
| Local Java | Bundled/downloaded Java | Managed runtimes simplify onboarding but add distribution, update, platform and licensing work; explicitly deferred. |
| System secret store | SQLite, logs or UI state for credentials | Persistent plaintext secrets violate the project constraint; unavailable secure storage must produce a clear recoverable error. |

## Validation before implementation

After architecture approval, resolve exact dependencies and commit `Cargo.lock` and the selected frontend package-manager lockfile together. Check peer dependencies, MSRV, Node engines, dependency licenses and vulnerability advisories, then build and smoke-test on Linux x86_64/CachyOS with the selected WebKitGTK. Confirm the linked SQLite version, keyring persistence after restart, Java discovery, and a licensed real-account launch separately from credential-free automated fixtures. No installation, lockfile, build, authenticated test, or package publication was performed in this research.
