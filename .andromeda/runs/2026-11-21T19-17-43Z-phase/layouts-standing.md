# layouts · standing conventions

- S1 · each new test proves a stated bound or cap and can fail when it is breached; it reaches no live claude start and writes no text to a file, stamp, fixture or log · surface-cli
- S2 · a new MCP tool result or refusal uses the existing result shapes and the existing refusal cause words, with no new human output · surface-cli
  contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`), never reused; no refusal maps to 11.
  contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
  contract/cli-exit-codes/20 · CLI exit codes · 20 — Wrapper fault, `error: wrapper fault  <code>`.
  contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead.
  contract/cli-refusals/instance-unreachable · CLI refusals · instance-unreachable — With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line.
