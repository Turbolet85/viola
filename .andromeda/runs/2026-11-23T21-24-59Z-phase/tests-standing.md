# tests · standing conventions

- S1 · Each new root integration test binary is prefixed by its surface, and each test function follows the test naming rule. · 2-test-strategy · test-strategy
  decision/integration-binary-surface-prefix · Root integration binaries prefixed by surface — Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · why: so nextest filtersets can select layers
  decision/test-function-naming · Test function naming — Test functions are named <subject>_<condition>_<expected> in snake_case. · except: critical-path scenarios are named path<N>_<slug>
- S2 · Each new test creates its own fresh private home, seeds its own data and holds no shared mutable state or process-wide env mutation. · 5-integration-test-strategy · integration-test-strategy
  contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
  decision/chaos-home-outside-scan · One chaos home outside the scans — Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design. · why: A ratified carve-out.
  decision/ci-homes-kept-until-gates · CI homes kept until gate steps — In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run. · except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
  decision/e2e-home-base-checks · Checked base of the test homes — The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
  decision/e2e-home-headroom-floor · Headroom floor of the e2e home base — A linked e2e home base whose filesystem reads less than the 9 GiB headroom floor free is refused; the floor is one compiled constant shared by the harness keeper and the test home, set by no env, variable, flag, feature or config key; a plain base reads no headroom. · why: The tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB. · guard: harness::tests::e2e_home_*
  decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
  decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
  decision/keep-failed-home-opt-in · Failed test homes kept only on request — A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1. · why: So that logs can inspect the home.
  decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads.
  decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
  decision/owner-record-removed-last · Owner record removed last — Removing a test home deletes its owner record last. · why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
  decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
  decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint or log is shared between tests.
  decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
  decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
  decision/tempfile-test-dirs · tempfile for test directories — tempfile is what provides temporary test directories.
  decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
  decision/test-home-owner-record-sweep · Owner-record sweep of test homes — Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record. · why: A test the runner kills never drops its home. · except: A build's verify stamp dir carries no owner record; another build's stamp is swept only when its lock is taken without waiting.
  decision/test-homes-made-private · Test-made homes are private directories — A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation. · why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
  decision/tests-run-in-parallel · Tests run in parallel — Tests are never serialised with a single thread; isolation comes from the per-test home. · except: The fuzz stages are single jobs, by design.
  decision/tests-seed-own-data · self-bootstrapping tests — Tests seed their own data; no developer pre-populates a home.
- S3 · Each new test synchronises on logged events or bounded polls, never sleeps or retries. · 11-test-anti-patterns-never-do-these
  decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
  decision/key-after-resize-waits-for-size · Key after a resize waits for the new size — A test that writes a key after a resize sends it only once the child reports the new size. · guard: spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code · except: The live rig's KR arm types its key 0, 20 or 100 ms after a resize of its own pty, in its rehearsals and its live starts, since a key typed right after a resize is what it measures.
  decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
  decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
  decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
- S4 · Each new test asserts against literal oracles of the contract, never Debug strings or fixture internals. · 11-test-anti-patterns-never-do-these
  decision/assert-on-contract-not-fixture · Assert on contract, not fixture internals — Assertions target the contract, never brittle fixture-internal values.
  decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
  decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
  decision/test-public-behaviour-only · Test public API and observable behaviour — Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.
- S5 · Each new stub or fixture is a real executable or synthetic scrubbed data, never mocking what viola does not own. · mocking-stubbing-discipline · 8-mocking-stubbing-discipline
  decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern.
  decision/no-mocking-unowned-code · No mocking of what is not owned — Libraries and the language runtime are never mocked without an abstraction layer.
  decision/no-over-mocking-integration · No over-mocking at integration level — Integration tests never over-mock. · why: Over-mocking stops testing the integration.
  decision/no-over-stubbing · No over-stubbing — Tests never over-stub. · why: Over-stubbing stops testing real behaviour.
  decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for home, drive-letter and user-folder paths and usernames.
- S6 · Each new security control (allowlist, no shell) gets one negative test. · e2e-test-strategy · 6-e2e-test-strategy
  decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
- S7 · Each new log line follows the process log line format and its corr rules. · log-format
  contract/log-event-corr/channel-request · log event corr · channel-request — corr is the JSON-RPC request id; null for an id-less hook.event notification.
  contract/log-event-corr/channel-response · log event corr · channel-response — corr is the JSON-RPC request id; null on a response to a frame that yielded no id (error_code -32700 or -32600).
  contract/log-event-corr/dialog-answered · log event corr · dialog-answered — corr is the dialog_id.
  contract/log-event-corr/dialog-raised · log event corr · dialog-raised — corr is the dialog_id.
  contract/log-event-corr/hook-decision · log event corr · hook-decision — corr is the dialog_id for dialog hooks, otherwise null; a hook-decision that carries detail may be null.
  contract/log-event-corr/hook-invoked · log event corr · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked may still be null when it fails open before the dialog_id is known.
  contract/log-event-corr/panic · log event corr · panic — corr is null.
  contract/log-event-corr/parse-rejected · log event corr · parse-rejected — corr is null.
  contract/log-event-corr/process-exit · log event corr · process-exit — corr is null.
  contract/log-event-corr/process-start · log event corr · process-start — corr is null.
  contract/log-event-corr/release-from-driver · log event corr · release-from-driver — corr is the JSON-RPC request id of a release the wrapper refused with -32602 for a string from; a from of another type is a plain -32602 with no such line.
  contract/log-event-corr/send-confirmed · log event corr · send-confirmed — corr is the send cursor.
  contract/log-event-corr/send-issued · log event corr · send-issued — corr is the send cursor.
  contract/log-event-corr/send-refused · log event corr · send-refused — corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.
  contract/log-event-corr/state-recovered · log event corr · state-recovered — corr is null.
  contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
  contract/log-line/conn-and-rpc-id · process log line · conn, rpc_id — Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines.
  contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
  contract/log-line/instance · process log line · instance — A ViolaName or null.
  contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/log-line/message · process log line · message — fields.message, or the flattened message; required.
  contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
  contract/log-line/process · process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.
  contract/log-line/target · process log line · target — A required field.
  contract/log-line/timestamp · process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-exit-detail/unclean-fixture · process-exit detail · unclean-fixture — verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.
  decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.
  decision/jsonschema-validator · jsonschema validates diagnostics — jsonschema is what validates diagnostics lines against their schemas.
  decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
  decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
