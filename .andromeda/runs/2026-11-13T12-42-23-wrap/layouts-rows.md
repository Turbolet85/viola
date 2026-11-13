# layout-templates · the rows 2026-11-13-epoch-8-cleanup touches — picked 16 of 224 · rule any · pin 945

## surface-cli · ## Surface: cli
decision/amber-means-dialog · Amber means DIALOG only — Amber means `DIALOG` and nothing else, and `stale` is SGR dim in the terminal. · r1
decision/argument-order-mirrors-channel · Argument order mirrors the channel params — CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`. · r1
decision/bay-line-unstyled · BAY line unstyled — The BAY line's label words and data figures are all in the default foreground, with no bold, no colour, no box drawing and no rule line under it. · r1
decision/board-ordered-by-name · Board ordered by name — viola list rows are ordered by name and nothing is re-sorted by state; NAME values are bold. · r1
decision/board-planning-forms-wording · Board planning forms as the ratified wording — The board's planning forms (a figure rounded down, the whole-minute age and `read unknown`, bare skip counts, `human - budget-paused`, the empty board's three lines) are its wording. · why: confirmed as built by the founder, superseding the operator's word · r1
decision/board-row-escaping · Board row escaping — Every board field derived from `claude agents --json` has C0 and C1 controls, newline and tab included, escaped as hex text, so a row never splits and the columns stay fixed-width. · why: Ratified; stricter than the security floor, which keeps newline and tab for `wait` and `last` text. · r1
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`. · r1
decision/clap-derive-cli-parsing · clap derive for CLI parsing — The viola command line is parsed with clap 4 in its derive form. · r1
decision/cli-field-separators · CLI field separators — ` - ` separates inside a CLI field and an arrow is `->`; there is no CLI inbound `<-` form. · r1
decision/cli-holds-every-action · The CLI holds every action — The CLI is where every action lives. · r1
decision/dialog-attention-only-on-board · DIALOG attention word only on the board — The amber DIALOG attention word appears only in viola list. · r1
decision/flat-verbs · Flat verbs plus a target — Each verb is one lower-case word with no nested noun hierarchy or subcommand tree. · except: `plugin install` is the one two-word verb. · r1
decision/hand-written-sgr-styling · Hand-written SGR styling module — Terminal styling and fixed-width column padding come from a small hand-written SGR module in the viola binary, with no colour or table crate. · r1
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`. · r1
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr. · r1
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs. · r1
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid. · r1
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output. · r1
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue. · r1
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`. · r1
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink. · r1
decision/mcp-server-without-human-surface · MCP server with no human surface — viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs. · r1
decision/mcp-verb-hidden · viola mcp is a hidden verb — `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row. · r1
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface. · r1
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused. · r1
decision/no-link-column · No link column on the board — The six shared board columns stay fixed; links are visible only in `link` / `unlink` verb output. · r1
decision/no-persistent-footer · No persistent footer — The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`. · r1
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`. · r1
decision/no-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output. · r1
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter. · r1
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where. · r1
decision/one-token-vocabulary-on-cli · One token vocabulary resolved on the cli — The one design token vocabulary (color-*, space-*, radius-*, the typography roles) resolves on the cli to the SGR tokens, the terminal default foreground or not used, through ANSI depth, the two-space gutter, ASCII `->` / ` - ` and brackets for the ruled box. · r1
decision/only-header-bay-line · Only header is the BAY line — The only header on any verb is the one static BAY line at the top of `viola list`. · r1
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control. · r1
decision/prompt-text-from-stdin-or-file · Prompt text from stdin or --file — Prompt text comes from stdin or `--file`, never from a leading-slash argument, and the help of `send` says so. · r1
decision/readback-mirror-no-colour · Readback mirror has no colour — The readback mirror is words and brackets only: `read back` is never green and `unable` is never red. · why: Colour on the CLI is reserved for `DIALOG` and `stale`. · r1
decision/readback-mirror-signature · Readback mirror as the send signature — The readback mirror `[RB]` / `[  ]` / `[/ ]` is the first column of viola send output, in an issue line and an outcome line. · r1
decision/send-lines-appended-never-redrawn · Send lines appended, never redrawn — viola send lines are appended and never redrawn: the issue line stays and the outcome is printed under it. · r1
decision/shared-captions-fixed · Shared captions fixed — The six captions, words and slot order are fixed, and the terminal fleet console reuses them. · r1
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone. · r1
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern. · r1
decision/spoken-phraseology-verb-output · Spoken phraseology for wheel, handoff and dialog verbs — The output of viola pause, release, link, unlink and answer is spoken phraseology. · r1
decision/std-isterminal-tty-detection · TTY detection through std IsTerminal — Whether a stream is a terminal is detected with the standard library's IsTerminal, with no crate. · r1
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed. · r1
decision/targets-are-wrapped-names · A CLI target is always a ViolaName — A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets. · r1
decision/truncate-only-unwrapped-names · Truncate only unwrapped names — Only unwrapped NAMEs are cut, with `...`, only where the terminal width is known and too narrow, never inside an escape; wrapped names are never clipped and no line ever wraps. · r1
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`. · r1
decision/viola-list-styling · viola list styling — On a colour terminal NAME is bold, the `DIALOG` word is accent amber with its kind word plain, a `stale` row is dim end to end with `DIALOG` still amber, nothing else is styled, and no SGR is written when colour is off. · r1
decision/violaname-never-case-transformed · ViolaName never case-transformed — `ViolaName` is a shared reference and is never case-transformed. · r1
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state. · r1
decision/wrapper-repeats-send-refusals · Wrapper repeats the send text refusals — The wrapper makes the same control-character and empty-text refusals for a frame sent straight to it as `viola send` makes itself. · r1
contract/bay-line/cell-order · the BAY line of viola list · cell order — `BAY`, `5H <n> %`, `7D <n> %` (each followed by `expired` when the window has passed), `read <age> ago`, `gate open` or `budget-paused`, `skipped a - b - c`, separated by the two-space gutter. · r1
contract/cli-catch-site/internal-error · CLI verbs outside a revive start · internal error — A failed or panicked verb prints exactly `error: internal error` once, from the one catch site, with no hint; with `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout instead, stderr empty. · r1
contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`. · r1
contract/cli-exit-codes/11 · CLI exit codes · 11 — `budget-paused`. · r1
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`. · r1
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`. · r1
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unknown`. · r1
contract/cli-exit-codes/20 · CLI exit codes · 20 — Wrapper fault, `error: wrapper fault  <code>`. · r1
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead. · r1
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text. · r1
contract/cli-global-flags/home · CLI global flags · --home — `--home` is the same on every verb. · r1
contract/cli-internal-error/human · CLI internal error · human output — A failure or a caught panic prints `error: internal error` once, exit 1, with no hint. · r1
contract/cli-internal-error/json · CLI internal error · --json output — Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1. · r1
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr. · r1
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`. · r1
contract/cli-refusals/instance-unreachable · CLI refusals · instance-unreachable — With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line. · r1
contract/cli-token-rendering/callsign-role · CLI token rendering · Callsign role — The callsign token: SGR bold on NAME values only. · r1
contract/cli-token-rendering/color-accent · CLI token rendering · color-accent — The design system's attention SGR token at truecolor, 256 or 16-colour depth, set only on the DIALOG word and always followed by the uncoloured kind word. · r1
contract/cli-token-rendering/color-role-handoff · CLI token rendering · color-role-handoff — Not used; link output is plain. · r1
contract/cli-token-rendering/color-surface · CLI token rendering · color-surface-* — Never set; the terminal background belongs to the user. · r1
contract/cli-token-rendering/color-text-muted · CLI token rendering · color-text-muted — The stale token: SGR dim across a whole stale row, and never without the word stale in LIVE. · r1
contract/cli-token-rendering/color-text-primary-secondary · CLI token rendering · color-text-primary / color-text-secondary — The terminal's default foreground; the terminal owns ink. · r1
contract/cli-token-rendering/data-role · CLI token rendering · Data role — Tabular figures and timestamps. · r1
contract/cli-token-rendering/label-role · CLI token rendering · Label role — Uppercase column captions (NAME LIVE STATUS WHEEL DIALOG CLI) and group labels, printed plain. · r1
contract/cli-token-rendering/motion-tokens · CLI token rendering · Motion tokens — None; nothing redraws in place. · r1
contract/cli-token-rendering/radius · CLI token rendering · radius-* — No terminal analogue; square brackets `[ ]` keep the ruled-rectangle reading. · r1
contract/cli-token-rendering/spacing · CLI token rendering · Spacing — The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator. · r1
contract/cli-verbs/help · CLI verbs · viola --help — The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `handoff:` holding link, unlink and `setup:` holding plugin install. · r1
contract/cli-verbs/last · CLI verbs · viola last <target> — Returns the last assistant message, with no spinner. · r1
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header. · r1
contract/cli-verbs/plugin-install · CLI verbs · viola plugin install — Writes an MCP-only local marketplace under `<home>/marketplace/` with no hooks, prints the two `claude plugin` commands that add it, and spawns no claude. · r1
contract/cli-verbs/revive · CLI verbs · viola revive <name> — Resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like viola run. · r1
contract/cli-verbs/revive-list · CLI verbs · viola revive <name> --list — Prints the instance's logged sessions, one static line each. · r1
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; prints nothing while the child runs. · r1
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable. · r1
contract/cli-verbs/wait · CLI verbs · viola wait <target> — Returns one turn-boundary result, with no spinner. · r1
contract/cursor/byte-offset · cursor · cursor — `cursor` is the byte offset `send` prints and `wait --after` consumes. · r1
contract/frame-verb-hints/instance-not-running · frame verb exit-21 hints · instance not running — `hint: <name> is not running; viola list shows the live instances` under `unable  <name>  instance-unreachable` for `wait`, `last`, `link`, `unlink`, and `send` in its mirror. · r1
contract/frame-verb-hints/not-wrapped · frame verb exit-21 hints · name with no instance directory — `hint: <name> is not a wrapped session; only sessions started with viola run take commands`, the last stderr line. · r1
contract/frame-verb-hints/server-verify-failed · frame verb exit-21 hints · endpoint not the recorded wrapper — No frame written, exit 21, `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`; under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`, stderr empty. · r1
contract/frame-verb-hints/socket-dir-refused · frame verb exit-21 hints · socket directory refused — No frame written, exit 21, `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`; under `--json` the `socket-dir-refused` detail, stderr empty. · r1
contract/frame-verb-hints/wrapper-went-away · frame verb exit-21 hints · wrapper lost during the call — `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running`, the last stderr line. · r1
contract/frame-verbs/strict-modes-refused · the frame verbs · strict-modes refusal — For send, wait, last, answer, pause, release, link and unlink: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty. · r1
contract/instance-unreachable-detail/endpoint-lost · exit-21 --json detail · endpoint-lost — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries. · r1
contract/instance-unreachable-detail/instance-dead · exit-21 --json detail · instance-dead — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries. · r1
contract/instance-unreachable-detail/server-verify-failed · exit-21 --json detail · server-verify-failed — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries. · r1
contract/instance-unreachable-detail/socket-dir-refused · exit-21 --json detail · socket-dir-refused — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries. · r1
contract/instance-unreachable-detail/strict-modes-failed · exit-21 --json detail · strict-modes-failed — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries. · r1
contract/instance-unreachable-detail/unwrapped-name · exit-21 --json detail · unwrapped-name — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries. · r1
contract/link-output/already-linked · viola link / unlink output · already linked — A pair already held prints `link    <driver> -> <driven>  already linked`, exit 0. · r1
contract/link-output/json-link · viola link / unlink output · link --json — Prints `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`. · r1
contract/link-output/json-unlink · viola link / unlink output · unlink --json — Prints `{"v":1,"ok":{}}`. · r1
contract/link-output/link · viola link / unlink output · link — Prints one stdout line in one write, plain in every mode, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, the reply's since as UTC clock time in whole seconds, a non-RFC 3339 since printed escaped. · r1
contract/link-output/printed-names · viola link / unlink output · printed names — The names printed are the verb's own arguments, never names read back from the reply. · r1
contract/link-output/unlink · viola link / unlink output · unlink — Prints `unlink  <driver> -> <driven>` in one write, exit 0, whether or not a pair was removed. · r1
contract/link-output/wrapper-error · viola link / unlink output · wrapper error — A wrapper error, a driver equal to driven (-32602) included, is the wrapper-fault line, exit 20. · r1
contract/mcp-tool-results/invalid-params · viola mcp tool results · invalid-params — Invalid parameters refuse with the code `invalid-params` and a closed field word as `detail`. · r1
contract/mcp-tool-results/refusal · viola mcp tool results · refusal — Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only. · r1
contract/mcp-tool-results/success · viola mcp tool results · success — Each success is isError false with the CLI verb's --json `ok` payload alone as structuredContent, without `v` or the `ok` wrapper. · r1
contract/mcp-tools/answer · viola mcp tools · answer — A driver's machine view of the viola answer verb. · r1
contract/mcp-tools/last · viola mcp tools · last — A driver's machine view of the viola last verb. · r1
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb. · r1
contract/mcp-tools/send · viola mcp tools · send — A driver's machine view of the viola send verb. · r1
contract/mcp-tools/wait · viola mcp tools · wait — A driver's machine view of the viola wait verb. · r1
contract/readback-mirror/issue-and-outcome · the readback mirror line · issue and outcome lines — The TTY issue line on stderr is the open box and the outcome line under it is the closed box, the two lined up column for column. · r1
contract/readback-mirror/layout · the readback mirror line · layout — A fixed first column holding the four-character box (`[RB]`, `[  ]`, `[/ ]`), one space, the padded state word (`open` / `read back` / `unable` / `unconfirmable`), the two-space gutter, NAME, then the outcome fields. · r1
contract/readback-vocabulary/words · readback vocabulary · state words — The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box. · r1
contract/release-budget-output/gate-lifted · viola release --budget output · gate lifted — When a pause stood, prints `<name>  budget gate lifted until <end>  wheel <holder>` on stdout, exit 0, wheel left where it is; `<end>` is `HH:MM:SSZ` on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` otherwise, a non-RFC 3339 value escaped. · r1
contract/release-budget-output/gate-open · viola release --budget output · gate open — When no pause stood, prints `<name>  budget gate open  wheel <holder>` on stdout, exit 0, with the wheel left where it is. · r1
contract/run-revive-start-json/internal-error · viola revive --json start refusals shared with run · internal-error — A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal. · r1
contract/run-revive-start-json/pinned-hash-mismatch · viola revive --json start refusals shared with run · pinned-hash-mismatch — After a passed preflight, a pinned-hash mismatch prints one exit-1 document with this code, stderr empty. · r1
contract/run-revive-start-json/socket-dir-refused · viola revive --json start refusals shared with run · socket-dir-refused — After a passed preflight, a refused socket directory prints one exit-1 document with this code, stderr empty; without `--json` the socket-directory pair. · r1
contract/run-revive-start-json/squatted-name · viola revive --json start refusals shared with run · squatted-name — After a passed preflight, a squatted name prints one exit-1 document with this code, stderr empty. · r1
contract/run-revive-start/home-tree-refused · viola run and viola revive starts · home tree refused — Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child. · r1
contract/run-revive-start/socket-dir-refused · viola run and viola revive starts · socket directory refused — Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`. · r1
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint. · r1
contract/send-refusals/control-character · viola send refusals before any request · control-character — A text holding a refused character is refused before any frame: `[/ ] unable  <name>  not-delivered  control-character` on stderr, exit 13, its hint last; under `--json` one stdout document, no stderr line. · r1
contract/send-refusals/empty-text · viola send refusals before any request · empty-text — A text empty once trailing CR and LF are removed is refused before any frame, exit 13, `[/ ] unable  <name>  not-delivered  empty-text` then `hint: the text is empty once its trailing newlines are removed; send a text with content`. · r1
contract/viola-answer-output/answered · viola answer output · answered — Prints `answered  <name>  dialog <id>`. · r1
contract/viola-answer-output/unknown-dialog · viola answer output · unknown-dialog — An answer to a dialog that is not pending prints `unable  <name>  not-delivered  unknown-dialog` on stderr, exit 13, then `hint: that dialog is not pending; viola list shows the current DIALOG`. · r1
contract/viola-help/usage-line · viola --help output · usage line — `viola --help` and `viola help` print exactly `Usage: viola [--home <dir>] <verb> [args]` as the first line, then a blank line. · r1
contract/viola-help/verb-groups · viola --help output · verb group rows — Five rows `board:    list`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink`, `setup:    run, revive, verify, plugin install`, served verbs only, plain ASCII, then a blank line, `Options:` and clap's options; no about line and no clap `Commands:` list. · r1
contract/viola-last-output/message · viola last output · message — The last assistant message prints on stdout with C0/C1 escaped, after a `last  <name>  <kind> <ts>` header on stderr on a TTY only; a null message prints `no message` on stderr. · r1
contract/viola-list-board/agents-read-failure · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no claude. · r1
contract/viola-list-board/cli-unknown · viola list board · CLI unknown — A row with no recorded version reads CLI `unknown`. · r1
contract/viola-list-board/colour-depth · viola list board · colour depth — On a terminal that takes colour, the depth is truecolor, 256 or 16, read from COLORTERM / TERM. · r1
contract/viola-list-board/column-widths · viola list board · column widths — NAME is padded to the longer of 4 and the longest name over both racks in characters, LIVE to 5, STATUS to 7, WHEEL to 23, DIALOG to 17, CLI last and unpadded; no line ends in a space. · r1
contract/viola-list-board/dialog-unknown · viola list board · DIALOG unknown — A pending dialog of unknown kind reads `DIALOG unknown`. · r1
contract/viola-list-board/empty-board · viola list board · empty board — An empty board prints `no wrapped sessions  start one: viola run <name> -- claude`. · r1
contract/viola-list-board/gate-closed · viola list board · closed budget gate — A closed budget gate prints `budget-paused` in place of `gate open`. · r1
contract/viola-list-board/name-truncation · viola list board · NAME truncation — With the terminal width known (from the host pty size, never COLUMNS), an unwrapped NAME longer than the column (capped at the width less 65) is cut to the column less 3 plus `...`, never inside a `\xHH` escape; a wrapped NAME is never cut. · r1
contract/viola-list-board/plain-lines · viola list board · plain lines — The BAY line, caption, empty line and separator print plain. · r1
contract/viola-list-board/single-write · viola list board · write order — viola list prints as one stdout write: the BAY line, the caption row, then one row per wrapped session in name order (or the empty line), then the unwrapped rack when there is one. · r1
contract/viola-list-board/stale-row-dim · viola list board · stale row — A wrapped stale row is dim from its first byte to its last, NAME dim and bold. · r1
contract/viola-list-board/stale-row-with-dialog · viola list board · stale row with a dialog — On a cocked and stale row, the DIALOG word stays amber and not dim, while the kind word and the rest of the row stay dim. · r1
contract/viola-list-board/status-join · viola list board · STATUS — One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown. · r1
contract/viola-list-board/uncoloured-modes · viola list board · uncoloured modes — The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb. · r1
contract/viola-list-board/unknown-width · viola list board · unknown terminal width — Nothing is truncated when the terminal width is unknown. · r1
contract/viola-list-board/unwrapped-rack · viola list board · unwrapped rack — Unjoined agent rows print after the wrapped rows under `-- UNWRAPPED - READ-ONLY --`, sorted by name: NAME the escaped sessionId, LIVE live, its STATUS, n/a in WHEEL, DIALOG and CLI; with no unwrapped row no separator prints. · r1
contract/viola-list-board/wheel-unknown · viola list board · WHEEL unknown — A replayed row with no logged holder reads WHEEL `unknown`. · r1
contract/viola-list-board/window-expired · viola list board · passed window — A passed budget window prints with `expired` after its figure, as `5H 62 % expired`. · r1
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item, budget and skipped shapes are the list payload contract. · r1
contract/viola-list-json/strict-modes-failed · viola list --json · strict-modes-failed — A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found. · r1
contract/viola-list/columns · viola list · columns — NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, each padded to its widest word, with the two-space gutter between columns and no box-drawing borders. · r1
contract/viola-list/strict-modes-refused · viola list · strict-modes refusal — Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance. · r1
contract/viola-list/unwrapped-separator · viola list · rack separator — `-- UNWRAPPED - READ-ONLY --` on its own line before the unwrapped rows, written only when unwrapped rows exist; the wrapped rack is always first, with no separator. · r1
contract/viola-plugin-install/bare-plugin · viola plugin install · bare viola plugin — `viola plugin` alone is a clap usage error, exit 2; clap serves its own `viola plugin help [install]`. · r1
contract/viola-plugin-install/dir-quoting · viola plugin install · <dir> quoting — `<dir>` is the canonical home joined with `marketplace`, single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :`, else printed bare. · r1
contract/viola-plugin-install/internal-error · viola plugin install · internal error — A failed exe read, dir creation or file write is `error: internal error`, exit 1, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty. · r1
contract/viola-plugin-install/json-ok · viola plugin install · --json ok document — Under `--json` it prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`, no path. · r1
contract/viola-plugin-install/output · viola plugin install · human output — After writing the marketplace tree, four lines in one plain-ASCII stdout write, no SGR, never redrawn: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`. · r1
contract/viola-plugin-install/strict-modes-refused · viola plugin install · strict-modes refusal — A refused home or `marketplace/` tree dir is exit 21 with the home pair on stderr, stdout empty, nothing created, no log opened; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty. · r1
contract/viola-revive-json/already-live · viola revive --json error codes · already-live — The code for a live holder or a fresh replayed beat. · r1
contract/viola-revive-json/cwd-missing · viola revive --json error codes · cwd-missing — The code for a missing recorded directory. · r1
contract/viola-revive-json/list-document · viola revive --json · --list document — `--list --json` prints the rows as one document `{"v":1,"ok":{"sessions":[{"ts":…,"cause":…,"id":…}]}}` in log order, fields as the human rows, or the `no-session` document with exit 1. · r1
contract/viola-revive-json/no-session · viola revive --json error codes · no-session — The code for both no-logged-session pairs: no logged session, and no logged session with that id. · r1
contract/viola-revive-json/refusal-document · viola revive --json · refusal document — Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical. · r1
contract/viola-revive-json/session-live · viola revive --json error codes · session-live — The code for a session open in another claude. · r1
contract/viola-revive-json/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The code for a stale holder. · r1
contract/viola-revive-json/strict-modes-failed · viola revive --json error codes · strict-modes-failed — The code for a refused home tree or instance state files. · r1
contract/viola-revive-refusals/already-live · viola revive refusals · already live — `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged. · r1
contract/viola-revive-refusals/directory-missing · viola revive refusals · recorded directory missing — `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id. · r1
contract/viola-revive-refusals/no-logged-session · viola revive refusals · no logged session — `unable: <name> has no logged session to resume` / `hint: start one: viola run <name> -- claude`. · r1
contract/viola-revive-refusals/no-session-with-id · viola revive refusals · no logged session with that id — `unable: <name> has no logged session with that id` / `hint: viola revive <name> --list`. · r1
contract/viola-revive-refusals/session-open-elsewhere · viola revive refusals · session open in another claude — `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`. · r1
contract/viola-revive-refusals/state-files-writable · viola revive refusals · state files writable by another user — `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`. · r1
contract/viola-revive/list-cause · viola revive · --list cause field — The cause in a `viola revive --list` row is `startup`, `clear`, `resume` or `compact` as logged, and `unknown` otherwise. · r1
contract/viola-revive/list-refusals · viola revive refusals · --list refusals — `viola revive --list` has two of the refusals: the state-files pair and the no-logged-session pair. · r1
contract/viola-revive/list-row · viola revive · --list row — `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr. · r1
contract/viola-revive/malformed-id · viola revive · malformed --id — A malformed `--id` is clap's usage error, exit 2. · r1
contract/viola-revive/preflight-order · viola revive start · preflight order — The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory. · r1
contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout. · r1
contract/viola-revive/session-live-check · viola revive start · session-live check — One `claude agents --json` read; a row whose `sessionId` is the logged id refuses; skipped under `--fork`; a failed read passes and the start goes on. · r1
contract/viola-revive/start-output · viola revive · passed start output — A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`. · r1
contract/viola-revive/usage · viola revive · usage — `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` or `viola revive <name> --list`, each with `[--json]`; `--list` conflicts with `--id`, `--fork` and the trailing arguments. · r1
contract/viola-run/already-live · viola run start refusals · already live — Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids. · r1
contract/viola-run/start-output · viola run start · passed start output — `viola run` prints nothing of its own; the terminal belongs to the wrapped claude TUI until it exits. · r1
contract/viola-send-output/input-not-ready-hint · viola send output · input-not-ready hint — An unable not-delivered input-not-ready outcome ends stderr with `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`. · r1
contract/viola-send-output/issue-line · viola send output · issue line — `[  ] open …` is printed on stderr, only on a TTY, when the request goes to the wrapper. · r1
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document. · r1
contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line. · r1
contract/viola-send-output/piped-stdout · viola send output · piped stdout — When stdout is piped the TTY-only issue line is dropped and the outcome line alone carries the mirror. · r1
contract/viola-send-output/read-back · viola send output · read back — `[RB] read back …` prints on stdout with exit 0 when the send is confirmed by the matching prompt-submitted. · r1
contract/viola-send-output/read-back-clear · viola send output · read back for /clear — On a verified CLI version, /clear is read back by its new-session post-condition: a session-start with cause clear and a new session id. · r1
contract/viola-send-output/read-back-compact · viola send output · read back for /compact — On a verified CLI version, /compact is read back by a session-start with cause compact and the session's own id inside its own 40 s window, else not-delivered / no-prompt-submitted. · r1
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/11/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text). · r1
contract/viola-send-output/unconfirmable · viola send output · unconfirmable — `[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, and /compact or /clear on an unverified CLI version. · r1
contract/viola-send-output/word-column · viola send output · word column — The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column. · r1
contract/viola-verb-help/usage · viola <verb> --help output · per-verb usage — `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable. · r1
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row passes. · r1 · stale
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop. · r1 · stale
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp. · r1 · stale
contract/viola-verify-output/help-paragraph · viola verify output · --help — viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe. · r1 · stale
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour. · r1 · stale
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line. · r1 · stale
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw. · r1 · stale
contract/viola-verify-output/stop · viola verify output · stop — A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp. · r1 · stale
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured. · r1 · stale
contract/viola-verify-output/usage · viola verify output · usage — `viola verify [--record <DIR>] [-- <program> [args…]]`, the program defaulting to claude. · r1 · stale
contract/viola-verify-refusals/cli-not-found · viola verify refusals · claude CLI not found — Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`. · r1 · stale
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`. · r1 · stale
contract/viola-verify-refusals/home-not-private · viola verify refusals · viola home not private — A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary. · r1 · stale
contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr. · r1 · stale
contract/viola-verify-refusals/version-unreadable · viola verify refusals · CLI version unreadable — Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`. · r1 · stale
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary. · r1 · stale
contract/viola-wait-output/dialog · viola wait output · dialog — A dialog result prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout with DIALOG not coloured, as a result and not the board. · r1
contract/viola-wait-output/dialog-expired · viola wait output · dialog-expired — A wait woken by the wrapper's record of an expired hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once. · r1
contract/viola-wait-output/dialog-unknown · viola wait output · dialog unknown — A dialog line whose event carries no dialog_id reads `dialog unknown`. · r1
contract/viola-wait-output/timed-out · viola wait output · timed out — A timeout prints `timed out  <name>  <ms> ms` on stdout. · r1
contract/viola-wait-output/turn-ended · viola wait output · turn-ended — `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form. · r1
contract/viola-wait-output/waiting-line · viola wait output · waiting line — `waiting: <name>` is printed once, on stderr, only when stderr is a terminal and --json is not set. · r1
contract/wheel-verb-output/pause · wheel verb output · viola pause — Prints `<name>  wheel human  manual-pause  I have control`. · r1
contract/wheel-verb-output/release · wheel verb output · viola release — Prints `<name>  wheel driver  you have control`. · r1
absence/cli-deep-links — Deep links or URL coupling in the CLI · r1
