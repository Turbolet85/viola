@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Surface: console: 3 rows) — added contract/fleet-console-keys/q, /ctrl-c and /unbound (every other key unbound and ignored, end of input ends with exit 0) under the new Surface: console area, PROVISIONAL on the overseer's word. This chunk binds keys to verbs, so it must amend /unbound (and likely add one row per bound key) while keeping `q` / Ctrl-C quitting with exit 0 from every mode (no key trap).

@history_item n=2 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** (Surface: cli: 5 rows) — `console` joined the help `board:` row, decision/list-is-the-board made `console` act on no strip, contract/cli-refusal-line/fixed-unable-form named `console`'s exit-1 refusals, and the no-terminal / home-refused pairs landed. A console that now acts on a selected strip through the verbs strains "console acts on no strip"; the planner must decide whether that row is reworded.

@history_item n=3 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Surface: console: 5 rows) — added the caption row (board's six + `LAST`), LAST, line cut (a whole line cut `...`, unwrapped NAME not cut separately), the `... <n> more` line, and the mid-run home-widened refusal (exit 1, pair after terminal restore), PROVISIONAL. A selection must name its row by the board's item, not drawn text; rows past the body need a scroll or move this chunk would add, which amends the more-line row; action refusals shown on the console must sit beside the mid-run refusal's restore-then-print rule.

@history_item n=4 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Surface: cli: decision/fleet-console-not-verb-output, rule) — reworded to «The terminal fleet console (`viola console`), its own surface, is governed by no verb-output rule», PROVISIONAL. An action runs a verb, yet the verb's printed `unable` line and readback mirror appear on the console's surface: the planner must decide which verb-output rules the actions' result lines carry into the console.

@history_item n=5 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Surface: cli: decision/fleet-console-not-verb-output, name) — the row's name became «Fleet console governed by no verb-output rule», following the reworded rule; same bearing as the rule entry.

@history_item n=6 marker=2026-11-28-fleet-console-view
**2026-11-28-fleet-console-view** (Anti-Patterns: decision/escape-controls-in-human-output) — the rule now names the fleet console beside `last` and `list` (every frame cell row-escaped, control `fleet-escape`). Any action result shown in the console (a `last` text, a refusal detail, a typed send text echo) must stay escaped.

@history_item n=7 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the console got its own surface section; «no interactive prompt, no TUI mode and no pager» and the spinner / progress-bar / live-redraw ban were scoped to a verb's output, the console's redraw rule left to it; the design direction is the CLI's, inherited by the console. A send's text entry, any confirm and the open-readback wait in the console are governed by this scoping: no spinner is still the brand's, and the console needs its own rule for them.

@history_item n=8 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — fixed the readback box's one place as its CLI mirror (`[RB]` / `[  ]` / `[/ ]` of `viola send`) and the Signature Test to that one place. A console send that "shows its readback" either draws the box in a second place (amending decision/readback-box and decision/readback-box-signature-check) or prints the verb's mirror line as is.

@history_item n=9 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Surface: cli: 4 rows) — `switch` joined the `setup:` help row and the exit-21 strict-modes refusal lists (fixed-unable-form, instance-state pair, strict-modes-refusal/exit-21). The console's switch leg (carry cry-mf2w3ioc) goes through this verb, so it inherits these refusals and must print the instance-state pair, not a mirror.

@history_item n=10 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Surface: cli: 3 rows, 1 of 2) — exit 13's details grew with `dialog-pending`, `no-session`, `child-not-ended`, `resume-failed` (fork F3, PROVISIONAL, ratified 2026-11-27). A console switch's refusal lines must show these closed details as the verb does.

@history_item n=11 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (Surface: cli: 3 rows, 2 of 2) — `switch` made the seventh strict-modes frame verb and the ninth frame-writing verb reading `server-verify-failed`. The set of frame verbs the console reaches, and the refusals each can give, is this list.

@history_item n=12 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (ratified) — the operator's own word ratified every PROVISIONAL mark standing at a80df0e5: the switch exit-13 details, `up`'s rows, adopt's not-resumed rows and the API-error turn end among them. These rows the console reaches are standing, not open.

@history_item n=13 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — contract/send-readback/prompt now says what a match is: the sent text or the sent text with its trailing ASCII whitespace run removed; a leading trim does not match. A console send's text entry must not trim the text's start, or its readback never fills.

@history_item n=14 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance of contract/send-readback/prompt; a leading trim waits for a live reading. The console must not widen it.

@history_item n=15 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on `/clear`'s post-condition too (a `session-start` with cause `clear`), no new word. A console send of a local command shows the same `read back` / `unconfirmable` words, never a console-made success.

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` reads back on a verified CLI once a same-id `session-start` of cause `compact` lands, else `unconfirmable`. Part of the readback set a console send inherits; the wait for it can outlast the view's re-read.

@history_item n=17 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `/mcp` reads back on a new local-command stdout tag; contract/send-readback/unconfirmable lists `/remote-control`, `/cost`, `/usage` and the unverified or no-transcript cases. The console's readback display must keep `unconfirmable` as the open drawing, never filled.

@history_item n=18 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added contract/send-readback/context and reworded cli-verification/verified and /unverified-cli (a stamp must read every relied row `pass`). A console send on an unverified CLI shows `unverified-cli` / `unconfirmable` as the verb does.

@history_item n=19 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' strict-modes refusal) — `send`, `wait`, `last`, `answer`, `pause`, `release` check strict modes before any frame and print the instance-state pair (exit 21), not the `[/ ]` mirror, writing no frame and no line. A console action's refusal for a failing instance is this pair, beside the console's own mid-run home refusal.

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause; the `endpoint-lost` hint and the four hint causes) — exit 21 carries one closed cause word, and `instance-unreachable` prints one hint per cause (not running, unwrapped, endpoint lost, server verification). A console action on a stale or unwrapped row meets these refusals, and a bare (unwrapped) strip is never a target.

@history_item n=21 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the eight frame verbs; `socket-dir-refused`) — a refused socket directory exits 21 before any frame with `unable  <name>  instance-unreachable` and the `socket-dir` hint, `send` in its mirror's form. One more refusal a console action must show as the verb prints it.

@history_item n=22 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — contract/refusal-hints/refusal-line now says `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque or no detail prints no hint. Console refusal lines must keep hint keying identical to the verbs'.

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the code word and the screen on a peer's reply; `unknown`'s detail) — both clients screen a peer's `refusal` / `detail` through the code word; a non-code-word refusal reads `unknown` (exit 14, no hint). A console action showing a verb's refusal must take the screened words, never peer text.

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli: 10 rows; Brand Identity: 3 rows) — decision/no-hint-names-viola-release's why now names only the human-typing refusal reaching drivers, and `budget-paused` left the vocabulary; exit 11 retired. The console, driven by the human, may reach `release`, but no hint it prints may name `viola release`.

@history_item n=25 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — fixed `revive`'s exit-1 refusals and hints, read in order stopping at the first, none showing a path, pid or logged id, and `revive` printing nothing once the child starts. A console revive acts through this verb: its child takes a terminal, which the console's own raw-mode frame holds.

@history_item n=26 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added `revive`'s session-live pair and built the unwrapped rack (`-- UNWRAPPED - READ-ONLY --`, `n/a` cells). A console selection may land on an unwrapped row, which takes no command.

@history_item n=27 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Brand Identity: decision/run-prints-nothing-while-wrapped) — the silence while wrapped covers `run`, a passed `revive` and `adopt`. A console reaching revive or open must not print over the child's screen.

@history_item n=28 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Surface: cli: 4 rows) — decision/no-json-after-child-held-terminal and contract/cli-catch-site/silent-verbs name `adopt` beside `revive`'s start: nothing is written past a child that held the terminal. A console that runs a child-starting verb inherits this.

@history_item n=29 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `run` / `revive` print exactly one of five fixed automatic-verify stderr lines at start (ratified 2026-11-22). A console revive that runs the verb would get these lines on its own surface, which the planner must place.

@history_item n=30 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — reworded auto-verify-line/did-not-verify and /no-stamp (an untrusted-folder refusal ends `unstamped`, the version unattempted). Same bearing as the automatic-verify lines for a console revive.

@history_item n=31 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Surface: cli: 6 rows, 1 of 2; 8 rows, 2 of 2) — added `viola open`, its exit-1 fixed pairs and `--json` documents, the eight closed placement words and the two origin words (`cli`, `agent`). matrix#v2-15 names "open a child": a console open goes through this verb and shows its placement word and refusals; the origin of a console-made open is to be read against the closed `cli` / `agent` pair.

@history_item n=32 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Surface: cli: 6 rows, 1 of 2; 5 rows, 2 of 2) — added `viola focus`, its `<name>  focused` line, six closed exit-1 codes and exit-21 refusals before any compositor read. A console focus action prints these outcomes; focus prints no window address, pid or workspace.

@history_item n=33 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (the operator's answers to F-A, F-B, F-C and F-E ratified) — the focus rows (help `board: list, focus`, the verb output, the six codes, the exit-21 refusals) stand as written. The console reaches a standing verb.

@history_item n=34 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (Surface: cli: decision/permission-request-question-held) — a PermissionRequest question is held as a pending dialog and answered by `viola answer`; a human key, an expired hold or an unverified CLI leave it to the human. A console answer action reaches exactly these dialogs, through `answer`'s `answered` / `unknown-dialog` lines.

@history_item n=35 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — added `paste-cut` to exit 13 and its human-only hint. A console send of a long text can meet it; the hint prints in human mode only.

@history_item n=36 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint became `<name> was not ready for input; send again, and if it repeats a human must look at the session`. A console send's refusal shows this hint verbatim.

@history_item n=37 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `wait` / `last` result lines and message mode (every control but `\n` and `\t` as `\xHH`). A console `last` action shows message text; its `\n` / `\t` keep-rule differs from the list row escape the console view uses.

@history_item n=38 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Surface: cli: row order; Iconography: list-six-fixed-columns, viola-list/columns, ascii-arrow) — rows keep the board's name order with each wrapped child under its parent; links print apart in a `-- LINKS --` block. A console selection moves over this order; slots stay stable (no re-sort by a selection or an action).

@history_item n=39 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour layer (bold NAME, amber `DIALOG` only, dim stale row) and the width from stdout's own terminal, `COLUMNS` never read. A selection marker in the console must not add a colour or a third hue (amber stays dialog-only, no background colour).

@history_item n=40 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — added decision/api-error-turn-end-shown-as-turn-end (ratified 2026-11-17 and 2026-11-27): an API-error end shows as any turn end, STATUS `idle`, no distinct mark. A console send's outcome must not treat STATUS as its readback, nor mark an API-error end.
