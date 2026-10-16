group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 96 · «- 2026-10-16: An oracle literal for a list or a count the pl»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «22 on Unix and 19 on Windows»
  source: .andromeda/test-plan.md:505 «the verdict `verified` reads `LedgerRow::relied()`: 23 rows on Unix, 20 on Windows, `agents-join` appended last and relied on every OS»
  replace-with: «23 on Unix and 20 on Windows»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «beside the options one mode word, `statusline-echo`)»
  source: .andromeda/test-plan.md:954 «`agents` (chunk 2026-10-16-unwrapped-sessions-on-the-board): a first word `agents` answers `agents --json` from files beside its exe (`agents-mode`, `agents-rows.json`, and the live `sessions/<pid>.json` registry»
  replace-with: «beside the options two mode words, `statusline-echo` and `agents`, a first word `agents` answering `agents --json` from files beside its exe (`agents-mode`, `agents-rows.json` and the live `sessions/<pid>.json` registry), built on the recorded `Agents.recorded-1.json`, with no option and no env added)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «five framing variants, two status line variants and one resume variant in 2.1.287»
  source: .andromeda/test-plan.md:928 «and Run B's one `claude agents --json` read as `Agents.recorded-1.json` (Run B's own row only, validated against `schemas/claude-agents-fixture.v1.json`; one today, under `2.1.287` only»
  replace-with: «five framing variants, two status line variants, one resume variant and one agents fixture (`Agents.recorded-1.json`, Run B's own row of its one `claude agents --json` read) in 2.1.287»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the `StatusLine.*.json` status line variants against `schemas/claude-statusline-fixture.v1.json`,»
  source: .andromeda/test-plan.md:379 «every `fixtures/claude/*/Agents.*.json` agents fixture to `schemas/claude-agents-fixture.v1.json`»
  replace-with: «the `StatusLine.*.json` status line variants against `schemas/claude-statusline-fixture.v1.json`, the `Agents.*.json` agents fixtures against `schemas/claude-agents-fixture.v1.json` (`agents_fixtures_pass_hygiene`),»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and replays the resume variant (`fake_agent_resume_replay_matches_every_recorded_resume_set`)»
  source: .andromeda/test-plan.md:379 «`contract_fake_agent_drift` also compares the key and type shape of the fake agent's `agents` rows with each recorded `Agents.*.json` set (`fake_agent_agents_replay_matches_every_recorded_agents_set`»
  replace-with: «and replays the resume variant (`fake_agent_resume_replay_matches_every_recorded_resume_set`), and compares the key and type shape of the fake agent's `agents` rows with each recorded `Agents.*.json` set (`fake_agent_agents_replay_matches_every_recorded_agents_set`)»

- leaf: .claude/docs/tests-summary.md:32   class: omission
  leaf-text: «unwrapped hooks no-op · R8 env strip»
  source: .andromeda/test-plan.md:789 «A `send` to the unwrapped row's name exits 21: a row `name` is never a target. As landed: `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable` (exit 21; the unwrapped-name hint not built»
  replace-with: «unwrapped hooks no-op (a `send` to an unwrapped row's name exits 21, as landed `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable`; the unwrapped-name hint not built, owed to "Exit-cause code catalogue") · R8 env strip»

- leaf: .claude/rules/testing.md:19   class: omission
  leaf-text: «the started-child guard and runner in `cli.rs`,»
  source: .andromeda/test-plan.md:397 «and `spawn_on` / `viola_on`, which put a planted bin dir on the child's `PATH`; a plain `spawn` of `list` runs with `PATH` set to an empty bin dir, so no test reads the host's `claude` sessions»
  replace-with: «the started-child guard and runner in `cli.rs` (with `spawn_on` / `viola_on`, which put a planted bin dir on the child's `PATH`; a plain `spawn` of `list` runs with `PATH` set to an empty bin dir, so no test reads the host's `claude` sessions), `empty_bin`, `claude_bin` and `claude_in` in `fake.rs`, the one place a test plants the fake agent as `claude`,»

- leaf: .claude/rules/verification-harness.md:28   class: stale
  leaf-text: «every relied row, `LedgerRow::relied()`: twenty-two on Unix, elsewhere nineteen, the first eighteen and `resume-session`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8 «stamp every relied ledger row (`LedgerRow::relied()`: all twenty-three on Unix, elsewhere twenty, the first eighteen, `resume-session` and `agents-join`)»
  replace-with: «every relied row, `LedgerRow::relied()`: twenty-three on Unix, elsewhere twenty, the first eighteen, `resume-session` and `agents-join`»

- leaf: .claude/rules/verification-harness.md:28   class: omission
  leaf-text: «Run B with its four added pastes and, on Unix, its one probe `--settings`,»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:8 «Run B also making its one `claude agents --json` read (chunk 2026-10-16-unwrapped-sessions-on-the-board) and carrying on Unix its one probe `--settings` (`typed/statusline.json`)»
  replace-with: «Run B with its four added pastes, its one `claude agents --json` read and, on Unix, its one probe `--settings`,»

- leaf: .claude/rules/verification-harness.md:41   class: retired
  leaf-text: «`agents --json` lands with its consumer.»
  source: .andromeda/test-plan.md:954 «`agents` (chunk 2026-10-16-unwrapped-sessions-on-the-board): a first word `agents` answers `agents --json` from files beside its exe (`agents-mode`, `agents-rows.json`, and the live `sessions/<pid>.json` registry»
  replace-with: «Built: the mode word `agents` (a first word `agents` answers `agents --json` from files beside its exe: `agents-mode` (`recorded` · `oversize` · `malformed`, absent = `recorded`), `agents-rows.json` (planted rows) and the live `sessions/<pid>.json` registry, a row written just before its launch SessionStart fires, a dead or reused pid skipped through one `viola_state::liveness::start_times()` read; built on the recorded `Agents.recorded-1.json`; no option and no env added).»

- leaf: .claude/rules/verification-harness.md:45   class: omission
  leaf-text: «under `--framing`, Unix).»
  source: .andromeda/test-plan.md:379 «`contract_fake_agent_drift` also compares the key and type shape of the fake agent's `agents` rows with each recorded `Agents.*.json` set (`fake_agent_agents_replay_matches_every_recorded_agents_set`»
  replace-with: «under `--framing`, Unix); it also compares the key and type shape of the fake agent's `agents` rows with each recorded `Agents.*.json` set (`fake_agent_agents_replay_matches_every_recorded_agents_set`).»
