# design · standing conventions

- S1 · every line a new rehearsal, guard or console text prints is ASCII only, with control characters escaped and no glyph or emoji prefix · surface-cli · anti-patterns-never-do-these
  decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last`, `list` and the fleet console escapes C0/C1 control characters.
  decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
  decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
- S2 · every new refusal or error line keeps the unable and hint form, with no stack trace, no internals and no path or pid · surface-cli · surface
- S3 · data on stdout, messages on stderr, for every new output line · anti-patterns-never-do-these
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
