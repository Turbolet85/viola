# layouts extract

## Relevance
partial — the chunk builds no web-spa element and no new verb. Its layout surface is the cli: `viola verify`'s step lines and summary grow with the new ledger rows, and `viola run`'s passthrough must stay unchanged while the vt100 feed taps it.

## Constraints
- `viola verify` prints one stdout step line per ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order. `MM` grows as owning chunks land their rows. So each new row (the input-box signature, the modal signature set, quiet period / maximum wait, and the confirmation-window row if it gets a `verify` step) adds exactly one step line at its ledger position and raises `MM` (per layout-templates §Surface: cli › Output structure — `viola verify`).
- The `stamped <ver>  <n> pass  <m> fail` summary stays the last stdout line. Exit 0 needs every row to pass. Exit 1 follows a failing row (the stamp is still written) or a refusal. `--record` adds no human line. The output is plain ASCII with no colour, and each line is appended once, with no progress bar and no redraw. The interactive typed-input probe's longer runtime does not license a spinner or a redraw (per layout-templates §Surface: cli › Output structure — `viola verify`; §Component — Footer / terminator).
- `viola verify` refusals are a closed set of fixed `unable: <text>` / `hint: <text>` pairs that name no path, pid or upstream text. A failed or panicked `verify` prints exactly `error: internal error`, with no hint. If the PTY-driven probe brings a new start-refusal cause, it needs its own fixed pair. A probe whose gate verdict is `input-not-ready` is a row `fail`, not free text (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- `viola run <name> -- claude` is passthrough and prints nothing while the child runs. The terminal belongs to the wrapped TUI. At expression level 0.0 it has no colour, no non-ASCII glyph and no cursor control of viola's own. So the screen-model feed, the gate verdict and a vt100 parser-panic degrade must add no byte to the human terminal (per layout-templates §Surface: cli › Output structure — `viola run`; §Surface: cli Expression level).
- Streams stay separate: results go to stdout, and refusals, hints and errors go to stderr. Every new verify step line goes to stdout and every new refusal to stderr (per layout-templates §Component — Primary content block 2 › Streams).
- Human output words are a stable contract that LLM drivers may read. New row ids and their row words become part of that contract once printed (per layout-templates §IA notes › Output as a contract).

## Patterns to follow
- The existing six-row `viola verify` step-line form (`[01/06] shim-resolution claude resolves to a real executable  pass`). New rows reuse the same `<row id> <row words>` shape and the two-space gap before `pass|fail` (per layout-templates §Output structure — `viola verify`).
- The fixed `unable:` / `hint:` pair form of the existing verify refusals, for example `the claude CLI was not found` / `install Claude Code or put it on PATH` (per layout-templates §Component — Primary content block 2).
- The refusal vocabulary `not-delivered  input-not-ready` that `viola send` later renders as `[/ ] unable  <name>  not-delivered  input-not-ready` with the hint `<name> was not ready for input; viola wait <name>, then send again`. This chunk's gate verdict must produce exactly that reason/detail pair for `:74` to render. This chunk does not render it (per layout-templates §Output structure — `viola send`; §Component — Hero / signature output line).

## Anti-patterns to avoid
- No spinner, progress bar, elapsed-time counter, `Sending…`/`done` or redraw-in-place, including while the typed-input probe waits on the quiet period and the maximum wait (per layout-templates §Component — Hero / signature output line › What never happens; §Output structure — `viola wait` / `viola last`).
- No viola-originated byte, banner or status line on the `viola run` terminal, including on gate failure or parser panic (per layout-templates §Component — Header / banner › No banner anywhere).
- No screen content, signature text or upstream text in any human line or hint (per layout-templates §Component — Primary content block 2 › Hint line).

## Contract bindings
- cli `viola verify` step order ↔ `viola_agent_claude::ledger` row order. The `[NN/MM]` numbering is the ledger order, so where the new rows sit in the closed set fixes their printed position. Whether the code derives `MM` from the ledger or from a literal is research's question.
- cli `viola run` passthrough ↔ obs-plan §11 / test-plan R7: screen content is never logged or asserted, and on the human surface it is never echoed back either. The only output the gate produces is its verdict.
- cli refusal reason/detail `not-delivered` / `input-not-ready` ↔ architecture §Conventions (refusal and exit-code table: exit 13) and the `:74` Confirmed-send chunk that renders the `[/ ]` line.

## Acceptance criteria contributions
- (layouts) `viola verify` prints one `[NN/MM] <row id> <row words>  pass|fail` stdout line per ledger row. These include each new signature and timing row in ledger order, `MM` equals the new ledger row count, and `stamped <ver>  <n> pass  <m> fail` is the last stdout line, with `n + m = MM` (per layout-templates §Output structure — `viola verify`).
- (layouts) `viola verify` output, including the typed-input probe's steps, has no ANSI/SGR or cursor-control byte, no non-ASCII glyph and no redrawn line. Each line is appended once (per layout-templates §Output structure — `viola verify`; §Surface: cli Expression level).
- (layouts) During `viola run`, the bytes reaching the human terminal equal the child's PTY output byte for byte, including after a forced vt100 parser panic (the `--vt100-panic-bytes` path). viola adds no byte of its own (per layout-templates §Output structure — `viola run`).
- (layouts) Any new `viola verify` refusal is one fixed `unable: <text>` line plus one `hint: <text>` line on stderr, with exit 1 and no path, pid or upstream text. A probe panic prints exactly `error: internal error` with no hint (per layout-templates §Component — Primary content block 2).
