# obs-plan · the rows that bind 2026-11-15-mutation-and-coverage-baselines-green — picked 98 of 1021 (31 by every draw) · rule any · pin 1565

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/hook-fail-open-silent · Hook fails open silently — Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty.
decision/hook-stderr-forbidden · No stderr from hooks — viola hook writes nothing to stderr.
contract/hook-log-file/path · the viola hook log file · path — viola hook logs only to a file sink at diagnostics/hook-<name>.ndjson.
decision/channel-json-rpc-over-ndjson · Channel is JSON-RPC 2.0 over ndjson — The viola-channel crate speaks JSON-RPC 2.0 over ndjson on local Unix sockets, one endpoint per viola run, with a sync client and server and a Tokio client behind the tokio feature.
decision/obs-tier-standard · Observability tier Standard — The observability tier is Standard (1), with Minimal-tier exporter carve-outs. · why: The tests tier is Comprehensive (2) and obs sits one step below it within the plus-or-minus-one band; the Minimal security tier gives no Comprehensive driver.
decision/refusal-telemetry · Refusal telemetry for filesystem refusals — Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry.
decision/strict-modes-failure-lines · Where a strict-modes failure is logged — A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
contract/diagnostics-files/hook · the diagnostics files · hook — Role hook writes <home>/diagnostics/hook-<name>.ndjson, shared by concurrent hook processes; a write failure is swallowed.
contract/process-exit-detail/strict-modes-failed · process-exit detail codes · strict-modes-failed — Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1.

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/strict-modes-refusal-writes-no-line · Strict-modes refusal writes no line — A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document. · why: Ratified as the answer to whether a refused strict-modes check logs. · except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
contract/hook-fail-open/exit · Hook fail-open · exit — A hook that fails open exits 0 and leaves stderr empty.
contract/viola-verify/home-tree-refused · viola verify · refused home-tree check — A refused home-tree check exits 1 with the home pair, opens no log and writes no line; a refused update_stamps exits 1 with the home pair and, when an instance is named, ends the opened log with a process-exit line carrying detail strict-modes-failed.
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/hook-fail-open-logged-to-hook-file · Hook fail-open logged to the hook file — Hook fail-open paths (oversize stdin, malformed JSON, channel failure, server-verify mismatch, refused socket directory) are logged to hook-<name>.ndjson only; a missing VIOLA_NAME writes no line.
decision/hook-lines-required · Hook invocation and decision lines required — Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither.
decision/hook-never-writes-stderr · Hook never writes stderr — A hook process never writes to stderr.
decision/hook-silent-without-trusted-instance · Hook silent without a trusted instance — A hook with no VIOLA_NAME, or whose instance is absent or refused by its strict-modes check, exits 0 with no config read, no stdin read, no log line, no stderr, no frame and no SessionEnd append.
decision/latency-aggregation-outside-process · Latency aggregation outside the process — Percentile aggregation is done by the harness or jq over the ndjson, never by an in-process metrics pipeline.
decision/linux-only-os-target · Linux-only OS target — The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
decision/no-formal-slo · No formal SLO — No formal latency or availability SLO is held beyond the tests perf gates.
decision/per-parser-parse-failure-counts · Per-parser parse-failure counts — Each parser that drops input (channel framing at MAX_FRAME, the viola mcp stdin at MAX_FRAME, hook stdin, claude agents --json, the hyprctl replies, vt100 under catch_unwind) keeps its own parse-failure count; paste validation refuses a send as not-delivered instead.
decision/server-check-peer-credentials · Server check by peer credentials — The server check reads the peer pid by SO_PEERCRED and the peer euid; a mismatch of either reads server-verify-failed and an unreadable euid is refused.
decision/socket-dir-one-code · One code for socket-directory causes — A refused check, a failed create or a lost race on the socket directory all read the one code socket-dir-refused.
contract/dialog-hook/refused-instance-check · Dialog hook · refused instance check — A refused instance check on the dialog hook exits 0 at once with empty streams, no frame and no line.
contract/dialog-log-line/server-verify-failed · Dialog log line · server-verify-failed — A server-verify failure exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
contract/dialog-log-line/socket-dir-refused · Dialog log line · socket-dir-refused — A refused socket directory exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
contract/exit-cause-detail/server-verify-failed · Exit cause detail codes · server-verify-failed — A failed server check of peer euid, peer pid or start time is logged by the CLI and the hook as server-verify-failed; MCP writes no line for it.
contract/hook-decision/server-verify-failed · Hook decision line · server-verify-failed — A server-verify mismatch is one hook-decision at WARN with detail server-verify-failed and no frame sent; on a dialog hook, hook-invoked with corr null then hook-decision with corr null and decision_emitted false; no pid, start time, path or endpoint.
contract/hook-decision/socket-dir-refused · Hook decision line · socket-dir-refused — A refused socket directory on viola hook writes one hook-decision in the server-verify form with detail socket-dir-refused, and the hook exits 0.
contract/replayed-snapshot/live-fields · A snapshot.json rebuilt by replay · endpoint, pid, started_at, statusline_command — A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, and run's collision check reads the heartbeat alone.

## telemetry-strategy · ## 2. Telemetry Strategy · twin 2-telemetry-strategy
contract/strict-modes-refusal-exit/1 · strict-modes refusal exit codes · 1 — run, revive's start and verify exit 1 on a strict-modes refusal; revive under --json prints the strict-modes-failed document.
decision/hook-silent-without-instance · Hook silent without an instance — A hook with no VIOLA_NAME, or with a refused or absent instance, exits 0 at once and writes no file, no line and no frame.
decision/pre-open-strict-modes-refusal-silent · Strict-modes refusal before a log opens writes no line — A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document. · except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
contract/viola-verify-refusal/home-tree-refused · viola verify strict-modes refusal · home tree refused — Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.
contract/state-recovered-detail/snapshot-replayed · state-recovered detail · snapshot-replayed — A state-recovered line at WARN carries this detail, one line per snapshot read call.
contract/state-recovered-detail/snapshot-unsupported-v · state-recovered detail · snapshot-unsupported-v — A state-recovered line at WARN carries this detail, one line per snapshot read call.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/dialog-hook-fails-open · Dialog hook fails open on a failed check — A dialog hook whose instance, socket-directory, euid or server check fails emits no decision and sends no frame, so the dialog falls back to the human.
decision/hook-refused-instance-silent · Refused instance makes the hook silent — On the event and dialog paths the instance check runs before the diagnostics level and log init; a refused or unresolved instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
decision/refused-home-tree-writes-no-line · Refused home tree writes no line — verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
contract/hook-decision-detail/socket-dir-refused · hook-decision line · socket-dir-refused — A refused socket directory writes one hook-decision{detail:"socket-dir-refused"} at WARN, on the dialog path after hook-invoked{corr:null}, with decision_emitted:false; exit 0, empty streams, no frame.
contract/verify-process-exit/strict-modes-failed · verify process-exit line · detail strict-modes-failed — A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
decision/dialog-hook-invoked-after-response · Dialog hook-invoked written after the response — A dialog hook writes hook-invoked only once dialog_id is known, after the hook.dialog response; non-dialog hooks and fail-open paths before dialog_id write hook-invoked{corr:null} at once. · why: invoked_at still records the hook's true start, and the wrapper's dialog-raised gives live visibility during the wait.
decision/home-tree-check-before-start-log · Home-tree check before the start's log — The start of run and of viola revive checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
decision/no-raw-snapshot-reads · Product readers read the snapshot through replay — Every product reader of the snapshot uses the replaying read; no product reader reads the snapshot raw.
decision/pinned-copy-never-replaced · Pinned copy placed, never replaced — An absent pinned copy is placed, never replaced; a start that loses the placement re-hashes the copy that stands, reusing an equal one and refusing a different one as pinned-hash-mismatch with the file left as found.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/refused-check-writes-no-line · A refused check opens no process log — A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
decision/replayed-snapshot-unreachable · Replayed snapshot is unreachable — Only a snapshot read whole leads to a connect; a replayed or absent snapshot is treated as unreachable and no frame is sent. · why: A replayed snapshot carries no endpoint, pid or started_at.
decision/revive-start-logs-as-run · viola revive start logs as run — The start arm of viola revive is not a cli verb and logs as process run once its instance check has passed.
decision/server-refusal-inside-root-span · Server-verification refusal inside the root span — A frame verb's or the hook's server-verification refusal happens inside its scenario's root span. · guard: request_refuses_an_unverified_server_inside_its_dispatch_span; hook_refuses_an_unverified_server_inside_hook_handle
contract/plugin-install-exit-codes/21 · plugin install exit codes · 21 — The strict-modes tree check refused; no log is opened and no line is written, with the stderr pair or, under --json, the instance-unreachable document.
contract/revive-preflight-detail/session-live · viola revive preflight process-exit · session-live — A preflight refusal adds detail session-live before any child is started; the reading is skipped under --fork and passes on a failed read.
contract/revive-process-log/agents-probe · viola revive run process log · agents-probe — The session-live reading writes one process-start and process-exit{subject:"agents-probe"} pair, plus one parse-rejected{parser:"claude-agents-json"} on an oversize or malformed answer.
contract/run-span-attributes/run-collision-check · viola run start span attributes · run.collision_check — outcome is free or already-live; a live or stale holder is already-live, a gone one free; over a replayed snapshot a beat no older than STALE_AFTER is already-live, an older one free; an absent snapshot or failed read is free.
contract/run-span-attributes/run-pin-copy · viola run start span attributes · run.pin_copy — outcome is ok or pinned-hash-mismatch, and the set stays those two.
contract/run-start-failure-detail/pinned-hash-mismatch · run start process-exit · pinned-hash-mismatch — process-exit{subject:"self", exit_code:1, detail:"pinned-hash-mismatch"}, with no path or pid.
contract/state-recovered/snapshot-replayed · state-recovered line · snapshot-replayed — state-recovered{detail:"snapshot-replayed", file:"snapshot.json"} at WARN with no corr for an unreadable snapshot, one line per call; none for a present or absent snapshot, and none when the replay itself fails.
contract/verify-exit-codes/1 · verify exit codes · 1 — A row failed, verify refused, or verify was stopped.
contract/verify-process-exit/interrupted · verify process-exit line · detail interrupted — A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".
contract/verify-stop-stderr/unable-hint-pair · verify stderr on a stop · the stop pair — A stopped verify prints exactly "unable: verify was stopped before it finished" and "hint: run viola verify again to stamp this CLI" on stderr, with empty stdout and no stamp written.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.
contract/log-line/message · every log line · message — Flattened, a static literal equal to the event name.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
contract/diag-detail/strict-modes-failed · diagnostic detail words · strict-modes-failed — Written on the process-exit exit 1 of viola verify under a parsed VIOLA_NAME and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
contract/process-exit-detail/interrupted · process-exit detail · interrupted — A viola verify stopped by SIGINT or SIGTERM before its rows are checked ends with one process-exit exit 1 detail interrupted, after every spawn pair it opened is closed.
contract/process-exit-detail/verify-refusal · process-exit detail · viola verify refusal — Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture or interrupted; a failing row carries no code.
contract/verify-process-exit/child-exit-status-and-duration · viola verify spawn process-exit lines · child_exit_status and duration_ms — Each of the eight verify spawn pairs' process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
contract/diag-detail/pinned-hash-mismatch · diagnostic detail words · pinned-hash-mismatch — A filesystem refusal word written on process-exit with exit 1.
contract/diag-detail/socket-dir-refused · diagnostic detail words · socket-dir-refused — A closed detail on process-exit and hook-decision for a socket dir refused as a symlink, no directory, foreign owner or mode bit: run and revive's start write process-exit exit 1, CLI frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP writes none.
contract/diag-line-fields/hook-events · diagnostic line additive fields · hook-invoked / hook-decision — hook_event (one of nine closed hook names), invoked_at, decision_emitted, deadline_hit, detail, duration_ms.
contract/diag-line-fields/process-exit · diagnostic line additive fields · process-exit — subject, exit_code (self), child_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
contract/diag-line-fields/state-recovered · diagnostic line additive fields · state-recovered — detail (torn-line-healed, snapshot-replayed, snapshot-unsupported-v), file (basename only), offset (torn-line-healed only), v_seen (snapshot-unsupported-v only), scanned (snapshot details only: the instance name when valid, never a path or null).
contract/hook-decision-detail/channel-unreachable · hook-decision detail · channel-unreachable — Written when the channel cannot be reached, a replayed or absent snapshot included; no frame is sent, the hook fails open and SessionEnd takes its direct append.
contract/hook-decision-detail/fail-open-codes · hook-decision detail · fail-open codes — oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline; no hook arm writes strict-modes-failed or unverified-cli.
contract/hook-decision-detail/server-verify-failed · hook-decision detail · server-verify-failed — viola hook writes it at WARN on a server of another euid, a server that is not the recorded wrapper, or a check that cannot be made; no frame is sent, the hook fails open, and on the dialog path it follows hook-invoked with decision_emitted false.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/pre-log-strict-refusals-write-no-line · Strict-modes refusals before the log opens write no line — Refusals of the instance and home-tree checks that run before a role file opens write no line in any log and open nothing under the home. · why: Opening a log narrows the home and would erase the refused state; the strict-modes negative tests assert that no diagnostics folder appears.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/obs-code-mutation-tested · Obs code mutation-tested like product code — Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red. · guard: test-plan Mutation gate
decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
decision/surviving-obs-mutant-fails · Surviving obs mutant is a failure — A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure. · except: A missed mutant the host never compiles is not counted as surviving.
decision/nonzero-count-blocks-chunk · Nonzero error count blocks the chunk — Any nonzero error-budget count makes the chunk red until the root cause is fixed within that chunk. · why: the tests quarantine rule

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
contract/hook-decision/channel-unreachable · Hook decision line · channel-unreachable — A hook whose snapshot read is replayed or absent fails open as channel-unreachable.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/latency-with-gate-statistic · Latency reported with the gate statistic — Latency is never reported without the gate statistic; p50 and p95 are report-only.
decision/budgets-asserted-by-exit-code · Budgets asserted by exit code — Every budget has an exit-code-setting assertion (hyperfine max via jq).
decision/hyperfine-max-is-enforced · Hyperfine max is the enforced number — The max from hyperfine is the enforced latency number.
decision/no-retry-once · No retry-once policies — No retry-once policy is added for panics or flaky telemetry assertions.

## log-file-location · key of ## 3. Observability Harness Contract
contract/diagnostics-files/home-resolution · diagnostics files · home resolution — The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
contract/diagnostics-files/paths · diagnostics files · process log paths — Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.

## otel-sdk-init · key of ## 3. Observability Harness Contract
decision/home-tree-check-before-log-init · Home tree check before log init — run, revive's start arm and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
decision/hook-refusal-exits-zero · Refused hook fails open — A hook whose instance is refused or absent exits 0 at once, reading nothing under the home and no stdin, writing no frame and no log line.
decision/failed-log-open-discards · Failed log open discards lines — When the role file cannot be opened, viola_obs_init returns the error and installs no subscriber, so no line is written; hook carries on and the other roles, mcp included, return the error.
decision/instance-check-before-log-init · Instance check before log init — Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found. · why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770. · except: The hook --capture arm runs ahead of the check.
decision/strict-modes-before-diagnostics · Home strict-modes check before diagnostics — Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line.

## service-identity · key of ## 3. Observability Harness Contract
decision/single-version-constant · One version constant for every crate — One compile-time version constant taken from the package version serves every crate, and every crate takes the workspace version. · why: The log version, channel sender and snapshot writer cannot drift apart.

# obs-plan · its absences, names and overviews, whole

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
absence/audit-log-compliance-trail — An audit-log compliance trail
absence/otlp-receiver — An OTLP or pulse-class telemetry receiver
absence/pii-scrubbing-regime — A PII-scrubbing regime for diagnostics lines beyond the NEVER-log floor

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
absence/deployment-environments — Deployment environments or hosting
absence/interactive-terminal-interface — An interactive terminal interface: prompt, TUI or pager

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
overview/2-telemetry-strategy — Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use corr and conn; errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by jq over the ndjson; events.ndjson is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
absence/web-or-graphical-frontend — A web or graphical frontend

## 5-metric-coverage · ## 5. Metric Coverage · twin metric-coverage
absence/in-process-metrics-pipeline — An in-process metrics pipeline, with no listener and no exporter

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
absence/database — A database
absence/grpc — gRPC of any kind

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
absence/otel-span-processing — An OTel SpanProcessor
absence/sentry-error-reporting — Sentry error reporting

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
absence/otel-trace-export — OTel trace export: no exporter

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
absence/on-call-paging — On-call or paging

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
absence/network-egress — Network egress of any kind
absence/telemetry-exporter-stage — A telemetry exporter stage

## service-identity · key of ## 3. Observability Harness Contract
absence/hosted-deployment — A hosted deployment or deployment environment; viola is local-only

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
absence/pulse-class-receiver — A pulse-class log receiver
