---
phase: "02"
slug: dark-interface-and-navigation
status: approved
shadcn_initialized: true
preset: b2VA7Moyo
created: "2026-11-14"
---

# Phase 2 — UI Design Contract

## Design System

| Property | Value |
| --- | --- |
| Tool | shadcn/ui |
| Preset | b2VA7Moyo (CLI reports the equivalent installed configuration as b2VA7Movi) |
| Component library | Base UI |
| Icon library | Remix Icon |
| Font | JetBrains Mono Variable |

The application uses the dark theme by default. Preserve the midnight blue and violet direction from 02-CONTEXT; use the preset's fuchsia sparingly as an accent. The registered font ships locally.

## Component Inventory

Enumerated by `npx shadcn@4.23.0 info` — 0 components — shadcn@4.23.0 — 2026-11-14. Cross-check: no files exist under `src/components/ui`.

This is a non-exhaustive inventory, not an allowlist. Add official components as needed after checking their source and compatibility.

## Spacing Scale

| Token | Value | Usage |
| --- | --- | --- |
| xs | 4px | Inline gaps |
| sm | 8px | Compact gaps |
| md | 16px | Default content spacing |
| lg | 24px | Section padding |
| xl | 32px | Major layout gaps |
| 2xl | 48px | Section separation |
| 3xl | 64px | Spacious page margins |

Interactive controls have a minimum 44px target. At 340px width compact navigation and padding must remain usable without clipping.

## Typography

| Role | Size | Weight | Line height |
| --- | --- | --- | --- |
| Label | 14px | 400 | 1.5 |
| Body | 16px | 400 | 1.5 |
| Heading | 20px | 600 | 1.2 |
| Display | 28px | 600 | 1.2 |

## Color

| Role | Value | Usage |
| --- | --- | --- |
| Dominant | #151321 | Main background |
| Secondary | #1c192a / #211d32 | Sidebar and cards |
| Accent | preset fuchsia `--primary` | Active navigation indicator, primary controls and focus treatment |
| Error | #ffbdc7 | Error text and status |

Use a restrained dominant/secondary/accent balance. Pair color with text or shape for status and selection. Preserve visible high-contrast keyboard focus.

## Copywriting Contract

| Element | Copy |
| --- | --- |
| Dashboard | “Dashboard”; explain that the launcher is being prepared without suggesting an install already exists. |
| Instances | “No instances yet”; explain that instance management will be available in a later phase. |
| Downloads | “No downloads yet”; explain that downloads appear here when available. |
| Accounts | “Minecraft access”; retain actual status, loading, safe error, retry and refresh behavior. |
| Settings | “Settings are coming later”; no inactive controls. |
| Diagnostics | “Diagnostics are coming later”; no invented diagnostic output. |
| Error | “Unable to refresh launcher status.” with a “Retry” action. |
| Destructive actions | None in this phase. |

## Interaction Contract

Show Dashboard first. Provide Dashboard, Instances, Downloads, Accounts, Settings and Diagnostics in a labelled navigation region. The current destination is announced with `aria-current`. Each view has a unique heading. Keep the status subscription and authoritative command response semantics intact across navigation. The Accounts retry stays keyboard-operable and retains focus when its label changes. At 340×370 use compact navigation without hiding destinations; verify also at 760×530 and a larger desktop size. Respect reduced-motion preferences.

## Registry Safety

| Registry | Blocks used | Safety gate |
| --- | --- | --- |
| shadcn official | None installed | Not applicable |
| Third-party | None | Not applicable |

## UI Considerations

The owner confirmed the six navigation destinations, Accounts loading/error/retry, and empty views as the intended surfaces. A retrospective probe classified navigation, account status controls, and empty-state content. Its considerations are resolved from the implemented behavior and the owner's tests:

| Category | Element | Resolution |
| --- | --- | --- |
| Loading, error | Navigation | The six fixed local destinations do not load data; destination-specific failures remain inside the destination. |
| Overflow, long text | Navigation | Fixed labels wrap into multiple rows at narrow widths; all routes were checked at 340px without horizontal overflow. |
| Loading, error | Accounts | Status announces refresh, keeps the last snapshot and exposes Retry on failure. |
| Overflow, long text | Accounts and empty states | Text wraps and vertical document scrolling keeps additional content reachable. |

The browser checks and owner confirmation do not independently measure the native compositor's final size or toggle the OS reduced-motion preference.

## Checker Sign-Off

- [x] Copywriting — inline review; the existing one-word Retry label is a non-blocking flag.
- [x] Visuals — inline review; an explicit focal-point declaration is a non-blocking flag.
- [x] Color and contrast — inline review; the 60/30/10 distribution is not numerically locked, a non-blocking flag.
- [x] Typography — four sizes and two weights.
- [x] Spacing — declared scale uses the standard multiples of four.
- [x] Registry safety — no third-party registry or blocks declared.
- [x] Inventory provenance — installed package version, count, date and enumeration command recorded.

Approval: retrospective inline review, 2026-11-14; three non-blocking flags above. This was not a separate subagent review.
