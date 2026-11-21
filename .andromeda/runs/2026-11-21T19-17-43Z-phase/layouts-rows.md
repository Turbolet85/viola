# layout-templates · the rows that bind 2026-11-21-mcp-calls-bounded — picked 72 of 223 (26 by every draw) · rule any · pin 3707

## surface-cli · ## Surface: cli
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
contract/cli-verbs/last · CLI verbs · viola last <target> — Returns the last assistant message, with no spinner.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/cli-verbs/wait · CLI verbs · viola wait <target> — Returns one turn-boundary result, with no spinner.
contract/mcp-tool-results/invalid-params · viola mcp tool results · invalid-params — Invalid parameters refuse with the code `invalid-params` and a closed field word as `detail`.
contract/mcp-tool-results/refusal · viola mcp tool results · refusal — Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
contract/mcp-tools/answer · viola mcp tools · answer — A driver's machine view of the viola answer verb.
contract/mcp-tools/last · viola mcp tools · last — A driver's machine view of the viola last verb.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/mcp-tools/send · viola mcp tools · send — A driver's machine view of the viola send verb.
contract/mcp-tools/wait · viola mcp tools · wait — A driver's machine view of the viola wait verb.
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
contract/send-refusals/control-character · viola send refusals before any request · control-character — A text holding a refused character is refused before any frame: `[/ ] unable  <name>  not-delivered  control-character` on stderr, exit 13, its hint last; under `--json` one stdout document, no stderr line.
contract/send-refusals/empty-text · viola send refusals before any request · empty-text — A text empty once trailing CR and LF are removed is refused before any frame, exit 13, `[/ ] unable  <name>  not-delivered  empty-text` then `hint: the text is empty once its trailing newlines are removed; send a text with content`.
contract/viola-list-board/agents-read-failure · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no claude.
contract/viola-list-board/status-join · viola list board · STATUS — One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown.
contract/viola-send-output/read-back-compact · viola send output · read back for /compact — On a verified CLI version, /compact is read back by a session-start with cause compact and the session's own id inside its own 40 s window, else not-delivered / no-prompt-submitted.
contract/viola-send-output/read-back-context · viola send output · read back for /context — On a verified CLI version, /context is read back as `[RB] read back` when a new stdout tag lands in the session transcript after the send; /context submits no prompt and is never read back by a turn end.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row counts: it reads pass, or absent on the browser-tools-prompt row alone.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|absent|fail`, in ledger order, MM being the relied count; `absent` is the browser-tools-prompt row's word alone, when no typed run met the prompt; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line, `n` counting the rows that count, an `absent` browser-tools-prompt row among them; `fail` is never coloured.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
contract/viola-wait-output/timed-out · viola wait output · timed out — A timeout prints `timed out  <name>  <ms> ms` on stdout.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink.
decision/mcp-server-without-human-surface · MCP server with no human surface — viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
decision/mcp-verb-hidden · viola mcp is a hidden verb — `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/wrapper-repeats-send-refusals · Wrapper repeats the send text refusals — The wrapper makes the same control-character and empty-text refusals for a frame sent straight to it as `viola send` makes itself.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unknown`; an older wrapper's `budget-paused` reply also reads as `unknown`.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cursor/byte-offset · cursor · cursor — `cursor` is the byte offset `send` prints and `wait --after` consumes.
contract/frame-verb-hints/instance-not-running · frame verb exit-21 hints · instance not running — `hint: <name> is not running; viola list shows the live instances` under `unable  <name>  instance-unreachable` for `wait`, `last`, `link`, `unlink`, and `send` in its mirror.
contract/frame-verb-hints/not-wrapped · frame verb exit-21 hints · name with no instance directory — `hint: <name> is not a wrapped session; only sessions started with viola run take commands`, the last stderr line.
contract/frame-verb-hints/server-verify-failed · frame verb exit-21 hints · endpoint not the recorded wrapper — No frame written, exit 21, `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`; under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`, stderr empty.
contract/frame-verb-hints/socket-dir-refused · frame verb exit-21 hints · socket directory refused — No frame written, exit 21, `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`; under `--json` the `socket-dir-refused` detail, stderr empty.
contract/frame-verb-hints/wrapper-went-away · frame verb exit-21 hints · wrapper lost during the call — `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running`, the last stderr line.
contract/frame-verbs/strict-modes-refused · the frame verbs · strict-modes refusal — For send, wait, last, answer, pause, release, link and unlink: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
contract/instance-unreachable-detail/endpoint-lost · exit-21 --json detail · endpoint-lost — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/instance-dead · exit-21 --json detail · instance-dead — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/server-verify-failed · exit-21 --json detail · server-verify-failed — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/socket-dir-refused · exit-21 --json detail · socket-dir-refused — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/strict-modes-failed · exit-21 --json detail · strict-modes-failed — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/unwrapped-name · exit-21 --json detail · unwrapped-name — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/mcp-tool-results/success · viola mcp tool results · success — Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.
contract/viola-send-output/read-back · viola send output · read back — `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by a prompt-submitted whose prompt equals the typed text, or equals it with a trailing run of space, TAB, CR and LF removed; any leading or inner difference leaves the send unconfirmed.
contract/viola-send-output/read-back-clear · viola send output · read back for /clear — On a verified CLI version, /clear is read back by its new-session post-condition: a session-start with cause clear and a new session id.
contract/viola-send-output/read-back-mcp · viola send output · read back for /mcp — On a verified CLI version, /mcp, alone or with arguments, is read back as `[RB] read back` when a new stdout tag lands in the session transcript after the send, the transcript read from its end as it stood before the send.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
contract/viola-send-output/unconfirmable · viola send output · unconfirmable — `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, /cost and /usage; /compact, /clear, /mcp or /context on an unverified CLI version; /mcp when the session declared no readable transcript; /context before its transcript exists.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-verify-output/stop · viola verify output · stop — A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp.
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
contract/viola-verify-refusals/home-not-private · viola verify refusals · viola home not private — A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.
contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
contract/viola-wait-output/dialog · viola wait output · dialog — A dialog result prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout with DIALOG not coloured, as a result and not the board.
contract/viola-wait-output/dialog-expired · viola wait output · dialog-expired — A wait woken by the wrapper's record of an expired hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once.
contract/viola-wait-output/dialog-unknown · viola wait output · dialog unknown — A dialog line whose event carries no dialog_id reads `dialog unknown`.
contract/viola-wait-output/turn-ended · viola wait output · turn-ended — `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
