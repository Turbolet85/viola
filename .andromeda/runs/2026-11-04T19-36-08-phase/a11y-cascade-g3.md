@carried n=1 label=a11y-plan/cognitive-accessibility fact=fct-dvgtuxyi
why: One diagnostics schema must hold every refusal word closed on every OS, so a word a Unix leg cannot map must read as `unknown`'s exit 14, never as a new OS-only word.
quote: a code word outside the closed refusal list is visible by design, in the document and in `refused: <word>`, and takes `unknown`'s exit 14.
@carried n=1 run=2 label=a11y-plan fact=fct-6jy2xx4r
quote: the one refusal with no hint is `unknown` (exit 14), whose detail is a code word or nothing, opaque to a reader
@carried n=2 label=a11y-plan/cognitive-accessibility fact=fct-lwbpjniq
why: A fault the openpty wrapper raises must reach `--json` and MCP through the same sanitised body on Linux and macOS as on Windows, so the fault document keeps the same fields and closed words on every OS.
quote: under `--json` and on MCP a fault's body is read through `viola_channel::sanitised_fault`: a code outside the five known reads the fixed `message` `wrapper fault` with null `data`. The human line is `error: wrapper fault  <code>`.
@carried n=3 label=a11y-plan/cognitive-accessibility fact=fct-aaun2rti
why: The openpty `run` wrapper and `revive`'s start must stay silent at a fault on Unix, and the `internal-error` document (revive's program-not-found among them) must read the same on every OS leg.
quote: where a CLI verb prints `error: internal error`, under `--json` it prints the `internal-error` document with a null `detail` on stdout in that line's place, stderr empty; without `--json` the line stands, and `run`, `revive`'s start, `mcp` and `hook` stay silent there.
@carried n=3 run=2 label=a11y-plan fact=fct-mkekg7f5
quote: every time bound viola holds — the `viola wait` timeout the caller passes, the MCP `wait` cap, the dialog hold, the `viola send` readiness gate, the confirmation windows, the status-line deadline, the `claude agents --json` read, the budget pause — falls on the driver, the user's own command or viola's own child, never on the human: no key is blocked, refused or delayed by it.
@carried n=4 label=a11y-plan/cognitive-accessibility fact=fct-wkabfqtn
why: The error-recovery layout re-asserted on the Unix legs must keep this one exception to one document, the same fixed stderr line and no document on every OS.
quote: the `viola answer` and `viola send` exit-2 paths write the fixed `UNUSABLE_RESPONSE` or `UNUSABLE_TEXT` line to stderr whatever `--json` says, and no document; `NOT_ERROR_BODIES` (`tests/contract_error_bodies.rs`) names them.
@carried n=4 run=2 label=a11y-plan fact=fct-6jy2xx4r
quote: `--json` gives one document with no hint, except the `viola answer` and `viola send` exit-2 paths (a fixed stderr line, no document).
@carried n=5 label=a11y-plan/cognitive-accessibility fact=fct-g3h7n6e2
why: The parity sweep re-reads every OS-gated test and keeps macOS's recorded no-peer-pid gap, so the server-identity witness must stay gated off macOS with its reason named, and the per-refusal layout must pass on each leg it names.
quote: These run on all three OS legs, except `tests/security_negatives_server_identity.rs`, gated `not(target_os = "macos")`, which runs on windows-2025 and ubuntu-latest only, a limit of its peer pid witness and not of the word: `server-verify-failed` also reaches macOS (below).
@carried n=5 run=2 label=a11y-plan/cognitive-accessibility fact=fct-g3h7n6e2
quote: no peer pid is read (the recorded gap, a named carve-out no route entry owns), so the peer pid cases compile out there; the server-verify refusal there means an euid mismatch alone, its decision and its real read held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`; the socket-directory refusal's macOS witness is `tests/security_negatives_socket_dir.rs`, on `macos-latest`.
@carried n=5 run=3 label=a11y-plan fact=fct-2jdilj7v
quote: trycmd pins the `unable <reason> <detail>` + final `hint:` layout per refusal and assert_cmd the typed exit codes and a hint-free `--json`, on all three OS legs (the frame verbs' server-verify peer pid witness on windows-2025 and ubuntu-latest only: macOS reports no peer pid, the word there an euid mismatch alone; the socket-directory refusal on ubuntu-latest and macos-latest).
@carried n=5 run=4 label=a11y-plan fact=fct-c6pzu5nt
quote: Every human-mode CLI refusal except `unknown` (exit 14, its detail a code word or nothing, design cli) ends with a `hint:` line.
@carried n=6 label=a11y-plan/cognitive-accessibility fact=fct-cetk7x7u
why: A `cfg(unix)` case the parity sweep re-reads must keep its Unix-only reason named where it stands and pass on ubuntu-latest and macos-latest.
quote: in `tests/cli_links.rs`, `link_verbs_against_a_stopped_instance_exit_21` reads the stopped-instance refusal (exit 21, the not-running hint last), and `link_verbs_refuse_a_widened_instance_before_any_log` the strict-modes refusal (a group-writable driven instance refused before any log, its mode read back as found; `cfg(unix)`, so on ubuntu-latest and macos-latest only).
@carried n=7 label=a11y-plan/cognitive-accessibility fact=fct-52gvy64u
why: `tests/run_cli.rs` is a file the chunk reaches whole, and the `run` wrapper on openpty must refuse a widened home tree before its log and child, these strict-modes cases passing on all three OS legs.
quote: `run_refuses_a_widened_home_tree_before_its_log` (`tests/run_cli.rs`) reads `run`'s exit 1 with the `HOME_PAIR` or `INSTANCE_PAIR` literal, stdout empty, no line and no child; `verify_refuses_a_widened_ledger_before_any_step` and `verify_under_a_regular_file_refuses_the_home` (`tests/cli_verify.rs`, the latter's oracle per OS: the home-state pair on Unix, the catch-site line on Windows) read `verify`'s home-state pair with no step line or summary. Unlike the server-verify cases, these run on all three OS legs.
@carried n=8 label=a11y-plan/cognitive-accessibility fact=fct-trymhbcp
why: The socket-directory refusal is a Unix-only witness the Unix legs must re-assert, its `#![cfg(unix)]` gate the reason named where it stands.
quote: it has no Windows witness: `tests/security_negatives_socket_dir.rs` is `#![cfg(unix)]`, covering `ubuntu-latest` and `macos-latest`, and off Unix the check reads nothing and passes (`check_socket_dir_off_unix_reads_nothing`).
@carried n=9 label=a11y-plan/cognitive-accessibility fact=fct-5fnuqr36
why: One diagnostics schema requires every `--json` failure document to read alike on every OS, so this witness and its body check must hold on the Unix legs, Unix paths included.
quote: `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) holds each `--json` failure it drives to one document, a literal exit, empty stderr, no ESC byte and a clean body.
@carried n=9 run=2 label=a11y-plan/cognitive-accessibility fact=fct-kia5lupj
quote: `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes.
@carried n=10 label=a11y-plan/cognitive-accessibility fact=fct-fuvobsk5
why: Any error body or product error writer the Unix openpty wrapper adds must join this census with a writer and a witness that stand, its message interpolating nothing.
quote: in `tests/contract_error_bodies.rs`, `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds each error body to a writer and a witness that stand, `product_error_writers_are_each_named` holds each product error writer to a name, and `thiserror_messages_interpolate_nothing` holds each `thiserror` message to no interpolation.
@carried n=11 label=a11y-plan/cognitive-accessibility fact=fct-dmbuxeqb
why: The full fake-agent suite must run this E2E binary on ubuntu-latest and macos-latest, its rows witnessing the same exit, document and hint per cause on every OS, and any cause held outside the table for an OS reason named in its top comment.
quote: `tests/cross_exit_causes.rs`, a `fake-agent` E2E binary, is the exit-cause table: its rows check each cause's exit, document and hint in human and `--json` modes, and the comment at its top names the causes held outside the table.
@carried n=12 label=a11y-plan/cognitive-accessibility fact=fct-oji2cu7x
why: Fixed CLI strings must stay identical on every OS, so these literal pins, the `run_cli.rs` pairs among them, must pass unchanged on the Unix legs and fail any OS-only wording.
quote: and the server-verify hint (`send_hint`'s cause `server-verify`, `src/human/hints.rs`), pinned by the `HINT` literal in `tests/security_negatives_server_identity.rs`, and the instance-state and home-state pairs as the six frame verbs and `run` print them, pinned by the `INSTANCE_PAIR` / `HOME_PAIR` literals in `tests/security_negatives_strict_modes.rs` and `tests/run_cli.rs` (`verify`'s home-state pair by literal in `tests/cli_verify.rs`). Any copy change therefore fails CI until the pinned string is updated on purpose.
@carried n=12 run=2 label=a11y-plan/cognitive-accessibility fact=fct-oji2cu7x
quote: The socket-directory hint (`send_hint`'s cause `socket-dir`, `src/human/hints.rs`) is pinned by the unit `send_hint_is_the_design_string` and by the literals `SOCKET_DIR` (`tests/cross_exit_causes.rs`) and `HINT` (`tests/security_negatives_socket_dir.rs`), with `no_hint_names_release`, `no_two_codes_share_a_hint` and `every_code_is_a_code_word` holding the catalogue.
@carried n=12 run=3 label=a11y-plan fact=fct-fhiyvzbh
quote: every fixed CLI string is pinned, by trycmd or, for `viola list`, `viola --help`, `viola plugin install`, the server-verify, socket-directory, `unwrapped-name` and `endpoint-lost` hints and the `viola link` / `viola unlink` result lines, by literal, so any copy change fails CI until the pinned string is updated on purpose.
@carried n=12 run=4 label=a11y-plan fact=fct-lsnfzz6p
quote: a kebab-case reason/detail code (`not-delivered`, `human-typing`, `unverified-cli`) never appears without the plain word `unable`.
@carried n=13 label=a11y-plan/cognitive-accessibility fact=fct-tfdrn5u5
why: The link and unlink result lines are fixed strings the task holds identical on every OS, each one write with no SGR in any mode on the Unix legs too.
quote: their three result lines (`write_linked`, `write_already_linked`, `write_unlinked`; the lines' text: design-system §Surface: cli), each one write with no SGR in any mode, are pinned by literal in the unit `link_verb_lines_are_the_design_lines_in_one_write` (`src/human/verbs.rs`) and the root case `link_verbs_print_the_design_lines` (`tests/cli_links.rs`).
@carried n=14 label=a11y-plan/cognitive-accessibility fact=fct-urfjsgc5
why: The P6 board read on the Unix legs, and the outer-PTY stale-row witness that reads `viola list` there, must keep the caption order this literal pins.
quote: The `CAPTION` literal in the root case `list_human_board_is_the_literal_rows` (`tests/cli_list.rs`) pins the caption order `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI` for `viola list` (chunk 2026-10-11-the-board-viola-list).
@carried n=14 run=2 label=a11y-plan fact=fct-xifq2q6n
quote: the `CAPTION` literal pins the caption order `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI` for `viola list`.
@carried n=14 run=3 label=a11y-plan fact=fct-vt5poetp
quote: NEVER let the caption set or order of CLI `viola list` (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI) drift from its `CAPTION` literal (SC 3.2.4).
@binding n=1
**cognitive-errors ↔ the scope's one diagnostics schema**: the closed refusal word with `unknown`'s exit 14, the sanitised fault body and the `internal-error` document must carry the same fields and closed words on Linux, macOS and Windows, a word one OS cannot fill reading its closed word, never an OS-only one (a11y-plan/cognitive-accessibility fct-dvgtuxyi) (a11y-plan/cognitive-accessibility fct-lwbpjniq) (a11y-plan/cognitive-accessibility fct-aaun2rti).
@binding n=2
**cognitive-errors verification ↔ the macOS carve-out and the cfg sweep**: every OS-gated error-recovery witness the sweep re-reads must keep its gate's reason named where it stands — the server-identity peer pid witness off macOS, the socket-directory and strict-modes link witnesses `cfg(unix)` — and no macOS test may read the server-verify refusal as a pid or start-time comparison (a11y-plan/cognitive-accessibility fct-g3h7n6e2) (a11y-plan/cognitive-accessibility fct-trymhbcp) (a11y-plan/cognitive-accessibility fct-cetk7x7u).
@binding n=3
**cognitive-plain-language ↔ cognitive-errors**: the `HOME_PAIR` / `INSTANCE_PAIR` literals in `tests/run_cli.rs` pin both the fixed copy and the strict-modes refusal of the openpty `run` wrapper, so a change to that file must keep the literal and the refusal's exit 1, empty stdout, no line and no child together on all three legs (a11y-plan/cognitive-accessibility fct-oji2cu7x) (a11y-plan/cognitive-accessibility fct-52gvy64u).
@binding n=4
**cognitive-consistency ↔ the plan's Owed outer-PTY stale-row witness**: the `viola list` board the Unix outer-PTY witness reads for dim only on stale rows must keep the `CAPTION` order its literal pins (a11y-plan/cognitive-accessibility fct-urfjsgc5) (a11y-plan fct-lchghj7u).
@binding n=5
**cognitive-errors fault silence ↔ the tui passthrough boundary**: at a fault the openpty `run` wrapper and `revive`'s start write nothing to the outer terminal, and no time bound it holds falls on the human's keys (a11y-plan/cognitive-accessibility fct-aaun2rti) (a11y-plan fct-mkekg7f5) (a11y-plan fct-rdegquaa).
