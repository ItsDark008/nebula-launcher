# Roadmap: Nebula Launcher

## Overview

**Status:** Approved by the project owner on 2026-11-12. The first complete Linux release spans all four milestones. The first validation target is CachyOS on Linux x86_64. Public packages, releases, and distribution require explicit owner authorization. [Project brief](PROJECT.md) · [Architecture](research/ARCHITECTURE.md)

Confirm the Minecraft Game Service access path during Phase 1. Authorized access for a Nebula-owned Microsoft public client and an eligible account are required for live authentication in Phase 6 and completion of Phase 7. While access is pending, independent desktop, metadata, installation, fixture, and catalog work can continue wherever its dependencies are met. Keep authenticated acceptance pending until each required live check succeeds. [Integration risks](research/PITFALLS.md)

## Phases

### Milestone A · Vanilla MVP

- [x] **Phase 1: Linux Bootstrap and Minecraft Access Feasibility** - Run the desktop shell and establish the protected API access path. (completed 2026-11-13)
- [ ] **Phase 2: Dark Interface and Navigation** - Navigate the desktop workspace with a mouse or keyboard.
- [ ] **Phase 3: Persistent Instances** - Create, edit, duplicate, and safely delete isolated instances.
- [ ] **Phase 4: Minecraft Manifest and Verified Downloads** - Browse official versions and follow verified transfers.
- [ ] **Phase 5: Vanilla Installation and Local Java** - Install, update, and repair Vanilla with compatible local Java.
- [ ] **Phase 6: Microsoft Accounts** - Sign in, refresh, switch, and remove eligible accounts securely.
- [ ] **Phase 7: Authenticated Vanilla Launch** - Play a real Vanilla instance on Linux and inspect launch diagnostics.

### Milestone B · Modloaders

- [ ] **Phase 8: Fabric and Quilt** - Install and launch a supported Fabric instance and a supported Quilt instance.
- [ ] **Phase 9: NeoForge and Forge** - Run the dedicated installer flows and launch a supported instance for each loader.

### Milestone C · Modrinth Content and Recovery

- [ ] **Phase 10: Modrinth Catalog** - Browse four content types with filters and resilient catalog access.
- [ ] **Phase 11: Mod Management** - Install and manage compatible mods and dependencies per instance.
- [ ] **Phase 12: Modpack Install, Import, and Export** - Install catalog packs, import local packs, and create compliant exports.
- [ ] **Phase 13: Resource Packs and Shaders** - Manage resource packs and shaders with per-instance compatibility checks.
- [ ] **Phase 14: Updates, Backup, and Recovery** - Preview pack changes and restore an instance after interrupted updates.

### Milestone D · Linux Release

- [ ] **Phase 15: Quality and Performance** - Reproduce regression, safety, smoke, and performance results across supported flows.
- [ ] **Phase 16: Linux Packaging and Release Preparation** - Install a Linux release candidate and prepare reproducible release jobs.

## Phase Details

### Phase 1: Linux Bootstrap and Minecraft Access Feasibility

**Goal**: As a project owner, I want to run the desktop shell on CachyOS and exercise its typed connection to Rust, so that I can see the current Minecraft access status.
**Mode:** mvp
**Depends on**: Roadmap and architecture approval, recorded on 2026-11-12.
**Requirements**: BASE-01, BASE-02, BASE-03
**Success Criteria** (what must be TRUE):

1. The development shell opens on CachyOS x86_64, displays a Rust-provided status, and the Rust core test suite runs without Tauri.
2. A desktop status request returns a typed result, emits a status event, and shows a sanitized error when the request fails. The WebView can reach only the required commands and events.
3. The owner can see who owns the proposed registration, the verified public-client and review steps, which prerequisites remain open, and whether protected access is blocked.

**Plans**: 3/3 plans complete
**UI hint**: yes

Plans:
**Wave 1**
- [x] 01-01: Pin development tools and build the standalone status core.
- [x] 01-02: Verify the Minecraft API access path and prepare the owner checklist.

**Wave 2** *(blocked on Wave 1 completion)*
- [x] 01-03: Connect the real desktop window to Rust and verify status, events, and recovery.

**Implementation tasks**

- Resolve compatible Rust, Node, Tauri, WebKitGTK, and frontend versions on CachyOS before pinning the toolchain and lockfiles. Create the development shell and a standalone Rust core.
- Define typed command, event, and error contracts with scoped capabilities. Establish credential-free fixtures, redaction, and authoritative status snapshots for long-running work.
- Confirm the registration owner, personal Microsoft account settings, redirect requirements, protected API review path, and permissions. Record which provider details are confirmed and which remain unavailable. Verify the target desktop's Secret Service availability for Phase 6.

**Verification and evidence**

- Automated checks cover standalone core builds and tests, IPC serialization, capability boundaries, and redacted failures.
- On CachyOS, record the exact runtime and build versions, open the shell, exercise the status command, and record the Secret Service probe. Record the sources and outcome of the access review without credentials.

### Phase 2: Dark Interface and Navigation

**Goal**: As a player, I want to move through the dark desktop workspace at supported window sizes, so that I can find and operate the launcher's primary controls.
**Mode:** mvp
**Depends on**: Phase 1.
**Requirements**: UI-01, UI-02
**Success Criteria** (what must be TRUE):

1. The sidebar reaches the dashboard, instances, downloads, accounts, settings, and diagnostics. Each area uses the same visual and interaction patterns.
2. A player can reach and operate primary controls by keyboard, identify visible focus, and follow a logical focus order.
3. Navigation and primary actions remain usable across the supported window sizes. The desktop's reduced-motion preference limits motion.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Define the midnight-blue, violet, and cosmic-accent theme, reusable components, navigation, and consistent loading, empty, and error states.
- Add accessible names, focus styles and management, responsive layout, and reduced-motion behavior. Establish the supported window sizes on the target desktop.

**Verification and evidence**

- Run component and accessibility checks for routes, semantics, keyboard interactions, and motion preferences.
- Walk through all destinations in the Linux WebView at the agreed window sizes, using keyboard navigation and reduced motion. Capture the layout and focus behavior.

### Phase 3: Persistent Instances

**Goal**: Players can manage isolated instances and reopen the launcher with their settings and instance status intact.
**Mode:** mvp
**Depends on**: Phases 1 and 2.
**Requirements**: INST-01, INST-02, INST-03
**Success Criteria** (what must be TRUE):

1. A player can create and edit an instance with a name and custom icon. Each instance has its own game directory and its changes remain visible after restart.
2. A duplicate can be edited independently. A deletion confirmation identifies the selected instance and the affected user data. Cancel preserves that data, and confirming deletion leaves other instances intact.
3. Reopening shows the saved settings, installation status, and last-launch value for each instance, including the empty state for an instance that has never launched.
4. If a create, duplicate, or delete operation is interrupted, reopening reconciles the affected instance and leaves unrelated instances usable.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Create the SQLite instance repository and migration, immutable instance IDs, managed game directories, icon handling, and instance create, edit, duplicate, and delete controls.
- Before the first filesystem mutation, enforce managed-path containment and per-instance serialization. Journal filesystem changes, stage on the destination filesystem, reconcile interrupted operations on restart, and preserve recoverable copies for destructive changes.
- Expose install and launch fields in instance details. Phases 5 and 7 populate these fields through the real installation and launch paths.

**Verification and evidence**

- Run SQLite and temporary-directory tests for persistence, duplication isolation, confirmed deletion, path containment, conflicts, and restart reconciliation at mutation boundaries.
- Create and edit an instance, duplicate it, cancel and confirm a deletion, then reopen the desktop app and inspect the remaining instance data and status.

### Phase 4: Minecraft Manifest and Verified Downloads

**Goal**: Players can choose an official Minecraft version and follow downloads that are checked before use.
**Mode:** mvp
**Depends on**: Phases 1 through 3.
**Requirements**: GAME-01, DOWN-01, DOWN-02, DOWN-03
**Success Criteria** (what must be TRUE):

1. A player can browse release and snapshot versions from Mojang, reuse cached metadata when available, and see an unavailable state if the network fails without a cache.
2. The downloads view shows queued and active transfers, bytes, speed, progress, cancellation, and actionable failures. Active transfers stay within the configured limit.
3. Eligible failures retry with bounded backoff and timeouts. An interrupted transfer resumes when the provider supports a validated byte range. A changed resource restarts safely.
4. A size or hash mismatch fails before publication. Cancellation or interruption leaves previously verified artifacts usable and allows the transfer to be recovered or retried.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Add the official version manifest adapter and cache, release and snapshot browsing, and metadata integrity checks where the manifest supplies a hash.
- Implement bounded transfers with streamed verification, cancellation, retry and timeout policy, progress snapshots and events, and validator-aware range requests.
- Stage on the destination filesystem and publish only verified artifacts. Reuse the operation journal for interruption recovery and keep partial files out of the usable cache.

**Verification and evidence**

- Use local HTTP fixtures for timeouts, stale validators, validated ranges, changed responses, corrupt artifacts, cancellation, and restart recovery. Assert the concurrency bound and safe publication.
- Browse the public manifest, observe a transfer, cancel or retry it, and confirm the cache and unavailable states in the Linux desktop app.

### Phase 5: Vanilla Installation and Local Java

**Goal**: Players can install, update, or repair Vanilla in an instance and save a compatible local Java configuration.
**Mode:** mvp
**Depends on**: Phases 3 and 4.
**Requirements**: GAME-02, JAVA-01, JAVA-02
**Success Criteria** (what must be TRUE):

1. Installing a selected Vanilla version leaves a complete instance with its verified client, compatible libraries, natives, asset index, and asset objects. Repair restores a missing or corrupt managed file.
2. A successful update selects the new verified installation. A failed or interrupted update preserves the prior usable installation and user files.
3. A player can see detected local Java versions and architectures, accept a compatible runtime automatically or choose an explicit path, and correct a missing or incompatible selection using the displayed guidance.
4. RAM and JVM arguments persist across restart. Invalid or incompatible settings are reported before launch.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Resolve platform rules, library and native variants, asset indexes and objects, and game-version Java requirements from official metadata. Define the supported metadata and Java combinations.
- Implement install, update, and repair using verified staging and publication, installed-state snapshots, and rollback to the previous installation.
- Discover and validate local Java executables and store per-instance runtime, memory, and JVM settings. Surface compatibility and validation errors in the instance controls.

**Verification and evidence**

- Run metadata and filesystem fixtures for the supported legacy and structured metadata shapes, representative releases and snapshots, Java major and architecture mismatches, repair, and interrupted updates.
- On CachyOS, install a representative official release with local Java, inspect its installed state, repair a managed artifact, and reopen the app to confirm that the Java configuration persists.

### Phase 6: Microsoft Accounts

**Goal**: Eligible players can sign in through their browser and manage persistent Minecraft sessions from the desktop.
**Mode:** mvp
**Depends on**: Phases 1 and 2. Live authentication requires protected Minecraft Game Service access for the authorized Nebula-owned public client and an eligible account.
**Requirements**: AUTH-01, AUTH-02, AUTH-03, AUTH-04
**Success Criteria** (what must be TRUE):

1. A player can start sign-in in the system browser using the supported public-client flow with PKCE where applicable. Redirect-based sign-in accepts only the verified callback and rejects invalid, expired, or repeated responses.
2. With the authorized Nebula client, sign-in completes the Microsoft, Xbox, and Minecraft exchanges and shows the eligible Minecraft profile. Eligibility or provider failures show the failed step with safe guidance.
3. After restart, the launcher refreshes the session from the Linux keyring when available. Missing, locked, expired, and revoked sessions show distinct states without exposing credentials.
4. A player can switch between accounts and log out. Logout removes the selected account's local session and keyring secrets.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Implement the confirmed public-client flow, using callback correlation, expiration and single use for redirect-based sign-in. Keep provider exchanges and entitlement checks in Rust.
- Back the secret store with Linux Secret Service, serialize refresh and logout per account, and expose only safe identity and status fields through IPC. Integrate account switching, logout, and keyring recovery in the accounts view.
- While provider access is pending, exercise the flow with credential-free exchange and keyring fixtures. Track the live path as blocked until authorized access is available.

**Verification and evidence**

- Run fixtures for callback validation, exchange and entitlement failures, refresh and logout races, keyring states, and redaction in IPC, logs, and persisted state.
- With authorized access, record a manual Linux sign-in using an eligible account and sanitized results for Microsoft, Xbox/XSTS, Minecraft session, entitlement, and profile retrieval. Exercise refresh, account switching, logout, and actual keyring behavior.

### Phase 7: Authenticated Vanilla Launch

**Goal**: An eligible player can install and play a real Vanilla instance on Linux and pick up their configuration after reopening the launcher.
**Mode:** mvp
**Depends on**: Phases 5 and 6. Completion requires a live eligible Minecraft session through the authorized Nebula-owned client.
**Requirements**: PLAY-01, PLAY-02, PLAY-03
**Success Criteria** (what must be TRUE):

1. An eligible player can launch the real Vanilla client to its main menu on Linux using the selected local Java, metadata-derived arguments, classpath, native libraries, and isolated working directory.
2. The launcher shows running, exited, or failed status, sanitized game logs, and actionable exit or failure details.
3. Starting from a fresh launcher profile, a player can create an instance, sign in, install, launch, exit, and reopen to find the saved settings, installation state, and last launch.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Build the launch specification from verified local installation metadata, validated Java and instance settings, and the Rust-owned session. Invoke Java directly with separate arguments and the managed working directory.
- Supervise the process, bound and redact logs, record exit details and last launch, and reconcile process snapshots in the desktop. Define and test behavior when the launcher exits while the game is running.
- While access is pending, exercise launch construction and process supervision with metadata and fake process fixtures. Accept the milestone after a real authenticated run.

**Verification and evidence**

- Run fixtures for metadata argument expansion, classpath and native resolution, process errors, concurrent launch or mutation attempts, redaction, and exit handling.
- Record the authenticated workflow on CachyOS x86_64 from a fresh launcher profile through the actual game menu, process exit, and reopening. Include game and Java versions and sanitized diagnostic evidence.

### Phase 8: Fabric and Quilt

**Goal**: Players can choose a compatible Fabric or Quilt version, install it, and launch the resulting Linux instance.
**Mode:** mvp
**Depends on**: Phases 5 and 7.
**Requirements**: LOAD-01, LOAD-02
**Success Criteria** (what must be TRUE):

1. A player can discover a compatible Fabric version, install it through the Fabric adapter, and reach the real game menu on Linux.
2. A player can discover a compatible Quilt version, install it through the Quilt adapter, and reach the real game menu on Linux.
3. Unsupported game, loader, or Java combinations show a clear error and leave the instance's current installation usable.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Add dedicated Fabric and Quilt profile adapters and expose compatible versions in instance creation and installation.
- Merge each loader's profile with official game metadata and route verified libraries, installation state, and launch contributions through the existing pipeline.

**Verification and evidence**

- Run provider fixtures independently for each loader, including profile inheritance, incompatible combinations, and failed installation recovery.
- Record two separate authenticated Linux install and launch runs, one for Fabric and one for Quilt, with Minecraft, loader, and Java versions and the observed game menu.

### Phase 9: NeoForge and Forge

**Goal**: Players can use the dedicated NeoForge or Forge installer flow and launch the resulting Linux instance.
**Mode:** mvp
**Depends on**: Phases 5 and 7.
**Requirements**: LOAD-03, LOAD-04
**Success Criteria** (what must be TRUE):

1. A player can discover a compatible NeoForge version, complete its dedicated installer flow, and reach the real game menu on Linux.
2. A player can discover a compatible Forge version, complete its dedicated installer flow, and reach the real game menu on Linux.
3. The launcher reports unsupported combinations or installer failure without marking the incomplete installation ready or losing the prior usable state.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Verify the supported versions, installer inputs and outputs, and Java requirements independently for NeoForge and Forge. Implement separate adapters and compatible version selection.
- Stage each installer run, verify its inputs and expected output paths, invoke Java directly, and publish the resulting client profile and files through the recoverable installation pipeline.

**Verification and evidence**

- Run per-loader fixtures for installer output handling, Java compatibility, bad artifacts, process failures, and interrupted publication.
- Record two separate authenticated Linux install and launch runs, one for NeoForge and one for Forge, with Minecraft, loader, and Java versions, installer outcomes, and the observed game menu.

### Phase 10: Modrinth Catalog

**Goal**: Players can find Modrinth content in the launcher and keep playing installed instances when the catalog is unavailable.
**Mode:** mvp
**Depends on**: Phases 1 and 2 for catalog work. Phase 7 for outage-independent launch acceptance.
**Requirements**: MODR-01, MODR-02, MODR-03
**Success Criteria** (what must be TRUE):

1. A player can search mods, modpacks, resource packs, and shaders, apply the applicable Minecraft version, loader, and category filters, use supported sorting, and page through results.
2. A project detail view shows icons and available images, descriptions, authors, versions, and a link to the original Modrinth project.
3. The catalog identifies requests with the configured User-Agent, uses a bounded cache, and gives clear feedback for throttling, timeouts, and network errors.
4. An installed instance still reaches the game menu with a valid Minecraft session when Modrinth is unavailable.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Add a typed Modrinth client with a descriptive User-Agent, bounded cache, timeouts, rate-limit handling, and stable error mapping.
- Build search, filter, sort, pagination, and project detail views for the four content types. Open original links outside the privileged WebView and keep catalog access out of the launch path.

**Verification and evidence**

- Use API fixtures to validate request identity, query parameters, pagination, result details, cache bounds, rate limits, and timeout behavior.
- Browse representative projects in all four categories. With Modrinth requests unavailable and a valid session, launch an installed Linux instance to the game menu.

### Phase 11: Mod Management

**Goal**: Players can manage compatible mods and dependencies in each instance and resolve changes to locally modified files.
**Mode:** mvp
**Depends on**: Phases 3, 4, 8, 9, and 10.
**Requirements**: MODS-01, MODS-02
**Success Criteria** (what must be TRUE):

1. Installing a compatible Modrinth mod adds its required dependencies to the selected instance after checking game and loader versions, incompatibilities, destination paths, and hashes.
2. A player can see managed and local mods separately, check for updates, update, enable, disable, and uninstall supported entries for the selected instance.
3. When a planned change conflicts with a user-modified file, the player sees the conflict and can preserve the file or explicitly resolve it. A failed or interrupted operation preserves the previous usable contents.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Resolve compatible versions and required dependencies, handle incompatibilities, and record managed files and hashes per instance while detecting local files.
- Add management controls and conflict prompts. Before the first Modrinth file mutation, apply the journal, staging, verification, containment, and recovery rules established for instances and game files. Preserve user-modified files before destructive replacement.

**Verification and evidence**

- Run dependency, compatibility, collision, integrity, managed-versus-local file, and interruption fixtures for install, update, disable, and removal.
- In a supported loader instance on Linux, install a mod with a required dependency, exercise management controls, resolve a local modification conflict, and launch the instance.

### Phase 12: Modpack Install, Import, and Export

**Goal**: Players can create isolated instances from catalog or local Modrinth packs and export an instance in a supported interchange format.
**Mode:** mvp
**Depends on**: Phases 5, 8, 9, 10, and 11.
**Requirements**: PACK-01, PACK-02, PACK-03
**Success Criteria** (what must be TRUE):

1. Selecting a compatible catalog modpack version creates a new isolated instance with the declared Minecraft and loader versions, verified files, and applicable client overrides.
2. A local `.mrpack` installs through the same flow after its index, declared paths, hashes, and client overrides are validated. An invalid pack shows a clear error and leaves existing instances usable.
3. A player can export an instance and import the result into a new instance. The export contains only compliant references or permitted overrides and excludes credentials and redistribution-restricted content.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Use a shared catalog and local `.mrpack` pipeline. Validate the manifest, client override order, destination paths, links, collisions, and expansion bounds before publishing staged files.
- Resolve the required game and loader versions, verify declared files, and use the existing journal to make partial imports recoverable.
- Implement supported export references and overrides with explicit file eligibility, a clear report of excluded content, and verified publication of the export.

**Verification and evidence**

- Run archive and manifest fixtures for malformed metadata, path containment, links, collisions, expansion bounds, corrupted files, client overrides, and interrupted imports. Inspect exports for secret and restricted content exclusion.
- Install a catalog pack and import a local pack on Linux. Export and reimport a supported instance, compare declared versions and allowed content, and launch a representative resulting instance.

### Phase 13: Resource Packs and Shaders

**Goal**: Players can manage resource packs and shaders per instance and see compatibility requirements before installation.
**Mode:** mvp
**Depends on**: Phases 3, 4, 8, 9, 10, and 11.
**Requirements**: ASSET-01, ASSET-02
**Success Criteria** (what must be TRUE):

1. A player can find a compatible resource pack, download and verify it, and install, update, or remove it from the selected instance.
2. A player can find a shader, see which compatible shader support it requires before installing, and install, update, or remove it from a supported instance.
3. A failed verification leaves previously installed content usable. Changes in one instance leave other instances' resource packs and shaders intact.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Extend per-instance content management with separate resource-pack and shader destinations, compatible version selection, and managed ownership records.
- Show required shader support and route downloads, updates, and removals through the existing verified, recoverable file operations.

**Verification and evidence**

- Run type and compatibility fixtures for managed paths, missing shader support, integrity failures, and update or removal isolation.
- On Linux, install, update, and remove a representative resource pack and shader. Confirm the installed content is visible in a compatible game instance and exercise the missing-support message.

### Phase 14: Updates, Backup, and Recovery

**Goal**: Players can review a modpack update, protect their local data, and restore a usable instance after a failed or interrupted change.
**Mode:** mvp
**Depends on**: Phases 3, 11, 12, and 13.
**Requirements**: REC-01, REC-02
**Success Criteria** (what must be TRUE):

1. A modpack update preview shows additions, replacements, removals, and conflicts. The launcher creates a recoverable backup of affected local files and saves before applying the update.
2. A player can create an instance backup, restore it, and reopen the restored instance with its saved settings, files, and worlds.
3. If a content update fails or is interrupted, reopening recovers the previous usable state, or exposes the retained backup and actionable restore controls. A recovered installed instance can launch with a valid session.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Compare installed and target pack versions, preview the changes, and require conflict resolution and a verified backup before applying destructive operations.
- Build backup and restore controls on the existing operation journal, including repeatable startup reconciliation, previous-state restoration, and safe handling of disk or permission errors. Define backup retention and cross-filesystem behavior during planning.

**Verification and evidence**

- Exercise failed and interrupted updates at journal and publication boundaries, including disk-full, permissions, repeated restart, and file conflicts. Compare the restored files and state with the backup.
- On Linux, preview and apply a representative pack update, interrupt a second update, reopen, restore the previous instance with its saves, and launch it.

### Phase 15: Quality and Performance

**Goal**: The owner can reproduce test, safety, performance, and Linux smoke evidence for the complete feature set.
**Depends on**: Phases 7 through 14.
**Requirements**: QUAL-01, QUAL-02, QUAL-03
**Success Criteria** (what must be TRUE):

1. The owner can run the CI checks for Rust, TypeScript, IPC, UI, dependency auditing, and fixture-based integration, and see passing regression results for the main user flows.
2. The release evidence shows that supported install paths preserve filesystem containment, archive safety, direct process invocation, secret redaction, file integrity, and recovery.
3. The owner can reproduce startup and installation measurements using the recorded Linux target, inputs, cache conditions, and procedure.
4. The smoke matrix records real Linux runs for Vanilla and each of Fabric, Quilt, NeoForge, and Forge, plus representative Modrinth content and outage-independent launch.

**Plans**: TBD
**UI hint**: yes

**Implementation tasks**

- Complete CI and regression coverage across desktop commands, user flows, provider fixtures, and supported install and recovery paths. Verify the security controls established in earlier phases.
- Measure startup and installation with defined inputs and cache conditions. Consolidate the reproducible smoke matrix and procedures, and resolve regressions found during verification.

**Verification and evidence**

- Retain CI results, audit output, containment and archive regression results, direct invocation checks, redaction assertions, integrity failures, and recovery results.
- Record separate authenticated Linux smoke runs for the required Vanilla and loader paths, representative Modrinth install and recovery paths, and an installed launch during a Modrinth outage. Record commands, versions, conditions, and repeatable measurements without secrets.

### Phase 16: Linux Packaging and Release Preparation

**Goal**: Players can install, open, and remove a Linux release candidate, and the owner can review the evidence and artifacts before authorizing publication.
**Mode:** mvp
**Depends on**: Phase 15. External publication requires explicit owner authorization.
**Requirements**: DIST-01, DIST-02, DIST-03
**Success Criteria** (what must be TRUE):

1. On a clean supported Linux session, a player can build from the Arch PKGBUILD, install the candidate, start it from its desktop entry with the correct icon, and uninstall it using the documented steps and dependencies.
2. The owner can inspect separate AppImage and Flatpak evaluations covering WebKitGTK, local Java, keyring, browser callback, and filesystem access. A suitable AppImage builds and passes these checks, and the Flatpak decision is recorded.
3. The owner can review the candidate's version, changelog, install and removal instructions, and reproducible build jobs. External publication remains gated on owner approval.

**Plans**: TBD

**Implementation tasks**

- Prepare the Arch PKGBUILD, icons, desktop entry, dependency declarations, and clean-session instructions. Evaluate AppImage and Flatpak independently and build an AppImage if it meets the target requirements.
- Prepare versioning, changelog, and reproducible release jobs. Verify branding, licensing, and distribution constraints, and exclude game binaries and credentials from the packages.

**Verification and evidence**

- Build from a clean environment with documented inputs and run local package and metadata checks, including inspection for embedded credentials and game binaries.
- Install, launch, and uninstall each candidate package on the supported Linux target. Check browser sign-in and callback, keyring access, Java discovery, instance storage, and a real game launch. Record distribution-format decisions and submit the local candidate for owner review.

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Linux Bootstrap and Minecraft Access Feasibility | 3/3 | Complete    | 2026-11-13 |
| 2. Dark Interface and Navigation | 0/TBD | Not started | - |
| 3. Persistent Instances | 0/TBD | Not started | - |
| 4. Minecraft Manifest and Verified Downloads | 0/TBD | Not started | - |
| 5. Vanilla Installation and Local Java | 0/TBD | Not started | - |
| 6. Microsoft Accounts | 0/TBD | Not started | - |
| 7. Authenticated Vanilla Launch | 0/TBD | Not started | - |
| 8. Fabric and Quilt | 0/TBD | Not started | - |
| 9. NeoForge and Forge | 0/TBD | Not started | - |
| 10. Modrinth Catalog | 0/TBD | Not started | - |
| 11. Mod Management | 0/TBD | Not started | - |
| 12. Modpack Install, Import, and Export | 0/TBD | Not started | - |
| 13. Resource Packs and Shaders | 0/TBD | Not started | - |
| 14. Updates, Backup, and Recovery | 0/TBD | Not started | - |
| 15. Quality and Performance | 0/TBD | Not started | - |
| 16. Linux Packaging and Release Preparation | 0/TBD | Not started | - |
