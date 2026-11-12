# Feature Research: Nebula Launcher

**Domain:** Linux Minecraft Java launcher and Modrinth client  
**Researched:** 2026-11-12  
**Confidence:** MEDIUM. Official documentation reviewed and three public Modrinth endpoints checked live. Compatibility and complete installations still require implementation and real launch tests.

## Feature Landscape

| Feature | User expectation / scope | Complexity | Delivery |
|---|---|---|---|
| Persistent isolated instances | Create, edit, duplicate, delete with confirmation; save paths, settings and last launch | High | A |
| Vanilla install and launch | Official releases and snapshots, verified downloads, local compatible Java, configurable memory and JVM arguments, process logs | High | A |
| Microsoft accounts | System browser, secure token storage, refresh, entitlement and profile checks, multiple accounts and logout | High | A; establish API access feasibility first |
| Downloads and diagnostics | Progress, cancellation, controlled retries, integrity, clear failure states and secret-free logs | High | A; extended throughout |
| Loader support | Separate Fabric, Quilt, NeoForge and Forge adapters; select compatible versions and genuinely launch each | High | B |
| Native Modrinth catalog | Search mods, modpacks, resource packs and shaders; filters, paging, details and supported sorts | Medium | C |
| Mod management | Compatible versions, dependency resolution, installation, enable/disable, update, remove, local files and conflict handling | High | C |
| Modpacks | Catalog installation, `.mrpack` import/export, loader selection, verified downloads, overrides, update and recovery | High | C |
| Resource packs and shaders | Search, compatibility, install, update and removal per instance; identify shader runtime requirements | Medium | C |
| Instance safety | Backups and restore before significant changes; protect saves and user-modified files | High | C, with deletion safeguards in A |
| Linux release | Accessible responsive dark UI, documentation, tests, CI and suitable Linux packages | High | A for usable UI; D for hardening and packaging |

**Differentiating value:** the coherent native Modrinth-to-isolated-instance workflow and recovery after failed changes are explicit project goals. They depend on a working launcher first, and should not displace the vanilla MVP.

## Exclusions and Constraints

| Avoid | Reason | Alternative |
|---|---|---|
| Cracked accounts or license bypass | Violates the project scope | Official authentication and entitlement checks |
| Automatic Java distribution in the first release | Explicitly deferred | Detect and select local compatible Java |
| Silent overwrite or deletion of user files | Risks worlds and configuration | Show conflicts, confirm deletion, back up and restore |
| Packaging content without distribution rights | Format support does not grant redistribution rights | Export only permitted references and files; report exclusions |
| Making installed instances depend on Modrinth availability | Violates offline resilience requirement | Persist launch metadata and assets locally; require a valid Minecraft session |
| Windows support or required light theme for first release | Explicitly out of scope | Focus on Linux x86_64 and accessible dark theme |

## Official API and Format Findings

- **Modrinth:** [Search](https://docs.modrinth.com/api/operations/searchprojects/) supports facets, offset/limit and `relevance`, `downloads`, `follows`, `newest`, `updated` sorts; do not invent a separate popularity sort. The [API overview](https://docs.modrinth.com/api/) specifies stable IDs, rate-limit headers and an identifiable User-Agent. A public `GET /v2/search?query=sodium&limit=1` returned HTTP 200 on 2026-11-12; that checks availability only. Version metadata distinguishes required, optional, incompatible and embedded dependencies; hashes and update lookups are documented in [version metadata](https://docs.modrinth.com/api/operations/getdependencies/) and [hash-based updates](https://docs.modrinth.com/api/operations/getlatestversionsfromhashes/). Treat unresolved or external dependencies as visible errors rather than assuming the metadata guarantees a complete solution.
- **`.mrpack`:** the [official specification](https://support.modrinth.com/en/articles/8802351-modrinth-modpack-format-mrpack) defines a ZIP with root `modrinth.index.json`, file paths, hashes, HTTPS downloads, environment flags and Minecraft/loader dependencies. Apply shared overrides then client overrides; do not apply server overrides to clients. Reject unsafe paths, unknown required dependencies and unsupported formats; respect host restrictions and verify hashes before applying. Plan rollback and conflicts before updates, not after them.
- **Fabric / Quilt:** [Fabric Meta](https://meta.fabricmc.net/) exposes v2 game-specific loader lists and launcher profile JSON; [Quilt Meta](https://meta.quiltmc.org/) exposes corresponding v3 routes and requests an identifying User-Agent. Profile consumption still requires compatible base-game metadata, libraries and tested launch construction.
- **NeoForge / Forge:** [NeoForge's client guide](https://docs.neoforged.net/user/docs/client/) describes selecting and running its installer, and its [user guide](https://docs.neoforged.net/user/docs/) documents version-dependent Java. [Forge's official downloads](https://files.minecraftforge.net/net/minecraftforge/forge/) offer versioned installers. These sources establish installer-based procedures, not a universal headless third-party client integration contract. Research exact version-specific metadata, installer execution and outputs before implementation; do not treat them as Fabric profile adapters. Each adapter needs a real install and launch check.

## Dependencies and Roadmap

Public Modrinth v2 requests for a Sodium search with mod and Fabric facets, the Sodium project, and its versions filtered to Fabric and Minecraft 1.21.1 all returned HTTP 200. The version request returned 23 entries and the first file exposed SHA-1 and SHA-512. The responses advertised a rate limit of 300. The client must read provider rate-limit headers at runtime. [API overview](https://docs.modrinth.com/api/)

```text
Authentication feasibility → account integration → authenticated real launch
Persistent instance + official version metadata + verified downloads + local Java → vanilla launch
Vanilla launch + loader-specific metadata/install procedure → each tested loader
Catalog API + instance compatibility + verified downloads → mod management
Loader adapters + safe archive import + dependency handling → modpack installation
Backup/restore + conflict handling → modpack and content updates
Resource/shader catalog + instance compatibility → per-instance management
```

1. **A — Vanilla MVP:** bootstrapped UI, persistence, accounts, downloads, Java detection and actual launch; verify a real entitled account end to end, including restarting the launcher. Authentication API access is an early external gate.
2. **B — Loaders:** Fabric and Quilt first, followed by separate NeoForge and Forge adapters; validate each through installation and actual launch. Exact supported game/loader/Java combinations need phase research.
3. **C — Modrinth:** API and discovery, then compatible mods, modpack import/install/export, resource packs and shaders. Implement baseline safe operations and recovery with the first mutating content workflow; deepen update and restore coverage thereafter.
4. **D — Release:** hardening, performance, CI, documentation and Linux packages. Security and integrity checks are required in the phase introducing each relevant operation, rather than postponed to D.

**Open questions:** Microsoft protected API registration/access; version-specific Forge/NeoForge installer and profile behavior; exact license-aware export policy and handling for unresolvable or externally hosted pack entries. These require dedicated research and tests; API documentation alone does not establish working launches or legal redistribution rights.
