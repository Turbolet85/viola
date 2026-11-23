@task_prose part=chunk
Adds the cli verb `viola open`: a closed flag allowlist, a foot terminal started directly with no shell, a wrapper that places its own window beside its parent's with one move dispatch, and a return once the new instance is live.
The started `viola run` takes two hidden checked flags (origin, target workspace); the wrapper records its window as a new log-only `window` event and a snapshot `window?`, and `list --json` and MCP `list` items gain `window?` / `origin?`.
Adds tests for the test-only recorder's three untested `record-move` failure codes, and makes the compositor read bound (5 s) a time-bound row of its own.
Adds a test-only one-command script for the single live reading R2, which gates the launch capability; the chunk stops before R2.
@task_file n=1 path=src/cmd/open.rs
what: the new `viola open` verb — its arguments, its refusals and exits, its terminal spawn, its liveness wait and its --json document
@task_file n=2 path=Cargo.toml mark=edits
what: the root package manifest — edits: the compositor seam dependency, moved from a dev-dependency to a product dependency
@task_file n=3 path=crates/viola-e2e/tests/hypr_tap.rs mark=edits
what: the end-to-end tests of the test-only compositor recorder over planted stubs — edits: tests for the `record-move` failure codes move-refused, move-unseen and window-left
@task_file n=4 path=scripts/r2-open.sh
what: the new test-only script for the one live reading R2 — starts a parent in its own window, runs one `viola open … -- claude`, reads the verdict and closes both windows on every exit path
@task_item n=1 part=needs
The new verb `viola open <name>` with flags `--parent`, `--role overseer|builder|worker`, `--cwd`, `--workspace N`, `--model` / `--effort`, `--json`, `--home`, and after `--` only the word `claude` with no claude flag; parsed by clap derive, a flat verb taking a wrapped ViolaName target.
@task_item n=2 part=needs
Refusal of a word outside the allowlist: exit 2 as clap's usage error, plus one fixed line for a word after `--` other than `claude`, standing beside the run/revive usage line for a launch pair after the dashes.
@task_item n=3 part=needs
New exits and refusal words of open: exit 1 `terminal-not-found`, exit 1 `not-live` (the window left standing), `already-live` and `stale-heartbeat` read before any spawn, exit 1 `strict-modes-failed` before any log line, and no exit 11; each refusal in the shared refusal line form with its hint.
@task_item n=4 part=needs
The open --json documents: `{"v":1,"ok":{"name","parent","window","placement"}}` on success and `{"v":1,"error":"<code>","detail":null}` on refusal, one document per call.
@task_item n=5 part=needs
The `viola --help` verb group row becomes `setup:  open, run, revive, verify, plugin install`, and open gets its per-verb usage text.
@task_item n=6 part=needs
`viola run` gains two hidden checked flags carrying the origin (`cli` or `agent`) and the target workspace that open hands it; they stay out of the visible run usage and help.
@task_item n=7 part=needs
The `viola list --json` document and the MCP `list` tool items gain optional `window?` and `origin?` fields, while the six-column board keeps its columns.
@task_item n=8 part=needs
A new log-only event kind `window {address?, workspace?, placement, origin}` with its validators, and a snapshot field `window?` replayed with the last line winning; written by the wrapper alone and logged codes-only.
@task_item n=9 part=needs
Placement values recorded by the wrapper: moved into the target workspace, `unplaced` (no parent window, or no window within 5 s), and the compositor seam's `unsupported` / `unreachable` when off Hyprland, where the terminal still starts; no focus change, and open proceeds on a locked desktop.
@task_item n=10 part=needs
Time bounds: open waits at most 30 s for the instance to be live, the window's appearance gets 5 s polled every 100 ms, and the compositor read bound HYPRCTL_DEADLINE (5 s) joins the time-bound rows that fall on the driver and never on a human, each bound as a row of its own.
@task_item n=11 part=needs
The terminal is started directly with no shell line built from the arguments whatever characters they hold, argv `foot --app-id=viola.<name> --title=<name> --working-directory=<cwd> <pinned viola> --home <home> run <name> …`; the launch authority stays bounded to the allowlist.
@task_item n=12 part=needs
The parent defaults to the caller's `VIOLA_NAME` when an agent calls open from its own shell; the parent's window is its recorded one, else an ancestry walk of at most 8 steps held in memory.
@task_item n=13 part=needs
The global `--home` flag is carried through open onto the started run's argv.
@task_item n=14 part=needs
Indirect, through the cli output rules: while open waits for liveness it prints results on stdout and the rest on stderr, with no spinner, progress glyph, prompt or pager, and plain output when not on a TTY.
@task_item n=15 part=needs
Indirect, through the hint rules: each new refusal of open has a hint as the last stderr line, naming the next verb and never quoting text or naming a path.
@task_item n=16 part=needs
Tests for the recorder's `record-move` failure codes move-refused, move-unseen and window-left, each reached over a stub variant with no live window.
@task_item n=17 part=needs
The live reading R2 as one test-only command under a scratch home with a private config dir, capped at one live claude start, preceded by a no-start probe; R2 gates the launch capability and is not run in this phase.
@task_standing n=1
convention: each new refusal of a verb takes the shared refusal line form and word column, with a hint as the last stderr line that names the next verb
home: surface-cli
@task_standing n=2
convention: each new data verb carries a --json mode writing one typed document, with its error codes in the exit-code --json form
home: surface-cli
@task_standing n=3
convention: each new verb is listed in a --help verb group row and has its own per-verb usage
home: surface-cli
@task_item n=1 part=not_needed
The web-spa surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
The send, wait, last and answer output rows, the readback mirror and its vocabulary: open sends no prompt text and reads no turn.
@task_item n=3 part=not_needed
The wheel and handoff verb outputs (pause, release) and the link / unlink outputs: open neither changes the wheel nor links instances.
@task_item n=4 part=not_needed
The revive-only rows: its --list document, rows and cause field, malformed --id, session-with-id and session-open-elsewhere refusals.
@task_item n=5 part=not_needed
The viola verify output and refusals, and the plugin install output: open does not record or verify a fixture nor install a plugin.
@task_item n=6 part=not_needed
The CLI token rendering rows (colour, spacing, radius, motion, callsign tokens): open adds no styled output.
@task_item n=7 part=not_needed
The cursor byte-offset row and the deep-link absence: open reads no message cursor and emits no link.
