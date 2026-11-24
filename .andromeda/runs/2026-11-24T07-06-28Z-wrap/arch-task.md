@task_prose part=session
The session added the CLI verb viola up [<NAME>...] [--dry-run] [--json]: after a reboot it re-opens each instance gone before the boot, through revive, parents first, live and stale ones skipped, one line per instance.
It added revive's hidden --origin <cli|agent> flag: under it a revive places its window on the workspace a real snapshot recorded; without it there is no window step, as before.
It added two bounded readers (the host boot time from /proc/stat, and whether events.ndjson ends with a session-end after its last session-start), the process-exit detail word boot-time-unread, and up on the help's setup row.
It mended two test reds by their fix: a settle wait in the e2e fd guard inside its existing 2 s deadline, and one shared teardown that ends a stand-in terminal's process tree from its leaves, start-time-checked pids only.
It measured that viola open writes only a process-start{subject:terminal} per spawned terminal and no process-exit; up does the same, and the rule saying otherwise is corrected to the code.
Proven over the fakes only; the live window and reboot reading (matrix v2-12) stays gated on R4, unfired.
@task_file n=1 path=src/cmd/up.rs
what: added — the up verb: the fleet scan, the selection rule, parents-first ordering, the terminal spawns and the per-instance outcome lines and document
@task_file n=2 path=tests/cli_up.rs
what: added — the up verb's integration cases over the fake agent, fake foot and fake hyprctl
@task_file n=3 path=scripts/r4-up.sh
what: added — the test-only R4 live-rig script for up (R4a windows with a rehearse form, R4b refusing first)
@task_file n=4 path=tests/support/terminal.rs
what: added — the shared test teardown that ends a stand-in terminal's process tree
@task_file n=5 path=src/cmd/mod.rs
what: edited — the CLI command enum and dispatch, gaining the up subcommand and revive's hidden --origin
@task_file n=6 path=src/cmd/revive.rs
what: edited — the revive verb: the hidden --origin flag and its window ask from the recorded workspace
@task_file n=7 path=src/cmd/open.rs
what: edited — the open verb: the direct terminal spawn, the live wait and the recorded-window read, now shared with up
@task_file n=8 path=src/human/verbs.rs
what: edited — the human output writers of the verbs: the up line and the hint line
@task_file n=9 path=src/human/hints.rs
what: edited — the human hint and refusal literals: the boot-time-unread pair and the up not-live hint
@task_file n=10 path=src/human.rs
what: edited — the human output module's exports
@task_file n=11 path=src/bin/viola-fake-agent/foot.rs
what: edited — the fake foot terminal of the fake agent binary, now recording each spawn
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited — the fake agent's unit cases of its modes
@task_file n=13 path=crates/viola-state/src/liveness.rs
what: edited — session liveness: heartbeat classification, now with the host boot time and a beat-before-boot read
@task_file n=14 path=crates/viola-state/src/replay.rs
what: edited — the snapshot replay and events log reader, now with the end-after-last-start read
@task_file n=15 path=crates/viola-state/src/board.rs
what: edited — the board read model's strict scan, gaining a read accessor over the scanned instances
@task_file n=16 path=schemas/diag-line.v1.json
what: edited — the diagnostics line schema: the process-exit detail enum
@task_file n=17 path=tests/cli_output_plain.rs
what: edited — the plain human output guard: the help literal and the served-verb list
@task_file n=18 path=tests/contract_error_bodies.rs
what: edited — the error-body contract guard and its CLI_DOCUMENTS list
@task_file n=19 path=tests/contract_bounded_readers.rs
what: edited — the bounded-readers guard: its BOUNDARIES and EXCEPTIONS lists
@task_file n=20 path=tests/cross_exit_causes.rs
what: edited — the exit-cause table guard
@task_file n=21 path=tests/cli_revive.rs
what: edited — the revive verb's integration cases, gaining the --origin window cases
@task_file n=22 path=tests/cli_machine_contract.rs
what: edited — the machine-output (--json) contract guard of the CLI
@task_file n=23 path=tests/contract_diag_schema.rs
what: edited — the diagnostics schema contract guard
@task_file n=24 path=tests/tui_channel_fds.rs
what: edited — the e2e guard that the wrapped child holds no viola home file descriptor
@task_file n=25 path=tests/support/mod.rs
what: edited — the shared test support module list
@task_file n=26 path=tests/cli_open.rs
what: edited — the open verb's integration cases, its terminal guard now handing its tree to the shared teardown
@task_item n=1 part=touches
The CLI verb set: a new served verb up, CLI-only (no MCP tool, no channel method); the cli-verbs contract items and the command names gain it, and the help's setup group now reads open, run, revive, up, verify, plugin install.
@task_item n=2 part=touches
The revive verb's contract and its CLI arguments: a hidden --origin <cli|agent> with closed values, conflicting with --list, with no --place; under it the revive places its window on the workspace a real snapshot recorded, else none.
@task_item n=3 part=touches
Revive resumes a gone instance in place: under --origin it now also replays the recorded workspace; without the flag nothing changes.
@task_item n=4 part=touches
Lineage is a label only: up reads the recorded parent only to order re-opens, parents first, with no path join.
@task_item n=5 part=touches
The CLI exit codes 0, 1 and 21: up exits 0 when every line is live, already-live, stale-heartbeat or would-open, else 1; 21 for the board's strict refusal; 1 with a document for boot-time-unread, strict-modes-failed, pinned-hash-mismatch and terminal-not-found.
@task_item n=6 part=touches
The process-exit detail enum: it gains boot-time-unread; up's exit 1 writes not-live when any line reads it; terminal-not-found and pinned-hash-mismatch are reused by up.
@task_item n=7 part=touches
Bounded readers at every boundary: two new bounded sources (the /proc/stat boot time through take(MAX_FRAME), and the events log's end-after-last-start through the bounded events reader) and one new exception, the up verb's own exe read; the guard contract_bounded_readers holds them.
@task_item n=8 part=touches
Strict-modes check before any log and the whole board refused on one failed check: up runs the strict board scan before opening the caller's cli log, and its strict refusal writes no line and exits 21.
@task_item n=9 part=touches
The cli-<name> diagnostics role file: up writes it only under a parsed VIOLA_NAME, after the scan.
@task_item n=10 part=touches
Session liveness rows (heartbeat-first liveness, replayed-snapshot liveness resting on the heartbeat, live/stale/gone): up classifies each instance by them, and adds a comparison of the heartbeat's mtime against the host boot time.
@task_item n=11 part=touches
Snapshot replay writes no file and the state-recovered diagnostics line: up reads each instance once through the replay, writes state-recovered as list does, and writes no snapshot, event line or stamp.
@task_item n=12 part=touches
list is a caller-side read needing no wrapper: up follows the same caller-side read model over the board, and the board read model gains a read accessor over its scanned instances.
@task_item n=13 part=touches
Pinned copy re-hashed before reuse and start pins the bytes it checked: before its first spawn up checks the home tree, pins the bytes and refuses pinned-hash-mismatch, and each terminal runs that pinned copy.
@task_item n=14 part=touches
Direct spawns with a closed argv (the open terminal spawn, hyprctl direct spawn, no shell-out): up spawns foot directly through open's spawn path with the closed argv --app-id=viola.<name> --title=<name> --working-directory=<recorded cwd> <pinned> --home <home> revive <name> --origin <word>; a boundary widening of what the open terminal starts.
@task_item n=15 part=touches
No start over a live or stale instance and revive refusing a session a live claude holds: up skips live and stale instances and names session-live, no-session and cwd-missing per instance instead of starting.
@task_item n=16 part=touches
Window features target Hyprland: up's placement is the recorded workspace through revive's window step; it reports placement per line.
@task_item n=17 part=touches
Short-lived CLI verbs may use stderr: up writes one stderr hint line when any line reads not-live, none under --json.
@task_item n=18 part=touches
The CLI --json output contracts: up's document {"v":1,"ok":{"instances":[{"name","outcome","placement"}]}} and its error documents join the CLI documents.
@task_item n=19 part=touches
Fake-agent test seams: the fake foot now appends one {"pid","argv"} record per spawn to foot-spawns.ndjson beside itself; foot-argv is still written.
@task_item n=20 part=touches
Test hygiene rows (test evidence survives a kill, root tests and their guards, test-side removals): a stand-in terminal's tree is now ended by one shared teardown, signalling only recorded pids whose start time still matches, leaves under a running parent first, the rest waited under one deadline.
@task_item n=21 part=touches
Profraw census as a dev-host witness: it measured the truncated coverage profile red (refused 7 of 2227 with the old kill, refused 0 with the shared teardown) and witnessed the fix.
@task_item n=22 part=touches
New capabilities as subcommands: up lands as a subcommand in the root bin, module cmd::up; no crate added.
@task_item n=23 part=touches
The automatic verify under its own lock (ledger/verify.lock): with verify_new_cli on, the first instance up re-opens may read not-live while its verify runs and later starts come up unverified behind the held lock; read from the code only.
@task_fact n=1 slug=occupied-resources
fact: The command name up is occupied: viola up, a CLI-only subcommand of the viola binary.
@task_fact n=2 slug=conventions
fact: viola up [<NAME>...] [--dry-run] [--json] re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; with names given the fleet narrows to them, a name not scanned reads not-taken; it is CLI-only, no MCP tool and no channel method.
@task_fact n=3 slug=established-decisions
fact: up's selection rule: an instance is re-opened only when it is not live or stale, its heartbeat is older than the host boot time, and its events log has no session-end after its last session-start (an instance ended on purpose or ended this boot stays down).
@task_fact n=4 slug=established-decisions
fact: up re-opens parents before children; a missing parent makes its child a root, and a cycle is visited once.
@task_fact n=5 slug=conventions
fact: up's outcome words are a closed set: live, already-live, stale-heartbeat, would-open, no-session, cwd-missing, session-live, not-live, not-taken; one plain line per instance, name then word, or live then its placement.
@task_fact n=6 slug=conventions
fact: up exits 0 when every line is live, already-live, stale-heartbeat or would-open, else 1; 21 on the board's strict refusal with no line; 1 with a document for boot-time-unread, strict-modes-failed, pinned-hash-mismatch and terminal-not-found.
@task_fact n=7 slug=standard-contracts
fact: up's --json document is {"v":1,"ok":{"instances":[{"name","outcome","placement"}]}}, one document for the whole fleet.
@task_fact n=8 slug=standard-contracts
fact: up writes no snapshot, no event line and no stamp; --dry-run spawns nothing and prints the plan as would-open lines.
@task_fact n=9 slug=standard-contracts
fact: up spawns each terminal running the pinned viola revive <name> --origin <cli|agent> under a closed argv, and waits for it under open's existing live bound; it mints no new time bound.
@task_fact n=10 slug=occupied-resources
fact: viola revive takes a hidden --origin <cli|agent>, conflicting with --list and with no --place: under it the revive places its window on the workspace a real snapshot recorded, else none; without it there is no window step.
@task_fact n=11 slug=occupied-resources
fact: The process-exit detail enum holds boot-time-unread: exit 1 of up when the host boot time cannot be read.
@task_fact n=12 slug=cross-cutting-patterns
fact: The host boot time (the btime line of /proc/stat) is read as a bounded source, and whether an events log ends with a session-end after its last session-start is read through the bounded events reader.
@task_fact n=13 slug=occupied-resources
fact: A stand-in terminal's process tree in tests is ended by one shared teardown: only recorded pids whose start time still matches are signalled, a leaf under a running parent is killed first, every other member is waited on under one deadline and killed only at it.
@task_fact n=14 slug=occupied-resources
fact: The fake foot records each spawn as one {"pid","argv"} line in foot-spawns.ndjson beside itself.
@task_item n=1 part=untouched
The MCP server and its tools, results and stdin rows: up adds no MCP tool and the server is unchanged.
@task_item n=2 part=untouched
The wrapper channel: its methods, params, frames, fault codes and responses; up speaks no frame.
@task_item n=3 part=untouched
Hooks, their tiers, exits, kind map and dialog rows; the plugin's contents and install.
@task_item n=4 part=untouched
Send, wheel, wait, answer, link, pause and release rows; dialog holds and confirmation windows.
@task_item n=5 part=untouched
The capability ledger, verify's probes, fixtures and stamps writer rows (beyond the verify lock touch above).
@task_item n=6 part=untouched
Normalised event kinds and per-kind event data: no event kind was added and up appends no event line.
@task_item n=7 part=untouched
config.json keys and configuration precedence: no config key or environment variable was added.
@task_item n=8 part=untouched
Dependencies, toolchain, cargo-deny, lint bans, CI workflows, release and deployment rows: no dependency, workflow or release change.
@task_item n=9 part=untouched
The design-philosophy and project-intent absences (no daemon, no database, no graphical frontend, no network port).
