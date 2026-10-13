# layouts extract

## Relevance
partial — the chunk builds no surface and changes no output; the cli surface is touched only by scope §3's pure-move split of `src/human.rs` / `src/run/send.rs` and by the §1 survivor at `src/cmd/send.rs` `Out<'_>::open` (the `viola send` output writer), so the cli output structures act as preservation constraints, not new layout; the web-spa surface is out of scope.

## Constraints
- The `viola send` readback mirror's two components (the TTY-only issue line `[  ] open …` on stderr, the outcome line `[RB]` / `[/ ]` / `[  ] unconfirmable` with its typed exit) and the padded word column must read the same after the split; when stdout is piped or `--json` is set the issue line is dropped (per layout-templates §Surface: cli, Signature placement; per layout-templates §Output structure — `viola send`). Whether the `Out<'_>::open` `:119:12` survivor (delete `!`) sits on this TTY / `--json` decision is research's question.
- `viola list` as built prints the uncoloured form on every stream as one stdout write, with fixed paddings (NAME to max(4, longest name), LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI last unpadded), no trailing space and no truncation; the `human.rs` split must leave these bytes unchanged (per layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)).
- The styled TTY form (bold NAME, amber `DIALOG`, dim stale row), the `-- UNWRAPPED - READ-ONLY --` rack and the grouped `--help` table are owed to later entries ("CLI output discipline", "Unwrapped sessions on the board"); a pure move does not build them (per layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`); per layout-templates §Primary screens (commands)).
- Refusal lines keep their form `unable  <name>  <reason>  <detail>` (and the fixed `unable: <text>` / `hint:` pairs for `run`, `revive`, `verify` and `list`'s exit 21), the `hint:` line is the last stderr line, and a failed or panicked verb prints exactly `error: internal error` once from the one catch site with no hint (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, never mixed; the exit code is the typed tail (10/11/12/13/14/20/21) (per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- No terminator word, no banner, no spinner, no redraw: lines are appended once and the last printed line says what happened, to whom (per layout-templates §Component — Footer / terminator).
- The human columns and words are an output contract LLM drivers may read; a change to them is a design change, not an implementation choice (per layout-templates §IA notes (Surface: cli)).

## Patterns to follow
- One writer per verb's output structure: the send mirror, the wait/last result lines, the wheel/handoff/dialog verb lines, the verify step counter and summary, the board — a natural phase/verb seam for splitting `src/human.rs` (per layout-templates §Primary screens (commands)).
- The send outcome lines share one word column and one NAME column across `open` / `read back` / `unable` / `unconfirmable` (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror)); a split keeps the padding in one place.
- `viola release --budget`'s two lines (`budget gate lifted until <end>  wheel <holder>` with its `HH:MM:SSZ` / `YYYY-MM-DD HH:MM:SSZ` forms, and `budget gate open  wheel <holder>`) are founder-confirmed wording carried byte-for-byte (per layout-templates §Output structure — wheel, handoff and dialog verbs).

## Anti-patterns to avoid
- No colour on the send mirror and no `Sending…`, `✓` or `done` (per layout-templates §Component — Hero / signature output line (the `viola send` readback mirror)).
- No hint names `viola release`, quotes sent or upstream text, or names a path or pid (per layout-templates §Component — Primary navigation (verb structure); per layout-templates §Component — Primary content block 2: refusal lines and the `unable` column).
- No new human column or reworded field during a pure move (per layout-templates §IA notes (Surface: cli)).

## Contract bindings
- cli output structures ↔ design-system §Surface: cli Component Patterns 1, 2 and 4 (the board's planning forms, the hint-per-cause rule, the `release --budget` wording), which layout-templates cites as their source.
- cli `--json` documents ↔ architecture §Standard Contracts (the `ok` / `refusal` / exit-21 `{"v":1,"error":"instance-unreachable","detail":null}` shapes), named by layout-templates §Output structure — `viola list --json` and §Component — Primary content block 2.
- Existing output tests for `send` / `wait` / `last` / `list` / `release` / `verify` ↔ the test plan: the split is proven by those tests staying green unchanged.

## Acceptance criteria contributions
- (layouts) After the `src/human.rs` / `src/run/send.rs` split, every existing human-output test passes unchanged, with no edited expected strings, so every verb's stdout/stderr bytes and exit codes stay as they were (per layout-templates §IA notes (Surface: cli)).
- (layouts) `viola send` still prints the `[  ] open` issue line only when stderr is a TTY and `--json` is not set; whether the test for the `Out<'_>::open` survivor already pins this is research's question (per layout-templates §Output structure — `viola send`).
- (layouts) `viola list` output after the split still has the as-built widths, no trailing space and one stdout write (per layout-templates §Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)).
