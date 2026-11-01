# layouts extract

## Relevance
partial: no screen, wireframe, focus order or breakpoint moves; the chunk touches the `cli` surface only at its refusal lines (stderr, `unable` column and last-line `hint:`) and their `--json` documents, for the socket-directory, peer-euid and board home-check refusals.

## Constraints
- Every new cause needs its own hint, keyed by reason or by reason and detail, so that an agent can tell exit 21 and exit 1 causes apart from the last stderr line alone; this applies to a refused socket directory and to a peer euid mismatch (per layout-templates §Surface: cli / Component — Primary content block 2, Hint line).
- A hint names no path and no pid and quotes no upstream text, so a socket-directory refusal may not print the directory it refused (per layout-templates §Surface: cli / Component — Primary content block 2, Hint line).
- The machine view of exit 21 carries its cause in the document's `detail`, from a closed set; the plan lists no socket-directory or peer-euid cause, so whether each reuses a listed word (e.g. `server-verify-failed`, `strict-modes-failed`) or adds one is for P3 and P4 to settle, and any addition amends this set (per layout-templates §Surface: cli / Component — Primary content block 2, Exit codes as the typed tail).
- The form follows the verb: frame verbs use the `unable  <name>  <reason>` column (`viola send` inside its `[/ ] unable` mirror), while the `viola run` and `viola revive` starts use the fixed `unable: <text>` and `hint:` pair with no log opened and no child; which exit a wrapper-start refusal takes is research's question (per layout-templates §Surface: cli / Component — Primary content block 2, Line form).
- The macOS gap is already worded in the plan's hint text as a server check making no comparison; the Linux and Windows `server-verify-failed` reading must not be extended to imply a comparison on macOS (per layout-templates §Surface: cli / Component — Primary content block 2, Hint line).
- The board's refusal keeps two distinct readings: the home or `instances/` takes the home pair, and one instance takes the state-files pair; making the home check one rule must leave a failing home reading as the home pair and never naming an instance, and which rule it becomes is P3's and P4's question (per layout-templates §Surface: cli / Component — Primary content block 2, Line form).
- Under `--json` each refusal is one stdout document, stderr empty, no hint text, and the directory's mode left as found (per layout-templates §Surface: cli / Output structure — `viola list --json`).

## Patterns to follow
- The chunk's surface placement: `cli` surface, stderr stream, refusal line then `hint:` as the closing line, with no new region, column or component (per layout-templates §Surface: cli / Component — Footer / terminator).
- The home-tree refusal of the `run` and `revive` starts is the model for a check made before the wrapper log opens: fixed preflight position, one pair, exit 1, nothing created (per layout-templates §Surface: cli / Output structure — `viola revive`).
- The strict-modes refusal of the frame verbs is the model for a check made before any role file or log opens and before any frame (per layout-templates §Surface: cli / Component — Primary content block 2, Line form).
- The home pair's hint states that viola does not change permissions, which is the same stance as the scope's "never narrowed in place"; a socket-directory hint should hold that stance in its own words, since the plan's home pair speaks of the viola home and not of the socket directory (per layout-templates §Surface: cli / Component — Primary content block 2, Line form).
- Refusal text is plain ASCII with no cursor control, level 0.0 in non-TTY, `--json`, `NO_COLOR` and `TERM=dumb` (per layout-templates §Surface: cli, Expression level).

## Anti-patterns to avoid
- Colouring a refusal: colour on this surface is reserved for `DIALOG` and `stale`, and `unable` is never red (per layout-templates §Surface: cli / Component — Hero / signature output line).
- A hint that names `viola release`, since hint-carrying refusals reach drivers (per layout-templates §Surface: cli / Component — Primary navigation).
- A human surface for `viola hook`, or a human line from `viola mcp`, carrying this chunk's refusals (per layout-templates §Surface: cli / Component — Header / banner).

## Contract bindings
- layouts ↔ tests: each new refusal's code and hint, as worded under Primary content block 2, is a row of `tests/cross_exit_causes.rs` (the scope's requirement); the plan's text and that row must agree.
- layouts ↔ design-system: the `viola run` sibling causes and per-cause hints are also listed in design-system cli pattern 2, which block 2 cites, so a new start pair binds both documents.
- layouts ↔ mcp: a `viola mcp` tool's refusal is the CLI verb's `--json` document less `v`, codes only, so a new exit-21 `detail` reaches drivers through that surface (per layout-templates §Surface: cli / Primary screens (commands)).
- layouts ↔ security: the no-path and no-pid hint rule holds the socket directory's location out of refusal text.

## Acceptance criteria contributions
- (layouts) A refused socket directory and a peer euid mismatch each print, on every verb they reach, a refusal line then a distinct `hint:` as the last stderr line, plain ASCII, naming no path and no pid, with stdout empty (per layout-templates §Surface: cli / Component — Primary content block 2).
- (layouts) Under `--json` each of those refusals prints exactly one stdout document, with `detail` from the amended closed cause set for exit 21 and stderr empty (per layout-templates §Surface: cli / Component — Primary content block 2).
- (layouts) `viola list` with a home failing the one rule prints the home pair whether the home holds zero instances or several, and never the instance pair for that failure, exit 21 (per layout-templates §Surface: cli / Output structure — `viola list --json`).
- (layouts) `viola list`, `viola send` and `viola --help` human output keep their columns, captions and rows unchanged by this chunk (per layout-templates §Surface: cli / IA notes).
