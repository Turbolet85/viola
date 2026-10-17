# layouts extract

## Relevance
partial — the chunk builds no surface. Its harness legs read cli surfaces this plan fixes: `viola list --json` (the `status` member, the Path 6 reads), the board's budget cells (CARRY 3's "the board's cell"), the `viola run` duplicate-start refusal (Path 1 step 3), `send`'s exit-11 refusal and `release --budget`.

## Constraints
- layout-templates §Output structure — `viola list --json` requires one JSON document on stdout with no header line, colour or glyph. A strict-modes failure prints instead `{"v":1,"error":"instance-unreachable","detail":null}` with exit 21 and an empty stderr. `status`, which nests the document verbatim as `list` and derives `down` / `list-exit-21`, reads exactly this shape. Whether the harness keeps the bytes verbatim is research's question.
- layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) (As built) requires the uncoloured form on every stream, a TTY included, written as one stdout write. The unwrapped rack (`-- UNWRAPPED - READ-ONLY --` and its rows: NAME the `sessionId`, LIVE `live`, `n/a` in WHEEL / DIALOG / CLI) prints only when the agents read lists an unjoined row. A failed, oversize or malformed read prints the board as if no `claude` were present. That makes `boot --agents-mode recorded|oversize|malformed` the switch between a board with the rack and one without.
- layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) (As built) also requires a closed budget gate to show in the WHEEL cell as `<holder> - budget-paused`, padded to 23. STATUS reads `idle` / `busy` only on a joined wrapped row, else `unknown`. Under the harness `PATH` the fake agent answers the agents read, so a booted instance's STATUS depends on that join.
- layout-templates §Component — Header / banner (the BAY context line) requires the BAY line's gate cell to read `gate open` or `budget-paused`, as stated in §Output structure — `viola list` (human TTY). It is the only header on any verb, with no bold or colour.
- layout-templates §Output structure — `viola run` requires a passthrough start to print nothing. A duplicate start prints `unable: builder is already live` then `hint: viola list` on stderr, exit 1, with no paths and no pids. This is the Path 1 duplicate-`run` step.
- layout-templates §Component — Primary content block 2 requires exit 11 to carry `budget-paused`. Under `--json` the same outcome is one document on stdout with no stderr line and no hint text. stdout carries results and stderr carries refusals and hints, and the two never mix.
- layout-templates §Output structure — wheel, handoff and dialog verbs requires `viola release <name> --budget` to print one of two stdout lines with exit 0 and the wheel left where it is: `budget gate lifted until <end>  wheel <holder>` when a pause stood, `budget gate open  wheel <holder>` when none stood. The Path 6 leg calls the `--json` form only.

## Patterns to follow
- Read machine state from `--json`, never from human columns: per layout-templates §IA notes ("Output as a contract"), new fields go to `--json` first. The `status` derivation (`liveness`) and the Path 6 signals (`.ok.budget.paused`, each item's `budget_paused`) read the `list` document. The human board is asserted only for its budget cells.
- Assert the board's cell from the as-built uncoloured characters, per layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`): the same words with no SGR, nothing truncated, no line ending in a space.
- Read a refusal by its last stderr line: per layout-templates §Component — Footer / terminator, the `hint:` line is the last line on stderr, and the exit code is the terminator.

## Anti-patterns to avoid
- Do not assert the styled human-TTY form (bold NAME, amber `DIALOG`, dim stale row, `...` truncation). Per layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) (As built) it is not built and is owed to "CLI output discipline".
- Do not add or rename a human column or word to make a leg easier to assert. Per layout-templates §IA notes, adding a human column is a design change, not an implementation choice.
- Do not let a harness hint, document or driver-facing message suggest `viola release`. Per layout-templates §Component — Primary navigation, `release` is a human verb, and the Path 6 leg's `release --budget` is the test playing the human.

## Contract bindings
- `status` ↔ list payload: the `list` member the harness nests is the `list` payload contract (architecture §Standard Contracts, cited by layout-templates §Output structure — `viola list --json`), bound to test-plan §3 Status endpoint shape.
- Board budget cell ↔ design-system: the `release --budget` line forms tie to design-system §Surface: cli Component Patterns 4 (cited by layout-templates §Output structure — wheel, handoff and dialog verbs). The WHEEL `- budget-paused` planning forms tie to design-system cli pattern 1 (cited by layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) (As built)).

## Acceptance criteria contributions
- Path 1's duplicate `run` leaves stdout empty, ends stderr with `hint: viola list` right after `unable: builder is already live`, and exits 1 (per layout-templates §Output structure — `viola run`).
- `status`'s `list` member is byte-equal to one `viola list --json` stdout document. On a list exit 21 the document read is `{"v":1,"error":"instance-unreachable","detail":null}` with an empty stderr (per layout-templates §Output structure — `viola list --json`).
- After Path 6's two-instance leg, the uncoloured `viola list` board shows `overseer`'s WHEEL cell as `<holder> - budget-paused` and `builder`'s without `budget-paused`. The BAY gate cell reads `budget-paused` while `.ok.budget.paused` is `true` (per layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) and §Component — Header / banner). Whether the BAY cell follows the budget reading or the per-instance pause is research's question.
- If the leg's `send` to `overseer` uses `--json`, it exits 11 with one stdout document and an empty stderr. Without `--json`, the `hint:` line is the last stderr line (per layout-templates §Component — Primary content block 2).
