# Minecraft Game Service access

The project owner creates and owns the Microsoft Entra registration for Nebula Launcher and submits the Minecraft API review request from their own account. The owner confirmed registration and approval on 2026-11-14. On the same date the development launcher retrieved an eligible Minecraft profile after the Microsoft, Xbox, Minecraft and entitlement exchanges. Further Phase 6 checks cover refresh, account switching and logout. Phase 7 verifies an authenticated Vanilla install and launch on Linux. [Phase 1 decisions](../.planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-CONTEXT.md) · [Roadmap](../.planning/ROADMAP.md)

## Registration checklist

Microsoft documentation checked on 2026-11-13.

1. From your own account, sign in to the Microsoft Entra admin center and select the directory that should own the registration. Check that you have permission to register apps there. Go to **Entra ID → App registrations → New registration** and use **Nebula Launcher** as the display name. [Microsoft Entra registration](https://learn.microsoft.com/en-us/entra/identity-platform/quickstart-register-app)
2. Under **Supported account types**, confirm that the selected audience includes personal Microsoft accounts. Entra documents both a personal-only audience and one that includes organizational and personal accounts. Personal accounts only is the starting choice for the launcher's player sign-in. Confirm the saved audience and any provider requirement for a different audience or sign-in authority before implementing the live flow. [Microsoft Entra registration](https://learn.microsoft.com/en-us/entra/identity-platform/quickstart-register-app)
3. Under **Authentication → Add a platform**, choose **Mobile and desktop applications**. Microsoft documents `http://localhost` for system-browser sign-in and allows an ephemeral port when matching a `localhost` redirect. Keep the registered and requested path consistent, with no query string in the registered URI for a personal-account audience. Confirm the exact saved redirect URI and the runtime callback before implementing sign-in. [Desktop app configuration](https://learn.microsoft.com/en-us/entra/identity-platform/scenario-desktop-app-configuration) · [Redirect URI rules](https://learn.microsoft.com/en-us/entra/identity-platform/reply-url)
4. Under **Authentication → Advanced settings**, review **Allow public client flows**. Microsoft's desktop guide documents setting it to **Yes** for public-client flows without a redirect, such as device code. Confirm the saved configuration for the chosen flow. The planned sign-in is OAuth 2.0 authorization code with PKCE (`S256`) through the system browser and a loopback listener. The desktop app is a public client and uses no client secret. Verify the required Minecraft and Xbox API permissions, consent, and exact scopes with provider or registration evidence before implementing the live exchange. [Desktop app configuration](https://learn.microsoft.com/en-us/entra/identity-platform/scenario-desktop-app-configuration) · [Authorization code with PKCE](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow)
5. Use the **Application (client) ID** from **Overview** to identify the Nebula app in OAuth and the review form. Use the **Directory (tenant) ID** to identify the owning Entra directory on the new approval request. Record only dated outcome categories in this runbook. [Desktop app configuration](https://learn.microsoft.com/en-us/entra/identity-platform/scenario-desktop-app-configuration) · [AppID Review form](https://aka.ms/mce-reviewappid)

## Minecraft API review

On 2026-11-13, Microsoft Forms returned the public **AppID Review** page and its form definition without sign-in. The Minecraft Help page did not expose the article body during the check. The checklist below comes from the form, and the Minecraft EULA and Usage Guidelines were available. [Minecraft Help article](https://help.minecraft.net/hc/en-us/articles/16254801392141) · [AppID Review form](https://aka.ms/mce-reviewappid) · [Minecraft EULA](https://www.minecraft.net/en-us/eula) · [Minecraft Usage Guidelines](https://www.minecraft.net/en-us/usage-guidelines)

After creating the registration, the owner opens the form and prepares these inputs.

| Input | What to supply |
| --- | --- |
| Request type | **New AppID for Approval**. |
| Application name | The Entra display name, **Nebula Launcher**. Follow the naming restrictions at the top of the form. |
| Application ID | The new registration's Application (client) ID. Submit one ID per request. |
| Tenant ID | The Directory (tenant) ID for the new registration. The form specifies that this is required for new approvals. |
| Contact information | A valid email address that the reviewer can cross-reference in the Azure portal. |
| Associated public website or domain | A publicly reachable URL that provides information about Nebula Launcher. A private repository cannot meet this requirement because reviewers need access to the page. The owner chooses the qualifying URL. |
| Justification | A short description of Nebula Launcher and why it needs API access to authenticate players and verify Minecraft: Java Edition ownership before enabling gameplay. |
| Terms acknowledgment | Confirm that you have read and understood the Minecraft EULA and Usage Guidelines. |

The form requires authentication and game ownership checks before enabling gameplay or access to servers or sites. Submit the request once from your own account when the inputs are ready, keep the acknowledgment and any approval privately, and record their dated outcomes below. The form reports a weekly review cadence. The owner's eligibility, approval outcome, turnaround for an individual request, required protected permissions, and working access for the Nebula client remain open. [AppID Review form](https://aka.ms/mce-reviewappid)

## Access status

| Milestone | Status on 2026-11-14 | Evidence required to advance |
| --- | --- | --- |
| Registration | Registered; owner confirmed | Owner confirmed the registration exists. |
| Review submission | Submitted; owner confirmed | Owner confirmed submission. |
| Review approval | Approved; owner confirmed | Owner confirmed approval; provider correspondence has not been inspected. |
| Protected access | Verified on 2026-11-14 | Owner supplied a screenshot of the eligible profile shown as active in the development launcher following sign-in. |

Record only dates and outcome categories. Keep account details, registration identifiers, correspondence, credentials, and response data outside repository evidence.

## Secret Service probe

On 2026-11-13, the CachyOS niri/Wayland desktop session had a reachable user D-Bus. The bus reported no owner for `org.freedesktop.secrets` and did not advertise the name for activation. With no service available, the probe stopped before `org.freedesktop.Secret.Service.ReadAlias` for `default`. The default collection and its `org.freedesktop.Secret.Collection.Locked` state remain unobserved in this session. [Secret Service specification](https://specifications.freedesktop.org/secret-service/latest/)

Phase 1 research recorded an unlocked default collection in a KDE/Wayland session on 2026-11-12. The current niri result gives Phase 6 a real missing-service case to handle alongside locked and available states. A credential round trip and session refresh still need verification in Phase 6. [Phase 1 research](../.planning/phases/01-linux-bootstrap-and-minecraft-access-feasibility/01-RESEARCH.md) · [Phase 6 roadmap](../.planning/ROADMAP.md#phase-6-microsoft-accounts)
