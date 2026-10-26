group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/ci.md · 35 · «- 2026-10-13: A CI job that reads `cancelled` with no steps »
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:31   class: omission
  leaf-text: «the only ignore is RUSTSEC-2017-0008 via portable-pty `=0.8.1`);»
  source: .andromeda/security-plan-detail/dependency-security.md:58 «The open advisory RUSTSEC-2026-0386 (`unsound`) against the locked tokio `=1.53.1` fails CI's `supply-chain` job. It is owned by the working-route entry "tokio RUSTSEC-2026-0386 lock bump": a lock bump»
  replace-with: «the only ignore is RUSTSEC-2017-0008 via portable-pty `=0.8.1`; the open RUSTSEC-2026-0386 (`unsound`) against the locked tokio `=1.53.1` fails CI's `supply-chain` job, owned by the route entry "tokio RUSTSEC-2026-0386 lock bump": a lock bump of `tokio` to `>=1.53.3`, never an ignore);»
