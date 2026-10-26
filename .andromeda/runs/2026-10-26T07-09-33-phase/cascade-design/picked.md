### brand-identity#1.1 · values · expression-motion · ## Brand Identity · lines 8–8
- `0.0-0.2`: static, grid-locked, zero animation, dense
### brand-identity#2.1 · values · expression-motion · ## Brand Identity · lines 18–20
| Surface | Expression | What it permits |
|---|---|---|
| cli, human TTY | 0.2 | Static k9s-style fixed-column rows and one static context header. Colour is only a second cue: amber `DIALOG` and dim `stale`, always beside the word. Nothing redraws in place and there are no spinners. |
### brand-identity#2.2 · values · expression-motion · ## Brand Identity · lines 21–21
| cli, `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough | 0.0 | No colour, no glyphs, no cursor control, no spinner. `viola run` prints nothing at all while the wrapped `claude` TUI runs, and neither does a `viola revive` that passed its preflight. |
### color-palette#1.3 · values · palette · ## Color Palette › ### Core Colors · lines 12–12
| Accent | #D97706 | The only attention colour: the 4px holder band and the `DIALOG` word of a cocked strip, and the CLI `DIALOG` word in `viola list` (the kind word after it stays uncoloured). The `DIALOG builder · viola` document title is plain text with no colour. Never used for errors, warnings, budget or hover. | Arrival-holder amber: the orange holder cocked out of line. It is the physical "needs you". |
### color-palette#5.33 · values · status-states · ## Color Palette › ### Semantic Colors · lines 94–94
| Readback::open (from CL-1 `send-issued`) | #1E2124 | #E6D8AE (1px) | #E6D8AE | Empty square, then the word `open` |
### color-palette#5.34 · values · status-states · ## Color Palette › ### Semantic Colors · lines 95–95
| Readback::read (matching `prompt-submitted`, or ledger post-condition met) | #E6D8AE | #E6D8AE | #3B3A36 in the box / #E6D8AE word | Box filled, `RB` inside, word `read back`. Instant, no transition. |
### color-palette#5.35 · values · status-states · ## Color Palette › ### Semantic Colors · lines 96–96
| Readback::refused (from CL-1 `send-refused`) | #1E2124 + 1px #E6D8AE `/` strike | #E6D8AE | #E6D8AE | Strike, then `unable`, then `reason · detail` |
### color-palette#5.42 · values · status-states · ## Color Palette › ### Semantic Colors · lines 103–103
| RefusalDetail::null | #1E2124 | #E6D8AE | #E6D8AE | Nothing after the reason (no dangling `·`) |
### color-palette#5.43 · values · status-states · ## Color Palette › ### Semantic Colors · lines 104–104
| RefusalDetail::manual-pause | #1E2124 | #E6D8AE | #E6D8AE | `human-typing · manual-pause` ("I have control" spoken via `viola pause`) |
### color-palette#5.49 · values · status-states · ## Color Palette › ### Semantic Colors · lines 110–110
| RefusalDetail::unknown-dialog | #1E2124 | #E6D8AE | #E6D8AE | `not-delivered · unknown-dialog` |
### color-palette#5.65 · values · status-states · ## Color Palette › ### Semantic Colors · lines 126–126
| EventKind::send-refused (CL-1) | #1E2124 | none | #E6D8AE | Folded into its send line: strike + reason. Never a separate line. |
### color-palette#5.77 · values · status-states · ## Color Palette › ### Semantic Colors · lines 138–138
| WheelCause::manual-pause | #1E2124 | none | #E6D8AE | `human · manual-pause`, i.e. "I have control" |
### color-palette#5.78 · values · status-states · ## Color Palette › ### Semantic Colors · lines 139–139
| WheelCause::release | #1E2124 | none | #E6D8AE | `driver · release`, i.e. "you have control" |
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
### surface#6.1 · values · cli-verbs · ## Surface: cli › ### Component Patterns · lines 80–80
- `viola release builder --budget` → `builder  budget gate lifted until 21:00:00Z  wheel human` when a pause stood. The time is the override's end, taken from the reply's `override_until`: `HH:MM:SSZ` when it falls on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` on any other date, and a value that is no RFC 3339 instant prints escaped. When no pause stood the line is `builder  budget gate open  wheel human`. Both are one unstyled stdout line with the two-space gutter of the other verb lines. The date form and the `budget gate open` line are the founder's wording: planning forms as built (chunk 2026-10-10-budget-governor, on the operator's word) that he confirmed as they were shown to him, his own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's word superseded 2026-10-13).
### surface#9.1 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 99–101
| Exit | Human stderr first word | `--json` |
|---|---|---|
| 0 | none (TTY context lines only); the result line goes to stdout (`[RB] read back`, `answered`, `wheel human` …) | `{"v":1,"ok":{…}}` |
### surface#9.3 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 103–103
| 2 | clap usage text (never produced by `viola hook`) | — |
### surface#9.7 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 107–107
| 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
### surface#9.10 · values · cli-exit-codes · ## Surface: cli › ### Component Patterns · lines 110–110
| 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / strict-modes / server verification); `viola list`'s refused strict-modes check prints instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer) | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), `list`'s refusal included, with stderr empty |
