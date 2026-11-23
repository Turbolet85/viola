# design · standing conventions

- S1 · every new human output or refusal line is ASCII-only, escapes control characters and carries no internals · typography · anti-patterns-never-do-these
  contract/cli-verb-output/plugin-install · CLI verb output · viola plugin install — Prints four plain ASCII stdout lines in one write, no SGR: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`; `<dir>` is quoted only where the host's shell would split it.
  contract/refusal-hints/endpoint-lost · refusal hints · endpoint-lost — When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
  contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
  contract/viola-link-output/result-line · viola link and unlink output · result line — `viola link` and `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0.
  contract/viola-revive-output/list-line · viola revive output · --list line — `viola revive <name> --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>`, two spaces between fields: no header, no colour, plain ASCII.
  contract/viola-verify-output/help · viola verify output · --help — `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
  contract/viola-verify-output/trust-record-names-nothing · viola verify output · trust-record row — The trust-record row prints its step line ending `pass` or `fail`, and no step line, summary or refusal of `viola verify` names the config file, the trust record's key or its value.
- S2 · every new --json document is one stdout document, versioned `v:1`, with ok or error and a code word · surface-cli
  contract/viola-link-json/ok · viola link --json · ok — Exit 0 prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link.
  contract/viola-list-board/json-refusal · viola list board · --json refusal — Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
  contract/viola-revive-output/list-json · viola revive output · --list --json — `viola revive <name> --list --json` prints `{"v":1,"ok":{"sessions":[…]}}`.
  contract/viola-unlink-json/ok · viola unlink --json · ok — Exit 0 prints `{"v":1,"ok":{}}`.
- S3 · every new refusal code carries a fixed message, an exit code from the CLI exit-code set and, in human mode only, at most one hint · surface · surface-cli
