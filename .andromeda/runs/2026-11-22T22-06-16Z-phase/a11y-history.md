@history_item n=1 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — pinned the board's caption order NAME · LIVE · STATUS · WHEEL · DIALOG · CLI by the `CAPTION` literal (`list_human_board_is_the_literal_rows`, `board_row_caption_and_default_row_are_the_literals`), fixed the board as BAY line → caption row → wrapped rows, and held the row escape (ESC, C1, DEL, `\n`, `\t` → `\xHH`) by units plus no-ESC root cases. This is the precedent for fleet trees on `list`: tree drawing has to fit inside these fixed captions and slot order and the literal pins (decision/list-captions-fixed, contract/viola-list-caption/order).

@history_item n=2 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** (board units re-cited) — the board's literal-pinning units (`board_row_*`, `bay_line_*`, `instance_and_home_state_pairs_are_the_fixed_texts`) moved to `src/human/board.rs`. This is where the tree-drawing pins start out.

@history_item n=3 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — the `viola list` literal-pinning units are now cited under `src/human/board/` (a module root). The scope's tree drawing in `src/human/board/` extends these units, and any move must re-cite them in the master.

@history_item n=4 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the `-- UNWRAPPED - READ-ONLY --` separator and unwrapped rows: they print only when the one `claude agents --json` read lists a session no wrapped row joins, NAME is the escaped `sessionId`, and WHEEL, DIALOG and CLI read `n/a`. A failed read leaves the board as it was (fail open, 5 s bound on viola's own child). Trees on `list` must keep this order and separator (contract/viola-list-board/order, /unwrapped-separator, decision/claude-agents-read-fails-open), and must decide how unwrapped rows (which have no lineage) sit beside the trees.

@history_item n=5 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour spans: bold NAME, amber only on `DIALOG`, and a stale row dim end to end, each beside its word. The colour decision reads `NO_COLOR` (empty value included), `TERM`, `COLORTERM` and stdout's terminal state, with no SGR under `--json` or a pipe. On a narrow terminal an unwrapped NAME is cut with `...` but never inside an escape. It also built the `--help` group labels. The scope's "plain when piped" and tree glyphs fall directly under these rules (decision/no-sgr-off-terminal, sgr-beside-its-word, unwrapped-name-truncation, colour-decision-inputs). Tree glyphs or indentation must not break the no-SGR or no-ESC pins or the truncation rule.

@history_item n=6 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified two things that now stand as built: `--help` rows over served verbs only, and the board's cut width from `viola_pty::host_size()`. A tree indent eats into this width source for NAME truncation.

@history_item n=7 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — added the stale-row dim witness under the outer PTY (`cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`). On a terminal a stale row's stream is exactly `[DIM, BOLD, RESET, DIM, RESET]`, with `stale` printed. Tree prefixes on a stale row must keep this exact SGR sequence (contract/viola-list-stale-row/escape-sequences, decision/stale-row-dim).

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli SGR and escape checks without their Windows arms) — the stale-row witness is now ungated, `tests/cross_list_rows.rs` holds the no-ESC assertion in each mode, and the colour depth needs no VT enable. These are the `cross_list_rows` cases the tree form of `list` must keep green.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home; the tui boundary on openpty alone) — the cli and tui a11y cases run in the `coverage` suite on `ubuntu-latest` alone, under `gate --require coverage,doctest`. Each tui boundary clause is its own nextest case, and zero viola bytes is checked byte for byte against an unwrapped run. A lineage line written at launch must keep these boundary cases green.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (revive's start refusals without batch-script-child; strict-modes Windows arms retired; server-verify carve-outs retired) — the closed set of `revive --json` exit-1 codes went from eleven to ten, which is the precedent that a closed word changes only with an explicit ruling. The strict-modes refusals (home tree for `run`, revive's start and `verify`) and the server-verify refusal of all eight frame verbs, `link`/`unlink` included, now run on ubuntu-latest. Revive keeping its parent and links drawn apart touch these verbs.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the manual screen-reader pass, Orca on Linux alone) — the founder-owned manual pass on P4/P6, the `viola list` board included, is Orca + GNOME Terminal for the cli and Orca + foot for the tui, and nothing is gated on it. A tree-shaped board has to stay linear and readable for that pass (decision/static-linear-output).

@history_item n=12 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (cli checks on std::process::Command and literal pins; §8 verification lines pinned by literal) — the cli output-discipline checks are now `std::process::Command` with literal pins (trycmd/assert_cmd are gone). Every fixed CLI string is pinned by literal: the `viola list` `CAPTION`, `--help`, the hints and the `link`/`unlink` result lines. New fixed copy for trees or link edges must be pinned the same way (decision/refusal-layout-literal-pins, cli-output-discipline-assertion-tools).

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0) — the tui boundary driver is the workspace's portable-pty `=0.9.0`, which a11y inherits and never re-pins (decision/tui-boundary-driver-pin, portable-pty-outer-pty-driver).

@history_item n=14 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: there is no web or graphical frontend, and the terminal fleet console (matrix#v2-14) reuses `viola list`'s caption order, its a11y scope owned by «Fleet a11y verification». The cli critical path is `viola list`'s fixed captions and slot order. The fleet console is out of scope here, but it inherits whatever the tree form of `list` fixes (contract/viola-list-board/caption).

@history_item n=15 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (decision/contract/absence), each verified against the code. From then on every change this chunk makes to the plan is a `+`/`~`/`−` row operation, with a new decision landing as its own row where a list is full at its bound.

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility; 6. Visual Design Verification) — exit 11 was retired and never reused, and `budget-paused` left decision/state-carried-by-printed-word. This is the precedent for how a closed word or exit leaves the plan. Any new board state word for lineage (for example a parent label) must join the printed-word list, not colour alone.

@history_item n=17 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' entity and the link line's escaped since) — `viola link`/`unlink` print one plain-ASCII line in one write, with no SGR in any mode, or one `{"v":1,"ok":…}` document. A `since` that is not RFC 3339 prints through `escape_message`. "Links drawn apart" builds directly on these 0.1.0 link verbs and the way their links are shown.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' result lines among the literal-pinned strings) — the names each link line prints are the verb's own parsed arguments, never names read back from the reply or the log (decision/printed-names-are-parsed-arguments). This is the direct precedent for "a link name is shown only after it parses as a viola name": showing observed links read from the log on `list` departs from this rule and needs it amended or extended.

@history_item n=19 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' refusals in Error recovery; the `handoff:` help row built; the eight CLI frame verbs' server-verify refusal) — `link`/`unlink` never refuse a pair client-side: a pair the wrapper refuses as invalid params is a fault, exit 20, no hint (decision/link-pair-validated-by-wrapper). `--help` gained `handoff:  link, unlink` (contract/cli-help/group-labels). Parent edges must not change these verbs or what they accept.

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a name with no endpoint reads the instance directory; the `unwrapped-name` and `endpoint-lost` hints) — a name with no endpoint and no instance directory reads `unwrapped-name`. Hints print the parsed `ViolaName`. This bears on the name-check and the "reused name never inherits an old parent" rule, which is decided per instance directory.

@history_item n=21 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — contract/instance-unreachable-detail/unwrapped-name dropped its symlink clause, because a symlinked instance directory now reads `strict-modes-failed` first. That is a constraint for any lineage read that resolves a parent's instance directory.

@history_item n=22 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start; a start against a holder) — every refusal of the start that `revive` shares with `run` prints one exit-1 `{"v":1,"error":"<code>","detail":null}` document. `stale-heartbeat` and `already-live` are now told apart. "Parent kept by revive" goes through this shared start (contract/viola-revive-json/start-refusal, start-json-refusal/document, start-holder-refusal/*).

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's `--json` document names its cause; the `cli` catch site's `internal-error` document; the exit-cause table and one hint per code) — exit 21's `detail` is one of a closed set of causes, and the catch site prints the `internal-error` document under `--json`. `tests/cross_exit_causes.rs` holds `no_two_codes_share_a_hint` and `no_hint_names_release`. Any new refusal or code for lineage or links joins this table and the closed sets as a fork (decision/one-hint-per-code, no-hint-names-release, contract/json-fault/internal-error).

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness; `--json`'s one document and its exit-2 exception) — each `--json` failure is one document with a literal exit, empty stderr, no ESC byte and a clean body (`cli_error_documents_hold_no_path_or_upstream_text`). The `answer`/`send` exit-2 paths are the one exception. `list --json`'s new lineage payload and any new failure must keep this (decision/json-failure-documents-are-clean, json-one-document-no-hint).

@history_item n=25 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail, a code word or nothing; a peer's refusal screened to a code word; a fault's body; the census of error writers) — code words are 1–64 bytes of `[a-z0-9-]`, and an unlisted one stays visible with exit 14. A fault's body is sanitised to `wrapper fault`. Every error body names a writer and a witness (`error_bodies_each_name_a_writer_and_a_witness_that_stand`, `thiserror_messages_interpolate_nothing`). New lineage error paths must be added to this census (contract/code-word/format, decision/unlisted-code-words-stay-visible, error-bodies-have-writer-and-witness).

@history_item n=26 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `revive` enters the TUI passthrough boundary through the same start as `run`, and writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`). `--list` prints static ASCII rows `<ts>  <cause>  <id>`. Revive's exit-1 refusals take the fixed-message form. Carrying the parent through `revive` must leave these unchanged (contract/revive-list/human-rows, revive-human-refusal/pair).

@history_item n=27 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive` takes `--json` on both arms: refusal documents, and `--list --json` as `{"v":1,"ok":{"sessions":[…]}}` in log order, with the human forms byte-identical. If the parent shows on revive's `--list` this payload contract is extended (contract/revive-list/json, revive-json-refusal/document).

@history_item n=28 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start; at the eight frame verbs; witnesses and literal pins) — `run` and revive's start refuse a bad socket directory with exit 1 before any snapshot, bind or child (`socket-dir-refused`, revive's code, the fixed-message pair). The frame verbs, `link`/`unlink` included, take it as exit 21's sixth cause. The hint is pinned by literal. A lineage write at launch must come after these checks and must not reorder them (contract/start-refusal/socket-dir, revive-json-refusal/socket-dir-refused, decision/socket-dir-refusal-before-any-frame).

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities; their text; their witnesses and pinned pairs) — the strict-modes check runs first: `run`, revive's start and `verify` refuse a failing home tree before any log opens, with exit 1, no line and no child. The frame verbs and `link`/`unlink` refuse with exit 21. Pins are `HOME_PAIR`/`INSTANCE_PAIR`. The lineage event in `events.ndjson` can only be written after this check passes (contract/start-refusal/home-strict-modes, decision/strict-modes-refusal-before-any-write).

@history_item n=30 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (1. A11y Scope Summary; 7. Screen Reader Support) — decision/run-diagnostics-off-terminal gained the trust step's one `trust-record` diagnostics line, which is never a refusal, exit code or hint. decision/wrapped-tui-reads-as-unwrapped gained its trust-form exception. This is the closest precedent for a new launch-time record in the start path that must stay off the terminal and change no exit.

@history_item n=31 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — contract/tui-passthrough/human-keystrokes, viola-terminal-bytes and decision/human-keystrokes-never-blocked gained the start's automatic verify window as their one exception: zero viola bytes holds only from the child's start. The founder ratified this at 2026-11-22T17-29-41Z-wrap. A launch-time lineage record must add no further exception.

@history_item n=32 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — contract/tui-passthrough/human-keystrokes and decision/human-keystrokes-never-blocked now bound a key's delay by the paste's in-flight chunk and closing marker, not an atomic paste. These are the current wordings of picked rows the start path must keep.

@history_item n=33 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (1. A11y Scope Summary; 3. A11y Assertion Harness Contract; 4. ARIA Patterns & Roles) — contract/tui-passthrough/own-bytes, decision/zero-viola-bytes-on-the-terminal, wheel-ignores-terminal-events and portable-pty-outer-pty-driver each gained the keyed start's automatic-verify lines as an exception. These are the current terminal-byte rows that any change to `src/run/` start must hold.

@history_item n=34 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (8. Cognitive Accessibility: time-bounds rows) — decision/time-bounds-fall-on-the-driver and time-bounds-never-on-human gained `paste write`, and each list is now full at its bound. If lineage adds a time bound (for example a parent read at launch), it cannot be appended to these lists.

@history_item n=35 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — added decision/start-verify-stop-grace-off-the-human as a row of its own, because the two time-bound lists are full. This is the operator's ruling for full lists: any new time bound this chunk adds lands the same way.

@history_item n=36 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — added the readiness gate's 8.5 s bound on the driver only: a human key during the wait takes the wheel and the send is refused `human-typing`. The outer-PTY case is sequenced on records, never a timer. This underpins decision/human-key-takes-the-wheel and boundary-tests-sequenced-on-records, and the scope's "a parent edge never changes the wheel or whether a send is accepted".

@history_item n=37 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the closed non-editing stdin list gained its exact-grammar replies, and the takeover clause got its live reading, supplemental and never gating (decision/stdin-classifier-non-editing-list, live-readings-never-gate). Lineage must not reach the wheel classifier.

@history_item n=38 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the web entities left. The terminal surfaces stand: cli output discipline, the TUI boundary and the manual passes. The tier `Standard (1)` now rests on the terminal surfaces alone and is the founder's to re-decide (decision/a11y-tier-standard, output-discipline-stands-in-for-sc).

@history_item n=39 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's answer keeps the a11y tier at `Standard (1)`, so a fleet tree view does not re-open the tier.

@history_item n=40 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli/tui cases run as nextest cases in the `coverage` suite with no a11y driver of their own, and obs's `process` enum is `run · hook · mcp · cli`. A `lineage` event written by the wrapper carries `process:"run"` identity (contract/log-identity/process, service-name).

@history_item n=41 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's interrupted pair, `unable: verify was stopped before it finished` plus a hint, with exit 1. It is one of the named fixed-message exceptions in contract/cli-refusal/unable-line, which a new refusal form would have to join.

@history_item n=42 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the confirm windows (10 s, and 40 s for `/compact`) fall on the driver only, ending `not-delivered` / `no-prompt-submitted` (contract/send-refusal/no-prompt-submitted). Send acceptance must not depend on parent edges.

@history_item n=43 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — decision/no-screen-content-parsing gained the test-only rig exception, later narrowed through 2026-11-21-key-after-a-resize and ratified at 2026-11-22T17-29-41Z-wrap. Under the scope's no-live-start directive, any lineage measurement through a TUI read stays within this ban.

@history_item n=44 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install` joined the `setup:` row, held by `help_lists_only_served_verbs`. A new launch flag on `viola run` for declaring a parent changes the `--help` table only through these pins (contract/cli-help/usage-line, group-labels).
