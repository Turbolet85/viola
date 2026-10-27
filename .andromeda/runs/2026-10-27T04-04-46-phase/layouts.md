# layouts extract

## Relevance
partial — the chunk adds no surface, region or focusable element; it touches only the cli surface's fixed refusal lines, its escaping of upstream-derived cells and its handling of a bounded read that fails, which the bounds work must keep byte-stable (the web-spa surface is deferred to 0.2.0 and out of scope).

## Constraints
- The two refusals `viola send` makes before any frame take the struck-mirror line `[/ ] unable  <name>  not-delivered  <detail>` (`control-character` or `empty-text`) on stderr, exit 13, with the hint as the last stderr line, and under `--json` one stdout document with no stderr line; the wrapper must make the same two refusals for a frame sent straight to it (per layout-templates §Surface: cli › Component — Primary content block 2: refusal lines and the `unable` column). Whether every client (CLI `send`, MCP `send`) and the wrapper already render this as specified is research's question.
- Every field derived from `claude agents --json` must have C0/C1 controls escaped as hex text, `\n` and `\t` included, so that a row never splits and the columns stay fixed-width; the row escape `escape_row` must cover every control character, DEL and C1 (per layout-templates §Surface: cli › Component — Primary content block 1: `viola list` strip rows).
- A failed, oversize or malformed `claude agents --json` read must print the board as if no `claude` were present: no unwrapped rack and no separator (per layout-templates §Surface: cli › Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`), its As built paragraph).
- `<target>` is always a `ViolaName`, unwrapped names are never valid targets, and the argument order mirrors the channel `params` (`<driver> <driven>`, `<target> <dialog_id>`) (per layout-templates §Surface: cli › Component — Primary navigation (verb structure)).
- `viola last` prints the message on stdout with C0/C1 escaped, and `no message` on stderr for null (per layout-templates §Surface: cli › Output structure — `viola wait` / `viola last`).
- A malformed `viola revive --id` must be clap's usage error, exit 2, and no refusal line may hold the recorded directory, a pid or a logged id (per layout-templates §Surface: cli › Output structure — `viola revive`).

## Patterns to follow
- One row escape, `escape_row`, for every upstream-derived cell on the board. An unwrapped NAME is cut only on a terminal whose width is known, and never inside a `\xHH` escape (per layout-templates §Surface: cli › Component — Primary content block 1: `viola list` strip rows).
- Streams are never mixed: results go to stdout, and refusals, hints and errors go to stderr. Under `--json` the outcome is exactly one stdout document with an empty stderr (per layout-templates §Surface: cli › Component — Primary content block 2: refusal lines and the `unable` column).
- A failed or panicked `cli` verb prints exactly `error: internal error` once, from the one catch site, with no hint (per layout-templates §Surface: cli › Component — Primary content block 2: refusal lines and the `unable` column).

## Anti-patterns to avoid
- Adding a human column, word or refusal line form to report a new bound. New fields go into `--json` first, and the human columns and words are a stable contract (per layout-templates §Surface: cli › IA notes).
- A hint that quotes the sent text or any upstream text, names a path or a pid, or names `viola release` (per layout-templates §Surface: cli › Component — Primary content block 2: refusal lines and the `unable` column; §Surface: cli › Component — Primary navigation (verb structure)).

## Contract bindings
- The board's escaping is stricter than the security floor: it escapes `\n` and `\t` too, while `wait` / `last` text keeps them. This binds to security-plan §Input Validation (T5 ratified, per layout-templates §Surface: cli › Component — Primary content block 1: `viola list` strip rows).
- The `not-delivered` / `control-character` / `empty-text` lines and exit 13 bind to architecture §Conventions (refusals, exit codes) and to the `send` `--json` refusal document in architecture §Standard Contracts.
- The board's handling of an oversize or failed agents read binds to the `list` payload contract in architecture §Standard Contracts (`--json` items with no unwrapped rows).

## Acceptance criteria contributions
- (layouts) `viola send` with a text holding a refused C0 character, and with a text that is empty once its trailing CR/LF are removed, each print `[/ ] unable  <name>  not-delivered  control-character|empty-text` on stderr with the hint last, exit 13, empty stdout and no frame written. Under `--json` each prints one stdout document with an empty stderr (per layout-templates §Surface: cli › Component — Primary content block 2: refusal lines and the `unable` column).
- (layouts) An agents row whose `sessionId` holds `\n`, `\t`, DEL and a C1 character prints on one line with each one escaped as hex text, and the columns stay fixed-width (per layout-templates §Surface: cli › Component — Primary content block 1: `viola list` strip rows).
- (layouts) A `claude agents --json` output over the `MAX_FRAME` cap, or malformed, gives the board byte-identical to the one printed with no `claude`: no `-- UNWRAPPED - READ-ONLY --` line (per layout-templates §Surface: cli › Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)).
- (layouts) `viola revive <name> --id <malformed>` exits 2 as clap's usage error, and its output holds no logged id, path or pid (per layout-templates §Surface: cli › Output structure — `viola revive`).
