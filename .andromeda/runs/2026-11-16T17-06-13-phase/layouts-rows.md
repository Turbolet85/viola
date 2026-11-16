# layout-templates · the rows that bind 2026-11-16-granular-api-error-cause — picked 33 of 222 (17 by every draw) · rule any · pin 2030

## surface-cli · ## Surface: cli
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cli-verbs/wait · CLI verbs · viola wait <target> — Returns one turn-boundary result, with no spinner.
contract/mcp-tool-results/refusal · viola mcp tool results · refusal — Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
contract/mcp-tool-results/success · viola mcp tool results · success — Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
contract/mcp-tools/last · viola mcp tools · last — A driver's machine view of the viola last verb.
contract/mcp-tools/wait · viola mcp tools · wait — A driver's machine view of the viola wait verb.
contract/readback-vocabulary/words · readback vocabulary · state words — The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
contract/viola-last-output/message · viola last output · message — The last assistant message prints on stdout with C0/C1 escaped, after a `last  <name>  <kind> <ts>` header on stderr on a TTY only; a null message prints `no message` on stderr.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-wait-output/turn-ended · viola wait output · turn-ended — `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form.
decision/amber-means-dialog · Amber means DIALOG only — Amber means `DIALOG` and nothing else, and `stale` is SGR dim in the terminal.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/viola-list-styling · viola list styling — On a colour terminal NAME is bold, the `DIALOG` word is accent amber with its kind word plain, a `stale` row is dim end to end with `DIALOG` still amber, nothing else is styled, and no SGR is written when colour is off.
contract/cli-exit-codes/20 · CLI exit codes · 20 — Wrapper fault, `error: wrapper fault  <code>`.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-verbs/last · CLI verbs · viola last <target> — Returns the last assistant message, with no spinner.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/viola-list-json/strict-modes-failed · viola list --json · strict-modes-failed — A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found.
contract/viola-list/strict-modes-refused · viola list · strict-modes refusal — Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
