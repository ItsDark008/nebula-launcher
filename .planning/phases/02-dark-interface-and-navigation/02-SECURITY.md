---
phase: "02"
slug: dark-interface-and-navigation
status: verified
threats_open: 0
asvs_level: 1
created: "2026-11-14"
---

# Phase 2 — Security

The plan did not include a threat register; this is a retrospective review of the merged UI, its IPC boundary and the dependency audit. `threats_open` counts unresolved high and critical threats; the lower-severity dependency item below remains open and is not an accepted risk.

| Threat | Severity | Evidence and disposition | Status |
| --- | --- | --- | --- |
| Untrusted page reaches privileged IPC | high | Production CSP restricts sources to self; local-only Tauri capability restricts commands to the main window; drop navigation is prevented. | closed |
| Provider data or errors expose secrets in UI | high | Account IPC returns identity and safe messages; tests cover redaction and status errors. Phase 6 retains its own separate security scope. | closed for Phase 2 UI boundary |
| shadcn build tooling transitively includes vulnerable packages | medium | `npm audit` on 2026-11-14 reports 4 moderate and 2 low findings through `shadcn`, including `minimatch` and `deepmerge`. The app imports shadcn CSS; the affected CLI packages are not imported by application source. No compatible upstream fix was offered by npm. | open, below high blocking threshold |

No risks were accepted on the owner's behalf. Track upstream fixes for the build tooling; do not execute untrusted registry inputs. This review does not replace a separate Phase 6 authentication security audit.
