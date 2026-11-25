# design · standing conventions

- S1 · every new or changed human output line is ASCII only, with words not glyphs, and escapes control characters · surface-cli
  contract/cli-colour-decision-order/no-color · CLI colour decision order · NO_COLOR — Fourth match of the colour decision order: with `NO_COLOR` set (present, an empty value included) output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/stdout-not-terminal · CLI colour decision order · stdout is not a terminal — Sixth match of the colour decision order: when stdout is not a terminal output has no colour, no glyph and no SGR.
  contract/cli-colour-decision-order/term-dumb · CLI colour decision order · TERM=dumb — Fifth match of the colour decision order: with `TERM=dumb` output has no colour, no glyph and no SGR.
  contract/cli-glyphs/arrow · CLI glyphs · -> — The arrow glyph is `->`, with no inbound form.
  contract/cli-glyphs/field-separator · CLI glyphs · " - " — ` - ` separates parts inside one field.
  contract/cli-message-escaping/control-characters · CLI message-mode escaping · control characters — In `wait`, `last` and a non-RFC 3339 `since` of `viola link`, every decoded control character (C0, DEL, C1) except `\n` and `\t` prints as `\xHH`, two uppercase hex digits; nothing is stripped.
  contract/cli-separators/space-hyphen-space · CLI separators · ` - ` — Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).
  contract/cli-transfer-arrow/ascii-arrow · CLI transfer arrows · -> — The CLI prints transfers with ASCII `->` and has no inbound form; links print only as `link` / `unlink` verb output (`overseer -> builder`) and as the `<driver> -> <driven>` lines of `viola list`'s `-- LINKS --` block.
  contract/viola-last-output/message · viola last output · message — `last` prints the message text on stdout with C0/C1 escaped, or `no message` on stderr when the message is `null`.
  decision/ascii-mirror-words-always-print · ASCII mirror marks are words, not glyphs — A glyph is a non-ASCII symbol; the ASCII readback mirror `[RB]` / `[  ]` / `[/ ]` and `->` are words that print in every human output, piped, `NO_COLOR` and `TERM=dumb` included, and only `--json` replaces them with its JSON document.
  decision/ascii-only-human-output · ASCII-only human output — Human output is ASCII only (`->`, `[RB]`, `[/ ]`, `[  ]`).
  decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density.
  decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last` and `list` escapes C0/C1 control characters.
  decision/no-check-glyph-or-emoji-prefixes · No check-mark or emoji prefixes — Output never uses green check / red cross prefixes or emoji; the readback mirror `[RB]` / `[  ]` / `[/ ]` and the words `read back` / `unable` are the prefixes.
  decision/no-decorative-icons · No decorative icons — Icons clarify, not decorate: no robot or person icons, no status dots, no checkmarks and no warning triangles, since every such state is a printed word.
  decision/text-glyphs-only · Custom text glyphs only — Iconography is custom text glyphs only; phraseology words carry the meaning.
- S2 · data goes on stdout and messages on stderr; a --json result stays one stdout document · anti-patterns-never-do-these
  contract/cli-catch-site/json-internal-error · CLI catch site · --json internal error — Under `--json` the catch site prints the `internal-error` document on stdout in place of `error: internal error`, stderr then empty; without `--json` it prints that line, byte for byte.
  contract/cli-stderr/context-lines · CLI stderr · context lines — Context lines (`waiting: <name>`, the issue line of a send) go to stderr.
  contract/cli-stderr/error-line · CLI stderr · error line — Errors print on stderr as `error: …`.
  contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
  contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
  decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
  decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
  decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.
- S3 · no internals (paths, pids, compositor dispatch names) in any new error, refusal or hint line · anti-patterns-never-do-these
  decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
- S4 · no spinner or elapsed counter while a verify or a move is waiting; the step counter is the one progress pattern · surface-cli · surface
  decision/busy-not-animated · Busy has no animation — The `busy` state has no spinner, pulse or animation.
  decision/cli-has-no-motion · No motion in the CLI — The cli (0.2 on a TTY, 0.0 otherwise) has no motion at all.
  decision/cli-tty-static-expression · Static expression for the human CLI — Human TTY CLI output is at expression 0.2: static fixed-column rows and one static context header, nothing redrawn in place and no spinners.
  decision/no-spinner · No spinner or elapsed-time counter — The CLI shows no spinner and no elapsed-time counter.
  decision/no-spinners-in-verb-output · No spinners in verb output — A verb's output has no spinner, progress bar or live-redrawing dashboard; `wait` prints one static `waiting:` line and `verify` prints step-counter lines. · why: A spinner is progress presumed before it is confirmed.
  decision/one-progress-pattern · One progress pattern — A step counter with static appended stdout lines is the project's one progress pattern.
  decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
