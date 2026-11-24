# design · standing conventions

- S1 · every new refusal or outcome word gets one fixed unable line and at most one fixed hint, a --json code of its own, and an exit code from the CLI exit codes table · surface-cli · surface
- S2 · every new human output line is ASCII-only, escapes control characters, carries no path, pid or internal detail, and puts messages on stderr · anti-patterns-never-do-these
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
  contract/cli-verb-output/focus · CLI verb output · viola focus — `viola focus <name>` prints `<name>  focused` on stdout, or under `--json` `{"v":1,"ok":{"name":"<name>"}}`; no window address, pid or workspace is ever printed or logged.
  contract/cli-verb-output/plugin-install · CLI verb output · viola plugin install — Prints four plain ASCII stdout lines in one write, no SGR: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`; `<dir>` is quoted only where the host's shell would split it.
- S3 · every new log line carries a detail code from the log detail codes and never a stack trace · surface
  contract/viola-revive-json-codes/internal-error · viola revive --json exit-1 codes · internal-error — A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
