# Phase 1 · Linux Bootstrap and Minecraft Access Feasibility

**Gathered:** 2026-11-12

**Status:** Ready for planning

<domain>
## Phase Boundary

Run the development desktop shell on CachyOS x86_64, verify typed status commands and events across the Tauri boundary, and document the route to protected Minecraft Game Service access for Nebula Launcher.

</domain>

<decisions>
## Implementation Decisions

### Microsoft app registration

- **D-01** · Start with a new Microsoft Entra application registration. The project owner will create and own the registration and submit the Minecraft API access request from their account.
- **D-02** · Phase 1 will prepare an owner-facing checklist for registration and API review. Verify the current provider instructions, required inputs, and access prerequisites before documenting them. Record the status as awaiting owner registration until there is evidence of a registration. Track registration, review submission, approval, and live access separately.
- **D-03** · The Linux bootstrap and fixture-based checks can proceed while registration and review are pending. Keep live authentication acceptance in Phase 6 and authenticated launch acceptance in Phase 7.

### Implementation discretion

- Choose the initial status screen and exact command and event names during planning. Retain the approved requirements for scoped capabilities, sanitized errors, and testing the Rust core independently of Tauri.

</decisions>

<canonical_refs>
## Canonical References

- `.planning/ROADMAP.md` · Phase 1 scope, dependencies, and verification.
- `.planning/REQUIREMENTS.md` · BASE-01 through BASE-03.
- `.planning/PROJECT.md` · Project constraints and approved architectural decisions.
- `.planning/research/ARCHITECTURE.md` · Desktop boundary and Linux credential handling.
- `.planning/research/STACK.md` · Candidate toolchains and CachyOS prerequisites.
- `.planning/research/PITFALLS.md` · Microsoft Entra and Minecraft review sources, evidence, and open questions.

</canonical_refs>

<code_context>
## Existing Code Insights

The repository currently contains project documentation and planning configuration. Phase 1 establishes the first application and Rust core.

</code_context>

<specifics>
## Specific Ideas

Write the registration checklist so the owner can complete the steps from their own account. Keep credentials and tokens out of repository artifacts and diagnostics.

</specifics>

<deferred>
## Deferred Ideas

Live account integration and authenticated game launch are covered by Phases 6 and 7.

</deferred>
