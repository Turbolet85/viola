group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 104 · «- 2026-10-20: A test that ends a served duplex session (`tok»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»

- leaf: .claude/rules/testing.md:18   class: stale
  leaf-text: «root `tests/{cli,hook,tui,channel,chaos,contract}_<topic>.rs`»
  source: .andromeda/test-plan-detail/test-strategy.md:42 «**Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs`»
  replace-with: «root `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs`»

- leaf: .claude/rules/testing.md:34   class: omission
  leaf-text: «Never hand-write `ledger/stamps.json` (only `viola verify` against the fake agent), `snapshot.json` (only the wrapper)»
  source: .andromeda/test-plan.md:631 «never hand-written — except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`»
  replace-with: «Never hand-write `ledger/stamps.json` (only `viola verify` against the fake agent), `snapshot.json` (only the wrapper, save the server-identity impostor cases' test-built snapshot, Linux and Windows, recording a held live process's pid beside the endpoint the test binds)»

- leaf: .claude/docs/tests-summary.md:33   class: omission
  leaf-text: «exit-cause matrix, security-control negatives»
  source: .andromeda/test-plan.md:604 «**Exit-cause matrix** (`cross_exit_causes.rs`, not built): one negative test per cause of exit 21 and of exit 1»
  replace-with: «exit-cause matrix (`cross_exit_causes.rs` not built: each built cause is a case of the binary its row names, the server-verification cause in `tests/security_negatives_server_identity.rs` on Linux and Windows), security-control negatives»

- leaf: .claude/rules/verification-harness.md:41   class: omission
  leaf-text: «`--exit-no-eof`, `--report-version`»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:69 «The `--exit-no-eof` holder's re-exec mode `--hold-stdout-internal` (it sleeps 10 s and writes nothing) is also spawned directly by the server-identity impostor cases»
  replace-with: «`--exit-no-eof` (its re-exec mode `--hold-stdout-internal`, which sleeps 10 s and writes nothing, is also spawned directly by the server-identity impostor cases, `tests/security_negatives_server_identity.rs` and `tests/hook_fail_open.rs`, as the live process their snapshot records, reaped by each file's `Held` `Drop` guard), `--report-version`»
