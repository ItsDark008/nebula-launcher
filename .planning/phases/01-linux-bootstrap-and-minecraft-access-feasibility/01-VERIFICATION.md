---
phase: 01-linux-bootstrap-and-minecraft-access-feasibility
verified: 2026-11-13T11:46:15Z
status: passed
score: 8/8 must-haves verified
covered_files: [
    ".gitignore",
    ".planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-01-PLAN.md",
    ".planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-01-SUMMARY.md",
    ".planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-02-PLAN.md",
    ".planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-02-SUMMARY.md",
    ".planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-03-PLAN.md",
    ".planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-03-SUMMARY.md",
    "Cargo.lock",
    "Cargo.toml",
    "README.md",
    "crates/nebula-core/Cargo.toml",
    "crates/nebula-core/src/lib.rs",
    "docs/minecraft-access.md",
    "index.html",
    "package-lock.json",
    "package.json",
    "rust-toolchain.toml",
    "src-tauri/Cargo.toml",
    "src-tauri/build.rs",
    "src-tauri/capabilities/main.json",
    "src-tauri/icons/icon.png",
    "src-tauri/src/main.rs",
    "src-tauri/tauri.conf.json",
    "src/App.test.tsx",
    "src/App.tsx",
    "src/fixtures/status-retry.html",
    "src/fixtures/status-retry.tsx",
    "src/main.tsx",
    "src/status.ts",
    "src/styles.css",
    "tsconfig.json",
    "vite.config.ts"
  ]
covered_digest: "v4:sha256:afd00789437032d6c79cece221d94796d9c9648ea06b7f8b3a6841db948d80ec"
behavior_unverified: 0
overrides_applied: 0
decision_coverage: {honored: 3, total: 3, not_honored: []}
---

# Phase 1 · Linux Bootstrap and Minecraft Access Feasibility verification

As a project owner, I want to run the desktop shell on CachyOS and exercise its typed connection to Rust, so that I can see the current Minecraft access status.

Initial verification passed on 2026-11-13.

## User flow coverage

| Step | Expected | Observed evidence | Status |
| --- | --- | --- | --- |
| Open the development shell on CachyOS | One window shows Minecraft access and explains the blocked state. | The native WebKitGTK window opened on niri and rendered the Rust-provided status. | Verified |
| Refresh by keyboard | The request returns a snapshot and Rust sends the matching update. | The WebView inspector captured the typed reply and Rust event independently. Tab and Enter under XWayland activated Refresh status and produced a fresh Rust event. | Verified |
| Retry a failed request in the development fixture | The screen announces the fixed error and keeps a visible keyboard target. Retry restores the status. | The WebKitGTK fixture announced the error and recovered on Enter. | Verified |
| Review the access path | The owner can identify the next registration and review steps and see which evidence is pending. | The runbook records the blocked state, separate milestones, and the public provider checklist. | Verified |

## Observable truths

The first three rows are the roadmap criteria. The remaining rows cover additional plan details.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | The development shell opens on CachyOS x86_64, displays a Rust-provided status, and the Rust core test suite runs without Tauri. | Verified | Native WebKitGTK smoke and a passing `nebula-core` test with `DISPLAY` and `WAYLAND_DISPLAY` unset. The core manifest depends on Serde and uses `serde_json` for its test. |
| 2 | A desktop status request returns a typed result, emits a status event, and shows a sanitized error when the request fails. The WebView can reach only the required commands and events. | Verified | Generated Tauri context and registered IPC test returned all four status fields and delivered the matching event. The capability grants the single app command plus local event listen and unlisten. Rust and frontend failure checks passed. |
| 3 | The owner can see who owns the proposed registration, the verified public-client and review steps, which prerequisites remain open, and whether protected access is blocked. | Verified | The runbook assigns ownership and submission to the project owner. Microsoft Learn confirms the personal-account audience options, native loopback redirect, and PKCE. The Microsoft Forms definition responded with HTTP 200 and confirmed the review input fields. [Owner runbook](../../../docs/minecraft-access.md#registration-checklist) |
| 4 | The React and Vite entry builds on the pinned Node and npm toolchain. | Verified | Node 26.13.1, npm 12.2.2, and Vite 8.3.8 compiled the production entry. `dist` contained the entry, CSS, and JS. |
| 5 | Registration starts at awaiting owner registration. Registration, review submission, approval, and verified protected access have independent status and evidence criteria. | Verified | The Rust test asserts all four serialized fields against literal expected values. The runbook records an independent evidence threshold for each milestone. [Access status](../../../docs/minecraft-access.md#access-status) |
| 6 | Pending registration and review allow Linux bootstrap and fixture work. Phase 6 covers live authentication and Phase 7 covers authenticated launch. | Verified | The native window and credential-free tests ran with protected access blocked. The runbook and roadmap identify the live acceptance prerequisites. [Owner runbook](../../../docs/minecraft-access.md) |
| 7 | A failed request shows a sanitized error and keyboard-accessible retry. The command result remains authoritative when no event arrives. | Verified | The React test renders the reply without an event. Two failure cases verify fixed error text, retry recovery, and retained focus. The native fixture confirmed keyboard activation. |
| 8 | The shell displays awaiting owner registration and blocked protected access from its typed Rust snapshot. | Verified | `App.tsx` renders both messages from the fields received by the typed client. The native window displayed the blocked state and the inspector showed the corresponding snapshot. |

## Required artifacts

| Artifact | Status | Evidence |
| --- | --- | --- |
| `crates/nebula-core/src/lib.rs` | Verified | Owns the credential-free read-only snapshot, enums, and wire-format test. |
| `src/main.tsx` | Verified | The HTML entry mounts the application and the production build includes it. |
| `docs/minecraft-access.md` | Verified | Owner checklist, review inputs, independent evidence ledger, and read-only Secret Service result. Linked from the README. |
| `src-tauri/src/main.rs` | Verified | Registers the command, calls the core, emits to the calling WebView, and maps reader failures to a fixed public error. |
| `src-tauri/capabilities/main.json` | Verified | Explicit `main` capability referenced by Tauri configuration. Generated manifests and the adapter checks confirm the allowlist. |
| `src/status.ts` | Verified | Types all four fields and calls the registered Rust command and event listener. |
| `src/App.tsx` | Verified | Mounts the typed client, handles the response and event, and renders the status and retry. |
| `README.md` | Verified | Records local prerequisites, development and test commands, the smoke procedure, and the owner runbook. |

## Key links and data flow

| From | To | Evidence |
| --- | --- | --- |
| `Cargo.toml` | `crates/nebula-core/Cargo.toml` | The Cargo workspace member resolves in locked metadata and the headless test selects it by package name. |
| `docs/minecraft-access.md` | `01-CONTEXT.md` and `01-RESEARCH.md` | Direct links accompany the owner responsibility, provider evidence, and phase boundaries. |
| `src/App.tsx` | `src/status.ts` | The default client subscribes before reading. The returned snapshot updates React state and drives the displayed registration and protected-access text. |
| `src/status.ts` | `src-tauri/src/main.rs` | Invoke and listener identifiers match. The registered IPC handler returns the core snapshot and sends the same payload to the initiating WebView. |
| `src-tauri/src/main.rs` | `crates/nebula-core/src/lib.rs` | The desktop crate path dependency imports and calls the core's `minecraft_access_status()`. |
| `src-tauri/capabilities/main.json` | `src-tauri/build.rs` | The enabled `main-status` capability grants the command permission generated from the app manifest. |

The Rust core holds the initial access state documented in the owner runbook. The render path consumes the Rust reply and receives Rust events through the typed client. The synthetic failure client is a development-only entry. Both lockfiles use their respective public registries.

## Automated checks

| Check | Result |
| --- | --- |
| Named headless core test `tests::initial_status_records_each_access_milestone` | Passed. |
| Named Rust adapter tests `scoped_status_command_returns_and_emits_the_core_snapshot`, `capability_grants_only_local_status_and_event_subscription`, and `status_failure_discloses_only_the_public_error` | All passed. |
| Frontend response authority, sanitized retry, and subscription cleanup tests | Five cases passed. |
| `cargo fmt --all -- --check`, locked desktop `cargo check`, and `npm run build` | Passed. The production frontend excludes the development fixture. |
| Plan 01-02 runbook structure checks | Both passed. |
| Conventional shell probes | The phase declares no shell probe files. The read-only D-Bus probe is recorded below. |

### Test quality audit

| Test file | Requirements | Active | Skipped | Strongest assertion |
| --- | --- | --- | --- | --- |
| `crates/nebula-core/src/lib.rs` | BASE-01, BASE-03 | 1 | 0 | Literal JSON wire values for all four milestones. |
| `src-tauri/src/main.rs` | BASE-02 | 3 | 0 | Command response and matching event, exact capability grants, and fixed serialized failure. |
| `src/App.test.tsx` | BASE-02 | 5 | 0 | Rendered reply without an event, fixed error followed by retry and focus recovery, and cleanup of late and asynchronously rejected subscriptions. |

The adapter compares its payload with the core result to check parity. The independent core test supplies the expected wire values. The frontend no-event test covers response authority with a registered listener. The initial subscription-rejection fallback was inspected in source.

## Native Linux and provider evidence

- The native development run used CachyOS x86_64, niri/Wayland, GTK 3.24.52, WebKitGTK 2.54.2, Rust and Cargo 1.100.0, Node 26.13.1, and npm 12.2.2. The WebKit inspector observed the command reply and matching event. A separate XWayland run verified keyboard refresh and the credential-free retry fixture. The loopback development server returned the development CSP. A no-bundle build ran with the embedded frontend and production CSP.
- Microsoft Learn documented the personal-account registration choices, system-browser loopback redirect rules, and authorization code with PKCE. The public AppID Review form definition returned the request type, name, client ID, tenant ID, contact, website or domain, justification, and acknowledgment inputs. The Minecraft Help article returned the site shell. The EULA and Usage Guidelines were available directly. [Entra registration](https://learn.microsoft.com/en-us/entra/identity-platform/quickstart-register-app) · [Desktop configuration](https://learn.microsoft.com/en-us/entra/identity-platform/scenario-desktop-app-configuration) · [Redirect rules](https://learn.microsoft.com/en-us/entra/identity-platform/reply-url) · [PKCE](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow) · [AppID Review](https://aka.ms/mce-reviewappid) · [Minecraft EULA](https://www.minecraft.net/en-us/eula) · [Usage Guidelines](https://www.minecraft.net/en-us/usage-guidelines)
- The current CachyOS user bus reported no owner or activation entry for `org.freedesktop.secrets`. The default collection and lock state remain unobserved in this session. [Secret Service probe](../../../docs/minecraft-access.md#secret-service-probe)

## Requirements coverage

| Requirement | Plans | Result |
| --- | --- | --- |
| BASE-01 | 01-01, 01-03 | Satisfied. Native desktop and standalone Rust core verified. |
| BASE-02 | 01-03 | Satisfied. Typed command, targeted app event, sanitized error, and local capability verified. |
| BASE-03 | 01-01, 01-02, 01-03 | Satisfied. Owner checklist, verified public review route, and blocked access state verified. |

All three tracked Phase 1 decisions are covered. The requirements map contains no additional Phase 1 IDs. The changed source scan found no unresolved debt markers or placeholder implementations in the application path. Tauri's local listen and unlisten grants are operation-level permissions. Framework lifecycle events remain observable through those grants, and the launcher defines one application event.

## Remaining prerequisites

Phase 1 manual checks are complete. The owner still needs to confirm Entra eligibility, create the Nebula registration, confirm its saved settings, select a qualifying public URL, and submit the review request. Provider approval, exact protected permissions, and a successful protected call remain open. Phase 6 covers live authentication and keyring verification. Phase 7 covers the authenticated Vanilla install and launch.
