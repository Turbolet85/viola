# layout-templates · the rows that bind 2026-11-16-local-command-delivery — picked 60 of 221 (45 by every draw) · rule any · pin 1879

## surface-cli · ## Surface: cli
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/prompt-text-from-stdin-or-file · Prompt text from stdin or --file — Prompt text comes from stdin or `--file`, never from a leading-slash argument, and the help of `send` says so.
decision/readback-mirror-no-colour · Readback mirror has no colour — The readback mirror is words and brackets only: `read back` is never green and `unable` is never red. · why: Colour on the CLI is reserved for `DIALOG` and `stale`.
decision/readback-mirror-signature · Readback mirror as the send signature — The readback mirror `[RB]` / `[  ]` / `[/ ]` is the first column of viola send output, in an issue line and an outcome line.
decision/send-lines-appended-never-redrawn · Send lines appended, never redrawn — viola send lines are appended and never redrawn: the issue line stays and the outcome is printed under it.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/mcp-tool-results/refusal · viola mcp tool results · refusal — Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
contract/mcp-tool-results/success · viola mcp tool results · success — Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
contract/readback-mirror/issue-and-outcome · the readback mirror line · issue and outcome lines — The TTY issue line on stderr is the open box and the outcome line under it is the closed box, the two lined up column for column.
contract/readback-mirror/layout · the readback mirror line · layout — A fixed first column holding the four-character box (`[RB]`, `[  ]`, `[/ ]`), one space, the padded state word (`open` / `read back` / `unable` / `unconfirmable`), the two-space gutter, NAME, then the outcome fields.
contract/readback-vocabulary/words · readback vocabulary · state words — The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
contract/viola-send-output/issue-line · viola send output · issue line — `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper.
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
contract/viola-send-output/piped-stdout · viola send output · piped stdout — When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror.
contract/viola-send-output/read-back · viola send output · read back — `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by a prompt-submitted whose prompt equals the typed text, or equals it with a trailing run of space, TAB, CR and LF removed; any leading or inner difference leaves the send unconfirmed.
contract/viola-send-output/read-back-clear · viola send output · read back for /clear — On a verified CLI version, /clear is read back by its new-session post-condition: a session-start with cause clear and a new session id.
contract/viola-send-output/read-back-compact · viola send output · read back for /compact — On a verified CLI version, /compact is read back by a session-start with cause compact and the session's own id inside its own 40 s window, else not-delivered / no-prompt-submitted.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
contract/viola-send-output/unconfirmable · viola send output · unconfirmable — `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, and /compact or /clear on an unverified CLI version.
contract/viola-send-output/word-column · viola send output · word column — The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row passes.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
contract/cli-token-rendering/motion-tokens · CLI token rendering · Motion tokens — None; nothing redraws in place.
contract/cli-token-rendering/radius · CLI token rendering · radius-* — No terminal analogue; square brackets `[ ]` keep the ruled-rectangle reading.
contract/cli-token-rendering/spacing · CLI token rendering · Spacing — The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
contract/mcp-tools/send · viola mcp tools · send — A driver's machine view of the viola send verb.
contract/viola-send-output/input-not-ready-hint · viola send output · input-not-ready hint — An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-wait-output/turn-ended · viola wait output · turn-ended — `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
