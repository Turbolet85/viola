### brand-identity#2.2 · values · expression-motion · ## Brand Identity · lines 21–21
| cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
### color-palette#5.22 · values · status-states · ## Color Palette › ### Semantic Colors · lines 83–83
| InstanceBudgetPaused::true | #2A2E33 | #9C9278 | #E6D8AE | The WHEEL cell prints `driver · budget-paused`: minimum fuel declared, new sends stop, the running turn finishes |
### color-palette#5.27 · values · status-states · ## Color Palette › ### Semantic Colors · lines 88–88
| BudgetEnvelope.paused::true | #1E2124 | #E6D8AE (1px box around the word) | #E6D8AE | `budget-paused`, boxed like a declared state. No amber, no red. |
### color-palette#5.28 · values · status-states · ## Color Palette › ### Semantic Colors · lines 89–89
| BudgetWindow::five-hour | #1E2124 | #4A5057 | #E6D8AE | Label `5H`. Detail word `five-hour` in refusals. |
### color-palette#5.29 · values · status-states · ## Color Palette › ### Semantic Colors · lines 90–90
| BudgetWindow::seven-day | #1E2124 | #4A5057 | #E6D8AE | Label `7D`. Detail word `seven-day` in refusals. |
### color-palette#5.35 · values · status-states · ## Color Palette › ### Semantic Colors · lines 96–96
| Readback::refused (from CL-1 `send-refused`) | #1E2124 + 1px #E6D8AE `/` strike | #E6D8AE | #E6D8AE | Strike, then `unable`, then `reason · detail` |
### color-palette#5.38 · values · status-states · ## Color Palette › ### Semantic Colors · lines 99–99
| RefusalReason::budget-paused | #1E2124 | #E6D8AE | #E6D8AE | `unable · budget-paused` |
### color-palette#5.44 · values · status-states · ## Color Palette › ### Semantic Colors · lines 105–105
| RefusalDetail::five-hour | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · five-hour` |
### color-palette#5.45 · values · status-states · ## Color Palette › ### Semantic Colors · lines 106–106
| RefusalDetail::seven-day | #1E2124 | #E6D8AE | #E6D8AE | `budget-paused · seven-day` |
### iconography#1.3 · values · icons · ## Iconography · lines 10–12
- **Separator.** Middle dot `·` on the web. The CLI is ASCII only:
  - Two spaces separate columns, and a refusal's reason from its detail (they are separate fields, as in the exit-code table).
  - ` - ` (space, hyphen, space) replaces `·` inside one field or label: `driver - budget-paused`, `skipped 0 - 0 - 0`, `-- UNWRAPPED - READ-ONLY --`.
### surface#2.1 · values · cli-output · ## Surface: cli › ### Tokens (platform-specific) · lines 21–21
- stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it, and errors (`error: …`).
### surface#5.1 · values · cli-send · ## Surface: cli › ### Component Patterns · lines 43–74
- A refusal prints `unable` plus `reason  detail`, then one `hint:` line, keyed by the reason (or by reason · detail where listed):
  - `human-typing`: `the human has the wheel; send again after the human hands it back`
  - `budget-paused`: `budget gate is closed; it lifts at the window reset, or when the human lifts it`
  - These two refusals reach drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20). So no hint ever names `viola release` (T3). `release` stays a human verb, documented under Wheel, link and gate verbs.
  - `unverified-cli`: `run viola verify for this CLI version`
  - `not-delivered · turn-running`: `a turn is running; viola wait <name> first`
  - `not-delivered · control-character`: `the text contains a control character (only LF, CR, TAB are allowed)`
  - `not-delivered · empty-text`: `the text is empty once its trailing newlines are removed; send a text with content`
  - `not-delivered · input-not-ready`: `<name> was not ready for input; send again, and if it repeats a human must look at the session`
  - `not-delivered · no-prompt-submitted`: `<name> did not submit the prompt; check it, then send again` (the sample above)
  - `not-delivered · unknown-dialog`: `that dialog is not pending; viola list shows the current DIALOG`
  - `instance-unreachable` (exit 21) has one hint per cause, because the code is shared (T4):
    - no wrapper is running for the name: `<name> is not running; viola list shows the live instances`
    - the name is an unwrapped session: `<name> is not a wrapped session; only sessions started with viola run take commands`
    - the viola home fails strict-modes: `the viola home is not private to you; <name> is not contacted until that is fixed`
    - server verification fails: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`
  - The exit-1 start refusals of `viola run` have one fixed-message line and one hint per cause (T4). None of them shows a path or a pid:
    - `unable: <name> is already live` → `hint: viola list`
    - `unable: <name> is still running but not answering` (stale heartbeat, live pid) → `hint: viola list shows it as stale; stop that process before starting <name> again`
    - `unable: the endpoint for <name> is held by another process` (squatted name) → `hint: another process holds this name's endpoint; stop it or pick another name`
    - `unable: the pinned viola copy failed its integrity check` (SHA-256 mismatch) → `hint: the pinned copy was changed after it was written, so viola will not run it`
    - `unable: <name>'s command is a .cmd or .bat script` (Windows) → `hint: pass the real executable, not a .cmd or .bat shim`
    - `unable: the viola home is not private to you` (strict-modes at start) → `hint: the viola home must be readable only by you; viola does not change its permissions`
  - The exit-1 refusals of `viola revive` take the same form, one fixed-message line and one hint per cause, read in a fixed order that stops at the first refusal. None of them shows the recorded directory, a pid or a logged id, and no hint names `viola release`:
    - `unable: <name>'s state files can be written by another user` (strict-modes on the instance's files) → `hint: viola will not read them; make the viola home and its files owner-only`
    - `unable: <name> is already live` and its stale sibling: `run`'s two lines and hints above, unchanged
    - `unable: <name> has no logged session to resume` → `hint: start one: viola run <name> -- claude`
    - `unable: <name> has no logged session with that id` (an `--id` the log does not hold) → `hint: viola revive <name> --list`
    - `unable: <name>'s session is open in another claude` (one `claude agents --json` read lists the logged id live in another `claude`; skipped under `--fork`; a failed read passes) → `hint: close it there first, or start a copy: viola revive <name> --fork` (the founder's wording, his own answer to P4 fork 4 of chunk 2026-10-16-unwrapped-sessions-on-the-board, `inputs#I1`, relayed verbatim by the operator)
    - `unable: <name>'s recorded directory is missing` → `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`
  - Hints are human-mode only. Under `--json`, no hint text is printed; the machine view carries the cause as a detail code, never as prose. **Pending an arch amendment:** arch currently fixes `{"v":1,"error":"instance-unreachable","detail":null}` for exit 21, and for exit 1 only `viola revive`'s `{"v":1,"error":"<code>","detail":null}` (its strict-modes and preflight refusals; chunk 2026-10-17-cli-machine-contract). Until arch names the `detail` codes, `--json` keeps arch's shape. obs-plan D-20 already names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and for `viola revive` `no-session` and `cwd-missing` (its strict-modes refusal writes no log line since chunk 2026-10-10-budget-governor, so it carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1). The stale-heartbeat and unwrapped-name causes have no code yet.
  - No hint line for `unknown` (exit 14), because its detail is opaque and a hint would have to guess. No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals, and their detail goes only to `diagnostics/`.
### surface#8.1 · values · cli-verify-run · ## Surface: cli › ### Component Patterns · lines 93–93
- **`revive`:** prints nothing once the child starts, as `run`. `viola revive builder --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>` with two spaces between fields: no header, no colour, plain ASCII. A refusal (exit 1) prints its cause's fixed-message line and then that cause's hint, listed under the send hints above. Under `--json` (both arms; chunk 2026-10-17-cli-machine-contract) a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) and no hint, stderr empty, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to the working-route entry "Exit-cause code catalogue").
### surface#9.2 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 102–102
| 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint), or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check` / `<name>'s command is a .cmd or .bat script` / `the viola home is not private to you`, or a `viola revive` refusal `unable: <name>'s state files can be written by another user` / `<name> has no logged session to resume` / `<name> has no logged session with that id` / `<name>'s session is open in another claude` / `<name>'s recorded directory is missing` (or `run`'s `already live` pair), or a `viola verify` refusal `unable: the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version could not be read` / `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; codes `home-path` · `absolute-path` · `username` · `email`; never the content), each followed by its own hint, or a `viola verify` stopped by Ctrl-C or TERM before its rows are checked, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` (stdout empty); a `verify` run with a failing ledger row prints no stderr word (its last stdout line reads `stamped <ver>  <n> pass  <m> fail`) | `viola revive --json`'s refusals of its strict-modes check and its preflight: `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; arch §Standard Contracts); every other exit 1 has no `--json` document (a `run` start refusal reached through `revive` included, owed to "Exit-cause code catalogue") |
### surface#9.5 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 105–105
| 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
