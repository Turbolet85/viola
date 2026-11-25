# layouts · standing conventions

- S1 · every changed or added human output line of a verb is a contract — words carry the state, results on stdout and the rest on stderr, a refusal ends with a hint naming the next verb · surface-cli
  contract/viola-send-output/input-not-ready-hint · viola send output · input-not-ready hint — An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
  contract/viola-send-output/paste-cut-hint · viola send output · paste-cut hint — An unable not-delivered paste-cut outcome, on its own struck mirror line, ends stderr with `hint: <name> did not take the whole text in time; a human must look at the session before the next send`; the answer hints are unchanged.
  decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
- S2 · every changed --json output stays one document with the shared exit codes and their --json form · surface-cli
  contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`.
  contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
  contract/cli-exit-codes/14 · CLI exit codes · 14 — `unknown`; an older wrapper's `budget-paused` reply also reads as `unknown`.
  contract/cli-internal-error/json · CLI internal error · --json output — Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1.
  contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
  contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
  contract/viola-switch-output/json-ok · viola switch output · --json ok — Under --json a switch that succeeds prints one stdout document whose `ok` carries the `model` and `effort` of the new pair.
