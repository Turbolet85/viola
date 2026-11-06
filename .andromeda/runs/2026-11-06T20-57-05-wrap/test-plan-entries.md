@amendment marker=2026-11-06-version-done-check
title: the portable-pty pin at =0.9.0
section: §1 Test Scope Summary (the `viola-pty` entity); §5 Integration Test Strategy (integration-test-strategy.md: the Module ↔ PTY tools line); §6 E2E Test Strategy (e2e-test-strategy.md: the tui (`viola run` passthrough) driver row); §3 the 5-command implementation contract (boot step 5)
change:
  - The `viola-pty` entity reads «the seam over portable-pty `=0.9.0` (spawn, read, write, resize, wait, kill)» (was `=0.8.1`), exit still detected on the process handle and never on EOF.
  - The Module ↔ PTY tools line reads «portable-pty `=0.9.0` via viola-pty seam» (was `=0.8.1`).
  - The tui driver reads «portable-pty `=0.9.0` outer PTY» (was `=0.8.1`), the rest of the row unchanged.
  - Boot step 5's supervisor owns one portable-pty (`=0.9.0`, via the `viola-pty` seam) outer PTY per instance (was `=0.8.1`), the rest of the step unchanged.
why: the chunk moved `portable-pty` `=0.8.1` → `=0.9.0` in the workspace dependencies and retired the `=0.8.1` pin with its Windows rationale; viola-pty runs 33 of 33 and the outer-PTY e2e binaries are green on `=0.9.0`, `pty_backend()` still `openpty`.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the retired Windows host and target named as retired
section: §1 Test Scope Summary (the inherited credentials strip entity; test-scope-summary.md: the tui surface's Notes); §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Process faults row); §10 Quality Gates & Coverage Targets (the coverage exclusion, the perf Status); §11 Test Anti-Patterns (the CI ban on widening the coverage regex)
change:
  - The 11-name identity floor reads «measured on the retired Windows host» (was «the Windows host»).
  - The tui surface's Notes trace Must-Work (R2, R3/S1) (was R2, R3/S1, Windows ConPTY); the no-screen-parse and readiness-gate sentences unchanged.
  - The Process faults row's `Frozen` guard reads «a `SIGSTOP`» (was «on Unix a `SIGSTOP`»).
  - The coverage exclusion reads «its separator class is kept from the retired Windows target, where llvm-cov reported paths with `\`» (was «llvm-cov reports Windows paths with `\`, so the forward-slash form never matched there»).
  - The perf Status's host reading at chunk 2026-09-28-hook-perf-gate reads «the retired Windows dev host, release build» (was «Windows dev host»), each row's `max` 72.8–73.7 ms unchanged.
  - The CI ban reads «The separator class, kept from the retired Windows target, is not a widening» (was «The separator class that makes it match Windows paths»).
why: the product and its tests are Linux only; the chunk retired the Windows remnants and named test-plan's Windows wording among the sites to move, and its harness changes are comments only, the coverage regex's four trees byte-identical.
kept: the `--ignore-filename-regex` `(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])` and its separator class stand unchanged; the Windows readings stand as measured on the retired host.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the CLI tests over std::process::Command, the strategy and scope
section: §1 Test Scope Summary (the cli surface, the hook driver; test-scope-summary.md: the cli driver); §2 Test Strategy (the Integration level, the no-human-in-loop bullet; test-strategy.md: the tier-specific depth, the human-mode literals entry, the Hook stdin (V4) row)
change:
  - The cli surface is driven as a subprocess by `std::process::Command` (was `assert_cmd`); the hook driver reads «subprocess (`std::process::Command`)» (was `assert_cmd`); the cli driver reads «subprocess + `std::process::Command` over `CARGO_BIN_EXE_viola`» (was «`assert_cmd` / `Command::cargo_bin("viola")`»).
  - The Integration level and the tier-specific depth read a real tempfile home and `std::process::Command` (against `CARGO_BIN_EXE_viola`) (was assert_cmd).
  - «and trycmd only in verify mode» is gone; insta runs only in check mode (`INSTA_UPDATE=no`).
  - The `trycmd cases:` entry (`tests/cmd/*.toml`) is now «Human-mode literals: in the verbs' own test files (the board in `tests/cli_list.rs`, `viola --help` in `tests/cli_output_plain.rs`)».
  - The Hook stdin (V4) row's level reads «integration (hook subprocess)» (was «assert_cmd hook»).
why: CARRY 1, the trycmd / assert_cmd text re-worded to the literal pins: no manifest holds `assert_cmd`, `predicates`, `trycmd` or `snapbox` and there is no `tests/cmd/` (the plan's lean, re-read at this wrap), so each tool's version number leaves with it.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the CLI tests over std::process::Command, the integration tier
section: §5 Integration Test Strategy (the CLI and hooks tools, the human-mode layouts, the CLI bullet; integration-test-strategy.md: the CLI tools line, the `viola-send` mirror, `tests/cli_verify.rs`, the fixed-strings bullet)
change:
  - The CLI and hooks tools read «`std::process::Command` against `CARGO_BIN_EXE_viola`, its outputs pinned by literal asserts» (was assert_cmd 2.2.2 + predicates 3.1.4); the CLI tools line reads «`std::process::Command` on the `viola` binary» (was assert_cmd 2.2.2).
  - The human-mode layouts are literal asserts in their test files, the `viola list` board in `tests/cli_list.rs` and `viola --help` by the `HELP` literal in `tests/cli_output_plain.rs` (was trycmd 1.2.1 case files, those two excepted); the CLI bullet checks the human layouts by literal (was trycmd or literal).
  - The `viola-send` mirror `[RB]`/`[  ]`/`[/ ]` is pinned by literal in `tests/cli_send.rs` (was trycmd `tests/cmd/*.toml`); the help and board pins no longer read «not by a trycmd case».
  - `tests/cli_verify.rs` pins `viola verify`'s human lines by literal asserts (was literal assert_cmd asserts); the sentence «A trycmd case for the verify step counter arrives with a redaction-heavy consumer» is gone.
  - The snapbox-redactions bullet is now «no manifest holds `assert_cmd`, `predicates`, `trycmd` or `snapbox`, and there is no `tests/cmd/`: each fixed string is a literal in its test file».
why: CARRY 1, the trycmd / assert_cmd text re-worded to the literal pins: none of those tools is in a manifest and `tests/cmd/` is absent (the plan's lean, re-read at this wrap); snapbox's redactions leave with trycmd.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the CLI tests over std::process::Command, the E2E tier
section: §6 E2E Test Strategy (the cli strategies; e2e-test-strategy.md: the cli (verbs + `--json`) and cli (`viola hook` / `hook statusline`) driver rows, Scenario: Path 2, Scenario: E5)
change:
  - The cli (verbs + `--json`) strategy reads «`std::process::Command`, with literals for the `--json` documents and the human layouts» (was assert_cmd with predicates, and trycmd or literals); the hook strategy reads `std::process::Command` with an `Instant` deadline (was assert_cmd).
  - The cli (verbs + `--json`) driver reads «`std::process::Command` on the `viola` binary; each fixed string, the human layouts included, pinned by a literal in its test file» (the board in `tests/cli_list.rs`, `viola --help` in `tests/cli_output_plain.rs`, the link / unlink lines in `tests/cli_links.rs`) (was assert_cmd 2.2.2 + predicates 3.1.4, trycmd 1.2.1 for the other human layouts); the hook driver reads `std::process::Command` with `Instant` deadline (was assert_cmd 2.2.2).
  - Path 2's human mode is verified against literal lines in the test file (was trycmd-redacted assert_cmd); the lines it lists unchanged.
  - E5's hook fail-open matrix in root `hook_fail_open.rs` reads (`std::process::Command`) (was assert_cmd).
why: CARRY 1, the trycmd / assert_cmd text re-worded to the literal pins: none of those tools is in a manifest and `tests/cmd/` is absent (the plan's lean, re-read at this wrap).
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the CLI tests over std::process::Command, fixtures, gates, bans and contracts
section: §7 Test Data & Fixtures (the human-mode expected output; test-data-fixtures.md: its row); §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Process mocking row); §10 Quality Gates & Coverage Targets (the unit-speed deadlines); §11 Test Anti-Patterns (E2E; Universal); §3 the 5-command implementation contract (harness `cleanup`); §3 the bootstrap phases contract (the test tools)
change:
  - The human-mode expected output reads «literals in their test files (`viola --help`, the `viola list` board)» (was trycmd cases with snapbox redactions); its fixtures row drops «trycmd `tests/cmd/*.toml`, with snapbox redactions for timestamps, cursors and paths», its literals and tables unchanged.
  - `viola-e2e` tests and the harness take the binary dir from the test executable's own target dir (was the assert_cmd target-dir lookup); `harness_session` no longer names «the assert_cmd `cargo_bin` lookup».
  - The deadlines are asserted at unit speed inside the hook subprocess tests (was the assert_cmd hook tests).
  - The E2E ban reads «NEVER use `std::process::Command` `.output()` or `.status()` on the long-lived `viola run` wrapper. It blocks until exit; use `.spawn()` + a `Drop` guard (`tests/support/cli.rs` `Running`)» (was `assert_cmd` `.assert()`, std::process + a `Drop` guard); the snapshot-review ban names `cargo insta review` alone (was also trycmd overwrite).
  - The bootstrap test tools drop assert_cmd 2.2.2, predicates 3.1.4 and trycmd 1.2.1; the others unchanged.
why: CARRY 1, the trycmd / assert_cmd text re-worded to the literal pins: none of those tools is in a manifest and `tests/cmd/` is absent (the plan's lean, re-read at this wrap); a rule that names a tool's API reads the same rule over `std::process::Command`.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the leading-slash rule without the rewritten-path warning
section: §1 Test Scope Summary (the CLI and env trigger); §5 Integration Test Strategy (integration-test-strategy.md: the `tests/cli_slash_args.rs` item)
change:
  - The CLI and env bullet reads «a leading-slash argument is never taken as a prompt» (was followed by «(§6 rewritten-path warning)»).
  - The `tests/cli_slash_args.rs` item's head reads «(Vector 6: a leading-slash argument is never taken as a prompt)» (was «(Vector 6, §6 rewritten-path warning)»); its case table unchanged.
why: the chunk retired the Git Bash rewritten-path warning, its emitter, unit table and CLI witness, by the overseer's T2 under the founder's own word of 2026-11-06 19:34Z (`inputs#I2`).
kept: a leading-slash argument is never delivered as a prompt; the `/clear`, `/model x`, `//server/share` cases stand.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the supply-chain advisories with no ignore
section: §1 Test Scope Summary (the supply-chain trigger); §9 CI Integration (ci-integration.md: the supply-chain advisories)
change:
  - The supply-chain trigger reads «`cargo deny check` (no advisory ignored, `[advisories] ignore = []`)» (was «only ignore RUSTSEC-2017-0008»).
  - The recorded supply-chain red is now «The supply-chain advisories»: the `supply-chain` job reads green with no advisory ignored, `deny.toml` `[advisories]` `ignore = []`, and `portable-pty` `=0.9.0` (through `nix 0.28.0`) puts no `pin-utils` in the graph (`cargo tree -i pin-utils` exit 101, no package), as measured at ci#42288148289 (was red on RUSTSEC-2026-0441 through `portable-pty =0.8.1` → `nix 0.25.1` at ci#42030025050, owner the CARRY on «Version done-check»).
why: the chunk's dependency move closed RUSTSEC-2026-0441 and the RUSTSEC-2017-0008 ignore left with the `serial` crate (ignore entries 1 → 0, a narrowing); `cargo deny check` reads advisories ok and CI `supply-chain` is green on `849aad18`.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the per-OS framing at one ubuntu-latest leg
section: §9 CI Integration (ci-integration.md: the Perf and Release build stages' Parallel lines); §10 Quality Gates & Coverage Targets (the `viola hook session-end` perf row, the build failure conditions); §3 the bootstrap phases contract (the coverage thresholds)
change:
  - The Perf stage reads «**Parallel:** one-leg matrix `[ubuntu-latest]` (the job name keeps its `${{ matrix.os }}` form)» (was per OS).
  - The Release build stage reads «**Parallel:** one `ubuntu-latest` leg» (was per OS).
  - The `viola hook session-end` perf row drops «per OS».
  - The build fails on a test failure (any level) (was any level, any OS) and on coverage below threshold for tier (the `test` job on `ubuntu-latest`) (was per OS).
  - The bootstrap coverage thresholds read lines 85 / functions 95 / regions 80 (was per OS).
why: the product is Linux only and CI reads one `ubuntu-latest` leg: ci#42288148289's seven jobs green, artifacts `junit-ubuntu-latest` and `perf-ubuntu-latest`; the chunk named test-plan's per-OS wording among the sites to move.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the child's fd inheritance on 0.9.0
section: §6 E2E Test Strategy (e2e-test-strategy.md: Scenario: E2 — R8 `CLAUDE*` env strip)
change:
  - The mechanism reads «portable-pty closes every inherited descriptor in the child (`close_random_fds()`, every fd above 2 at spawn)» (was «portable-pty's `close_random_fds()` closes every fd above 2 at spawn»).
  - Added: as measured on live `claude` 2.1.287 on the Linux dev host at chunk 2026-11-06-version-done-check (a local live measurement, no suite), the wrapped child's `/proc/<child>/fd` held 31 fds (pty 7, socket 14, other 10), fds 0–2 on the pty, no socket that is viola's endpoint or the wrapper's, and no file under the home.
why: the chunk's live round read session 2's child fd table on `=0.9.0`; the mechanism's wording is the overseer's wrap directive item 4 (the operator's request, founder-delegated), not the founder's own word.
kept: the count «0/1/2 plus the PTY slave» does not hold (the Linux child reads `[0,1,2,3,4]`), so a planted leak cannot go red either way.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the Windows test sites retired
section: §6 E2E Test Strategy (e2e-test-strategy.md: Scenario: E5, the single writers); §5 Integration Test Strategy (integration-test-strategy.md: `tests/cli_output_plain.rs`, the home resolution item)
change:
  - The single-writers sub-line naming the two `CreateNamedPipeW` controls (`security_negative_control_accept_remote_pipe_reads_the_bit_clear`, `security_negative_control_reject_remote_pipe_reads_the_bit_set`) is gone; the strict-modes case now stands in its own «As landed:» sub-line.
  - `help_lists_only_served_verbs` reads its usage program as `viola` (was `viola` or `viola.exe`).
  - The home resolution is backed by the `src/cmd/mod.rs` units `resolve_home_*` and `canonical_home_*` (was also `plain_drive_*`).
why: the chunk removed `plain_drive` and its rstest table, every `EXE_SUFFIX` join and the `.exe` trim, and named the named-pipe sub-line among test-plan's Windows wording to move; the tests are Linux only.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the planted annotation drift witness
section: §6 E2E Test Strategy (e2e-test-strategy.md: Scenario: E5, a new bullet); §2 Test Strategy (test-strategy.md: the Contract level's replay sentence)
change:
  - Added «The planted annotation drift»: `dialog_drift_names_a_changed_annotation` (`tests/contract_fake_agent_drift.rs`) plants a change to a forwarded `annotations` byte of `synthetic_dialog_set` and reads `dialog_drift` red.
  - The Contract level's dialog replay names that test as its synthetic witness.
why: the chunk added `tests/contract_fake_agent_drift.rs::dialog_drift_names_a_changed_annotation`, placed at tests e2e (the contract binary).
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the always-admit survivor as 0.1.0's known limit
section: §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the mutation gate's survivor); §2 Test Strategy (test-strategy.md: the Mutation level)
change:
  - The survivor paragraph reads «That survivor stands as version 0.1.0's known limit, read at each epoch-boundary audit (matrix#v1-44)» (was «The mutation audit reads that same survivor at the route entry "Version done-check"»).
  - The Mutation level's survivor «stands as the version's known limit, read at the epoch-boundary audit (matrix#v1-44)» (was read at the route entry "Version done-check").
why: the chunk's expected amendment 14 (CARRY 8): matrix#v1-44's always-admit survivor stands as the version's known limit, read at the epoch-boundary audit, its owner no longer this entry.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the statusline-rate-limits arm's live red
section: §4 Unit Test Strategy (unit-test-strategy.md: the ledger's three statusline arms)
change:
  - Added: on an unchanged `claude` 2.1.287 a live `viola verify` reads `statusline-rate-limits` red (`23 pass  1 fail`) where the account's status-line payload carries `five_hour` `unknown` and only `seven_day`; the row reads the account's plan state, not the CLI; owner 0.2.0 (finding F-06i).
why: the chunk's live round disproved the row as a CLI behaviour relied on every build; the founder's own word of 2026-11-06 20:53Z deferred v1-27 and asked why viola leans on the user's status line; the overseer's wrap directive (items 1 and 3, founder-delegated) routes that question and the row's reading to 0.2.0's intent as finding F-06i — the routing is the directive's, not the founder's word.
kept: the arm and its nine `check_statusline_*` cases stand; the ledger stays 24 rows.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
