@carried n=1 label=test-plan/e2e-test-strategy fact=fct-4m5t4osf
why: Item 1's masters-agree check must decide between the trycmd tool and the text that names it, and the edit to tests/cli_output_plain.rs's Windows remnants must keep the `viola --help` literal holding.
quote: | cli (verbs + `--json`) | assert_cmd 2.2.2 + predicates 3.1.4; trycmd 1.2.1 for human layouts other than the `viola list` board, `viola --help` and the `viola link` / `viola unlink` result lines, pinned by literal in `tests/cli_list.rs`, `tests/cli_output_plain.rs` and `tests/cli_links.rs` (`link_verbs_print_the_design_lines`) | direct |
@carried n=1 run=2 label=test-plan/e2e-test-strategy fact=fct-qbytqn26
quote: Human mode, verified with trycmd-redacted assert_cmd: stdout `[RB] read back … cursor <n>`
@carried n=1 run=3 label=test-plan fact=fct-gnzxb6ex
quote: **cli (verbs + `--json`):** assert_cmd with predicates, and trycmd or literals for human layouts;
@carried n=1 run=4 label=test-plan fact=fct-t57awnjr
quote: N/A — no frontend in 0.1.0: the frontend is deferred to 0.2.0; its technology is not chosen.
@carried n=2 label=test-plan/e2e-test-strategy fact=fct-tr6c4sml
why: The portable-pty move off `=0.8.1` must leave the outer-PTY driver's exit read on `child.wait()` and its drained reader working, and this row's version text must follow the new pin.
quote: | tui (`viola run` passthrough) | portable-pty `=0.8.1` outer PTY (`tests/support/outer_pty.rs` over the `viola-pty` seam) | direct (openpty) | Exit only via `child.wait()`, and the master reader is drained on its own thread.
@carried n=2 run=2 label=test-plan fact=fct-ktlsbc3h
quote: **tui (`viola run` passthrough):** a portable-pty outer PTY over the `viola-pty` seam; exit only via `child.wait()`, the master reader drained on its own thread
@carried n=2 run=3 label=test-plan fact=fct-ukay3vyw
quote: NEVER detect child exit by reading master EOF; use `child.wait()` on the handle.
@carried n=3 label=test-plan/e2e-test-strategy fact=fct-okzgoz3x
why: The test gate the chunk must read green on Linux runs through these drivers, the doctests included.
quote: | library-only | cargo-nextest 0.9.146 + `cargo test --doc` | direct | Covered in §4. |
@carried n=4 label=test-plan/e2e-test-strategy fact=fct-mdqnbyad
why: The chunk's live session in which a driver drives a builder through viola must be recorded the same way: a local measurement on the Linux dev host, kept in the chunk's evidence, and no suite.
quote: On live `claude` 2.1.296 on the Linux dev host, a local measurement and no suite, a product `wait --json` woke on `turn-ended` with `background_tasks` 0 and `last` read the turn (as measured at `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-readings.ndjson`).
@carried n=4 run=2 label=test-plan fact=fct-6ugas3oa
quote: A route entry's live readings of a real session are that chunk's own local measurement, kept in its `evidence/` with its own launcher, driver and readers (as at chunk 2026-10-08-first-live-test-and-self-drive): not a test layer, not harness surface, never in CI.
@carried n=5 label=test-plan/e2e-test-strategy fact=fct-p2i2onck
why: Item 2's live reading of a dialog hold near the 3600 s ceiling must be recorded the way this live dialog answer is: a local measurement on the Linux dev host, with its evidence named, and no suite.
quote: On live `claude` 2.1.296 on the Linux dev host, a local measurement and no suite, a `question` carried descriptions and previews and a product `viola answer` answered it with `dialog-answered{deadline_hit:false}` and no `from`, its process carrying no `VIOLA_NAME` (as measured at `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-readings.ndjson`).
@carried n=6 label=test-plan/e2e-test-strategy fact=fct-lhsyucqg
why: Freight item 11 keeps this fake-agent reading standing for the legs the earlier chunk could not take, and the portable-pty move must leave it green; a green fake-agent run is no proof of the real CLI's behaviour.
quote: **A terminated wrapper's child:** `tests/run_cli.rs` `run_child_ends_when_its_wrapper_is_terminated` terminates the wrapper (`kill -KILL <wrapper pid>`, its exit asserted) and asserts that the child, its pid and start time read from `snapshot_data`, is gone within `WITHIN`.
@carried n=6 run=2 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=7 label=test-plan/e2e-test-strategy fact=fct-3r2a46vd
why: Freight item 10's hypothesis about this closed list needs a live-terminal reading, and a shape joins the list only when it has been measured and the founder gives his word; the foot 1.28.0 record must stay the measured basis until then.
quote: As landed (chunk 2026-10-08-first-live-test-and-self-drive), the stdin classifier's closed non-editing list gained seven terminal reply shapes, pinned at the unit tier in `src/run/wheel.rs`: the rstest table `classifier_terminal_reply_is_not_editing` (eight reply cases), the whole-and-split test over the 23 `REPLIES` strings, and 27 negative controls in `classifier_typing_is_editing` that stay typing (the nearest human keys, and a sequence one field or one prefix away from each shape).
  The list's reading on a real terminal is a local live measurement, not a suite, because the outer-PTY tier writes no terminal reply of its own: as measured on foot 1.28.0 on the Linux dev host at that chunk (`evidence/reply-probe.ndjson`, `evidence/reply-probe-fixed.ndjson`), seven replies were read as typing before the seven joined and none of 23 queries after, and the keys `n`, `t` and `m` typed with `wtype` each took the wheel.
@binding n=1
**e2e cli driver (trycmd) ↔ the masters-agree check of item 1 and the remnant edit of item 17**: the tool-or-text decision must make the cli driver row, Path 2's human-mode line and the body's driver line all name the same thing: either trycmd 1.2.1 is added with `tests/cmd`, or the text is reworded to the literal and assert_cmd pins that stand. The `HELP` literal in `tests/cli_output_plain.rs` must keep holding after lines 104 and 116 are edited (test-plan/e2e-test-strategy fct-4m5t4osf) (test-plan/e2e-test-strategy fct-qbytqn26) (test-plan fct-gnzxb6ex)
@binding n=2
**e2e tui driver ↔ the dependency move under viola-pty (Cargo.toml, Cargo.lock, crates/viola-pty)**: the portable-pty release chosen to drop `pin-utils` must keep the outer PTY's exit read on `child.wait()`, never on master EOF, with the reader drained on its own thread. The driver row's `=0.8.1` text must follow the new pin, and `run_child_ends_when_its_wrapper_is_terminated` must stay green across the move (test-plan/e2e-test-strategy fct-tr6c4sml) (test-plan fct-ukay3vyw) (test-plan/e2e-test-strategy fct-lhsyucqg)
@binding n=3
**e2e live-measurement records ↔ the chunk's two headless live sessions (v1-53's live `viola run` and the driver driving a builder)**: each live reading the chunk takes, including the dialog hold near the 3600 s ceiling and any wheel-list shape, must be a local measurement on the Linux dev host. It is kept in the chunk's `evidence/` with its own launcher, driver and readers, and it is never a suite, a harness surface or a CI step. A fake-agent green never stands in for it (test-plan/e2e-test-strategy fct-mdqnbyad) (test-plan/e2e-test-strategy fct-p2i2onck) (test-plan/e2e-test-strategy fct-3r2a46vd) (test-plan fct-6ugas3oa) (test-plan fct-egysilpo)
@binding n=4
**e2e library-only driver ↔ the hint's «every gate green on Linux»**: the test gate is read green through cargo-nextest 0.9.146 and `cargo test --doc` on `ubuntu-latest`, beside CI's other six jobs (test-plan/e2e-test-strategy fct-okzgoz3x)
