# layout-templates · the rows that bind 2026-11-24-viola-focus — picked 74 of 231 (47 by every draw) · rule any · pin 4392

## surface-cli · ## Surface: cli
decision/argument-order-mirrors-channel · Argument order mirrors the channel params — CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/clap-derive-cli-parsing · clap derive for CLI parsing — The viola command line is parsed with clap 4 in its derive form.
decision/cli-holds-every-action · The CLI holds every action — The CLI is where every action lives.
decision/flat-verbs · Flat verbs plus a target — Each verb is one lower-case word with no nested noun hierarchy or subcommand tree. · except: `plugin install` is the one two-word verb.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/no-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/std-isterminal-tty-detection · TTY detection through std IsTerminal — Whether a stream is a terminal is detected with the standard library's IsTerminal, with no crate.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/targets-are-wrapped-names · A CLI target is always a ViolaName — A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets.
decision/violaname-never-case-transformed · ViolaName never case-transformed — `ViolaName` is a shared reference and is never case-transformed.
contract/cli-catch-site/internal-error · CLI verbs outside a revive start · internal error — A failed or panicked verb prints exactly `error: internal error` once, from the one catch site, with no hint; with `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout instead, stderr empty.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`), never reused; no refusal maps to 11.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-global-flags/home · CLI global flags · --home — `--home` is the same on every verb.
contract/cli-internal-error/human · CLI internal error · human output — A failure or a caught panic prints `error: internal error` once, exit 1, with no hint.
contract/cli-internal-error/json · CLI internal error · --json output — Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-refusals/instance-unreachable · CLI refusals · instance-unreachable — With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line.
contract/cli-verbs/help · CLI verbs · viola --help — The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `handoff:` holding link, unlink and `setup:` holding open, run, revive, verify and plugin install.
contract/frame-verb-hints/instance-not-running · frame verb exit-21 hints · instance not running — `hint: <name> is not running; viola list shows the live instances` under `unable  <name>  instance-unreachable` for `wait`, `last`, `link`, `unlink`, and `send` in its mirror.
contract/frame-verb-hints/not-wrapped · frame verb exit-21 hints · name with no instance directory — `hint: <name> is not a wrapped session; only sessions started with viola run take commands`, the last stderr line.
contract/frame-verbs/strict-modes-refused · the frame verbs · strict-modes refusal — For send, wait, last, answer, pause, release, link and unlink: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
contract/instance-unreachable-detail/endpoint-lost · exit-21 --json detail · endpoint-lost — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/instance-dead · exit-21 --json detail · instance-dead — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/server-verify-failed · exit-21 --json detail · server-verify-failed — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/strict-modes-failed · exit-21 --json detail · strict-modes-failed — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/instance-unreachable-detail/unwrapped-name · exit-21 --json detail · unwrapped-name — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/viola-help/verb-groups · viola --help output · verb group rows — Five rows `board:    list`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink`, `setup:    open, run, revive, verify, plugin install`, served verbs only, plain ASCII, then a blank line, `Options:` and clap's options; no about line and no clap `Commands:` list.
contract/viola-list-json/strict-modes-failed · viola list --json · strict-modes-failed — A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found.
contract/viola-list/strict-modes-refused · viola list · strict-modes refusal — Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance.
contract/viola-plugin-install/internal-error · viola plugin install · internal error — A failed exe read, dir creation or file write is `error: internal error`, exit 1, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty.
contract/viola-send-output/word-column · viola send output · word column — The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
contract/viola-verb-help/usage · viola <verb> --help output · per-verb usage — `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
decision/mcp-verb-hidden · viola mcp is a hidden verb — `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row.
decision/no-persistent-footer · No persistent footer — The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
decision/only-header-bay-line · Only header is the BAY line — The only header on any verb is the one static BAY line at the top of `viola list`.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unknown`; an older wrapper's `budget-paused` reply also reads as `unknown`.
contract/cli-exit-codes/20 · CLI exit codes · 20 — Wrapper fault, `error: wrapper fault  <code>`.
contract/cli-token-rendering/label-role · CLI token rendering · Label role — Uppercase column captions (NAME LIVE STATUS WHEEL DIALOG CLI) and group labels, printed plain.
contract/cli-token-rendering/motion-tokens · CLI token rendering · Motion tokens — None; nothing redraws in place.
contract/cli-token-rendering/spacing · CLI token rendering · Spacing — The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
contract/frame-verb-hints/server-verify-failed · frame verb exit-21 hints · endpoint not the recorded wrapper — No frame written, exit 21, `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`; under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`, stderr empty.
contract/instance-unreachable-detail/socket-dir-refused · exit-21 --json detail · socket-dir-refused — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/run-revive-start-json/internal-error · viola revive --json start refusals shared with run · internal-error — A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
contract/run-revive-start/home-tree-refused · viola run and viola revive starts · home tree refused — Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
contract/viola-help/usage-line · viola --help output · usage line — `viola --help` and `viola help` print exactly `Usage: viola [--home <dir>] <verb> [args]` as the first line, then a blank line.
contract/viola-list-board/uncoloured-modes · viola list board · uncoloured modes — The board is uncoloured under --json, a piped stdout, NO_COLOR (an empty value included) or TERM=dumb.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-revive-json/refusal-document · viola revive --json · refusal document — Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
contract/viola-revive-json/strict-modes-failed · viola revive --json error codes · strict-modes-failed — The code for a refused home tree or instance state files.
contract/viola-revive-refusals/state-files-writable · viola revive refusals · state files writable by another user — `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
