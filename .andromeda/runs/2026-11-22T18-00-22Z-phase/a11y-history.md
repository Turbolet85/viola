@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants (P4, P6) held in crate tests meet mutation only at the epoch-boundary audit (`run --mutants`), the per-chunk and CI mutation gates removed (row decision/mutation-audit-covers-crate-invariants). Why it matters: items 2 and 3 kill that audit's survivors with tests, and the chunk runs no mutation itself; the next boundary's audit is the one that reads them.

@history_item n=2 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli / tui a11y cases became nextest cases in the CI `coverage` suite gated by `gate --require coverage,doctest`, and the zero-flakiness parallel bans `#[ignore]` (row decision/zero-flakiness). Why it matters: items 5 and 6 (the boot supervisor test's 20 s bound, the `mutants` profile's 15 s kill) must be mended by root cause, never by a retry, an ignore or a raised timeout.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg; the tui boundary on openpty alone) — every a11y reach reads `ubuntu-latest`, the three tui boundary clauses each a nextest case there, the zero-viola-bytes case compared byte for byte with an unwrapped fake-agent run. Why it matters: any test this chunk adds or moves (pump, boot, nextest profile) lands in that one leg's `coverage` suite.

@history_item n=4 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (cli checks on `std::process::Command` and literal pins; outer-PTY driver on portable-pty `=0.9.0`) — refusal layouts, typed exit codes and hint-free `--json` are held by literal asserts over `std::process::Command` (row decision/refusal-layout-literal-pins), and the tui boundary driver is portable-pty `=0.9.0`, which a11y never re-pins (row decision/portable-pty-outer-pty-driver). Why it matters: a mend's new or changed refusal (item 1's `no-prompt-submitted`, item 4's agents read) is pinned by literal, never by trycmd / assert_cmd.

@history_item n=5 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's 8.5 s bound falls on the driver only; a key during the wait takes the wheel and the send is refused `human-typing`, held by `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on records never a timer (row decision/human-key-takes-the-wheel). Why it matters: item 1's mend of `SendSlot::confirm` sits on the same send path; whatever end it gives a claimed line must leave a human key's win and that guard intact.

@history_item n=6 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — a typed send waits for its confirmation within `CONFIRM_WINDOW_FALLBACK` (10 s), a `/compact` on a verified CLI within `COMPACT_CONFIRM_WINDOW` (40 s); unconfirmed it is `not-delivered` / `no-prompt-submitted`, a bound on the driver only (row contract/send-refusal/no-prompt-submitted). Why it matters: item 1 (a claimed line whose settle never comes under a failing snapshot write) must end inside these windows with that refusal or a fault, adding no bound and moving no window.

@history_item n=7 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `list` and `revive` each make one `claude agents --json` read (direct spawn, `take(MAX_FRAME + 1)`, a 5 s kill); a failed, oversize, malformed or timed-out read leaves the board and passes revive (fail open), the bound falling on viola's own child (row decision/claude-agents-read-fails-open). Why it matters: item 4 gives verify's agents read in Run B a bound of its own; it must stay a bound on viola's child, measured first, and keep the fail-open reading where it applies.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C or TERM prints no step line and no summary, only the fixed `unable:` / `hint:` pair, exit 1; the stop path adds no time bound (row contract/cli-refusal/unable-line names this pair). Why it matters: item 7 measures a `verify` ended by SIGKILL, which no handler sees; a mend there must not add a bound or a terminal line beside this pair.

@history_item n=9 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line reads `[NN/NN] … pass|absent|fail`, `absent` only on the browser-tools-prompt row (row contract/viola-verify/step-line), ratified at 2026-11-22T17-29-41Z-wrap. Why it matters: item 4's agents-read bound inside Run B must leave verify's step lines in this closed form.

@history_item n=10 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the start writes the CLI's trust record before the spawn, reports its word in one `trust-record` diagnostics line never as a refusal, exit code or hint, and its Claude config lock wait joined both time-bound rows as a bound on viola's own start (rows decision/time-bounds-never-on-human, decision/time-bounds-fall-on-the-driver). Why it matters: item 7 measures whether a SIGKILLed verify leaves a project entry in the Claude config; a mend of the trust-record path stays off the terminal and adds no human-facing bound.

@history_item n=11 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — a keyed start's automatic verify window precedes the child: human keystrokes and zero viola bytes hold from the child's start, a key in that window waiting in the terminal's buffer (rows contract/tui-passthrough/human-keystrokes, contract/tui-passthrough/viola-terminal-bytes, decision/human-keystrokes-never-blocked except), ratified at 2026-11-22T17-29-41Z-wrap. Why it matters: items 4 and 7 touch the verify path a keyed start runs; a new agents-read bound or a SIGKILL mend there lengthens or changes that window only as the exception allows.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — the budget pause left both time-bound rows (decision/time-bounds-fall-on-the-driver, decision/time-bounds-never-on-human) by `~` edits that kept every other member. Why it matters: precedent for how a bound list is edited; the scope's boundary says a new bound joins these near-full cells by name without dropping another member's words.

@history_item n=13 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the status-line deadline left both time-bound rows, each other member kept. Why it matters: the same list-editing precedent for the two rows item 4's new agents-read bound would touch.

@history_item n=14 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — a new bound (the MCP call deadline) was added by name to both time-bound rows, falling on the driver's call. Why it matters: the closest precedent for item 4 — a new time bound is a named member of these rows, and its value is the operator's fork.

@history_item n=15 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — the paste became chunks on one queued writer thread; a human key waits behind at most the in-flight chunk and closing marker (rows contract/tui-passthrough/human-keystrokes, decision/human-keystrokes-never-blocked). Why it matters: item 3's killing tests for the paste-writer survivors (`pump.rs:191`, `:337`, `:372`) read exactly this promise; a test must hold the key-ordering bound, never loosen it.

@history_item n=16 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (paste write in the time-bound rows) — `paste write` joined decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human, every word kept, the start's verify stop grace standing in its own row. Why it matters: the current text of the two rows item 4 would extend, already near full; and the verify stop grace is a bound on viola's own child that item 7's SIGKILL measurement sits beside.

@history_item n=17 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (the zero-bytes exceptions) — contract/tui-passthrough/own-bytes, decision/zero-viola-bytes-on-the-terminal, decision/wheel-ignores-terminal-events and decision/portable-pty-outer-pty-driver gained the keyed start's automatic-verify exception, and the 2026-11-20 and 2026-11-22-paste-write rows' PROVISIONAL marks were ratified. Why it matters: these are the picked tui rows as they stand at pin 3998; any verify-path or paste-writer change must keep them true.

@history_item n=18 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (compiled shapes in their own row) — `+ decision/compiled-shapes-over-signatures`: the shapes the screen is parsed for are a modal label row's option shape and the box's rows below it, no row text kept. Why it matters: item 2's killing tests on `screen.rs:126`, `:135` probe that parser; they must stay inside the compiled shapes and signatures.

@history_item n=19 marker=2026-11-20-a-quoted-modal-never-blocks-a-send
**2026-11-20-a-quoted-modal-never-blocks-a-send** — decision/no-screen-content-parsing's rule names the compiled shapes over the compiled literals. Why it matters: the screen-parsing bound item 2's `screen.rs` tests must respect.

@history_item n=20 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** — decision/no-screen-content-parsing's except reads the test-only rig's closed-word readings (box-row cell attributes before any send or after a turn end, send or KR resize), ratified at 2026-11-22T17-29-41Z-wrap. Why it matters: the current text of that exception; this chunk starts no rig and no live `claude`, so it adds no reading to it.

@history_item n=21 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's live readings on Linux stay supplemental and never gating (row decision/live-readings-never-gate). Why it matters: the chunk runs no live start, so every claim it makes rests on CI-run tests, never a live reading.

@history_item n=22 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the closed non-editing stdin list by exact grammar, neighbours one field or prefix away staying typing (rows decision/stdin-classifier-non-editing-list, decision/non-editing-sequences). Why it matters: item 3's pump tests sit beside the stdin path; nothing they assert may widen that list.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness; one document and its exit-2 exception; the census of error writers) — each `--json` failure is one clean document (`cli_error_documents_hold_no_path_or_upstream_text`), `answer`/`send` exit-2 paths excepted, and `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds every error body to a writer and a witness (rows decision/json-failure-documents-are-clean, decision/json-one-document-no-hint, decision/error-bodies-have-writer-and-witness). Why it matters: item 1's end for an unsettled claimed line, or item 4's new agents-read outcome, if it adds an error body, must be named in that census and stay clean.

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (one hint per code; the `cli` catch site's `internal-error` document) — `no_two_codes_share_a_hint` holds each code to its own hint, and where a verb prints `error: internal error` `--json` prints the `internal-error` document (rows decision/one-hint-per-code, contract/json-fault/internal-error). Why it matters: if item 1's mend ends a claimed line as a fault rather than a refusal, it takes the fault form (no hint), and any new code needs its own hint — a new closed word being the operator's fork.
