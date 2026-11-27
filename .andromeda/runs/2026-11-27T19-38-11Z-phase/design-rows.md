# design-system · the rows that bind 2026-11-27-epoch-5-cleanup — picked 105 of 361 (68 by every draw) · rule any · pin 5129

## brand-identity · ## Brand Identity
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` or `viola adopt` that passed its preflight.
decision/one-strip-per-run-instance · One flight-progress strip per instance — Every `viola run` instance is shown as exactly one flight-progress strip.
contract/cli-verification/unverified-cli · CLI verification state · unverified-cli — A CLI build with no stamp reading every relied row `pass` runs transport only, shown as `unverified-cli`.
contract/cli-verification/verified · CLI verification state · verified — A CLI version whose `viola verify` stamp reads every relied row `pass` prints `verified`; a stamp lacking a relied row, or reading one `fail`, does not.

## iconography · ## Iconography
contract/cli-separators/two-spaces · CLI separators · two spaces — Two spaces separate columns, and a refusal's reason from its detail, as separate fields.

## surface-cli · ## Surface: cli · twin surface
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/no-hint-under-json · No hint line under --json — No `hint:` line is printed under `--json`; the typed `refusal` / `error` fields and their `detail` are the whole answer, exit 21's cause being its `detail`.
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run`, `viola revive` and `viola adopt` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
contract/auto-verify-line/did-not-verify · viola run and revive automatic-verify line · did not verify — A start whose automatic verify ran and left a stamp that is not green prints the fixed did-not-verify line naming the parsed claude version.
contract/auto-verify-line/earlier-red · viola run and revive automatic-verify line · earlier red — A start whose claude version already has an entry holding every relied row that is not green prints the fixed earlier-red line naming the parsed version and runs no verify.
contract/auto-verify-line/line-form · viola run and revive automatic-verify line · line form — A start prints exactly one of the five automatic-verify lines, each one stderr write, ASCII, with no SGR, holding only the parsed version and a closed word; none prints under `--json`.
contract/auto-verify-line/no-stamp · viola run and revive automatic-verify line · no stamp — A start whose automatic verify left no stamp, a verify refused for an untrusted folder included, prints the fixed no-stamp line naming the parsed version and one closed word of `spawn-failed`, `unstamped` or `lock-refused`; the version stays unattempted.
contract/auto-verify-line/verifying · viola run and revive automatic-verify line · verifying — A start that runs its automatic verify first prints the fixed verifying line naming the parsed claude version.
contract/auto-verify-line/verifying-elsewhere · viola run and revive automatic-verify line · verifying elsewhere — A start that finds another start holding the verify attempt lock prints the fixed verifying-elsewhere line naming the parsed version, runs no verify and starts unverified.
contract/cli-exit-codes/1 · CLI exit codes · 1 — `error: internal error` (fixed; no chain, path or hint) or a fixed `unable:` refusal of `run`, `revive`, `open`, `focus` or `verify` and its hint; `--json` only for revive's shared-start, open's and focus's refusals and the catch site's `{"v":1,"error":"internal-error","detail":null}`; stderr empty.
contract/cli-exit-codes/1-adopt-start · CLI exit codes · 1 of viola adopt — A start refusal of `viola adopt`, or its not-resumed end after the child ran, exits 1 with its fixed `unable:` / `hint:` pair, or under `--json` one stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; a resumed child ends exit 0.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` print the fixed `unable: <text>` form.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run`, `revive` or `adopt` prints a fixed-message `unable:` line and one hint per cause; `revive`'s and `adopt`'s causes are read in a fixed order that stops at the first.
contract/cli-verb-output/answer · CLI verb output · viola answer — `viola answer <name> <id>` prints `answered  <name>  dialog <id>`, or `unable  <name>  not-delivered  unknown-dialog`.
contract/cli-verb-output/up · CLI verb output · viola up — `viola up` prints one stdout line per instance, `<name>  <word>` or `<name>  live  <placement>` with a closed placement word, or under `--json` one document `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`.
contract/placement-word/malformed · placement word · malformed — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/oversize · placement word · oversize — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/placed · placement word · placed — One of the eight closed placement words `viola open` prints and the window record carries: the window was moved to the asked workspace.
contract/placement-word/refused · placement word · refused — One of the eight closed placement words `viola open` prints and the window record carries: the move was refused.
contract/placement-word/unplaced · placement word · unplaced — One of the eight closed placement words `viola open` prints and the window record carries: no workspace was asked.
contract/placement-word/unreachable · placement word · unreachable — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/unsupported · placement word · unsupported — One of the eight closed placement words `viola open` prints and the window record carries: off Hyprland, the open still live.
contract/placement-word/window-unseen · placement word · window-unseen — One of the eight closed placement words `viola open` prints and the window record carries: no window of the wrapper's own terminal was seen.
contract/revive-json-refusal-code/already-live · revive --json refusal code · already-live — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/cwd-missing · revive --json refusal code · cwd-missing — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/no-session · revive --json refusal code · no-session — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/pinned-hash-mismatch · revive --json refusal code · pinned-hash-mismatch — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/session-live · revive --json refusal code · session-live — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/socket-dir-refused · revive --json refusal code · socket-dir-refused — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/squatted-name · revive --json refusal code · squatted-name — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/stale-heartbeat · revive --json refusal code · stale-heartbeat — One of the closed set of codes a `revive --json` start refusal document carries.
contract/revive-json-refusal-code/strict-modes-failed · revive --json refusal code · strict-modes-failed — One of the closed set of codes a `revive --json` start refusal document carries.
contract/viola-adopt-refusals/session-live · viola adopt refusals · session-live — A session open in another `claude` is refused with exit 1 and no child started: `unable: that session is open in another claude` then `hint: close it there first, then adopt it again`.
contract/viola-revive-output/json-refusal · viola revive output · --json refusal — Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
contract/viola-up-outcome/already-live · viola up outcome words · already-live — `already-live` is one of the nine closed outcome words of `viola up`: the instance is live and is not re-opened.
contract/viola-up-outcome/cwd-missing · viola up outcome words · cwd-missing — `cwd-missing` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
contract/viola-up-outcome/live · viola up outcome words · live — `live` is one of the nine closed outcome words of `viola up`: the spawned instance came up live, and its line carries the placement word.
contract/viola-up-outcome/no-session · viola up outcome words · no-session — `no-session` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
contract/viola-up-outcome/not-live · viola up outcome words · not-live — `not-live` is one of the nine closed outcome words of `viola up`: a spawned instance that did not come up live; a line reading it makes `up` exit 1.
contract/viola-up-outcome/not-taken · viola up outcome words · not-taken — `not-taken` is one of the nine closed outcome words of `viola up`: a name given that the scan did not find, or a named instance whose heartbeat is not older than the boot; a line reading it makes `up` exit 1.
contract/viola-up-outcome/session-live · viola up outcome words · session-live — `session-live` is one of the nine closed outcome words of `viola up`, and a line reading it makes `up` exit 1.
contract/viola-up-outcome/stale-heartbeat · viola up outcome words · stale-heartbeat — `stale-heartbeat` is one of the nine closed outcome words of `viola up`: the instance reads stale and is not re-opened.
contract/viola-up-outcome/would-open · viola up outcome words · would-open — `would-open` is one of the nine closed outcome words of `viola up`: a `--dry-run` plan line for an instance that would be spawned.
contract/viola-up-output/dry-run · viola up output · --dry-run — `viola up --dry-run` prints the plan as `would-open` lines and spawns nothing.
contract/viola-up-output/line-order · viola up output · line order — Every `viola up` line that is not a spawn comes first, in name order, then the spawned or `would-open` lines in spawn order, parents before children.
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
decision/permission-request-question-held · PermissionRequest question held for an answer — A question first raised by a PermissionRequest is held as a pending dialog and answered by `viola answer` like any dialog; a human key, an expired hold or an unverified CLI leave it to the human.
decision/step-counter-only-progress-pattern · Step counter as the one progress pattern — The step counter of `verify` is the project's one progress pattern.
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-exit-codes/21 · CLI exit codes · 21 — `unable  instance-unreachable` then the cause's hint; strict-modes refusals print the fixed home-state or instance-state `unable:` / `hint:` pair instead, with no frame; under `--json` `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, stderr empty.
contract/cli-refusal-line/two-space-form · CLI refusal line · two-space form — A refusal prints `unable  <name>  <reason>  <detail>` on stderr, two spaces between fields.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/cli-streams/stdout · CLI streams · stdout — stdout carries results and data: tables, send/wait results, `last` text.
contract/revive-json-refusal-code/internal-error · revive --json refusal code · internal-error — One of the closed set of codes a `revive --json` start refusal document carries.
contract/viola-run-output/child-started · viola run and revive output · child started — `run`, `revive` and `adopt` print nothing once the child starts, except `adopt`'s not-resumed end, which writes its fixed pair or `--json` document after the child released the terminal.
contract/viola-verify-output/step-line · viola verify output · step line — One static appended uncoloured stdout line per relied ledger row, counted against `MM`, the relied count, ending in one verdict word of `pass`, `absent`, `fail`; `absent` is written only on the `browser-tools-prompt` row, when no run met the prompt, and counts as a pass on that row alone.
contract/viola-verify-output/summary · viola verify output · summary — A summary line `stamped <ver>  <n> pass  <m> fail` follows, n + m being the relied count, `n` counting the rows read `pass` and the `browser-tools-prompt` row read `absent`.
contract/viola-verify-output/trust-record-names-nothing · viola verify output · trust-record row — The trust-record row prints its step line ending `pass` or `fail`, and no step line, summary or refusal of `viola verify` names the config file, the trust record's key or its value.

## surface · ## Surface: cli · twin surface-cli
decision/hints-human-mode-only · Hints in human mode only — Hints print in human mode only; under `--json` no hint text is printed and the machine view carries the cause as a detail code, never as prose.
decision/json-failure-documents-clean · Clean --json failure documents — Each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text. · guard: tests/cli_error_bodies.rs
decision/no-json-after-child-held-terminal · No document once the child held the terminal — An error while pumping the child under `viola revive --json` or `viola adopt --json` prints no document. · why: The child has held the terminal and viola writes nothing past it. · except: `viola adopt`'s not-resumed end writes its fixed pair or `--json` document after the child released the terminal; `run` and `revive` still write nothing past the child.
contract/cli-catch-site/silent-verbs · CLI catch site · silent verbs — `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent at the catch site, under `--json` too.
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` use `unable: <text>`.
contract/cli-stderr/hint-line · CLI stderr · hint line — Each refusal's `hint:` line follows it directly on stderr.
contract/exit-21-json-cause/strict-modes-failed · exit 21 --json causes · strict-modes-failed — The `list`, `link`, `unlink`, `focus`, `up` and `plugin install` refusals and the seven frame verbs' strict-modes refusals read `strict-modes-failed`.
contract/log-detail-codes/socket-dir-refused · log detail codes · socket-dir-refused — Written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 socket-directory refusal, and at `run`'s and `revive`'s start.
contract/viola-revive-json-codes/already-live · viola revive --json exit-1 codes · already-live — `already-live` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/internal-error · viola revive --json exit-1 codes · internal-error — A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
contract/viola-revive-json-codes/pinned-hash-mismatch · viola revive --json exit-1 codes · pinned-hash-mismatch — `pinned-hash-mismatch` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/socket-dir-refused · viola revive --json exit-1 codes · socket-dir-refused — A failed create of the socket directory and a refused check both print one stdout document with the code `socket-dir-refused`, no hint, stderr empty.
contract/viola-revive-json-codes/squatted-name · viola revive --json exit-1 codes · squatted-name — `squatted-name` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/stale-heartbeat · viola revive --json exit-1 codes · stale-heartbeat — `stale-heartbeat` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/strict-modes-failed · viola revive --json exit-1 codes · strict-modes-failed — `strict-modes-failed` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-refusals/home-tree-check · viola revive refusals · home-tree check — The home-tree check of its start, after a passed preflight and before any log, with no child, prints the home-state or instance-state pair, or under `--json` the `strict-modes-failed` document.
contract/viola-revive-refusals/order · viola revive refusals · order — `viola revive` reads its refusal causes in a fixed order that stops at the first refusal; its already-live and stale causes print `run`'s lines and hints unchanged.
contract/viola-revive-refusals/socket-dir · viola revive refusals · socket directory — A refused socket directory at `viola revive`'s start prints `run`'s socket-directory pair.
contract/viola-revive-refusals/strict-modes-no-log · viola revive refusals · strict-modes — `viola revive`'s strict-modes refusal writes no log line and carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1.
contract/viola-up-output/not-live-hint · viola up output · not-live hint — After its lines, `viola up` writes one fixed stderr `hint:` line when any line reads `not-live`, and none under `--json`.
decision/no-hint-names-viola-release · No hint names viola release — No refusal hint ever names `viola release`; `release` stays a human verb. · why: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
contract/cli-exit-codes/12 · CLI exit codes · 12 — `unable  unverified-cli`; under `--json` the refusal object.
contract/exit-21-json-cause/instance-dead · exit 21 --json causes · instance-dead — With no live endpoint, an instance directory that is a directory reads `instance-dead`, as do a failed connect and a frame verb against a stale instance; stale heartbeat is exit 1's cause alone.
contract/exit-21-json-cause/unwrapped-name · exit 21 --json causes · unwrapped-name — With no live endpoint, a frame verb whose instance directory, read after the strict-modes check, is absent or a regular file reads `unwrapped-name`; a symbolic link in its place is refused by the strict-modes check and reads `strict-modes-failed`.
contract/fixed-refusal-pairs/boot-time-unread · fixed refusal pairs · boot-time-unread pair — When the host boot time cannot be read, `viola up` selects nothing and prints its own fixed `unable:` / `hint:` pair, or under `--json` an exit-1 document with the code `boot-time-unread`.
contract/fixed-refusal-pairs/home-state · fixed refusal pairs · home-state pair — `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
contract/fixed-refusal-pairs/instance-state · fixed refusal pairs · instance-state pair — `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
contract/refusal-hints/unverified-cli · refusal hints · unverified-cli — `run viola verify for this CLI version`
contract/strict-modes/home-under-regular-file · strict-modes refusals · home under a regular file — A home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21).
contract/viola-revive-json-codes/cwd-missing · viola revive --json exit-1 codes · cwd-missing — `cwd-missing` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-json-codes/session-live · viola revive --json exit-1 codes · session-live — `session-live` is one of the closed exit-1 codes of `viola revive --json`.
contract/viola-revive-refusals/cwd-missing · viola revive refusals · cwd-missing — `unable: <name>'s recorded directory is missing` then `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`.
contract/viola-revive-refusals/session-live · viola revive refusals · session-live — When one agents read lists the logged id live in another `claude` (skipped under `--fork`; a failed read passes): `unable: <name>'s session is open in another claude` then `hint: close it there first, or start a copy: viola revive <name> --fork`.
contract/viola-run-start-refusals/already-live · viola run start refusals · already-live — `unable: <name> is already live` then `hint: viola list`; a live holder and a fresh replayed beat log `already-live`.
contract/viola-run-start-refusals/home-tree-check · viola run start refusals · home-tree check — At start, before any log and with no child, the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/` are checked, each that exists; a home-level refusal prints the home-state pair, an instance-level one the instance-state pair.
contract/viola-run-start-refusals/pinned-hash-mismatch · viola run start refusals · pinned-hash-mismatch — A SHA-256 mismatch prints `unable: the pinned viola copy failed its integrity check` then `hint: the pinned copy was changed after it was written, so viola will not run it`.
contract/viola-run-start-refusals/socket-dir · viola run start refusals · socket directory — Before any snapshot, child or bind: `unable: the socket directory for <name> cannot be trusted` then `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
contract/viola-run-start-refusals/squatted-name · viola run start refusals · squatted-name — `unable: the endpoint for <name> is held by another process` then `hint: another process holds this name's endpoint; stop it or pick another name`.
contract/viola-run-start-refusals/stale-heartbeat · viola run start refusals · stale-heartbeat — A stale heartbeat with a live pid prints `unable: <name> is still running but not answering` then `hint: viola list shows it as stale; stop that process before starting <name> again`, and logs `stale-heartbeat`.

# design-system · its absences, names and overviews, whole

## brand-identity · ## Brand Identity
N/A — viola has no web or graphical frontend, so no frontend expression level.
overview/brand-identity — The direction the CLI carries, and the terminal fleet console inherits, is a control-tower strip bay after dark: buff paper strips in anthracite holder boots on ruled aluminium racks. Every session holds the same slot for its whole life, and the only warm light is the one arrival holder cocked out of line when a session needs the human.

## color-palette · ## Color Palette
Surface Scale (elevation hierarchy) › N/A — viola renders no page, so no surface scale. ¶ Text Hierarchy › N/A — viola renders no page, so no text hierarchy; the cli's ink is the terminal's (Surface: cli). ¶ Semantic Colors › N/A — viola renders no page, so no colour states; the cli's status guards below stand.

## typography · ## Typography
N/A — viola renders no page; the cli's type is the terminal's (below).

## spacing · ## Spacing
N/A — viola renders no page; the cli's spacing is the terminal grid (below).

## depth-strategy · ## Depth Strategy
N/A — viola renders no page, so no depth tokens; the cli draws no depth.

## border-radius · ## Border Radius
N/A — viola renders no page, so no radius tokens.
absence/rendered-page — A rendered page: viola renders no page, so no radius, motion or spacing tokens; the cli's type is the terminal's

## surface-web-spa · ## Surface: web-spa
absence/web-page-surface — A web page or web surface of any kind

## surface-cli · ## Surface: cli · twin surface
absence/cli-tape — A tape in the CLI
name/command/answer — answer
name/command/focus — focus
name/command/hook — hook
name/command/last — last
name/command/link — link
name/command/list — list
name/command/mcp — mcp
name/command/open — open
name/command/pause — pause
name/command/plugin-install — plugin install
name/command/release — release
name/command/revive — revive
name/command/run — run
name/command/send — send
name/command/unlink — unlink
name/command/up — up
name/command/verify — verify
name/command/wait — wait

## self-validation-protocol · ## Self-Validation Protocol
4. Token Test › N/A — viola renders no page, so no stylesheet tokens to trace. ¶ 4. Token Test › N/A — viola renders no page, so no stylesheet token to trace.
