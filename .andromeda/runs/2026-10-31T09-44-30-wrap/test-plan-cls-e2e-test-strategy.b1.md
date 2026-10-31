# Classified — test-plan · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · add-to · F4 adds cli_error_bodies, one driven CLI --json failure witness (one document, literal exit, empty stderr, clean body), which joins this cli verbs and --json row.
e2e-test-strategy#1.2 · no · the hook statusline row states only the assert_cmd driver, the fixture stdin and the env_clear rule, none of which the session moved.
e2e-test-strategy#1.3 · no · the outer-PTY verdict sources (events.ndjson, receipt, exit status, byte search) are unchanged, and the one check added to tests/support/events.rs is an error-body checker, not a verdict reader.
e2e-test-strategy#1.4 · yes · add-to · F3 adds channel_error_bodies, seven raw-channel fault rows against a planted wrapper with their three detail words read, which this wrapper-channel row's raw stream and jsonschema now carry.
e2e-test-strategy#1.5 · yes · add-to · F6 adds the before-initialize MAX_FRAME case and two more mcp_tools error tests beside the raw-stdin case, and F7 says rmcp reads null data as None, so the unknown-tool case asserts None.
e2e-test-strategy#1.6 · no · the library-only row names cargo-nextest and cargo test --doc only, and the new [[test]] tables are root binaries, not library-only.
e2e-test-strategy#2.1 · no · the path-1 procedure names stamped_home, the run and list verbs and the squat and tamper steps, all unchanged, and states no count or reached file.
e2e-test-strategy#2.2 · yes · stale · it cites the unit test statusline_override_of_a_start_is_rewritten_whole_at_owner_only in src/cmd/run.rs, reached whole, and the budget-gate record in src/run/budget.rs, also reached whole; the report clears neither.
e2e-test-strategy#2.3 · yes · add-to · F10 makes channel_error_bodies and cli_error_bodies root [[test]] tables with required-features fake-agent and ci kill-line terms, as path1_run_start is here; the kill-line count at :38 reads no change.
e2e-test-strategy#2.4 · no · the cleanup steps (Drop guard, TempDir removal, squatter drop) name no reached file, and the CI kept-home lifecycle sits in the untouched ci.yml.
e2e-test-strategy#3.1 · no · the MCP send comparison in mcp_tools.rs still holds, since CLI and MCP share one screen and return the same {refusal, detail} body.
e2e-test-strategy#3.2 · no · the send steps are procedure only (boot, send with stdin, mute and local, rmcp call) and name no reached file or changed verb.
e2e-test-strategy#3.3 · yes · stale · it cites the stamped paste-newline verify probe reading (18 pass, 0 fail), whose probe is reached in src/cmd/verify.rs and may sit in reached viola-0.1.0/verification-matrix.json, and the report does not clear that count.
e2e-test-strategy#3.4 · no · the cleanup line (agent-run cleanup --session p2) names no reached file and nothing it covers moved.
e2e-test-strategy#4.1 · yes · stale · it cites handlers_wait_sends_the_clamped_bound_and_maps_a_timeout in crates/viola-mcp/tests/handlers.rs, reached whole, and the background_tasks reading in reached crates/viola-agent-claude/src/lib.rs; the report clears neither.
e2e-test-strategy#4.2 · no · the wait and last steps are procedure only (boot, send, spawn wait, release, kill the chaos helper) and name no reached file.
e2e-test-strategy#4.3 · no · its signals are code words (turn-ended, instance-unreachable), and the CLI and MCP bodies keep the shared codes-only screen the report describes, so the body claims hold.
e2e-test-strategy#4.4 · no · the cleanup line (harness cleanup, pid and start-time check) names no reached file and nothing in it moved.
e2e-test-strategy#5.1 · yes · stale · its exit 12 unverified-cli refusal of an unstamped allow lives in reached src/run/version_gate.rs and its dialog behaviour in reached src/run/dialog.rs; the report clears neither.
e2e-test-strategy#5.2 · no · the dialog steps are procedure only (boot, prompt scripts, answers, pause) and state no product rule or reached file.
e2e-test-strategy#5.3 · yes · stale · its decision-body shapes and the rule that no body is emitted unless the wheel is driver and the version is stamped live in reached src/run/dialog.rs and src/run/version_gate.rs, and the report does not clear them.
e2e-test-strategy#5.4 · no · the cleanup line (harness cleanup) names no reached file and nothing it covers moved.
e2e-test-strategy#6.1 · yes · stale · it cites mcp_tools_path4_path5_answer_by_id_and_a_paused_send, and its no-release claim about MCP results and tool descriptions lives in reached crates/viola-mcp/src/result.rs and lib.rs; the report clears the CLI parity but not that negative.
e2e-test-strategy#6.2 · no · the wheel steps are procedure only (outer-PTY keys, pause, release, relaunch with the gated script) and name no reached file.
e2e-test-strategy#6.3 · yes · stale · its MCP refusal text and structured_content must not contain release, a negative claim living in reached crates/viola-mcp/src/result.rs and lib.rs, and the report does not show that release-from-driver stays off MCP text.
