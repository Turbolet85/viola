# layout-templates · the rows that bind 2026-11-15-live-round-on-the-trimmed-product — picked 72 of 221 (53 by every draw) · rule any · pin 1297

## surface-cli · ## Surface: cli
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/mcp-server-without-human-surface · MCP server with no human surface — viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
decision/mcp-verb-hidden · viola mcp is a hidden verb — `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/no-persistent-footer · No persistent footer — The CLI keeps no persistent footer: terminal scrollback is the history and the tape lives in `events.ndjson`.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-global-flags/home · CLI global flags · --home — `--home` is the same on every verb.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cli-verbs/plugin-install · CLI verbs · viola plugin install — Writes an MCP-only local marketplace under `<home>/marketplace/` with no hooks, prints the two `claude plugin` commands that add it, and spawns no claude.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; prints nothing while the child runs.
contract/cli-verbs/wait · CLI verbs · viola wait <target> — Returns one turn-boundary result, with no spinner.
contract/mcp-tools/list · viola mcp tools · list — A driver's machine view of the viola list verb.
contract/run-revive-start-json/internal-error · viola revive --json start refusals shared with run · internal-error — A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
contract/run-revive-start-json/pinned-hash-mismatch · viola revive --json start refusals shared with run · pinned-hash-mismatch — After a passed preflight, a pinned-hash mismatch prints one exit-1 document with this code, stderr empty.
contract/run-revive-start-json/socket-dir-refused · viola revive --json start refusals shared with run · socket-dir-refused — After a passed preflight, a refused socket directory prints one exit-1 document with this code, stderr empty; without `--json` the socket-directory pair.
contract/run-revive-start-json/squatted-name · viola revive --json start refusals shared with run · squatted-name — After a passed preflight, a squatted name prints one exit-1 document with this code, stderr empty.
contract/run-revive-start/home-tree-refused · viola run and viola revive starts · home tree refused — Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
contract/run-revive-start/socket-dir-refused · viola run and viola revive starts · socket directory refused — Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-plugin-install/dir-quoting · viola plugin install · <dir> quoting — `<dir>` is the canonical home joined with `marketplace`, single-quoted unless every character is an ASCII letter, a digit or one of `_ - . / :`, else printed bare.
contract/viola-plugin-install/json-ok · viola plugin install · --json ok document — Under `--json` it prints `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`, no path.
contract/viola-plugin-install/output · viola plugin install · human output — After writing the marketplace tree, four lines in one plain-ASCII stdout write, no SGR, never redrawn: `viola@viola  written  <dir>`, `run these to add it to Claude Code:`, `  claude plugin marketplace add <dir>`, `  claude plugin install viola@viola`.
contract/viola-revive-refusals/already-live · viola revive refusals · already live — `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged.
contract/viola-run/already-live · viola run start refusals · already live — Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
contract/viola-run/start-output · viola run start · passed start output — `viola run` prints nothing of its own; the terminal belongs to the wrapped claude TUI until it exits.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row passes.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
contract/viola-verify-output/usage · viola verify output · usage — `viola verify [--record <DIR>] [-- <program> [args…]]`, the program defaulting to claude.
contract/viola-verify-refusals/cli-not-found · viola verify refusals · claude CLI not found — Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`.
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
contract/viola-verify-refusals/home-not-private · viola verify refusals · viola home not private — A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.
contract/viola-verify-refusals/version-unreadable · viola verify refusals · CLI version unreadable — Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
contract/viola-wait-output/dialog · viola wait output · dialog — A dialog result prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout with DIALOG not coloured, as a result and not the board.
contract/viola-wait-output/dialog-expired · viola wait output · dialog-expired — A wait woken by the wrapper's record of an expired hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once.
decision/argument-order-mirrors-channel · Argument order mirrors the channel params — CLI argument order mirrors the channel `params`: `viola <verb> <target>` for one strip, `viola link|unlink <driver> <driven>` for a handoff, `viola answer <target> <dialog_id>`.
decision/dialog-attention-only-on-board · DIALOG attention word only on the board — The amber DIALOG attention word appears only in viola list.
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
decision/mcp-hides-wheel-and-handoff-verbs · MCP hides wheel and handoff verbs — No viola mcp tool, description or result names release, pause, link or unlink.
decision/no-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
decision/only-header-bay-line · Only header is the BAY line — The only header on any verb is the one static BAY line at the top of `viola list`.
decision/shared-captions-fixed · Shared captions fixed — The six captions, words and slot order are fixed, and the terminal fleet console reuses them.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/targets-are-wrapped-names · A CLI target is always a ViolaName — A CLI `<target>` is always a `ViolaName`, and unwrapped names are never valid targets.
decision/words-carry-every-state · Words carry every state — Piped viola list output is the same characters with no SGR, and nothing is lost because the words carry every state.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/viola-list-board/dialog-unknown · viola list board · DIALOG unknown — A pending dialog of unknown kind reads `DIALOG unknown`.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
contract/viola-revive-json/refusal-document · viola revive --json · refusal document — Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
contract/viola-revive-refusals/state-files-writable · viola revive refusals · state files writable by another user — `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
contract/viola-verify-output/help-paragraph · viola verify output · --help — viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
contract/viola-wait-output/timed-out · viola wait output · timed out — A timeout prints `timed out  <name>  <ms> ms` on stdout.
contract/viola-wait-output/waiting-line · viola wait output · waiting line — `waiting: <name>` is printed once, on stderr, only when stderr is a terminal and --json is not set.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
