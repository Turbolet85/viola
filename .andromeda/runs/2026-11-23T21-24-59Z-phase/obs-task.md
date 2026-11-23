@task_prose part=chunk
Builds the CLI verb `viola open`: a closed flag allowlist, a terminal (foot) started directly with no shell, the started wrapper placing its own window with one compositor move dispatch, and open returning once the instance is live.
The wrapper records its own window and the origin it was handed: a new log-only event kind `window`, a snapshot `window?` replayed last line wins, and `list --json` and MCP `list` items gain `window?` and `origin?`.
The Hyprland seam becomes a product dependency for the first time; its compositor read deadline (5 s) joins the time-bound rows beside the new 30 s liveness wait and 5 s window-appearance bound.
Adds tests for the test-only recorder's three untested move failure codes, and a test-only live script for the one live reading R2, which gates the launch capability.
@task_file n=1 path=src/cmd/open.rs
what: the new CLI verb `viola open` — its argument allowlist, terminal spawn, liveness wait, exit codes and --json document
@task_file n=2 path=src/cmd/mod.rs mark=edits
what: the CLI verb dispatch of the root binary — edits: the verb list, which gains open
@task_file n=3 path=src/run/mod.rs mark=edits
what: the wrapper `viola run` start — edits: the hidden checked origin and target-workspace flags, the wrapper's window lookup and its one move dispatch, the window record
@task_file n=4 path=src/run/snapshot.rs mark=edits
what: the wrapper's snapshot and start-line writer — edits: the snapshot's window field
@task_file n=5 path=src/human/verbs.rs mark=edits
what: the human help and refusal lines of the CLI verbs — edits: the setup help line, the fixed refusal line for a word after `--` other than claude
@task_file n=6 path=crates/viola-core/src/lib.rs mark=edits
what: viola-core's event kinds and validators — edits: the new log-only event kind window
@task_file n=7 path=crates/viola-state/src/replay.rs mark=edits
what: the snapshot replay from the events log — edits: the window field, last line winning
@task_file n=8 path=crates/viola-state/src/snapshot.rs mark=edits
what: the snapshot.json shape in viola-state — edits: the optional window field
@task_file n=9 path=crates/viola-state/src/board.rs mark=edits
what: the board and its list --json items — edits: the window and origin item fields; the six-column board unchanged
@task_file n=10 path=crates/viola-mcp/src/tools.rs mark=edits
what: the viola-mcp stdio server tools — edits: the list tool's items, which gain window and origin
@task_file n=11 path=Cargo.toml mark=edits
what: the workspace manifest and its dependency tables — edits: viola-hyprland moved from dev-dependencies to a product dependency
@task_file n=12 path=crates/viola-e2e/tests/hypr_tap.rs mark=edits
what: the integration tests of the test-only compositor recorder over planted hyprctl stubs — edits: tests for the record-move failure codes move-refused, move-unseen and window-left
@task_file n=13 path=scripts/r2-open.sh
what: the new test-only live script for R2: one viola open end to end beside a live parent under a scratch home, closing both windows on every exit path
@task_item n=1 part=needs
The new cli verb open logs as the cli role: a process log only under a parsed instance name, process-start and process-exit lines with a closed detail code, codes-only, its manual dispatch span per verb.
@task_item n=2 part=needs
New exit-1 detail codes terminal-not-found and not-live join the closed exit-1 detail code lists, beside already-live and stale-heartbeat read before any spawn; exit 2 for a word outside the allowlist; no exit 11; each cause a distinct code with no path or pid in it.
@task_item n=3 part=needs
strict-modes-failed is refused before any log opens: that refusal writes no line, and its stderr pair and --json error document follow the existing strict-modes-failed rows for CLI verbs.
@task_item n=4 part=needs
The open --json document `{"v":1,"ok":{name,parent,window,placement}}` or `{"v":1,"error":code,"detail":null}`, and the cli internal-error and panic outcomes for a cli and cli --json verb, apply to open.
@task_item n=5 part=needs
stderr carries fixed strings only: the refusal pairs and the one fixed line for a word after `--` interpolate nothing from the arguments, whatever characters a name, cwd or flag value holds; controls escaped where terminal output shows them.
@task_item n=6 part=needs
The foot spawn and the compositor reads and move dispatch are child and OS operations instrumented at viola's call sites: spawn lines carry a subject from the closed subject enum, never the argv, cwd, home, pinned binary path or environment values.
@task_item n=7 part=needs
Compositor replies are untrusted: a hyprctl JSON reply that fails to parse surfaces as a parse-rejected line for parser hyprctl-json and an unknown outcome, never a guess; the seam's unsupported and unreachable words recorded as the placement reason.
@task_item n=8 part=needs
The new event kind window `{address?, workspace?, placement, origin}` lands in the events log as a product event with closed values (placement, origin cli or agent), log-only, never copied into process logs; the event line shape and the replay rows bind it.
@task_item n=9 part=needs
Product readers read the window through the replayed snapshot, last line winning; the replayed snapshot's live-field rules stay as they are and window is an additive field.
@task_item n=10 part=needs
list --json items and MCP list tool items gain the optional fields window and origin, additively; absent values written as key absence; the board's six columns unchanged.
@task_item n=11 part=needs
Parent and role names, and the window address, stay on product surfaces (events, snapshot, list); they are never fields of a process log line.
@task_item n=12 part=needs
The parent defaults to the caller's VIOLA_NAME when an agent calls open from its own shell — the instance-name source of the cli role; no other environment variable is read as configuration, and no environment read decides whether Hyprland is present.
@task_item n=13 part=needs
The started run carries the origin and target workspace as two hidden checked flags on its argv; the wrapper alone writes the window record and its start lines, and stays silent on the terminal.
@task_item n=14 part=needs
New time bounds each land as a row of their own beside the time-bound lists: open's liveness wait at most 30 s, the window's appearance 5 s polling clients at 100 ms, and the settled 5 s compositor read deadline HYPRCTL_DEADLINE; bounded polling is not a retry policy.
@task_item n=15 part=needs
Liveness is read through the heartbeat and live endpoint: open returns when the instance is live, and already-live or stale-heartbeat by the heartbeat staleness threshold is read before any spawn; liveness transitions are not logged.
@task_item n=16 part=needs
The parent-window ancestry walk is at most 8 steps held in memory; no move when no parent window (placement unplaced); on a locked desktop open proceeds; off Hyprland the terminal still starts and the record says why.
@task_item n=17 part=needs
viola-hyprland becomes a product dependency of the root binary: obs dependency rules bind it — synchronous, Tokio-free, building no C, passing cargo deny.
@task_item n=18 part=needs
The recorder's three failure codes move-refused, move-unseen and window-left are each reached over a planted stub variant with no live window; their tests' assertions are machine-checked verdicts.
@task_item n=19 part=needs
R2 is one live reading run only by a test-only script at the operator's host, never in CI: a scratch home outside every repository and a private CLAUDE_CONFIG_DIR, its local outputs never uploaded, one live claude start and no retry.
@task_item n=20 part=needs
The R2 script's verdict and its parent wrapper over a stand-in child read the product's own logs and events as the machine verdict; no telemetry is read from screen content.
@task_standing n=1
convention: each new diagnostics line goes through the one obs_event macro with a closed event value, required fields only added to, schema-conformant to the line schema
home: 6-log-coverage
@task_standing n=2
convention: each new integration or E2E test keeps its home under the one test-home root, kept until the gates run, its emitted lines schema-checked and counted by the zero-panics gate
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: each new test waits on a condition, never a sleep, and carries no retry-once
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: each new instrumented function declares its span with skip-all, a static low-cardinality name and an explicit field allow-list, no err or ret options
home: 4-span-trace-coverage
@task_standing n=5
convention: each new error type carries a fixed Display message that interpolates nothing; serde chains never printed
home: 7-error-capture-reporting
@task_standing n=6
convention: each new fixture or stub is synthetic and is no telemetry source
home: pii-scrubbing-compliance
@task_item n=1 part=not_needed
Hook role rows: hook stdout and stderr, hook fail-open, hook latency and session-end budgets, hook correlation — open adds no hook and changes no hook path.
@task_item n=2 part=not_needed
The send, wait, last, dialog and wheel paths: their log fields, span chains, refusal details, confirm windows and corr rows — open sends no text and answers no dialog.
@task_item n=3 part=not_needed
Channel JSON-RPC framing, its -32602 and -32603 errors, conn and srv_conn join keys and peer-credential checks — open speaks no channel request of its own.
@task_item n=4 part=not_needed
The MCP stdin framing, MAX_FRAME and the MCP frame-tool refusals for wait and last — only the list tool's items change.
@task_item n=5 part=not_needed
Link, unlink, pause and release rows, and the implicit link on send and answer — open records no link.
@task_item n=6 part=not_needed
verify, plugin install, the capability ledger and stamps, the version gate and the revive preflight order — open neither verifies nor revives.
@task_item n=7 part=not_needed
PTY seam, vt100, paste, typed keys and screen-reading rows — open drives no PTY of its own and reads no screen.
@task_item n=8 part=not_needed
Future error reporter, OTel SDK or exporter, Sentry, OTLP receiver and network egress rows — the chunk adds no reporter, exporter or network path.
@task_item n=9 part=not_needed
The claude child's environment strip rows (env_kept, env_stripped counts) — open starts no claude directly; the wrapper's start is unchanged.
