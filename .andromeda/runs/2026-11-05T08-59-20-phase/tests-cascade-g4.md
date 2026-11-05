@carried n=1 label=test-plan/e2e-test-strategy fact=fct-4m5t4osf
why: The chunk must remove the windows-2025 arm of the outer-PTY style case and the macos-latest leg of the dim-stale case, ungating `tests/cross_list_rows.rs` where its `cfg(unix)` gate loses its only sibling, with every Linux SGR assertion kept as it is.
quote: As landed (chunk 2026-10-17-cli-output-discipline): bold NAME and amber `DIALOG` under the outer PTY (`cross_list_rows_style_each_span_beside_its_word_on_a_terminal`; on windows-2025 the parsed attribute set); dim `stale` at the unit tier (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`) and under the outer PTY on `ubuntu-latest` and `macos-latest` (`cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, `tests/cross_list_rows.rs`, `cfg(unix)`; `list` row integrity).
@carried n=2 label=test-plan/e2e-test-strategy fact=fct-tr6c4sml
why: The tui driver's ConPTY mode leaves with the Windows code, so the outer PTY must stay openpty alone, with the forced vt100 panic case keeping both its Unix child-bytes witness and its no-viola-literal witness, and the `.cmd` refusal must stay only where P4 decides it still names live code.
quote: direct (ConPTY / openpty) | Exit only via `child.wait()`, and the master reader is drained on its own thread. Verdicts come from `events.ndjson`, the fake-agent receipt, the exit status and, in the forced vt100 panic case, a byte search of what the drained master reader read (the child's own bytes on Unix, viola's literals absent on every OS). |
@carried n=2 run=2 label=test-plan fact=fct-ktlsbc3h
quote: for the forced vt100 panic chaos case a byte search of the drained master reader's stream (the child's own bytes on Unix, no viola literal on every OS), never a parse of the rendered screen.
@carried n=2 run=3 label=test-plan fact=fct-47fi4ugg
quote: a forced vt100 panic refusing `send` `input-not-ready` while passthrough continues on the outer terminal (§1 security-vector-coverage (child spawn)), forward-compat lines counted in `skipped`, a start over a truncated or `v:99` snapshot replaying the log and a fresh beat over the replayed snapshot refusing the start `already-live` (`chaos_snapshot_replay.rs`), and a `.cmd` child refused exit 1.
@carried n=2 run=4 label=test-plan fact=fct-ukay3vyw
quote: NEVER detect child exit by reading master EOF; use `child.wait()` on the handle (ConPTY).
@carried n=3 label=test-plan/e2e-test-strategy fact=fct-4khqx2uf
why: The chunk must remove the `cfg!(windows)` branch of `run_rewrites_the_settings_override_each_start` and make the Unix `settings.json` write unconditional, keeping on Linux the whole rewrite at 0600, the absolute pinned path and its path-character rule.
quote: The `settings.json` half of step 5 is Unix's. On Unix the restart rewrites the instance's `settings.json` whole at 0600 as `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}` with the absolute pinned path, and only when every character of that path is an ASCII letter, a digit or one of `_ - . / :`. On Windows no `settings.json` is written. Landed as `run_rewrites_the_settings_override_each_start` (`tests/cli_instance_state.rs`, with a `cfg!(windows)` branch) and the unit `statusline_override_of_a_start_is_rewritten_whole_at_owner_only` (`src/cmd/run.rs`).
@carried n=3 run=2 label=test-plan/e2e-test-strategy fact=fct-qdldsdqk
quote: every signal above asserted, the `settings.json` half Unix's, its start-refusal helpers shared with `tests/cli_instance_state.rs` through `tests/support/start.rs`.
@carried n=3 run=3 label=test-plan fact=fct-mwv2sqc7
quote: a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup, and on Unix the `settings.json` override.
@carried n=3 run=4 label=test-plan fact=fct-tenptmlu
quote: on Unix a `claude_env_keep` name survives and the child holds no viola socket or home path, and the secret scan finds none of the canary values.
@carried n=3 run=5 label=test-plan/e2e-test-strategy fact=fct-l7jcwkqi
quote: 5. Tamper and rewrite: stop the first wrapper (Ctrl-C into the outer PTY, `child.wait()`). Overwrite one byte of the file at snapshot `pinned_bin`, and write a sentinel into the instance's `plugin/` hooks.json and `settings.json`. Then run `viola run builder --home H` again.
@carried n=4 label=test-plan/e2e-test-strategy fact=fct-4khqx2uf
why: The socket-directory check is a Unix arm whose Linux refusal (exit 1, the one `socket-dir-refused` line, no bind, no child) must hold unchanged while its negatives lose their macOS leg and the server-identity and Windows channel negatives lose their Windows legs.
quote: On Unix the start checks the socket directory (`socket_dir_ready`) before any snapshot, child or `Server::bind`, `revive`'s start sharing the check: a refused directory exits 1, logs `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` (a step with no span, one line), binds nothing and spawns no child; a `Server::bind` failure is not this refusal (`tests/security_negatives_socket_dir.rs` `security_negative_a_start_refuses_the_socket_directory_before_any_bind`).
@carried n=4 run=2 label=test-plan fact=fct-ndx7lm2p
quote: beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (macOS compares no peer pid, the named carve-out `server_verdict_rows` holds, so these negatives compile out there; a loosened socket directory on Linux and macOS is a case of `tests/security_negatives_socket_dir.rs`), the strict-modes cause of exit 21 for the six frame verbs is a case of `tests/security_negatives_strict_modes.rs` on three OSes
@carried n=4 run=3 label=test-plan fact=fct-4uc5ehox
quote: the Unix socket directory refusal (`tests/security_negatives_socket_dir.rs`, Linux and macOS: a directory at 0755 or a symlink to a 0700 directory refuses `viola run` exit 1 before any bind, the eight frame verbs exit 21 with no frame and the hook exit 0 with empty streams and no frame, each leaving it as found and naming no path, pid or uid, and a clean start under an open umask keeps the socket private), Windows client SQOS at the identification level, the Windows listener rejecting remote clients and denying an anonymous open (each beside its controls),
@carried n=4 run=4 label=test-plan fact=fct-4uc5ehox
quote: a Windows x64 ConPTY companion another user may write leaving the child on the inbox ConPTY (`tests/conpty_sideload.rs`), and, on Linux and Windows, a CLI frame verb writing no frame to a server that is not the recorded wrapper (`tests/security_negatives_server_identity.rs`, beside its control reaching the recorded wrapper).
@carried n=4 run=5 label=test-plan fact=fct-4aawmv2h
quote: on Linux and Windows, a server that is not the recorded wrapper and, on Unix, a socket directory the check refuses or a server of another euid (its decision held by injected euids, §1 Untestable zones), each exit 0 with no body
@carried n=5 label=test-plan/e2e-test-strategy fact=fct-n6muav3b
why: Shrinking the CI matrix to ubuntu-latest makes every landed path suite's leg on all three CI OSes a Linux-only leg, which the plan must name among the per-OS legs the wrap's test-plan amendment rewrites, every suite still green on Linux.
quote: `tests/cli_send.rs` `path2_send_confirms_with_cl1_events` covers cli, ipc-internal (the wrapper channel) and tui (the receipt) on all three CI OSes;
@carried n=5 run=2 label=test-plan/e2e-test-strategy fact=fct-mdqnbyad
quote: `tests/cli_wait_last.rs` covers cli and ipc-internal (the wrapper channel) on all three CI OSes,
@carried n=5 run=3 label=test-plan/e2e-test-strategy fact=fct-p2i2onck
quote: `tests/cli_answer.rs` over `fixtures/fake-scripts/path4.json` covers cli and ipc-internal (the wrapper channel) on all three CI OSes:
@carried n=5 run=4 label=test-plan/e2e-test-strategy fact=fct-kylcwwrw
quote: `path6_list_shows_the_reading_and_the_pause` (`tests/cli_budget.rs`), green on all three CI runners, `test (windows-2025)` included.
@carried n=5 run=5 label=test-plan fact=fct-x7oaiapn
quote: macOS local runs, which are CI-only.
@carried n=5 run=6 label=test-plan fact=fct-dcxdz6xw
quote: Non-persistent-ACL volumes and foreign-owner SIDs. Covered by synthetic SDDL descriptor stubs (§4).
@carried n=5 run=7 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=6 label=test-plan/e2e-test-strategy fact=fct-qbytqn26
why: The plan must name as an expected amendment the re-pointing of this master hit, whose Windows paste reading is owed to the retired route entry "Windows-only live measurements", and of the unmeasured Windows paste shape beside it.
quote: stamped on live `claude` 2.1.287 on the Linux dev host there, `stamped 2.1.287  18 pass  0 fail`; its Windows reading is owed to "Windows-only live measurements").
@carried n=6 run=2 label=test-plan/e2e-test-strategy fact=fct-qbytqn26
quote: Not measured: any CLI version but 2.1.287, a long or wrapped multi-line text, an LF typed inside a paste on Windows, and what the CLI does with a typed CR CR or LF CR (no `send` types one).
@carried n=7 label=test-plan/e2e-test-strategy fact=fct-lhsyucqg
why: The chunk must remove the Windows `OpenProcess` + `TerminateProcess` arm of this case and keep its Unix arm unconditional, still asserting the wrapper's exit after `kill -KILL` and the child gone within `WITHIN`.
quote: `tests/run_cli.rs` `run_child_ends_when_its_wrapper_is_terminated`, on every OS, terminates the wrapper (Windows `OpenProcess` + `TerminateProcess`; Unix `kill -KILL <wrapper pid>`, its exit asserted) and asserts that the child, its pid and start time read from `snapshot_data`, is gone within `WITHIN`.
@carried n=8 label=test-plan/e2e-test-strategy fact=fct-3r2a46vd
why: The chunk must remove the windows-2025 platform-fact arm of the focus case, leaving the Unix full assertion (focus, mouse and resize never take the wheel) as its only, ungated assertion.
quote: cover tui, cli and ipc-internal (the wrapper channel) on all three CI OSes; on `windows-2025` the focus case asserts the platform fact of the founder's live ruling F-W3 (an injected mouse report takes the wheel, focus reports are swallowed), Unix the full assertion.
@carried n=9 label=test-plan/e2e-test-strategy fact=fct-kn7ryhs7
why: The statusline pass-through is a Unix arm the chunk makes unconditional, so on Linux the marker equality, the `/bin/sh -c <command>` run and its 5 s bound must hold unchanged while the Windows no-override, no-command arm and its `cfg(not(unix))` and `cfg(unix)` gates leave.
quote: The marker equality is Unix's: there `run` writes the instance's `settings.json` override and the arm runs the recorded command as `/bin/sh -c <command>` under a 5 s bound (PROVISIONAL). On Windows `run` writes no override and the arm runs no command (`override_document` and `shell_argv` return `None` for every input). Exit 0 with empty stderr holds on every OS.
@carried n=9 run=2 label=test-plan/e2e-test-strategy fact=fct-kylcwwrw
quote: on Unix the statusline arm prints the marker output and files the marker, and on Windows (`cfg(not(unix))`, run on `windows-2025` in CI) it prints nothing and files no marker.
@carried n=9 run=3 label=test-plan/e2e-test-strategy fact=fct-kylcwwrw
quote: Step 6 is `hook_statusline_a_home_another_user_can_write_runs_nothing_however_often` in `tests/hook_statusline.rs` (`cfg(unix)`, two invocations, no role line added).
@carried n=9 run=4 label=test-plan/e2e-test-strategy fact=fct-bo4nzuwn
quote: (steps 1 and 7 over `harness_session` booted with the flag; the planted source read 0600 on Unix)
@carried n=9 run=5 label=test-plan/e2e-test-strategy fact=fct-bo4nzuwn
quote: Chmod the home to 0770 (Unix) and pipe statusline stdin again.
@carried n=9 run=6 label=test-plan fact=fct-jtonrpse
quote: `budget.json` holds the fed reading and, on Unix, the statusline output passes through byte for byte;
@binding n=1
**e2e-test-strategy ↔ ci-integration**: the CI matrix of ubuntu-latest alone turns every e2e suite landed on all three CI OSes into a Linux leg, and the plan names the per-OS legs as expected amendments while each suite stays green on ubuntu (test-plan/e2e-test-strategy fct-n6muav3b) (test-plan fct-shyrfs6v)
@binding n=2
**e2e-test-strategy ↔ the scope's Linux-behaviour boundary**: each Unix arm made unconditional (the `settings.json` override, the statusline pass-through, the socket-directory check, the `kill -KILL` termination) keeps its Linux assertion word for word, and only the Windows and macOS siblings leave (test-plan/e2e-test-strategy fct-4khqx2uf) (test-plan/e2e-test-strategy fct-kn7ryhs7) (test-plan/e2e-test-strategy fct-lhsyucqg)
@binding n=3
**e2e-test-strategy ↔ security negatives**: the socket-directory negatives keep their Linux leg and lose macOS, and the server-identity negatives keep Linux and lose Windows, so `server_verdict_rows`' macOS carve-out and the Windows channel negatives leave with their gates (test-plan fct-ndx7lm2p) (test-plan fct-4uc5ehox)
@binding n=4
**e2e-test-strategy ↔ quality-gates-coverage-targets**: removing the Windows arms of the e2e cases changes what the per-OS coverage gate measures on ubuntu, where the `cfg(unix)` code ungated stays covered and the ignore regex is never widened (test-plan fct-ryyndnld) (test-plan fct-b4avhaux)
