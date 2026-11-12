@row_add n=1 row=contract/viola-help/usage-line area=surface-cli rec=1,2
boundary: viola --help output
item: usage line
promise: `viola --help` and `viola help` print exactly `Usage: viola [--home <dir>] <verb> [args]` as the first line, then a blank line.
@row_add n=2 row=contract/viola-help/verb-groups area=surface-cli rec=1,2,26
boundary: viola --help output
item: verb group rows
promise: Five rows `board:    list`, `traffic:  send, wait, last, answer`, `wheel:    pause, release`, `handoff:  link, unlink`, `setup:    run, revive, verify, plugin install`, served verbs only, plain ASCII, then a blank line, `Options:` and clap's options; no about line and no clap `Commands:` list.
@row_add n=3 row=decision/mcp-verb-hidden area=surface-cli rec=2
name: viola mcp is a hidden verb
rule: `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row.
@row_add n=4 row=contract/viola-verb-help/usage area=surface-cli rec=3
boundary: viola <verb> --help output
item: per-verb usage
promise: `viola <verb> --help` gives clap's per-verb usage unchanged, its usage line naming the program by its executable.
@row_add n=5 row=decision/prompt-text-from-stdin-or-file area=surface-cli rec=3
name: Prompt text from stdin or --file
rule: Prompt text comes from stdin or `--file`, never from a leading-slash argument, and the help of `send` says so.
@row_add n=6 row=contract/viola-run/start-output area=surface-cli rec=4,21
boundary: viola run start
item: passed start output
promise: `viola run` prints nothing of its own; the terminal belongs to the wrapped claude TUI until it exits.
@row_add n=7 row=contract/viola-run/already-live area=surface-cli rec=4
boundary: viola run start refusals
item: already live
promise: Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
@row_add n=8 row=contract/viola-revive/usage area=surface-cli rec=6
boundary: viola revive
item: usage
promise: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` or `viola revive <name> --list`, each with `[--json]`; `--list` conflicts with `--id`, `--fork` and the trailing arguments.
@row_add n=9 row=contract/viola-revive/start-output area=surface-cli rec=6,5
boundary: viola revive
item: passed start output
promise: A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
@row_add n=10 row=contract/viola-revive/list-row area=surface-cli rec=6,5
boundary: viola revive
item: --list row
promise: `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
@row_add n=11 row=contract/viola-revive/list-cause area=surface-cli rec=6
boundary: viola revive
item: --list cause field
promise: The cause in a `viola revive --list` row is `startup`, `clear`, `resume` or `compact` as logged, and `unknown` otherwise.
@row_add n=12 row=contract/viola-revive/refusal-shape area=surface-cli rec=6
boundary: viola revive refusals
item: refusal shape
promise: Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
@row_add n=13 row=contract/viola-revive-refusals/state-files-writable area=surface-cli rec=7,6
boundary: viola revive refusals
item: state files writable by another user
promise: `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
@row_add n=14 row=contract/viola-revive-refusals/already-live area=surface-cli rec=8,6
boundary: viola revive refusals
item: already live
promise: `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged.
@row_add n=15 row=contract/viola-revive-refusals/no-logged-session area=surface-cli rec=9,6
boundary: viola revive refusals
item: no logged session
promise: `unable: <name> has no logged session to resume` / `hint: start one: viola run <name> -- claude`.
@row_add n=16 row=contract/viola-revive-refusals/no-session-with-id area=surface-cli rec=10,6
boundary: viola revive refusals
item: no logged session with that id
promise: `unable: <name> has no logged session with that id` / `hint: viola revive <name> --list`.
@row_add n=17 row=contract/viola-revive-refusals/session-open-elsewhere area=surface-cli rec=11,6
boundary: viola revive refusals
item: session open in another claude
promise: `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`.
@row_add n=18 row=contract/viola-revive-refusals/directory-missing area=surface-cli rec=12,5
boundary: viola revive refusals
item: recorded directory missing
promise: `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
@row_add n=19 row=decision/no-hint-names-release area=surface-cli rec=13,26
name: No hint names viola release
rule: Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`.
why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
@row_add n=20 row=contract/viola-revive/list-refusals area=surface-cli rec=13
boundary: viola revive refusals
item: --list refusals
promise: `viola revive --list` has two of the refusals: the state-files pair and the no-logged-session pair.
@row_add n=21 row=contract/run-revive-start/home-tree-refused area=surface-cli rec=13,39
boundary: viola run and viola revive starts
item: home tree refused
promise: Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
@row_add n=22 row=contract/viola-revive/preflight-order area=surface-cli rec=13
boundary: viola revive start
item: preflight order
promise: The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
@row_add n=23 row=contract/viola-revive/session-live-check area=surface-cli rec=13
boundary: viola revive start
item: session-live check
promise: One `claude agents --json` read; a row whose `sessionId` is the logged id refuses; skipped under `--fork`; a failed read passes and the start goes on.
@row_add n=24 row=contract/viola-revive/malformed-id area=surface-cli rec=13
boundary: viola revive
item: malformed --id
promise: A malformed `--id` is clap's usage error, exit 2.
@row_add n=25 row=contract/viola-revive-json/refusal-document area=surface-cli rec=14,15
boundary: viola revive --json
item: refusal document
promise: Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
@row_add n=26 row=contract/viola-revive-json/strict-modes-failed area=surface-cli rec=14
boundary: viola revive --json error codes
item: strict-modes-failed
promise: The code for a refused home tree or instance state files.
@row_add n=27 row=contract/viola-revive-json/already-live area=surface-cli rec=14,15
boundary: viola revive --json error codes
item: already-live
promise: The code for a live holder or a fresh replayed beat.
@row_add n=28 row=contract/viola-revive-json/stale-heartbeat area=surface-cli rec=14
boundary: viola revive --json error codes
item: stale-heartbeat
promise: The code for a stale holder.
@row_add n=29 row=contract/viola-revive-json/no-session area=surface-cli rec=14
boundary: viola revive --json error codes
item: no-session
promise: The code for both no-logged-session pairs: no logged session, and no logged session with that id.
@row_add n=30 row=contract/viola-revive-json/session-live area=surface-cli rec=14
boundary: viola revive --json error codes
item: session-live
promise: The code for a session open in another claude.
@row_add n=31 row=contract/viola-revive-json/cwd-missing area=surface-cli rec=14
boundary: viola revive --json error codes
item: cwd-missing
promise: The code for a missing recorded directory.
@row_add n=32 row=contract/viola-revive-json/list-document area=surface-cli rec=14,15
boundary: viola revive --json
item: --list document
promise: `--list --json` prints the rows as one document `{"v":1,"ok":{"sessions":[{"ts":…,"cause":…,"id":…}]}}` in log order, fields as the human rows, or the `no-session` document with exit 1.
@row_add n=33 row=contract/run-revive-start-json/pinned-hash-mismatch area=surface-cli rec=14
boundary: viola revive --json start refusals shared with run
item: pinned-hash-mismatch
promise: After a passed preflight, a pinned-hash mismatch prints one exit-1 document with this code, stderr empty.
@row_add n=34 row=contract/run-revive-start-json/squatted-name area=surface-cli rec=14
boundary: viola revive --json start refusals shared with run
item: squatted-name
promise: After a passed preflight, a squatted name prints one exit-1 document with this code, stderr empty.
@row_add n=35 row=contract/run-revive-start-json/socket-dir-refused area=surface-cli rec=14
boundary: viola revive --json start refusals shared with run
item: socket-dir-refused
promise: After a passed preflight, a refused socket directory prints one exit-1 document with this code, stderr empty; without `--json` the socket-directory pair.
@row_add n=36 row=contract/run-revive-start-json/internal-error area=surface-cli rec=14
boundary: viola revive --json start refusals shared with run
item: internal-error
promise: A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
@row_add n=37 row=contract/viola-plugin-install/output area=surface-cli rec=16,17
boundary: viola plugin install
item: human output
promise: After writing the marketplace tree, four lines in one plain-ASCII stdout write, no SGR, never redrawn: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`.
@row_add n=38 row=contract/viola-plugin-install/dir-quoting area=surface-cli rec=17
boundary: viola plugin install
item: <dir> quoting
promise: `<dir>` is the canonical home joined with `marketplace`, single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :`, else printed bare.
@row_add n=39 row=contract/viola-plugin-install/json-ok area=surface-cli rec=16,17
boundary: viola plugin install
item: --json ok document
promise: Under `--json` it prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`, no path.
@row_add n=40 row=contract/viola-plugin-install/strict-modes-refused area=surface-cli rec=16,17,39
boundary: viola plugin install
item: strict-modes refusal
promise: A refused home or `marketplace/` tree dir is exit 21 with the home pair on stderr, stdout empty, nothing created, no log opened; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
@row_add n=41 row=contract/viola-plugin-install/internal-error area=surface-cli rec=17
boundary: viola plugin install
item: internal error
promise: A failed exe read, dir creation or file write is `error: internal error`, exit 1, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty.
@row_add n=42 row=contract/viola-plugin-install/bare-plugin area=surface-cli rec=17
boundary: viola plugin install
item: bare viola plugin
promise: `viola plugin` alone is a clap usage error, exit 2; clap serves its own `viola plugin help [install]`.
@row_add n=43 row=decision/only-header-bay-line area=surface-cli rec=18
name: Only header is the BAY line
rule: The only header on any verb is the one static BAY line at the top of `viola list`.
@row_add n=44 row=contract/bay-line/cell-order area=surface-cli rec=19
boundary: the BAY line of viola list
item: cell order
promise: `BAY`, `5H <n> %`, `7D <n> %` (each followed by `expired` when the window has passed), `read <age> ago`, `gate open` or `budget-paused`, `skipped a - b - c`, separated by the two-space gutter.
@row_add n=45 row=decision/bay-line-unstyled area=surface-cli rec=20
name: BAY line unstyled
rule: The BAY line's label words and data figures are all in the default foreground, with no bold, no colour, no box drawing and no rule line under it.
@row_add n=46 row=decision/no-banner area=surface-cli rec=21
name: No banner anywhere
rule: Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
@row_add n=47 row=decision/flat-verbs area=surface-cli rec=22,48
name: Flat verbs plus a target
rule: Each verb is one lower-case word with no nested noun hierarchy or subcommand tree.
except: `plugin install` is the one two-word verb.
@row_add n=48 row=decision/no-prompt-or-pager area=surface-cli rec=22
name: No interactive prompt or pager
rule: No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
@row_add n=49 row=decision/argument-order-mirrors-channel area=surface-cli rec=23
name: Argument order mirrors the channel params
rule: CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
@row_add n=50 row=decision/targets-are-wrapped-names area=surface-cli rec=23
name: Targets are wrapped ViolaNames
rule: A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets.
@row_add n=51 row=decision/board-vs-strip area=surface-cli rec=24,48
name: Board versus strip verbs
rule: `viola list` shows the whole bay, and every other verb acts on one strip named by its `ViolaName`.
@row_add n=52 row=contract/cli-global-flags/home area=surface-cli rec=25
boundary: CLI global flags
item: --home
promise: `--home` is the same on every verb.
@row_add n=53 row=decision/json-on-every-data-verb area=surface-cli rec=25
name: --json on every data verb
rule: `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
@row_add n=54 row=decision/hints-name-next-verb area=surface-cli rec=26
name: Hints name the next verb
rule: `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
@row_add n=55 row=contract/readback-mirror/layout area=surface-cli rec=27
boundary: the readback mirror line
item: layout
promise: A fixed first column holding the four-character box (`[RB]`, `[  ]`, `[/ ]`), one space, the padded state word (`open` / `read back` / `unable` / `unconfirmable`), the two-space gutter, NAME, then the outcome fields.
@row_add n=56 row=contract/readback-mirror/issue-and-outcome area=surface-cli rec=28
boundary: the readback mirror line
item: issue and outcome lines
promise: The TTY issue line on stderr is the open box and the outcome line under it is the closed box, the two lined up column for column.
@row_add n=57 row=decision/readback-mirror-no-colour area=surface-cli rec=29
name: Readback mirror has no colour
rule: The readback mirror is words and brackets only: `read back` is never green and `unable` is never red.
why: Colour on the CLI is reserved for `DIALOG` and `stale`.
@row_add n=58 row=decision/no-progress-glyphs area=surface-cli rec=30
name: No send progress glyphs
rule: A send shows no `Sending…` spinner, no `✓` and no `done`.
@row_add n=59 row=decision/turn-ended-not-send-outcome area=surface-cli rec=30
name: turn-ended is not a send outcome
rule: `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
@row_add n=60 row=contract/send-json/output area=surface-cli rec=31
boundary: viola send --json
item: typed document
promise: `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
@row_add n=61 row=contract/viola-list/columns area=surface-cli rec=32
boundary: viola list
item: columns
promise: NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, each padded to its widest word, with the two-space gutter between columns and no box-drawing borders.
@row_add n=62 row=decision/viola-list-styling area=surface-cli rec=33,38
name: viola list styling
rule: On a colour terminal NAME is bold, the `DIALOG` word is accent amber with its kind word plain, a `stale` row is dim end to end with `DIALOG` still amber, nothing else is styled, and no SGR is written when colour is off.
@row_add n=63 row=contract/viola-list/unwrapped-separator area=surface-cli rec=34,38
boundary: viola list
item: rack separator
promise: `-- UNWRAPPED - READ-ONLY --` on its own line before the unwrapped rows, written only when unwrapped rows exist; the wrapped rack is always first, with no separator.
@row_add n=64 row=decision/truncate-only-unwrapped-names area=surface-cli rec=35,38
name: Truncate only unwrapped names
rule: Only unwrapped NAMEs are cut, with `...`, only where the terminal width is known and too narrow, never inside an escape; wrapped names are never clipped and no line ever wraps.
@row_add n=65 row=decision/board-row-escaping area=surface-cli rec=36,38
name: Board row escaping
rule: Every board field derived from `claude agents --json` has C0 and C1 controls, newline and tab included, escaped as hex text, so a row never splits and the columns stay fixed-width.
why: Ratified; stricter than the security floor, which keeps newline and tab for `wait` and `last` text.
@row_add n=66 row=decision/no-link-column area=surface-cli rec=37
name: No link column on the board
rule: The six shared board columns stay fixed; links are visible only in `link` / `unlink` verb output.
@row_add n=67 row=contract/cli-refusal-line/form area=surface-cli rec=39
boundary: CLI refusal line
item: line form
promise: `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
@row_add n=68 row=contract/viola-verify-refusals/cli-not-found area=surface-cli rec=39
boundary: viola verify refusals
item: claude CLI not found
promise: Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`.
@row_add n=69 row=contract/viola-verify-refusals/version-unreadable area=surface-cli rec=39
boundary: viola verify refusals
item: CLI version unreadable
promise: Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`.
@row_add n=70 row=contract/viola-verify-refusals/fixture-not-clean area=surface-cli rec=39
boundary: viola verify refusals
item: recorded fixture not clean
promise: Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
@row_add n=71 row=contract/viola-verify-refusals/stopped area=surface-cli rec=39,46
boundary: viola verify refusals
item: verify stopped
promise: A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
@row_add n=72 row=contract/viola-verify-refusals/home-not-private area=surface-cli rec=39
boundary: viola verify refusals
item: viola home not private
promise: A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.
@row_add n=73 row=contract/run-revive-start/socket-dir-refused area=surface-cli rec=39
boundary: viola run and viola revive starts
item: socket directory refused
promise: Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
@row_add n=74 row=contract/viola-list/strict-modes-refused area=surface-cli rec=39
boundary: viola list
item: strict-modes refusal
promise: Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance.
@row_add n=75 row=contract/frame-verbs/strict-modes-refused area=surface-cli rec=39
boundary: the frame verbs
item: strict-modes refusal
promise: For send, wait, last, answer, pause, release, link and unlink: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
@row_add n=76 row=contract/cli-catch-site/internal-error area=surface-cli rec=39
boundary: CLI verbs outside a revive start
item: internal error
promise: A failed or panicked verb prints exactly `error: internal error` once, from the one catch site, with no hint; with `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout instead, stderr empty.
@row_add n=77 row=contract/send-refusals/control-character area=surface-cli rec=40
boundary: viola send refusals before any request
item: control-character
promise: A text holding a refused character is refused before any frame: `[/ ] unable  <name>  not-delivered  control-character` on stderr, exit 13, its hint last; under `--json` one stdout document, no stderr line.
@row_add n=78 row=contract/send-refusals/empty-text area=surface-cli rec=40
boundary: viola send refusals before any request
item: empty-text
promise: A text empty once trailing CR and LF are removed is refused before any frame, exit 13, `[/ ] unable  <name>  not-delivered  empty-text` then `hint: the text is empty once its trailing newlines are removed; send a text with content`.
@row_add n=79 row=decision/wrapper-repeats-send-refusals area=surface-cli rec=40
name: Wrapper repeats the send text refusals
rule: The wrapper makes the same control-character and empty-text refusals for a frame sent straight to it as `viola send` makes itself.
@row_add n=80 row=decision/hint-for-every-cause area=surface-cli rec=41
name: A hint for every refusal cause
rule: Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint.
why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line.
except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
@row_add n=81 row=contract/frame-verb-hints/instance-not-running area=surface-cli rec=41
boundary: frame verb exit-21 hints
item: instance not running
promise: `hint: <name> is not running; viola list shows the live instances` under `unable  <name>  instance-unreachable` for `wait`, `last`, `link`, `unlink`, and `send` in its mirror.
@row_add n=82 row=contract/frame-verb-hints/not-wrapped area=surface-cli rec=41
boundary: frame verb exit-21 hints
item: name with no instance directory
promise: `hint: <name> is not a wrapped session; only sessions started with viola run take commands`, the last stderr line.
@row_add n=83 row=contract/frame-verb-hints/wrapper-went-away area=surface-cli rec=41
boundary: frame verb exit-21 hints
item: wrapper lost during the call
promise: `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running`, the last stderr line.
@row_add n=84 row=contract/frame-verb-hints/server-verify-failed area=surface-cli rec=41
boundary: frame verb exit-21 hints
item: endpoint not the recorded wrapper
promise: No frame written, exit 21, `hint: the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`; under `--json` `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`, stderr empty.
@row_add n=85 row=contract/frame-verb-hints/socket-dir-refused area=surface-cli rec=41
boundary: frame verb exit-21 hints
item: socket directory refused
promise: No frame written, exit 21, `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`; under `--json` the `socket-dir-refused` detail, stderr empty.
@row_add n=86 row=decision/hints-never-quote-or-locate area=surface-cli rec=41
name: Hints never quote text or name paths
rule: A hint never quotes the sent text or any upstream text, and never names a path or a pid.
@row_add n=87 row=contract/cli-exit-codes/10 area=surface-cli rec=42
boundary: CLI exit codes
item: 10
promise: `human-typing`.
@row_add n=88 row=contract/cli-exit-codes/11 area=surface-cli rec=42
boundary: CLI exit codes
item: 11
promise: `budget-paused`.
@row_add n=89 row=contract/cli-exit-codes/12 area=surface-cli rec=42
boundary: CLI exit codes
item: 12
promise: `unverified-cli`.
@row_add n=90 row=contract/cli-exit-codes/13 area=surface-cli rec=42
boundary: CLI exit codes
item: 13
promise: `not-delivered`.
@row_add n=91 row=contract/cli-exit-codes/14 area=surface-cli rec=42
boundary: CLI exit codes
item: 14
promise: `unknown`.
@row_add n=92 row=contract/cli-exit-codes/20 area=surface-cli rec=42
boundary: CLI exit codes
item: 20
promise: Wrapper fault, `error: wrapper fault  <code>`.
@row_add n=93 row=contract/cli-exit-codes/21 area=surface-cli rec=42
boundary: CLI exit codes
item: 21
promise: `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead.
@row_add n=94 row=contract/cli-exit-codes/json-form area=surface-cli rec=42
boundary: CLI exit codes
item: --json form
promise: Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
@row_add n=95 row=contract/instance-unreachable-detail/strict-modes-failed area=surface-cli rec=42
boundary: exit-21 --json detail
item: strict-modes-failed
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=96 row=contract/instance-unreachable-detail/instance-dead area=surface-cli rec=42
boundary: exit-21 --json detail
item: instance-dead
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=97 row=contract/instance-unreachable-detail/unwrapped-name area=surface-cli rec=42
boundary: exit-21 --json detail
item: unwrapped-name
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=98 row=contract/instance-unreachable-detail/endpoint-lost area=surface-cli rec=42
boundary: exit-21 --json detail
item: endpoint-lost
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=99 row=contract/instance-unreachable-detail/server-verify-failed area=surface-cli rec=42
boundary: exit-21 --json detail
item: server-verify-failed
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=100 row=contract/instance-unreachable-detail/socket-dir-refused area=surface-cli rec=42
boundary: exit-21 --json detail
item: socket-dir-refused
promise: One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
@row_add n=101 row=decision/stdout-results-stderr-rest area=surface-cli rec=43
name: Results on stdout, the rest on stderr
rule: Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
@row_add n=102 row=decision/no-terminator-word area=surface-cli rec=44
name: No terminator word
rule: No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
@row_add n=103 row=decision/hint-last-on-stderr area=surface-cli rec=45
name: Hint is the last stderr line
rule: On a refusal, the `hint:` line is the last line on stderr.
@row_add n=104 row=contract/viola-verify/summary area=surface-cli rec=46
boundary: viola verify output
item: summary line
promise: `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
@row_add n=105 row=decision/no-persistent-footer area=surface-cli rec=47
name: No persistent footer
rule: The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
@row_add n=106 row=absence/cli-deep-links area=surface-cli rec=49
capability: Deep links or URL coupling in the CLI
@row_add n=107 row=decision/violaname-never-case-transformed area=surface-cli rec=49
name: ViolaName never case-transformed
rule: `ViolaName` is a shared reference and is never case-transformed.
@row_add n=108 row=contract/cursor/byte-offset area=surface-cli rec=49
boundary: cursor
item: cursor
promise: `cursor` is the byte offset `send` prints and `wait --after` consumes.
@row_add n=109 row=decision/human-output-is-contract area=surface-cli rec=50
name: Human output is a contract
rule: The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice.
why: LLM drivers may read the human output.
@row_add n=110 row=decision/shared-captions-fixed area=surface-cli rec=51
name: Shared captions fixed
rule: The six captions, words and slot order are fixed, and the terminal fleet console reuses them.
@row_add n=111 row=contract/readback-vocabulary/words area=surface-cli rec=51
boundary: readback vocabulary
item: state words
promise: The readback vocabulary is `open`, `read back`, `unable`, `unconfirmable`, and the CLI brackets the box.
@row_add n=112 row=decision/cli-field-separators area=surface-cli rec=51
name: CLI field separators
rule: ` - ` separates inside a CLI field and an arrow is `->`; there is no CLI inbound `<-` form.
@row_add n=113 row=decision/amber-means-dialog area=surface-cli rec=51
name: Amber means DIALOG only
rule: Amber means `DIALOG` and nothing else, and `stale` is SGR dim in the terminal.
@row_add n=114 row=decision/cli-holds-every-action area=surface-cli rec=51
name: The CLI holds every action
rule: The CLI is where every action lives.
@row_none n=1 rec=52 type=S
@row_none n=2 rec=53 type=S
