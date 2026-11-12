# Integration and Release Risks

Research snapshot taken on 2026-11-12.

## Microsoft and Minecraft access

Microsoft Entra documents desktop applications as public clients and supports authorization code with PKCE. The proposed flow uses the system browser, a short-lived listener bound to loopback, a single-use state value and PKCE verifier, and a project-owned client ID configured for personal Microsoft accounts. Verify the exact redirect URI with the registration and provider before implementation. A distributed desktop binary cannot keep a client secret. [Microsoft desktop configuration](https://learn.microsoft.com/en-us/entra/identity-platform/scenario-desktop-app-configuration) · [Microsoft authorization code flow](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow) · [RFC 8252](https://www.rfc-editor.org/info/rfc8252/)

Minecraft Game Service API access is a separate feasibility dependency. The official Java support index lists an application review. The review information shortcut redirected to the official help article, whose body was unavailable during verification. The review form shortcut reached Microsoft Forms with HTTP 200. This confirms reachability only. Current submission eligibility, required fields, turnaround, and access for Nebula remain unverified. [Java Edition support](https://help.minecraft.net/hc/en-us/sections/44460521913485) · [API review article](https://help.minecraft.net/hc/en-us/articles/16254801392141) · [Review form](https://aka.ms/mce-reviewappid)

During the foundation phase, establish who owns the app registration, confirm the review route and required permissions, and record its status. Sign-in acceptance requires a live test with a Nebula-owned authorized ID and an eligible account. Microsoft sign-in, Xbox/XSTS exchange, Minecraft session, entitlement, and profile retrieval each need their own success and failure evidence. An unapproved client or an unavailable review path blocks authenticated launch acceptance. Independent metadata and installation work can continue. [Minecraft Launcher troubleshooting](https://help.minecraft.net/hc/en-us/articles/6662588435597-Minecraft-Launcher-Troubleshooting-FAQ-)

Keep refresh credentials in a Linux Secret Service implementation. Test absent and locked services as distinct states. Session refresh and sign-out need per-account coordination. Filter sensitive OAuth values and game arguments before emitting logs, IPC events, or diagnostic exports. [Secret Service specification](https://specifications.freedesktop.org/secret-service/latest/)

## Minecraft metadata and Java compatibility

A live GET of Mojang's version manifest returned HTTP 200. It listed 923 entries, including release `26.3` and snapshot `26.4-snapshot-8` as latest at the time of the request. These values are observations for fixtures and investigation. Resolve current versions from the manifest at runtime. [Mojang version manifest](https://piston-meta.mojang.com/mc/game/version_manifest_v2.json)

| Sample | Java major | Argument shape |
|---|---:|---|
| [1.8.9](https://piston-meta.mojang.com/v1/packages/d546f1707a3f2b7d034eece5ea2e311eda875787/1.8.9.json) | 8 | Legacy |
| [1.20.1](https://piston-meta.mojang.com/v1/packages/3ecf2cab995685d345a257326ba72192b7911250/1.20.1.json) | 17 | Structured |
| [1.21.1](https://piston-meta.mojang.com/v1/packages/9aa86ea608c3ec777dcdc09e6c5a06c85b53599e/1.21.1.json) | 21 | Structured |
| [26.3](https://piston-meta.mojang.com/v1/packages/9f67e10ed013deeaad20ac3c800652765d493408/26.3.json) | 25 | Structured |
| [26.4-snapshot-8](https://piston-meta.mojang.com/v1/packages/46f6dc6a5cefe084af0ce2ecf7f433c1222d4dd1/26.4-snapshot-8.json) | 25 | Structured |

The downloaded metadata SHA-1 matched the manifest for all five samples. The release and snapshot exposed arguments, main class, libraries, downloads, asset index, and logging metadata. The 1.8.9 sample also uses a Linux native classifier mapping. These reads validate public metadata availability and selected schema variations. The installation and launch phases must verify version and architecture rules, native selection, asset resolution, and client launch on representative versions. Preserve unknown fields and report unsupported metadata or combinations explicitly.

## Phase-specific failure modes

| Area | Failure mode | Required evidence |
|---|---|---|
| Downloads | An interrupted or changed HTTP response leaves a partial artifact in use. | Simulate truncated downloads, stale validators, hash mismatch, cancellation, and retry. Publish only a verified file and retain the previous valid installation on failure. |
| Instances | A rename, delete, or failed update affects another instance or overwrites user files. | Verify filesystem containment, concurrent operation handling, interrupted changes, and preservation of local files and saves. |
| Modloaders | Profile metadata and installer-based flows require different Java, library, and launch handling. | Use dedicated adapters and record separate Linux installation and launch evidence for each loader. |
| Modrinth | Provider limits or an outage can prevent catalog access. | Exercise throttling, cache expiry, and failure states with fixtures. Launch an installed instance with Modrinth unavailable and a valid Minecraft session. |
| Modpacks | An archive can request unsafe destinations or contain more data than anticipated. | Validate paths, links, size limits, and manifest integrity before promotion. Verify that interrupted updates preserve or restore the prior usable instance. |
| Packaging | Browser callbacks, keyring access, and local Java discovery differ by package and desktop environment. | Test the release build from a clean Linux user session. Evaluate AppImage and Flatpak independently before choosing deliverables. |

Review Minecraft branding and distribution terms and Modrinth project redistribution restrictions before external packaging. Keep game binaries and user credentials out of launcher packages and exports. [Minecraft Usage Guidelines](https://www.minecraft.net/en-us/usage-guidelines)
