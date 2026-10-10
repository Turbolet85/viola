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
