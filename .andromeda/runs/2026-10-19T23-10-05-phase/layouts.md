# layouts extract

## Relevance
partial: the chunk adds no surface, region, column or verb. It touches two existing cli outputs: `viola verify`'s step lines and summary, which are the oracle of "24 pass / 0 fail", and the `viola send` readback mirror's `/clear` / `/compact` confirm path, if a row resolves by way (b).

## Constraints
- `viola verify` prints one stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in the fixed ledger order. `MM` is `LedgerRow::relied()`'s length: 24 on Unix, 21 on Windows. A fix to `tag-escaping` (16), `local-command-clear` (17), `paste-newline` (18) or `local-command-compact` (24) keeps each row's id and position, and adds or removes no row (per layout-templates §Output structure — `viola verify`).
- The last stdout line of a completed `viola verify` is the summary `stamped <ver>  <n> pass  <m> fail`. It is plain ASCII with no colour, and `fail` is never coloured. The target reading for this chunk is `stamped 2.1.302  24 pass  0 fail`. A failing row still exits 1 with the stamp written (per layout-templates §Output structure — `viola verify`).
- Step lines are appended once each, with no progress bar and no redraw. `--record` (the conditional `fixtures/claude/2.1.302/` capture) adds no human line (per layout-templates §Output structure — `viola verify`).
- `viola verify`'s exit-1 refusals keep their fixed `unable:` / `hint:` pairs, with no name, path or pid. These include the `--record` fixture refusal `a recorded fixture is not clean: <file> <code>`, whose codes are `home-path` · `absolute-path` · `username` · `email` and which never shows the content (per layout-templates §Component — Primary content block 2).
- On a verified CLI version, `viola send` confirms `/clear` and `/compact` by their post-conditions and prints the `[RB] read back` outcome on stdout. `/clear` is confirmed by a `session-start` with cause `clear` and a new session id. `/compact` is confirmed by a `session-start` with cause `compact` under the session's own id, inside its own 40 s PROVISIONAL window, and otherwise reads `not-delivered` / `no-prompt-submitted`. `[  ] unconfirmable` is printed only on an unverified CLI version, or for a local command with no measured post-condition. If a way-(b) change to the `send` confirm path reaches these lines, it keeps their words and columns (per layout-templates §Signature placement).
- The readback mirror stays words and brackets only. There is no colour, no spinner, no `✓` and no `done`, and `turn-ended` is never a send outcome (per layout-templates §Component — Hero / signature output line).

## Patterns to follow
- If a probe's row words change, the new text keeps the existing step-line shape: `[NN/MM]`, then the row id, then the row words, then two spaces and `pass|fail`. Every row's words read as one present-tense clause of what the CLI does, for example `tag-like text un-escapes to the text as pasted` (per layout-templates §Output structure — `viola verify`).
- The `--json` machine view of `send` is unchanged: the `ok` document with `submitted_at` + `cursor`, or `ok` with `confirmed:false` + `unconfirmable`, or `refusal` + `detail`. It has no glyph and no hint (per layout-templates §Component — Hero / signature output line).

## Anti-patterns to avoid
- A changed row's words, summary or `send` outcome line must not gain a colour, a `✓`, a redraw or a new human column. Human columns and words are a stable contract that LLM drivers read, and a new field goes into `--json` first (per layout-templates §IA notes).
- No `hint:` line added or reworded by this chunk may name `viola release`, a path, a pid or upstream text (per layout-templates §Component — Primary navigation).

## Contract bindings
- layouts ↔ tests: the verify step-line and summary forms are the witness text for "24 pass / 0 fail on 2.1.302" and for the 2.1.287 / 2.1.296 no-regress readings. Whether the existing tests pin the row-words literals is research's question.
- layouts ↔ architecture/ledger: the `[NN/MM]` order and `MM` follow `LedgerRow::ALL` / `relied()` in `crates/viola-agent-claude/src/ledger.rs`. Row words come from the ledger, so a wording change there is a layouts-visible change.
- layouts ↔ design-system: `fail` is never coloured, and the mirror has no colour. Colour on the cli is reserved for `DIALOG` and `stale`.

## Acceptance criteria contributions
- (layouts) A live `viola verify` on claude 2.1.302 on the Linux dev host prints exactly 24 step lines, `[01/24]` through `[24/24]`, in ledger order. Rows 16, 17, 18 and 24 keep their row ids. The last stdout line reads `stamped 2.1.302  24 pass  0 fail`, with exit 0 and no SGR byte in stdout (per layout-templates §Output structure — `viola verify`).
- (layouts) Every step line's `<row words>` is either unchanged from the wireframe or a deliberate wording change that the plan names. No step line is added, removed or reordered (per layout-templates §Output structure — `viola verify`).
- (layouts) On a verified CLI version, a `viola send` of `/clear` and of `/compact` prints `[RB] read back      <name>  <time>  cursor <n>` on stdout with exit 0. On an unverified CLI version it prints `[  ] unconfirmable` with exit 0. If the chunk's change reaches the confirm path, both forms read the same as before it (per layout-templates §Signature placement).
