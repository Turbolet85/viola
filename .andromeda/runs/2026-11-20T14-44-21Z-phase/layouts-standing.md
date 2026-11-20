# layouts · standing conventions

- S1 · each new refusal cause owes its hint, last on stderr, naming the next verb, and its --json error code · surface-cli
- S2 · each new or changed human output line is a contract, fixed in words and plain ASCII, with a test reading it back · surface-cli
  contract/viola-plugin-install/output · viola plugin install · human output — After writing the marketplace tree, four lines in one plain-ASCII stdout write, no SGR, never redrawn: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`.
  contract/viola-revive/list-row · viola revive · --list row — `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
