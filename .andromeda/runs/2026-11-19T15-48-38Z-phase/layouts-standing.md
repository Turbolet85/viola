# layouts · standing conventions

- S1 · each new capability-ledger row and its viola verify probe prints in verify's shared step-line and failing-row form, in plain ASCII, and is counted in the summary line · surface-cli
  decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
- S2 · any new human output line is a contract: results go on stdout and everything else on stderr, with no colour in non-TTY modes and no spinner or progress glyph · surface-cli
  contract/readback-mirror/issue-and-outcome · the readback mirror line · issue and outcome lines — The TTY issue line on stderr is the open box and the outcome line under it is the closed box, the two lined up column for column.
  contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
  contract/viola-last-output/message · viola last output · message — The last assistant message prints on stdout with C0/C1 escaped, after a `last  <name>  <kind> <ts>` header on stderr on a TTY only; a null message prints `no message` on stderr.
  contract/viola-list-board/plain-lines · viola list board · plain lines — The BAY line, caption, empty line and separator print plain.
  contract/viola-list-board/uncoloured-modes · viola list board · uncoloured modes — The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
  contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
  contract/viola-plugin-install/output · viola plugin install · human output — After writing the marketplace tree, four lines in one plain-ASCII stdout write, no SGR, never redrawn: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`.
  contract/viola-revive/list-row · viola revive · --list row — `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
  contract/viola-send-output/issue-line · viola send output · issue line — `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper.
  contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
  contract/viola-send-output/piped-stdout · viola send output · piped stdout — When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror.
  decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
  decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
  decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
  decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
  decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
