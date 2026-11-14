# Phase 2 · Dark Interface and Navigation

**Gathered:** 2026-11-14

**Status:** Ready for planning

<domain>
## Phase Boundary

Build the dark desktop workspace and accessible navigation for UI-01 and UI-02. All six destinations should be reachable. The Accounts destination should preserve the real Minecraft access status and retry flow from Phase 1.
</domain>

<decisions>
## Implementation Decisions

- **D-01:** Use the approved midnight blue and violet palette with restrained accents. Start on Dashboard and keep Dashboard, Instances, Downloads, Accounts, Settings, and Diagnostics in a consistent sidebar. Use shared visual tokens and components.
- **D-02:** Keep each destination meaningful at this stage. Show the existing status and retry controls under Accounts. For destinations awaiting their feature phases, use a brief honest empty state and avoid sample data or controls with no action.
- **D-03:** Keep all six destinations discoverable at the existing desktop window minimum of 340×370 logical pixels. Use a compact navigation layout when width is limited. Keep keyboard focus visible, announce the active destination, and respect reduced motion. Validate the minimum, the initial 760×530 window, and a larger desktop size.
</decisions>

<canonical_refs>
## Canonical References

- `.planning/ROADMAP.md` · Phase 2 scope and verification.
- `.planning/REQUIREMENTS.md` · UI-01 and UI-02.
- `.planning/PROJECT.md` · Approved visual direction and desktop target.
- `src/App.tsx` and `src/status.ts` · Existing status, event subscription, and retry behavior.
- `src/styles.css` · Existing dark color and focus styles.
- `src-tauri/tauri.conf.json` · Initial and minimum window dimensions.
- `.planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-VERIFICATION.md` · Baseline desktop and keyboard verification.
</canonical_refs>

<code_context>
## Existing Code Insights

React currently renders one Minecraft access view. Its credential-free fixture covers a failed read followed by a retry. Preserve both that behavior and the Tauri command and event contract while integrating the view into the new workspace.
</code_context>

<deferred>
## Deferred Ideas

Full instance management, download progress, and Microsoft sign-in follow in their roadmap phases.
</deferred>
