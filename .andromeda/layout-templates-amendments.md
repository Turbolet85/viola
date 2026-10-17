# layout-templates — amendments

History of amendments to `.andromeda/layout-templates.md`, one entry per amendment (form: `/andromeda-wrap-session` `references/sidecar-contract.md`). The body holds only current truth.

## 2026-09-28-capability-ledger-and-viola-verify — `viola verify`'s output structure and its refusal pairs
**Section:** Surface: cli Output structure — `viola verify`; Component — Primary content block 2 (refusal lines)
**Change:**
- The `viola verify` wireframe shows the six ledger rows `[01/06] shim-resolution …` … `[06/06] largest-hook-payload …`, the last stdout line `stamped 2.1.283  6 pass  0 fail`, a failing-row run (`4 pass  2 fail`, exit 1, the stamp still written) and the usage `viola verify [--record <DIR>] [-- <program> [args…]]`; `MM` grows as owning chunks land rows (was the 14-row `[01/14] S3 …` / `stamped 2.1.280  14 pass` sketch).
- The fixed-message exit-1 exceptions add `viola verify`'s four `unable:`/`hint:` pairs (CLI not found, `.cmd`/`.bat`, unreadable version, a dirty recording), run's pinned-copy refusal, and a failed verify's exact `error: internal error`.
**Why:** the chunk landed the verb with those lines (report Symbols; `tests/cli_verify.rs`).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-29-t15-07-57-wrap — registry migration (U35): the layout-templates Decisions Log leaves the body
**Section:** Decisions Log; Surface: web-spa; Component — Header (`<viola-atis>`); Component — Primary navigation (rack separators and keyboard order); Component — Hero / signature section; Surface: cli
**Change:**
- The log moved verbatim to layout-templates-amendments-archive.md (4 entries: the 2026-09-24 initial generation and the three 2026-09-24 overseer fix passes T3–T5, Y4, Z10).
- Surface: web-spa — the document changes no design-system token, colour, font stack, expression level or signature drawing; design-system Component Patterns are its base.
- Header — every boxed ATIS word carries `space-micro` inline padding.
- Primary navigation — the semantic roles of racks, strips and tape, and the announcement mechanics, are a11y's.
- Primary navigation — the v1 bay reserves no control slot; the v1.x brake controls are placed with the security plan at implementation.
- Hero / signature — the cock plays only on a WRAPPED-rack strip, never in the tape, on an unwrapped strip or on the CLI.
- Hero / signature — the `dialog_pending` data plumbing is decided at implementation, outside the layout.
- Surface: cli — one token vocabulary on both surfaces; the differences are platform mechanics only.
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/

## 2026-10-01-t12-19-55-wrap — web-spa tooling context: React + TypeScript
**Section:** Surface: web-spa → Tooling context
**Change:** Framework was Lit 3.3.3 (vendored ESM, no JS build step) with light-DOM `viola-*` Lit elements; now React + TypeScript, a built bundle embedded in `viola`, with the bundler, versions and embedding OPEN (owned by the route's frontend-toolchain entry) and the page built from plain `viola-*` components whose mapping onto React's output is OPEN. The no-`style` rule names React's `style` prop; `dangerouslySetInnerHTML` replaces `styleMap` / `unsafeHTML` in it.
**Why:** founder ruling of 2026-09-30, relayed by the overseer (0.2.0's node-graph view does not fit a build-less Lit page).
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/

## 2026-10-04-wait-and-last — wait / last lines as landed
**Section:** Surface: cli → signature placement · Output structure — `viola wait` / `viola last` · Component — Primary content block 2
**Change:**
- The output structure gains the `session-end` line (every non-dialog kind takes the `turn-ended` form), `dialog unknown`, the exit-21 pair `unable  <name>  instance-unreachable` + its `hint:`, `waiting:` only on a terminal stderr without `--json`, one `--json` document, and the once-printed `error: internal error`.
- Block 2: was "a failed or panicked `verify` prints `error: internal error`"; now every `cli` verb (`send`, `wait`, `last`, `verify`) from the one catch site; the wait/last exit-21 hint is listed; `send` keeps its `[/ ] unable` mirror.
- wait/last join the verbs sharing the `unable` word column.
**Why:** the lines the chunk shipped (the plan's lean: the layout form with the name, over the design line that omits it).
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/

## 2026-10-04-dialog-answers-by-dialog-id — the catch-site verb list gains answer
**Section:** §Surface: cli → Component — Primary content block 2 (refusal lines)
**Change:** a failed or panicked `cli` verb was `send`, `wait`, `last`, `verify`; now `send`, `wait`, `last`, `answer`, `verify` — each prints exactly `error: internal error` once from the one catch site, no hint.
**Why:** the chunk added the `viola answer` verb, which exits 1 through the shared catch site.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/

## 2026-10-04-the-wheel — pause and release join the internal-error verbs
**Section:** §Surface: cli → Component — Primary content block 2 (refusal lines)
**Change:** the failed-or-panicked `cli` verbs: was `send`, `wait`, `last`, `answer`, `verify`; now also `pause` and `release` — each prints exactly `error: internal error` once from the one catch site, no hint.
**Why:** the chunk added the `viola pause` / `viola release` verbs, which exit 1 through the shared catch site on any unexpected reply.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/

## 2026-10-05-real-cli-verify-probes — the verify wireframe at ten rows, its help paragraph, the named refusal
**Section:** §Surface: cli → Output structure — `viola verify` · Component — Primary content block 2 (refusal lines)
**Change:**
- Wireframe: `[NN/06]` → `[NN/10]`, adding `[07/10] modal-signature …` through `[10/10] confirm-window …`. The summary is `stamped 2.1.288  10 pass  0 fail`, and on a failing row `8 pass  2 fail` (was 2.1.283, 6 / 4). The ledger order lists the ten rows, and `MM` is 10 today.
- `viola verify --help` adds one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
- Refusal lines: the fixed `a recorded payload still holds a path or a username` became `a recorded fixture is not clean: <file> <code>` (`<file> row <n>[ seam] <code>` for a screen; codes `home-path` · `absolute-path` · `username` · `email`; never the content). The hint is unchanged.
**Why:** the chunk landed the four screen and timing rows and the help paragraph. The named refusal is the founder's live ruling at this wrap.
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/

## 2026-10-05-dialog-rows-and-re-probe — verify output at fourteen rows
**Section:** §Output structure — `viola verify`
**Change:** the wireframe counters `/10` → `/14`, rows `[11/14] question-answer …` and `[14/14] dialog-concurrency …` shown, the summary `14 pass  0 fail`, the failing example `12 pass  2 fail` (was `8 pass  2 fail`); the ledger-order list gains the four dialog rows and `MM` is 14 (was 10).
**Why:** the chunk landed the dialog rows.
**Kept:** the `--help` sentence "verify never answers it" is about the CLI-native external-import dialog, which verify still never answers; the probe's own dialogs are answered by its hook, not by verify typing.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/
## 2026-10-06-local-command-and-paste-framing-rows — the verify wireframe at seventeen rows
**Section:** §Surface: cli → Output structure — `viola verify` (both wireframe runs; the usage paragraph)
**Change:** the six shown counters read `/17` (was `/14`); three lines follow `[14/17] dialog-concurrency`: `[15/17] long-paste-wrapper a long paste unwraps to the text as pasted`, `[16/17] tag-escaping tag-like text un-escapes to the text as pasted`, `[17/17] local-command-clear /clear starts a new session and submits no prompt`; the passing summary reads `stamped 2.1.287  17 pass  0 fail` (was `stamped 2.1.288  14 pass  0 fail`); the failing example reads `stamped 2.1.287  15 pass  2 fail` (was `12 pass  2 fail`); the ledger-order list gains `long-paste-wrapper` · `tag-escaping` · `local-command-clear`; `MM` is 17 today (was 14).
**Why:** the chunk landed three ledger rows; the examples name 2.1.287, the one stamped set, and the failing example is the chunk's own first record round.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
## 2026-10-06-local-command-send-outcomes — the read-back line's second trigger
**Section:** §Surface: cli → Signature placement (item 2, the outcome line)
**Change:** was "`[RB] read back …` on stdout with exit 0 when the matching `prompt-submitted` confirms the send"; now it prints when the send is confirmed, by the matching `prompt-submitted` or, for `/clear` on a verified CLI version, by its new-session post-condition (a `session-start` with cause `clear` and a new session id). The `unable` and `unconfirmable` clauses are unchanged.
**Why:** the chunk made `send` confirm `/clear` by its post-condition; the web statements of the same trigger already carried it.
**Kept:** the `viola send` wireframe's `clear.txt` example ending `not-delivered  input-not-ready` stays: the refusal rungs apply to a listed command as to any text.
**Ref:** .andromeda/runs/2026-10-06T23-49-33-wrap/
## 2026-10-08-first-live-test-and-self-drive — the `viola send` specimen's `input-not-ready` hint line
**Section:** §Surface: cli › Output structure — `viola send`
**Change:** the specimen's last stderr line reads `hint: builder was not ready for input; send again, and if it repeats a human must look at the session` (was: `hint: builder was not ready for input; viola wait builder, then send again`). Nothing else in the block moved.
**Why:** the specimen follows the shipped text (the founder's wording).
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-09-epoch-3-cleanup — refusals `viola send` decides before any request
**Section:** §Surface: cli › Signature placement (item 2); §Surface: cli › Component — Primary content block 2: refusal lines and the `unable` column
**Change:** block 2 gains a bullet: `viola send` refuses two texts itself before any frame, one holding a refused character (`not-delivered  control-character`) and one whose typed text is empty (`not-delivered  empty-text`); each takes the `[/ ] unable` line on stderr with exit 13 and its hint as the last stderr line, and one document under `--json`; the wrapper makes the same two refusals for a frame sent straight to it. The signature sentence reads: the struck line prints when the wrapper returns a refusal, or with exit 13 when the client itself refuses the text before any frame (was: only "when the wrapper returns a refusal").
**Why:** the chunk added the `empty-text` refusal on the client side. The `control-character` client refusal predates it and the sentence had never named it.
**Kept:** no new wireframe: the existing struck form carries the shape. Whether the issue line prints on a client-side refusal is not stated: nothing measured it.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/

## 2026-10-10-viola-revive — an output structure for `viola revive`; the verb in the command list and the `setup` help group
**Section:** §Surface: cli (Expression level; Primary screens (commands); Output structure — `viola --help`; a new Output structure — `viola revive`; Component — Primary navigation; Component — Primary content block 2, Line form)
**Change:**
- New Output structure — `viola revive`: a start that passes its preflight prints nothing of its own while the child holds the terminal; `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>`, two spaces between fields, no header, no colour, plain ASCII, empty stderr, the cause one of `startup`, `clear`, `resume`, `compact`, else `unknown`; each refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout, the five pairs listed (the state-files pair, `run`'s collision pair, the two no-session pairs, the recorded-directory pair); `--list` has two of them; a malformed `--id` is clap's usage error, exit 2; the verb takes no `--json`.
- Primary screens: a `viola revive` bullet after `viola run`. `viola --help`: `setup:    run, revive, verify, plugin install`.
- Primary navigation: `--json` is the machine view of every data verb except `viola revive`, owed to the route entry "CLI machine contract" (was "every data verb").
- Line form: the fixed-message exceptions name the exit-1 refusals of `viola revive` beside `run`'s and `verify`'s. Expression level: the 0.0 list names the `viola run` / `viola revive` passthrough.
**Why:** the chunk landed a new user-facing verb with a list arm and four refusals of its own. The help group is the operator's answer at this wrap's Phase 2 halt.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-budget-governor — the two release --budget output lines
**Section:** §Output structure — wheel, handoff and dialog verbs
**Change:** the verb block shows both outcomes of `viola release <name> --budget`: `<name>  budget gate lifted until <end>  wheel <holder>` when a pause stood and `<name>  budget gate open  wheel <holder>` when none did, each one stdout line, exit 0, the wheel left where it is. `<end>` is the reply's `override_until` as `HH:MM:SSZ` on the current UTC date and `YYYY-MM-DD HH:MM:SSZ` on any other; a value that is no RFC 3339 instant prints escaped. Was the time-only lifted line alone. The date form and the `budget gate open` line are marked planning forms, the founder's wording owed.
**Why:** the block showed one of the two lines the verb now prints, and only one form of its time. The two new forms landed on the operator's word and are not worded by the founder.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-11-the-board-viola-list — viola list as built, its widths and its refusals
**Section:** §Surface: cli (Primary screens; Output structure — `viola list` piped and `--json`; Primary content block 1 strip rows; Primary content block 2 refusal lines)
**Change:**
- Output structure gains an As built paragraph: the uncoloured form on every stream as one stdout write; the styled TTY form not built ("CLI output discipline"); the unwrapped rack not built and STATUS always `unknown` ("Unwrapped sessions on the board"). Widths: NAME the longer of 4 and the longest name, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI unpadded; no trailing space, nothing truncated; `unknown` WHEEL / CLI and `DIALOG unknown` cells; the planning forms named.
- `list --json` gains its refusal document, exit 21, stderr empty, no log, mode as found.
- Block 1 gains an As built note (wrapped rows only, no styling or separator; `escape_row` covers DEL and C1; one upstream cell, the CLI version).
- Block 2: the fixed-message exceptions gain `list`'s two exit-21 state pairs, stdout empty; the catch-site verbs gain `list` (an unlistable `instances/`, a failed log start; a per-instance read error gives no row); the exit-code tail notes `list`'s 21 prints the pair.
- `--help` is marked not built (clap's ordered list), owed to "CLI output discipline".
**Why:** the board as built. The refusal and the planning forms stand on the operator's word, not ruled by the founder; WHEEL 23 and the exit-1 paths are implement's decisions, accepted by the operator at this wrap.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/

## 2026-10-13-t06-38-35-wrap — the board's forms, WHEEL 23, the list refusal, the unlistable instances/ exit 1 and the two budget lines are the founder's
**Section:** Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) · Output structure — `viola list --json` · Output structure — wheel, handoff and dialog verbs · Component — Primary content block 2: refusal lines and the `unable` column
**Change:**
- WHEEL 23 confirmed by the founder (was accepted by the operator); the planning forms of design-system cli pattern 1 are his wording (was "on the operator's word, not worded by the founder").
- `viola list --json`'s exit-21 refusal with no log opened, and the human refusal pairs in the line form, are his (was the operator's answer, not ruled).
- The `cli` internal-error clause: the exit 1 of an `instances/` that cannot be listed is his; the failed log start and the no-row read error stay implement's decisions accepted by the operator, not ruled by him.
- `release --budget`'s date form and `budget gate open` line are his wording (was owed).
Each superseded mark stands as the operator's word, superseded 2026-10-13.
**Why:** the founder's own answer of 2026-10-13 («Подтвердить всё»), relayed verbatim by the Viola overseer through the operator's directive. The operator scoped it at this wrap: the question shown to him named the unlistable `instances/` exit 1 and WHEEL 23 only, so the other two list decisions go to his next batch.
**Ref:** .andromeda/runs/2026-10-13T06-38-35-wrap/

## 2026-10-13-interrupted-verify-cleanup — verify's interrupted form
**Section:** §Surface: cli → Output structure — `viola verify` · Component — Primary content block 2 · Component — Footer / terminator
**Change:**
- Output structure gains a third example: a run stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) before its rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` on stderr, with stdout empty, no summary and no stamp, and exits 1. The exit list reads: 1 on a failing row, a refusal or a stop.
- The pair joins verify's fixed-message exit-1 pairs in Primary content block 2.
- Was: the summary closes the run. Now it closes a completed run; a stopped run ends on its `hint:` line.
**Why:** The operator's P4 answer gave the interrupted run exit 1 and a fixed pair; the wording is a planning form with the founder's wording owed.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/

## 2026-10-13-paste-newline-ledger-row — `viola verify` output at eighteen rows
**Section:** §Output structure — `viola verify`
**Change:** The example block reads `/18`, ending `[17/18] local-command-clear …` and `[18/18] paste-newline an LF inside a paste stays inside the one prompt submitted  pass`, then `stamped 2.1.287  18 pass  0 fail`. The failing example reads `16 pass  2 fail`. The ledger-order list gains `paste-newline`. `MM` was 17; now it is `LedgerRow::ALL.len()`, 18 today.
**Why:** The capability ledger gained its eighteenth row, and `viola verify` prints one step line per row in ledger order. The count now carries its rule.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/

## 2026-10-13-owed-ledger-rows — the `viola verify` output at the relied count
**Section:** §Output structure — `viola verify`
**Change:**
- The worked example is a Unix host at twenty-one relied rows: `[01/21]` … `[18/21] paste-newline`, then `[19/21] statusline-override`, `[20/21] statusline-shell`, `[21/21] statusline-rate-limits`, and `stamped 2.1.287  21 pass  0 fail`. The failing example is `19 pass  2 fail`.
- The usage line lists the three ids after `paste-newline`. `MM` is the relied count, `LedgerRow::relied()`'s length: 21 on Unix, 18 on Windows, which stops at `paste-newline`. It was `LedgerRow::ALL.len()`, 18.
- One step line per relied row; exit 0 when every relied row passes.
**Why:** the three statusline rows are relied only where the status line is wrapped (the founder's cut, `inputs#I1`). The count carries its rule (`relied()`).
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/

## 2026-10-14-resume-and-compact-ledger-rows — the verify example block at 22 rows
**Section:** §Output structure — `viola verify` (the example block, the failing-row example, Usage)
**Change:**
- The Unix example block has 22 relied rows (`[NN/22]`). Its new last step line is `[22/22] resume-session --resume <id> reopens the session under that id  pass`, and its summary is `22 pass  0 fail`.
- The failing example reads `20 pass  2 fail`, summing to 22.
- Usage:
  - the ledger order ends `resume-session`;
  - MM is 22 on Unix and 19 on Windows;
  - was "18 on Windows, which stops at `paste-newline`"; now the Windows set skips the three statusline rows and ends at `resume-session`.
- Signature placement's generic local-command wording covers `/compact` unchanged.
**Why:** The 22nd ledger row landed, relied on every OS.
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/

## 2026-10-16T14-58-10-wrap — Surface: web-spa cut to a stub; the cli surface stands alone
**Section:** the preamble; Surface: web-spa; Surface: cli (token names, primary screens, `viola list --json`, `--help`, the BAY line, board vs strip, the readback mirror, columns, streams, footer, IA notes and multi-surface coordination)
**Change:**
- Surface: web-spa (tooling context, wireframes, components and IA notes) is cut to the stub line «The frontend is deferred to 0.2.0; its technology is not chosen.».
- The preamble keeps only the security-tier note.
- The cli surface no longer compares itself with the web: no `viola ui` launch line, no `ui` help word, no "web ATIS", no "same as the web racks".
- `list --json`'s item, `budget` and `skipped` shapes are the `list` payload contract (architecture §Standard Contracts), no longer "identical to `/api/sessions`".
- Multi-surface coordination becomes «Shared vocabulary». Its one parity line reads that the six captions, words and slot order are fixed and a future page reuses them.
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) makes 0.1.0 a pure CLI and moves the frontend to 0.2.0, its technology not chosen. CLI-board parity is the 0.2.0 frontend's starting intent.
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — viola list's unwrapped rack, revive's fifth pair, verify at 23 rows
**Section:** Output structure — `viola list` (piped) As built; Component — Primary content block 1 As built; Output structure — `viola revive`; Component — Primary content block 2 (Line form); Output structure — `viola verify` (wireframe, Usage)
**Change:**
- `viola list` (piped): one `claude agents --json` read follows the board read; a joined wrapped row reads STATUS `idle` or `busy`; every unjoined row prints after the wrapped rows (or after the empty line) under `-- UNWRAPPED - READ-ONLY --`, sorted by name, NAME the row's `sessionId` through `escape_row`, LIVE `live`, its STATUS, `n/a` in WHEEL, DIALOG and CLI, never truncated (the wireframe's clipping is not built); NAME's width over both racks in characters; no separator without an unwrapped row; a failed, oversize or malformed read prints the board as with no `claude`. Block 1: the separator and unwrapped rows built; the unwrapped NAME is an upstream-derived, escaped cell beside the CLI version. Was: the rack not built, STATUS always `unknown`.
- `viola revive`: the pair `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`; the start reads the holder, the logged session, `session-live` (skipped under `--fork`; a failed read passes), then the recorded directory. Block 2: revive's own pairs are five (was four).
- `viola verify`: 23 relied rows on a Unix host, `[NN/23]`, `[23/23] agents-join claude agents --json names the live session by pid and session id  pass` last, `stamped 2.1.287  23 pass  0 fail`, the failing example `21 pass  2 fail`; the Usage's row list ends `agents-join`, `MM` 23 on Unix and 20 on Windows.
**Why:** The pair is the founder's own wording and the reading his answer to P4 forks 4 and 3 (`inputs#I1`, relayed verbatim by the operator); the card confirm provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary.
**Kept:** Line 284's design-target truncation of unwrapped NAMEs stays (owed to "CLI output discipline"); no unwrapped-name hint line (not built).
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-17-cli-machine-contract — layouts: viola revive takes --json; revive --list at the cli catch site
**Section:** §Output structure — `viola revive`; §Component — Primary navigation (global flags); §Component — Primary content block 2 (line form)
**Change:**
- `viola revive` usage carries `[--json]` on both forms (was: "The verb takes no `--json`"); under it each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr, `<code>` one of `strict-modes-failed` · `already-live` · `no-session` (both pairs) · `session-live` · `cwd-missing`; `--list --json` prints the rows as one document, or the `no-session` document; a wireframe shows both. Without `--json` pairs and rows are byte-identical. A post-preflight `run` start refusal prints no document (owed to "Exit-cause code catalogue").
- `--json` is the machine view of every data verb, `viola revive` included (was: "except `viola revive`, which takes none yet", owed to "CLI machine contract").
- Line form: revive's pairs are its document under `--json`; `revive --list` joins the `cli` verbs that print exactly `error: internal error` from the one catch site (a `revive` start stays outside it).
**Why:** the chunk landed revive's machine form and filed `revive --list` under the `cli` role.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/

## 2026-10-17-cli-output-discipline — viola --help built as the grouped table; viola list styled and cut on a terminal
**Section:** §Primary screens (commands); §Output structure — `viola --help`; §Component — Primary navigation (Discoverability); §Output structure — `viola list` (piped) As built; §Component — Primary content block 1 (As built)
**Change:**
- Primary screens: was "not built: the built help is clap's ordered list"; now the grouped verb table is built over the served verbs.
- `viola --help` As built: exactly `Usage: viola [--home <dir>] <verb> [args]`, a blank line, `board:    list` · `traffic:  send, wait, last, answer` · `wheel:    pause, release` · `setup:    run, revive, verify`, a blank line, `Options:` and clap's options; no about line, no `Commands:` list; `viola help` the same; pinned by the `HELP` literal; the usage line is the override literal on every OS, while `viola <verb> --help` stays clap's per-verb usage, naming `viola.exe` on Windows. The wireframe's `handoff:` row joins with "Explicit session links", `plugin install` with "Optional plugin install".
- Discoverability: the built groups are board / traffic / wheel / setup.
- `viola list` As built: was "uncoloured on every stream, a TTY included … owed to 'CLI output discipline'", "never truncated", "nothing is truncated"; now uncoloured under `--json`, a pipe, `NO_COLOR` (empty included), `TERM=dumb` or a failed VT enable, else styled (bold NAME, `DIALOG` amber with its kind plain, a stale row dim end to end with only `DIALOG` amber and not dim, depth from `COLORTERM` / `TERM`); with the width known (`viola_pty::host_size()`, never `COLUMNS`) an unwrapped NAME over its column (largest of 4, longest wrapped, longest unwrapped capped at width less 65) cut to the column less 3 plus `...`, never inside an escape; with no width nothing truncated.
- Block 1: was "no styling, no truncation"; now the spans and the unwrapped cut as above. The 2026-10-16-unwrapped-sessions-on-the-board entry's Kept (line 284's truncation owed to "CLI output discipline") is discharged.
**Why:** the chunk built both. Served verbs only and the width source are the Viola overseer's technical answers to P4 forks Q3 and Q2, a delegate's — PROVISIONAL, the founder's own word owed at his next batch.
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/

## 2026-10-17-mcp-server-for-drivers — viola mcp in --help and as the driver's machine view
**Section:** §Surface: cli → Output structure — `viola --help` (As built); Primary screens (commands)
**Change:**
- `--help` As built: `viola mcp` is a hidden verb (no argument but `--home`), run only by the plugin (`.mcp.json` registers the pinned path with `["mcp"]`); it appears in no row and the `HELP` literal is unchanged.
- Primary screens: `viola mcp` is no screen; its five tools are a driver's machine view of those verbs — each success and refusal the CLI verb's `--json` document less `v`, a refusal `isError: true` with text `refused: <code>`, bodies codes only (`invalid-params` with a closed field word), and no tool, description or result naming `release`, `pause`, `link` or `unlink`.
**Why:** the chunk built the hidden verb and its tools; `layout-templates.md`'s "no human surface" stays true.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
