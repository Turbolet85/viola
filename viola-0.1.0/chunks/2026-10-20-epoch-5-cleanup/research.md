# Codebase Research — 2026-10-20-epoch-5-cleanup

## Scope
- **Depth:** deep on items (1), (2), (3), (6), (8), (9), (10) — the founder's partition (inputs#I2); (4), (5), (7) read
  only to size them, they go to "Epoch 5 cleanup II" · **Reads:** 52 · **Globs/Greps:** 29
- **Harness rules consulted:** `.claude/rules/testing.md` — read in full (auto-loaded), 52 Session Additions; applied:
  2026-09-24 (the kill line below cargo-mutants' floor), 2026-09-25 (inline test modules; the remove-the-guard run,
  extended 2026-10-20 to a scratch clone), 2026-09-28 (a timing red is never fixed by raising a bound; the 2026-10-05
  by-design-floor exception and its planted-hang pair), 2026-10-06 (stamped-home cases sized against the 10 s kill),
  2026-10-07 (an uninstrumented child) · `.claude/rules/verification-harness.md` — read in full (auto-loaded), 17
  Session Additions; applied: the `run --mutants` build layout (`target/mutants`, `NEXTEST_PROFILE=mutants`, the
  keep flags at 0), the fake agent's hook quiescing at `\x03` / EOF, 2026-10-03/04 (a short `TMP`, the NOCOW
  `TMPDIR`) · `.claude/rules/host-linux.md` (always loaded) — `TMPDIR` unset, `hyprctl locked`, the census key
- **Platform issues consulted:** item (3)'s runner readings (run 39326811932) searched at P5 (validation check 9):
  `cargo-mutants Windows unviable every mutant after test leaves child process running exe locked` → no report of
  the pattern; the nearest fetched-by-search sources are cargo-mutants' wiki "Trials" page
  (https://github.com/sourcefrog/cargo-mutants/wiki/Trials: child-process termination mutants ending as timeouts, and
  no notion of mutating only on platforms that can test them) · `github actions windows runner "LNK1104" cannot open file
  exe running test process cargo` → rust-lang/rust#127883 (https://github.com/rust-lang/rust/issues/127883:
  intermittent "Access is denied" / "used by another process" / "cannot open file" on MSVC Windows builders, cause not
  found) and users.rust-lang.org/t/link-errors-on-windows-with-vs-code/107169 (LNK1104 from another process holding the
  output — antivirus or a still-running test binary). None names the dispatch's shape; step 7's own mechanism (a test
  binary left running) is the class the second source names
- **External inputs:** `inputs#I1` — the overseer's phase directive (no mutation, witness or dispatch run; size
  against one window; items (8) and (10) the founder's word; `df -h /tmp` before stamped-home gates; the batch 29
  letters) · `inputs#I2` — the founder's own answer to the P4 split fork, «Два чанка»: this chunk (1)(2)(3)(6)(8)(9)(10),
  "Epoch 5 cleanup II" (4)(5)(7) directly after it, the wrap writing the insertion

## Files inspected
- `.config/nextest.toml` (full) — `[profile.mutants]` 10 s default; overrides first-match: `verify_window_` 45 s
  (`15s` × 3) · `package(viola-e2e)` 30 s · `send_window_` 30 s; `[profile.ci]` the 21-binary 20 s line and
  `verify_window_` 60 s
- `tests/contract_lints.rs` (128–130, 242–301) — `test_deadlines_sit_below_the_nextest_kill_line` reads the mutants
  default and the `package(viola-e2e)` kill; `mutants_verify_window_override_stands_before_the_harness_package_override`
  asserts the class first and its kill above the package's
- `crates/viola-e2e/src/harness/run/mutants.rs` (140–262) — the boundary form runs `cargo mutants --package <m>
  --features fake-agent --test-tool=nextest --copy-target=true` with `NEXTEST_PROFILE=mutants`,
  `AGENT_RUN_KEEP_HOMES=0`, `AGENT_RUN_KEEP_FAILED=0`, `CARGO_TARGET_DIR=target/mutants`, after `cargo build --package
  viola --features fake-agent` into `target/mutants`; on Windows `TMP` / `TEMP` / `--output` the scratch
- `tests/contract_fake_agent_drift.rs` (29, 109, 200–300) · `tests/contract_ledger_probes.rs` (75–95) — every replay
  case walks `recorded_versions` in sequence, one whole `viola verify` per set (`:221`, `:315`, `:425`, `:491`, `:541`,
  `:642`; probes `:91`)
- `tests/cli_send.rs` (930–1060) — the paste-hint pair, `--paste-hint-ms 3000` plus the gate's wait (`GATE_MAX_WAIT`
  8.5 s, test-plan history 2026-10-07-send-waits-out-the-paste-hint)
- `tests/cli_verify.rs` (1251, 1300–1320) — the no-screen `verify_window_` case; the dialog-less literal oracle
- `tests/cli_budget.rs` (180–260) — `path6_the_boundaries_and_the_window_word`, three sends in one session
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (1–70, 240–300) — `prepare` / `wipe` lay the scratch down with
  `viola_state::fs::create_private_dir`; the `#[cfg(windows)]` case (`:258`) makes its plain control with
  `fs::create_dir_all` under `target/e2e-home` and asserts it `Err(Writable)` — a refusal it inherits from its parent
- `crates/viola-pty/src/lib.rs` (15, 144–233, 825–845, 901, 1133, 1295) — the tests spawn `std::env::current_exe()`
  (the test binary) as their PTY child; the guard `impl Drop for Child` (`:833`) kills it only when `try_wait()` reads
  `Ok(None)`; `tempfile` is a dev-dependency (`crates/viola-pty/Cargo.toml:27`)
- `tests/support/home.rs` (115–160, 380–425) · `tests/support/verify.rs` (440–466) — `prepare_home_base`,
  `dialogless_home` (literals `20` / `17`)
- `crates/viola-agent-claude/src/ledger.rs` (21–75, 138–165) — `LedgerRow`, `ALL` (24), `relied()`; the four dialog
  rows `QuestionAnswer`, `PlanApproveRevise`, `QuestionNotes`, `DialogConcurrency` (positions 10–13)
- `src/bin/viola-fake-agent.rs` (600–650, 1000–1020) — `run_hook` spawns each hook in the fake agent's group (`:631`);
  `hold_inherited_stdout` takes `process_group(0)` against the session leader's SIGHUP (`:1018`)
- `tests/chaos_wait_vanish.rs` (46–80) · `scripts/profraw-census.sh` (1–40) ·
  `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/profraw-red.md` (full)
- `.andromeda/runs/2026-10-19T21-08-30-code-audit/proposals.md` (full) · `.andromeda/runs/2026-10-19T19-16-36-wrap/relay-3.md`
- `.claude/rules/{frontend,a11y,api,testing}.md` (frontmatter) · `.claude/docs/services/viola-ui.md` · `CLAUDE.md`
  (markers) · `.andromeda/design-system.md:16,21,41,45` · `.andromeda/a11y-plan.md:10–30` · `.andromeda/obs-plan.md:647`
  · the two log-format key files
- Run 39383206773 (`0807ee3c`, green), its `junit-{windows-2025,ubuntu-latest,macos-latest}` artifacts, read in the
  session scratchpad by `junit_times.py` — the `ci` profile's per-case times
- Run 39326811932 (the Windows mutants dispatch), job 118314687311 `mutants (viola-pty)`'s log, read in the session
  scratchpad (`gh api --allow-escape-sequences repos/Turbolet85/viola/actions/jobs/118314687311/logs`)

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-20T07-58-07-phase/tree-query-2026-10-20-epoch-5-cleanup.json`)
- **dialogless_home** — 1 caller: `path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human` @
  `tests/cli_answer.rs:793`. A derived summary changes no caller.
- **prepare_home_base** — `TestHome::new` @ `tests/support/home.rs:156`, and `tests/cli_instance_state.rs` (`:22` the
  module, `:586`, `:589`, `:596`, `:629`, `:653`). A headroom refusal there reaches every root test that makes a home.
- **ensure_e2e_home** — `harness::boot::start` @ `crates/viola-e2e/src/harness/boot.rs:164`, `harness::run::local_live`
  @ `crates/viola-e2e/src/harness/run.rs:391`, `mcp_tools_a_widened_home_exits_1_and_prints_nothing` @
  `crates/viola-e2e/tests/mcp_tools.rs:403`, and four `harness/mod.rs` / `cleanup.rs` units; a refusal there is boot's
  closed `build-failed`.
- **run_hook** / **hook_commands** — `Agent::fire_answered` @ `src/bin/viola-fake-agent.rs:534` / `:521`, `:530`, and
  the bin's own unit cases: a process-group change at the spawn reaches every hook the fake agent fires.
- The replay helpers (`recorded_versions`, `verify_receipt`) and the `viola-pty` test guard are test-local: no
  product caller (a name grep, `## Sweeps`).

## Patterns detected
- **A slow class under its own kill line** (`.config/nextest.toml`, `send_window_` / `verify_window_`): a by-design
  wait gets a named override, proved by a planted-hang control both ways (testing.md 2026-10-05; tests history
  2026-10-09-epoch-3-cleanup: TIMEOUT at 45.005 s under `mutants`, PASS under the default profile).
- **Remove the slow work** (testing.md 2026-09-28): a cost that grows with the recorded sets is the work's, not a
  floor — the replay contracts walk sets one after another.
- **Leave the session leader's group** (`src/bin/viola-fake-agent.rs:1018`).
- **A test-only sibling module** (`src/run/send/relabel.rs:1–5`).
- **Keepers refuse with a fixed message naming no path** (`tests/support/home.rs:121–136`).
- **A literal oracle beside a fixture precondition** (`tests/cli_verify.rs:1315` vs `tests/support/home.rs:410`).

## Conventions to follow
- **Oracles are literals in the test** (testing.md §What to assert; test-plan §11 Unit).
- **No env var, flag or config key** for a floor or a kill (security.md §Dependencies and CI).
- **No test-side deadline at or past its kill line** (`test_deadlines_sit_below_the_nextest_kill_line`).
- **Removals:** a tracked file the permission layer refuses to remove is the founder's at the desk (host-linux.md
  2026-10-07, extended 2026-10-16).
- **Masters are the wrap's**: item (10)'s master bodies and `CLAUDE.md`'s `GENERATED:setup:deeper-topics` block, and
  the profile, kill and class facts test-plan §3 / §10 hold, are Expected amendments.

## New files to create
- `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/` — the baseline readings, the planted-hang pairs, the item
  (8) two-sided reading, the item (9) floor's measurement, the remove-the-guard readings, the operator pass

## Files to modify
- `.config/nextest.toml` — the paste-hint class's `mutants` override (item 1); `verify_window_`'s `mutants` kill (item 2)
- `tests/contract_lints.rs` — the pins of the new override and the moved kill (items 1, 2)
- `tests/contract_fake_agent_drift.rs` — the replay walks run their sets concurrently (item 1)
- `tests/contract_ledger_probes.rs` — the stamped-set walk runs its sets concurrently (item 1)
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` — the Windows case's plain control made, not inherited (item 3)
- `crates/viola-pty/src/lib.rs` — the tests' PTY child a per-test copy and the guard's kill unconditional (item 3)
- `tests/support/home.rs` — `dialogless_home` derived (item 6); the headroom read in `prepare_home_base` (item 9)
- `tests/cli_instance_state.rs` — the backing cases gain the headroom refusal case (item 9)
- `crates/viola-e2e/src/harness/mod.rs` — the same headroom read in `Workspace::ensure_e2e_home` (item 9)
- `crates/viola-e2e/Cargo.toml` — a `cfg(unix)` `libc` (workspace pin `=0.2.189`) for `statvfs` (item 9)
- `Cargo.lock` — the `viola-e2e` → `libc` edge (item 9)
- `src/bin/viola-fake-agent.rs` — the hook spawn's process group, if item (8)'s reading names the hangup (item 8)
- `scripts/profraw-census.sh` — a census over a kill-shaped case, if item (8)'s reading needs one (item 8)
- `.claude/rules/frontend.md` — `paths:` without `crates/viola-ui/assets/**`; the web-spa stub section out (item 10)
- `.claude/rules/a11y.md` — `paths:` onto the terminal surfaces its body governs (item 10)
- `.claude/rules/api.md` — `paths:` without `crates/viola-ui/src/**` (item 10)
- `.claude/rules/testing.md` — `paths:` without `e2e-web/tests/**` (item 10)
- `.claude/docs/services/viola-ui.md` — removed (item 10)

## Sweeps
- `dialogless_home` over `tests/`, `crates/`, `src/`: 2 hits · 1 changed (`tests/support/home.rs:405`) · 1 no-change
  (`tests/cli_answer.rs:793`, a caller) — `grep -rn 'dialogless_home'`
- `pass  4 fail` over `tests/`, `crates/viola-e2e/src`: 2 hits · 0 changed · 2 no-change (`tests/cli_verify.rs:1315`,
  `:1317`, the literal oracle) — `grep -rn 'pass  4 fail'`
- `for version in` over `tests/contract_fake_agent_drift.rs`, `tests/contract_ledger_probes.rs`: 7 hits · 7 changed
  (each sequential set walk) · 0 no-change — `grep -n 'for version in'`
- `viola-ui|e2e-web|viola_ui` over `.claude/` (`*.md`, the handoff excluded): 13 hits · 11 changed · 2 no-change
  (`.claude/docs/commands.md:5`, `:16`, history) — `grep -rn -E 'viola-ui|e2e-web|viola_ui' .claude/ --include=*.md`
- `reserved` over `.andromeda/obs-plan.md` and the two log-format key files: 4 hits in `obs-plan.md` (`:86`, `:195`,
  `:770`, `:1043`, none the log-format copy) · 0 in either key file — `grep -n -i 'reserved'`
- `cfg(windows)` over `src/cmd/verify/typed.rs`, `src/human/board.rs`, `crates/viola-agent-claude/src/ledger.rs`: 0 ·
  0 · 1 — `grep -c 'cfg(windows)'` (item (5)'s, for "Epoch 5 cleanup II")

## The CI and dispatch readings (items 1–3)
- **Run 39383206773 JUnit (`ci` profile, `0807ee3c`), seconds, windows-2025 / ubuntu / macOS** — the cases at or past
  the `mutants` profile's 10 s kill, or within a second of it:
  - `fake_agent_dialog_replay_matches_every_recorded_dialog_set` 15.027 / 14.097 / 16.962;
  - `contract_ledger_probes_pass_over_every_stamped_set` 10.146 / 9.233 / 11.634;
  - `fake_agent_framing_replay_matches_every_recorded_framing_set` 9.847 / 9.373 / 11.546;
  - `fake_agent_statusline_replay_matches_every_recorded_statusline_set` — / 9.266 / 11.348 (Unix only);
  - `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` 8.802 / 8.755 / 9.857;
  - `send_under_the_paste_hint_on_a_verified_cli::case_1_hint` 8.866 / 8.715 / 9.555;
  - `verify_window_without_screens_fails_every_interactive_row` 42.874 / 43.078 / 43.714 (against 45 s);
  - `path6_the_boundaries_and_the_window_word` 5.502 / 6.560 / 5.964;
  - `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not` 0.019 (windows, passing).
  The audit's root baseline (22:51Z, this host) timed out three of them; the JUnit shows four more within reach.
- **The dialog sets at HEAD:** three carry dialogs (`2.1.287` 11 dialog-hook files, `2.1.288` 7, `2.1.302` 7; `ls
  fixtures/claude/<v> | grep -c -E '^(PreToolUse|PermissionRequest)\.'`); `2.1.302` landed at `4eaca1d3`, after the
  audit's `b6ddc198`.
- **Run 39326811932, `mutants (viola-pty)`:** baseline `ok` (11 s build + 3 s test); 11 caught, then from 21:27:00
  every mutant `unviable` in a 0–4 s build — `kill -> Ok(())`, `close with ()`, `child_pid` included, replacements
  that compile — right after the three `try_wait -> Ok(Some(_))` / `Ok(None)` mutants were caught (log lines
  439–442). A build failing for every mutant from one point on is the recorded leaked-process class (tests history,
  2026-09-24-epoch-1-cleanup).
- **`mutants (viola-e2e)` / `(viola-cmd-verify-stop)`:** their baselines failed on the scratch case and on
  `path6_the_boundaries…` at 10 s (the audit's dispatch table); CI's own windows-2025 test leg reads the first green
  in 0.019 s and the second at 5.502 s.

## Mechanisms, each the equality the plan needs
- **(1) the replay walks:** each case's wall is the sum of one `viola verify` per recorded set; run concurrently it is
  the slowest set's, its assertions unchanged. Verified for the summing half: the walks are sequential loops
  (`## Sweeps`); the concurrent wall is implement's measurement.
- **(1) the paste-hint pair:** each waits the product's gate (`GATE_MAX_WAIT` 8.5 s) by design — the by-design floor
  class `send_window_` already holds (`.config/nextest.toml`, the `send_window_` comment).
- **(2):** the case waits the gate's maximum four times plus Run E by design (42.9–43.7 s on three OSes, `## The CI and
  dispatch readings`); the class's `mutants` kill sits 1.3–2.1 s above it, under the 60 s it has under `ci`.
- **(3a):** under cargo-mutants the tree is copied into the scratch, laid down owner-only; the case's `target/e2e-home`
  is then inside it, so its plain control inherits owner-only and reads `Ok(())`. In CI's checkout it inherits the
  workspace drive's `BUILTIN\Users` write grant and reads `Err(Writable)` (0.019 s green). The equality the fix needs:
  a plain control that reads `Err(Writable)` whatever its parent's DACL. [hypothesis: the copy location is read from
  `mutants.rs:255–262`'s `TMP` / `TEMP`; the reading inside the copy is the next boundary's]
- **(3c):** a `try_wait` mutant that reports the child gone makes the guard skip its kill (`:834–836`); the child is
  the test binary itself (`:901`, `:1295`), which then stays running, and on Windows a running image cannot be
  replaced, so every later mutant's link of that test binary fails — `unviable`. The equality the fix needs: no
  mutant can leave the build's own test binary running. [hypothesis: the lock is read from the log's shape, not from
  a recorded link error — the job log names no error line]
- **(3b):** no local mechanism: CI's windows-2025 leg reads 5.502 s under `ci`, the dispatch's baseline 10 s under
  `mutants`; the difference is not measured, and measuring it is a dispatch.
- **(8):** see scope item (8) — research's hypothesis, a hangup reaching an in-flight hook; the census script reads one
  fixed unit test, not a kill-shaped case.
- **(9):** `statvfs` reads the filesystem (14 134 MiB free now); the per-user quota is unreadable from the session
  shell (`quotactl_fd` → EPERM); EDQUOT struck at about 8 G free by `df` (24 G used of 32 G); a full root run pins up to
  32 × 101 163 992 B of `viola` copies at once.

## Open questions
- Item (9)'s floor: set at implement from a fresh `statvfs` reading against the recorded EDQUOT reading and the
  footprint, with its basis written beside the constant → blocks: implementation-scope
- Item (8)'s fix: the fake agent's hook spawn changes only if the two-sided reading names the hangup; another cause
  names another site, recorded before any change → blocks: implementation-scope
