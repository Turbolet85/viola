# design-system — amendments

## 2026-09-27-browser-verdict-reachability — render and contrast judged on the ubuntu leg while the browser runs on three OSes
**Section:** §Typography (the per-OS fallback rationale; "Assertions hold on the Linux fallback") · §Surface: web-spa → Platform-Specific Notes (Fonts: "Linux is the CI render")
**Change:** the headless GUI checks' render and contrast verdicts are judged on the ubuntu leg (was "the headless GUI checks run on ubuntu"); the browser suite itself runs on all three CI OSes, where those assertions are not the verdict. Linux stays the CI render, with DejaVu Sans Condensed + DejaVu Sans Mono resolved.
**Why:** founder ruling W125 put the browser pipe on all three CI OSes (chunk 2026-09-27-browser-verdict-reachability); the font stacks and the DejaVu requirement are unchanged.
**Kept:** the "the CI render" labels in the font table and the token comment, and the 2026-09-24 Decisions Log line, stand as written.
**Ref:** .andromeda/runs/2026-09-27T19-50-23-wrap/

## 2026-09-28-cli-output-tokens — clap built without `color`
**Section:** §Surface: cli → Toolkit / Framework
**Change:**
- clap 4.6.7 (derive) is built without its `color` feature: its help and usage output is plain in every mode and no dependency reads a colour or terminal variable, so every styled byte of human output is viola's own SGR module's (was "clap 4.6.7 (derive), Rust stable").
**Why:** clap's `color` put bold/underline headers into `--help` and let `CLICOLOR_FORCE` force SGR into a pipe (research F1), against the cli colour ban; the chunk turned it off (report Dependencies; `cli_output_plain` green on 3 OSes).
**Ref:** .andromeda/runs/2026-09-28T09-46-16-wrap/

## 2026-09-28-capability-ledger-and-viola-verify — `viola verify`'s six-row step counter and its exit-1 phraseology
**Section:** §Surface: cli Component Patterns (`verify`); Exit-code phraseology (exit 1 row)
**Change:**
- `verify`'s step counter prints one static stdout line per ledger row, `[01/06] shim-resolution claude resolves to a real executable  pass` … (six rows today; the count grows as owning chunks land rows), then the last stdout line `stamped 2.1.283  6 pass  0 fail` (was the 14-row `[03/14] S3 …` / `stamped 2.1.280  14 pass` sketch).
- Exit 1 also carries verify's refusals `unable: the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version could not be read` / `a recorded payload still holds a path or a username`, each with its own hint; a failing row prints no stderr word.
**Why:** the chunk landed the ledger's six rows and the verb's refusals (report Symbols; `tests/cli_verify.rs`).
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/

## 2026-09-29-t15-07-57-wrap — registry migration (U35): the design-system Decisions Log leaves the body
**Section:** §Design Decisions Log · §Brand Identity · §Color Palette · §Surface: web-spa (Component Patterns 1 and 7; Navigation Pattern) · §Self-Validation Protocol (Squint Test)
**Change:** the log moved verbatim to design-system-amendments-archive.md (10 entries, all 2026-09-24). Each lift:
- §Brand Identity — the library shortlist direction (Precision & Density + Utility & Function; Minimalism & Swiss Style #1 without its hover, E-Ink / Paper #56 surface reference; rejected presets #7, #31, #51).
- §Color Palette — a second deviation note: holder colour marks kind, with the two contrast-forced state exceptions (`stale` fill, cocked inset DIALOG cell) and how kind stays readable without the fill.
- §Surface: web-spa component 1 — a grid `<tr>` still exposes `row`; the aria snapshot asserts `table` / `row` / `cell` in every state.
- §Surface: web-spa component 7 — scope: "view-only" / "never on this page" are v1 statements; the v1.x reservations do not contradict them.
- §Surface: web-spa Navigation Pattern — content jumping (UX guideline 5, #19) is met for order, not pixel position.
- §Self-Validation Protocol Squint Test — `open` and `unconfirmable` look identical by design; only the word cell tells them apart.
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/

## 2026-10-01-t12-19-55-wrap — web-spa toolkit: React + TypeScript, measured details open
**Section:** Iconography → Library; Surface: web-spa → Toolkit / Framework; Surface: web-spa → state attributes; Component Patterns → session strip Semantics; Component Patterns → event tape Expanded body; Platform-Specific Notes → CSP; Platform-Specific Notes → Constructable stylesheets; Anti-Patterns (the output-encoding NEVER)
**Change:**
- Toolkit was Lit 3.3.3, vendored ESM, no JS build step; now React + TypeScript, a built bundle embedded in `viola`. The bundler and its version, the React version, the embedding, the CSP the bundle needs and the npm gates are OPEN, owned by the route's frontend-toolchain entry (Epoch 8's head). Until it lands the page has no JS build step.
- The Lit-era host form (light-DOM custom elements via `createRenderRoot`, role-less `display: contents` hosts, Lit `static styles`) is retired; how each `viola-*` name maps onto React's output is OPEN, and the rendered-DOM requirements stand. Styling stays `/assets/app.css` alone: no Shadow DOM, no component-scoped or runtime-injected styles.
- Session strip: renders a native `<tr>` with `<td>` cells; any element left between `<tbody>` and the `<tr>` is role-less with `display: contents`; no role set by script.
- Text bindings are JSX text children (was Lit `${}`).
- CSP: `require-trusted-types-for 'script'` stands; "lit-html's built-in policy satisfies it" is retired, and what satisfies it under React is OPEN.
- Bans name React sinks: `dangerouslySetInnerHTML`, `innerHTML`, `style="…"` with React's `style` prop (were `unsafeHTML`, `unsafeSVG`, `styleMap`).
**Why:** founder ruling of 2026-09-30, relayed by the overseer: 0.2.0 renders the session hierarchy as a node graph, which a build-less Lit page does not fit. Standing rule: relaxing any CSP directive or a ban is a boundary widening the founder rules live at the frontend-toolchain entry.
**Kept:** every CSP directive and every ban (Lit API names swapped for their React sink counterparts, a narrowing); `viola-*` names and selectors as component names.
**Ref:** .andromeda/runs/2026-10-01T12-19-55-wrap/

## 2026-10-04-wait-and-last — cli pattern 3: session-end line and message mode
**Section:** Surface: cli → Component Patterns 3 (`viola wait` / `viola last`)
**Change:** The result lines gain `session-end` (every non-dialog kind takes the `turn-ended` form) and `dialog unknown` for an event without a `dialog_id`; message mode is spelled out: every control character but `\n` and `\t` prints as `\xHH`, two uppercase hex digits, never stripped; `--json` stays serde-escaped.
**Why:** CARRY 1's escaper landed with its live consumer, `last`; the `session-end` result line had no form.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/

## 2026-10-05-real-cli-verify-probes — the verify step counter at ten rows and its help paragraph
**Section:** §Surface: cli → pattern 5 `verify` · the exit-code table (exit 1 row)
**Change:**
- `verify`: was `[01/06]`, "six rows today", `stamped 2.1.283  6 pass  0 fail`; now `[01/10]`, ten rows today, `stamped 2.1.288  10 pass  0 fail`. `viola verify --help` carries one static ASCII paragraph naming no path: a trusted folder, and the external-import blocker.
- Exit 1: the fixed `a recorded payload still holds a path or a username` became the named `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; closed codes; never the content).
**Why:** the chunk landed four ledger rows and the help paragraph; the named refusal is the founder's live ruling at this wrap.
**Ref:** .andromeda/runs/2026-10-05T10-37-44-wrap/

## 2026-10-05-dialog-rows-and-re-probe — verify's step counter at fourteen rows
**Section:** §Surface: cli Component Patterns (`verify`)
**Change:** the counter reads `[01/14]` (was `[01/10]`), the summary `stamped 2.1.288  14 pass  0 fail`; the four dialog rows named with their words (`question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency`).
**Why:** the chunk landed the dialog rows; the pattern (static appended lines, uncoloured `fail`) is unchanged.
**Ref:** .andromeda/runs/2026-10-05T14-27-34-wrap/
## 2026-10-06-local-command-and-paste-framing-rows — the verify step counter at seventeen rows
**Section:** §Surface: cli → Component Patterns → 5 (the `verify` bullet)
**Change:** the example step line reads `[01/17]` (was `[01/14]`); "seventeen rows today, the last seven the dialog rows and the framing rows" (was fourteen, the last four); the three new rows are named with their words: `long-paste-wrapper` a long paste unwraps to the text as pasted · `tag-escaping` tag-like text un-escapes to the text as pasted · `local-command-clear` /clear starts a new session and submits no prompt; the summary example reads `stamped 2.1.287  17 pass  0 fail` (was `stamped 2.1.288  14 pass  0 fail`).
**Why:** the chunk landed three ledger rows, each one static ASCII step line; the example version follows the one stamped set, 2.1.287.
**Ref:** .andromeda/runs/2026-10-06T21-43-53-wrap/
## 2026-10-06-local-command-send-outcomes — the readback fills on a post-condition too
**Section:** §Brand Identity (Domain anchors → Readback; Signature element → `RB` / `read back`) · the rejected-defaults bullet on the optimistic "Sending…" spinner
**Change:** three sentences named the matching `prompt-submitted` as the only thing that fills the box; each now names `/clear`'s measured post-condition beside it (a new session: a `session-start` with cause `clear`). The `Readback::read` row and the web component's state sources already carried the trigger; the CLI component pattern states none. No new word: a post-condition-confirmed `/clear` prints `[RB] read back`, and the unconfirmable line uses the existing word and the open box.
**Why:** the chunk made `send` confirm `/clear` by its post-condition, so the prompt is no longer the sole confirmer.
**Ref:** .andromeda/runs/2026-10-06T23-49-33-wrap/
## 2026-10-08-first-live-test-and-self-drive — the `input-not-ready` hint text
**Section:** §Surface: cli → Component Patterns → 2 (`viola send`), the `not-delivered · input-not-ready` bullet
**Change:** the hint reads `<name> was not ready for input; send again, and if it repeats a human must look at the session` (was: `<name> was not ready for input; viola wait <name>, then send again`). No other hint, exit code or refusal moved.
**Why:** the founder's wording, chosen among three shown at the chunk's planning: in none of the refusal's causes is a turn running, so a `viola wait` woke on nothing.
**Ref:** .andromeda/runs/2026-10-08T09-10-03-wrap/
## 2026-10-09-epoch-3-cleanup — the `empty-text` row, hint and exit-13 word
**Section:** §Color Palette › Domain status colors; §Surface: cli › Component Patterns 2 (the hint list); §Surface: cli › Exit-code phraseology
**Change:** a row `RefusalDetail::empty-text` with the three tokens of its `control-character` neighbour, shown as `not-delivered · empty-text`. Its hint: `the text is empty once its trailing newlines are removed; send a text with content`. The exit-13 row's detail list ends `control-character` / `empty-text`.
**Why:** the chunk added the refusal; the hint's wording is the founder's, relayed. No detector covers this vocabulary, so the plan's own list raised it.
**Ref:** .andromeda/runs/2026-10-09T17-10-00-wrap/

## 2026-10-10-viola-revive — the `revive` refusals and hints, its `--list` rows, the verb in the `setup` help group
**Section:** §Brand Identity (the expression table, the cli 0.0 row) · §Surface: cli → Component Patterns (the stderr rule; pattern 2, hints; pattern 5, a `revive` bullet; Exit-code phraseology, the exit 1 row) · §Surface: cli → Navigation Pattern · §Per-Surface Bans (the no-colour ban)
**Change:**
- Pattern 2: the exit-1 refusals of `viola revive`, one fixed-message line and one hint per cause, read in a fixed order that stops at the first refusal: `<name>'s state files can be written by another user` → `viola will not read them; make the viola home and its files owner-only`; `run`'s `already live` pair and its stale sibling, unchanged; `<name> has no logged session to resume` → `start one: viola run <name> -- claude`; `<name> has no logged session with that id` → `viola revive <name> --list`; `<name>'s recorded directory is missing` → `resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`. None shows the recorded directory, a pid or a logged id; no hint names `viola release`.
- The stderr rule: the fixed message form `unable: <text>` is used by the exit-1 refusals of `viola run`, `viola revive` and `viola verify` (was "only the exit-1 start refusal"). The exit 1 phraseology row lists revive's four lines.
- Pattern 5, `revive`: prints nothing once the child starts; `--list` prints one static stdout line per logged session, `<time>  <cause>  <session id>`, two spaces between fields, no header, no colour, plain ASCII; no `--json`.
- The detail-code sentence names `no-session` and `cwd-missing`; revive's strict-modes refusal reuses `strict-modes-failed` at exit 1.
- Navigation Pattern: the flat verb list names `revive`; `viola --help` groups it as `setup: run, revive, verify, plugin install`.
- The 0.0 expression row and the no-colour ban name a passed `viola revive` as the same passthrough as `viola run`.
**Why:** the chunk landed the verb. The four pairs stand as the plan worded them on the founder's answer, live, 2026-10-10, relayed by the operator. The help group is the operator's answer at this wrap's Phase 2 halt: the binary's own help lists `revive` right after `run`.
**Ref:** .andromeda/runs/2026-10-10T15-07-22-wrap/

## 2026-10-10-budget-governor — the release --budget lines as planning forms
**Section:** §Surface: cli → Component Patterns 4 (Wheel, link and gate verbs) · Component Patterns 2 (the `--json` detail-code note)
**Change:** `viola release <name> --budget` prints `<name>  budget gate lifted until <end>  wheel <holder>` when a pause stood, `<end>` from the reply's `override_until` as `HH:MM:SSZ` on the current UTC date and `YYYY-MM-DD HH:MM:SSZ` on any other, a value that is no RFC 3339 instant escaped; and `<name>  budget gate open  wheel <holder>` when no pause stood. The date form and the `budget gate open` line are marked planning forms, the founder's wording owed. The detail-code note: `viola revive`'s strict-modes refusal carries no detail code; was "reuses `strict-modes-failed` at exit 1".
**Why:** the chunk built two forms no master worded. They landed on the operator's word; the phraseology is the founder's, and he has not worded them.
**Kept:** the `budget-paused` hint stands as written; it landed as the design's string.
**Ref:** .andromeda/runs/2026-10-10T22-36-31-wrap/

## 2026-10-11-the-board-viola-list — the board as built, its five planning forms and its exit-21 refusal
**Section:** §Surface: cli (Stream rules; Component Patterns 1 `viola list`; the exit-code phraseology table, row 21)
**Change:**
- Pattern 1 gains an As built note: wrapped rows only, uncoloured, STATUS always `unknown`, no unwrapped separator or row (owed to "Unwrapped sessions on the board"), no SGR (owed to "CLI output discipline"); the row mode escapes every control character, DEL and C1 included, as `\xHH` (`escape_row`); WHEEL padded to 23 (`unknown - budget-paused`).
- Five planning forms, not worded by the founder: a figure rounded down (`89.6` → `89 %`); the age in whole minutes, rounded down, and `read unknown` for no, an unparseable or a future instant; bare nonzero skip counts in key order; `human - budget-paused`; the empty board's BAY line and caption row above its line.
- `list`'s refusal (exit 21): the home → `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions` (`list` its first writer in source); an instance → `revive`'s state-files pair. The stream rule's fixed-message form gains `list`'s exit-21 refusal (was: only the exit-1 refusals of `run`, `revive`, `verify`); the exit-21 row names the pair in human mode and the `instance-unreachable` document under `--json`.
**Why:** the board as built. The five forms and the whole-board refusal stand on the operator's word (planning), not ruled by the founder, whose wording is owed. WHEEL 23 is implement's decision, accepted by the operator at this wrap.
**Ref:** .andromeda/runs/2026-10-11T08-23-35-wrap/

## 2026-10-13-t06-38-35-wrap — the board's forms, WHEEL 23, the list refusal and the two budget-governor lines are the founder's
**Section:** §Surface: cli (Tokens (platform-specific): stderr stream rule; Component Patterns 1 `viola list` As built and Refusal; Component Patterns 4 `viola release --budget`; the exit-code phraseology table, row 21)
**Change:**
- WHEEL padded to 23 (the widest word `unknown - budget-paused`): implement's decision, now confirmed by the founder; was accepted by the operator.
- The five board forms (a figure rounded down, the whole-minute age and `read unknown`, nonzero skip counts as bare numbers, `human - budget-paused`, the empty board's three lines) are his wording; was "planning forms on the operator's word, not worded by the founder (his wording is owed)".
- `viola list`'s exit-21 whole-board refusal and its fixed `unable:` / `hint:` pairs (the home pair `the viola home is not private to you` included) are his, in the stream rule, the Refusal pattern and row 21; was the operator's answer, not ruled.
- The `release --budget` date form and the `budget gate open` line are his wording; was "the founder has not worded them, and his wording is owed".
Each superseded mark stands in the body as the operator's word, superseded 2026-10-13.
**Why:** the founder's own answer of 2026-10-13 («Подтвердить всё»: the board's shapes, WHEEL 23, the budget governor's item 4), relayed verbatim by the Viola overseer through the operator's directive; the operator confirmed at this wrap that the two lines were shown to him as built, so they are his wording.
**Ref:** .andromeda/runs/2026-10-13T06-38-35-wrap/

## 2026-10-13-interrupted-verify-cleanup — verify's interrupted pair
**Section:** §Surface: cli → Component Patterns 5 (`verify`) · Exit-code phraseology (row 1)
**Change:** A `viola verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary. Its stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI` (pattern 2's fixed form, plain ASCII, no path or pid), stdout is empty, and it exits 1. Exit-code row 1 lists the pair beside verify's refusals.
**Why:** The operator's P4 answer chose exit 1 with a fixed pair over a new code. The wording is a planning form on the operator's word, and the founder's wording is owed in his next batch.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/

## 2026-10-13-paste-newline-ledger-row — `verify` step lines at eighteen rows
**Section:** §Surface: cli, Component Patterns 5 (`verify`)
**Change:** Was `[01/17]` and seventeen rows; now `[01/18]` and eighteen rows, the members of `LedgerRow::ALL`. The last eight are the dialog rows, the framing rows and the newline row `paste-newline` (an LF inside a paste stays inside the one prompt submitted). The summary reads `stamped 2.1.287  18 pass  0 fail`.
**Why:** The capability ledger gained its eighteenth row (`paste-newline`), and the step counter derives from `LedgerRow::ALL.len()`. The count now carries its rule.
**Ref:** .andromeda/runs/2026-10-13T21-52-39-wrap/

## 2026-10-13-owed-ledger-rows — verify's step counter at the relied count
**Section:** §Surface: cli → Component Patterns, pattern 5 (`verify`)
**Change:**
- Was one step line per ledger row, eighteen today, the summary `stamped 2.1.287  18 pass  0 fail`.
- Now one step line per relied ledger row: `MM` is `LedgerRow::relied()`'s count, 21 on Unix and 18 on Windows, where the status line is not wrapped.
- The rows named after `paste-newline` are `statusline-override` · `statusline-shell` · `statusline-rate-limits`, each with its words.
- The summary's n + m equals `MM`: `stamped 2.1.287  21 pass  0 fail` on Unix.
**Why:** the three statusline rows landed relied only where the status line is wrapped (`statusline::WRAPPED_HERE`), the cut the founder chose (`inputs#I1`). The count carries its rule (`relied()`).
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/

## 2026-10-14-resume-and-compact-ledger-rows — the verify step list gains resume-session
**Section:** §Surface: cli, pattern 5 (`verify`)
**Change:**
- The step counter reads `[01/22]`. MM is 22 on Unix and 19 on Windows.
- The ledger order ends with the resume row, `resume-session --resume <id> reopens the session under that id`, relied on every OS.
- The summary reads `stamped 2.1.287  22 pass  0 fail`.
- Pattern 2's `[  ] unconfirmable … local command, no measured post-condition` covers `/compact` unchanged.
**Why:** The 22nd ledger row landed (`resume-session`), relied on every OS.
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/

## 2026-10-16T14-58-10-wrap — Surface: web-spa, web motion and web bans cut; the shared sections stand as 0.2.0 inputs
**Section:** Surface: web-spa; Motion (easing, values, moments, limits); Anti-Patterns (Universal Bans, Rejected Defaults, Per-Surface Bans web-spa); Surface: cli (component 5, help groups, streams, the launch-line ban); and a note line on Surface Scale, Text Hierarchy, Semantic Colors, the domain-status table, Border Progression, Typography, Spacing, Depth Strategy, Border Radius, Iconography and Self-Validation's Token Test
**Change:**
- Surface: web-spa (tokens CSS, components, navigation, platform notes) is cut to the stub line «The frontend is deferred to 0.2.0; its technology is not chosen.».
- The web motion values, moments, limits and easing are cut, and so are the web-only universal bans, two web rejected defaults (node-graph canvas, chat-transcript feed) and the web-spa per-surface bans.
- The cli surface no longer names `viola ui`, its launch line, the web ATIS or the `127.0.0.1:47319` address.
- Each kept section that describes only the page opens with «A 0.2.0 frontend design input; 0.1.0 renders no page.».
**Why:** The founder's ruling of 2026-10-16 (his own words, relayed verbatim by the Viola overseer) named Surface: web-spa, the web motion and the web bans, and kept Brand Identity and the concept.
**Kept:**
- Brand Identity (L3–L51) untouched, by the ruling. Its L41 / L45 still cite the cut Motion values and its L16 / L21 the cut web-spa components; this is named for the founder.
- The palette, type, spacing, depth, radius, token, iconography and domain-status sections, as 0.2.0 frontend design inputs. The operator's answer at this wrap (overseer, technical, founder-delegated): escher, the 0.2.0 candidate, renders HTML/CSS too.
- The Typography browser-render assertions stand, marked as owned by the route entry "Remove the browser leg".
**Ref:** .andromeda/runs/2026-10-16T14-58-10-wrap/

## 2026-10-16-remove-the-browser-leg — Typography: the CI render and the DejaVu provisioning leave with the browser leg
**Section:** Typography (the Rationale's «Both faces are Windows-only»; the per-OS fallback table's Linux column; «Assertions hold on the Linux fallback»; the stand-until note)
**Change:**
- Was: the headless GUI checks' render and contrast verdicts were judged on the ubuntu CI leg, the Linux stack was "what CI renders" (the table's «the CI render»), the CI image had to provide `fonts-dejavu-core` / `fonts-dejavu-extra`, and a note held this text until the route entry "Remove the browser leg" (the Kept clause of «2026-10-16T14-58-10-wrap — Surface: web-spa, web motion and web bans cut; the shared sections stand as 0.2.0 inputs»). Now 0.1.0 renders no page, so no CI step renders these stacks or provides their families, and the note is gone.
- The contrast and render assertions stand as a 0.2.0 design input: they hold with "DejaVu Sans Condensed" + "DejaVu Sans Mono" resolved, on the resolved family, and a resolved face outside the stacks breaks them. How a 0.2.0 frontend checks them is that version's to decide, its technology not chosen.
- The table's Linux column reads «Linux (CI-tested)».
**Why:** This chunk removed the browser test leg, on the founder's ruling of 2026-10-16 that 0.1.0 is a pure CLI (item 3). No DejaVu provisioning site existed at the base.
**Kept:** The per-OS stacks (`--font-label`, `--font-field`), the fallback table and the column sizing that takes DejaVu Sans Mono as the widest case, all 0.2.0 design inputs.
**Ref:** .andromeda/runs/2026-10-16T17-15-05-wrap/

## 2026-10-16-unwrapped-sessions-on-the-board — cli patterns 1 and 2: the unwrapped rack built, the session-live pair
**Section:** §Surface: cli pattern 1 (`viola list` As built), pattern 2 (`viola revive` refusals), the exit-code table row `1`, the `verify` step counter
**Change:**
- Pattern 1 As built: a wrapped row joined to a `claude agents --json` row reads STATUS `idle` or `busy`, else `unknown`; when the read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line follows the wrapped rows (or the empty-board line), then one row per unwrapped session sorted by name — NAME its `sessionId`, LIVE `live`, its STATUS, `n/a` in WHEEL, DIALOG and CLI, never truncated (the sample's clipping is not built); no separator without an unwrapped row; a failed, oversize or malformed read prints the board as with no `claude`. The upstream-derived cells are the CLI version and each unwrapped NAME, NAME's width in characters. Was: STATUS always `unknown`, no separator, no unwrapped row, both owed.
- Pattern 2: `viola revive` gains `unable: <name>'s session is open in another claude` → `hint: close it there first, or start a copy: viola revive <name> --fork`, read after the logged session and before the recorded directory (skipped under `--fork`; a failed read passes); the exit-code table's row `1` names it.
- `verify`: `[01/23]`, `MM` 23 on Unix and 20 on Windows, `agents-join` last in the member list, `stamped 2.1.287  23 pass  0 fail`.
**Why:** The pair is the founder's own wording, his answer to P4 fork 4, and the reading his answer to fork 3 (`inputs#I1`, relayed verbatim by the operator); the card confirm provisional (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. NAME = `sessionId`, the empty-rack form and no truncation or colour are the operator's yes at P5 (`inputs#I2`).
**Kept:** The colour layer, the styled TTY form and width-known truncation stay owed to "CLI output discipline"; the unwrapped-name `instance-unreachable` hint is not built (owed to "Exit-cause code catalogue").
**Ref:** .andromeda/runs/2026-10-16T22-30-02-wrap/

## 2026-10-17-cli-machine-contract — design: revive's --json documents in the cli surface
**Section:** §Surface: cli → Output structure per verb (`revive`); Exit-code phraseology (exit 1's `--json` cell); the hints-are-human-mode-only bullet (exit 1's document, revive's strict-modes refusal)
**Change:**
- `revive`: under `--json` (both arms) a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`), no hint, stderr empty; `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}` (was: "The verb takes no `--json`"). A post-preflight `run` start refusal prints no document (owed to "Exit-cause code catalogue").
- Exit 1's `--json` cell names revive's document; every other exit 1 has none (was: "no `--json` document until arch amends it"). The hints bullet: arch fixes exit 1's document for revive only; revive's strict-modes refusal is the stderr pair or, under `--json`, the `strict-modes-failed` document.
**Why:** the chunk landed revive's machine form; the plan's expected amendment, raised by the wrap where no detector reached it.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/

## 2026-10-17-cli-output-discipline — the colour layer and the width cut built; --help groups the served verbs
**Section:** §Component Patterns pattern 1 (As built); §Surface: cli Width; §Navigation Pattern; §Tokens (platform-specific)
**Change:**
- Pattern 1 As built: was "uncoloured … owed to 'CLI output discipline'" and unwrapped NAMEs "never truncated"; now on a terminal that takes colour every NAME bold, the `DIALOG` word of a wrapped row with a pending dialog in the attention token with its kind word plain, a wrapped `stale` row dim end to end (NAME dim and bold; with a dialog only `DIALOG` amber and not dim), the BAY line, caption, empty-board line and separator plain, spans over the escaped padded cells, one write; on a terminal whose width is known NAME's column is the largest of 4, the longest wrapped NAME and the longest unwrapped NAME capped at the width less 65, an unwrapped NAME longer than it cut to the column less 3 plus `...`, never splitting an escape; a wrapped NAME never cut; pipe and `--json` whole.
- Width: was `COLUMNS` off Windows; now stdout's own size by `viola_pty::host_size()` (`TIOCGWINSZ` / `GetConsoleScreenBufferInfo`), `COLUMNS` not read, a piped stdout no width.
- Navigation: as built `--help` lists the served verbs only — board / traffic / wheel / setup, then `Options:`; `handoff:` joins with "Explicit session links", `plugin install` with "Optional plugin install"; the five-group target row kept.
- Tokens: rung 4 reads `NO_COLOR` present, an empty value included; on a stale row NAME is dim and bold, every span opening dim again after its reset.
**Why:** the chunk built the layer. The `NO_COLOR` read is inside the founder's own answer «Читать их» to P4 fork Q1 (relayed verbatim by the operator; card confirm a delegate's, the Viola overseer, PROVISIONAL). The width source and served-verbs-only are the Viola overseer's technical answers to Q2 and Q3, a delegate's — PROVISIONAL, the founder's own word owed at his next batch.
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/

## 2026-10-17-mcp-server-for-drivers — viola mcp, a hidden verb
**Section:** §Surface: cli → Platform-Specific Notes
**Change:** the `mcp` note adds that `mcp` is a hidden verb: run by the plugin, never by a person, it appears in no `viola --help` row.
**Why:** the chunk built `viola mcp` with `#[command(hide = true)]`; `--help` stays the served verbs' table.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/

## 2026-10-17-optional-plugin-install — plugin install served in setup:, its lines and its exit-21 pair
**Section:** §Surface: cli (Navigation Pattern; Component Patterns 4; Exit-code phraseology)
**Change:**
- Navigation Pattern: was "`setup: run, revive, verify`, … and `plugin install` when "Optional plugin install" serves it"; now the as-built `setup:` row is `run, revive, verify, plugin install`, only the `handoff:` row still waiting on "Explicit session links".
- Component Patterns 4: `viola plugin install` → four stdout lines in one write (`viola@viola  written  <dir>` / `run these to add it to Claude Code:` / `  claude plugin marketplace add <dir>` / `  claude plugin install viola@viola`), `<dir>` quoted only where the host's shell would split it; plain ASCII, no SGR; a planning form, the founder's wording owed.
- Exit-code phraseology: its exit-21 refusal prints the home-state pair as `viola list`'s does, or the `instance-unreachable` document under `--json`.
**Why:** the chunk served the verb. Served verbs only stays the Viola overseer's PROVISIONAL answer of an earlier chunk.
**Ref:** .andromeda/runs/2026-10-18T02-36-56-wrap/

## 2026-10-18-compact-post-condition-ledger-row — /compact read back by its same-session post-condition; twenty-four verify rows
**Section:** Brand Identity (the Readback anchor; the Signature element's `RB` state); Color Palette → Semantic Colors (`SessionStartCause::compact`); Anti-Patterns → Rejected Defaults (the spinner bullet); §Surface: cli → Component Patterns (`verify`)
**Change:** a `/compact` send on a verified CLI is read back once its measured post-condition, a `session-start` of cause `compact` carrying the session's own id, lands — beside `/clear`'s new session; on an unverified CLI it stays `unconfirmable`; the `SessionStartCause::compact` row names that readback role. `verify`'s step lines read `[NN/24]`, `MM` 24 on Unix and 21 on Windows, the row list ending `local-command-compact /compact keeps the session id and submits no prompt`, the summary `stamped 2.1.287  24 pass  0 fail`. Was: `/clear` the only post-condition read back; `[01/23]`, 23 / 20, `23 pass  0 fail`.
**Why:** the chunk built the compact post-condition and the 24th row (report Symbols / APIs, Retired); the unconfirmable line stays for `/remote-control` and an unverified CLI.
**Ref:** .andromeda/runs/2026-10-18T21-59-05-wrap/
