@history_item n=1 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (run --local-live specified) — set `run --local-live`: refuses `live-in-ci` (exit 2) under `CI` before any build or spawn, builds into `target/harness`, then runs one verify against the real `claude` in `target/e2e-home/viola-live-<pid>/home` through the runner seam, with closed failure codes. This is the established live-verify path and home naming that this round's rigs and its ledger stamping on a test home reuse (rows decision/live-verify-never-in-ci, decision/local-live-runner-seam, contract/local-live-*).

@history_item n=2 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — made `target/e2e-home` an operator-set link to an owner-only tmpfs on the dev host, with `ensure_e2e_home` / `prepare_home_base` checking the link. The keeper runs before the `--local-live` verify, and cleanup deletes through the link. Standing rule: a stalled-start red is a finding about the backing, never re-run for green. The scope puts live homes on tmpfs `target/e2e-home/viola-live-*` (rows decision/dev-host-e2e-home-on-tmpfs, decision/e2e-home-deletion-through-link, decision/e2e-home-link-base).

@history_item n=3 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the headroom floor) — a linked e2e-home base with less than `HEADROOM_FLOOR_MIB` (9 GiB) free is refused with `e2e-home backing: {free} MiB free, below the {floor} MiB floor`. The floor is one compiled constant that no setting changes. The scope's `df -h /tmp` read first comes from this. A live home that stays up for an hour on tmpfs must stay above the floor (rows decision/e2e-home-headroom-floor, contract/e2e-home-refusal/message).

@history_item n=4 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** (the verify_window_ kill at 60 s) — the `verify_window_` class and the paste-hint class got their own kill lines (60 s / 30 s under `mutants`), each moved only with a planted-hang control read both ways. Precedent: a rig or test that runs whole verifies (`--record`) sits on a by-design floor, and a kill line moves only with a two-sided control. This bears on "every spawn has a kill deadline".

@history_item n=5 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (live readings are not a test layer) — a route entry's live readings of a real session are that chunk's own local measurement. They sit outside any test layer and harness surface, never run in CI, and run outside `viola verify` on the founder's ruling. This is the governing precedent for this round's operator steps and evidence records (rows decision/live-readings-not-a-test-layer, decision/no-real-cli-in-ci, decision/real-claude-local-only).

@history_item n=6 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** (`--local-live` alone runs no other suite) — `--local-live` passed alone is a selection of its own. Because `verify` names no program, it stamps the first `claude` on `PATH`. This trap bears directly on fork (3) of the scope: the shim pins 2.1.296 while `claude` on `PATH` is newer, so which CLI gets stamped depends on `PATH` (row contract/run-suite/local-live).

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — recorded that the twenty-two-row live firing took six starts and read `22 pass  0 fail`. The `verify_window_` no-screen case went from 34.42 s to 42.98 s, and no kill moved. This is the precedent for counting live starts per verify, which fork (1) of the scope (the live-start cap, measured per item) needs.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the relied set, all twenty-four rows on every build) — `LedgerRow::relied()` is 24 rows on every build. `STEPS_PASS` / `ROW_IDS` / `LEDGER_ROWS` are `[&str; 24]`. The `--local-live` row list follows them. The wrapped start must read `cli_verified:true` on a ledger that reads green, and this is that ledger (rows contract/viola-verify/stamped-line, contract/verify-output/*, decision/ledger-row-verify-probe).

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the OS scope: Linux only, one CI leg) — the product is Linux only, CI runs on one `ubuntu-latest` leg, and the Windows/macOS live-measurement entries are retired. The round runs on the Linux dev host only, and no Windows live item is owed (rows decision/linux-only, decision/single-os-ci-leg).

@history_item n=10 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the statusline-rate-limits arm's live red) — on an unchanged `claude` 2.1.287, a live `viola verify` read `statusline-rate-limits` red (`23 pass  1 fail`) because of the account's plan state. The founder deferred v1-27. This chunk's wrapped start (v2-03) is v1-27's heir, and a green ledger for the founder's account rests on that row now being retired.

@history_item n=11 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the child's fd inheritance on 0.9.0) — a live `claude` 2.1.287 reading (a local measurement, not a suite) of the wrapped child's `/proc/<child>/fd`: 31 fds, fds 0–2 on the pty, no viola endpoint socket, no file under the home. portable-pty `=0.9.0` closes inherited fds. This is the prior reading that the wrapped start's "no byte of viola's own" evidence extends (rows decision/fd-test-reads-live-child-table, decision/child-env-strip).

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Test Scope Summary, start events) — `contract/run-start-events/order` now reads wheel with cause start, then the agent's session-start with source hook, with no budget-gate record. The replay drops the budget fields. This is "the documented start order with no budget step" that the wrapped start must show live.

@history_item n=13 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (3. Test Harness Contract) — `contract/boot-readiness/events` now reads two start records (wheel start, then session-start), and the status document's budget fields are removed. Harness-booted rehearsals of the rigs wait on this readiness.

@history_item n=14 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Unit Test Strategy) — `decision/version-gate-needs-all-relied-rows` gained "a retired row it also holds changing nothing", for the retired `statusline-rate-limits` row. The send refusal order lost the budget rung. A stamp written for the founder's account verifies on the relied rows alone.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (6. E2E Test Strategy: 9 rows) — `contract/event-log/start-wheel-record` became the heir: every start, revive included, appends wheel driver/start first, and session-start is its second and last start record. `contract/viola-run-start/event-order` is retired into it. The live wrapped start's order reading is held to this.

@history_item n=16 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (6. E2E Test Strategy: 5 rows) — added `contract/viola-run-start/settings`: a start hands its child no `--settings`, writes no instance `settings.json`, and leaves the user's settings byte- and mtime-equal. The `generated-plugin/hook-commands` promise lost the settings override. This is the by-construction proof of v2-05 that CARRY 3's live status-line reading completes. The reading must write nothing to `~/.claude/settings.json`.

@history_item n=17 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (1. Test Scope Summary: 5 rows) — removed the status line from `decision/fixtures-recorded-by-live-verify` and `decision/fake-agent-contract-suite`, and retired the status-line passthrough rows. The StatusLine fixtures and verify's `--record` status-line variants are gone. That is what unblocks CARRY 2's clean `--record` of a box-only screen fixture.

@history_item n=18 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (7. Test Data & Fixtures: 6 rows) — added `contract/fake-agent-cli/settings`: the fake agent keeps `--settings` for verify's Runs C/D, while its status-line options, the `statusline` receipt kind and `statusline-echo` are removed. The `fixture-hygiene-walk` except lost its subject. Fake-agent rehearsals of the rigs must use only the options that remain.

@history_item n=19 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (recorder-scrub and plugin-rewritten-on-start) — `decision/recorder-scrub` now scrubs hook payloads only, and its `workspace.repo` except is gone. `decision/plugin-rewritten-on-start` now covers the plugin directory alone. The re-recorded `Mcp.tools-list-1.json` and the new `Screen.*` fixture go through this scrub, and the wrapped start's "freshly written plugin" is this row.

@history_item n=20 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (3. Test Harness Contract) — retired `contract/harness-boot/statusline-echo`: `viola-harness boot` no longer takes `--statusline-echo`. Harness-based rigs cannot plant a status-line source.

@history_item n=21 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** (the `Mcp.` exchange) — added the seed row "Recorded MCP exchange": `Mcp.<variant>.json` (`Mcp.tools-list-1.json` in `2.1.302`) is recorded by the test-only `viola-mcp-tap` (`record` between one live `claude -p` start and a direct `viola mcp`; `fixture` through the `--record` scrub and four refusal codes). It is walked by `mcp_fixtures` against `schemas/claude-mcp-fixture.v1.json` and replayed by `tests/mcp_recorded_exchange.rs`. This is exactly the path CARRY 1's re-record takes (rows decision/mcp-exchange-recorded-by-tap, decision/mcp-tap-refuses-dirty-exchange, contract/mcp-fixture-scrub-refusal/*).

@history_item n=22 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — introduced the screen fixture class. `Screen.*.json` holds signature rows only, is validated against `schemas/claude-screen.v1.json`, and is checked scrub-as-detector: a kept row the scrub would rewrite refuses the recording whole with a named code. The hygiene walk also refuses email tokens and seam-split usernames. CARRY 2's box-only `Screen.*` fixture must pass this (rows decision/screen-fixture-scrub-as-detector, contract/verify-recording-refusal/unable-message).

@history_item n=23 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (7. Test Data & Fixtures) — added `decision/screen-fixture-box-rows-as-literals`: a box rule row is recorded as its rule chars alone and the prompt row as the prompt char alone, enforced by the walk (`planted_box_row_beyond_its_literal_is_rejected`). This is the form CARRY 2's box-only fixture must take. The CARRY originates in this chunk.

@history_item n=24 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** (4. Unit Test Strategy) — `contract/readiness-gate/gate-max-wait` now reads the input box as up on its rule and prompt rows with no footer literal, waiting up to 8 500 ms only when the box is not up. The box-only fixture is what lets the fake agent replay this reading in CI.

@history_item n=25 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — brought the `2.1.302` set into the suite: its dialog, framing, resume, agents and screen variants, `contract_ledger_probes` stamping `2.1.287` and `2.1.302`, the second input-box literal, and the long-turn screen. The fixture dir named in fork (3) of the scope, and the set that CARRY 1 and CARRY 2 add files to, is this set.

@history_item n=26 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — made the dialog hold configurable: `dialog_hold_s` is read once through a bounded read, `DialogHold` bounds are unit-cased, and the controls row `setting_does_not_unbound_the_dialog_hold` (`0`, `3601`, …) records `parse-rejected` for out-of-range values. The expiry records one `dialog-expired` and `dialog-answered{deadline_hit:true}`. The live hold near the 3600 s ceiling (v2-02) reads this surface, and a move of `DIALOG_HOLD_MAX` would move these cases (rows decision/dialog-hold-read-once, decision/dialog-hold-expiry, decision/dialog-timeouts-outlast-the-hold).

@history_item n=27 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the bounded readers' cap-driving witnesses) — the dialog-hold reader's cap is held by `read_hold_keeps_no_key_past_the_frame_cap` (`src/run/dialog.rs`), among the five `BOUNDARIES` rows given cap-driving cases. This is the standing witness for the hold's config read if fork (4) moves the ceiling (row decision/bounded-readers-keep-the-cap).

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (remove-the-guard witnesses bounded by `WITHIN`) — a control whose neutralised guard lets a product process start needs a witness bounded through `tests/support/cli.rs` `Running`. nextest's default profile has no kill line, and an unbounded witness hung for 27 minutes. The scope requires every guard test's control through `gate.py control` (row decision/bounded-remove-the-guard-witness).

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the start's and verify's home-tree checks before any log) — the start runs `check_home_tree` before the wrapper log, so a refused tree exits 1 with no log and no child, and `verify` refuses a widened `ledger/` before any step. This is the front of the "documented start order" the live wrapped start is read against (rows decision/run-checks-before-child-spawn, decision/widened-install-dirs-refused, decision/home-readable-by-others-refused).

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5 built) — a start over a replayed snapshot reads liveness from the beat alone: a fresh beat refuses `already-live` (exit 1), and an older one leaves the name free (`tests/chaos_snapshot_replay.rs`). This is the live/stale name refusal the wrapped start must show (rows decision/start-collision-by-heartbeat, decision/collision-check-over-replay-reads-beat).

@history_item n=31 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder reads `stale-heartbeat`) — a stale holder refuses the start exit 1 with `stale-heartbeat`, while a live holder or fresh replayed beat keeps `already-live`. The stale cases stand on the real `Frozen` SIGSTOP guard. This is the exit and code a live "stale name refused" reading must match (rows contract/run-refusal/stale-heartbeat, contract/start-exit-1/*, decision/real-process-faults).

@history_item n=32 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the Frozen guard's one home) — `Frozen` lives only in `tests/support/frozen.rs` (on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, `kill -CONT` on drop). A rig or rehearsal that needs a stale holder reuses it and never makes a copy (row decision/real-process-faults).

@history_item n=33 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a start's socket directory refusal) — Path 1's start runs the socket-directory check after the version gate and before the spawn. A refused directory exits 1 before any snapshot, child or bind, logged `socket-dir-refused`. The documented start order and the start refusals the wrapped start is held to include it (rows contract/viola-run/start-refusal-exit-1, decision/run-checks-before-child-spawn).

@history_item n=34 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** (the pinned copy's no-clobber placement) — the pinned copy is placed once, owner-only and mode-first, through `fs::place_private_new`, with a forced-window ETXTBSY witness. A copy another start placed is re-hashed. The wrapped start's "pinned copy run through a freshly written plugin by absolute path" rests on this (rows decision/private-placement-mode-first, decision/pinned-exe-hash-checked).

@history_item n=35 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (fake-agent copies planted one after another) — on tmpfs `target/e2e-home`, each fake-agent copy is written rather than hard-linked. Every copy must be planted before any thread starts a child, or a fork inherits an open write handle (ETXTBSY). This trap applies to rig rehearsals that plant more than one fake agent (row decision/fake-agent-copies-planted-first).

@history_item n=36 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the fake agent stands as `claude` in a test-scoped bin dir and answers from files beside its exe, with no env var. Every test-started `list` / `revive` runs under a test-scoped `PATH` so that a test never reads the host's live sessions. Rehearsals must keep this isolation, while live operator steps deliberately reach the host `claude` (rows decision/tests-never-read-host-sessions, decision/fake-agent-on-test-path, decision/agents-stub-from-files-no-env).

@history_item n=37 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tests as landed) — MCP tests spawn and own a `viola mcp` child, closing its stdin and awaiting its exit, and never use rmcp's child-process transport, which truncated a coverage profile. `tools/list` is exactly the five tools with `ttlMs` 0 and `cacheScope` private. CARRY 1's re-record must leave `tests/mcp_recorded_exchange.rs` still holding this shape (rows decision/mcp-stdio-test-owns-child, contract/mcp-server/tools-list, contract/mcp-tools-list/tool-set).

@history_item n=38 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (7. Test Data & Fixtures: 1 of 2) — added `viola-hypr-tap` as a second test-only recorder that applies the same scrub, with a hygiene class and a closed keep-list. This is the latest precedent for a test-only tap's recording path and for the rows naming who refreshes each fixture kind (decision/recorder-scrub, decision/fixture-hygiene-walk, decision/fake-agent-on-test-path).

@history_item n=39 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (11. Test Anti-Patterns) — `decision/no-production-data-in-tests` now allows fixtures from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames. Both of this round's new fixtures (the `viola-mcp-tap` capture and the `--record` screen) must come through one of these sources.
