# design extract

## Relevance
partial — the chunk renders no web-spa surface; its design reach is the CLI text of `viola verify` (one step line per new ledger row, the summary count, the re-recorded fixtures' refusal) and, if `/compact` joins the local-command list, the `viola send` readback mirror.

## Constraints
- `viola verify` prints one static, appended stdout step-counter line per ledger row, `[NN/MM] <row-id> <one-sentence effect>  pass` / `... fail`, so each row this chunk adds to `LedgerRow::ALL` owes one such line and moves the `MM` denominator; the plan's own row list reads "seventeen rows today … the count grows as owning chunks land their rows" (per design-system §Surface: cli §Component Patterns, pattern 5 `verify`).
- The last stdout line of `verify` is the summary `stamped <ver>  <n> pass  <m> fail`, with the word `fail` uncoloured; a run stopped by Ctrl-C or TERM prints no step line and no summary, only the fixed `unable:` / `hint:` pair on stderr. A new child, such as a `--resume` probe or a fifth Run B paste, leaves that pair unchanged (per design-system §Surface: cli §Component Patterns, pattern 5 `verify`).
- Human output is plain ASCII and lines are never wrapped. A new row's id and description sentence must stay ASCII on one line (per design-system §Surface: cli §Tokens (platform-specific), Width).
- Streams: step lines and the summary go to stdout; refusals with their `hint:` line directly after go to stderr, in the fixed `unable: <text>` form that `verify`'s exit-1 refusals use (per design-system §Surface: cli §Tokens (platform-specific), Streams).
- A re-recorded fixture that fails the clean check is refused as `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), using only the closed codes `home-path` · `absolute-path` · `username` · `email` and never the content (per design-system §Surface: cli §Component Patterns, Exit-code phraseology, row 1).
- If `/compact` joins the local-command list, its `viola send` result follows the readback mirror: `[RB] read back` only on a measured post-condition (as `/clear` reads back on its new-session post-condition), otherwise `[  ] unconfirmable  <name>  local command, no measured post-condition` (per design-system §Surface: cli §Component Patterns, pattern 2 `viola send`).

## Patterns to follow
- Row line wording follows the seven dialog and framing rows the plan names: a kebab-case row id, then a short present-tense effect sentence (e.g. `local-command-clear /clear starts a new session and submits no prompt`, `long-paste-wrapper a long paste unwraps to the text as pasted`). Each new row (the inner-LF paste, `--resume`, `/compact`, the settings override, `/bin/sh`, `rate_limits`, the `claude agents --json` join field) takes the same shape (per design-system §Surface: cli §Component Patterns, pattern 5 `verify`).
- A step counter is the project's one progress pattern, made of static appended lines. More rows or a longer probe never turn it into a redrawn line (per design-system §Surface: cli §Component Patterns, pattern 5 `verify`).
- A read-back that is not presumed: a send stays `open` until its matching post-condition, then fills or is struck with `unable` and its typed reason. For a local command, the post-condition is that command's own measured one (`/clear`: its new session) (per design-system §Anti-Patterns §Rejected Defaults, the "Sending…" spinner bullet).

## Anti-patterns to avoid
- No spinner, progress bar or live-redrawing line during a longer probe such as a live turn for `/compact` or a resumed child; no green `✓` / red `✗` or emoji on a pass or fail (per design-system §Anti-Patterns §Per-Surface Bans, cli).
- No colour on any `verify` line; colour belongs only to `DIALOG` (amber) and `stale` rows (dim) (per design-system §Anti-Patterns §Per-Surface Bans, cli).
- No path, pid, upstream text (a probe's captured payload, a session id, the `--settings` file path, a statusline command) or anyhow chain in any new `unable:` / `hint:` line or step line (per design-system §Anti-Patterns §Per-Surface Bans, cli).

## Contract bindings
- design ↔ architecture capability ledger: the step counter's `MM` and the summary's pass count equal the length of `LedgerRow::ALL`, so adding a row moves architecture §Cross-cutting Patterns (Capability ledger) and design-system §Surface: cli §Component Patterns pattern 5's inline row list and count together. Whether a test pins today's `[NN/17]` / `17 pass` text is research's question.
- design ↔ security-plan `--record` refusal: the fixture refusal's closed codes and the never-the-content rule are shared with `.claude/rules/security.md`'s `viola verify` bullet.
- design ↔ architecture local-command list: `/compact`'s readback form (pattern 2) depends on whether that list gains it with a measured post-condition.

## Acceptance criteria contributions
- (design) On a stamped home where every row passes, `viola verify` prints exactly one `[NN/MM]` stdout line per `LedgerRow::ALL` row, with `MM` equal to that length, then `stamped <ver>  <MM> pass  0 fail`. Every line is plain ASCII with no SGR byte (per design-system §Surface: cli §Component Patterns, pattern 5 `verify`).
- (design) No new step line, refusal or hint holds a path, pid, session id, captured payload text or a non-ASCII glyph (per design-system §Anti-Patterns §Per-Surface Bans, cli).
- (design) If `/compact` lands on the local-command list, `viola send` of `/compact` ends in `[RB] read back` on its measured post-condition, or in the `unconfirmable` line, and never in a presumed fill (per design-system §Surface: cli §Component Patterns, pattern 2 `viola send`).
- (design) After the chunk lands, design-system §Surface: cli §Component Patterns pattern 5's row count and named row list match `LedgerRow::ALL` (per design-system §Surface: cli §Component Patterns, pattern 5 `verify`).
