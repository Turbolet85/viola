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
