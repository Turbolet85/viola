# layouts extract

## Relevance
partial: the cli surface only (`viola pause` / `viola release` output, the exit-10 refusal line and its hint, `--help` grouping, `viola run` passthrough). The web-spa wheel rendering is Epoch 8 and out of scope (scope §Boundaries).

## Constraints
- The two new verbs print the spoken-phraseology success lines laid out per layout-templates §Surface: cli › Output structure — wheel, handoff and dialog verbs: `pause` prints `<name>  wheel human  manual-pause  I have control` and `release` prints `<name>  wheel <holder>  you have control`. These are two-space-gutter fields, with no glyph and no colour. The `release --budget` line in that block belongs to the budget governor (`:91`, scope §5/§Boundaries), so this chunk owes at most the flag's wire shape and not that line.
- Refusals from `pause`, `release`, `send` and `answer` use the shared struck-box word column, `unable  <name>  <reason>  <detail>`, on stderr (per layout-templates §Surface: cli › Signature placement and Component — Primary content block 2). On `viola send` it is padded after `[/ ] unable` in the mirror (Component — Hero / signature output line). The wheel refusal is exit 10 `human-typing`. Its detail word (`manual-pause` for a pause) comes from arch, not from this plan.
- Every refusal is followed by one `hint:` line as the last stderr line, keyed by reason · detail (per §Component — Primary content block 2, Hint line). No hint may name `viola release`, because hints reach drivers and `release` is a human verb reached only through `--help` (per §Component — Primary navigation, Discoverability). This covers the `human-typing` / `manual-pause` hints on `send` and `answer`.
- Argument order mirrors the channel params: `viola pause <target>` and `viola release <target>`, where `<target>` is a `ViolaName`. The verbs are flat, with no interactive prompt (per §Component — Primary navigation (verb structure)). `--json` is the machine view: one typed document on stdout, no stderr line, no hint and the typed exit code (per §Component — Primary content block 2, Exit codes / Streams).
- `--help` groups the verbs as `wheel:    pause, release`, alongside board / traffic / handoff / setup (per §Output structure — `viola --help`). Whether the help table at HEAD already lists them is research's question.
- `viola run` passthrough prints nothing while the child runs, at expression level 0.0: no colour, no non-ASCII glyph and no cursor control (per §Surface: cli › Expression level and Output structure — `viola run`; Component — Header / banner "`viola run` prints nothing"). A wheel move, a held paste byte or a `null` dialog answer must add no byte to the human's terminal. The wheel is recorded only in `events.ndjson` and the snapshot.
- Streams are never mixed: results go to stdout, refusals and hints go to stderr (per §Component — Primary content block 2, Streams). The last printed line states what happened and to whom, with no `done.` or `success` terminator (per §Component — Footer / terminator).

## Patterns to follow
- Copy the existing `viola answer` / `viola wait` refusal-line and hint shapes from the same plan block (`unable  builder  not-delivered  unknown-dialog` + `hint:`). The wheel verbs use the same two-space field grammar (per §Output structure — wheel, handoff and dialog verbs).
- `viola send`'s exit-10 outcome reuses the mirror's TTY issue line followed by the `[/ ] unable` outcome line, as the exit-13 example shows (per §Output structure — `viola send`).
- The one catch site prints `error: internal error` once, exit 1, with no hint, for a failed or panicked verb (per §Component — Primary content block 2). Whether `pause` / `release` join the listed `cli` verbs there is P4's call.
- The WHEEL column words `driver` / `human` in `viola list` are the same words the new verbs print (per §Component — Primary content block 1). Whether `viola list` exists at HEAD and already reads the snapshot's `wheel` is research's question.

## Anti-patterns to avoid
- No colour, glyph, spinner or `✓` on `pause` / `release` / refusal output. CLI colour is reserved for `DIALOG` and `stale` (per §Component — Hero / signature output line, Colour; §IA notes, Multi-surface coordination).
- No hint text, doc line or driver-facing message that suggests `viola release` (per §Component — Primary navigation, Discoverability).
- No banner, status line or redraw printed into the `viola run` terminal when the wheel moves (per §Component — Header / banner, No banner anywhere).

## Contract bindings
- cli ↔ architecture: the exit codes (10 `human-typing`, 20 wrapper fault for `release-from-driver` `-32602`) and the `{wheel, budget_paused}` reply shapes come from arch's channel `pause` / `release` rows. This plan only renders them (per §Component — Primary content block 2, Exit codes).
- cli ↔ design-system: the hint-per-cause rule is design-system cli pattern 2 (per §Component — Primary content block 2, Hint line).
- cli ↔ web-spa (deferred): the web tape line `wheel  human · human-input` (per §Surface: web-spa › Wireframe — Bay, steady state) uses the same `holder · cause` words as the wheel event. No web rendering lands here.

## Acceptance criteria contributions
- `viola pause <name>` prints exactly `<name>  wheel human  manual-pause  I have control` on stdout, exit 0. `viola release <name>` prints `<name>  wheel <holder>  you have control` (per layout-templates §Output structure — wheel, handoff and dialog verbs).
- A `send` / `answer` refused for the wheel prints `unable  <name>  human-typing  <detail>` on stderr, exit 10, with the `hint:` line last, and that hint does not contain `release`. Under `--json` the verb prints one document on stdout and nothing on stderr (per §Component — Primary content block 2; §Component — Primary navigation, Discoverability).
- `viola --help` lists `wheel:    pause, release` (per §Output structure — `viola --help`).
- A wheel move during `viola run` writes no byte to the outer terminal beyond the child's own passthrough (per §Output structure — `viola run`; §Surface: cli › Expression level).
