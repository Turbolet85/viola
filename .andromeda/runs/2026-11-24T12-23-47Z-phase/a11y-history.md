@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — added `revive` to the CLI verbs and made a passed revive enter the TUI passthrough boundary. Its exit-1 refusals take the fixed `unable:`/`hint:` form, a passed revive adds no SGR or line of its own, and the keystroke ban covers its start. This is the precedent for a restart-based switch that rides revive's start: the same boundary rows (tui-passthrough/entry, own-bytes, human-keystrokes) bind the switch, and any new way into the passthrough must be named on contract/tui-passthrough/entry.

@history_item n=2 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive` `--json` in both arms: each exit-1 refusal is one `{"v":1,"error":"<code>","detail":null}` document with empty stderr, and the human pair stays byte-identical. A new switch verb's `--json` document and refusals extend this register (contract/revive-json-refusal/*, contract/start-json-refusal/document).

@history_item n=3 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the SGR spans: a word beside every span, and the colour decision reading NO_COLOR, TERM, COLORTERM and TTY, with no SGR under `--json` or a passthrough role. It also built the `--help` group labels over served verbs only, pinned by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`. A new verb must take a row in one of board / traffic / wheel / handoff / setup and must print no SGR of its own in plain modes.

@history_item n=4 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified two things: `--help` rows over the served verbs only, and the board's cut width from `viola_pty::host_size()`. The served-verbs-only rule (contract/cli-help/served-verbs-only) is a standing founder decision that the switch verb's help row has to meet.

@history_item n=5 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — the closest precedent for a new verb's a11y footprint. It added the `link`/`unlink` verb entity: one plain-ASCII result line in one write with no SGR, or one `{"v":1,"ok":…}` document. Refusals are exit 21 (strict-modes first, then a stopped instance, then server-verify) and wrapper invalid-params as an exit-20 fault with no hint. It also built the `handoff:` help row and the result lines pinned by literal, and fixed that printed names are the verb's parsed arguments, never read back from a reply or the log (decision/printed-names-are-parsed-arguments).

@history_item n=6 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — the CLI frame verbs refuse a server that is not the recorded wrapper before any frame: exit 21 with the server-verify `hint:` last, or the `--json` document. The hint is pinned by literal, and the plugin and MCP against an impostor write no human-facing byte. If the switch reaches the live wrapper by a frame, it joins this list (contract/frame-verb-refusal/server-verify-failed, now eight verbs) and the literal-pin register.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — put the strict-modes check first. Frame verbs get exit 21 with the instance-state pair and run / revive start / verify get exit 1 with the home-state pair, before any log, with no line written; the plugin and MCP stay not-assertable. A switch verb (frame or start path) inherits decision/strict-modes-refusal-before-any-write and must be added to its exit lists.

@history_item n=8 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — set the `--json` failure discipline: one document, a literal exit, empty stderr, no ESC and no path or upstream text, witnessed by `cli_error_documents_hold_no_path_or_upstream_text`. It named the exit-2 exception (answer/send fixed line, no document), screened peer replies to code words (unknown exit 14, no hint), and added the error-writer census (`error_bodies_each_name_a_writer_and_a_witness_that_stand`, `thiserror_messages_interpolate_nothing`). A new verb's error bodies and any model/effort usage refusal must enter this census and the exit-2 exception if they follow its form.

@history_item n=9 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — `revive --json` documents every refusal of the start it shares with `run`, and a `pump_child` error prints no document once the child has held the terminal. Exit 21's `--json` `detail` names its closed cause, and stale-heartbeat / already-live mark a start against a holder. It also built the exit-cause table `tests/cross_exit_causes.rs`, `no_two_codes_share_a_hint` and `no_hint_names_release`. A new verb's codes and hints must take rows in this table and one hint per code, and none may name release.

@history_item n=10 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — a refused socket directory is exit 1 at a start (revive's `socket-dir-refused` code, the fixed-message pair) and exit 21's sixth cause at the frame verbs, before any frame / snapshot / bind, with the directory left as found and the hint pinned by literal. A refused peer is dropped with no terminal byte and no wheel move. The switch's path (start or frame) inherits the matching socket-dir rows.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — narrowed to Linux only: the TUI boundary on openpty alone, the a11y cases on the one `ubuntu-latest` leg, and revive's `--json` codes ten (`batch-script-child` retired with its emitter). It also retired the macOS/Windows carve-outs for server-verify, socket-dir and strict-modes, and made the manual screen-reader pass Orca alone. The switch's cases run on ubuntu-latest only, and no OS arm is owed.

@history_item n=12 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — CLI output discipline is checked through `std::process::Command` with every fixed string pinned by literal (trycmd/assert_cmd retired), and the outer-PTY driver is portable-pty `=0.9.0`, inherited and never re-pinned. The switch's result lines, hints and refusal layout must be pinned by literal on this harness (decision/fixed-cli-strings-pinned, decision/refusal-layout-literal-pins).

@history_item n=13 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli/tui a11y cases run as nextest cases inside the `coverage` suite under `gate --require coverage,doctest`. It bans a separate a11y driver or `suite` value outside `scripts/agent-run.sh run`, so the switch's a11y cases land in that suite with no harness of their own.

@history_item n=14 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder kept the a11y tier at `Standard (1)` (decision/a11y-tier-standard): output-discipline and boundary assertions with no WCAG conformance claim. The switch is planned at this tier.

@history_item n=15 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — fixed the closed non-editing list (focus, mouse, terminal replies), with every other byte taking the wheel, and set the wheel probe. Bears on the scope's open premise of whose wheel a switch needs: a switch typed into the live session must not be mistaken for human editing, and the human always wins.

@history_item n=16 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the readiness gate's bound falls on the driver, and a human key during the wait takes the wheel so the send is refused `human-typing` (exit 10) with nothing typed. This is the precedent for a switch that types into a live session: the human key wins, and the bound joins the driver-only time bounds.

@history_item n=17 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — a `/compact` send (a slash command typed into the session) has its own confirm window (40 s), a bound on the driver only, and fails with `not-delivered` / `no-prompt-submitted`. This is the precedent if the switch's channel is a command typed into the live session: its confirm window must be listed as a driver-only bound.

@history_item n=18 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — made the dialog hold configurable (`dialog_hold_s` 60–3600). The hold bounds the driver, and any human key hands a held dialog back at once (decision/dialog-hold-bounds-the-driver). Bears on the scope's premise of what a switch does to a session holding a dialog.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — retired exit 11 (`budget-paused`), never to be reused, with an older wrapper's word reading `unknown` (14). It also dropped the budget pause from the time-bound lists. A trap for the switch's exit codes: 11 must not be reused.

@history_item n=20 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the start's trust step reports its word in one `trust-record` diagnostics line, never on the terminal or as a refusal, and the start's Claude config lock wait joins the time bounds, falling on viola's own start. A restart-based switch rides this start, and any settings-adjacent write must stay off the terminal (decision/run-diagnostics-off-terminal).

@history_item n=21 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — a keyed start's automatic-verify window is the one exception to the never-blocked keystroke and zero-viola-bytes promises, holding from the child's start. Ratified at 2026-11-22T17-29-41Z-wrap. A restarted session carries the same exception and no new one.

@history_item n=22 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — human keystrokes wait at most behind the paste's in-flight chunk and closing marker (contract/tui-passthrough/human-keystrokes, decision/human-keystrokes-never-blocked). This binds any switch that writes into the live session.

@history_item n=23 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — reworded own-bytes, zero-viola-bytes, wheel-ignores-terminal-events and portable-pty-outer-pty-driver for the automatic-verify exception. It added paste write to both time-bound lists and the row decision/compiled-shapes-over-signatures. These are the standing forms of rows the switch's boundary rests on.

@history_item n=24 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — the two time-bound lists are full at their bound, so a new time bound lands as a row of its own (decision/start-verify-stop-grace-off-the-human). Any bound the switch adds (a restart wait, a confirm window) follows this ruling.

@history_item n=25 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — the closest recent precedents for a new verb. It added contract/viola-up-output/instance-line for a new verb's output line and put `up` on exit 21's list. It reworded the two time-bound lists to «Eight» (state provisional) and gave three new bounds rows of their own (open live wait, window wait, hyprctl read). A new switch verb follows these: an output-line row, its own exit-cause wording, and any new bound in a row of its own.

@history_item n=26 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — a non-frame verb joined exit 21 (contract/cli-exit-codes/21) and the strict-modes exit-21 list, refused before any compositor read (provisional, delegate). Precedent for naming the switch verb in those rows if it refuses there.

@history_item n=27 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — adopt's start was added to every start row: home-strict-modes, socket-dir, strict-modes-before-any-write, json-fault internal-error silence, fixed-message form, the unable-line exceptions, start-json-refusal/document, tui-passthrough/entry, log-identity/process, and agents-read fail-open. A switch that restarts through the shared start must be named the same way in each row.

@history_item n=28 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `viola open`'s refused word after `--` became an exit-2 fixed stderr line with no document that echoes none of the refused words (contract/cli-exit-codes/2, decision/json-one-document-no-hint, contract/cli-refusal/json). This is the precedent for an unusable model/effort word: either it joins the exit-2 exception or it takes a hint-free document.

@history_item n=29 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's single `claude agents --json` read (5 s kill) fails open, a bound on viola's own child, never the human. A restart-based switch through revive's preflight inherits this read and its fail-open rule.

@history_item n=30 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — added the MCP call deadline (55 s) as a driver-only bound and `call-deadline` as an MCP-only unreachable cause, with the CLI's exit-21 causes staying six. If a fork brings in an MCP tool for the switch, it inherits this deadline. On the CLI-only arm, no exit-21 cause is added.

@history_item n=31 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — the rendered TUI screen is parsed only for compiled signatures, and later wraps narrowed this to compiled shapes, with the prompt row read only for its opening char. If the switch's channel is a command typed into the live session, confirming it may not read screen text beyond these signatures (decision/no-screen-content-parsing).

@history_item n=32 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — a second live reading of the takeover clause, supplemental and never gating (decision/live-readings-never-gate). This phase runs nothing live, so the switch's wheel claims rest on outer-PTY cases alone.

@history_item n=33 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the after-feed-panic boundary: `send` refused `input-not-ready` while a human key still reaches the child, and no viola literal on the outer terminal. A switch typed through the send path meets the same readiness refusal.

@history_item n=34 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — the terminal structural equivalents gained the `-- LINKS --` separator and the NAME tree indent. If the switch surfaces the new pair on the board, it must fit these equivalents and the static, linear output.

@history_item n=35 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — the CLI output-discipline invariants (P4, P6) report inside the CI `coverage` suite (decision/cli-invariants-pass-in-ci). The switch's output-discipline cases are gated there.
