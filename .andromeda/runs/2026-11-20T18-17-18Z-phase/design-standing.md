# design · standing conventions

- S1 · every new viola verify step and summary line follows the verify output's step-line and summary-line forms, in ASCII only, with the step counter as its only progress pattern · surface-cli
- S2 · every new refusal or failure line names no internals, path or pid, and goes to stderr · anti-patterns-never-do-these
  contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
  contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
  decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
  decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run` and `viola revive` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
