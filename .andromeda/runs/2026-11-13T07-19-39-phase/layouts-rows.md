# layout-templates · the rows that bind 2026-11-13-epoch-8-cleanup — picked 101 of 224 (64 by every draw) · rule any · pin 885

## surface-cli · ## Surface: cli
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/mcp-server-without-human-surface · MCP server with no human surface — viola mcp is a hidden, plugin-run stdio MCP server with no human surface: a driver's machine view of the CLI verbs.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
decision/turn-ended-not-send-outcome · turn-ended is not a send outcome — `turn-ended` is never reported as a send outcome; it is the result of `viola wait`.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install` and the frame verbs' strict-modes refusal the fixed state pair instead.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-verbs/list · CLI verbs · viola list — The board: every session as one strip row, with the BAY context header.
contract/cli-verbs/revive · CLI verbs · viola revive <name> — Resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like viola run.
contract/cli-verbs/revive-list · CLI verbs · viola revive <name> --list — Prints the instance's logged sessions, one static line each.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; prints nothing while the child runs.
contract/cli-verbs/send · CLI verbs · viola send <target> — Types at a turn boundary and reports read back, unable or unconfirmable.
contract/cli-verbs/wait · CLI verbs · viola wait <target> — Returns one turn-boundary result, with no spinner.
contract/cursor/byte-offset · cursor · cursor — `cursor` is the byte offset `send` prints and `wait --after` consumes.
contract/frame-verbs/strict-modes-refused · the frame verbs · strict-modes refusal — For send, wait, last, answer, pause, release, link and unlink: exit 21 before any role file or log opens, stdout empty, the state-files pair, mode left as found; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, stderr empty.
contract/instance-unreachable-detail/strict-modes-failed · exit-21 --json detail · strict-modes-failed — One of the causes `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` carries.
contract/run-revive-start-json/internal-error · viola revive --json start refusals shared with run · internal-error — A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
contract/run-revive-start-json/pinned-hash-mismatch · viola revive --json start refusals shared with run · pinned-hash-mismatch — After a passed preflight, a pinned-hash mismatch prints one exit-1 document with this code, stderr empty.
contract/run-revive-start-json/socket-dir-refused · viola revive --json start refusals shared with run · socket-dir-refused — After a passed preflight, a refused socket directory prints one exit-1 document with this code, stderr empty; without `--json` the socket-directory pair.
contract/run-revive-start-json/squatted-name · viola revive --json start refusals shared with run · squatted-name — After a passed preflight, a squatted name prints one exit-1 document with this code, stderr empty.
contract/run-revive-start/home-tree-refused · viola run and viola revive starts · home tree refused — Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
contract/run-revive-start/socket-dir-refused · viola run and viola revive starts · socket directory refused — Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-list-board/agents-read-failure · viola list board · failed agents read — A failed, oversize or malformed `claude agents --json` read prints the board as with no claude.
contract/viola-list-json/strict-modes-failed · viola list --json · strict-modes-failed — A failed strict-modes check of the home, instances/ or any one instance prints `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with exit 21 and stderr empty, opens no log and leaves the home's mode as found.
contract/viola-list/strict-modes-refused · viola list · strict-modes refusal — Exit 21 refusing the whole board, stdout empty: the home pair for the home or `instances/`, the state-files pair for one instance.
contract/viola-plugin-install/strict-modes-refused · viola plugin install · strict-modes refusal — A refused home or `marketplace/` tree dir is exit 21 with the home pair on stderr, stdout empty, nothing created, no log opened; under `--json` `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` with stderr empty.
contract/viola-revive-json/already-live · viola revive --json error codes · already-live — The code for a live holder or a fresh replayed beat.
contract/viola-revive-json/cwd-missing · viola revive --json error codes · cwd-missing — The code for a missing recorded directory.
contract/viola-revive-json/list-document · viola revive --json · --list document — `--list --json` prints the rows as one document `{"v":1,"ok":{"sessions":[{"ts":…,"cause":…,"id":…}]}}` in log order, fields as the human rows, or the `no-session` document with exit 1.
contract/viola-revive-json/no-session · viola revive --json error codes · no-session — The code for both no-logged-session pairs: no logged session, and no logged session with that id.
contract/viola-revive-json/refusal-document · viola revive --json · refusal document — Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
contract/viola-revive-json/session-live · viola revive --json error codes · session-live — The code for a session open in another claude.
contract/viola-revive-json/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The code for a stale holder.
contract/viola-revive-json/strict-modes-failed · viola revive --json error codes · strict-modes-failed — The code for a refused home tree or instance state files.
contract/viola-revive-refusals/already-live · viola revive refusals · already live — `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged.
contract/viola-revive-refusals/state-files-writable · viola revive refusals · state files writable by another user — `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
contract/viola-revive/list-refusals · viola revive refusals · --list refusals — `viola revive --list` has two of the refusals: the state-files pair and the no-logged-session pair.
contract/viola-revive/session-live-check · viola revive start · session-live check — One `claude agents --json` read; a row whose `sessionId` is the logged id refuses; skipped under `--fork`; a failed read passes and the start goes on.
contract/viola-revive/start-output · viola revive · passed start output — A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
contract/viola-run/already-live · viola run start refusals · already live — Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
contract/viola-run/start-output · viola run start · passed start output — `viola run` prints nothing of its own; the terminal belongs to the wrapped claude TUI until it exits.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row passes.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/help-paragraph · viola verify output · --help — viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, MM being the relied count; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/stop · viola verify output · stop — A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line; `fail` is never coloured.
contract/viola-verify-output/usage · viola verify output · usage — `viola verify [--record <DIR>] [-- <program> [args…]]`, the program defaulting to claude.
contract/viola-verify-refusals/cli-not-found · viola verify refusals · claude CLI not found — Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`.
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.
contract/viola-verify-refusals/home-not-private · viola verify refusals · viola home not private — A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.
contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
contract/viola-verify-refusals/version-unreadable · viola verify refusals · CLI version unreadable — Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
contract/viola-wait-output/dialog · viola wait output · dialog — A dialog result prints `<kind>  <name>  dialog <id>  cursor <n>` on stdout with DIALOG not coloured, as a result and not the board.
contract/viola-wait-output/dialog-expired · viola wait output · dialog-expired — A wait woken by the wrapper's record of an expired hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once.
contract/viola-wait-output/dialog-unknown · viola wait output · dialog unknown — A dialog line whose event carries no dialog_id reads `dialog unknown`.
contract/viola-wait-output/timed-out · viola wait output · timed out — A timeout prints `timed out  <name>  <ms> ms` on stdout.
contract/viola-wait-output/turn-ended · viola wait output · turn-ended — `turn-ended  <name>  <ts>  cursor <n>` on stdout; every kind without a dialog_id takes the turn-ended form.
contract/viola-wait-output/waiting-line · viola wait output · waiting line — `waiting: <name>` is printed once, on stderr, only when stderr is a terminal and --json is not set.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/mcp-verb-hidden · viola mcp is a hidden verb — `viola mcp` is a hidden verb with no argument but the global `--home`, run only by the plugin and never by a person, and it appears in no help row.
decision/no-banner · No banner anywhere — Normal output has no ASCII-art logo, no tagline and no version line; the version appears only in `--version` and `--help`, and `viola hook` and `viola mcp` have no human surface.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
contract/cli-catch-site/internal-error · CLI verbs outside a revive start · internal error — A failed or panicked verb prints exactly `error: internal error` once, from the one catch site, with no hint; with `--json` before a `--` it prints `{"v":1,"error":"internal-error","detail":null}` on stdout instead, stderr empty.
contract/cli-exit-codes/10 · CLI exit codes · 10 — `human-typing`.
contract/cli-exit-codes/11 · CLI exit codes · 11 — `budget-paused`.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-exit-codes/13 · CLI exit codes · 13 — `not-delivered`.
contract/cli-exit-codes/14 · CLI exit codes · 14 — `unknown`.
contract/cli-exit-codes/20 · CLI exit codes · 20 — Wrapper fault, `error: wrapper fault  <code>`.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-internal-error/human · CLI internal error · human output — A failure or a caught panic prints `error: internal error` once, exit 1, with no hint.
contract/cli-internal-error/json · CLI internal error · --json output — Under --json a failure or a caught panic prints `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty, exit 1.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-refusals/instance-unreachable · CLI refusals · instance-unreachable — With no live instance a verb exits 21, printing `unable  <name>  instance-unreachable` on stderr and a hint line as the last stderr line.
contract/mcp-tool-results/invalid-params · viola mcp tool results · invalid-params — Invalid parameters refuse with the code `invalid-params` and a closed field word as `detail`.
contract/mcp-tool-results/refusal · viola mcp tool results · refusal — Each refusal is the CLI verb's --json document less `v`, marked `isError: true` with the text `refused: <code>`; the bodies carry codes only.
contract/send-json/output · viola send --json · typed document — `--json` prints only the typed document (`ok` with `submitted_at` and `cursor`, `ok` with `confirmed:false` and `unconfirmable`, or `refusal` with `detail`) with the typed exit code, no glyph and no hint.
contract/send-refusals/control-character · viola send refusals before any request · control-character — A text holding a refused character is refused before any frame: `[/ ] unable  <name>  not-delivered  control-character` on stderr, exit 13, its hint last; under `--json` one stdout document, no stderr line.
contract/viola-list-board/cli-unknown · viola list board · CLI unknown — A row with no recorded version reads CLI `unknown`.
contract/viola-list-board/status-join · viola list board · STATUS — One `claude agents --json` read follows the board read, joined by child pid or agent session id; a joined wrapped row reads STATUS idle or busy, else unknown.
contract/viola-list-board/wheel-unknown · viola list board · WHEEL unknown — A replayed row with no logged holder reads WHEEL `unknown`.
contract/viola-list-json/document · viola list --json · document — One JSON document on stdout with no header line, no colour and no glyph; its item, budget and skipped shapes are the list payload contract.
contract/viola-plugin-install/internal-error · viola plugin install · internal error — A failed exe read, dir creation or file write is `error: internal error`, exit 1, or under `--json` `{"v":1,"error":"internal-error","detail":null}` on stdout with stderr empty.
contract/viola-revive-refusals/directory-missing · viola revive refusals · recorded directory missing — `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
contract/viola-revive-refusals/no-logged-session · viola revive refusals · no logged session — `unable: <name> has no logged session to resume` / `hint: start one: viola run <name> -- claude`.
contract/viola-revive-refusals/no-session-with-id · viola revive refusals · no logged session with that id — `unable: <name> has no logged session with that id` / `hint: viola revive <name> --list`.
contract/viola-revive-refusals/session-open-elsewhere · viola revive refusals · session open in another claude — `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`.
contract/viola-revive/list-cause · viola revive · --list cause field — The cause in a `viola revive --list` row is `startup`, `clear`, `resume` or `compact` as logged, and `unknown` otherwise.
contract/viola-revive/list-row · viola revive · --list row — `viola revive <name> --list` prints one stdout row per logged session in log order, `<ts>  <cause>  <id>` with two spaces between fields, no header, no colour, plain ASCII, with an empty stderr.
contract/viola-revive/preflight-order · viola revive start · preflight order — The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
contract/viola-send-output/json-mode · viola send output · --json — Under --json the mirror is replaced by the typed ok / refusal document.
contract/viola-send-output/json-ok · viola send output · --json ok — Under --json a confirmed send prints `{"v":1,"ok":{"submitted_at":<ts>,"cursor":<n>}}` on stdout, with no glyph and no issue line.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
