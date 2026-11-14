# layout-templates · the rows that bind 2026-11-14-budget-gate-removed — picked 48 of 225 (22 by every draw) · rule any · pin 1142

## surface-cli · ## Surface: cli
decision/bay-line-unstyled · BAY line unstyled — The BAY line's label words and data figures are all in the default foreground, with no bold, no colour, no box drawing and no rule line under it.
decision/board-planning-forms-wording · Board planning forms as the ratified wording — The board's planning forms (a figure rounded down, the whole-minute age and `read unknown`, bare skip counts, `human - budget-paused`, the empty board's three lines) are its wording. · why: confirmed as built by the founder, superseding the operator's word
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
contract/bay-line/cell-order · the BAY line of viola list · cell order — `BAY`, `5H <n> %`, `7D <n> %` (each followed by `expired` when the window has passed), `read <age> ago`, `gate open` or `budget-paused`, `skipped a - b - c`, separated by the two-space gutter.
contract/cli-exit-codes/11 · CLI exit codes · 11 — `budget-paused`.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/mcp-tools/send · viola mcp tools · send — A driver's machine view of the viola send verb.
contract/release-budget-output/gate-lifted · viola release --budget output · gate lifted — When a pause stood, prints `<name>  budget gate lifted until <end>  wheel <holder>` on stdout, exit 0, wheel left where it is; `<end>` is `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` otherwise, a non-RFC 3339 value escaped.
contract/release-budget-output/gate-open · viola release --budget output · gate open — When no pause stood, prints `<name>  budget gate open  wheel <holder>` on stdout, exit 0, with the wheel left where it is.
contract/viola-list-board/gate-closed · viola list board · closed budget gate — A closed budget gate prints `budget-paused` in place of `gate open`.
contract/viola-list-board/window-expired · viola list board · passed window — A passed budget window prints with `expired` after its figure, as `5H 62 % expired`.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item, budget and skipped shapes are the list payload contract.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/11/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
contract/viola-verb-help/usage · viola <verb> --help output · per-verb usage — `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/only-header-bay-line · Only header is the BAY line — The only header on any verb is the one static BAY line at the top of `viola list`.
decision/spoken-phraseology-verb-output · Spoken phraseology for wheel, handoff and dialog verbs — The output of viola pause, release, link, unlink and answer is spoken phraseology.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
decision/wrapper-repeats-send-refusals · Wrapper repeats the send text refusals — The wrapper makes the same control-character and empty-text refusals for a frame sent straight to it as `viola send` makes itself.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unknown`.
contract/cli-exit-codes/20 · CLI exit codes · 20 — Wrapper fault, `error: wrapper fault  <code>`.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead.
contract/mcp-tool-results/success · viola mcp tool results · success — Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper.
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
contract/send-refusals/control-character · viola send refusals before any request · control-character — A text holding a refused character is refused before any frame: `[/ ] unable  <name>  not-delivered  control-character` on stderr, exit 13, its hint last; under `--json` one stdout document, no stderr line.
contract/send-refusals/empty-text · viola send refusals before any request · empty-text — A text empty once trailing CR and LF are removed is refused before any frame, exit 13, `[/ ] unable  <name>  not-delivered  empty-text` then `hint: the text is empty once its trailing newlines are removed; send a text with content`.
contract/viola-list-board/column-widths · viola list board · column widths — NAME is padded to the longer of 4 and the longest name over both racks in characters, LIVE to 5, STATUS to 7, WHEEL to 23, DIALOG to 17, CLI last and unpadded; no line ends in a space.
contract/viola-list-board/name-truncation · viola list board · NAME truncation — With the terminal width known (from the host pty size, never COLUMNS), an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut.
contract/viola-list-board/plain-lines · viola list board · plain lines — The BAY line, caption, empty line and separator print plain.
contract/viola-list-board/single-write · viola list board · write order — viola list prints as one stdout write: the BAY line, the caption row, then one row per wrapped session in name order (or the empty line), then the unwrapped rack when there is one.
contract/viola-list-board/wheel-unknown · viola list board · WHEEL unknown — A replayed row with no logged holder reads WHEEL `unknown`.
contract/viola-list/columns · viola list · columns — NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, each padded to its widest word, with the two-space gutter between columns and no box-drawing borders.
contract/viola-revive/preflight-order · viola revive start · preflight order — The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
contract/viola-send-output/input-not-ready-hint · viola send output · input-not-ready hint — An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`.
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
contract/wheel-verb-output/release · wheel verb output · viola release — Prints `<name>  wheel driver  you have control`.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
