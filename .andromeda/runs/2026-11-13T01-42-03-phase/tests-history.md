@history_item n=1 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the long-turn screen, the second literal and the 2.1.302 set) — added the `input-box-signature` arm reading three Run B screens (`ready`, `turn`, `long_turn`) with `check_input_box_signature_*` cases; `SIGNATURES.input_box` became two literals, `for agents` and `(shift+tab to cycle)`, pinned by the `resting_footer` cases; `--long-turn-screen <phase>` joined the fake agent (thirteen argv options); `verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal`; 2.1.302 became a stamped set in `contract_ledger_probes` and the hygiene walk. This is the exact literal set and arm this chunk replaces (both literals are footer hints): every pin named here (resting_footer cases, the input-box-signature arm, the screen fixtures, the stamped 2.1.302 set) moves or is re-proved with it, and a new footer-notice screen variant follows the same fake-agent option + recorded screen pattern.

@history_item n=2 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** (the readiness stall report in wait_ready) — `tests/support/home.rs` `wait_ready` appends the stalled wrapper's last event, `/proc/<pid>/wchan` and `stat` state to the watch report before panicking (row `decision/readiness-stall-report`). A readiness change that stalls a boot reports through this witness; it signals nothing and adds nothing on a passing boot, so the chunk must not widen it.

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — introduced verify's interactive runs, the screen fixture class (`Screen.*.json`, signature-rows-only, scrub-as-detector, refused whole with a named code, `schemas/claude-screen.v1.json`), the fake agent's `--screens` / `--trusted-root` / `--turn-stop`, wrapper boots passing `--screens --trusted-root <cwd>`, and stated that `SIGNATURES` feed both the full gate on a verified CLI and verify's settle and record helpers; added the `verify_window_` mutants override. Constrains any new screen fixture for a footer notice (rows `decision/screen-fixture-scrub-as-detector`, `decision/wrapper-boots-pass-trusted-screens`, `decision/screen-signatures-only`) and shows the literal is shared with the run gate.

@history_item n=4 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — named the spine bound, landed the vt100 feed property (`feed_prop_a_caught_panic_poisons_until_resize`) and fuzz target `vt100_feed`, and built `--vt100-panic-bytes` from measured bytes with no env seam. The readiness gate that reads `SIGNATURES.input_box` for `viola run` sends is the out-of-bounds sibling of this chunk's change: if research shows it shares the literal, its feed-panic and gate cases (rows `decision/vt100-feed-panic-safe`, `contract/readiness-gate/gate-max-wait`) are what a changed signature must keep green.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the gate verdict is ready / wait / `input-not-ready`; on a verified CLI a quiet screen with no input-box literal waits up to `GATE_MAX_WAIT` 8 500 ms (unit-pinned); `send` re-reads human-typing and turn-running after the gate; `verify_window_` kills moved to 60 s (ci) / 45 s (mutants) because the no-screen case waits the 8.5 s maximum four times; `--paste-hint-ms` capped at 10 000 ms. Standing rule: a kill line moves only for a by-design floor, with a planted-hang control both ways. A changed input-box reading changes how often verify's no-screen waits hit that bound (rows `contract/readiness-gate/gate-max-wait`, `decision/verify-window-60s-kill`, `decision/send-refusal-order`).

@history_item n=6 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the forced-window pair `send_under_the_paste_hint_on_a_verified_cli` (gate reading under the fake agent's paste-hint hold), `--tag-turn-screen <phase>` and `verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal` beside the turn-screen modal case. Precedent: verify's guard before each Run B paste is tested by fake-agent screen phases plus a modal case; a footer-notice phase would sit beside these (row `decision/local-command-unconfirmable`, `contract/driver-send/local-command`).

@history_item n=7 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — Run B's added pastes and the framing tier (`paste-<n>`, `clear-<n>` variants recorded by Run B, replayed byte for byte under `--framing`), the fake agent's `--framing` and test-only `--paste-hint-ms`, and the 20 s verify-driven kill kept on a measured 9.98 s reading. Defines the paste sequence whose `input_box_up` head this chunk re-keys, and the replay a footer-notice fixture must not break (rows `contract/fake-agent-input/framing-paste`, `decision/fake-agent-contract-suite`).

@history_item n=8 marker=2026-10-13-epoch-4-survivors-ii
**2026-10-13-epoch-4-survivors-ii** — the 16 `typed.rs` survivors (`framing_turns` 5, `SharedPty` 5, `Run`'s ending 6) were disposed after behaviour-preserving restatements (`added_turn_ended`, `local_command_answered`, `probe_until`, generic `SharedPty` with a recording double, `ctrl_c_ending`, `ended_by`), each killed by a hand-applied-mutant case. This chunk edits `framing_turns`, `turn`'s settle and `Run`'s `box_wait`: those named predicates and their kill cases are the standing witnesses not to regress; mutation runs only at the epoch boundary (scope: no mutation run).

@history_item n=9 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the moved units re-cited) — the stop's and typed run's unit cases now sit in the `src/cmd/verify/typed/` module (was `typed.rs`), the Epoch 4 dispositions re-cited there. Locates `turns.rs`'s `input_box_up` tests for this chunk.

@history_item n=10 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the mutants profile's paste-hint class and the verify_window_ kill at 60 s) — `[profile.mutants]` holds four overrides with `verify_window_` first at a 60 s kill; the no-screen `verify_window_` case reads ~43 s; each kill moved only with a planted-hang pair. A readiness change that alters verify's wait floor must re-read against these kills, never raise the default (rows `decision/verify-window-60s-kill`, `decision/mutants-kill-below-cargo-mutants-timeout`).

@history_item n=11 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** (the `mutants` override order and the `verify_window_` class) — the `verify_window_` override stands first (first match wins), pinned by `mutants_verify_window_override_stands_before_the_harness_package_override`; the class's floor measured as four ~8.56 s gate waits. Guard named by row `decision/verify-window-60s-kill`.

@history_item n=12 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — eighteen rows with `paste-newline`, Run B's four added pastes (`paste-3`), `cli_verify` per-row literals, the `LIVE_PASS` proof; the tmpfs live home reads `row-missing` until re-stamped. Precedent for adding a ledger row: `LedgerRow::ALL`, `STEPS_PASS` / `ROW_IDS` / `LEDGER_ROWS`, `cli_verify` and `contract_ledger_probes` literals all move together (rows `decision/ledger-row-verify-probe`, `decision/ledger-probe-contract-suite`, `contract/verify-output/step-line`).

@history_item n=13 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — step lines `[NN/MM]` with MM = `LedgerRow::relied()`'s length; the three statusline arms; StatusLine fixtures replayed under `--framing`; a `workspace.repo` naming this repo's remote kept by scrub and walk. Bears on any new row or recorded fixture (rows `contract/verify-output/step-line`, `decision/no-production-data-in-tests`, `decision/recorder-scrub`).

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — relied 24 (`local-command-compact` last), Run B's five added pastes (`/compact` before `/clear`), the compact variant and `fake_agent_compact_replay_matches_every_recorded_compact_set`, the older-stamp oracle. The current Run B paste sequence and row count this chunk's proof must read pass on the newest CLI (rows `contract/fake-agent-input/compact`, `decision/version-gate-needs-all-relied-rows`).

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `LedgerRow::relied()` is all 24 rows on every build; `STEPS_PASS`, `ROW_IDS`, `LEDGER_ROWS` are `[&str; 24]`; the older-stamp oracle reads the 18+2 and +1 sets unverified. A ledger row added or reshaped for the input-box literal moves these literals and the oracle (row `decision/version-gate-needs-all-relied-rows`).

@history_item n=16 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits arm's live red) — a live verify reads `statusline-rate-limits` red where the account's payload carries `five_hour` `unknown`; the row reads the plan state, not the CLI; owner 0.2.0 (F-06i); the ledger stays 24 rows. Explains why this chunk's proof accepts every row pass but `statusline-rate-limits`.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the supply-chain advisories with no ignore) — `supply-chain` green with `[advisories] ignore = []`, `portable-pty =0.9.0` dropping `pin-utils`, the RUSTSEC-2017-0008 ignore removed. The precedent and the standing state this chunk's supply-chain item must restore; adding a `deny.toml` ignore per advisory would be the first widening since (rows `decision/no-advisory-ignored`, `decision/no-ignored-advisories`).

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the recorded supply-chain red, RUSTSEC-2026-0441) — a supply-chain red was recorded with an owner CARRY whose remedy was a dependency move off the pin, "the ignore list is not widened". Direct precedent for this chunk's choice between an rmcp bump and reasoned ignores.

@history_item n=19 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — a RustSec advisory closed by a workspace exact-pin and lock bump (tokio `=1.53.1` → `=1.53.3`), the test-plan's pin mentions (viola-e2e dev-deps, time-mocking rows, time-sensitive fixtures row) moved to match. Precedent: an rmcp bump past `>=3.4.1, <3.5` must move every master site naming that range (viola-e2e's rmcp `client`-only pin).

@history_item n=20 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the CI gates; harness keys; the MCP tests as landed) — `scripts/rmcp-features.sh` asserts rmcp's release graph requests exactly `server` + `transport-io`; viola-e2e takes rmcp `>=3.4.1, <3.5` with `client` only and no child-process transport (it truncated a coverage profile); the MCP suites (`handlers.rs`, `mcp_tools.rs`) pin `tools/list` with `ttlMs`/`cacheScope`. An rmcp bump must keep the feature assertion, the client-only pin and these suites green (rows `decision/rmcp-feature-limit`, `decision/rmcp-features-server-transport-io`, `decision/viola-e2e-rmcp-client-only`, `decision/rmcp-client-only`, `decision/mcp-stdio-test-owns-child`).

@history_item n=21 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange) — `Mcp.tools-list-1.json` recorded by the test-only `viola-mcp-tap`, walked against `schemas/claude-mcp-fixture.v1.json`, and `tests/mcp_recorded_exchange.rs` holds the served `tools/list` result to the recorded key/type shape. An rmcp bump that changes the served shape reads red here; refreshing the exchange is the tap's local step, never CI (row `decision/fixtures-recorded-by-live-verify`).

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the `viola mcp` stdin line cap; the serde_json ban, deny-probes 14/14) — rmcp 3.4.1's stdio transport reads lines unbounded, so viola hands it `CappedLines` at `MAX_FRAME`, witnessed by `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` and `capped_lines_*`; `deny.toml` bans serde_json `unbounded_depth` with a firing probe (`deny-probes: 14/14`). Any rmcp bump must keep the transport fed through the capped reader, and any `deny.toml` edit must keep the probes' line (rows `decision/bounded-readers`, `decision/no-unbounded-json-depth`, `decision/deny-bans-probed`).

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the error bodies' surface witnesses) — rmcp's client reads the wire's `"data":null` as `None`, so the live unknown-tool case asserts `None`; `mcp_tools_error_results_hold_no_path_or_upstream_text` and the error-body enumeration pin MCP results. Trap for an rmcp version move: client-side decoding behaviour is asserted in these oracles (rows `decision/error-sanitization`, `decision/error-writers-each-named`).

@history_item n=24 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — defined the `supply-chain` job (cargo deny, the sole-root tokio ban, deny probes, zizmor) and the weekly nightly advisories, recording the cargo-deny 0.20 CLI form. The job this chunk turns green (rows `decision/supply-chain-fail-closed`, `decision/cargo-deny-gate`, `decision/weekly-advisories-run`).

@history_item n=25 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — the supply-chain row gained the `fuzz/Cargo.lock` advisories + sources step and weekly advisories on both lockfiles. A `rustix` lock bump must also clear the fuzz lockfile if it carries the advisory (rows `decision/cargo-deny-supply-chain-gate`, `decision/weekly-advisory-scan`).

@history_item n=26 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — `--local-live` alone runs no other suite and, naming no program, stamps the first `claude` on `PATH`; a route entry's live readings of a real session are that chunk's own local measurement, not a test layer, never in CI. Governs how this chunk's proof (one live verify on the newest CLI) is run and recorded (rows `decision/live-readings-not-a-test-layer`, `decision/live-verify-never-in-ci`, `contract/local-live-suite/pass`).

@history_item n=27 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (stamped root seam; run --local-live specified; fake-agent default) — root `stamped_home` runs verify against the fake agent and is stamped only on exit 0 and a `stamped <ver>  <n> pass  0 fail` last line; `--local-live` refuses `live-in-ci` (exit 2) and checks literal row ids; the fake agent's default CLI version is the recorded set's. The stamping rules the newest-CLI proof and any new fixture set obey (rows `decision/verify-sole-stamp-writer`, `contract/viola-verify/stamped-line`, `decision/harness-cli-version-matches-fake`).

@history_item n=28 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the default CLI version moved 2.1.283 → 2.1.287 across `DEFAULT_CLI_VERSION`, `RECORDED_CLI_VERSION`, `stamped_home` and `Wrapper::boot`. Precedent if a newest-CLI recorded set becomes the default (row `decision/harness-cli-version-matches-fake`).

@history_item n=29 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — `tests/cli_verify.rs` pins verify's step lines, the `stamped` summary and `unable:`/`hint:` pairs by literal; the fixture hygiene walk is a run-time directory walk (rstest refuses an empty glob). New screen fixtures for a footer notice are walked and pinned this way (rows `decision/human-output-pinned-by-literals`, `decision/fixture-hygiene-walk`).

@history_item n=30 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — `contract_fake_agent_drift` compares the fake agent's hook stdin byte for byte with every recorded set, walked at run time, an empty walk failing; insta stays for decision bodies only. A footer-notice fixture or fake-agent screen option must keep this contract (rows `decision/fake-agent-matches-recordings`, `decision/fixture-drift-byte-compare`, `decision/fake-agent-replays-recorded-bytes`).

@history_item n=31 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another, ETXTBSY) — a test planting more than one fake-agent copy plants them sequentially before any thread spawns a child (6/40 red before). Trap if the chunk adds a recorded set the drift or ledger contracts plant per set.

@history_item n=32 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the start's and verify's home-tree checks; the stamps and pin checks) — `verify` refuses a widened `ledger/`, `bin/` or `plugin/` before any step (`verify_refuses_a_widened_ledger_before_any_step`), and `update_stamps` refuses a widened ledger before its create and under the lock. The proof's stamps are written through this path (rows `decision/widened-install-dirs-refused`, `decision/stamps-refuse-widened-ledger`).

@history_item n=33 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` signals a started verify once the receipt shows its Run B progress; unit cases in `stop.rs`, `typed.rs`, `version_gate.rs` back it. A changed Run B wait sequence must keep the receipt cue this case keys on (row `contract/verify-stop/signal`, `contract/verify-exit-detail/interrupted`).

@history_item n=34 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola verify`'s refusal codes) — verify writes its refusal code on `process-exit` under a parsed `VIOLA_NAME`; a failing row is no refusal and carries no code; `unclean-fixture` keeps the file name out. A row failing on the new readiness reading stays a plain fail line (rows `decision/verify-failing-row-no-refusal`, `contract/verify-exit-detail/*`).

@history_item n=35 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (agents-join at 23/20 rows, Run B's agents read) — Run B makes its agents read once its first turn has settled, before the framing pastes; an uninterrupted verify closes eight spawn pairs. The first-turn settle this chunk re-keys gates that read, so the order must hold.

@history_item n=36 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Run E and the resume variant; the no-screen `verify_window_` case went 34.42 s → 42.98 s against 60 s / 45 s kills, no kill moved. A readiness change that adds waits per paste re-reads this floor.

@history_item n=37 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — named the shared root helpers in `tests/support/` (`events.rs`, `cli.rs`, …), no root test file defining its own copy. Any new test helper for the footer-notice cases goes there (rows `decision/one-shared-test-helper`, `decision/shared-e2e-helpers`).

@history_item n=38 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became the rows this chunk is bound by, each verified against the code at 8834195b; the picked rows are that conversion's output, so a row the chunk changes moves through the row registry, not prose.
