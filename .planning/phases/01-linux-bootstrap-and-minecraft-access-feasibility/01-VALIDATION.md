---
phase: "01"
slug: linux-bootstrap-and-minecraft-access-feasibility
status: draft
nyquist_compliant: false
wave_0_complete: false
created: "2026-11-12"
---

# Phase 1 Validation Strategy

Run all commands from the repository root. Confirm actual feedback time during execution.

| Check | Command |
| --- | --- |
| Rust core | `cargo test --locked -p nebula-core` |
| Desktop adapter | `cargo test --locked -p nebula-launcher` |
| Frontend | `npm run test -- --run` and `npm run build` |
| Workspace | `cargo test --locked --workspace` and `npm run test -- --run` and `npm run build` |

## Sampling Rate

- After each task, run its applicable checks as soon as the task creates the required package or file.
- After the desktop plan, run the workspace checks and desktop smoke test.
- Before phase verification, record the output of the full checks and any manual evidence or blockers.

## Per-Task Verification Map

| Task | Requirements | Threats | Verification | Status |
| --- | --- | --- | --- | --- |
| 01-01-01 | BASE-01, BASE-03 | T-01-07 | Core tests and clean frontend build | Pending |
| 01-03-01 | BASE-01, BASE-02, BASE-03 | T-01-01, T-01-03, T-01-08 | Adapter tests and real WebView command and event smoke | Pending |
| 01-03-02 | BASE-02 | T-01-01, T-01-02, T-01-03 | Frontend and desktop adapter tests for error handling and the capability allowlist | Pending |
| 01-03-03 | BASE-01, BASE-02 | T-01-02 | Full workspace checks and CachyOS desktop smoke | Pending |
| 01-02-01 | BASE-03 | T-01-04, T-01-06 | Runbook structure check and manual review of provider sources | Pending |
| 01-02-02 | BASE-03 | T-01-05, T-01-06 | Status ledger check and manual Secret Service probe | Pending |

## Test Prerequisites

The foundation plan creates the Rust workspace, frontend build, and initial test commands. The desktop plan adds the status tests and fixtures before the full workspace checks. The runbook checks require Node.js and the documentation created by plan 01-02.

## Manual-Only Verifications

| Requirement | Evidence |
| --- | --- |
| BASE-01 | Run `npm run tauri dev` in a real CachyOS x86_64 desktop session. Record Rust, Node, npm, desktop, and WebKitGTK versions. Confirm the window opens and displays the Rust-provided status. |
| BASE-02 | Refresh from the WebView and confirm the typed response and a corresponding Rust event. Exercise keyboard navigation and confirm the retry affordance in a credential-free failure fixture. |
| BASE-03 | Review the registration runbook against the official provider sources and record the confirmed fields and open prerequisites. Probe the default Secret Service collection using non-secret service and lock status. |

## Validation Sign-Off

Pending execution evidence.
