@task_prose part=session
Added the CLI verb viola up: after a reboot it reopens the fleet's pre-boot instances, parents first. Each gets a foot terminal running the pinned viola revive, live and stale ones are skipped, and it prints one line per instance.
viola up is CLI only (no MCP tool, no channel method). It scans the board strictly before any log opens, then logs to the caller's cli file only under a parsed VIOLA_NAME, with span cli.up. It writes one process-start subject terminal per spawned terminal and no process-exit for it, and no snapshot, event line or stamp.
Added a new exit-1 process-exit detail word, boot-time-unread, to the diag-line schema's process-exit detail enum and the exit-cause table. The exit-1 detail of viola up is not-live when any instance reads not-live, else none.
viola revive gains a hidden --origin cli|agent flag. Under it a revive moves its window to the workspace a real snapshot recorded, through the existing run.window span and window line. Without it there is no window step.
Measured: the obs rule and its master row that promise a process-start and process-exit pair around viola open's terminal are disproved. The code writes the process-start alone, and the rule is corrected to the code.
Test-only: one shared teardown for a stand-in terminal's process tree ends the truncated-coverage-profile red. An fd guard now re-reads the child's fd table until it settles within its 2 s deadline. A gated live-rig script (R4) is added, and the fake foot records each spawn.
@task_file n=1 path=src/cmd/up.rs
what: the viola up CLI verb: the reboot fleet reopen, its selection, parents-first order, closed outcome words, exits and process-exit detail
@task_file n=2 path=src/cmd/mod.rs
what: the CLI verb dispatch (Command enum) and its per-verb log and span wiring
@task_file n=3 path=src/cmd/revive.rs
what: the viola revive start arm and its preflight, now with the hidden --origin window step
@task_file n=4 path=src/cmd/open.rs
what: the viola open verb: the foot terminal spawn and its process-start subject terminal line, and the wait for the instance to come up live
@task_file n=5 path=src/human/verbs.rs
what: the human stdout line writers for CLI verbs (the up line and the hint line)
@task_file n=6 path=src/human/hints.rs
what: the fixed stderr unable and hint pairs, now with the boot-time-unread pair and the up not-live hint
@task_file n=7 path=src/human.rs
what: the human output module that re-exports the fixed stderr lines
@task_file n=8 path=src/bin/viola-fake-agent/foot.rs
what: the fake foot terminal of the stand-in agent, which records each spawn
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: the stand-in agent's unit cases for its modes
@task_file n=10 path=crates/viola-state/src/liveness.rs
what: the heartbeat liveness classifier, now with the boot-time read and the beat-before-boot check
@task_file n=11 path=crates/viola-state/src/replay.rs
what: the events-log replay reader, now with the session-ended-after-last-start read
@task_file n=12 path=crates/viola-state/src/board.rs
what: the board scan of the home's instances (a read accessor added)
@task_file n=13 path=schemas/diag-line.v1.json
what: the diag-line v1 schema, whose process-exit detail enum gains boot-time-unread
@task_file n=14 path=tests/contract_diag_schema.rs
what: the diag-line schema conformance check over the detail enum
@task_file n=15 path=tests/cross_exit_causes.rs
what: the exit-cause table check: each exit cause with its code and its distinct detail word
@task_file n=16 path=tests/contract_error_bodies.rs
what: the clean error-body check over every CLI refusal document
@task_file n=17 path=tests/contract_bounded_readers.rs
what: the bounded-readers guard: the list of bounded input boundaries and its exceptions
@task_file n=18 path=tests/cli_output_plain.rs
what: the plain-output check over the help text and its served-verb list
@task_file n=19 path=tests/cli_machine_contract.rs
what: the CLI --json machine-contract check over each verb's documents
@task_file n=20 path=tests/cli_revive.rs
what: the viola revive integration cases, now with the --origin window placement
@task_file n=21 path=tests/cli_up.rs
what: the viola up integration cases over the fakes, each line schema-checked
@task_file n=22 path=tests/cli_open.rs
what: the viola open integration cases and their stand-in terminal guard
@task_file n=23 path=tests/support/terminal.rs
what: the shared test teardown that ends a stand-in terminal's process tree from its leaves
@task_file n=24 path=tests/support/mod.rs
what: the integration-test support module
@task_file n=25 path=tests/tui_channel_fds.rs
what: the e2e fd guard: the claude child holds no viola home file once its launch settles
@task_file n=26 path=scripts/r4-up.sh
what: the gated live rig for viola up's real windows (R4), with a rehearsal form over the fakes
@task_item n=1 part=touches
contract/diag-subject/terminal is STALE: it promises a process-start and process-exit pair around the foot terminal viola open spawns. The code writes the process-start alone, viola up does the same, and the row must be set to say so.
@task_item n=2 part=touches
contract/process-exit-detail/exit-1 gains the detail word boot-time-unread (exit 1, viola up when the host boot time cannot be read). The diag-line schema's process-exit detail enum grows by that one word.
@task_item n=3 part=touches
decision/exit-1-detail-codes and decision/distinct-detail-code-per-exit-cause: a new exit-1 cause, boot-time-unread, takes its own distinct detail word and its own unable and hint pair.
@task_item n=4 part=touches
decision/diag-schema-additive-only, decision/log-fields-additive-only and decision/log-schema-additive-only: the detail enum change is additive, one word, and must stay so.
@task_item n=5 part=touches
The CLI process-log file rows (contract/cli-log-file/open, focus, revive-start, revive-list) and decision/cli-log-only-under-viola-name: a new verb, viola up, logs to the caller's cli file only under a parsed VIOLA_NAME.
@task_item n=6 part=touches
The strict-modes refusal rows (decision/strict-modes-refusal-writes-no-line, decision/pre-open-strict-modes-refusal-silent, decision/strict-modes-failure-lines, contract/strict-modes-refusal-exit/1 and /21, contract/exit-21-detail/strict-modes-failed, contract/cli-json-error/strict-modes-failed): viola up refuses the strict board scan with exit 21 and no line, and a home-tree refusal after the scan with exit 1 strict-modes-failed.
@task_item n=7 part=touches
contract/cli-exit-codes/21 and the exit-21 cause words: viola up joins the verbs exiting 21 on the board's strict refusal (instance-unreachable / strict-modes-failed).
@task_item n=8 part=touches
The exit-1 detail rows for start causes (contract/exit-1-detail/no-session, cwd-missing, session-live, stale-heartbeat, already-live, pinned-hash-mismatch): viola up shows these as per-instance outcome words on its lines. Its own process-exit detail is not-live or none, and only its refusals write their own word (pinned-hash-mismatch, strict-modes-failed, terminal-not-found, boot-time-unread).
@task_item n=9 part=touches
The state-recovered rows (contract/state-recovered/*, contract/state-recovered-detail/*) and decision/no-raw-snapshot-reads: viola up reads each instance through the replay reader and writes state-recovered as viola list does. It never calls the strict board read, so it writes no parse-rejected state-entry line.
@task_item n=10 part=touches
Heartbeat liveness rows (decision/heartbeat-staleness-threshold, decision/heartbeat-liveness, contract/list-liveness/stale-threshold, decision/liveness-transitions-not-logged): viola up classifies liveness with the stale threshold on a real snapshot, and on a replayed one by the beat alone. A new beat-before-boot check reads the heartbeat mtime against the host boot time, and no line is written for it.
@task_item n=11 part=touches
decision/no-endpoint-over-replayed-snapshot and decision/replayed-snapshot-unreachable: viola up takes a replayed snapshot's liveness from the heartbeat alone and never its endpoint.
@task_item n=12 part=touches
contract/child-spawn-log/subject and contract/process-log/child-and-probe-lifecycle: the terminal subject now has a second spawner, viola up, through the shared terminal spawn. It writes a process-start per spawn and no process-exit.
@task_item n=13 part=touches
Span rows (decision/cli-manual-dispatch-span, decision/static-span-names, decision/span-naming): a new manual dispatch span, cli.up.
@task_item n=14 part=touches
Window placement rows (contract/run-span-attributes/run-window, contract/run-start-events/window-line, decision/window-line-untraced): a revive now places a window only under its hidden --origin flag, through the existing run.window span and window line.
@task_item n=15 part=touches
Revive start rows (decision/revive-start-logs-as-run, decision/revive-preflight-order, contract/cli-log-file/revive-start): revive's preflight gains the window ask from --origin, which conflicts with --list. Revive start still logs as run.
@task_item n=16 part=touches
Stream rows (contract/stdout-by-role/cli, decision/short-lived-verbs-stderr, contract/cli-stderr/refusal-pair): viola up writes one ASCII stdout line per instance, or one --json document, and one stderr hint line when any instance reads not-live (none under --json). Its refusals use fixed unable and hint pairs.
@task_item n=17 part=touches
Path-free and lineage rows (decision/detail-codes-no-paths, decision/home-level-lines-path-free, decision/lineage-names-confined): viola up's lines and document carry codes and names only, with no address, pid or workspace. It reads the parent only to order the spawns.
@task_item n=18 part=touches
Home and pin checks before a spawn (decision/home-tree-check-before-start-log, decision/pinned-copy-never-replaced, contract/run-start-failure-detail/pinned-hash-mismatch): viola up checks the home tree and the pinned copy's bytes before its first terminal spawn.
@task_item n=19 part=touches
decision/state-crate-sync-instrumentation and decision/uninstrumented-components: viola-state gains three readers that write no line and open no span: the boot-time read of /proc/stat, the beat-before-boot check and the ended-after-last-start read.
@task_item n=20 part=touches
Agents-probe rows (decision/agents-cli-instrumented-at-boundary, contract/revive-process-log/agents-probe): viola up runs one claude agents read for its whole candidate set before ordering.
@task_item n=21 part=touches
decision/no-sleep-based-log-waits: the new shared terminal teardown and the fd guard's settle wait poll within one deadline with yield_now between reads, not by sleeping.
@task_item n=22 part=touches
decision/diag-lines-schema-checked-in-unit-tests and decision/schema-conformance-gate: the viola up cases schema-check every line they read, and the schema conformance check covers the new detail word.
@task_fact n=1 slug=log-coverage
fact: process-exit detail word boot-time-unread is exit 1 of viola up when the host boot time cannot be read, and comes with its own unable and hint pair.
@task_fact n=2 slug=6-log-coverage
fact: viola up's exit-1 process-exit detail is not-live when any instance reads not-live, else none. An exit 1 from no-session, cwd-missing, session-live or not-taken alone carries no detail word, and each refusal writes its own word: boot-time-unread, strict-modes-failed, pinned-hash-mismatch, terminal-not-found.
@task_fact n=3 slug=span-trace-coverage
fact: viola up logs to the caller's cli process-log file only under a parsed VIOLA_NAME, after its strict board scan. The scan's refusal writes no line.
@task_fact n=4 slug=span-trace-coverage
fact: viola up writes state-recovered per instance as viola list does, and no parse-rejected line with parser state-entry, since it never takes the strict board read.
@task_fact n=5 slug=span-trace-coverage
fact: viola up has a manual dispatch span, cli.up.
@task_fact n=6 slug=span-trace-coverage
fact: viola revive places its window only under the hidden --origin cli|agent flag, at the workspace a real snapshot recorded, else unplaced, through the existing run.window span and window line. Without the flag there is no window step and no window line.
@task_fact n=7 slug=4-span-trace-coverage
fact: the host boot-time read, the beat-before-boot check and the ended-after-last-start read are uninstrumented: no line and no span of their own.
@task_fact n=8 slug=log-coverage
fact: viola up writes no snapshot, event line or stamp. Its only lines are its own process-start and process-exit pair, one process-start subject terminal per spawn, and state-recovered.
@task_item n=1 part=untouched
MCP rows (viola-mcp tools, MCP frame refusals, MCP stdin, MCP stdout): viola up is CLI only and adds no MCP tool.
@task_item n=2 part=untouched
Channel and JSON-RPC rows (channel lines, params, conn and corr joins, -32603): no channel method was added or changed.
@task_item n=3 part=untouched
Hook rows (hook decision lines, fail-open, hook budgets, hook stdout and stderr): no hook path changed.
@task_item n=4 part=untouched
Send, wait, last, wheel and dialog path rows (their log fields, spans and refusal words): none of these paths changed.
@task_item n=5 part=untouched
Panic capture and error-reporting rows (panic line, panic hook, reporter decisions, Sentry and OTel absences): unchanged.
@task_item n=6 part=untouched
PII scrubbing, secret-scan, artifact upload and CI-secret rows: no upload, scan or redaction rule moved.
@task_item n=7 part=untouched
Exporter, OTLP, metrics-pipeline, network-egress and deployment absences: none added or removed.
@task_item n=8 part=untouched
config.json, log-level, log-rotation and service-identity rows: no config key, env var or version source was added.
@task_item n=9 part=untouched
Performance budget and SLO rows (hook perf gates, hyperfine): no budget or threshold moved.
