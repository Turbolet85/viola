group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 94 · «- 2026-10-14: A probe for count literals a move leaves stale»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-11: On the Linux dev host `schema-check` (G4) read»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the `paste-<n>` / `clear-<n>` framing variants recorded by `viola verify`, the dialog tier by its Runs C and D, the framing tier by Run B's four added pastes;»
  source: .andromeda/test-plan.md:408 «and through Run B's probe status line (Unix) the two status line variants (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`), locally.»
  replace-with: «the `paste-<n>` / `clear-<n>` framing variants and the `StatusLine.default-1.json` / `StatusLine.rate-limits-1.json` status line variants recorded by `viola verify`, the dialog tier by its Runs C and D, the framing tier by Run B's four added pastes, the status line variants by Run B's probe status line (Unix);»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and `--framing` replays Run B's recorded paste and `/clear` variants, for verify's four interactive runs»
  source: .andromeda/test-plan.md:1094 «and, with an absolute `--settings` status line command, the set's `StatusLine.default-1.json` after the launch SessionStart and `StatusLine.rate-limits-1.json` once after the first Stop»
  replace-with: «and `--framing` replays Run B's recorded paste and `/clear` variants and, with an absolute `--settings` status line command, the set's `StatusLine.default-1.json` after the launch SessionStart and `StatusLine.rate-limits-1.json` once after the first Stop, for verify's four interactive runs»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «five framing variants in 2.1.287 and the superseded relayed fixtures included; 2.1.287 stamped at eighteen rows, 2.1.283 and 2.1.288 drift-only)»
  source: .andromeda/test-plan.md:667 «and end `stamped <ver>  <n> pass  0 fail`, `<n>` the relied count (`LedgerRow::relied()`'s length: 21 on Unix, 18 on Windows)»
  replace-with: «five framing variants and two status line variants in 2.1.287 and the superseded relayed fixtures included; 2.1.287 stamped at every relied row, 21 on Unix and 18 on Windows, 2.1.283 and 2.1.288 drift-only)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «are walked for hygiene by a run-time directory walk against `schemas/claude-fixture.v1.json`, the `Screen.*.json` screens apart»
  source: .andromeda/test-plan.md:438 «every `fixtures/claude/*/StatusLine.*.json` status line payload conforms to `schemas/claude-statusline-fixture.v1.json`»
  replace-with: «are walked for hygiene by a run-time directory walk against `schemas/claude-fixture.v1.json`, the `StatusLine.*.json` status line variants against `schemas/claude-statusline-fixture.v1.json`, the `Screen.*.json` screens apart»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and replays every dialog variant and every framing variant byte for byte (one payload is set»
  source: .andromeda/test-plan.md:438 «and compares the `statusline` receipts' `stdin_hex` byte for byte, in order and count, with each recorded set's `StatusLine.default-1.json` then `StatusLine.rate-limits-1.json` under `--framing`»
  replace-with: «and replays every dialog variant and every framing variant byte for byte, and compares the `statusline` receipts' `stdin_hex` byte for byte, in order and count, with each recorded set's `StatusLine.default-1.json` then `StatusLine.rate-limits-1.json` under `--framing` (Unix) (one payload is set»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «run the settings file's `statusLine.command` with the stdin file's bytes, only with both and only when it is absolute, and file a `statusline` receipt;»
  source: .andromeda/test-plan.md:1094 «and under `--framing` without `--statusline-stdin` with the set's recorded `StatusLine` variants»
  replace-with: «run the settings file's `statusLine.command` with the stdin file's bytes, only with both and only when it is absolute, or under `--framing` without `--statusline-stdin` with the set's recorded `StatusLine` variants, and file a `statusline` receipt;»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «no recorded fixture of source `resume` exists yet, owed to the route entry "Owed ledger rows")»
  source: .andromeda/test-plan.md:1094 «the recorded payload is owed to the route entry "Resume and compact ledger rows", directly after "Owed ledger rows"»
  replace-with: «no recorded fixture of source `resume` exists yet, owed to the route entry "Resume and compact ledger rows", directly after "Owed ledger rows")»

- leaf: .claude/rules/testing.md:36   class: omission
  leaf-text: «Fixtures are synthetic probe text only, scrubbed of absolute paths and usernames; every input embeds the tests-owned canary string.»
  source: .andromeda/test-plan.md:1312 «Fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames; a `workspace.repo` naming this repository's own remote is kept (§7 Fixture hygiene).»
  replace-with: «Fixtures are synthetic probe text only, scrubbed of absolute paths and usernames; a `workspace.repo` naming this repository's own remote is kept (the `StatusLine` variants' `workspace.repo.owner`, neither rewritten by the scrub nor refused by the hygiene walk; nothing else widens); every input embeds the tests-owned canary string.»

- leaf: .claude/rules/verification-harness.md:28   class: stale
  leaf-text: «(print probe + all four interactive runs, Run B with its four added pastes, eighteen rows)»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8 «stamp every relied ledger row (`LedgerRow::relied()`: all twenty-one on Unix, the first eighteen elsewhere), Run B also carrying on Unix its one probe `--settings` (`typed/statusline.json`)»
  replace-with: «(print probe + all four interactive runs, Run B with its four added pastes and, on Unix, its one probe `--settings`; every relied row, `LedgerRow::relied()`: twenty-one on Unix, the first eighteen elsewhere)»

- leaf: .claude/rules/verification-harness.md:42   class: omission
  leaf-text: «`--framing` (verify's Run B: a compiled paste text fires its recorded `paste-<n>` variant unchanged, the probed local command its recorded `clear-<n>` SessionEnd then SessionStart)»
  source: .andromeda/test-plan.md:1077 «it replays the set's `StatusLine.default-1.json` after the launch SessionStart's hooks and `StatusLine.rate-limits-1.json` once after the first Stop»
  replace-with: «`--framing` (verify's Run B: a compiled paste text fires its recorded `paste-<n>` variant unchanged, the probed local command its recorded `clear-<n>` SessionEnd then SessionStart; with an absolute `--settings` `statusLine.command` and no `--statusline-stdin`, the set's `StatusLine.default-1.json` after the launch SessionStart's hooks and `StatusLine.rate-limits-1.json` once after the first Stop, each a `statusline` receipt with `stdin_hex`, the command spawned directly)»

- leaf: .claude/rules/verification-harness.md:42   class: stale
  leaf-text: «run only with both and only when absolute, with the stdin file's bytes; a `statusline` receipt)»
  source: .andromeda/test-plan.md:1094 «and under `--framing` without `--statusline-stdin` with the set's recorded `StatusLine` variants»
  replace-with: «run with both only when absolute, with the stdin file's bytes, and under `--framing` without `--statusline-stdin` with the set's recorded `StatusLine` variants; a `statusline` receipt)»

- leaf: .claude/rules/verification-harness.md:46   class: omission
  leaf-text: «the spine, the dialog variants and the framing variants (`paste-<n>`, `clear-<n>`), which `contract_fake_agent_drift` replays byte for byte (contract suite).»
  source: .andromeda/test-plan.md:438 «and compares the `statusline` receipts' `stdin_hex` byte for byte, in order and count, with each recorded set's `StatusLine.default-1.json` then `StatusLine.rate-limits-1.json` under `--framing`»
  replace-with: «the spine, the dialog variants, the framing variants (`paste-<n>`, `clear-<n>`) and the status line variants (`StatusLine.default-1.json`, `StatusLine.rate-limits-1.json`), which `contract_fake_agent_drift` replays byte for byte (contract suite; the status line variants by the `statusline` receipts' `stdin_hex`, in order and count, under `--framing`, Unix).»
