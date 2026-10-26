# layouts extract

## Relevance
partial — the chunk is IPC/security work in `viola-channel` with no new human surface; its one layout touch is the existing `viola run` exit-1 start refusal (`squatted-name`), which scope.md says may be touched in `src/cmd/run.rs` and must keep its form.

## Constraints
- `viola run` is passthrough: it prints nothing of its own while the child holds the terminal, and a start refusal is the only output it has (per layout-templates §Output structure — `viola run`).
- A `viola run` start refusal is a fixed-message `unable: <text>` line on stderr followed by its own `hint:` line, exit 1, with no name-independent detail beyond the fixed text and never a path or a pid; the squatted-name cause is one of the "sibling causes" of `unable: <name> is already live` (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Every exit-1 cause gets its own hint, so a driver can tell the causes apart from the last stderr line; the hint never names a path or a pid and never quotes upstream text (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- On a refusal the `hint:` line is the last line on stderr; there is no terminator word or summary banner (per layout-templates §Component — Footer / terminator).
- Expression level 0.0 for the `viola run` passthrough: no colour, no non-ASCII glyph, no cursor control on anything `run` prints (per layout-templates §Surface: cli).
- Results go to stdout and refusals/hints to stderr, never mixed; a run-start refusal leaves stdout empty (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- A `run` start refusal made after `revive`'s passed preflight (`squatted-name` among them) prints no `--json` document as landed — that document is owed to the working-route entry "Exit-cause code catalogue", not this chunk (per layout-templates §Output structure — `viola revive`).

## Patterns to follow
- The `viola run` wireframe pair (`unable: builder is already live` / `hint: viola list`, stderr, exit 1) is the shape any squatted-name line follows; whether the code's `squatted-name` pair already matches it byte for byte is research's question (per layout-templates §Output structure — `viola run`).
- The fixed-message exception list (run's start refusals, revive's pairs, verify's pairs, list's and plugin install's strict-modes pairs) is where a run-start cause's wording is anchored; design-system cli pattern 2 holds the sibling causes' text (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Output as a contract: human words and lines are stable because LLM drivers read them, so an asserted-but-unchanged refusal stays byte-identical (per layout-templates §IA notes).

## Anti-patterns to avoid
- No new human line, banner, progress or diagnostic text from `viola run` or the channel listener at start (e.g. a "pipe admitted"/"remote client rejected" message): `viola run` prints nothing, and there is no banner on any verb (per layout-templates §Component — Header / banner (the BAY context line)).
- No pipe name, SID, SDDL text, path or pid in any refusal or hint line the chunk touches (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).

## Contract bindings
- layouts ↔ design: the squatted-name pair's exact wording is design-system cli pattern 2's (layout-templates cites it; design owns the text).
- layouts ↔ architecture §Conventions: `squatted-name`, exit 1, stands unchanged per scope.md; its `--json` document binds to the "Exit-cause code catalogue" entry, not this chunk.
- layouts ↔ security: the "no paths, no pids" line rule coincides with security-plan §Error Handling's fixed-message rule.

## Acceptance criteria contributions
- (layouts) A `viola run` start on a squatted pipe name prints exactly one `unable:` line and one `hint:` line on stderr, the hint last, stdout empty, exit 1, and its bytes are unchanged from HEAD (per layout-templates §Output structure — `viola run`).
- (layouts) No line the chunk adds or touches holds a path, pipe name, SID or pid, and `viola run` prints no new line on a successful start (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
