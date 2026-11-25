# layout-templates · the rows that bind 2026-11-25-placing-without-moving-the-view — picked 71 of 237 (56 by every draw) · rule any · pin 4974

## surface-cli · ## Surface: cli
decision/hint-for-every-cause · A hint for every refusal cause — Directly under each refusal is `hint: <one plain instruction>`, keyed by reason or reason and detail, every cause with its own hint. · why: Exit 21 and exit 1 each have several causes, so an agent can tell the causes apart from the last stderr line. · except: No hint for the `unknown` refusal, for `wrapper fault` or for `internal error`.
decision/hint-last-on-stderr · Hint is the last stderr line — On a refusal, the `hint:` line is the last line on stderr.
decision/hints-name-next-verb · Hints name the next verb — `hint:` lines point to the next verb by name, as the CLI's breadcrumbs.
decision/hints-never-quote-or-locate · Hints never quote text or name paths — A hint never quotes the sent text or any upstream text, and never names a path or a pid.
decision/human-output-is-contract · Human output is a contract — The human columns and words are stable; new fields go into `--json` first, and adding a human column is a design change, not an implementation choice. · why: LLM drivers may read the human output.
decision/json-on-every-data-verb · --json on every data verb — `--json` is the machine view of every data verb, `viola revive` included in both its refusal document and `--list`.
decision/no-hint-names-release · No hint names viola release — Every hint is distinct and no hint names `viola release`; `release` is a human verb reached through `--help`. · why: The refusals that carry hints reach drivers, and a driver's `release` is refused.
decision/no-prompt-or-pager · No interactive prompt or pager — No verb has an interactive prompt or a pager, and the terminal fleet console is no verb's output.
decision/no-spinner · No spinner — The cli shows no spinner and no elapsed-time counter.
decision/shared-refusal-word-column · Shared refusal word column — The word half of the struck box (`unable` plus `reason  detail`) is the first-word column of every other verb's refusal line, while the struck glyph itself belongs to viola send alone.
decision/stdout-results-stderr-rest · Results on stdout, the rest on stderr — Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr, and the two are never mixed.
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unverified-cli`.
contract/cli-exit-codes/json-form · CLI exit codes · --json form — Under `--json` the same outcome is one document on stdout, with no stderr line and no hint text.
contract/cli-json-mode/one-document · CLI --json mode · one document — Under --json each verb prints exactly one document on stdout (`{"v":1,"ok":…}`, or the exit-21 / wrapper-fault document) and nothing on stderr.
contract/cli-refusal-line/form · CLI refusal line · line form — `unable  <name>  <reason>  <detail>` on stderr, fields separated by two spaces; `viola send` pads `unable` into the mirror's word column after `[/ ]`.
contract/cli-verbs/open · CLI verbs · viola open <name> — Starts a wrapped claude session in a new foot terminal window beside its parent; its flags are a closed allowlist (--parent, --role overseer|builder|worker, --cwd, --workspace, --model, --effort, --json), the only words allowed after `--` are `claude` alone, and no shell runs.
contract/cli-verbs/revive · CLI verbs · viola revive <name> — Resumes a dead instance in place (its newest logged session, in the recorded directory, which also keys the trust record its start writes) and is passthrough like viola run.
contract/cli-verbs/run · CLI verbs · viola run <name> -- claude — Passthrough; on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder, flag only, so no trust dialog stands and no key is typed; prints nothing while the child runs, nor at a switch's respawn of the child.
contract/cli-verbs/up · CLI verbs · viola up [<NAME>...] [--dry-run] [--json] — Re-opens the fleet after a reboot: each candidate instance is revived in its own terminal at its recorded working directory, parents before children, and instances already live are skipped; it is CLI-only, with no MCP tool and no channel method.
contract/run-revive-start-json/internal-error · viola revive --json start refusals shared with run · internal-error — A program not found or an error of the start itself prints one exit-1 document with this code, stderr empty; a child-pump error prints no document, the child having held the terminal.
contract/run-revive-start-json/pinned-hash-mismatch · viola revive --json start refusals shared with run · pinned-hash-mismatch — After a passed preflight, a pinned-hash mismatch prints one exit-1 document with this code, stderr empty.
contract/run-revive-start-json/socket-dir-refused · viola revive --json start refusals shared with run · socket-dir-refused — After a passed preflight, a refused socket directory prints one exit-1 document with this code, stderr empty; without `--json` the socket-directory pair.
contract/run-revive-start-json/squatted-name · viola revive --json start refusals shared with run · squatted-name — After a passed preflight, a squatted name prints one exit-1 document with this code, stderr empty.
contract/run-revive-start/home-tree-refused · viola run and viola revive starts · home tree refused — Before the log opens, a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy prints `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, a refused instance the state-files pair; exit 1, no log or child.
contract/run-revive-start/socket-dir-refused · viola run and viola revive starts · socket directory refused — Exit 1 before any snapshot, child or bind, stdout empty, directory left as found: `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-revive-json/already-live · viola revive --json error codes · already-live — The code for a live holder or a fresh replayed beat.
contract/viola-revive-json/cwd-missing · viola revive --json error codes · cwd-missing — The code for a missing recorded directory.
contract/viola-revive-json/no-session · viola revive --json error codes · no-session — The code for both no-logged-session pairs: no logged session, and no logged session with that id.
contract/viola-revive-json/session-live · viola revive --json error codes · session-live — The code for a session open in another claude.
contract/viola-revive-json/stale-heartbeat · viola revive --json error codes · stale-heartbeat — The code for a stale holder.
contract/viola-revive-json/strict-modes-failed · viola revive --json error codes · strict-modes-failed — The code for a refused home tree or instance state files.
contract/viola-revive-refusals/already-live · viola revive refusals · already live — `unable: <name> is already live` / `hint: viola list`, the same collision pair as `viola run`, with its stale sibling unchanged.
contract/viola-revive-refusals/directory-missing · viola revive refusals · recorded directory missing — `unable: <name>'s recorded directory is missing` / `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`; no paths, no pids, no logged id.
contract/viola-revive-refusals/no-logged-session · viola revive refusals · no logged session — `unable: <name> has no logged session to resume` / `hint: start one: viola run <name> -- claude`.
contract/viola-revive-refusals/no-session-with-id · viola revive refusals · no logged session with that id — `unable: <name> has no logged session with that id` / `hint: viola revive <name> --list`.
contract/viola-revive-refusals/session-open-elsewhere · viola revive refusals · session open in another claude — `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`.
contract/viola-revive-refusals/state-files-writable · viola revive refusals · state files writable by another user — `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only`.
contract/viola-revive/no-window · viola revive · window placement — viola revive places no window unless given its hidden `--origin <cli|agent>`, under which the revived window moves to the workspace a real snapshot recorded, or reads unplaced when none was recorded.
contract/viola-revive/preflight-order · viola revive start · preflight order — The start reads its preflight in a fixed order: the holder, the logged session, session-live, then the recorded directory.
contract/viola-revive/session-live-check · viola revive start · session-live check — One `claude agents --json` read; a row whose `sessionId` is the logged id refuses; skipped under `--fork`; a failed read passes and the start goes on.
contract/viola-revive/start-output · viola revive · passed start output — A `viola revive` start that passes its preflight prints nothing of its own while the resumed child holds the terminal, as `viola run`.
contract/viola-run/already-live · viola run start refusals · already live — Exit 1 with `unable: <name> is already live` on stderr and `hint: viola list` as the last stderr line; no paths, no pids.
contract/viola-run/start-output · viola run start · passed start output — Before the child, only a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`); the trust step adds no refusal, exit code, hint, `--json` code or terminal byte; then the terminal belongs to the wrapped claude TUI until it exits.
contract/viola-send-output/word-column · viola send output · word column — The word column is padded so open, read back, unable and unconfirmable line up, and the name always starts in the same column.
contract/viola-verify-output/exit-1 · viola verify output · exit 1 — Exit 1 on a failing row, a refusal (the unable: / hint: pairs) or a stop.
contract/viola-verify-output/failing-row · viola verify output · failing row — A run with a failing row exits 1 and still writes the stamp.
contract/viola-verify-output/help-paragraph · viola verify output · --help — viola verify --help adds one static ASCII paragraph naming no path: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe.
contract/viola-verify-output/plain-ascii · viola verify output · plain ASCII — All viola verify output is plain ASCII with no colour.
contract/viola-verify-output/step-line · viola verify output · step line — One stdout step line per relied ledger row, `[NN/MM] <row id> <row words>  pass|absent|fail`, in ledger order, MM being the relied count; `absent` is the browser-tools-prompt row's word alone, when no typed run met the prompt; each appended once, with no progress bar and no redraw.
contract/viola-verify-output/stop · viola verify output · stop — A run stopped by SIGINT or SIGTERM before its rows are checked prints only the plain ASCII unable: / hint: pair on stderr, with no name, path or pid, stdout empty, and writes no stamp.
contract/viola-verify-output/summary · viola verify output · summary — `stamped <ver>  <n> pass  <m> fail` is the last stdout line, `n` counting the rows that count, an `absent` browser-tools-prompt row among them; `fail` is never coloured.
contract/viola-verify-refusals/cli-not-found · viola verify refusals · claude CLI not found — Exit 1, `unable: the claude CLI was not found` / `hint: install Claude Code or put it on PATH`.
contract/viola-verify-refusals/home-not-private · viola verify refusals · viola home not private — A refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives the home pair, exit 1, stdout empty, no step line, no summary.
contract/viola-verify-refusals/stopped · viola verify refusals · verify stopped — A run stopped by Ctrl-C or TERM is exit 1, stdout empty, no summary, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, the hint last on stderr.
contract/viola-verify-refusals/version-unreadable · viola verify refusals · CLI version unreadable — Exit 1, `unable: the CLI version could not be read` / `hint: run claude --version to check the install`.
contract/viola-verify/summary · viola verify output · summary line — `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary.
decision/board-vs-strip · Board versus strip verbs — `viola list` shows the whole bay, `viola up` re-opens the whole fleet or the instances it names, `verify` and `plugin install` name no strip, and every other verb a person runs acts on one strip named by its `ViolaName`.
decision/human-tty-expression-level · Human TTY expression level — On a human TTY the cli prints static fixed-column rows and one static context header, with colour only as a second cue.
decision/no-progress-glyphs · No send progress glyphs — A send shows no `Sending…` spinner, no `✓` and no `done`.
decision/no-terminator-word · No terminator word — No `done.`, `success` or summary banner; the exit code is the terminator and the last printed line says what happened, to whom and where.
decision/plain-output-modes · Plain output modes — Under --json, a non-TTY, NO_COLOR, TERM=dumb and the viola run / viola revive passthrough the cli prints no colour, no non-ASCII glyph and no cursor control.
decision/single-progress-pattern · Single progress pattern — The step counter of viola verify, which stamps a CLI version, is the project's only progress pattern.
contract/cli-exit-codes/11 · CLI exit codes · 11 — Retired (was `budget-paused`), never reused; no refusal maps to 11.
contract/cli-exit-codes/20 · CLI exit codes · 20 — Wrapper fault, `error: wrapper fault  <code>`.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable`; for `viola list`, `viola plugin install`, the board refusal of `viola up` and the strict-modes refusal of the frame verbs and `viola focus` the fixed state pair instead.
contract/viola-open/parent-default · viola open · --parent default — The parent defaults to the calling agent's own ViolaName, recorded with origin `agent`; from a plain shell there is no parent and the origin is `cli`.
contract/viola-revive-json/refusal-document · viola revive --json · refusal document — Each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr; without `--json` the pairs and rows stay byte-identical.
contract/viola-revive/refusal-shape · viola revive refusals · refusal shape — Each `viola revive` refusal is exit 1 with one `unable:` line and one `hint:` line last on stderr and an empty stdout.
contract/viola-verify-output/exit-0 · viola verify output · exit 0 — Exit 0 when every relied row counts: it reads pass, or absent on the browser-tools-prompt row alone.
contract/viola-verify-output/record · viola verify output · --record — --record adds no human line.
contract/viola-verify-refusals/fixture-not-clean · viola verify refusals · recorded fixture not clean — Exit 1, `unable: a recorded fixture is not clean: <file> <code>` (screen fixture `<file> row <n>[ seam] <code>`; codes `home-path`, `absolute-path`, `username`, `email`), never the content / `hint: record with a viola home under your user home`.

# layout-templates · its absences, names and overviews, whole

## surface-web-spa · ## Surface: web-spa
absence/web-surface — A web surface: viola renders no page

## surface-cli · ## Surface: cli
absence/cli-deep-links — Deep links or URL coupling in the CLI
