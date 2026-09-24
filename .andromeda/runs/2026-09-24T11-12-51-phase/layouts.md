# layouts extract

## Relevance
Partial. The chunk builds no UI surface, region, focus order, modal or breakpoint. It does touch what the cli surface prints on error: the fixed error and refusal lines on stderr, the stream split, and the `--json` error document. The web-spa surface is out of scope, because viola-ui does not exist at HEAD (scope §Boundaries OUT).

## Constraints
- A refusal or error line on stderr has a fixed form: `unable  <name>  <reason>  <detail>`, with the exit-1 start-refusal exceptions (`unable: <name> is already live` and its sibling causes). The only variable fields are the name and the typed codes, so fixed thiserror `Display` text is what fills the line (per layout-templates §Surface: cli › Component — Primary content block 2: refusal lines and the `unable` column).
- A `hint:` line never quotes sent text or upstream text, and never names a path or a pid. No hint is printed for `unknown`, `wrapper fault` or `internal error` (per layout-templates §Surface: cli › Component — Primary content block 2, Hint line).
- `viola run`'s exit-1 start refusal prints "no paths, no pids" on stderr (per layout-templates §Surface: cli › Output structure — `viola run`). Whether the current `main` catch site (which today drops `Err(_)` and exits 1) prints any stderr line at all is a question for research. Whatever line it prints must keep this form. The anyhow chain goes to the detail sink, not to this line.
- Streams: results go to stdout. Refusals, hints, errors and the `viola ui` launch line go to stderr, and the two are never mixed. Under `--json`, an outcome is one document on stdout with no stderr line and no hint text. So no content-bearing chain may appear in either the stderr line or the `--json` document (per layout-templates §Surface: cli › Component — Primary content block 2, Streams / Exit codes as the typed tail).
- Exit code 20's line is exactly `error: wrapper fault  <code>`: a code, never upstream text (per layout-templates §Surface: cli › Component — Primary content block 2, Exit codes as the typed tail).
- The credential-bearing content of the `viola ui` launch line is deferred to the security plan. The layout gives it one unstyled stderr line only (per layout-templates §Surface: cli › Primary screens (commands); §Decisions Log › Notable surface-specific deferrals). The NEVER-log launch URL/token floor sits on top of this. `viola ui` is OUT at HEAD, so this is a forward constraint only.
- The human error and refusal words are an output contract that LLM drivers read. Redaction work must not change the words or column layout of existing lines. New fields go into `--json` first (per layout-templates §Surface: cli › IA notes, Output as a contract).

## Patterns to follow
- A typed code in each field, plus one hint per cause, lets an agent tell causes apart from the last stderr line without any payload text (per layout-templates §Surface: cli › Component — Primary content block 2; design-system cli pattern 2 is referenced there).
- The exit code is the terminator. On a refusal, the `hint:` line is the last stderr line, and no summary or banner follows it (per layout-templates §Surface: cli › Component — Footer / terminator).
- Expression level 0.0 applies under `--json`, on non-TTY, with `NO_COLOR` and `TERM=dumb`: no colour, no glyph, no cursor control. Error lines are uncoloured on every path (per layout-templates §Surface: cli, Expression level; § Hero / signature output line, Colour).
- Normal output never shows a version string, banner or path. The version appears only in `--version` / `--help` (per layout-templates §Surface: cli › Component — Header / banner).

## Anti-patterns to avoid
- Any path, pid, upstream text or sent content in a stderr refusal line, error line or hint (per layout-templates §Surface: cli › Component — Primary content block 2; § Output structure — `viola run`).
- Hint text, or a second stderr line, under `--json` (per layout-templates §Surface: cli › Component — Primary content block 2, Exit codes as the typed tail).

## Contract bindings
- layouts cli error and refusal line form ↔ security §error-sanitization-wire / §logging-redaction-wire: the fixed thiserror `Display` messages are what the `unable`/`error:` fields render. The layout owns the line shape and security owns what is allowed in it.
- layouts `--json` refusal document (`"detail":null` for exit 21, pending arch detail codes) ↔ architecture error shape: no chain or drift text may enter `detail`.
- layouts cli stderr contract ↔ test-plan exit-cause matrix: each cause needs its own hint, and the tests assert the last stderr line (per the T4 amendment below).

## Acceptance criteria contributions
- (layouts) Every stderr error or refusal line this chunk emits or changes matches `unable  <name>  <reason>  <detail>`, `unable: <name> <fixed cause>` or `error: <fixed kind>  <code>`, and contains no filesystem path, pid, serde/anyhow chain text or user content (per layout-templates §Surface: cli › Component — Primary content block 2).
- (layouts) With `--json`, an error or refusal outcome prints exactly one JSON document on stdout, nothing on stderr, and no hint text or chain text (per layout-templates §Surface: cli › Component — Primary content block 2, Exit codes as the typed tail).
- (layouts) When a refusal line has a hint, that `hint:` line stays the last stderr line after the detail-sink routing is added. Routing to `detail-<process>.ndjson` adds no extra stderr output (per layout-templates §Surface: cli › Component — Footer / terminator).
- (layouts) The words and columns of existing human output lines do not change (per layout-templates §Surface: cli › IA notes, Output as a contract).

## Relevant amendment history
- The sidecar `layout-templates-amendments.md` does not exist, so the sidecar history is empty. The plan's own §Decisions Log has one fix-pass entry that applies here: **T4** (overseer fix pass, 2026-09-24). It gave every exit-21 and exit-1 cause its own hint, because the tests' exit-cause matrix needs cause-specific hints. It also ruled that `--json` carries no hint text, with per-cause detail codes pending an arch amendment (`"detail":null` for exit 21). The reason this matters here: fixed error messages must keep causes distinct through typed codes, not through descriptive upstream text. T3, T5, Y4 and Z10 do not touch this chunk's area.
