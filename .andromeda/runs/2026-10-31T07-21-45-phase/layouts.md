# layouts extract

## Relevance
partial: the chunk renders on `## Surface: cli` through the `--json` documents, the `viola mcp` machine view and the error lines; it requires no new screen, column, wireframe region, focus order, breakpoint or modal, and its channel `error.data` and `thiserror` work carry no layout content except where it reaches a CLI line.

## Constraints
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires that, under `--json`, an outcome is one stdout document with no stderr line and no hint text, and that its cause is a detail code only once architecture names those codes, the architecture shape holding until then. Whether every verb already does so is research's question.
- layout-templates §Surface: cli, Primary screens (commands) requires the `viola mcp` bullet's form for all five tools: each refusal `isError: true` with the `refused: <code>` text, bodies of codes only, `detail` a closed field word for `invalid-params`, and no held-back verb named in any tool, description or result. Whether the code already satisfies this on every path is research's question.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires the one catch site to print the fixed internal-error line once, with no hint, for the failed or panicked `cli` verbs it lists; a `revive` start stays outside that list.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires that a hint never quotes the sent text or any upstream text and never names a path or a pid, and that the opaque `unknown` refusal, `wrapper fault` and `internal error` carry no hint.
- layout-templates §Component — Primary content block 2: refusal lines and the `unable` column requires the exit-1 fixed-message pairs of `run`, `revive` and `verify` to hold no name, path or pid beyond its stated forms, and the fixture refusal to name the file and the check, never the content. If the `Refusal` enum is folded, these wordings stay as the section lists them; which pairs that enum feeds is research's question.
- layout-templates §Output structure — `viola revive` requires the non-`--json` pairs and rows to stay byte-identical beside the closed-code document, and assigns the documents of the post-preflight start refusals to the working-route entry "Exit-cause code catalogue", out of this chunk's boundary.
- layout-templates §IA notes requires the human columns and words to stay stable as an output contract: new fields go into `--json` first, and an added human column is a design change, not an implementation choice.

## Patterns to follow
- The closed-code error document with a null `detail` and empty stderr, per layout-templates §Output structure — `viola list --json`.
- One document per verb under `--json`, with the internal-error line as the failure form outside the document, per layout-templates §Output structure — `viola wait` / `viola last`.
- The typed document replacing glyph and hint in the machine view, per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror).
- The two-stream split of results on stdout and refusals, hints and errors on stderr, per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column.
- A failed exe read, directory creation or file write landing on the `cli` catch site, per layout-templates §Output structure — `viola plugin install`.

## Anti-patterns to avoid
- A hint or document that carries upstream text, a path or a pid, per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column.
- A hint that names `viola release`, per layout-templates §Component — Primary navigation (verb structure).
- A human-line rewording or added column as a side effect of sanitising the machine view, per layout-templates §IA notes.

## Contract bindings
- layouts ↔ architecture: the `list` payload shapes are architecture §Standard Contracts, per layout-templates §Output structure — `viola list --json`; the detail-code wording waits on architecture naming the codes, per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column.
- layouts ↔ design-system: each cause of exit 1 and exit 21 keeps its own hint under design-system cli pattern 2, per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column.
- layouts ↔ channel: a wrapper error, `-32602` included, surfaces on `link` / `unlink` as the wrapper-fault line, exit 20, per layout-templates §Output structure — wheel, handoff and dialog verbs; a change to the causes of `-32602` must leave that line's form as specified.
- layouts ↔ working-route: the post-preflight `run` start-refusal documents are owed to "Exit-cause code catalogue", per layout-templates §Output structure — `viola revive`.

## Acceptance criteria contributions
- (layouts) Under `--json`, each error outcome the chunk enumerates is one stdout document with an empty stderr, no hint and no glyph (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- (layouts) Each `isError` result of `send`, `wait`, `last`, `answer` and `list` holds the `refused: <code>` text and code-only bodies, and no tool, description or result names `release`, `pause`, `link` or `unlink` (per layout-templates §Surface: cli, Primary screens (commands)).
- (layouts) With a path, an upstream text and a planted stripped `CLAUDE*` value in play, no hint line carries any of them, and a failed or panicked listed verb prints only the fixed internal-error line, once, with no hint (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- (layouts) The human-mode refusal pairs and rows of `viola revive` are byte-identical before and after the chunk (per layout-templates §Output structure — `viola revive`).
