# layouts extract

## Relevance
partial — the chunk adds tests and behaviour-preserving restatements and creates or changes no surface; three survivor sites may sit behind cli output forms the plan fixes (`src/cmd/send.rs` `Out<'_>::open`, `viola-state` `board.rs` `SkippedTotal::add` and the `check_if_present` / `scan` NotFound guards). Whether each site actually drives a printed form is research's question. The web-spa surface and every other survivor (`typed.rs`, `wheel.rs`, `dialog.rs`, `hook.rs`, `fs.rs`, `statusline.rs`, `scratch.rs`, `viola-pty`) are outside this domain.

## Constraints
- A restatement at `src/cmd/send.rs` `Out<'_>::open` must leave the readback mirror's form byte-identical: per layout-templates §Surface: cli · Output structure — `viola send`, the `[  ] open` issue line goes to stderr on a TTY only, is dropped when stdout is piped, and is absent under `--json`. Whether `Out::open` is the site that decides whether the issue line prints is research's question.
- Per layout-templates §Surface: cli · Component — Hero / signature output line (the `viola send` readback mirror), the issue line and the outcome line line up column for column: a fixed four-character box, one space, the padded state word, the two-space gutter, then NAME. They are appended and never redrawn. A restatement must keep that alignment.
- Per layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column (Streams), results go to stdout and the issue line, refusals and hints go to stderr, and the two streams are never mixed. A test on the `Out` site asserts the stream as well as the text.
- Per layout-templates §Surface: cli · Component — Header / banner (the BAY context line), the BAY line's last cell is `skipped a - b - c`, built from the three skip counts. `SkippedTotal::add` must keep summing each count across instances. Whether `add` feeds that cell is research's question.
- Per layout-templates §Surface: cli · Output structure — `viola list --json`, the `skipped` object (`unknown_kinds`, `unknown_fields`, `torn_lines`) has the same shape as `/api/sessions`. A test that kills the `+=` mutants reads the summed counts from that object or from the BAY cell, not from a new field.
- Per layout-templates §Surface: cli · Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) (As built), an empty board prints the BAY line, the caption row and the empty line as one stdout write, and per layout-templates §Surface: cli · Output structure — `viola list` (human TTY) the empty line reads `no wrapped sessions  start one: viola run <name> -- claude`. If the `board.rs` NotFound guards (`check_if_present` `:45:19`, `scan` `:67:19`) decide between an empty board and a failure, a restatement must keep the empty-board output. Whether they do is research's question.
- Per layout-templates §Surface: cli · Component — Primary content block 2: refusal lines and the `unable` column, a `list` whose `instances/` cannot be listed prints `error: internal error` with exit 1, and a strict-modes refusal is the fixed exit-21 pair with stdout empty. A test that makes the NotFound guard `true` asserts which of these forms the original prints.

## Patterns to follow
- Assert whole lines as fixed strings: per layout-templates §Surface: cli · IA notes, the human columns and words are a stable contract that LLM drivers may read, so a test checks the exact line rather than a substring.
- Test the `viola list` output in the as-built uncoloured form only, per layout-templates §Surface: cli · Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) (As built): no SGR on any stream, no line ending in a space, nothing truncated.
- For a mirror-line test, read the TTY / piped / `--json` cases as three separate forms, per layout-templates §Surface: cli · Output structure — `viola send`.

## Anti-patterns to avoid
- Adding or rewording a human column, cell or line as a side effect of a restatement. Per layout-templates §Surface: cli · IA notes, a human column change is a design change, not an implementation choice, and this chunk changes no production behaviour.
- Adding a styled form, the unwrapped rack or truncation while touching `board.rs` or the list path. Per layout-templates §Surface: cli · Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`) (As built), these are owed to "CLI output discipline" and "Unwrapped sessions on the board".

## Contract bindings
- layouts ↔ tests: an owning-package test that pins a printed form (the send mirror's issue line, the BAY `skipped` cell, the empty board) is also the check that the layout contract holds, per layout-templates §Surface: cli · IA notes ("Output as a contract").
- layouts ↔ web-spa / API: the `skipped` counts must stay identical in the `viola list --json` shape and in `/api/sessions`, per layout-templates §Surface: cli · Output structure — `viola list --json`.

## Acceptance criteria contributions
- (layouts) If `Out<'_>::open` is disposed by a test or a restatement, the `[  ] open` issue line still prints on stderr only on a TTY without `--json`, and is absent when piped and under `--json` (per layout-templates §Surface: cli · Output structure — `viola send`).
- (layouts) After the `SkippedTotal::add` disposal, the BAY line's `skipped a - b - c` cell and `list --json`'s `skipped` object carry the per-count sums across instances. A test with two instances that each skip lines fails under `+=`→`*=` and under `+=`→`-=` (per layout-templates §Surface: cli · Component — Header / banner (the BAY context line)).
- (layouts) After the `board.rs` NotFound-guard disposal, `viola list` on a home without the scanned path prints the as-built empty board unchanged (per layout-templates §Surface: cli · Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)).
- (layouts) No human output line of `send` or `list` changes byte-for-byte between HEAD and the chunk's end (per layout-templates §Surface: cli · IA notes).
