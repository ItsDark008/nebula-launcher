# Walking Skeleton for Nebula Launcher

**Phase** 1
**Generated** 2026-11-12

## Capability to verify end to end

On CachyOS x86_64, the owner can open one local desktop window and refresh the current Minecraft access status. A scoped Tauri command reads the initial status from the Rust core and returns a typed snapshot to the window. Rust also emits a status event to that window. The status begins at awaiting owner registration and identifies protected access as blocked.

## Architectural decisions

| Decision | Choice | Rationale |
| --- | --- | --- |
| Desktop framework | Tauri 2 and a React, TypeScript, and Vite WebView | The window handles presentation while Rust owns status and later launcher operations. |
| Core | The `nebula-core` Rust workspace crate | Domain tests run with Cargo independently of Tauri, WebKitGTK, and a graphical session. |
| Data layer | A read-only status derived from the documented Phase 1 state. SQLite owns non-secret persistence beginning with Phase 3. | The bootstrap status has no user writes. Phase 3 introduces persistence with instance creation and reopening. |
| IPC | `get_launcher_status` returns a typed snapshot. `launcher-status-updated` reports the same snapshot to the calling window. | The command result stays authoritative when event delivery fails. The local `main` window can invoke the status command and manage its event listener. |
| Account access | Owner-created Microsoft Entra public client supporting personal accounts, followed by a separate Minecraft API review | Provider approval and a live protected-access check each need their own evidence. Phase 6 owns system-browser sign-in and Linux Secret Service integration. |
| Deployment target | Local CachyOS x86_64 desktop development using `npm run tauri dev` | A real Linux session exercises the Vite frontend, WebKitGTK, the Tauri boundary, and Rust together. |
| Directory layout | `src/` for React and the typed IPC client, `src-tauri/` for desktop wiring and capabilities, `crates/nebula-core/` for domain logic, and `docs/minecraft-access.md` for the owner runbook | The Rust core, desktop shell, and external access evidence have separate responsibilities. |

## Stack checks

- [ ] Pin a compatible development toolchain and commit npm and Cargo lockfiles after verification.
- [ ] Open the root WebView in the local `main` window.
- [ ] Run the standalone core test and verify the app reads its documented initial state through the IPC adapter.
- [ ] Activate the refresh control by keyboard and observe the returned snapshot and the Rust event in the actual WebView.
- [ ] Confirm the sanitized failure and recovery path with a credential-free fixture.
- [ ] Record the real CachyOS smoke separately from automated checks and the non-secret Secret Service availability probe.

## Subsequent slices

- Phase 2 adds the dark desktop workspace and keyboard navigation.
- Phase 3 adds SQLite-backed instances and verifies that local state survives reopening.
- Phases 4 and 5 add official Minecraft metadata, verified downloads, Vanilla installation, and local Java.
- Phase 6 verifies live authentication and secure account storage for a Nebula-owned authorized client and an eligible account.
- Phase 7 verifies a real authenticated Vanilla install and launch on Linux.
