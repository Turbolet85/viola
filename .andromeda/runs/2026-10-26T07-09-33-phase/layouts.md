# layouts extract

## Relevance
partial — the chunk adds no surface, screen, column or component; it adds a new cause behind the existing exit-21 refusal output of the CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`), and touches two verbs that have no human surface (`viola hook`, `viola mcp`).

## Constraints
- A CLI verb whose server verification fails or cannot be made reports through the existing exit-21 form, `unable  <name>  instance-unreachable` on stderr with its `hint:` line as the last stderr line, and adds no new line shape (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Exit codes as the typed tail").
- `viola send`'s exit 21 keeps its `[/ ] unable` readback-mirror line with the same hint as `wait` / `last`, the word padded into the mirror's word column; the mirror stays uncoloured (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror)).
- `layout-templates §Component — Primary content block 2` also says every cause of exit 21 gets its own hint, so an agent can tell the causes apart from the last stderr line. A verification mismatch is a new cause of exit 21. The plan does not say whether it reuses the "not running" hint or needs a distinct one. P4 decides that, and a distinct hint is a design change (see the IA notes constraint below), not an implementation choice (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Hint line").
- A hint never names a path, a pid or `viola release`, and never quotes upstream text. A verification refusal must not print the snapshot's recorded pid, the peer pid or the endpoint (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Hint line"; per layout-templates §Component — Primary navigation (verb structure), "Discoverability").
- Under `--json` the exit-21 outcome is one document on stdout with no stderr line and no hint, keeping `"detail":null` for exit 21 until arch names the detail codes (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Exit codes as the typed tail"; per layout-templates §Output structure — `viola wait` / `viola last`).
- Streams stay split: refusals and hints go to stderr, results to stdout, never mixed (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Streams").
- `viola hook` and `viola mcp` have no human surface. A mismatch adds no human line to either. The MCP tools' refusal stays the machine view: `isError: true`, text `refused: <code>`, codes only (per layout-templates §Component — Header / banner (the BAY context line), "No banner anywhere"; per layout-templates §Primary screens (commands), the `viola mcp` row).

## Patterns to follow
- The `wait` / `last` exit-21 pair as the wireframe shows it: `unable  builder  instance-unreachable` then `hint: builder is not running; viola list shows the live instances` (per layout-templates §Output structure — `viola wait` / `viola last`). Whether `answer`, `pause` and `release` print the same pair at exit 21 today is research's question.
- `viola send`'s struck mirror line `[/ ] unable         <name>  <reason>  <detail>`, with the hint last on stderr (per layout-templates §Output structure — `viola send`).
- The one `cli` catch site's `error: internal error` (exit 1, no hint) handles failures and panics only. A verification that cannot be made takes exit 21 per the scope, so it must not fall through to that catch site (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Line form").
- If this chunk takes the strict-modes half into the frame verbs, the existing fixed pairs are the precedent: `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`, and the home pair (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Line form"). Which pair a frame verb would print is P4's to decide, together with the strict-modes boundary question.

## Anti-patterns to avoid
- No new human column, word or line form for a verification outcome without a design change: human output is a contract that LLM drivers read, and new fields go to `--json` first (per layout-templates §IA notes, "Output as a contract").
- No spinner, retry counter or redraw while the client verifies the server: lines are appended once (per layout-templates §Output structure — `viola wait` / `viola last`; per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror), "What never happens").
- No colour on any refusal: colour is reserved for `DIALOG` and `stale` (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror), "Colour").

## Contract bindings
- The exit-21 `--json` document (`"detail":null`) binds to architecture §Conventions (exit codes, refusals) and §Standard Contracts. A detail code for a verification mismatch would land in arch first, and the layout follows it (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Exit codes as the typed tail").
- The MCP tools' `instance-unreachable` result is the CLI verb's `--json` document less `v`, so it binds to security-plan §Error Handling (codes only, text `refused: <code>`) (per layout-templates §Primary screens (commands), the `viola mcp` row).
- `viola hook`'s silent fail-open binds to the universal invariant in CLAUDE.md (exit 0, nothing on stderr). Layouts adds only that the hook has no human surface (per layout-templates §Component — Header / banner (the BAY context line)).

## Acceptance criteria contributions
- (layouts) When `wait` or `last` connects to an endpoint whose server pid or start time does not match `snapshot.json`, it prints `unable  <name>  instance-unreachable` and then its `hint:` line as the last stderr line, with stdout empty and exit 21. The output holds no path and no pid (per layout-templates §Output structure — `viola wait` / `viola last`).
- (layouts) When `viola send` hits the same mismatch, it prints the `[/ ] unable  <name>  instance-unreachable` line with the same hint as the last stderr line, and exit 21 (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Hint line").
- (layouts) Under `--json`, each frame verb on a mismatch prints exactly one stdout document with `"detail":null` and an empty stderr, with exit 21 (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column, "Exit codes as the typed tail").
- (layouts) When an MCP tool hits a mismatch, it returns `isError: true` with the text `refused: instance-unreachable` and a body of codes only (per layout-templates §Primary screens (commands), the `viola mcp` row).
