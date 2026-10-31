# design-system · Surface: {type}
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Surface: {type} detail.

## Surface: cli › ### Tokens (platform-specific)

<!-- detail: values · cli-tokens -->

| Token | Truecolor (`COLORTERM=truecolor\|24bit`) | 256-colour | 16-colour fallback | Use |
|---|---|---|---|---|
| attention (arrival amber #D97706) | `\x1b[38;2;217;119;6m` | `\x1b[38;5;172m` | `\x1b[33m` | The `DIALOG` word only, always followed by the kind word |
| stale (lamp-off) | `\x1b[2m` (SGR dim) | `\x1b[2m` | `\x1b[2m` | A whole `stale` row. The word `stale` is always in LIVE. On a row that is also cocked, only the `DIALOG` word is amber and not dim (the cock wins for `DIALOG`). The kind word and the rest of the row stay dim. NAME on a stale row is dim and bold, and every span inside the row opens dim again after its reset. |
| callsign | `\x1b[1m` (bold) | `\x1b[1m` | `\x1b[1m` | NAME column values |
| reset | `\x1b[0m` | `\x1b[0m` | `\x1b[0m` | After every styled span |
| handoff blue #5B8DB8 | not used | not used | not used | The quiz limits CLI colour to amber and dim, so link output (`overseer -> builder`) is plain at every depth. No index is reserved: any blue SGR would break the ban on colouring anything but `DIALOG` and `stale`. |
| backgrounds | never set | never set | never set | The terminal background belongs to the user |

<!-- /detail -->

<!-- detail: values · cli-output -->

- stderr: context lines (`waiting: <name>`, the issue line of a send), refusals (`unable  <name>  <reason>  <detail>`, two-space separated; `viola send` prefixes the `[/ ]` mirror and pads `unable` to the readback-word column, as in its sample; the exit-code table omits the `<name>` field; only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal (chunk 2026-10-11-the-board-viola-list, the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer superseded 2026-10-13), and the exit-21 strict-modes refusal `viola link` / `viola unlink` make of the driven instance, and the one `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance, use the fixed message form, `unable: <text>`, as in `unable: <name> is already live`), each refusal's `hint:` line directly after it, and errors (`error: …`).
- Code word: 1 to 64 bytes of lower-case letters, digits and hyphens, a lower-case letter first (`viola_core::is_code_word`); both clients, `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`), screen a peer's `refusal` and `detail` through it.
- A peer's `refusal` that is no code word reads `unknown`; a string `detail` that is no code word reads as no detail, null under `--json`; a non-string `detail` is no known reply.
- A code word outside the closed refusal list stays visible, by design, in the document and in `refused: <word>`, and exits 14 as `unknown` does (`reason_of`).

<!-- /detail -->

<!-- detail: values · cli-output -->

- Where the width is detectable, human tables truncate only the NAME of unwrapped rows (`...`). The width is stdout's own terminal size, read by `viola_pty::host_size()` (Windows `GetConsoleScreenBufferInfo`, a `TIOCGWINSZ` ioctl elsewhere); `COLUMNS` is not read, and a piped stdout has no width (chunk 2026-10-17-cli-output-discipline). The CLI has no tape.

<!-- /detail -->

## Surface: cli › ### Component Patterns

<!-- detail: values · cli-list -->

- **As built** (chunk 2026-10-11-the-board-viola-list; the agents read, chunk 2026-10-16-unwrapped-sessions-on-the-board; the colour layer and the width cut, chunk 2026-10-17-cli-output-discipline). A wrapped row joined to a `claude agents --json` row (by child pid or agent session id) reads STATUS `idle` or `busy`, else `unknown`. When the read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line follows the wrapped rows (or the empty-board line when there are none), then one row per unwrapped session sorted by name: NAME the row's `sessionId`, LIVE `live`, its STATUS, `n/a` in WHEEL, DIALOG and CLI; with no unwrapped row no separator prints, and a failed, oversize or malformed read prints the board as with no `claude`. On a terminal whose width is known (`viola_pty::host_size()`) NAME's column is the largest of the caption's 4, the longest wrapped NAME and the longest unwrapped NAME capped at the width less the 65 columns the rest of an unwrapped row takes; an unwrapped NAME longer than the column is cut to the column less 3 characters and ends in `...` (the sample's `c1f9e2a4...`), the cut falling before an escape it would split; a wrapped NAME is never cut, and a piped board or `--json` keeps every name whole. On a terminal that takes colour (the decision order above) every NAME value is bold; the `DIALOG` word of a wrapped row with a pending dialog takes the attention token, its kind word plain; a wrapped `stale` row is dim from its first byte to its last, NAME dim and bold, and on a stale row with a dialog only `DIALOG` is amber and not dim; the BAY line, the caption, the empty-board line and the separator carry no SGR; the spans are laid over the escaped, padded cells, so the columns are the plain board's, and the board stays one write.
 The row mode escapes every control character of a row, DEL and C1 included, as `\xHH` (`escape_row`); as built its upstream-derived cells are the recorded CLI version and each unwrapped NAME, and NAME's width is counted in characters. WHEEL is padded to 23, the widest word `unknown - budget-paused` (a replayed row with a logged pause and no logged holder): implement's decision, confirmed by the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's acceptance superseded 2026-10-13). Five forms are the founder's wording: planning forms on the operator's word that he confirmed as built in the same answer (superseded 2026-10-13): a window's figure rounded down to a whole number (`89.6` → `89 %`); the reading's age in whole minutes, rounded down at every age (`read 0m ago`, `read 75m ago`), and `read unknown` for no instant, one that does not parse, or one ahead of now; nonzero skip counts as bare numbers in key order (`skipped 2 - 0 - 1`); a `human` holder under a budget pause prints `human - budget-paused`; the empty board prints the BAY line and the caption row above its one line.
- **Refusal** (exit 21; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer; the operator's answer at that chunk's planning, superseded 2026-10-13): a failed strict-modes check of the home, `instances/` or any one instance refuses the whole board, nothing from state printed, in the fixed `unable:` / `hint:` form of the start refusals (pattern 2): the home → `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions` (`list` is this pair's first writer in source); an instance → `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` (`revive`'s pair, shared; `viola link` / `viola unlink` print it, naming the driven instance, at their exit-21 strict-modes refusal; `viola send`, `wait`, `last`, `answer`, `pause` and `release` print it, naming their instance, at theirs; `viola run`'s and `viola revive`'s start print it at an instance-level refusal of their home-tree check, exit 1).

<!-- /detail -->

<!-- detail: values · cli-list -->

- The board's strict-modes scan holds the home to two rules: it checks the home by the write rule, then each instance's set, which holds the home to the read rule. So a home that group or other may read but not write passes as the empty board when no instance exists, and with an instance refuses the whole board with the instance-state pair naming that instance: which pair the refusal prints depends on whether an instance exists. Pending, owner: a CARRY on the working-route entry «Unix endpoint and home hardening».

<!-- /detail -->

<!-- detail: values · cli-send -->

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
    - the home or the instance's state files fail the strict-modes check (`send` after reading its text; `wait`, `last`, `answer`, `pause` and `release` first; each before any frame): no `unable  instance-unreachable` line and no hint of this list, but the fixed instance-state pair `unable: <name>'s state files can be written by another user` / `hint: viola will not read them; make the viola home and its files owner-only` (pattern 1, Refusal), not the `[/ ]` mirror; under `--json` the `instance-unreachable` document
    - server verification fails: `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted`
  - The exit-1 start refusals of `viola run` have one fixed-message line and one hint per cause (T4). None of them shows a path or a pid:
    - `unable: <name> is already live` → `hint: viola list`
    - `unable: <name> is still running but not answering` (stale heartbeat, live pid) → `hint: viola list shows it as stale; stop that process before starting <name> again`
    - `unable: the endpoint for <name> is held by another process` (squatted name) → `hint: another process holds this name's endpoint; stop it or pick another name`
    - `unable: the pinned viola copy failed its integrity check` (SHA-256 mismatch) → `hint: the pinned copy was changed after it was written, so viola will not run it`
    - `unable: <name>'s command is a .cmd or .bat script` (Windows) → `hint: pass the real executable, not a .cmd or .bat shim`
    - `unable: the viola home is not private to you` (a home-level refusal of the home-tree check made at start before any log, of the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/`, each that exists; no child) → `hint: the viola home must be readable only by you; viola does not change its permissions`; an instance-level refusal of the same check prints `unable: <name>'s state files can be written by another user` → `hint: viola will not read them; make the viola home and its files owner-only`
  - The exit-1 refusals of `viola revive` take the same form, one fixed-message line and one hint per cause, read in a fixed order that stops at the first refusal. None of them shows the recorded directory, a pid or a logged id, and no hint names `viola release`:
    - `unable: <name>'s state files can be written by another user` (strict-modes on the instance's files) → `hint: viola will not read them; make the viola home and its files owner-only`
    - `unable: <name> is already live` and its stale sibling: `run`'s two lines and hints above, unchanged
    - `unable: <name> has no logged session to resume` → `hint: start one: viola run <name> -- claude`
    - `unable: <name> has no logged session with that id` (an `--id` the log does not hold) → `hint: viola revive <name> --list`
    - `unable: <name>'s session is open in another claude` (one `claude agents --json` read lists the logged id live in another `claude`; skipped under `--fork`; a failed read passes) → `hint: close it there first, or start a copy: viola revive <name> --fork` (the founder's wording, his own answer to P4 fork 4 of chunk 2026-10-16-unwrapped-sessions-on-the-board, `inputs#I1`, relayed verbatim by the operator)
    - `unable: <name>'s recorded directory is missing` → `hint: resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`
    - the home-tree check of its start, after a passed preflight and before any log (no child): a home-level refusal → `unable: the viola home is not private to you` → `hint: the viola home must be readable only by you; viola does not change its permissions`; an instance-level refusal → the instance pair above; under `--json` the `strict-modes-failed` document
  - Hints are human-mode only. Under `--json`, no hint text is printed; the machine view carries the cause as a detail code, never as prose. **Pending an arch amendment:** arch currently fixes `{"v":1,"error":"instance-unreachable","detail":null}` for exit 21, and for exit 1 only `viola revive`'s `{"v":1,"error":"<code>","detail":null}` (its strict-modes and preflight refusals; chunk 2026-10-17-cli-machine-contract). Until arch names the `detail` codes, `--json` keeps arch's shape. obs-plan D-20 already names the log detail codes: `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`, and for `viola revive` `no-session` and `cwd-missing` (its strict-modes refusal writes no log line since chunk 2026-10-10-budget-governor, so it carries no logged detail code: the stderr pair, or under `--json` the `strict-modes-failed` document, and exit 1). Of them, `server-verify-failed` is written to the log by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` at their exit-21 server-verification refusal, whose `--json` document stays `detail:null`, as the not-running cause's does. The stale-heartbeat and unwrapped-name causes have no code yet.
  - No hint line for `unknown` (exit 14), because its detail is a code word or nothing and a hint would have to guess. No hint line for `error: wrapper fault` (exit 20) or `error: internal error` (exit 1), because they are faults, not refusals: the human line holds the code only, further detail goes to `diagnostics/`, and the `--json` document's `detail` is row 20's closed object.

<!-- /detail -->

<!-- detail: values · cli-send -->

- The server-verification hint of `instance-unreachable` is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, the eight verbs that write a frame, as the hint cause `server-verify` beside `not-running`: it prints the server-verification line above byte for byte, plain ASCII with no SGR, and it holds no pid, start time, path or endpoint.

<!-- /detail -->

<!-- detail: values · cli-send -->

- On macOS no peer pid is available, so the server check passes with no comparison and the server-verification hint cannot print there (a recorded gap; owner: the route entry "Unix endpoint and home hardening").

<!-- /detail -->

<!-- detail: values · cli-verbs -->

- `viola release builder --budget` → `builder  budget gate lifted until 21:00:00Z  wheel human` when a pause stood. The time is the override's end, taken from the reply's `override_until`: `HH:MM:SSZ` when it falls on the current UTC date, `YYYY-MM-DD HH:MM:SSZ` on any other date, and a value that is no RFC 3339 instant prints escaped. When no pause stood the line is `builder  budget gate open  wheel human`. Both are one unstyled stdout line with the two-space gutter of the other verb lines. The date form and the `budget gate open` line are the founder's wording: planning forms as built (chunk 2026-10-10-budget-governor, on the operator's word) that he confirmed as they were shown to him, his own answer, 2026-10-13, relayed verbatim by the Viola overseer (the operator's word superseded 2026-10-13).

<!-- /detail -->

<!-- detail: values · cli-verbs -->

- `viola link` / `viola unlink` print their one result line on stdout in one write, plain ASCII with no SGR whatever the terminal, and exit 0; the names printed are the verb's own parsed arguments, never names read back from the reply or the log.
- The `since` of the `link` line is the reply's `since` as UTC clock time in whole seconds with `Z` (`HH:MM:SSZ`); a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through the message-mode escaper of pattern 3.
- A second `viola link` of a held pair prints the `already linked` form, the wrapper keeping the first `since`; `viola unlink` of a pair that is not held prints the same `unlink  <driver> -> <driven>` line; both exit 0.

<!-- /detail -->

<!-- detail: values · cli-verify-run -->

- **`verify`:** a step counter (the project's one progress pattern) with static appended stdout lines, one per relied ledger row: `[01/24] shim-resolution claude resolves to a real executable  pass` / `... fail` (`MM` is the relied count, `LedgerRow::relied()`: 24 on Unix, 21 on Windows, where the three statusline rows are not relied because the status line is not wrapped there; the members of `LedgerRow::ALL` in its order, after the six spine and four screen and timing rows the dialog rows, the framing rows, the newline row, on Unix the statusline rows, then the resume row, the agents row and the compact row, all three relied on every OS: `question-answer` a question answered through PreToolUse takes effect · `plan-approve-revise` a plan revise and approve each take effect · `question-notes` free text and notes reach the question · `dialog-concurrency` two parallel questions each raise a dialog · `long-paste-wrapper` a long paste unwraps to the text as pasted · `tag-escaping` tag-like text un-escapes to the text as pasted · `local-command-clear` /clear starts a new session and submits no prompt · `paste-newline` an LF inside a paste stays inside the one prompt submitted · `statusline-override` a --settings statusLine runs in the status line's place · `statusline-shell` a statusline command runs through /bin/sh · `statusline-rate-limits` the statusline payload carries two whole-number rate_limits windows · `resume-session` --resume <id> reopens the session under that id · `agents-join` claude agents --json names the live session by pid and session id · `local-command-compact` /compact keeps the session id and submits no prompt; the count grows as owning chunks land their rows). Then the last stdout line, a summary, n + m equal to `MM`: `stamped 2.1.287  24 pass  0 fail` on Unix. The word `fail` is uncoloured.
 `viola verify --help` carries one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and an unapproved external CLAUDE.md import blocks the probe. A run stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary: stderr carries exactly `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI` (pattern 2's fixed form, plain ASCII, no path or pid), and it exits 1.

<!-- /detail -->

<!-- detail: values · cli-verify-run -->

- Before its log, `verify` checks its home tree (the home, `ledger/`, `bin/`, `plugin/`, each that exists): a refusal prints no step line and no summary, only the home-state pair `unable: the viola home is not private to you` then `hint: the viola home must be readable only by you; viola does not change its permissions` (pattern 2's fixed form), and exits 1; a `stamps.json` update refused by its strict-modes check prints the same pair.

<!-- /detail -->

<!-- detail: values · cli-verify-run -->

- On Unix a home under a regular file cannot be read and is a strict-modes refusal, not the catch-site line: `verify` prints the home-state pair (exit 1) and the frame verbs the instance-state pair (exit 21). On Windows such a path stats `NotFound`, is passed as absent, and keeps the catch-site line.

<!-- /detail -->

<!-- detail: values · cli-verify-run -->

- **`revive`:** prints nothing once the child starts, as `run`. `viola revive builder --list` prints one static stdout line per logged session, in log order, `<time>  <cause>  <session id>` with two spaces between fields: no header, no colour, plain ASCII. A refusal (exit 1) prints its cause's fixed-message line and then that cause's hint, listed under the send hints above. Under `--json` (both arms; chunk 2026-10-17-cli-machine-contract) a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`) and no hint, stderr empty, and `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}`; a refusal `run`'s start makes after a passed preflight prints no document as landed (owed to the working-route entry "Exit-cause code catalogue").

<!-- /detail -->

<!-- detail: values · cli-exit-codes -->

| Exit | Human stderr first word | `--json` |
|---|---|---|
| 0 | none (TTY context lines only); the result line goes to stdout (`[RB] read back`, `answered`, `wheel human` …) | `{"v":1,"ok":{…}}` |
| 1 | `error: internal error` (fixed message; no chain with serde sources, no paths, no hint), or a `viola run` start refusal `unable: <name> is already live` / `… is still running but not answering` / `the endpoint for <name> is held by another process` / `the pinned viola copy failed its integrity check` / `<name>'s command is a .cmd or .bat script` / `the viola home is not private to you` / `<name>'s state files can be written by another user`, or a `viola revive` refusal `unable: <name>'s state files can be written by another user` / `<name> has no logged session to resume` / `<name> has no logged session with that id` / `<name>'s session is open in another claude` / `<name>'s recorded directory is missing` / `the viola home is not private to you` (or `run`'s `already live` pair), or a `viola verify` refusal `unable: the claude CLI was not found` / `the claude CLI is a .cmd or .bat script` / `the CLI version could not be read` / `the viola home is not private to you` (its home tree refused before its log, no step line and no summary, or a `stamps.json` update refused by its strict-modes check) / `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; codes `home-path` · `absolute-path` · `username` · `email`; never the content), each followed by its own hint, or a `viola verify` stopped by Ctrl-C or TERM before its rows are checked, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` (stdout empty); a `verify` run with a failing ledger row prints no stderr word (its last stdout line reads `stamped <ver>  <n> pass  <m> fail`) | `viola revive --json`'s refusals of its strict-modes check and its preflight: `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; arch §Standard Contracts); every other exit 1 has no `--json` document (a `run` start refusal reached through `revive` included, owed to "Exit-cause code catalogue") |
| 2 | clap usage text (never produced by `viola hook`), or the fixed `UNUSABLE_RESPONSE` line (an oversize `viola answer`) or `UNUSABLE_TEXT` line (an unusable `viola send`) on stderr, whatever `--json` says; `tests/contract_error_bodies.rs` names them in `NOT_ERROR_BODIES` | no document |
| 10 | `unable  human-typing` (+ `manual-pause`) | `{"v":1,"refusal":"human-typing","detail":…}` |
| 11 | `unable  budget-paused  five-hour` / `seven-day` | refusal object |
| 12 | `unable  unverified-cli` | refusal object |
| 13 | `unable  not-delivered  input-not-ready` / `no-prompt-submitted` / `turn-running` / `unknown-dialog` / `control-character` / `empty-text` | refusal object |
| 14 | `unable  unknown  <detail>`, the detail a code word or nothing | refusal object |
| 20 | `error: wrapper fault  <code>` | `{"v":1,"error":"wrapper-fault","detail":{…}}` |
| 21 | `unable  instance-unreachable` (no detail; the name is the omitted `<name>` field, as in every row), then the cause's hint (not running / unwrapped / server verification); `viola list`'s refused strict-modes check, and `viola link` / `viola unlink`'s of the driven instance, print instead the fixed home-state or instance-state `unable:` / `hint:` pair (pattern 1, Refusal; the founder's own answer, 2026-10-13, relayed verbatim by the Viola overseer), and so does the instance check `viola send`, `wait`, `last`, `answer`, `pause` and `release` make of their instance before any frame, printing the instance-state pair naming it, with no frame and no line written | `{"v":1,"error":"instance-unreachable","detail":null}` (a per-cause `detail` code awaits an arch amendment), the `list`, `link` and `unlink` refusals and the six frame verbs' strict-modes refusals included, with stderr empty |

<!-- /detail -->

<!-- detail: values · cli-exit-codes -->

- Exit 0 under `--json`: `viola link`'s document is `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}`, `<ts>` the time the wrapper recorded the link, and `viola unlink`'s is `{"v":1,"ok":{}}`.
- Exit 20 is also reached by `viola link` / `viola unlink` on a pair the wrapper refuses (`-32602`; a `driver` equal to `driven`, which the CLI never refuses itself): `error: wrapper fault  <code>`, or under `--json` the `wrapper-fault` document.
- Exit 20's `--json` `detail` is closed for the five known codes (`viola_channel::sanitised_fault`): `message` is each code's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with null `data`; every other known code has null `data`.
- A code outside the five prints the same human line, `error: wrapper fault  <code>`, and under `--json` the document `{"v":1,"error":"wrapper-fault","detail":{"code":<code>,"message":"wrapper fault","data":null}}`; MCP carries the same fixed `message`.
- Each CLI `--json` failure `tests/cli_error_bodies.rs` drives prints one document, exits its literal code, leaves stderr empty, and holds no ESC, no path and no upstream text.

<!-- /detail -->

<!-- detail: values · cli-exit-codes -->

- Exit 21's server-verification cause is built for `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`: a process on the endpoint that is not the recorded wrapper (its pid or start time differs, or the check cannot be made) gets no frame, and the verb prints `unable  <name>  instance-unreachable` with the server-verification hint last on stderr, or under `--json` the document `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty.

<!-- /detail -->

## Surface: cli › ### Navigation Pattern

<!-- detail: values · cli-help -->

- `viola --help` groups verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install` (`revive` beside `run`: the operator's answer at the chunk 2026-10-10-viola-revive wrap). As built (chunk 2026-10-17-cli-output-discipline) it lists the served verbs only: `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.

<!-- /detail -->
