@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants moved from a per-chunk mutation gate to the epoch-boundary `run --mutants` audit; why it matters: this chunk runs no mutation (directive item 2), so its guard tests (item 12's replay guard) are proven by a one-off remove-the-guard control instead.

@history_item n=2 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — an `answer` for an expired or absent dialog is refused `not-delivered` / `unknown-dialog` (exit 13); why it matters: one of `answer`'s `not-delivered` details item 1 must give a hint under decision/refusal-ends-with-hint (contract/cli-exit-codes/13).

@history_item n=3 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — focus, mouse and resize sequences and terminal replies are the closed non-editing list; the resize case runs step by step, the resize behind its size receipt; why it matters: item 7 drops the needless key in `tests/tui_pty_seam.rs` and relies on the size receipt following a resize alone (contract/tui-passthrough/focus-mouse-resize-sequences, decision/wheel-ignores-terminal-events).

@history_item n=4 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — `send` waits in the readiness gate at most 8.5 s and is then refused `not-delivered` / `input-not-ready`; a human key during the wait wins (`human-typing`); why it matters: `not-delivered` details are among the refusals item 1 must cover with a hint, without changing the bound or the wheel rule.

@history_item n=5 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the closed non-editing list was widened by exact grammar; one-field or one-prefix neighbours stay typing; why it matters: the stdin classifier row (decision/stdin-classifier-non-editing-list) stands as ratified; the PTY test edit of item 7 must not loosen it.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's exit-1 refusals take the fixed-message form, one `unable:` line and one `hint:` line last; a passed revive enters the TUI passthrough and writes no line of its own; why it matters: the fixed-message exception in contract/cli-refusal/unable-line that item 1's hint work must keep byte-identical.

@history_item n=7 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — `viola hook statusline` passes the user's output byte for byte, held by `passthrough_with_a_statusline_source_adds_no_viola_bytes`; why it matters: the zero-viola-bytes guard (decision/statusline-output-passed-byte-for-byte) that the diagnostics-open change (item 11) must not break by sending anything to the terminal.

@history_item n=8 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — the budget pause (`budget-paused`, exit 11) was added as a driver-only bound with its closed-gate outer-PTY reading; why it matters: the budget leg whose replay guard (`tests/cli_budget.rs`) leaves with the budget gate, which is why item 12 adds a replacement guard in `tests/cli_links.rs` (contract/cli-exit-codes/11).

@history_item n=9 marker=2026-10-10-session-links
**2026-10-10-session-links** — `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`) holds the four verbs under a closed gate; why it matters: more budget-leg witnesses in `cli_budget.rs` that leave with the gate; item 12 must not remove them (Boundaries: no budget-gate removal).

@history_item n=10 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list`'s fixed strings and exit-21 refusal pairs were pinned by literals, not trycmd; why it matters: precedent for pinning each new hint item 1 adds by literal (decision/fixed-cli-strings-pinned).

@history_item n=11 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify`'s interrupted pair (`unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`) pinned in `tests/cli_verify.rs`; why it matters: a fixed-message refusal pair (contract/interrupted-verify/stderr) item 1's hint changes must leave byte-identical.

@history_item n=12 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli output-discipline and TUI boundary cases run as nextest cases in the `coverage` suite under `gate --require coverage,doctest`, with no separate a11y driver and `#[ignore]` banned; why it matters: where item 1's hint pins and item 7/8/9's PTY and fd tests land and are judged.

@history_item n=13 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive takes `--json`: each exit-1 refusal is one hint-free document, the human pair byte-identical; why it matters: any hint item 1 adds is human-mode only, `--json` stays hint-free (contract/revive-json-refusal/document, decision/json-one-document-no-hint).

@history_item n=14 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the SGR spans were built (bold NAME, amber DIALOG, dim stale), each beside its word, checked under the outer PTY; why it matters: decision/sgr-beside-its-word and decision/unable-fail-never-coloured hold for any new hint line (plain, uncoloured).

@history_item n=15 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install`'s exit-21 home pair and `--json` refusal pinned by `plugin_install_refuses_a_widened_home`; why it matters: a strict-modes refusal (contract/plugin-install/exit-21) whose behaviour item 6's symlink refusal extends.

@history_item n=16 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold is configurable (60–3600 s); an `answer` after the hold reads `not-delivered` / `unknown-dialog`; why it matters: another route into `answer`'s `not-delivered` details that item 1 enumerates.

@history_item n=17 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's live reading: a human key, then `send` exits 10 `human-typing`, and `release` returns the wheel; why it matters: decision/critical-path-human-takes-wheel and decision/send-refused-while-human-typing, whose refusal text and hint item 1 touches through `send`'s `hint_cause`.

@history_item n=18 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the six CLI frame verbs' server-verify refusal) — `send_hint` gained the cause `server-verify` (exit 21, hint last); why it matters: the `hint_cause` / `send_hint` mapping item 1 extends to `unverified-cli` and `unknown` (contract/frame-verb-refusal/server-verify-failed).

@history_item n=19 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the plugin against an impostor wrapper writes no human-facing byte) — the hook and MCP refusals are not-assertable; why it matters: decision/no-assertions-for-plugin-and-ipc. Item 10's `Redact` on hook and MCP payload types stays outside a11y assertion.

@history_item n=20 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-verify refusal's literal pin and its two-leg reach) — the `HINT` literal lives in `tests/security_negatives_server_identity.rs`; why it matters: the file holding one of the three `digit_runs` copies item 13 merges into `tests/support/`, and the hint pin must survive that edit.

@history_item n=21 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the server-verify hint among the literal-pinned strings) — every new fixed hint joins the literal-pinned list; why it matters: the precedent for each hint item 1 adds (decision/fixed-cli-strings-pinned-by-literal).

@history_item n=22 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the eight CLI frame verbs' server-verify refusal) — `link` and `unlink` joined the server-verify refusal and the identity table grew to eight verbs; why it matters: the scope of the frame-verb refusals and the server_identity test file item 13 edits.

@history_item n=23 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' entity and the link line's escaped since) — the `link` / `unlink` entity: exit 21 / exit 20 refusals, with `tests/cli_links.rs` as its test; why it matters: item 12's replay guard `e5_the_replay_reads_the_link_the_wrapper_wrote` lives in `tests/cli_links.rs` (contract/link-unlink/exit-20, exit-21).

@history_item n=24 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' refusals in Error recovery) — check-first order, a stopped-instance refusal and an exit-20 wrapper fault with no hint, witnessed in `tests/cli_links.rs`; why it matters: faults keep no hint while item 1 adds hints (decision/faults-carry-no-hint, contract/cli-refusal/fault).

@history_item n=25 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' result lines among the literal-pinned strings) — the names each line prints are the verb's own parsed arguments; why it matters: decision/printed-names-are-parsed-arguments, which item 5's move of `run` onto the shared `parse_name` must keep.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities) — the strict-modes check comes first in the six frame verbs, `run`, revive's start and `verify`: exit 21 / exit 1, `hint:` last, no line; why it matters: these are the refusals item 6's symlink-refusing `strict.rs` will now raise for a symlink (contract/frame-verb-refusal/strict-modes-failed, contract/start-refusal/home-strict-modes, decision/strict-modes-refusal-before-any-write).

@history_item n=27 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the plugin against a strict-modes-refused instance, not assertable) — a refused hook call fails open at once with empty streams, and MCP returns `isError`; why it matters: contract/hook-call/strict-modes-failed and contract/mcp-frame-tool/strict-modes-failed, which a symlink now also triggers under item 6.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (an unstattable home, per OS) — a home under a regular file is a strict-modes refusal with its hint; why it matters: decision/unstattable-home-is-strict-modes-refusal. This is the precedent for how a non-directory node in the checked path reads, which item 6's `symlink_metadata` change must match.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the strict-modes refusals' text) — the fixed instance-state and home-state `unable` / `hint:` pairs printed through `human::refuse`; why it matters: item 6's symlink refusal reuses these pairs and must not add a new reason or code (Boundaries).

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the strict-modes refusals' witnesses and pinned pairs) — the `INSTANCE_PAIR` / `HOME_PAIR` literals live in `tests/security_negatives_strict_modes.rs` and `tests/run_cli.rs`; why it matters: `security_negatives_strict_modes.rs` holds the third `digit_runs` copy (item 13), and it is the witness file a symlink case for item 6 would extend.

@history_item n=31 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — after a feed panic `send` is refused `not-delivered` / `input-not-ready` and no viola literal reaches the outer terminal; why it matters: contract/tui-passthrough/after-feed-panic. Another `not-delivered` refusal, and the no-viola-bytes rule item 11's diagnostics change must keep.

@history_item n=32 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness) — `cli_error_documents_hold_no_path_or_upstream_text` holds each `--json` failure to one clean document; why it matters: contract/cli-json-failure/document and decision/json-failure-documents-are-clean stay true when item 1 adds hints (human mode only).

@history_item n=33 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`--json`'s one document and its exit-2 exception) — the `answer` / `send` exit-2 paths write a fixed stderr line and no document; why it matters: contract/cli-json-failure/send-answer-exit-2 and contract/cli-refusal/json. These exit-2 lines are not refusals for item 1's hint sweep.

@history_item n=34 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail, a code word or nothing) — `unknown` (exit 14) still prints no hint, its detail opaque; why it matters: a direct constraint on item 1. `hint_cause` mapping `Unknown => None` is the ratified carve-out (decision/unknown-refusal-has-no-hint, contract/cli-refusal/unknown-refusal), so only `unverified-cli` and `answer`'s `not-delivered` details gain hints.

@history_item n=35 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's refusal screened to a code word) — a non-code-word refusal reads `unknown`, and an unlisted code word stays visible at exit 14; why it matters: decision/peer-replies-screened-to-code-words and decision/unlisted-code-words-stay-visible bound which reasons item 1 can give a hint.

@history_item n=36 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a fault's body and the `wrapper fault` literal) — the human fault line `error: wrapper fault  <code>` carries no hint; why it matters: contract/human-fault-line/wrapper-fault and decision/faults-carry-no-hint. Item 1 must not add hints to faults.

@history_item n=37 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the census of error writers and the body check) — `tests/contract_error_bodies.rs` holds each error body to a writer and a witness; why it matters: decision/error-bodies-have-writer-and-witness. Any new hint or refusal writer from item 1 must stand in that census.

@history_item n=38 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — every refusal of the start shared with `run` gives one exit-1 document; why it matters: contract/viola-revive-json/start-refusal and contract/start-json-refusal/document. Item 5's `run` `parse_name` change sits on that start.

@history_item n=39 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's `--json` document names its cause) — exit 21's `detail` names the cause (`strict-modes-failed`, …); why it matters: contract/instance-unreachable-json/document. Item 6's symlink refusal reads `strict-modes-failed` with no new word.

@history_item n=40 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (MCP's `isError` body names its cause) — the MCP frame tools carry `detail` `strict-modes-failed` / `server-verify-failed`; why it matters: contract/mcp-frame-tool/strict-modes-failed, reached by item 6 and adjacent to item 10's MCP payload types.

@history_item n=41 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a lost wrapper and a stale holder each read their own code) — `endpoint-lost` and `instance-dead` are separate exit-21 causes, and `stale-heartbeat` exits 1; why it matters: part of the closed code set whose hints item 1 audits.

@history_item n=42 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document) — under `--json` the catch site prints the `internal-error` document, and `run`, revive's start, `mcp` and `hook` stay silent; why it matters: contract/json-fault/internal-error. A fault with no hint, outside item 1.

@history_item n=43 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a name with no endpoint reads the instance directory) — a directory reads `instance-dead`, and absent, a regular file or a symlink reads `unwrapped-name`; why it matters: contract/instance-unreachable-detail/unwrapped-name already reads a symlink at the name level. Item 6's symlink refusal must keep the strict-modes check first and not collide with this reading.

@history_item n=44 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `unwrapped-name` and `endpoint-lost` hints and their pins) — new hints were added to `src/human/hints.rs` and pinned by `send_hint_is_the_design_string` and `tests/cross_exit_causes.rs`; why it matters: the template for item 1's new hints, their pins and parsed-name rule.

@history_item n=45 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table and one hint per code) — `no_two_codes_share_a_hint` (with its planted-duplicate control) and `no_hint_names_release` hold the hint catalogue; why it matters: a trap for item 1. Each new hint for `unverified-cli` and the `answer` `not-delivered` details must be distinct and must not name `release` (decision/one-hint-per-code, decision/no-hint-names-release).

@history_item n=46 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the Epoch 6 provisional records (sanitised-error-surfaces, exit-cause-code-catalogue) are the founder's own; why it matters: the hint, `unknown` and code-catalogue decisions item 1 rests on are ratified, not provisional.

@history_item n=47 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start: exit 1, revive's eleventh code) — `run` / revive start refuse a failing socket directory with the fixed-message pair; why it matters: contract/start-refusal/socket-dir and contract/revive-json-refusal/socket-dir-refused sit on `run`'s start, which item 5 touches. A symlinked socket directory is already refused there, the precedent for item 6.

@history_item n=48 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`server-verify-failed` reaches macOS, as an euid mismatch alone) — the server-verify verification was re-scoped (later retired with macOS); why it matters: the history of `tests/security_negatives_server_identity.rs`, which item 13 edits.

@history_item n=49 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the eight frame verbs: exit 21's sixth cause) — the frame verbs refuse a failing socket directory (a symlink among the failures) through `send_hint`'s cause `socket-dir`; why it matters: contract/frame-verb-refusal/socket-dir-refused. The symlink-refusal precedent for item 6 and part of the hint map item 1 extends.

@history_item n=50 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the tui boundary and the wrapper's drop of a refused peer) — a refused peer is dropped before any frame and writes nothing to the outer terminal; why it matters: a zero-viola-bytes boundary clause item 11's diagnostics change must keep.

@history_item n=51 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (hook and MCP on a refused socket directory: not-assertable) — hook and MCP refuse with no human-facing byte; why it matters: decision/no-assertions-for-plugin-and-ipc covers item 10's payload types.

@history_item n=52 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client euid refusal's limit of proof) — some seam wiring is read only by forced refusals in remove-the-guard runs, which no committed test repeats; why it matters: the precedent for item 12's one-off remove-the-guard control through `gate.py control`.

@history_item n=53 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal's witnesses and literal pins) — the socket-dir hint is pinned by `SOCKET_DIR` and the `HINT` literal in `tests/security_negatives_socket_dir.rs`, with `no_two_codes_share_a_hint` and `every_code_is_a_code_word`; why it matters: `security_negatives_socket_dir.rs` holds a `digit_runs` copy and its unit test (item 13). The precedent for pinning item 1's hints.

@history_item n=54 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (P6's stale-row dim witness under the outer PTY, on the Unix legs) — `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` uses the `Frozen` guard in `tests/support/`; why it matters: `tests/support/` is the shared test-helper home where item 13 places `digit_runs`.

@history_item n=55 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone, the ConPTY and F-W3 readings retired) — the TUI is hosted in openpty alone; focus, mouse and resize pass through with no Windows arm, and each boundary clause is its own nextest case on ubuntu-latest; why it matters: the current form of the resize / PTY tests item 7 edits (decision/boundary-tests-sequenced-on-records, decision/zero-viola-bytes-on-the-terminal).

@history_item n=56 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg) — the cli and tui cases run in the `coverage` suite on `[ubuntu-latest]`; why it matters: items 8/9 drop the `#[cfg(target_os = "linux")]` split and the self-dump, since every run is Linux (decision/linux-only).

@history_item n=57 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (server-verify and socket-directory refusals, the macOS and off-Unix carve-outs retired) — `security_negatives_server_identity.rs` was made ungated and `security_negatives_socket_dir.rs` is the ubuntu witness; why it matters: these two files hold `digit_runs` copies item 13 merges.

@history_item n=58 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict-modes refusals, the Windows ACL and NotFound arms retired) — `strict.rs` lost its Windows arm; the mode is left as found; a home under a regular file has one oracle; why it matters: the Unix-only `strict.rs` (`unix::reading`) item 6 changes to stop following symlinks.

@history_item n=59 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Linux-only platform line) — Platform: Linux only, CI on `ubuntu-latest`; why it matters: decision/linux-only. This is why the fd test (items 8/9) can always read `/proc/<pid>/fd`.

@history_item n=60 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli SGR and escape checks without their Windows arms, the stale-row witness ungated) — the SGR and no-ESC checks run on ubuntu-latest; why it matters: the guards behind decision/sgr-beside-its-word and decision/unable-fail-never-coloured, which new hint lines must keep.

@history_item n=61 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the three-runner greens re-worded as dated readings) — the closed-gate case in `tests/cli_budget.rs` reads green at a dated CI run; why it matters: the budget-leg readings that leave with the budget gate, which is why item 12 adds a replay guard outside `cli_budget.rs`.

@history_item n=62 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins) — the cli checks are `std::process::Command` with literal pins (trycmd / assert_cmd retired); why it matters: item 1's hints are pinned this way (decision/cli-output-discipline-assertion-tools, decision/cli-a11y-by-literal-tests).

@history_item n=63 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0) — the tui boundary driver is on the workspace's `=0.9.0` pin, which a11y never re-pins; why it matters: decision/tui-boundary-driver-pin and decision/portable-pty-outer-pty-driver for the PTY tests of item 7.

@history_item n=64 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the §8 verification lines pinned by literal) — the `unable <reason> <detail>` + final `hint:` layout, the typed exit codes and a hint-free `--json` are held by literal pins; every fixed CLI string is pinned; why it matters: decision/refusal-layout-literal-pins. Each hint item 1 adds needs its literal pin.

@history_item n=65 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli tooling in the a11y testing contracts) — the tool pick reads `std::process::Command` with literal pins in the nextest and `coverage` suites; why it matters: the harness item 1's and item 12's tests ride on.

@history_item n=66 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the frontend stubs were retired to N/A (no web or graphical frontend); why it matters: explains the plan's absences and N/A areas in the rows file; this chunk adds no web surface.

@history_item n=67 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (decision · contract · absence), each verified against the code; why it matters: the picked rows (decision/refusal-ends-with-hint among them) are the form this chunk's route-owed rule lines close against. The rows landing's `route-owed.md` is this chunk's source.

@history_item n=68 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing`: the vendor screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char; why it matters: the latest change to a picked row. The readiness gate behind `not-delivered` / `input-not-ready` is the one item 1's hint covers, and no test may parse vendor content past these signatures.
