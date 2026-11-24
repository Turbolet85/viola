# layouts · standing conventions

- S1 · every new refusal cause owes an exit code row, a --json detail word, and a hint that is the last stderr line and names the next verb · surface-cli
  contract/viola-send-output/input-not-ready-hint · viola send output · input-not-ready hint — An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
  contract/viola-send-output/paste-cut-hint · viola send output · paste-cut hint — An unable not-delivered paste-cut outcome, on its own struck mirror line, ends stderr with `hint: <name> did not take the whole text in time; a human must look at the session before the next send`; the answer hints are unchanged.
  contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
- S2 · every new verb owes its CLI verbs row, its --help usage, human output on the stdout/stderr split, and a single --json document · surface-cli
  contract/cli-internal-error/human · CLI internal error · human output — A failure or a caught panic prints `error: internal error` once, exit 1, with no hint.
  contract/cli-internal-error/json · CLI internal error · --json output — Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1.
  contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
