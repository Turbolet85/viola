# layouts · standing conventions

- S1 · every new human output line or board form is a contract, pinned by a test of its exact words and slot order · surface-cli
- S2 · every new data field shown on a verb also has its --json form inside that verb's one JSON document · surface-cli
  contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
- S3 · every new refusal carries its own hint as the last stderr line and a fixed refusal word in the shared column · surface-cli
  contract/viola-send-output/input-not-ready-hint · viola send output · input-not-ready hint — An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
  contract/viola-send-output/paste-cut-hint · viola send output · paste-cut hint — An unable not-delivered paste-cut outcome, on its own struck mirror line, ends stderr with `hint: <name> did not take the whole text in time; a human must look at the session before the next send`; the answer hints are unchanged.
  contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
