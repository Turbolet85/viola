@task_prose part=session
The session split three oversized units by pure moves, with no behaviour change: the viola up handler, the PTY pump's respawn path, and viola run's start with its window and trust steps.
The up handler was split into select, plan and act helpers; it keeps the cli.up span and is still up's only callee.
The pump's respawn, reap-within-bound and drain code moved into a new private pump restart module, and the pump loop got one helper, next_request. No per-tick logging was added.
The window-opening step moved from cmd::run into the run window module, and the trust-folder step moved into the trust-record module. Because of that move, the start's trust-record INFO line now carries the tracing target viola::trust_record (it was viola::cmd::run). The event and the detail word are unchanged.
The test modules moved into separate files and keep their module paths. Case counts are unchanged, and no public API, IPC method, endpoint, env var, socket, dependency, schema or gate changed. Nothing was retired.
@task_file n=1 path=src/cmd/up.rs
what: the viola up verb, its handler under the cli.up span and its process-exit on exit 1 from viola up; edited (the handler split into select, plan and act)
@task_file n=2 path=crates/viola-pty/src/pump.rs
what: the viola-pty pump, the PTY seam that copies child output, forwards resizes, writes pastes and ends on the child's exit; edited (respawn code moved out, the loop's next_request helper added)
@task_file n=3 path=crates/viola-pty/src/pump/restart.rs
what: the pump's respawn of the claude child: kill of the old child with a bounded reap, the swap to the new child and the drain of the old workers; new file, moved code
@task_file n=4 path=crates/viola-pty/src/pump/test_cases.rs
what: the pump's unit tests (worker panic kills the child, resize forwarding, drain after exit), moved out of the pump file
@task_file n=5 path=crates/viola-pty/src/pump/test_cases_paste.rs
what: the pump's paste-bound unit tests (paste cut, human keys behind a paste, refused paste), moved
@task_file n=6 path=crates/viola-pty/src/pump/test_cases_respawn.rs
what: the pump's respawn unit tests (swap of the child, a child ignoring the signal, a failed spawn, the drain bound), moved
@task_file n=7 path=src/cmd/run.rs
what: viola run's start, which runs the start spans under run.start: version gate, collision check, pin copy, PTY spawn, window, snapshot write and the channel methods; edited (window and trust steps moved out)
@task_file n=8 path=src/cmd/run/test_cases.rs
what: viola run's unit tests: channel method routing, the release-from-driver refusal line, hook-event checks, the replayed-snapshot collision check and the run.start span capture; moved
@task_file n=9 path=src/cmd/run/test_cases_start.rs
what: viola run's start-state tests: the order of the lineage, launch and window lines in the events log, the snapshot cwd, and run argument parsing; moved
@task_file n=10 path=src/run/window.rs
what: the run window step, which opens the instance's terminal window and records the window line under the run.window span; now also holds the moved open_window step
@task_file n=11 path=src/trust_record.rs
what: the trust record, the start's folder-trust write and removal and the trust-record log line; now also holds the moved trust_folder and trust_word steps
@task_file n=12 path=src/cmd/mod.rs
what: the cli verb module list; the run module is now visible crate-wide (pub(crate))
@task_item n=1 part=touches
The start's trust-record log line (contract/start-trust-log/trust-record, contract/diag-line-fields/trust-record) is now written from the trust-record module. Its tracing target reads viola::trust_record instead of viola::cmd::run. It is still written once per verified start, with the same event and the same detail word.
@task_item n=2 part=touches
The target field rows (contract/log-line/target, contract/process-log/target, contract/process-log-line/target, contract/detail-line/target): the target is the Rust module path, so one emitted line's value changed when it moved. No row pins that value.
@task_item n=3 part=touches
The per-module target filter (decision/unlisted-targets-log-off, decision/viola-crates-log-at-info, decision/flat-log-lines-and-one-level-gate): the moved line's new target sits under the listed viola prefix, so it is still written.
@task_item n=4 part=touches
decision/trust-record-write-uninstrumented: the trust-record write step moved into the trust-record module and is still uninstrumented, with no span added.
@task_item n=5 part=touches
The run start spans and their attributes (contract/run-start-spans/span-chain, contract/run-span-attributes/run-window, contract/run-span-attributes/run-start): the window step moved into the run window module, and its span chain and attributes are unchanged.
@task_item n=6 part=touches
decision/window-line-untraced and decision/window-view-restore-uninstrumented: these rule over the window step, which moved without any change.
@task_item n=7 part=touches
The viola up verb (contract/cli-log-file/up, contract/process-exit-detail/exit-1-up, decision/cli-manual-dispatch-span): the handler was split into three helpers and keeps the cli.up span as before.
@task_item n=8 part=touches
The PTY seam log events (contract/pty-seam-log/kill-fallback, contract/pty-seam-log/spawn, contract/pty-seam-log/paste): the respawn's kill, reap and drain code moved into a new pump restart module. The events it emits did not change.
@task_item n=9 part=touches
The hot-path rules (decision/no-hot-path-instrumentation, decision/no-info-in-hot-loops, decision/pty-byte-stream-never-logged): the pump loop gained the next_request helper, with no logging per tick.
@task_item n=10 part=touches
Worker panic containment (decision/run-worker-panic-ends-run, decision/run-threads-inside-catch-unwind): the pump's worker-panic test moved files and is unchanged.
@task_item n=11 part=touches
decision/release-from-driver-refused and contract/release-refusal/release-from-driver: the test of the refused-and-logged release moved into viola run's test file and is unchanged.
@task_item n=12 part=touches
The start's events-log order (contract/run-start-events/order, contract/run-start-events/window-line): the tests of the lineage, launch and window line order moved and are unchanged.
@task_item n=13 part=touches
decision/run-sync-instrumentation and decision/span-naming-skip-all: these rule over viola run's start code, which moved across modules with byte-equal bodies.
@task_item part=touches n=14
decision/span-names-not-wire-contract and decision/spans-not-wire-contract: the moved steps keep their static span names, and no span was renamed.
@task_item n=1 part=untouched
Hook, MCP and channel rows: their refusals, exit codes, stdout and stderr streams, and corr and conn joins. No IPC method, endpoint, socket or env var changed.
@task_item n=2 part=untouched
The send, wait, dialog and wheel log fields and span chains. None of their paths changed.
@task_item n=3 part=untouched
Metrics, SLO budgets, heartbeat thresholds and perf gates. No count or bound that a row holds moved, and no master states file sizes or cognitive sums.
@task_item n=4 part=untouched
CI gates, the secret scan, artifact uploads and banned dependencies. No dependency, gate or harness surface changed.
@task_item n=5 part=untouched
PII and redaction rows, the detail-file schema, panic capture rows and the config.json keys. No schema, config or redaction changed.
@task_item n=6 part=untouched
The absences (no OTel exporter, no database, no gRPC, no network egress). No capability was added or removed.
@task_item n=7 part=untouched
Cognitive-accessibility rows in other masters. They share the word cognitive only and are not reached.
