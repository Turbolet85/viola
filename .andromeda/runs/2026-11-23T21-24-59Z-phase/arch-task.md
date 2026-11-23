@task_prose part=chunk
Builds the new CLI verb `viola open`: a terminal window (foot, spawned directly, no shell) per session beside its parent, over a closed flag allowlist with only the word claude after the double dash.
The started wrapper records its own window and origin (cli or agent) as a new log-only event kind `window`, a snapshot field `window?` replayed last-line-wins, and `window?` / `origin?` on list --json and MCP list items; logged codes-only.
Placement is the wrapper's: it reads Hyprland clients through the compositor seam (its first product caller) and issues the one closed move dispatch to a target workspace open computed before the spawn; off Hyprland or with no parent window it records why and skips placement.
Adds tests for the hypr tap recorder's three untested record-move failure codes, and the settled 5 s compositor read bound as an a11y time-bound row of its own.
Adds the test-only one-command live reading script R2, which gates v2-07; the build stops once before R2.
@task_file n=1 path=src/cmd/open.rs
what: the new open subcommand — the closed flag allowlist, the direct foot spawn, the target workspace computed before the spawn, the liveness wait and the open exit codes and --json document
@task_file n=2 path=src/cmd/mod.rs mark=edits
what: the root bin's CLI subcommand set — edits: the verb list, adding open
@task_file n=3 path=src/run/mod.rs mark=edits
what: the wrapper's run start path — edits: the hidden checked origin and target-workspace flags, the window lookup by terminal ancestor pid, the move dispatch and the window record
@task_file n=4 path=src/human/verbs.rs mark=edits
what: the human help and refusal lines of the CLI verbs — edits: the setup help line, the fixed refusal line for a word after the double dash other than claude
@task_file n=5 path=crates/viola-core/src/lib.rs mark=edits
what: viola-core's normalised event kinds and validators — edits: the new log-only event kind window and its validators
@task_file n=6 path=crates/viola-state/src/snapshot.rs mark=edits
what: the instance snapshot — edits: the optional window field
@task_file n=7 path=crates/viola-state/src/replay.rs mark=edits
what: the snapshot replay from the events log — edits: replay of window lines, the last line winning
@task_file n=8 path=crates/viola-state/src/board.rs mark=edits
what: the board read model behind CLI list and MCP list — edits: the list payload item fields window and origin
@task_file n=9 path=Cargo.toml mark=edits
what: the root bin's manifest — edits: viola-hyprland moved from a dev-dependency to a product dependency
@task_file n=10 path=src/bin/viola-fake-agent/hyprctl.rs mark=edits
what: the fake agent's stand-in hyprctl — edits: a clients reply keyed to the stand-in foot's pid, and the stand-in foot
@task_file n=11 path=crates/viola-e2e/tests/hypr_tap.rs mark=edits
what: the viola-hypr-tap recorder tests over planted hyprctl stubs — edits: tests for the record-move failure codes move-refused, move-unseen and window-left
@task_file n=12 path=scripts/r2-open.sh
what: the new test-only live reading script R2 — one viola open end to end beside a live parent under a scratch home, closing both windows on every exit path
@task_item n=1 part=needs
The new CLI verb open: its arguments (name, parent, closed role set overseer, builder, worker, cwd, workspace, model and effort, json, home), the claude-only word after the double dash, and the refusal of any flag outside the allowlist before anything starts.
@task_item n=2 part=needs
The terminal is spawned directly with a closed argv, no shell and no shell line built from the arguments; the launch authority stays bounded (v2-08).
@task_item n=3 part=needs
Open's exit codes and words: exit 2 for a usage error, exit 1 terminal-not-found and not-live, already-live and stale-heartbeat read before any spawn, strict-modes-failed before any log opens, no exit 11; the --json ok and error documents.
@task_item n=4 part=needs
The liveness wait: open returns once the instance has a live endpoint, bounded at 30 s; the window's appearance polled over clients at 100 ms within 5 s, else unplaced; each new bound is a time-bound row of its own.
@task_item n=5 part=needs
The compositor seam gains its first product caller: a clients query and the one closed move dispatch, its error words unsupported and unreachable read as placement skipped, hyprctl spawned directly with a closed argv, no environment read.
@task_item n=6 part=needs
The settled 5 s compositor read deadline joins the a11y time-bound rows (bounds fall on the driver, never on the human) as a row of its own.
@task_item n=7 part=needs
The new log-only event kind window (address?, workspace?, placement, origin), added in viola-core and appended only by the wrapper, with its validators and agent-neutral shape.
@task_item n=8 part=needs
The instance snapshot gains window?; the replay recovers it from the log, the last line winning, and replay writes no file.
@task_item n=9 part=needs
The board's list payload items and MCP list items gain optional window and origin, absent optional fields omitted; the six-column human board stays unchanged.
@task_item n=10 part=needs
The started run's argv carries two new hidden checked flags, origin and target workspace; the wrapper alone writes its snapshot and start lines.
@task_item n=11 part=needs
The parent's window is its recorded window, else an ancestry walk of at most 8 steps over its wrapper's ancestor pids matched against clients, held in memory; the parent defaults to the caller's VIOLA_NAME.
@task_item n=12 part=needs
The new start path stays inside the existing start rules: no start over a live or stale instance, a held name refused, the launch pair handed as recorded, the child's environment and argv as the run start sets them.
@task_item n=13 part=needs
The window record and open's process lines are codes-only; no address, title or path text reaches a home-level process log.
@task_item n=14 part=needs
The fake agent gains a stand-in foot and a clients reply keyed to its pid; the fake replays the recorded move dispatch on its exact argv, so CI runs open against the fake with no live claude.
@task_item n=15 part=needs
Tests for the hypr tap recorder's record-move failure codes move-refused, move-unseen and window-left, over planted stubs with no live window.
@task_item n=16 part=needs
The test-only live reading script R2: one live claude start under a scratch home outside every repository with a private CLAUDE_CONFIG_DIR, a no-start probe first, both windows closed on every exit path; never in CI; the capability ledger rows v2-07 (gated on R2) and v2-08.
@task_item n=17 part=needs
New names: the command open, its hidden run flags, the event kind window, and the binary viola-hypr-tap's codes, each held to the naming registers (kebab-case wire values, snake_case JSON fields, VIOLA_ prefix).
@task_item n=18 part=needs
The crate dependency direction: the root bin now depends on viola-hyprland as the one compositor seam; window features target Hyprland only.
@task_standing n=1
convention: each new end-to-end test runs in its own viola home, against the fake agent, with its waits under the shared bound and its home cleaned by owner record
home: occupied-resources
@task_standing n=2
convention: each new process log line is codes-only, emitted through the one event emitter, never with a print or level macro
home: cross-cutting-patterns · build-system
@task_standing n=3
convention: each new error type stays in its crate's one thiserror enum whose messages interpolate nothing; anyhow only at the bin edge
home: conventions
@task_standing n=4
convention: each new test-only script or fixture stays out of product builds and releases, and no live claude runs in CI
home: ci-cd-approach
@task_standing n=5
convention: each new Linux- or Hyprland-gated test names its reason
home: cross-cutting-patterns
@task_item n=1 part=not_needed
Send, wait, answer, last, pause, link and release channel methods and their refusal details: open sends no frame to a wrapper.
@task_item n=2 part=not_needed
The wheel, dialog hold and paste rules: open types nothing into the child.
@task_item n=3 part=not_needed
The hook contract, hook tiers and hook-to-kind map: no hook changes.
@task_item n=4 part=not_needed
Verify's probe runs, ledger stamps and their writer: the chunk adds no verify step and writes no stamp.
@task_item n=5 part=not_needed
Revive's own refusal codes and its --list document: revive is not changed.
@task_item n=6 part=not_needed
Plugin install and the plugin's marketplace tree: untouched.
@task_item n=7 part=not_needed
The fuzz workspace, mutation testing and release tooling: no fuzz target, mutation run or release change.
@task_item n=8 part=not_needed
Trust writer code words and trust records: open writes no trust record.
@task_item n=9 part=not_needed
Project-intent rows on a planned fleet console and the focus, up and adopt verbs: out of this chunk.
