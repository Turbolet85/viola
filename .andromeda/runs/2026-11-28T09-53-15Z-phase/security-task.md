@task_prose part=chunk
Builds the view `viola console` draws, past the scaffold's one fixed frame: the home's instances as lineage trees in the board's own order, each session's LIVE, STATUS, WHEEL and DIALOG in the board's words, and a seventh console-only cell `LAST`, the age of the newest event log line.
Keeps it current: the board re-read in-process once a second, a redraw only when the drawn rows change, the `claude agents --json` read at most once every 5 s off the drawing loop.
Every disk- or upstream-sourced cell passes the board's control escaping; each row is cut at the terminal's width ending `...`, never inside an escape, and a fleet past the frame ends in one `... <n> more` line.
The `console-frame` event gains `shown` and `hidden` counts; the skip-count warning is written by the console only when the count changes. No action, key, send or selection is added, and nothing live is fired.
@task_file n=1 path=src/human/console.rs mark=edits
what: the console's one frame writer, a whole redraw from a cleared top-left in plain ASCII with no SGR — edits: the frame it draws, now the view's rows, the six board captions and `LAST`, cut at the width
@task_file n=2 path=src/console/mod.rs mark=edits
what: the console loop, with its 100 ms size poll and its bounded key read — edits: `console::run`, which now takes an injected board source, re-reads the board once a second, redraws on a change and runs the agents read off the loop
@task_file n=3 path=src/console/view.rs
what: the console's new frame model: the board rows laid out as lineage trees, the row cut at the width, the `... <n> more` line and the shown and hidden counts
@task_file n=4 path=src/cmd/console.rs mark=edits
what: the `viola console` verb, which checks its terminal and home first and then hands the loop its terminal — edits: the call into `console::run`, which now passes the board source
@task_file n=5 path=crates/viola-state/src/board.rs mark=edits
what: the board read model that scans the home and reads each instance's state and event log — edits: `scan` and `read`, whose row gains the newest event line's time, held in the row and never serialised
@task_item n=1 part=needs
The console view: instances as parent/child trees in the order the board already derives for `viola list` (no second tree builder), each session's state and wheel holder in the board's own words (live/stale, idle/busy/unknown, `DIALOG <kind>`, driver/human/unknown), the six board captions in their words and slot order, and `LAST` after them.
@task_item n=2 part=needs
Human output escaping: every cell the view draws from disk or from upstream (names, lineage, CLI words, dialog kinds, agents rows) passes the board's control-escaping function, so no raw C0, C1 or DEL byte reaches the screen; the cut at the width never splits an escape. Upstream text stays content, never a command.
@task_item n=3 part=needs
The board read model, now called in-process once a second by the console: the board's home read rule, strict-modes checks before reading trusted state, its whole-board refusal and its re-parse rules for lineage, links, launch pair and window hold on every re-read exactly as for `viola list`; the console shows only what the board passes.
@task_item n=4 part=needs
The new last-activity time: taken in the pass the board read already makes over each event log, through the bounded event-line reader that tolerates a torn last line; no new unbounded read, and the reader stays a reader that writes nothing.
@task_item n=5 part=needs
The last-activity time is never serialised: it stays out of `viola list --json`, the MCP `list` tool's item and `viola list`'s board, whose fields and documents are unchanged.
@task_item n=6 part=needs
The `claude agents --json` read, called as it is: bounded, resolved from the caller's PATH, failing open to unknown; now at most once every 5 s and off the drawing loop. Its rows' names are shown, never treated as targets, and its probe log pair in the console's role file stays content-free and redacted.
@task_item n=7 part=needs
The console's role file and its `console-frame` event: the new `shown` and `hidden` fields are counts only, with every list and schema that pins the event's fields updated; the lines the board read and the agents read write there (`parse-rejected`, `state-recovered`, the agents-probe pair) keep their content-free form.
@task_item n=8 part=needs
The symlink skip-count warning line stays a count only, and is written by the console only when the count changes, as a named console exception to that rule.
@task_item n=9 part=needs
The frame's width and height come from the terminal's own size poll only: no environment variable or flag overrides the width or colour, and the frame stays plain ASCII with no SGR.
@task_item n=10 part=needs
The console key reader stays bounded and closed to the scaffold's keys (`q`, Ctrl-C, anything else unbound); the view adds no key, no send and no selection.
@task_item n=11 part=needs
The console's start checks (its terminal and home, the strict-modes refusal of `viola console`) keep their order and their fixed, path-free messages; the view's refusals and log lines interpolate no path, pid, id or upstream text.
@task_item n=12 part=needs
The board source injected into `console::run` is a plain parameter of the product code: no new environment read or test seam reaches the release build, and any test-only seam lives under the fake-agent test feature and is compiled out of release.
@task_item n=13 part=needs
The new code adds no panic path past the zero-panics gate and writes nothing: no file under the viola home beyond the console's own role file lines, nothing under the Claude home, no state file.
@task_item n=14 part=needs
The view's tests: booted fake-agent wrappers for live rows and planted instance directories for the size cases, every home test-owned and removed only by the test that made it; no real `claude`, no read of the operator's real Claude or viola homes, no window opened or compositor call.
@task_standing n=1
convention: every new test runs in its own test-created home, never the user's, with no real claude CLI and no secret in CI
home: secret-management
@task_standing n=2
convention: every new log line or log field carries no user content, no CLAUDE value and no path; full detail goes only to diagnostics
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=3
convention: every new or widened product reader is bounded or named in the bounded readers census
home: input-validation
@task_standing n=4
convention: every new fixture or planted test directory is synthetic, holds no home path, username or email, and is reviewed before commit
home: data-protection
@task_item n=1 part=not_needed
The wrapper channel methods, channel frames, send, answer, paste and release refusal rows: the view sends no frame and drives no instance; actions belong to a later chunk.
@task_item n=2 part=not_needed
The hook contract rows (hook stdin, hook decisions, hook fail-open): the view touches no hook.
@task_item n=3 part=not_needed
The revive, run, up, adopt, link and unlink refusal codes and JSON documents: the view changes none of those verbs.
@task_item n=4 part=not_needed
The verify rows, its probes, stamps, fixture refusal codes and residuals: the view runs no verify.
@task_item n=5 part=not_needed
The live rig arms and live plugin-install rounds: nothing live is fired by this chunk.
@task_item n=6 part=not_needed
The Hyprland recorder, focus and open window rows: the view opens, moves, focuses or closes no window and makes no compositor call.
@task_item n=7 part=not_needed
The pinned binary, pinned copy, plugin file rewrite, marketplace and trust record rows: the view places, hashes or writes none of them.
@task_item n=8 part=not_needed
The MCP stdin cap, MCP tool input and MCP isError rows beyond the list item: the view adds no tool and no MCP input.
@task_item n=9 part=not_needed
The config.json key rows: the view reads no new config value.
@task_item n=10 part=not_needed
The release, signing, deploy, hosting and network absences: the view adds no release stage and no network surface.
