# layout-templates · the rows that bind 2026-11-13-verify-readiness-on-the-input-box — picked 51 of 224 (39 by every draw) · rule any · pin 803

## surface-cli · ## Surface: cli
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/no-persistent-footer · No persistent footer — The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/mcp-tool-results/invalid-params · viola mcp tool results · invalid-params — Invalid parameters refuse with the code `invalid-params` and a closed field word as `detail`.
contract/mcp-tool-results/refusal · viola mcp tool results · refusal — Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
contract/mcp-tool-results/success · viola mcp tool results · success — Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
contract/mcp-tools/answer · viola mcp tools · answer — A driver's machine view of the viola answer verb.
contract/mcp-tools/last · viola mcp tools · last — A driver's machine view of the viola last verb.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/mcp-tools/send · viola mcp tools · send — A driver's machine view of the viola send verb.
contract/mcp-tools/wait · viola mcp tools · wait — A driver's machine view of the viola wait verb.
contract/readback-vocabulary/words · readback vocabulary · state words — The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
contract/viola-send-output/read-back · viola send output · read back — `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by the matching prompt-submitted.
contract/viola-send-output/read-back-clear · viola send output · read back for /clear — On a verified CLI version, /clear is read back by its new-session post-condition: a session-start with cause clear and a new session id.
contract/viola-send-output/read-back-compact · viola send output · read back for /compact — On a verified CLI version, /compact is read back by a session-start with cause compact and the session's own id inside its own 40 s window, else not-delivered / no-prompt-submitted.
contract/viola-send-output/unconfirmable · viola send output · unconfirmable — `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, and /compact or /clear on an unverified CLI version.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row passes.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/stop · viola verify output · stop — A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
contract/viola-verify-refusals/cli-not-found · viola verify refusals · claude CLI not found — Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`.
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
contract/viola-verify-refusals/version-unreadable · viola verify refusals · CLI version unreadable — Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink.
decision/mcp-server-without-human-surface · MCP server with no human surface — viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
decision/readback-mirror-no-colour · Readback mirror has no colour — The readback mirror is words and brackets only: `read back` is never green and `unable` is never red. · why: Colour on the CLI is reserved for `DIALOG` and `stale`.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-token-rendering/motion-tokens · CLI token rendering · Motion tokens — None; nothing redraws in place.
contract/viola-send-output/input-not-ready-hint · viola send output · input-not-ready hint — An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
contract/viola-verify-output/help-paragraph · viola verify output · --help — viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
contract/viola-verify-output/usage · viola verify output · usage — `viola verify [--record <DIR>] [-- <program> [args…]]`, the program defaulting to claude.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
