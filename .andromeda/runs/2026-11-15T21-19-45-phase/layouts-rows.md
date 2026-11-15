# layout-templates · the rows that bind 2026-11-15-api-error-turn-end — picked 39 of 221 (25 by every draw) · rule any · pin 1610

## surface-cli · ## Surface: cli
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-verbs/last · CLI verbs · viola last <target> — Returns the last assistant message, with no spinner.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/cli-verbs/wait · CLI verbs · viola wait <target> — Returns one turn-boundary result, with no spinner.
contract/mcp-tools/wait · viola mcp tools · wait — A driver's machine view of the viola wait verb.
contract/viola-last-output/message · viola last output · message — The last assistant message prints on stdout with C0/C1 escaped, after a `last  <name>  <kind> <ts>` header on stderr on a TTY only; a null message prints `no message` on stderr.
contract/viola-list-board/status-join · viola list board · STATUS — One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row passes.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
contract/viola-wait-output/turn-ended · viola wait output · turn-ended — `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-verbs/plugin-install · CLI verbs · viola plugin install — Writes an MCP-only local marketplace under `<home>/marketplace/` with no hooks, prints the two `claude plugin` commands that add it, and spawns no claude.
contract/cursor/byte-offset · cursor · cursor — `cursor` is the byte offset `send` prints and `wait --after` consumes.
contract/mcp-tools/last · viola mcp tools · last — A driver's machine view of the viola last verb.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/mcp-tools/send · viola mcp tools · send — A driver's machine view of the viola send verb.
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-verify-refusals/home-not-private · viola verify refusals · viola home not private — A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
