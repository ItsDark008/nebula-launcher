# Nebula Launcher

## What This Is

Nebula Launcher is a desktop launcher for Minecraft Java Edition on Linux. Players can install and run the game in isolated instances, use supported modloaders, and manage Modrinth content inside the launcher.

## Core Value

A Linux player with an eligible Minecraft account can install and launch Minecraft Java Edition in an isolated instance and keep their settings across restarts.

## Requirements

### Validated

- Phase 1 verified the CachyOS development window, standalone Rust status core, typed Tauri command and event, and keyboard retry. The owner registration and Minecraft review checklist is available. [Phase 1 verification](phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-VERIFICATION.md)
- Phase 2 delivered six dark-interface destinations and verified keyboard navigation and status retry. Native window measurements and the OS reduced-motion toggle retain the limits documented in the verification report. [Phase 2 verification](phases/02-dark-interface-and-navigation/02-VERIFICATION.md)
- The owner completed a real Microsoft sign-in, profile retrieval, refresh and removal on Linux. Two-real-account switching and the rest of Phase 6 verification remain open. [Authentication evidence](../docs/microsoft-auth-verification.md)

### Active

- [ ] A · Vanilla MVP with persistent instances, official version metadata, verified downloads, local Java, Microsoft sign-in, real game launch, and diagnostics.
- [ ] B · Installation and real launch with Fabric, Quilt, NeoForge, and Forge through dedicated adapters.
- [ ] C · Native Modrinth catalog, mod and modpack management, import and export, resource packs, shaders, updates, and recovery.
- [ ] D · Hardening, performance, CI, documentation, and Linux packaging.

### Out of Scope

- Windows support in the initial Linux release.
- Automatic Java provisioning in the initial release. Local runtimes cover the MVP.
- A light theme as a release requirement. The design system prioritizes the dark theme.
- Cracked mode and license or authentication bypasses.

## Context

- Development target is CachyOS. Linux x86_64 is the proposed first validation target.
- The desktop UI includes a dashboard and instance library, instance creation and details, downloads, accounts, settings, logs, and diagnostics. Modrinth adds views for mods, modpacks, resource packs, and shaders.
- The visual identity uses midnight blue, violet, and cosmic accents. The UI needs a sidebar, reusable components, keyboard navigation, visible focus, restrained motion, and support for resized windows.
- The Vanilla milestone requires a real Linux run with an eligible Minecraft account, from first sign-in through reopening the launcher. Each modloader requires a real install and launch.

## Constraints

- GSD Core manages discovery, planning, and verification. The project owner approves the roadmap and architecture before implementation.
- Deliver changes through GitHub pull requests using the connected GitHub account. Commits belong on the PR branch and explain what changed and why.
- Rust owns critical behavior and remains testable independently of Tauri. React communicates through typed commands and events.
- Build integrity and security controls into downloads, token handling, archive import, and process launch. Automated tests use fixtures and authenticated smoke tests use a separate manual workflow.
- Installed instances must launch during a Modrinth outage when a valid Minecraft session is available. Keep credentials and tokens out of the WebView, SQLite, and logs.
- Respect distribution restrictions and licenses. Public packages and releases require explicit owner authorization.

## Key Decisions

| Decision | Rationale | Status |
|---|---|---|
| Tauri 2, Rust, Tokio, SQLite, React, TypeScript, Vite, Tailwind, and shadcn/ui | Preferred stack. Resolve and verify compatible versions before creating lockfiles. | Proposed |
| Linux x86_64 and local Java for the first smoke test | Defines a reproducible initial test target on CachyOS. | Approved |
| Establish the authentication access path at the start | Minecraft Game Service access is an external dependency of the MVP. | Approved |
| Local desktop application with Rust services and provider adapters | Keeps UI, filesystem, network, and provider integration independently testable. | Approved |
| Vanilla MVP and native Modrinth in the complete Linux release | Keeps the first milestone focused and preserves Modrinth as a release requirement. | Product brief |

## Evolution

- At each phase transition, update the project context, verified requirements, decisions, and reasons for scope changes.
- At each milestone boundary, review the core value, constraints, verification evidence, and remaining scope.

---
*Last updated: 2026-11-14 after Phase 2 verification*
