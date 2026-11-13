---
phase: "01"
slug: linux-bootstrap-and-minecraft-access-feasibility
status: verified
threats_open: 0
asvs_level: 1
created: "2026-11-13"
---

# Phase 1 Security

| Threat | Disposition | Evidence |
| --- | --- | --- |
| T-01-01 | Mitigated | Application manifest registers one read-only command. The local main-window capability and desktop adapter tests verify the command and event subscription grants. |
| T-01-02 | Mitigated | Public error serialization and frontend tests verify fixed text without internal diagnostics. |
| T-01-03 | Mitigated | Rust targets the main WebView. The capability excludes event emission. The frontend preserves the authoritative command reply. |
| T-01-04 | Mitigated | Owner instructions use verified first-party provider sources and the project-owned registration. |
| T-01-05 | Mitigated | The runbook tracks registration, review submission, approval, and protected access independently. |
| T-01-06 | Mitigated | The runbook contains only non-secret status evidence. The desktop probe inspected service availability without accessing credential values. |
| T-01-07 | Mitigated | The audited dependency identifiers resolve from crates.io and registry.npmjs.org. The lockfiles were inspected and the locked builds passed. |
| T-01-08 | Mitigated | Cargo locked the audited Tauri packages and the desktop check and tests passed. |

The local main-window subscription operations also expose framework lifecycle events. The main window disables native file drag and drop and cancels default WebView drop navigation. The review disposition records the remaining warning.
