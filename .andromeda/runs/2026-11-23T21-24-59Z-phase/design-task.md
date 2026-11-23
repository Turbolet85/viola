@task_prose part=chunk
Adds the flat verb `viola open`: one terminal window (foot, started directly, no shell) per session beside its parent, a closed flag allowlist with only the word `claude` after `--`, placement by the compositor seam's one move dispatch, waiting for liveness, with its own exits, refusal words and a `--json` document.
The wrapper (`viola run`) gains two hidden checked flags (origin `cli`/`agent` and target workspace), finds and records its own window, and logs a new codes-only `window` kind replayed into the snapshot; `list --json` and MCP `list` items gain `window?` and `origin?` while the six-column board stays unchanged.
Adds tests for the hypr tap recorder's three untested `record-move` failure codes over the existing stubs, a stand-in foot and a pid-keyed `clients` reply in the fake agent, and a test-only one-command live script for R2.
Adds the 5 s compositor read bound and the new open bounds (30 s liveness, 5 s window appearance polled at 100 ms) as time-bound rows of their own; the live reading R2 is the stop, not run in the build.
@task_file n=1 path=src/cmd/open.rs
what: the new `viola open` verb: its flag allowlist, terminal spawn without a shell, target workspace computation, liveness wait, refusals, exit codes and --json document
@task_file n=2 path=src/cmd/mod.rs mark=edits
what: the flat CLI verb set and the viola --help verb groups — edits: the verb list (open joins), the help line `setup:  open, run, revive, verify, plugin install`
@task_file n=3 path=src/cmd/run.rs mark=edits
what: the viola run verb and its start refusals — edits: two hidden checked flags (origin, target workspace) on the started run's argv
@task_file n=4 path=src/run/mod.rs mark=edits
what: the wrapper's lifecycle — edits: the window record (find its own window by its terminal ancestor's pid, one move dispatch, placement and its reason)
@task_file n=5 path=src/human/verbs.rs mark=edits
what: human verb output and refusal lines — edits: open's refusal lines, the fixed line for a word after `--` other than `claude` beside PAIR_AFTER_DASHES
@task_file n=6 path=crates/viola-core/src/lib.rs mark=edits
what: the event log kinds and their validators — edits: the new log-only kind `window {address?, workspace?, placement, origin}`
@task_file n=7 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot — edits: the optional `window?` field
@task_file n=8 path=crates/viola-state/src/replay.rs mark=edits
what: the log replay into the snapshot — edits: the `window` kind, last line winning
@task_file n=9 path=crates/viola-state/src/board.rs mark=edits
what: the board behind viola list and list --json — edits: `window?` and `origin?` on list --json items
@task_file n=10 path=crates/viola-mcp/src/tools.rs mark=edits
what: the MCP tools — edits: `window?` and `origin?` on the MCP `list` items
@task_file n=11 path=Cargo.toml mark=edits
what: the workspace manifest — edits: the compositor seam crate moves from dev-dependencies to a product dependency
@task_file n=12 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake agent's compositor replies — edits: a `clients` reply keyed to the stand-in terminal's pid, the replayed move dispatch
@task_file n=13 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent — edits: a stand-in `foot`
@task_file n=14 path=crates/viola-e2e/tests/hypr_tap.rs mark=edits
what: the hypr tap recorder tests over planted stubs — edits: tests for `record-move` failure codes move-refused, move-unseen, window-left
@task_file n=15 path=scripts/r2-open.sh
what: a new test-only one-command live script: starts a parent over sleep in its own window, runs one viola open, reads the verdict, closes both windows on every exit path
@task_item n=1 part=needs
The new verb `open` joins the flat CLI verb set and the command name register, sitting in the help's setup group as `setup:  open, run, revive, verify, plugin install`.
@task_item n=2 part=needs
Open's exit codes: exit 2 for any flag outside the allowlist (clap's usage error) plus one fixed refusal line for a word after `--` other than `claude`; exit 1 for terminal-not-found, not-live, already-live, stale-heartbeat and strict-modes-failed; never exit 11.
@task_item n=3 part=needs
Open's --json result is one stdout document: `{"v":1,"ok":{"name","parent","window","placement"}}` on success, `{"v":1,"error":"<code>","detail":null}` on failure, clean, with no hint line under --json.
@task_item n=4 part=needs
Open's human refusals take the fixed CLI refusal line form (unable line, hint only in human mode, none for faults or unknown), show no path or pid, no internals and no stack trace, and name no viola release.
@task_item n=5 part=needs
Refusal order: already-live and stale-heartbeat are read before any spawn; strict-modes-failed refuses before any log is written, as the run and revive start refusals and the strict-modes refusal pair do.
@task_item n=6 part=needs
Open waits for liveness up to 30 s and for the window up to 5 s with no spinner, no elapsed-time counter, no interactive prompt and no pager; nothing announces success before the instance is confirmed live.
@task_item n=7 part=needs
A placement outcome (placed, unplaced, unsupported, unreachable) is a code word carried in the record and the --json document; off the compositor the terminal still starts and the record says why; a locked desktop does not stop the open.
@task_item n=8 part=needs
The wrapper records its own window: a new log-only event kind `window` with address, workspace, placement and origin, logged codes-only (detail codes, never free text), and a snapshot `window?` replayed with the last line winning.
@task_item n=9 part=needs
viola run stays silent while wrapped: the hidden origin and target-workspace flags and the window record print nothing to the terminal the child holds, and hidden flags do not appear in help.
@task_item n=10 part=needs
list --json items and MCP `list` items gain optional `window?` and `origin?`; the human list board keeps its six fixed columns, row order and width rules unchanged.
@task_item n=11 part=needs
The verb's argument values reach the terminal's argv and the started run without any shell line and without escaping the CLI's own message rules: a name or cwd holding control characters is escaped wherever a human line shows it.
@task_item n=12 part=needs
The parent defaults to the caller's VIOLA_NAME when an agent calls open from its own shell; the CLI callers include LLM driver sessions, whose output stays plain and uncoloured.
@task_item n=13 part=needs
Open's own output goes data on stdout, messages on stderr, ASCII-only with no glyphs, emoji or check-mark prefixes, with colour only through the hand-written styling module and only as a second cue.
@task_item n=14 part=needs
Unwrapped sessions are never a target: the parent window is found only from a wrapped parent's recorded window, else an ancestry walk of at most 8 steps held in memory.
@task_standing n=1
convention: every new human output or refusal line is ASCII-only, escapes control characters and carries no internals
home: typography · anti-patterns-never-do-these
@task_standing n=2
convention: every new --json document is one stdout document, versioned `v:1`, with ok or error and a code word
home: surface-cli
@task_standing n=3
convention: every new refusal code carries a fixed message, an exit code from the CLI exit-code set and, in human mode only, at most one hint
home: surface · surface-cli
@task_item n=1 part=not_needed
The flight-progress strip, cocked dialog-pending strip, arrival amber and handoff display: open starts a session but shows no strip or dialog state.
@task_item n=2 part=not_needed
The readback box and send readback rows (prompt, /clear, /compact, /context, /mcp) and the send mirror: open sends no key and no prompt.
@task_item n=3 part=not_needed
viola verify output and refusals, and the automatic-verify line of run and revive: open adds no verify step.
@task_item n=4 part=not_needed
viola wait output, viola last output, viola link and unlink output and their --json: not touched by opening a window.
@task_item n=5 part=not_needed
viola revive --list output and revive-only refusals (no-session, cwd-missing, session-live, no such id): revive is not changed.
@task_item n=6 part=not_needed
Wrapper fault --json detail, peer reply screening and the wrapper socket's refused peer: open makes no peer request over the instance socket.
@task_item n=7 part=not_needed
The rendered-page and web surface absences, colour depth tiers and colour tokens for handoff blue and callsign: no page and no new colour.
@task_item n=8 part=not_needed
Self-validation protocol checks (squint, swap, readback-box signature, design from the domain): design-review checks, not rules on this verb's code.
@task_item n=9 part=not_needed
The hook and mcp verbs' own stdout and exit code: open adds nothing to the hook verb or the mcp verb's stream rules.
