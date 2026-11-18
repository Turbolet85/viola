# design · standing conventions

- S1 · every new test or recorded reading over viola verify's human output keeps data on stdout and messages on stderr, ASCII only, no stack trace and no internals in its lines · surface-cli · surface
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
  contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
  contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
  decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
  decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
  decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
  decision/no-stack-traces · No stack traces — Stack traces never print; errors are fixed messages and full detail goes only to the instance's diagnostics.
  decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
