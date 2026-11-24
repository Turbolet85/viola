@task_prose part=chunk
Adds a new human CLI verb `viola up` that re-opens the fleet after a host reboot (capability v2-12, Restore the fleet after a reboot).
It reads the board's instances under the home from normalised state and events, selects those that read as live before the host went down, and orders them parents before children by the recorded lineage.
It skips every instance whose recorded pid and start time verify live, and re-opens each remaining one resumed in place through the existing revive path, in the terminal window viola open would give it, in its recorded workspace.
It prints one line per instance naming the instance and a closed outcome word, and under --json a document of codes and closed words only.
It adds tests over the fake agent and the Hyprland fakes under a test-owned home, and re-measures at HEAD the lineage and launch lines at every start, open's window record under --origin, and revive resuming in place.
@task_file n=1 path=src/cmd/ mark=edits
what: the root bin's clap dispatch of CLI verbs, the human CLI command surface — edits: a new verb `up` in the dispatch
@task_file n=2 path=src/human/ mark=edits
what: the human module, the one human writer of the root bin for human text lines — edits: one line per instance for `up`
@task_item n=1 part=needs
The CLI verb table and command name register gain a verb `up`: its help row, its arguments, its flag and subcommand style, and its entry as a new capability added as a subcommand.
@task_item n=2 part=needs
The exit-cause table and CLI exit codes for `up`: which exit code each outcome takes, retired codes never reused, the internal-error line and document.
@task_item n=3 part=needs
The --json machine contract for `up`: a document of codes and closed words only, field names in snake_case, enum values in kebab-case, the code word format, declared key order kept, absent optional fields omitted, the --json error document shape.
@task_item n=4 part=needs
Human output: one line per instance through the single human writer in the root bin, human text by default, no print macros, no upstream text or option text in the human line, colour variables deciding only SGR output, sanitised error surfaces.
@task_item n=5 part=needs
The board read model: `up` reads instances under the home through the viola-state board scan, sync and Claude-free, with the strict-modes read rule held on the home tree and the whole board refused on one failed check; the home resolved by the viola home resolution order.
@task_item n=6 part=needs
The root bin consumes only normalised state and events: the instance snapshot (pid and started_at, cwd, lineage, launch, window, agent_session_id), the event line and its kinds (session-start, session-end, lineage, launch, window), snapshots read through the replay, replay writing no file, missing and null reading the same, tolerant versioned own formats.
@task_item n=7 part=needs
Session liveness: heartbeat first, pid plus recorded start time deciding gone, the live, stale and gone readings; `up` skips an instance whose recorded pid and start time verify live, and a replayed snapshot's liveness rests on the heartbeat.
@task_item n=8 part=needs
The selection of instances that read as live before the host went down versus ended on purpose, from what the disk holds after a reboot (snapshot, heartbeat, events log, runtime-dir sockets and locks on a tmpfs); stale socket files left in place and a missing socket read as gone.
@task_item n=9 part=needs
Parents-before-children order from the recorded lineage (lineage parent and role, a label only), and what a missing, never-live or cyclic parent does; identity by instance, with ViolaName as the instance key.
@task_item n=10 part=needs
Revive's start arm reused per instance: revive resumes a gone instance in place through the start path, reads the snapshot once, refuses a session a live claude holds, no start over a live or stale instance, a start refuses a held name, the spawn directory confined to the snapshot's cwd, revive over a replayed snapshot refused.
@task_item n=11 part=needs
Revive's preflight and refusals as `up` meets them per instance: the claude agents --json listing that degrades and never errors, session-live, the strict-modes checks, socket-directory checks, pinned copy re-hashed before reuse, and the closed revive refusal codes (already-live, session-live, stale-heartbeat, cwd-missing, no-session, squatted-name, pinned-hash-mismatch, strict-modes-failed, socket-dir-refused, internal-error) mapped to an outcome word.
@task_item n=12 part=needs
The argv handed to each start: the recorded launch pair (model, effort) handed from the record and never verified, --resume with the logged session id, --plugin-dir, the child's environment (VIOLA_NAME, VIOLA_DIR, VIOLA_BIN, PATH) with inherited CLAUDE names stripped, the program resolved by revive from its own PATH, and the lineage and launch lines written at every start.
@task_item n=13 part=needs
The terminal window viola open gives an instance: open's terminal spawn, the wrapper's window record with address, workspace, placement and origin under --origin, and what a recorded window means after a reboot when compositor addresses die with it.
@task_item n=14 part=needs
The Hyprland seam: viola-hyprland is the one compositor seam, its closed request set, hyprctl spawned directly with a closed argv, no shelling out, window features targeting Hyprland, and the seam's closed errors (unreachable, refused, malformed, oversize, unsupported).
@task_item n=15 part=needs
The driver boundary: re-opening the fleet stays a human control, CLI-only, with no MCP tool and no channel method added; the conservative arm of a boundary widening.
@task_item n=16 part=needs
Observability: a per-instance line in the obs event catalog through obs_event! as the only emitter, codes-only process logs per role in the cli role file, and the never-log floor of no address, pid or workspace in a process-log line; error chains only in the detail file.
@task_item n=17 part=needs
Error typing for the new code: thiserror per crate with messages interpolating nothing, anyhow only at the root bin edge, nutype errors kept out of crate enums.
@task_item n=18 part=needs
The cost of a start and a fleet of N in wall time against existing bounds; a new time bound is not added or split here and waits on a founder fork, and root test waits share one bound.
@task_item n=19 part=needs
The capability ledger claim for v2-12: proved by the fakes where they can carry a clause, and GATED on the operator's live reading at the host where a clause needs a real host restart or a real resumed session; no live claude in CI, the real claude CLI only locally.
@task_item n=20 part=needs
The tests that prove selection, order, skip, lines and argv run against the fake agent and the Hyprland fakes, each in its own test-owned viola home, with fake-agent test seams, test endpoints outside the product namespace, and test homes cleaned by their owner.
@task_standing n=1
convention: each new test runs in its own isolated viola home, its home removed by owner record only, its sockets under the short test root
home: occupied-resources
@task_standing n=2
convention: each new test runs against the fake agent, never a live claude, on Linux in CI
home: ci-cd-approach
@task_standing n=3
convention: each new log line is codes-only, emitted through obs_event!, with no content, address, pid or workspace
home: cross-cutting-patterns
@task_standing n=4
convention: each new end-to-end test holds zero panic lines and keeps insta snapshots in check mode only
home: project-directory-structure
@task_standing n=5
convention: each new module is wired into the module tree with no orphan module, no print macro and no tracing level macro
home: build-system
@task_standing n=6
convention: each new CLI flag, subcommand and --json field follows the CLI flag style, snake_case field names and kebab-case enum values
home: conventions
@task_standing n=7
convention: each new test keeps the coverage floors enforced by the harness gate
home: ci-cd-approach
@task_item n=1 part=not_needed
The send path: bracketed-paste send, send confirmation, paste bounds, typed-text normalisation, send refusals and the flight lock; `up` types nothing into any session.
@task_item n=2 part=not_needed
The wheel and dialogs: wheel claim, release and human take, dialog hold, pending dialogs and the answer path; `up` neither moves a wheel nor answers a dialog.
@task_item n=3 part=not_needed
The hook contract and hook tiers, the hook-to-kind map and hook diagnostics; `up` adds no hook and changes no hook behaviour.
@task_item n=4 part=not_needed
The MCP server's tools, results, stdin caps and deadlines; `up` is CLI-only.
@task_item n=5 part=not_needed
viola verify, the ledger stamps envelope, verify probes, recorded fixtures and the trust-record writer words; `up` writes no stamp and runs no probe.
@task_item n=6 part=not_needed
Link and unlink recording, link sets and the from and last channel reads.
@task_item n=7 part=not_needed
The PTY seam, vt100 screen model, readiness gate and screen signatures; `up` reaches the child only through the revive start path.
@task_item n=8 part=not_needed
Plugin install, the marketplace tree and the plugin's .mcp.json.
@task_item n=9 part=not_needed
Fuzzing, the fuzz workspace, release tooling, dist and crates.io publishing.
@task_item n=10 part=not_needed
The status-line, usage-budget, graphical frontend, mobile view and terminal fleet console absences; the console and model and effort switching are their own entries.
