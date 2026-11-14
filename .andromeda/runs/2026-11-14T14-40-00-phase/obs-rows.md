# obs-plan · the rows that bind 2026-11-14-budget-gate-removed — picked 152 of 1060 (81 by every draw) · rule any · pin 1142

## 1-obs-scope-summary · ## 1. Obs Scope Summary · twin obs-scope-summary
decision/diag-schema-additive-only · Diagnostic log schema is additive only — The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove any field.
decision/error-chain-fixed-stderr-message · Error chains reach stderr as fixed messages — In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
decision/escape-controls-in-terminal · Control characters escaped in terminal rendering — C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
decision/hook-stdout-decision-body-only · Hook stdout carries the decision body only — viola hook stdout carries only the decision body; for viola hook statusline, the output of the user's status-line command.
decision/parse-failures-unknown-counter · Parse failures surface as unknown plus a counter — A child-output parse failure surfaces as `unknown` plus a counter. · except: A hyprctl reply that fails its parse reads the closed error oversize or malformed with one parse-rejected line, never unknown.
decision/serde-errors-fixed-message · Serde error chains become a fixed message — A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
decision/stderr-fixed-strings · Fixed strings on stderr — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
contract/process-log-event/release-from-driver · the process log event field · release-from-driver — Release from the driver is logged as its own distinct event.
contract/viola-mcp-tools/list · viola-mcp stdio server tools · list — The viola-mcp stdio server offers a list tool.
decision/distinct-detail-code-per-exit-cause · Distinct detail code per exit cause — Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid. · except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
decision/exit-1-detail-codes · Distinct detail codes for exit-1 causes — On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
decision/send-text-confined · Send text confined to instance diagnostics — `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
decision/statusline-command-confined · Statusline command confined to diagnostics — On the budget governor path, the `statusline_command` string never appears outside `diagnostics/`.
decision/statusline-command-duration-exit-only · Statusline command observed by duration and exit — The user's statusline_command shell-out is observed by its duration and exit status only. · why: A user-written shell command.
contract/agent-run-status/state-last-error · agent-run status · state and last_error — The aggregate health snapshot gives `state` (`ready`, `degraded` or `down`) and `last_error` (null, or `list-exit-<code>` built from the list exit code).

## obs-scope-summary · ## 1. Obs Scope Summary · twin 1-obs-scope-summary
decision/budget-metadata-loggable · Budget metadata loggable as fields — Operational metadata in budget.json (percentages, resets_at) may be logged as structured fields.
decision/config-observed-at-load · Configuration observed only at load — Configuration is observed only at its parse at load time, logged as skipped-key counts. · why: Static configuration and data.
decision/controls-escaped-in-terminal-output · Controls escaped in terminal output — C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
decision/exit-code-log-field · Exit code as a log field — The exit code (0, 1, 2, 10–14, 20, 21), the machine-readable outcome, is carried as a log field.
decision/fault-telemetry-per-class · Fault telemetry per class — Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
decision/fixed-stderr-messages · Fixed stderr refusal and fault lines — Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
decision/hook-lines-required · Hook invocation and decision lines required — Every hook process in a trusted wrapped session writes its hook-invoked and hook-decision lines, bar a PreToolUse for a tool outside the dialog matcher; a hook outside such a session or on a --capture probe writes neither.
decision/log-fields-additive-only · Required log fields only added to — Fields may be added to process-log lines, but the required ones are never renamed or removed. · why: The harness greps on them.
decision/skipped-counters · Skipped counters — Unknown event kinds, unknown fields and torn lines are each counted in skipped counters.
contract/budget-log-line/window · Budget log line · window — The budget window is five-hour or seven-day.
contract/budget-path-log-events/event-set · Budget path log events · event set — The budget path logs hook-invoked and hook-decision for statusline, and send-refused with budget-paused.
contract/channel-response/detail · Channel response line · detail — On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
contract/diag-line-event/release-from-driver · Diag line event · release-from-driver — release-from-driver is its own logged event, admitted by the diag-line schema, not a generic -32602 wrapper fault.
contract/mcp-iserror/refusal · MCP isError result · refusal — A peer refusal that is no code word reads unknown, and a string detail that is no code word reads null.
contract/process-log/event · Process log line · event — A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
contract/refusal-reason/unknown-fallback · Refusal reason · unknown fallback — An unrecognised RefusalReason falls back to unknown with exit 14.
contract/run-start-events/order · run start events in events.ndjson · order — A run start writes wheel{cause:"start"}, then budget-gate, then session-start.
decision/channel-log-scrub-rule · Channel log scrub rule — Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
decision/correlation-in-corr-field · Correlation in the corr log field — Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
decision/hook-mcp-instrumented-at-boundary · Hook and MCP instrumented at the boundary — Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut. · why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
decision/no-process-log-without-instance · No process log without an instance — A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
decision/parse-failures-surface-as-unknown · Parse failures surface as unknown — A parse failure surfaces as the value unknown plus a counter. · except: A hyprctl reply that fails its parse reads the closed error oversize or malformed with one parse-rejected line, never unknown.
decision/process-init-order · Process logging init order — Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start. · why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
decision/send-refused-by-each-deciding-side · Send refusal logged by each deciding side — A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
decision/statusline-command-never-in-home-logs · Statusline command never in home-level logs — The statusline_command string never appears outside diagnostics/, reaching a detail file only as part of an error chain; its shell-out records only shell_exit_status and duration_ms. · why: The statusline command is classed High for integrity.
decision/statusline-command-only-in-diagnostics · Status-line command string kept to diagnostics — The statusline_command string is never logged outside diagnostics/.
decision/statusline-shell-out-instrumentation · Status-line shell-out instrumented by duration and exit — The `viola hook statusline` shell-out of the user's command is instrumented by its duration and exit status only. · why: A user-written shell command.
decision/tests-event-list-adopted · Tests event list adopted — The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A strict-modes refusal of send, wait, last, answer, pause or release writes no line and prints only the fixed instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21, with no pid, path or endpoint.
contract/event-line/shape · events.ndjson event line · line shape — Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
contract/hook-stdout/statusline · Hook stdout · viola hook statusline — stdout carries only the user's status-line command's output.
contract/send-log-line/detail · Send log line · detail — The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text or turn-running.

## 2-telemetry-strategy · ## 2. Telemetry Strategy · twin telemetry-strategy
contract/process-log-line/event · the process log line · event — The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
decision/corr-conn-correlation · Correlation through corr and conn — Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.

## 3-observability-harness-contract · ## 3. Observability Harness Contract
contract/hook-output/statusline-stdout · hook output streams · viola hook statusline stdout — Carries the user's status-line command output byte for byte; nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden --capture arm.
decision/codes-only-process-logs · Codes-only process logs, content in detail files — Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
contract/list-stderr/unable-hint-pair · viola list stderr · unable / hint pair — The unable / hint: pair carries no SGR colour codes.
contract/run-terminal-output/start-refusal · run terminal output · start refusal — A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.

## 4-span-trace-coverage · ## 4. Span / Trace Coverage · twin span-trace-coverage
decision/ci-verify-fake-agent-only · CI verify against the fake agent only — In CI, viola verify runs only against the fake agent.
decision/detail-codes-only · detail is a closed code — The detail field is a closed kebab-case code catalog and never carries a path or pid.
decision/reader-never-heals-torn-lines · Reader never heals torn lines — A reader of an append log neither heals a torn line nor logs one; only the appender heals.
decision/statusline-stdout-passthrough · Statusline stdout passes through unlogged — The user's statusline stdout passes through unchanged and is never logged.
contract/state-recovered-detail/snapshot-replayed · state-recovered detail · snapshot-replayed — A state-recovered line at WARN carries this detail, one line per snapshot read call.
decision/uninstrumented-components · Components left uninstrumented — viola-core, config.json and plugin files, and build/CI infrastructure are not instrumented. · why: viola-core only supplies ObsEvent and RefusalReason values; the rest are not instrumentable.

## span-trace-coverage · ## 4. Span / Trace Coverage · twin 4-span-trace-coverage
decision/clients-screen-peer-replies-to-code-words · Clients screen peer replies to code words and a fixed fault — The CLI and MCP clients screen a peer reply before using it: a refusal and its detail must be code words, a wrapper fault keeps only its code, a fixed message and closed data, and the screening adds no span and no line. · why: No path or upstream text may cross from a wrapper into a document, a tool result or a log.
decision/product-order-from-events-log · Start product order read from the events log — The product order wheel{cause:"start"} then budget-gate then session-start{source:"hook"} is verified from the instance's events log, not duplicated into process logs.
decision/real-cli-verify-never-in-ci · Real-CLI verify never runs in CI — In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
decision/rejected-statusline-stdin-runs-nothing · Rejected statusline stdin runs nothing — Oversize or malformed statusline stdin prints nothing and runs no shell.
decision/release-from-driver-refused · A release from a driver is refused — A release that carries a string from is refused with -32602, and viola release forwards VIOLA_NAME, so a driver session's own viola release is refused too.
decision/unreadable-budget-reads-as-none · Bad budget file reads as no reading — A refused, unreadable, over-cap or other-version budget file reads as no reading.
contract/budget-span-attributes/run-budget-eval · budget span attributes · run.budget_eval — trigger (start|send|turn-ended), paused, and window only when paused; opened once per evaluation: at start, at a send's arrival, and after a turn-ended hook line is appended.
contract/budget-span-attributes/state-budget-write · budget span attributes · state.budget_write — state.budget_write carries five_hour_pct and seven_day_pct.
contract/budget-spans/span-chain · budget governor spans · span chain — hook.statusline (SERVER) › state.budget_write › statusline.shell_out (CLIENT); in run, run.budget_eval (INTERNAL).
contract/cli-stderr/refusal-pair · CLI stderr · unable and hint pair — A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
contract/dialog-log-fields/hook-lines · dialog log fields · hook-invoked and hook-decision — hook-invoked{corr:<dialog_id>, hook_event, invoked_at}, then hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}.
contract/events-log/budget-gate · events log · budget-gate — A budget-gate record is appended only when the triple (paused, window, override end) changes.
contract/release-budget/budget-gate · release method · budget true — A release with budget true does not touch the wheel and appends one budget-gate record when a pause stood, none otherwise; the string-from refusal and the budget type check come first.
contract/release-refusal/invalid-params · release refusal lines · invalid-params — A from of another type, or a non-bool budget, is a plain -32602 (data null) with no release-from-driver line, its channel-response carrying detail invalid-params on both sides at WARN.
contract/run-start-spans/span-chain · viola run start spans · span chain — run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › run.budget_eval (trigger start) › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT).
contract/send-refused/budget-paused · wrapper send-refused · budget-paused — send-refused{corr:<cursor>, refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}, with client exit_code 11.
contract/state-recovered/snapshot-replayed · state-recovered line · snapshot-replayed — state-recovered{detail:"snapshot-replayed", file:"snapshot.json"} at WARN with no corr for an unreadable snapshot, one line per call; none for a present or absent snapshot, and none when the replay itself fails.
contract/statusline-log-fields/hook-decision · statusline hook log fields · hook-decision — hook-decision{hook_event:"statusline", budget_written, duration_ms}, deadline_hit only when true; budget_written is true only when the payload held a rate_limits object and the write succeeded.
decision/expired-dialog-renders-for-human · Expired dialog returns to the human — At the hold's end the reply stays null, the hook writes hook-decision with decision_emitted false, exits 0 with empty stdout, and the dialog renders for the human.
decision/no-raw-snapshot-reads · Product readers read the snapshot through replay — Every product reader of the snapshot uses the replaying read; no product reader reads the snapshot raw.
decision/replayed-snapshot-unreachable · Replayed snapshot is unreachable — Only a snapshot read whole leads to a connect; a replayed or absent snapshot is treated as unreachable and no frame is sent. · why: A replayed snapshot carries no endpoint, pid or started_at.
decision/send-line-correlation · Send lines correlate with channel lines — Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
contract/budget-span-attributes/statusline-shell-out · budget span attributes · statusline.shell_out — statusline.shell_out carries shell_exit_status and duration_ms.
contract/cli-exit-21/endpoint-lost · cli exit 21 process-exit · endpoint-lost — detail endpoint-lost with during call: the connection was lost after the connect; it is the only exit-21 detail whose during is call.
contract/cli-exit-21/instance-dead · cli exit 21 process-exit · instance-dead — detail instance-dead with during connect: no live snapshot, a replayed one included, or the connect failed.
contract/cli-exit-21/unwrapped-name · cli exit 21 process-exit · unwrapped-name — detail unwrapped-name with during connect: no instance directory.
contract/cli-exit/1-internal-error · send.client process-exit · 1 internal-error — exit_code 1 internal-error on a reply with neither a parseable result nor an error.code.
contract/cli-exit/20-wrapper-fault · send.client process-exit · 20 wrapper-fault — Exit 20 carries detail wrapper-fault.
contract/cli-stderr/internal-error · CLI stderr · internal fault message — An internal fault prints exactly "error: internal error".
contract/config-json/dialog-hold-s · config.json · dialog_hold_s — The dialog hold, a whole number of seconds from 60 to 3600, default 60; any other value gives one parse-rejected malformed WARN line and the hold takes 60.
contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
contract/release-refusal/release-from-driver · release refusal lines · release-from-driver — One release-from-driver{corr:<rpc id>, conn, from (only a valid name), from_trust:"self-reported"} at INFO beside the -32602 reply with data {"reason":"release-from-driver"}; its channel-response carries detail release-from-driver on both sides at WARN.
contract/run-span-attributes/run-collision-check · viola run start span attributes · run.collision_check — outcome is free or already-live; a live or stale holder is already-live, a gone one free; over a replayed snapshot a beat no older than STALE_AFTER is already-live, an older one free; an absent snapshot or failed read is free.
contract/run-start-failure-detail/already-live · run start process-exit · already-live — process-exit{subject:"self", exit_code:1, detail:"already-live"} for a live holder or a fresh replayed beat, with no path or pid.
contract/state-recovered/scanned · state-recovered line · scanned — From the board read, the line carries instance (the caller) and scanned (the scanned instance's name when it parses, absent otherwise); with no VIOLA_NAME no log is open and the line goes nowhere.
contract/state-recovered/snapshot-unsupported-v · state-recovered line · snapshot-unsupported-v — state-recovered{detail:"snapshot-unsupported-v", file:"snapshot.json", v_seen} at WARN with no corr for a newer snapshot, one line per call.
contract/statusline-decision-detail/malformed-json · statusline hook-decision detail · malformed-json — The statusline hook-decision carries detail malformed-json on that fail-open path.
contract/statusline-decision-detail/oversize-stdin · statusline hook-decision detail · oversize-stdin — The statusline hook-decision carries detail oversize-stdin on that fail-open path.
contract/statusline-log-fields/parse-rejected · statusline hook log fields · parse-rejected — parse-rejected{parser:"hook-stdin", detail:"oversize"|"malformed", count:1} for stdin the arm rejects.
contract/verify-stdout/step-line · verify stdout · step line — One "[NN/MM] <row id> <row words>  pass|fail" line is printed for each relied ledger row, MM being the relied row count.
contract/verify-stdout/summary-line · verify stdout · summary line — The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM.

## metric-coverage · ## 5. Metric Coverage · twin 5-metric-coverage
decision/gauges-from-harness-snapshots · Gauges from harness snapshots — A current-value gauge is read from harness snapshots: budget.read_at, and liveness via agent-run status.

## 6-log-coverage · ## 6. Log Coverage · twin log-coverage
decision/log-schema-additive-only · Log schema additive to the tests' format — The observability log schema only adds fields and enum values to the tests' log format; it renames and removes nothing. · why: The tests' log format is the binding text.
contract/log-levels/warn · log levels · warn — WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
contract/log-line/event · every log line · event — A value of the closed kebab-case event enum.

## log-coverage · ## 6. Log Coverage · twin 6-log-coverage
decision/unemittable-schema-words-removed · Schema words no build can emit are removed — A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
contract/diag-line-fields/hook-events · diagnostic line additive fields · hook-invoked / hook-decision — hook_event (one of ten closed hook names incl. statusline), invoked_at, decision_emitted, deadline_hit, budget_written, detail, duration_ms.
contract/diag-line-fields/send-events · diagnostic line additive fields · send-issued / send-confirmed / send-refused — from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, window, used_percentage, resets_at, paused, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
contract/diag-line-fields/state-recovered · diagnostic line additive fields · state-recovered — detail (torn-line-healed, snapshot-replayed, snapshot-unsupported-v), file (basename only), offset (torn-line-healed only), v_seen (snapshot-unsupported-v only), scanned (snapshot details only: the instance name when valid, never a path or null).
contract/parse-rejected-detail/unknown-keys · parse-rejected detail · unknown-keys — config-json skipped keys, counted in count.
contract/send-refused-detail/refusal-reason-words · send-refused detail · RefusalReason detail words — Refusal details are the arch RefusalReason details verbatim: manual-pause, five-hour, seven-day, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
contract/diag-line-fields/parse-rejected · diagnostic line additive fields · parse-rejected — parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry, hyprctl-json), detail, count.
contract/diag-line-fields/release-from-driver · diagnostic line additive fields · release-from-driver — conn, from (only when a valid ViolaName), from_trust (self-reported).

## 7-error-capture-reporting · ## 7. Error Capture & Reporting · twin error-capture-reporting
decision/boundary-errors-end-in-logged-line · Crate-seam errors end in a logged line — Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.

## error-capture-reporting · ## 7. Error Capture & Reporting · twin 7-error-capture-reporting
decision/peer-replies-screened-as-code-words · Peer replies screened as code words — A peer reply's refusal and detail are kept only as code words (1-64 bytes of [a-z0-9-], letter first); an unknown code word stays visible and exits 14. · why: by design

## 8-pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin pii-scrubbing-compliance
decision/default-deny-log-fields · Default-deny for logged fields — A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure. · guard: gate G4 against schemas/diag-line.v1.json
decision/detail-schema-self-contained · Separate self-contained detail-line schema — Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
decision/schema-top-level-unevaluated-properties · Default-deny schema via top-level unevaluatedProperties — The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists. · why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then · guard: contract_diag_schema
decision/fixed-error-messages · Fixed error messages — Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
decision/home-level-lines-path-free · Home-level lines path-free — Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files. · why: The OS username inside absolute paths is the only PII in scope.
decision/no-allow-list-widening · Allow-list cannot be widened — No config key, flag or env var can widen the logged-field allow-list or disable redaction.
decision/payload-types-derive-redact · Type-level redaction of payload types — Every channel, hook and MCP payload type that carries content derives veil Redact, so Debug never reveals content; a payload type with no content field derives none.
decision/source-allow-list-logging · Source allow-list for logged fields — Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control. · guard: disallowed-macros; raw_event_hits; G1 bare instrument

## pii-scrubbing-compliance · ## 8. PII Scrubbing & Compliance · twin 8-pii-scrubbing-compliance
decision/closed-protocol-codes · Closed protocol codes — Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value. · guard: answer_invalid_params_names_its_cause_and_no_param
decision/error-bodies-clean · Clean error bodies — Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named. · guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
decision/typed-ids-never-in-detail · Ids logged as typed fields — pid, child_pid, agent_session_id, started_at, budget percentages and resets_at are logged as typed fields, never inside detail codes.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/g4-schema-conformance-gate · Schema conformance gate on every line — Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs. · guard: G4

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
decision/schema-conformance-gate · Schema conformance gate on real process output — Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted. · why: schema failures count as a build failure and unit tests check only in-memory lines · guard: gate G4 (schema-conformance step)
decision/obs-code-mutation-tested · Obs code mutation-tested like product code — Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red. · guard: test-plan Mutation gate

## 10-slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin slo-invariants-telemetry-budgets
decision/schema-conformance-failure-fails · Schema-conformance failure is a failure — An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
decision/error-budget-zero-tolerance · Zero-tolerance error budget — The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures. · except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
decision/surviving-obs-mutant-fails · Surviving obs mutant is a failure — A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure. · except: A missed mutant the host never compiles is not counted as surviving.

## slo-invariants-telemetry-budgets · ## 10. SLO Invariants & Telemetry Budgets · twin 10-slo-invariants-telemetry-budgets
contract/hook-decision/channel-unreachable · Hook decision line · channel-unreachable — A hook whose snapshot read is replayed or absent fails open as channel-unreachable.

## 11-obs-anti-patterns-never-do-these · ## 11. Obs Anti-Patterns (NEVER do these) · twin obs-anti-patterns
decision/closed-event-and-corr-on-every-line · Closed event enum on every line, corr where correlated — Every log line carries the closed event enum, and corr rides only on the correlated events the schema allows it on, so the signal pyramid never collapses into free text.
decision/event-values-closed · event values closed — No event value is added without a Decisions Log entry.
decision/field-allow-list-default-deny · Field allow-list is default-deny — No field is logged unless named in the field catalog.
decision/must-trace-paths-always-traced · Must-trace paths always traced — Spans and events are never skipped on the 6 must-trace paths. · why: The agent loses its ability to debug without them.
contract/stdout-by-role/hook · stdout by role · hook — stdout carries the decision body or, for hook statusline, the user's status-line output.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Observability Harness Contract
decision/diag-schema-matches-event-enum · Diagnostic line schema matches the event enum — A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.

## log-format-json-schema · key of ## 3. Observability Harness Contract
decision/obs-aligns-to-tests-log-format · Observability aligns to the tests log format — The log format is bound by the test plan's harness contract; observability may add fields but must not rename or remove its fields. · why: the harness greps on them
contract/corr-by-event/state-recovered · corr by event · state-recovered — corr is null.
contract/corr-by-event/parse-rejected · corr by event · parse-rejected — corr is null.
contract/corr-by-event/release-from-driver · corr by event · release-from-driver — corr is the JSON-RPC request id of the release refused with -32602 for carrying a string from.
contract/event-stream/line · the event stream · event line — Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.

## otel-sdk-init · key of ## 3. Observability Harness Contract
contract/config-json/five-hour · config.json · five_hour — Key five_hour holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.
contract/config-json/known-keys · config.json · known keys — config.json has six known keys: v, diagnostics_level, claude_env_keep, five_hour, seven_day, dialog_hold_s; any other key is counted as skipped.
contract/config-json/seven-day · config.json · seven_day — Key seven_day holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.
contract/config-json/v · config.json · v — A document that is no object, or whose v is not 1, gives the threshold and hold defaults and no further line.
contract/diagnostics-events/config-parse-rejected · diagnostics events · parse-rejected config-json — Right after process-start, a config.json parse that failed or skipped keys logs parse-rejected{parser:"config-json", detail, count}.
contract/cli-exit-codes/21 · CLI exit codes · 21 — An instance strict-modes refusal exits 21 with the fixed instance-state pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.

## snapshot-paste-to-ai-integration · key of ## 3. Observability Harness Contract
contract/agent-run-logs/events-record · agent-run logs output · events record — Each events.ndjson line is merged as {"src":"events","instance","offset","record"}.

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
