# Task — design · 2026-10-25-windows-endpoint-admission

## The chunk
Asserts the Windows named-pipe listener's admission on the Windows CI leg: only the owning user's SID and SYSTEM are admitted, a remote (network) pipe client is rejected, a second listener on a name that is already held fails (first-pipe-instance), and handles are not inheritable.
Its new work is three assertions: the pipe's remote-client rejection mode read back, first-instance as a property of the listener apart from the CLI squat refusal, and the DACL's effect (an open by a principal outside the two ACEs is denied) where a Windows CI runner can measure it.
It re-verifies, and does not rebuild, what is already asserted: the protected DACL read back as two allow ACEs, non-inheritable handles on both ends, and the `squatted-name` exit-1 start refusal of `viola run`.
It also takes the CARRY on the `budget-paused` send-refusal test case that went red once under plain `cargo test`. It either runs that case three times under plain `cargo test` or fixes the case's log capture.
No wire contract, exit code or refusal word changes: `squatted-name` and exit 1 stand.

## Reached files
- crates/viola-channel/src/server.rs · edits the doc comment on remote-client rejection (`PIPE_REJECT_REMOTE_CLIENTS`)
- crates/viola-channel/src/server/win.rs · edits its unit tests (`owner_only_sddl_is_protected_user_and_system`, `bound_pipe_dacl_reads_back_protected_user_and_system`, `pipe_handles_on_both_ends_are_not_inheritable`)
- crates/viola-channel/src/endpoint.rs
- tests/security_negatives_channel.rs
- src/cmd/run.rs · edits `refuse_squatted`
- tests/path1_run_start.rs
- tests/cli_instance_state.rs
- src/run/send/budget.rs · edits the test `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` (its log capture)

## Needs
- The cli surface's `viola run` start refusal for a squatted name: one fixed-message `unable:` line on stderr, then that cause's one `hint:` line. Its wording and exit 1 are re-asserted unchanged.
- The rule that a `run` start refusal shows no path, pipe name or pid in its line or hint. It reaches the chunk through `refuse_squatted`, since the squatted endpoint name is never printed.
- The exit-code phraseology for exit 1: a fixed-message `unable:` refusal with its hint, as `run`'s start refusals print it. Exit 1 must not move to another typed code.
- The rule that a refusal `run`'s start makes prints no `--json` document. That document is pending under the route entry "Exit-cause code catalogue", so this chunk adds none for `squatted-name`.
- The colour decision order's `viola run` row: `run` prints no colour, glyph or SGR, and once the child starts it prints nothing. A start refusal is plain ASCII on stderr.
- The stream rule: refusals and their `hint:` lines go to stderr, results to stdout, and the two never mix.
- The ban on stack traces and anyhow chains in human output. A pipe-creation failure at start reaches the human only as a fixed message, and full detail goes only to `diagnostics/`.
- The phraseology word `budget-paused` and the `unable` refusal with reason and detail on a `send`. The re-greened test case asserts that the refusal carries the window word and no cursor, and the chunk changes only the case's log capture, not the refusal's wording.
- The rule that `hook` never writes stderr and `mcp` writes only MCP frames on stdout. Both are clients of the pipe whose admission the chunk asserts, so an admission failure gives neither of them a human surface.

## Not needed
- Color Palette (core colours, surface scale, text hierarchy, semantic and domain status colours, border progression): a 0.2.0 frontend design input, and the chunk renders no page.
- Typography, Spacing, Depth Strategy, Border Radius, Motion and Iconography: 0.2.0 frontend inputs or CLI styling, and none of them is reached by a pipe-admission or test-capture change.
- Self-Validation Protocol, the Token Test included: it applies to UI output, and the chunk produces none.
- The `viola list` board (strip table, BAY line, unwrapped rows, truncation, SGR dim and amber): `list` is not touched.
- The `viola send` readback mirror's success states (`[RB] read back`, `[  ] open`, `unconfirmable`, the `/clear` and `/compact` post-conditions): the chunk changes no send delivery or confirmation.
- `viola wait` / `viola last` result lines and their message-mode control escaper: not touched.
- The exit-21 `instance-unreachable` refusals of the client verbs and the client's server-identity check: they belong to the next route entry, "Server verification before any frame".
- `viola verify`'s step-counter output and the capability-ledger rows: the chunk adds no ledger row or probe.
- The wheel, link, gate and `plugin install` verb outputs, and `viola --help`'s verb rows: none of them is touched.
- `viola revive`'s refusals and its `--json` documents: `revive` is not touched.
- Brand Identity anchors (strips, cocked strip, ATIS age, fuel state), the expression level and the design direction: they do not apply to an endpoint-admission change.
- Platform notes on Git Bash path rewriting and on conhost without VT: the chunk changes no argument intake and no colour enabling.
