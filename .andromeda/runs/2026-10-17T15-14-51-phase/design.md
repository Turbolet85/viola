# design extract

## Relevance
partial — the chunk adds one human CLI surface (the served `viola plugin install` verb, its `--help` row, its refusals) and one `viola verify` step line; no colour, tokens or page are in play (0.1.0 renders no page, per design-system §Surface Scale), and the plan holds no component pattern for `plugin install`'s own output, so its human line form is unspecified by this plan.

## Constraints
- design-system §Surface: cli → Navigation Pattern requires `plugin install` to be the one two-word verb in the flat verb list, and `viola --help`'s `setup:` row to read `run, revive, verify, plugin install` once the verb is served (served-verbs-only is recorded there as a delegate's PROVISIONAL answer, the founder's word owed); whether the as-built `HELP` literal already omits it until served is research's question.
- design-system §Surface: cli → Navigation Pattern requires the hidden `mcp` verb to stay out of every `viola --help` row, so the server the installed plugin's `.mcp.json` registers adds no help row of its own.
- design-system §Surface: cli → Colour decision order requires `plugin install`'s human output to carry no SGR at all: only `DIALOG` and `stale` rows are ever styled, so the verb's result and refusal lines are plain at every depth, and under `--json`, a pipe, `NO_COLOR` or `TERM=dumb` print no colour, glyph or cursor control.
- design-system §Surface: cli → Streams requires the verb's result on stdout and any refusal (`unable…`), its one `hint:` line and errors on stderr; under `--json` the whole result, refusal or `error` object included, is one stdout document with the typed exit code and no `hint:` line.
- design-system §Surface: cli → Width requires human output to be ASCII only and never wrapped (no non-ASCII path arrows or check glyphs in the install line).
- design-system §Surface: cli → Component Patterns (5. `viola verify` / `viola run`) requires a new relied ledger row to print one static `[NN/MM] <row-id> <description>  pass|fail` step line in `LedgerRow::ALL` order, with `MM` equal to `LedgerRow::relied()`'s count and the closing `stamped <ver>  <n> pass  <m> fail` summary summing to `MM`; the row's place in the list and its one-line description are owed by this chunk ("the count grows as owning chunks land their rows").
- design-system §Surface: cli → Exit-code phraseology requires any exit-1 refusal of the new verb to take a typed form from this table (a fixed `unable: <text>` line plus its hint, or `error: internal error` with no hint), and a `--json` exit 1 to print a document only where arch names one; the verb has no row in the table today, so its refusal causes and wording are a planning gap.

## Patterns to follow
- design-system §Surface: cli → Component Patterns (4. Wheel, link and gate verbs): a one-shot setup verb's success is one unstyled stdout line of fixed phraseology words with two-space gutters (e.g. `builder  wheel human  manual-pause  I have control`), saying what was done rather than "done".
- design-system §Surface: cli → Component Patterns (2. `viola send`, the start-refusal list): each exit-1 cause gets one fixed-message `unable: …` line and exactly one `hint:` line, neither showing a path or a pid.
- design-system §Surface: cli → Component Patterns (5. `viola verify`): the step counter is the project's one progress pattern; a new ledger row extends it in place, never with a separate progress display.
- design-system §Surface: cli → Platform-Specific Notes: `mcp` has no human design surface (stdout MCP frames only), so the installed plugin's server path is judged on frame discipline, not on any printed line.

## Anti-patterns to avoid
- design-system §Anti-Patterns → Per-Surface Bans (cli): never print "done" / "success" without context, and never a green `✓` / red `✗` prefix or emoji on the install result.
- design-system §Anti-Patterns → Per-Surface Bans (cli): never print paths, pids or upstream text in errors or hints — binding on `plugin install`, whose natural output (the marketplace directory, the absolute exe written into `.mcp.json`) is a path; whether its human success line may name the marketplace path or the CLI's own `claude plugin …` steps is a ruling the plan does not make (the ban as written covers errors and hints only).
- design-system §Anti-Patterns → Per-Surface Bans (cli): never a spinner or progress bar while the verb writes the marketplace or (if a spawn is chosen) waits on the CLI.

## Contract bindings
- design ↔ security: §Anti-Patterns → Per-Surface Bans (cli) "no paths, pids or upstream text in errors or hints" is the security plan's NEVER-log floor and error sanitization; the install line's path question binds to security-plan §Error Handling.
- design ↔ architecture: §Surface: cli → Exit-code phraseology defers `--json` document shapes to arch §Standard Contracts; the new verb's `--json` `ok` / `error` shape is arch's to name.
- design ↔ tests: §Surface: cli → Navigation Pattern's `setup:` row binds to the `HELP` literal of `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs` (`tests/cli_output_plain.rs`), per the chunk scope's CARRY.
- design ↔ a11y: the plain-ASCII, colour-free output satisfies the cli not-colour-alone rule (a11y-plan cli landmarks) trivially; no token contrast binding arises.

## Acceptance criteria contributions
- (design) `viola --help` lists `plugin install` in the `setup:` row after `verify` and no `mcp` row, and the help output stays SGR-free (per design-system §Surface: cli → Navigation Pattern).
- (design) `viola plugin install`'s stdout and stderr contain no `\x1b` byte and only ASCII, on a TTY and piped, with or without `NO_COLOR` (per design-system §Surface: cli → Colour decision order).
- (design) Under `--json`, `viola plugin install` prints exactly one stdout document with no `hint:` line, and in human mode no refusal or hint line holds an absolute path or a pid (per design-system §Surface: cli → Streams).
- (design) `viola verify` prints one `[NN/MM]` step line for the new ledger row with `MM` equal to the relied count, and its summary's pass + fail equals `MM` (per design-system §Surface: cli → Component Patterns, 5. `viola verify`).
