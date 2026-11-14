# Requirements for Nebula Launcher

**Defined:** 2026-11-12
**Core Value:** A Linux player with an eligible Minecraft account can install and launch Minecraft Java Edition in an isolated instance and keep their settings across restarts.

## v1 Requirements

The first complete Linux release spans milestones A through D. Milestone A delivers the Vanilla MVP.

### Desktop foundation

- [x] **BASE-01**: Run the development desktop shell on CachyOS with a Rust core that can be tested without Tauri.
- [x] **BASE-02**: Expose typed commands, status events, and sanitized errors through narrowly scoped Tauri capabilities.
- [x] **BASE-03**: Document and verify the application registration and review path for a Nebula-owned Microsoft client ID, with an explicit blocked state when access is unavailable.

### Interface

- [x] **UI-01**: Navigate the dashboard, instances, downloads, accounts, settings, and diagnostics through an original dark interface with reusable design tokens and components.
- [x] **UI-02**: Use primary controls by keyboard with visible focus, reduced-motion support, and usable layouts at supported window sizes.

### Instances and persistence

- [ ] **INST-01**: Create and edit a named instance with a custom icon, an independent game directory, and persistent configuration.
- [ ] **INST-02**: Duplicate an instance and delete it through an explicit confirmation that identifies the instance and affected user data.
- [ ] **INST-03**: See each instance's installation state, last launch, and saved settings after reopening the launcher.

### Minecraft metadata and downloads

- [ ] **GAME-01**: Browse release and snapshot versions from Mojang's manifest, with cached metadata and an explicit unavailable state.
- [ ] **DOWN-01**: Queue asynchronous transfers with bounded concurrency and visible progress, bytes transferred, speed, cancellation, and errors.
- [ ] **DOWN-02**: Retry eligible failures with bounded backoff, honor timeouts, and resume when the provider supports validated range requests.
- [ ] **DOWN-03**: Verify expected sizes and hashes before safely publishing downloaded files and recover cleanly from interrupted transfers.

### Vanilla and Java

- [ ] **GAME-02**: Install, update, and repair Vanilla from version metadata, including the client, compatible libraries, natives, and asset index and objects.
- [ ] **JAVA-01**: Detect local Java executables, check their version and architecture, and select a compatible runtime automatically or by explicit path.
- [ ] **JAVA-02**: Persist per-instance RAM and JVM arguments and show actionable errors when the selected runtime or configuration is incompatible.

### Accounts

- [ ] **AUTH-01**: Sign in via the system browser with the supported public-client OAuth flow and PKCE where applicable, with a verified callback for redirect-based sign-in.
- [ ] **AUTH-02**: Use a project-owned, authorized client ID to complete Microsoft, Xbox, and Minecraft authentication and verify entitlement and profile.
- [ ] **AUTH-03**: Persist account secrets through a Linux keyring, refresh sessions, and surface missing, locked, expired, or revoked sessions without disclosing secrets.
- [ ] **AUTH-04**: Switch among multiple accounts and remove an account's local session and stored secrets on logout.

### Launching and diagnostics

- [ ] **PLAY-01**: Launch a real authenticated Vanilla client using its metadata-derived arguments, classpath, native libraries, and instance working directory.
- [ ] **PLAY-02**: See process status, sanitized game logs, and exit or failure details in the launcher.
- [ ] **PLAY-03**: Complete the full Vanilla workflow on Linux with an eligible account and confirm that settings and instance state survive reopening the app.

### Modloaders

- [ ] **LOAD-01**: Discover compatible Fabric versions, install one through its adapter, and launch a Fabric instance on Linux.
- [ ] **LOAD-02**: Discover compatible Quilt versions, install one through its adapter, and launch a Quilt instance on Linux.
- [ ] **LOAD-03**: Discover compatible NeoForge versions, complete its installer flow through its adapter, and launch a NeoForge instance on Linux.
- [ ] **LOAD-04**: Discover compatible Forge versions, complete its installer flow through its adapter, and launch a Forge instance on Linux.

### Modrinth catalog

- [ ] **MODR-01**: Search mods, modpacks, resource packs, and shaders inside the launcher with applicable game-version, loader and category filters, supported sorting, and pagination.
- [ ] **MODR-02**: View project icons and images, descriptions, authors, versions, and original Modrinth links.
- [ ] **MODR-03**: Use a typed API client with an identifiable User-Agent, bounded caching, rate-limit handling, timeout and network errors, and no catalog dependency for launching installed instances.

### Mods

- [ ] **MODS-01**: Install a compatible Modrinth mod and its required dependencies after checking game version, loader, incompatibilities, hashes, and destination paths.
- [ ] **MODS-02**: List managed and local mods per instance, detect available updates, update, enable, disable, and uninstall while preserving user-modified files through explicit conflict handling.

### Modpacks

- [ ] **PACK-01**: Install a selected Modrinth modpack version into an isolated instance with its required game and loader versions.
- [ ] **PACK-02**: Import a local .mrpack through the same install pipeline, validating modrinth.index.json, declared files and hashes, client overrides, and archive paths.
- [ ] **PACK-03**: Export an instance to a supported interchange format with compliant references or overrides, excluding credentials and any content whose redistribution is restricted.

### Resource packs and shaders

- [ ] **ASSET-01**: Find compatible resource packs and download, verify, install, update, and remove them for the selected instance.
- [ ] **ASSET-02**: Find, install, update, and remove shaders per instance and show the required compatible shader support before installation.

### Recovery

- [ ] **REC-01**: Preview a modpack update and protect local files and saves with a backup before applying it.
- [ ] **REC-02**: Back up and restore an instance, and recover the previous usable state after a failed or interrupted content update.

### Release quality

- [ ] **QUAL-01**: Run Rust, TypeScript, IPC, UI, and fixture-based integration checks in CI with dependency auditing and regression coverage for the main user flows.
- [ ] **QUAL-02**: Verify filesystem containment, safe archive handling, secure process invocation, secret redaction, data integrity, and recovery across all supported install flows.
- [ ] **QUAL-03**: Record reproducible startup and installation performance measurements and manual Linux smoke results for Vanilla, each loader, and representative Modrinth content.

### Linux distribution

- [ ] **DIST-01**: Build, install, launch, and uninstall the Linux release package with desktop entry, icons, and documented runtime dependencies. Provide an Arch PKGBUILD.
- [ ] **DIST-02**: Evaluate AppImage and Flatpak against WebKitGTK, Java, keyring, callback, and filesystem needs. Produce an AppImage if suitable and document the Flatpak decision.
- [ ] **DIST-03**: Prepare versioning, changelog, installation and removal instructions, and reproducible release jobs with owner approval required for external publication.

## v2 Requirements

- Evaluate managed Java downloads from trusted sources with license and integrity checks.
- Evaluate Windows support and an optional light theme.

## Out of Scope

| Feature | Reason |
|---|---|
| Cracked mode and license bypass | Sign-in and eligibility use official services. |
| External publication during bootstrap | Public releases require owner authorization. |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| BASE-01 | Phase 1 | Complete |
| BASE-02 | Phase 1 | Complete |
| BASE-03 | Phase 1 | Complete |
| UI-01 | Phase 2 | Complete |
| UI-02 | Phase 2 | Complete |
| INST-01 | Phase 3 | Pending |
| INST-02 | Phase 3 | Pending |
| INST-03 | Phase 3 | Pending |
| GAME-01 | Phase 4 | Pending |
| DOWN-01 | Phase 4 | Pending |
| DOWN-02 | Phase 4 | Pending |
| DOWN-03 | Phase 4 | Pending |
| GAME-02 | Phase 5 | Pending |
| JAVA-01 | Phase 5 | Pending |
| JAVA-02 | Phase 5 | Pending |
| AUTH-01 | Phase 6 | Pending |
| AUTH-02 | Phase 6 | Pending |
| AUTH-03 | Phase 6 | Pending |
| AUTH-04 | Phase 6 | Pending |
| PLAY-01 | Phase 7 | Pending |
| PLAY-02 | Phase 7 | Pending |
| PLAY-03 | Phase 7 | Pending |
| LOAD-01 | Phase 8 | Pending |
| LOAD-02 | Phase 8 | Pending |
| LOAD-03 | Phase 9 | Pending |
| LOAD-04 | Phase 9 | Pending |
| MODR-01 | Phase 10 | Pending |
| MODR-02 | Phase 10 | Pending |
| MODR-03 | Phase 10 | Pending |
| MODS-01 | Phase 11 | Pending |
| MODS-02 | Phase 11 | Pending |
| PACK-01 | Phase 12 | Pending |
| PACK-02 | Phase 12 | Pending |
| PACK-03 | Phase 12 | Pending |
| ASSET-01 | Phase 13 | Pending |
| ASSET-02 | Phase 13 | Pending |
| REC-01 | Phase 14 | Pending |
| REC-02 | Phase 14 | Pending |
| QUAL-01 | Phase 15 | Pending |
| QUAL-02 | Phase 15 | Pending |
| QUAL-03 | Phase 15 | Pending |
| DIST-01 | Phase 16 | Pending |
| DIST-02 | Phase 16 | Pending |
| DIST-03 | Phase 16 | Pending |

Coverage is 44 of 44 v1 requirements, each mapped once. [Proposed roadmap](ROADMAP.md)

---
*Last updated: 2026-11-12 after project discovery*
