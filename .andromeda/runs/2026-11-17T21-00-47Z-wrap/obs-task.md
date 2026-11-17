@task_prose part=session
Added the 23rd relied ledger row, trust-record: viola verify writes a trust record for its own temp dir into the claude config file its child reads, starts a sixth interactive run (Run F, after Run E) and checks that the child opens with no trust modal, then removes only that record (also on a stop, through a guard).
The row's stamp value is pass, fail or the writer's closed code word when nothing was written (14 words: written, removed, config-absent, config-unreadable, config-link, config-not-regular, config-foreign, config-oversize, config-malformed, config-dir-unusable, home-unknown, key-present, lock-held, lost); the stamps gain measured.trust_probe with write and remove; no log event or field was added.
verify now reads one environment value, CLAUDE_CONFIG_DIR, only when the strip keeps that name, else the user home; the writer takes a private lock, retries up to 3 attempts, bounds every read at MAX_FRAME and keeps the file's own mode.
Counts moved: relied rows 22 to 23, verify step counter [NN/23], summary 23 pass 0 fail, verify interactive runs five to six (A to F), verify spawn pairs eight to nine, fake-agent starts per verify six to seven.
The fake agent learned a test-only --trust-config read and a --trust-record-blank hold; every test-started verify now runs under a test-owned user home (a --record start keeps the real HOME and a kept CLAUDE_CONFIG_DIR), held by a new census test; the live rig gained a v arm.
One live S0 on claude 2.1.325 read trust-record fail (7 of 23 pass, confounded): recorded as read, the row stays proven on the fake agent and unverified on the real CLI.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: added — the claude-shapes crate's pure trust-record rules: the config file location, the record key, adding and removing the record in the config document, and the closed trust code words
@task_file n=2 path=src/trust_record.rs
what: added — the CLI's trust-record writer and remover for the claude config file: link, regular-file and owner checks, a private lock, three attempts, bounded read-backs, a private replace keeping the mode
@task_file n=3 path=tests/contract_verify_home.rs
what: added — the census check that every test-started viola verify runs under a test-owned user home, with its planted-copy red case
@task_file n=4 path=crates/viola-agent-claude/src/lib.rs
what: edited — the claude-shapes crate's module list, now exporting the trust module
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the capability ledger's relied rows (now 23, trust-record last), each row's check and the stamp merge writing rows and measured values
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited — the capability ledger's unit cases
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited — the capability ledger's per-row check unit cases, now with the trust-record check
@task_file n=8 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — the ledger stamp-merge unit cases, now with the trust writer's word for a record never written
@task_file n=9 path=src/main.rs
what: edited — the root binary's module list, now with the trust-record writer module
@task_file n=10 path=src/cmd/verify.rs
what: edited — the viola verify command: its inputs, the environment strip it reads, and the trust config file it resolves from the kept CLAUDE_CONFIG_DIR else the user home
@task_file n=11 path=src/cmd/verify/typed.rs
what: edited — viola verify's typed interactive runs and the run results the ledger checks read (trust frame, write and remove words)
@task_file n=12 path=src/cmd/verify/typed/runs.rs
what: edited — viola verify's interactive PTY runs A to F, now with Run F, the trust-record run, and its guard that removes the record
@task_file n=13 path=src/run/version_gate.rs
what: edited — viola run's version gate over the capability ledger stamps (companion edit for the 23rd row)
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: edited — the test-only fake agent: argv-only options, now --trust-config (reads its own record key in the named file) and the --trust-record-blank hold
@task_file n=15 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited — the fake agent's option-parsing unit cases
@task_file n=16 path=tests/support/verify.rs
what: edited — the test support that starts viola verify: now a test-owned user home, a seeded config file and the --trust-config argument for every start
@task_file n=17 path=tests/cli_verify.rs
what: edited — the viola verify integration tests, now with the trust-record pass, fail, config-absent, kept-config-dir and stop cases
@task_file n=18 path=tests/cli_verify/runs.rs
what: edited — the viola verify interactive-run integration cases
@task_file n=19 path=tests/cli_verify/stamps.rs
what: edited — the viola verify stamp-writing integration cases
@task_file n=20 path=tests/contract_ledger_probes.rs
what: edited — the ledger probes contract check over the stamped row count
@task_file n=21 path=tests/contract_bounded_readers.rs
what: edited — the bounded-readers contract check, now listing the trust config readers bounded at MAX_FRAME
@task_file n=22 path=crates/viola-e2e/src/harness/boot.rs
what: edited — the E2E harness boot, whose stamping verify now runs under the session's own user home
@task_file n=23 path=crates/viola-e2e/src/harness/run.rs
what: edited — the E2E harness run steps and its expected verify pass transcript (23 rows)
@task_file n=24 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the live rig's arms and summary keys, now the v arm reading verify-exit, trust-record, pass-count and record-left
@task_file n=25 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's isolation of the child's config and temp root, now reading whether a verify record was left
@task_file n=26 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited — the live rig binary's entry and its child-argument substitution
@task_file n=27 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the live rig's rehearsal tests, now with the v arm rehearsal
@task_item n=1 part=touches
contract/diag-subject/verify-pty-probe: verify's interactive PTY runs move from five to six (A to F); a cell counting the runs is stale.
@task_item n=2 part=touches
contract/verify-spawn-subject/verify-pty-probe: the verify-pty-probe subject now covers six runs, A to F, Run F the trust-record run.
@task_item n=3 part=touches
contract/verify-process-exit/child-exit-status-and-duration: verify's spawn pairs move from eight to nine; a cell holding eight is stale.
@task_item n=4 part=touches
decision/claude-internals-not-a-contract: verify now writes and reads a claude internal, the config file's projects entry with hasTrustDialogAccepted, and relies on it for a ledger row (add-to; unverified on the real CLI 2.1.325).
@task_item n=5 part=touches
contract/verify-stdout/step-line and contract/verify-stdout/summary-line: the step counter reads [NN/23] and the summary 23 pass 0 fail; any cell holding 22 is stale.
@task_item n=6 part=touches
contract/verify-exit-codes/1: a failed trust-record row exits 1 (22 pass 1 fail), and a stop during Run F removes the record and exits 1.
@task_item n=7 part=touches
contract/verify-process-exit/interrupted and contract/verify-stop-stderr/unable-hint-pair: a stop now can land in Run F; the guard removes the record before the exit.
@task_item n=8 part=touches
decision/verify-sole-stamps-writer: the stamp merge now writes rows.trust-record as pass, fail or a trust code word, and measured.trust_probe with write and remove.
@task_item n=9 part=touches
decision/ledger-contents-never-logged: the stamps gain the trust-record row value and trust_probe; still never logged, no path, key or value in any line, stamp or summary.
@task_item n=10 part=touches
decision/unreadable-ledger-is-unverified, decision/unreadable-ledger-means-unverified and contract/run-log-parse-rejected/ledger-stamps: a stamp row value may now be a trust code word, not only pass or fail; readers of the ledger stamps meet a new value.
@task_item n=11 part=touches
decision/verify-lines-redacted and decision/verify-lines-carry-no-content: the trust-record outcome rides the stamp only; verify adds no log line for the record, its path or its key.
@task_item n=12 part=touches
decision/env-names-only, decision/env-vars-not-config-channel and decision/env-vars-not-configuration: verify now reads one environment value, CLAUDE_CONFIG_DIR, only when the strip keeps that name, to locate the config file; it is not logged and sets no configuration of viola.
@task_item n=13 part=touches
decision/claude-env-strip, decision/stripped-claude-env-never-logged, decision/stripped-claude-values-never-logged and decision/r8-strip-logged-as-names-only: the kept CLAUDE_CONFIG_DIR value is read by verify, still never logged.
@task_item n=14 part=touches
decision/uninstrumented-components and decision/claude-boundary-only-instrumentation: the new trust-record writer and remover carry no log line or span; Run F rides the existing verify-pty-probe pair.
@task_item n=15 part=touches
decision/parse-failures-surface-as-unknown, decision/parse-failures-unknown-counter and decision/per-parser-parse-failure-counts: a malformed, oversize or unreadable claude config reads a trust code word on the stamp, with no parse-rejected line and no counter.
@task_item n=16 part=touches
decision/no-retry-policies and decision/no-retry-once: the trust-record writer retries a lost write up to 3 attempts behind a lock, then reads lost.
@task_item n=17 part=touches
decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: CI proves the trust-record row only against the fake agent reading the record; the one real-CLI reading ran on the live rig, not in CI.
@task_item n=18 part=touches
decision/test-homes-under-e2e-home and decision/test-homes-under-scan-root: every test-started verify now runs under a test-owned user home inside the test home; a --record start keeps the real HOME with a kept CLAUDE_CONFIG_DIR.
@task_item n=19 part=touches
decision/os-gated-test-census: the new verify stop case carries no cfg(unix) gate and adds no census row.
@task_item n=20 part=touches
decision/home-level-lines-path-free and decision/no-paths-or-pids-in-detail-codes: the trust code words carry no path, key or pid, and the rig summary keeps none.
@task_fact n=1 slug=4-span-trace-coverage
fact: viola verify's trust-record write and removal emit no log line and no span; the outcome rides only the stamps, as rows.trust-record (pass, fail or one closed trust code word) and measured.trust_probe with write and remove, and no path, key or value of the claude config reaches any line, stamp or summary.
@task_fact n=2 slug=service-identity
fact: viola verify reads one environment value, CLAUDE_CONFIG_DIR, and only when the strip keeps that name, to find the claude config file its child reads; that value is never logged and configures nothing of viola.
@task_item n=1 part=untouched
Panic capture, catch sites, panic lines and the zero-panics gates: no panic path or role changed.
@task_item n=2 part=untouched
The channel, dialog, hook, send, wait, wheel and link contracts and their spans and log fields: no message path changed.
@task_item n=3 part=untouched
The viola mcp server, its stdin frame limits, tool refusals and stdout rules.
@task_item n=4 part=untouched
The process log line schema, its field names, the event enum and corr-by-event rows: no obs_event, event or field was added.
@task_item n=5 part=untouched
Telemetry absences and exporters (OTel, OTLP, Sentry, error reporter, network egress, metrics pipeline): nothing added.
@task_item n=6 part=untouched
Hook and session-end performance budgets, heartbeat liveness and staleness rows.
@task_item n=7 part=untouched
CI secret scan, artifact upload and retention rows, and the PII scrubbing regime of detail files.
