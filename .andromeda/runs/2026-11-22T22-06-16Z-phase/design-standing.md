# design · standing conventions

- S1 · each new human-output line or column uses only the fixed phraseology vocabulary and ASCII-only glyphs, colour only as a second cue · anti-patterns-never-do-these
  decision/colour-carries-meaning · Colour carries meaning, never decoration — Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used. · why: Any third hue has no domain meaning.
  decision/colour-only-as-second-cue · Colour only as a second cue — Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them. · why: The quiz confirms colour as a second cue only.
  decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
- S2 · each new refusal names its fixed unable line, its hint where one is owed, its exit code and its --json code · surface · surface-cli
  contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
  contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
  contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
  contract/cli-stderr/refusal-line · CLI stderr · refusal line — A refusal prints `unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column.
  contract/refusal-hints/refusal-line · refusal hints · refusal line — A refusal prints `unable` plus `reason  detail`, then one `hint:` line keyed by the reason, or by reason and detail where listed; `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint.
- S3 · each new verb output is checked in both the TTY form and the plain piped form, and in --json as one stdout document · surface-cli
