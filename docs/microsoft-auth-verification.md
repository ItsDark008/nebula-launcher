# Microsoft account verification

## Manual Linux run — 2026-11-14

- Environment: CachyOS x86_64, niri/Wayland, development Tauri window, GNOME Keyring Secret Service. The owner used an eligible Java Edition account and the approved project-owned public client.
- The owner confirmed account selection in the system browser. The development launcher displayed the eligible Minecraft profile after sign-in; this path includes the Microsoft, Xbox, XSTS, Minecraft session, entitlement and profile requests.
- The owner confirmed the stored account returned after restarting the application and that **Refresh session** completed without an error.
- The owner removed the account, observed it disappear, and signed in again successfully.
- The owner has one eligible account. Switching between two real accounts was not exercised; the account-selection and removal behavior is covered by credential-free fixtures instead.
- No passwords, authorization codes, provider tokens, registration identifiers or account identifiers are recorded in this evidence.

This is an authentication check, not a Minecraft install or launch. The authenticated Vanilla and each modloader install and launch remain separate roadmap checks.

## Automated checks

Run `cargo test --locked --workspace`, `npm test -- --run`, and `npm run build`. The Rust tests cover PKCE, callback validation, expiry, replay, provider denial, account-state fixtures, redaction and representative exchange responses. Frontend tests cover navigation and account actions. All automated data is synthetic.
