# design extract

## Relevance
partial — the chunk renders nothing new and touches no web-spa file, so no colour, type, spacing, radius or motion token is engaged; the design plan binds only as a preservation floor on the cli surface, where the scope's §1 lifts scaffolding out of tests that assert human output, §2 splits `record` (`src/cmd/verify.rs`) and names `sgr_attributes` (`tests/cli_output_plain.rs`), and §5 adds tests on `src/cmd/hook.rs` and `src/cmd/mod.rs`.

## Constraints
- design-system §Surface: cli → Component Patterns 5 (`viola verify` / `viola ui` / `viola run`) requires `verify`'s human output to be the static step-counter lines, one per ledger row, and one summary line, with the word `fail` uncoloured. The splits of `record` and `dialog_variants` must leave those bytes as they are; whether either function produces any of them is research's question.
- design-system §Surface: cli → Component Patterns (Exit-code phraseology, the exit 1 row) requires the `viola verify` record refusal to name the fixture file and one closed code, never the content, and to be followed by its own hint. If `record` is the function behind that refusal (research's question), its split keeps the line's fixed form and the closed code set.
- design-system §Surface: cli → Tokens (Streams) requires results on stdout and context lines, refusals, `hint:` lines and errors on stderr. The child-run-and-wait helper and the `Ran` runner lifted into `tests/support/` must keep the two streams as separate captures, so every stream-specific assertion still distinguishes them.
- design-system §Surface: cli → Tokens (Colour decision order) requires a piped, `--json`, `NO_COLOR` or `TERM=dumb` run to print no colour, no non-ASCII glyph and no SGR, while the ASCII readback mirror and `->` still print in human output. A shared runner must not change which of those modes a test's child runs in (piped stays piped, an outer-PTY run stays a TTY run); whether the per-file copies differ in environment or stdio set-up is research's question.
- design-system §Surface: cli → Tokens (the token table) requires colour to be amber on the `DIALOG` word only, SGR dim on a whole `stale` row, bold on NAME values, and a reset after every styled span. If P4 splits `sgr_attributes`, the split test must still pin that whole table; whether the function already reads every row of it is research's question.
- design-system §Surface: cli → Platform-Specific Notes requires `hook` to write nothing to stderr and always exit 0, with no human design surface. A test added for the survivor at `src/cmd/hook.rs:214:72` asserts inside that floor and introduces no printed line.
- design-system §Surface: cli → Component Patterns 2 (`viola send`: the readback mirror) requires the `[RB]` / `[  ]` / `[/ ]` mirror, its padded word column and the per-reason hint lines. The scaffolding lift in `tests/cli_send.rs` leaves every assertion on those lines unedited.

## Patterns to follow
- TTY-only context lines: design-system §Surface: cli → Component Patterns 2 and 3 mark the send issue line, `waiting:` and the `last` header as TTY-only stderr lines, so a shared helper keeps the piped runner and the outer-PTY runner as two distinct entry points rather than one with a hidden default.
- Phraseology as the assertion vocabulary: design-system §Brand Identity (domain anchor "Standard phraseology and unable plus a reason") fixes the short words tests match on; a lifted helper passes those words through as the callers wrote them and adds no wording of its own.
- Expression 0.0 for non-TTY output: design-system §Brand Identity (per-surface expression table, the `--json` / non-TTY row) is the reading every piped child of the shared runner must produce.
- ASCII-only human output: design-system §Surface: cli → Tokens (Width) is the form any new expected-output literal in a §5 test takes.

## Anti-patterns to avoid
- Merging stderr into stdout in the shared runner, or asserting on a combined capture (design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER mix data and messages").
- A new test or helper that expects, forces or tolerates colour, glyphs or cursor control from a piped, `--json`, `NO_COLOR` or `TERM=dumb` child (design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER emit colour, glyphs or cursor control…").
- A split `record` or a §5 test fixture whose error or hint line carries upstream text, a path or a pid (design-system §Anti-Patterns → Per-Surface Bans → cli, "NEVER print upstream text, paths, pids…").

## Contract bindings
- design ↔ tests: the cli output strings of design-system §Surface: cli → Component Patterns are what the root `tests/cli_*.rs` files assert; the scope's "lifting shared scaffolding moves no assertion" is the test-side statement of the same floor.
- design ↔ security: the record refusal's closed codes and "never the content" in design-system §Surface: cli → Component Patterns (Exit-code phraseology) restate a security-plan rule; the security extract owns the rule, design owns the printed form.
- design ↔ a11y: none for this chunk — no web-spa surface, token pair or motion is touched, so the contrast, reduced-motion and not-colour-alone bindings are not engaged.

## Acceptance criteria contributions
- (design) After §1, every root test that asserted on stdout or on stderr alone still asserts on that stream alone, and the shared runner exposes the two as separate captures (per design-system §Surface: cli → Tokens (Streams)).
- (design) After §2, the existing assertions on `viola verify`'s step-counter lines, its summary line and its record refusal pass with no expected-output literal edited (per design-system §Surface: cli → Component Patterns 5).
- (design) The chunk's diff adds no SGR escape literal and no colour value to a product crate, and none to a test outside `tests/cli_output_plain.rs` (per design-system §Surface: cli → Tokens (the token table)).
- (design) Any test added for the `src/cmd/hook.rs` survivor reads an empty stderr and exit 0 from the child (per design-system §Surface: cli → Platform-Specific Notes).
