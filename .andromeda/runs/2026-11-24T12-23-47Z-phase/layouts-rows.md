# layout-templates · the rows that bind 2026-11-24-switching-a-session-s-model-and-effort — picked 95 of 234 (60 by every draw) · rule any · pin 4716

## surface-cli · ## Surface: cli
decision/argument-order-mirrors-channel · Argument order mirrors the channel params — CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `viola up` re-opens the whole fleet or the instances it names, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/cli-holds-every-action · The CLI holds every action — The CLI is where every action lives.
decision/flat-verbs · Flat verbs plus a target — Each verb is one lower-case word with no nested noun hierarchy or subcommand tree. · except: `plugin install` is the one two-word verb.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names the release, pause, link or unlink verbs; the list tool's result carries the board's held links as data.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
decision/no-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/spoken-phraseology-verb-output · Spoken phraseology for wheel, handoff and dialog verbs — The output of viola pause, release, link, unlink and answer is spoken phraseology.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/targets-are-wrapped-names · A CLI target is always a ViolaName — A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets.
decision/violaname-never-case-transformed · ViolaName never case-transformed — `ViolaName` is a shared reference and is never case-transformed.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`), never reused; no refusal maps to 11.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install`, the board refusal of `viola up` and the strict-modes refusal of the frame verbs and `viola focus` the fixed state pair instead.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-refusals/instance-unreachable · CLI refusals · instance-unreachable — With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line.
contract/cli-verbs/help · CLI verbs · viola --help — The grouped verb table with the served verbs only: the board, traffic, wheel, handoff and setup rows, `board:` holding list and focus, `handoff:` holding link, unlink and `setup:` holding open, run, revive, adopt, up, verify and plugin install.
contract/cli-verbs/revive · CLI verbs · viola revive <name> — Resumes a dead instance in place (its newest logged session, in the recorded directory, which also keys the trust record its start writes) and is passthrough like viola run.
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
contract/run-revive-usage/pair-after-dashes · viola run and viola revive usage · a launch pair after the dashes — A `--model` or `--effort`, or its equals form, among the words after the dashes of `viola run` or `viola revive` is a usage refusal, exit 2, one fixed stderr line naming `--model <MODEL>` and `--effort <EFFORT>` and echoing none of the caller's words.
contract/viola-help/usage-line · viola --help output · usage line — `viola --help` and `viola help` print exactly `Usage: viola [--home <dir>] <verb> [args]` as the first line, then a blank line.
contract/viola-help/verb-groups · viola --help output · verb group rows — Five rows `board:    list, focus`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink` and the setup row, served verbs only, plain ASCII, then a blank line and `Options:`; no about line and no clap `Commands:` list.
contract/viola-revive-json/already-live · viola revive --json error codes · already-live — The code for a live holder or a fresh replayed beat.
contract/viola-revive-json/cwd-missing · viola revive --json error codes · cwd-missing — The code for a missing recorded directory.
contract/viola-revive-json/refusal-document · viola revive --json · refusal document — Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
contract/viola-revive-json/session-live · viola revive --json error codes · session-live — The code for a session open in another claude.
contract/viola-revive-refusals/already-live · viola revive refusals · already live — `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged.
contract/viola-revive-refusals/directory-missing · viola revive refusals · recorded directory missing — `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
contract/viola-revive-refusals/session-open-elsewhere · viola revive refusals · session open in another claude — `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`.
contract/viola-revive/preflight-order · viola revive start · preflight order — The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
contract/viola-revive/session-live-check · viola revive start · session-live check — One `claude agents --json` read; a row whose `sessionId` is the logged id refuses; skipped under `--fork`; a failed read passes and the start goes on.
contract/viola-revive/usage · viola revive · usage — `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` or `viola revive <name> --list`, each with `[--json]`; `--list` conflicts with `--id`, `--fork`, `--model`, `--effort`, the hidden `--origin` and the trailing arguments.
contract/viola-send-output/unable · viola send output · unable — `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
contract/viola-send-output/word-column · viola send output · word column — The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
contract/viola-verb-help/usage · viola <verb> --help output · per-verb usage — `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
contract/wheel-verb-output/pause · wheel verb output · viola pause — Prints `<name>  wheel human  manual-pause  I have control`.
contract/wheel-verb-output/release · wheel verb output · viola release — Prints `<name>  wheel driver  you have control`.
decision/clap-derive-cli-parsing · clap derive for CLI parsing — The viola command line is parsed with clap 4 in its derive form.
decision/cli-field-separators · CLI field separators — ` - ` separates inside a CLI field and an arrow is `->`; there is no CLI inbound `<-` form.
decision/mcp-server-without-human-surface · MCP server with no human surface — viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
decision/no-persistent-footer · No persistent footer — The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/only-header-bay-line · Only header is the BAY line — The only header on any verb is the one static BAY line at the top of `viola list`.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/truncate-only-unwrapped-names · Truncate only unwrapped names — Only unwrapped NAMEs are cut, with `...`, only where the terminal width is known and too narrow, never inside an escape; wrapped names are never clipped and no line ever wraps.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`.
contract/cli-global-flags/home · CLI global flags · --home — `--home` is the same on every verb.
contract/cli-token-rendering/motion-tokens · CLI token rendering · Motion tokens — None; nothing redraws in place.
contract/cli-token-rendering/spacing · CLI token rendering · Spacing — The column gutter is two spaces on the terminal grid; inside one field ` - ` is the separator.
contract/cli-verbs/open · CLI verbs · viola open <name> — Starts a wrapped claude session in a new foot terminal window beside its parent; its flags are a closed allowlist (--parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort, --json), the only words allowed after `--` are `claude` alone, and no shell runs.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs.
contract/cli-verbs/up · CLI verbs · viola up [<NAME>...] [--dry-run] [--json] — Re-opens the fleet after a reboot: each candidate instance is revived in its own terminal at its recorded working directory, parents before children, and instances already live are skipped; it is CLI-only, with no MCP tool and no channel method.
contract/readback-mirror/layout · the readback mirror line · layout — A fixed first column holding the four-character box (`[RB]`, `[  ]`, `[/ ]`), one space, the padded state word (`open` / `read back` / `unable` / `unconfirmable`), the two-space gutter, NAME, then the outcome fields.
contract/readback-vocabulary/words · readback vocabulary · state words — The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
contract/run-revive-start/home-tree-refused · viola run and viola revive starts · home tree refused — Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
contract/run-revive-start/socket-dir-refused · viola run and viola revive starts · socket directory refused — Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
contract/viola-help/setup-row · viola --help output · setup row — The `setup:` verb group row prints exactly `setup:    open, run, revive, adopt, up, verify, plugin install`, `adopt` after `revive`.
contract/viola-list-json/strict-modes-failed · viola list --json · strict-modes-failed — A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found.
contract/viola-list/strict-modes-refused · viola list · strict-modes refusal — Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance.
contract/viola-plugin-install/strict-modes-refused · viola plugin install · strict-modes refusal — A refused home or `marketplace/` tree dir is exit 21 with the home pair on stderr, stdout empty, nothing created, no log opened; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
contract/viola-revive-json/no-session · viola revive --json error codes · no-session — The code for both no-logged-session pairs: no logged session, and no logged session with that id.
contract/viola-revive-json/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The code for a stale holder.
contract/viola-revive-json/strict-modes-failed · viola revive --json error codes · strict-modes-failed — The code for a refused home tree or instance state files.
contract/viola-revive-refusals/no-logged-session · viola revive refusals · no logged session — `unable: <name> has no logged session to resume` / `hint: start one: viola run <name> -- claude`.
contract/viola-revive-refusals/state-files-writable · viola revive refusals · state files writable by another user — `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
contract/viola-revive/no-window · viola revive · window placement — viola revive places no window unless given its hidden `--origin <cli|agent>`, under which the revived window moves to the workspace a real snapshot recorded, or reads unplaced when none was recorded.
contract/viola-revive/start-output · viola revive · passed start output — A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
contract/viola-run/already-live · viola run start refusals · already live — Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
