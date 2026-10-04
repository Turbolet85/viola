# layouts extract

## Relevance
partial — the chunk builds no surface and no web element; its only layout touch is the cli surface's `viola send` refusal output (the struck readback mirror carrying `not-delivered  turn-running`, its hint line, exit 13, the `--json` refusal document), plus a parity guard on `viola list` / the web strip if the running-turn state reaches `snapshot.json`.

## Constraints
- The `turn-running` refusal renders as the `viola send` struck-box outcome line: `[/ ]` in the fixed first column, `unable` padded into the mirror's word column, then NAME, then `reason  detail` (`not-delivered  turn-running`), on stderr, appended under the TTY-only `[  ] open` issue line and never redrawn (per layout-templates §Surface: cli / Output structure — `viola send`; §Component — Hero / signature output line).
- Exit code is the typed tail: `not-delivered` is exit 13; the outcome line plus its hint is the whole human output, with no terminator word (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column; §Component — Footer / terminator).
- A refusal line takes a `hint:` line directly under it, keyed by reason · detail, as the last stderr line; every cause gets its own hint so a driver can tell `turn-running` apart from `input-not-ready` and the other exit-13 details from the last line alone. The hint never quotes the sent text or upstream text and never names a path or pid (per layout-templates §Component — Primary content block 2). Whether a `turn-running`-keyed hint already exists in the cli rendering (it was reachable before, for the in-flight slot) is research's question.
- The hint points to the next verb by name — the scope's driver contract is "call `wait` first", i.e. the `viola wait <name>` breadcrumb form shown for `input-not-ready` (per layout-templates §Component — Primary navigation (verb structure), Discoverability; §Output structure — `viola send`).
- Under `--json` the refusal is exactly one typed document on stdout (`refusal` + `detail`), with the typed exit code, no glyph, no stderr line and no hint (per layout-templates §Component — Hero / signature output line, Machine view; §Component — Primary content block 2, exit codes).
- Streams never mix: the issue line, refusal and hint go to stderr; results to stdout (per layout-templates §Component — Primary content block 2, Streams).
- `turn-ended` is never reported as a send outcome — the running-turn bookkeeping must not leak a `turn-ended` into `send`'s output; that is `viola wait`'s result (per layout-templates §Component — Hero / signature output line, What never happens).

## Patterns to follow
- The existing `input-not-ready` struck mirror + `hint: <name> was not ready for input; viola wait <name>, then send again` pair is the template for a `turn-running` line and hint (per layout-templates §Output structure — `viola send`).
- `viola wait` / `viola last` result lines (`turn-ended  builder  …  cursor …`, `session-end  builder  …`) are the human form of the turn-end kinds this chunk consumes; nothing in this chunk changes them (per layout-templates §Output structure — `viola wait` / `viola last`).
- `viola release` keeps its spoken phraseology `builder  wheel driver  you have control`; clearing the running-turn state adds no human field or line to it (per layout-templates §Output structure — wheel, handoff and dialog verbs).
- On the web, a `send-refused` strikes the readback box with `unable · not-delivered · turn-running` in both placements; `turn-ended` never touches the box (per layout-templates §Component — Hero / signature section, Trigger events) — target state only; the web page is not built by this chunk.

## Anti-patterns to avoid
- No hint names `viola release`, even though `release` is what clears a running turn left by a human interrupt: refusal hints reach drivers, and a driver's `release` is refused (`-32602`, exit 20) (per layout-templates §Component — Primary navigation (verb structure), Discoverability).
- No new human column, word or colour for the running-turn state: no spinner, no `busy` colouring, no `Sending…`; colour on the cli is reserved for `DIALOG` and `stale`, and a new field goes into `--json` first because the six-column parity with the web strip is a design contract (per layout-templates §Component — Hero / signature output line, Colour; §IA notes, Output as a contract).

## Contract bindings
- layouts ↔ architecture §Conventions (the `send` refusal order and the `not-delivered` / `turn-running` detail code): the cli line prints whatever reason · detail the wrapper returns; the hint table is keyed on that pair (per layout-templates §Component — Primary content block 2).
- layouts ↔ architecture §Standard Contracts (snapshot): if P3 puts the running-turn state on `snapshot.json`, the `viola list` STATUS column and the web strip STATUS cell keep their existing words and six-column parity (`viola list --json` item shape identical to `/api/sessions`); a new reading enters `--json` first (per layout-templates §Output structure — `viola list --json`; §IA notes, Multi-surface coordination).
- layouts ↔ design-system cli pattern 2 (one hint per cause, the last stderr line) (per layout-templates §Component — Primary content block 2).

## Acceptance criteria contributions
- (layouts) `viola send <name>` refused for a running turn prints, on stderr, `[/ ] unable` padded to the mirror's word column, then `<name>  not-delivered  turn-running`, exits 13, and its last stderr line is a `hint:` naming `viola wait <name>` and not `viola release` (per layout-templates §Output structure — `viola send`; §Component — Primary navigation (verb structure)).
- (layouts) The `turn-running` hint text differs from the `input-not-ready` hint, so the last stderr line alone tells the two exit-13 causes apart (per layout-templates §Component — Primary content block 2, Hint line).
- (layouts) `viola send <name> --json` refused for a running turn prints one `refusal` document with detail `turn-running` on stdout, nothing on stderr, exit 13 (per layout-templates §Component — Hero / signature output line, Machine view).
- (layouts) `viola release <name>` output stays `<name>  wheel driver  you have control` after the running-turn clear, and `viola list`'s six captions and columns are unchanged (per layout-templates §Output structure — wheel, handoff and dialog verbs; §IA notes, Output as a contract).
