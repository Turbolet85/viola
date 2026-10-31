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

## 2026-10-19-dialog-hold-for-the-driver — `viola wait`'s `dialog-expired` line takes the dialog form
**Section:** §Surface: cli §Component Patterns 3 (`viola wait` / `viola last`).
**Change:**
- A new example line: `dialog-expired  builder  dialog 7  cursor 49400`. An expired hold takes the dialog form, as `question`, `permission` and `plan` do.
- The time form is now for "every other kind without a `dialog_id`". Was "every other non-dialog kind", which would have put `dialog-expired` in the time form.
**Why:** The wrapper's new record of an expired hold carries `dialog_id`, so its `wait` line names the dialog. The line and the kind word are a planning form: the founder's wording is owed to his Epoch 5 boundary batch (the P5 card's row 3, approved by the operator's word, `inputs#I3`). The Domain status colors table gains no row: a colour for the kind is a 0.2.0 frontend decision this chunk did not make.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/

## 2026-10-20-t06-58-31-wrap — the founder's Epoch 5 batch: the width source, served verbs only and three wordings his
**Section:** §Surface: cli (Tokens; Component Patterns 3, 4 and `verify`; Navigation Pattern)
**Change:**
- The width from `viola_pty::host_size()` and `viola --help` listing the served verbs only stand as built; was "the Viola overseer's technical answer, `inputs#I1` Q2 / Q3 of that chunk, a delegate's — PROVISIONAL, the founder's own word owed"; now the parentheticals are gone.
- `wait`'s `dialog-expired  builder  dialog 7  cursor 49400` line, `viola plugin install`'s four lines and verify's interrupted `unable:` / `hint:` pair stand as worded; was "a planning form; the kind word and the line are the founder's to word", "A planning form as built, the founder's wording owed" and "a planning form on the operator's word …; the founder's wording is owed"; now the marks are gone.
- In this file's history the provisional marks of these entries are superseded 2026-10-20, their other claims standing, none pruned: 2026-10-13-interrupted-verify-cleanup — verify's interrupted pair; 2026-10-16T14-58-10-wrap — Surface: web-spa, web motion and web bans cut; the shared sections stand as 0.2.0 inputs; 2026-10-16-unwrapped-sessions-on-the-board — cli patterns 1 and 2: the unwrapped rack built, the session-live pair; 2026-10-17-cli-output-discipline — the colour layer and the width cut built; --help groups the served verbs; 2026-10-17-optional-plugin-install — plugin install served in setup:, its lines and its exit-21 pair; 2026-10-19-dialog-hold-for-the-driver — `viola wait`'s `dialog-expired` line takes the dialog form.
**Why:** the founder's own answers to his Epoch 5 batch, 2026-10-20 (ratify every delegate mark of the epoch; keep `dialog-expired`; accept the plugin install text as built), relayed by the Viola overseer through the operator's directive, and his own word widening it to the interrupted pair's wording. The bodies take writing rule 7's form, by the overseer's technical answer at this wrap.
**Ref:** .andromeda/runs/2026-10-20T06-58-31-wrap/

## 2026-10-20-epoch-5-cleanup — the web-surface references cut from Brand Identity, the palette, Typography, Motion and the Signature Test
**Section:** Brand Identity (ATIS anchor; Signature element; Supporting attention element; the per-surface expression table and the single-source paragraph; Design direction) · Color Palette → Semantic Colors (DialogPending::true, TapeConnection::closed, Problem::unauthorized) · Typography (Display) · Motion (heading) · Self-Validation Protocol → 3. Signature Test
**Change:**
- Brand Identity: the ATIS anchor no longer names the `<viola-atis>` header. The readback box's one 0.1.0 place is its CLI mirror; where a 0.2.0 frontend draws it is that version's to name (was: every send line in `<viola-event-feed>` and every outbound transfer marker). The cocked strip keeps its slot and shows `DIALOG` plus the kind, amber on the CLI; its 12px / 160ms translate and 4px holder band are a 0.2.0 frontend design input (was a standing motion). The per-surface table holds the two cli rows, and the frontend's expression level is an `N/A` deferred to 0.2.0. Was a `web-spa (viola ui)` 0.3 row with two motions; «viola's web page is 0.3 …» is cut from the single-source paragraph. The Design direction is what a 0.2.0 frontend inherits (was «The page is …»).
- Semantic Colors: the pointers to the cut web-spa components 1 and 6 are gone. TapeConnection::closed reads `TAPE stopped · not answering` (was `… viola ui not answering`).
- Typography Display: `VIOLA` is the board's display word; a 0.2.0 frontend's page markup is that version's (was the `<viola-atis>` header and the page's one `<h1>`).
- Motion: calibrated to the base expression level `0.2` (was `0.3` on web-spa).
- Signature Test: one 0.1.0 place, the `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send` (was three places with `<viola-event-feed>`, `<viola-transfer>`, `<viola-readback>`, the `.rb` rule and web-spa component 3).
**Why:** the founder's own word on item (10), «Вычистить в cleanup»: the restructure's web leftovers are cleaned, and 0.1.0 renders no page. The superseded text stays in this entry for the 0.2.0 frontend.
**Kept:** `## Surface: web-spa` keeps its heading over its deferral stub. It is the setup grammar's surface slot for the deferred frontend, shared with layout-templates. The sections framed «A 0.2.0 frontend design input» (tokens, status rows, spacing, depth, iconography) stand as that input.
**Ref:** .andromeda/runs/2026-10-20T09-40-55-wrap/

## 2026-10-25-u55-door — design-system: split into its body and 9 category files
**Section:** (the whole master — the door)
**Change:** split into its body and 9 category files; the map .andromeda/design-system.map.tsv
**Why:** the operator's word: the founder's standing word of 2026-10-25 for the door on viola's masters (F2), the review closed on spec 3m at 1.37× its cost guide, the third pass skipped as it would cross the stop; relayed by overseer1.
**Ref:** $TMPDIR/claude-1000/-home-turbolet-dev-projects-additional-andromeda-worker/a2a855dc-9f94-4016-87d5-2f52c0246ab5/scratchpad/design-system/

## 2026-10-26-server-verification-before-any-frame — `server-verify-failed` written by the six frame verbs
**Section:** Surface: cli › Component Patterns (surface.md: the cli-send hints group, its pending-arch-amendment bullet on the D-20 log detail codes)
**Change:**
- After the D-20 code list the bullet now says that, of those codes, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause` and `release` at their exit-21 server-verification refusal, and that the refusal's `--json` document stays `detail:null`, as the not-running cause's does (was: the code named in the list with no writer). The code list itself, the arch-shape rule and the stale-heartbeat and unwrapped-name causes having no code are unchanged.
**Why:** this build is the first that writes `server-verify-failed`, as `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` into the CLI log; the code was already a member of the diag-line `detail` enum, and no `--json` code, exit code or refusal reason was added.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-verification hint as built, and its macOS gap
**Section:** Surface: cli › Component Patterns (surface.md: two new cli-send groups)
**Change:**
- New: the server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause` and `release`, the six verbs that write a frame, as the hint cause `server-verify` beside `not-running`; it prints the hints group's server-verification line byte for byte, plain ASCII with no SGR, and holds no pid, start time, path or endpoint.
- New: on macOS no peer pid is available, so the server check passes with no comparison and the hint cannot print there, a recorded gap owned by the route entry "Unix endpoint and home hardening".
**Why:** the hint function's new `server-verify` cause prints exactly the hint row's string, and the tests assert it plain ASCII with no SGR and no pid, path or endpoint in any output; the server verdict returns a match with no comparison when no peer pid is reported, which is the case on macOS, and macOS stays on the recorded gap, owned by "Unix endpoint and home hardening".
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — exit 21's server-verification cause as built
**Section:** Surface: cli › Component Patterns (surface.md: a new cli-exit-codes group)
**Change:**
- New: exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause` and `release`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.
**Why:** a mismatch or a check that cannot be made is exit 21 with the unchanged document, measured on Linux and Windows by `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (six verbs, two modes) beside its control `security_negative_cli_verb_reaches_the_recorded_wrapper`; no exit code or `--json` code was added.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-explicit-session-links — `link` / `unlink`'s exit-21 strict-modes refusal takes the fixed instance-state pair
**Section:** Surface: cli › Tokens (platform-specific) and Component Patterns (surface.md: the cli-output stderr bullet, the cli-list Refusal bullet, the cli-exit-codes table's row 21); design-system.md: the Streams stderr line, the Exit-code phraseology
**Change:**
- surface.md stderr: the fixed `unable: <text>` form is used by the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance (was: without the two verbs).
- Refusal: the instance pair `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` stays `revive`'s, shared; `viola link` / `viola unlink` print it, naming the driven instance, at their exit-21 strict-modes refusal.
- Row 21: `viola list`'s refused strict-modes check, and `viola link` / `viola unlink`'s of the driven instance, print the fixed home-state or instance-state pair in place of `unable  instance-unreachable` and a hint; the `--json` cell reads "the `list`, `link` and `unlink` refusals included" (was "`list`'s refusal included"), stderr empty.
- Streams: the fixed form names "the exit-21 strict-modes refusals of `list`, `link` and `unlink`" (was "`list`'s exit-21 refusal").
- Exit-code phraseology, new: `viola link` / `viola unlink`'s refused strict-modes check of the driven instance (exit 21) prints the instance-state pair naming that instance, as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty.
**Why:** the verbs check the home and the driven instance before any log or snapshot read and print the instance-state pair on a refusal. The form is the founder's own answer to this chunk's P4 fork (inputs#I3, relayed verbatim by the operator); exit 21, as `list` and `plugin install`, not `revive`'s exit 1, is his option's text. Row 21's extension of the closed fixed-form list lands beside the stderr rule it extends, on the overseer's founder-delegated answer at the halt (inputs#I6), a delegate's.
**Kept:** `viola plugin install`'s exit-21 home-state pair is still absent from the fixed-form lists of the stderr bullet and the Streams line: a standing gap, not this chunk's.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the server-verification cause and hint cover eight frame verbs
**Section:** Surface: cli › Component Patterns (surface.md: the cli-send hints bullet on the D-20 log detail codes, the cli-send server-verification hint group, the cli-exit-codes server-verification group)
**Change:**
- `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal (was the six, per "2026-10-26-server-verification-before-any-frame — `server-verify-failed` written by the six frame verbs"); the `--json` document stays `detail:null`.
- The server-verification hint of `instance-unreachable` is built for the eight verbs that write a frame (was "the six verbs", per "2026-10-26-server-verification-before-any-frame — the server-verification hint as built, and its macOS gap"); its cause `server-verify`, its byte-for-byte line, plain ASCII with no SGR and no pid, start time, path or endpoint unchanged.
- Exit 21's server-verification cause is built for the same eight (was six, per "2026-10-26-server-verification-before-any-frame — exit 21's server-verification cause as built"); its stderr line and `--json` document unchanged.
**Why:** `link` / `unlink` write their frame after the liveness pre-check through the same server identity check; a server that is not the recorded wrapper gets no frame, exit 21 with the server-verify hint, and `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}`. The CLI frame verbs are now eight.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `link` / `unlink`'s result lines as built
**Section:** Surface: cli › Component Patterns (surface.md: a new cli-verbs group); design-system.md: pattern 3's message-mode item
**Change:**
- New: `viola link` / `viola unlink` print their one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0; the names printed are the verb's own parsed arguments, never names read back from the reply or the log.
- New: the `link` line's `since` is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through pattern 3's message-mode escaper.
- New: a second `viola link` of a held pair prints the `already linked` form, the wrapper keeping the first `since`; `viola unlink` of a pair not held prints the same `unlink  <driver> -> <driven>` line; both exit 0.
- Pattern 3: message mode is the escaper `wait` and `last` share and `viola link` uses for a `since` that is no RFC 3339 instant (was "the escaper both verbs share"); its rule unchanged.
**Why:** the chunk built the three result lines, each one write, plain in every mode, from the parsed arguments; a repeated `link` answers the same `since` with `already_linked: true`, and `unlink` answers alike whether or not a pair was removed.
**Kept:** pattern 4's `link` / `already linked` / `unlink` lines stand as worded: they are the built lines.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — `link` / `unlink`'s `--json` documents and their exit 20
**Section:** Surface: cli › Component Patterns (surface.md: a new cli-exit-codes group)
**Change:**
- New: exit 0 under `--json`: `viola link`'s document is `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link, and `viola unlink`'s is `{"v":1,"ok":{}}`.
- New: exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
**Why:** the wrapper now serves `link` / `unlink` (the founder's own answer to this chunk's P4 fork, inputs#I3, relayed verbatim by the operator) and the verb prints its `ok` as `{"v":1,"ok":<ok>}`; a wrapper error, `-32602` included, is exit 20. A `driver` equal to `driven` refused by the wrapper only is the operator's answer of chunk 2026-10-10-session-links.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-explicit-session-links — the `handoff:` help row built
**Section:** Surface: cli › Navigation Pattern (surface.md: the cli-help bullet; design-system.md: the `viola --help` line)
**Change:**
- surface.md As built: `--help` lists the served verbs `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`; was without the `handoff:` row, which "joins when "Explicit session links" serves `link` and `unlink`" (that clause gone); the planned grouping and its provenance unchanged.
- design-system.md: the same five rows; was the four, "the `handoff: link, unlink` row is pending, owner: the route entry "Explicit session links"" (retired).
- Partial retirement: the `handoff:` waiting clause of "2026-10-17-cli-output-discipline — the colour layer and the width cut built; --help groups the served verbs" and of "2026-10-17-optional-plugin-install — plugin install served in setup:, its lines and its exit-21 pair" no longer stands.
**Why:** the chunk serves `link` and `unlink`, and `viola --help` gains the `handoff:` row between `wheel:` and `setup:`, held by the help tests.
**Ref:** .andromeda/runs/2026-10-26T15-35-13-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — the six frame verbs' strict-modes refusal prints the instance-state pair
**Section:** Surface: cli — design-system-detail/surface.md (`values · cli-output`: the stderr bullet; Component Patterns: the board's Refusal item, the `instance-unreachable` hint causes, the exit-code table's 21 row); design-system.md (Streams; pattern 1's refusal paragraph; Exit-code phraseology)
**Change:**
- `viola send`, `wait`, `last`, `answer`, `pause` and `release` run the instance strict-modes check before any frame (`send` after reading its text, the other five first); refused, each prints the fixed instance-state pair `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` naming its instance, exit 21, not the `[/ ]` mirror, with no frame and no line written; under `--json` the `instance-unreachable` document, stderr empty.
- The six verbs join `link` / `unlink` in every list of the pair's printers and of the fixed `unable: <text>` form's users: the surface.md stderr bullet, the Refusal item, the 21 row (its `--json` column: the `list`, `link` and `unlink` refusals and the six frame verbs' strict-modes refusals included), the master's Streams bullet (was `list`, `link` and `unlink`) and its `link` / `unlink` exit-code paragraph.
- The `instance-unreachable` strict-modes cause retired: was the hint `the viola home is not private to you; <name> is not contacted until that is fixed`; now no `unable  instance-unreachable` line and no hint of that list, the instance pair instead. The 21 row's hint causes read not running / unwrapped / server verification (was not running / unwrapped / strict-modes / server verification).
- The master's exit-21 phraseology: `unable  instance-unreachable` then the cause's hint, or for a refused strict-modes check its fixed home-state or instance-state pair instead; pattern 1's refusal paragraph names the six verbs' pair, not the mirror.
**Why:** the chunk built one instance check shared by the six verbs and `link` / `unlink` (`link`'s unchanged in behaviour). The pair and the refusal writing no line are the founder's own answers to P4 forks Q3 and Q2, live by dialog 2026-10-26T18:11Z, relayed by the overseer (`inputs#I3`). The retired cause was rewritten with its companion at the P2 halt's Q2 (a check-7 contradiction), the overseer's answer, founder-delegated, PROVISIONAL (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — run's and revive's start refusals at the home-tree check
**Section:** Surface: cli — design-system-detail/surface.md (Component Patterns: the board's Refusal item, `viola run`'s start refusals, `viola revive`'s refusals, the exit-code table's 1 row)
**Change:**
- `viola run`'s start checks its home tree before any log (the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/`, each that exists): a home-level refusal prints the home-state pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, an instance-level refusal the instance-state pair; exit 1, no child. Was: the home pair qualified only as strict-modes at start.
- `viola revive`'s list gains the same check of its start, after a passed preflight and before any log: home-level → the home pair, instance-level → the instance pair, no child; under `--json` the `strict-modes-failed` document.
- The Refusal item names `run`'s and `revive`'s start among the instance pair's printers, at an instance-level refusal of their home-tree check (exit 1).
- The 1 row: `run`'s refusals gain `<name>'s state files can be written by another user`, `revive`'s gain `the viola home is not private to you`.
**Why:** the chunk's start check, which `run` and `revive`'s start make before the wrapper log opens: a home-level refusal maps to the home pair, an instance-level one to the instance pair, exit 1, no log, no line, no child; `revive --json` keeps `strict-modes-failed`.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — which pair viola list's refusal prints, and its owner
**Section:** Surface: cli — design-system-detail/surface.md (Component Patterns: a new `values · cli-list` block after the board's Refusal item)
**Change:** New: the board's strict-modes scan holds the home to two rules, the write rule on the home, then each instance's set, which holds the home to the read rule. A home that group or other may read but not write passes as the empty board with no instance and, with one, refuses the whole board with the instance-state pair naming that instance: which pair the refusal prints depends on whether an instance exists. Pending, owner: a CARRY on the working-route entry «Unix endpoint and home hardening».
**Why:** the chunk widened the instance check to the read rule on the home while the board's scan, unchanged, checks the home by the write rule first; the operator's wrap directive asked that this inconsistency get an owner (`inputs#I5`). The owner is the overseer's answer to the P2 halt's Q3, founder-delegated, PROVISIONAL (`inputs#I6`), and no other owner is named.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — verify's home-tree and stamps refusals
**Section:** Surface: cli — design-system-detail/surface.md (Component Patterns: a new `values · cli-verify-run` block; the exit-code table's 1 row); design-system.md (Component Patterns: pattern 5, `viola verify`)
**Change:**
- Before its log `verify` checks its home tree (the home, `ledger/`, `bin/`, `plugin/`, each that exists): a refusal prints no step line and no summary, only the home-state pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions` (pattern 2's fixed form), exit 1; a `stamps.json` update refused by its strict-modes check prints the same pair.
- Pattern 5's no-step-line case: was a run stopped by Ctrl-C or TERM before its rows are checked; now that, or a home-tree refusal before its log.
- The 1 row: `verify`'s refusals gain `the viola home is not private to you` (its home tree refused before its log, or a refused `stamps.json` update), with its hint.
**Why:** the chunk's `verify` checks its home tree before its `cli` log and refuses with the home pair; the stamps update now runs its strict-modes check first, and `verify` maps that refusal to the same pair.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-26-home-and-code-bearing-file-integrity — a home under a regular file
**Section:** Surface: cli — design-system-detail/surface.md (Component Patterns: a new `values · cli-verify-run` block)
**Change:** New: on Unix a home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21). On Windows such a path stats `NotFound`, is passed as absent, and keeps the catch-site line.
**Why:** the chunk's checks refuse a path they cannot stat where the catch-site line stood; on Windows, measured in CI on `windows-2025`, the path stats `NotFound`, so the outcome is per-OS.
**Ref:** .andromeda/runs/2026-10-26T21-34-52-wrap/

## 2026-10-31-sanitised-error-surfaces — `unknown`'s detail, a code word or nothing
**Section:** §Color Palette, Semantic Colors (color-palette.md: status-states, the `RefusalReason::unknown` row) · §Surface: cli, Component Patterns (surface.md: cli-send, the no-hint line; cli-exit-codes, row 14)
**Change:**
- `RefusalReason::unknown`'s row: was `unable · unknown` with the detail printed as opaque plain text; now with the detail, a code word or nothing, printed as plain text. The row's colours are unchanged.
- The exit-14 row: was `unable  unknown  <detail as opaque text>`; now `unable  unknown  <detail>`, the detail a code word or nothing. Its `--json` cell stays the refusal object.
- The no-hint line: `unknown` (exit 14) still has no hint line; its reason was that the detail is opaque; now that the detail is a code word or nothing and a hint would have to guess.
**Why:** both clients now screen a peer's reply, so a free-text detail reaches no document or text and the opaque-text wording no longer holds.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the code word and the screen on a peer's reply
**Section:** §Surface: cli, Tokens (surface.md: cli-output, three new items after the stderr line)
**Change:**
- New: a code word is 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first (`viola_core::is_code_word`); both clients, `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`), screen a peer's `refusal` and `detail` through it.
- New: a peer's `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is no known reply.
- New: a code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does (`reason_of`).
**Why:** `viola_core::is_code_word` and its two callers landed in this chunk, so a peer's `refusal` or `detail` reaches no document or text except as a code word. The second item's contradiction with the exit-14 row's line was ruled apply by the overseer as the founder's delegate (Q6), PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — exit 20's closed `detail` and the fixed `wrapper fault` message
**Section:** §Surface: cli, Component Patterns (surface.md: cli-send, the no-hint line; cli-exit-codes, two new items after the exit-20 link / unlink item)
**Change:**
- The no-hint line: `error: wrapper fault` (exit 20) and `error: internal error` (exit 1) still have no hint line, being faults, not refusals; was their detail goes only to `diagnostics/`; now the human line holds the code only, further detail goes to `diagnostics/`, and the `--json` document's `detail` is row 20's closed object.
- New: exit 20's `--json` `detail` is closed for the five known codes (`viola_channel::sanitised_fault`). `message` is each code's fixed text. `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values; `data.reason` exactly `release-from-driver` reads that body; any other `-32602` reads `invalid params` with null `data`. Every other known code has null `data`.
- New: a code outside the five prints the same human line, `error: wrapper fault  <code>`, and under `--json` the document `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
**Why:** `viola_channel::sanitised_fault` replaced the two private `fault_detail` copies, so a peer's `message` and `data` reach no document or text, and `wrapper fault` is a new fixed output word on `--json` and MCP. The two new items' contradiction with the no-hint line was ruled apply by the overseer as the founder's delegate (Q1), PROVISIONAL, with that line's last clause reading as above.
**Kept:** row 20's own cells stand, its document still elided as `"detail":{…}`; the closed contents are the two items below the table.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — exit 2's fixed stderr lines, no `--json` document
**Section:** §Surface: cli, Component Patterns (surface.md: cli-exit-codes, row 2) · §Surface: cli, Tokens, Streams (design-system.md: the `--json` line) · §Surface: cli, Component Patterns, Exit-code phraseology (design-system.md: the exit-code sentence)
**Change:**
- The exit-2 row: was clap usage text (never produced by `viola hook`) with `—` in the `--json` cell; now that clap text, or the fixed `UNUSABLE_RESPONSE` line (an oversize `viola answer`) or `UNUSABLE_TEXT` line (an unusable `viola send`) on stderr, whatever `--json` says, with `tests/contract_error_bodies.rs` naming them in `NOT_ERROR_BODIES`; the `--json` cell reads no document.
- The `--json` line: was the whole result is one JSON document on stdout with the typed exit code, no exception stated; now except an oversize `viola answer` and an unusable `viola send`, which exit 2 with their fixed stderr line and no document.
- The exit-code sentence: was 2 clap usage; now 2 clap usage or the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` stderr line of those two paths (no document under `--json`).
**Why:** measured, the exit-2 paths write the fixed line to stderr whatever `--json` says, with no document, so one document for every result does not hold. The row's contradiction, and the two master lines that depend on it, were ruled apply by the overseer as the founder's delegate (Q5), PROVISIONAL, the clap text kept in the row.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-sanitised-error-surfaces — the `--json` failures the witness drives
**Section:** §Surface: cli, Component Patterns (surface.md: cli-exit-codes, a new item after the exit-20 items)
**Change:** New: each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.
**Why:** `cli_error_documents_hold_no_path_or_upstream_text` measures this shape and reads met; no standing text of the file stated it across the failures.
**Ref:** .andromeda/runs/2026-10-31T09-44-30-wrap/

## 2026-10-31-exit-cause-code-catalogue — exit 21's document names its cause
**Section:** §Surface: cli (design-system.md: Tokens → Streams; Component Patterns 1 `viola list` and 2 `viola send`; Exit-code phraseology) · surface.md: cli-list (the Refusal item), cli-send (the strict-modes hint row), cli-exit-codes (row 21 and the server-verification item)
**Change:**
- Row 21's `--json` cell and the phraseology's code 21: was one document, `{"v":1,"error":"instance-unreachable","detail":null}`, its per-cause `detail` code awaiting an arch amendment; now `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, one per cause, `<cause>` one of five closed words `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, stderr empty.
- The refused strict-modes check of `list`, `link`, `unlink`, `plugin install` and the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`): was the bare `instance-unreachable` document; now with `detail` `strict-modes-failed`. The board's Refusal item and pattern 1's as-built line now state it: the board's only refusal, one stdout document, stderr empty.
- The server-verification item: was `detail` null; now `detail` `server-verify-failed`.
- Pattern 2: was a per-cause `detail` code pending, owner the arch amendment naming them; now exit 21's document names its cause in `detail`, one of five closed words.
- Streams: was the typed `refusal` / `error` fields being the whole answer; now those fields and their `detail`, exit 21's cause being its `detail`.
**Why:** one document could not tell a same-user driver which of exit 21's causes it met; the chunk put the cause in the document. Crossing that boundary is the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him (`inputs#I2` Q1). That these rows lie inside his answer is the overseer's confirm at this wrap's halt, as delegate, PROVISIONAL (`inputs#I6`); the ratification stays the founder's.
**Kept:** `v` stays 1, `error` stays `instance-unreachable` and no exit moved; the human `unable` lines and fixed pairs stand as written.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `endpoint-lost` hint and the four hint causes
**Section:** §Surface: cli · surface.md: cli-send (the `instance-unreachable` hint list, the server-verification hint item), cli-exit-codes (row 21's human cell)
**Change:**
- A new hint row, a wrapper lost with the call open (`endpoint-lost`): `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII. It carries a gap marker: this wording PROVISIONAL, owner the wording of the `endpoint-lost` hint.
- Row 21's human cell: was not running / unwrapped / server verification; now not running / unwrapped / endpoint lost / server verification.
- The server-verification hint item: was the hint cause `server-verify` beside `not-running`; now beside `not-running`, `unwrapped-name` and `endpoint-lost`, with `not-running` named the hint key of the log's `instance-dead`.
**Why:** `instance-unreachable` keeps one hint per cause because the code is shared, and a wrapper lost mid-call is a cause of its own. The chunk built the unwrapped hint and added this one. The wording is the delegate's, the overseer's answer at the P4 fork, PROVISIONAL (`inputs#I2` Q3), and stays open until its owner settles it.
**Kept:** `not-running` stays the hint cause's key, not renamed to the log's `instance-dead`; the unwrapped hint's words did not move.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stale holder's own code
**Section:** §Surface: cli · surface.md: cli-send (the exit-1 start refusals of `viola run`), cli-exit-codes (the stale-instance item)
**Change:**
- A new row under the two holder refusals: these two print and log their own codes, a stale holder `stale-heartbeat`, a live holder and a fresh replayed beat `already-live`. Was the stale-heartbeat cause having no code yet.
- A new item: a frame verb against a stale instance reads `instance-dead`; stale heartbeat is exit 1's cause alone.
**Why:** the chunk gave a stale holder its own printed and logged code and retired `already-live` as a stale holder's refusal; a frame verb finds no live endpoint for a stale wrapper, so it never meets that code. The row landing over the file's "no code yet" line is the overseer's answer to this wrap's halt, Q4, as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** both holder refusals' human pairs stand as written.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the pending arch block spent, the log's codes and the two carve-outs
**Section:** §Surface: cli · surface.md: cli-send (the "Hints are human-mode only" item)
**Change:**
- Was a pending-an-arch-amendment block: arch fixing `detail` null for exit 21 and, for exit 1, only `viola revive`'s strict-modes and preflight refusals, `--json` keeping arch's shape until arch named the codes. Now exit 21's document names its cause in `detail`, one of five closed words (row 21); exit 1's, `detail` null, is printed by `viola revive` for every refusal of the start it shares with `run` and, as `internal-error`, by the `cli` catch site (row 1).
- obs-plan D-20's log detail codes gain `stale-heartbeat`, `unwrapped-name` and `endpoint-lost`, beside `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and for `viola revive` `no-session` and `cwd-missing`.
- The server-verification refusal's `--json` document: was `detail:null`, as the not-running cause's; now reads the same word as its log line.
- Was the stale-heartbeat and unwrapped-name causes having no code yet; now the socket-directory cause and macOS's server check stay without these codes, owner the route entry "Unix endpoint and home hardening".
**Why:** the block's wait is over: `detail` carries the cause by the founder's own word (`inputs#I2` Q1). The claim with its named carve-outs is his too, live, through the overseer's dialog (`inputs#I2` Q4), and the two causes left out stay with that route entry. The socket-directory row is also the delegate's, the overseer's P5 approval, PROVISIONAL (`inputs#I3`): the code holds no such refusal, its failure reaching the catch site as an anyhow error. The overseer's confirm at this wrap's halt is as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** `viola revive`'s strict-modes refusal still writes no log line, so it carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the catch site's `internal-error` document
**Section:** §Surface: cli (design-system.md: Tokens → Streams; Exit-code phraseology) · surface.md: cli-send (the no-hint fault item), cli-exit-codes (row 1's `--json` cell and the catch-site items)
**Change:**
- Row 1's `--json` cell and the phraseology's code 1: was no document at the catch site; now `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error` under `--json`, stderr then empty. Without `--json` the catch site prints that line, byte for byte.
- The no-hint fault item: was the `--json` document's `detail` being row 20's closed object for both faults; now row 20's closed object for the wrapper fault and null for the internal error.
- New: the catch site counts `--json` only among the arguments before a `--`, read from argv.
- New, in the items and in Streams: `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too.
**Why:** a verb asked for one machine document had none where its error reached the catch site. The founder's own word, live, through the overseer's dialog, moved it, exit 1 and the human line staying (`inputs#I2` Q1). The document's item landing over row 1's "every other exit 1 has no document" is the overseer's answer to this wrap's halt, Q4, and the confirm of the rewritten lines its Q1, both as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** the human line is unchanged byte for byte, and neither fault gains a hint.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `revive --json`'s documents and its ten codes
**Section:** §Surface: cli (design-system.md: Tokens → Streams; Component Patterns 5 `viola verify` / `viola run`; Exit-code phraseology) · surface.md: cli-verify-run (the `revive` item and three new items), cli-exit-codes (row 1's `--json` cell)
**Change:**
- Was one document for a refused strict-modes check or a preflight refusal, five codes, and none for a refusal `run`'s start makes after a passed preflight, owed to the working-route entry "Exit-cause code catalogue". Now one stdout exit-1 document `{"v":1,"error":"<code>","detail":null}` for every refusal of the start it shares with `run`, no hint, stderr empty; the owed clause is gone.
- Its closed codes, was five, now ten: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`. `run` itself prints none of these documents.
- New: a program not found prints `internal-error`, a cause with no human pair; an anyhow error of its start prints the same document, and the catch site logs its chain and its line.
- New, in the item, row 1 and Streams: a `pump_child` error prints no document, the child having held the terminal and viola writing nothing past it.
**Why:** the chunk closed the post-preflight gap its route carried: "a refusal `run`'s start makes after a passed preflight prints no document" stopped being true. `run`'s causes reaching a machine only through `revive --json` is inside the founder's own answer, live, through the overseer's dialog (`inputs#I2` Q4). The codes and `internal-error` items landing over the file's five codes and no-document clause are the overseer's answer to this wrap's halt, Q4, and the confirm of the rewritten lines its Q1, both as delegate, PROVISIONAL (`inputs#I6`).
**Kept:** a `pump_child` error stays without a document: a document printed from `main` when the start returned was weighed and rejected, since that error reaches the same catch site after the child has held the terminal.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — how a frame verb reads no endpoint and a lost call
**Section:** §Surface: cli · surface.md: cli-exit-codes (two new items)
**Change:**
- With no live endpoint a frame verb reads its instance directory after the strict-modes check: a directory is `instance-dead`; absent, a regular file or a symlink is `unwrapped-name`. It carries a gap marker: this reading PROVISIONAL, owner whether an unwrapped name reads by its instance directory.
- A failed connect is `instance-dead`; a wrapper lost with the call open is `endpoint-lost`.
**Why:** the five causes need a rule for which one a verb names, and the chunk built this one, the strict-modes check first at every caller. Reading an unwrapped name by its instance directory is the delegate's word, the overseer's at the P4 fork, PROVISIONAL (`inputs#I2` Q2), and stays open until its owner settles it. These items landing over the file's "no code yet" and `detail:null` lines are the overseer's answer to this wrap's halt, Q4, as delegate, PROVISIONAL (`inputs#I6`).
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
