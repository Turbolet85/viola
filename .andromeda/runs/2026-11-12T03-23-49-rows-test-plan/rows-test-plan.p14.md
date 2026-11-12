@row_add n=1 row=contract/agent-run-status-polled-fields/name area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: name
promise: A per-item field of the status document that the agent polls.

@row_add n=2 row=contract/agent-run-status-polled-fields/wrapped area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: wrapped
promise: A per-item field of the status document that the agent polls.

@row_add n=3 row=contract/agent-run-status-polled-fields/liveness area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: liveness
promise: A per-item field of the status document that the agent polls.

@row_add n=4 row=contract/agent-run-status-polled-fields/status area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: status
promise: A per-item field of the status document that the agent polls.

@row_add n=5 row=contract/agent-run-status-polled-fields/wheel area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: wheel
promise: A per-item field of the status document that the agent polls.

@row_add n=6 row=contract/agent-run-status-polled-fields/budget-paused area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: budget_paused
promise: A per-item field of the status document that the agent polls.

@row_add n=7 row=contract/agent-run-status-polled-fields/dialog-pending area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: dialog_pending
promise: A per-item field of the status document that the agent polls.

@row_add n=8 row=contract/agent-run-status-polled-fields/cli-verified area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: cli_verified
promise: A per-item field of the status document that the agent polls.

@row_add n=9 row=contract/agent-run-status-polled-fields/skipped area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: skipped
promise: An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.

@row_add n=10 row=contract/agent-run-status-polled-fields/budget area=5-command-implementation rec=1
boundary: agent-run status polled fields
item: budget
promise: An envelope field of the status document that the agent polls.

@row_add n=11 row=contract/agent-run-status-exit-codes/0 area=5-command-implementation rec=2,41
boundary: agent-run status exit codes
item: 0
promise: Exit 0 when ok is true, which is only for state ready.

@row_add n=12 row=contract/agent-run-status-exit-codes/1 area=5-command-implementation rec=2,41
boundary: agent-run status exit codes
item: 1
promise: Exit 1 for degraded or down; the full status document is still printed, so an agent waiting for a stale transition reads exit 1 as a state, not a harness fault.

@row_add n=13 row=contract/agent-run-status-exit-codes/2 area=5-command-implementation rec=2,41
boundary: agent-run status exit codes
item: 2
promise: Exit 2 on a usage error, or when no session.json exists for the id, printing {"v":1,"cmd":"status","ok":false,"reason":"unknown-session"}.

@row_add n=14 row=decision/cleanup-ctrl-c-repeated-until-exit area=5-command-implementation rec=4
name: Cleanup stops agents with a repeated Ctrl-C
rule: Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline.
why: A Ctrl-C written into a child that has not set raw mode yet is swallowed.

@row_add n=15 row=decision/e2e-home-deletion-through-link area=5-command-implementation rec=4,45
name: Test-home deletion through the dev-host link
rule: On the Linux dev host, where target/e2e-home is a link, harness cleanup and a TestHome's drop and owner sweep delete outside the working directory, but only dirs the harness or the test side itself created.
why: A boundary widening, ratified.

@row_add n=16 row=decision/ci-keeps-test-homes area=5-command-implementation rec=4,10,45
name: CI keeps every test home until the gates ran
rule: In CI, AGENT_RUN_KEEP_HOMES=1 is set workflow-wide (only run --mutants overrides it to 0), so every rstest home and every harness session home stays under target/e2e-home until obs-plan's G2, G4, secret scan and scan-gated uploads have run.
why: obs-plan's gate steps and uploads read the homes; the ephemeral runner then discards them with the job.

@row_add n=17 row=decision/cleanup-is-idempotent area=5-command-implementation rec=5
name: Idempotent cleanup
rule: Harness cleanup must be idempotent: with no session record it prints {"v":1,"cmd":"cleanup","ok":true,"cleaned":[]} and exits 0.

@row_add n=18 row=contract/cleanup-document/processes-gone area=5-command-implementation rec=6
boundary: cleanup document
item: processes_gone
promise: True when the supervisor, every wrapper and every child are gone, checked by pid and start time; survivors are force-killed first.

@row_add n=19 row=contract/cleanup-document/endpoint-gone area=5-command-implementation rec=6,4
boundary: cleanup document
item: endpoint_gone
promise: True when the recorded wrapper channel endpoint is gone: a connect is refused or the socket path is absent; a leftover socket file no one listens on after a force-killed wrapper reads true, and cleanup removes nothing of it.

@row_add n=20 row=contract/cleanup-document/home-removed area=5-command-implementation rec=6,4
boundary: cleanup document
item: home_removed
promise: True when the session home was removed; reports "kept" when homes are kept, which is not a failure.

@row_add n=21 row=contract/cleanup-document/killed area=5-command-implementation rec=6
boundary: cleanup document
item: killed
promise: Lists the names force-killed; a force-kill is reported but does not by itself fail cleanup.

@row_add n=22 row=contract/cleanup-exit-codes/1 area=5-command-implementation rec=6
boundary: cleanup exit codes
item: 1
promise: Any false field in the cleanup document means exit 1.

@row_add n=23 row=contract/agent-run-logs-stream/events-line area=5-command-implementation rec=8
boundary: agent-run logs ndjson stream
item: events line
promise: Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.

@row_add n=24 row=contract/agent-run-logs-stream/diag-line area=5-command-implementation rec=8
boundary: agent-run logs ndjson stream
item: diag line
promise: Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.

@row_add n=25 row=contract/agent-run-logs-stream/detail-line area=5-command-implementation rec=8
boundary: agent-run logs ndjson stream
item: detail line
promise: Every content-bearing detail line of an instance's detail files, wrapped as {"src":"diag","file":<basename>,"instance":<name>,"record":<line>}; instance disambiguates the same basename across instances.

@row_add n=26 row=contract/agent-run-logs-stream/torn-line area=5-command-implementation rec=8
boundary: agent-run logs ndjson stream
item: torn line
promise: A torn or unparseable line is emitted as {"src":…,"torn":true,"offset":n} and never dropped.

@row_add n=27 row=decision/panic-counts-read-role-files-only area=5-command-implementation rec=8
name: Panic counts read role files only
rule: Panic counts over the logs stream use the role files only, excluding detail files.
why: A panic line appears both in a role file and in its detail file.

@row_add n=28 row=decision/log-assertions-via-jq-or-jaq area=5-command-implementation rec=8,26
name: Log assertions through jq or jaq
rule: Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.

@row_add n=29 row=decision/session-home-retention area=5-command-implementation rec=10
name: Session home retention
rule: Logs are retained for the whole lifetime of the session home; events are never truncated, and cleanup deletes the home except where homes are kept.

@row_add n=30 row=decision/e2e-homes-under-one-root area=5-command-implementation rec=10
name: All test homes under target/e2e-home
rule: Every harness and rstest home lives under target/e2e-home.
why: It is the root obs-plan's CI gates and upload read.

@row_add n=31 row=decision/ci-uploads-only-after-secret-scan area=5-command-implementation rec=10,14,15
name: CI uploads only after the secret scan
rule: On CI failure the home's diagnostics, events and harness artifacts leave CI only in scan-gated uploads, after the secret scan has passed on them; there is no unscanned upload.

@row_add n=32 row=decision/dev-host-e2e-home-on-tmpfs area=5-command-implementation rec=10,43,45
name: Dev-host test homes on tmpfs
rule: On the Linux dev host target/e2e-home is an operator-set link to an owner-only tmpfs directory, so a kept home lives in memory, does not survive a reboot and ages out of /tmp after ten days untouched; CI is unchanged.

@row_add n=33 row=decision/internal-harness-subcommands-off-agent-surface area=5-command-implementation rec=11
name: Internal harness subcommands
rule: The internal harness subcommands are implemented in viola-harness, forwarded unchanged by the shims, and are not part of the agent's 5-command surface.

@row_add n=34 row=decision/supervisor-spawned-only-by-boot area=5-command-implementation rec=12
name: Supervisor spawned only by boot
rule: The supervise subcommand is spawned only by boot, as an ordinary child process that outlives boot, never by an agent or directly by a test; it owns the outer PTYs and runs the stop steps when a stop request appears.

@row_add n=35 row=contract/supervisor-exit-file/exits area=5-command-implementation rec=12
boundary: supervisor-exit.json
item: exits
promise: One entry per instance: {"name":<instance>,"exit_code":<int|null>,"killed":<bool>}, inside {"v":1,"cmd":"supervise","ok":<bool>,"exits":[…]}.

@row_add n=36 row=contract/supervisor-exit-file/stdout area=5-command-implementation rec=12
boundary: supervisor-exit.json
item: stdout
promise: The supervisor's stdout is discarded; the JSON document is the file.

@row_add n=37 row=contract/supervise-exit-codes/0 area=5-command-implementation rec=12
boundary: supervise exit codes
item: 0
promise: Exit 0 when no child needed kill().

@row_add n=38 row=contract/supervise-exit-codes/1 area=5-command-implementation rec=12
boundary: supervise exit codes
item: 1
promise: Exit 1 when any child needed kill().

@row_add n=39 row=contract/supervise-exit-codes/2 area=5-command-implementation rec=12
boundary: supervise exit codes
item: 2
promise: Exit 2 on a usage error.

@row_add n=40 row=decision/diagnostics-schema-validation area=5-command-implementation rec=13
name: Schema conformance check of diagnostics
rule: Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.

@row_add n=41 row=contract/schema-check-output/failures area=5-command-implementation rec=13
boundary: schema-check output
item: failures
promise: Each failure is {file, line, keyword}, file relative to target/e2e-home; no line content is ever printed.

@row_add n=42 row=contract/schema-check-output/torn area=5-command-implementation rec=13
boundary: schema-check output
item: torn
promise: The count of non-JSON lines.

@row_add n=43 row=contract/schema-check-output/skipped-non-file area=5-command-implementation rec=13
boundary: schema-check output
item: skipped_non_file
promise: The count of directories at a diagnostics file path, which are never read.

@row_add n=44 row=contract/schema-check-reason/empty-scope area=5-command-implementation rec=13,17
boundary: schema-check reason
item: empty-scope
promise: No diagnostics file was found.

@row_add n=45 row=contract/schema-check-reason/schema-unreadable area=5-command-implementation rec=13,17
boundary: schema-check reason
item: schema-unreadable
promise: A schema could not be read.

@row_add n=46 row=contract/schema-check-exit-codes/2 area=5-command-implementation rec=13
boundary: schema-check exit codes
item: 2
promise: Exit 2 on a usage error.

@row_add n=47 row=decision/secret-scan-scope area=5-command-implementation rec=14
name: Secret scan scope
rule: The secret scan covers every diagnostics ndjson under the test homes (detail files included), every file of the harness capture and the nextest CI junit report, and runs in CI before any upload.
why: The run --mutants chunk diff is repository source text that hits the scan's own needles.
except: The run --mutants chunk.diff at the capture root is never scanned and never uploaded; a chunk.diff anywhere else is still scanned.

@row_add n=48 row=decision/canary-list-single-sourced area=5-command-implementation rec=14
name: Single-sourced canary list
rule: Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table.
guard: crates/viola-e2e/tests/scan_patterns.rs

@row_add n=49 row=contract/secret-scan-class/claude-stripped area=5-command-implementation rec=14,17
boundary: secret-scan hit class
item: claude-stripped
promise: A hit of the fixed CLAUDE* canaries the root tests plant.

@row_add n=50 row=contract/secret-scan-class/cookie area=5-command-implementation rec=14,17
boundary: secret-scan hit class
item: cookie
promise: A hit of the cookie: header, case-insensitive.

@row_add n=51 row=contract/secret-scan-class/content-canary area=5-command-implementation rec=14,17
boundary: secret-scan hit class
item: content-canary
promise: A content canary, failing only in a home-level role file.

@row_add n=52 row=contract/secret-scan-class/mode area=5-command-implementation rec=14,17
boundary: secret-scan hit class
item: mode
promise: Unix only: a diagnostics file that is not 0600.

@row_add n=53 row=contract/secret-scan-class/unreadable area=5-command-implementation rec=14,17
boundary: secret-scan hit class
item: unreadable
promise: A file in scope that could not be read.

@row_add n=54 row=contract/secret-scan-output/hits area=5-command-implementation rec=14
boundary: secret-scan output
item: hits
promise: Each hit is {file, line, offset, class}; the matched bytes are never printed.

@row_add n=55 row=contract/secret-scan-output/report area=5-command-implementation rec=14
boundary: secret-scan output
item: report
promise: "report":"unwritten" is added when the hit file cannot be written.

@row_add n=56 row=contract/secret-scan-hits-file/hits-json area=5-command-implementation rec=14
boundary: secret-scan hits file
item: target/secret-scan/hits.json
promise: Holds the same document as stdout, written only when there are hits; a stale one is deleted on every run.

@row_add n=57 row=contract/secret-scan-reason/empty-scope area=5-command-implementation rec=14,17
boundary: secret-scan reason
item: empty-scope
promise: Nothing in scope was found.

@row_add n=58 row=contract/secret-scan-exit-codes/1 area=5-command-implementation rec=14
boundary: secret-scan exit codes
item: 1
promise: Exit 1 on any hit.

@row_add n=59 row=contract/secret-scan-exit-codes/2 area=5-command-implementation rec=14
boundary: secret-scan exit codes
item: 2
promise: Exit 2 on a usage error.

@row_add n=60 row=contract/gate-require/suites area=5-command-implementation rec=15,17
boundary: gate --require
item: suites
promise: Takes only the eight closed suite values other than local-live; an empty list or any other value (a11y included) is usage unknown-suite.

@row_add n=61 row=decision/junit-copied-after-each-invocation area=5-command-implementation rec=15
name: JUnit report copied after each nextest invocation
rule: The run command deletes the nextest junit report before each invocation and copies it per suite right after; a report missing afterwards is an artifact-missing breach, never a copy of an earlier run's file.
why: nextest overwrites that path on every invocation.

@row_add n=62 row=decision/mutants-survived-count area=5-command-implementation rec=15
name: Mutants survived count
rule: The mutants suite's survived count is missed minus host-excluded plus timeout, from the run's own cargo-mutants outcomes.

@row_add n=63 row=decision/missing-artifact-is-breach area=5-command-implementation rec=15
name: A missing artifact is a breach
rule: A gate input that is missing or unreadable is a breach, never a pass.

@row_add n=64 row=decision/perf-gate-spine-deadline area=5-command-implementation rec=15,27
name: Hook perf gate at the spine deadline
rule: The five hook perf rows (session-start, user-prompt-submit, stop, session-end, pre-tool-use) are required by name, and every present hyperfine result's max must be below the product's 1.0 s spine deadline constant.

@row_add n=65 row=contract/gate-breach/suite-missing area=5-command-implementation rec=15
boundary: gate breach
item: suite-missing
promise: A required suite is absent from the run summary.

@row_add n=66 row=contract/gate-breach/suite-failed area=5-command-implementation rec=15
boundary: gate breach
item: suite-failed
promise: A required suite has failed > 0.

@row_add n=67 row=contract/gate-breach/suite-skipped area=5-command-implementation rec=15
boundary: gate breach
item: suite-skipped
promise: A required suite has skipped > 0.

@row_add n=68 row=contract/gate-breach/coverage area=5-command-implementation rec=15
boundary: gate breach
item: coverage
promise: Coverage below a floor, each floor its own breach.

@row_add n=69 row=contract/gate-breach/mutants area=5-command-implementation rec=15
boundary: gate breach
item: mutants
promise: The mutants suite has survived > 0.

@row_add n=70 row=contract/gate-breach/perf area=5-command-implementation rec=15
boundary: gate breach
item: perf
promise: A perf result's max is at or over its gate; detail reads {file} max {max} >= {bound}.

@row_add n=71 row=contract/gate-breach/artifact-missing area=5-command-implementation rec=15
boundary: gate breach
item: artifact-missing
promise: A required artifact is missing, each absent perf row its own breach naming that file.

@row_add n=72 row=contract/gate-output/detail area=5-command-implementation rec=15
boundary: gate output
item: detail
promise: A fixed code or a count, never file content.

@row_add n=73 row=contract/gate-exit-codes/0 area=5-command-implementation rec=15
boundary: gate exit codes
item: 0
promise: Exit 0 with no breaches.

@row_add n=74 row=contract/gate-exit-codes/1 area=5-command-implementation rec=15
boundary: gate exit codes
item: 1
promise: Exit 1 on any breach.

@row_add n=75 row=contract/gate-exit-codes/2 area=5-command-implementation rec=15
boundary: gate exit codes
item: 2
promise: Exit 2 on a usage error.

@row_add n=76 row=decision/ci-gate-last-step area=5-command-implementation rec=15,27
name: Gate as the last step of every test job
rule: CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.

@row_add n=77 row=decision/no-ci-mutants-gate area=5-command-implementation rec=15
name: No CI mutants gate
rule: No CI job gates the mutants suite.

@row_add n=78 row=decision/pre-push-local-linux-gate area=5-command-implementation rec=16
name: pre-push local Linux gate
rule: The operator pass runs pre-push, Linux host only, on the uncommitted working tree before the pre-CI commit, and a red stage stops the pass; pre-push is not an agent command.

@row_add n=79 row=decision/pre-push-clean-environment area=5-command-implementation rec=16
name: pre-push children run in a clean environment
rule: Every pre-push child runs under env -i with HOME from the passwd entry and a PATH of constants only, inheriting no host environment; the harness's own HOME is never read.
why: No CLAUDE* value crosses.

@row_add n=80 row=contract/pre-push-stage/host area=5-command-implementation rec=16,17
boundary: pre-push stage
item: host
promise: Off Linux, exit 2 with pre-push-linux-only.

@row_add n=81 row=contract/pre-push-stage/tools area=5-command-implementation rec=16,17
boundary: pre-push stage
item: tools
promise: Checks the passwd home, cc, and rustc, cargo-nextest, cargo-mutants and cargo-llvm-cov at their pinned versions.

@row_add n=82 row=contract/pre-push-stage/linux-tests area=5-command-implementation rec=16,17
boundary: pre-push stage
item: linux-tests
promise: Runs run --coverage then gate --require coverage,doctest; the last stage, ok:true when green.

@row_add n=83 row=decision/pre-push-runs-no-mutation area=5-command-implementation rec=16
name: pre-push runs no mutation
rule: No pre-push stage runs mutation testing.
why: The cargo-mutants pin stays in the tools stage because run --coverage runs the real-cargo-mutants harness tests.

@row_add n=84 row=contract/pre-push-output/paths area=5-command-implementation rec=16
boundary: pre-push output
item: paths
promise: Codes, counts and repo-relative names only, never a home or repository path.

@row_add n=85 row=contract/pre-push-exit-codes/0 area=5-command-implementation rec=16
boundary: pre-push exit codes
item: 0
promise: Exit 0 when every stage is green.

@row_add n=86 row=contract/pre-push-exit-codes/1 area=5-command-implementation rec=16
boundary: pre-push exit codes
item: 1
promise: Exit 1 on a red stage or a stop reason.

@row_add n=87 row=contract/pre-push-exit-codes/2 area=5-command-implementation rec=16
boundary: pre-push exit codes
item: 2
promise: Exit 2 on pre-push-linux-only or a usage error.

@row_add n=88 row=decision/closed-enums-need-decisions-log area=5-command-implementation rec=17,34
name: Closed enums change only by decision
rule: A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.

@row_add n=89 row=contract/run-suite/local-live area=5-command-implementation rec=17
boundary: run suite
item: local-live
promise: Appears only under --local-live and is not accepted by gate --require.

@row_add n=90 row=contract/local-live-failure/build area=5-command-implementation rec=17
boundary: local-live failure code
item: build
promise: A closed local-live failure code.

@row_add n=91 row=contract/local-live-failure/verify-exit-n area=5-command-implementation rec=17
boundary: local-live failure code
item: verify-exit-<n>
promise: A closed local-live failure code.

@row_add n=92 row=contract/local-live-failure/verify-exit-none area=5-command-implementation rec=17
boundary: local-live failure code
item: verify-exit-none
promise: A closed local-live failure code.

@row_add n=93 row=contract/local-live-failure/row-missing area=5-command-implementation rec=17
boundary: local-live failure code
item: row-missing
promise: A closed local-live failure code.

@row_add n=94 row=contract/boot-readiness-missing/run-process-start area=5-command-implementation rec=17
boundary: boot readiness missing code
item: <name>:run-process-start
promise: The run process start record of an instance is missing.

@row_add n=95 row=contract/boot-readiness-missing/child-process-start area=5-command-implementation rec=17
boundary: boot readiness missing code
item: <name>:child-process-start
promise: The child process start record of an instance is missing.

@row_add n=96 row=contract/boot-readiness-missing/snapshot area=5-command-implementation rec=17
boundary: boot readiness missing code
item: <name>:snapshot
promise: The snapshot of an instance is missing.

@row_add n=97 row=contract/boot-readiness-missing/endpoint area=5-command-implementation rec=17
boundary: boot readiness missing code
item: <name>:endpoint
promise: The endpoint of an instance is missing.

@row_add n=98 row=contract/boot-readiness-missing/heartbeat area=5-command-implementation rec=17
boundary: boot readiness missing code
item: <name>:heartbeat
promise: The heartbeat of an instance is missing.

@row_add n=99 row=contract/boot-readiness-missing/events area=5-command-implementation rec=17
boundary: boot readiness missing code
item: <name>:events
promise: The events of an instance are missing.

@row_add n=100 row=contract/status-last-error/null area=5-command-implementation rec=17,40
boundary: status last_error
item: null
promise: No error.

@row_add n=101 row=contract/status-last-error/list-exit-code area=5-command-implementation rec=17,40,41
boundary: status last_error
item: list-exit-<code>
promise: viola list exited with that code (for example list-exit-21); list-exit-0 when it exited 0 with no JSON object on stdout.

@row_add n=102 row=contract/status-last-error/list-exit-none area=5-command-implementation rec=17,41
boundary: status last_error
item: list-exit-none
promise: viola list gave no exit code, could not be spawned, or had no supervise record to run from.

@row_add n=103 row=contract/mutants-verdict/counted area=5-command-implementation rec=17
boundary: run --mutants verdict
item: counted
promise: A closed mutants.verdict value.

@row_add n=104 row=contract/mutants-verdict/scoped area=5-command-implementation rec=17
boundary: run --mutants verdict
item: scoped
promise: The run was scoped by --file.

@row_add n=105 row=contract/mutants-verdict/no-rust-delta area=5-command-implementation rec=17
boundary: run --mutants verdict
item: no-rust-delta
promise: A closed mutants.verdict value.

@row_add n=106 row=contract/mutants-verdict/test-only-rust-delta area=5-command-implementation rec=17
boundary: run --mutants verdict
item: test-only-rust-delta
promise: A closed mutants.verdict value.

@row_add n=107 row=contract/mutants-verdict/package area=5-command-implementation rec=17
boundary: run --mutants verdict
item: package
promise: The run was scoped by --package.

@row_add n=108 row=contract/run-reason/fuzz-linux-only area=5-command-implementation rec=17
boundary: run reason
item: fuzz-linux-only
promise: Exit 2.

@row_add n=109 row=contract/run-reason/tool-missing area=5-command-implementation rec=17,26
boundary: run reason
item: tool-missing
promise: run --perf and --fuzz-replay check their tool is on PATH first; when missing they exit 1 with this reason and the tool name, never a silent pass.

@row_add n=110 row=contract/run-reason/corpus-empty area=5-command-implementation rec=17
boundary: run reason
item: corpus-empty
promise: A closed run reason.

@row_add n=111 row=contract/run-reason/live-in-ci area=5-command-implementation rec=17
boundary: run reason
item: live-in-ci
promise: Exit 2.

@row_add n=112 row=contract/run-reason/package-refused area=5-command-implementation rec=17
boundary: run reason
item: package-refused
promise: Exit 2.

@row_add n=113 row=contract/run-usage-detail/unknown-suite area=5-command-implementation rec=17
boundary: run usage detail
item: unknown-suite
promise: The closed usage detail for a suite value outside the closed list.

@row_add n=114 row=contract/pre-push-reason/pre-push-linux-only area=5-command-implementation rec=17
boundary: pre-push reason
item: pre-push-linux-only
promise: Exit 2.

@row_add n=115 row=contract/pre-push-reason/tool-missing area=5-command-implementation rec=17
boundary: pre-push reason
item: tool-missing
promise: Reused from run; its detail names the missing tool or passwd-home.

@row_add n=116 row=contract/pre-push-reason/tool-pin-mismatch area=5-command-implementation rec=17
boundary: pre-push reason
item: tool-pin-mismatch
promise: A closed pre-push reason.

@row_add n=117 row=contract/pre-push-reason/linux-document-unreadable area=5-command-implementation rec=17
boundary: pre-push reason
item: linux-document-unreadable
promise: The detail, run or gate, names the harness command whose document was unreadable.

@row_add n=118 row=contract/pre-push-detail/passwd-home area=5-command-implementation rec=17
boundary: pre-push detail
item: passwd-home
promise: Detail of tool-missing.

@row_add n=119 row=contract/pre-push-detail/cc area=5-command-implementation rec=17
boundary: pre-push detail
item: cc
promise: A closed pre-push detail.

@row_add n=120 row=contract/pre-push-detail/rustc area=5-command-implementation rec=17
boundary: pre-push detail
item: rustc
promise: A closed pre-push detail.

@row_add n=121 row=contract/pre-push-detail/cargo-nextest area=5-command-implementation rec=17
boundary: pre-push detail
item: cargo-nextest
promise: A closed pre-push detail.

@row_add n=122 row=contract/pre-push-detail/cargo-mutants area=5-command-implementation rec=17
boundary: pre-push detail
item: cargo-mutants
promise: A closed pre-push detail.

@row_add n=123 row=contract/pre-push-detail/cargo-llvm-cov area=5-command-implementation rec=17
boundary: pre-push detail
item: cargo-llvm-cov
promise: A closed pre-push detail.

@row_add n=124 row=contract/pre-push-detail/pins-unreadable area=5-command-implementation rec=17
boundary: pre-push detail
item: pins-unreadable
promise: A closed pre-push detail.

@row_add n=125 row=contract/pre-push-detail/run area=5-command-implementation rec=17
boundary: pre-push detail
item: run
promise: The run document was unreadable.

@row_add n=126 row=contract/pre-push-detail/gate area=5-command-implementation rec=17
boundary: pre-push detail
item: gate
promise: The gate document was unreadable.

@row_add n=127 row=decision/nextest-test-runner area=bootstrap-phases-derive-for-route-setup-project rec=19
name: cargo-nextest as the test runner
rule: cargo-nextest 0.9.146 is the test runner, installed SHA-pinned through taiki-e/install-action in CI or with cargo install --locked locally.

@row_add n=128 row=decision/ci-profile-no-retries area=bootstrap-phases-derive-for-route-setup-project rec=19,27
name: CI test profile without retries
rule: The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).

@row_add n=129 row=decision/verify-window-60s-kill area=bootstrap-phases-derive-for-route-setup-project rec=19
name: 60 s kill for the verify_window_ class
rule: Tests named verify_window_ take a 60 s per-test kill, in both the ci and mutants profiles, through an override that stands first.
why: Their cases wait out the gate's 8.5 s maximum several times by design.
guard: mutants_verify_window_override_stands_before_the_harness_package_override

@row_add n=130 row=decision/verify-driven-tests-20s-kill area=bootstrap-phases-derive-for-route-setup-project rec=19
name: Per-test kill bounds verify-driven tests
rule: A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound.
why: Ratified; sized from the measured verify floor plus a 3x runner tail.

@row_add n=131 row=decision/within-7s-wait-bound area=bootstrap-phases-derive-for-route-setup-project rec=19
name: WITHIN bound for every other wait
rule: WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.

@row_add n=132 row=decision/mutants-fail-fast-wait area=bootstrap-phases-derive-for-route-setup-project rec=19
name: Mutants profile stops after one failure and waits
rule: The mutants nextest profile stops scheduling at the first failure and lets the tests already running finish (terminate = wait).
why: Running tests then drop their temp dirs and session guards; immediate termination left leftover temp dirs.

@row_add n=133 row=decision/mutants-kill-below-cargo-mutants-timeout area=bootstrap-phases-derive-for-route-setup-project rec=19
name: Kill lines below cargo-mutants' timeout
rule: Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout.
why: At or above that floor a hang grades Timeout instead of caught.
guard: test_deadlines_sit_below_the_nextest_kill_line

@row_add n=134 row=decision/viola-e2e-30s-mutants-kill area=bootstrap-phases-derive-for-route-setup-project rec=19
name: 30 s mutants kill for the harness package
rule: Under the mutants profile the viola-e2e package takes a 30 s per-test kill.
why: The harness's own tests wait out its 20 s boot deadline by design.

@row_add n=135 row=decision/send-window-paste-hint-30s-kill area=bootstrap-phases-derive-for-route-setup-project rec=19
name: 30 s mutants kill for send-window classes
rule: Under the mutants profile the send_window_ and send_under_the_paste_hint_ test classes take a 30 s per-test kill.
why: The paste-hint cases wait out the product's 8.5 s gate maximum under a held paste hint, within a second of the 10 s default kill.
guard: mutants_paste_hint_override_holds_the_class_above_the_default_kill

@row_add n=136 row=decision/fixture-git-no-auto-maintenance area=bootstrap-phases-derive-for-route-setup-project rec=19
name: Fixture git repos without auto maintenance
rule: Fixture git repositories in tests run git with maintenance.auto=false.
why: The detached auto maintenance each fixture commit spawned wrote into .git while the temp dir's drop walked it, leaving half-removed dirs.

@row_add n=137 row=decision/rmcp-client-only area=bootstrap-phases-derive-for-route-setup-project rec=19
name: rmcp client only in the harness
rule: The harness uses rmcp with the client feature only, without transport-child-process.
why: The test owns the viola mcp child process.

@row_add n=138 row=decision/syn-cfg-reader-for-host-exclusion area=bootstrap-phases-derive-for-route-setup-project rec=19
name: syn as the cfg reader
rule: syn and proc-macro2 are ordinary harness dependencies that read #[cfg] attributes for run --mutants' host exclusion.

@row_add n=139 row=decision/agent-run-shim-over-harness area=bootstrap-phases-derive-for-route-setup-project rec=20,11
name: agent-run shim over viola-harness
rule: The agent-run script is a shim over viola-harness for the five commands boot, run, status, cleanup and logs.

@row_add n=140 row=decision/status-shape-ownership area=bootstrap-phases-derive-for-route-setup-project rec=21,29,30,31
name: Ownership of the status shape
rule: The test plan is the binding source for the agent-run status shape; the nested product shapes inside it, viola list --json, come from the arch CLI contract; obs-plan reads it and may extend but not redefine it.

@row_add n=141 row=decision/harness-log-fields-owned-by-test-plan area=bootstrap-phases-derive-for-route-setup-project rec=22,29,31,34
name: Ownership of the harness-grepped log fields
rule: The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first.
why: The harness greps logs for assertions, so a format break is a harness break.

@row_add n=142 row=decision/cargo-llvm-cov-coverage area=bootstrap-phases-derive-for-route-setup-project rec=25
name: cargo-llvm-cov for coverage
rule: cargo-llvm-cov 0.9.1, installed through taiki-e/install-action, produces coverage as lcov and a JSON summary.

@row_add n=143 row=decision/env-clear-keeps-llvm-profile-file area=bootstrap-phases-derive-for-route-setup-project rec=25
name: env_clear tests re-add LLVM_PROFILE_FILE
rule: Any test that clears the child environment must re-add LLVM_PROFILE_FILE.

@row_add n=144 row=decision/ci-tools-version-pinned area=bootstrap-phases-derive-for-route-setup-project rec=26
name: Version-pinned CI tools
rule: Every CI-invoked binary is version-pinned.

@row_add n=145 row=decision/g2-zero-panics area=bootstrap-phases-derive-for-route-setup-project rec=26
name: G2 zero panics
rule: CI's log gate G2 fails closed on any event panic role line under the test homes, run as a probe then the check in the test and perf jobs.
except: A panic_location of exactly src/cmd/hook/seam.rs:<digits>.

@row_add n=146 row=decision/runner-provided-jq area=bootstrap-phases-derive-for-route-setup-project rec=26
name: Runner-provided jq in CI scripts
rule: CI scripts use the runner-provided jq, presence-checked and never installed, and refuse with tool-missing: jq without it.

@row_add n=147 row=decision/ripgrep-verified-install area=bootstrap-phases-derive-for-route-setup-project rec=26
name: ripgrep for G1 and G3, checksum-verified
rule: ripgrep 15.2.0 with PCRE2 is the G1/G3 gate tool, installed by a script that downloads the official release asset and checks it against its published sha256.
why: taiki-e/install-action has no ripgrep manifest.

@row_add n=148 row=decision/coverage-floors area=bootstrap-phases-derive-for-route-setup-project rec=27,15
name: Coverage floors
rule: CI enforces coverage floors of lines 85, functions 95 and regions 80, each floor its own breach.

@row_add n=149 row=decision/ci-actions-sha-pinned area=bootstrap-phases-derive-for-route-setup-project rec=27
name: SHA-pinned actions, no default permissions
rule: Every CI action is SHA-pinned, with permissions: {} at the workflow's top level.

@row_add n=150 row=decision/ci-exit-code-gates area=bootstrap-phases-derive-for-route-setup-project rec=27
name: CI exit-code gates
rule: CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.

@row_add n=151 row=contract/log-format/json-per-line area=log-format rec=33,36
boundary: log format
item: line format
promise: JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.

@row_add n=152 row=contract/log-streams/event-stream area=log-format rec=33
boundary: log streams
item: event stream
promise: Product events in each instance's events.ndjson, exactly the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}.

@row_add n=153 row=decision/process-logs-via-tracing-subscriber area=log-format rec=33
name: tracing-subscriber JSON process logs
rule: Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.

@row_add n=154 row=decision/codes-only-role-logs area=log-format rec=33
name: Codes-only role logs, detail apart
rule: Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.

@row_add n=155 row=contract/log-files/modes area=log-format rec=33
boundary: log files
item: modes and writes
promise: Files are 0600, dirs 0700, with one write per line.

@row_add n=156 row=contract/log-line/timestamp area=log-format rec=34
boundary: process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.

@row_add n=157 row=contract/log-line/level area=log-format rec=34
boundary: process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.

@row_add n=158 row=contract/log-line/target area=log-format rec=34
boundary: process log line
item: target
promise: A required field.

@row_add n=159 row=contract/log-line/message area=log-format rec=34
boundary: process log line
item: message
promise: fields.message, or the flattened message; required.

@row_add n=160 row=contract/log-line/event area=log-format rec=34
boundary: process log line
item: event
promise: A closed kebab-case enum; each value fixes what its corr holds.

@row_add n=161 row=contract/log-line/process area=log-format rec=34
boundary: process log line
item: process
promise: One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.

@row_add n=162 row=contract/log-line/instance area=log-format rec=34
boundary: process log line
item: instance
promise: A ViolaName or null.

@row_add n=163 row=contract/log-line/corr area=log-format rec=34
boundary: process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).

@row_add n=164 row=contract/log-line/null-encoding area=log-format rec=34
boundary: process log line
item: null encoding
promise: A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.

@row_add n=165 row=contract/log-event-corr/channel-request area=log-format rec=34
boundary: log event corr
item: channel-request
promise: corr is the JSON-RPC request id; null for an id-less hook.event notification.

@row_add n=166 row=contract/log-event-corr/channel-response area=log-format rec=34
boundary: log event corr
item: channel-response
promise: corr is the JSON-RPC request id; null on a response to a frame that yielded no id (error_code -32700 or -32600).

@row_add n=167 row=contract/log-event-corr/dialog-raised area=log-format rec=34
boundary: log event corr
item: dialog-raised
promise: corr is the dialog_id.

@row_add n=168 row=contract/log-event-corr/dialog-answered area=log-format rec=34
boundary: log event corr
item: dialog-answered
promise: corr is the dialog_id.

@row_add n=169 row=contract/log-event-corr/hook-invoked area=log-format rec=34
boundary: log event corr
item: hook-invoked
promise: corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked may still be null when it fails open before the dialog_id is known.

@row_add n=170 row=contract/log-event-corr/hook-decision area=log-format rec=34
boundary: log event corr
item: hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null; a hook-decision that carries detail may be null.

@row_add n=171 row=contract/log-event-corr/send-issued area=log-format rec=34
boundary: log event corr
item: send-issued
promise: corr is the send cursor.

@row_add n=172 row=contract/log-event-corr/send-confirmed area=log-format rec=34
boundary: log event corr
item: send-confirmed
promise: corr is the send cursor.

@row_add n=173 row=contract/log-event-corr/send-refused area=log-format rec=34
boundary: log event corr
item: send-refused
promise: corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.

@row_add n=174 row=contract/log-line/conn-and-rpc-id area=log-format rec=34
boundary: process log line
item: conn, rpc_id
promise: Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines.

@row_add n=175 row=contract/log-event-corr/release-from-driver area=log-format rec=34
boundary: log event corr
item: release-from-driver
promise: corr is the JSON-RPC request id of a release the wrapper refused with -32602 for a string from; a from of another type is a plain -32602 with no such line.

@row_add n=176 row=contract/log-event-corr/process-start area=log-format rec=34
boundary: log event corr
item: process-start
promise: corr is null.

@row_add n=177 row=contract/log-event-corr/process-exit area=log-format rec=34
boundary: log event corr
item: process-exit
promise: corr is null.

@row_add n=178 row=contract/log-event-corr/panic area=log-format rec=34
boundary: log event corr
item: panic
promise: corr is null.

@row_add n=179 row=contract/log-event-corr/state-recovered area=log-format rec=34
boundary: log event corr
item: state-recovered
promise: corr is null.

@row_add n=180 row=contract/log-event-corr/parse-rejected area=log-format rec=34
boundary: log event corr
item: parse-rejected
promise: corr is null.

@row_add n=181 row=decision/diag-line-schema-requires-corr area=log-format rec=34
name: Schema requires corr where always defined
rule: The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.

@row_add n=182 row=decision/hook-trace-write-failure-swallowed area=log-format rec=35
name: Hook trace write failures swallowed
rule: A failed write to the hook trace file is swallowed, so the hook still exits 0 and never writes to stderr.

@row_add n=183 row=decision/no-secrets-in-logs area=log-format rec=35
name: No secrets in log lines
rule: No log line may contain a Cookie header or any stripped CLAUDE* value.
guard: the secret-scan test

@row_add n=184 row=contract/log-line/hook-panic area=log-format rec=36
boundary: process log line
item: hook panic
promise: A panic in a hook is caught and logged as one level ERROR, event panic line.

@row_add n=185 row=decision/no-product-pid-file area=pid-file rec=37
name: No product pid file
rule: The product has no pid file: product pids live in each instance's snapshot with liveness in its heartbeat, and the harness's own process record is its per-session session.json.

@row_add n=186 row=contract/session-json/supervisor-pid area=pid-file rec=37
boundary: harness session.json
item: supervisor_pid
promise: The supervisor's pid.

@row_add n=187 row=contract/session-json/wrapper-pid area=pid-file rec=37
boundary: harness session.json
item: wrapper_pid
promise: The wrapper's pid.

@row_add n=188 row=contract/session-json/lifecycle area=pid-file rec=38,23
boundary: harness session.json
item: lifecycle
promise: Written by boot once readiness passes, read by status, logs and cleanup, removed by cleanup.

@row_add n=189 row=decision/pid-start-time-verified-kill area=pid-file rec=38,23
name: Pid and start time verified before a kill
rule: Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling.
why: A reused pid is never killed.

@row_add n=190 row=contract/status-document/list area=status-endpoint-shape rec=39,40,41
boundary: agent-run status document
item: list
promise: The parsed viola list --json document verbatim, the exit-21 error document included; null when list exits 0 with no JSON object on stdout.

@row_add n=191 row=contract/status-document/ok area=status-endpoint-shape rec=40,41
boundary: agent-run status document
item: ok
promise: True only for state ready.

@row_add n=192 row=contract/status-document/pid area=status-endpoint-shape rec=40,41
boundary: agent-run status document
item: pid
promise: Stays null.

@row_add n=193 row=contract/status-document/uptime-ms area=status-endpoint-shape rec=40,41
boundary: agent-run status document
item: uptime_ms
promise: Stays null.

@row_add n=194 row=contract/status-document/instances area=status-endpoint-shape rec=40,41
boundary: agent-run status document
item: instances
promise: One entry per booted instance: name, wrapper_pid, and alive by pid and start-time liveness.

@row_add n=195 row=contract/status-state/ready area=status-endpoint-shape rec=41,40
boundary: agent-run status state
item: ready
promise: viola list exits 0 and every booted instance is an item wrapped:true, liveness:"live"; an unwrapped item is ignored, and a session that booted none reads ready.

@row_add n=196 row=contract/status-state/degraded area=status-endpoint-shape rec=41,40
boundary: agent-run status state
item: degraded
promise: A booted instance is stale or absent from the list, or the document holds no items.

@row_add n=197 row=contract/status-state/down area=status-endpoint-shape rec=41,40
boundary: agent-run status state
item: down
promise: viola list exits non-zero, gives no exit code, cannot be spawned, has no supervise record to run from, or exits 0 with no JSON object on stdout.

@row_add n=198 row=decision/self-bootstrapping-test-data area=test-data-bootstrap rec=42
name: Self-bootstrapping test data
rule: Test data is self-bootstrapped; no developer-seeded data.

@row_add n=199 row=decision/fresh-home-per-test area=test-data-bootstrap rec=43,44
name: A fresh temp home per test
rule: Every test or harness session gets a fresh tempfile 3.27.0 home, so no endpoint, log or budget file is shared; the endpoint hash includes the absolute home path.

@row_add n=200 row=decision/rstest-fixture-chain area=test-data-bootstrap rec=43
name: rstest fixture chain
rule: Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.

@row_add n=201 row=decision/fake-agent-replay-for-stamps area=test-data-bootstrap rec=43
name: Fake-agent replay for stamps
rule: Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.

@row_add n=202 row=decision/proptest-committed-seeds area=test-data-bootstrap rec=43
name: proptest with committed seeds
rule: Randomized inputs come from proptest 1.11.0 strategies with committed regression seeds.

@row_add n=203 row=decision/chaos-home-outside-scan area=test-data-bootstrap rec=43,45
name: One chaos home outside the scans
rule: Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design.
why: A ratified carve-out.

@row_add n=204 row=decision/e2e-home-base-checks area=test-data-bootstrap rec=43
name: Checked base of the test homes
rule: The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.

@row_add n=205 row=decision/e2e-home-headroom-floor area=test-data-bootstrap rec=43
name: Headroom floor of a linked test-home base
rule: A linked test-home base whose filesystem has less than 9 GiB free is refused; the floor is one compiled constant shared with the harness keeper, set by no variable, flag or config key; a plain base reads no headroom.
why: The tmpfs's quota reading of about 8 GiB free plus 1 GiB.

@row_add n=206 row=decision/test-home-owner-record-sweep area=test-data-bootstrap rec=45
name: Owner-record sweep of test homes
rule: Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record.
why: A test the runner kills never drops its home.

@row_add n=207 row=decision/owner-record-removed-last area=test-data-bootstrap rec=45
name: Owner record removed last
rule: Removing a test home deletes its owner record last.
why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.

@row_add n=208 row=decision/keep-failed-home-opt-in area=test-data-bootstrap rec=45
name: Failed test homes kept only on request
rule: A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1.
why: So that logs can inspect the home.

@row_none n=1 rec=3 type=S

@row_none n=2 rec=7 type=S

@row_none n=3 rec=9 type=S

@row_none n=4 rec=18 type=S

@row_none n=5 rec=24 type=5

@row_none n=6 rec=28 type=S

@row_none n=7 rec=32 type=S
