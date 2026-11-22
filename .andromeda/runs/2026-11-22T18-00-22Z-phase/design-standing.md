# design · standing conventions

- S1 · each new test that asserts a CLI line asserts the fixed phraseology word for word, ASCII-only, on its proper stream · surface-cli
- S2 · each new error or refusal path a test exercises shows no internals, no stack trace, and no success before confirmation · anti-patterns-never-do-these
  contract/viola-revive-json-codes/internal-error · viola revive --json exit-1 codes · internal-error — A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
  decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
