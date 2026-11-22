# design · standing conventions

- S1 · every new human-facing or log line is ASCII-only text glyphs, control characters escaped, no colour beyond amber and dim, no decorative icons · surface-cli · surface
  contract/cli-colour-decision-order/json · CLI colour decision order · --json — First match of the colour decision order: `--json` output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/no-color · CLI colour decision order · NO_COLOR — Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/stdout-not-terminal · CLI colour decision order · stdout is not a terminal — Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/term-dumb · CLI colour decision order · TERM=dumb — Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/viola-hook · CLI colour decision order · viola hook — Third match of the colour decision order: `viola hook` prints no colour, glyph or SGR and never writes stderr.
  contract/cli-colour-decision-order/viola-mcp · CLI colour decision order · viola mcp — Third match of the colour decision order: `viola mcp` prints no colour, glyph or SGR, and its stdout carries MCP frames only.
  contract/cli-colour-depth/16-colour · CLI colour depth · 16-colour — Where no earlier depth matches, the depth is the 16-colour fallback column; the depth is chosen once per process, first match wins.
  contract/cli-colour-depth/256-colour · CLI colour depth · 256-colour — Where colour applies and truecolor did not match, the depth is 256-colour if `TERM` contains `256color`, chosen once per process.
  contract/cli-colour-depth/truecolor · CLI colour depth · truecolor — Where colour applies, the depth is chosen once per process, first match wins: truecolor if `COLORTERM` is `truecolor` or `24bit`.
  contract/cli-colour-tokens/attention · CLI colour tokens · attention — Arrival amber #D97706 on the `DIALOG` word only, always followed by its kind word, which stays plain: `\x1b[38;2;217;119;6m` in truecolor, `\x1b[38;5;172m` at 256 colours, `\x1b[33m` at 16.
  contract/cli-colour-tokens/bold · CLI colour tokens · bold — NAME values are bold.
  contract/cli-colour-tokens/callsign · CLI colour tokens · callsign — Bold (`\x1b[1m`) at every colour depth, on every NAME column value.
  contract/cli-colour-tokens/dim · CLI colour tokens · dim — SGR dim is laid on a whole `stale` row; the attention colour wins for `DIALOG`.
  contract/cli-colour-tokens/handoff-blue · CLI colour tokens · handoff blue — Handoff blue #5B8DB8 is not used at any colour depth and no index is reserved for it; link output (`overseer -> builder`) is plain.
  contract/cli-colour-tokens/reset · CLI colour tokens · reset — `\x1b[0m` at every colour depth, after every styled span.
  contract/cli-colour-tokens/stale · CLI colour tokens · stale — SGR dim (`\x1b[2m`) at every depth over a whole `stale` row, first byte to last; `stale` always in LIVE; on a cocked stale row only `DIALOG` is amber and not dim; NAME on a stale row is dim and bold; every span reopens dim after its reset.
  contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
  contract/viola-link-output/result-line · viola link and unlink output · result line — `viola link` and `viola unlink` print one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0.
  contract/viola-list-board/row-escaping · viola list board · row escaping — The row mode escapes every control character of a row, DEL and C1 included, as `\xHH`; NAME's width is counted in characters.
  contract/viola-list-board/uncoloured-lines · viola list board · uncoloured lines — The BAY line, the caption, the empty-board line and the separator carry no SGR.
  contract/viola-revive-output/list-line · viola revive output · --list line — `viola revive <name> --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>`, two spaces between fields: no header, no colour, plain ASCII.
  contract/viola-verify-output/fail-word · viola verify output · fail word — The word `fail` is uncoloured.
  contract/viola-verify-output/help · viola verify output · --help — `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
  decision/cli-colour-amber-and-dim-only · CLI colour limited to amber and dim — CLI output colours nothing but the `DIALOG` word and `stale` rows, so no blue or other colour SGR is ever emitted. · why: The quiz limits CLI colour to amber and dim.
  decision/colour-carries-meaning · Colour carries meaning, never decoration — Every colour communicates meaning: amber means `dialog_pending` and nothing else, blue means handoff and nothing else, and no third hue is used. · why: Any third hue has no domain meaning.
  decision/colour-only-as-second-cue · Colour only as a second cue — Only `DIALOG` (amber) and `stale` rows (dim) are coloured, and never without their word beside them. · why: The quiz confirms colour as a second cue only.
  decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last` and `list` escapes C0/C1 control characters.
  decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
  decision/no-green-accent · No run-green accent — The run-green accent of the Developer Tool palette is not used; amber is the single sharp accent.
  decision/no-handoff-blue-in-cli · No handoff blue in the CLI — The CLI never uses the handoff blue colour.
  decision/no-terminal-background · No background colours — CLI output never sets a background colour. · why: the terminal background belongs to the user
  decision/terminal-background-never-set · Terminal background never set — The CLI never sets a background colour, at any colour depth. · why: The terminal background belongs to the user.
  decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
- S2 · every new refusal is a closed code word with the fixed unable line and one hint in human mode only, never quoting text or naming internals · surface
  contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run` or `revive` prints a fixed-message `unable:` line and one hint per cause; `revive`'s causes are read in a fixed order that stops at the first.
  contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`, `pause` and `release` use `unable: <text>`.
  contract/refusal-hints/server-verify · refusal hints · server-verify — `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`, printed byte for byte by the eight frame-writing verbs, plain ASCII with no SGR, holding no pid, start time, path or endpoint.
  decision/instance-unreachable-hint-per-cause · One instance-unreachable hint per cause — The `instance-unreachable` refusal (exit 21) prints one hint per cause. · why: The exit code is shared by several causes.
