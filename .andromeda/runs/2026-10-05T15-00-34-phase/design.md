# design extract

## Relevance
partial — no rendered surface (no web-spa, tokens, motion or iconography change); the chunk's e2e case drives three CLI verbs whose output the design-system §Surface: cli fixes (`viola wait`'s dialog result line, `viola answer`'s result/refusal, `viola hook`'s no-human-surface rule), so only the cli surface's phraseology and stream rules bind.

## Constraints
- design-system §Surface: cli → Component Patterns 3 requires `viola wait`'s dialog wake to print one stdout result line in the `<kind>  <name>  dialog <id>  cursor <n>` form (the `question` sample generalises to `permission`), `dialog unknown` when the event carries no `dialog_id`, `DIALOG`/kind uncoloured there ("a result, not the board"), and one static `waiting: <name>` stderr line on a TTY only; whether the binary already prints this for the `permission` kind is research's question.
- design-system §Surface: cli → Component Patterns 4 requires `viola answer <name> <id>` to print `answered  <name>  dialog <id>` on success and `unable  <name>  not-delivered  unknown-dialog` on an unknown id; §Component Patterns 2's hint list fixes that refusal's one `hint:` line (`that dialog is not pending; viola list shows the current DIALOG`), human mode only.
- design-system §Surface: cli → Platform-Specific Notes and the Colour decision order (rule 3) require `viola hook` (the `permission-request` arm included) to write nothing to stderr and carry no human design surface: its stdout is the decision body only, never styled, never a phraseology line.
- design-system §Surface: cli → Streams requires results on stdout and context/refusal/hint lines on stderr; under `--json` the whole result (refusal included) is one JSON document on stdout with the typed exit code and no `hint:` line.
- design-system §Surface: cli → Exit-code phraseology fixes `unknown-dialog` as an exit-13 `not-delivered` refusal (`unable  not-delivered  unknown-dialog` human; refusal object under `--json`).
- design-system §Color Palette → Semantic Colors ("Not visualised in v1") records that `answer` behaviours (`allow` / `deny`) are not logged as events, so no tape or strip visual is owed for the decision this chunk carries; the `permission` kind's existing visuals (`PendingDialogKind::permission`, `EventKind::permission`) are unchanged by this chunk.

## Patterns to follow
- The `question` / `plan` wait and answer phraseology (design-system §Surface: cli → Component Patterns 3 and 4) is the shape the `permission` case reuses — one kind word swapped, no new line form.
- The colour decision order (design-system §Surface: cli → Tokens): a test harness reading verb output through a pipe or `--json` sees no SGR, so assertions match plain words.
- `wait` / `last` message escaping (design-system §Surface: cli → Component Patterns 3, message mode): any upstream text a permission event carries (tool name, `input`) prints C0/C1/DEL as `\xHH` in human mode and stays serde-escaped under `--json`.

## Anti-patterns to avoid
- Printing upstream text (the tool `input`, a deny `message`) or a path/pid in a refusal or hint line (design-system §Anti-Patterns → Per-Surface Bans, cli: "NEVER print upstream text, paths, pids…").
- Any colour, glyph or cursor control under `--json`, non-TTY or from `viola hook` (design-system §Anti-Patterns → Per-Surface Bans, cli: "NEVER emit colour, glyphs or cursor control…"; "NEVER mix data and messages").
- A bare "done"/"success" result for `answer` instead of the contextual `answered  <name>  dialog <id>` (design-system §Anti-Patterns → Per-Surface Bans, cli: "NEVER print \"done\" or \"success\" without context").

## Contract bindings
- design ↔ security: the hook-stdout-only rule and the no-upstream-text-in-hints ban bind to the security plan's NEVER-log floor and fixed-message error handling (cited by design-system §Anti-Patterns → Per-Surface Bans, cli).
- design ↔ architecture: the `--json` shapes and exit codes the cli surface cites come from architecture §Conventions (design-system §Surface: cli → Exit-code phraseology: "typed codes from architecture.md").

## Acceptance criteria contributions
- (design) If the `permission` case asserts human-mode output, `viola wait` prints `permission  <name>  dialog <id>  cursor <n>` on stdout, uncoloured (per design-system §Surface: cli → Component Patterns 3).
- (design) If it asserts human-mode output, `viola answer` prints `answered  <name>  dialog <id>` on stdout for a pending id, and `unable  <name>  not-delivered  unknown-dialog` plus its one `hint:` line on stderr (exit 13) for an unknown id; under `--json` no hint line appears (per design-system §Surface: cli → Component Patterns 2 and 4, Streams).
- (design) The `viola hook permission-request` run writes zero bytes to stderr, and its stdout holds only the decision body, with no SGR (per design-system §Surface: cli → Platform-Specific Notes).
