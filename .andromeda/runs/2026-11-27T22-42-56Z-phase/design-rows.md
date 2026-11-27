# design-system · the rows that bind 2026-11-27-epoch-5-cleanup-2 — picked 49 of 361 (27 by every draw) · rule any · pin 5179

## brand-identity · ## Brand Identity
decision/no-success-before-confirmation · Nothing announces success before confirmation — Nothing announces success before it is confirmed; sends stay visibly open until they are read back.
decision/run-prints-nothing-while-wrapped · viola run silent while wrapped — `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` or `viola adopt` that passed its preflight.

## surface-cli · ## Surface: cli · twin surface
decision/json-one-document-on-stdout · --json result is one stdout document — Under `--json` the whole result, refusals and `error` objects included, is one JSON document on stdout with the typed exit code. · except: an oversize `viola answer` and an unusable `viola send` exit 2 with their fixed stderr line, no document; a `pump_child` error under `viola revive --json` prints none, the child holding the terminal
decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, `up` acts on the whole fleet or on the instances it names, and every other verb a person runs acts on one strip.
contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify` and the exit-21 strict-modes refusals of `list`, `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` print the fixed `unable: <text>` form.
contract/cli-verb-output/up · CLI verb output · viola up — `viola up` prints one stdout line per instance, `<name>  <word>` or `<name>  live  <placement>` with a closed placement word, or under `--json` one document `{"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}`.
contract/placement-word/malformed · placement word · malformed — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/oversize · placement word · oversize — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/placed · placement word · placed — One of the eight closed placement words `viola open` prints and the window record carries: the window was moved to the asked workspace.
contract/placement-word/refused · placement word · refused — One of the eight closed placement words `viola open` prints and the window record carries: the move was refused.
contract/placement-word/unplaced · placement word · unplaced — One of the eight closed placement words `viola open` prints and the window record carries: no workspace was asked.
contract/placement-word/unreachable · placement word · unreachable — One of the eight closed placement words `viola open` prints and the window record carries.
contract/placement-word/unsupported · placement word · unsupported — One of the eight closed placement words `viola open` prints and the window record carries: off Hyprland, the open still live.
contract/placement-word/window-unseen · placement word · window-unseen — One of the eight closed placement words `viola open` prints and the window record carries: no window of the wrapper's own terminal was seen.
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
decision/silent-catch-sites · Silent catch sites — At the catch site, `run`, `revive`'s start, `adopt`, `mcp` and `hook` stay silent.
decision/start-refusals-show-no-path-or-pid · Start refusals show no path or pid — The exit-1 refusals of `viola run`, `viola revive` and `viola adopt` print one fixed-message line and one hint per cause, and none shows a path, a pid, the recorded directory or a logged id.
contract/auto-verify-line/no-stamp · viola run and revive automatic-verify line · no stamp — A start whose automatic verify left no stamp, a verify refused for an untrusted folder included, prints the fixed no-stamp line naming the parsed version and one closed word of `spawn-failed`, `unstamped` or `lock-refused`; the version stays unattempted.
contract/cli-colour-decision-order/viola-run · CLI colour decision order · viola run — Second match of the colour decision order: under `viola run` no colour, glyph or SGR is printed, and while the child runs nothing is printed at all.
contract/cli-exit-codes/0 · CLI exit codes · 0 — No stderr first word (TTY context lines only); the result line goes to stdout; under `--json` the document `{"v":1,"ok":{…}}`.
contract/cli-refusal-line/hint · CLI refusal line · hint — A refusal is followed directly by one `hint:` line per reason, detail or cause.
contract/cli-start-refusal/fixed-message · run and revive start refusals · fixed message — A start refusal of `run`, `revive` or `adopt` prints a fixed-message `unable:` line and one hint per cause; `revive`'s and `adopt`'s causes are read in a fixed order that stops at the first.
contract/cli-streams/stderr · CLI streams · stderr — stderr carries context lines, refusals with each refusal's `hint:` line directly after it, and errors.
contract/revive-json-refusal-code/pinned-hash-mismatch · revive --json refusal code · pinned-hash-mismatch — One of the closed set of codes a `revive --json` start refusal document carries.
contract/viola-revive-output/json-refusal · viola revive output · --json refusal — Under `--json` every refusal of the start `revive` shares with `run` prints one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` and no hint, stderr empty; `run` itself prints none of these documents; a `pump_child` error prints none.
contract/viola-run-output/child-started · viola run and revive output · child started — `run`, `revive` and `adopt` print nothing once the child starts, except `adopt`'s not-resumed end, which writes its fixed pair or `--json` document after the child released the terminal.

## surface · ## Surface: cli · twin surface-cli
contract/cli-stderr/fixed-message-refusal · CLI stderr · fixed message form — The exit-1 refusals of `viola run`, `revive`, `adopt`, `open`, `focus`, `up` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `focus`, `send`, `wait`, `last`, `answer`, `pause`, `release` and `switch` use `unable: <text>`.
contract/exit-21-json-cause/strict-modes-failed · exit 21 --json causes · strict-modes-failed — The `list`, `link`, `unlink`, `focus`, `up` and `plugin install` refusals and the seven frame verbs' strict-modes refusals read `strict-modes-failed`.
contract/fixed-refusal-pairs/boot-time-unread · fixed refusal pairs · boot-time-unread pair — When the host boot time cannot be read, `viola up` selects nothing and prints its own fixed `unable:` / `hint:` pair, or under `--json` an exit-1 document with the code `boot-time-unread`.
contract/viola-up-output/not-live-hint · viola up output · not-live hint — After its lines, `viola up` writes one fixed stderr `hint:` line when any line reads `not-live`, and none under `--json`.
decision/no-json-after-child-held-terminal · No document once the child held the terminal — An error while pumping the child under `viola revive --json` or `viola adopt --json` prints no document. · why: The child has held the terminal and viola writes nothing past it. · except: `viola adopt`'s not-resumed end writes its fixed pair or `--json` document after the child released the terminal; `run` and `revive` still write nothing past the child.
contract/fixed-refusal-pairs/home-state · fixed refusal pairs · home-state pair — `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions`.
contract/fixed-refusal-pairs/instance-state · fixed refusal pairs · instance-state pair — `unable: <name>'s state files can be written by another user` then `hint: viola will not read them; make the viola home and its files owner-only`, naming the instance checked.
contract/viola-revive-json-codes/internal-error · viola revive --json exit-1 codes · internal-error — A program not found, a cause with no human pair, prints `internal-error`; an anyhow error of the start prints the same document, and the catch site logs its chain and its line.
contract/viola-revive-output/after-child-start · viola revive output · after the child starts — `viola revive`, as `run`, prints nothing once the child starts.
contract/viola-revive-refusals/order · viola revive refusals · order — `viola revive` reads its refusal causes in a fixed order that stops at the first refusal; its already-live and stale causes print `run`'s lines and hints unchanged.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
decision/no-internals-in-errors · No internals in errors or hints — Errors and hints never print upstream text, paths, pids or anyhow chains with serde sources. · why: The security plan's NEVER-log floor.
decision/plain-output-modes · Plain output modes — No colour, glyphs, cursor control or spinner are emitted under `--json`, non-TTY, `NO_COLOR`, `TERM=dumb`, `viola run`, or a passed `viola revive`. · why: LLM drivers parse this output, and `viola run`'s terminal belongs to the child's screen.
decision/stdout-data-stderr-messages · Data on stdout, messages on stderr — Results go to stdout; `waiting:`, refusals with their `hint:` lines, and errors go to stderr.

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
