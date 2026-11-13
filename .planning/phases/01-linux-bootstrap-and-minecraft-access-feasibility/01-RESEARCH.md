# Phase 1 Research

**Date:** 2026-11-12

## Linux bootstrap

The target host reported x86_64, KDE Wayland, WebKitGTK 2.54.2, GTK 3.24.52, Node v26.13.1, and npm 12.2.2. Rust, Cargo, and rustup were absent from PATH. The session bus exposed Secret Service and its default KDE Wallet collection reported an unlocked state. This establishes availability in the observed session. Check the dependencies again during execution and record the toolchain and desktop versions used in the actual smoke test. [Tauri Linux prerequisites](https://v2.tauri.app/start/prerequisites/) · [Vite requirements](https://vite.dev/guide/) · [Secret Service specification](https://specifications.freedesktop.org/secret-service/latest/)

Use Tauri 2 with the approved React, TypeScript, and Vite frontend. Resolve compatible versions against the actual package engines, Cargo requirements, and Linux prerequisites before pinning them. Use npm and commit both lockfiles to make clean installs reproducible. Keep `nebula-core` independently testable with no Tauri or GUI dependencies. The desktop crate depends on that core and maps its result to an IPC response. [Tauri Vite setup](https://v2.tauri.app/start/frontend/vite/)

## Desktop status and IPC

Use one local WebView and one read-only typed status command. A successful refresh returns a typed snapshot and emits a status event to the calling window. Subscribe before requesting status and unsubscribe when the view unmounts. Give the initial screen an explicit loading state, a readable result, and an accessible retry path for a sanitized failure. Keep the response authoritative if event delivery fails. Use credential-free fixtures for the error path and for frontend checks. The core owns status semantics, including the initial registration state. The Tauri adapter owns delivery. [Tauri commands](https://v2.tauri.app/develop/calling-rust/) · [Tauri events](https://v2.tauri.app/develop/calling-frontend/)

Register the custom command in the application manifest so the capability governs it. Scope the permission and required event listen and unlisten permissions to the local main window. Emit events from Rust to that window. Keep arbitrary path, network, and shell operations outside the IPC surface. Test the capability configuration against the application manifest, and check the real WebView for a success response and event. [Tauri capabilities](https://v2.tauri.app/security/capabilities/) · [Tauri permissions](https://v2.tauri.app/security/permissions/)

## Package Legitimacy Audit

Queried the npm and crates.io package metadata on 2026-11-12. The identifiers below resolved to their expected upstream repositories. Observed versions are candidates. Resolve engines, Rust version requirements, peer dependencies, and platform support together before pinning.

| Registry and exact identifiers | Observed version and upstream |
| --- | --- |
| npm `@tauri-apps/api`, `@tauri-apps/cli` | 2.12.4 each. [Tauri](https://github.com/tauri-apps/tauri) |
| crates.io `tauri`, `tauri-build` | 2.12.5 and 2.7.4. [Tauri](https://github.com/tauri-apps/tauri) |
| npm `react`, `react-dom` | 19.3.1 each. [React](https://github.com/react/react) |
| npm `@types/react`, `@types/react-dom` | 19.3.0 each. [DefinitelyTyped](https://github.com/DefinitelyTyped/DefinitelyTyped) |
| npm `typescript` | 7.0.2. [TypeScript](https://github.com/microsoft/TypeScript) |
| npm `vite`, `@vitejs/plugin-react` | 8.3.8 and 6.3.0. [Vite](https://github.com/vitejs/vite) · [React plugin](https://github.com/vitejs/vite-plugin-react) |
| npm `vitest`, `@testing-library/react`, `jsdom` | 5.1.0, 16.3.3, and 31.0.0. [Vitest](https://github.com/vitest-dev/vitest) · [React Testing Library](https://github.com/testing-library/react-testing-library) · [jsdom](https://github.com/jsdom/jsdom) |
| npm `@testing-library/dom` | 10.4.2 satisfies the React Testing Library 16.3.3 peer range `^10.0.0`. [DOM Testing Library](https://github.com/testing-library/dom-testing-library) |
| crates.io `serde`, `serde_json` | 1.0.229 and 1.0.154. [Serde](https://github.com/serde-rs/serde) · [Serde JSON](https://github.com/serde-rs/json) |

## Microsoft Entra and Minecraft review

Start with an owner-created Entra application registration that supports personal Microsoft accounts. Microsoft documents public desktop clients using system-browser authorization code with PKCE and a registered loopback redirect. Determine the exact audience, redirect and required permissions before implementing sign-in. Use the Application (Client) ID as the app identifier and keep account credentials and tokens out of repository artifacts. [Microsoft desktop app registration](https://learn.microsoft.com/en-us/entra/identity-platform/scenario-desktop-app-registration) · [Microsoft authorization code flow](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow)

Minecraft has a separate application review. The official review form loaded, and its first-party form definition exposed fields for request type, app name, application client ID, tenant ID for new approvals, a valid contact email, an associated public website or domain, and an API-access justification. The form also requests acknowledgment of Minecraft's terms and usage guidelines. A private repository is not the public project URL the form asks for. The owner needs to supply a qualifying public URL before submission. [Minecraft API review article](https://help.minecraft.net/hc/en-us/articles/16254801392141) · [Minecraft review form](https://aka.ms/mce-reviewappid) · [Minecraft Usage Guidelines](https://www.minecraft.net/en-us/usage-guidelines)

Record the current state as awaiting owner registration. Track registration, submission, approval, and verified protected access independently. The review form establishes what it asks for. Owner eligibility, access for a Nebula-owned client, the precise provider permissions, and approval timing remain open. Phase 1 prepares the runbook and records the evidence. Phases 6 and 7 own live authentication and authenticated launch acceptance. The Linux bootstrap can proceed while the owner completes registration or waits for review.

## Validation Architecture

| Area | Automated evidence | Manual evidence |
| --- | --- | --- |
| BASE-01 | `cargo test --locked -p nebula-core`, `npm run build`, and `cargo check --locked -p nebula-launcher` once the planned packages exist. | Open the dev shell with `npm run tauri dev` from a CachyOS desktop session. Record toolchain, architecture, and WebKitGTK versions. |
| BASE-02 | Core and adapter tests for public status/error serialization, frontend tests for status and retry states, and checks for the main-window capability and manifest. | Refresh the status in the actual WebView and confirm receipt of the Rust event. |
| BASE-03 | Validate the status shown for a fresh setup and confirm that the runbook and referenced sources exist. | Reopen official registration and review sources, verify form fields, and record the Secret Service availability probe without accessing credential values. |

All build and test commands above are execution targets. The planning review observed host prerequisites and public documentation. Application builds, desktop smoke tests, and protected API verification remain pending.
