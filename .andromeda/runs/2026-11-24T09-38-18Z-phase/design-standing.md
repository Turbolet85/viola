# design · standing conventions

- S1 · every new verb gets its command name, its --help row in a verb group, and its verb-output contract · surface-cli · surface
- S2 · every new refusal carries a fixed unable line and, in human mode only, one hint line, and a closed --json code · surface-cli · surface
- S3 · every new message names no internals, no stack trace, no path or pid, and keeps data on stdout and messages on stderr · anti-patterns-never-do-these
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
  decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
- S4 · every new line of human output is ASCII text with no decorative glyph, check-mark or emoji prefix · typography · iconography
  decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
  decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
