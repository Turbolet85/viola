# design · standing conventions

- S1 · each new human-mode line or refusal a start or revive prints uses the fixed unable and hint forms, ASCII-only, on stderr, with no colour beyond the CLI's amber and dim · surface-cli · surface
  contract/cli-colour-tokens/handoff-blue · CLI colour tokens · handoff blue — Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
  decision/cli-colour-amber-and-dim-only · CLI colour limited to amber and dim — CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted. · why: The quiz limits CLI colour to amber and dim.
  decision/colour-carries-meaning · Colour carries meaning, never decoration — Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used. · why: Any third hue has no domain meaning.
  decision/colour-only-as-second-cue · Colour only as a second cue — Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them. · why: The quiz confirms colour as a second cue only.
  decision/no-green-accent · No run-green accent — The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
  decision/no-handoff-blue-in-cli · No handoff blue in the CLI — The CLI never uses the handoff blue colour.
  decision/no-terminal-background · No background colours — CLI output never sets a background colour. · why: the terminal background belongs to the user
  decision/stale-rows-sgr-dim · Stale rows in SGR dim — In the cli, SGR dim marks `stale` rows and amber marks the `DIALOG` word.
  decision/terminal-background-never-set · Terminal background never set — The CLI never sets a background colour, at any colour depth. · why: The terminal background belongs to the user.
- S2 · each new field a --json result carries is part of one clean stdout document, never a second document · surface-cli · surface
