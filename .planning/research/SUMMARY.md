# Project Research Summary

**Project:** Nebula Launcher  
**Domain:** Linux desktop launcher for Minecraft Java Edition and Modrinth  
**Researched:** 2026-11-12  
**Confidence:** MEDIUM

## Executive Summary

Nebula Launcher should first prove that an eligible player can sign in, install and run Minecraft Java Edition in an isolated instance, and reopen the launcher with their settings intact. Use a thin Tauri shell over a standalone Rust core. Make instance persistence, verified downloads, local Java selection, and redacted process diagnostics part of the first playable slice. Add Fabric, Quilt, NeoForge, and Forge through dedicated adapters, then build native Modrinth workflows on the same installation and recovery infrastructure. Native Modrinth is required for the first complete Linux release.

Start the protected Minecraft Game Service access work in the foundation phase. The research confirms a documented Microsoft desktop sign-in flow and a reachable Minecraft review form. Nebula's registration eligibility, approval status, and protected endpoint access remain open. Public Mojang and Modrinth reads support the proposed data integrations. Authenticated launch, loader installs, pack changes, and packaged desktop behavior require separate verification on Linux x86_64, beginning on CachyOS.

## Key Findings

### Recommended Stack

- Tauri 2 provides the desktop shell and a narrow IPC surface. Test the actual WebKitGTK runtime and resolve compatible Tauri Rust, CLI, and JavaScript releases together.
- Stable Rust and Tokio 1.x own concurrent downloads, installation, account handling, and Java process supervision. Use SQLite via `rusqlite` for local non-secret state and a persistent Linux Secret Service backend for credentials.
- React 19 with matching React DOM, TypeScript, Vite, Tailwind 4, and shadcn/ui support the dark, keyboard-accessible desktop UI. Node 24 LTS, Vite 8.3.x, and TypeScript 7.0.x are candidates. Confirm engines, peer dependencies, Rust MSRV, and the linked SQLite version in a clean target build before pinning toolchains and lockfiles. [Stack research](STACK.md)

### Expected Features

- **Vanilla MVP** includes isolated persistent instances, official release and snapshot metadata, integrity-checked downloads with cancellation and retries, local Java detection and configuration, eligible Microsoft accounts, actual game launch, and progress, logs, and diagnostics.
- **Complete Linux release** adds four individually verified loaders and native Modrinth browsing and per-instance management for mods, modpacks, resource packs, and shaders. It includes compatibility and dependency handling, `.mrpack` import and export, updates, backups, and restore. Installed instances must launch with a valid Minecraft session during a Modrinth outage.
- **Deferred scope** includes Windows support, automatic Java provisioning, and a required light theme. The launcher uses official authentication and entitlement checks. [Feature research](FEATURES.md)

### Architecture Approach

React owns presentation. Tauri transports typed commands and events under least-privilege capabilities. The independent Rust core owns validation, orchestration, install and launch plans, and redacted errors. Infrastructure adapters handle SQLite, filesystem, HTTP, Secret Service, and direct Java process invocation. Provider adapters handle Mojang metadata, each loader's installation procedure, and Modrinth.

Use one mutable directory per instance and share verified immutable artifacts. Serialize conflicting operations per instance. Persist an operation journal before staging files, publish verified content on the destination filesystem, and reconcile interrupted operations on restart. SQLite tracks logical state and recovery across filesystem changes. Long-running IPC operations expose authoritative snapshots so the UI can recover after missed events. [Architecture research](ARCHITECTURE.md)

### Critical Pitfalls

1. **Protected API access blocks authenticated acceptance.** Confirm ownership, the current review process, required permissions, and project access early. Test each authentication exchange and account check with a Nebula-owned authorized client and eligible account.
2. **Game and loader metadata varies by version.** Derive Java and platform requirements, native libraries, and arguments from supported metadata. Test representative releases and snapshots and prove each loader's real installation and launch.
3. **Interrupted operations can corrupt an installation or lose user data.** Stage and verify downloads, journal publication, preserve the previous usable installation, and exercise restart recovery and rollback. Validate pack paths and links, limit expansion, and handle file conflicts before applying overrides.
4. **Credentials can leak through storage or diagnostics.** Keep them in the Rust account service and Linux Secret Service, redact logs, diagnostics, and IPC, and test restart and locked or unavailable keyring behavior.
5. **External services and packaging can break local workflows.** Keep Modrinth outside the launch dependency path, honor catalog rate limits, and test browser callbacks, keyring, Java discovery, and launch from a clean packaged desktop session. [Integration and release risks](PITFALLS.md)

## Implications for Roadmap

### Phase 1 · Foundation and protected access

The project owner approved the architecture and roadmap on 2026-11-12. Identify the owner of the project registration, verify personal Microsoft account and redirect settings, confirm the current protected API review process and permissions, and track access status immediately. Establish the Linux build and typed IPC contract. Check Secret Service availability early. Authorized protected access is a prerequisite for live authentication in Phase 6 and accepting Phase 7's authenticated launch. Public metadata, fixture tests, and local installation work can advance while access is pending.

### Phases 2–7 · Vanilla MVP

Connect official metadata and verified artifact installation to persistent instances, local Java, account management, launch supervision, and diagnostics. Exercise cancellation, integrity failures, and interrupted installs with credential-free fixtures. In a separate manual Linux test, record success for Microsoft sign-in, Xbox/XSTS, Minecraft session, entitlement, profile retrieval, real game launch, and reopening the launcher with saved instance settings. Cover relevant failure paths and secure session persistence. This is the first playable milestone.

### Phases 8–9 · Loader adapters

Build Fabric and Quilt first using their documented profile APIs. Develop NeoForge and Forge separately after confirming the supported game, loader, and Java combinations and how their installers produce usable client profiles and files. Record independent real installation and launch evidence for all four. The stable Vanilla pipeline supplies shared libraries, artifact verification, and process supervision.

### Phases 10–14 · Native Modrinth and recovery

Start with catalog search, filtering, paging, and detail views, then deliver compatible mods and dependencies, `.mrpack` install/import/export, resource packs, shaders, and updates. Build conflict handling, safe staging, backup, and recovery into the first workflow that changes instance content. Validate license-aware exports, client override order, interrupted changes, and launch of an installed instance during a Modrinth outage with a valid Minecraft session. Loader and Vanilla installation support underpin this phase.

### Phases 15–16 · Complete Linux release

Harden and profile the full application, finalize CI and documentation, and choose and validate Linux packages on the supported target. Check desktop integration, accessibility, installed package behavior, and applicable branding and distribution terms. The complete release includes the Vanilla MVP, all four loaders, and native Modrinth with recovery. Public distribution requires explicit owner authorization.

### Phase Ordering Rationale

Start the external access dependency alongside local foundation work. A real Vanilla run establishes the baseline for troubleshooting loader issues. Loader support then enables modpack installation, and the operation journal gives Modrinth updates a way to preserve and restore working instances. Implement integrity and recovery controls in each phase that introduces the relevant operation.

### Research Flags

- **Phase 1** needs early provider confirmation for protected API access and target testing for the exact WebKitGTK and Secret Service configuration. Standard Tauri IPC, SQLite migrations, and React UI patterns can proceed with documented approaches.
- **Phases 4–7** need a focused check of supported metadata rules, local Java and native-library combinations, and authenticated end-to-end behavior. Standard process supervision and fixture-based testing can follow documented patterns.
- **Phases 8–9** need version-specific installer research for NeoForge and Forge and a real compatibility matrix for all four loaders. Fabric and Quilt profile retrieval follows documented APIs.
- **Phases 10–14** need targeted research on dependency edge cases, allowed export content and external references, and recovery for imports or exports across filesystems. Catalog search and pagination follow the documented Modrinth API.
- **Phases 15–16** need target-specific package validation and distribution review. Routine CI and documentation can use standard patterns.

## Confidence Assessment

| Area | Confidence | Notes |
|---|---|---|
| Stack | MEDIUM | Official documentation and registry snapshots support the stack. Exact versions, compatibility, and target build are open. |
| Features | MEDIUM | Scope is explicit and public Modrinth search, project, and filtered version reads succeeded. Install, update, export, and launch remain to be tested. |
| Architecture | MEDIUM | Primary documentation supports IPC, process, persistence, and filesystem boundaries. Their composition and crash recovery require implementation evidence. |
| Pitfalls | MEDIUM | Official sources and live metadata samples establish concrete constraints. Protected API access, loader behavior, and package behavior remain open. |

**Overall confidence:** MEDIUM

### Gaps to Address

- **Minecraft authentication.** Microsoft documents public-client authorization code with PKCE. Minecraft's review form returned HTTP 200 and the support index listed an application review. The research could not verify the help article body, submission eligibility, required fields, turnaround, or Nebula's protected access. Close with provider confirmation and manual authenticated evidence. [Microsoft desktop configuration](https://learn.microsoft.com/en-us/entra/identity-platform/scenario-desktop-app-configuration) · [Minecraft Java support](https://help.minecraft.net/hc/en-us/sections/44460521913485) · [Review form](https://aka.ms/mce-reviewappid)
- **Minecraft and loader integration.** Five public Mojang version metadata samples matched the manifest's SHA-1 and covered legacy and structured arguments and different Java requirements. Loader sources establish profile APIs or installer availability. Actual launch behavior, the supported version matrix, and installer execution and outputs require Linux verification. [Mojang manifest](https://piston-meta.mojang.com/mc/game/version_manifest_v2.json) · [Fabric Meta](https://meta.fabricmc.net/) · [Quilt Meta](https://meta.quiltmc.org/) · [NeoForge client guide](https://docs.neoforged.net/user/docs/client/) · [Forge downloads](https://files.minecraftforge.net/net/minecraftforge/forge/)
- **Modrinth, recovery, and packaging.** Public catalog reads confirmed sample availability and returned version hashes. Verify dependency resolution and pack round trips, export permissions, interrupted publication and restore, backup retention, launcher-exit behavior, and packaged desktop integration. [Modrinth API](https://docs.modrinth.com/api/) · [Modrinth pack format](https://support.modrinth.com/en/articles/8802351-modrinth-modpack-format-mrpack) · [Minecraft Usage Guidelines](https://www.minecraft.net/en-us/usage-guidelines)

## Sources

- [Stack research](STACK.md), [feature research](FEATURES.md), [architecture research](ARCHITECTURE.md), and [integration and release risks](PITFALLS.md) contain the detailed evidence and provider references.
- Platform references cover Linux build requirements, WebView behavior, secret storage, and durable state. [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) · [Tauri WebView versions](https://v2.tauri.app/reference/webview-versions/) · [Tauri IPC](https://v2.tauri.app/concept/inter-process-communication/) · [Secret Service specification](https://specifications.freedesktop.org/secret-service/latest/) · [SQLite atomic commit](https://www.sqlite.org/atomiccommit.html) · [Linux rename](https://man7.org/linux/man-pages/man2/rename.2.html) · [Linux fsync](https://man7.org/linux/man-pages/man2/fsync.2.html)

---
*Research completed on 2026-11-12. Ready for roadmap drafting.*
