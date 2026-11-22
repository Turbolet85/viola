@history_item n=1 marker=2026-10-01-t12-19-55-wrap
**2026-10-01-t12-19-55-wrap** — background only: the founder's 2026-09-30 ruling that 0.2.0 renders the session hierarchy as a node graph drove the web-spa toolkit change to React. The 2026-11-08 census later superseded this. It is the first precedent for drawing a parent/child hierarchy, which this chunk now draws as fleet trees inside the CLI board.

@history_item n=2 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa and the web rejected defaults, among them the node-graph canvas. A tree drawing must not bring back a canvas or graph-style rendering. The hierarchy belongs to the CLI board's fixed columns.

@history_item n=3 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0, brand, surface and bans scoped to the CLI) — no web or graphical surface remains. The design direction is the one the CLI carries and the terminal fleet console (Epoch 6, matrix#v2-14) inherits. The console gets its own surface at «Console scaffold», and the spinner, progress-bar and live-redraw ban covers every verb's output. So `list`'s tree must stay a static verb output, and the console is out of scope here.

@history_item n=4 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without `color`, so every styled byte of human output comes from viola's own SGR module and nothing forces SGR into a pipe. This is the basis of «plain when piped» for the tree drawing.

@history_item n=5 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — defines the message-mode escaper: every control character except `\n` and `\t` prints as `\xHH`, and `--json` stays serde-escaped. `link` later reused it for a non-RFC 3339 `since`. It is the precedent for showing untrusted link and parent names safely.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — sets `revive`'s refusals and their fixed order, its `--list` lines, its silence once the child starts, and its place in `setup:`. This chunk makes `revive` keep the parent its lineage recorded, so any new refusal or output must fit this order and the rule against showing paths, pids or logged ids.

@history_item n=7 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: six columns, row-mode `\xHH` escaping (`escape_row`), WHEEL padded to 23, the empty board's three lines, and the whole-board exit-21 refusal with its fixed pairs. Fleet trees have to be drawn inside this column set and these escaping rules.

@history_item n=8 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ratified the board's forms, WHEEL 23 and the list refusal pairs as his own wording. Changing any of them for the tree, such as column widths or the empty-board lines, needs the founder's word.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the unwrapped rack: the `-- UNWRAPPED - READ-ONLY --` separator, NAME = `sessionId`, `n/a` cells, STATUS joined from `claude agents --json`, and a failed agents read printed as if `claude` were absent. It also added `revive`'s session-live pair. Trees must say where unwrapped and bare strips sit, since they are never a target and never cocked.

@history_item n=10 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive --json` documents and `--list --json` `{"v":1,"ok":{"sessions":[…]}}`. This is the payload precedent if a revived parent, or what `list --json` gains, shows up in a machine document.

@history_item n=11 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer (bold NAME, amber DIALOG, dim stale rows, spans laid over escaped padded cells, one write) and the width cut. NAME's column is capped at the width less 65, and only unwrapped NAMEs are cut with `...`. Pipes and `--json` stay whole. Any tree indent or prefix in NAME changes this width arithmetic and the truncation rule.

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the width source (`viola_pty::host_size()`, `COLUMNS` never read) and `--help` listing only served verbs. A new `run` flag for declaring a parent shows up in help under these rules.

@history_item n=13 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the cocked strip keeps its slot and shows `DIALOG` plus the kind, amber on the CLI. Its translate and band motion were moved out. Tree layout must not move a cocked strip either (decision/dialog-pending-cocked-strip, stable slots).

@history_item n=14 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — the Decisions Log moved into the body. This is where the squint-test exception (`open` and `unconfirmable` look identical) comes from. The self-validation rows the tree drawing must pass trace back to this entry.

@history_item n=15 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s result lines as built) — one plain stdout write, `link    <a> -> <b>  since <time>` / `already linked` / `unlink  <a> -> <b>`. The names printed are the verb's own parsed arguments, never names read back from the reply or the log. A non-RFC 3339 `since` goes through the message-mode escaper. This chunk shows observed links apart from parent edges and name-checks them, which extends that rule.

@history_item n=16 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`--json` documents and exit 20) — `link` returns `{"v":1,"ok":{"since":…,"already_linked":…}}` and `unlink` returns `{"v":1,"ok":{}}`. The wrapper alone refuses `driver == driven` with -32602 and exit 20. Links are wrapper-held state, apart from a lineage parent, which is a label only.

@history_item n=17 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (strict-modes refusal and the `handoff:` help row) — `link` / `unlink` print the instance-state pair at exit 21 and `--help` gains `handoff: link, unlink`. Also in this group: the server-verification cause covers eight frame verbs. These are the verbs whose observed links the board will now show.

@history_item n=18 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the six frame verbs print the instance-state pair. `run` and `revive` check their home tree before any log, at the start where the lineage line would be written. The board's pair depended on whether an instance existed, which was later fixed. A lineage write at start comes after this check, and the check's order and no-log rule stand.

@history_item n=19 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the code-word screen (`viola_core::is_code_word`) on a peer's `refusal` and `detail`. Exit 2's fixed stderr lines are the exception to one `--json` document. Every `--json` failure is one document with no ESC, path or upstream text (tests/cli_error_bodies.rs). This screening is the precedent for showing a link or parent name only after it parses as a viola name.

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — exit 21's document names its cause. A stale holder logs its own code at start. `revive --json` has a closed list of exit-1 codes, and a `pump_child` error prints no document. The catch site prints `internal-error`, and `run`, `revive`'s start, `mcp` and `hook` stay silent. A frame verb with no live endpoint reads its instance directory to tell `unwrapped-name` from `instance-dead`. New start or list refusals from lineage would extend these closed sets, which are founder forks.

@history_item n=21 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder ratified the unwrapped-name reading by instance directory and the `endpoint-lost` wording. The name-check of link names builds on the unwrapped and wrapped distinction made this way.

@history_item n=22 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the board's scan holds the home to the read rule and `instances/` to the write rule, and a readable home refuses the whole board with the home-state pair. It added the socket-directory pair at `run` and `revive` start, `socket-dir-refused` as exit 21's sixth cause and a log code, and `revive --json`'s socket-dir code. A tree read of every instance's lineage must keep to this scan's order.

@history_item n=23 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — the stale row's colour layer is witnessed end to end (DIM, BOLD, RESET, DIM, RESET, with no SGR on BAY, caption or separator) by tests/cross_list_rows.rs. Tree glyphs on stale rows have to keep this byte sequence, or the witness has to change.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — width comes only from `TIOCGWINSZ`. The strict-modes readings lost their OS arm. `revive`'s codes went back to ten when `batch-script-child` was retired. The stale-row witness now runs on `ubuntu-latest` only. These are the current baselines for the board and `revive` that this chunk extends.

@history_item n=25 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — the founder confirmed the socket-directory pair's wording and closed its OPEN mark. This settles a `run` and `revive` start refusal the lineage write path sits behind.

@history_item n=26 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — changed `contract/exit-21-json-cause/unwrapped-name`: a symlinked instance directory is refused by strict-modes, not read as unwrapped. This affects how a linked or parent name that is not a wrapped instance is classified.

@history_item n=27 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity) — changed `decision/fixed-phraseology-vocabulary` and dropped `budget-paused`. Any new word for parent or lineage on the board joins this closed vocabulary as a fork.

@history_item n=28 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Iconography) — changed `contract/cli-separators/space-hyphen-space`, whose example is now `skipped 2 - 0 - 1`. Separators inside a tree cell follow this ASCII rule.

@history_item n=29 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli) — changed `contract/viola-list-table/bay-line` (skip counts only, no TAPE), `contract/cli-exit-codes/10-14` and `contract/cli-exit-codes/11` (11 retired, never reused), `decision/wheel-column-width-23` (the width kept without its widest word) and `decision/no-hint-names-viola-release`. It removed the board's budget cells. The board this chunk extends has exactly the BAY line and the six columns these rows state.

@history_item n=30 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Anti-Patterns) — removed `decision/budget-as-ruled-figures`, so the anti-pattern set the tree drawing must respect no longer holds that rule.

@history_item n=31 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — added `decision/api-error-turn-end-shown-as-turn-end`: STATUS reads `idle` with no special mark. A tree must not add a status mark for this case.

@history_item n=32 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — changed `decision/api-error-turn-end-shown-as-turn-end` to one closed cause word of nine in `--json` and MCP `wait` data. The founder ratified it at 2026-11-17T05-21-35Z-wrap.

@history_item n=33 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — changed the state of `decision/api-error-turn-end-shown-as-turn-end` to standing and ratified the delegate's confirm. The STATUS rule the trees inherit stands on the founder's word.

@history_item n=34 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — added the `contract/auto-verify-line/*` rows, the fixed stderr lines `run` and `revive` print at start before the child, silent under `--json`. This is the latest precedent for new start-time output beside `decision/run-prints-nothing-while-wrapped`. A parent declaration at `run` must fit between these lines and the child's silence.

@history_item n=35 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (auto-verify lines ratified) — the founder ratified the six `contract/auto-verify-line/*` rows and ended every PROVISIONAL mark of Epoch 4. This chunk opens Epoch 5 with no provisional design rows carried over, so its own new words go to the founder as fresh forks.
