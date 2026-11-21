# design · standing conventions

- S1 · every new error, refusal or hint line is ASCII only, escapes control characters, and keeps data on stdout and messages on stderr · anti-patterns-never-do-these
  contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
  contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
  contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
  contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
  contract/refusal-hints/endpoint-lost · refusal hints · endpoint-lost — When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
  decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
  decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
- S2 · every new refusal or error names no internals: no path, pid, stack trace or caller text in an error or hint · anti-patterns-never-do-these
  contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
  decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
- S3 · every new failure under --json stays one clean stdout document with its fixed code, and no hint line under --json · surface · surface-cli
  contract/frame-verb-refusals/socket-dir · frame verb refusals · socket directory — A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
  contract/viola-list-board/json-refusal · viola list board · --json refusal — Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
  contract/viola-revive-json-codes/socket-dir-refused · viola revive --json exit-1 codes · socket-dir-refused — A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
  decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
  decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
  decision/no-hint-under-json · No hint line under --json — No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
