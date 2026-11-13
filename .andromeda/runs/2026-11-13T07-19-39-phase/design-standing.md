# design · standing conventions

- S1 · every test the chunk moves or adds that asserts a verb's output asserts the fixed forms: data on stdout, messages on stderr, the unable and hint lines in their fixed shapes, the exit code of the contract · surface-cli · surface
  contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
  contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
  contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed.
  decision/hint-never-quotes-text · Hints never quote text — A hint never quotes the sent text or any upstream text.
  decision/no-hint-names-release · No hint names viola release — No hint names `viola release`. · why: a driver's `release` is refused
