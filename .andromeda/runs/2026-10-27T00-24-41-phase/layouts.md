# layouts extract

## Relevance
partial — the chunk adds no screen. Its only layout contact is the cli surface's existing `viola run` and `viola verify` output, and their refusal lines, which the pin-race fix (part A) must leave as they are. Part B is a build measurement with no surface.

## Constraints
- `viola run` is passthrough and prints nothing of its own while the child runs. Its expression level is 0.0: no colour, no non-ASCII glyph, no cursor control. A wait or retry added to the absent-copy pin write therefore prints no line, no spinner and no progress text (per layout-templates §Surface: cli, Primary screens (commands) and Expression level).
- `viola run`'s start refusals are exit 1, with one `unable: <text>` line and one `hint:` line last on stderr, and no name, path or pid. The pinned-copy refusal is one of these fixed-message exceptions. Whether the race fix keeps that refusal's existing text, or needs a new cause, is research's question. A new cause would need its own distinct hint (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Every exit-1 cause carries its own hint, so that an agent can tell the causes apart from the last stderr line. A hint never names a path or a pid. A concurrent first start that loses the pin race must not read as the pinned-hash-mismatch refusal unless it really is one (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, Hint line).
- `viola verify` prints one stdout step line per relied ledger row and then the `stamped <ver>  <n> pass  <m> fail` summary as its last stdout line. The output is plain ASCII, each line appended once, with no redraw. Its `pin_exe` path adds no human line, and the chunk adds no ledger row, so `MM` stays at 24 on Unix and 21 on Windows (per layout-templates §Output structure — `viola verify`).
- A failed exe read, directory creation or file write in a `cli` verb prints exactly `error: internal error` once, from the one catch site, with no hint. A pin write that still fails after the fix surfaces only through that line, never through a new message. `verify` is in the `cli` list. The `run` start's own failure path is research's question (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- The human columns and words are a stable contract that LLM drivers read. The chunk adds no human line or word, and any new field goes into `--json` first (per layout-templates §IA notes, Output as a contract).

## Patterns to follow
- Silent degradation without a terminal byte: the passthrough start writes nothing to the terminal it hands to the child (per layout-templates §Output structure — `viola run`).
- Refusal pair form: `unable: <fixed text>` and then `hint: <one plain instruction>` as the last stderr line, with stdout empty (per layout-templates §Output structure — `viola run` and §Component — Footer / terminator).
- The exit code is the terminator: no `done.`, no success word, no summary banner on `run` (per layout-templates §Component — Footer / terminator).

## Anti-patterns to avoid
- No progress, spinner or "waiting for pinned copy" line during the start's pin write. Motion tokens are "None. Nothing redraws in place" (per layout-templates §Surface: cli token table, Motion tokens).
- No refusal or diagnostic text on stdout or stderr that names `bin/<key>/`, the pinned path, a hash prefix or a pid (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, Hint line).

## Contract bindings
- layouts ↔ tests: the plain-ASCII and empty-output pins of the cli surface, such as the `viola run` passthrough printing nothing, bind to the test harness that reads stdout and stderr byte for byte. Whether an existing test pins `run`'s empty stderr on a concurrent first start is research's question (per layout-templates §Surface: cli, Expression level).

## Acceptance criteria contributions
- (layouts) On two concurrent first `viola run`s of one version on a fresh home, neither start writes a byte of its own to the terminal before the child holds it. Neither prints the pinned-copy refusal pair unless the copy truly differs (per layout-templates §Output structure — `viola run`).
- (layouts) Any refusal the fix adds or keeps on the `run` start is one `unable:` line followed by one distinct `hint:` line as the last stderr line, with no path, pid or hash, and exit 1 (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- (layouts) `viola verify`'s stdout stays one step line per relied row, followed by the `stamped` summary, with `MM` unchanged at 24 on Unix and 21 on Windows and no added line from its `pin_exe` path (per layout-templates §Output structure — `viola verify`).
