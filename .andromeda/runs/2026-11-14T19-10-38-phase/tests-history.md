@history_item n=1 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (Path 6 and Path 1 as landed) — named the statusline source (`<home>/statusline-source.json`, else the default home's `.claude/settings.json`), landed the pass-through as root tests (`tests/hook_statusline.rs` cases, `path6_wrapped_statusline_prints_the_user_output_unchanged` in `tests/cli_instance_state.rs`) and the `settings.json` override rewrite on each start (`run_rewrites_the_settings_override_each_start`, unit `statusline_override_of_a_start_is_rewritten_whole_at_owner_only`); this is the origin of contract/statusline-settings-source/source-file, contract/viola-run-start/settings-override, contract/instance-settings/statusline-override and contract/hook-statusline/passthrough, the rows and cases this chunk retires.

@history_item n=2 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (fake agent options, property, seeds, kill line) — added the fake agent's `--settings` / `--statusline-stdin` options, the `statusline-echo` mode word and the `statusline` receipt kind; the statusline reader's property with `proptest-regressions/statusline.txt`; `hook_stdin` seeds 10 → 12 (rule `ls fuzz/corpus/hook_stdin | wc -l`); the `plant_statusline_source` seed row (decision/statusline-source-written-by-test, decision/statusline-source-read-only); `hook_statusline` joined the 20 s verify-driven kill line. Every one of these registers moves back when the interception leaves (option count, mode words, receipt kinds, seed count, kill-line names).

@history_item n=3 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — brought in the relied set (`LedgerRow::relied()`), the three statusline arms with nine `check_statusline_*` cases, Run B's one probe `--settings`, the `StatusLine` variants replayed by the drift contract under `--framing` through an absolute `--settings` command, `schemas/claude-statusline-fixture.v1.json`, and the kept `workspace.repo` exception in fixture hygiene / recorder scrub; the founder's own answer for the probe `--settings`. This chunk undoes the `statusline-override` / `statusline-shell` arms, the probe, the StatusLine fixtures and schema; the `workspace.repo` exception (decision/fixture-hygiene-walk, decision/recorder-scrub, decision/no-production-data-in-tests) exists only for StatusLine variants and needs a decision.

@history_item n=4 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the last full statement of the relied-row literals: `[NN/24]`, `STEPS_PASS` / `ROW_IDS` / `LEDGER_ROWS` `[&str; 24]`, the no-framing and tag-turn summaries (`19 pass  5 fail`, `21 pass  3 fail`) and the older-stamp oracle; the register contract/verify-output/step-line and contract_ledger_probes extend, which drops to 21 rows here.

@history_item n=5 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — fake agent at thirteen argv options, "eleven of them after the statusline two"; the hygiene walk and Recorded hook payloads row include `2.1.302`'s status line variants (`fixtures/claude/2.1.302/StatusLine.default-1.json`); `contract_ledger_probes`' stamped sets `2.1.287` and `2.1.302`. The option count and the per-set variant lists move with this removal.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the statusline pass-through on every build) — made `viola hook statusline` run `/bin/sh -c <command>` with no OS qualifier (`shell_argv` → `[&str; 3]`), the `settings.json` override written at every start, Run B's probe status-line recording unqualified; these are the exact surfaces (`BIN_SH`, `shell_argv`, override, Run B probe) this chunk deletes.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `LedgerRow::relied()` became 24 on every build with `STEPS_PASS`, `ROW_IDS`, `LEDGER_ROWS` `[&str; 24]` ungated; the older-stamp oracle sets restated. Base for the 23 → 21 move (statusline-rate-limits already left) in `cli_verify`, `contract_ledger_probes` and `--local-live`.

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits arm's live red) — recorded `statusline-rate-limits` red live (`23 pass  1 fail`), the row reading the account's plan state not the CLI, routed to 0.2.0 as finding F-06i with the founder's question why viola leans on the user's status line; the origin of this removal and of the box-only screen fixture CARRY's blocker.

@history_item n=9 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy) — ~ decision/version-gate-needs-all-relied-rows («a retired row it also holds changing nothing»), the precedent for retiring `statusline-override` / `statusline-shell` while older stamps still hold them; ~ decision/bounded-readers-keep-the-cap dropped the thresholds reader, the status line shell-out now the next reader to leave that list.

@history_item n=10 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Test Scope Summary, 3. Test Harness Contract) — ~ contract/viola-hook-statusline/passthrough and ~ contract/harness-boot/statusline-echo trimmed of budget.json, − contract/viola-hook-statusline/budget-json; ~ decision/self-bootstrapping-test-data; ~ contract/boot-readiness/events to two lines. The statusline rows trimmed here are the ones this chunk removes whole (passthrough, `--statusline-echo`), and the same remove-whole pattern of − rows with «→ none» applies.

@history_item n=11 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. E2E Test Strategy, 8. Mocking) — ~ decision/property-tests-on-parsers and ~ decision/property-tests-with-proptest dropped the statusline rate_limits reader and its regressions; − decision/statusline-proptest-whole-seconds; ~ contract/hook-statusline/passthrough; ~ decision/fakes-for-external-services dropped generated statusline stdin; ~ decision/snapshot-replay-from-log guard re-pointed after a test file was deleted. Precedent and traps: the statusline proptest seeds file (census C-057) is what remains, and guards naming deleted tests must be re-pointed.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (7. Test Data & Fixtures: 2 row(s)) — + decision/retired-words-as-fixtures: retired words and 0.1.0 own-format shapes live as committed fixtures in `fixtures/viola-0.1.0/` outside the identifier-probed paths, because a removal's identifier probe and the compatibility cases spelling old words cannot share a tree; this chunk's scope asks whether the removed status-line words join `retired-words.json`.

@history_item n=13 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the state-file ban scoped, two counts, the boot flag's owner) — scoped decision/state-files-never-hand-written and named `budget.json` fed only through `viola hook statusline`; Path 6 step 6 was `hook_statusline_a_home_another_user_can_write_runs_nothing_however_often`; the 20 s kill-line count rule (the `binary(…)` names on the filter line) — a register `hook_statusline`'s removal moves.

@history_item n=14 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (boot's two flags, harness_session, status over viola list) — built `boot --statusline-echo`: conflicts with `--unstamped`, writes `<home>/statusline-source.json` with the `viola-fake-agent statusline-echo` command, makes `<home>/fake/`; and Path 6's harness leg asserts the statusline branches (`path6_budget_harness`). The harness flag (contract/harness-boot/statusline-echo, census C-278/C-450/C-514) leaves with this chunk; the closed harness usage surface changes (decision/closed-enums-need-decisions-log).

@history_item n=15 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (test data) — the fake agent took two mode words (was one: `statusline-echo`), `agents` added; the mode-word register drops back to one when `statusline-echo` leaves.

@history_item n=16 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the bounded-reader enumeration in the Contract suite) — `tests/contract_bounded_readers.rs` with literal `BOUNDARIES` / `EXCEPTIONS` tables and the census `product_readers_are_each_bounded_or_named`; the status-line command's stdout is a `BOUNDARIES` row, which must leave with the shell-out or the census reads a stale row.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the bounded readers' cap-driving witnesses) — the `BOUNDARIES` row for the status-line stdout names `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap` and `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command` (`src/cmd/hook/statusline.rs`); these are the guard of decision/statusline-shell-out-frame-cap and part of decision/bounded-readers-keep-the-cap, which go when the arm is deleted.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the four readers on the replay; E5 built) — statusline `serve` became one of the six replay readers and finds no `statusline_command` over a replayed snapshot (`statusline_over_a_replayed_snapshot_runs_nothing`, guard of decision/statusline-command-never-runs-unsafe); E5's reader list names it. Its removal shrinks the replay-reader register to five and retires that guard.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the seeded home made private; the hook's instance check) — `hook_statusline`'s three cases were the witnesses that seeded homes pass `check_instance`, and the hook's instance check kept the two `--capture` arms ahead of it; `hook statusline --capture` is one of those arms and leaves, the other stays.

@history_item n=20 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (the strict-modes cases without a log line) — the hook's statusline arm reads and opens nothing after a refused check; with decision/statusline-command-needs-strict-home / decision/statusline-command-after-strict-modes this is a security-control negative whose arm disappears (decision/one-negative-per-security-control).

@history_item n=21 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (E5 steps 1–3 and the snapshot readers) — counted `src/cmd/hook/statusline.rs` among the snapshot readers by the rule `read_snapshot(` under `src/`; a count register that moves when the arm leaves.

@history_item n=22 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the headroom floor, the hook hangup named) — on Unix every command the fake agent spawns, each hook and the status line command, runs in its own process group (decision/fake-agent-children-own-process-group); the status-line half of that row leaves with the fake agent's statusline modes.

@history_item n=23 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange) — §2's jsonschema clause lists per-class schemas (status line, agents, screen, MCP apart from `claude-fixture.v1.json`); decision/fixtures-schema-checked loses the status-line class with `schemas/claude-statusline-fixture.v1.json`.

@history_item n=24 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (1. and 7. Test Data & Fixtures) — ~ decision/fixtures-recorded-by-live-verify (names «status line» among the fixtures verify refreshes) and ~ decision/recorder-scrub (names «status line variants»); both rows spell the status line and must be rewritten, not removed, when the StatusLine fixtures leave.

@history_item n=25 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — + decision/screen-fixture-box-rows-as-literals and ~ contract/readiness-gate/gate-max-wait (the box reads up on its rule and prompt rows with no footer literal); the CARRY cry-hqyx2rou's box-only screen fixture is the recording these rows await, blocked only on a live `viola verify --record` at 0 fail.

@history_item n=26 marker=2026-11-13-epoch-8-cleanup
**2026-11-13-epoch-8-cleanup** (2. Test Strategy) — + decision/os-gated-cases-in-binary-root: when a test binary is split into a module directory its `cfg` cases stay in the root file (the `cli_verify` split); this chunk edits `cli_verify{,/runs,/stamps}` and `contract_os_gated_tests`, whose `GATES` rows must still stand.

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS-gate census reads another OS as a fault; the bootstrap phases) — census count 39 functions + 2 files = 41 `GATES` rows, and the `ci` kill line at twenty-six verify-driven binaries with the root waits at 31 sites in 23 files; deleting `tests/hook_statusline.rs` and statusline cases moves these counts by their rules.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — decision/bounded-remove-the-guard-witness: a control whose neutralised guard lets a product process start needs a bounded witness (an unbounded one hung 27 min); applies to the directive's `gate.py control` remove-the-guard runs.
