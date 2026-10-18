# layouts extract

## Relevance
partial — the chunk adds the cli surface's one two-word verb and moves the `--help` table and `viola verify`'s step count, but the plan gives no wireframe for `viola plugin install`'s own output (its success and refusal lines are a gap for P3 / plan to settle, not a layout given).

## Constraints
- `plugin install` is the one two-word verb in an otherwise flat verb set: no nested noun hierarchy, no interactive prompt, no TUI mode, no pager (per layout-templates §Component — Primary navigation (verb structure)).
- The global `--home` is the same on every verb, and `--json` is the machine view of every data verb; whether `plugin install` counts as a data verb taking `--json` is a plan decision the scope already leans to ("the CLI's `--json` / exit-code contract like every other verb") (per layout-templates §Component — Primary navigation (verb structure)).
- There are no subcommand trees beyond `plugin install`, so the verb adds no further sub-verbs (an uninstall or list arm would be a second tree) (per layout-templates §IA notes).
- `viola --help`'s `setup:` row reads `run, revive, verify, plugin install` in the wireframe; as built it lists served verbs only, and `plugin install` joins that row when this entry serves the verb, with the `HELP` literal in `tests/cli_output_plain.rs` moving with it; the served-verbs-only rule is PROVISIONAL (the Viola overseer's answer, the founder's word owed) (per layout-templates §Output structure — `viola --help`).
- The same served-verbs rule is restated on the primary-screens list: the `--help` grouped table's `plugin install` joins when "Optional plugin install" serves it (per layout-templates §Primary screens (commands)).
- No banner, tagline or version line on normal output; the only header on any verb is `viola list`'s BAY line, so `plugin install` prints none (per layout-templates §Component — Header / banner (the BAY context line)).
- `viola verify` prints one `[NN/MM]` step line per relied ledger row in ledger order, `MM` being `LedgerRow::relied()`'s length, which "grows as owning chunks land their rows": a new precedence row adds a step line, moves `MM` and the summary counts, and its row id and words take their ledger position in that order (per layout-templates §Output structure — `viola verify`).

## Patterns to follow
- Refusals as `unable: <text>` plus its own `hint:` line, the hint last on stderr, no name, path or pid (the exit-1 fixed-message form used by `run`, `revive` and `verify`); a failed or panicked verb prints `error: internal error` once with no hint; under `--json` one stdout document with stderr empty (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Footer: no terminator word (`done.`, `success`), the exit code is the terminator and the last printed line says what happened, to whom and where (per layout-templates §Component — Footer / terminator).
- Streams: results to stdout; refusals, hints and errors to stderr, never mixed (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Expression level 0.0 for `--json` and non-TTY (no colour, no non-ASCII glyph, no cursor control); colour on the cli is reserved for `DIALOG` and `stale`, so `plugin install` output is plain ASCII (per layout-templates §Surface: cli).
- `hint:` lines are the CLI's breadcrumbs, naming the next verb by name; if `plugin install` prints next steps (e.g. the CLI's own `claude plugin …` commands, a scope premise), they follow this plain-instruction form (per layout-templates §Component — Primary navigation (verb structure)).

## Anti-patterns to avoid
- No spinner, progress bar, `✓` or redraw; lines are appended once (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror)).
- No hint or help text naming `viola release` (per layout-templates §Component — Primary navigation (verb structure)).
- No absolute path in a human refusal or hint line, even though the verb writes an absolute exe path into `.mcp.json`; whether a success line may print the marketplace path is unruled by this plan (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).

## Contract bindings
- layouts ↔ tests: the `--help` `setup:` row binds to the `HELP` literal of `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` in `tests/cli_output_plain.rs` (per layout-templates §Output structure — `viola --help`).
- layouts ↔ architecture: the verify step list's row ids and order bind to `LedgerRow::ALL` / `LedgerRow::relied()` in `crates/viola-agent-claude/src/ledger.rs`; whether the new row is relied (and on which OS) sets `MM` (per layout-templates §Output structure — `viola verify`).
- layouts ↔ a11y / design: plain-ASCII, no-SGR output binds to the cli expression level (per layout-templates §Surface: cli).

## Acceptance criteria contributions
- (layouts) `viola --help` prints `setup:    run, revive, verify, plugin install` and the `HELP` literal in `tests/cli_output_plain.rs` pins it; no other row changes (per layout-templates §Output structure — `viola --help`).
- (layouts) `viola plugin install` is reachable only as the two-word verb under the global `--home`, with no further sub-verb and no interactive prompt (per layout-templates §Component — Primary navigation (verb structure)).
- (layouts) Every `plugin install` refusal ends with a `hint:` line last on stderr, names no path or pid, and under `--json` is one stdout document with stderr empty (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- (layouts) If a precedence ledger row lands, `viola verify` prints one more `[NN/MM]` step line at the row's ledger position and the `stamped <ver>  <n> pass  <m> fail` summary counts it (per layout-templates §Output structure — `viola verify`).
