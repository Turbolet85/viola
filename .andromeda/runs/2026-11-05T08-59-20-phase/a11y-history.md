@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** (Platform: ci.yml is the one push/PR workflow) — §9 Platform now reads "one push/PR workflow `ci.yml`" and says the scheduled `nightly.yml` carries no a11y step. When this chunk removes `nightly.yml`'s Windows and macOS jobs, no a11y case is lost, because the a11y leg lives in `ci.yml` alone. This is the sentence that names the `[windows-2025, macos-latest, ubuntu-latest]` matrix, which the wrap amends.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** (CLI output-discipline evidence under the CI `coverage` suite) — established that the CLI output-discipline cases run "on all three OS legs" and report in CI inside the per-OS `test` job's single `coverage` suite, gated by `gate --require coverage,doctest`. §1, §9 and §10 carry this wording. Removing the matrix collapses it to the ubuntu-latest leg, which must still run every case. The three-leg wording becomes an expected amendment.

@history_item n=3 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** (Windows zero-viola-bytes oracle is literal absence) — split the zero-viola-bytes clause into two oracles: byte for byte against an unwrapped run on Linux and macOS, and viola's literals absent on windows-2025, because ConPTY emits its own preamble. §6 now reads "zero SGR of viola's own". This chunk removes the Windows arm only. The byte-for-byte Unix oracle stays as the sole assertion, and §3 and §6 are where the ConPTY text gets amended. §1 was kept verbatim, which is a precedent.

@history_item n=4 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — §10 says the epoch-boundary mutation audit (`run --mutants`) covers the P4 and P6 invariants that live in crate tests. When `windows-mutants.yml` leaves, that audit must still cover them on Linux. The §10 invariant's "all three OS legs" is an expected amendment.

@history_item n=5 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — the Windows zero-viola-literals check names both ConPTY hosts: the sideloaded `OpenConsole.exe` preamble, and the inbox fallback that `conpty_sideload`'s tampered case drives. `tests/conpty_sideload.rs`, `crates/viola-pty/src/sideload.rs` and `vendor/conpty/` all leave in this chunk. The text in §3 Keyboard test harness and §6 CLI equivalent that credits these hosts' bytes now names code that no longer exists, so it is an expected amendment.

@history_item n=6 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — defined case (3) as step-wise, with the wheel probed by `answer` (exit 13 means driver, 10 means human). Linux and macOS got the full assertion. `windows-2025` got the F-W3 platform fact: focus reports are swallowed, and the injected mouse report takes the wheel. It also added the Keyboard ban's ConPTY key-down clause. This chunk removes only the F-W3 arm and that ban clause. The step-wise Linux probe and the closed non-editing list stand unchanged.

@history_item n=7 marker=2026-10-07-t05-47-07-wrap
**2026-10-07-t05-47-07-wrap** — re-pointed the real Windows terminal's mouse-report measurement to the route entry "Windows-only live measurements" (blocked on an interactive Windows host). That entry is now retired, and its master hits are re-pointed under the scope's CARRY. This §3 keyboard-harness sentence is one of those hits.

@history_item n=8 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — added the outer-PTY case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` under boundary clause (2). It is sequenced on records, never on a timer. The extract's acceptance requires it to run and pass on ubuntu-latest inside `coverage`, never skipped or ignored, after any OS arm around it leaves.

@history_item n=9 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — §1's NVDA line calls Windows "the first target" (it used to say "live-supported target"). It also recorded a supplemental live Linux reading of the takeover clause. Under the Linux-only OS target, the NVDA wording in §1 and §7 is an expected amendment. Orca with GNOME Terminal stays the Linux target.

@history_item n=10 marker=2026-10-09-t14-44-30-wrap
**2026-10-09-t14-44-30-wrap** — cites "Windows-only live measurements" by route title instead of a bare route number. This is a precedent for how master text cites route entries. The title citation is exactly what the wrap's `rec.py find --text 'Windows-only live measurements'` sweep finds and re-points.

@history_item n=11 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary. `tests/chaos_revive.rs` reads that it writes no line of its own. The extract names `chaos_revive.rs` as a reading whose Windows side leaves. Its Linux assertion stays as it is.

@history_item n=12 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — added `passthrough_with_a_statusline_source_adds_no_viola_bytes` (`tests/tui_passthrough.rs`), green on three runners. On Unix the start writes the settings override and passes `--settings`. On Windows it writes none. The Windows clause leaves with its code, and the Unix reading stays unchanged.

@history_item n=13 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — added `path6_a_reading_over_the_threshold_refuses_the_send` (`tests/cli_budget.rs`) as the closed gate's keyboard reading: a human key reaches the child, and the outer PTY carries no `unable`, `hint:` or `budget-paused`. The extract names its Windows side as leaving. The Linux assertion stays.

@history_item n=14 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — pins the caption order by the `CAPTION` literal in `tests/cli_list.rs`, and `list`'s fixed strings and exit-21 pairs by literal. Any copy change the removal makes to a pinned string must move its literal in the same commit.

@history_item n=15 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the web surfaces left 0.1.0, while the terminal surfaces and the manual terminal screen-reader passes on P4 and P6 stood. The tier `Standard (1)` is the founder's to re-decide. The manual NVDA, VoiceOver and Orca passes in §7 are the rows this chunk lists as expected amendments, with Orca staying.

@history_item n=16 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — this is the source of the CI rules this chunk must keep. The cli and tui cases are nextest cases in the per-OS `test` job's `coverage` suite "on all three legs". Each leg's `gate --require coverage,doctest` is its own verdict, and the failure conditions include `suite-skipped` and `suite-missing`. The zero-flakiness rule bans `#[ignore]`. §11 bans a `suite` value or a second a11y driver outside `scripts/agent-run.* run`. Trimming the matrix keeps ubuntu-latest as the only verdict. No a11y case may become a skip. If `agent-run.ps1` leaves, `.sh` must remain the only runner.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — P6: `tests/cross_list_rows.rs` runs in three modes (piped, `NO_COLOR` / `TERM=dumb`, outer PTY). The no-ESC assertion is Unix-only, and on windows-2025 each escaped name is read as text under a 200-column terminal. This chunk removes the windows-2025 arm, and the Unix-only no-ESC assertion falls under the scope's "made unconditional" rule.

@history_item n=18 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour decision, which reads `NO_COLOR` / `TERM` / `COLORTERM` and whether stdout is a terminal. On Windows it enables VT processing and restores it afterwards. It added the outer-PTY check `cross_list_rows_style_each_span_beside_its_word_on_a_terminal`, the Unix-only rule that `viola wait` and `viola --help` carry no ESC byte and no lone CR (`tests/support/sgr.rs` `redraws`), and the `HELP` literal pin. The Windows VT-enable arm leaves. The Linux rungs, the Unix-only `redraws` assertion (now unconditional) and the literal pins must stay.

@history_item n=19 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's own word made the board's cut width come from `viola_pty::host_size()`. If `host_size` loses a Windows arm, the width source he ratified must keep its Linux behaviour.

@history_item n=20 marker=2026-10-25-u55-door
**2026-10-25-u55-door** — a11y-plan was split into its body and 4 category files (`a11y-scope-summary`, `aria-patterns-roles`, `visual-design-verification`, `cognitive-accessibility`), mapped by `.andromeda/a11y-plan.map.tsv`. Every expected amendment this chunk lists lands either in the body or in one of these category files.

@history_item n=21 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the six frame verbs' server-verify refusal; the plugin against an impostor wrapper) — introduced the exit-21 server-verify refusal and the hook/MCP impostor entity. It also recorded the macOS gap, where no peer pid is read, and verified the impostor rows "on Linux and Windows". The macOS carve-out sentence in the CLI and plugin entities now names code that leaves. That makes it an expected amendment, while the Linux exit 21, hint and document stay unchanged.

@history_item n=22 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the literal pin and its two-leg reach; the hint among literal-pinned strings) — `tests/security_negatives_server_identity.rs` is gated `not(target_os = "macos")` and runs on windows-2025 and ubuntu-latest only. The `HINT` literal pins the server-verify hint. Once the macOS and Windows legs leave, the gate either has no sibling or must be re-pointed, and the census may count it. The ubuntu-latest witness and the `HINT` pin must stay.

@history_item n=23 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the eight frame verbs' server-verify refusal; the link verbs' refusals) — extended the server-verify refusal to eight verbs and left its macOS gating unchanged. `link_verbs_refuse_a_widened_instance_before_any_log` is `cfg(unix)` and runs on ubuntu-latest and macos-latest only. Its gate falls under the made-unconditional rule, and it must keep running in full on ubuntu-latest.

@history_item n=24 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict-modes refusals as scope entities, their text, witnesses and pinned pairs) — the six frame verbs, `run` / the revive start and `verify` refuse a home or instance that fails strict modes. The text says "the mode or ACL left as found, on all three OSes" and "these run on all three OS legs". The `INSTANCE_PAIR` / `HOME_PAIR` literals pin the pairs. The Windows ACL arm leaves, and the "all three OSes" wording becomes an expected amendment. The Linux pairs, exit codes and pins must not move.

@history_item n=25 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (an unstattable home, per OS) — a home under a regular file is a strict-modes refusal with its hint on Unix. On Windows it stats `NotFound`, and the `cli` catch-site fault line stands. `verify_under_a_regular_file_refuses_the_home` uses a per-OS oracle, and the entry asks for a per-OS oracle wherever a test plants a regular file above a checked path. The Windows clause and its oracle arm leave. The Unix oracle becomes the only one.

@history_item n=26 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — `tests/chaos_feed_panic.rs` asserts that no viola literal reaches the outer terminal "on all three OSes", and that on Unix the child's own bytes (`e4 b8 ad`) do. The Windows arm leaves. The Linux literal-absence and byte assertions must stay, and the three-OS wording is an expected amendment.

@history_item n=27 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the Windows byte reading after a feed panic, owed) — added the **Owed:** line in a11y-scope-summary and the debt at the end of the body's tui Entity line. Both owe the Windows byte reading to "Windows-only live measurements". That route entry is retired. These two sites are among the master hits to re-point or retire as expected amendments.

@history_item n=28 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the census of error writers and the body check) — `tests/contract_error_bodies.rs` holds each error body to a writer and a witness that stand, and each product error writer to a name. When Windows-only files and arms leave, every writer and witness the census names must still exist, or the census has to be updated in the same change.

@history_item n=29 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — `batch-script-child`, `pinned-hash-mismatch` and `squatted-name` each print one exit-1 document. `batch-script-child` is the `.cmd` / `.bat` refusal the scope names as Windows-flavoured text. Any rewrite P4 decides must keep the code word and exit 1 on Linux, and must move its pin in the same commit.

@history_item n=30 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's `--json` document names its cause; MCP's `isError` body names its cause) — this is the closed exit-21 cause set and `detail` `server-verify-failed` / `strict-modes-failed` on both CLI and MCP. It kept "the macOS server check stays the recorded gap". The scope keeps every Linux closed word. Only the macOS carve-out wording is amendable.

@history_item n=31 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — under `--json`, the unstattable home on Windows prints the `internal-error` document in place of the catch-site line. This Windows clause leaves with the `NotFound` arm. The general catch-site rule (`run`, revive's start, `mcp` and `hook` silent) stays.

@history_item n=32 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `unwrapped-name` and `endpoint-lost` hints and their pins; the exit-cause table) — `tests/cross_exit_causes.rs` is the exit-cause table, with its top comment naming the causes held outside it. It and `send_hint_is_the_design_string` pin the hints. This table witnesses the scope's claim that no Linux exit code or closed word moves, so its rows must hold after the removal.

@history_item n=33 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start; at the eight frame verbs) — the socket-directory check covers the start (exit 1, `socket-dir-refused`, revive's eleventh code, pinned by `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`) and the eight frame verbs (exit 21's sixth cause). It is stated "on Unix", with "off Unix the check reads nothing". The off-Unix arm leaves, and the Unix gate may be made unconditional. Every refusal must stay byte for byte as it is on Linux.

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`server-verify-failed` reaches macOS, as an euid mismatch alone) — macOS reads no peer pid, a named carve-out that no route entry owns, so there the refusal means an euid mismatch alone. `tests/security_negatives_server_identity.rs` stays gated `not(target_os = "macos")`. Both the carve-out text and this gate name code this chunk removes. P4 decides them under the masters' change, and the eight verbs' `error` word and exit codes stay.

@history_item n=35 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (hook and MCP on a refused socket directory; the client euid refusal's limit of proof) — the hook fails open (exit 0, empty streams, one `hook-decision` WARN) and the MCP `isError` carries `socket-dir-refused`, each stated "on Unix". The euid decision's real read is tested "on Linux and macOS". Once the Unix gates go, the hook contract and the MCP record must stay unchanged on Linux. The "Linux and macOS" wording is amendable.

@history_item n=36 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal's witnesses and literal pins) — `tests/security_negatives_socket_dir.rs` is `#![cfg(unix)]` on ubuntu-latest and macos-latest. `check_socket_dir_off_unix_reads_nothing` is the off-Unix case. The hint is pinned by `SOCKET_DIR` / `HINT` literals. The off-Unix case and the file gate are arms the removal reaches, and the ubuntu-latest witness and its pins must stay.

@history_item n=37 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (P6's stale-row dim witness under the outer PTY, on the Unix legs) — added `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`cfg(unix)`, ubuntu-latest and macos-latest, not windows-2025) and the `path-p6-stale-dim` subject. This is the preceding chunk. The census of OS gates it built (`tests/contract_os_gated_tests.rs`) counts this gate. The case must lose or re-point its gate, never be deleted. The "Unix legs only" line is an expected amendment.

@history_item n=38 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (the stale row's SGR sequence on a terminal, and its Unix-only check) — fixed the exact sequence `[DIM, BOLD, RESET, DIM, RESET]` with `stale` shown, and the qualifier "checked … on `[windows-2025, macos-latest, ubuntu-latest]`, the stale-row dim witness on the two Unix legs only" (`visual-sgr-stale-row`). The sequence must still be asserted on ubuntu-latest. The three-leg qualifier is an expected amendment.
