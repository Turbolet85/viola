@task_prose part=session
Added the CLI-only verb viola up [NAME...] [--dry-run] [--json]: after a reboot it re-opens each instance whose heartbeat predates the boot and whose log has no session-end after its last session-start, skips live and stale ones, orders parents before children, and spawns one foot terminal per instance running the pinned viola revive <name> --origin <cli|agent>.
viola up prints one line per instance from a closed outcome word set (live, already-live, stale-heartbeat, would-open, no-session, cwd-missing, session-live, not-live, not-taken), exits 0 or 1, 21 on the board's strict refusal, writes no snapshot, stamp or event line, and adds the exit cause boot-time-unread with its own hint and process-exit detail word.
viola revive gained a hidden --origin flag: only under it does a revive place its window, at the workspace a real snapshot recorded, else unplaced; without it there is no window step.
Two new bounded readers (the /proc/stat boot time, an instance log's end after its last start), one bounded-reader exception, five new error-body documents, and the help setup row now lists up.
Test-only: one shared teardown ends a stand-in terminal's process tree from its leaves with start-time-checked kills, closing truncated coverage profiles (refused 0 at 192 x 16, refused 7 of 2227 with the old kill); the e2e fd guard re-reads the child's fd table inside its existing 2 s deadline.
A gated live rig script for R4 was added (rehearsal only; R4a and R4b unfired, R4b refuses first); matrix v2-12 stays gated, proven over the fakes only.
@task_file n=1 path=src/cmd/up.rs
what: added — the viola up verb: fleet selection, parents-first order, terminal spawn per instance, closed outcome words and exits
@task_file n=2 path=tests/cli_up.rs
what: added — the viola up integration cases run as a subprocess over the fake agent and fake foot, with their terminal drop guard
@task_file n=3 path=scripts/r4-up.sh
what: added — the gated live rig for R4 (R4a windows with a rehearse form, R4b plant and after refusing first)
@task_file n=4 path=tests/support/terminal.rs
what: added — the one shared test helper that ends a stand-in terminal's process tree from its leaves, start-time-checked
@task_file n=5 path=src/cmd/mod.rs
what: edited — the CLI command table and verb dispatch, now serving up
@task_file n=6 path=src/cmd/revive.rs
what: edited — viola revive, its preflight and the hidden --origin window step
@task_file n=7 path=src/cmd/open.rs
what: edited — viola open's terminal spawn, its live wait and recorded-window read, now shared with up
@task_file n=8 path=src/human/verbs.rs
what: edited — the human-mode verb line writers (the up line and the hint line)
@task_file n=9 path=src/human/hints.rs
what: edited — the fixed human hint and refusal literals (boot-time-unread pair, the up not-live hint)
@task_file n=10 path=src/human.rs
what: edited — the human output module's exports
@task_file n=11 path=src/bin/viola-fake-agent/foot.rs
what: edited — the fake foot stand-in terminal, now recording each spawn's pid and argv
@task_file n=12 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited — the fake agent's own mode unit cases
@task_file n=13 path=crates/viola-state/src/liveness.rs
what: edited — instance liveness and heartbeat staleness, now with the bounded boot-time read and beat-before-boot check
@task_file n=14 path=crates/viola-state/src/replay.rs
what: edited — the event log replay and snapshot recovery, now with the bounded end-after-last-start reader
@task_file n=15 path=crates/viola-state/src/board.rs
what: edited — the board scan of instances under the home (a read accessor)
@task_file n=16 path=schemas/diag-line.v1.json
what: edited — the diagnostics line schema, its process-exit detail enum
@task_file n=17 path=tests/cli_output_plain.rs
what: edited — the plain human output and help verb-table literal checks, with the served-verb list
@task_file n=18 path=tests/contract_error_bodies.rs
what: edited — the error-body contract table of named CLI documents and their surface witnesses
@task_file n=19 path=tests/contract_bounded_readers.rs
what: edited — the bounded-readers guard, its boundaries list and exceptions list
@task_file n=20 path=tests/cross_exit_causes.rs
what: edited — the exit-cause table, each cause with its exit code and own hint
@task_file n=21 path=tests/cli_revive.rs
what: edited — the viola revive integration cases, now covering --origin placement and no window step without it
@task_file n=22 path=tests/cli_machine_contract.rs
what: edited — the CLI --json machine contract cases (one document per verb)
@task_file n=23 path=tests/contract_diag_schema.rs
what: edited — the diagnostics schema conformance check
@task_file n=24 path=tests/tui_channel_fds.rs
what: edited — the e2e guard that the claude child holds nothing of viola's, reading the live child's fd table
@task_file n=25 path=tests/support/mod.rs
what: edited — the shared root test helper module list
@task_file n=26 path=tests/cli_open.rs
what: edited — the viola open integration cases, their terminal drop guard handed to the shared teardown
@task_item n=1 part=touches
contract/viola-help/verb-table: the help setup row now reads open, run, revive, up, verify, plugin install, and the served-verb list gained up.
@task_item n=2 part=touches
decision/window-placement-proven-live and decision/window-raise-proven-live: up's recorded-workspace placement and revive's --origin placement are proven over the fakes only; real placement stays gated on R4 (v2-12 gated, unfired).
@task_item n=3 part=touches
decision/live-rig-clean-start and decision/standing-live-rig: a second committed live rig for R4 runs its root outside every repository with the stand-in agent built in its own target dir.
@task_item n=4 part=touches
decision/refusal-rehearsal-reads-own-effects, decision/kr-seams-rehearse-only and decision/live-failing-reading-not-verification: the R4 rig's rehearse form prints a reading that is not a live reading, and its R4b phases refuse first.
@task_item n=5 part=touches
decision/strict-modes-before-any-log, contract/strict-modes-refusal/no-line, contract/strict-modes-refusal/other-verbs and contract/strict-modes-refusal/21: up scans the board strictly before opening its cli log and refuses with exit 21 and no line.
@task_item n=6 part=touches
contract/cli-exit-codes/21 and contract/exit-21-detail/strict-modes-failed: a new verb, up, exits 21 on instance-unreachable or strict-modes-failed from the board scan.
@task_item n=7 part=touches
decision/bounded-readers and decision/bounded-readers-keep-the-cap: the bounded-readers guard gains two boundaries (the boot time read, the log's end after its last start) and one exception (up's own exe read).
@task_item n=8 part=touches
decision/error-bodies-held-by-surface-witnesses and decision/error-writers-each-named: five up documents join the named CLI documents; two are held by existing witnesses by name and boot-time-unread by its unit pair alone.
@task_item n=9 part=touches
decision/hint-names-its-cause and decision/each-exit-cause-own-hint: the exit-cause table gains boot-time-unread at exit 1 with its own hint.
@task_item n=10 part=touches
decision/diagnostics-schema-conformance, decision/jsonschema-validator and contract/process-exit-detail rows: the process-exit detail enum gains boot-time-unread.
@task_item n=11 part=touches
contract/revive-exit-codes, contract/revive-refusal-code rows and decision/revive-preflight-order: revive takes a hidden --origin (closed values cli and agent, conflicts with --list) whose preflight adds a window step from the recorded workspace.
@task_item n=12 part=touches
contract/revive-refusal-code/already-live, stale-heartbeat, no-session, cwd-missing and session-live: up reuses these words as per-instance outcome words.
@task_item n=13 part=touches
decision/heartbeat-staleness-pure-classifier, contract/heartbeat-staleness/age-threshold and contract/instance-liveness rows: up classifies a real snapshot, and on a replayed one the beat alone against the staleness threshold, plus a beat-before-boot check.
@task_item n=14 part=touches
decision/snapshot-read-through-replay, decision/list-reads-live-through-replay and decision/collision-check-over-replay-reads-beat: up reads each instance through read-or-replay and writes state-recovered as list does, without the board read's parse-rejected line.
@task_item n=15 part=touches
decision/single-writers, decision/single-writer-state-files and decision/replay-writes-no-file: up writes no snapshot, stamp or event line.
@task_item n=16 part=touches
decision/cli-json-one-document and contract/cli-json/one-document-per-verb: up --json gives one document of closed words and prints no hint.
@task_item n=17 part=touches
decision/no-sgr-without-tty, contract/cli-human-output/no-sgr, decision/cli-fixed-strings-pinned-by-literal and decision/human-output-pinned-by-literals: up's lines and its not-live hint are plain ASCII literals, one write per line.
@task_item n=18 part=touches
decision/pinned-exe-hash-checked and contract/start-exit-1/pinned-hash-mismatch: up checks the pinned copy before the first spawn and refuses pinned-hash-mismatch at exit 1.
@task_item n=19 part=touches
decision/home-another-user-may-read-refused, decision/widened-home-tree-refused-at-start and contract/start-exit-1/strict-modes-failed: up runs the home-tree check before the first spawn and refuses strict-modes-failed at exit 1.
@task_item n=20 part=touches
decision/wrapper-drop-guard, decision/wrapper-spawned-with-guard and decision/no-untied-viola-spawn: the open and up terminal drop guards hand their roots to one shared teardown in place of their own kill loops.
@task_item n=21 part=touches
decision/pid-start-time-verified-kill: every teardown signal goes only to a recorded pid whose start time still matches.
@task_item n=22 part=touches
decision/killed-test-child-uninstrumented, decision/child-process-coverage and decision/coverage-includes-child-processes: the truncated coverage profile came from SIGKILLing a fake agent already in its own exit; the leaf-first wait closes it.
@task_item n=23 part=touches
decision/profraw-census-dev-host-only: the census measured the teardown fix at 192 x 16 and the old-kill control as a recorded rate.
@task_item n=24 part=touches
decision/one-shared-test-helper and decision/shared-e2e-helpers: the terminal teardown is now one shared root test helper.
@task_item n=25 part=touches
decision/within-7s-wait-bound and decision/root-wait-bound: the shared teardown waits under the one WITHIN deadline before its last-resort kill.
@task_item n=26 part=touches
decision/claude-child-holds-nothing-of-viola, decision/fd-test-reads-live-child-table and decision/no-fd-count-assertion: the fd guard re-reads the child's fd table while a whole table holds a non-fixture home path, inside its 2 s deadline, assertions unchanged.
@task_item n=27 part=touches
decision/external-processes-real-stubs and decision/fake-agent-on-test-path: the fake foot stub now appends each spawn's pid and argv to a spawn record beside itself.
@task_item n=28 part=touches
decision/real-claude-local-only and decision/fake-agent-not-proof-of-real-cli: up and revive --origin are proven over the fakes only; no real claude, compositor or reboot was read.
@task_item n=29 part=touches
decision/agents-json-fallback-unknown and decision/agents-stub-from-files-no-env: up reads the agents listing once for the set; the tests and the rig run the agents stub malformed so a shared session id is not read as session-live.
@task_item n=30 part=touches
decision/fake-agent-own-process-group and decision/fake-agent-children-own-process-group: the teardown records each terminal tree's members and parents from one process table rather than signalling a group.
@task_fact n=1 slug=integration-test-strategy
fact: viola up is a CLI-only verb (no MCP tool, no channel method) whose outcome words are closed — live, already-live, stale-heartbeat, would-open, no-session, cwd-missing, session-live, not-live, not-taken — exit 0 only when every line is live, already-live, stale-heartbeat or would-open, else 1, and 21 on the board's strict refusal.
@task_fact n=2 slug=integration-test-strategy
fact: Named instances narrow viola up to them; a name not scanned reads not-taken, and with none it reads the whole fleet.
@task_fact n=3 slug=integration-test-strategy
fact: viola up selects an instance only when its heartbeat predates the boot and its log has no session-end after its last session-start; live and stale instances are skipped, and candidates spawn parents first, a missing parent a root, a cycle once.
@task_fact n=4 slug=6-e2e-test-strategy
fact: boot-time-unread is an exit-1 cause of viola up with its own hint and process-exit detail word, witnessed by its unit pair only, since a host's boot-time source cannot be hidden from a case.
@task_fact n=5 slug=6-e2e-test-strategy
fact: The boot time read and an instance log's end-after-last-start read are bounded readers.
@task_fact n=6 slug=e2e-test-strategy
fact: viola revive places its window only under its hidden --origin, at the workspace a real snapshot recorded, else unplaced; without the flag a revive runs no window step.
@task_fact n=7 slug=quality-gates-coverage-targets
fact: A stand-in terminal's process tree is ended by one shared teardown: it kills first only a leaf whose recorded parent still runs, waits for every other member under one WITHIN deadline, and kills at the deadline only a pid whose start time still matches.
@task_fact n=8 slug=e2e-test-strategy
fact: The child fd guard re-reads the child's fd table inside its existing 2 s deadline while a whole table holds a non-fixture home path, then asserts the last whole table unchanged.
@task_fact n=9 slug=6-e2e-test-strategy
fact: The R4 live rig refuses its R4b phases first (r4b-not-granted) until granted, and its rehearse form reads windows from the instance's own window record and prints a reading that is not a live reading.
@task_fact n=10 slug=test-data-fixtures
fact: The fake foot stand-in records each spawn's pid and argv as one line of a spawn record beside itself, and still writes its argv file.
@task_fact n=11 slug=new
fact: The name up is occupied as a viola CLI verb.
@task_item n=1 part=untouched
CI workflows, Actions pinning, cargo-deny bans, zizmor, MSRV and the CI artifact set: no workflow or CI job changed and CI read green 7 of 7.
@task_item n=2 part=untouched
Coverage floors, coverage thresholds, mutation gate thresholds and perf budgets: no number moved.
@task_item n=3 part=untouched
Mutation testing, fuzzing and fuzz corpora: no target, corpus or mutants rule changed.
@task_item n=4 part=untouched
MCP tools, tools/list and MCP-CLI parity rows: up is CLI-only and adds no MCP tool.
@task_item n=5 part=untouched
Wrapper channel JSON-RPC errors, link and unlink methods, and channel wait: no channel method changed.
@task_item n=6 part=untouched
send, answer, wait, last and release contracts, dialog decision bodies and the wheel rules: none of these verbs changed.
@task_item n=7 part=untouched
Hook exec-form contracts, hook deadlines and hook perf budgets: no hook path changed.
@task_item n=8 part=untouched
The harness CLI, its session record, status document, boot, cleanup, pre-push and gate contracts: no agent-run command changed.
@task_item n=9 part=untouched
Fixture recording, scrub refusal codes and the recorded Claude fixtures: nothing was recorded and nothing committed under fixtures.
@task_item n=10 part=untouched
Process log line fields and corr rows: no log field changed; the terminal process-start is unchanged in shape.
