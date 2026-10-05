# design extract

## Relevance
partial — the chunk is ledger/probe/gate mechanism with no web-spa work; its design surface is the cli human output of `viola verify` (step counter, summary, `--record` refusal) and, through the R2 closure, the `verified` / `unverified-cli` words and the `answer` / `wait` dialog result lines.

## Constraints
- design-system §Surface: cli → Component Patterns 5 (`verify`) requires `verify` to print a step counter as static appended stdout lines, one per ledger row (`[NN/TT] <row-id> <claim>  pass|fail`), with `TT` the row count, and the count grows as owning chunks land rows. Going from 10 to 14 rows means `/14` and a summary line `stamped <ver>  <n> pass  <m> fail`. The two-digit zero-padded counter stays as it is. Whether the code builds the total from `LedgerRow::ALL` or hardcodes `10` is research's question.
- design-system §Surface: cli → Component Patterns 5 and the Exit-code phraseology row 1 require that a failing ledger row, including a dialog row whose probe did not raise its dialog, prints no stderr word. It shows only as `fail` on its step line and in the summary count. `fail` is never coloured.
- design-system §Surface: cli → Exit-code phraseology row 1 requires a `--record` refusal that covers the new dialog payload fixtures to use the fixed form `unable: a recorded fixture is not clean: <file> <code>`. The codes are `home-path` · `absolute-path` · `username` · `email`, and the line never includes the content. Plan text or a path in `tool_input` must not reach stderr.
- design-system §Brand Identity (domain anchor "Verified aircraft type") and §Color Palette → Semantic Colors (`CliVerified::true` / `CliVerified::false` rows) define two CLI words: `<ver> verified`, and `<ver> unverified-cli`, which means transport only with dialog answers held back. The R2 closure changes what a stamp must contain before dialog answers flow. The plan has no word for a version whose ten spine rows pass while a dialog row fails. Inventing a third CLI word, colour or state is out of bounds here. P4 must decide which existing word such a stamp prints, and whether the refusal is `unverified-cli` (exit 12, hint `run viola verify for this CLI version`, per §Surface: cli → Component Patterns 2). Any new word needs a design-system amendment.
- design-system §Surface: cli → Component Patterns 3 and 4 fix the dialog result lines that the `permission` end-to-end case exercises: `wait` → `permission  <name>  dialog <id>  cursor <n>` (no amber, because this is a result line, or `dialog unknown` when there is no `dialog_id`), and `answer` → `answered  <name>  dialog <id>` or `unable  <name>  not-delivered  unknown-dialog`.
- design-system §Surface: cli → Tokens and the Colour decision order allow only two colours: amber on the `DIALOG` word in `viola list`, and dim on `stale` rows. `verify` output stays uncoloured at every depth, and `verify --help` stays one static ASCII paragraph that names no path.
- design-system §Expression level (cli rows 0.2 / 0.0) means no motion or redraw anywhere. That includes a long interactive re-probe run: it may not show progress through a spinner or an in-place counter.

## Patterns to follow
- The step-counter line pattern, the project's only progress pattern (§Surface: cli → Component Patterns 5): give each new dialog row (S3, S7, S8, dialog concurrency) one static line in `LedgerRow::ALL` order, each with a short fixed claim phrase in the voice of the existing ten.
- Standard phraseology (§Brand Identity, domain anchor "Standard phraseology and 'unable' plus a reason"): row ids and claims use short fixed lower-case words. Any new refusal follows `unable` + reason + detail and has its own `hint:` line on stderr (§Surface: cli → Streams).
- Hint discipline (§Surface: cli → Component Patterns 2): hints are human-mode only and never printed under `--json`. A hint never quotes upstream text, and no hint names `viola release`.
- Streams (§Surface: cli → Streams): step lines and the summary go to stdout. Refusals, `hint:` lines and `error:` go to stderr.

## Anti-patterns to avoid
- design-system §Anti-Patterns → Per-Surface Bans → cli bans a spinner, progress bar or live redraw while the interactive re-probe waits for a dialog to rise, and bans green `✓` / red `✗` or emoji on pass/fail lines.
- design-system §Anti-Patterns → Per-Surface Bans → cli (no upstream text, paths or pids in errors or hints) means recorded dialog payload content (question text, plan text, tool `input`, paths) must never appear in a `verify` line, refusal or hint.
- design-system §Anti-Patterns → Universal Bans ("every color communicates meaning") bans colouring a failed dialog row or a partially verified version. Amber means `dialog_pending` and nothing else.

## Contract bindings
- design ↔ tests: the `verify` step-counter text (`/14`, the summary line) and the `--record` refusal line are human output, and the fake-agent CI replay and the live stamp compare against them (test-plan §6 Path 4 / §10). A snapshot or golden of the verify output has to move from `/10` to `/14` together with the row count.
- design ↔ security: the `--record` refusal's closed codes and the never-the-content rule (§Surface: cli → Exit-code phraseology row 1) mirror the security plan's NEVER-log floor for recorded dialog payloads.
- design ↔ architecture: the `verified` / `unverified-cli` words (§Color Palette → Semantic Colors `CliVerified`) follow architecture's stamp and gate semantics. Closing R2 needs those two specs to agree on what `verified` means once the dialog rows exist.
- design ↔ a11y: none new. The web page's `DIALOG permission` strip and its announcements are unchanged by this chunk.

## Acceptance criteria contributions
- (design) `viola verify` on a TTY prints exactly one static stdout line per ledger row, `[01/14]` through `[14/14]`, then `stamped <ver>  <n> pass  <m> fail` with n + m = 14. There is no colour, no glyph and no in-place redraw (per design-system §Surface: cli → Component Patterns 5).
- (design) When a dialog row's re-probe fails to raise its dialog, that row prints `fail` on its own step line, the summary counts it, and nothing goes to stderr for it (per design-system §Surface: cli → Exit-code phraseology).
- (design) A `--record` refusal over a dirty dialog fixture prints only `unable: a recorded fixture is not clean: <file> <code>` with a closed code, plus its hint. No payload text appears on any stream (per design-system §Surface: cli → Exit-code phraseology, row 1).
- (design) The `permission` end-to-end case's human output matches the fixed forms `permission  <name>  dialog <id>  cursor <n>` (`wait`) and `answered  <name>  dialog <id>` (`answer`), with no SGR when stdout is not a TTY (per design-system §Surface: cli → Component Patterns 3 / 4 and Colour decision order).
