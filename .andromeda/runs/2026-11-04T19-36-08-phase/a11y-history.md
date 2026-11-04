@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** (Platform: ci.yml is the one push/PR workflow) — §9 Platform reads "one push/PR workflow `ci.yml`"; `nightly.yml` carries no a11y step. Any Unix a11y/boundary case this chunk adds or ungates runs in `ci.yml`'s three-OS matrix, never in nightly.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** (CLI output-discipline evidence under the CI `coverage` suite) — the CLI output-discipline tests run on all three OS legs and in CI report inside the per-OS `test` job's single `coverage` suite under `gate --require coverage,doctest`. Origin of the §9 binding: Unix parity cases join that suite, not a new one.

@history_item n=3 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (Windows zero-viola-bytes oracle is literal absence) — split the zero-viola-bytes clause: viola's literals absent on every leg; byte-for-byte against an unwrapped run holds on Linux and macOS only (ConPTY emits its own preamble). §6 became "zero SGR of viola's own". This chunk's openpty wrapper is where the full byte-for-byte oracle must actually hold; the Windows literal-absence oracle must not move.

@history_item n=4 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants (P4, P6) in crate tests meet mutation only at the epoch-boundary `run --mutants` audit. Context for the stale-dim CARRY: it was held at the unit tier because the `mutants` profile's 10 s kill left no room beside the 5 s stale threshold; an outer-PTY witness here must stay workable under that profile.

@history_item n=5 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** (the Windows zero-viola-bytes check names both ConPTY hosts) — Windows check (viola's literals absent) runs on both the sideloaded `OpenConsole.exe` and the inbox fallback; §6 credits both hosts' SGR/cursor/query bytes. Windows stays the live-supported target: no Unix case may move this oracle.

@history_item n=6 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** (registry migration U35) — lifted into §3 tool pick: the tests' portable-pty `=0.8.1` pin is an inherited risk (0.9.0 maintained) that a11y never re-pins (D-A11Y-13). The openpty backend and the outer-PTY driver both sit on that crate; a re-pin is not a11y's to make.

@history_item n=7 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** (terminal replies non-editing, the Windows platform fact) — non-editing set became the closed list (focus, mouse X10/SGR/urxvt, replies DA1, DA2, CPR, DECRPM, kitty flags, OSC, DCS); case (3) landed step-wise (resize behind its size receipt, focus with a mouse read barrier, wheel probed by an `answer`, exit 13 driver / 10 human), full assertion on Linux and macOS, F-W3 platform fact on windows-2025. Founder rulings F-W2/F-W3. Defines the P4 clause (3) the Unix legs must pass and the list the CARRY cannot widen without measurement.

@history_item n=8 marker=2026-10-07-t05-47-07-wrap
**2026-10-07-t05-47-07-wrap** — a real Windows terminal's mouse report moved to the Epoch 7 entry "Windows-only live measurements" (no interactive Windows host). That entry keeps its block; this chunk takes none of it.

@history_item n=9 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `GATE_MAX_WAIT` 8.5 s bound falls on the driver only; added the outer-PTY case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` under boundary clause (2), sequenced on the `channel-request` line and `wheel` record, never a timer, with its stated limit. The acceptance's "readiness-gate case exiting 10 `human-typing`" on ubuntu/macos is this case.

@history_item n=10 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — extended the non-editing replies by seven exact-grammar `CSI` shapes, proven by the unit table plus the reply probe on foot 1.28.0; recorded one supplemental live takeover reading on Linux (`wtype` key → `wheel{human, human-input}`, next send exit 10). Source of the scope's CARRY (CPR / kitty graphics not measured) and of the "first live run on the Linux dev host" that `v1-53`'s live leg is read against.

@history_item n=11 marker=2026-10-09-t14-44-30-wrap
**2026-10-09-t14-44-30-wrap** — the keyboard harness cites "Windows-only live measurements" by title, not bare route number (precedent, not rule). Citations of owed route entries this chunk writes (e.g. discharging the "Linux and macOS parity" CARRY) should go by title.

@history_item n=12 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary; its own case (`tests/chaos_revive.rs`) reads that it writes no line while the child holds the terminal; zero-SGR list names a passed revive; keystroke and focus/mouse/resize clauses have no revive-specific case. `chaos_revive.rs` is one of the boundary files the Unix legs must run.

@history_item n=13 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — `passthrough_with_a_statusline_source_adds_no_viola_bytes` (`tests/tui_passthrough.rs`) asserts no viola literals for a statusline-bearing start, green on three runners; on Unix the start writes the settings override and passes `--settings`, on Windows none. A per-OS start difference the parity sweep must keep with its reason; `STATUSLINE_DEADLINE` falls on the user's command.

@history_item n=14 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — budget pause a driver-only bound; `path6_a_reading_over_the_threshold_refuses_the_send` asserts a human key reaches the child and the outer PTY holds none of `unable`, `hint:`, `budget-paused`. A `tests/cli_budget.rs` boundary reading the Unix legs must keep green.

@history_item n=15 marker=2026-10-10-session-links
**2026-10-10-session-links** — `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`) is the driver-side closed-gate reading; the keystroke clause keeps its one closed-gate reading and focus/mouse/resize has no budget case. Part of the cli_budget set the extract names green on three runners.

@history_item n=16 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — caption order pinned by the `CAPTION` literal (`list_human_board_is_the_literal_rows`, `tests/cli_list.rs`) and the unit `board_row_caption_and_default_row_are_the_literals`, not trycmd; no SGR then (spans owed to "CLI output discipline"). The stale-row witness reads a board whose caption this literal pins.

@history_item n=17 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C or TERM prints only the fixed `unable:` / `hint:` pair, exit 1, pinned by the stop table in `tests/cli_verify.rs`; the stop path adds no time bound. Signal delivery differs per OS, so this stop table is a cfg-sensitive file the sweep re-reads.

@history_item n=18 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — web a11y leaves 0.1.0 (frontend to 0.2.0); terminal surfaces stand: CLI output discipline, the TUI boundary, the wheel case, the manual terminal screen-reader passes on P4/P6; tier `Standard (1)` kept, resting on terminal surfaces alone. Why the chunk's a11y load is boundary and output-discipline only.

@history_item n=19 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — cli/tui cases run as nextest cases in the per-OS `coverage` suite on all three legs, each leg's `gate --require coverage,doctest` its own verdict; failure = `suite-failed` / `suite-skipped` / `suite-missing` / `artifact-missing`; `#[ignore]` banned; §11 bans a `suite` value or separate a11y driver outside `scripts/agent-run.* run`. Directly the §9 binding and the anti-pattern the Unix cases must respect.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — separator only with unwrapped rows; `tests/cross_list_rows.rs` in three modes (piped, `NO_COLOR`/`TERM=dumb`, outer PTY), the no-ESC assertion Unix-only, windows-2025 reading escaped names under a 200-column terminal (ConPTY re-renders); `list`/`revive` agents read is a 5 s bound on viola's own child. The file the stale-dim witness extends already carries a per-OS split with reason.

@history_item n=21 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive` takes `--json` on both arms; each strict-modes or preflight exit-1 refusal is one document with empty stderr (`revive_json_*`, `tests/cli_revive.rs`). Part of the --json documents the one-diagnostics-schema goal holds identical across OSes.

@history_item n=22 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** (the colour spans built and checked) — built bold NAME, amber only on `DIALOG`, wrapped `stale` row dim end to end (NAME dim and bold, `DIALOG` amber not dim), colour decision on `NO_COLOR`/`TERM`/`COLORTERM`/TTY, narrow-terminal cut from `viola_pty::host_size()`; P6 checked by `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` and `board_row_style_*`; dim-only-on-stale-rows unit tier only, its outer-PTY witness a CARRY on "Linux and macOS parity"; `viola wait` / `--help` no ESC or lone CR on Unix. Source of the v1-40 CARRY this chunk discharges and the SGR values it must read.

@history_item n=23 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install` built in `setup:`; four plain-ASCII lines in one write, no SGR/ESC piped or under `NO_COLOR`/`TERM=dumb`; literal-pinned lines and `plugin_install_refuses_a_widened_home`. Part of the output discipline and HELP literal the Unix legs re-read.

@history_item n=24 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — MCP `wait` clamps to `WAIT_CAP` 50 000 ms, a driver-only bound. One of the time bounds (cascade binding: every bound falls on the driver) the Unix wrapper must keep off the human.

@history_item n=25 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — confirm windows (`CONFIRM_WINDOW_FALLBACK` 10 s, `COMPACT_CONFIRM_WINDOW` 40 s) bound the driver only, taking no key. Another bound in the list the Unix wrapper must hold off the human.

@history_item n=26 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold (`dialog_hold_s` 60–3600, default 60) bounds the driver, never the human: any human key hands the held dialog back at once. The cascade quote on time bounds rests on this; the openpty wrapper must keep it.

@history_item n=27 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — second supplemental live takeover reading on Linux: the founder's own key on the product binary over a live `claude` 2.1.296 wrote `wheel{human, human-input}`, the next send exit 10 `human-typing`, `viola release` wrote `wheel{driver, release}`. Earlier live Linux evidence that `v1-53`'s "one live `viola run` on Linux" leg is read against (no live session this chunk).

@history_item n=28 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — founder's own word: `--help` over served verbs only and the board's cut width from `viola_pty::host_size()` stand, PROVISIONAL marks gone (incl. 2026-10-17-cli-output-discipline's). The width source binding n=3 rests on is founder-ratified; the openpty resize must report host size so the cut holds on Unix.

@history_item n=29 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — the `viola list` literal-pinning units (`board_row_*`, incl. the stale-dim unit peer) are cited under `src/human/board/` (was `board.rs`). Where the unit-tier peer of the stale-dim witness lives.

@history_item n=30 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-verify refusal's literal pin and its two-leg reach) — `tests/security_negatives_server_identity.rs` gated `not(target_os = "macos")`, windows-2025 and ubuntu-latest only (macOS reports no peer pid); its `HINT` literal pins the hint; the frame verbs' refusal entity names macOS as the recorded gap. The gated file the parity sweep must keep gated with its reason; the macOS carve-out the scope keeps.

@history_item n=31 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' entity, refusals, pinned result lines, `handoff:` row, eight-verb server-verify) — added `viola link` / `unlink` (plain-ASCII one-write result lines, `escape_message` for a non-RFC-3339 `since`), their refusals (strict-modes exit 21, stopped instance, `-32602` fault exit 20), `link_verbs_refuse_a_widened_instance_before_any_log` `cfg(unix)` (ubuntu/macos only), literal pins (`link_verbs_print_the_design_lines`), and the `handoff:` row in the HELP literal. These are cascade-carried strings and a `cfg(unix)` case whose reason the sweep must keep named.

@history_item n=32 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict-modes entities, text, witnesses and pinned pairs) — frame verbs exit 21 and `run`/revive start/`verify` exit 1 refuse a widened tree before any log, on all three OSes; witnesses `frame_verb_refuses_a_widened_instance_before_any_log`, `run_refuses_a_widened_home_tree_before_its_log` (`tests/run_cli.rs`), `verify_*` (`tests/cli_verify.rs`); `INSTANCE_PAIR` / `HOME_PAIR` literals pinned. `tests/run_cli.rs` is a file this chunk reaches (its `cfg(windows)` standalone kill pin); its literals and refusal must stay together on all legs.

@history_item n=33 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (an unstattable home, per OS) — a home under a regular file: strict-modes refusal with hint on Unix, `NotFound` and the catch-site fault line on Windows; `verify_under_a_regular_file_refuses_the_home` reads a per-OS oracle, and "a per-OS oracle is owed wherever a test plants a regular file above a checked path". Measured from a CI red; the precedent for any test the parity sweep ungates that plants such a path.

@history_item n=34 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (tui boundary after a contained vt100 feed panic; Windows byte reading owed) — `chaos_feed_panic_refuses_send_and_keeps_passthrough` (`tests/chaos_feed_panic.rs`): no viola literal on any OS, the child's `e4 b8 ad` bytes on Unix; the Windows byte reading owed to "Windows-only live measurements" (ConPTY re-renders a wide char on 1 column). Binding n=4: the Unix legs carry the byte reading, the Windows debt stays out of this chunk.

@history_item n=35 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness; exit-2 exception; `unknown`'s detail; peer refusal screen; fault body; census) — `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) one document / literal exit / empty stderr / no ESC / clean body; `answer`/`send` exit-2 paths a fixed stderr line, no document (`NOT_ERROR_BODIES`); `unknown` detail a code word or nothing; `is_code_word` screen, out-of-list words exit 14; `sanitised_fault` `wrapper fault`; error-writer census in `tests/contract_error_bodies.rs` and body check classes incl. `absolute-path`. The one-diagnostics-schema binding: any Unix writer joins the census, Unix paths must stay out of bodies.

@history_item n=36 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json documents every start refusal; exit 21's cause; lost/stale wrapper; `internal-error` document; name with no endpoint; hints pinned; exit-cause table) — revive start refusals one document each, `pump_child` error prints nothing; exit-21 `detail` names closed causes; `endpoint-lost` vs `instance-dead`, `stale-heartbeat` vs `already-live`; `internal-error` document with `run`/revive start/`mcp`/`hook` silent; directory rule for `instance-dead`/`unwrapped-name`; `tests/cross_exit_causes.rs` the fake-agent exit-cause table whose top comment names causes held outside it. The stale-heartbeat state the stale-dim witness boots, and the E2E table the full Unix suite must run, with any OS-held cause named at its top.

@history_item n=37 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — founder's own word supersedes Epoch 6 provisional records (feed-panic, sanitised-error, exit-cause entries) and keeps the tier `Standard (1)`. The records this chunk rests on are ratified; the tier is not re-opened here.

@history_item n=38 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (socket directory at the start and at the eight frame verbs; witnesses and pins) — revive's eleventh exit-1 code and exit 21's sixth cause `socket-dir-refused`; `tests/security_negatives_socket_dir.rs` `#![cfg(unix)]` (ubuntu/macos), `check_socket_dir_off_unix_reads_nothing`; hint pinned by `SOCKET_DIR` / `HINT` literals. The Unix-only witnesses the sweep must keep `cfg(unix)` with reason, and the start refusal the openpty `run` keeps before any bind or child.

@history_item n=39 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`server-verify-failed` reaches macOS, as an euid mismatch alone) — macOS reads no peer pid (a named carve-out no route entry owns), the refusal there an euid mismatch alone, held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported` and the `*_server_euid_is_this_user` cases; `security_negatives_server_identity.rs` stays `not(target_os = "macos")` as a limit of its pid witness, not of the word. The scope's macOS boundary exactly; no macOS test may read a pid/start-time comparison.

@history_item n=40 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tui boundary and the wrapper's drop of a refused peer) — new boundary clause: a refused peer is dropped in `accept_loop` before any frame, writes nothing to the outer terminal, moves no wheel (`serve_drops_a_refused_peer_before_any_frame`). Part of the tui boundary the openpty wrapper must hold.

@history_item n=41 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client euid refusal's limit of proof) — no foreign peer is driven; the decision is tested over injected euids and its real read on Linux and macOS; no seam may be added. Matches the scope's "no env, flag or feature seam forces a refusing peer-euid decision"; a Unix-only seam needs a Decisions Log entry.
