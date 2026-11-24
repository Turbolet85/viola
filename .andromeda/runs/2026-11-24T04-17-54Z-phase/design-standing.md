# design · standing conventions

- S1 · every new CLI verb's human output is ASCII-only, escapes control characters, prints data on stdout and messages on stderr, and carries no internals, emoji, check glyphs or spinners · anti-patterns-never-do-these
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
  decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
  decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
- S2 · every new verb's --json output is one stdout document of codes and closed words, with no hint line and a clean failure document · surface
- S3 · every new verb's outcome and refusal lines use the fixed phraseology vocabulary and confirm nothing before confirmation · brand-identity
  decision/confirmations-carry-context · Confirmations carry context — Output never says "done" or "success" bare; it says what was read back, by whom, and at which cursor.
