# design · standing conventions

- S1 · every new human output line is ASCII-only, never wrapped, with control characters escaped · surface-cli
  contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
  contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
  contract/viola-last-output/message · viola last output · message — `last` prints the message text on stdout with C0/C1 escaped, or `no message` on stderr when the message is `null`.
  contract/viola-link-output/result-line · viola link and unlink output · result line — `viola link` and `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0.
  contract/viola-link-output/since · viola link and unlink output · since — The `link` line's `since` is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant prints through the message-mode escaper.
  contract/viola-list-board/row-escaping · viola list board · row escaping — The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
  contract/viola-revive-output/list-line · viola revive output · --list line — `viola revive <name> --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>`, two spaces between fields: no header, no colour, plain ASCII.
  contract/viola-verify-output/help · viola verify output · --help — `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
  decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
  decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
  decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last` and `list` escapes C0/C1 control characters.
  decision/lines-never-wrapped · Lines never wrapped — CLI output lines are never wrapped.
  decision/no-hardcoded-widths-or-wrapping · No hardcoded widths or wrapping — Widths are never hardcoded; output is truncated only where the terminal width is known, and never wrapped.
- S2 · data goes on stdout and messages on stderr, with no spinner or elapsed-time counter in verb output while a probe waits · anti-patterns-never-do-these
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
  contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
  contract/frame-verb-refusals/socket-dir · frame verb refusals · socket directory — A refused socket directory exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the `socket-dir` hint, no strict-modes pair; `--json`: `detail` `socket-dir-refused`, stderr empty.
  contract/refusal-hints/endpoint-lost · refusal hints · endpoint-lost — When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
  contract/viola-list-board/json-refusal · viola list board · --json refusal — Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
  contract/viola-revive-json-codes/socket-dir-refused · viola revive --json exit-1 codes · socket-dir-refused — A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
  decision/busy-not-animated · Busy has no animation — The `busy` state has no spinner, pulse or animation.
  decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
  decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
  decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
  decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
