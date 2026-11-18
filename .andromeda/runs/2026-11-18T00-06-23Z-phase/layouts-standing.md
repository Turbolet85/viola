# layouts · standing conventions

- S1 · each new verify output line or stamp field a test asserts is an exact, plain, uncoloured contract line with results on stdout and the rest on stderr · surface-cli
  contract/cli-token-rendering/color-text-primary-secondary · CLI token rendering · color-text-primary / color-text-secondary — The terminal's default foreground; the terminal owns ink.
  contract/link-output/link · viola link / unlink output · link — Prints one stdout line in one write, plain in every mode, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, the reply's since as UTC clock time in whole seconds, a non-RFC 3339 since printed escaped.
  contract/viola-help/verb-groups · viola --help output · verb group rows — Five rows `board:    list`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink`, `setup:    run, revive, verify, plugin install`, served verbs only, plain ASCII, then a blank line, `Options:` and clap's options; no about line and no clap `Commands:` list.
  contract/viola-last-output/message · viola last output · message — The last assistant message prints on stdout with C0/C1 escaped, after a `last  <name>  <kind> <ts>` header on stderr on a TTY only; a null message prints `no message` on stderr.
  contract/viola-list-board/plain-lines · viola list board · plain lines — The BAY line, caption, empty line and separator print plain.
  contract/viola-list-board/uncoloured-modes · viola list board · uncoloured modes — The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
  contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
  contract/viola-plugin-install/output · viola plugin install · human output — After writing the marketplace tree, four lines in one plain-ASCII stdout write, no SGR, never redrawn: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`.
  contract/viola-revive/list-row · viola revive · --list row — `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
  contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
  contract/viola-send-output/issue-line · viola send output · issue line — `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper.
  contract/viola-send-output/piped-stdout · viola send output · piped stdout — When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror.
