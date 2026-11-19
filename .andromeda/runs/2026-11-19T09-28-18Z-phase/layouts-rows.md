# layout-templates · the rows that bind 2026-11-19-the-2-1-328-recorded-fixture-set — picked 67 of 222 (45 by every draw) · rule any · pin 2980

## surface-cli · ## Surface: cli
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink.
decision/mcp-server-without-human-surface · MCP server with no human surface — viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
decision/mcp-verb-hidden · viola mcp is a hidden verb — `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/prompt-text-from-stdin-or-file · Prompt text from stdin or --file — Prompt text comes from stdin or `--file`, never from a leading-slash argument, and the help of `send` says so.
decision/readback-mirror-signature · Readback mirror as the send signature — The readback mirror `[RB]` / `[  ]` / `[/ ]` is the first column of viola send output, in an issue line and an outcome line.
decision/send-lines-appended-never-redrawn · Send lines appended, never redrawn — viola send lines are appended and never redrawn: the issue line stays and the outcome is printed under it.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/targets-are-wrapped-names · A CLI target is always a ViolaName — A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-global-flags/home · CLI global flags · --home — `--home` is the same on every verb.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/mcp-tool-results/invalid-params · viola mcp tool results · invalid-params — Invalid parameters refuse with the code `invalid-params` and a closed field word as `detail`.
contract/mcp-tool-results/refusal · viola mcp tool results · refusal — Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
contract/mcp-tool-results/success · viola mcp tool results · success — Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
contract/mcp-tools/answer · viola mcp tools · answer — A driver's machine view of the viola answer verb.
contract/mcp-tools/last · viola mcp tools · last — A driver's machine view of the viola last verb.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/mcp-tools/send · viola mcp tools · send — A driver's machine view of the viola send verb.
contract/mcp-tools/wait · viola mcp tools · wait — A driver's machine view of the viola wait verb.
contract/run-revive-start/home-tree-refused · viola run and viola revive starts · home tree refused — Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
contract/viola-run/start-output · viola run start · passed start output — `viola run` prints nothing of its own, and the start's trust step adds no refusal, exit code, hint, `--json` code or terminal byte; the terminal belongs to the wrapped claude TUI until it exits.
contract/viola-send-output/read-back · viola send output · read back — `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by a prompt-submitted whose prompt equals the typed text, or equals it with a trailing run of space, TAB, CR and LF removed; any leading or inner difference leaves the send unconfirmed.
contract/viola-send-output/read-back-mcp · viola send output · read back for /mcp — On a verified CLI version, /mcp, alone or with arguments, is read back as `[RB] read back` when a new stdout tag lands in the session transcript after the send, the transcript read from its end as it stood before the send.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
contract/viola-send-output/unconfirmable · viola send output · unconfirmable — `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, /cost and /usage; /compact, /clear or /mcp on an unverified CLI version; and /mcp when the session declared no readable transcript.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row counts: it reads pass, or absent on the browser-tools-prompt row alone.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|absent|fail`, in ledger order, MM being the relied count; `absent` is the browser-tools-prompt row's word alone, when no typed run met the prompt; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/stop · viola verify output · stop — A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line, `n` counting the rows that count, an `absent` browser-tools-prompt row among them; `fail` is never coloured.
contract/viola-verify-output/usage · viola verify output · usage — `viola verify [--record <DIR>] [-- <program> [args…]]`, the program defaulting to claude.
contract/viola-verify-refusals/cli-not-found · viola verify refusals · claude CLI not found — Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`.
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
contract/viola-verify-refusals/home-not-private · viola verify refusals · viola home not private — A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.
contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
contract/viola-verify-refusals/version-unreadable · viola verify refusals · CLI version unreadable — Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
decision/argument-order-mirrors-channel · Argument order mirrors the channel params — CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/only-header-bay-line · Only header is the BAY line — The only header on any verb is the one static BAY line at the top of `viola list`.
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unknown`; an older wrapper's `budget-paused` reply also reads as `unknown`.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/readback-mirror/issue-and-outcome · the readback mirror line · issue and outcome lines — The TTY issue line on stderr is the open box and the outcome line under it is the closed box, the two lined up column for column.
contract/readback-mirror/layout · the readback mirror line · layout — A fixed first column holding the four-character box (`[RB]`, `[  ]`, `[/ ]`), one space, the padded state word (`open` / `read back` / `unable` / `unconfirmable`), the two-space gutter, NAME, then the outcome fields.
contract/readback-vocabulary/words · readback vocabulary · state words — The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
contract/viola-send-output/issue-line · viola send output · issue line — `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper.
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
contract/viola-send-output/piped-stdout · viola send output · piped stdout — When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror.
contract/viola-send-output/read-back-clear · viola send output · read back for /clear — On a verified CLI version, /clear is read back by its new-session post-condition: a session-start with cause clear and a new session id.
contract/viola-send-output/read-back-compact · viola send output · read back for /compact — On a verified CLI version, /compact is read back by a session-start with cause compact and the session's own id inside its own 40 s window, else not-delivered / no-prompt-submitted.
contract/viola-send-output/word-column · viola send output · word column — The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
contract/viola-verify-output/help-paragraph · viola verify output · --help — viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
