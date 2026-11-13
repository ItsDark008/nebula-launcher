---
phase: 01-linux-bootstrap-and-minecraft-access-feasibility
plan: "02"
subsystem: auth
tags: [microsoft-entra, minecraft-api, secret-service, cachyos]
requires: []
provides:
  - Owner checklist for Entra registration and Minecraft API review
  - Initial access status ledger and Secret Service availability evidence
affects: [phase-06, phase-07]
actuals:
  tokens: 1955
  tasks: 2
  commits: 0
  plan_head_before: a3facbd716e92c795c72f79dbe35524dbac9fc07
  plan_head_after: a3facbd716e92c795c72f79dbe35524dbac9fc07
tech-stack:
  added: []
  patterns: [owner-managed public-client registration, independent access evidence milestones, read-only Secret Service probe]
key-files:
  created: [docs/minecraft-access.md]
  modified: []
key-decisions:
  - Treat Entra personal-account and loopback settings as preparation pending registration and provider confirmation.
  - Keep live authentication in Phase 6 and authenticated Vanilla launch in Phase 7.
patterns-established:
  - Record dates and outcome categories for access milestones and keyring availability.
requirements-completed: [BASE-03]
coverage:
  - id: D1
    description: Owner registration checklist, review inputs, and four independent access milestones
    requirement: BASE-03
    verification:
      - kind: other
        ref: 01-02-PLAN.md Task 1 and Task 2 Node document checks
        status: pass
      - kind: manual_procedural
        ref: Public Microsoft Learn and Microsoft Forms source verification on 2026-11-13
        status: pass
    human_judgment: false
  - id: D2
    description: Read-only Secret Service availability evidence from the target desktop session
    requirement: BASE-03
    verification:
      - kind: manual_procedural
        ref: CachyOS niri Wayland session bus ownership and activation checks on 2026-11-13
        status: pass
    human_judgment: false
duration: 4min
completed: 2026-11-13
status: complete
---

# Phase 1 Plan 2 · Minecraft access feasibility summary

The owner runbook records the public AppID review inputs, the evidence needed to advance each access milestone, and a missing Secret Service in the current CachyOS session.

## Verification

- Both plan-defined Node document checks passed from the repository root.
- Microsoft Learn confirmed the Entra registration options, native desktop redirect configuration, and authorization code with PKCE. Microsoft Forms returned the public AppID Review page and its form definition with HTTP 200. The definition confirmed the new approval branch, required inputs, and acknowledgment. The Minecraft Help article body was unavailable. The Minecraft EULA and Usage Guidelines were reachable directly.
- The CachyOS niri/Wayland session had a reachable compositor and user D-Bus. `gdbus` and `busctl` both found no owner for `org.freedesktop.secrets`. The bus did not advertise the name for activation. The default collection alias and lock state were unobservable. The earlier unlocked collection in KDE/Wayland remains a separate observation.
- The runbook and summary were inspected for private identifiers, credentials, collection paths, raw probe output, and unsupported claims of registration, submission, approval, or live access.

## Files created

- `docs/minecraft-access.md` provides the owner checklist, access status, and dated probe evidence.

## Task commits

Both tasks were verified. The phase orchestrator handles the PR branch commit.

## Open owner prerequisites

The owner still needs to verify Entra registration eligibility, create the registration, confirm its saved audience and redirect, choose a qualifying public project URL, and submit the review request. The provider's approval, individual turnaround, exact protected permissions and scopes, and successful access with the Nebula-owned client and an eligible account remain open. Desktop bootstrap and credential-free fixtures can proceed. Phase 6 verifies the live account and keyring path, and Phase 7 verifies an authenticated Vanilla install and launch.

## Deviations from plan

The review article and session keyring limitations were recorded using the plan's fallback evidence. Both tasks were completed within the assigned documentation scope.

## Self-Check: PASSED
