# Architecture Research: Nebula Launcher

**Domain:** Linux Minecraft Java Edition launcher  
**Researched:** 2026-11-12  
**Status:** Approved by the project owner on 2026-11-12.  
**Confidence:** MEDIUM. Primary documentation supports the platform boundaries. The component integration and recovery protocol still require implementation and crash testing.

## Recommended architecture

Use one local desktop application, with a thin Tauri shell and a Rust core that has no Tauri dependency. Keep provider-specific behavior behind adapters, not a generic plugin system. React owns presentation state; Rust owns authoritative operations, validation, secrets, and persistence. [Tauri IPC](https://v2.tauri.app/concept/inter-process-communication/) supports request/response commands and one-way events.

```text
React views + typed client
       │ commands / results; progress and lifecycle events
Tauri transport: validation, permissions, DTO mapping
       │
Rust application services + domain models / typed errors
       ├─ instances and operations ───── SQLite + instance directories
       ├─ downloads / artifact cache ─── HTTPS + verified files
       ├─ authentication ─────────────── browser + Linux Secret Service
       ├─ metadata + installer ───────── official version sources
       ├─ Java + process supervisor ──── local Java process
       └─ provider ports ─────────────── Modrinth / loader adapters
```

### Component boundaries

| Component | Responsibility | Boundary |
|---|---|---|
| Domain and services | Instance IDs, operation states, install plans, launch specifications, typed errors, orchestration and invariants | Rust interfaces, independent of Tauri and concrete network clients |
| Tauri transport | Validate IPC input, call services, map safe DTOs, publish progress; enforce least-privilege capabilities | No raw filesystem, shell or secret API exposed to the WebView; restrict window origins and permissions ([runtime authority](https://v2.tauri.app/security/runtime-authority/)) |
| Persistence | Schema migrations, instance settings, operation journal, account identifiers and cached metadata | SQLite repository; secrets are excluded |
| Instance service | Create, edit, duplicate, remove, import/export and resolve managed paths | One mutable game directory per immutable instance ID; shared immutable verified artifacts only |
| Download manager | Bounded concurrency, retry/backoff, cancellation, streamed hashing, progress and staging | Download spec includes source, expected size/hash when available, destination and policy |
| Minecraft metadata / Vanilla | Resolve platform rules, dependencies, assets, native libraries and launch arguments | Official metadata adapter produces a version-specific installation and launch plan |
| Auth and secret store | Browser sign-in, account entitlement/session handling, refresh and logout | Secret-store port backed by Linux Secret Service; UI receives account identity/status only |
| Java and process supervisor | Discover/validate local Java, check compatibility, build command, spawn and monitor child | Process runner port and redacted diagnostics; never execute a constructed shell string |
| Loader adapters | Resolve compatibility and execute the specific Fabric, Quilt, NeoForge or Forge installation procedure | Dedicated adapter per loader; shared install plan and launch contribution, no assumption of identical installers |
| Modrinth adapter | Typed API, catalog cache, compatible file selection, dependency metadata and pack import/export | Independent from launch path; preserve provider IDs and hashes; respect rate-limit headers and identifying User-Agent ([Modrinth API](https://docs.modrinth.com/api/)) |

## Suggested layout

```text
src/                        React views, components, state, typed IPC client
src-tauri/                  application setup, command DTOs, events, capabilities
crates/nebula-core/         domain, ports, services, install and launch plans
crates/nebula-infra/        SQLite, HTTP, filesystem, keyring and process adapters
crates/nebula-providers/    official metadata, loaders and Modrinth adapters
tests/fixtures/             sanitized metadata, archives and fake process fixtures
```

Start with a Rust workspace and a few cohesive crates; split further only when boundaries warrant it. The paths are proposed, not an existing implementation. Ports should be narrow enough to inject fake HTTP, clock, filesystem, process and secret stores into tests.

## IPC and operation model

Commands such as `list_instances`, `create_instance`, `start_install`, `get_operation`, `cancel_operation`, `start_launch`, `list_accounts` and `start_sign_in` accept validated IDs and typed inputs and return serializable results or stable error codes plus safe display messages. Long operations return an operation ID immediately. Publish throttled `operation_progress` and `process_state` events carrying IDs, phase, progress and sequence/version, with no secrets. Events can be missed: subscribe first, then query an authoritative snapshot and reconcile by sequence; reconnect or reload must recover through `get_operation`. Enforce per-instance mutation/launch locks and serialize conflicting operations. Apply Tauri capabilities to explicitly registered commands, and validate authorization and inputs again in Rust; do not load untrusted remote content into a privileged WebView.

## Data flows

1. **Install:** validate instance and local Java requirement → persist operation intent → resolve metadata and artifact manifest → stage bounded downloads and verify supplied hashes/sizes → publish immutable artifacts and instance installation → commit installed state → emit status. A failed verification never becomes a launchable artifact.
2. **Sign in:** open the system browser with the approved public-client flow; validate callback correlation, expiry and redirect according to the chosen flow → backend exchanges and refreshes tokens → verify eligibility → store secrets in Secret Service and only account references/status in SQLite. Handle missing, locked or unavailable keyring explicitly; never silently fall back to plaintext. The precise access prerequisites are a separate early feasibility gate.
3. **Launch:** resolve an installed instance and valid Minecraft session → select compatible local Java and assemble JVM/classpath/game arguments from validated metadata and instance settings → spawn Java directly with separate arguments and instance working directory → concurrently drain bounded stdout/stderr, redact sensitive output, wait/reap child, persist outcome and emit lifecycle events. Do not log arguments containing credentials; command-line arguments can be visible to local processes, so minimize their exposure and document unavoidable game-protocol limitations. Decide launcher-exit behavior explicitly; Tokio does not kill children on drop by default ([Tokio process](https://docs.rs/tokio/latest/tokio/process/struct.Command.html)).
4. **Content:** browse cached Modrinth results → resolve compatible version/dependencies → preview changes → stage and verify files and any pack overrides → commit with recovery journal. Installed game launch uses local metadata and files; Modrinth availability is not a launch dependency. A valid Minecraft session is still required.

## Persistence, atomicity and recovery

- Keep SQLite and mutable instance data on local storage; schema migrations run transactionally with backups and explicit versioning. Track instances, operations, artifact references and publication state; retain keyring references only, never credentials.
- SQLite atomicity does **not** encompass external files ([SQLite atomic commit](https://www.sqlite.org/atomiccommit.html)). Before altering files, persist operation ID, intended paths, previous state and phase. Stage on the destination filesystem, verify, flush files, rename into place, and sync parent directories where durability is required ([Linux rename](https://man7.org/linux/man-pages/man2/rename.2.html), [fsync](https://man7.org/linux/man-pages/man2/fsync.2.html)). Cross-filesystem rename is not assumed atomic.
- Use explicit `planned → staging → publishing → committed` states. On startup reconcile each unfinished operation idempotently: verify the staged or published artifacts and finish the commit when safe, otherwise restore previous references or mark repairable failure. SQLite transaction marks the final logical state; never claim DB and filesystem commit together atomically. Fault-inject termination at every boundary and test disk-full, hash mismatch and permissions failures.
- Keep a prior working installation until replacement is verified and published. Deletion requires confirmation, a canonical managed-path check, and a recoverable trash/backup policy; never delete shared cache artifacts while referenced. Back up user worlds/configuration before destructive changes. Archive extraction rejects absolute paths, traversal, symlink escapes, duplicate/colliding paths and unbounded expansion; do not trust downloaded filenames.

## Test and roadmap implications

- First establish core ports, persistence and operation recovery, then a complete Vanilla vertical slice including real account and game launch on Linux x86_64. Keep credentials out of fixtures and automated tests; use a separate manual authenticated smoke test.
- Unit-test metadata rules and launch-plan generation with representative fixtures; integration-test actual SQLite, temporary directories, local HTTP responses, fake Java executables, cancellation, concurrent operations and restart reconciliation. Test a real Linux keyring separately, including locked/missing service.
- Add each loader as an independent adapter with fixture coverage and a real installation plus launch acceptance test. Research each installer protocol before implementation. Add Modrinth content and safe pack operations after Vanilla is stable; verify outage-independent launch and recovery.
- Practical scaling is local: bound concurrent downloads, streaming memory and log retention; avoid premature distributed infrastructure. A cache cleanup policy must use references and never discard mutable instance data.

## Open decisions and risks

- Secret Service availability varies by desktop session; validate selected Rust implementation, unlock prompts, backend behavior, and packaging on the target system ([Secret Service specification](https://specifications.freedesktop.org/secret-service/latest/)).
- Minecraft metadata, authentication access and each loader's actual installer behavior require current provider-specific validation. This document does not assert entitlement access is available.
- Choose a concrete cross-filesystem import/export strategy, backup retention, child-process behavior when the launcher exits, SQLite journal/synchronous settings and exact Java compatibility matrix during phase design. Crash durability depends on filesystem and storage guarantees.

## Sources

- [Tauri IPC](https://v2.tauri.app/concept/inter-process-communication/) and [runtime authority](https://v2.tauri.app/security/runtime-authority/) — commands, events and capabilities.
- [SQLite atomic commit](https://www.sqlite.org/atomiccommit.html), [Linux rename](https://man7.org/linux/man-pages/man2/rename.2.html), [Linux fsync](https://man7.org/linux/man-pages/man2/fsync.2.html) — transactional and filesystem boundaries.
- [Secret Service specification](https://specifications.freedesktop.org/secret-service/latest/) — Linux secret storage interfaces.
- [Tokio process Command](https://docs.rs/tokio/latest/tokio/process/struct.Command.html) — direct process invocation and lifecycle.
- [Modrinth API overview](https://docs.modrinth.com/api/) — API identity, rate limits and stable identifiers.
