@carried n=1 label=test-plan/e2e-test-strategy fact=fct-4m5t4osf
why: The chunk must land the v1-40 dim stale row's integration-tier witness under the outer PTY on Unix, asserting only on the SGR sequences of verb output.
quote: TTY rendering (bold NAME, amber `DIALOG`, dim `stale`) is checked by running `viola list` inside the portable-pty outer PTY and asserting only on the SGR sequences in verb output, never the child agent screen. As landed (chunk 2026-10-17-cli-output-discipline): bold NAME and amber `DIALOG` under the outer PTY (`cross_list_rows_style_each_span_beside_its_word_on_a_terminal`; on windows-2025 the parsed attribute set); dim `stale` at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), its outer-PTY witness a CARRY on "Linux and macOS parity".
@carried n=1 run=2 label=test-plan fact=fct-gnzxb6ex
quote: TTY rendering is asserted only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
@carried n=1 run=3 label=test-plan fact=fct-45jodiii
quote: E2E may block on a deadline inside the product that it cannot inject: the send confirmation window behind Path 2's `no-prompt-submitted`, `wait --timeout-ms` in Path 3, and heartbeat staleness in the chaos SIGSTOP case. Two conditions apply: the verdict is the product's own exit code or status field, and the same deadline is clock-driven in a §4 unit case.
@carried n=1 run=4 label=test-plan fact=fct-47fi4ugg
quote: a SIGSTOPped wrapper read `stale` and its name refused to a new `run` with `stale-heartbeat`
@carried n=2 label=test-plan/e2e-test-strategy fact=fct-tr6c4sml
why: The Unix outer-PTY driver over openpty must hold the same rules as over ConPTY, and the forced vt100 panic case must read the child's own bytes on Unix by byte search.
quote: portable-pty `=0.8.1` outer PTY (`tests/support/outer_pty.rs` over the `viola-pty` seam) | direct (ConPTY / openpty) | Exit only via `child.wait()`, and the master reader is drained on its own thread. Verdicts come from `events.ndjson`, the fake-agent receipt, the exit status and, in the forced vt100 panic case, a byte search of what the drained master reader read (the child's own bytes on Unix, viola's literals absent on every OS). |
@carried n=2 run=2 label=test-plan fact=fct-ktlsbc3h
quote: a portable-pty outer PTY over the `viola-pty` seam; exit only via `child.wait()`, the master reader drained on its own thread, verdicts from `events.ndjson`, the receipt and the exit status, and for the forced vt100 panic chaos case a byte search of the drained master reader's stream (the child's own bytes on Unix, no viola literal on every OS), never a parse of the rendered screen.
@carried n=2 run=3 label=test-plan fact=fct-47fi4ugg
quote: a forced vt100 panic refusing `send` `input-not-ready` while passthrough continues on the outer terminal (§1 security-vector-coverage (child spawn))
@carried n=3 label=test-plan/e2e-test-strategy fact=fct-6q7xvkh4
why: A channel case the parity suite runs over the Unix socket must take its endpoint only from snapshot.json, and its detail words read on the lines are the diagnostics the one-schema obligation holds on every OS.
quote: The endpoint is read only from `snapshot.json`. `tests/channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` writes seven fault rows to the channel of a wrapper booted with a planted value (through `OuterPty::spawn_without`'s `env_set`, as `tests/tui_env_strip.rs` does), its bodies clean and the three `detail` words read on the lines.
@carried n=3 run=2 label=test-plan fact=fct-uz5vlm6m
quote: the sync client, a raw interprocess stream and jsonschema; the endpoint is read only from `snapshot.json`.
@carried n=4 label=test-plan/e2e-test-strategy fact=fct-4khqx2uf
why: The Unix-only settings.json rewrite and socket directory check are OS-specific behaviours whose one-OS tests must name that reason where they stand, and the home-tree refusal in tests/run_cli.rs is a case of the reached file.
quote: The `settings.json` half of step 5 is Unix's. On Unix the restart rewrites the instance's `settings.json` whole at 0600 as `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}` with the absolute pinned path, and only when every character of that path is an ASCII letter, a digit or one of `_ - . / :`. On Windows no `settings.json` is written. Landed as `run_rewrites_the_settings_override_each_start` (`tests/cli_instance_state.rs`, with a `cfg!(windows)` branch) and the unit `statusline_override_of_a_start_is_rewritten_whole_at_owner_only` (`src/cmd/run.rs`).
    - The start runs the home-tree strict-modes check (`check_home_tree`: the home, `check_instance`'s set, `ledger/`, `bin/` and `plugin/`, then `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists) before the wrapper log opens. A refused tree exits 1 with the home-state `unable:` / `hint:` pair, or the instance-state pair when the refusal is at instance level, and opens no log, writes no line and starts no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`; `tests/security_negatives_strict_modes.rs` `a_home_another_user_may_read_refuses_run`).
    - On Unix the start checks the socket directory (`socket_dir_ready`) before any snapshot, child or `Server::bind`, `revive`'s start sharing the check: a refused directory exits 1, logs `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` (a step with no span, one line), binds nothing and spawns no child; a `Server::bind` failure is not this refusal (`tests/security_negatives_socket_dir.rs` `security_negative_a_start_refuses_the_socket_directory_before_any_bind`).
@carried n=4 run=2 label=test-plan/e2e-test-strategy fact=fct-l7jcwkqi
quote: 5. Tamper and rewrite: stop the first wrapper (Ctrl-C into the outer PTY, `child.wait()`). Overwrite one byte of the file at snapshot `pinned_bin`, and write a sentinel into the instance's `plugin/` hooks.json and `settings.json`. Then run `viola run builder --home H` again.
@carried n=4 run=3 label=test-plan fact=fct-mwv2sqc7
quote: a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup, and on Unix the `settings.json` override.
@carried n=4 run=4 label=test-plan fact=fct-tenptmlu
quote: on Unix a `claude_env_keep` name survives and the child holds no viola socket or home path
@carried n=4 run=5 label=test-plan fact=fct-jtonrpse
quote: `budget.json` holds the fed reading and, on Unix, the statusline output passes through byte for byte
@carried n=4 run=6 label=test-plan fact=fct-4aawmv2h
quote: on Linux and Windows, a server that is not the recorded wrapper and, on Unix, a socket directory the check refuses or a server of another euid (its decision held by injected euids, §1 Untestable zones)
@carried n=4 run=7 label=test-plan fact=fct-ndx7lm2p
quote: beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (macOS compares no peer pid, the named carve-out `server_verdict_rows` holds, so these negatives compile out there; a loosened socket directory on Linux and macOS is a case of `tests/security_negatives_socket_dir.rs`)
@carried n=5 label=test-plan/e2e-test-strategy fact=fct-qdldsdqk
why: A flow the chunk adds or moves onto the Unix runners, the v1-40 witness above all, must stay inside the mutants profile's 10 s kill and the ci profile's 20 s kill line, as this measured flow does.
quote: steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs`, `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` `required-features = ["fake-agent"]`; in the E2E layer, `nextest-e2e`; on the `ci` profile's verify-driven 20 s kill line), every signal above asserted, the `settings.json` half Unix's, its start-refusal helpers shared with `tests/cli_instance_state.rs` through `tests/support/start.rs`. The flow lives on the root chain, not a `viola_e2e::fixtures` copy (fork 1, `inputs#I1`); its wall read 4.69 s (5.14 s instrumented) on the Linux dev host against the `mutants` profile's 10 s kill.
@carried n=6 label=test-plan/e2e-test-strategy fact=fct-qbytqn26
why: v1-53's live Linux leg is read against the live readings earlier chunks took on the Linux dev host, which stay local measurements and never a suite, while this chunk claims no real-CLI behaviour from a green fake-agent run.
quote: As measured on live `claude` 2.1.287 on the Linux dev host at chunk 2026-10-08-first-live-test-and-self-drive (`evidence/live-readings.ndjson`; a local live measurement, no suite): a send ending in two newlines is confirmed, `text_bytes` 31 of 33 sent, the prompt filed `driver`, no `send-refused` and no `wheel` record; `/clear` and one newline is confirmed by a new session.
@carried n=6 run=2 label=test-plan/e2e-test-strategy fact=fct-mdqnbyad
quote: On live `claude` 2.1.296 on the Linux dev host, a local measurement and no suite, a product `wait --json` woke on `turn-ended` with `background_tasks` 0 and `last` read the turn (as measured at `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-readings.ndjson`).
@carried n=6 run=3 label=test-plan/e2e-test-strategy fact=fct-p2i2onck
quote: On live `claude` 2.1.296 on the Linux dev host, a local measurement and no suite, a `question` carried descriptions and previews and a product `viola answer` answered it with `dialog-answered{deadline_hit:false}` and no `from`, its process carrying no `VIOLA_NAME` (as measured at `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-readings.ndjson`).
@carried n=6 run=4 label=test-plan fact=fct-6ugas3oa
quote: A route entry's live readings of a real session are that chunk's own local measurement, kept in its `evidence/` with its own launcher, driver and readers (as at chunk 2026-10-08-first-live-test-and-self-drive): not a test layer, not harness surface, never in CI.
@carried n=6 run=5 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=6 run=6 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=6 run=7 label=test-plan fact=fct-x7oaiapn
quote: macOS local runs, which are CI-only.
@carried n=7 label=test-plan/e2e-test-strategy fact=fct-sefkfuph
why: The Unix reading of a killed wrapper and its child must verify each gone by pid and start time, never by a sleep or a missing file.
quote: killed wrappers verified gone via sysinfo pid + start time.
@carried n=8 label=test-plan/e2e-test-strategy fact=fct-3r2a46vd
why: The wheel CARRY's reply shapes are measured on foot alone and the outer-PTY tier writes no terminal reply, so what of it the fake agent can read is P4's, and any new reply shape joins the list only by its measured grammar.
quote: The list's reading on a real terminal is a local live measurement, not a suite, because the outer-PTY tier writes no terminal reply of its own: as measured on foot 1.28.0 on the Linux dev host at that chunk (`evidence/reply-probe.ndjson`, `evidence/reply-probe-fixed.ndjson`), seven replies were read as typing before the seven joined and none of 23 queries after, and the keys `n`, `t` and `m` typed with `wtype` each took the wheel.
@carried n=8 run=2 label=test-plan/e2e-test-strategy fact=fct-3r2a46vd
quote: on `windows-2025` the focus case asserts the platform fact of the founder's live ruling F-W3 (an injected mouse report takes the wheel, focus reports are swallowed), Unix the full assertion.
@binding n=1
**e2e-test-strategy (v1-40 outer-PTY witness) ↔ quality gates (mutants 10 s kill) and the heartbeat**: the Unix witness of a stale `viola list` row must read SGR dim from the row's first byte to its last under the outer PTY (test-plan/e2e-test-strategy fct-4m5t4osf), reach staleness only through the product's 1 s beat and 5 s threshold with its verdict the product's own status (test-plan fct-w7y4wg56) (test-plan fct-45jodiii), and keep its wall inside the mutants profile's 10 s kill as the measured Path 1 flow does (test-plan/e2e-test-strategy fct-qdldsdqk).
@binding n=2
**e2e-test-strategy (tui outer-PTY driver) ↔ integration (Module ↔ PTY) and unit (viola-pty seam)**: the outer-PTY driver on openpty must read exit only by `child.wait()` and drain the master on its own thread (test-plan/e2e-test-strategy fct-tr6c4sml), over the same real openpty spawns that must propagate a resize and write a key only once the child reports the new size (test-plan fct-nvtf2boy), with exit read on the handle and never on reader EOF (test-plan fct-sbml5vi6).
@binding n=3
**e2e-test-strategy (Unix-only halves of Paths 1 and 6 and E2) ↔ per-OS gating and the per-OS coverage gate**: each one-OS case (the `settings.json` rewrite, the statusline pass-through, the `claude_env_keep` survival, the socket directory check) must carry its OS reason where it stands (test-plan/e2e-test-strategy fct-4khqx2uf), and a case moved onto the Unix runners changes what each Unix job's coverage gate counts (test-plan fct-ryyndnld).
@binding n=4
**e2e-test-strategy (server identity and socket directory negatives) ↔ the macOS carve-out**: the server-identity negatives must stay on Linux and Windows, compiled out on macOS where no peer pid is compared, while the socket directory refusals run on Linux and macOS (test-plan fct-ndx7lm2p) (test-plan fct-4uc5ehox); no parity test may read the macOS check as a comparison.
@binding n=5
**e2e-test-strategy (live Linux dev-host readings) ↔ anti-patterns and v1-53's live leg**: the live Linux readings stand as local measurements v1-53's live leg is read against, never re-made in this chunk (test-plan/e2e-test-strategy fct-qbytqn26), and no green fake-agent or Linux-only local run may stand as proof of real-CLI or macOS behaviour, every macOS reading coming from `macos-latest` (test-plan fct-egysilpo) (test-plan fct-lktzdsoq).
@binding n=6
**e2e-test-strategy (killed wrappers gone by pid and start time) ↔ test data (the fake agent's Unix process groups and SIGHUP-ignoring exit)**: the Unix child-ends-with-its-wrapper reading must hold the recorded child gone by pid and start time (test-plan/e2e-test-strategy fct-sefkfuph) while the fake agent's own process groups and SIGHUP-ignoring exit decide what a killed wrapper's hangup reaches (test-plan fct-7o4nbra2).
@binding n=7
**e2e-test-strategy (channel detail words on the lines) ↔ schema conformance (G4)**: the `detail` words a channel case reads on the diagnostics lines (test-plan/e2e-test-strategy fct-6q7xvkh4) must validate against `diag-line.v1.json` and `diag-detail.v1.json` on each OS's runner, the same schema on every OS (test-plan fct-nfrombzn).
@binding n=8
**e2e-test-strategy (the wheel's reply list) ↔ the fake agent**: the outer-PTY tier writes no terminal reply, so any reading of the CARRY under the fake agent is P4's to choose, and a reply shape joins the closed list only when measured by its exact grammar on the founder's word (test-plan/e2e-test-strategy fct-3r2a46vd).
