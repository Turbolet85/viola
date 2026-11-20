# design · standing conventions

- S1 · every new human-facing or log line is ASCII-only, words not glyphs, with control characters escaped · surface-cli
  contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
  contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
  contract/viola-link-output/since · viola link and unlink output · since — The `link` line's `since` is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant prints through the message-mode escaper.
  contract/viola-list-board/row-escaping · viola list board · row escaping — The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
  contract/viola-revive-output/list-line · viola revive output · --list line — `viola revive <name> --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>`, two spaces between fields: no header, no colour, plain ASCII.
  contract/viola-verify-output/help · viola verify output · --help — `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
  decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
  decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
  decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last` and `list` escapes C0/C1 control characters.
  decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
  decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
- S2 · data on stdout and messages on stderr; no colour, spinner or decoration added to any new output · anti-patterns-never-do-these
  contract/cli-colour-decision-order/json · CLI colour decision order · --json — First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/no-color · CLI colour decision order · NO_COLOR — Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/stdout-not-terminal · CLI colour decision order · stdout is not a terminal — Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/term-dumb · CLI colour decision order · TERM=dumb — Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
  contract/cli-colour-tokens/handoff-blue · CLI colour tokens · handoff blue — Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
  contract/cli-colour-tokens/reset · CLI colour tokens · reset — `\x1b[0m` at every colour depth, after every styled span.
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
  contract/refusal-hints/endpoint-lost · refusal hints · endpoint-lost — When a wrapper is lost with the call open: `<name>'s wrapper went away during the call; viola list shows whether it is still running`, last on stderr, plain ASCII.
  contract/viola-link-output/result-line · viola link and unlink output · result line — `viola link` and `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0.
  contract/viola-list-board/json-refusal · viola list board · --json refusal — Under `--json` the strict-modes refusal, the board's only one, is one stdout document `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
  contract/viola-list-board/uncoloured-lines · viola list board · uncoloured lines — The BAY line, the caption, the empty-board line and the separator carry no SGR.
  contract/viola-revive-json-codes/socket-dir-refused · viola revive --json exit-1 codes · socket-dir-refused — A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
  contract/viola-verify-output/fail-word · viola verify output · fail word — The word `fail` is uncoloured.
  contract/viola-verify-refusals/interrupted · viola verify refusals · interrupted run — A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary; stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI`; exit 1.
  decision/bold-only-name-column · Bold only for the NAME column — In the cli, bold is used only for the NAME column.
  decision/busy-not-animated · Busy has no animation — The `busy` state has no spinner, pulse or animation.
  decision/cli-colour-amber-and-dim-only · CLI colour limited to amber and dim — CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted. · why: The quiz limits CLI colour to amber and dim.
  decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
  decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
  decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
  decision/no-green-accent · No run-green accent — The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
  decision/no-handoff-blue-in-cli · No handoff blue in the CLI — The CLI never uses the handoff blue colour.
  decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
  decision/no-terminal-background · No background colours — CLI output never sets a background colour. · why: the terminal background belongs to the user
  decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
  decision/terminal-background-never-set · Terminal background never set — The CLI never sets a background colour, at any colour depth. · why: The terminal background belongs to the user.
