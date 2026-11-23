@task_prose part=chunk
Builds the verb `viola open`: a terminal window (foot, started directly, no shell) per session beside its parent, over a closed flag allowlist, with only `claude` after the dashes.
The started wrapper finds its own window through the Hyprland seam (its first product caller), issues the one closed move dispatch to the target workspace, and records its window and origin in its own state, logged codes-only.
Adds a log-only event kind `window`, a snapshot `window?` replayed last-line-wins, and `window?` / `origin?` on `list --json` and MCP `list` items.
Adds tests for the recorder's three untested move failure codes over planted stubs, and the 5 s compositor read bound as a time-bound row of its own.
Adds a test-only one-command live reading script (R2) and stops before running it; re-reads at P3 the seam counts and the rows the cancelled take-up reached.
@task_file n=1 path=src/cmd/open.rs
what: the new CLI verb `open` — its closed flag allowlist, the direct terminal spawn, the liveness wait and its exit codes and `--json` document
@task_file n=2 path=src/cmd/mod.rs mark=edits
what: the CLI verb table of the root bin — edits: the new `open` verb entry
@task_file n=3 path=src/run/mod.rs mark=edits
what: the wrapper `viola run` — edits: the two hidden checked flags (origin, target workspace), the window find by terminal-ancestor pid and the one move dispatch
@task_file n=4 path=src/run/snapshot.rs mark=edits
what: the wrapper's snapshot writer — edits: the snapshot `window?` field and its start line
@task_file n=5 path=src/human/verbs.rs mark=edits
what: human-mode help and refusal lines — edits: the help line `setup:` list and the fixed refusal line for a word after `--` other than `claude`
@task_file n=6 path=crates/viola-core/src/lib.rs mark=edits
what: the core crate's shared validators and event kinds — edits: the event kind `window` and its validators
@task_file n=7 path=crates/viola-state/src/snapshot.rs mark=edits
what: the state crate's snapshot model — edits: the `window?` field
@task_file n=8 path=crates/viola-state/src/replay.rs mark=edits
what: the event-log replay into the snapshot — edits: the `window` kind replayed, last line winning
@task_file n=9 path=crates/viola-state/src/board.rs mark=edits
what: the board behind `viola list --json` — edits: the `window?` and `origin?` item fields; the six-column human board unchanged
@task_file n=10 path=Cargo.toml mark=edits
what: the root manifest's dependency graph — edits: the Hyprland seam crate moved from dev-dependencies to a product dependency
@task_file n=11 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent test seam — edits: a stand-in `foot` and a `clients` reply keyed to the stand-in's pid
@task_file n=12 path=crates/viola-e2e/tests/hypr_tap.rs mark=edits
what: the Hyprland recorder's tests over planted stubs — edits: tests for the `record-move` failure codes `move-refused`, `move-unseen`, `window-left`
@task_file n=13 path=scripts/r2-open.sh
what: a new test-only live-reading script — one `viola open` end to end beside a stand-in parent under a scratch home with a private Claude config dir, closing both windows on every exit path
@task_item n=1 part=needs
The open verb's CLI input is a trust boundary: a closed flag allowlist (name, parent, role overseer|builder|worker, cwd, workspace, model / effort, json, home), refused with exit 2 before anything starts; after `--` the word `claude` alone, any claude flag refused.
@task_item n=2 part=needs
The terminal is resolved from PATH and spawned directly as argv elements, never through a shell, whatever characters the arguments hold; the viola it runs is the pinned absolute path, re-hashed before use.
@task_item n=3 part=needs
Every name open takes — the session name, `--parent`, the parent defaulted from the caller's `VIOLA_NAME` — passes the viola name check before any path, app id or title is composed from it; the self-reported parent authenticates nothing.
@task_item n=4 part=needs
Open checks the home tree by strict modes before any log opens or state is read (exit 1 `strict-modes-failed`), and reads the start collision check (`already-live`, `stale-heartbeat`) before any spawn.
@task_item n=5 part=needs
New refusal and error words (`terminal-not-found`, `not-live`, the after-dashes refusal) are fixed code words; the `--json` document `{"v":1,"ok":{name,parent,window,placement}}` / `{"v":1,"error":code,"detail":null}`; refusals hold no paths, pids or ids; no exit 11; the CLI exit code rows reached.
@task_item n=6 part=needs
The Hyprland seam becomes a product dependency: the product issues only the one closed move dispatch, moving only the window whose pid is its own terminal parent, with no focus change; off Hyprland the seam's own `unsupported` / `unreachable` words, no environment read.
@task_item n=7 part=needs
Every compositor read is bounded by the 5 s hyprctl deadline; the window's appearance poll is bounded at 5 s (100 ms steps) then `unplaced`; open's liveness wait is bounded at 30 s then `not-live` with the window left standing — each new bound a row beside the time-bound lists.
@task_item n=8 part=needs
Hyprland reply text is dropped at the parse: only the window address, workspace and pid are kept from a `clients` reply, and no reply text reaches a log, an error or an output.
@task_item n=9 part=needs
A new log-only event kind `window {address?, workspace?, placement, origin}`, codes-only, its fields closed shapes validated by the shared core validators and re-checked on read.
@task_item n=10 part=needs
Only the instance's wrapper writes its snapshot and start lines: the window record is the wrapper's own, never open's; the replayed `window?` is inert, runs and frames nothing, last line winning.
@task_item n=11 part=needs
The origin (`cli` | `agent`) and the target workspace cross onto the started `run`'s argv as two hidden flags, checked before anything is read, as the run's lineage and launch-pair flags are.
@task_item n=12 part=needs
The parent's window is read from its recorded snapshot only after the strict-modes check, else from an ancestry walk of at most 8 steps over process identity, held in memory and never recorded.
@task_item n=13 part=needs
`list --json` and the MCP `list` tool items gain `window?` and `origin?`, shown only when they re-parse; the six-column human board is unchanged.
@task_item n=14 part=needs
The open verb reads `VIOLA_NAME` for the parent default: a new environment read, held to the rule that a new seam or environment read needs a log entry.
@task_item n=15 part=needs
The recorder's `record-move` failure codes `move-refused`, `move-unseen` and `window-left` tested over planted stubs with no live window; the test-only recorder moves only its own terminal's window.
@task_item n=16 part=needs
The fake agent's stand-in `foot` and pid-keyed `clients` reply are test seams under the fake-agent feature only, compiled out of the release build.
@task_item n=17 part=needs
The R2 script is test-only: a scratch home outside every repository, a private Claude config dir, nothing read or written under the operator's real Claude home, one live claude start, both windows closed on every exit path; this phase makes no live start, moves or opens no window.
@task_item n=18 part=needs
The started claude child under the wrapper keeps the existing environment strip and pinned-folder PATH; open passes `--model` / `--effort` through the checked launch-pair flags only.
@task_standing n=1
convention: each new test runs under its own test home, never the user's home, and its homes stay out of git
home: secret-management · bootstrap-phases
@task_standing n=2
convention: each new or changed recorded fixture (a `clients` reply, a stub reply) is scrubbed by the closed allowlist and under the fixture hygiene walk
home: data-protection
@task_standing n=3
convention: each new log line carries code words only — no user content, no CLAUDE values, no paths outside diagnostics
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=4
convention: each new error or refusal message is fixed text interpolating nothing, with no path or upstream text
home: error-handling
@task_standing n=5
convention: each new product reader of external bytes is bounded or named in the bounded-readers census
home: input-validation
@task_standing n=6
convention: each new test seam sits only under the fake-agent feature and is compiled out of release
home: security-anti-patterns
@task_item n=1 part=not_needed
Paste and typed-text rules (bracketed paste, the paste control-character set, empty-text refusal, line-ending normalisation): open types nothing into a session.
@task_item n=2 part=not_needed
Dialog answering, the answer behaviour enum, dialog free text and the dialog hold: open answers no dialog.
@task_item n=3 part=not_needed
The link and unlink channel methods and their event lines, and the pause and release driver wheel rules: open links, pauses or releases nothing.
@task_item n=4 part=not_needed
viola verify's probes, its stamps, its probe residuals and its unclean-fixture refusal codes: open runs no verify.
@task_item n=5 part=not_needed
Plugin install, the marketplace tree and the trust record writer: open installs no plugin and writes no trust record of its own.
@task_item n=6 part=not_needed
Hook payload parsing, hook-decision details and hook capture arms: the chunk adds no hook path.
@task_item n=7 part=not_needed
Transcript reads, stop background-task and session-cron counts, and API-error turn-end classes in the Claude adapter: untouched.
@task_item n=8 part=not_needed
config.json keys (claude_env_keep, diagnostics_level, dialog_hold_s): the chunk adds no configuration.
@task_item n=9 part=not_needed
The channel's JSON-RPC error codes and the wrapper channel methods' params: open adds no channel method and sends no frame of its own beyond the existing liveness read.
@task_item n=10 part=not_needed
Release signing, dist attestations, self-update signatures and the weekly advisory and fuzzing schedules: no release stage is touched.
