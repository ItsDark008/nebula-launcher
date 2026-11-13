---
phase: 01-linux-bootstrap-and-minecraft-access-feasibility
reviewed: 2026-11-13T11:44:55Z
depth: standard
files_reviewed: 23
files_reviewed_list:
  - .gitignore
  - Cargo.toml
  - README.md
  - crates/nebula-core/Cargo.toml
  - crates/nebula-core/src/lib.rs
  - docs/minecraft-access.md
  - index.html
  - package.json
  - rust-toolchain.toml
  - src-tauri/Cargo.toml
  - src-tauri/build.rs
  - src-tauri/capabilities/main.json
  - src-tauri/icons/icon.png
  - src-tauri/src/main.rs
  - src-tauri/tauri.conf.json
  - src/App.test.tsx
  - src/App.tsx
  - src/fixtures/status-retry.html
  - src/fixtures/status-retry.tsx
  - src/main.tsx
  - src/status.ts
  - src/styles.css
  - tsconfig.json
  - vite.config.ts
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
  necessity: 0
status: issues_found
---

# Phase 1 Code Review Report

## Summary

Reviewed the Rust workspace, Tauri boundary, frontend, development configuration, and Minecraft access runbook. The live Vite response at `127.0.0.1:1420` includes the development Content Security Policy from `vite.config.ts:10`. The main window disables Tauri's native drag-and-drop handler in `src-tauri/tauri.conf.json:14`, and `src/main.tsx:4` cancels the WebView's default drop behavior. The status client now models asynchronous disposal and handles rejections during both cleanup paths. These changes resolved the development policy, native drag path, and unhandled cleanup findings.

## Findings

### WR-01 — The event permission exposes framework lifecycle events

**Severity** WARNING
**File** `src-tauri/capabilities/main.json:5`

Tauri's generic event permission accepts subscriptions to framework events including resize, move, focus, and close requests. The pinned runtime also suppresses the normal close behavior when a matching JavaScript close-request listener is present. The shipped status client subscribes to `launcher-status-updated`. The Phase 1 mitigation removes native file path events while the framework subscription permission remains available to the local renderer.

**Fix** Enforce the event boundary with an app-owned typed status channel or status-specific subscription commands. Remove the generic event permissions and verify at the runtime ACL that the approved status subscription works and generic subscriptions are rejected.
