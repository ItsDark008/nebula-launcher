# Nebula Launcher

## Project context

- [Project brief](.planning/PROJECT.md)
- [Current state](.planning/STATE.md)
- [Roadmap](.planning/ROADMAP.md) and [requirements](.planning/REQUIREMENTS.md)
- [Architecture](.planning/research/ARCHITECTURE.md), [stack](.planning/research/STACK.md), and [integration risks](.planning/research/PITFALLS.md)

## Development

Use GSD Core to discuss, plan, execute, and verify phases. Start with the current phase recorded in the project state. Use English for repository documentation and Git commit messages.

Deliver changes through GitHub pull requests using the connected GitHub account. Create commits only on the PR branch. Give each commit an English subject that names the change and a body that explains why it was needed. Include the scope, rationale, and verification in the PR description. Leave merging to the repository owner.

Keep the Rust core testable independently of Tauri. Use narrowly scoped, typed IPC. Keep account secrets in the Linux keyring and use credential-free fixtures in automated tests.

Verify authenticated Vanilla and each modloader through a real install and launch on Linux. Record those results separately from automated checks. Public packages and releases require explicit owner authorization.
