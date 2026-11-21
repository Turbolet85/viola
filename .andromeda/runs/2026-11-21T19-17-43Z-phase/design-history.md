@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — made `mcp` a hidden verb (`#[command(hide = true)]`), run by the plugin and never by a person, in no `viola --help` row (decision/mcp-hidden-verb). Any bound, cap or split this chunk adds to the MCP tools gets no human help row or human wording.

@history_item n=2 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, `--json` included (decision/silent-catch-sites, contract/cli-catch-site/silent-verbs). A timeout or deadline fault inside `viola mcp` must surface as an MCP frame, never as a catch-site line on stdout or stderr.

@history_item n=3 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `endpoint-lost` hint; how a frame verb reads a lost call) — added `endpoint-lost`, the wrapper lost while the call is open, as a closed cause of exit 21 with its own hint, and set the rule that a failed connect reads `instance-dead`. The founder confirmed the wording at 2026-11-01-t02-46-44-wrap. A client-side deadline that gives up on an open call must not be confused with this cause; either it reuses a closed word or it needs a new founder-ruled word.

@history_item n=4 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause) — exit 21's `--json` `detail` became one of a closed set of cause words (now six with `socket-dir-refused`), on the founder's own word. Adding a cause word is a widening shown to the founder, so a new timeout cause cannot be added silently.

@history_item n=5 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 20's closed `detail` and the fixed `wrapper fault` message) — made exit 20's `detail` closed for five known codes through `viola_channel::sanitised_fault`, with `message` each code's fixed text and MCP carrying the same fixed message; any other `-32602` reads `invalid params` (contract/wrapper-fault-detail/*, contract/cli-exit-codes/20). An error class for a timed-out or bounded call must fit this closed set and must not carry upstream text.

@history_item n=6 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the code word and the screen on a peer's reply) — both clients, `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`), screen a peer's `refusal` and `detail` through `viola_core::is_code_word`. `result.rs` is a file in this chunk's scope, so any new result shape for a deadline goes through the same screen.

@history_item n=7 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail, a code word or nothing; the `--json` failures the witness drives) — `unknown` (exit 14) carries a code word or no detail, and has no hint (decision/no-hint-for-faults). Each `--json` failure prints one document with no ESC, path or upstream text (decision/no-internals-in-errors). A timeout reading must hold to both.

@history_item n=8 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines, no `--json` document) — carved out the exception to decision/json-one-document-on-stdout: an oversize `answer` and an unusable `send` exit 2 with a fixed stderr line. A cap on a caller's argument, such as `wait`'s timeout, either reuses this precedent or must be named as a new exception.

@history_item n=9 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json`'s documents) — added the `pump_child` exception now in decision/json-one-document-on-stdout. This shows that every exception to one document is named in the row, so a new one would need the same.

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' strict-modes refusal prints the instance-state pair) — `send`, `wait`, `last`, `answer`, `pause` and `release` run the instance strict-modes check before any frame (`send` after reading its text). A refusal exits 21 with the fixed pair, writes no frame and no line, and does not use the `[/ ]` mirror (contract/strict-modes-refusal/exit-21, contract/cli-strict-modes-refusal/instance-state-pair). This check is part of `send`'s client-side path, which the chunk must time within the bound.

@history_item n=11 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal; their `--json` documents and exit 20) — `link` / `unlink` take the instance-state pair at exit 21. A pair the wrapper refuses (`-32602`) exits 20 as a wrapper fault (contract/viola-link-output/refused-pair). These are the precedents for the strict-modes and wrapper-fault rows this chunk rests on.

@history_item n=12 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the server-verification cause and hint cover eight frame verbs) — the eight frame verbs run the server identity check before any frame (`server-verify-failed`, exit 21). The 2026-10-26-server-verification-before-any-frame entries first built it for six verbs. This check is another client-side step in `send`'s path before the frame.

@history_item n=13 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the eight frame verbs) — the client's `connect`, shared by every frame verb, checks the endpoint's directory before it connects (`socket-dir-refused`, exit 21). This is one more pre-frame step inside `send`'s and `wait`'s bounded block.

@history_item n=14 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the wrapper's silent drop of a refused peer) — the wrapper drops a refused peer before any frame and writes no line, and the client reads end-of-file. This is a client read path whose end must stay bounded and classed.

@history_item n=15 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint became `<name> was not ready for input; send again, and if it repeats a human must look at the session`, because no turn runs in any of its causes. This is the wrapper's typing and delivery stage of `send`, the longest block this chunk bounds. Its refusal and hint wording stay fixed.

@history_item n=16 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on `/clear`'s measured post-condition as well as on `prompt-submitted` (contract/send-readback/clear). `send`'s confirmation wait covers each of these confirmers, so the bound must cover the longest of them.

@history_item n=17 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — on a verified CLI, a `/compact` send reads back on a same-session `session-start` of cause `compact`. The entry also brought verify to 24 rows. `/compact` is one of `send`'s post-condition waits that the MCP bound must cover (contract/send-readback/compact). The ledger row is also a precedent for growing the verify step counter.

@history_item n=18 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — made the prompt readback match tolerate a trailing whitespace run, provisionally (contract/send-readback/prompt). The founder ratified it at 2026-11-16T06-05-33-wrap, and a leading trim waits on a live reading. This is the matching step at the end of `send`'s confirmation wait, and the chunk must not change it.

@history_item n=19 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added contract/send-readback/mcp, which confirms on a new local-command stdout tag in the transcript, and widened contract/send-readback/unconfirmable. The `/mcp` transcript-tag wait is one of `send`'s confirmation waits whose length the chunk must bound.

@history_item n=20 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added contract/send-readback/context, extended `unconfirmable`, and tied contract/cli-verification/verified and unverified-cli to a stamp that reads every relied row `pass`. If the bound becomes a 30th relied row, every stamp that lacks it reads `unverified-cli`. Each verified-only confirmation (`/compact`, `/mcp`, `/context`) would then drop to `unconfirmable` until a re-stamp.

@history_item n=21 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — added the automatic-verify start lines (contract/auto-verify-line/*, earlier-red among them), provisional on the delegate's confirm. A new relied row changes what a start's automatic verify runs and which versions read earlier-red. The P3 fork must price this.

@history_item n=22 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the verify step counter counts relied rows (`MM` = `LedgerRow::relied()`), and the summary's n + m equals `MM`. A 30th relied row moves `MM` and the summary through this rule, with no new wording.

@history_item n=23 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — `LedgerRow::relied()` reads the same count on every build, with no OS qualifier. A new relied row counts once, with no OS split.

@history_item n=24 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — tied the step counter to `LedgerRow::ALL` so the count carries its rule. Earlier entries grew the counter row by row: 2026-09-28-capability-ledger-and-viola-verify (6), 2026-10-05-real-cli-verify-probes (10), 2026-10-05-dialog-rows-and-re-probe (14), 2026-10-06-local-command-and-paste-framing-rows (17), 2026-10-14-resume-and-compact-ledger-rows (22) and 2026-10-16-unwrapped-sessions-on-the-board (23). This is the precedent for a ledger-row option: one static ASCII step line with its words, and the master example updated.

@history_item n=25 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added contract/viola-verify-output/trust-record-names-nothing for a verify row that writes into the Claude config. This is the precedent for a verify probe that newly writes or spawns, which the scope requires the P3 block to state in full. No output may name the file, key or value.

@history_item n=26 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — added the verdict word `absent` on the `browser-tools-prompt` row only, counted as a pass in the summary (contract/viola-verify-output/step-line, summary, summary-line), provisional on the delegate's confirm. A new row reads `pass` or `fail` unless a new verdict word is ruled. This is the precedent and the trap for a probe that cannot always measure the client timeout.

@history_item n=27 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — added contract/viola-verify-output/browser-tools-line, a fixed extra stdout line before the summary. This is the precedent: any extra verify output a new probe needs is a new row that the masters must state.

@history_item n=28 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — fixed `wait` / `last` result lines (`session-end` takes the event form) and message-mode `\xHH` escaping (contract/viola-wait-output/event-form, contract/cli-message-escaping/control-characters). `wait`'s caller-supplied wait, which this chunk caps or bounds, ends in these forms or in the `timed out` line.

@history_item n=29 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog-expired` takes the `wait` dialog form (contract/viola-wait-output/dialog-form); the founder confirmed it at 2026-10-20-t06-58-31-wrap. This is a kind `wait` can return while a hold is in progress, inside the bounded wait.

@history_item n=30 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — added decision/api-error-turn-end-shown-as-turn-end. 2026-11-16-granular-api-error-cause widened the cause to one closed word of nine in `--json` and MCP `wait` event data, and 2026-11-17T05-21-35Z-wrap ratified it. MCP `wait`'s event data is fixed. A bound on `wait` must not add a distinct status or text.

@history_item n=31 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the `claude agents --json` join. A failed, oversize or malformed read prints the board as if no `claude` were present (contract/viola-list-board/failed-agents-read). The MCP `list` tool uses the same board reader. Its `claude agents` child read is a block whose length this chunk must bound, and its failure reading is fixed.

@history_item n=32 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — set `list`'s exit-21 whole-board refusal. 2026-10-13-t06-38-35-wrap made it the founder's, and 2026-11-01-unix-endpoint-and-home-hardening set the board's one rule on the home (decision/list-refuses-board-on-strict-modes-failure). The MCP `list` tool reads the same refusal document byte for byte.

@history_item n=33 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — amended contract/refusal-hints/refusal-line: `send` and `answer` print the same hint for the same reason or detail, and an opaque or absent `not-delivered` detail prints no hint. A new refusal from a bound gets a hint keyed this way, or none.

@history_item n=34 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — removed `budget-paused` from decision/fixed-phraseology-vocabulary and from decision/no-hint-names-viola-release's why. Exit 11 is retired and never reused. A bound that refuses must not reuse exit 11, and any new word joins the fixed vocabulary.

@history_item n=35 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — scoped the spinner, progress-bar and live-redraw ban to a verb's output (decision/no-spinners-in-verb-output). A long-running `send` or `wait` may not gain a progress display. MCP progress notifications, if used to reset the client's timeout, are MCP frames and not human output.

@history_item n=36 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master text into rows: a master holds what a planner must respect, and the code holds the rest. This chunk's bound, cap or split lands as amended or new rows (`+` / `~`), each verified against the code. A compiled bound with a test and no master row is a valid outcome only when no row states the figure, which matches the scope's rule that no doc leaf gains a figure the code states.
