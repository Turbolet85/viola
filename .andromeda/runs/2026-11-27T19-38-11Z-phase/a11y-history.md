@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — moved the CLI output-discipline invariants (P4, P6) from a per-chunk mutation gate to the epoch-boundary mutation audit (`run --mutants`), the origin of decision/mutation-audit-covers-crate-invariants. Item 1's root-crate baseline and its one addendum run are that audit's reading. Mending the `mutants` profile's kill so the baseline passes is what lets these invariants be scored at all.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — put the CLI output-discipline evidence inside the CI `coverage` suite, gated by `gate --require coverage,doctest`, the origin of decision/cli-invariants-pass-in-ci. The chunk's moved and split code and its new killing tests have to stay green there.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home; the tui boundary on openpty alone; the Linux-only platform line; revive's start refusals without batch-script-child; strict-modes without the Windows arms) — set decision/linux-only and made the one `ubuntu-latest` leg the a11y verdict. The tui boundary clauses became nextest cases on openpty with no Windows arm, and revive's start codes dropped to ten. The pump.rs restart tests and the r2-open.sh check need no other-OS arm, and the start-refusal code set the prologue move must keep is the post-2026-11-05 one.

@history_item n=4 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (cli checks on std::process::Command and literal pins; the outer-PTY driver on portable-pty =0.9.0; §8 verification lines pinned by literal; cli tooling in the testing contracts) — replaced trycmd/assert_cmd with literal pins over `std::process::Command`, the basis of decision/refusal-layout-literal-pins and decision/fixed-cli-strings-pinned-by-literal. It also pinned the tui driver at portable-pty `=0.9.0`, inherited and never re-pinned (decision/tui-boundary-driver-pin). For the `refused()` merge and the splits, "no test edited beyond a re-pointed path or name" means these literal pins must stay byte-identical and unedited. New pump tests use the inherited driver pin.

@history_item n=5 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — precedent for a pure-move split in a cleanup chunk: the board units were re-cited at `src/human/board.rs` after `src/human.rs` was split, every test keeping its name. A split that moves a test or source path the plan cites (here `src/cmd/run.rs`, `src/cmd/up.rs`, `crates/viola-pty/src/pump.rs`) owes a re-citing wrap amendment, not a rule change.

@history_item n=6 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — second pure-move precedent: when `board.rs` became a module root over `board/`, the literal-pinning units were re-cited under `src/human/board/`. This is the same shape the run.rs, pump.rs and up.rs splits will produce for any plan citation of their paths.

@history_item n=7 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — made a passed `viola revive` enter the TUI passthrough boundary, and gave revive's exit-1 refusals the fixed-message `unable:` / `hint:` pair (contract/revive-human-refusal/pair, contract/cli-fixed-message/form). The shared revive/adopt prologue (item 3) must keep that pair byte-identical and keep the passthrough entry.

@history_item n=8 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive` `--json`: each strict-modes or preflight exit-1 refusal is one `{"v":1,"error":"<code>","detail":null}` document with empty stderr, and the human pair stays byte-identical (contract/revive-json-refusal/document). The prologue move must keep both forms. They are witnessed by `revive_json_*` in `tests/cli_revive.rs`.

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict-modes refusals as scope entities; their text; their witnesses and pinned pairs; an unstattable home) — put the strict-modes check first, before any log opens, in `run`, revive's start and `verify` (contract/start-refusal/home-strict-modes, decision/strict-modes-refusal-before-any-write, decision/unstattable-home-is-strict-modes-refusal). It pinned the `HOME_PAIR` / `INSTANCE_PAIR` literals in `tests/run_cli.rs` and `tests/security_negatives_strict_modes.rs`. This is exactly the ordering item 3's shared prologue carries: the check must stay ahead of the log open in one place for both verbs, and these witnesses must stay green unedited.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every refusal of its start; exit 21's document names its cause; a stale holder's own code; the catch site's internal-error document; one hint per code) — made every refusal of the start revive shares with `run` one exit-1 document, `pinned-hash-mismatch`, `squatted-name` and `internal-error` among them. It also ruled that a `pump_child` error prints no document. It named the exit-21 causes, split `stale-heartbeat` from `already-live`, and added `no_two_codes_share_a_hint` in `tests/cross_exit_causes.rs`. Rows held: contract/viola-revive-json/start-refusal, contract/viola-revive-json/pump-child-error, contract/start-holder-refusal/*, contract/json-fault/internal-error, decision/one-hint-per-code. The prologue move and the pump.rs restart split must keep all of these, and the exit-cause table must stay green.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness; one document and its exit-2 exception; `unknown`'s detail; the census of error writers) — added `cli_error_documents_hold_no_path_or_upstream_text` and the exit-2 exception to the one-document rule. It made `unknown`'s detail a code word or nothing, and built the census in `tests/contract_error_bodies.rs` (`error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`). A merged `refused()` helper or a moved prologue changes which file holds an error writer, so the census must still name every writer and witness after the move.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start: exit 1, revive's code; at the frame verbs: exit 21's cause) — added the start's socket-directory refusal before any snapshot, bind or child. It takes the fixed-message pair and revive's `socket-dir-refused` document (contract/start-refusal/socket-dir, contract/revive-json-refusal/socket-dir-refused, decision/socket-dir-refusal-before-any-frame), plus the exit-21 `socket-dir-refused` detail. This is part of the shared start that the run.rs split and the revive/adopt prologue must leave in order.

@history_item n=13 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (start refusals gain adopt; strict-modes, internal-error, socket-dir and agents-read rows; fixed-message and `--json` forms; passthrough entry; unable-line) — extended to `viola adopt` every start-refusal and strict-modes row this chunk's prologue unification touches. These are contract/start-refusal/home-strict-modes, contract/start-refusal/socket-dir, decision/strict-modes-refusal-before-any-write, contract/json-fault/internal-error, decision/socket-dir-refusal-before-any-frame, decision/claude-agents-read-fails-open, contract/cli-fixed-message/form, contract/start-json-refusal/document, contract/tui-passthrough/entry and contract/cli-refusal/unable-line. This is where adopt got its copy of revive's prologue. Item 3 makes the two one function, and these rows must read the same after.

@history_item n=14 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (the «Eight» lead of the two time-bound rows ratified, standing) — the operator ratified decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver as standing with the lead «Eight». The lists are full at their bound, so any bound the chunk touches stays in its own row.

@history_item n=15 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** — added adopt's third own refusal, `not-resumed`, to contract/cli-refusal/unable-line, contract/cli-fixed-message/form and contract/start-json-refusal/document. It also gave decision/wheel-ignores-terminal-events its after-the-child exception (a not-resumed adopt's pair or document). That code is adopt-specific: the shared prologue must leave it on adopt's side of the move.

@history_item n=16 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — set the 5 s `claude agents --json` read and its fail-open rule for `list` and revive's preflight, the origin of decision/claude-agents-read-fails-open, later widened to adopt. Any preflight code the shared prologue absorbs must keep that fail-open.

@history_item n=17 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — gave `verify` its interrupted fixed pair, one of the one-line forms named in contract/cli-refusal/unable-line. The run.rs split that moves the window and trust steps out sits beside the start's automatic verify, so verify's pairs must stay where they print.

@history_item n=18 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (wrapped TUI reads as unwrapped; time bounds name the config lock wait; run diagnostics off terminal) — created the start's trust step, which item 6 moves out of `src/cmd/run.rs`. Its word goes in one `trust-record` diagnostics line, never a refusal, exit code or hint (decision/run-diagnostics-off-terminal). The step writes the trust record and types no key (the except of decision/wrapped-tui-reads-as-unwrapped). Its Claude config lock wait falls on viola's own start (both time-bound rows). The pure move must keep all three.

@history_item n=19 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — scoped contract/tui-passthrough/human-keystrokes and contract/tui-passthrough/viola-terminal-bytes to "once the wrapped child holds the terminal". It named the start's automatic verify window as the one exception, and gave decision/human-keystrokes-never-blocked the same except. That start-before-child window lives in `src/cmd/run.rs`, which item 6 splits. The split must not move a byte of viola's own past the child's start.

@history_item n=20 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — reworded contract/tui-passthrough/human-keystrokes and decision/human-keystrokes-never-blocked to the paste's in-flight chunk and closing marker, after the paste became chunks on one queued writer thread. The restart code split out of `crates/viola-pty/src/pump.rs`, and the killing tests for its generation guard, `reap_within` and `drain`, sit on that writer path. They must not let a restart block, drop or reorder a human key.

@history_item n=21 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (own-bytes; zero-viola-bytes except; wheel and portable-pty excepts; paste write among the time bounds) — gave contract/tui-passthrough/own-bytes, decision/zero-viola-bytes-on-the-terminal, decision/wheel-ignores-terminal-events and decision/portable-pty-outer-pty-driver their "before the child starts" excepts. It added paste write to both time-bound lists. These are the boundary rows the pump.rs and run.rs moves must leave exactly as the outer-PTY cases read them.

@history_item n=22 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — the previous epoch's cleanup chunk. It added decision/start-verify-stop-grace-off-the-human as a row of its own because the two time-bound lists were full at their bound. That is the precedent if this chunk's `reap_within` / `drain` tests surface a bound not yet named.

@history_item n=23 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — added decision/respawn-reap-bound: a switch waits at most 10 s (`RESPAWN_REAP_BOUND`) for the old child to exit, else `not-delivered` `child-not-ended`, exit 13, a bound on the wrapper and never on the human. It also extended exit 10 `human-typing` to `switch`. This is the restart path in `crates/viola-pty/src/pump.rs` whose nine survivors (`reap_within`, `drain`, the generation guard) item 2 kills and item 6 moves out. The killing tests read this bound and must keep it off the human.

@history_item n=24 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — the operator's word ratified decision/respawn-reap-bound as standing, its text unchanged, along with the provisional marks of `viola up`'s instance line and switch's rows. The reap bound this chunk's pump tests pin is now standing and not provisional, and the same wrap's directive is this chunk's source.

@history_item n=25 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (instance line; exit 21 on the board's strict refusal; the window-wait, hyprctl-read and open-live-wait rows) — added contract/viola-up-output/instance-line (one plain-ASCII line per instance, one write) and `viola up` to contract/cli-exit-codes/21. It also added decision/window-wait-bound and decision/hyprctl-read-deadline as rows of their own. `src/cmd/up.rs`'s `upped` split and the `refused()` merge must keep up's line and exit-21 refusal shape, and the window step moved out of `run.rs` carries the 5 s bound that falls on viola's own start.

@history_item n=26 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (exit 21 and strict-modes gain focus; the answers to F-A and F1 ratified) — added `viola focus` to decision/strict-modes-refusal-before-any-write and contract/cli-exit-codes/21. It ratified decision/window-wait-bound and decision/hyprctl-read-deadline, where a compositor read is bounded at 5 s on viola's own start or verb. The window step moved out of `run.rs` holds those reads.

@history_item n=27 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — added `viola open`'s exit-2 fixed-line path to contract/cli-exit-codes/2, decision/json-one-document-no-hint and contract/cli-refusal/json. Item 7 merges `refused()` in `src/cmd/open.rs` and `src/cmd/up.rs`, and must keep open's exit-2 exception and both verbs' `--json` refusal documents byte-identical.

@history_item n=28 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — added `viola verify`'s untrusted-folder refusal to contract/cli-fixed-message/form. It landed with the window step that keeps the view, which item 6 moves out of `run.rs`. The move must keep the pair and the window step's behaviour.

@history_item n=29 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — fixed the tui wheel boundary: focus, mouse and resize reports and terminal replies are non-editing and never move the wheel (decision/wheel-ignores-terminal-events). These are the outer-PTY cases that must stay green across the pump.rs restart split.

@history_item n=30 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the passthrough boundary after a contained vt100 feed panic (`tests/chaos_feed_panic.rs`): no viola literal reaches the outer terminal, the child's bytes do, and a human key still reaches the child. This is a pump-path boundary witness the pump.rs split must keep green unedited.

@history_item n=31 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause as supplemental and never gating, the origin of decision/live-readings-never-gate. Matching the chunk's ban on any live start or live arm, the r2-open.sh precondition check (item 5) is read by a rehearsal or a stub of `hyprctl`, never a live reading.
