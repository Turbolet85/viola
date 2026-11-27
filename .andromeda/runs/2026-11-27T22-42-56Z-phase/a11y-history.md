@history_item n=1 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (the board units re-cited after the human split) the pure-move split of `src/human.rs` re-cited every board unit at its new file, each test keeping its name. This is the direct precedent for this chunk's three pure moves: a moved test keeps its name, and each a11y citation of a moved path is re-pointed as a wrap amendment, never by a rename.

@history_item n=2 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — `board.rs` became a module root over `board/` and the `viola list` literal-pinning units were re-cited under `src/human/board/`, every unit keeping its name. This is the second pure-move precedent. Turning `pump.rs`, `run.rs` or `up.rs` into a module root, or moving their inline test modules out, will need the same re-cite of any cited path.

@history_item n=3 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — set the row decision/mutation-audit-covers-crate-invariants: the CLI output-discipline invariants P4 and P6 held in crate tests meet mutation only at the epoch-boundary `run --mutants` audit. Moving test modules and functions changes the sites that audit's diff reads. The scope rules out any mutation run, so each moved function must keep its tests next to it.

@history_item n=4 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the census of error writers and the body check) added `tests/contract_error_bodies.rs`, where `error_bodies_each_name_a_writer_and_a_witness_that_stand` and `product_error_writers_are_each_named` hold each error writer and its witness by name and place. This is the guard of the row decision/error-bodies-have-writer-and-witness. Moving the refusal and error writers in `run.rs` (window and trust steps), `pump.rs` (respawn) or `up.rs` means re-pointing this census, a contract census the scope's Boundaries name.

@history_item n=5 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the trust step that this chunk moves out of `src/cmd/run.rs` landed here. It changed decision/run-diagnostics-off-terminal (the step reports one `trust-record` line in `diagnostics/`, never a refusal, exit code or hint). It added the except on decision/wrapped-tui-reads-as-unwrapped (viola's start writes the trust record and types no key). It added the start's Claude config lock wait to decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver, a bound on viola's own start. The move must keep all three exactly as they are.

@history_item n=6 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — added contract/viola-up-output/instance-line (one plain-ASCII line per instance, no SGR, in one write) and put `viola up` on contract/cli-exit-codes/21 (the board's strict-modes refusal, no line). It also gave decision/window-wait-bound (the window step's 5 s wait, on viola's own start) and decision/hyprctl-read-deadline rows of their own, and set the «Eight» lead of the two time-bound lists. The `upped` split, the window step's move and their tests must keep each of these promises unchanged.

@history_item n=7 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — (the operator's answers to F-A and F1 ratified) ratified decision/strict-modes-refusal-before-any-write and contract/cli-exit-codes/21, and made F1's rows decision/window-wait-bound and decision/hyprctl-read-deadline standing. It also set the rule that a later bound gets a row only when it is new on a new bearer. This chunk adds no bound, so no row is due.

@history_item n=8 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — the switch's respawn, the restart code this chunk moves out of `crates/viola-pty/src/pump.rs`, landed here. It added decision/respawn-reap-bound (a 10 s reap wait on the wrapper; a child that has not exited is not replaced, exit 13 not-delivered child-not-ended) and contract/viola-switch-output/result-line. It widened decision/send-refused-while-human-typing and contract/cli-exit-codes/10 to cover `switch` (refused `human-typing`, no child replaced). The respawn module must carry each of these unchanged.

@history_item n=9 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — ratified decision/respawn-reap-bound (provisional to standing, text unchanged) and the switch rows contract/cli-exit-codes/10 and decision/send-refused-while-human-typing, plus the `viola up` instance line. This is the boundary wrap whose operator word (`inputs#I1` §2) is the source of this chunk's carry. The rows the moved code bears now stand on that word, and any change to their text needs a new ruling.

@history_item n=10 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — narrowed contract/tui-passthrough/viola-terminal-bytes to hold «once the wrapped child holds the terminal», with a start's automatic-verify lines L1 to L5 and verify's own output ahead of the child's bytes. It also gave decision/human-keystrokes-never-blocked its automatic-verify-window except. Both describe the `run::start` boot sequence that the window and trust move restructures, so that order (verify, then the child) must hold unchanged.

@history_item n=11 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — widened contract/tui-passthrough/own-bytes to cover the keyed start's automatic-verify lines. It gave decision/portable-pty-outer-pty-driver and decision/wheel-ignores-terminal-events their «from the child's start» excepts, and added paste write to both time-bound lists. These are the outer-PTY boundary rows whose nextest cases are the smoke for the shared start that `run.rs` holds. It also ratified the Epoch 4 delegate confirms on these rows.

@history_item n=12 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — the paste became chunks on one queued writer thread, so contract/tui-passthrough/human-keystrokes and decision/human-keystrokes-never-blocked now bound a key's delay at the in-flight chunk and closing marker. Moving the restart code out of `pump.rs` must not touch that writer path or the pump's key ordering.

@history_item n=13 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — adopt joined run's shared start in `run.rs`. It widened contract/tui-passthrough/entry, contract/log-identity/process (`process:"run"` for an adopt start), decision/strict-modes-refusal-before-any-write, decision/socket-dir-refusal-before-any-frame, contract/json-fault/internal-error, contract/start-json-refusal/document and contract/cli-refusal/unable-line to cover adopt. Every caller of the start (`run`, `revive`, `adopt`, through them `open` and `up`) must keep these refusals and their order once the window and trust steps leave `run.rs`.

@history_item n=14 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — (the «Eight» leads standing and ratified, 4 entries) made decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver standing, their lead «Eight time bounds» the operator's own. The two lists are full at their bound: the ruling of 2026-11-22-epoch-4-cleanup gives any new bound a row of its own, and this chunk is held to adding none.

@history_item n=15 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** — added adopt's third own refusal, not-resumed, to contract/cli-refusal/unable-line and contract/start-json-refusal/document. It gave decision/wheel-ignores-terminal-events its second except: after the child ends, a not-resumed adopt's refusal pair or document reaches the terminal. That post-child write sits on the shared start and pump path the moves touch.

@history_item n=16 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — the window step chunk, whose `open_window` call in `run::start` this chunk moves next to `src/run/window.rs`. Its a11y change added `viola verify`'s untrusted-folder refusal to the fixed `unable:` / `hint:` form. The window code's logic is outside this chunk's boundary, so only paths and module names are re-pointed.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — added the start's socket-directory refusal (`refuse_socket_dir`, exit 1 before any snapshot, bind or child) on the start in `run.rs` that run and revive share, and the frame verbs' exit-21 `socket-dir-refused`. These are now the row decision/socket-dir-refusal-before-any-frame. It also added the tui boundary clause that a refused peer is dropped in `accept_loop` and writes nothing to the outer terminal. The move must keep the start's refusal order.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — put the strict-modes check first in `run` and revive's start (`checked_exe`, `refuse_tree` in `src/cmd/run.rs`), held by `run_refuses_a_widened_home_tree_before_its_log` with the `HOME_PAIR` / `INSTANCE_PAIR` literals. This is the origin of decision/strict-modes-refusal-before-any-write and contract/instance-unreachable-detail/strict-modes-failed. Those cited symbols live in the `run.rs` that this chunk splits.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — made every refusal of the start that `revive` shares with `run` print one exit-1 document (contract/start-json-refusal/document), with witnesses in `tests/cross_exit_causes.rs`. It also established that a `pump_child` error prints no document and viola writes nothing after it. It added the cause in exit 21's `detail` (contract/instance-unreachable-json/document) and the catch site's `internal-error` document (contract/json-fault/internal-error). The exit-cause table's rows read the moved code's refusals.

@history_item n=20 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — reworded the cli checks to `std::process::Command` with literal pins, which is the guard of decision/refusal-layout-literal-pins. It also moved the outer-PTY driver to portable-pty `=0.9.0` (decision/portable-pty-outer-pty-driver), which a11y never re-pins. A moved test module keeps its literal pins verbatim, and the outer-PTY cases keep their driver.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — the tui boundary is now hosted on openpty alone, with each of the three boundary clauses a nextest case of its own in the `coverage` suite on `ubuntu-latest`. The zero-viola-bytes case compares the stream byte for byte with an unwrapped fake-agent run. These are the green layers that judge the pure move of `pump.rs` and `run.rs`.

@history_item n=22 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the tui passthrough boundary after a contained vt100 feed panic (`tests/chaos_feed_panic.rs`): no viola literal reaches the outer terminal and the child's own bytes do. This is a pump-path witness that must stay green and unedited when the restart code leaves `pump.rs`.

@history_item n=23 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough through the same start as `run`. The one revive-specific clause, no line of its own while the child holds the terminal, is read by `tests/chaos_revive.rs`. This is part of contract/tui-passthrough/own-bytes and of the shared start whose boot path the `run.rs` move changes.

@history_item n=24 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — set the closed non-editing list (focus and mouse reports, terminal replies) behind decision/wheel-ignores-terminal-events, proven by the classifier's unit table and the step-wise outer-PTY case (3). These are the pump-side wheel semantics a move out of `pump.rs` must leave unchanged.

@history_item n=25 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — put the readiness gate's bound on the driver: a key during the gate wait takes the wheel and the waiting send is refused `human-typing`. The outer-PTY case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` is sequenced on the wrapper's records, never a timer. It is part of the human-wins guarantee behind decision/send-refused-while-human-typing, which a switch respawn moved out of the pump must still honour.

@history_item n=26 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — added the MCP call deadline to decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver, and added contract/instance-unreachable-detail/call-deadline. This is one step in how the two picked time-bound lists reached their present eight members.

@history_item n=27 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — dropped the budget pause from the two time-bound lists and retired exit 11. This is history of the picked rows decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver: a retired bound leaves the list and is never reused.

@history_item n=28 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — dropped the status-line deadline from both time-bound lists and removed the statusline passthrough case. This is history of the picked time-bound rows and of the tui passthrough cases that no longer exist for a moved start to keep.

@history_item n=29 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows, each verified against the code, with six guards re-pointed. Rows naming code paths and guards (the picked rows' `guard:` cells) are the sites a move re-points. The scope expects a master row that names a moved path to become a wrap amendment.
