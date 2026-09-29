# design extract

## Relevance
partial — the chunk is Windows PTY transport with no web-spa surface; only design-system §Surface: cli bears, through `viola run`'s human output around the sideload check and its fallback.

## Constraints
- `viola run` prints nothing once the child starts, and its only human lines are the exit-1 start refusals, each a fixed-message line plus one hint (per design-system §Surface: cli → Component Patterns 5 `run`, and the Colour decision order step 2). A sideload that is absent, fails its hash or fails its signature check is a degrade to the inbox ConPTY, not a start refusal, so it adds no stderr line and no new `unable:` cause. Whether the start path prints anything today between pin and spawn is research's question.
- The existing start refusal `unable: the pinned viola copy failed its integrity check` belongs to the pinned exe's SHA-256 mismatch only (per design-system §Surface: cli → Component Patterns 2, the `run` start-refusal list, and the exit-code phraseology table row 1). The design leaves no place to reuse it for a ConPTY file mismatch. A new refusal cause would need its own fixed line and hint in the plan first, which would be a boundary change, not a lean.
- The strip's fields are fixed as NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the CLI `list` table has the same columns and words (per design-system §Brand Identity "Flight-progress strip" and §Surface: cli → Component Patterns 1). The PTY backend the chunk records has no board column or field word, so it stays an obs field and never reaches `list` or the web strip.
- `viola run`'s terminal belongs to the child's screen: nothing viola emits there may carry colour, glyphs or cursor control (per design-system §Anti-Patterns → Per-Surface Bans cli, and §Surface: cli Colour decision order step 2). The sideloaded `OpenConsole.exe` renders the child, and viola adds no bytes of its own to that stream.
- Any human-facing message the chunk adds on another verb (for example a build or pin-time failure surfaced through `viola verify` or `run`'s exit 1) uses the fixed-message phraseology, ASCII only, and never names a path, a pid or a hash value (per design-system §Surface: cli → Width "Human output is ASCII only", and §Anti-Patterns → Per-Surface Bans cli "NEVER print upstream text, paths, pids …").

## Patterns to follow
- The fixed-message start refusal plus one keyed hint (the `unable: <cause>` → `hint: …` pair), if P4 ever makes a sideload condition a refusal (per design-system §Surface: cli → Component Patterns 2 `run` start refusals).
- The stream split: results to stdout, refusals, hints and errors to stderr, and under `--json` one document on stdout with no hint (per design-system §Surface: cli → Streams).
- The conhost-without-VT note: when a capability is missing, output degrades silently and the words still carry the state (per design-system §Surface: cli → Platform-Specific Notes "conhost without VT"). The inbox-ConPTY fallback follows the same silent-degrade stance on the human surface.

## Anti-patterns to avoid
- A warning, notice or progress line on `viola run`'s terminal when the sideload is missing or rejected (per design-system §Anti-Patterns → Per-Surface Bans cli: no colour/glyph/cursor control under `viola run`, and "NEVER mix data and messages").
- Printing the pinned bin dir, the DLL path, the package version or a SHA-256 in any human error or hint (per design-system §Anti-Patterns → Per-Surface Bans cli "NEVER print … paths …", and "NEVER print stack traces").
- A new board field, column or colour for the PTY backend (per design-system §Anti-Patterns → Per-Surface Bans cli "NEVER colour anything except `DIALOG` … and `stale`", and §Brand Identity's fixed strip fields).

## Contract bindings
- design ↔ obs: the backend the chunk records (obs-plan's `pty.spawn` `pty_backend`) is the only place the sideload-vs-inbox choice surfaces. design-system §Surface: cli gives it no human surface, so the obs extract owns the shape.
- design ↔ security: the no-paths/no-hash rule on human messages (design-system §Anti-Patterns → Per-Surface Bans cli) is the human-surface side of the security plan's fixed-message error rule. Any sideload-failure detail goes only to `instances/<name>/diagnostics/` (design-system §Surface: cli → Platform-Specific Notes "Stack traces").
- design ↔ arch: `run`'s exit-1 start-refusal list (design-system §Surface: cli → exit-code phraseology table) mirrors architecture's refusal set. A new cause would have to be added in both.

## Acceptance criteria contributions
- (design) With the sideload absent, hash-mismatched or signature-rejected, `viola run` starts on the inbox ConPTY and writes no byte of its own to stderr or stdout before or after the child starts. A test captures `run`'s own streams and asserts both are empty on the fallback path (per design-system §Surface: cli → Component Patterns 5 `run`).
- (design) No human-facing string the chunk adds (a refusal, hint or error) contains a path separator, a hex digest or a version string of the ConPTY package. A string-table or source grep over the new messages passes (per design-system §Anti-Patterns → Per-Surface Bans cli).
- (design) The `viola list` header and row columns are unchanged: NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, with no backend column (per design-system §Surface: cli → Component Patterns 1).
