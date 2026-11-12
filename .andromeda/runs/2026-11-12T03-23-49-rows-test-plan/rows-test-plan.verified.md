@row_add n=1 row=decision/agent-driven-test-layers area=test-plan-viola rec=fct-bmqjnwhn@1
name: Agent-driven test layers
rule: Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review.
except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.
@row_check n=1 row=decision/agent-driven-test-layers sha=a07f57ee5434 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:44
quote: bash scripts/agent-run.sh boot --session ci-smoke --instance builder
@row_add n=2 row=decision/comprehensive-test-tier area=1-test-scope-summary rec=fct-cyzmyvps@2
name: Comprehensive test tier
rule: The project is tested at the Comprehensive tier (2).
why: Nine attack vectors need negative tests, seven parser surfaces need property and fuzz coverage, and four drivers beyond the Standard baseline push the tier up.
@row_check n=2 row=decision/comprehensive-test-tier sha=fc6eb81e50e6 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=3 row=decision/tests-run-on-linux-ubuntu-latest area=1-test-scope-summary rec=fct-cyzmyvps@2,fct-oftlgaxh@2
name: Tests run on Linux
rule: The test suites run on Linux, with CI on the ubuntu-latest leg.
@row_check n=3 row=decision/tests-run-on-linux-ubuntu-latest sha=062639d15143 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
@row_add n=4 row=decision/chaos-tests-for-self-healing-state area=1-test-scope-summary rec=fct-eij3zkzc@1,fct-cyzmyvps@2
name: Chaos tests for self-healing state
rule: The crash-safe self-healing state is covered by chaos tests.
why: A driver of the Comprehensive tier.
@row_check n=4 row=decision/chaos-tests-for-self-healing-state sha=409f28e35292 v=holds judge=sonnet head=8834195b0683
at: tests/chaos_torn_append.rs:1
quote: //! Chaos (test-plan §6 Chaos suite, torn append): a log whose last line was cut short costs the
@row_add n=5 row=decision/performance-budget-tests-for-hook-deadlines area=1-test-scope-summary rec=fct-zvdpu2dw@1,fct-cyzmyvps@2
name: Performance-budget tests for hook deadlines
rule: Hook deadlines are covered by performance-budget tests.
why: A driver of the Comprehensive tier.
@row_check n=5 row=decision/performance-budget-tests-for-hook-deadlines sha=d203808f85cd v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:129
quote: /// is judged on hyperfine's `max` sample, the gated statistic (test-plan §10; obs-plan §10).
at: crates/viola-e2e/src/harness/run/perf.rs:21
quote: pub const ROWS: [&str; 5] = [
@row_add n=6 row=decision/compat-tests-for-cli-versions area=1-test-scope-summary rec=fct-ih6wd2ud@1,fct-cyzmyvps@2
name: Compatibility tests across CLI versions
rule: Multi-version Claude CLI fixtures are covered by compatibility tests.
why: A driver of the Comprehensive tier.
@row_check n=6 row=decision/compat-tests-for-cli-versions sha=1833b3b6488d v=holds judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:139
quote: fn fake_agent_print_turn_matches_every_recorded_set() {
at: tests/contract_fake_agent_drift.rs:253
quote: fn fake_agent_dialog_replay_matches_every_recorded_dialog_set() {
@row_add n=7 row=decision/mutation-testing area=1-test-scope-summary rec=fct-4k247nk7@1,fct-cyzmyvps@2
name: Mutation testing
rule: Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit.
why: A mandated driver of the Comprehensive tier.
@row_check n=7 row=decision/mutation-testing sha=a994d1057a86 v=unverifiable judge=sonnet head=8834195b0683
searched: andromeda-code-audit · in src, crates, scripts, tests, .github
why: not a code claim
@row_add n=8 row=decision/no-compliance-verification area=1-test-scope-summary rec=fct-pvam5xzl@1
name: No compliance verification
rule: Compliance verification, the one Comprehensive-tier component, is not applied.
why: The security excerpt names no compliance triggers.
@row_check n=8 row=decision/no-compliance-verification sha=c366e6db9e12 v=holds judge=sonnet head=8834195b0683
searched: compliance (case-insensitive) · in src/**, crates/**, tests/**, scripts/**, .github/**
@row_add n=9 row=decision/cross-user-tested-at-peer-check area=1-test-scope-summary rec=fct-qg6j3l2y@2,fct-upolcsa3@1
name: Cross-user attacks tested at the peer-credential check
rule: Cross-user peer rejection is asserted at the peer-credential check over injected euids, the real euid read on Linux and the wrapper's drop over an injected decision; no test seam is added, so each client seam's euid wiring has no committed test.
why: Hosted runners provide one account; the trade is that a mutant making a seam's peer-admission read always admit survives every committed test.
@row_check n=9 row=decision/cross-user-tested-at-peer-check sha=b53d81286a4f v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:517
quote: fn peer_euid_admitted_takes_only_the_owning_user(
at: crates/viola-channel/src/client.rs:534
quote: assert!(!peer_admitted(Some(own.wrapping_add(1))));
at: crates/viola-channel/src/server.rs:1043
quote: .serve_admitting(Arc::clone(&dispatched) as Arc<dyn Dispatch>, admitted);
searched: peer_admitted · in tests/**/*.rs (no hit; the seams src/cmd/client.rs:177, src/cmd/hook.rs:236, crates/viola-mcp/src/tools.rs:352 are read by no committed test)
@row_add n=10 row=contract/mcp-tools-list/tool-set area=1-test-scope-summary rec=fct-rmbr6xeu@1,fct-apiugjwf@1
boundary: MCP tools/list
item: tool set
promise: tools/list is exactly the five tools send, wait, last, answer and list, never release, pause, link or unlink.
@row_check n=10 row=contract/mcp-tools-list/tool-set sha=f1501cd0e551 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=11 row=contract/viola-hook/exit-code area=1-test-scope-summary rec=fct-paewrdc5@2,fct-hyr7b53t@2
boundary: viola hook exec-form entry points
item: exit code
promise: viola hook <event> and viola hook statusline always exit 0.
@row_check n=11 row=contract/viola-hook/exit-code sha=c7689f61bcb8 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=12 row=contract/viola-hook/stderr area=1-test-scope-summary rec=fct-paewrdc5@2,fct-hyr7b53t@2
boundary: viola hook exec-form entry points
item: stderr
promise: viola hook never writes to stderr.
@row_check n=12 row=contract/viola-hook/stderr sha=4b7d29565938 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
searched: eprint · in src/cmd/hook.rs
@row_add n=13 row=contract/viola-hook/deadline area=1-test-scope-summary rec=fct-hyr7b53t@2
boundary: viola hook exec-form entry points
item: deadline
promise: Spine hooks finish within the spine deadline, and SessionEnd within about 1 s.
@row_check n=13 row=contract/viola-hook/deadline sha=b2e5decfbc1b v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
at: src/cmd/hook.rs:59
quote: started + CONNECT_DEADLINE
@row_add n=14 row=contract/viola-hook-statusline/budget-json area=1-test-scope-summary rec=fct-paewrdc5@2,fct-tcg4lsy4@2
boundary: viola hook statusline
item: budget.json
promise: rate_limits is written to budget.json only when the payload holds a rate_limits object.
@row_check n=14 row=contract/viola-hook-statusline/budget-json sha=7f7d8c9e912c v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:32
quote: let Some(Value::Object(limits)) = payload.get("rate_limits") else {
at: src/cmd/hook/statusline.rs:126-127
quote: let budget_written =         reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
@row_add n=15 row=contract/viola-hook-statusline/passthrough area=1-test-scope-summary rec=fct-paewrdc5@2,fct-tcg4lsy4@2
boundary: viola hook statusline
item: passthrough
promise: The user's statusline is passed through unchanged, after budget.json is written.
@row_check n=15 row=contract/viola-hook-statusline/passthrough sha=78cfa9aeecae v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:127
quote: reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
@row_add n=16 row=decision/r8-claude-env-strip area=1-test-scope-summary rec=fct-u3dkdc2w@3,fct-otwmx5qk@1
name: R8 CLAUDE* environment strip
rule: Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including the 11-name identity floor.
except: Persistent-environment names listed in config.json claude_env_keep are kept.
@row_check n=16 row=decision/r8-claude-env-strip sha=9979f0fbc614 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:69
quote: if !text.starts_with("CLAUDE") {
at: crates/viola-agent-claude/src/lib.rs:72-73
quote: let floor = IDENTITY_FLOOR.contains(&text.as_str()); if !floor && persistent.contains(&text) {
at: crates/viola-agent-claude/src/lib.rs:17
quote: pub const IDENTITY_FLOOR: [&str; 11] = [
@row_add n=17 row=decision/cargo-deny-tokio-and-c-crate-bans area=1-test-scope-summary rec=fct-elcdxxgr@2
name: Tokio and C-crate bans
rule: The dependency policy bans tokio where it is not allowed and bans C crates.
guard: cargo deny
@row_check n=17 row=decision/cargo-deny-tokio-and-c-crate-bans sha=cd0f1858e6cd v=holds guard=true judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
guard_at: .github/workflows/ci.yml:367
@row_add n=18 row=decision/clippy-deny-warnings area=1-test-scope-summary rec=fct-elcdxxgr@2
name: Clippy warnings are errors
rule: Clippy runs with warnings denied.
guard: clippy -D warnings
@row_check n=18 row=decision/clippy-deny-warnings sha=92d1ff61d85d v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
guard_at: .github/workflows/ci.yml:281
@row_add n=19 row=decision/cargo-modules-orphans-gate area=1-test-scope-summary rec=fct-elcdxxgr@2
name: No orphan modules
rule: Every lib and bin target is checked for orphan modules.
guard: cargo-modules orphans gate per lib/bin target
@row_check n=19 row=decision/cargo-modules-orphans-gate sha=064691dea5fc v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:314
quote: run: bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh
guard_at: .github/workflows/ci.yml:314
@row_add n=20 row=decision/acyclic-boundary-review area=1-test-scope-summary rec=fct-elcdxxgr@2
name: On-demand acyclic boundary review
rule: Module boundaries are reviewed for cycles on demand rather than gated on every run.
@row_check n=20 row=decision/acyclic-boundary-review sha=a15cce010bae v=holds guard=false judge=sonnet head=8834195b0683
at: scripts/orphans-check.sh:6-7
quote: # `dependencies --acyclic` is deliberately absent: at cargo-modules 0.27.0 every type with an inherent method # reads as a type<->method cycle whatever the filters, so it stays an on-demand review
@row_add n=21 row=decision/cli-tests-as-subprocess area=1-test-scope-summary rec=fct-a6gfb3bv@2
name: CLI tested as a subprocess
rule: The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
@row_check n=21 row=decision/cli-tests-as-subprocess sha=24dfc76dcdab v=holds judge=sonnet head=8834195b0683
at: tests/support/cli.rs:108
quote: let mut command = Command::new(VIOLA);
at: tests/support/cli.rs:110
quote: .arg("--home")
at: tests/cli_machine_contract.rs:157
quote: args: &["list", "--json"],
@row_add n=22 row=contract/cli-json-output/stdout-document area=1-test-scope-summary rec=fct-a6gfb3bv@2
boundary: CLI --json output
item: stdout document
promise: Under --json a verb prints one JSON document on stdout.
@row_check n=22 row=contract/cli-json-output/stdout-document sha=0b4b740a6cd6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:220-221
quote: pub(crate) fn document(doc: &Value) { let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
@row_add n=23 row=contract/cli-json-output/answer-send-exit-2 area=1-test-scope-summary rec=fct-a6gfb3bv@2
boundary: CLI --json output
item: exit 2 of answer and send
promise: On the exit-2 unusable paths of answer and send, a fixed stderr line is printed and no JSON document.
@row_check n=23 row=contract/cli-json-output/answer-send-exit-2 sha=92e67d7c7872 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:164-165
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes()); return Ok(ExitCode::from(2));
at: src/cmd/answer.rs:91-92
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes()); return Ok(ExitCode::from(2));
@row_add n=24 row=contract/cli-human-output/no-sgr area=1-test-scope-summary rec=fct-a6gfb3bv@2
boundary: CLI human output
item: SGR
promise: Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
@row_check n=24 row=contract/cli-human-output/no-sgr sha=25422917ec68 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62-66
quote: if facts.json         || facts.passthrough         || facts.no_color         || facts.term.as_deref() == Some("dumb")         || !facts.stdout_tty
@row_add n=25 row=decision/no-child-screen-assertions area=1-test-scope-summary rec=fct-a6gfb3bv@2,fct-pbrzczk2@1
name: No assertions on the child's screen
rule: TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content.
except: The forced vt100 panic case is checked by a byte search of the outer terminal.
@row_check n=25 row=decision/no-child-screen-assertions sha=771aa86e5180 v=holds judge=sonnet head=8834195b0683
at: tests/chaos_feed_panic.rs:123
quote: for literal in ["panic", "parse-rejected", "vt100", "unable", "hint:"] {
searched: Parser::new · in tests/**/*.rs
searched: .screen() · in tests/**/*.rs
searched: contents() · in tests/**/*.rs
@row_add n=26 row=decision/readiness-gate-tested-by-refusals area=1-test-scope-summary rec=fct-pbrzczk2@1
name: Readiness gate tested by its refusals
rule: The readiness gate of viola run is tested through its refusal outputs.
@row_check n=26 row=decision/readiness-gate-tested-by-refusals sha=347e4d1137db v=holds judge=sonnet head=8834195b0683
at: tests/cli_send.rs:1074
quote: fn send_on_a_verified_cli_refuses_while_the_trust_dialog_is_up(stamped_home: StampedHome) {
at: tests/cli_send.rs:1079
quote: assert_eq!(sent.code, Some(13));
@row_add n=27 row=decision/ipc-tests-schema-asserted area=1-test-scope-summary rec=fct-b7vg2zil@1,fct-apiugjwf@1
name: IPC surfaces tested by schema-asserting clients
rule: The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
@row_check n=27 row=decision/ipc-tests-schema-asserted sha=71542b36a91a v=holds judge=sonnet head=8834195b0683
at: tests/channel_endpoint.rs:247
quote: let validator = jsonschema::validator_for(&schema).expect("schema");
at: crates/viola-e2e/tests/mcp_tools.rs:501
quote: let validator = jsonschema::validator_for(&schema).expect("valid schema");
at: tests/cli_wait_last.rs:198
quote: assert_eq!(woken["ok"]["cursor"], turn_end);
@row_add n=28 row=decision/mcp-negative-inputs-violate-schema area=1-test-scope-summary rec=fct-apiugjwf@1
name: MCP negative inputs violate the schema
rule: Negative MCP test inputs must violate the advertised tool schema.
why: The advertised schema is advisory.
@row_check n=28 row=decision/mcp-negative-inputs-violate-schema sha=291f4a50c25c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/tests/mcp_tools.rs:314
quote: mcp.call("send", json!({"target": "Builder!", "text": CANARY})),
at: crates/viola-e2e/tests/mcp_tools.rs:317
quote: json!({"target": "builder", "dialog_id": "1", "response": {"behavior": "allow"}}),
@row_add n=29 row=decision/single-test-only-crate area=1-test-scope-summary rec=fct-oftlgaxh@2
name: One test-only workspace member
rule: The workspace has exactly one test-only member, crates/viola-e2e, never published.
@row_check n=29 row=decision/single-test-only-crate sha=a64f3b065156 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:7
quote: publish.workspace = true
at: Cargo.toml:190
quote: publish = false
@row_add n=30 row=decision/fake-agent-and-verify-fixtures-are-contract area=1-test-scope-summary rec=fct-l7edbh4k@2
name: Fake agent and verify fixtures are the contract
rule: The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to.
why: A binding founder direction.
@row_check n=30 row=decision/fake-agent-and-verify-fixtures-are-contract sha=6fcb95041082 v=holds judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:1-2
quote: //! The fake agent's drift contract (test-plan §6 Contract suite, §7 Fake agent): for every recorded //! `fixtures/claude/<version>/` set, a print-mode turn fires the spine hooks in the recorded order
@row_add n=31 row=decision/structured-json-process-logs area=1-test-scope-summary rec=fct-l7edbh4k@2,fct-yp3slbux@1
name: Structured JSON logs from every process
rule: Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/.
why: A binding founder direction.
@row_check n=31 row=decision/structured-json-process-logs sha=9f52e54c953c v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:56-57
quote: (ObsProcess::Mcp, _) => "mcp".to_owned(), (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => {
at: src/obs.rs:121
quote: .json()
at: crates/viola-e2e/src/bin/viola-harness.rs:58
quote: Logs {
at: src/obs.rs:87
quote: let dir = home.join("diagnostics");
@row_add n=32 row=contract/test-harness/boot area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: boot
promise: boot builds viola and starts the wrappers in a fresh home to readiness.
@row_check n=32 row=contract/test-harness/boot sha=448d177f68c6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:142
quote: && let Err(code) = cargo_build(&opts.ws)
at: crates/viola-e2e/src/harness/boot.rs:200
quote: match wait_ready(&home, inst, Instant::now() + INSTANCE_DEADLINE) {
@row_add n=33 row=contract/test-harness/run area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: run
promise: run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives.
@row_check n=33 row=contract/test-harness/run sha=94b37bceb61a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:152
quote: let ok = refusal.is_none() && suites.iter().all(Suite::green);
at: crates/viola-e2e/src/harness/run.rs:307
quote: if sel.mutants && refusal.is_none() {
@row_add n=34 row=contract/test-harness/status area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: status
promise: status queries viola list --json.
@row_check n=34 row=contract/test-harness/status sha=af317e25ff14 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:57
quote: .args(["list", "--json"])
@row_add n=35 row=contract/test-harness/cleanup area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: cleanup
promise: cleanup stops every wrapper and deletes the home, idempotently.
@row_check n=35 row=contract/test-harness/cleanup sha=48e11c0855d9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:91
quote: let _ = fs::write(dir.join("stop.request"), b"");
at: crates/viola-e2e/src/harness/cleanup.rs:116
quote: json!(remove_session_home(ws, &record.home))
at: crates/viola-e2e/src/harness/cleanup.rs:53-54
quote: return Outcome::new(             json!({"v": 1, "cmd": "cleanup", "ok": true, "cleaned": []}),
@row_add n=36 row=contract/test-harness/logs area=1-test-scope-summary rec=fct-tfd2itkl@1
boundary: test harness commands
item: logs
promise: logs prints the per-process logs as JSON lines.
@row_check n=36 row=contract/test-harness/logs sha=f9fe9e80d5da v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:214-215
quote: for line in lines {                 println!("{line}");
@row_add n=37 row=contract/viola-list-json/items area=1-test-scope-summary rec=fct-hlmuruc2@1
boundary: viola list --json
item: ok.items
promise: In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, budget_paused?, dialog_pending, cli_version?, cli_verified?}.
@row_check n=37 row=contract/viola-list-json/items sha=c97f6f875d0c v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:169
quote: pub wrapped: bool,
at: crates/viola-state/src/board.rs:170-171
quote: pub liveness: RowLiveness, pub status: SessionStatus,
at: crates/viola-state/src/board.rs:173
quote: pub wheel: Option<Wheel>,
at: crates/viola-state/src/board.rs:175-176
quote: pub budget_paused: Option<bool>, pub dialog_pending: bool,
at: crates/viola-state/src/board.rs:178
quote: pub cli_version: Option<String>,
at: crates/viola-state/src/board.rs:180
quote: pub cli_verified: Option<bool>,
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
@row_add n=38 row=contract/viola-list-json/mirrors-channel area=1-test-scope-summary rec=fct-hlmuruc2@1
boundary: viola list --json
item: channel mirror
promise: The CLI --json document mirrors the channel list result.
@row_check n=38 row=contract/viola-list-json/mirrors-channel sha=891a0a499d0a v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
@row_add n=39 row=contract/event-line/format area=1-test-scope-summary rec=fct-yp3slbux@1
boundary: the event line
item: format
promise: Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
@row_check n=39 row=contract/event-line/format sha=78f28b896b39 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:32
quote: pub v: u32,
at: crates/viola-state/src/events.rs:33
quote: pub ts: String,
at: crates/viola-state/src/events.rs:34
quote: pub instance: String,
at: crates/viola-state/src/events.rs:36
quote: pub kind: EventKind,
at: crates/viola-state/src/events.rs:37
quote: pub source: Source,
at: crates/viola-state/src/events.rs:38
quote: pub data: Value,
@row_add n=40 row=contract/event-line/source area=1-test-scope-summary rec=fct-yp3slbux@1
boundary: the event line
item: source
promise: source is one of hook or wrapper.
@row_check n=40 row=contract/event-line/source sha=b499c56e8ad9 v=stale judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:25-28
quote: pub enum Source { Wrapper, Hook, }
shows: The event-line Source enum has only Wrapper and Hook; no code emits source "cli".
cell: promise
value: source is one of hook or wrapper.
@row_add n=41 row=contract/heartbeat-file/interval area=1-test-scope-summary rec=fct-w7y4wg56@1
boundary: instance heartbeat file
item: touch interval
promise: The instance heartbeat file is touched every 1 s.
@row_check n=41 row=contract/heartbeat-file/interval sha=e21a46ffdcc9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/heartbeat.rs:13
quote: pub const BEAT_EVERY: Duration = Duration::from_secs(1);
at: crates/viola-state/src/heartbeat.rs:41-42
quote: while let Err(RecvTimeoutError::Timeout) = stopped.recv_timeout(BEAT_EVERY) { let _ = touch_heartbeat(&instance_dir);
@row_add n=42 row=contract/instance-liveness/gone area=1-test-scope-summary rec=fct-w7y4wg56@1
boundary: instance liveness
item: gone
promise: An instance whose pid and start time no longer name a running process is gone, whatever the heartbeat's age.
@row_check n=42 row=contract/instance-liveness/gone sha=5ceba028b213 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:28-29
quote: if !same_process { return Liveness::Gone;
@row_add n=43 row=contract/instance-liveness/live area=1-test-scope-summary rec=fct-w7y4wg56@1
boundary: instance liveness
item: live
promise: An instance whose process runs and whose heartbeat is at most 5 s old is live.
@row_check n=43 row=contract/instance-liveness/live sha=061ffdf630b8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
at: crates/viola-state/src/liveness.rs:16
quote: pub const STALE_AFTER: Duration = Duration::from_secs(5);
@row_add n=44 row=contract/instance-liveness/stale area=1-test-scope-summary rec=fct-w7y4wg56@1
boundary: instance liveness
item: stale
promise: An instance whose process runs but whose heartbeat is older than 5 s or absent is stale.
@row_check n=44 row=contract/instance-liveness/stale sha=115cad4646bb v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:33
quote: _ => Liveness::Stale,
@row_add n=45 row=decision/scrubbed-fixtures area=1-test-scope-summary rec=fct-yeykk4zl@1
name: Fixtures free of real data
rule: Fixtures never come from real prompts and are checked for absolute paths and usernames.
@row_check n=45 row=decision/scrubbed-fixtures sha=1baa8a21500e v=holds judge=sonnet head=8834195b0683
at: tests/contract_fixture_hygiene.rs:18
quote: PLACEHOLDER_USER, Violation, check, has_absolute_path, has_username, host_user, load_schema,
@row_add n=46 row=decision/property-fuzz-for-parsers area=1-test-scope-summary rec=fct-yeykk4zl@1,fct-cyzmyvps@2
name: Property and fuzz tests for parsers
rule: Property and fuzz generators cover the parser surfaces.
@row_check n=46 row=decision/property-fuzz-for-parsers sha=f42bbcc1e83e v=holds judge=sonnet head=8834195b0683
at: fuzz/fuzz_targets/paste_text.rs:22
quote: fuzz_target!(|bytes: &[u8]| {
at: crates/viola-agent-claude/src/hook.rs:893
quote: proptest! {
@row_add n=47 row=contract/viola-run/start-refusal-exit-1 area=1-test-scope-summary rec=fct-2ls6kelo@2
boundary: viola run start
item: exit 1
promise: A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
@row_check n=47 row=contract/viola-run/start-refusal-exit-1 sha=83e4dd7aa01a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:282
quote: return Ok(refused_with(
at: src/cmd/run.rs:173
quote: return Ok(ExitCode::from(1));
at: src/cmd/run.rs:413
quote: Started::Refused(ExitCode::from(1))
@row_add n=48 row=contract/driver-send/send-issued area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: send-issued
promise: A driver send logs a send-issued event.
@row_check n=48 row=contract/driver-send/send-issued sha=35185c9f6d1a v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:186
quote: EventKind::SendIssued,
at: src/run/send.rs:147
quote: let cursor = ctx.issue(text.len())?;
@row_add n=49 row=contract/driver-send/typed-text area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: typed text
promise: The typed text is written as one bracketed paste plus Enter, with CR LF and CR turned to LF and trailing CR and LF dropped.
@row_check n=49 row=contract/driver-send/typed-text sha=f74562eb5c72 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232-234
quote: let text = text.trim_end_matches(['\r', '\n']);     if text.contains('\r') {         Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: crates/viola-pty/src/pump.rs:79
quote: bytes.extend_from_slice(b"\x1b[200~");
at: crates/viola-pty/src/pump.rs:81
quote: bytes.extend_from_slice(b"\x1b[201~\r");
@row_add n=50 row=contract/driver-send/confirmation area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: confirmation
promise: A send is confirmed by the matching prompt-submitted with origin driver, else refused not-delivered or no-prompt-submitted.
@row_check n=50 row=contract/driver-send/confirmation sha=cc665e64d245 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:154-156
quote: match slot.confirm(slot.clock.now()) {         Some(submitted_at) => ctx.confirm(cursor, &submitted_at, slot.clock.now()),         None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/run/send/slot.rs:203
quote: line.data["origin"] = json!("driver");
@row_add n=51 row=contract/driver-send/empty-text area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: empty-text
promise: An empty typed text is refused empty-text.
@row_check n=51 row=contract/driver-send/empty-text sha=3963f7f86871 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:93-94
quote: if text.is_empty() {         return ctx.not_delivered(None, NotDelivered::EmptyText);
@row_add n=52 row=contract/driver-send/local-command area=1-test-scope-summary rec=fct-bmeigurx@1
boundary: driver send
item: ledger-listed local command
promise: A ledger-listed local command is matched by exact equality and confirmed only by its post-condition, else reported unconfirmable, never presumed delivered.
@row_check n=52 row=contract/driver-send/local-command sha=ad1f784c645d v=holds judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:28
quote: match LOCAL_COMMANDS.iter().find(|(command, _)| *command == text) {
at: src/run/send/classify.rs:30-31
quote: Some((_, Some(post))) if cli_verified => Confirms::Post(*post),         Some(_) => Confirms::Nothing,
at: src/run/send.rs:151-152
quote: if matches!(confirms, Confirms::Nothing) {         return ctx.unconfirmable(cursor, slot.clock.now());
@row_add n=53 row=contract/wait/wake-kinds area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: wait
item: wake kinds
promise: wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it.
@row_check n=53 row=contract/wait/wake-kinds sha=7bb2a64db569 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:99-106
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired, ];
at: src/run/wait.rs:201-203
quote: if EventKind::WAIT_WAKE .iter() .any(|k| Some(k.as_str()) == kind)
@row_add n=54 row=contract/wait/already-logged area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: wait
item: after cursor
promise: wait returns at once when a waking event is already logged at or after the after cursor.
@row_check n=54 row=contract/wait/already-logged sha=ddb3eef41d40 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:153
quote: if let Some(line) = scan(instance_dir, &mut from)? {
@row_add n=55 row=contract/wait/timeout area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: wait
item: timeout
promise: wait times out with {timed_out:true}.
@row_check n=55 row=contract/wait/timeout sha=e45516916026 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:161
quote: return Ok(json!({"ok": {"timed_out": true}}));
@row_add n=56 row=contract/wait/instance-unreachable area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: wait
item: unreachable wrapper
promise: A wrapper killed mid-wait gives CLI exit 21 and MCP instance-unreachable.
@row_check n=56 row=contract/wait/instance-unreachable sha=cc8c0d302e96 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => { return Ok(Err(Unreachable::Lost));
at: src/cmd/client.rs:265
quote: 21
at: crates/viola-mcp/src/result.rs:54-55
quote: Err(ChannelError::Closed | ChannelError::Io(_)) => { return Ok(unreachable(UnreachableCause::EndpointLost));
@row_add n=57 row=contract/last/result area=1-test-scope-summary rec=fct-ruqul2ub@1
boundary: last
item: result
promise: last returns {last_assistant_message, ts} from the newest turn-ended event.
@row_check n=57 row=contract/last/result sha=0262789683f0 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:177
quote: Ok(json!({"ok": {"last_assistant_message": message, "ts": ts}}))
@row_add n=58 row=contract/dialog/one-pending area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook dialogs
item: one pending dialog
promise: Exactly one dialog is pending per instance; a second concurrent dialog gets a null response.
@row_check n=58 row=contract/dialog/one-pending sha=77fcde77fdfa v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:251-254
quote: let hold = self.cli_verified && state.pending.is_none() && !unanswerable && self.wheel.holder() == Wheel::Driver;
at: src/run/dialog.rs:271-272
quote: if !hold { return Ok(reply(dialog_id, None));
@row_add n=59 row=contract/dialog/event-logged-once area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook dialogs
item: dialog event
promise: A question, permission or plan event is logged exactly once with a dialog_id, and wait returns it at once.
@row_check n=59 row=contract/dialog/event-logged-once sha=fede44a841b4 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:288
quote: data.insert("dialog_id".to_owned(), json!(dialog_id));
at: src/run/dialog.rs:302-304
quote: let appended = self.feed.appending_dialog(hold, || { append_event_at(&self.instance_dir, |_| line.clone()) });
at: src/run/wait.rs:201-203
quote: if EventKind::WAIT_WAKE .iter() .any(|k| Some(k.as_str()) == kind)
@row_add n=60 row=contract/hook-decision-body/question area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook decision body
item: question
promise: A question is answered with PreToolUse allow plus updatedInput.answers, annotations included.
@row_check n=60 row=contract/hook-decision-body/question sha=8ba60bf72570 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:359-364
quote: let mut updated = input.as_object().cloned().unwrap_or_default(); updated.insert("answers".to_owned(), json!(answers)); if let Some(annotations) = annotations { updated.insert("annotations".to_owned(), Value::Object(annotations.clone())); } pre_tool_use_allow(Some(Value::Object(updated)))
at: crates/viola-agent-claude/src/dialog.rs:395
quote: let mut output = json!({"hookEventName": "PreToolUse", "permissionDecision": "allow"});
@row_add n=61 row=contract/hook-decision-body/permission area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook decision body
item: permission
promise: A permission is answered with PermissionRequest allow or deny.
@row_check n=61 row=contract/hook-decision-body/permission sha=4c7b8681cebf v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:382
quote: (HookEvent::PermissionRequest, Response::Permission { behavior, message }) => {
at: crates/viola-agent-claude/src/dialog.rs:384
quote: Permit::Allow => "allow",
at: crates/viola-agent-claude/src/dialog.rs:385
quote: Permit::Deny => "deny",
at: crates/viola-agent-claude/src/dialog.rs:387
quote: permission_decision(behavior, message.as_deref())
@row_add n=62 row=contract/hook-decision-body/plan area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: hook decision body
item: plan
promise: A plan is approved through PreToolUse only and revised through PermissionRequest deny plus message.
@row_check n=62 row=contract/hook-decision-body/plan sha=482932b902d0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:369
quote: behavior: Verdict::Approve,
at: crates/viola-agent-claude/src/dialog.rs:372
quote: ) => pre_tool_use_allow(Some(Value::Object(
at: crates/viola-agent-claude/src/dialog.rs:378
quote: behavior: Verdict::Revise,
at: crates/viola-agent-claude/src/dialog.rs:381
quote: ) => permission_decision("deny", message.as_deref()),
@row_add n=63 row=contract/cli-exit-codes/13 area=1-test-scope-summary rec=fct-5f5m22qs@1
boundary: CLI exit codes
item: 13
promise: An unknown dialog_id given to answer exits 13 unknown-dialog.
@row_check n=63 row=contract/cli-exit-codes/13 sha=f96116b327c4 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:1169
quote: Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
at: src/cmd/answer.rs:142
quote: exit_of(reason_of(&reason))
@row_add n=64 row=decision/human-takes-wheel-on-edit area=1-test-scope-summary rec=fct-nzaflfcl@1
name: Human editing takes the wheel
rule: A human editing key since the last turn boundary moves the wheel to the human, and a harness-injected turn does not flip the wheel.
@row_check n=64 row=decision/human-takes-wheel-on-edit sha=bb93e84abe6b v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:322-323
quote: if self.keys.feed(&buf[..n]) { self.wheel.human_input();
at: src/run/send/slot.rs:219-220
quote: if line.kind == EventKind::PromptSubmitted && line.data["origin"] == "human" { slot.wheel.human_input();
@row_add n=65 row=contract/driver-send/human-typing area=1-test-scope-summary rec=fct-nzaflfcl@1
boundary: driver send
item: human-typing
promise: A driver send while the human holds the wheel is refused human-typing.
@row_check n=65 row=contract/driver-send/human-typing sha=fb34e2f5d1bb v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:96
quote: if let Some(detail) = slot.wheel.human_typing() {
at: src/run/send.rs:99
quote: RefusalReason::HumanTyping,
at: src/run/wheel.rs:113
quote: (held.holder == Wheel::Human)
@row_add n=66 row=decision/pause-forces-human-wheel area=1-test-scope-summary rec=fct-nzaflfcl@1
name: Pause forces the human wheel
rule: pause moves the wheel to the human manually.
@row_check n=66 row=decision/pause-forces-human-wheel sha=d714e3a85e14 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:44
quote: (Input::Pause, _, _) => Some((Wheel::Human, WheelCause::ManualPause)),
@row_add n=67 row=decision/wheel-returns-only-via-cli-release area=1-test-scope-summary rec=fct-nzaflfcl@1
name: Wheel returns only through viola release
rule: The wheel returns from the human only through the CLI viola release, never through a release carrying from.
@row_check n=67 row=decision/wheel-returns-only-via-cli-release sha=4c620cda034b v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: src/run/wheel.rs:45
quote: (Input::Release, Wheel::Human, _) => Some((Wheel::Driver, WheelCause::Release)),
@row_add n=68 row=decision/budget-pause-thresholds area=1-test-scope-summary rec=fct-tcg4lsy4@2
name: Budget pause thresholds
rule: Crossing five_hour at 90 % or more, or seven_day at 85 % or more, refuses automated send with budget-paused, and release --budget overrides it.
@row_check n=68 row=decision/budget-pause-thresholds sha=5bead318ff2e v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:30
quote: const FIVE_HOUR_DEFAULT: u8 = 90;
at: src/run/budget.rs:31
quote: const SEVEN_DAY_DEFAULT: u8 = 85;
at: src/run/budget.rs:157
quote: (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
at: src/run/send/budget.rs:11
quote: RefusalReason::BudgetPaused,
at: src/run/wheel.rs:214
quote: let lifted = gate.lift().map_err(|_| ProtocolError::Internal)?;
@row_add n=69 row=decision/unverified-cli-gate area=1-test-scope-summary rec=fct-yl2sew2w@1
name: Unverified CLI version gate
rule: On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli.
why: A non-null dialog decision needs a viola verify stamp.
@row_check n=69 row=decision/unverified-cli-gate sha=f65dff3a83d2 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:251
quote: let hold = self.cli_verified
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
@row_add n=70 row=contract/hook-dialog/unverified-null area=1-test-scope-summary rec=fct-yl2sew2w@1
boundary: channel hook.dialog
item: unverified CLI
promise: On an unverified CLI version hook.dialog returns null at once, so the hook's stdout is empty.
@row_check n=70 row=contract/hook-dialog/unverified-null sha=f0b17155bc19 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:251
quote: let hold = self.cli_verified
at: src/run/dialog.rs:271-272
quote: if !hold {             return Ok(reply(dialog_id, None));
@row_add n=71 row=contract/cli-exit-codes/12 area=1-test-scope-summary rec=fct-yl2sew2w@1
boundary: CLI exit codes
item: 12
promise: answer on an unverified CLI version exits 12.
@row_check n=71 row=contract/cli-exit-codes/12 sha=44f73236d2e7 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
@row_add n=72 row=contract/mcp-answer/unverified-cli area=1-test-scope-summary rec=fct-yl2sew2w@1
boundary: MCP answer tool
item: unverified-cli
promise: MCP answer on an unverified CLI version gives isError:true with "refused: unverified-cli".
@row_check n=72 row=contract/mcp-answer/unverified-cli sha=49bb6341896b v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified {             return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: crates/viola-mcp/src/result.rs:40-41
quote: let mut result = CallToolResult::structured_error(body);     result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=73 row=decision/unwrapped-hooks-silent-noop area=1-test-scope-summary rec=fct-54ym6y3r@1
name: Unwrapped session hooks are a no-op
rule: A viola hook fired in a session viola does not wrap is a silent no-op.
@row_check n=73 row=decision/unwrapped-hooks-silent-noop sha=42d8b3347c85 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=74 row=decision/ipc-vector-negative-tests area=1-test-scope-summary rec=fct-3znwz3ga@1
name: IPC security vector negative tests
rule: The IPC vector is covered by negative tests for oversize frames, higher protocol versions, server pid or start-time mismatch, squatted endpoints, socket permissions, foreign owners, peers of another euid and inherited channel handles.
@row_check n=74 row=decision/ipc-vector-negative-tests sha=6a682463ea24 v=holds judge=sonnet head=8834195b0683
at: tests/channel_endpoint.rs:243
quote: .any(|l| l["error_code"] == -32600 && l.get("corr").is_none())
at: tests/channel_error_bodies.rs:112
quote: let newer = r#""error":{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}"#;
at: tests/security_negatives_server_identity.rs:88
quote: fn served(label: &str, recorded: u32) -> Served {
at: tests/cli_instance_state.rs:264
quote: fn run_refuses_a_squatted_endpoint(stamped_home: StampedHome) {
at: tests/channel_endpoint.rs:262-263
quote: assert_eq!(mode(socket.parent().expect("socket dir")), 0o700); assert_eq!(mode(socket), 0o600);
at: crates/viola-state/src/strict.rs:310
quote: fn socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit(
at: crates/viola-channel/src/server.rs:1033
quote: fn serve_drops_a_refused_peer_before_any_frame(
at: tests/tui_channel_fds.rs:19
quote: fn tui_channel_fds_the_child_holds_nothing_of_violas(booted_wrapper: Wrapper) {
@row_add n=75 row=contract/channel-errors/32600 area=1-test-scope-summary rec=fct-3znwz3ga@1
boundary: channel errors
item: -32600
promise: A frame over MAX_FRAME is answered with -32600 and the connection is closed.
@row_check n=75 row=contract/channel-errors/32600 sha=0d93c677d144 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:162
quote: Err(ChannelError::Oversize) => Some(refuse_oversize(srv_conn)),
at: crates/viola-channel/src/server.rs:268-269
quote: frame: error_response(None, ProtocolError::InvalidRequest),         close: true,
at: crates/viola-channel/src/lib.rs:72
quote: Self::InvalidRequest => -32600,
@row_add n=76 row=contract/channel-errors/32602 area=1-test-scope-summary rec=fct-3znwz3ga@1,fct-bpxqtijx@1
boundary: channel errors
item: -32602
promise: A request with a higher params.v than supported is rejected with -32602, carrying data.supported and wrapper.
@row_check n=76 row=contract/channel-errors/32602 sha=dc2fd0c10812 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:238-239
quote: if params.v > PROTOCOL_V {         return Err(ProtocolError::UnsupportedVersion);
at: crates/viola-channel/src/lib.rs:95
quote: Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
@row_add n=77 row=contract/unix-socket-permissions/directory area=1-test-scope-summary rec=fct-3znwz3ga@1
boundary: Unix socket permissions
item: directory
promise: The Unix socket directory is mode 0700; a directory at 0755 or a symlink to a 0700 directory is refused and left as found.
@row_check n=77 row=contract/unix-socket-permissions/directory sha=86a347b58186 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:10
quote: pub const DIR_MODE: u32 = 0o700;
at: crates/viola-state/src/strict.rs:168-169
quote: if is_symlink {         return Err(Refused::Symlink);
at: crates/viola-state/src/strict.rs:174
quote: unix_read_verdict(mode, uid, euid)
@row_add n=78 row=contract/unix-socket-permissions/file area=1-test-scope-summary rec=fct-3znwz3ga@1
boundary: Unix socket permissions
item: file
promise: The Unix socket file is mode 0600.
@row_check n=78 row=contract/unix-socket-permissions/file sha=63ff804ddeb8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
@row_add n=79 row=decision/peer-euid-checked area=1-test-scope-summary rec=fct-3znwz3ga@1
name: Peer of another euid dropped
rule: A peer of another euid is dropped by the wrapper before any frame, and a client reads server-verify-failed.
@row_check n=79 row=decision/peer-euid-checked sha=3da8d0d3dbf6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
@row_add n=80 row=decision/paste-control-characters-rejected area=1-test-scope-summary rec=fct-tbtv5wyr@1
name: Control characters in pasted text rejected
rule: ESC, other C0, DEL and C1 characters in send.text and free-text answers are rejected, never stripped, as not-delivered/control-character before other refusals, at both the client and the wrapper.
except: LF, CR, TAB and multi-byte UTF-8 are accepted.
@row_check n=80 row=decision/paste-control-characters-rejected sha=a1ef31cd89ae v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:367
quote: '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => true,
at: crates/viola-core/src/lib.rs:366
quote: '\n' | '\r' | '\t' => false,
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
@row_add n=81 row=decision/hooks-fail-open area=1-test-scope-summary rec=fct-qtwskmiu@1
name: Hooks fail open
rule: A hook facing oversize stdin, malformed JSON, an argument error, a channel failure, a panic or a missing VIOLA_NAME exits 0 with empty stderr and no body.
@row_check n=81 row=decision/hooks-fail-open sha=24323c2e39a4 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:51
quote: Err(_) if role == Role::Hook => return Outcome::Unparsed,
at: src/main.rs:62
quote: .unwrap_or_else(|_| {
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/cmd/hook/statusline.rs:118
quote: reject_stdin("oversize");
at: src/cmd/hook/statusline.rs:121
quote: let Ok(reading) = statusline::reading(&bytes) else {
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:235
quote: Client::connect_by(endpoint, "hook", connect_by).map_err(|_| NotSent::Unreachable)?;
@row_add n=82 row=decision/statusline-command-needs-strict-home area=1-test-scope-summary rec=fct-qtwskmiu@1
name: Statusline command gated on strict home
rule: The configured statusline_command never runs when the home fails strict-modes.
@row_check n=82 row=decision/statusline-command-needs-strict-home sha=f96d3bd283d7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
@row_add n=83 row=decision/mcp-schema-violations-rejected area=1-test-scope-summary rec=fct-bia7axqu@1
name: MCP schema-violating tool calls rejected
rule: MCP tool calls that violate the schema (a non-ViolaName target, a non-u64 dialog_id, an open-string behavior, control characters in text) are rejected by the handler.
@row_check n=83 row=decision/mcp-schema-violations-rejected sha=a2ea74a39d0b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:265
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
at: crates/viola-mcp/src/tools.rs:283
quote: .and_then(Value::as_u64)
at: crates/viola-mcp/src/tools.rs:290
quote: .and_then(|value| Response::parse(value).ok())
at: crates/viola-mcp/src/tools.rs:297
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
@row_add n=84 row=contract/mcp-iserror/payload area=1-test-scope-summary rec=fct-bia7axqu@1
boundary: MCP isError payloads
item: payload
promise: An isError payload carries codes only.
@row_check n=84 row=contract/mcp-iserror/payload sha=9bc1f124bd83 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:40-41
quote: let mut result = CallToolResult::structured_error(body);     result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=85 row=decision/cli-env-vector-negative-tests area=1-test-scope-summary rec=fct-engo5ir7@2
name: CLI and env vector negative tests
rule: Path-traversal names, a --home or VIOLA_DIR failing strict-modes, oversize answer JSON and an out-of-range Percent in config.json are each covered by negative tests that show them refused.
@row_check n=85 row=decision/cli-env-vector-negative-tests sha=238022446f08 v=holds judge=sonnet head=8834195b0683
at: tests/run_cli.rs:179
quote: for bad in ["Builder", "../x", "1abc"] {
at: tests/cli_controls_not_disableable.rs:568
quote: "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"strict-modes-failed\"}\n"
at: src/cmd/answer.rs:91
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
at: tests/cli_answer.rs:848
quote: fn answer_unusable_response_is_exit_2() {
at: tests/cli_controls_not_disableable.rs:364
quote: #[case::over_one_hundred(r#"{"v":1,"five_hour":101}"#, (91, 10), "five-hour")]
@row_add n=86 row=decision/no-switch-disables-a-control area=1-test-scope-summary rec=fct-engo5ir7@2
name: No switch disables a security control
rule: No environment variable, flag or config key disables a security control.
@row_check n=86 row=decision/no-switch-disables-a-control sha=be99518814ec v=holds judge=sonnet head=8834195b0683
searched: (?i)(disable|insecure|skip_strict|no_strict|unsafe_)[a-z_]*["\)]|VIOLA_[A-Z_]*(DISABLE|SKIP|INSECURE|NO_) · in src/ non-test files
searched: var_os\(|env::var\(|"VIOLA_[A-Z_]+" · in src/ non-test files (only VIOLA_NAME, VIOLA_DIR, VIOLA_BIN, NO_COLOR, PATH and the fake-agent seams found)
searched: VEIL_DISABLE_REDACTION · in deny.toml (only as a banned feature, deny.toml:40)
@row_add n=87 row=decision/private-filesystem-permissions area=1-test-scope-summary rec=fct-55p24js3@1
name: Private filesystem permissions enforced
rule: A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
@row_check n=87 row=decision/private-filesystem-permissions sha=7683ade3ae8d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:219
quote: if mode & 0o022 != 0 {
at: crates/viola-state/src/strict.rs:229
quote: if mode & 0o077 != 0 {
at: crates/viola-state/src/strict.rs:122
quote: if meta.file_type().is_symlink() {
@row_add n=88 row=decision/widened-install-dirs-refused area=1-test-scope-summary rec=fct-55p24js3@1
name: Widened installed directories refused
rule: A widened ledger, bin or plugin directory or pinned copy makes run exit 1 before any log, and a widened ledger, bin or plugin directory makes verify exit 1 before any step.
@row_check n=88 row=decision/widened-install-dirs-refused sha=71532a2e78ea v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:94-98
quote: let mut code = vec![ (ledger_dir(home), Rule::Write), (home.join("bin"), Rule::Write), (home.join("plugin"), Rule::Write), ];
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/cmd/verify.rs:67-68
quote: if check_home_tree(&home, None, None).is_err() { refuse_home();
@row_add n=89 row=decision/pinned-exe-hash-checked area=1-test-scope-summary rec=fct-55p24js3@1
name: Pinned executable hash checked
rule: A pinned executable whose hash does not match makes the command exit 1.
@row_check n=89 row=decision/pinned-exe-hash-checked sha=d5cc4e7de433 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
at: src/cmd/verify.rs:81
quote: Ended::Refused(detail) => (1, Some(detail)),
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=90 row=decision/plugin-rewritten-on-start area=1-test-scope-summary rec=fct-55p24js3@1
name: Plugin and settings rewritten on start
rule: The plugin directory and the instance's settings.json are rewritten on every start.
@row_check n=90 row=decision/plugin-rewritten-on-start sha=a2f01a9cf4d9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:478
quote: replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
at: src/cmd/run.rs:568
quote: replace_private(&path, document.as_bytes(), FILE_MODE)?;
@row_add n=91 row=decision/agents-json-fallback-unknown area=1-test-scope-summary rec=fct-vzvcqlaz@1
name: Unreadable agents listing reads unknown
rule: Oversize or malformed claude agents --json output is read as unknown.
@row_check n=91 row=decision/agents-json-fallback-unknown sha=837ad863e3ef v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/agents.rs:91
quote: return Err(AgentsFailure::Oversize);
at: crates/viola-agent-claude/src/agents.rs:94
quote: return Err(AgentsFailure::Malformed);
at: src/cmd/list.rs:126
quote: board::join(&mut board, observations(list));
at: crates/viola-state/src/board.rs:236
quote: status: SessionStatus::Unknown,
@row_add n=92 row=decision/row-name-never-a-target area=1-test-scope-summary rec=fct-vzvcqlaz@1
name: Agent row name never a target
rule: An agent row name is never usable as a target.
@row_check n=92 row=decision/row-name-never-a-target sha=fcae35a82900 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:193
quote: name: observation.session_id,
at: crates/viola-agent-claude/src/agents.rs:127
quote: name: text("name"),
@row_add n=93 row=decision/human-output-escapes-control area=1-test-scope-summary rec=fct-vzvcqlaz@1
name: Human-mode output escapes control characters
rule: Human-mode list, wait and last output escapes C0 and C1 characters.
@row_check n=93 row=decision/human-output-escapes-control sha=b719a6f93d8b v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
at: src/human/wait.rs:21
quote: out.write_all(escape_message(&line).as_bytes())
at: src/human/wait.rs:49
quote: let mut shown = escape_message(message);
@row_add n=94 row=decision/no-advisory-ignored area=1-test-scope-summary rec=fct-hlx2zkbt@2
name: No dependency advisory ignored
rule: CI gates on cargo deny check with no advisory ignored.
@row_check n=94 row=decision/no-advisory-ignored sha=fbd98864f41c v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: deny.toml:11
quote: ignore = []
@row_add n=95 row=decision/parser-property-fuzz-coverage area=1-test-scope-summary rec=fct-urt6oicn@1
name: Parser surfaces under property and fuzz tests
rule: Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the vt100 feed and prompt-submitted normalisation.
@row_check n=95 row=decision/parser-property-fuzz-coverage sha=be0c96c40e7f v=holds judge=sonnet head=8834195b0683
at: fuzz/fuzz_targets/paste_text.rs:22
quote: fuzz_target!(|bytes: &[u8]| {
at: fuzz/fuzz_targets/channel_frame.rs:13
quote: fuzz_target!(|bytes: &[u8]| {
at: fuzz/fuzz_targets/hook_stdin.rs:11
quote: fuzz_target!(|bytes: &[u8]| {
at: fuzz/fuzz_targets/claude_agents_json.rs:9
quote: fuzz_target!(|bytes: &[u8]| {
at: fuzz/fuzz_targets/vt100_feed.rs:21
quote: fuzz_target!(|bytes: &[u8]| {
at: crates/viola-agent-claude/src/hook.rs:985
quote: fn prompt_text_prop_round_trips_a_wrapped_paste(
@row_add n=96 row=decision/chaos-tests-required area=1-test-scope-summary rec=fct-pw5tvowr@1
name: Chaos tests required
rule: Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
@row_check n=96 row=decision/chaos-tests-required sha=f3f8a3b3ad63 v=holds judge=sonnet head=8834195b0683
at: tests/chaos_torn_append.rs:50
quote: fn chaos_torn_tail_left_by_a_stopped_wrapper_is_healed_by_the_next_start(
at: tests/chaos_snapshot_replay.rs:88
quote: fn a_start_over_an_unreadable_snapshot_replays_the_log(
at: tests/tui_pty_seam.rs:30
quote: fn pty_exit_is_read_on_the_handle_while_the_output_is_held(#[from(home)] tmp: TestHome) {
at: tests/chaos_wait_vanish.rs:22
quote: fn chaos_wait_parked_wrapper_killed_exits_21() {
at: crates/viola-state/src/liveness.rs:246
quote: fn live_endpoint_of_a_live_instance_is_its_endpoint_and_of_a_stale_one_none() {
@row_add n=97 row=decision/hook-deadline-timing area=1-test-scope-summary rec=fct-2xjimxfu@1
name: Hook deadlines and heartbeat staleness asserted
rule: Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion.
@row_check n=97 row=decision/hook-deadline-timing sha=f64e56beb55a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:151
quote: Some(max) if max < bound => {}
at: crates/viola-e2e/src/harness/run/perf.rs:25
quote: "session-end",
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
at: tests/chaos_snapshot_replay.rs:97
quote: assert!(age_beat(&dir, STALE_AFTER * 2) > STALE_AFTER);
@row_add n=98 row=decision/cli-mcp-identical-payloads area=1-test-scope-summary rec=fct-wmsiydy7@2
name: CLI and MCP return identical payloads
rule: MCP and CLI return identical ok and refusal payloads for the same channel result.
@row_check n=98 row=decision/cli-mcp-identical-payloads sha=4cc0092616c7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/tests/mcp_tools.rs:860
quote: assert_eq!(woke, cli_woke["ok"]);
@row_add n=99 row=contract/cli-exit-codes/10-14 area=1-test-scope-summary rec=fct-wmsiydy7@2
boundary: CLI exit codes
item: 10-14
promise: A refused send shows the refused marker and exits with a code from 10 to 14.
@row_check n=99 row=contract/cli-exit-codes/10-14 sha=7a8be58c6f34 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67-71
quote: RefusalReason::HumanTyping => 10, RefusalReason::BudgetPaused => 11, RefusalReason::UnverifiedCli => 12, RefusalReason::NotDelivered => 13, RefusalReason::Unknown => 14,
at: src/cmd/send.rs:221
quote: out.unable(&reason, detail.as_deref());
@row_add n=100 row=decision/replay-every-cli-version-fixture area=1-test-scope-summary rec=fct-bpxqtijx@1
name: Every CLI-version fixture set replayed
rule: Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
@row_check n=100 row=decision/replay-every-cli-version-fixture sha=2ebd3eb321fb v=holds judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:30
quote: let mut found: Vec<String> = fs::read_dir(root)
at: tests/contract_ledger_probes.rs:67-68
quote: let lists = usize::from(STAMPED.contains(&version.as_str())) + usize::from(DRIFT_ONLY.contains(&version.as_str()));
at: tests/cli_version_gate.rs:231-232
quote: #[case::stamped(true)] #[case::unstamped(false)]
@row_add n=101 row=decision/unknown-events-counted-skipped area=1-test-scope-summary rec=fct-bpxqtijx@1
name: Unknown events counted, not failed
rule: Unknown event kinds and fields are counted in skipped rather than failing.
@row_check n=101 row=decision/unknown-events-counted-skipped sha=09ffd08d88fa v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
at: crates/viola-state/src/events.rs:266
quote: self.skipped.unknown_fields += 1;
@row_add n=102 row=decision/ledger-row-verify-probe area=1-test-scope-summary rec=fct-bpxqtijx@1
name: Every ledger row has a verify probe
rule: Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
@row_check n=102 row=decision/ledger-row-verify-probe sha=3c25aefe9fa4 v=holds judge=sonnet head=8834195b0683
at: tests/contract_ledger_probes.rs:110
quote: assert_eq!(*last, format!("stamped {version}  24 pass  0 fail"));
@row_add n=103 row=decision/fake-agent-matches-verify-fixtures area=1-test-scope-summary rec=fct-fvjvvfmg@1,fct-xagti75h@1
name: Fake agent held to recorded verify fixtures
rule: A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally.
why: guards against the fake agent drifting from the real CLI
@row_check n=103 row=decision/fake-agent-matches-verify-fixtures sha=019403063aeb v=holds judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:249
quote: /// Every set recorded with its dialog tier replays it byte for byte: `viola verify` against the
at: tests/contract_fake_agent_drift.rs:342
quote: /// Every set recorded with its framing variants replays them byte for byte: `viola verify` against
@row_add n=104 row=decision/error-sanitization area=1-test-scope-summary rec=fct-kpze4wbb@1
name: Errors sanitized at every surface
rule: CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
@row_check n=104 row=decision/error-sanitization sha=7367d20ebf15 v=holds judge=sonnet head=8834195b0683
at: tests/contract_error_bodies.rs:36
quote: const CLI_DOCUMENTS: &[Body] = &[
at: tests/contract_error_bodies.rs:242
quote: const MCP_RESULTS: &[Body] = &[
at: tests/contract_error_bodies.rs:312
quote: const CHANNEL_FAULTS: &[(ProtocolError, &str, &str)] = &[
@row_add n=105 row=contract/channel-errors/32603 area=1-test-scope-summary rec=fct-kpze4wbb@1
boundary: channel errors
item: -32603
promise: The message is exactly "internal error" and data is null.
@row_check n=105 row=contract/channel-errors/32603 sha=4320d235e201 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
at: crates/viola-channel/src/lib.rs:97
quote: _ => Value::Null,
@row_add n=106 row=decision/no-screen-scraping-in-e2e area=1-test-scope-summary rec=fct-shszmhpb@1
name: E2E asserts on events, never on screen content
rule: E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
@row_check n=106 row=decision/no-screen-scraping-in-e2e sha=779930f0a381 v=holds judge=sonnet head=8834195b0683
searched: screen_text|contents\(\)|\.screen\(\)|rendered screen · in tests/** and crates/viola-e2e/**/*.rs
@row_add n=107 row=decision/no-blanket-approval area=1-test-scope-summary rec=fct-shszmhpb@1
name: No blanket dialog approval
rule: No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test.
@row_check n=107 row=decision/no-blanket-approval sha=9b7e427857b2 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: tests/cli_answer.rs:743
quote: fn path4_unstamped_dialogs_are_left_to_the_human_and_answers_refused() {
@row_add n=108 row=decision/waits-key-on-events area=1-test-scope-summary rec=fct-shszmhpb@1,fct-s5fluv4p@1
name: Test waits key on logged events
rule: Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
@row_check n=108 row=decision/waits-key-on-events sha=70ac20eeb569 v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:2
quote: retries = 0
searched: thread::sleep|tokio::time::sleep · in tests/
@row_add n=109 row=decision/test-tier-single-source area=1-test-scope-summary rec=fct-imyfbj3w@1
name: Test tier is the single source of rigor
rule: The test tier is the single source of truth for how much test rigor downstream skills apply.
@row_check n=109 row=decision/test-tier-single-source sha=4d040fd703a9 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=110 row=decision/e2e-for-critical-paths area=2-test-strategy rec=fct-q3ukwekc@1
name: E2E for every critical path
rule: All seven critical paths and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
@row_check n=110 row=decision/e2e-for-critical-paths sha=5deb5814fe76 v=holds judge=sonnet head=8834195b0683
at: tests/path1_run_start.rs:46
quote: fn path1_run_start_one_home_in_order(stamped_home: StampedHome) {
at: crates/viola-e2e/tests/path6_budget_harness.rs:128
quote: fn path6_two_instance_release_over_a_harness_session() {
at: crates/viola-e2e/tests/mcp_tools.rs:1119
quote: fn mcp_tools_path7_unverified_cli_answer() {
@row_add n=111 row=decision/property-and-fuzz-layer area=2-test-strategy rec=fct-oa2vhv4g@1
name: Property tests and fuzzing
rule: proptest runs over the parser surfaces with its regressions committed, and cargo-fuzz targets run in a time-boxed nightly job with a corpus replay gate.
@row_check n=111 row=decision/property-and-fuzz-layer sha=b45c1720a626 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:428
quote: #![proptest_config(config())]
at: crates/viola-core/proptest-regressions/lib.txt:1
quote: # Seeds for failure cases proptest has generated in the past. It is
at: .github/workflows/nightly.yml:48
quote: - name: Time-boxed fuzz (120 s per target)
at: .github/workflows/ci.yml:249
quote: run: bash scripts/agent-run.sh run --fuzz-replay
@row_add n=112 row=decision/heartbeat-staleness-pure-function area=2-test-strategy rec=fct-egpbbwom@1
name: Heartbeat staleness checked without a clock
rule: The heartbeat staleness flip is checked by a pure function fed the beat's age, with no clock.
@row_check n=112 row=decision/heartbeat-staleness-pure-function sha=46af3f16d8cf v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:27
quote: pub fn classify(beat_age: Option<Duration>, same_process: bool) -> Liveness {
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
@row_add n=113 row=decision/fault-injection-sources area=2-test-strategy rec=fct-2yi4u2mc@1
name: Fault injection sources
rule: Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.
@row_check n=113 row=decision/fault-injection-sources sha=654e6d458252 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:107
quote: #[cfg_attr(test, mockall::automock)]
@row_add n=114 row=decision/mutation-run-failure-conditions area=2-test-strategy rec=fct-cv73iuuc@1
name: Mutation run failure conditions
rule: A mutation run fails on a missed mutant not left out as host-excluded, on a timed-out mutant, or when unviable mutants outnumber caught ones.
@row_check n=114 row=decision/mutation-run-failure-conditions sha=ecbda1de31f1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
at: crates/viola-e2e/src/harness/run/mutants.rs:63
quote: failures.push("unviable-exceeds-caught".to_owned());
at: crates/viola-e2e/src/harness/run/mutants.rs:68
quote: failed: survived + u64::from(swamped),
@row_add n=115 row=decision/no-human-in-loop-verification area=2-test-strategy rec=fct-gcer5i4i@2
name: No human-in-loop verification
rule: Verification has no human-in-loop step, no visual review service and no manual exploratory step.
@row_check n=115 row=decision/no-human-in-loop-verification sha=e3c4d526108b v=holds judge=sonnet head=8834195b0683
searched: Percy|Chromatic|exploratory · in crates, tests, src, scripts, .github, fuzz/fuzz_targets
@row_add n=116 row=decision/insta-check-mode-only area=2-test-strategy rec=fct-gcer5i4i@2
name: Snapshot tests in check mode only
rule: insta runs only in check mode, never updating snapshots.
@row_check n=116 row=decision/insta-check-mode-only sha=5f8652d7f8f8 v=holds judge=sonnet head=8834195b0683
searched: INSTA_UPDATE · in crates, tests, src, scripts, .github, .config
searched: insta:: · in crates, tests, src, scripts, .github, .config
@row_add n=117 row=decision/ci-never-real-claude area=2-test-strategy rec=fct-s5fluv4p@1
name: CI never touches the real agent CLI
rule: CI never touches the real claude.
@row_check n=117 row=decision/ci-never-real-claude sha=fecb6ae35de1 v=holds judge=sonnet head=8834195b0683
searched: claude · in .github/workflows/ci.yml
searched: claude · in .github/workflows/nightly.yml
@row_add n=118 row=decision/tests-create-own-home area=2-test-strategy rec=fct-qyzxqqix@1
name: Every test creates its own home
rule: Every test creates its own temporary --home, with no developer-seeded data.
@row_check n=118 row=decision/tests-create-own-home sha=1bf078995141 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:200
quote: .tempdir_in(base)
@row_add n=119 row=decision/stamps-from-fake-agent-verify area=2-test-strategy rec=fct-qyzxqqix@1
name: Stamps only from verify against the fake agent
rule: Test stamps come only from viola verify run against the fake agent.
@row_check n=119 row=decision/stamps-from-fake-agent-verify sha=bc997735b25d v=holds judge=sonnet head=8834195b0683
at: tests/cli_version_gate.rs:37
quote: let ran = verify(home.path(), &fixtures, version, &[], after, &[]);
@row_add n=120 row=decision/budget-json-from-statusline area=2-test-strategy rec=fct-qyzxqqix@1
name: Budget file only from statusline stdin
rule: In every booted-wrapper test, budget.json comes only from viola hook statusline stdin.
except: a reader's own unit cases may plant a malformed one only by altering the writer's own bytes
@row_check n=120 row=decision/budget-json-from-statusline sha=3e3f15a9bda7 v=holds judge=sonnet head=8834195b0683
at: tests/support/statusline.rs:55
quote: .args(["hook", "statusline"])
@row_add n=121 row=decision/tokio-only-in-e2e-crate area=2-test-strategy rec=fct-h4mygvf3@1
name: Tokio test code confined to the E2E crate
rule: One test-only, unpublished crate, viola-e2e, hosts the harness and the tokio-based test clients.
why: so the root package and the sync crates carry no tokio dev-dependency
@row_check n=121 row=decision/tokio-only-in-e2e-crate sha=52762606244a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:45
quote: tokio = { workspace = true, features = ["rt", "process"] }
at: crates/viola-e2e/Cargo.toml:7
quote: publish.workspace = true
at: Cargo.toml:190
quote: publish = false
@row_add n=122 row=decision/shared-e2e-helpers area=2-test-strategy rec=fct-h4mygvf3@1
name: Shared E2E helpers never copied
rule: The sync E2E suites in the root share one set of helpers, which no root test file copies.
@row_check n=122 row=decision/shared-e2e-helpers sha=7f09d3e58bc5 v=holds judge=sonnet head=8834195b0683
at: tests/support/events.rs:74
quote: pub fn events(instance_dir: &Path) -> Vec<Value> {
@row_add n=123 row=decision/agent-driven-harness area=3-test-harness-contract rec=fct-kalxmzsn@1
name: Agent-driven harness development
rule: Development is agent-driven through a test harness whose contract the test plan specifies.
@row_check n=123 row=decision/agent-driven-harness sha=9bd45f3069f9 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=124 row=decision/harness-logic-in-rust area=3-test-harness-contract rec=fct-yqn2bvnu@2
name: Harness logic once, in Rust
rule: One thin POSIX shim runs the Rust harness binary and forwards its exit code and stdout unchanged; all harness logic lives once, in Rust.
@row_check n=124 row=decision/harness-logic-in-rust sha=34cb01b32539 v=holds judge=sonnet head=8834195b0683
at: scripts/agent-run.sh:43
quote: exec cargo run -q -p viola-e2e --bin viola-harness -- "$@"
@row_add n=125 row=contract/harness-session-record/v area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: v
promise: The session record carries its format version v.
@row_check n=125 row=contract/harness-session-record/v sha=e94df37ea018 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:297
quote: pub v: u32,
@row_add n=126 row=contract/harness-session-record/session area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: session
promise: The session record carries the session id; the default session is default.
@row_check n=126 row=contract/harness-session-record/session sha=7e7cc78505d8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:298
quote: pub session: String,
at: crates/viola-e2e/src/bin/viola-harness.rs:31
quote: #[arg(long, default_value = "default")]
@row_add n=127 row=contract/harness-session-record/home area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: home
promise: The session record carries the session's home.
@row_check n=127 row=contract/harness-session-record/home sha=57a6869810af v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:299
quote: pub home: PathBuf,
@row_add n=128 row=contract/harness-session-record/instances area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: instances
promise: The session record lists instances, each with name, wrapper_pid, started_at and fake_args.
@row_check n=128 row=contract/harness-session-record/instances sha=43383a9ec785 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:300
quote: pub instances: Vec<InstanceRecord>,
at: crates/viola-e2e/src/harness/mod.rs:270-275
quote: pub name: String, pub wrapper_pid: u32, pub started_at: u64, pub child_pid: u32, pub child_started_at: u64, pub fake_args: Vec<String>,
@row_add n=129 row=contract/harness-session-record/supervisor-pid area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: supervisor_pid
promise: The session record carries the supervisor's pid.
@row_check n=129 row=contract/harness-session-record/supervisor-pid sha=865f349ff3e8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:301
quote: pub supervisor_pid: u32,
@row_add n=130 row=contract/harness-session-record/supervisor-started-at area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: supervisor_started_at
promise: The session record carries the supervisor's start time.
@row_check n=130 row=contract/harness-session-record/supervisor-started-at sha=f92189b074c5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:302
quote: pub supervisor_started_at: u64,
@row_add n=131 row=contract/harness-session-record/agents-mode area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness session record
item: agents_mode
promise: agents_mode is one of recorded, oversize or malformed, defaults to recorded, is written by boot and stands in supervise.json too.
@row_check n=131 row=contract/harness-session-record/agents-mode sha=7b1b7a1c2aa9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:51-55
quote: pub enum AgentsMode { #[default] Recorded, Oversize, Malformed,
at: crates/viola-e2e/src/harness/mod.rs:303-304
quote: #[serde(default)] pub agents_mode: boot::AgentsMode,
at: crates/viola-e2e/src/harness/boot.rs:216
quote: agents_mode: opts.agents_mode,
at: crates/viola-e2e/src/harness/boot.rs:193
quote: agents_mode: opts.agents_mode,
@row_add n=132 row=decision/harness-records-tolerate-unknown-fields area=3-test-harness-contract rec=fct-bilmaai6@2
name: Harness records tolerate missing and unknown fields
rule: The harness crate never denies unknown fields, so an older session record lacking a newer field still parses.
@row_check n=132 row=decision/harness-records-tolerate-unknown-fields sha=ff2f3efe79e3 v=holds judge=sonnet head=8834195b0683
searched: deny_unknown_fields · in crates/viola-e2e
@row_add n=133 row=contract/harness-cli/session area=3-test-harness-contract rec=fct-bilmaai6@2
boundary: harness CLI
item: --session
promise: --session with an id selects a session, allowing parallel sessions.
@row_check n=133 row=contract/harness-cli/session sha=12130227eda5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:31
quote: #[arg(long, default_value = "default")]
at: crates/viola-e2e/src/harness/mod.rs:77-78
quote: pub fn session_dir(&self, session: &str) -> PathBuf { self.agent_run().join(session)
@row_add n=134 row=contract/harness-exit-codes/0 area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness exit codes
item: 0
promise: The harness command succeeded.
@row_check n=134 row=contract/harness-exit-codes/0 sha=7b4302f1c1d2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=135 row=contract/harness-exit-codes/1 area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness exit codes
item: 1
promise: The harness command failed.
@row_check n=135 row=contract/harness-exit-codes/1 sha=7d4312835646 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=136 row=contract/harness-exit-codes/2 area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness exit codes
item: 2
promise: The harness command was given a usage error, and its document carries reason usage and a detail.
@row_check n=136 row=contract/harness-exit-codes/2 sha=d6906aa606b6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:50-51
quote: doc: json!({"v": 1, "cmd": cmd, "ok": false, "reason": "usage", "detail": detail}), code: 2,
@row_add n=137 row=contract/harness-usage-detail/invalid-session-id area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: invalid-session-id
promise: The session id given is not valid.
@row_check n=137 row=contract/harness-usage-detail/invalid-session-id sha=d050fe870fc2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:136
quote: return Outcome::usage(Some("boot"), "invalid-session-id");
@row_add n=138 row=contract/harness-usage-detail/session-exists area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: session-exists
promise: The session named already exists.
@row_check n=138 row=contract/harness-usage-detail/session-exists sha=9571eaa426b4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:139
quote: return Outcome::usage(Some("boot"), "session-exists");
@row_add n=139 row=contract/harness-usage-detail/invalid-instance area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: invalid-instance
promise: The instance named is not valid.
@row_check n=139 row=contract/harness-usage-detail/invalid-instance sha=553a4c05aa27 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:173
quote: None => emit(Outcome::usage(Some("boot"), "invalid-instance")),
@row_add n=140 row=contract/harness-usage-detail/arguments area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: arguments
promise: The arguments do not parse, including a flag or selector whose surface is not built yet.
@row_check n=140 row=contract/harness-usage-detail/arguments sha=966af401e1d7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:241
quote: return emit(Outcome::usage(cmd, "arguments"));
@row_add n=141 row=contract/harness-usage-detail/no-supervise-spec area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: no-supervise-spec
promise: No supervise spec exists for the request.
@row_check n=141 row=contract/harness-usage-detail/no-supervise-spec sha=3aad8478851e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/supervise.rs:87
quote: return Outcome::usage(Some("supervise"), "no-supervise-spec");
@row_add n=142 row=contract/harness-usage-detail/unknown-suite area=3-test-harness-contract rec=fct-sqrvsbbz@1
boundary: harness usage error detail
item: unknown-suite
promise: A gate --require value lies outside the eight gate suites: nextest-unit, nextest-integration, doctest, nextest-e2e, mutants, coverage, perf, fuzz-replay.
@row_check n=142 row=contract/harness-usage-detail/unknown-suite sha=e4223ef4c7f9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:225
quote: return emit(Outcome::usage(Some("gate"), "unknown-suite"));
at: crates/viola-e2e/src/harness/gate.rs:14
quote: pub const SUITES: [&str; 8] = [
@row_add n=143 row=decision/unbuilt-surface-is-usage-error area=3-test-harness-contract rec=fct-sqrvsbbz@1
name: Unbuilt harness surface is a usage error
rule: A harness flag or selector whose surface is not built yet is a usage error, never a vacuous pass.
@row_check n=143 row=decision/unbuilt-surface-is-usage-error sha=97814da0fdfe v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:241
quote: return emit(Outcome::usage(cmd, "arguments"));
at: crates/viola-e2e/src/bin/viola-harness.rs:225
quote: return emit(Outcome::usage(Some("gate"), "unknown-suite"));
@row_add n=144 row=decision/doctests-via-cargo-test area=4-unit-test-strategy rec=fct-uptme7ma@1
name: Doctests run through cargo test
rule: Doctests run through cargo test --workspace --doc.
why: nextest cannot run doctests
@row_check n=144 row=decision/doctests-via-cargo-test sha=046728b998f2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/doctest.rs:12-14
quote: "test", "--workspace", "--doc",
@row_add n=145 row=decision/llvm-cov-coverage area=4-unit-test-strategy rec=fct-uptme7ma@1
name: cargo-llvm-cov measures coverage
rule: Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary.
@row_check n=145 row=decision/llvm-cov-coverage sha=275d5e41f445 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:25-26
quote: "llvm-cov", "nextest",
at: crates/viola-e2e/src/harness/run/coverage.rs:32
quote: "--lcov",
at: crates/viola-e2e/src/harness/run/coverage.rs:53
quote: .args(["llvm-cov", "report", "--json", "--summary-only"])
@row_add n=146 row=decision/rstest-fixtures-for-constructors area=4-unit-test-strategy rec=fct-xesm7yum@1,fct-o2g5knlj@1
name: rstest fixtures for shared constructors
rule: Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.
@row_check n=146 row=decision/rstest-fixtures-for-constructors sha=aa86df261b52 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:353-355
quote: #[fixture] pub fn home() -> TestHome { TestHome::new()
@row_add n=147 row=decision/proptest-parser-invariants area=4-unit-test-strategy rec=fct-klpawgfb@1
name: Proptest for parser invariants
rule: Parser invariants are held by proptest property blocks run at 512 cases.
@row_check n=147 row=decision/proptest-parser-invariants sha=10c5f94a3cfb v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:813
quote: cases: 512,
at: crates/viola-agent-claude/src/agents.rs:328
quote: cases: 512,
@row_add n=148 row=decision/identity-floor-tested-against-literal-copy area=4-unit-test-strategy rec=fct-nr357doe@2
name: Identity floor tested against a literal copy
rule: The R8 strip is tested against a literal copy of the identity floor, never an imported one.
@row_check n=148 row=decision/identity-floor-tested-against-literal-copy sha=05ba6f218939 v=holds judge=sonnet head=8834195b0683
at: tests/cli_controls_not_disableable.rs:725-726
quote: /// The identity floor's names, one per family: the oracle, written out here. const FLOOR: [(&str, &str); 2] = [
at: crates/viola-agent-claude/src/lib.rs:288
quote: assert_eq!(plan.remove, names(&["CLAUDE_CODE_SESSION_ID"]));
@row_add n=149 row=decision/no-assertion-beyond-compiled-literals area=4-unit-test-strategy rec=fct-nr357doe@2
name: No assertion beyond compiled literals
rule: viola-agent-claude unit tests never assert agent content beyond the compiled literals.
@row_check n=149 row=decision/no-assertion-beyond-compiled-literals sha=e1b92de237b6 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=150 row=contract/mcp-tools-list/tools-list area=4-unit-test-strategy rec=fct-xeug7oqh@1
boundary: MCP tools/list
item: tools/list
promise: tools/list is exactly send, wait, last, answer, list.
@row_check n=150 row=contract/mcp-tools-list/tools-list sha=d842e69d4d4d v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=151 row=contract/mcp-tool-arguments/invalid-argument area=4-unit-test-strategy rec=fct-xeug7oqh@1
boundary: MCP tool arguments
item: invalid argument
promise: A non-ViolaName target, a non-u64 dialog_id, an open-string behavior or control characters in text give isError:true with a code-only payload.
@row_check n=151 row=contract/mcp-tool-arguments/invalid-argument sha=81a56b1ec47b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:265
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
at: crates/viola-mcp/src/tools.rs:282-284
quote: args.get("dialog_id") .and_then(Value::as_u64) .ok_or("dialog_id")
at: crates/viola-mcp/src/tools.rs:290
quote: .and_then(|value| Response::parse(value).ok())
at: crates/viola-mcp/src/tools.rs:297
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: crates/viola-mcp/src/result.rs:35-36
quote: pub(crate) fn invalid(field: &'static str) -> CallToolResult { error("invalid-params", Value::from(field))
@row_add n=152 row=contract/mcp-stdin/line-cap area=4-unit-test-strategy rec=fct-xeug7oqh@1
boundary: MCP stdin
item: line cap
promise: A line of exactly MAX_FRAME bytes, its newline counted, passes whole; one byte more passes no byte, reads as end of stream, and serve ends the session.
@row_check n=152 row=contract/mcp-stdin/line-cap sha=8016a0aad2a1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/stdin.rs:60-64
quote: if !self.fits(self.held.len() + segment.len()) { self.held.clear(); self.done = true; self.cut.store(true, Ordering::Relaxed); return;
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
@row_add n=153 row=contract/viola-mcp-exit/oversize-before-initialize area=4-unit-test-strategy rec=fct-xeug7oqh@1
boundary: viola mcp exit
item: oversize line before initialize
promise: A stdin line past MAX_FRAME before initialize completes makes viola mcp exit 1 with both streams empty and one parse-rejected mcp-stdin / oversize line of count 1.
@row_check n=153 row=contract/viola-mcp-exit/oversize-before-initialize sha=96bcc2c47246 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:86-87
quote: if cut.load(Ordering::Relaxed) { reject_oversize_line();
at: crates/viola-mcp/src/lib.rs:98-100
quote: parser = "mcp-stdin", detail = "oversize", count = 1u64,
at: src/main.rs:134-138
quote: (Role::Mcp | Role::Cli | Role::Other, Outcome::Done(code)) => code, ( Role::Mcp | Role::Cli | Role::Other, Outcome::Unparsed | Outcome::Failed | Outcome::Panicked, ) => ExitCode::from(1),
@row_add n=154 row=contract/reply-screen/refusal area=4-unit-test-strategy rec=fct-xeug7oqh@1,fct-fn3ybwy5@2
boundary: the reply screen
item: refusal
promise: A refusal that is no code word reads unknown.
@row_check n=154 row=contract/reply-screen/refusal sha=57b0a7aab202 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:73-77
quote: let reason = if is_code_word(reason) { reason } else { RefusalReason::Unknown.as_str() };
@row_add n=155 row=contract/reply-screen/detail area=4-unit-test-strategy rec=fct-xeug7oqh@1,fct-fn3ybwy5@2
boundary: the reply screen
item: detail
promise: A string detail that is no code word reads null.
@row_check n=155 row=contract/reply-screen/detail sha=2fc36b92f9e0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
@row_add n=156 row=contract/reply-screen/peer-fault area=4-unit-test-strategy rec=fct-xeug7oqh@1,fct-fn3ybwy5@2
boundary: the reply screen
item: peer fault
promise: A peer fault reads through sanitised_fault.
@row_check n=156 row=contract/reply-screen/peer-fault sha=40b946b0511e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:60
quote: return sanitised_fault(fault)
@row_add n=157 row=decision/mocks-only-at-product-seams area=4-unit-test-strategy rec=fct-zaiak6ok@1
name: Mocks only at product seams
rule: Unit tests use mockall doubles only for traits the product already defines at its seams: Pty, the liveness probe and Clock.
@row_check n=157 row=decision/mocks-only-at-product-seams sha=2dd7855d155d v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:107
quote: #[cfg_attr(test, mockall::automock)]
@row_add n=158 row=decision/cli-outputs-pinned-by-literal area=5-integration-test-strategy rec=fct-s42l2rwd@2,fct-e73lmy47@1
name: CLI outputs pinned by literal asserts
rule: CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
@row_check n=158 row=decision/cli-outputs-pinned-by-literal sha=bc4298d89136 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:20
quote: pub const VIOLA: &str = env!("CARGO_BIN_EXE_viola");
at: tests/cli_list.rs:118
quote: assert_eq!(human.stdout, format!("{BAY_UNKNOWN}\n{CAPTION}\n{ROW}\n"));
@row_add n=159 row=decision/human-layouts-pinned-by-literal area=5-integration-test-strategy rec=fct-klrvzf5j@2,fct-e73lmy47@1
name: Human-mode layouts pinned by literal
rule: Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files.
guard: HELP literal in cli_output_plain; cli_list board tests
@row_check n=159 row=decision/human-layouts-pinned-by-literal sha=01f0e3f01488 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cli_list.rs:118
quote: assert_eq!(human.stdout, format!("{BAY_UNKNOWN}\n{CAPTION}\n{ROW}\n"));
at: tests/cli_output_plain.rs:75
quote: assert_eq!(String::from_utf8(out.stdout).expect("utf-8"), HELP);
guard_at: tests/cli_output_plain.rs:55
@row_add n=160 row=decision/real-pty-in-integration area=5-integration-test-strategy rec=fct-5vfqglut@1,fct-e73lmy47@1
name: Real PTY in integration tests
rule: Integration tests use the real portable-pty through viola-pty.
@row_check n=160 row=decision/real-pty-in-integration sha=3fee9c803955 v=holds judge=sonnet head=8834195b0683
at: tests/support/outer_pty.rs:78
quote: let mut pty = viola_pty::spawn(&spec).expect("outer pty spawn");
@row_add n=161 row=decision/real-disk-in-integration area=5-integration-test-strategy rec=fct-jl2uwhfc@2,fct-e73lmy47@1
name: Real disk in integration tests
rule: Integration tests use a real tempfile filesystem.
@row_check n=161 row=decision/real-disk-in-integration sha=da4fd3860c70 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:198
quote: let dir = tempfile::Builder::new()
@row_add n=162 row=decision/module-integration-direct-calls area=5-integration-test-strategy rec=fct-crxvk5ao@1
name: Module integration by direct calls
rule: Module-to-module integration is tested by direct calls across crate APIs with shared rstest fixtures.
@row_check n=162 row=decision/module-integration-direct-calls sha=2cbc9e7e6176 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:353
quote: #[fixture]
at: tests/tui_pty_seam.rs:22
quote: OuterPty::spawn_sized(Path::new(FAKE), &args, &[], size)
@row_add n=163 row=decision/ipc-tested-on-real-endpoint area=5-integration-test-strategy rec=fct-ctentyeb@2
name: IPC tested on a real endpoint
rule: IPC integration binds a real endpoint in a temp home with jsonschema-validated frames, creates the squatter first through the server API, never try_overwrite, and reads the Unix socket modes back.
@row_check n=163 row=decision/ipc-tested-on-real-endpoint sha=352e2d32a371 v=holds judge=sonnet head=8834195b0683
at: tests/cross_exit_causes.rs:391
quote: let squatter = viola_channel::Server::bind(&endpoint_of(home)).expect("the squatter binds");
at: tests/channel_endpoint.rs:247
quote: let validator = jsonschema::validator_for(&schema).expect("schema");
at: tests/channel_endpoint.rs:262-263
quote: assert_eq!(mode(socket.parent().expect("socket dir")), 0o700); assert_eq!(mode(socket), 0o600);
searched: try_overwrite · in tests/ (no match)
@row_add n=164 row=decision/hook-exec-against-live-wrapper area=5-integration-test-strategy rec=fct-3aeso5ca@1
name: Hook exec tested against a live wrapper
rule: viola hook is fed fixture stdin against a live test wrapper, and its event line, hook trace and budget.json are asserted.
@row_check n=164 row=decision/hook-exec-against-live-wrapper sha=b82f46915ef1 v=holds judge=sonnet head=8834195b0683
at: tests/hook_statusline.rs:136
quote: let ran = session.hook(stdin.clone());
at: tests/hook_statusline.rs:139
quote: let budget = session.budget().expect("a reading");
at: tests/hook_fail_open.rs:190
quote: fn serve(home: &Path, label: &str) -> Self {
@row_add n=165 row=contract/wrapper-channel/-32600 area=5-integration-test-strategy rec=fct-r2yuicrg@1
boundary: the wrapper channel
item: -32600
promise: An oversize frame is answered with -32600 and the connection closed.
@row_check n=165 row=contract/wrapper-channel/-32600 sha=f52978e625cd v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:162
quote: Err(ChannelError::Oversize) => Some(refuse_oversize(srv_conn)),
at: crates/viola-channel/src/server.rs:268-269
quote: frame: error_response(None, ProtocolError::InvalidRequest), close: true,
@row_add n=166 row=contract/event-line/round-trip area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: the event line
item: event kind
promise: Every event kind round-trips the event line.
@row_check n=166 row=contract/event-line/round-trip sha=73a0bfc53ba6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/tests/state_events.rs:143
quote: assert_eq!(lines.len(), 16);
at: crates/viola-state/src/events.rs:65-66
quote: pub fn append_event(instance_dir: &Path, line: &EventLine) -> Result<(), StateError> { let bytes = line_bytes(line)?;
@row_add n=167 row=contract/event-tailing/ignored-entries area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: event tailing
item: symlinks and invalid names
promise: Tailing ignores symlinks and invalid names.
@row_check n=167 row=contract/event-tailing/ignored-entries sha=7cb041772f6f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:77-78
quote: if kind.is_symlink() { scan.links_ignored += 1;
at: crates/viola-state/src/board.rs:91
quote: _ => scan.malformed += 1,
@row_add n=168 row=contract/strict-modes-refusal/run-mcp-revive-verify area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: strict-modes refusal
item: run, mcp, viola revive, viola verify
promise: A strict-modes refusal exits 1.
@row_check n=168 row=contract/strict-modes-refusal/run-mcp-revive-verify sha=154da9a56753 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:171-173
quote: Err(refused) => { refuse_tree(&args.name, refused); return Ok(ExitCode::from(1));
at: src/cmd/mcp.rs:22-23
quote: if !home_passes(home) { return Ok(ExitCode::from(1));
at: src/cmd/revive.rs:137-139
quote: if !trusted(home, &args.name)? { Refusal::StrictModes.write(&args.name, args.json); return Ok(Started::Refused(ExitCode::from(1)));
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
@row_add n=169 row=contract/strict-modes-refusal/mcp-frame-tools area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: strict-modes refusal
item: MCP frame tools
promise: A strict-modes refusal in the MCP frame tools gives instance-unreachable.
@row_check n=169 row=contract/strict-modes-refusal/mcp-frame-tools sha=67c9f21429e6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-340
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=170 row=contract/strict-modes-refusal/21 area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: strict-modes refusal
item: 21
promise: Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
@row_check n=170 row=contract/strict-modes-refusal/21 sha=7664389d2aaa v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:37
quote: let (unable, hint) = human::instance_state_pair(name);
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/wait.rs:43
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/last.rs:30
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/answer.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/pause.rs:32
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/release.rs:35
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/list.rs:47
quote: return Ok(ExitCode::from(21));
at: src/cmd/plugin.rs:60
quote: return Ok(ExitCode::from(21));
@row_add n=171 row=contract/strict-modes-refusal/no-line area=5-integration-test-strategy rec=fct-zpi22xot@1
boundary: strict-modes refusal
item: no line
promise: A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line.
@row_check n=171 row=contract/strict-modes-refusal/no-line sha=2bfced8ed001 v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=172 row=contract/viola-hook/exit-and-stderr area=5-integration-test-strategy rec=fct-627s4o4i@1
boundary: viola hook
item: exit and stderr
promise: Every hook event exits 0 with empty stderr.
@row_check n=172 row=contract/viola-hook/exit-and-stderr sha=ba52b13453fa v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=173 row=contract/viola-hook/viola-dir area=5-integration-test-strategy rec=fct-627s4o4i@1
boundary: viola hook
item: VIOLA_DIR
promise: A VIOLA_DIR instance the instance check refuses, or one naming no existing instance directory, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append.
@row_check n=173 row=contract/viola-hook/viola-dir sha=65edba133827 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:78
quote: let dir = std::fs::canonicalize(&instance.dir).ok()?;
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=174 row=contract/cli-output/no-escape-without-tty area=5-integration-test-strategy rec=fct-w5xcngvx@3
boundary: CLI output
item: escape sequences
promise: Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
@row_check n=174 row=contract/cli-output/no-escape-without-tty sha=ece8647cedd9 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62-66
quote: if facts.json         || facts.passthrough         || facts.no_color         || facts.term.as_deref() == Some("dumb")         || !facts.stdout_tty
@row_add n=175 row=contract/cli-argv/leading-slash area=5-integration-test-strategy rec=fct-w5xcngvx@3
boundary: CLI argv
item: leading slash
promise: A leading-slash argument is never delivered as a prompt.
@row_check n=175 row=contract/cli-argv/leading-slash sha=644168fab4b6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:39
quote: #[arg(hide = true, value_parser = refuse_text_argument)]
@row_add n=176 row=decision/board-on-the-read-rule area=5-integration-test-strategy rec=fct-w5xcngvx@3
name: Board held to the read rule
rule: viola list refuses a home another user may read, with and without an instance.
guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
@row_check n=176 row=decision/board-on-the-read-rule sha=34e11633eb00 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cli_list.rs:568
quote: assert_eq!(human.code, Some(21), "{holding}: {}", human.stderr);
guard_at: tests/cli_list.rs:555
@row_add n=177 row=decision/test-started-viola-drops-env area=5-integration-test-strategy rec=fct-4jpum6e2@1
name: Test-started viola drops inherited env
rule: A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.
@row_check n=177 row=decision/test-started-viola-drops-env sha=02e32fe997cd v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:24
quote: pub const UNWRAPPED: [&str; 3] = ["VIOLA_NAME", "VIOLA_DIR", "VIOLA_BIN"];
at: tests/support/home.rs:340
quote: pub fn resolved(home: &Path) -> PathBuf {
@row_add n=178 row=decision/mcp-handlers-use-tokio-client area=5-integration-test-strategy rec=fct-gri7u5le@1
name: MCP handlers use the Tokio client
rule: MCP handlers use viola-channel's Tokio client, never the sync client, and are tested at adapter level through the transport-free Tools handle against a test wrapper.
@row_check n=178 row=decision/mcp-handlers-use-tokio-client sha=0bba3374ea7d v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:348
quote: let mut client = match TokioClient::connect(&live.endpoint, "mcp").await {
at: crates/viola-mcp/tests/handlers.rs:14
quote: use viola_mcp::Tools;
@row_add n=179 row=contract/mcp-frame-tools/socket-dir-refused area=5-integration-test-strategy rec=fct-gri7u5le@1
boundary: MCP frame tools
item: socket-dir-refused
promise: A loosened socket directory gives isError with socket-dir-refused, no frame and no line.
@row_check n=179 row=contract/mcp-frame-tools/socket-dir-refused sha=65b0036c1198 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() {         return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=180 row=decision/live-readings-not-a-test-layer area=6-e2e-test-strategy rec=fct-6ugas3oa@1
name: Live readings are not a test layer
rule: A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
@row_check n=180 row=decision/live-readings-not-a-test-layer sha=1dd448e5d1e0 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=181 row=contract/run-start-records/first-three area=6-e2e-test-strategy rec=fct-mwv2sqc7@2
boundary: the event log at viola run start
item: first three records
promise: The first three records are wheel{cause:"start"}, budget-gate and the hook's session-start, and the child is spawned after the first two of them; the session-start is the hook of the spawned child.
@row_check n=181 row=contract/run-start-records/first-three sha=c47a9aae36bc v=stale judge=sonnet head=8834195b0683
at: src/cmd/run.rs:390
quote: let spawned = spawn_child(
shows: The wrapper appends only wheel{cause:"start"} and budget-gate (start_state, run.rs:617-626) before it spawns the child; the third record, session-start, comes from the child's hook after the spawn, so the child is spawned after two records, not three.
cell: promise
value: The first three records are wheel{cause:"start"}, budget-gate and the hook's session-start, and the child is spawned after the first two of them; the session-start is the hook of the spawned child.
@row_add n=182 row=contract/snapshot/run-fields area=6-e2e-test-strategy rec=fct-mwv2sqc7@2
boundary: the instance snapshot
item: pinned path, pids and endpoint
promise: The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
@row_check n=182 row=contract/snapshot/run-fields sha=f48338464256 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:598-601
quote: endpoint: Some(endpoint),         pid,         started_at: own_start(pid).context("own start time unreadable")?,         pinned_bin: pinned.path_fwd.clone(),
at: src/cmd/run.rs:652
quote: snapshots.update(|s| s.child_pid = child_pid)?;
at: crates/viola-state/src/board.rs:276
quote: Liveness::Live => Some(RowLiveness::Live),
@row_add n=183 row=contract/viola-run-exit-codes/1 area=6-e2e-test-strategy rec=fct-mwv2sqc7@2
boundary: viola run exit codes
item: 1
promise: A duplicate run, a squatted endpoint and a tampered pinned exe each exit 1.
@row_check n=183 row=contract/viola-run-exit-codes/1 sha=5eeebd56381f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/run.rs:467
quote: Err(PinError::HashMismatch) => {
at: src/cmd/run.rs:413
quote: Started::Refused(ExitCode::from(1))
@row_add n=184 row=contract/generated-plugin/hook-commands area=6-e2e-test-strategy rec=fct-mwv2sqc7@2
boundary: the generated plugin
item: hook commands
promise: A restart rewrites plugin/ with absolute hook commands, never a PATH lookup, and the settings.json override.
@row_check n=184 row=contract/generated-plugin/hook-commands sha=5eb3e173883d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:475
quote: for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) {
at: src/cmd/run.rs:478
quote: replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
at: crates/viola-agent-claude/src/lib.rs:151
quote: .replace(BIN_TOKEN, pinned_bin_fwd)
at: src/cmd/run.rs:564
quote: let Some(document) = statusline::override_document(&pinned.path_fwd) else {
@row_add n=185 row=contract/viola-send/confirmed area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: viola send
item: confirmed send
promise: A confirmed send exits 0 with {submitted_at, cursor}, logging send-issued then prompt-submitted{origin:"driver"} with one bracketed paste in the receipt.
@row_check n=185 row=contract/viola-send/confirmed sha=f8c735eca246 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
at: src/run/send.rs:186
quote: EventKind::SendIssued,
at: src/run/send/slot.rs:203
quote: line.data["origin"] = json!("driver");
at: crates/viola-pty/src/pump.rs:79-81
quote: bytes.extend_from_slice(b"\x1b[200~"); bytes.extend_from_slice(text.as_bytes()); bytes.extend_from_slice(b"\x1b[201~\r");
@row_add n=186 row=contract/viola-send/cr-lf-text area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: viola send
item: text ending in or holding CR or LF
promise: A text ending in or holding CR or LF is confirmed as its typed text.
@row_check n=186 row=contract/viola-send/cr-lf-text sha=7f43479b40b1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232
quote: let text = text.trim_end_matches(['\r', '\n']);
at: crates/viola-agent-claude/src/hook.rs:234
quote: Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
@row_add n=187 row=contract/viola-send/not-delivered area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: viola send
item: not-delivered
promise: A suppressed prompt is refused not-delivered with detail no-prompt-submitted.
@row_check n=187 row=contract/viola-send/not-delivered sha=00c3ad7a2982 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=188 row=contract/viola-send/unconfirmable area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: viola send
item: unconfirmable
promise: A listed local command is confirmed only by its post-condition, or else is unconfirmable.
@row_check n=188 row=contract/viola-send/unconfirmable sha=e8593c77315f v=holds judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:30-31
quote: Some((_, Some(post))) if cli_verified => Confirms::Post(*post), Some(_) => Confirms::Nothing,
at: src/run/send.rs:152
quote: return ctx.unconfirmable(cursor, slot.clock.now());
@row_add n=189 row=contract/mcp-send/ok-keys area=6-e2e-test-strategy rec=fct-waypbx6k@1
boundary: the MCP send tool
item: ok result keys
promise: The MCP send result carries the same ok key set as the CLI.
@row_check n=189 row=contract/mcp-send/ok-keys sha=e349ea886c64 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:65-66
quote: if let Some(payload) = result.get("ok") { return Ok(ok(payload.clone()));
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
@row_add n=190 row=contract/viola-wait/wake area=6-e2e-test-strategy rec=fct-fdbwlgnw@1
boundary: viola wait
item: wake on turn-ended
promise: wait stays parked through activity and wheel records and wakes on turn-ended with its cursor; a re-run from that cursor returns the same event at once.
@row_check n=190 row=contract/viola-wait/wake sha=c071bddb429c v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:99-106
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired, ];
at: src/run/wait.rs:155
quote: return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
at: src/run/wait.rs:136
quote: let after = count(params, "after")?;
@row_add n=191 row=contract/viola-wait/timed-out area=6-e2e-test-strategy rec=fct-fdbwlgnw@1
boundary: viola wait
item: timed_out
promise: A wait past the log end times out with {timed_out:true}.
@row_check n=191 row=contract/viola-wait/timed-out sha=b75213187976 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:161
quote: return Ok(json!({"ok": {"timed_out": true}}));
@row_add n=192 row=contract/viola-last/newest-message area=6-e2e-test-strategy rec=fct-fdbwlgnw@1
boundary: viola last
item: newest turn message
promise: last returns the newest turn's message, and the MCP result is equal to the CLI's.
@row_check n=192 row=contract/viola-last/newest-message sha=9c33cd5661e0 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:80
quote: feed.newest = Some(Turn::of(&line.data, &line.ts));
at: crates/viola-mcp/src/result.rs:65-66
quote: if let Some(payload) = result.get("ok") { return Ok(ok(payload.clone()));
@row_add n=193 row=contract/wait-and-last/killed-wrapper area=6-e2e-test-strategy rec=fct-fdbwlgnw@1
boundary: viola wait and last
item: killed wrapper
promise: A killed wrapper gives CLI exit 21 and MCP instance-unreachable.
@row_check n=193 row=contract/wait-and-last/killed-wrapper sha=f50fd2e700b5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => { return Ok(Err(Unreachable::Lost));
at: src/cmd/client.rs:265
quote: 21
at: crates/viola-mcp/src/result.rs:54
quote: Err(ChannelError::Closed | ChannelError::Io(_)) => {
@row_add n=194 row=contract/dialog-event/logged-once area=6-e2e-test-strategy rec=fct-zjfsmr2z@1
boundary: the event log
item: dialog event
promise: Each dialog event is logged exactly once with a dialog_id, and wait returns it at once.
@row_check n=194 row=contract/dialog-event/logged-once sha=dabd51c16f96 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:288
quote: data.insert("dialog_id".to_owned(), json!(dialog_id));
at: src/run/dialog.rs:302-303
quote: let appended = self.feed.appending_dialog(hold, || { append_event_at(&self.instance_dir, |_| line.clone())
@row_add n=195 row=contract/hook-stdout/decision-body area=6-e2e-test-strategy rec=fct-zjfsmr2z@1
boundary: hook stdout
item: decision body
promise: The hook prints the exact decision body per dialog kind, pinned by insta snapshots.
@row_check n=195 row=contract/hook-stdout/decision-body sha=5d272ae2b142 v=stale judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:687
quote: insta::assert_snapshot!(
shows: Decision bodies are pinned by insta snapshots named per dialog kind and response (question_answered, plan_approved, permission_allowed and so on); no snapshot is keyed by CLI version and decision_body takes no version argument.
cell: promise
value: The hook prints the exact decision body per dialog kind, pinned by insta snapshots.
@row_add n=196 row=contract/hook-stdout/concurrent-dialog area=6-e2e-test-strategy rec=fct-zjfsmr2z@1
boundary: hook stdout
item: second concurrent dialog
promise: A second concurrent dialog gets an empty hook stdout.
@row_check n=196 row=contract/hook-stdout/concurrent-dialog sha=f2724bbef9f4 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:252
quote: && state.pending.is_none()
at: src/run/dialog.rs:271-272
quote: if !hold { return Ok(reply(dialog_id, None));
@row_add n=197 row=decision/decision-body-gate area=6-e2e-test-strategy rec=fct-zjfsmr2z@1
name: Decision body gate
rule: No dialog decision body is ever emitted unless the wheel is driver and the CLI version is stamped.
@row_check n=197 row=decision/decision-body-gate sha=4bb133b8dfa3 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:251-254
quote: let hold = self.cli_verified && state.pending.is_none() && !unanswerable && self.wheel.holder() == Wheel::Driver;
@row_add n=198 row=contract/wheel-record/human-input area=6-e2e-test-strategy rec=fct-x4oyim7i@1
boundary: the event log
item: wheel human-input
promise: A human key logs wheel{holder:"human",cause:"human-input"} and the human's prompt still lands.
@row_check n=198 row=contract/wheel-record/human-input sha=3ce483c36eaf v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
at: src/run/wheel.rs:289
quote: json!({"holder": step.to.as_str(), "cause": step.cause.as_str()}),
at: crates/viola-pty/src/pump.rs:100
quote: writer.write_all(buf)?;
@row_add n=199 row=contract/viola-send/exit-10 area=6-e2e-test-strategy rec=fct-x4oyim7i@1
boundary: viola send
item: 10
promise: A driver send while the human holds the wheel exits 10, with detail null, or manual-pause after pause.
@row_check n=199 row=contract/viola-send/exit-10 sha=28a2d25014c0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
at: src/run/wheel.rs:114
quote: .then(|| (held.cause == WheelCause::ManualPause).then_some(HumanTyping::ManualPause))
@row_add n=200 row=contract/viola-release/exit-20 area=6-e2e-test-strategy rec=fct-x4oyim7i@1
boundary: viola release
item: 20
promise: release returns the wheel; release with from exits 20.
@row_check n=200 row=contract/viola-release/exit-20 sha=6ac6ab7c40ab v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
at: src/cmd/client.rs:245
quote: 20
@row_add n=201 row=decision/injected-turn-keeps-wheel area=6-e2e-test-strategy rec=fct-x4oyim7i@1
name: Harness-injected turn keeps the wheel
rule: A harness-injected turn flips no wheel, and send is refused turn-running until that turn's turn-ended.
@row_check n=201 row=decision/injected-turn-keeps-wheel sha=d49991beb889 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:219-220
quote: if line.kind == EventKind::PromptSubmitted && line.data["origin"] == "human" { slot.wheel.human_input();
at: src/run/send/slot.rs:211
quote: EventKind::PromptSubmitted => slot.wheel.turn_started(),
at: src/run/send.rs:114-116
quote: if flight.send.is_some() || slot.wheel.turn_running() { drop(flight); return ctx.not_delivered(None, NotDelivered::TurnRunning);
@row_add n=202 row=decision/no-release-in-driver-hints area=6-e2e-test-strategy rec=fct-x4oyim7i@1,fct-jtonrpse@2,fct-ndx7lm2p@2
name: No release in driver-facing hints
rule: Driver-facing hints, refusal hints included, never suggest or name viola release.
guard: no_hint_names_release
@row_check n=202 row=decision/no-release-in-driver-hints sha=8a6ffa65c327 v=holds guard=true judge=sonnet head=8834195b0683
searched: release · in src/human/hints.rs (only the doc comment at line 127 and the test assert at line 162, no hint string)
guard_at: src/human/hints.rs:129
@row_add n=203 row=contract/hook-statusline/passthrough area=6-e2e-test-strategy rec=fct-jtonrpse@2
boundary: viola hook statusline
item: statusline output
promise: The statusline output passes through byte for byte, and budget.json holds the fed reading.
@row_check n=203 row=contract/hook-statusline/passthrough sha=cffc04a15ffb v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:127
quote: reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
@row_add n=204 row=contract/viola-send/exit-11 area=6-e2e-test-strategy rec=fct-jtonrpse@2
boundary: viola send
item: 11
promise: A budget crossing logs budget-gate{paused:true} and send is refused exit 11 with its window word.
@row_check n=204 row=contract/viola-send/exit-11 sha=b7f5e04f222b v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:68
quote: RefusalReason::BudgetPaused => 11,
at: src/run/send/budget.rs:12
quote: Some(crossed.window.as_str()),
at: src/run/budget.rs:125
quote: let mut data = json!({"paused": self.paused.is_some()});
@row_add n=205 row=decision/budget-pause-threshold area=6-e2e-test-strategy rec=fct-jtonrpse@2
name: Budget pause threshold
rule: A budget reading of exactly 90 pauses the driver and 89 does not.
@row_check n=205 row=decision/budget-pause-threshold sha=b268afe36766 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:30
quote: const FIVE_HOUR_DEFAULT: u8 = 90;
at: src/run/budget.rs:157
quote: (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
@row_add n=206 row=contract/viola-release/budget area=6-e2e-test-strategy rec=fct-jtonrpse@2
boundary: viola release
item: --budget
promise: release --budget lifts the budget pause for its own instance only.
@row_check n=206 row=contract/viola-release/budget sha=acb8c44340d6 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:213-214
quote: if budget { let lifted = gate.lift().map_err(|_| ProtocolError::Internal)?;
at: src/run/budget.rs:309
quote: pub(crate) fn lift(&self) -> Result<Option<String>, StateError> {
@row_add n=207 row=decision/statusline-command-after-strict-modes area=6-e2e-test-strategy rec=fct-jtonrpse@2
name: Status-line command gated by strict modes
rule: The configured statusline_command never runs when the strict-modes check fails.
@row_check n=207 row=decision/statusline-command-after-strict-modes sha=93db95821f31 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42-43
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=208 row=decision/unverified-cli-send-only area=6-e2e-test-strategy rec=fct-v74hzzdp@1
name: Unverified CLI degrades to send only
rule: On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer is refused.
@row_check n=208 row=decision/unverified-cli-send-only sha=4ae097eb6bc6 v=holds judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:31
quote: Some(_) => Confirms::Nothing,
at: src/run/dialog.rs:251
quote: let hold = self.cli_verified
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
@row_add n=209 row=decision/unwrapped-name-never-a-target area=6-e2e-test-strategy rec=fct-nzkgjegi@1
name: Unwrapped row name is never a target
rule: A send naming an unwrapped list row's name exits 21 with detail unwrapped-name: a row name is never a target.
guard: e1_send_naming_an_unwrapped_row_is_unreachable
@row_check n=209 row=decision/unwrapped-name-never-a-target sha=ce3fe7ba5f94 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:113
quote: _ => UnreachableCause::UnwrappedName,
at: src/cmd/client.rs:145
quote: Self::Unwrapped => "unwrapped-name",
guard_at: tests/cli_list.rs:370
@row_add n=210 row=decision/child-env-strip area=6-e2e-test-strategy rec=fct-tenptmlu@2
name: Child environment strip
rule: No stripped environment name reaches the agent child.
guard: the secret scan of the canary values
except: a name listed in claude_env_keep survives
@row_check n=210 row=decision/child-env-strip sha=ef9b1550d941 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:72-76
quote: let floor = IDENTITY_FLOOR.contains(&text.as_str()); if !floor && persistent.contains(&text) { plan.kept.push(text); } else { plan.remove.push(name);
at: crates/viola-pty/src/lib.rs:140-141
quote: for key in &spec.env_remove { cmd.env_remove(key);
guard_at: .github/workflows/ci.yml:76
@row_add n=211 row=contract/process-start/claude-child area=6-e2e-test-strategy rec=fct-tenptmlu@2
boundary: the event log
item: process-start{subject:"claude-child"}
promise: The record carries the strip counts and the stripped names only, never their values.
@row_check n=211 row=contract/process-start/claude-child sha=ab3286dc0724 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:54
quote: subject = "claude-child",
at: src/run/mod.rs:57-58
quote: env_stripped_count = strip.count(), env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
at: crates/viola-agent-claude/src/lib.rs:46
quote: joined(self.remove.iter().map(|n| n.to_string_lossy().into_owned()))
@row_add n=212 row=decision/link-is-idempotent area=6-e2e-test-strategy rec=fct-b5i2ykog@1,fct-lhyzlw5x@1
name: Link is idempotent
rule: Re-linking a driver to the same driven instance adds no second link record.
@row_check n=212 row=decision/link-is-idempotent sha=a22c4f1ed76d v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62-64
quote: if let Some(since) = self.since(&set, driver) { return Ok((since, true)); }
@row_add n=213 row=contract/event-log/unlink area=6-e2e-test-strategy rec=fct-b5i2ykog@1
boundary: the event log
item: unlink record
promise: viola unlink logs an unlink record.
@row_check n=213 row=contract/event-log/unlink sha=f192833ef742 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
@row_add n=214 row=decision/driver-name-self-reported area=6-e2e-test-strategy rec=fct-7bm6a42y@1
name: Driver identity is self-reported
rule: The driver of a link is the self-reported VIOLA_NAME and need not be a booted instance.
@row_check n=214 row=decision/driver-name-self-reported sha=72d8598a0cb4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:210
quote: let raw = std::env::var_os("VIOLA_NAME")?.into_string().ok()?;
at: src/run/links.rs:94
quote: let (driver, driven) = (named("driver")?, named("driven")?);
at: src/run/links.rs:95
quote: if driven != self.name || driver == driven {
@row_add n=215 row=decision/unlink-survives-restart area=6-e2e-test-strategy rec=fct-7bm6a42y@1
name: Unlink survives a wrapper restart
rule: An unlink holds across a wrapper restart.
guard: links_an_unlink_survives_a_wrapper_restart
@row_check n=215 row=decision/unlink-survives-restart sha=7ea244d3ca75 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/links.rs:45-46
quote: for line in read_from(&self.instance_dir, 0)? { apply_link_line(&mut restored, &line?.value);
at: crates/viola-state/src/replay.rs:94
quote: (Some(EventKind::Unlink), Some(at)) => {
guard_at: tests/cli_links.rs:572
@row_add n=216 row=decision/implicit-link-on-send area=6-e2e-test-strategy rec=fct-7bm6a42y@1
name: Implicit link on send or answer
rule: A send or answer from another instance records an implicit link.
@row_check n=216 row=decision/implicit-link-on-send sha=abc38ca040e7 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:142-145
quote: if let (Some(links), Some(from)) = (slot.links.get(), ctx.from.as_ref()) && from != name { links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/dialog.rs:452
quote: if let (Some(links), Some(from)) = (self.links.get(), from.as_ref())
@row_add n=217 row=decision/start-hands-wheel-to-driver area=6-e2e-test-strategy rec=fct-uq5phmo2@1
name: Start hands the wheel to the driver
rule: Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
@row_check n=217 row=decision/start-hands-wheel-to-driver sha=7c0bdf287cea v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:620
quote: json!({"holder": Wheel::Driver.as_str(), "cause": WheelCause::Start.as_str()}),
@row_add n=218 row=decision/hook-suites-in-integration-layer area=6-e2e-test-strategy rec=fct-guyemqyz@1
name: Non-path suites layer
rule: Non-path trigger-coverage suites run in the E2E layer.
except: root hook_* files run in the integration layer
@row_check n=218 row=decision/hook-suites-in-integration-layer sha=cd73ec8d05fd v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/nextest.rs:15
quote: pub(super) const E2E: &str = "binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)";
at: crates/viola-e2e/src/harness/run/nextest.rs:14
quote: "kind(test) & !binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)";
@row_add n=219 row=contract/mcp-tools/invalid-params area=6-e2e-test-strategy rec=fct-4aawmv2h@2
boundary: MCP tools
item: invalid-params
promise: A schema-violating call is answered invalid-params naming its field, and no frame is written.
@row_check n=219 row=contract/mcp-tools/invalid-params sha=559dc04cc0a5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:372
quote: (Err(field), _) | (_, Err(field)) => return Ok(result::invalid(field)),
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
@row_add n=220 row=decision/panic-seam-test-only area=6-e2e-test-strategy rec=fct-4aawmv2h@2
name: Panic seam is test-only
rule: The forced-panic seam fires only under the fake-agent feature and never reaches the release build.
@row_check n=220 row=decision/panic-seam-test-only sha=9f8534dc05e7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/seam.rs:16-17
quote: #[cfg(feature = "fake-agent")] fn panic_on(value: Option<OsString>) {
at: src/cmd/hook.rs:128-129
quote: #[cfg(feature = "fake-agent")] seam::panic_if_asked();
@row_add n=221 row=decision/error-payloads-sanitized area=6-e2e-test-strategy rec=fct-gjahavf7@1
name: Error payloads sanitized
rule: No error payload carries an absolute path, a serde path or a Caused by chain.
guard: tests/contract_error_bodies.rs census of writers
@row_check n=221 row=decision/error-payloads-sanitized sha=55a7c18de34f v=holds guard=true judge=sonnet head=8834195b0683
at: tests/support/events.rs:30
quote: } else if text.contains("Caused by") {
guard_at: tests/contract_error_bodies.rs:771
@row_add n=222 row=decision/diagnostics-files-owner-only area=6-e2e-test-strategy rec=fct-gjahavf7@1
name: Diagnostics files owner-only
rule: Diagnostics files are mode 0600 on Unix.
guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
@row_check n=222 row=decision/diagnostics-files-owner-only sha=68ac649cf402 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:48
quote: std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
guard_at: src/obs.rs:660
@row_add n=223 row=decision/scan-hits-never-echo-bytes area=6-e2e-test-strategy rec=fct-gjahavf7@1
name: Scan hits never echo bytes
rule: A secret-scan hit is reported by place and class, never by the matched bytes.
@row_check n=223 row=decision/scan-hits-never-echo-bytes sha=587b3906e1db v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
@row_add n=224 row=decision/diagnostics-schema-conformance area=6-e2e-test-strategy rec=fct-nfrombzn@1
name: Diagnostics schema conformance
rule: After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step.
why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
@row_check n=224 row=decision/diagnostics-schema-conformance sha=447a4c17612c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:110-113
quote: let schema = match kind { DiagKind::Role => &line_schema, DiagKind::Detail => &detail_schema, };
at: crates/viola-e2e/src/harness/schema_check.rs:132
quote: let ok = !empty && failures.is_empty();
@row_add n=225 row=decision/jsonschema-validator area=6-e2e-test-strategy rec=fct-nfrombzn@1
name: jsonschema validates diagnostics
rule: jsonschema is what validates diagnostics lines against their schemas.
@row_check n=225 row=decision/jsonschema-validator sha=7566c05296c7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:89
quote: .and_then(|s| jsonschema::validator_for(&s).ok())
@row_add n=226 row=decision/non-json-lines-skipped area=6-e2e-test-strategy rec=fct-nfrombzn@1
name: Non-JSON diagnostics lines skipped
rule: A diagnostics line that is not JSON is skipped and counted, not failed.
why: chaos cases tear lines by design
@row_check n=226 row=decision/non-json-lines-skipped sha=619fc19d123e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:115-117
quote: let Ok(line) = serde_json::from_str::<Value>(raw) else { torn += 1; continue;
@row_add n=227 row=decision/schema-failure-no-content area=6-e2e-test-strategy rec=fct-nfrombzn@1
name: Schema failure report without content
rule: A schema failure prints file, line number and the failing schema keyword, never the line's content.
@row_check n=227 row=decision/schema-failure-no-content sha=7344ca311e93 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:126
quote: failures.push(json!({"file": file, "line": n + 1, "keyword": keyword}));
@row_add n=228 row=decision/hint-names-its-cause area=6-e2e-test-strategy rec=fct-ndx7lm2p@2
name: Each exit cause has its own hint
rule: Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint.
guard: no_two_codes_share_a_hint
@row_check n=228 row=decision/hint-names-its-cause sha=109201eb6268 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cross_exit_causes.rs:109
quote: .filter(|hint| held[hint].len() > 1)
guard_at: tests/cross_exit_causes.rs:116
@row_add n=229 row=contract/exit-causes/socket-dir-refused area=6-e2e-test-strategy rec=fct-ndx7lm2p@2
boundary: exit causes
item: socket-dir-refused
promise: A loosened socket directory is socket-dir-refused, at exit 21 on a frame verb and at exit 1 on a start.
@row_check n=229 row=contract/exit-causes/socket-dir-refused sha=1e82691468d1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=230 row=decision/socket-dir-refusal area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Socket directory refusal
rule: A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named.
guard: security_negatives_socket_dir
@row_check n=230 row=decision/socket-dir-refusal sha=e53968e8a385 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:511
quote: if check_endpoint_dir(endpoint).is_err() {
at: src/cmd/hook.rs:232
quote: return Err(NotSent::SocketDir);
guard_at: tests/security_negatives_socket_dir.rs:205
@row_add n=231 row=decision/link-name-validation area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Raw-channel link name validation
rule: link and unlink on the raw channel validate their names.
guard: channel_links
@row_check n=231 row=decision/link-name-validation sha=af79b41e0bef v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/links.rs:91
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
guard_at: tests/channel_links.rs:54
@row_add n=232 row=decision/home-readable-by-others-refused area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Home readable by others refused
rule: A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21.
guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
@row_check n=232 row=decision/home-readable-by-others-refused sha=302c1614af1e v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/cmd/list.rs:47
quote: return Ok(ExitCode::from(21));
guard_at: tests/cli_list.rs:555
@row_add n=233 row=decision/frame-verbs-refuse-widened-instance area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Frame verbs refuse a widened instance
rule: The six CLI frame verbs refuse a widened instance with exit 21, writing no frame and no line.
guard: security_negatives_strict_modes
@row_check n=233 row=decision/frame-verbs-refuse-widened-instance sha=fac9ea3d8a8e v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:40
quote: Ok(Some(21))
guard_at: tests/security_negatives_strict_modes.rs:82
@row_add n=234 row=decision/list-board-refusal-opens-no-log area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Board refusal opens no log
rule: viola list's whole-board strict-modes refusal exits 21 and opens no log.
guard: strict_modes_on_the_board_is_exit_21_and_opens_no_log
@row_check n=234 row=decision/list-board-refusal-opens-no-log sha=50b3416c0f53 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/list.rs:43
quote: let scan = match board::scan(&home)? {
at: src/cmd/list.rs:47
quote: return Ok(ExitCode::from(21));
guard_at: tests/cross_exit_causes.rs:269
@row_add n=235 row=decision/plugin-install-refusal-creates-nothing area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Plugin install refusal creates nothing
rule: viola plugin install refused by strict modes exits 21 and creates nothing.
guard: strict_modes_on_plugin_install_is_exit_21_and_creates_nothing
@row_check n=235 row=decision/plugin-install-refusal-creates-nothing sha=c2ee94c11381 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:53
quote: if !tree_passes(&home) {
at: src/cmd/plugin.rs:60
quote: return Ok(ExitCode::from(21));
guard_at: tests/cross_exit_causes.rs:284
@row_add n=236 row=decision/list-names-escaped area=6-e2e-test-strategy rec=fct-nsd7seww@1
name: List names escaped
rule: Unwrapped session names holding newline, tab or a C1 character print escaped in every mode: no row splits, columns stay aligned to the caption row and the raw bytes never reach a NAME cell.
guard: cross_list_rows
@row_check n=236 row=decision/list-names-escaped sha=8e67b62b7277 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
guard_at: tests/cross_list_rows.rs:76
@row_add n=237 row=contract/mcp-cli-parity/refusal area=6-e2e-test-strategy rec=fct-4hhqtfrz@1
boundary: MCP and CLI parity
item: refusal
promise: For the same refusal the MCP structured_content {refusal, detail} equals the CLI --json {refusal, detail}.
@row_check n=237 row=contract/mcp-cli-parity/refusal sha=3898cf40f643 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
at: src/cmd/client.rs:87
quote: Some(Reply::Refused { reason, detail })
@row_add n=238 row=contract/mcp-cli-parity/exit-21 area=6-e2e-test-strategy rec=fct-4hhqtfrz@1
boundary: MCP and CLI parity
item: exit 21 causes
promise: For each cause of exit 21 the MCP isError body {error, detail} equals the CLI --json document less v, its text refused: instance-unreachable.
@row_check n=238 row=contract/mcp-cli-parity/exit-21 sha=c47a28e08b9d v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=239 row=decision/torn-append-healed area=6-e2e-test-strategy rec=fct-47fi4ugg@2
name: Torn append healed
rule: A torn log append is counted and the next record starts on a fresh line.
@row_check n=239 row=decision/torn-append-healed sha=75c524723643 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
@row_add n=240 row=contract/viola-wait/endpoint-lost area=6-e2e-test-strategy rec=fct-47fi4ugg@2
boundary: viola wait
item: endpoint-lost
promise: An endpoint vanishing during wait exits 21 with endpoint-lost.
@row_check n=240 row=contract/viola-wait/endpoint-lost sha=53780dafcdbf v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
at: src/cmd/client.rs:265
quote: 21
@row_add n=241 row=contract/viola-run/stale-heartbeat area=6-e2e-test-strategy rec=fct-47fi4ugg@2
boundary: viola run
item: stale-heartbeat
promise: A stopped wrapper is read stale, and a new run of its name is refused stale-heartbeat.
@row_check n=241 row=contract/viola-run/stale-heartbeat sha=d2270c302e95 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:33
quote: _ => Liveness::Stale,
at: src/cmd/run.rs:455-457
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || { refuse_stale(name); })),
@row_add n=242 row=decision/forward-compat-lines-skipped area=6-e2e-test-strategy rec=fct-47fi4ugg@2
name: Forward-compatible lines skipped
rule: Log lines of a newer form are counted in skipped rather than failing the read.
@row_check n=242 row=decision/forward-compat-lines-skipped sha=62b8dedae4d5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
at: crates/viola-state/src/events.rs:266
quote: self.skipped.unknown_fields += 1;
@row_add n=243 row=decision/fuzz-separate-workspace area=6-e2e-test-strategy rec=fct-advgjcvl@1
name: Fuzz targets in a separate workspace
rule: cargo-fuzz targets live in a separate fuzz workspace and join as their parsers land.
@row_check n=243 row=decision/fuzz-separate-workspace sha=b2da43e4a118 v=holds judge=sonnet head=8834195b0683
at: fuzz/Cargo.toml:20-21
quote: [workspace] members = ["."]
at: Cargo.toml:182
quote: exclude = ["fuzz"]
@row_add n=244 row=decision/fake-agent-matches-recordings area=6-e2e-test-strategy rec=fct-q6ltykaz@1,fct-vnlgjdeq@1
name: Fake agent matches recordings byte for byte
rule: For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
@row_check n=244 row=decision/fake-agent-matches-recordings sha=0f61f7ddc2d7 v=holds judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:142
quote: assert!(!versions.is_empty(), "no recorded fixture set to compare");
at: tests/contract_fake_agent_drift.rs:90
quote: let recorded = fs::read(fixtures.join(version).join(format!("{event}.default.json"))).ok();
@row_add n=245 row=decision/pinned-exe-hash-vectors area=6-e2e-test-strategy rec=fct-q6ltykaz@1
name: Pinned-exe hash pinned by vectors
rule: The pinned-exe content hash is pinned to its library by literal test vectors.
@row_check n=245 row=decision/pinned-exe-hash-vectors sha=c5ca61a82885 v=holds judge=sonnet head=8834195b0683
at: tests/contract_content_hash.rs:24
quote: fn sha256_published_vector_matches_digest(#[case] message: &[u8], #[case] digest: &str) {
@row_add n=246 row=decision/no-unbounded-json-depth area=6-e2e-test-strategy rec=fct-q6ltykaz@1
name: No unbounded JSON depth
rule: serde_json's unbounded_depth feature is banned.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_check n=246 row=decision/no-unbounded-json-depth sha=48585e202f4f v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:55
quote: deny = ["unbounded_depth"]
guard_at: tests/contract_bounded_readers.rs:764
@row_add n=247 row=decision/rstest-fixtures area=7-test-data-fixtures rec=fct-khyzsutg@1
name: rstest for fixtures
rule: rstest is what provides test fixtures and cases.
@row_check n=247 row=decision/rstest-fixtures sha=49a2d8fe686d v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:224
quote: rstest = { version = "=0.27.0", default-features = false }
at: tests/security_negatives_strict_modes.rs:250
quote: #[rstest]
@row_add n=248 row=decision/tempfile-test-dirs area=7-test-data-fixtures rec=fct-khyzsutg@1
name: tempfile for test directories
rule: tempfile is what provides temporary test directories.
@row_check n=248 row=decision/tempfile-test-dirs sha=a543ae8226c1 v=holds judge=sonnet head=8834195b0683
at: tests/run_cli.rs:23
quote: let dir = tempfile::tempdir().expect("receipt dir");
@row_add n=249 row=decision/insta-snapshots area=7-test-data-fixtures rec=fct-khyzsutg@1
name: insta for snapshots
rule: insta is what holds snapshot expectations.
@row_check n=249 row=decision/insta-snapshots sha=d1fe190aedd8 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:210
quote: insta = { version = "=1.48.0", default-features = false }
at: crates/viola-agent-claude/src/dialog.rs:687
quote: insta::assert_snapshot!(
@row_add n=250 row=decision/fixtures-per-cli-version area=7-test-data-fixtures rec=fct-khyzsutg@1
name: Recorded fixtures per CLI version
rule: Recorded verify, dialog, framing and MCP fixtures live in one directory per agent CLI version.
@row_check n=250 row=decision/fixtures-per-cli-version sha=2dd6d468b974 v=holds judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:90
quote: let recorded = fs::read(fixtures.join(version).join(format!("{event}.default.json"))).ok();
@row_add n=251 row=decision/budget-file-from-statusline-hook area=7-test-data-fixtures rec=fct-sgyzq76e@1
name: Budget file from the statusline hook
rule: budget.json comes from generated statusline stdin piped into viola hook statusline; the reader's own unit cases alter only bytes the writer wrote.
@row_check n=251 row=decision/budget-file-from-statusline-hook sha=c8cbdd76d4e8 v=holds judge=sonnet head=8834195b0683
at: tests/support/statusline.rs:55
quote: .args(["hook", "statusline"])
@row_add n=252 row=decision/statusline-source-read-only area=7-test-data-fixtures rec=fct-iwzuwruj@1
name: Statusline source is read-only to viola
rule: The statusline source is written by the test, never by viola, which only reads it.
@row_check n=252 row=decision/statusline-source-read-only sha=9bd24c2239d3 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:302
quote: fs::write(&path, doc.to_string()).expect("statusline source");
at: src/cmd/run.rs:533
quote: let own = home.join(STATUSLINE_SOURCE);
@row_add n=253 row=decision/fake-agent-on-test-path area=7-test-data-fixtures rec=fct-ew5v6fx5@1
name: Fake agent on a test-scoped PATH
rule: The fake agent stands as claude in a test-scoped bin dir, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
@row_check n=253 row=decision/fake-agent-on-test-path sha=a1f14fa316a4 v=holds judge=sonnet head=8834195b0683
at: tests/support/fake.rs:40
quote: if !claude.is_file() && fs::hard_link(FAKE, &claude).is_err() {
at: tests/support/fake.rs:44
quote: fs::write(dir.join("agents-mode"), mode).expect("the agents mode");
at: tests/support/home.rs:868
quote: pub fn path_with(dir: &Path) -> String {
@row_add n=254 row=decision/hook-payloads-recorded-locally area=7-test-data-fixtures rec=fct-tq5dzqhd@1
name: Hook payloads recorded locally, replayed in CI
rule: Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
@row_check n=254 row=decision/hook-payloads-recorded-locally sha=132642e82bc8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:585
quote: let scrubbed = ledger::scrub(&payload, home, user);
at: src/cmd/verify.rs:617
quote: std::fs::write(out.join(name), text)?;
@row_add n=255 row=decision/dirty-payload-refuses-recording area=7-test-data-fixtures rec=fct-tq5dzqhd@1,fct-auxau5hd@1
name: Dirty payload refuses the recording
rule: A recorded payload or MCP exchange that fails the unclean check refuses the whole recording.
@row_check n=255 row=decision/dirty-payload-refuses-recording sha=907c6aa77a08 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:586
quote: if let Some(why) = ledger::unclean(&scrubbed, user) {
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:212
quote: let scrubbed = ledger::scrub(&doc, &home, &user);
@row_add n=256 row=decision/human-output-as-literals area=7-test-data-fixtures rec=fct-puo2hazj@2
name: Human-mode output as literals
rule: Expected human-mode output, such as help text and the list board, is held as literals in its test files.
@row_check n=256 row=decision/human-output-as-literals sha=c5d469016e7b v=holds judge=sonnet head=8834195b0683
at: tests/cli_output_plain.rs:55-56
quote: const HELP: &str = concat!( "Usage: viola [--home <dir>] <verb> [args]\n",
@row_add n=257 row=decision/decision-bodies-snapshotted area=7-test-data-fixtures rec=fct-qx745xaf@1
name: Decision bodies snapshotted
rule: Dialog decision bodies are held as insta snapshots, regenerated only by the local fixture refresh.
@row_check n=257 row=decision/decision-bodies-snapshotted sha=1611e6d83c61 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:687
quote: insta::assert_snapshot!(
@row_add n=258 row=decision/proptest-persisted-seeds area=7-test-data-fixtures rec=fct-ehbrsv74@1,fct-fpfimgp4@1
name: proptest with persisted seeds
rule: Randomized test data comes from proptest strategies, and their seeds are persisted in committed proptest-regressions files.
@row_check n=258 row=decision/proptest-persisted-seeds sha=3b1cae02fade v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:420
quote: failure_persistence: Some(Box::new(FileFailurePersistence::SourceParallel(
at: crates/viola-core/proptest-regressions/lib.txt:1
quote: # Seeds for failure cases proptest has generated in the past. It is
@row_add n=259 row=decision/time-as-argument area=7-test-data-fixtures rec=fct-jrpg7it3@1,fct-46mmbbun@1
name: pure time decisions take their instant
rule: A pure time decision, such as the budget gate's override expiry, takes its instant as an argument instead of reading a clock.
@row_check n=259 row=decision/time-as-argument sha=ab9b7fc8df88 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:173
quote: if let Some(end) = override_until.filter(|end| later(end, now)) {
@row_add n=260 row=contract/fake-agent-cli/cli-version area=7-test-data-fixtures rec=fct-izkbqvor@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: --cli-version
promise: The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
@row_check n=260 row=contract/fake-agent-cli/cli-version sha=43b06f636faf v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:162
quote: format!("{v} (Claude Code)")
@row_add n=261 row=contract/fake-agent-hooks/command-spawn area=7-test-data-fixtures rec=fct-tx5uvsd4@1,fct-gbrktuzh@1
boundary: viola-fake-agent hook execution
item: hook command
promise: The fake agent reads the hook commands from the plugin dir run passes and runs every command entry for the event; an absolute command is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run.
@row_check n=261 row=contract/fake-agent-hooks/command-spawn sha=69d9960f16d7 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:620
quote: let absolute = Path::new(command).is_absolute();
at: src/bin/viola-fake-agent.rs:631
quote: let mut cmd = Command::new(command);
@row_add n=262 row=contract/fake-agent-hooks/stdin-payload area=7-test-data-fixtures rec=fct-tx5uvsd4@1,fct-gbrktuzh@1
boundary: viola-fake-agent hook execution
item: hook stdin payload
promise: The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.
@row_check n=262 row=contract/fake-agent-hooks/stdin-payload sha=05a78a850c76 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:341-343
quote: if set.is_empty() { return fixture; }
@row_add n=263 row=contract/fake-agent-hooks/status-line-command area=7-test-data-fixtures rec=fct-tx5uvsd4@1,fct-gbrktuzh@1
boundary: viola-fake-agent hook execution
item: status line command
promise: The fake agent runs a status line command only when the command is absolute.
@row_check n=263 row=contract/fake-agent-hooks/status-line-command sha=c5f7095b4998 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:622-625
quote: if !absolute { fields.insert("ran".to_owned(), json!(false)); self.receipt.write(kind, Value::Object(fields)); return false;
@row_add n=264 row=contract/fake-agent-events/session-start area=7-test-data-fixtures rec=fct-cv6ihlor@1,fct-gbrktuzh@1
boundary: viola-fake-agent fired events
item: SessionStart
promise: After start_receipts the fake agent fires SessionStart (variant default) once, silent without a registered hook or a fixture, like every fire.
@row_check n=264 row=contract/fake-agent-events/session-start sha=46c9577bbe44 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:1184-1188
quote: agent.fire( "SessionStart", agent.launch_variant(), &agent.opts.resume_fields(), );
@row_add n=265 row=contract/fake-agent-input/bracketed-paste area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: bracketed paste
promise: Bracketed pastes plus Enter are accepted as one prompt and fire UserPromptSubmit (variant default) with that text.
@row_check n=265 row=contract/fake-agent-input/bracketed-paste sha=ef041a542ea5 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:707
quote: None => self.fire("UserPromptSubmit", "default", &[("prompt", &text)]),
@row_add n=266 row=contract/fake-agent-input/framing-paste area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: --framing paste
promise: Under --framing a compiled paste text of verify's Run B fires its recorded paste-1, paste-2 or paste-3 variant instead, its bytes unchanged; a set without the variant fires nothing for it.
@row_check n=266 row=contract/fake-agent-input/framing-paste sha=4bbe310b393a v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:706
quote: Some(stem) => self.fire("UserPromptSubmit", stem, &[]),
at: crates/viola-agent-claude/src/ledger.rs:203
quote: (PROBE_LONG_PASTE, "paste-1"),
@row_add n=267 row=contract/fake-agent-input/clear area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: /clear
promise: Under --framing the probed /clear fires the recorded clear-1 SessionEnd then SessionStart and no UserPromptSubmit.
@row_check n=267 row=contract/fake-agent-input/clear sha=18ee17b64851 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:701-703
quote: } else if let Some(stem) = local { self.fire("SessionEnd", stem, &[]); self.fire("SessionStart", stem, &[])
@row_add n=268 row=contract/fake-agent-input/compact area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: /compact
promise: Under --framing the probed /compact fires the set's recorded SessionStart.compact-1 with session_id set to the last SessionStart the agent fired (only a SessionStart whose fixture exists moves it) and no UserPromptSubmit.
@row_check n=268 row=contract/fake-agent-input/compact sha=b004611000e2 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:700
quote: self.fire("SessionStart", stem, &set)
at: src/bin/viola-fake-agent.rs:478
quote: // A SessionStart with no fixture fires nothing, and leaves the session as it was.
@row_add n=269 row=contract/fake-agent-input/keystroke area=7-test-data-fixtures rec=fct-u4efda3g@1,fct-gbrktuzh@1
boundary: viola-fake-agent input
item: keystroke
promise: Every byte typed outside a paste is a keystroke, and a CR submits what was typed.
@row_check n=269 row=contract/fake-agent-input/keystroke sha=d9e3e8b1a2e4 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:1000-1003
quote: b'\r' => { let mut actions = vec![Action::Key(byte)]; if !self.prompt.is_empty() { actions.push(Action::Submit(std::mem::take(&mut self.prompt)));
@row_add n=270 row=decision/control-file-gating area=7-test-data-fixtures rec=fct-ktnlwgrn@1
name: control-file gating of fake agent steps
rule: Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep.
why: Deterministic ordering without sleeps.
@row_check n=270 row=decision/control-file-gating sha=2ad709040490 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:778-779
quote: if step.gate {                 self.wait_control(&mut offset);
@row_add n=271 row=contract/fake-agent-cli/script area=7-test-data-fixtures rec=fct-ktnlwgrn@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: --script
promise: The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse and Stop steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
@row_check n=271 row=contract/fake-agent-cli/script sha=ad6fe24868ac v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:670
quote: args.push(workspace_path(script).into());
@row_add n=272 row=contract/fake-agent-cli/control area=7-test-data-fixtures rec=fct-ktnlwgrn@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: --control
promise: The control path is <home>/fake/<name>.control (<name>.revived.control for a revive start), and the fake agent reads it by byte offset.
@row_check n=272 row=contract/fake-agent-cli/control sha=305a0df4ea9f v=stale judge=sonnet head=8834195b0683
at: tests/support/home.rs:881
quote: let control = home.join("fake").join(format!("{name}.revived.control"));
shows: The control path is <home>/fake/<name>.control for a start, but a revive start uses <name>.revived.control, so it is not always the former; the fake agent does read it by byte offset.
cell: promise
value: The control path is <home>/fake/<name>.control (<name>.revived.control for a revive start), and the fake agent reads it by byte offset.
@row_add n=273 row=decision/synthetic-fixture-text area=7-test-data-fixtures rec=fct-ktnlwgrn@1,fct-lq7tfxzv@1
name: synthetic fixture text only
rule: Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.
@row_check n=273 row=decision/synthetic-fixture-text sha=b19dae51bcce v=holds judge=sonnet head=8834195b0683
at: fixtures/fake-scripts/path3.json:4
quote: { "event": "PostToolUse", "variant": "default", "gate": true },
@row_add n=274 row=contract/fake-agent-receipt/line area=7-test-data-fixtures rec=fct-3es552c5@1,fct-gbrktuzh@1
boundary: viola-fake-agent receipt file
item: receipt line
promise: The fake agent writes ndjson receipt lines to --receipt, one write_all per line, each carrying "v":1.
@row_check n=274 row=contract/fake-agent-receipt/line sha=a22dec032026 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:250
quote: let mut line = json!({"v": 1, "kind": kind});
at: src/bin/viola-fake-agent.rs:257
quote: let _ = f.write_all(&bytes);
@row_add n=275 row=contract/fake-agent-receipt/kind area=7-test-data-fixtures rec=fct-3es552c5@1,fct-gbrktuzh@1
boundary: viola-fake-agent receipt file
item: kind
promise: Each receipt line has a kebab kind, one of start, size, cwd, hold, env, fds, key, prompt, hook, statusline, step.
@row_check n=275 row=contract/fake-agent-receipt/kind sha=8888cf915ee2 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:917
quote: "start",
at: src/bin/viola-fake-agent.rs:1058
quote: .write("size", json!({"cols": size.cols, "rows": size.rows}));
at: src/bin/viola-fake-agent.rs:925
quote: agent.receipt.write("cwd", json!({"cwd": cwd.ok()}));
at: src/bin/viola-fake-agent.rs:1142
quote: Receipt::open(opts.receipt.as_deref()).write("hold", json!({"pid": std::process::id()}));
at: src/bin/viola-fake-agent.rs:930
quote: agent.receipt.write("env", json!({"names": names}));
at: src/bin/viola-fake-agent.rs:940
quote: agent.receipt.write("fds", json!({"fds": fds}));
at: src/bin/viola-fake-agent.rs:1112
quote: Action::Key(b) => agent.receipt.write("key", json!({"hex": hex(&[b])})),
at: src/bin/viola-fake-agent.rs:718
quote: "prompt",
at: src/bin/viola-fake-agent.rs:564
quote: self.run_receipted("hook", Some(event), command, args, body)
at: src/bin/viola-fake-agent.rs:583
quote: self.run_receipted("statusline", None, &command, &args, &body);
at: src/bin/viola-fake-agent.rs:782
quote: .write("step", json!({"index": index, "event": step.event}));
@row_add n=276 row=contract/fake-script/format area=7-test-data-fixtures rec=fct-rosshttf@1
boundary: fake agent turn script
item: script format
promise: A script is {"v":1,"steps":[{"event","variant","gate"}]} over the nine registered hook events, validated by the fake-script v1 schema.
@row_check n=276 row=contract/fake-script/format sha=ee36ac5b52c7 v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=277 row=contract/fake-script/unreadable-exit area=7-test-data-fixtures rec=fct-rosshttf@1
boundary: fake agent turn script
item: unreadable script
promise: An unreadable script makes the fake agent exit 2.
@row_check n=277 row=contract/fake-script/unreadable-exit sha=29ab8f9b875e v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:1155-1157
quote: let Some(steps) = script_steps(&opts) else { eprintln!("fake agent: unreadable script"); return ExitCode::from(2);
@row_add n=278 row=contract/fake-script/gated-step area=7-test-data-fixtures rec=fct-rosshttf@1
boundary: fake agent turn script
item: gated step
promise: A gated step waits for one more newline-terminated control line past the byte offset.
@row_check n=278 row=contract/fake-script/gated-step sha=27f6ef917b8b v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:764
quote: let nl = buf.get(start..)?.iter().position(|&b| b == b'\n')?;
at: src/bin/viola-fake-agent.rs:778-779
quote: if step.gate { self.wait_control(&mut offset);
@row_add n=279 row=contract/fake-agent-cli/argv-only-options area=7-test-data-fixtures rec=fct-yxk5mpg3@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: modes and options
promise: The fake agent's modes and options are taken from argv only, never from the environment.
@row_check n=279 row=contract/fake-agent-cli/argv-only-options sha=ef931ae943e4 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:108
quote: fn parse(args: &[String]) -> Self {
at: src/bin/viola-fake-agent.rs:1131
quote: let args: Vec<String> = std::env::args().skip(1).collect();
@row_add n=280 row=contract/fake-agent-cli/unknown-arguments area=7-test-data-fixtures rec=fct-yxk5mpg3@1,fct-gbrktuzh@1
boundary: viola-fake-agent command line
item: unknown arguments
promise: Unknown arguments are ignored.
@row_check n=280 row=contract/fake-agent-cli/unknown-arguments sha=47bff6b7b47d v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:150
quote: _ => {}
@row_add n=281 row=contract/fake-agent-exit/exit-triggers area=7-test-data-fixtures rec=fct-7o4nbra2@1,fct-gbrktuzh@1
boundary: viola-fake-agent exit
item: exit trigger
promise: The fake agent exits on \x03 or at stdin EOF; before it exits it waits for a hook still running and starts no other.
@row_check n=281 row=contract/fake-agent-exit/exit-triggers sha=1f1fae7ccfbb v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:999
quote: 0x03 => vec![Action::Exit],
at: src/bin/viola-fake-agent.rs:1109
quote: while let Ok(1) = stdin.read(&mut byte) {
at: src/bin/viola-fake-agent.rs:1116
quote: agent.close_hooks();
at: src/bin/viola-fake-agent.rs:627-629
quote: let closed = self.hooks.lock().unwrap_or_else(PoisonError::into_inner); if *closed { return false;
@row_add n=282 row=decision/fake-agent-children-own-process-group area=7-test-data-fixtures rec=fct-7o4nbra2@1
name: fake agent children in their own process group
rule: On Unix every command the fake agent spawns, each hook and the status line command, runs in a process group of its own.
why: A hangup of the PTY's foreground group, at the agent's exit or when a killed wrapper closes the PTY, would leave a cut-short coverage profile from a viola hook.
@row_check n=282 row=decision/fake-agent-children-own-process-group sha=d5d2131ecf27 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:640
quote: std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
at: src/bin/viola-fake-agent.rs:602
quote: self.run_receipted("statusline", None, &command, &shell_expanded(args), &body);
@row_add n=283 row=decision/fake-agent-exit-ignores-sighup area=7-test-data-fixtures rec=fct-7o4nbra2@1
name: fake agent exit ignores SIGHUP
rule: On Unix the fake agent ignores SIGHUP from the first step of its exit (\x03 or stdin's end), while a hangup landing before the exit begins still ends it.
why: The agent is its PTY's session leader and cannot leave that group, so a hangup during exit would cut its own exit-time coverage write.
@row_check n=283 row=decision/fake-agent-exit-ignores-sighup sha=5decdddd737e v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:1101
quote: let _ = unsafe { libc::signal(libc::SIGHUP, libc::SIG_IGN) };
@row_add n=284 row=decision/hygiene-reports-class-only area=7-test-data-fixtures rec=fct-qyi324hl@1
name: hygiene checker reports class only
rule: The hygiene checker reports a violation class only (absolute-path, username, schema), never the offending content.
@row_check n=284 row=decision/hygiene-reports-class-only sha=e42f55909838 v=holds judge=sonnet head=8834195b0683
at: tests/support/hygiene.rs:19-21
quote: Self::AbsolutePath => "absolute-path", Self::Username => "username", Self::Schema => "schema",
@row_add n=285 row=decision/planted-hygiene-violations area=7-test-data-fixtures rec=fct-qyi324hl@1,fct-hsf33llc@1
name: planted inputs prove hygiene classes red
rule: Every hygiene violation class is proven red by a planted input.
@row_check n=285 row=decision/planted-hygiene-violations sha=2219ef0f3b3e v=holds judge=sonnet head=8834195b0683
at: tests/contract_fixture_hygiene.rs:216
quote: Err(Violation::Schema)
at: tests/contract_fixture_hygiene.rs:227
quote: Err(Violation::AbsolutePath)
at: tests/contract_fixture_hygiene.rs:240
quote: Err(Violation::Username)
@row_add n=286 row=decision/recorder-scrub area=7-test-data-fixtures rec=fct-eu46zdvx@1,fct-dviikzsc@1
name: recorder scrubs home paths and usernames
rule: The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap fixture applies the same scrub.
except: The StatusLine variants' workspace.repo.owner, the remote owner's handle, is not rewritten by the whole-word OS-user rule.
@row_check n=286 row=decision/recorder-scrub sha=3b2be3d49711 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1261
quote: out = out.replace(&spelling, "~");
at: crates/viola-agent-claude/src/ledger.rs:1265
quote: out = replace_word(&out, user, "<user>");
at: src/cmd/verify.rs:585
quote: let scrubbed = ledger::scrub(&payload, home, user);
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:212
quote: let scrubbed = ledger::scrub(&doc, &home, &user);
@row_add n=287 row=decision/record-only-at-zero-failing-rows area=7-test-data-fixtures rec=fct-eu46zdvx@1
name: fixtures recorded only when nothing fails
rule: The viola verify recorder writes fixtures only at 0 failing rows.
@row_check n=287 row=decision/record-only-at-zero-failing-rows sha=789c3bc36711 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:297-301
quote: if failed > 0 {         return Ok(Ended::Exit(1));     }     match &args.record {         Some(dir) => record(dir, &version, &probes),
@row_add n=288 row=decision/screen-fixture-scrub-as-detector area=7-test-data-fixtures rec=fct-eu46zdvx@1
name: screen fixtures checked scrub-as-detector
rule: A screen fixture keeps only the rows holding a compiled signature literal, and a kept row the scrub would rewrite refuses the recording whole, never written rewritten.
@row_check n=288 row=decision/screen-fixture-scrub-as-detector sha=a3b76feae0ab v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:599
quote: let kept = ledger::signature_rows(rows);
at: src/cmd/verify.rs:601
quote: if let Some(fault) = ledger::screen_fault(rows, &kept, home, user) {
at: src/cmd/verify.rs:522-526
quote: match signature_screens(&probes.typed, &home, &user) {         Ok(screens) => files.extend(screens),         Err(named) => return Ok(refuse_recording(&named)),     }     write_recording(&dir.join(version), files)?;
@row_add n=289 row=decision/mcp-tap-refuses-dirty-exchange area=7-test-data-fixtures rec=fct-eu46zdvx@1
name: MCP tap refuses a dirty exchange whole
rule: The test-only viola-mcp-tap fixture refuses a dirty exchange whole with the same four codes as the verify recorder, naming the file only.
@row_check n=289 row=decision/mcp-tap-refuses-dirty-exchange sha=f555d907e371 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:213-215
quote: if let Some(code) = fault(&scrubbed, &home, &user) {         eprintln!("viola-mcp-tap: {name} {code}");         return ExitCode::from(1);
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:299
quote: ("home", "/srv/h/plantuser/x", Some("home-path")),
@row_add n=290 row=contract/verify-recording-refusal/unable-message area=7-test-data-fixtures rec=fct-eu46zdvx@1
boundary: viola verify fixture recording refusal
item: unable message
promise: A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.
@row_check n=290 row=contract/verify-recording-refusal/unable-message sha=8dd38f431c63 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:626
quote: &format!("a recorded fixture is not clean: {named}"),
at: src/cmd/verify.rs:603
quote: return Err(format!("{name} row {}{at} {}", fault.row, fault.why.code()));
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: crates/viola-agent-claude/src/ledger.rs:1306-1309
quote: Self::HomePath => "home-path", Self::AbsolutePath => "absolute-path", Self::Username => "username", Self::Email => "email",
@row_add n=291 row=decision/tests-seed-own-data area=7-test-data-fixtures rec=fct-r66s5egb@1
name: self-bootstrapping tests
rule: Tests seed their own data; no developer pre-populates a home.
@row_check n=291 row=decision/tests-seed-own-data sha=556bad24c4cb v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:198-201
quote: let dir = tempfile::Builder::new()             .prefix("viola-test-")             .tempdir_in(base)             .expect("tempdir");
@row_add n=292 row=decision/per-test-home area=7-test-data-fixtures rec=fct-io5pl4i6@1
name: isolated home per test
rule: Each test has its own home, so no endpoint, log or budget.json is shared between tests.
@row_check n=292 row=decision/per-test-home sha=452ccb08b43c v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:198-203
quote: let dir = tempfile::Builder::new()             .prefix("viola-test-")             .tempdir_in(base)             .expect("tempdir");         write_owner(dir.path());         let home = dir.path().join("home");
@row_add n=293 row=decision/env-per-child area=7-test-data-fixtures rec=fct-io5pl4i6@1
name: environment set per child process
rule: Test environment is set per child process with Command::env, never with std::env::set_var.
why: std::env::set_var is unsafe in edition 2024.
@row_check n=293 row=decision/env-per-child sha=81caf1cc9801 v=holds judge=sonnet head=8834195b0683
searched: set_var · in **/*.rs (only doc comments "never `set_var`" at tests/support/home.rs:867 and crates/viola-e2e/src/harness/boot.rs:496; no call)
@row_add n=294 row=decision/ci-homes-kept-until-gates area=7-test-data-fixtures rec=fct-lrldphvg@1
name: CI homes kept until gate steps
rule: In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run.
except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
@row_check n=294 row=decision/ci-homes-kept-until-gates sha=fa183bc10836 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:20
quote: AGENT_RUN_KEEP_HOMES: "1"
at: tests/support/home.rs:193
quote: let base = workspace_path("target/e2e-home");
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: tests/support/home.rs:215-218
quote: let dir = tempfile::Builder::new()             .prefix("viola-chaos-")             .tempdir()             .expect("tempdir");
@row_add n=295 row=decision/no-http-mocking area=8-mocking-stubbing-discipline rec=fct-yqmk3k5s@1,fct-qta73xkp@1
name: no HTTP mock library
rule: No HTTP mock library is used.
why: viola makes no outbound HTTP calls.
@row_check n=295 row=decision/no-http-mocking sha=dd84cb2050a2 v=holds judge=sonnet head=8834195b0683
searched: wiremock|httpmock|mockito|reqwest|hyper|ureq|isahc · in Cargo.toml, crates/*/Cargo.toml, deny.toml
@row_add n=296 row=decision/fakes-for-external-services area=8-mocking-stubbing-discipline rec=fct-azuqhcwj@1
name: fakes replace external services
rule: The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and Anthropic usage values by generated statusline stdin; tests never reach the real services.
@row_check n=296 row=decision/fakes-for-external-services sha=e44abafbd6bb v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:156-159
quote: fs::copy( exe(&opts.bin_dir, "viola-fake-agent"), exe(&session_bin, "claude"), )?;
at: tests/support/fake.rs:13
quote: pub const FAKE: &str = env!("CARGO_BIN_EXE_viola-fake-agent");
@row_add n=297 row=decision/filesystem-not-mocked area=8-mocking-stubbing-discipline rec=fct-hkmaq6ye@2,fct-rddvvdrx@1
name: real filesystem in tests
rule: The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere.
why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
@row_check n=297 row=decision/filesystem-not-mocked sha=d7ab8445e083 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:214
quote: mockall = "=0.15.0"
at: crates/viola-pty/src/lib.rs:107
quote: #[cfg_attr(test, mockall::automock)]
@row_add n=298 row=decision/pure-functions-tested-directly area=8-mocking-stubbing-discipline rec=fct-6xqpyyhr@1
name: pure functions tested directly
rule: Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.
@row_check n=298 row=decision/pure-functions-tested-directly sha=00aa67a01f88 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:356
quote: pub fn validate_paste_text(text: &str) -> Result<(), NotDelivered> {
searched: automock · in **/*.rs (only the viola-pty Pty trait; no pure function is mocked)
@row_add n=299 row=decision/no-mocks-in-e2e area=8-mocking-stubbing-discipline rec=fct-dlsufsua@1,fct-kjqvkfwd@1
name: nothing mocked in E2E
rule: E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
@row_check n=299 row=decision/no-mocks-in-e2e sha=2bc38a944766 v=holds judge=sonnet head=8834195b0683
searched: mockall|automock · in crates/viola-e2e, tests/ (none; the only automock is crates/viola-pty/src/lib.rs:107)
searched: wiremock|httpmock|mockito · in Cargo.toml, crates/*/Cargo.toml (none)
@row_add n=300 row=decision/no-http-fake-servers area=8-mocking-stubbing-discipline rec=fct-qull53ri@1
name: no HTTP fake servers
rule: Fake servers are the interprocess server API binding the squatter before run and viola-fake-agent as the fake CLI; no HTTP fake server is used.
@row_check n=300 row=decision/no-http-fake-servers sha=a9b4c179076d v=holds judge=sonnet head=8834195b0683
searched: TcpListener|http://|https://|axum|warp|tiny_http · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs (one test string in crates/viola-e2e/src/harness/secret_scan.rs:273, no server)
@row_add n=301 row=decision/github-actions-ci area=9-ci-integration rec=fct-3x7gvt6f@2
name: GitHub Actions for CI
rule: CI runs on GitHub Actions: one workflow on push and pull request, and a scheduled nightly workflow for fuzz and weekly advisories.
@row_check n=301 row=decision/github-actions-ci sha=5f1cd768f57b v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:3-5
quote: on:   push:   pull_request:
at: .github/workflows/nightly.yml:4-5
quote: schedule:     - cron: "17 4 * * 1"
at: .github/workflows/nightly.yml:48
quote: - name: Time-boxed fuzz (120 s per target)
@row_add n=302 row=absence/deploy-stage area=9-ci-integration rec=fct-3x7gvt6f@2
capability: a deploy stage
@row_check n=302 row=absence/deploy-stage sha=f38e988318a5 v=holds judge=sonnet head=8834195b0683
searched: (?i)deploy · in .github/**
searched: publish · in .github/**
@row_add n=303 row=decision/rust-cache-only-cache area=9-ci-integration rec=fct-3x7gvt6f@2
name: rust-cache as the only CI cache
rule: rust-cache is the only cache, used in the push and pull-request workflow only.
@row_check n=303 row=decision/rust-cache-only-cache sha=eb500997ea37 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:29
quote: - uses: Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2.9.2
searched: actions/cache · in .github/**
searched: rust-cache · in .github/workflows/nightly.yml
@row_add n=304 row=decision/ci-lint-gate area=9-ci-integration rec=fct-346brcpe@2
name: CI lint gate
rule: CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion.
except: The acyclic module review is not a gate.
@row_check n=304 row=decision/ci-lint-gate sha=393eb6a2ad59 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:275
quote: cargo check $(sed 's/^/-p /' scripts/sync-crates.txt)
at: .github/workflows/ci.yml:278
quote: run: cargo fmt --all --check
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
at: .github/workflows/ci.yml:314
quote: run: bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh
at: .github/workflows/ci.yml:308
quote: run: bash scripts/rmcp-features.sh
at: scripts/orphans-check.sh:6
quote: # `dependencies --acyclic` is deliberately absent: at cargo-modules 0.27.0 every type with an inherent method
@row_add n=305 row=decision/supply-chain-fail-closed area=9-ci-integration rec=fct-vuxpgop2@1
name: fail-closed supply-chain job
rule: One fail-closed job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the deny probes and zizmor, uploading their JSON, and a weekly nightly job re-runs the advisories.
@row_check n=305 row=decision/supply-chain-fail-closed sha=3415b2fbce9d v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
at: .github/workflows/ci.yml:382
quote: cargo deny --config deny-sync.toml --manifest-path "crates/$crate/Cargo.toml" check bans
at: .github/workflows/ci.yml:387
quote: run: bash scripts/deny-probes.sh
at: .github/workflows/ci.yml:392
quote: zizmor --format=json .github/workflows/ > target/supply-chain/zizmor.json || { cat target/supply-chain/zizmor.json; exit 1; }
at: .github/workflows/ci.yml:399
quote: path: target/supply-chain/
at: .github/workflows/nightly.yml:29-30
quote: cargo deny check advisories cargo deny --manifest-path fuzz/Cargo.toml check advisories
@row_add n=306 row=decision/single-instrumented-test-run area=9-ci-integration rec=fct-onifqrhg@2,fct-wxdsyw3l@1
name: one instrumented coverage run in CI
rule: CI's test job is one instrumented nextest run that replaces separate unit, integration and E2E runs; the per-suite agent-run runs are its local equivalents.
@row_check n=306 row=decision/single-instrumented-test-run sha=21c28e14074c v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:35-38
quote: # One instrumented nextest run replaces the unit and integration runs; doctest follows it. - name: Coverage and doctest (sh shim) shell: bash run: bash scripts/agent-run.sh run --coverage
at: scripts/agent-run.sh:13
quote: #   run      - invoke the suites (--unit|--integration|--e2e|--mutants|--coverage|--perf|
@row_add n=307 row=decision/jobs-end-in-harness-gate area=9-ci-integration rec=fct-onifqrhg@2
name: CI jobs end in the harness gate
rule: Each CI test job ends in viola-harness gate requiring its suites.
@row_check n=307 row=decision/jobs-end-in-harness-gate sha=37f64fb36631 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
at: .github/workflows/ci.yml:189
quote: run: bash scripts/agent-run.sh gate --require perf
at: .github/workflows/ci.yml:229
quote: run: bash scripts/agent-run.sh gate --require nextest-unit
at: .github/workflows/ci.yml:253
quote: run: bash scripts/agent-run.sh gate --require fuzz-replay
@row_add n=308 row=decision/mutation-not-a-ci-gate area=9-ci-integration rec=fct-fo2372wn@2
name: mutation testing outside CI
rule: Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
@row_check n=308 row=decision/mutation-not-a-ci-gate sha=1878070f3d22 v=holds judge=sonnet head=8834195b0683
searched: agent-run.sh run --mutants · in .github/workflows/*.yml
searched: mutants · in crates/viola-e2e/src/harness/pre_push.rs (no mutation stage)
@row_add n=309 row=decision/perf-verdict-from-harness area=9-ci-integration rec=fct-3q2uf724@2
name: perf verdict from the harness gate
rule: Perf runs in its own CI job, and its verdict comes from viola-harness gate, never from a jq binary.
@row_check n=309 row=decision/perf-verdict-from-harness sha=65e84f0e0720 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:122
quote: perf:
at: .github/workflows/ci.yml:189
quote: run: bash scripts/agent-run.sh gate --require perf
@row_add n=310 row=decision/msrv-check area=9-ci-integration rec=fct-bpswkwdk@1,fct-kumfcynn@2
name: MSRV check on the security toolchain floor
rule: An ubuntu CI job checks the workspace and runs the unit suite on Rust 1.96, the minimum supported Rust version.
why: Rust 1.96 is the security toolchain floor.
@row_check n=310 row=decision/msrv-check sha=28540758dd5f v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:220
quote: run: cargo check --workspace
at: .github/workflows/ci.yml:225
quote: run: bash scripts/agent-run.sh run --unit
at: .github/workflows/ci.yml:214
quote: RUSTUP_TOOLCHAIN: "1.96"
@row_add n=311 row=decision/fuzz-corpus-replay-in-ci area=9-ci-integration rec=fct-mq72t7wc@1
name: Fuzz corpus replay in CI
rule: Every push and pull request replays the fuzz corpus, and a nightly run fuzzes each target for a bounded time.
@row_check n=311 row=decision/fuzz-corpus-replay-in-ci sha=c368d2353418 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:249
quote: run: bash scripts/agent-run.sh run --fuzz-replay
at: .github/workflows/nightly.yml:55
quote: cargo "+$channel" fuzz run --fuzz-dir fuzz "$t" "fuzz/corpus/$t" -- -max_total_time=120
@row_add n=312 row=decision/quality-gate-command area=9-ci-integration rec=fct-bzvycnyj@1,fct-phqnbmfn@1
name: One quality gate over the test outputs
rule: The viola-harness gate reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach, failing the build.
@row_check n=312 row=decision/quality-gate-command sha=0a5050d92430 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:47
quote: let summary = read_json::<Value>(&artifacts.join("run-summary.json")).ok();
at: crates/viola-e2e/src/harness/gate.rs:52
quote: if JUNIT.contains(&suite) && !artifacts.join(format!("junit-{suite}.xml")).is_file() {
at: crates/viola-e2e/src/harness/gate.rs:102
quote: let Ok(summary) = read_json::<Value>(&artifacts.join("llvm-cov-summary.json")) else {
at: crates/viola-e2e/src/harness/gate.rs:128-130
quote: /// Every row `run --perf` times must be present (test-plan §3 `gate`); every `perf-*.json` present /// is judged on hyperfine's `max` sample, the gated statistic (test-plan §10; obs-plan §10). fn perf(artifacts: &Path, breaches: &mut Vec<Value>) {
at: crates/viola-e2e/src/harness/gate.rs:66-70
quote: let ok = breaches.is_empty();     Outcome::new(         json!({"v": 1, "cmd": "gate", "ok": ok, "breaches": breaches}),         ok,     )
@row_add n=313 row=decision/workflow-least-permissions area=9-ci-integration rec=fct-r5xptmoc@1
name: Least workflow permissions
rule: Every CI workflow grants no permissions at the top and only read access to contents per job.
@row_check n=313 row=decision/workflow-least-permissions sha=afa5e134fce9 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:7
quote: permissions: {}
at: .github/workflows/ci.yml:13-14
quote: permissions:       contents: read
at: .github/workflows/nightly.yml:8
quote: permissions: {}
at: .github/workflows/nightly.yml:14-15
quote: permissions:       contents: read
@row_add n=314 row=decision/single-tool-version-source area=9-ci-integration rec=fct-r5xptmoc@1
name: One version source for each tool
rule: Each tool has exactly one version source, the CI workflow's pins, and the local pre-push gate checks the host's tools against them and installs nothing.
@row_check n=314 row=decision/single-tool-version-source sha=3d7b3443bf47 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:115
quote: let pins = ci_pins(&yml).ok_or_else(unreadable)?;
at: crates/viola-e2e/src/harness/pre_push/linux.rs:102-103
quote: /// The C linker, then every tool at its pin: `rust-toolchain.toml`'s channel and ci.yml's test-job /// tool line, never a version written here.
@row_add n=315 row=decision/no-ci-concurrency-groups area=9-ci-integration rec=fct-7mqjkvmt@1
name: No concurrency groups in CI
rule: No CI workflow has a concurrency block, and zizmor's concurrency-limits finding is declined and left visible.
why: A concurrency group cancels pending runs, which would drop a push's always-run gate and upload chain.
@row_check n=315 row=decision/no-ci-concurrency-groups sha=1e47acb1eb40 v=holds judge=sonnet head=8834195b0683
searched: concurrency · in .github/**
@row_add n=316 row=decision/linux-only-ci-matrix area=9-ci-integration rec=fct-shyrfs6v@2,fct-q4ch4yge@1
name: Linux-only CI matrix
rule: CI runs on ubuntu-latest as the one leg of the OS matrix in the test, perf, lint and release jobs, each job and artifact name keeping its matrix.os form, with fail-fast off.
@row_check n=316 row=decision/linux-only-ci-matrix sha=0c330e7a1ba3 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: .github/workflows/ci.yml:16
quote: fail-fast: false
at: .github/workflows/ci.yml:318
quote: name: release (${{ matrix.os }})
at: .github/workflows/ci.yml:256
quote: name: lint (${{ matrix.os }})
@row_add n=317 row=decision/pinned-rust-toolchain area=9-ci-integration rec=fct-kumfcynn@2
name: Exactly pinned Rust toolchain
rule: The Rust toolchain is pinned to one exact stable version at or above the 1.96 floor and installed by rustup on both the CI runner and the host.
@row_check n=317 row=decision/pinned-rust-toolchain sha=f51dd03b4eed v=holds judge=sonnet head=8834195b0683
at: rust-toolchain.toml:2
quote: channel = "1.98.1"
at: .github/workflows/ci.yml:26
quote: run: rustup toolchain install
@row_add n=318 row=decision/machine-parseable-ci-reports area=9-ci-integration rec=fct-yruy4jpq@1
name: Machine-parseable CI test reports
rule: CI test reports are machine-parseable and leave CI as uploaded artifacts.
@row_check n=318 row=decision/machine-parseable-ci-reports sha=a665827a5bb1 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:102
quote: path: target/nextest/ci/junit.xml
at: .github/workflows/ci.yml:96
quote: if-no-files-found: warn
@row_add n=319 row=decision/artifacts-behind-secret-scan area=9-ci-integration rec=fct-mos7j36w@2,fct-ap7wweiw@1
name: Artifacts leave CI behind a secret scan
rule: Every test or perf artifact leaves CI only after its job's secret scan passed; when the scan fails only the scan's hit report is uploaded.
@row_check n=319 row=decision/artifacts-behind-secret-scan sha=9b157fd73f66 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:106
quote: if: always() && steps.secret-scan.outcome == 'failure'
at: .github/workflows/ci.yml:162
quote: if: always() && steps.secret-scan.outcome == 'success'
@row_add n=320 row=contract/ci-artifacts/harness area=9-ci-integration rec=fct-mos7j36w@2
boundary: CI artifacts
item: harness-${{ matrix.os }}
promise: Uploaded by the test job on failure after a passed secret scan: the agent-run output minus the mutation diff, with per-suite JUnit, run summary, coverage summary and the failure-only logs and status capture; no perf export.
@row_check n=320 row=contract/ci-artifacts/harness sha=f52b935480b6 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:87
quote: if: failure() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:92-94
quote: path: | target/agent-run/ !target/agent-run/chunk.diff
@row_add n=321 row=contract/ci-artifacts/diag area=9-ci-integration rec=fct-mos7j36w@2
boundary: CI artifacts
item: diag-${{ matrix.os }}
promise: Holds the E2E homes' diagnostics NDJSON files from the test job, uploaded only behind a passed secret scan.
@row_check n=321 row=contract/ci-artifacts/diag sha=383e0cc909e0 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:83
quote: path: target/e2e-home/**/diagnostics/*.ndjson
@row_add n=322 row=contract/ci-artifacts/junit area=9-ci-integration rec=fct-mos7j36w@2
boundary: CI artifacts
item: junit-${{ matrix.os }}
promise: Holds the one JUnit report of the test job's single instrumented coverage nextest invocation.
@row_check n=322 row=contract/ci-artifacts/junit sha=c7f44312a474 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:101-102
quote: name: junit-${{ matrix.os }} path: target/nextest/ci/junit.xml
@row_add n=323 row=contract/ci-artifacts/secret-scan area=9-ci-integration rec=fct-mos7j36w@2
boundary: CI artifacts
item: secret-scan-${{ matrix.os }}
promise: Holds the test job's secret-scan hit report, uploaded only when the scan failed.
@row_check n=323 row=contract/ci-artifacts/secret-scan sha=ecf8a7e9dce6 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:106
quote: if: always() && steps.secret-scan.outcome == 'failure'
at: .github/workflows/ci.yml:109-110
quote: name: secret-scan-${{ matrix.os }} path: target/secret-scan/
@row_add n=324 row=decision/ci-artifact-retention area=9-ci-integration rec=fct-mos7j36w@2
name: Seven-day artifact retention
rule: Every CI test artifact is kept for 7 days.
@row_check n=324 row=decision/ci-artifact-retention sha=7c27ad3af2eb v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:84
quote: retention-days: 7
at: .github/workflows/ci.yml:95
quote: retention-days: 7
at: .github/workflows/ci.yml:103
quote: retention-days: 7
at: .github/workflows/ci.yml:111
quote: retention-days: 7
@row_add n=325 row=decision/per-suite-junit-copies area=9-ci-integration rec=fct-mos7j36w@2
name: Per-suite JUnit copies kept
rule: Each suite's JUnit report is kept as its own copy among the run artifacts.
why: Each nextest invocation overwrites the raw JUnit file.
@row_check n=325 row=decision/per-suite-junit-copies sha=f14db3183d2f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:173
quote: format!("junit-{}.xml", s.suite),
@row_add n=326 row=decision/mutation-output-never-uploaded area=9-ci-integration rec=fct-2ymlhohm@2
name: Mutation output never uploaded
rule: The mutation-testing output directory is never uploaded from CI.
why: Its outcomes file holds absolute argv paths and its log holds test output.
@row_check n=326 row=decision/mutation-output-never-uploaded sha=fdb8bed7da8a v=holds judge=sonnet head=8834195b0683
searched: mutants.out · in .github/
searched: mutants · in .github/workflows/ci.yml upload-artifact paths (none names the mutation output)
@row_add n=327 row=contract/ci-artifacts/perf area=9-ci-integration rec=fct-ap7wweiw@1
boundary: CI artifacts
item: perf-${{ matrix.os }}
promise: Holds the perf job's hyperfine exports, uploaded only when the perf job's secret scan succeeded.
@row_check n=327 row=contract/ci-artifacts/perf sha=b9ca9e36153f v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:162
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:165-166
quote: name: perf-${{ matrix.os }} path: target/agent-run/artifacts/perf-*.json
@row_add n=328 row=contract/ci-artifacts/diag-perf area=9-ci-integration rec=fct-ap7wweiw@1
boundary: CI artifacts
item: diag-perf-${{ matrix.os }}
promise: Holds the perf run homes' diagnostics NDJSON files, uploaded only when the perf job's secret scan succeeded.
@row_check n=328 row=contract/ci-artifacts/diag-perf sha=d85ba1172c9d v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:170
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:173-174
quote: name: diag-perf-${{ matrix.os }} path: target/e2e-home/**/diagnostics/*.ndjson
@row_add n=329 row=contract/ci-artifacts/secret-scan-perf area=9-ci-integration rec=fct-ap7wweiw@1
boundary: CI artifacts
item: secret-scan-perf-${{ matrix.os }}
promise: Holds the perf job's secret-scan hit report, uploaded only when that scan failed.
@row_check n=329 row=contract/ci-artifacts/secret-scan-perf sha=20d66c5dd287 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:178
quote: if: always() && steps.secret-scan.outcome == 'failure'
at: .github/workflows/ci.yml:181-182
quote: name: secret-scan-perf-${{ matrix.os }} path: target/secret-scan/
@row_add n=330 row=decision/agent-reads-ci-through-gh area=9-ci-integration rec=fct-kprehror@1
name: Agent reads CI through gh
rule: The agent reads CI job conclusions and fetches artifacts with the gh CLI, and locally gets the same files from agent-run run and agent-run logs.
@row_check n=330 row=decision/agent-reads-ci-through-gh sha=47e2f3f302c5 v=unverifiable judge=sonnet head=8834195b0683
searched: gh run (view|download) · in tracked source outside .claude .andromeda viola-0.1.0 viola-0.2.0
why: not a code claim
@row_add n=331 row=decision/build-fails-on-test-failure area=9-ci-integration rec=fct-q4zg6vp5@2,fct-u7ux54l5@1
name: Build fails on any test failure
rule: The build fails when any unit, integration, E2E, contract or property test fails.
@row_check n=331 row=decision/build-fails-on-test-failure sha=89e95aa64a46 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:89
quote: for (gate, key) in [("suite-failed", "failed"), ("suite-skipped", "skipped")] {
@row_add n=332 row=decision/build-fails-below-coverage area=9-ci-integration rec=fct-6iojik4n@2,fct-u7ux54l5@1
name: Build fails below coverage threshold
rule: The build fails when coverage falls below the quality-gate threshold.
@row_check n=332 row=decision/build-fails-below-coverage sha=0e1d9bee6ff0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:114-117
quote: Some(pct) => breaches.push(breach( "coverage", "coverage", format!("{metric} {pct:.2} < {floor}"),
@row_add n=333 row=decision/build-fails-on-perf-breach area=9-ci-integration rec=fct-y7sxtbos@1,fct-u7ux54l5@1
name: Build fails on a performance budget breach
rule: The build fails on any hyperfine performance budget breach.
@row_check n=333 row=decision/build-fails-on-perf-breach sha=7eba7562e0e9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:152-155
quote: Some(max) => breaches.push(breach( "perf", "perf", format!("{file} max {max} >= {bound}"),
@row_add n=334 row=decision/build-fails-on-fuzz-crash area=9-ci-integration rec=fct-y7sxtbos@1,fct-u7ux54l5@1
name: Build fails on a fuzz replay crash
rule: The build fails when the fuzz corpus replay crashes.
@row_check n=334 row=decision/build-fails-on-fuzz-crash sha=142b1181d510 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:253
quote: run: bash scripts/agent-run.sh gate --require fuzz-replay
at: crates/viola-e2e/src/harness/gate.rs:89
quote: for (gate, key) in [("suite-failed", "failed"), ("suite-skipped", "skipped")] {
@row_add n=335 row=decision/fmt-and-clippy-deny-warnings area=9-ci-integration rec=fct-7h3hdkfi@1,fct-u7ux54l5@1
name: Formatting and clippy warnings fail the build
rule: Any rustfmt difference or clippy warning, with warnings denied, fails the build.
@row_check n=335 row=decision/fmt-and-clippy-deny-warnings sha=749280538264 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:278
quote: run: cargo fmt --all --check
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
@row_add n=336 row=decision/rmcp-feature-limit area=9-ci-integration rec=fct-7h3hdkfi@1,fct-u7ux54l5@1
name: rmcp limited to server and transport-io
rule: The build fails when rmcp is absent from viola's release graph or any rmcp feature beyond server and transport-io is requested.
guard: rmcp-features
@row_check n=336 row=decision/rmcp-feature-limit sha=370ee8f9a3df v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/rmcp-features.sh:18
quote: echo "rmcp-features: FAILED — rmcp is absent from viola's release graph"
at: scripts/rmcp-features.sh:34
quote: echo "rmcp-features: FAILED — rmcp features requested: ${requested:-none}; expected: server transport-io"
guard_at: .github/workflows/ci.yml:308
@row_add n=337 row=decision/cargo-deny-gate area=9-ci-integration rec=fct-ys3szg34@1,fct-u7ux54l5@1
name: cargo-deny supply-chain gate
rule: Any cargo deny ban, advisory, licence or source finding, including an advisory or source finding on the fuzz lockfile, fails the build.
guard: cargo deny; deny-probes, one probe per ban
@row_check n=337 row=decision/cargo-deny-gate sha=b07307fc5524 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
guard_at: .github/workflows/ci.yml:387
@row_add n=338 row=decision/channel-tokio-free-by-default area=9-ci-integration rec=fct-ys3szg34@1,fct-u7ux54l5@1
name: viola-channel reaches tokio only by feature
rule: The build fails when viola-channel reaches tokio without its feature enabled.
guard: channel-tokio-free
@row_check n=338 row=decision/channel-tokio-free-by-default sha=1c89407e7c8e v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:384
quote: bash scripts/channel-tokio-free.sh
at: scripts/channel-tokio-free.sh:15
quote: out=$(cargo tree -p viola-channel -e normal --target all -i tokio --prefix none "$@" 2>&1) || rc=$?
guard_at: scripts/channel-tokio-free.sh:1
@row_add n=339 row=decision/zizmor-gate area=9-ci-integration rec=fct-ys3szg34@1,fct-u7ux54l5@1
name: zizmor findings fail the build
rule: Any zizmor finding on the CI workflows fails the build.
@row_check n=339 row=decision/zizmor-gate sha=6813310589ab v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:392
quote: zizmor --format=json .github/workflows/ > target/supply-chain/zizmor.json || { cat target/supply-chain/zizmor.json; exit 1; }
@row_add n=340 row=decision/weekly-advisories-run area=9-ci-integration rec=fct-ys3szg34@1,fct-u7ux54l5@1
name: Weekly advisories run
rule: A weekly nightly workflow checks advisories on the root and fuzz lockfiles, and its failure is a build failure.
@row_check n=340 row=decision/weekly-advisories-run sha=6f2988de6dee v=holds judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:5
quote: - cron: "17 4 * * 1"
at: .github/workflows/nightly.yml:29
quote: cargo deny check advisories
at: .github/workflows/nightly.yml:30
quote: cargo deny --manifest-path fuzz/Cargo.toml check advisories
@row_add n=341 row=decision/comprehensive-coverage-floors area=10-quality-gates-coverage-targets rec=fct-nwua7cus@1,fct-4ixdwl5o@1,fct-ryyndnld@2
name: Comprehensive-tier coverage floors
rule: viola's coverage must be at least 85% of lines, 80% of branches (enforced as region coverage) and 95% of functions, enforced in the ubuntu test job.
guard: cargo llvm-cov fail-under-lines 85, fail-under-functions 95, fail-under-regions 80
@row_check n=341 row=decision/comprehensive-coverage-floors sha=6ceb9534638d v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:15-16
quote: pub const COVERAGE_FLOORS: [(&str, f64); 3] = [("lines", 85.0), ("functions", 95.0), ("regions", 80.0)];
at: .github/workflows/ci.yml:38
quote: run: bash scripts/agent-run.sh run --coverage
guard_at: crates/viola-e2e/src/harness/run/coverage.rs:165
@row_add n=342 row=decision/coverage-workspace-rollup area=10-quality-gates-coverage-targets rec=fct-2xpzggiy@1
name: Coverage rolled up across the workspace
rule: The coverage number rolls up across all workspace crates.
@row_check n=342 row=decision/coverage-workspace-rollup sha=87fb8ac177e9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:27
quote: "--workspace",
@row_add n=343 row=decision/coverage-exclusions area=10-quality-gates-coverage-targets rec=fct-3mj52n2x@2
name: Coverage exclusions
rule: The fake agent, the E2E crate, test support code and fuzz targets are excluded from coverage by one separator-agnostic filename pattern held in the harness.
why: The separator class is kept from the retired Windows target, where llvm-cov reported paths with backslashes.
@row_check n=343 row=decision/coverage-exclusions sha=e2a7b6b54ac0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:11-12
quote: pub const COVERAGE_IGNORE: &str = r"(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])";
@row_add n=344 row=decision/child-process-coverage area=10-quality-gates-coverage-targets rec=fct-3zrfkvpu@1
name: Child viola processes counted in coverage
rule: viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE.
why: LLVM_PROFILE_FILE propagates to child processes.
except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
@row_check n=344 row=decision/child-process-coverage sha=6a2c1f4e6f65 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:518-520
quote: const COVERAGE_ENV: [&str; 4] = [ "CARGO_LLVM_COV", "LLVM_PROFILE_FILE",
at: crates/viola-e2e/src/harness/run.rs:527-528
quote: for name in COVERAGE_ENV { cmd.env_remove(name);
@row_add n=345 row=decision/doctests-outside-coverage area=10-quality-gates-coverage-targets rec=fct-hxwzxyew@1
name: Doctests outside the coverage number
rule: Doctests are run but are not part of the coverage number.
@row_check n=345 row=decision/doctests-outside-coverage sha=43004a70b1e0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/doctest.rs:12-14
quote: "test", "--workspace", "--doc",
at: crates/viola-e2e/src/harness/run/coverage.rs:26
quote: "nextest",
@row_add n=346 row=decision/mutation-gate-cadence area=10-quality-gates-coverage-targets rec=fct-xnmc6vwq@1
name: Mutation gate at the epoch boundary only
rule: Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
@row_check n=346 row=decision/mutation-gate-cadence sha=bc88e5be1704 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:328
quote: assert!(!fake.calls.iter().any(|c| c.contains("--mutants")));
searched: mutants gate step · in .github/workflows/ci.yml (only tool installs and comments, lines 32, 33, 91, 209)
@row_add n=347 row=decision/perf-gate-on-max area=10-quality-gates-coverage-targets rec=fct-3c7vh3a2@1
name: Performance gate on the max sample
rule: The performance gate judges the max sample; mean, median and stddev are reported for trend only, and no percentile is computed or gated.
why: Claude Code hook deadlines are hard cutoffs.
@row_check n=347 row=decision/perf-gate-on-max sha=08656e25af54 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:149
quote: .and_then(|v| v["results"][0]["max"].as_f64());
searched: percentile|p95|p99 · in crates/viola-e2e/src/harness/gate.rs and crates/viola-e2e/src/harness/run/perf.rs
@row_add n=348 row=decision/perf-own-release-build area=10-quality-gates-coverage-targets rec=fct-db2dcqzo@1,fct-nzahfwbg@1
name: Perf times its own release build
rule: The perf run times its own non-instrumented release workspace build with the fake agent, never a debug or coverage build, and never with LLVM_PROFILE_FILE set.
@row_check n=348 row=decision/perf-own-release-build sha=9baf9dd851d8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:159-160
quote: .args(["build", "--release", "--workspace", "--features"]) .args(["viola/fake-agent", "--target-dir", TARGET_DIR])
@row_add n=349 row=decision/perf-real-hook-path area=10-quality-gates-coverage-targets rec=fct-sdxsh5tc@1
name: Perf times hooks through the real path
rule: The perf run boots one unstamped session with a builder instance and times each hook through the real logging and channel path on synthetic payloads.
@row_check n=349 row=decision/perf-real-hook-path sha=31d1f7d5886e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:180
quote: stamp: false,
at: crates/viola-e2e/src/harness/run/perf.rs:119
quote: .arg(format!("{} hook {hook}", viola_exe()))
@row_add n=350 row=decision/perf-panic-check area=10-quality-gates-coverage-targets rec=fct-sdxsh5tc@1
name: Perf run checks for hook panics
rule: Before cleanup the perf run asserts no panic line in the home's role files.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_check n=350 row=decision/perf-panic-check sha=bf5eb973b5a4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:192
quote: check(&mut suite, role_panics(&home) == 0, "panic");
@row_add n=351 row=decision/perf-failure-turns-red area=10-quality-gates-coverage-targets rec=fct-sdxsh5tc@1
name: Perf failures turn the suite red
rule: A failed build, boot, timed row, panic check or cleanup turns suite perf red, never a missing or skipped row.
@row_check n=351 row=decision/perf-failure-turns-red sha=6d5fe3d8908b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:164
quote: fail(&mut suite, "build");
at: crates/viola-e2e/src/harness/run/perf.rs:187
quote: fail(&mut suite, &format!("boot: {reason}"));
at: crates/viola-e2e/src/harness/run/perf.rs:214
quote: check(suite, staged && code == Some(0), hook);
at: crates/viola-e2e/src/harness/run/perf.rs:192
quote: check(&mut suite, role_panics(&home) == 0, "panic");
at: crates/viola-e2e/src/harness/run/perf.rs:194
quote: check(&mut suite, cleaned.doc["ok"] == true, "cleanup");
@row_add n=352 row=decision/perf-hyperfine-invocation area=10-quality-gates-coverage-targets rec=fct-lqftgban@2
name: Hyperfine invocation shape
rule: Every hyperfine call runs without a shell, with 3 warm-up runs and 30 timed runs, exporting JSON to the run artifacts; warm-up runs are not part of max.
why: Warm-up runs absorb first-execution costs.
@row_check n=352 row=decision/perf-hyperfine-invocation sha=198738531ae3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:115
quote: cmd.args(["-N", "--warmup", "3", "--runs", "30", "--input"])
at: crates/viola-e2e/src/harness/run/perf.rs:118
quote: .arg(format!("target/agent-run/artifacts/perf-{hook}.json"))
@row_add n=353 row=decision/perf-no-retry area=10-quality-gates-coverage-targets rec=fct-5j2njdi5@1
name: No retry of a performance breach
rule: The gate evaluates the exported perf JSON once, with no rerun, outlier trimming or second attempt; a breach is fixed in the product or escalated through a Decisions Log entry.
@row_check n=353 row=decision/perf-no-retry sha=d090c23606ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:146-149
quote: for file in files { let max = read_json::<Value>(&artifacts.join(&file)) .ok() .and_then(|v| v["results"][0]["max"].as_f64());
@row_add n=354 row=decision/spine-deadline-single-source area=10-quality-gates-coverage-targets rec=fct-ggkcj6j7@1
name: Spine deadline from the product constant
rule: The perf gate reads the spine deadline from the product's own constant (1.0 s), never from a literal edited into CI or the test plan.
@row_check n=354 row=decision/spine-deadline-single-source sha=45403e3590ca v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:145
quote: let bound = viola_core::SPINE_DEADLINE.as_secs_f64();
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
@row_add n=355 row=decision/hook-connect-deadline area=10-quality-gates-coverage-targets rec=fct-ggkcj6j7@1
name: Hook connect deadline below the spine deadline
rule: The hook's own connect deadline is 750 ms, below the spine deadline the gate enforces.
guard: connect_deadline_is_below_the_spine_deadline
state: provisional
@row_check n=355 row=decision/hook-connect-deadline sha=b7a5050cc918 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:56
quote: const CONNECT_DEADLINE: Duration = Duration::from_millis(750);
guard_at: src/cmd/hook.rs:713
@row_add n=356 row=decision/perf-in-own-ci-job area=10-quality-gates-coverage-targets rec=fct-vxnii5yh@3
name: Perf runs in its own CI job
rule: CI runs perf in its own perf job ending in a gate that requires perf; the test job and the pre-push gate carry no perf step, and no async-tier hook is timed.
@row_check n=356 row=decision/perf-in-own-ci-job sha=c464e9b37429 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:122
quote: perf:
at: .github/workflows/ci.yml:147
quote: run: bash scripts/agent-run.sh run --perf
at: .github/workflows/ci.yml:189
quote: run: bash scripts/agent-run.sh gate --require perf
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
at: crates/viola-e2e/src/harness/pre_push/linux.rs:186
quote: let gate = linux_harness(linux, &["gate", "--require", "coverage,doctest"], runner)?;
at: crates/viola-e2e/src/harness/run/perf.rs:22-26
quote: "session-start", "user-prompt-submit", "stop", "session-end", "pre-tool-use",
@row_add n=357 row=decision/fail-open-spine-bound-at-unit-speed area=10-quality-gates-coverage-targets rec=fct-vxnii5yh@3
name: Fail-open spine bound asserted at unit speed
rule: Each case of the hook fail-open matrix also asserts it completes within the 1.0 s spine bound at unit speed.
guard: hook_fails_open_silently_within_the_spine_bound
@row_check n=357 row=decision/fail-open-spine-bound-at-unit-speed sha=5f61dd8fde45 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:319
quote: assert!(out.took < SPINE_BOUND, "took {:?}", out.took);
guard_at: tests/hook_fail_open.rs:286
@row_add n=358 row=contract/perf-budgets/session-end area=10-quality-gates-coverage-targets rec=fct-yoazurel@2
boundary: Performance budgets
item: viola hook session-end
promise: Timed on a synthetic payload through the live wrapper; mean, median and stddev reported for trend, gated on max under 1.0 s.
@row_check n=358 row=contract/perf-budgets/session-end sha=06017d9e62d7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:25
quote: "session-end",
at: crates/viola-e2e/src/harness/run/perf.rs:115
quote: cmd.args(["-N", "--warmup", "3", "--runs", "30", "--input"])
at: crates/viola-e2e/src/harness/gate.rs:149-151
quote: .and_then(|v| v["results"][0]["max"].as_f64()); match max { Some(max) if max < bound => {}
@row_add n=359 row=contract/perf-budgets/spine-hooks area=10-quality-gates-coverage-targets rec=fct-7bp7lvdw@1
boundary: Performance budgets
item: spine hooks session-start, user-prompt-submit, stop
promise: Timed the same way with VIOLA_NAME and VIOLA_DIR exported to hyperfine; mean, median and stddev reported for trend, gated on max under the spine deadline (1.0 s).
@row_check n=359 row=contract/perf-budgets/spine-hooks sha=dc9442bfe7ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:120-121
quote: .env("VIOLA_NAME", INSTANCE) .env("VIOLA_DIR", dir)
at: crates/viola-e2e/src/harness/gate.rs:145
quote: let bound = viola_core::SPINE_DEADLINE.as_secs_f64();
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
@row_add n=360 row=contract/perf-budgets/pre-tool-use area=10-quality-gates-coverage-targets rec=fct-ermnme7k@1
boundary: Performance budgets
item: viola hook pre-tool-use
promise: Timed on an unverified CLI whose dialog answer is null at once, a required perf row; mean, median and stddev reported for trend, gated on max under the spine deadline (1.0 s).
@row_check n=360 row=contract/perf-budgets/pre-tool-use sha=059e7c04e1ca v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:26
quote: "pre-tool-use",
at: crates/viola-e2e/src/harness/gate.rs:131
quote: for row in PERF_ROWS {
at: crates/viola-e2e/src/harness/gate.rs:151
quote: Some(max) if max < bound => {}
@row_add n=361 row=contract/heartbeat-staleness/age-threshold area=10-quality-gates-coverage-targets rec=fct-4tlnkrfv@1
boundary: Heartbeat staleness classification
item: heartbeat age
promise: For the same process, a heartbeat aged 5.0 s classifies live and one aged 5.1 s classifies stale, held by an exact boundary test.
@row_check n=361 row=contract/heartbeat-staleness/age-threshold sha=4f554c47d3d6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
at: crates/viola-state/src/liveness.rs:169-170
quote: #[case::at_the_bound(Some(5_000), true, Liveness::Live)] #[case::just_past(Some(5_100), true, Liveness::Stale)]
@row_add n=362 row=contract/heartbeat-staleness/identity-mismatch area=10-quality-gates-coverage-targets rec=fct-4tlnkrfv@1
boundary: Heartbeat staleness classification
item: pid or start-time mismatch
promise: A pid or start-time mismatch classifies the instance gone at any age.
@row_check n=362 row=contract/heartbeat-staleness/identity-mismatch sha=3335ee675ef6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:28-29
quote: if !same_process {         return Liveness::Gone;
@row_add n=363 row=decision/send-and-wait-latency-not-timed area=10-quality-gates-coverage-targets rec=fct-7l3hnykk@1,fct-umz6vque@1
name: Send confirmation and wait wake latency not timed
rule: The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed.
why: Both are bounded by product constants that are still open.
@row_check n=363 row=decision/send-and-wait-latency-not-timed sha=c6476d045ffc v=holds judge=sonnet head=8834195b0683
searched: send_window_|prompt-submitted.*(elapsed|Duration)|wake · in crates/viola-e2e/src/harness/run
searched: "session-start", "user-prompt-submit", "stop", "session-end", "pre-tool-use" (the only timed rows) · in crates/viola-e2e/src/harness/run/perf.rs
@row_add n=364 row=decision/boot-deadline-not-a-budget area=10-quality-gates-coverage-targets rec=fct-6txqcbhi@1,fct-umz6vque@1
name: Boot deadline is not a performance budget
rule: The 20 s `boot` deadline is a harness readiness timeout, deliberately not a product performance budget in the quality gates.
@row_check n=364 row=decision/boot-deadline-not-a-budget sha=a5489e82aba0 v=holds judge=sonnet head=8834195b0683
searched: the perf rows list (five hook rows, no boot) · in crates/viola-e2e/src/harness/run/perf.rs:22-26
searched: PERF_ROWS gate judged against viola_core::SPINE_DEADLINE only · in crates/viola-e2e/src/harness/gate.rs
@row_add n=365 row=decision/hyperfine-max-perf-gate area=10-quality-gates-coverage-targets rec=fct-rvltlmnj@2,fct-22efn7xx@1,fct-7vidmhnu@1
name: hyperfine max is the performance gate
rule: Performance budgets, hook deadlines among them, are gated by hyperfine on `max`, never on `mean`, and a budget regression fails the build; criterion 0.8.2 is not a gate.
why: criterion 0.8.2 does not exit non-zero on a regression.
@row_check n=365 row=decision/hyperfine-max-perf-gate sha=affc91c92993 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:149
quote: .and_then(|v| v["results"][0]["max"].as_f64());
at: crates/viola-e2e/src/harness/gate.rs:152-155
quote: Some(max) => breaches.push(breach( "perf", "perf", format!("{file} max {max} >= {bound}"),
searched: criterion · in Cargo.toml, crates/*/Cargo.toml, deny.toml
@row_add n=366 row=decision/hook-deadlines-asserted-at-unit-speed area=10-quality-gates-coverage-targets rec=fct-rvltlmnj@2
name: Hook deadlines asserted at unit speed
rule: The hook deadlines are also asserted at unit speed inside the hook subprocess tests, with a monotonic `std::time::Instant` bound.
@row_check n=366 row=decision/hook-deadlines-asserted-at-unit-speed sha=557160aa0e31 v=holds judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:49
quote: let started = Instant::now();
at: tests/hook_fail_open.rs:319
quote: assert!(out.took < SPINE_BOUND, "took {:?}", out.took);
@row_add n=367 row=decision/zero-flakiness area=10-quality-gates-coverage-targets rec=fct-zrh4qt5s@1,fct-2cc4sa5w@1,fct-go7zgoyw@1,fct-nqu6lr5l@1
name: Zero-flakiness budget
rule: Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is.
why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
@row_check n=367 row=decision/zero-flakiness sha=279bb7351cee v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:2
quote: retries = 0
@row_add n=368 row=decision/no-ignore-parking area=10-quality-gates-coverage-targets rec=fct-zrh4qt5s@1
name: No #[ignore] parking
rule: `#[ignore]` is not allowed as a parking place for a failing or flaky test.
why: Agent-driven development cannot tell a flake from a real bug.
@row_check n=368 row=decision/no-ignore-parking sha=c41a9ec7ab4a v=holds judge=sonnet head=8834195b0683
searched: #\[ignore · in **/*.rs
@row_add n=369 row=decision/test-failure-fails-build area=10-quality-gates-coverage-targets rec=fct-plbmhdnw@2,fct-bphyjtz7@1
name: Any test failure fails the build
rule: A test failure at any level is a build failure condition.
@row_check n=369 row=decision/test-failure-fails-build sha=941710727cf9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:89
quote: for (gate, key) in [("suite-failed", "failed"), ("suite-skipped", "skipped")] {
@row_add n=370 row=decision/coverage-threshold-fails-build area=10-quality-gates-coverage-targets rec=fct-2ztzeoct@2,fct-bphyjtz7@1
name: Coverage below tier threshold fails the build
rule: Coverage below the threshold for its tier, measured in the `test` job on `ubuntu-latest`, is a build failure condition.
@row_check n=370 row=decision/coverage-threshold-fails-build sha=1bdb72b5ec47 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:114
quote: Some(pct) => breaches.push(breach(
at: crates/viola-e2e/src/harness/run/coverage.rs:16
quote: [("lines", 85.0), ("functions", 95.0), ("regions", 80.0)];
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
@row_add n=371 row=decision/mutants-verdict-counted-by-harness area=10-quality-gates-coverage-targets rec=fct-wjz5endv@1,fct-n6iwafhi@1
name: Mutation verdict counted by the harness
rule: At a `run --mutants` the harness counts missed, timeout, unviable and caught outcomes itself, leaving host-excluded missed mutants out; any other missed mutant, any timed-out mutant, or more unviable than caught mutants is red.
why: The cargo-mutants exit code and summary cannot be trusted alone: exit 3 masks exit 2, and the summary counts a host-excluded mutant as missed.
@row_check n=371 row=decision/mutants-verdict-counted-by-harness sha=8e37fb143da1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
at: crates/viola-e2e/src/harness/run/mutants.rs:61-63
quote: let swamped = unviable > caught; if swamped { failures.push("unviable-exceeds-caught".to_owned());
@row_add n=372 row=decision/lint-gates-fail-build area=10-quality-gates-coverage-targets rec=fct-ikzozfqb@1,fct-bphyjtz7@1
name: Lint and audit gates fail the build
rule: A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
@row_check n=372 row=decision/lint-gates-fail-build sha=5195cb9004b2 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: .github/workflows/ci.yml:392
quote: zizmor --format=json .github/workflows/ > target/supply-chain/zizmor.json || { cat target/supply-chain/zizmor.json; exit 1; }
at: .github/workflows/ci.yml:314
quote: run: bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh
@row_add n=373 row=decision/no-test-binary-in-release area=10-quality-gates-coverage-targets rec=fct-ikzozfqb@1,fct-bphyjtz7@1
name: No test-only binary in a release build
rule: A release build carrying a test-only binary is a build failure condition.
@row_check n=373 row=decision/no-test-binary-in-release sha=8a45bc8ecc30 v=holds judge=sonnet head=8834195b0683
at: scripts/release-check.sh:32
quote: echo "release-check: FAILED — test-only binary $name"
at: .github/workflows/ci.yml:338
quote: run: bash scripts/release-check.sh
@row_add n=374 row=decision/e2e-asserts-only-cross-process area=11-test-anti-patterns-never-do-these rec=fct-a34t7dfp@1
name: E2E asserts only cross-process behaviour
rule: A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour.
why: The test pyramid.
@row_check n=374 row=decision/e2e-asserts-only-cross-process sha=62c00ea72eb4 v=unverifiable judge=sonnet head=8834195b0683
searched: lower-layer vs E2E placement is a judgement over every test · in tests/ crates/viola-e2e/tests
why: not a code claim
@row_add n=375 row=decision/no-over-mocking-integration area=11-test-anti-patterns-never-do-these rec=fct-l46kd4t2@1
name: No over-mocking at integration level
rule: Integration tests never over-mock.
why: Over-mocking stops testing the integration.
@row_check n=375 row=decision/no-over-mocking-integration sha=a264945cb56d v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=376 row=decision/no-real-cli-in-ci area=11-test-anti-patterns-never-do-these rec=fct-lbxoplwk@1
name: No real claude CLI in CI
rule: The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too.
guard: `agent-run run --local-live` refuses when `CI` is set
@row_check n=376 row=decision/no-real-cli-in-ci sha=bb177c0adb11 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:131
quote: if sel.local_live && ci {
guard_at: crates/viola-e2e/src/harness/run.rs:131
@row_add n=377 row=decision/fake-agent-not-proof-of-real-cli area=11-test-anti-patterns-never-do-these rec=fct-egysilpo@1
name: Fake-agent run is not proof of real-CLI behaviour
rule: A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite.
why: The fake agent must not drift from the real CLI.
@row_check n=377 row=decision/fake-agent-not-proof-of-real-cli sha=33f2d2c83152 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=378 row=decision/test-public-behaviour-only area=11-test-anti-patterns-never-do-these rec=fct-7rhfof6k@1
name: Test public API and observable behaviour
rule: Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.
@row_check n=378 row=decision/test-public-behaviour-only sha=db96273616bb v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=379 row=decision/assert-on-contract-not-fixture area=11-test-anti-patterns-never-do-these rec=fct-6im6ezik@1
name: Assert on contract, not fixture internals
rule: Assertions target the contract, never brittle fixture-internal values.
@row_check n=379 row=decision/assert-on-contract-not-fixture sha=0559ff7f2162 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=380 row=decision/literal-test-oracles area=11-test-anti-patterns-never-do-these rec=fct-jfqpalye@1
name: Test oracles written as literals
rule: Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
@row_check n=380 row=decision/literal-test-oracles sha=4ff85f33c6ce v=holds judge=sonnet head=8834195b0683
searched: IDENTITY_FLOOR · in tests/ (no match)
searched: viola_agent_claude::ledger::LedgerRow · in tests/support/home.rs:13 is a type import for planting rows, not an expected-value oracle
@row_add n=381 row=decision/no-debug-string-comparison area=11-test-anti-patterns-never-do-these rec=fct-ee3ranmx@1
name: No comparison by Debug string
rule: Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
@row_check n=381 row=decision/no-debug-string-comparison sha=0301a48874c5 v=holds judge=sonnet head=8834195b0683
searched: format!("{:?}" compared to a reason or exit code · in tests/ and tests/support/ (only failure-message {:?} captures found, no comparison)
@row_add n=382 row=decision/real-on-disk-state area=11-test-anti-patterns-never-do-these rec=fct-7gyjknnu@1
name: On-disk state is never mocked
rule: Tests never mock the on-disk state and use a real tempfile home instead.
why: The architecture has no database; this is the database-equivalent rule.
@row_check n=382 row=decision/real-on-disk-state sha=5935de9158be v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:200
quote: .tempdir_in(base)
@row_add n=383 row=decision/cleanup-between-integration-tests area=11-test-anti-patterns-never-do-these rec=fct-fmecvcgv@1
name: Cleanup between integration tests
rule: Integration tests never skip cleanup between tests.
why: Order-dependent tests hide bugs.
@row_check n=383 row=decision/cleanup-between-integration-tests sha=da05ea514dc8 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:246
quote: impl Drop for TestHome {
@row_add n=384 row=decision/endpoint-from-snapshot-only area=11-test-anti-patterns-never-do-these rec=fct-aqwq345t@1
name: Endpoint taken only from the snapshot
rule: Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
@row_check n=384 row=decision/endpoint-from-snapshot-only sha=1c21b0c361cb v=holds judge=sonnet head=8834195b0683
searched: try_overwrite · in tests/ and crates/*/tests (no match; only a doc comment in crates/viola-channel/src/server.rs:98 saying it stays off)
@row_add n=385 row=decision/no-process-env-mutation-in-tests area=11-test-anti-patterns-never-do-these rec=fct-ieax375m@1
name: No process-wide env mutation in tests
rule: Tests never set process environment variables globally; environment is set per child command.
why: Setting the process environment is unsafe in edition 2024 and races across threads.
@row_check n=385 row=decision/no-process-env-mutation-in-tests sha=d39d3fa958b1 v=holds judge=sonnet head=8834195b0683
searched: set_var · in tests/ crates/*/tests crates/viola-e2e (only comments "never set_var" found, no call)
@row_add n=386 row=decision/no-sleep-synchronization area=11-test-anti-patterns-never-do-these rec=fct-cdb2tplq@1
name: No sleep for synchronization
rule: Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
@row_check n=386 row=decision/no-sleep-synchronization sha=b75c72bd0699 v=holds judge=sonnet head=8834195b0683
searched: thread::sleep · in tests/ (no match); crates/viola-e2e/tests holds two 10 ms sleeps inside bounded try_wait loops (mcp_tools.rs:480, mcp_recorded_exchange.rs:174), polling for exit, not a sync-by-sleep
@row_add n=387 row=decision/no-screen-parsing area=11-test-anti-patterns-never-do-these rec=fct-nh5slhtz@1
name: Rendered child screen is never parsed
rule: Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt.
@row_check n=387 row=decision/no-screen-parsing sha=1b34bb1b3fdd v=holds judge=sonnet head=8834195b0683
searched: vt100::Parser · in tests/ (no match; only the fake agent flag --vt100-panic-bytes and a byte search in tests/chaos_feed_panic.rs)
@row_add n=388 row=decision/child-exit-by-wait area=11-test-anti-patterns-never-do-these rec=fct-ukay3vyw@2
name: Child exit detected by waiting on the handle
rule: Child exit is detected by waiting on the child handle, never by reading master EOF.
@row_check n=388 row=decision/child-exit-by-wait sha=5c6496e66e97 v=holds judge=sonnet head=8834195b0683
at: tests/run_cli.rs:562
quote: let status = child.wait().expect("exits");
@row_add n=389 row=decision/wrapper-spawned-with-guard area=11-test-anti-patterns-never-do-these rec=fct-ajzn4txb@2
name: Long-lived wrapper spawned with a drop guard
rule: Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call.
why: A blocking call waits until the wrapper exits.
@row_check n=389 row=decision/wrapper-spawned-with-guard sha=f3695d6ee6c2 v=holds judge=sonnet head=8834195b0683
at: tests/support/cli.rs:60
quote: impl Drop for Running {
@row_add n=390 row=decision/no-production-data-in-tests area=11-test-anti-patterns-never-do-these rec=fct-ic7ldbxq@1
name: No production data in tests
rule: Tests never use production data; fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames.
except: A `workspace.repo` naming this repository's own remote is kept.
@row_check n=390 row=decision/no-production-data-in-tests sha=e11fc684ed43 v=holds judge=sonnet head=8834195b0683
at: tests/support/hygiene.rs:34
quote: drive || s.contains("/home/") || s.contains("/Users/") || s.contains("\\Users\\")
at: tests/support/hygiene.rs:27
quote: pub const PLACEHOLDER_USER: &str = "<user>";
@row_add n=391 row=decision/no-shared-mutable-state area=11-test-anti-patterns-never-do-these rec=fct-x76wnqv5@1
name: No shared mutable state across parallel tests
rule: Parallel tests never share mutable state.
@row_check n=391 row=decision/no-shared-mutable-state sha=aa46306f837e v=holds judge=sonnet head=8834195b0683
searched: static mut|lazy_static|thread_local!|serial_test|#\[serial\] · in **/*.{rs,toml,yml} (no match)
searched: max-threads|test-threads · in .config/nextest.toml (no match; each test owns a home)
@row_add n=392 row=decision/seeded-generators area=11-test-anti-patterns-never-do-these rec=fct-lg3od2u6@1
name: Seed-controlled generators
rule: Non-deterministic generators are always seed-controlled, and `proptest-regressions/` is committed.
@row_check n=392 row=decision/seeded-generators sha=88b54e520392 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:420
quote: failure_persistence: Some(Box::new(FileFailurePersistence::SourceParallel(
at: crates/viola-core/proptest-regressions/lib.txt:1
quote: # Seeds for failure cases proptest has generated in the past. It is
@row_add n=393 row=decision/state-files-never-hand-written area=11-test-anti-patterns-never-do-these rec=fct-o75kb2kj@1
name: State files are never hand-written
rule: Every booted-wrapper test takes `ledger/stamps.json` from `viola verify`, `snapshot.json` from the wrapper and `budget.json` from `viola hook statusline`, never writing them by hand.
except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
@row_check n=393 row=decision/state-files-never-hand-written sha=f72cf56c7486 v=holds judge=sonnet head=8834195b0683
at: tests/cli_version_gate.rs:174
quote: stamp(&home, RECORDED_CLI_VERSION, None);
at: tests/security_negatives_strict_modes.rs:252
quote: let (stopped, stamped) = booted_wrapper(stamped_home).stop_keep();
at: tests/hook_statusline.rs:50
quote: let bytes = fs::read(self.home.join("budget.json")).ok()?;
@row_add n=394 row=decision/no-committed-test-secrets area=11-test-anti-patterns-never-do-these rec=fct-ejn23r66@1
name: No test dirs, env files or tokens committed
rule: `--home` test directories, `.env` files and tokens are never committed.
@row_check n=394 row=decision/no-committed-test-secrets sha=a1d44a812682 v=holds judge=sonnet head=8834195b0683
searched: .env · in .gitignore (lines 30-31: .env and .env.* ignored)
searched: **/.env* · in repository (Glob, no file found)
@row_add n=395 row=decision/no-mocking-unowned-code area=11-test-anti-patterns-never-do-these rec=fct-z5k5vjqr@1
name: No mocking of what is not owned
rule: Libraries and the language runtime are never mocked without an abstraction layer.
@row_check n=395 row=decision/no-mocking-unowned-code sha=5331c87bf90a v=holds judge=sonnet head=8834195b0683
searched: automock|mock! · in **/*.rs (one match: crates/viola-pty/src/lib.rs:107 `#[cfg_attr(test, mockall::automock)]` on the product's own `pub trait Pty` at line 108)
@row_add n=396 row=decision/no-over-stubbing area=11-test-anti-patterns-never-do-these rec=fct-epqm6key@1
name: No over-stubbing
rule: Tests never over-stub.
why: Over-stubbing stops testing real behaviour.
@row_check n=396 row=decision/no-over-stubbing sha=829f074e2bd6 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=397 row=decision/no-runtime-monkey-patching area=11-test-anti-patterns-never-do-these rec=fct-sw7m3jb3@1
name: No runtime monkey-patching
rule: Production code is never monkey-patched at runtime; dependency injection is used instead.
@row_check n=397 row=decision/no-runtime-monkey-patching sha=4e1b5658996a v=holds judge=sonnet head=8834195b0683
searched: monkey|set_var · in {src,crates,tests}/**/*.rs (only comments saying "never set_var" in tests/support/home.rs:867 and crates/viola-e2e/src/harness/boot.rs:496)
@row_add n=398 row=decision/no-tokio-failpoint-crate area=11-test-anti-patterns-never-do-these rec=fct-mbozqxei@1
name: No failpoint crate pulling tokio
rule: No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.
@row_check n=398 row=decision/no-tokio-failpoint-crate sha=e08e437a33f6 v=holds judge=sonnet head=8834195b0683
searched: fail-parallel|failpoint|fail_point · in repository tracked files incl. Cargo.toml, Cargo.lock, crates/*/Cargo.toml (no match)
searched: mockall · in Cargo.toml (line 214 `mockall = "=0.15.0"`)
@row_add n=399 row=decision/real-hook-in-e2e area=11-test-anti-patterns-never-do-these rec=fct-ud76yym4@1
name: Real viola hook in E2E
rule: `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
@row_check n=399 row=decision/real-hook-in-e2e sha=7917b4f9d211 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:520
quote: .and_then(|d| fs::read_to_string(d.join("hooks").join("hooks.json")).ok());
at: src/bin/viola-fake-agent.rs:631
quote: let mut cmd = Command::new(command);
@row_add n=400 row=decision/tests-run-in-parallel area=11-test-anti-patterns-never-do-these rec=fct-yijholqq@1
name: Tests run in parallel
rule: Tests are never serialised with a single thread; isolation comes from the per-test home.
except: The fuzz stages are single jobs, by design.
@row_check n=400 row=decision/tests-run-in-parallel sha=73d0db996204 v=holds judge=sonnet head=8834195b0683
searched: max-threads · in .config/nextest.toml (no match)
searched: test-threads · in .github/**, Cargo.toml, .config/** (no match; only crates/viola-pty/src/lib.rs:573 runs one named child test entry with --test-threads 1, not the suite)
@row_add n=401 row=decision/rust-cache-only-in-ci area=11-test-anti-patterns-never-do-these rec=fct-qk7zl643@1,fct-tam4bqkb@1
name: rust-cache is the only cache, CI only
rule: Swatinem/rust-cache is the only cache in `ci.yml`, and no release workflow uses it.
@row_check n=401 row=decision/rust-cache-only-in-ci sha=ca64a38e0f23 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:29
quote: - uses: Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2.9.2
searched: cache · in .github/** (only the six rust-cache lines in ci.yml; none in nightly.yml, no actions/cache)
@row_add n=402 row=decision/coverage-ignore-regex-not-widened area=11-test-anti-patterns-never-do-these rec=fct-b4avhaux@2
name: Coverage ignore regex is never widened
rule: The coverage ignore-filename regex, covering the fake agent, the E2E crate, test support and fuzz trees, is never widened to pass a gate without a Decisions Log entry first.
why: Widening it is a back door to lowering the coverage thresholds.
except: The path-separator class kept in the regex is not a widening, because the four trees are unchanged.
@row_check n=402 row=decision/coverage-ignore-regex-not-widened sha=fbbd97d664b7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:11-12
quote: pub const COVERAGE_IGNORE: &str = r"(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])";
@row_add n=403 row=decision/coverage-threshold-never-lowered area=11-test-anti-patterns-never-do-these rec=fct-r52fahdc@1
name: Coverage threshold never lowered to pass
rule: The coverage threshold is never lowered to pass a build.
why: Raise quality, not the bar.
@row_check n=403 row=decision/coverage-threshold-never-lowered sha=aba5e7b21b58 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:15-16
quote: pub const COVERAGE_FLOORS: [(&str, f64); 3] = [("lines", 85.0), ("functions", 95.0), ("regions", 80.0)];
@row_add n=404 row=decision/no-skipping-quality-gates area=11-test-anti-patterns-never-do-these rec=fct-55sujg7u@1
name: Quality gates never skipped
rule: Quality gates are never skipped, not even once.
@row_check n=404 row=decision/no-skipping-quality-gates sha=a77b006005de v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=405 row=decision/no-human-reviewed-visual-regression area=11-test-anti-patterns-never-do-these rec=fct-neagcwig@1
name: No human-reviewed visual regression
rule: Percy, Chromatic, Applitools and other visual regression with human review are never used.
@row_check n=405 row=decision/no-human-reviewed-visual-regression sha=82feb8d03607 v=holds judge=sonnet head=8834195b0683
searched: Percy|Chromatic|Applitools · in tracked source outside .claude/.andromeda/viola-0.x (no match)
@row_add n=406 row=decision/no-manual-smoke-step area=11-test-anti-patterns-never-do-these rec=fct-vds3ppgv@1
name: No manual smoke step
rule: No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
@row_check n=406 row=decision/no-manual-smoke-step sha=8cd211dc3ea7 v=holds judge=sonnet head=8834195b0683
searched: manual smoke|developer verifies|smoke · in .github/workflows/ci.yml (only the automated `ci-smoke` session of scripts/agent-run.sh), scripts/, tests/, crates/
@row_add n=407 row=decision/no-recorder-authored-tests area=11-test-anti-patterns-never-do-these rec=fct-xjawgeec@1
name: No recorder-authored tests
rule: Tests are never authored by Selenium IDE or the Playwright codegen recorder.
@row_check n=407 row=decision/no-recorder-authored-tests sha=e67c68d9d3f9 v=holds judge=sonnet head=8834195b0683
searched: codegen|Selenium|Playwright · in tests/, crates/*/tests, crates/*/src, src/, scripts/, .github/
@row_add n=408 row=decision/no-human-screenshot-review area=11-test-anti-patterns-never-do-these rec=fct-dx5w5yd6@1
name: No human screenshot review
rule: No test step relies on a human reviewing screenshots.
@row_check n=408 row=decision/no-human-screenshot-review sha=7bb4e0ae083e v=holds judge=sonnet head=8834195b0683
searched: screenshot · in tests/, crates/*/tests, crates/*/src, src/, scripts/, .github/
@row_add n=409 row=decision/no-real-network-in-tests area=11-test-anti-patterns-never-do-these rec=fct-bom5r3of@1
name: No real network calls in tests
rule: Tests never make real network calls; they use mocks or localhost stubs.
@row_check n=409 row=decision/no-real-network-in-tests sha=1b09cdf8a820 v=holds judge=sonnet head=8834195b0683
searched: TcpStream|TcpListener|reqwest|ureq|UdpSocket|https?://[a-z0-9] · in tests/**/*.rs, crates/*/tests/**/*.rs, crates/viola-e2e/src/**/*.rs (only a literal fixture string http://127.0.0.1:47319 in secret_scan.rs)
@row_add n=410 row=decision/no-in-process-clock-across-processes area=11-test-anti-patterns-never-do-these rec=fct-v6in7u4x@1
name: No in-process clock for another process
rule: An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case.
why: A global mock clock reaches only the test process's threads, never a spawned viola run.
@row_check n=410 row=decision/no-in-process-clock-across-processes sha=d8071793bbd3 v=holds judge=sonnet head=8834195b0683
searched: mock_instant|MockClock · in tests/, crates/*/tests, crates/*/src, src/, Cargo.toml, crates/*/Cargo.toml
@row_add n=411 row=decision/no-interactive-snapshot-review area=11-test-anti-patterns-never-do-these rec=fct-qxrlql3x@2
name: No interactive snapshot review as a gate
rule: Interactive snapshot review is never a gate step; snapshots change only through the local viola verify fixture refresh and are then committed.
@row_check n=411 row=decision/no-interactive-snapshot-review sha=7eae50f46a77 v=holds judge=sonnet head=8834195b0683
searched: cargo insta|insta review · in .github/, scripts/, crates/viola-e2e/src, tests/, src/
@row_add n=412 row=decision/no-blanket-approval-tests area=11-test-anti-patterns-never-do-these rec=fct-m765nh6l@1
name: No blanket-approval tests
rule: No test auto-approves a dialog.
@row_check n=412 row=decision/no-blanket-approval-tests sha=592736f9c882 v=holds judge=sonnet head=8834195b0683
searched: approve|auto-approve|auto_answer|auto-answer|blanket · in tests/, crates/*/tests, crates/viola-e2e/src, src/
@row_add n=413 row=decision/no-test-passing-unstamped-decision-body area=11-test-anti-patterns-never-do-these rec=fct-m765nh6l@1
name: No test passing an unguarded decision body
rule: No test passes when a decision body is emitted without a verify stamp and the wheel held by driver.
@row_check n=413 row=decision/no-test-passing-unstamped-decision-body sha=01138c7fa4ad v=unverifiable judge=sonnet head=8834195b0683
searched: unstamped|decision_emitted · in tests/ (the unstamped cases assert `decision_emitted` false, e.g. tests/hook_fail_open.rs:346, but no search shows the absence of a passing counter-case)
why: no evidence found
@row_add n=414 row=decision/harness-output-json-and-exit-code area=11-test-anti-patterns-never-do-these rec=fct-jyezwnnp@1
name: Harness commands always give JSON and exit code
rule: Every harness command prints its JSON document and exit code, never a human-only message alone.
@row_check n=414 row=decision/harness-output-json-and-exit-code sha=59dd2f561f64 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:128-129
quote: println!("{}", outcome.doc); ExitCode::from(outcome.code)
@row_add n=415 row=decision/no-flaky-test-retries area=11-test-anti-patterns-never-do-these rec=fct-jpy6mjkf@1
name: No retries for flaky tests
rule: No retry-once policy or any retry budget exists for flaky tests.
why: The zero-flakiness budget.
@row_check n=415 row=decision/no-flaky-test-retries sha=190403b775df v=holds judge=sonnet head=8834195b0683
searched: retries · in .config/nextest.toml (only `retries = 0` at line 2)
searched: retry · in crates/viola-e2e/tests/zero_retries.rs (a checker that refuses any retry), tests/, src/
@row_add n=416 row=decision/proptest-seeds-committed area=11-test-anti-patterns-never-do-these rec=fct-3bpkukik@1
name: Property tests replay from committed seeds
rule: Property tests replay deterministically: proptest regression seeds persist in committed regression files.
@row_check n=416 row=decision/proptest-seeds-committed sha=cfc5e2792d84 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:814-815
quote: failure_persistence: Some(Box::new(FileFailurePersistence::SourceParallel( "proptest-regressions",
@row_add n=417 row=decision/chaos-faults-explicit area=11-test-anti-patterns-never-do-these rec=fct-3bpkukik@1
name: Chaos faults only from explicit operations
rule: Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
@row_check n=417 row=decision/chaos-faults-explicit sha=c544f0167c74 v=holds judge=sonnet head=8834195b0683
at: tests/chaos_torn_append.rs:66
quote: .set_len(cut)
at: tests/chaos_snapshot_replay.rs:73
quote: .set_modified(SystemTime::now() - age)
@row_add n=418 row=decision/lint-fmt-and-clippy-deny-warnings area=ci-integration rec=fct-pk32flwi@2
name: Lint denies format drift and clippy warnings
rule: Lint fails on any rustfmt difference and on any clippy warning across the workspace and all targets, the fake-agent feature on.
guard: CI job lint
@row_check n=418 row=decision/lint-fmt-and-clippy-deny-warnings sha=0c89582a2313 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:278
quote: run: cargo fmt --all --check
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
guard_at: .github/workflows/ci.yml:255
@row_add n=419 row=decision/sync-crates-compile-without-features area=ci-integration rec=fct-pk32flwi@2
name: Sync crates compile alone without features
rule: Each crate on the sync-crates list is checked alone with no features, so without tokio; a crate joins the list with its creation, and an empty list fails.
guard: CI job lint, one cargo check per listed crate
@row_check n=419 row=decision/sync-crates-compile-without-features sha=6c0a9b00e287 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:274-275
quote: test -s scripts/sync-crates.txt cargo check $(sed 's/^/-p /' scripts/sync-crates.txt)
guard_at: .github/workflows/ci.yml:271
@row_add n=420 row=decision/module-acyclicity-not-gated area=ci-integration rec=fct-pk32flwi@2
name: Module acyclicity is a review, not a gate
rule: Module dependency acyclicity is an on-demand boundary review, not a gate.
why: cargo-modules reads every type and its own inherent method as a cycle whatever the filters.
@row_check n=420 row=decision/module-acyclicity-not-gated sha=773afca3753b v=holds judge=sonnet head=8834195b0683
searched: acyclic · in .github/** and scripts/*.sh (only a comment in scripts/orphans-check.sh:6 saying it is deliberately absent)
searched: cargo modules dependencies · in .github/** and scripts/*.sh (no hit)
@row_add n=421 row=decision/rmcp-features-server-transport-io area=ci-integration rec=fct-pk32flwi@2
name: rmcp takes exactly server and transport-io
rule: rmcp is present in the release graph, and packages other than rmcp request exactly its server and transport-io features.
guard: lint step rmcp features (rmcp-features.sh)
@row_check n=421 row=decision/rmcp-features-server-transport-io sha=04ffd432c5a3 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/rmcp-features.sh:16
quote: if ! printf '%s\n' "$out" | grep -q '^0rmcp v'; then
at: scripts/rmcp-features.sh:33
quote: if [ "$requested" != "server transport-io" ]; then
guard_at: .github/workflows/ci.yml:306
@row_add n=422 row=decision/viola-channel-tokio-free area=ci-integration rec=fct-pk32flwi@2,fct-3gkvok74@1
name: viola-channel normal graph is tokio-free
rule: viola-channel's normal dependency graph holds no tokio; its Tokio client sits behind its tokio feature.
guard: channel-tokio-free.sh and deny probe channel-tokio-edge
@row_check n=422 row=decision/viola-channel-tokio-free sha=c775065747e0 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/channel-tokio-free.sh:15
quote: out=$(cargo tree -p viola-channel -e normal --target all -i tokio --prefix none "$@" 2>&1) || rc=$?
at: scripts/channel-tokio-free.sh:38-39
quote: if [ "$own" != absent ]; then echo "channel-tokio-free: FAILED — viola-channel reaches tokio without its feature"
guard_at: scripts/channel-tokio-free.sh:1
@row_add n=423 row=decision/ci-single-ubuntu-leg area=ci-integration rec=fct-pk32flwi@2,fct-ijfdjbph@2,fct-sd3lvsrr@2,fct-bfdvmti3@2
name: CI runs on one ubuntu-latest leg
rule: CI jobs run on a single ubuntu-latest leg.
@row_check n=423 row=decision/ci-single-ubuntu-leg sha=d3f90b1b9328 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: .github/workflows/ci.yml:342
quote: runs-on: ubuntu-latest
at: .github/workflows/nightly.yml:13
quote: runs-on: ubuntu-latest
@row_add n=424 row=decision/cargo-deny-supply-chain-gate area=ci-integration rec=fct-3gkvok74@1
name: cargo deny supply-chain gate
rule: The supply-chain job fails closed on cargo deny advisories, licences, sources and bans for the workspace, and on advisories and sources for the fuzz workspace.
guard: CI job supply-chain
@row_check n=424 row=decision/cargo-deny-supply-chain-gate sha=71151a39b7fc v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
guard_at: .github/workflows/ci.yml:340
@row_add n=425 row=decision/serde-json-unbounded-depth-banned area=ci-integration rec=fct-3gkvok74@1
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned from the dependency graph.
guard: deny probe serde_json-unbounded-depth
@row_check n=425 row=decision/serde-json-unbounded-depth-banned sha=e88f0d6e0f9d v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:53-55
quote: [[bans.features]] crate = "serde_json" deny = ["unbounded_depth"]
guard_at: scripts/deny-probes.sh:32
@row_add n=426 row=decision/tokio-banned-per-sync-crate area=ci-integration rec=fct-3gkvok74@1
name: tokio banned from each sync crate
rule: tokio is banned from each crate on the sync-crates list, checked with that crate as sole root.
guard: step Tokio ban per sync crate (sole root)
@row_check n=426 row=decision/tokio-banned-per-sync-crate sha=d766d865ed9c v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:382
quote: cargo deny --config deny-sync.toml --manifest-path "crates/$crate/Cargo.toml" check bans
guard_at: .github/workflows/ci.yml:377
@row_add n=427 row=decision/deny-bans-probed area=ci-integration rec=fct-3gkvok74@1
name: Every deny ban has a negative probe
rule: Every cargo deny ban has a permanent negative probe that must fire, beside a clean control.
guard: deny-probes.sh
@row_check n=427 row=decision/deny-bans-probed sha=3ed927cc2664 v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=428 row=decision/workflows-audited-by-zizmor area=ci-integration rec=fct-3gkvok74@1
name: Workflows audited by zizmor
rule: The GitHub workflows are audited by zizmor in the supply-chain job.
guard: CI job supply-chain
@row_check n=428 row=decision/workflows-audited-by-zizmor sha=7fb655c302ef v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:392
quote: zizmor --format=json .github/workflows/ > target/supply-chain/zizmor.json || { cat target/supply-chain/zizmor.json; exit 1; }
guard_at: .github/workflows/ci.yml:340
@row_add n=429 row=decision/weekly-advisory-scan area=ci-integration rec=fct-3gkvok74@1
name: Weekly advisory scan
rule: Advisories for the workspace and the fuzz workspace are re-checked weekly on schedule and on manual dispatch.
guard: nightly job advisories
@row_check n=429 row=decision/weekly-advisory-scan sha=afd440c8a4a9 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:5
quote: - cron: "17 4 * * 1"
at: .github/workflows/nightly.yml:6
quote: workflow_dispatch:
at: .github/workflows/nightly.yml:29
quote: cargo deny check advisories
at: .github/workflows/nightly.yml:30
quote: cargo deny --manifest-path fuzz/Cargo.toml check advisories
guard_at: .github/workflows/nightly.yml:11
@row_add n=430 row=decision/one-instrumented-ci-test-run area=ci-integration rec=fct-sd3lvsrr@2
name: One instrumented test run in CI
rule: In CI one instrumented coverage run under nextest replaces the unit, integration and e2e runs, followed by the doctest run; the separate runs are the local equivalents.
guard: CI job test
@row_check n=430 row=decision/one-instrumented-ci-test-run sha=d1aa35a061a7 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:35
quote: # One instrumented nextest run replaces the unit and integration runs; doctest follows it.
at: .github/workflows/ci.yml:38
quote: run: bash scripts/agent-run.sh run --coverage
guard_at: .github/workflows/ci.yml:10
@row_add n=431 row=decision/ci-jobs-end-in-harness-gate area=ci-integration rec=fct-sd3lvsrr@2
name: CI test jobs end in the harness gate
rule: Each test-bearing CI job's last step is viola-harness gate requiring that job's own suites.
@row_check n=431 row=decision/ci-jobs-end-in-harness-gate sha=9f229e7bf687 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
at: .github/workflows/ci.yml:189
quote: run: bash scripts/agent-run.sh gate --require perf
at: .github/workflows/ci.yml:229
quote: run: bash scripts/agent-run.sh gate --require nextest-unit
at: .github/workflows/ci.yml:253
quote: run: bash scripts/agent-run.sh gate --require fuzz-replay
at: scripts/agent-run.sh:43
quote: exec cargo run -q -p viola-e2e --bin viola-harness -- "$@"
@row_add n=432 row=decision/mutation-testing-off-ci area=ci-integration rec=fct-ixlw7x3t@2
name: Mutation testing outside CI
rule: Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
@row_check n=432 row=decision/mutation-testing-off-ci sha=3161d7a68842 v=holds judge=sonnet head=8834195b0683
searched: run --mutants · in .github/** (only a comment at ci.yml:91 and the cargo-mutants install for harness tests at ci.yml:32-33, no job or step runs it)
searched: cargo mutants · in .github/** and scripts/*.sh (no run step)
@row_add n=433 row=decision/perf-verdict-from-harness-gate area=ci-integration rec=fct-bfdvmti3@2
name: Perf verdict from the harness gate
rule: The perf verdict comes from viola-harness gate, never from a jq binary; CI neither installs nor pins jq.
guard: CI job perf, gate --require perf
@row_check n=433 row=decision/perf-verdict-from-harness-gate sha=8cc732d598d5 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:189
quote: run: bash scripts/agent-run.sh gate --require perf
at: .github/workflows/ci.yml:144
quote: run: jq --version
guard_at: .github/workflows/ci.yml:122
@row_add n=434 row=decision/hyperfine-measures-perf area=ci-integration rec=fct-bfdvmti3@2
name: hyperfine for perf measurement
rule: hyperfine is what measures perf, exporting the JSON the gate reads.
@row_check n=434 row=decision/hyperfine-measures-perf sha=ad1fa10eb4b3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:117
quote: .arg("--export-json")
at: crates/viola-e2e/src/harness/run/perf.rs:118
quote: .arg(format!("target/agent-run/artifacts/perf-{hook}.json"))
at: crates/viola-e2e/src/harness/gate.rs:147-149
quote: let max = read_json::<Value>(&artifacts.join(&file)) .ok() .and_then(|v| v["results"][0]["max"].as_f64());
@row_add n=435 row=decision/msrv-1-96 area=ci-integration rec=fct-whs3hmpm@1
name: Minimum supported Rust 1.96
rule: The workspace is checked and unit-tested on Rust 1.96, its minimum supported version.
why: 1.96 is the security toolchain floor and the requested workspace rust-version; sysinfo needs 1.95.
guard: CI job msrv, gate --require nextest-unit
@row_check n=435 row=decision/msrv-1-96 sha=8f9ce2d5643a v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:220
quote: run: cargo check --workspace
at: .github/workflows/ci.yml:225
quote: run: bash scripts/agent-run.sh run --unit
guard_at: .github/workflows/ci.yml:229
@row_add n=436 row=decision/fuzz-corpus-replay-on-push area=ci-integration rec=fct-j6sjgyk3@1
name: Fuzz corpus replay on every push
rule: Every push and pull request replays each fuzz target's corpus without fuzzing.
guard: CI job fuzz-replay, gate --require fuzz-replay
@row_check n=436 row=decision/fuzz-corpus-replay-on-push sha=1ed2190bda17 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:249
quote: run: bash scripts/agent-run.sh run --fuzz-replay
at: crates/viola-e2e/src/harness/run/fuzz.rs:76
quote: .args(["--", "-runs=0"])
guard_at: .github/workflows/ci.yml:253
@row_add n=437 row=decision/nightly-fuzzing area=ci-integration rec=fct-j6sjgyk3@1
name: Nightly fuzzing
rule: Each fuzz target is fuzzed for 120 seconds nightly, its crash artifacts uploaded on failure.
guard: nightly job fuzz
@row_check n=437 row=decision/nightly-fuzzing sha=807cacff5bf4 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:55
quote: cargo "+$channel" fuzz run --fuzz-dir fuzz "$t" "fuzz/corpus/$t" -- -max_total_time=120
at: .github/workflows/nightly.yml:59
quote: if: failure()
guard_at: .github/workflows/nightly.yml:32
@row_add n=438 row=decision/no-cache-in-release-workflow area=ci-integration rec=fct-2qyicniz@2
name: No build cache in a release workflow
rule: The Rust build cache is used in ci.yml only, never in a release workflow.
@row_check n=438 row=decision/no-cache-in-release-workflow sha=6bd342f685f7 v=holds judge=sonnet head=8834195b0683
searched: rust-cache · in .github (only .github/workflows/ci.yml matches; nightly.yml has none; no release workflow file exists)
searched: Swatinem · in .github/workflows/nightly.yml
@row_add n=439 row=decision/harness-gate-fails-on-breach area=ci-integration rec=fct-k7t43itf@2
name: Harness gate fails on any quality breach
rule: viola-harness gate reads the junit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach.
@row_check n=439 row=decision/harness-gate-fails-on-breach sha=0e999ea14775 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:47
quote: let summary = read_json::<Value>(&artifacts.join("run-summary.json")).ok();
at: crates/viola-e2e/src/harness/gate.rs:52
quote: if JUNIT.contains(&suite) && !artifacts.join(format!("junit-{suite}.xml")).is_file() {
at: crates/viola-e2e/src/harness/gate.rs:102
quote: let Ok(summary) = read_json::<Value>(&artifacts.join("llvm-cov-summary.json")) else {
at: crates/viola-e2e/src/harness/gate.rs:147-149
quote: let max = read_json::<Value>(&artifacts.join(&file)) .ok() .and_then(|v| v["results"][0]["max"].as_f64());
at: crates/viola-e2e/src/harness/gate.rs:66-70
quote: let ok = breaches.is_empty(); Outcome::new( json!({"v": 1, "cmd": "gate", "ok": ok, "breaches": breaches}), ok, )
@row_add n=440 row=decision/least-workflow-permissions area=ci-integration rec=fct-k7t43itf@2
name: Least workflow permissions
rule: Every workflow has empty permissions at the top and contents read per job.
@row_check n=440 row=decision/least-workflow-permissions sha=21ab27b0f984 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:7
quote: permissions: {}
at: .github/workflows/nightly.yml:8
quote: permissions: {}
at: .github/workflows/ci.yml:13-14
quote: permissions: contents: read
at: .github/workflows/nightly.yml:14-15
quote: permissions: contents: read
@row_add n=441 row=decision/actions-sha-pinned area=ci-integration rec=fct-k7t43itf@2
name: Actions pinned by SHA
rule: Every action a workflow uses is pinned by SHA.
guard: zizmor unpinned-uses
@row_check n=441 row=decision/actions-sha-pinned sha=06058c09fa41 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:22
quote: - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
at: .github/workflows/nightly.yml:60
quote: uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
guard_at: .github/workflows/ci.yml:392
@row_add n=442 row=decision/one-version-source-per-tool area=ci-integration rec=fct-k7t43itf@2
name: One version source per tool
rule: Each CI tool has exactly one pinned version source; cargo-deny and zizmor run as pinned CLIs, never through their actions.
@row_check n=442 row=decision/one-version-source-per-tool sha=aff1e3198ecd v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:354
quote: tool: cargo-deny@0.20.2
at: .github/workflows/ci.yml:356
quote: run: cargo install --locked zizmor@1.30.1
searched: cargo-deny-action · in .github/workflows (none)
searched: zizmor-action · in .github/workflows (none)
@row_add n=443 row=decision/jq-runner-provided area=ci-integration rec=fct-k7t43itf@2
name: jq taken from the runner
rule: jq is taken from the CI runner, never installed; the scripts that need it refuse with tool-missing: jq when it is absent.
@row_check n=443 row=decision/jq-runner-provided sha=9ef8608d51be v=holds judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:13
quote: echo "tool-missing: jq"
at: scripts/release-check.sh:9
quote: echo "tool-missing: jq"
at: scripts/orphans-check.sh:19
quote: echo "tool-missing: jq"
at: .github/workflows/ci.yml:55
quote: run: jq --version
@row_add n=444 row=decision/ripgrep-pinned-checksum-verified area=ci-integration rec=fct-k7t43itf@2
name: ripgrep pinned and checksum-verified
rule: ripgrep is installed from a pinned, checksum-verified official release.
@row_check n=444 row=decision/ripgrep-pinned-checksum-verified sha=00250fe94c38 v=holds judge=sonnet head=8834195b0683
at: scripts/install-ripgrep.sh:10
quote: version=15.2.0
at: scripts/install-ripgrep.sh:52
quote: sha=33e15bcf1624b25cdd2a55813a47a2f95dbe126268203e76aa6a585d1e7b149c
at: scripts/install-ripgrep.sh:73
quote: verify "$work/$asset" "$sha" "$asset" || exit 1
@row_add n=445 row=decision/local-pre-push-gate-installs-nothing area=ci-integration rec=fct-k7t43itf@2
name: Local pre-push gate installs nothing
rule: The local pre-push gate installs nothing; it checks the host's tools against the pins parsed from ci.yml and refuses a missing or off-pin tool.
why: ci.yml stays the one version source.
@row_check n=445 row=decision/local-pre-push-gate-installs-nothing sha=c20b55d7d6ce v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:113
quote: let yml = fs::read_to_string(ws.root.join(".github").join("workflows").join("ci.yml"))
at: crates/viola-e2e/src/harness/pre_push/linux.rs:125
quote: return Err(Stop::new("tool-missing", name));
at: crates/viola-e2e/src/harness/pre_push/linux.rs:132
quote: return Err(Stop::new("tool-pin-mismatch", name));
@row_add n=446 row=decision/no-workflow-concurrency area=ci-integration rec=fct-k7t43itf@2
name: No workflow concurrency block
rule: No workflow has a concurrency block; zizmor's concurrency-limits finding is declined and stays visible.
why: A concurrency group cancels pending runs even without cancel-in-progress, which would drop a push's always() gate and upload chain.
@row_check n=446 row=decision/no-workflow-concurrency sha=ee80a6af3dd5 v=holds judge=sonnet head=8834195b0683
searched: concurrency · in .github/workflows/ci.yml (read whole, none)
searched: concurrency · in .github/workflows/nightly.yml (read whole, none)
@row_add n=447 row=decision/ci-stamped-home-kill-line area=ci-integration rec=fct-7d3yogoz@2
name: 20 s kill line for stamped-home tests
rule: On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
@row_check n=447 row=decision/ci-stamped-home-kill-line sha=c6833afa8049 v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:24
quote: filter = 'binary(channel_endpoint) | binary(channel_error_bodies) | binary(chaos_forward_compat) | binary(channel_links) | binary(chaos_torn_append) | binary(cli_answer) | binary(cli_budget) | binary(
at: .config/nextest.toml:25
quote: slow-timeout = { period = "10s", terminate-after = 2 }
@row_add n=448 row=decision/no-ignored-advisories area=ci-integration rec=fct-zwsm4wxk@2
name: No advisory ignored
rule: cargo deny ignores no advisory.
guard: CI job supply-chain
@row_check n=448 row=decision/no-ignored-advisories sha=6fcc12bf9b34 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:11
quote: ignore = []
guard_at: .github/workflows/ci.yml:340
@row_add n=449 row=decision/cli-e2e-via-process area=e2e-test-strategy rec=fct-4m5t4osf@5
name: CLI tested as a process
rule: CLI verbs are tested end to end by running the viola binary as a child process.
@row_check n=449 row=decision/cli-e2e-via-process sha=259fa27f4f31 v=holds judge=sonnet head=8834195b0683
at: tests/cli_output_plain.rs:22
quote: Command::new(VIOLA)
@row_add n=450 row=decision/cli-fixed-strings-pinned-by-literal area=e2e-test-strategy rec=fct-4m5t4osf@5
name: CLI fixed strings pinned by literals
rule: Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
@row_check n=450 row=decision/cli-fixed-strings-pinned-by-literal sha=7f11d0b2812b v=holds judge=sonnet head=8834195b0683
at: tests/cli_links.rs:403
quote: assert_eq!(held.stdout, "link    overseer -> builder  already linked\n");
@row_add n=451 row=decision/tty-styling-asserted-on-sgr area=e2e-test-strategy rec=fct-4m5t4osf@5
name: Terminal styling asserted on SGR sequences
rule: Terminal styling is checked by running a verb inside the outer PTY and asserting only on the SGR sequences in verb output, never on the child agent screen.
@row_check n=451 row=decision/tty-styling-asserted-on-sgr sha=eafe871c6e76 v=holds judge=sonnet head=8834195b0683
at: tests/cross_list_rows.rs:243
quote: let stream = on_a_terminal(&bin, &home, env, 200);
at: tests/cross_list_rows.rs:246
quote: escape_sequences(&stream),
@row_add n=452 row=contract/cli-json-failure/document area=e2e-test-strategy rec=fct-4m5t4osf@5
boundary: CLI --json failures
item: failure document
promise: Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
@row_check n=452 row=contract/cli-json-failure/document sha=2438f255d3b0 v=holds judge=sonnet head=8834195b0683
at: tests/cli_error_bodies.rs:102-104
quote: assert!(ran.stderr.is_empty(), "{label}: stderr holds bytes"); assert_eq!(ran.stdout.lines().count(), 1, "{label}: one document"); assert!(!ran.stdout.contains('\u{1b}'), "{label}: an ESC byte");
at: tests/cli_error_bodies.rs:107
quote: assert_eq!(ran.code, Some(row.exit), "{label}");
@row_add n=453 row=decision/hook-e2e-recorded-stdin area=e2e-test-strategy rec=fct-ennvbmje@2
name: Hook verbs fed recorded stdin
rule: The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
@row_check n=453 row=decision/hook-e2e-recorded-stdin sha=f467c1c7c7a9 v=holds judge=sonnet head=8834195b0683
at: tests/hook_events.rs:254
quote: let path = workspace_path("fixtures/claude")
at: tests/hook_events.rs:416
quote: let mut child = Command::new(VIOLA)
@row_add n=454 row=decision/tui-outer-pty-driver area=e2e-test-strategy rec=fct-tr6c4sml@3
name: Passthrough tested through an outer PTY
rule: The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
@row_check n=454 row=decision/tui-outer-pty-driver sha=37b561a06a1c v=holds judge=sonnet head=8834195b0683
at: tests/support/outer_pty.rs:78
quote: let mut pty = viola_pty::spawn(&spec).expect("outer pty spawn");
at: tests/support/outer_pty.rs:124
quote: self.exit = self.pty.try_wait().expect("try_wait");
@row_add n=455 row=decision/outer-pty-reader-drained area=e2e-test-strategy rec=fct-tr6c4sml@3
name: Outer PTY reader drained on its own thread
rule: An outer-PTY test drains the master reader on its own thread.
@row_check n=455 row=decision/outer-pty-reader-drained sha=060746a91eab v=holds judge=sonnet head=8834195b0683
at: tests/support/outer_pty.rs:84
quote: std::thread::spawn(move || {
@row_add n=456 row=decision/tui-verdicts-from-records area=e2e-test-strategy rec=fct-tr6c4sml@3
name: TUI verdicts from records
rule: TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status.
except: The forced vt100 panic case byte-searches what the drained master reader read.
@row_check n=456 row=decision/tui-verdicts-from-records sha=4a4ec4c4d761 v=holds judge=sonnet head=8834195b0683
at: tests/chaos_feed_panic.rs:92
quote: let events = support::ndjson::read_lines(&wrapper.instance_dir().join("events.ndjson"));
at: tests/chaos_feed_panic.rs:116
quote: while !wrapper.shown().windows(3).any(|w| w == [0xe4, 0xb8, 0xad]) {
@row_add n=457 row=decision/channel-e2e-driver area=e2e-test-strategy rec=fct-6q7xvkh4@1
name: Channel tested with the sync client
rule: The wrapper channel is tested with the viola-channel sync client, a raw interprocess stream and jsonschema validation.
@row_check n=457 row=decision/channel-e2e-driver sha=37c01e00ebd7 v=holds judge=sonnet head=8834195b0683
at: tests/channel_links.rs:15
quote: use viola_channel::Client;
at: tests/channel_error_bodies.rs:16
quote: use interprocess::local_socket::traits::Stream as _;
at: tests/channel_error_bodies.rs:204
quote: let validator = jsonschema::validator_for(&schema).expect("the schema");
@row_add n=458 row=decision/channel-endpoint-from-snapshot area=e2e-test-strategy rec=fct-6q7xvkh4@1
name: Channel endpoint read from the snapshot
rule: Channel tests read the wrapper endpoint only from the instance snapshot.
@row_check n=458 row=decision/channel-endpoint-from-snapshot sha=fea085ffa169 v=holds judge=sonnet head=8834195b0683
at: tests/channel_endpoint.rs:191-192
quote: let snapshot = snapshot_data(&booted_wrapper.instance_dir()).expect("snapshot"); let endpoint = snapshot["endpoint"].as_str().expect("endpoint").to_owned();
@row_add n=459 row=decision/mcp-tests-own-server-child area=e2e-test-strategy rec=fct-thp7vui4@1
name: MCP tests own their server child
rule: MCP tests spawn and own a viola mcp child, closing its stdin and awaiting its exit, never using rmcp's child-process transport.
why: That transport kills the server 3 s after a close, leaving a truncated coverage profile.
@row_check n=459 row=decision/mcp-tests-own-server-child sha=ecf719e260f4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:44
quote: rmcp = { workspace = true, features = ["client"] }
@row_add n=460 row=decision/viola-e2e-rmcp-client-only area=e2e-test-strategy rec=fct-thp7vui4@1
name: viola-e2e takes rmcp client only
rule: The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
@row_check n=460 row=decision/viola-e2e-rmcp-client-only sha=cef939733494 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:44
quote: rmcp = { workspace = true, features = ["client"] }
@row_add n=461 row=decision/library-surface-at-unit-tier area=e2e-test-strategy rec=fct-okzgoz3x@1
name: Library-only surface covered at unit tier
rule: Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
@row_check n=461 row=decision/library-surface-at-unit-tier sha=eda6fec21a30 v=unverifiable judge=sonnet head=8834195b0683
searched: library-only · in crates/viola-e2e/tests
why: no evidence found; what is not covered end to end cannot be shown by a bounded search
@row_add n=462 row=contract/viola-run-start/event-order area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: first three events
promise: The first three events.ndjson records are wheel{cause:"start"}, budget-gate, then session-start{source:"hook"} with data.cause "startup", in that order.
@row_check n=462 row=contract/viola-run-start/event-order sha=d62fc22c7baa v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:620
quote: json!({"holder": Wheel::Driver.as_str(), "cause": WheelCause::Start.as_str()}),
at: src/cmd/run.rs:622
quote: (EventKind::BudgetGate, budget.data()),
at: src/cmd/run.rs:111
quote: Source::Hook,
at: crates/viola-agent-claude/src/hook.rs:109
quote: const SESSION_CAUSES: [&str; 4] = ["startup", "clear", "resume", "compact"];
@row_add n=463 row=contract/viola-run-start/child-after-events area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: child spawn
promise: The child agent is spawned only after the start events are written.
@row_check n=463 row=contract/viola-run-start/child-after-events sha=5a1f5a86c689 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:366
quote: let (beat, snapshot) = start_state(
at: src/cmd/run.rs:390
quote: let spawned = spawn_child(
at: src/cmd/run.rs:625
quote: append_event(instance_dir, &line)?;
@row_add n=464 row=contract/instance-snapshot/pinned-bin area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: instance snapshot
item: pinned_bin
promise: The instance snapshot holds pinned_bin.
@row_check n=464 row=contract/instance-snapshot/pinned-bin sha=8d3b9a01b494 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:44
quote: pub pinned_bin: String,
@row_add n=465 row=contract/instance-snapshot/started-at area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: instance snapshot
item: started_at
promise: The instance snapshot holds started_at.
@row_check n=465 row=contract/instance-snapshot/started-at sha=c444b1972909 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:43
quote: pub started_at: String,
@row_add n=466 row=contract/instance-snapshot/child-pid area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: instance snapshot
item: child_pid
promise: The instance snapshot holds child_pid.
@row_check n=466 row=contract/instance-snapshot/child-pid sha=d20c39ecd25d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:56
quote: pub child_pid: Option<u32>,
@row_add n=467 row=contract/instance-snapshot/endpoint area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: instance snapshot
item: endpoint
promise: The instance snapshot holds the wrapper channel endpoint.
@row_check n=467 row=contract/instance-snapshot/endpoint sha=e448be76deee v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:41
quote: pub endpoint: Option<String>,
@row_add n=468 row=contract/viola-list-json/live-instance area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola list --json
item: running wrapped instance
promise: A running wrapped instance's item reads wrapped:true and liveness "live".
@row_check n=468 row=contract/viola-list-json/live-instance sha=a1ee06ba81d3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:234
quote: wrapped: true,
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
@row_add n=469 row=contract/viola-run-start/duplicate-run area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: duplicate run
promise: A second run of an instance already running exits 1.
@row_check n=469 row=contract/viola-run-start/duplicate-run sha=1264bc9bafc7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:413
quote: Started::Refused(ExitCode::from(1))
@row_add n=470 row=contract/viola-run-start/squatted-endpoint area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: squatted endpoint
promise: A run whose channel endpoint is already held by another listener exits 1.
@row_check n=470 row=contract/viola-run-start/squatted-endpoint sha=b3537941fa78 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:502
quote: Err(ChannelError::BindTaken) => Ok(Bound::Taken),
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=471 row=contract/viola-run-start/tampered-pinned-exe area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: tampered pinned exe
promise: A run whose pinned executable was altered exits 1.
@row_check n=471 row=contract/viola-run-start/tampered-pinned-exe sha=68086869b663 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:467
quote: Err(PinError::HashMismatch) => {
at: src/cmd/run.rs:282
quote: return Ok(refused_with(
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=472 row=contract/viola-run-start/plugin-hooks area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: plugin hooks.json
promise: Each start rewrites the instance's plugin hooks.json, its hook commands the absolute pinned_bin path, never a PATH lookup.
@row_check n=472 row=contract/viola-run-start/plugin-hooks sha=0065537c62c1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:475-478
quote: for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) {         let path = plugin_dir.join(rel);         create_private_dir(path.parent().unwrap_or(&plugin_dir))?;         replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
at: crates/viola-agent-claude/src/lib.rs:151
quote: .replace(BIN_TOKEN, pinned_bin_fwd)
at: plugin/hooks/hooks.json:4
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "session-start"], "timeout": 5}]}
@row_add n=473 row=contract/viola-run-start/settings-override area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: settings.json
promise: Each start rewrites settings.json whole at 0600 as a statusLine command of the absolute pinned path plus hook statusline, only when every path character is an ASCII letter, a digit or one of _ - . / :.
@row_check n=473 row=contract/viola-run-start/settings-override sha=38dbea360025 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:95
quote: if !pinned_bin_fwd.chars().all(is_shell_plain) {
at: crates/viola-agent-claude/src/statusline.rs:103
quote: c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':')
at: crates/viola-agent-claude/src/statusline.rs:98-99
quote: let command = format!("{pinned_bin_fwd} hook statusline");     Some(json!({"statusLine": {"type": "command", "command": command}}).to_string())
at: src/cmd/run.rs:568
quote: replace_private(&path, document.as_bytes(), FILE_MODE)?;
@row_add n=474 row=contract/viola-run-start/home-tree-check area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: home-tree strict-modes check
promise: The home-tree check runs before the wrapper log opens; a refused tree exits 1 with the home-state or instance-state unable/hint pair, opens no log, writes no line and starts no child.
@row_check n=474 row=contract/viola-run-start/home-tree-check sha=8bcba61aa833 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:172
quote: refuse_tree(&args.name, refused);
at: src/cmd/run.rs:173
quote: return Ok(ExitCode::from(1));
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/cmd/run.rs:211
quote: let (unable, hint) = human::home_state_pair();
at: src/cmd/run.rs:215
quote: let (unable, hint) = human::instance_state_pair(name.as_ref());
@row_add n=475 row=contract/viola-run-start/socket-dir-check area=e2e-test-strategy rec=fct-4khqx2uf@2
boundary: viola run start
item: socket directory check
promise: The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.
@row_check n=475 row=contract/viola-run-start/socket-dir-check sha=34bf2c242698 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:497
quote: if !socket_dir_ready(&endpoint) {
at: src/cmd/run.rs:292
quote: let (endpoint, server) = match bind_endpoint(name, &home)? {
at: src/cmd/run.rs:300-301
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {                 refuse_socket_dir(name);
at: src/cmd/run.rs:412
quote: run::log_self_exit(1, Some(detail));
@row_add n=476 row=contract/viola-send/turn-running area=e2e-test-strategy rec=fct-n6muav3b@2,fct-imixgk6n@1
boundary: viola send
item: turn-running refusal
promise: While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended a send is accepted.
@row_check n=476 row=contract/viola-send/turn-running sha=7b23fa118e9b v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:116
quote: return ctx.not_delivered(None, NotDelivered::TurnRunning);
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
at: src/human/hints.rs:48
quote: "turn-running" => format!("a turn is running; viola wait {name} first"),
at: src/run/send/slot.rs:213
quote: slot.wheel.turn_ended();
@row_add n=477 row=contract/mcp-tools/result-body area=e2e-test-strategy rec=fct-n6muav3b@2,fct-qbytqn26@3,fct-mdqnbyad@2,fct-p2i2onck@2,fct-3r2a46vd@2
boundary: MCP tools
item: result and refusal body
promise: Each MCP tool's result and refusal equal the CLI's --json document less v.
@row_check n=477 row=contract/mcp-tools/result-body sha=08688ac74e5a v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:66
quote: return Ok(ok(payload.clone()));
at: crates/viola-mcp/src/result.rs:83
quote: Ok(refused(reason, detail))
@row_add n=478 row=contract/viola-send/ok-document area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: ok document
promise: A confirmed send exits 0 with ok.cursor at the events file's length before the send and ok.submitted_at in RFC 3339 ms Z.
@row_check n=478 row=contract/viola-send/ok-document sha=90059911ffb7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:90
quote: let at = tail.line_start();
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
at: crates/viola-state/src/lib.rs:31
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=479 row=contract/viola-send/records area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: send records
promise: The records after the send's cursor are send-issued{cursor, from} then prompt-submitted{origin:"driver"} holding the typed text.
@row_check n=479 row=contract/viola-send/records sha=e6cd10764b37 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:180
quote: let mut data = json!({"cursor": cursor});
at: src/run/send.rs:182
quote: data["from"] = json!(from);
at: src/run/send/slot.rs:203
quote: line.data["origin"] = json!("driver");
@row_add n=480 row=contract/viola-send/paste-framing area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: typed framing
promise: A sent text is typed as one bracketed paste followed by one CR, its newlines kept inside.
@row_check n=480 row=contract/viola-send/paste-framing sha=44e2959b0d7f v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:79-81
quote: bytes.extend_from_slice(b"\x1b[200~"); bytes.extend_from_slice(text.as_bytes()); bytes.extend_from_slice(b"\x1b[201~\r");
@row_add n=481 row=contract/viola-send/trailing-newlines area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: trailing newlines
promise: A text ending in LF, CR, CRLF or two CRs is typed without that ending and confirmed.
@row_check n=481 row=contract/viola-send/trailing-newlines sha=8dfaba5ee284 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232
quote: let text = text.trim_end_matches(['\r', '\n']);
at: src/run/send.rs:91
quote: let typed = typed_text(text);
@row_add n=482 row=contract/viola-send/inner-cr area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: inner CR
promise: A CR or CR LF inside a text is typed as one LF, so the send is confirmed and the driver keeps the wheel.
@row_check n=482 row=contract/viola-send/inner-cr sha=9cfe8ace8cca v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:234
quote: Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: src/run/send/slot.rs:219
quote: if line.kind == EventKind::PromptSubmitted && line.data["origin"] == "human" {
@row_add n=483 row=contract/viola-send/no-prompt-submitted area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: unconfirmed send
promise: A send with no submitted prompt exits 13 {"refusal":"not-delivered","detail":"no-prompt-submitted"} and logs send-refused with the same refusal and detail.
@row_check n=483 row=contract/viola-send/no-prompt-submitted sha=49ebab0cfb0b v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/run/send.rs:271
quote: let mut data = json!({"refusal": refusal.as_str(), "detail": detail});
at: src/run/send.rs:279
quote: self.record(EventKind::SendRefused, data)?;
@row_add n=484 row=contract/viola-send/unconfirmable-local-command area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: local command with no measured post-condition
promise: Exits 0 with ok {confirmed:false, detail:"unconfirmable", cursor}, logs send-issued then send-confirmed{confirmed:false}, no send-refused, no confirmation window.
@row_check n=484 row=contract/viola-send/unconfirmable-local-command sha=acf0e6ee3afa v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
at: src/run/send.rs:221-222
quote: let data = json!({"cursor": cursor, "confirmed": false}); self.record(EventKind::SendConfirmed, data)?;
@row_add n=485 row=contract/viola-send/compact-confirmed area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: /compact on a verified CLI
promise: Confirmed by its post-condition alone, a session-start with cause compact and the same agent_session_id, within its confirmation window, else not-delivered/no-prompt-submitted.
@row_check n=485 row=contract/viola-send/compact-confirmed sha=b15338b3b148 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:112-113
quote: Some("compact") if prior.as_deref() == Some(id) => { Some(PostCondition::CompactedSession)
at: src/run/send/slot.rs:177
quote: Some(Confirms::Post(PostCondition::CompactedSession)) => COMPACT_CONFIRM_WINDOW,
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
@row_add n=486 row=contract/viola-send/clear-confirmed area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: /clear on a verified CLI
promise: Confirmed by its post-condition alone, a session-start with cause clear and a new agent_session_id, ending exit 0 {submitted_at, cursor}.
@row_check n=486 row=contract/viola-send/clear-confirmed sha=0792c13bdf1c v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:111
quote: Some("clear") if prior.as_deref() != Some(id) => Some(PostCondition::NewSession),
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
@row_add n=487 row=contract/viola-send/off-list-slash-text area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: slash text off the compiled list
promise: Exits 13 not-delivered/no-prompt-submitted.
@row_check n=487 row=contract/viola-send/off-list-slash-text sha=aa7b2bab8914 v=holds judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:29
quote: None => Confirms::Prompt,
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=488 row=contract/viola-send/human-key-during-gate area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send
item: human key during the gate wait
promise: The human key wins: the wheel record precedes send-refused, send exits 10 human-typing with nothing typed, and the key reaches the child.
@row_check n=488 row=contract/viola-send/human-key-during-gate sha=149733b5f955 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:132
quote: if let Some(detail) = slot.wheel.human_typing() {
at: src/run/send.rs:135
quote: RefusalReason::HumanTyping,
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
at: src/run/wheel.rs:127-128
quote: pub(crate) fn human_input(&self) { self.apply(Input::Human, false);
@row_add n=489 row=contract/viola-send-human-mode/read-back area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send human mode
item: confirmed line
promise: A confirmed send prints the stdout line [RB] read back … cursor <n>.
@row_check n=489 row=contract/viola-send-human-mode/read-back sha=6406cfdc9bd9 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:26
quote: out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
at: src/cmd/send.rs:114
quote: let _ = human::write_read_back(
@row_add n=490 row=contract/viola-send-human-mode/unable area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send human mode
item: refusal lines
promise: A refused send prints a stderr [/ ] unable line naming instance, refusal and detail, then a last-line hint:.
@row_check n=490 row=contract/viola-send-human-mode/unable sha=2b309ffa0674 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:44
quote: let mut text = mirror_line("[/ ]", "unable", name, reason);
at: src/human/mirror.rs:46
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=491 row=contract/viola-send-human-mode/unconfirmable area=e2e-test-strategy rec=fct-qbytqn26@3
boundary: viola send human mode
item: unconfirmable line
promise: A local command with no measured post-condition prints one stdout line [  ] unconfirmable <name> local command, no measured post-condition, exit 0, nothing on stderr.
@row_check n=491 row=contract/viola-send-human-mode/unconfirmable sha=544caa080e29 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:33
quote: out.write_all(mirror_line("[  ]", "unconfirmable", name, note).as_bytes())
at: src/cmd/send.rs:128
quote: let _ = human::write_send_unconfirmable(&mut io::stdout().lock(), self.name);
@row_add n=492 row=contract/turn-ended/background-tasks area=e2e-test-strategy rec=fct-mdqnbyad@2
boundary: turn-ended event data
item: background_tasks
promise: The Stop array's length, or null.
@row_check n=492 row=contract/turn-ended/background-tasks sha=f280f6b8d8b2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:172
quote: "background_tasks": known.background_tasks.map(|tasks| tasks.len()),
@row_add n=493 row=contract/mcp-wait/timeout-ms area=e2e-test-strategy rec=fct-mdqnbyad@2
boundary: MCP wait
item: timeout_ms
promise: The bound sent is clamped to 50 000 ms, which is also its default.
@row_check n=493 row=contract/mcp-wait/timeout-ms sha=4728f6cd9d50 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:46
quote: given.map_or(WAIT_CAP, |given| given.min(WAIT_CAP))
at: crates/viola-mcp/src/tools.rs:42
quote: pub const WAIT_CAP: u64 = 50_000;
@row_add n=494 row=contract/viola-wait/non-waking-records area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola wait
item: activity and wheel records
promise: Activity and wheel records never wake a parked wait.
@row_check n=494 row=contract/viola-wait/non-waking-records sha=1b24e680b9cd v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:201-206
quote: if EventKind::WAIT_WAKE .iter() .any(|k| Some(k.as_str()) == kind) { return Ok(Some(line)); }
at: crates/viola-core/src/lib.rs:99-106
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired, ];
@row_add n=495 row=contract/viola-wait/turn-ended area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola wait
item: turn-ended wake
promise: Exits 0 with event.kind "turn-ended" and a cursor at or past that record's offset.
@row_check n=495 row=contract/viola-wait/turn-ended sha=403a4a1a3126 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:155
quote: return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
at: src/cmd/wait.rs:85
quote: Ok(ExitCode::SUCCESS)
@row_add n=496 row=contract/viola-wait/already-past area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola wait
item: event already past the cursor
promise: A wait after a cursor whose event is already logged returns that event at once, exit 0, no timed_out.
@row_check n=496 row=contract/viola-wait/already-past sha=0efc151a0c1e v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:153-155
quote: if let Some(line) = scan(instance_dir, &mut from)? { span.record("woken_kind", line.value["kind"].as_str()); return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
at: src/cmd/wait.rs:85
quote: Ok(ExitCode::SUCCESS)
@row_add n=497 row=contract/viola-wait/timeout area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola wait
item: timeout
promise: A wait that times out gives {timed_out:true} with exit 0.
@row_check n=497 row=contract/viola-wait/timeout sha=8f5379d2e6fe v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:161
quote: return Ok(json!({"ok": {"timed_out": true}}));
at: src/cmd/wait.rs:85
quote: Ok(ExitCode::SUCCESS)
@row_add n=498 row=contract/viola-last/document area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: viola last
item: last document
promise: Gives {last_assistant_message, ts} of the last turn.
@row_check n=498 row=contract/viola-last/document sha=3af0e362d172 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:177
quote: Ok(json!({"ok": {"last_assistant_message": message, "ts": ts}}))
@row_add n=499 row=contract/cli-exit-codes/21 area=e2e-test-strategy rec=fct-nb7ralop@1,fct-mdqnbyad@2
boundary: CLI exit codes
item: 21
promise: Instance unreachable: {"v":1,"error":"instance-unreachable","detail":<cause>}, the cause endpoint-lost, instance-dead or unwrapped-name.
@row_check n=499 row=contract/cli-exit-codes/21 sha=3e8f31e7e621 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: crates/viola-core/src/lib.rs:197-199
quote: Self::InstanceDead => "instance-dead", Self::UnwrappedName => "unwrapped-name", Self::EndpointLost => "endpoint-lost",
@row_add n=500 row=contract/mcp-tools/instance-unreachable area=e2e-test-strategy rec=fct-nb7ralop@1
boundary: MCP tools
item: instance-unreachable
promise: is_error:true with the CLI document less v, text refused: instance-unreachable.
@row_check n=500 row=contract/mcp-tools/instance-unreachable sha=4681975395ef v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=501 row=contract/viola-run/child-on-wrapper-kill area=e2e-test-strategy rec=fct-lhsyucqg@2
boundary: viola run wrapper
item: wrapper killed
promise: When the wrapper is killed, its child agent is gone within a bounded time.
@row_check n=501 row=contract/viola-run/child-on-wrapper-kill sha=e026ffff67f5 v=holds judge=sonnet head=8834195b0683
at: tests/run_cli.rs:679
quote: while process_start(child_pid) == Some(child_started) {
at: tests/run_cli.rs:681
quote: watch.deadline_check(deadline, "the child outlived its wrapper");
@row_add n=502 row=contract/question-event/option-details area=e2e-test-strategy rec=fct-p2i2onck@2
boundary: question event
item: option_details
promise: One {description, preview} per option, each a string or null.
@row_check n=502 row=contract/question-event/option-details sha=d4ee751e2f73 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:154
quote: |o| json!({"description": o["description"].as_str(), "preview": o["preview"].as_str()}),
@row_add n=503 row=contract/viola-answer/allow-suggestion-fields area=e2e-test-strategy rec=fct-p2i2onck@2
boundary: viola answer
item: permission allow
promise: An allow's suggestion fields never reach the agent CLI.
@row_check n=503 row=contract/viola-answer/allow-suggestion-fields sha=f3a36d2b6050 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:403
quote: let mut decision = json!({"behavior": behavior});
at: crates/viola-agent-claude/src/dialog.rs:404-406
quote: if let Some(message) = message { decision["message"] = json!(message); }
@row_add n=504 row=contract/dialog-events/logged-once area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog events
item: question, permission, plan
promise: Each dialog event is logged exactly once with a dialog_id, and a parked wait returns it at once.
@row_check n=504 row=contract/dialog-events/logged-once sha=baf2f5f1c731 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:302-304
quote: let appended = self.feed.appending_dialog(hold, || { append_event_at(&self.instance_dir, |_| line.clone()) });
at: src/run/dialog.rs:288
quote: data.insert("dialog_id".to_owned(), json!(dialog_id));
@row_add n=505 row=contract/hook-process/exit area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook process
item: exit and stderr
promise: The hook process exits 0 with empty stderr.
@row_check n=505 row=contract/hook-process/exit sha=28687c5900f9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:135
quote: Ok(()) => Ok(ExitCode::SUCCESS),
@row_add n=506 row=contract/hook-decision/question area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: question answer
promise: PreToolUse permissionDecision "allow" with updatedInput.answers, annotations included.
@row_check n=506 row=contract/hook-decision/question sha=ff4f5931ee70 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:360
quote: updated.insert("answers".to_owned(), json!(answers));
at: crates/viola-agent-claude/src/dialog.rs:362
quote: updated.insert("annotations".to_owned(), Value::Object(annotations.clone()));
at: crates/viola-agent-claude/src/dialog.rs:395
quote: let mut output = json!({"hookEventName": "PreToolUse", "permissionDecision": "allow"});
@row_add n=507 row=contract/hook-decision/permission area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: permission answer
promise: PermissionRequest decision.behavior "allow", or "deny" with message.
@row_check n=507 row=contract/hook-decision/permission sha=cc8cba073b72 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:382
quote: (HookEvent::PermissionRequest, Response::Permission { behavior, message }) => {
at: crates/viola-agent-claude/src/dialog.rs:407
quote: json!({"hookSpecificOutput": {"hookEventName": "PermissionRequest", "decision": decision}})
@row_add n=508 row=contract/hook-decision/plan-approve area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: plan approve
promise: PreToolUse allow with updatedInput as the tool's own input unchanged, and no PermissionRequest body.
@row_check n=508 row=contract/hook-decision/plan-approve sha=747fdef1cd40 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:372-374
quote: ) => pre_tool_use_allow(Some(Value::Object( input.as_object().cloned().unwrap_or_default(), ))),
at: crates/viola-agent-claude/src/dialog.rs:389
quote: _ => return None,
@row_add n=509 row=contract/hook-decision/plan-revise area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: plan revise
promise: PermissionRequest deny with message.
@row_check n=509 row=contract/hook-decision/plan-revise sha=61325c0ead75 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:381
quote: ) => permission_decision("deny", message.as_deref()),
@row_add n=510 row=contract/hook-decision/concurrent-dialog area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: dialog hook decision body
item: second concurrent dialog
promise: A second dialog raised while one is pending gets an empty hook stdout.
@row_check n=510 row=contract/hook-decision/concurrent-dialog sha=803813cbe008 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:252
quote: && state.pending.is_none()
at: src/run/dialog.rs:271-272
quote: if !hold { return Ok(reply(dialog_id, None));
@row_add n=511 row=contract/viola-answer/human-wheel area=e2e-test-strategy rec=fct-2zxurlqa@1
boundary: viola answer
item: wheel held by human
promise: answer exits 10 and the hook stdout stays empty.
@row_check n=511 row=contract/viola-answer/human-wheel sha=a969c69fcc9a v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:429-430
quote: if let Some(detail) = self.wheel.human_typing() { return Ok(refusal(
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
at: src/run/dialog.rs:254
quote: && self.wheel.holder() == Wheel::Driver;
at: src/run/dialog.rs:272
quote: return Ok(reply(dialog_id, None));
@row_add n=512 row=decision/no-decision-body-without-driver-and-stamp area=e2e-test-strategy rec=fct-2zxurlqa@1
name: Decision bodies only for a driver on a stamped CLI
rule: No dialog decision body is ever emitted unless the wheel is driver and the agent CLI version is stamped.
@row_check n=512 row=decision/no-decision-body-without-driver-and-stamp sha=1261f695b21d v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:251-254
quote: let hold = self.cli_verified && state.pending.is_none() && !unanswerable && self.wheel.holder() == Wheel::Driver;
@row_add n=513 row=decision/terminal-replies-not-editing area=e2e-test-strategy rec=fct-3r2a46vd@2
name: Terminal replies are not editing
rule: Terminal reply shapes on a closed list are read as not editing and never take the wheel; any other stdin input is typing.
guard: classifier_terminal_reply_is_not_editing, classifier_typing_is_editing
@row_check n=513 row=decision/terminal-replies-not-editing sha=9f4d0d3b8746 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/wheel.rs:443-445
quote: let reply = is_reply(&self.params, byte); self.reset(); !reply
guard_at: src/run/wheel.rs:641
@row_add n=514 row=decision/focus-mouse-resize-keep-wheel area=e2e-test-strategy rec=fct-3r2a46vd@2
name: Focus, mouse and resize never take the wheel
rule: Focus, mouse and resize input never takes the wheel from the driver.
guard: tui_focus_mouse_and_resize_never_take_the_wheel
@row_check n=514 row=decision/focus-mouse-resize-keep-wheel sha=1eb0ed22a0bd v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/wheel.rs:514
quote: (None, [], b'I' | b'O') => fields == 0,
at: src/run/wheel.rs:438
quote: b'M' if self.params.is_empty() => {
guard_at: tests/tui_wheel.rs:226
@row_add n=515 row=contract/viola-run-passthrough/ctrl-z area=e2e-test-strategy rec=fct-3r2a46vd@2
boundary: viola run passthrough
item: Ctrl-Z
promise: Ctrl-Z reaches the child, and later keys still do.
@row_check n=515 row=contract/viola-run-passthrough/ctrl-z sha=a02186d26e7a v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:321-325
quote: let n = self.inner.read(buf)?; if self.keys.feed(&buf[..n]) { self.wheel.human_input(); } Ok(n)
@row_add n=516 row=decision/terminal-reply-reading-live-measured area=e2e-test-strategy rec=fct-3r2a46vd@2
name: Terminal reply reading measured live
rule: The classifier's reading of terminal replies on a real terminal is a local live measurement, not a suite.
why: The outer-PTY tier writes no terminal reply of its own.
@row_check n=516 row=decision/terminal-reply-reading-live-measured sha=102f1bb3a97c v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=517 row=contract/viola-send/human-typing area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola send
item: human-typing refusal
promise: While the human holds the wheel, send exits 10 human-typing with detail null.
@row_check n=517 row=contract/viola-send/human-typing sha=0e6550a005ec v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:96
quote: if let Some(detail) = slot.wheel.human_typing() {
at: src/run/send.rs:99
quote: RefusalReason::HumanTyping,
at: src/run/wheel.rs:114
quote: .then(|| (held.cause == WheelCause::ManualPause).then_some(HumanTyping::ManualPause))
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
@row_add n=518 row=contract/mcp-send/human-typing area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: MCP send
item: human-typing refusal
promise: is_error:true with text refused: human-typing.
@row_check n=518 row=contract/mcp-send/human-typing sha=1f7660586ef7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
@row_add n=519 row=contract/viola-release/document area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola release
item: release document
promise: Gives {wheel:"driver", budget_paused:false}, after which a send exits 0.
@row_check n=519 row=contract/viola-release/document sha=0b03c2f9e5ba v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:222
quote: Ok(json!({"ok": {"wheel": self.holder().as_str(), "budget_paused": gate.paused()}}))
@row_add n=520 row=contract/viola-send/manual-pause area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola send
item: manual-pause refusal
promise: After pause, send exits 10 with detail "manual-pause".
@row_check n=520 row=contract/viola-send/manual-pause sha=44d74622d305 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:44
quote: (Input::Pause, _, _) => Some((Wheel::Human, WheelCause::ManualPause)),
at: src/run/wheel.rs:114
quote: .then(|| (held.cause == WheelCause::ManualPause).then_some(HumanTyping::ManualPause))
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
@row_add n=521 row=decision/hints-never-suggest-release area=e2e-test-strategy rec=fct-imixgk6n@1,fct-3r2a46vd@2
name: Driver hints never suggest release
rule: No driver-facing hint, MCP refusal text, structured content, result or tool description names release.
why: Drivers are refused release, so such a hint sends the driver into a dead end.
@row_check n=521 row=decision/hints-never-suggest-release sha=f8b1ae47bda8 v=holds judge=sonnet head=8834195b0683
searched: viola release|`release`|run release|, release · in src/**/*.rs crates/**/*.rs (only comments, the verb's own files and tests; no hint or refusal string)
searched: release (case-insensitive) · in src/human/hints.rs (only the doc comment at 127 and the test at 129-162) and crates/viola-mcp/src (only tool-name lists in tests, tools.rs:483, tools.rs:652)
@row_add n=522 row=contract/viola-release/from-driver area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola release
item: release from a driver
promise: Exits 20 {"error":"wrapper-fault","detail":{"code":-32602,…}} carrying release-from-driver.
@row_check n=522 row=contract/viola-release/from-driver sha=2aa2c793314a v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:91
quote: json!({"v": 1, "error": "wrapper-fault", "detail": detail})
at: crates/viola-channel/src/lib.rs:96
quote: Self::ReleaseFromDriver => json!({"reason": "release-from-driver"}),
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
@row_add n=523 row=contract/viola-release/running-turn area=e2e-test-strategy rec=fct-imixgk6n@1
boundary: viola release
item: turn left running
promise: Under a driver-held wheel a bare release leaves a running turn and send stays 13; pause then release clears it.
@row_check n=523 row=contract/viola-release/running-turn sha=850ca30ae27c v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:151
quote: let (to, cause) = next(held.holder, held.cause, input)?;
at: src/run/wheel.rs:155-156
quote: if input == Input::Release { held.turn = Turn::Idle;
at: src/run/wheel.rs:44
quote: (Input::Pause, _, _) => Some((Wheel::Human, WheelCause::ManualPause)),
@row_add n=524 row=contract/statusline-settings-source/source-file area=e2e-test-strategy rec=fct-bo4nzuwn@1
boundary: statusline settings source
item: source-file
promise: viola reads the user statusline command from <home>/statusline-source.json (the Claude settings shape) when it exists, else in the default home from the user's .claude/settings.json, and in any other home from none; viola only reads it.
@row_check n=524 row=contract/statusline-settings-source/source-file sha=47c07c1c5020 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:523
quote: const STATUSLINE_SOURCE: &str = "statusline-source.json";
at: src/cmd/run.rs:534-536
quote: if own.exists() { return Some(own); }
at: src/cmd/run.rs:539
quote: (default == home).then(|| {
@row_add n=525 row=contract/statusline-payload/resets-at area=e2e-test-strategy rec=fct-bo4nzuwn@1,fct-kn7ryhs7@2
boundary: statusline payload
item: resets_at
promise: The statusline reader takes resets_at as epoch seconds or RFC 3339; a malformed value is recorded as "unknown" in budget.json.
@row_check n=525 row=contract/statusline-payload/resets-at sha=879ef5613b57 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:62-64
quote: Some(Value::Number(seconds)) => seconds .as_i64() .and_then(|seconds| DateTime::from_timestamp(seconds, 0)),
at: crates/viola-agent-claude/src/statusline.rs:65-67
quote: Some(Value::String(text)) => DateTime::parse_from_rfc3339(text) .ok() .map(|at| at.with_timezone(&Utc)),
at: crates/viola-agent-claude/src/statusline.rs:74
quote: _ => Reading::UNKNOWN,
at: crates/viola-core/src/lib.rs:335
quote: pub const UNKNOWN: Self = Self::Unknown(UnknownWord::Unknown);
@row_add n=526 row=decision/override-expiry-tested-at-unit-level area=e2e-test-strategy rec=fct-bo4nzuwn@1
name: Budget override expiry tested at unit level
rule: Budget override expiry is asserted only in unit cases of the pure gate decision, which takes the instant as an argument, never end to end.
why: No in-process clock reaches the separately spawned wrapper, and waiting in real time would break the sleep(N) ban.
guard: gate_decide_an_expired_override_is_dropped, gate_release_override_ends_at_its_instant
@row_check n=526 row=decision/override-expiry-tested-at-unit-level sha=b66147c48f58 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:171
quote: now: DateTime<Utc>,
searched: expire|expiry|expired (case-insensitive) · in tests/*.rs and crates/viola-e2e/tests/*.rs (only an unrelated send-window comment and a fake-agent field; no end-to-end override expiry case)
guard_at: src/run/budget.rs:615
@row_add n=527 row=contract/budget-paused-refusal/send-exit-11 area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: budget-paused refusal
item: send exit 11
promise: A send while the budget is paused exits 11 with detail naming the window, "five-hour" or "seven-day".
@row_check n=527 row=contract/budget-paused-refusal/send-exit-11 sha=557962004c2e v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:68
quote: RefusalReason::BudgetPaused => 11,
at: src/run/send/budget.rs:12
quote: Some(crossed.window.as_str()),
@row_add n=528 row=contract/statusline-hook/stdout area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: viola hook statusline
item: stdout
promise: The wrapped statusline prints the user's command output byte for byte, with exit 0 and empty stderr.
@row_check n=528 row=contract/statusline-hook/stdout sha=57e41862c83e v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:62
quote: Ok(()) => Ok(ExitCode::SUCCESS),
@row_add n=529 row=decision/statusline-command-shell-and-bound area=e2e-test-strategy rec=fct-kn7ryhs7@2
name: Statusline command run through sh under a bound
rule: The wrapped statusline arm runs the recorded user command as /bin/sh -c <command> under a 5 s bound.
state: provisional
@row_check n=529 row=decision/statusline-command-shell-and-bound sha=47df41dd54d9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:124
quote: [BIN_SH, "-c", command]
at: src/cmd/hook.rs:64
quote: const STATUSLINE_DEADLINE: Duration = Duration::from_secs(5);
@row_add n=530 row=contract/budget-json/write-condition area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: budget.json
item: write condition
promise: budget.json is written only when the statusline payload holds a rate_limits object.
@row_check n=530 row=contract/budget-json/write-condition sha=030e73623639 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:126-127
quote: let budget_written = reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
at: crates/viola-agent-claude/src/statusline.rs:32-34
quote: let Some(Value::Object(limits)) = payload.get("rate_limits") else { return Ok(None); };
@row_add n=531 row=contract/budget-gate-record/paused-true area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: budget-gate record
item: paused true
promise: A budget pause logs a budget-gate{paused:true, window} record.
@row_check n=531 row=contract/budget-gate-record/paused-true sha=983128131615 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:125-128
quote: let mut data = json!({"paused": self.paused.is_some()}); if let Some(crossed) = &self.paused { data["window"] = json!(crossed.window.as_str()); }
at: src/run/budget.rs:327
quote: EventKind::BudgetGate,
@row_add n=532 row=contract/budget-gate-record/paused-false area=e2e-test-strategy rec=fct-kn7ryhs7@2,fct-kylcwwrw@2
boundary: budget-gate record
item: paused false
promise: A per-instance release logs one budget-gate{paused:false, override_until} record for that instance alone, its override_until later than its ts and equal to the instance snapshot's budget_override_until.
@row_check n=532 row=contract/budget-gate-record/paused-false sha=7ff15d25a3ad v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:317
quote: override_until: Some(end.clone()),
at: src/run/budget.rs:333-337
quote: self.snapshots.update(|snapshot| { snapshot.budget_paused = standing.paused.is_some(); snapshot .budget_override_until .clone_from(&standing.override_until);
at: src/run/budget.rs:129-131
quote: if let Some(end) = &self.override_until { data["override_until"] = json!(end); }
@row_add n=533 row=contract/release-budget-json/document area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: release --budget --json document
item: document
promise: release --budget gives {budget_paused:false, override_until} when a pause stood and {budget_paused:false} alone when none did; the next send exits 0.
@row_check n=533 row=contract/release-budget-json/document sha=388f2ca9703a v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:215-219
quote: let mut ok = json!({"wheel": self.holder().as_str(), "budget_paused": false}); if let Some(end) = lifted { ok["override_until"] = json!(end); } return Ok(json!({"ok": ok}));
at: src/run/budget.rs:311-313
quote: let Some(crossed) = &recorded.paused else { return Ok(None); };
@row_add n=534 row=decision/statusline-command-never-runs-unsafe area=e2e-test-strategy rec=fct-kn7ryhs7@2
name: Statusline command never runs over unsafe state
rule: The user statusline command never runs when strict-modes fails or over a replayed snapshot; the arm then exits 0 and prints nothing.
guard: statusline_over_a_replayed_snapshot_runs_nothing
@row_check n=534 row=decision/statusline-command-never-runs-unsafe sha=3da15aaa5458 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42-43
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
at: src/cmd/hook/statusline.rs:129-132
quote: let command = match read_snapshot_or_replay(&instance.dir) { Ok(Recovered::Snapshot(snapshot)) => snapshot.statusline_command, _ => None, };
guard_at: src/cmd/hook/statusline.rs:530
@row_add n=535 row=contract/list-json/budget-paused area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: list --json
item: budget_paused
promise: The top-level .ok.budget.paused reports the budget reading's pause, while each item's budget_paused reports that instance's own gate, false under a per-instance override.
@row_check n=535 row=contract/list-json/budget-paused sha=30dc4ec7a294 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:159-161
quote: let paused = budget::decide(Some(&stamped.reading), thresholds, None, now) .paused .is_some();
at: crates/viola-state/src/board.rs:238
quote: budget_paused: Some(snapshot.budget_paused),
at: src/run/budget.rs:173-177
quote: if let Some(end) = override_until.filter(|end| later(end, now)) { return Standing { paused: None, override_until: Some(end.to_owned()), };
@row_add n=536 row=decision/budget-override-per-instance area=e2e-test-strategy rec=fct-kn7ryhs7@2,fct-kylcwwrw@2
name: Budget override is per instance
rule: release --budget lifts the pause for the named instance alone; other instances under the same reading stay paused and are refused.
guard: path6_two_instance_release_over_a_harness_session, gate_release_on_one_instance_leaves_another_paused
@row_check n=536 row=decision/budget-override-per-instance sha=4c6e9fadeb82 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:270
quote: let standing = decide(reading.as_ref(), self.thresholds, override_until, now);
at: src/run/budget.rs:317
quote: override_until: Some(end.clone()),
guard_at: crates/viola-e2e/tests/path6_budget_harness.rs:128
@row_add n=537 row=contract/release-verb/driver-caller area=e2e-test-strategy rec=fct-kn7ryhs7@2
boundary: release verb
item: driver caller
promise: A driver calling release is refused with -32602, exit 20.
@row_check n=537 row=contract/release-verb/driver-caller sha=a5453a1312d0 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
at: src/cmd/client.rs:244-245
quote: run::log_self_exit(20, Some("wrapper-fault")); 20
@row_add n=538 row=decision/unknown-reset-does-not-pause area=e2e-test-strategy rec=fct-kylcwwrw@2
name: Unknown or passed reset does not pause
rule: A budget window whose resets_at is "unknown", unparseable or already passed does not pause.
guard: path6_an_unknown_or_passed_reset_does_not_pause
@row_check n=538 row=decision/unknown-reset-does-not-pause sha=286a50156b5d v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:157
quote: (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
guard_at: tests/cli_budget.rs:226
@row_add n=539 row=decision/release-without-pause-records-nothing area=e2e-test-strategy rec=fct-kylcwwrw@2
name: Release with no pause records nothing
rule: release --budget with no pause standing records nothing.
guard: path6_release_budget_lifts_the_pause
@row_check n=539 row=decision/release-without-pause-records-nothing sha=318334f051c7 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:311-312
quote: let Some(crossed) = &recorded.paused else { return Ok(None);
guard_at: tests/cli_budget.rs:254
@row_add n=540 row=decision/budget-override-ends-with-wrapper area=e2e-test-strategy rec=fct-kylcwwrw@2
name: Budget override ends with its wrapper
rule: A budget override does not outlive its wrapper, so a restart over the same reading pauses again.
guard: path6_a_restart_pauses_again
@row_check n=540 row=decision/budget-override-ends-with-wrapper sha=1b8bf620f63b v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:284
quote: *recorded = self.decide_at(Trigger::Start, now, None);
guard_at: tests/cli_budget.rs:482
@row_add n=541 row=decision/budget-gate-covers-send-only area=e2e-test-strategy rec=fct-kylcwwrw@2
name: Budget gate covers send only
rule: The budget gate refuses automated send alone; answer, wait, last and pause behave under a closed gate as under an open one.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_check n=541 row=decision/budget-gate-covers-send-only sha=fa3ee4f3d981 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send.rs:105
quote: .evaluate(Trigger::Send, Utc::now())
at: src/run/send/budget.rs:11
quote: RefusalReason::BudgetPaused,
guard_at: tests/cli_budget.rs:521
@row_add n=542 row=contract/mcp-tool-result/budget-paused area=e2e-test-strategy rec=fct-kylcwwrw@2
boundary: MCP tool result
item: budget-paused
promise: A budget-paused send is isError:true with {"refusal":"budget-paused","detail":"five-hour"} and no cursor, equal to the CLI's document less v.
@row_check n=542 row=contract/mcp-tool-result/budget-paused sha=be4ef81d064e v=holds judge=sonnet head=8834195b0683
at: src/run/send/budget.rs:11
quote: RefusalReason::BudgetPaused,
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
@row_add n=543 row=contract/list-human/wheel-budget-paused area=e2e-test-strategy rec=fct-kylcwwrw@2
boundary: viola list human board
item: WHEEL cell
promise: A budget-paused instance's WHEEL cell reads "driver - budget-paused" on that instance's row alone, with no ESC byte.
@row_check n=543 row=contract/list-human/wheel-budget-paused sha=01ae5658c4a2 v=holds judge=sonnet head=8834195b0683
at: src/human/board/cells.rs:18
quote: format!("{holder} - budget-paused")
@row_add n=544 row=decision/unverified-cli-gates-answer-only area=e2e-test-strategy rec=fct-5mlkcfqk@1
name: Unverified CLI version gates answer only
rule: Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
@row_check n=544 row=decision/unverified-cli-gates-answer-only sha=8491347dba26 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/run/send.rs:111
quote: let confirms = classify(text, slot.cli_verified);
@row_add n=545 row=contract/cli-answer/unverified-cli area=e2e-test-strategy rec=fct-5mlkcfqk@1
boundary: viola answer
item: unverified-cli
promise: answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
@row_check n=545 row=contract/cli-answer/unverified-cli sha=13dc5ac67dac v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
@row_add n=546 row=contract/mcp-tool-result/unverified-cli area=e2e-test-strategy rec=fct-5mlkcfqk@1
boundary: MCP tool result
item: unverified-cli
promise: MCP answer on an unverified CLI version gives is_error:true with text "refused: unverified-cli".
@row_check n=546 row=contract/mcp-tool-result/unverified-cli sha=3460cb6217df v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=547 row=contract/verify-output/last-line area=e2e-test-strategy rec=fct-5mlkcfqk@1
boundary: viola verify output
item: last line
promise: A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
@row_check n=547 row=contract/verify-output/last-line sha=8d2dccdeabc8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:294
quote: "stamped {version}  {} pass  {failed} fail",
at: src/cmd/verify.rs:302
quote: None => Ok(Ended::Exit(0)),
@row_add n=548 row=decision/claude-child-env-strip area=e2e-test-strategy rec=fct-ja62ftpj@3,fct-blbpbqaj@1
name: claude child environment strip
rule: The claude child receives none of the identity-floor names nor unknown CLAUDE_ names; only names on claude_env_keep survive.
guard: tui_env_strip
@row_check n=548 row=decision/claude-child-env-strip sha=c225d86aaacc v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:69
quote: if !text.starts_with("CLAUDE") {
at: crates/viola-agent-claude/src/lib.rs:73
quote: if !floor && persistent.contains(&text) {
guard_at: tests/tui_env_strip.rs:86
@row_add n=549 row=contract/process-start-claude-child/env-stripped-count area=e2e-test-strategy rec=fct-ja62ftpj@3
boundary: process-start{subject:"claude-child"} record
item: env_stripped_count
promise: Carries the count of environment names stripped from the claude child.
@row_check n=549 row=contract/process-start-claude-child/env-stripped-count sha=0ab2e15a4702 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:57
quote: env_stripped_count = strip.count(),
@row_add n=550 row=contract/process-start-claude-child/env-stripped-known area=e2e-test-strategy rec=fct-ja62ftpj@3
boundary: process-start{subject:"claude-child"} record
item: env_stripped_known
promise: Carries the known stripped environment names, names only, never values.
@row_check n=550 row=contract/process-start-claude-child/env-stripped-known sha=094d327c455b v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:58
quote: env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
@row_add n=551 row=contract/process-start-claude-child/env-kept area=e2e-test-strategy rec=fct-ja62ftpj@3
boundary: process-start{subject:"claude-child"} record
item: env_kept
promise: Carries the environment names kept for the claude child, names only, never values.
@row_check n=551 row=contract/process-start-claude-child/env-kept sha=a993043b0a9c v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:59
quote: env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()),
@row_add n=552 row=decision/claude-child-holds-nothing-of-viola area=e2e-test-strategy rec=fct-ja62ftpj@3
name: claude child holds nothing of viola's
rule: The wrapped claude child holds no viola socket and no path under the viola home, and its fds 0 to 2 are the PTY.
guard: tui_channel_fds
@row_check n=552 row=decision/claude-child-holds-nothing-of-viola sha=37bcff4a5a48 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/tui_channel_fds.rs:97
quote: !shown.starts_with("socket:"),
at: tests/tui_channel_fds.rs:104-105
quote: let viola_file = target.starts_with(&home) && !fixtures.contains(target); assert!(!viola_file, "fd {fd} holds a viola home file: {table:?}");
guard_at: tests/tui_channel_fds.rs:19
@row_add n=553 row=decision/secrets-never-in-logs area=e2e-test-strategy rec=fct-ja62ftpj@3,fct-iplgfuqy@2
name: Secrets never in logs or diagnostics
rule: No stripped-variable value and no Cookie header appears in any log or diagnostics file.
@row_check n=553 row=decision/secrets-never-in-logs sha=7bbf12dea658 v=holds judge=sonnet head=8834195b0683
searched: Cookie|cookie · in src/**/*.rs crates/*/src/**/*.rs (only the harness secret scan, crates/viola-e2e/src/harness/secret_scan.rs:129-130, which looks for it; no product code writes one)
searched: env_stripped_count · in crates/viola-pty/src/lib.rs (the span carries a count of removed names, line 128, never a value)
@row_add n=554 row=decision/link-driver-self-reported area=e2e-test-strategy rec=fct-53es7vlo@2
name: Link driver is a self-reported name
rule: The driver of a link is a self-reported name and need not be a booted instance.
@row_check n=554 row=decision/link-driver-self-reported sha=54cac663cc31 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:94
quote: let (driver, driven) = (named("driver")?, named("driven")?);
at: src/run/links.rs:95-96
quote: if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
@row_add n=555 row=decision/send-from-another-instance-links-once area=e2e-test-strategy rec=fct-53es7vlo@2
name: Send from another instance links once
rule: An issued send from another instance appends one link{driver, driven} record with source wrapper before its send-issued, and a later send of the same pair adds none.
guard: links_a_send_from_another_instance_links_once
@row_check n=555 row=decision/send-from-another-instance-links-once sha=a2c5d7496d23 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send.rs:145
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/links.rs:62-64
quote: if let Some(since) = self.since(&set, driver) {             return Ok((since, true));         }
guard_at: tests/cli_links.rs:106
@row_add n=556 row=decision/refused-or-own-send-records-no-link area=e2e-test-strategy rec=fct-53es7vlo@2
name: Refused or own-name send records no link
rule: A refused send and a send to an instance's own name record no link.
guard: links_a_refused_send_and_an_own_name_send_record_none
@row_check n=556 row=decision/refused-or-own-send-records-no-link sha=a566199959ac v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send.rs:142-143
quote: if let (Some(links), Some(from)) = (slot.links.get(), ctx.from.as_ref())         && from != name
guard_at: tests/cli_links.rs:189
@row_add n=557 row=contract/channel-link/already-linked area=e2e-test-strategy rec=fct-53es7vlo@2
boundary: wrapper link method
item: already_linked
promise: A second link of an already linked pair appends no record, keeps the first since, and answers already_linked: true.
@row_check n=557 row=contract/channel-link/already-linked sha=d49bcb42dfec v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62-63
quote: if let Some(since) = self.since(&set, driver) { return Ok((since, true));
at: src/run/links.rs:75
quote: Ok(json!({"ok": {"since": since, "already_linked": already_linked}}))
@row_add n=558 row=contract/channel-unlink/unlink-record area=e2e-test-strategy rec=fct-53es7vlo@2
boundary: wrapper unlink method
item: unlink record
promise: unlink appends one unlink record (source: wrapper, {driver, driven}) and empties the snapshot's links; a second unlink appends nothing, and a later link is a new pair with a later since.
@row_check n=558 row=contract/channel-unlink/unlink-record sha=3f7118f904c6 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:105-109
quote: if self.since(&set, driver).is_none() { return Ok(false); } let line = self.line(EventKind::Unlink, driver); self.change(&mut set, &line)?;
at: src/run/links.rs:134-135
quote: self.snapshots .update(|snapshot| snapshot.links.clone_from(set))
@row_add n=559 row=decision/links-survive-wrapper-restart area=e2e-test-strategy rec=fct-53es7vlo@2,fct-3svwp7h4@1
name: Links survive a wrapper restart
rule: A wrapper started over a log restores the link set from it, an unlinked pair restored as absent, writes that set into its first snapshot and appends no line.
guard: links_survive_a_wrapper_restart, links_an_unlink_survives_a_wrapper_restart
@row_check n=559 row=decision/links-survive-wrapper-restart sha=fd28b3ed90e6 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:333
quote: links.restore()?;
at: src/cmd/run.rs:377
quote: links: links.current(),
at: crates/viola-state/src/replay.rs:94-95
quote: (Some(EventKind::Unlink), Some(at)) => {             links.remove(at);
guard_at: tests/cli_links.rs:224
@row_add n=560 row=decision/replay-writes-no-file area=e2e-test-strategy rec=fct-3svwp7h4@1
name: Replay writes no file
rule: The snapshot replay writes no file, so no snapshot is rebuilt from it.
@row_check n=560 row=decision/replay-writes-no-file sha=17ecf372546c v=holds judge=sonnet head=8834195b0683
searched: fs::write|File::create|OpenOptions|persist|replace_private|write_all|create_dir · in crates/viola-state/src/replay.rs (non-test part, lines 1-182; matches only from line 372, inside cfg(test))
@row_add n=561 row=decision/replayed-snapshot-has-no-endpoint area=e2e-test-strategy rec=fct-3svwp7h4@1
name: Replayed snapshot has no endpoint or cwd
rule: A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing.
guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
@row_check n=561 row=decision/replayed-snapshot-has-no-endpoint sha=d05834f4b593 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
at: src/cmd/revive.rs:264
quote: Recovered::Absent | Recovered::Replayed { .. } => None,
guard_at: crates/viola-state/src/liveness.rs:267
@row_add n=562 row=decision/collision-check-over-replay-reads-beat area=e2e-test-strategy rec=fct-3svwp7h4@1
name: Collision check over a replay reads the beat
rule: Over a replayed snapshot the start's collision check reads the heartbeat alone: a beat no older than STALE_AFTER is already-live, an older one leaves the name free.
guard: collision_check_over_a_replayed_snapshot_reads_the_beat_alone
@row_check n=562 row=decision/collision-check-over-replay-reads-beat sha=c6a4372fcb5e v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age()             .is_some_and(|age| age <= STALE_AFTER)             .then_some(Liveness::Live),
guard_at: src/cmd/run.rs:1162
@row_add n=563 row=decision/list-reads-live-through-replay area=e2e-test-strategy rec=fct-3svwp7h4@1
name: list reads a live wrapper through the replay
rule: list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line.
guard: e5_list_reads_a_live_wrapper_through_the_replay
@row_check n=563 row=decision/list-reads-live-through-replay sha=647820dd885f v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:286
quote: if beat_age(dir, now).is_some_and(|age| age <= STALE_AFTER) {
at: crates/viola-state/src/board.rs:290
quote: liveness: RowLiveness::Live,
at: crates/viola-state/src/replay.rs:165
quote: ObsEvent::StateRecovered,
guard_at: tests/cli_list.rs:221
@row_add n=564 row=contract/event-log/start-wheel-record area=e2e-test-strategy rec=fct-3svwp7h4@1
boundary: instance event log
item: first record of a start
promise: Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} as its first record.
@row_check n=564 row=contract/event-log/start-wheel-record sha=eaa7fe884582 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:620
quote: json!({"holder": Wheel::Driver.as_str(), "cause": WheelCause::Start.as_str()}),
@row_add n=565 row=decision/security-sweep-under-nextest area=e2e-test-strategy rec=fct-fbkvffnw@2
name: Security sweep runs under nextest alone
rule: The security sweep runs under nextest alone.
@row_check n=565 row=decision/security-sweep-under-nextest sha=d34a959d781c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/nextest.rs:61
quote: "nextest",
at: .github/workflows/ci.yml:38
quote: run: bash scripts/agent-run.sh run --coverage
@row_add n=566 row=decision/hook-instance-check-before-config area=e2e-test-strategy rec=fct-fbkvffnw@2
name: Hook instance check before any config read
rule: The hook's event and dialog paths take the strict-modes instance check before any config.json read or observability init.
guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
except: The two --capture arms stay ahead of the check.
@row_check n=566 row=decision/hook-instance-check-before-config sha=75172166d68d v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:119
quote: let (level, _) = obs::read_diagnostics_level(&instance.home);
at: src/cmd/hook.rs:101
quote: if let Some(dir) = &args.capture {
guard_at: tests/hook_fail_open.rs:465
@row_add n=567 row=decision/fake-agent-panic-seam-test-only area=e2e-test-strategy rec=fct-fbkvffnw@2
name: Forced-panic seam is test-only
rule: The hook forced-panic seam fires only on exactly 1 and compiles only with feature fake-agent, so a release build of viola never contains it.
why: Ratified in the security-plan Decisions Log.
guard: cli_controls_not_disableable
@row_check n=567 row=decision/fake-agent-panic-seam-test-only sha=c2c05325b88d v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook/seam.rs:18
quote: if value.as_deref() == Some(OsStr::new("1")) {
at: src/cmd/hook.rs:128-129
quote: #[cfg(feature = "fake-agent")]     seam::panic_if_asked();
guard_at: tests/cli_controls_not_disableable.rs:119
@row_add n=568 row=decision/hook-panic-payload-to-detail-file area=e2e-test-strategy rec=fct-fbkvffnw@2
name: Hook panic payload kept to the detail file
rule: A caught hook panic writes one payload-free role panic line, and its payload and backtrace go to one detail line only.
guard: hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line
@row_check n=568 row=decision/hook-panic-payload-to-detail-file sha=4ebb85b14ff7 v=holds guard=true judge=sonnet head=8834195b0683
at: src/main.rs:191
quote: let _ = (&*sink.file).write_all(line.as_bytes());
at: src/main.rs:220
quote: fields.insert("panic_payload".to_owned(), payload.into());
guard_at: tests/hook_fail_open.rs:649
@row_add n=569 row=contract/caught-panic-exit-code/hook area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: exit code on a caught panic
item: hook
promise: hook exits 0 on a caught panic.
@row_check n=569 row=contract/caught-panic-exit-code/hook sha=46012710cc4f v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=570 row=contract/caught-panic-exit-code/run area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: exit code on a caught panic
item: run
promise: run exits 1 on a caught panic.
@row_check n=570 row=contract/caught-panic-exit-code/run sha=0bf3e1176835 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:136-138
quote: Role::Mcp | Role::Cli | Role::Other, Outcome::Unparsed | Outcome::Failed | Outcome::Panicked, ) => ExitCode::from(1),
@row_add n=571 row=contract/mcp-tool-input/schema-violation area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: MCP tool input
item: schema violation
promise: A schema-violating tool call is invalid-params naming its field, revalidated before any frame, with no send-issued.
@row_check n=571 row=contract/mcp-tool-input/schema-violation sha=a8917e42b5e2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:372
quote: (Err(field), _) | (_, Err(field)) => return Ok(result::invalid(field)),
at: crates/viola-mcp/src/tools.rs:418
quote: Err(field) => return Ok(result::invalid(field)),
@row_add n=572 row=contract/mcp-tool-input/empty-or-control-text area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: MCP tool input
item: empty or control-character text
promise: A send text that is empty or holds a control character is not-delivered.
@row_check n=572 row=contract/mcp-tool-input/empty-or-control-text sha=d1667b64e704 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:374-375
quote: if let Some(detail) = refusal_of(text) { return Ok(refused_client_side(detail));
at: crates/viola-mcp/src/tools.rs:297-302
quote: if let Err(detail) = viola_core::validate_paste_text(text) { return Some(detail); } typed_text(text) .is_empty() .then_some(NotDelivered::EmptyText)
@row_add n=573 row=contract/hook-decision/server-verify-failed area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: hook-decision record
item: server-verify-failed
promise: A hook facing a server that is not the recorded wrapper logs one hook-decision line with detail "server-verify-failed", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
@row_check n=573 row=contract/hook-decision/server-verify-failed sha=afcfe0e9f404 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:214
quote: Self::ServerUnverified => "server-verify-failed",
at: src/cmd/hook.rs:296
quote: invoked(event, &invoked_at, dialog_id);
at: src/cmd/hook.rs:389-396
quote: Some(detail) => obs_event!( WARN, ObsEvent::HookDecision, hook_event = event.as_str(), decision_emitted = emitted, deadline_hit = deadline_hit, duration_ms = duration_ms, detail = detail,
@row_add n=574 row=contract/hook-decision/socket-dir-refused area=e2e-test-strategy rec=fct-fbkvffnw@2
boundary: hook-decision record
item: socket-dir-refused
promise: A refused socket directory logs one hook-decision line at WARN with detail "socket-dir-refused", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
@row_check n=574 row=contract/hook-decision/socket-dir-refused sha=2d0e3a524df0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:231-232
quote: if check_endpoint_dir(endpoint).is_err() { return Err(NotSent::SocketDir);
at: src/cmd/hook.rs:215
quote: Self::SocketDir => UnreachableCause::SocketDir.as_str(),
at: src/cmd/hook.rs:389-390
quote: Some(detail) => obs_event!( WARN,
@row_add n=575 row=decision/error-bodies-held-by-surface-witnesses area=e2e-test-strategy rec=fct-iplgfuqy@2
name: Error bodies held by surface witnesses
rule: No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
@row_check n=575 row=decision/error-bodies-held-by-surface-witnesses sha=4e8acdc80c91 v=holds judge=sonnet head=8834195b0683
searched: assert_error_body_clean|error_body_hit · in tests/**/*.rs, crates/**/*.rs, src/**/*.rs (matches only in tests/support/events.rs, tests/channel_error_bodies.rs, tests/cli_error_bodies.rs; no helper run after every E2E test)
@row_add n=576 row=contract/cli-json/internal-error area=e2e-test-strategy rec=fct-iplgfuqy@2
boundary: CLI --json document
item: internal-error
promise: Under --json an internal error prints {"v":1,"error":"internal-error","detail":null} on stdout and leaves stderr empty.
@row_check n=576 row=contract/cli-json/internal-error sha=3112534b537a v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67-68
quote: if json_asked(std::env::args_os()) { cmd::internal_error_document();
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/main.rs:69-70
quote: } else { human::internal_error();
@row_add n=577 row=contract/cli-human/internal-error area=e2e-test-strategy rec=fct-iplgfuqy@2
boundary: CLI human output
item: internal error line
promise: Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, mcp and hook stay silent there.
@row_check n=577 row=contract/cli-human/internal-error sha=c6317bab07cd v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/main.rs:66-70
quote: if prints_internal_error(role, outcome) {         if json_asked(std::env::args_os()) {             cmd::internal_error_document();         } else {             human::internal_error();
@row_add n=578 row=contract/channel-error/-32603 area=e2e-test-strategy rec=fct-iplgfuqy@2
boundary: channel error
item: -32603
promise: Every -32603 is exactly "internal error" with data null.
@row_check n=578 row=contract/channel-error/-32603 sha=fdcc80752878 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
at: crates/viola-channel/src/lib.rs:97
quote: _ => Value::Null,
@row_add n=579 row=decision/internal-error-held-at-unit-layer area=e2e-test-strategy rec=fct-iplgfuqy@2
name: -32603 held at the unit layer
rule: No -32603 is driven in a scanned home and no failpoint or counted panic stands for one; the unit layer holds it.
guard: answer_a_panicking_dispatch_is_internal_error_and_closes
@row_check n=579 row=decision/internal-error-held-at-unit-layer sha=4820a0c9c637 v=holds guard=true judge=sonnet head=8834195b0683
searched: failpoint|fail_point|fail::|FAILPOINT|PANIC_AFTER|panic_after|PANIC_COUNT · in tests/**/*.rs, crates/**/*.rs, src/**/*.rs
searched: -32603 · in tests/ (only a doc comment and error-body literals; none drives one in a scanned home)
guard_at: crates/viola-channel/src/server.rs:677
@row_add n=580 row=decision/diagnostics-files-0600 area=e2e-test-strategy rec=fct-iplgfuqy@2
name: Diagnostics files are owner-only
rule: diagnostics/ files are mode 0600.
@row_check n=580 row=decision/diagnostics-files-0600 sha=9072e7adc624 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:92
quote: Ok(Arc::new(open_private_append(&path)?))
at: crates/viola-state/src/fs.rs:48
quote: std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
@row_add n=581 row=decision/synthetic-inputs-with-canary area=e2e-test-strategy rec=fct-iplgfuqy@2
name: Synthetic test inputs carry a canary
rule: Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
@row_check n=581 row=decision/synthetic-inputs-with-canary sha=2f2372e37b79 v=holds judge=sonnet head=8834195b0683
at: tests/support/verify.rs:24
quote: pub const CANARY: &str = "canary-chain-value-5c1e";
at: crates/viola-e2e/src/harness/secret_scan.rs:33
quote: pub const CONTENT: [(&str, &str); 1] = [("content-canary", "canary-chain-value-5c1e")];
at: crates/viola-e2e/src/harness/secret_scan.rs:133
quote: for (class, needle) in CONTENT {
@row_add n=582 row=decision/scan-hits-never-print-bytes area=e2e-test-strategy rec=fct-iplgfuqy@2
name: Scan hits never print matched bytes
rule: A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes.
guard: error_body_check_reports_no_byte_of_what_it_read
@row_check n=582 row=decision/scan-hits-never-print-bytes sha=29a9f1b9a01e v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
guard_at: tests/cli_error_bodies.rs:399
@row_add n=583 row=decision/ci-scan-before-upload area=e2e-test-strategy rec=fct-iplgfuqy@2
name: CI secret scan before any upload
rule: The canary scan runs in CI after each job's end-to-end run over the kept test homes, the harness capture and the junit report, before any upload.
except: The harness chunk.diff and the viola-chaos-* home are out of the scan.
@row_check n=583 row=decision/ci-scan-before-upload sha=bcf9304e95b0 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:76
quote: run: bash scripts/agent-run.sh secret-scan
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: crates/viola-e2e/src/harness/secret_scan.rs:80
quote: capture.retain(|path| *path != mutation_diff);
at: tests/support/home.rs:216
quote: .prefix("viola-chaos-")
@row_add n=584 row=decision/each-exit-cause-own-hint area=e2e-test-strategy rec=fct-jaqoqtdh@1
name: Each exit cause has its own hint
rule: Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none.
guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
@row_check n=584 row=decision/each-exit-cause-own-hint sha=a0a4bdf8c1a6 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cross_exit_causes.rs:116
quote: fn no_two_codes_share_a_hint() {
at: tests/cross_exit_causes.rs:91
quote: ("internal-error", 1, None),
guard_at: tests/cross_exit_causes.rs:116
@row_add n=585 row=contract/exit-21-detail/strict-modes-failed area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: strict-modes-failed
promise: The home or instance fails strict-modes (a widened mode); refused before any log, frame or line.
@row_check n=585 row=contract/exit-21-detail/strict-modes-failed sha=b681e00939a1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
@row_add n=586 row=contract/exit-21-detail/instance-dead area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: instance-dead
promise: No instance is running for the name with its directory standing, a stale wrapper included; during connect.
@row_check n=586 row=contract/exit-21-detail/instance-dead sha=bc1b0c15266f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:112
quote: Ok(metadata) if metadata.is_dir() => UnreachableCause::InstanceDead,
at: crates/viola-state/src/liveness.rs:97
quote: if classify(age, same_process(&snapshot)) != Liveness::Live {
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=587 row=contract/exit-21-detail/unwrapped-name area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: unwrapped-name
promise: The name holds no instance directory (an unwrapped session or a name never started); its own hint; during connect.
@row_check n=587 row=contract/exit-21-detail/unwrapped-name sha=6c6ceadbf868 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:113
quote: _ => UnreachableCause::UnwrappedName,
at: src/cmd/client.rs:145
quote: Self::Unwrapped => "unwrapped-name",
@row_add n=588 row=contract/exit-21-detail/endpoint-lost area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: endpoint-lost
promise: The wrapper was lost with the call open; its own hint; during call.
@row_check n=588 row=contract/exit-21-detail/endpoint-lost sha=191f944efad6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => { return Ok(Err(Unreachable::Lost));
at: src/cmd/client.rs:136
quote: Self::Lost => "call",
@row_add n=589 row=contract/exit-21-detail/server-verify-failed area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: server-verify-failed
promise: The listening server is not the recorded wrapper (pid or start time mismatch, or another euid); its own hint; logged as process-exit at WARN; during connect.
@row_check n=589 row=contract/exit-21-detail/server-verify-failed sha=76cfe764ece3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:256-263
quote: obs_event!( WARN, ObsEvent::ProcessExit, subject = "self", exit_code = 21u8, detail = cause.detail(), during = cause.during(), duration_ms = obs::duration_ms(),
@row_add n=590 row=contract/exit-21-detail/socket-dir-refused area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: exit 21 instance-unreachable detail
item: socket-dir-refused
promise: The socket directory is refused, on the eight frame verbs alone; its own hint; process-exit at WARN with no channel-request; during connect.
@row_check n=590 row=contract/exit-21-detail/socket-dir-refused sha=01f3a8f09991 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
@row_add n=591 row=contract/cli-json/instance-unreachable area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: CLI --json document
item: instance-unreachable
promise: Exit 21 prints {"v":1,"error":"instance-unreachable","detail":"<cause>"}; MCP's text is "refused: instance-unreachable".
@row_check n=591 row=contract/cli-json/instance-unreachable sha=7e1eb3ca8249 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:265
quote: 21
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=592 row=contract/start-exit-1/already-live area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: already-live
promise: The name is held by a live wrapper or a fresh replayed beat; the hint is viola list.
@row_check n=592 row=contract/start-exit-1/already-live sha=2f03b1372fa9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:716
quote: human::refuse(&format!("{name} is already live"), "viola list");
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
@row_add n=593 row=contract/start-exit-1/stale-heartbeat area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: stale-heartbeat
promise: The holder's pid is live but its heartbeat is older than 5 s; stale heartbeat is exit 1's cause alone.
@row_check n=593 row=contract/start-exit-1/stale-heartbeat sha=9b8b41b89ec5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
at: crates/viola-state/src/liveness.rs:97
quote: if classify(age, same_process(&snapshot)) != Liveness::Live {
@row_add n=594 row=contract/start-exit-1/squatted-name area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: squatted-name
promise: The endpoint name is squatted.
@row_check n=594 row=contract/start-exit-1/squatted-name sha=f3efcc98bba2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/run.rs:502
quote: Err(ChannelError::BindTaken) => Ok(Bound::Taken),
@row_add n=595 row=contract/start-exit-1/pinned-hash-mismatch area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: pinned-hash-mismatch
promise: The pinned executable fails its SHA-256 re-hash.
@row_check n=595 row=contract/start-exit-1/pinned-hash-mismatch sha=1f69cb6567c5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:467
quote: Err(PinError::HashMismatch) => {
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=596 row=contract/start-exit-1/strict-modes-failed area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: strict-modes-failed
promise: The home or instance state could be written, or read, by another user; no log opened, no line, no child.
@row_check n=596 row=contract/start-exit-1/strict-modes-failed sha=727e320dbed2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/cmd/revive.rs:138
quote: Refusal::StrictModes.write(&args.name, args.json);
@row_add n=597 row=contract/start-exit-1/socket-dir-refused area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: run and revive exit 1 causes
item: socket-dir-refused
promise: The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
@row_check n=597 row=contract/start-exit-1/socket-dir-refused sha=1ecb032fe15e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:497-498
quote: if !socket_dir_ready(&endpoint) { return Ok(Bound::SocketDirRefused);
at: src/cmd/run.rs:518
quote: (!absent || create_private_dir(dir).is_ok()) && check_endpoint_dir(endpoint).is_ok()
at: src/cmd/run.rs:424
quote: refused(detail)
@row_add n=598 row=contract/revive-exit-1/no-session area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: revive exit 1 causes
item: no-session
promise: No logged session for the name, or an --id the log does not hold.
@row_check n=598 row=contract/revive-exit-1/no-session sha=02fbd561614f v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:249
quote: Some(asked) => ids.find(|id| *id == asked).ok_or(Refusal::UnknownId),
at: src/cmd/revive.rs:250
quote: None => ids.next_back().ok_or(Refusal::NoSession),
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=599 row=contract/revive-exit-1/session-live area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: revive exit 1 causes
item: session-live
promise: One claude agents --json read lists the logged session live in another claude; no child starts.
@row_check n=599 row=contract/revive-exit-1/session-live sha=a2a520abe168 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:223
quote: |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
at: src/cmd/revive.rs:226-229
quote: let ((id, _), spawn_dir) = match passed { Ok(passed) => passed, Err(refusal) => return Ok(Err(refusal)), };
@row_add n=600 row=contract/revive-exit-1/cwd-missing area=e2e-test-strategy rec=fct-jaqoqtdh@1
boundary: revive exit 1 causes
item: cwd-missing
promise: The recorded directory is gone, or the snapshot holds no cwd.
@row_check n=600 row=contract/revive-exit-1/cwd-missing sha=bae6d331febc v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:259-263
quote: Recovered::Snapshot(snapshot) => snapshot .cwd .as_deref() .map(PathBuf::from) .filter(|dir| dir.is_dir()),
at: src/cmd/revive.rs:264
quote: Recovered::Absent | Recovered::Replayed { .. } => None,
@row_add n=601 row=decision/revive-preflight-order area=e2e-test-strategy rec=fct-jaqoqtdh@1
name: revive preflight order
rule: revive takes the strict-modes check first, then holder, session, session-live and recorded directory in that order, a later reading not taken once an earlier one refuses.
@row_check n=601 row=decision/revive-preflight-order sha=4b1a1c62f20c v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=602 row=decision/session-live-read-fails-open area=e2e-test-strategy rec=fct-jaqoqtdh@1
name: session-live reading fails open
rule: revive --fork skips the session-live reading, and a failed claude agents read passes it.
guard: revive_passes_the_session_live_reading, revive_whose_agents_read_fails_starts
@row_check n=602 row=decision/session-live-read-fails-open sha=65b4928faa51 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:223
quote: |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
at: src/cmd/revive.rs:193
quote: .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
guard_at: tests/cli_revive.rs:426
@row_add n=603 row=decision/revive-json-no-document-after-terminal area=e2e-test-strategy rec=fct-jaqoqtdh@1
name: No revive document once the child holds the terminal
rule: A pump_child error under revive --json prints no document.
why: The child has held the terminal.
@row_check n=603 row=decision/revive-json-no-document-after-terminal sha=c3f560aa5140 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:122
quote: let started = started(home, args).inspect_err(|_| {
at: src/cmd/revive.rs:128
quote: Started::Launched(launched) => pump_child(*launched),
@row_add n=604 row=decision/strict-modes-before-any-log area=e2e-test-strategy rec=fct-jaqoqtdh@1,fct-hhwc4gim@2
name: Strict-modes check before any read or open
rule: Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found.
why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700.
guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
@row_check n=604 row=decision/strict-modes-before-any-log sha=0a907ef0b145 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/revive.rs:137
quote: if !trusted(home, &args.name)? {
guard_at: tests/cli_links.rs:531
@row_add n=605 row=decision/one-negative-per-security-control area=e2e-test-strategy rec=fct-hhwc4gim@2
name: One negative test per security control
rule: Every security-plan control has a negative test of its own.
@row_check n=605 row=decision/one-negative-per-security-control sha=6ae91ffaaa48 v=unverifiable judge=sonnet head=8834195b0683
searched: security_negatives_ · in tests/*.rs
why: not a code claim
@row_add n=606 row=decision/socket-dir-check-never-chmods area=e2e-test-strategy rec=fct-hhwc4gim@2
name: Socket directory refused, never repaired
rule: The per-user socket directory is refused when it is a symlink, wider than 0700, not a directory or of another owner, and the check never chmods it.
guard: security_negative_a_start_refuses_the_socket_directory_before_any_bind
@row_check n=606 row=decision/socket-dir-check-never-chmods sha=141d8b9ac779 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:168-174
quote: if is_symlink { return Err(Refused::Symlink); } if !is_dir { return Err(Refused::NotADirectory); } unix_read_verdict(mode, uid, euid)
at: crates/viola-state/src/strict.rs:229
quote: if mode & 0o077 != 0 {
at: crates/viola-state/src/strict.rs:216
quote: if uid != euid {
guard_at: tests/security_negatives_socket_dir.rs:205
@row_add n=607 row=contract/mcp-frame/socket-dir-refused area=e2e-test-strategy rec=fct-hhwc4gim@2
boundary: MCP frame
item: socket-dir-refused
promise: A refused socket directory gives instance-unreachable with no line at MCP's frame.
@row_check n=607 row=contract/mcp-frame/socket-dir-refused sha=b72d8208f5a5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=608 row=contract/channel-link-names/invalid-name area=e2e-test-strategy rec=fct-hhwc4gim@2
boundary: channel link and unlink methods
item: invalid name
promise: A driver or driven that is not a valid name, a driven other than the serving wrapper's own name, or a driver equal to driven gets -32602 with data null and appends no record.
@row_check n=608 row=contract/channel-link-names/invalid-name sha=0c0212e74e69 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:91
quote: .and_then(|name| ViolaName::try_new(name.to_owned()).ok())
at: src/run/links.rs:95
quote: if driven != self.name || driver == driven {
at: src/run/links.rs:96
quote: return Err(ProtocolError::InvalidParams);
at: src/run/links.rs:73
quote: let driver = self.driver_of(params)?;
@row_add n=609 row=decision/home-readable-by-other-refused area=e2e-test-strategy rec=fct-hhwc4gim@2,fct-jaqoqtdh@1
name: Home another user may read is refused
rule: A pre-existing home another user may read is refused at run with exit 1.
guard: a_home_another_user_may_read_refuses_run
@row_check n=609 row=decision/home-readable-by-other-refused sha=d8c91d55a5d2 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:89
quote: check_existing(&[(home.to_path_buf(), Rule::Read)]).map_err(TreeRefused::HomeLevel)?;
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
guard_at: tests/security_negatives_strict_modes.rs:385
@row_add n=610 row=decision/single-writers area=e2e-test-strategy rec=fct-hhwc4gim@2
name: Single writers of snapshot and stamps
rule: Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json.
guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
@row_check n=610 row=decision/single-writers sha=7ecdd26ec4cb v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/snapshot.rs:47
quote: write_snapshot(&self.instance_dir, snapshot)
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
guard_at: tests/security_negatives_strict_modes.rs:251
@row_add n=611 row=decision/client-verifies-server-identity area=e2e-test-strategy rec=fct-hhwc4gim@2
name: Client verifies the server before any frame
rule: A CLI frame verb writes no frame to a server that is not the recorded wrapper and names no pid, start time, path or endpoint in its output.
guard: security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
@row_check n=611 row=decision/client-verifies-server-identity sha=b3a44bc24589 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177-184
quote: if !peer_admitted(client.server_euid()) { return Err(CallError::ServerUnverified); } let verdict = verify_server(live.pid, &live.started_at, client.server_pid()); match verdict { ServerVerdict::Matches => Ok(client), ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
guard_at: tests/security_negatives_server_identity.rs:200
@row_add n=612 row=decision/list-rows-escape-control-chars area=e2e-test-strategy rec=fct-hdfefjry@2
name: list rows escape control characters
rule: viola list never prints raw newline, tab or C1 bytes in a NAME cell, so each row stays one line and every column starts at its caption offset in every mode.
guard: cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode
@row_check n=612 row=decision/list-rows-escape-control-chars sha=e7134687c2bd v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/escape.rs:23
quote: if c.is_control() {
at: src/human/board/rows.rs:26
quote: let names: Vec<String> = items.iter().map(|item| escape_row(&item.name)).collect();
guard_at: tests/cross_list_rows.rs:76
@row_add n=613 row=contract/list-human/columns area=e2e-test-strategy rec=fct-hdfefjry@2
boundary: viola list human board
item: columns
promise: The caption row is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the table is fixed-width.
@row_check n=613 row=contract/list-human/columns sha=18f381792043 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:42
quote: [NAME_CAPTION, "LIVE", "STATUS", "WHEEL", DIALOG_WORD, "CLI"],
at: src/human/board/cells.rs:95
quote: "{open}{shown_name}{}  {live:<LIVE_WIDTH$}  {status:<STATUS_WIDTH$}  {wheel:<WHEEL_WIDTH$}  \
@row_add n=614 row=contract/list-human/unwrapped-separator area=e2e-test-strategy rec=fct-hdfefjry@2
boundary: viola list human board
item: unwrapped separator
promise: Unwrapped sessions follow the separator line "-- UNWRAPPED - READ-ONLY --".
@row_check n=614 row=contract/list-human/unwrapped-separator sha=fd6b66bed017 v=holds judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:80
quote: const UNWRAPPED_SEPARATOR: &str = "-- UNWRAPPED - READ-ONLY --";
at: src/human/board/rows.rs:100-101
quote: lines.push(UNWRAPPED_SEPARATOR.to_owned());         lines.extend(rows);
@row_add n=615 row=decision/stale-row-dim area=e2e-test-strategy rec=fct-3fk5wzsy@2
name: Stale row shown dim
rule: On a terminal a stale row is dim from first byte to last with its NAME dim and bold, shows stale, and no SGR touches the caption or separator.
guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
@row_check n=615 row=decision/stale-row-dim sha=0d2c06d8cefb v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/board/rows.rs:60
quote: stale: item.wrapped && item.liveness == RowLiveness::Stale,
at: src/human/board/cells.rs:76
quote: let base = if spans.stale { tokens.stale } else { "" };
guard_at: tests/cross_list_rows.rs:266
@row_add n=616 row=decision/torn-append-heals area=e2e-test-strategy rec=fct-nhle4u5a@1
name: Torn append heals on the next start
rule: A torn last log line is counted in skipped.torn_lines, the next append starts on a fresh line with every earlier byte unchanged, and one state-recovered line is logged.
guard: chaos_torn_append
@row_check n=616 row=decision/torn-append-heals sha=83f64039b893 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
at: crates/viola-state/src/events.rs:339
quote: detail = "torn-line-healed",
guard_at: tests/chaos_torn_append.rs:50
@row_add n=617 row=decision/torn-append-by-truncation area=e2e-test-strategy rec=fct-nhle4u5a@1
name: Torn append simulated by truncation
rule: The torn-append chaos case stops the wrapper and shortens its log in place rather than killing it mid-append.
why: The file on disk is the same, and a killed instrumented process can leave a short coverage profile.
@row_check n=617 row=decision/torn-append-by-truncation sha=1c17bf0e68b1 v=holds judge=sonnet head=8834195b0683
at: tests/chaos_torn_append.rs:62-66
quote: OpenOptions::new() .write(true) .open(&log) .expect("the log") .set_len(cut)
@row_add n=618 row=decision/exit-detected-on-handle area=e2e-test-strategy rec=fct-nhle4u5a@1
name: Child exit detected on the handle
rule: The wrapper detects the child's exit on the process handle (exit_source "handle-wait"), not on output EOF.
@row_check n=618 row=decision/exit-detected-on-handle sha=cea29115dff4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:169
quote: if let Some(code) = pty.try_wait()? {
at: crates/viola-pty/src/pump.rs:176
quote: source: ExitSource::HandleWait,
@row_add n=619 row=contract/list-json/liveness-stale area=e2e-test-strategy rec=fct-nhle4u5a@1
boundary: list --json
item: liveness stale
promise: A wrapper whose heartbeat is older than 5 s reads liveness "stale"; a killed one is omitted as gone.
@row_check n=619 row=contract/list-json/liveness-stale sha=9e61d34f68da v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:16
quote: pub const STALE_AFTER: Duration = Duration::from_secs(5);
at: crates/viola-state/src/liveness.rs:32-33
quote: Some(age) if age <= STALE_AFTER => Liveness::Live, _ => Liveness::Stale,
at: crates/viola-state/src/board.rs:278
quote: Liveness::Gone => None,
@row_add n=620 row=decision/vt100-panic-keeps-passthrough area=e2e-test-strategy rec=fct-nhle4u5a@1,fct-gjbygtvj@1
name: vt100 panic keeps human passthrough
rule: After a caught vt100 feed panic the screen model reads input-not-ready and drops bytes, send is refused, human passthrough still reaches the child, and the outer terminal shows no panic text.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_check n=620 row=decision/vt100-panic-keeps-passthrough sha=07bc2d26bfbd v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:96-99
quote: if let Some(parser) = &mut self.parser { parser.process(bytes); self.last_fed = now; }
at: crates/viola-agent-claude/src/screen.rs:109-111
quote: pub fn poison(&mut self) { self.parser = None; }
at: tests/chaos_feed_panic.rs:87
quote: assert_eq!(out.status.code(), Some(13));
at: tests/chaos_feed_panic.rs:105-106
quote: let receipt = fake::wait_for(&wrapper.receipt(), "the typed key", |l| { of_kind(l, "key").iter().any(|k| k["hex"] == "6b")
at: tests/chaos_feed_panic.rs:123
quote: for literal in ["panic", "parse-rejected", "vt100", "unable", "hint:"] {
guard_at: tests/chaos_feed_panic.rs:44
@row_add n=621 row=contract/cli-send/input-not-ready area=e2e-test-strategy rec=fct-nhle4u5a@1
boundary: viola send
item: input-not-ready
promise: send exits 13 with input-not-ready when the screen model is not ready.
@row_check n=621 row=contract/cli-send/input-not-ready sha=e7587d3010e0 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:129-130
quote: if readiness != Readiness::Ready { return ctx.not_delivered(None, NotDelivered::InputNotReady);
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=622 row=decision/forward-compatible-log-reading area=e2e-test-strategy rec=fct-nhle4u5a@1
name: Forward-compatible event log reading
rule: Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state.
guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
@row_check n=622 row=decision/forward-compatible-log-reading sha=27a1492b50a6 v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=623 row=contract/list-json/skipped-counts area=e2e-test-strategy rec=fct-nhle4u5a@1
boundary: list --json
item: skipped
promise: skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
@row_check n=623 row=contract/list-json/skipped-counts sha=d50e4453ba25 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
at: crates/viola-state/src/events.rs:266
quote: self.skipped.unknown_fields += 1;
@row_add n=624 row=decision/fuzzing-with-cargo-fuzz area=e2e-test-strategy rec=fct-gjbygtvj@1
name: cargo-fuzz in a separate workspace
rule: Fuzz targets live in a separate fuzz/ cargo workspace under cargo-fuzz with shared arbitrary inputs and synthetic seeds; each parser joins with its own target.
@row_check n=624 row=decision/fuzzing-with-cargo-fuzz sha=c7838e96ae67 v=holds judge=sonnet head=8834195b0683
at: fuzz/Cargo.toml:12
quote: arbitrary = "=1.4.2"
at: fuzz/Cargo.toml:20-21
quote: [workspace] members = ["."]
at: fuzz/Cargo.toml:58-60
quote: [[bin]] name = "claude_agents_json" path = "fuzz_targets/claude_agents_json.rs"
@row_add n=625 row=decision/fuzz-corpora-byte-exact area=e2e-test-strategy rec=fct-gjbygtvj@1
name: Fuzz corpora kept byte-exact
rule: Byte-sensitive fuzz corpora are marked binary in .gitattributes so their seeds stay byte-exact.
@row_check n=625 row=decision/fuzz-corpora-byte-exact sha=4951fcc6af04 v=holds judge=sonnet head=8834195b0683
at: .gitattributes:4
quote: fuzz/corpus/paste_text/** binary
at: .gitattributes:5
quote: fuzz/corpus/claude_agents_json/** binary
@row_add n=626 row=decision/vt100-fuzz-silent-panic-hook area=e2e-test-strategy rec=fct-gjbygtvj@1
name: vt100 fuzz target installs a silent panic hook
rule: The vt100_feed fuzz target installs a silent panic hook once before its first feed.
why: libfuzzer-sys's own hook aborts on every panic, a caught one included; an escaping panic still aborts through its catch.
@row_check n=626 row=decision/vt100-fuzz-silent-panic-hook sha=f82911478766 v=holds judge=sonnet head=8834195b0683
at: fuzz/fuzz_targets/vt100_feed.rs:24
quote: SILENT_HOOK.call_once(|| panic::set_hook(Box::new(|_| {})));
@row_add n=627 row=decision/statusline-proptest-whole-seconds area=e2e-test-strategy rec=fct-gjbygtvj@1
name: Statusline property draws whole seconds
rule: The statusline reader's property strategy draws whole seconds across the date range.
why: As first written the property stayed green on a stub.
@row_check n=627 row=decision/statusline-proptest-whole-seconds sha=50ed4be00c83 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/statusline.rs:452
quote: (-70_000_000_000_i64..300_000_000_000_i64).prop_map(Value::from),
@row_add n=628 row=decision/agents-proptest-reparse-oracle area=e2e-test-strategy rec=fct-gjbygtvj@1
name: agents parser oracle re-parses its input
rule: The claude agents --json property oracle compares a row against the input re-parsed from its bytes.
why: A float does not round-trip through serde_json text.
@row_check n=628 row=decision/agents-proptest-reparse-oracle sha=c766558d0097 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/agents.rs:367-368
quote: let text = doc.to_string(); let input: Value = serde_json::from_str(&text).expect("the text reads back");
@row_add n=629 row=decision/fixture-drift-byte-compare area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: Fixture drift compared byte for byte
rule: The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails.
guard: contract_fake_agent_drift
@row_check n=629 row=decision/fixture-drift-byte-compare sha=cc05b270bcb4 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:90-92
quote: let recorded = fs::read(fixtures.join(version).join(format!("{event}.default.json"))).ok(); let sent = hook["stdin_hex"].as_str().map(unhex); if recorded.is_none() || sent != recorded {
at: tests/contract_fake_agent_drift.rs:142
quote: assert!(!versions.is_empty(), "no recorded fixture set to compare");
guard_at: tests/contract_fake_agent_drift.rs:81
@row_add n=630 row=decision/pinned-exe-hash-sha2 area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: sha2 for the pinned-exe content hash
rule: The pinned-exe content hash is SHA-256 from sha2, truncated to 8 bytes as 16 hex.
guard: sha256_published_vector_matches_digest, sha256_truncated_to_eight_bytes_yields_sixteen_hex
@row_check n=630 row=decision/pinned-exe-hash-sha2 sha=0d8a182dfbae v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:42
quote: digest[..8].iter().map(|b| format!("{b:02x}")).collect()
at: crates/viola-state/src/pin.rs:47
quote: hex16(&Sha256::digest(bytes))
guard_at: tests/contract_content_hash.rs:24
@row_add n=631 row=decision/contract-tables-are-literal area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: Contract tables are literal
rule: Contract tests hold literal tables, never the product's own list.
@row_check n=631 row=decision/contract-tables-are-literal sha=aaa2df177afd v=holds judge=sonnet head=8834195b0683
at: tests/contract_bounded_readers.rs:29
quote: const BOUNDARIES: &[Boundary] = &[
at: tests/contract_bounded_readers.rs:394
quote: const EXCEPTIONS: &[(&str, &str, &str)] = &[
@row_add n=632 row=decision/no-serde-json-unbounded-depth area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: serde_json unbounded depth banned
rule: The serde_json unbounded-depth feature is banned in deny.toml.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_check n=632 row=decision/no-serde-json-unbounded-depth sha=dc2a338963a5 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:55
quote: deny = ["unbounded_depth"]
guard_at: tests/contract_bounded_readers.rs:764
@row_add n=633 row=decision/error-writers-each-named area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: Every product error writer is named
rule: Every product error writer is named in the error-body enumeration with a writer and a witness that stand.
guard: product_error_writers_are_each_named
@row_check n=633 row=decision/error-writers-each-named sha=5a9588b70f39 v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=634 row=decision/thiserror-messages-interpolate-nothing area=e2e-test-strategy rec=fct-sn6e6lxw@2
name: thiserror messages interpolate nothing
rule: thiserror error messages interpolate no value.
guard: thiserror_messages_interpolate_nothing
@row_check n=634 row=decision/thiserror-messages-interpolate-nothing sha=99bb7599278a v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_error_bodies.rs:806
quote: assert!(faults.is_empty(), "messages that interpolate: {faults:#?}");
guard_at: tests/contract_error_bodies.rs:794
@row_add n=635 row=contract/channel-error/fault-bodies area=e2e-test-strategy rec=fct-sn6e6lxw@2
boundary: channel error
item: fault bodies
promise: The channel's faults are seven literal bodies.
@row_check n=635 row=contract/channel-error/fault-bodies sha=3b1e69ceb16f v=holds judge=sonnet head=8834195b0683
at: tests/contract_error_bodies.rs:314
quote: ProtocolError::Parse,
at: tests/contract_error_bodies.rs:319
quote: ProtocolError::InvalidRequest,
at: tests/contract_error_bodies.rs:324
quote: ProtocolError::MethodNotFound,
at: tests/contract_error_bodies.rs:329
quote: ProtocolError::UnsupportedVersion,
at: tests/contract_error_bodies.rs:334
quote: ProtocolError::InvalidParams,
at: tests/contract_error_bodies.rs:339
quote: ProtocolError::ReleaseFromDriver,
at: tests/contract_error_bodies.rs:344
quote: ProtocolError::Internal,
at: crates/viola-channel/src/lib.rs:99
quote: json!({"code": self.code(), "message": self.message(), "data": data})
@row_add n=636 row=decision/os-gated-tests-each-named area=e2e-test-strategy rec=fct-liulfbun@2
name: OS-gated tests each named with a reason
rule: Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named.
guard: os_gated_tests_are_each_named_with_their_reason
@row_check n=636 row=decision/os-gated-tests-each-named sha=c6e784ccf039 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_os_gated_tests.rs:423-425
quote: fn names_another_os(predicate: &str) -> bool { let spaced: String = predicate.split_whitespace().collect(); names_a_word(predicate, &[WINDOWS]) || spaced.contains("target_os=\"macos\"")
at: tests/contract_os_gated_tests.rs:532
quote: .filter(|(_, _, gate)| names_another_os(gate))
guard_at: tests/contract_os_gated_tests.rs:539
@row_add n=637 row=decision/module-boundary-integration-tests area=integration-test-strategy rec=fct-jvtm6f2a@1
name: Module boundaries tested by direct calls
rule: The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
@row_check n=637 row=decision/module-boundary-integration-tests sha=023966268895 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:224
quote: rstest = { version = "=0.27.0", default-features = false }
at: .config/nextest.toml:1
quote: [profile.ci]
@row_add n=638 row=decision/on-disk-state-integration-tests area=integration-test-strategy rec=fct-hctsd5xl@2
name: On-disk state integration tests
rule: Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests.
why: The project has no database.
@row_check n=638 row=decision/on-disk-state-integration-tests sha=adbdecfd59aa v=holds judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:719
quote: fn hook_processes_append_whole_lines_side_by_side() {
at: crates/viola-state/src/stamps.rs:198
quote: held = Some(other.try_lock().is_err());
@row_add n=639 row=decision/channel-tests-real-endpoint area=integration-test-strategy rec=fct-ouc4a2b7@2
name: Channel tests on a real endpoint
rule: Wrapper channel tests bind a real endpoint with the viola-channel server in a temp home and validate frames against JSON schemas.
@row_check n=639 row=decision/channel-tests-real-endpoint sha=f30543a314f3 v=holds judge=sonnet head=8834195b0683
at: tests/channel_endpoint.rs:247
quote: let validator = jsonschema::validator_for(&schema).expect("schema");
at: tests/cross_exit_causes.rs:391
quote: let squatter = viola_channel::Server::bind(&endpoint_of(home)).expect("the squatter binds");
@row_add n=640 row=decision/squatter-via-server-api area=integration-test-strategy rec=fct-ouc4a2b7@2,fct-pwscsozw@2
name: Squatter listener through the server API
rule: A test squatter listener binds the computed endpoint before run through the interprocess server API, never through try_overwrite.
@row_check n=640 row=decision/squatter-via-server-api sha=9b56cfa280fc v=holds judge=sonnet head=8834195b0683
at: tests/cli_instance_state.rs:269
quote: let squatter = viola_channel::Server::bind(&endpoint).expect("the squatter binds first");
at: crates/viola-channel/src/server.rs:125
quote: .reclaim_name(false)
@row_add n=641 row=decision/owner-only-socket-modes area=integration-test-strategy rec=fct-ouc4a2b7@2
name: Owner-only socket modes
rule: The Unix socket directory is 0700 and the socket file 0600, the file's mode set by a chmod after the bind.
@row_check n=641 row=decision/owner-only-socket-modes sha=9cdb00a0d139 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
at: crates/viola-state/src/fs.rs:27
quote: std::os::unix::fs::DirBuilderExt::mode(&mut builder, DIR_MODE);
@row_add n=642 row=decision/socket-dir-check area=integration-test-strategy rec=fct-ouc4a2b7@2
name: Socket directory check before use
rule: The socket directory is checked before use: a directory with a foreign permission bit or a foreign owner, a symlink or a non-directory is refused and left as found, an absent path passes, and the check never chmods.
guard: security_negatives_socket_dir
@row_check n=642 row=decision/socket-dir-check sha=e9b012ceed65 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:137-143
quote: pub fn check_socket_dir(dir: &Path) -> Result<(), Refused> { use std::os::unix::fs::MetadataExt as _; let meta = match std::fs::symlink_metadata(dir) { Ok(meta) => meta, Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
at: crates/viola-state/src/strict.rs:168-172
quote: if is_symlink { return Err(Refused::Symlink); } if !is_dir { return Err(Refused::NotADirectory);
guard_at: tests/security_negatives_socket_dir.rs:205
@row_add n=643 row=decision/channel-client-no-dir-check area=integration-test-strategy rec=fct-ouc4a2b7@2
name: Channel client carries no directory check
rule: The viola-channel client's connect carries no socket directory check, so a raw client writes past every client-side check.
why: viola-channel has no viola-state edge.
guard: channel_link_names_are_validated_by_the_wrapper
@row_check n=643 row=decision/channel-client-no-dir-check sha=a02f23217d38 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:32
quote: pub fn connect(endpoint: &str, process: &'static str) -> Result<Self, ChannelError> {
guard_at: tests/channel_links.rs:54
@row_add n=644 row=decision/pty-tests-real-openpty area=integration-test-strategy rec=fct-vmzc7an2@3
name: PTY tests on a real openpty
rule: PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
@row_check n=644 row=decision/pty-tests-real-openpty sha=c5f0666845f6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:132
quote: .openpty(pty_size(spec.size))
@row_add n=645 row=decision/resize-baseline-is-spawn-size area=integration-test-strategy rec=fct-vmzc7an2@3
name: Pump resize baseline is the spawn size
rule: The pump's resize baseline is the size the PTY was spawned with, never a second host read, so a resize landing before the pump's first look still reaches the child.
why: A resize can land between the spawn sizing and the pump's first look.
guard: pump_forwards_a_resize_that_lands_before_its_first_look
@row_check n=645 row=decision/resize-baseline-is-spawn-size sha=424c79f5cebe v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:165
quote: let mut last = Some(spawned);
guard_at: crates/viola-pty/src/pump.rs:378
@row_add n=646 row=decision/key-after-resize-waits-for-size area=integration-test-strategy rec=fct-vmzc7an2@3
name: Key after a resize waits for the new size
rule: A test that writes a key after a resize sends it only once the child reports the new size.
guard: spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code
@row_check n=646 row=decision/key-after-resize-waits-for-size sha=9b69e1156d59 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:866
quote: wait_line(&mut child, "size 120x40");
guard_at: crates/viola-pty/src/lib.rs:850
@row_add n=647 row=decision/hook-tests-real-binary area=integration-test-strategy rec=fct-aajqyzhy@2
name: Hook tests run the real binary
rule: Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line, the hook diagnostics trace and budget.json.
@row_check n=647 row=decision/hook-tests-real-binary sha=567f9a70f213 v=holds judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:39
quote: let mut cmd = Command::new(VIOLA);
at: tests/hook_fail_open.rs:74
quote: support::ndjson::read_lines(&home.join("diagnostics").join("hook-builder.ndjson"))
at: tests/hook_statusline.rs:50
quote: let bytes = fs::read(self.home.join("budget.json")).ok()?;
@row_add n=648 row=decision/hook-instance-check-first area=integration-test-strategy rec=fct-aajqyzhy@2
name: Hook instance check comes first
rule: The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing.
guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
@row_check n=648 row=decision/hook-instance-check-first sha=47b15a163801 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
guard_at: tests/hook_fail_open.rs:465
@row_add n=649 row=decision/hook-line-cases-in-made-instance-dir area=integration-test-strategy rec=fct-aajqyzhy@2
name: Hook line cases run in a viola-made instance dir
rule: Hook test cases that expect lines run in a viola-made instance directory.
why: A VIOLA_DIR naming no existing instance directory makes the hook write nothing.
@row_check n=649 row=decision/hook-line-cases-in-made-instance-dir sha=89f5df3f24c7 v=holds judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:304-305
quote: if matches!(env, Env::Made) { viola_state::fs::create_private_dir(&dir).expect("the instance dir");
@row_add n=650 row=decision/placed-copy-execs-at-once area=integration-test-strategy rec=fct-okfymegs@2
name: Placed copy closed before it is persisted
rule: A privately placed new copy is persisted only once its write handle is closed, so the copy can be executed right after placement.
why: Executing a copy whose write handle is still open fails with ExecutableFileBusy (ETXTBSY).
guard: place_private_new_placed_copy_execs_at_once
@row_check n=650 row=decision/placed-copy-execs-at-once sha=7dae65971cf0 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:117-118
quote: tmp.as_file().sync_all()?; match tmp.into_temp_path().persist_noclobber(path) {
guard_at: crates/viola-state/src/fs.rs:260
@row_add n=651 row=decision/guarded-paths-start-stamped area=integration-test-strategy rec=fct-qmrx4mno@2
name: Check-guarded cases start stamped
rule: A test case that drives a check-guarded path starts from a stamped home.
@row_check n=651 row=decision/guarded-paths-start-stamped sha=45c720144723 v=holds judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:299
quote: Env::Instance | Env::Made | Env::DirOnly | Env::InvalidName => stamped,
@row_add n=652 row=decision/wrapper-drop-guard area=integration-test-strategy rec=fct-qmrx4mno@2
name: Test wrappers held in a Drop guard
rule: Wrappers started in a test are held in a Drop guard that kills the child, then waits for it.
@row_check n=652 row=decision/wrapper-drop-guard sha=bdd6c23bad39 v=holds judge=sonnet head=8834195b0683
at: tests/support/cli.rs:63-64
quote: let _ = child.kill(); let _ = child.wait();
@row_add n=653 row=decision/private-dir-for-test-endpoints area=integration-test-strategy rec=fct-qmrx4mno@2,fct-pwscsozw@2
name: Test endpoint directories made private
rule: A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir.
why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
@row_check n=653 row=decision/private-dir-for-test-endpoints sha=7bd9d213a460 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:354
quote: viola_state::fs::create_private_dir(dir.path()).expect("the socket dir, private");
at: tests/security_negatives_server_identity.rs:94
quote: viola_state::fs::create_private_dir(socket_dir.path()).expect("the socket dir, private");
at: crates/viola-mcp/tests/handlers.rs:92
quote: viola_state::fs::create_private_dir(dir.path()).expect("the socket dir");
@row_add n=654 row=decision/readiness-stall-report area=integration-test-strategy rec=fct-qmrx4mno@2
name: Readiness stall report
rule: When the test readiness wait reaches its deadline, the stalled wrapper's last run event, kernel wait channel and process state are appended to the watch report before the panic; it signals nothing and a passing boot gains nothing.
why: So the report reaches the JUnit.
@row_check n=654 row=decision/readiness-stall-report sha=3bcbcc78448b v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:741-744
quote: if Instant::now() >= deadline { self.note_stall(&watch); } watch.deadline_check(deadline, &format!("wrapper {} not ready", self.name));
at: tests/support/home.rs:758
quote: watch.note(&format!("stall last event {}", last.unwrap_or("none")));
at: tests/support/home.rs:1000
quote: let wchan = fs::read_to_string(proc.join("wchan")).unwrap_or_default();
@row_add n=655 row=contract/wrapper-channel/params-v-higher area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel
item: params.v higher than served
promise: A request whose params.v is higher than the served version is answered with error -32602.
@row_check n=655 row=contract/wrapper-channel/params-v-higher sha=aa4753dc5aca v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:238-239
quote: if params.v > PROTOCOL_V { return Err(ProtocolError::UnsupportedVersion);
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
@row_add n=656 row=contract/wrapper-channel/frame-over-max-frame area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel
item: frame over MAX_FRAME
promise: A frame over MAX_FRAME is answered with error -32600 and the connection is closed; a pending dialog stays pending.
@row_check n=656 row=contract/wrapper-channel/frame-over-max-frame sha=da5af998e141 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:162
quote: Err(ChannelError::Oversize) => Some(refuse_oversize(srv_conn)),
at: crates/viola-channel/src/server.rs:266-269
quote: logged.class = Class::Error(Some(ProtocolError::InvalidRequest)); Reply { frame: error_response(None, ProtocolError::InvalidRequest), close: true,
@row_add n=657 row=decision/server-identity-verified area=integration-test-strategy rec=fct-ksdwqwkf@2
name: Server identity verified before any frame
rule: A server whose pid or start time does not match snapshot.json gets no frame: the CLI exits 21, the hook exits 0 with no body, and MCP frame tools report instance-unreachable with detail server-verify-failed.
guard: security_negatives_server_identity
@row_check n=657 row=decision/server-identity-verified sha=ad4c1635482a v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:260
quote: exit_code = 21u8,
at: src/cmd/hook.rs:242
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(NotSent::ServerUnverified),
at: src/cmd/hook.rs:294
quote: Asked::Deadline | Asked::Unreachable | Asked::ServerUnverified | Asked::SocketDir => None,
at: crates/viola-mcp/src/tools.rs:352-353
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? { return Ok(result::unreachable(UnreachableCause::ServerVerify));
guard_at: tests/security_negatives_server_identity.rs:1
@row_add n=658 row=contract/mcp-unreachable-detail/strict-modes-failed area=integration-test-strategy rec=fct-ksdwqwkf@2,fct-bt3npe7x@2
boundary: MCP frame tools instance-unreachable detail
item: strict-modes-failed
promise: A refused instance check gives detail strict-modes-failed, before any snapshot read or connection, with no frame, no log line and the instance left as found.
@row_check n=658 row=contract/mcp-unreachable-detail/strict-modes-failed sha=d32a5a68f692 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-340
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=659 row=contract/mcp-unreachable-detail/socket-dir-refused area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools instance-unreachable detail
item: socket-dir-refused
promise: A refused socket directory check after the live endpoint is read gives detail socket-dir-refused, with no frame and no log line.
@row_check n=659 row=contract/mcp-unreachable-detail/socket-dir-refused sha=84be6982d595 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=660 row=contract/mcp-unreachable-detail/server-verify-failed area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools instance-unreachable detail
item: server-verify-failed
promise: A peer euid the admission refuses, or a failed server verification, gives detail server-verify-failed, with no frame and no log line.
@row_check n=660 row=contract/mcp-unreachable-detail/server-verify-failed sha=024a940af63f v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:352-353
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? { return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=661 row=contract/mcp-unreachable-detail/instance-dead area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools instance-unreachable detail
item: instance-dead
promise: A failed connect gives detail instance-dead, with no frame and no log line.
@row_check n=661 row=contract/mcp-unreachable-detail/instance-dead sha=fa7fdfb88e1d v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:53
quote: Err(ChannelError::Connect(_)) => return Ok(unreachable(UnreachableCause::InstanceDead)),
@row_add n=662 row=contract/mcp-unreachable-detail/endpoint-lost area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools instance-unreachable detail
item: endpoint-lost
promise: A closed connection or an I/O failure, a wrapper killed under a parked wait included, gives detail endpoint-lost.
@row_check n=662 row=contract/mcp-unreachable-detail/endpoint-lost sha=3aa637d8f2ab v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:54-55
quote: Err(ChannelError::Closed | ChannelError::Io(_)) => { return Ok(unreachable(UnreachableCause::EndpointLost));
@row_add n=663 row=contract/mcp-frame-tools/refusal-result area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools
item: refusal result
promise: A refused send, wait, last or answer returns isError with {"error":"instance-unreachable","detail":<cause>} and the text refused: instance-unreachable.
@row_check n=663 row=contract/mcp-frame-tools/refusal-result sha=cf3fc5580ccd v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=664 row=contract/mcp-frame-tools/free-text-refusal area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: MCP frame tools
item: refusal of free text
promise: A wrapper refusal of free text reads as unknown, with no detail.
@row_check n=664 row=contract/mcp-frame-tools/free-text-refusal sha=a805dccbb462 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:73-77
quote: let reason = if is_code_word(reason) {         reason     } else {         RefusalReason::Unknown.as_str()     };
@row_add n=665 row=decision/paste-validated-at-both-ends area=integration-test-strategy rec=fct-ksdwqwkf@2
name: Sent text validated at client and wrapper
rule: Sent and answered text is validated at both the client and the wrapper, so a raw client that skips client validation gets the same control-character and empty-text rules from the wrapper.
guard: channel_paste_validation
@row_check n=665 row=decision/paste-validated-at-both-ends sha=57ec997c1e7d v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/cmd/answer.rs:110
quote: .find_map(|text| viola_core::validate_paste_text(text).err());
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:93-94
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText);
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
guard_at: tests/channel_paste_validation.rs:1
@row_add n=666 row=contract/wrapper-channel/empty-text area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel
item: refusal detail empty-text
promise: An empty typed text is refused {"refusal":"not-delivered","detail":"empty-text"}, with a send-refused record holding no cursor and no send-issued record.
@row_check n=666 row=contract/wrapper-channel/empty-text sha=aa5daea59336 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:93-95
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText); }
at: src/run/send.rs:279
quote: self.record(EventKind::SendRefused, data)?;
@row_add n=667 row=contract/wrapper-channel/accepted-text area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel
item: text with LF, CR, TAB and multi-byte characters
promise: Text with LF, a lone inner CR, TAB and multi-byte characters is accepted and typed as one prompt, its inner CR as one LF.
@row_check n=667 row=contract/wrapper-channel/accepted-text sha=e2d506223128 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:234
quote: Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: src/run/send.rs:91
quote: let typed = typed_text(text);
@row_add n=668 row=contract/cli-exit-codes/13-text-refused area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: CLI exit codes
item: 13 for refused text
promise: viola send and viola answer with a control character or an empty text exit 13 before any frame is sent.
@row_check n=668 row=contract/cli-exit-codes/13-text-refused sha=e7008252e881 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/cmd/send.rs:195
quote: return Ok(13);
at: src/cmd/answer.rs:110
quote: .find_map(|text| viola_core::validate_paste_text(text).err());
at: src/cmd/answer.rs:114
quote: return Ok(13);
@row_add n=669 row=contract/viola-answer/oversize-stdin area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: viola answer
item: stdin over MAX_FRAME
promise: Exits 2 before connecting, with the fixed unusable-response line on stderr whatever --json says and no document; the dialog stays pending.
@row_check n=669 row=contract/viola-answer/oversize-stdin sha=659a74a88b46 v=holds judge=sonnet head=8834195b0683
at: src/cmd/answer.rs:50-51
quote: if bytes.len() as u64 > MAX_FRAME { return None;
at: src/cmd/answer.rs:91
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
at: src/cmd/answer.rs:92
quote: return Ok(ExitCode::from(2));
@row_add n=670 row=decision/bad-config-value-falls-back area=integration-test-strategy rec=fct-ksdwqwkf@2,fct-lzdm6e7w@6
name: Bad config value falls back to its default
rule: A config.json value out of range or of the wrong type falls back to that key's default with exactly one parse-rejected config-json malformed line per bad key; the product never refuses to start over it.
guard: setting_does_not_disable_the_budget_gate
@row_check n=670 row=decision/bad-config-value-falls-back sha=e74b96be29f2 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:79-81
quote: Some(None) => { rejections.push(ConfigRejection::Malformed); compiled(default)
at: src/obs.rs:210-211
quote: let rejection = if bad_value { Some(ConfigRejection::Malformed)
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
guard_at: tests/cli_controls_not_disableable.rs:369
@row_add n=671 row=contract/channel-log/link-existing-pair area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel log
item: link of an existing pair
promise: Linking an already linked pair adds no log line.
@row_check n=671 row=contract/channel-log/link-existing-pair sha=6ffd690ec3cc v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62-63
quote: if let Some(since) = self.since(&set, driver) { return Ok((since, true));
@row_add n=672 row=contract/channel-log/unserved-method area=integration-test-strategy rec=fct-ksdwqwkf@2
boundary: Wrapper channel log
item: response line of an unserved method
promise: The conn-less response line of an unserved method carries no method key; method names only the closed method list.
@row_check n=672 row=contract/channel-log/unserved-method sha=722d1ae56b19 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:353-354
quote: pub(crate) fn method_label(method: &str) -> Option<&'static str> { METHODS.iter().find(|m| **m == method).copied()
at: crates/viola-channel/src/server.rs:479
quote: method = self.method,
@row_add n=673 row=contract/event-line/keys area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event line
item: keys
promise: Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.
@row_check n=673 row=contract/event-line/keys sha=b9ec63eb6fee v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:140
quote: const LINE_KEYS: [&str; 6] = ["v", "ts", "instance", "kind", "source", "data"];
at: crates/viola-state/src/events.rs:54-59
quote: v: 1, ts: crate::timestamp(at), instance: instance.as_ref().to_owned(), kind, source, data,
@row_add n=674 row=contract/event-data/link area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: link
promise: data holds the keys driver and driven.
@row_check n=674 row=contract/event-data/link sha=72bac45cee22 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:130
quote: Self::Link | Self::Unlink => &["driver", "driven"],
@row_add n=675 row=contract/event-data/unlink area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: unlink
promise: data holds the keys driver and driven.
@row_check n=675 row=contract/event-data/unlink sha=98e667fe7b76 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:130
quote: Self::Link | Self::Unlink => &["driver", "driven"],
@row_add n=676 row=contract/event-data/dialog-expired area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: dialog-expired
promise: data holds the keys dialog_id and dialog_kind.
@row_check n=676 row=contract/event-data/dialog-expired sha=70b84aa7b67a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:131
quote: Self::DialogExpired => &["dialog_id", "dialog_kind"],
@row_add n=677 row=contract/event-data/turn-ended area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: turn-ended
promise: data holds the keys last_assistant_message and background_tasks.
@row_check n=677 row=contract/event-data/turn-ended sha=521356826f83 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:119
quote: Self::TurnEnded => &["last_assistant_message", "background_tasks"],
@row_add n=678 row=contract/event-data/question area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Event data keys
item: question
promise: option_details is a key of each entry in the questions array of a question line's data; the data keys themselves are dialog_id and questions.
@row_check n=678 row=contract/event-data/question sha=eba70506a19a v=stale judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:127
quote: Self::Question => &["dialog_id", "questions"],
shows: A question line's data keys are dialog_id and questions; option_details is no data key (a top-level one is counted unknown by events.rs:214), it is a key inside each entry of questions.
cell: promise
value: option_details is a key of each entry in the questions array of a question line's data; the data keys themselves are dialog_id and questions.
@row_add n=679 row=decision/tolerant-event-reader area=integration-test-strategy rec=fct-bt3npe7x@2
name: Tolerant event log reader
rule: The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
@row_check n=679 row=decision/tolerant-event-reader sha=d95ce0243b84 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
at: crates/viola-state/src/events.rs:265-266
quote: if has_unknown_field(&object, kind) { self.skipped.unknown_fields += 1;
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
@row_add n=680 row=decision/snapshot-replayed-from-log area=integration-test-strategy rec=fct-bt3npe7x@2
name: Broken snapshot replayed from the log
rule: A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
@row_check n=680 row=decision/snapshot-replayed-from-log sha=c5994afd5e47 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:154-157
quote: SnapshotRead::Unreadable => ReplayCause::Unreadable, SnapshotRead::Unsupported { v_seen } => ReplayCause::Unsupported { v_seen }, }; let state = replay(instance_dir)?;
@row_add n=681 row=decision/link-since-kept area=integration-test-strategy rec=fct-bt3npe7x@2
name: Link keeps its first since
rule: A linked pair keeps its first since until an unlink, and takes a later one after it.
@row_check n=681 row=decision/link-since-kept sha=b36ffe6a2007 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62-63
quote: if let Some(since) = self.since(&set, driver) { return Ok((since, true));
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
at: src/run/links.rs:67
quote: Ok((line.ts, false))
@row_add n=682 row=contract/strict-modes-refusal/run area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: viola run
promise: Exit 1 with the home or instance pair, made before the wrapper log opens: no line and no child.
@row_check n=682 row=contract/strict-modes-refusal/run sha=75a5ac11a8b0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:172-173
quote: refuse_tree(&args.name, refused); return Ok(ExitCode::from(1));
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
@row_add n=683 row=contract/strict-modes-refusal/mcp area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: viola mcp
promise: Exit 1.
@row_check n=683 row=contract/strict-modes-refusal/mcp sha=762e25aae736 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mcp.rs:22-23
quote: if !home_passes(home) { return Ok(ExitCode::from(1));
@row_add n=684 row=contract/strict-modes-refusal/revive area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: viola revive
promise: Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found.
@row_check n=684 row=contract/strict-modes-refusal/revive sha=adb16e78eae2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:272-274
quote: if !trusted(home, name)? { Refusal::StrictModes.write(name, json); return Ok(ExitCode::from(1));
at: src/cmd/revive.rs:137-139
quote: if !trusted(home, &args.name)? { Refusal::StrictModes.write(&args.name, args.json); return Ok(Started::Refused(ExitCode::from(1)));
@row_add n=685 row=contract/strict-modes-refusal/verify area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: viola verify
promise: Exit 1 with the home pair before any step: no step line, no log line, stamps.json unchanged.
@row_check n=685 row=contract/strict-modes-refusal/verify sha=6aa755dbefdc v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
@row_add n=686 row=contract/strict-modes-refusal/other-verbs area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: Strict-modes refusal
item: every other CLI verb
promise: Exit 21.
@row_check n=686 row=contract/strict-modes-refusal/other-verbs sha=c68636b6fcf3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=687 row=decision/instance-check-before-role-file area=integration-test-strategy rec=fct-bt3npe7x@2
name: Instance check before any role file or frame
rule: viola link, unlink, send, wait, last, answer, pause and release run the instance check on the driven instance before any role file or log opens or any frame is sent, leaving the mode as found.
why: Opening a role file narrows an existing home to 0700, so a check placed after it would pass a 0770 home.
guard: link_verbs_refuse_a_widened_instance_before_any_log
@row_check n=687 row=decision/instance-check-before-role-file sha=e01f88df98d5 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/unlink.rs:15
quote: let code = link_call(home, args, Verb::Unlink, request)?;
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/wait.rs:43
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/last.rs:30
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/answer.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/pause.rs:32
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/release.rs:35
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
guard_at: tests/cli_links.rs:531
@row_add n=688 row=contract/cli-json-error/instance-unreachable area=integration-test-strategy rec=fct-bt3npe7x@2
boundary: CLI --json error document
item: instance-unreachable strict-modes-failed
promise: An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.
@row_check n=688 row=contract/cli-json-error/instance-unreachable sha=6c1447cea902 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:34-35
quote: if json { unreachable_document(UnreachableCause::StrictModes);
@row_add n=689 row=decision/widened-home-tree-refused-at-start area=integration-test-strategy rec=fct-bt3npe7x@2
name: Widened home tree refused at start
rule: viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found.
guard: run_refuses_a_widened_home_tree_before_its_log
@row_check n=689 row=decision/widened-home-tree-refused-at-start sha=0f9d13548812 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/verify.rs:67
quote: if check_home_tree(&home, None, None).is_err() {
guard_at: tests/run_cli.rs:611
@row_add n=690 row=decision/home-read-rule area=integration-test-strategy rec=fct-bt3npe7x@2
name: Home readable by another user is refused
rule: A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing.
guard: a_home_another_user_may_read_refuses_run
@row_check n=690 row=decision/home-read-rule sha=f2f7057deb96 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:89
quote: check_existing(&[(home.to_path_buf(), Rule::Read)]).map_err(TreeRefused::HomeLevel)?;
at: crates/viola-state/src/board.rs:59-62
quote: if check_if_present(home, check_read).is_err() || check_if_present(&instances_dir, check_path).is_err() { return Ok(Err(ScanRefused::Home));
guard_at: tests/security_negatives_strict_modes.rs:385
@row_add n=691 row=decision/human-output-pinned-by-literals area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Human output pinned by literals
rule: Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
@row_check n=691 row=decision/human-output-pinned-by-literals sha=c61b178c2feb v=holds judge=sonnet head=8834195b0683
at: tests/cli_output_plain.rs:55
quote: const HELP: &str = concat!(
at: tests/cli_list.rs:27
quote: const CAPTION: &str = "NAME     LIVE   STATUS   WHEEL                    DIALOG             CLI";
at: tests/cli_send.rs:260
quote: let prefix = "[RB] read back      builder  ";
at: tests/cli_verify.rs:26
quote: const STEPS_PASS: [&str; 24] = [
@row_add n=692 row=decision/no-cli-snapshot-crates area=integration-test-strategy rec=fct-lzdm6e7w@6
name: No CLI snapshot crates
rule: No manifest holds assert_cmd, predicates, trycmd or snapbox, and there is no command fixture directory.
@row_check n=692 row=decision/no-cli-snapshot-crates sha=ad4e638e6035 v=holds judge=sonnet head=8834195b0683
searched: assert_cmd|predicates|trycmd|snapbox · in **/Cargo.toml
searched: tests/cmd/** · in the repository tree
@row_add n=693 row=decision/no-escape-in-plain-output area=integration-test-strategy rec=fct-lzdm6e7w@6
name: No escape bytes in plain output
rule: No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE.
guard: cli_output_plain
@row_check n=693 row=decision/no-escape-in-plain-output sha=d0775d60f731 v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/style.rs:62-68
quote: if facts.json || facts.passthrough || facts.no_color || facts.term.as_deref() == Some("dumb") || !facts.stdout_tty { return None;
at: Cargo.toml:197-204
quote: clap = { version = "=4.6.7", default-features = false, features = [ "std", "derive", "help", "usage", "error-context", "suggestions", ] }
guard_at: tests/cli_output_plain.rs:145
@row_add n=694 row=decision/help-no-bold-underline area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Help carries no bold or underline
rule: viola --help under a terminal carries no SGR holding bold or underline, and on Unix no CR not directly followed by LF.
guard: cli_output_plain
@row_check n=694 row=decision/help-no-bold-underline sha=aacaa40e2b08 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:197-204
quote: clap = { version = "=4.6.7", default-features = false, features = [ "std", "derive", "help", "usage", "error-context", "suggestions", ] }
at: tests/cli_output_plain.rs:160
quote: !attributes.iter().any(|&a| a == 1 || a == 4),
guard_at: tests/cli_output_plain.rs:154
@row_add n=695 row=contract/viola-help/verb-table area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola --help
item: grouped verb table
promise: Prints the grouped verb table listing only served verbs, plugin install one entry of setup:, with no hook, help or bare plugin; exit 0, stderr empty.
@row_check n=695 row=contract/viola-help/verb-table sha=5f8ac9c9472f v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:34-38
quote: board:    list traffic:  send, wait, last, answer wheel:    pause, release handoff:  link, unlink setup:    run, revive, verify, plugin install
@row_add n=696 row=contract/verify-output/step-line area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify output
item: step line
promise: One [NN/MM] <row id> <row words>  pass|fail line per relied ledger row, MM the relied count.
@row_check n=696 row=contract/verify-output/step-line sha=42e98c1c2144 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
@row_add n=697 row=contract/verify-output/summary-line area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify output
item: summary line
promise: The last stdout line is stamped <ver>  <n> pass  <m> fail.
@row_check n=697 row=contract/verify-output/summary-line sha=55847925fcd3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:293-294
quote: human::result(&format!(         "stamped {version}  {} pass  {failed} fail",
@row_add n=698 row=contract/verify-stop/signal area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify interrupted
item: SIGINT or SIGTERM
promise: Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.
@row_check n=698 row=contract/verify-stop/signal sha=04b8a99ca995 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:84-87
quote: "verify was stopped before it finished", "run viola verify again to stamp this CLI", ); (1, Some("interrupted"))
@row_add n=699 row=contract/verify-exit-detail/program-not-found area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: program-not-found
promise: Written on verify's process-exit under a parsed VIOLA_NAME when no claude program is found.
@row_check n=699 row=contract/verify-exit-detail/program-not-found sha=2684cb02b186 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:164
quote: return Ok(Ended::Refused("program-not-found"));
@row_add n=700 row=contract/verify-exit-detail/pinned-hash-mismatch area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: pinned-hash-mismatch
promise: Written on verify's process-exit under a parsed VIOLA_NAME when the pinned copy is tampered.
@row_check n=700 row=contract/verify-exit-detail/pinned-hash-mismatch sha=5ae40194ab55 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
@row_add n=701 row=contract/verify-exit-detail/version-unreadable area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: version-unreadable
promise: Written on verify's process-exit under a parsed VIOLA_NAME when the CLI version cannot be read.
@row_check n=701 row=contract/verify-exit-detail/version-unreadable sha=9102442e3c10 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:199
quote: return Ok(Ended::Refused("version-unreadable"));
@row_add n=702 row=contract/verify-exit-detail/strict-modes-failed area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: strict-modes-failed
promise: Written on verify's process-exit under a parsed VIOLA_NAME when a strict-modes check refuses.
@row_check n=702 row=contract/verify-exit-detail/strict-modes-failed sha=31b906f4992d v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=703 row=contract/verify-exit-detail/unclean-fixture area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: unclean-fixture
promise: Written on verify's process-exit under a parsed VIOLA_NAME when a recorded fixture is not clean; the fixture's file name enters no code.
@row_check n=703 row=contract/verify-exit-detail/unclean-fixture sha=54684829f5cf v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
@row_add n=704 row=contract/verify-exit-detail/interrupted area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: viola verify process-exit detail
item: interrupted
promise: Written on verify's process-exit when verify is stopped by a signal.
@row_check n=704 row=contract/verify-exit-detail/interrupted sha=2ed9a1f9d521 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
@row_add n=705 row=decision/verify-failing-row-no-refusal area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Failing ledger row is no refusal
rule: A failing ledger row in viola verify is no refusal and carries no code.
@row_check n=705 row=decision/verify-failing-row-no-refusal sha=0769b65cbe69 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:297-298
quote: if failed > 0 { return Ok(Ended::Exit(1));
at: src/cmd/verify.rs:80-81
quote: Ended::Exit(code) => (code, None), Ended::Refused(detail) => (1, Some(detail)),
@row_add n=706 row=decision/verify-takes-no-json area=integration-test-strategy rec=fct-lzdm6e7w@6
name: viola verify takes no --json
rule: viola verify takes no --json flag.
@row_check n=706 row=decision/verify-takes-no-json sha=cae813fb2aa2 v=holds judge=sonnet head=8834195b0683
searched: json: bool · in src/cmd/verify.rs (VerifyArgs holds only `record` and `program`)
searched: long.*json|json: bool|global = true · in src/ (the only global flag is `home`, src/cmd/mod.rs:53; no json field in the verify args)
@row_add n=707 row=decision/plugin-marketplace-mcp-only area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Plugin marketplace is MCP-only
rule: viola plugin install writes a marketplace registering viola --home <home> mcp on the executable's absolute path, with no hooks anywhere.
guard: plugin_install_writes_the_mcp_only_marketplace
@row_check n=707 row=decision/plugin-marketplace-mcp-only sha=227010071d2c v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/marketplace.rs:45
quote: NAME: {"command": exe_fwd, "args": ["--home", home_fwd, "mcp"]},
at: src/cmd/plugin.rs:108
quote: let exe = super::canonical_home(&std::env::current_exe()?);
guard_at: tests/cli_plugin_install.rs:94
@row_add n=708 row=decision/plugin-install-owner-only area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Plugin install writes owner-only
rule: viola plugin install writes its tree at 0700 / 0600 whatever the umask, and refuses a widened home with exit 21, leaving the home as found.
guard: plugin_install_writes_owner_only_modes
@row_check n=708 row=decision/plugin-install-owner-only sha=e77e6fe6bb2b v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:30
quote: restrict(dir, DIR_MODE)?;
at: src/cmd/plugin.rs:116
quote: replace_private(&root.join(rel), content.as_bytes(), FILE_MODE)?;
at: src/cmd/plugin.rs:53
quote: if !tree_passes(&home) {
at: src/cmd/plugin.rs:60
quote: return Ok(ExitCode::from(21));
guard_at: tests/cli_plugin_install.rs:168
@row_add n=709 row=contract/channel-response-detail/unsupported-version area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: channel-response line detail
item: unsupported-version
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
@row_check n=709 row=contract/channel-response-detail/unsupported-version sha=79f7f7dd749f v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:105
quote: Self::UnsupportedVersion => Some("unsupported-version"),
@row_add n=710 row=contract/channel-response-detail/invalid-params area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: channel-response line detail
item: invalid-params
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
@row_check n=710 row=contract/channel-response-detail/invalid-params sha=3ba824ca2d34 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:106
quote: Self::InvalidParams => Some("invalid-params"),
@row_add n=711 row=contract/channel-response-detail/release-from-driver area=integration-test-strategy rec=fct-lzdm6e7w@6
boundary: channel-response line detail
item: release-from-driver
promise: One of the closed detail values, on both the server's and the client's line at WARN, with no name or param on the line; detail is absent on every other channel-response line.
@row_check n=711 row=contract/channel-response-detail/release-from-driver sha=b5b34277160d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:107
quote: Self::ReleaseFromDriver => Some("release-from-driver"),
@row_add n=712 row=decision/no-names-in-wrapper-log area=integration-test-strategy rec=fct-lzdm6e7w@6
name: No session names in the wrapper log
rule: The wrapper's process log carries no driver or driven name, and channel lines carry no name or param.
guard: link_verbs_log_the_channel_pair_and_no_names
@row_check n=712 row=decision/no-names-in-wrapper-log sha=1cc4ac385e1f v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:367-375
quote: ObsEvent::ChannelRequest, corr = corr, conn = peer.conn(), srv_conn = peer.srv_conn(), method = method, sender = params.sender.as_deref().and_then(version_label), v = params.v, after = after, timeout_ms = timeout_ms,
at: crates/viola-channel/src/server.rs:475-484
quote: ObsEvent::ChannelResponse, corr = self.corr, conn = self.peer.conn(), srv_conn = self.peer.srv_conn(), method = self.method, result_class = result_class, error_code = fault.map(ProtocolError::code), detail = fault.and_then(ProtocolError::cause),
guard_at: tests/cli_links.rs:468
@row_add n=713 row=decision/home-resolution-order area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Home resolution order
rule: The home resolves from --home, then the VIOLA_NAME + VIOLA_DIR instance home, then <user home>/.viola, canonicalised once it exists.
why: The CLI verbs' VIOLA_DIR step and the canonicalisation are a ratified widening.
guard: home_resolution_takes_the_flag_then_the_session_then_the_default
@row_check n=713 row=decision/home-resolution-order sha=8dd6851c5143 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:294-295
quote: if let Some(home) = flag { return Ok(home);
at: src/cmd/mod.rs:297-298
quote: if let Some(instance) = instance_of(name, dir) { return Ok(instance.home);
at: src/cmd/mod.rs:300-301
quote: user_home .map(|user_home| default_home(&user_home))
at: src/cmd/mod.rs:281
quote: .map(|home| canonical_home(&home))
guard_at: tests/cli_machine_contract.rs:445
@row_add n=714 row=decision/test-viola-drops-session-vars area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Test-started viola drops session variables
rule: A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
@row_check n=714 row=decision/test-viola-drops-session-vars sha=b351b89c80e0 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:24
quote: pub const UNWRAPPED: [&str; 3] = ["VIOLA_NAME", "VIOLA_DIR", "VIOLA_BIN"];
at: tests/support/home.rs:903-905
quote: .env_remove(UNWRAPPED[0]) .env_remove(UNWRAPPED[1]) .env_remove(UNWRAPPED[2])
at: tests/cli_machine_contract.rs:26
quote: StampedHome, TestHome, UNWRAPPED, VIOLA, resolved, snapshot_data, stamped_home,
@row_add n=715 row=decision/ledger-probe-contract-suite area=integration-test-strategy rec=fct-lzdm6e7w@6
name: Ledger probe contract suite
rule: For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list.
guard: contract_ledger_probes
@row_check n=715 row=decision/ledger-probe-contract-suite sha=381656a7f2c2 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_ledger_probes.rs:44
quote: const STAMPED: [&str; 2] = ["2.1.287", "2.1.302"];
at: tests/contract_ledger_probes.rs:94-98
quote: for id in ROW_IDS { let naming: Vec<&&str> = steps .iter() .filter(|l| l.split(' ').nth(1) == Some(id)) .collect();
at: tests/contract_ledger_probes.rs:110
quote: assert_eq!(*last, format!("stamped {version}  24 pass  0 fail"));
at: tests/contract_ledger_probes.rs:67-69
quote: let lists = usize::from(STAMPED.contains(&version.as_str())) + usize::from(DRIFT_ONLY.contains(&version.as_str())); assert_eq!(lists, 1, "{version} is on {lists} lists");
guard_at: tests/contract_ledger_probes.rs:80
@row_add n=716 row=decision/wrapper-deadline-injected-clock area=mocking-stubbing-discipline rec=fct-je5xeiyl@1
name: Wrapper deadlines driven by the injected clock
rule: The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.
@row_check n=716 row=decision/wrapper-deadline-injected-clock sha=3fd8e8343b1f v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:149
quote: let deadline = timeout_ms.and_then(|ms| deadline(self.clock.now(), ms));
@row_add n=717 row=decision/external-processes-real-stubs area=mocking-stubbing-discipline rec=fct-sik7tvk4@2
name: External processes are real stub executables
rule: External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
@row_check n=717 row=decision/external-processes-real-stubs sha=6889e6014c3a v=holds judge=sonnet head=8834195b0683
at: tests/support/fake.rs:13
quote: pub const FAKE: &str = env!("CARGO_BIN_EXE_viola-fake-agent");
at: crates/viola-e2e/src/harness/mod.rs:195-196
quote: pub fn bin_dir_from_exe(exe: &Path) -> PathBuf { let parent = exe.parent().unwrap_or(exe);
@row_add n=718 row=decision/impostor-holder-real-process area=mocking-stubbing-discipline rec=fct-sik7tvk4@2
name: Impostor's held process is real
rule: A held live process whose pid an impostor's snapshot records is a real process, never a double, killed and reaped by a Drop guard.
@row_check n=718 row=decision/impostor-holder-real-process sha=65738aff2ab7 v=holds judge=sonnet head=8834195b0683
at: tests/security_negatives_server_identity.rs:49-50
quote: let _ = self.0.kill(); let _ = self.0.wait();
@row_add n=719 row=decision/real-process-faults area=mocking-stubbing-discipline rec=fct-63urz52n@3,fct-lzdm6e7w@6
name: Process faults are real
rule: A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.
@row_check n=719 row=decision/real-process-faults sha=40cff485845b v=holds judge=sonnet head=8834195b0683
at: tests/support/frozen.rs:14
quote: frozen.signal("-STOP");
at: tests/support/start.rs:151
quote: .args(["-KILL", &pid.to_string()])
@row_add n=720 row=decision/impostor-server-real-listener area=mocking-stubbing-discipline rec=fct-pwscsozw@2
name: Impostor server is a real listener
rule: Server-verification negatives use an impostor: a test-owned server bound on the recorded endpoint while the snapshot records another held live process.
@row_check n=720 row=decision/impostor-server-real-listener sha=b82730f034ba v=holds judge=sonnet head=8834195b0683
at: tests/security_negatives_server_identity.rs:102
quote: let serving = Server::bind(&endpoint)
at: tests/hook_fail_open.rs:212
quote: let serving = viola_channel::Server::bind(&endpoint)
@row_add n=721 row=decision/hook-unit-impostor-varies-start-time area=mocking-stubbing-discipline rec=fct-pwscsozw@2
name: Hook unit impostor varies the start time
rule: The hook's unit impostor varies the recorded start time, a second early or an unparseable started_at, instead of using a second process.
why: A root-bin unit test has no portable second process.
@row_check n=721 row=decision/hook-unit-impostor-varies-start-time sha=14445c84f901 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:1021-1022
quote: #[case::another_start_time(an_earlier_start())] #[case::started_at_not_a_time("s".to_owned())]
@row_add n=722 row=decision/real-filesystem-faults area=mocking-stubbing-discipline rec=fct-zsbxg2jw@2
name: Filesystem faults are real
rule: A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found.
except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.
@row_check n=722 row=decision/real-filesystem-faults sha=d9bcbc897c82 v=holds judge=sonnet head=8834195b0683
at: tests/security_negatives_socket_dir.rs:54
quote: fs::set_permissions(dir, fs::Permissions::from_mode(0o755)).expect("chmod");
at: crates/viola-state/src/strict.rs:161
quote: pub fn socket_dir_verdict(
@row_add n=723 row=decision/forced-window-hook-is-test-parameter area=mocking-stubbing-discipline rec=fct-7ilveif5@1
name: Forced-window hook is a test parameter
rule: A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list.
@row_check n=723 row=decision/forced-window-hook-is-test-parameter sha=4e52b11a7527 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:111
quote: placed: &mut dyn FnMut(&Path),
@row_add n=724 row=decision/peer-admission-by-injection area=mocking-stubbing-discipline rec=fct-6xaoh2kl@2
name: Peer admission tested by injection
rule: The peer euid decision takes its euids as arguments and the private serve function takes its admission as an argument, with no env, flag or feature seam.
@row_check n=724 row=decision/peer-admission-by-injection sha=1408a4a03721 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:139
quote: pub fn peer_euid_admitted(observed: Option<u32>, own: u32) -> bool {
at: crates/viola-channel/src/server.rs:72
quote: fn serve_admitting(self, dispatch: Arc<dyn Dispatch>, admitted: Admission) -> Serving {
@row_add n=725 row=decision/coverage-includes-child-processes area=quality-gates-coverage-targets rec=fct-s6rhzqu4@1
name: Coverage includes child viola processes
rule: Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE.
except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
@row_check n=725 row=decision/coverage-includes-child-processes sha=1356cbb47cf5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:518-523
quote: const COVERAGE_ENV: [&str; 4] = [ "CARGO_LLVM_COV", "LLVM_PROFILE_FILE", "RUSTC_WRAPPER", "__CARGO_LLVM_COV_RUSTC_WRAPPER_RUSTFLAGS", ];
at: crates/viola-e2e/src/harness/run.rs:528
quote: cmd.env_remove(name);
@row_add n=726 row=decision/fake-agent-own-process-group area=quality-gates-coverage-targets rec=fct-s6rhzqu4@1
name: Fake agent spawns in its own process group
rule: Every command the fake agent spawns runs in a process group of its own.
why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.
@row_check n=726 row=decision/fake-agent-own-process-group sha=1d297f9f840d v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:640
quote: std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
at: src/bin/viola-fake-agent.rs:1026
quote: std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
@row_add n=727 row=decision/killed-test-child-uninstrumented area=quality-gates-coverage-targets rec=fct-s6rhzqu4@1
name: Killed test child is uninstrumented
rule: A test that kills its child straight after the start uses an uninstrumented host program as that child.
why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.
@row_check n=727 row=decision/killed-test-child-uninstrumented sha=848b6034ead4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:1622
quote: program: OsString::from("whoami"),
@row_add n=728 row=decision/profraw-census-dev-host-only area=quality-gates-coverage-targets rec=fct-s6rhzqu4@1
name: Profraw census is a dev-host tool
rule: The profraw census script is a dev-host tool: no CI job, pre-push stage or harness command runs it.
@row_check n=728 row=decision/profraw-census-dev-host-only sha=06b08f001ba1 v=holds judge=sonnet head=8834195b0683
searched: profraw-census · in .github/**, .config/**, crates/**/*.rs, src/**/*.rs, tests/**/*.rs (only scripts/profraw-census.sh itself names it)
searched: profraw-census · in scripts/agent-run.sh and the harness pre_push stages (no match)
@row_add n=729 row=decision/mutation-gate-at-epoch-boundary area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Mutation gate at the epoch boundary
rule: Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
@row_check n=729 row=decision/mutation-gate-at-epoch-boundary sha=9df45b7e0974 v=holds judge=sonnet head=8834195b0683
searched: --mutants · in .github/workflows/ci.yml (no run step; only comments at lines 32, 91)
searched: --mutants · in crates/viola-e2e/src/harness/pre_push.rs and pre_push/linux.rs (only the assertions `assert!(!fake.calls.iter().any(|c| c.contains("--mutants")))` at pre_push.rs:248, linux.rs:328, 343)
@row_add n=730 row=contract/mutants-verdict/unviable-exceeds-caught area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
boundary: Mutation run verdict
item: unviable-exceeds-caught
promise: The run is red when unviable mutants outnumber caught ones.
@row_check n=730 row=contract/mutants-verdict/unviable-exceeds-caught sha=9ba2835aea8e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:61-63
quote: let swamped = unviable > caught; if swamped { failures.push("unviable-exceeds-caught".to_owned());
@row_add n=731 row=decision/doctest-only-catch-is-missed area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Doctest-only catch counts as missed
rule: A mutant caught only by a doctest counts as missed, and the missing unit test must be added.
@row_check n=731 row=decision/doctest-only-catch-is-missed sha=645e2fd853b8 v=unverifiable judge=sonnet head=8834195b0683
searched: doctest · in crates/viola-e2e/src/harness/run/mutants.rs (no match; the run only passes --test-tool=nextest at line 239)
why: not a code claim; it describes how cargo-mutants under nextest grades a doctest-only catch, no repository code emits or checks it
@row_add n=732 row=decision/host-unreachable-mutants area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Host-unreachable mutants recorded, not caught
rule: A mutant this host cannot compile or reach is recorded not measured here, owed to the route entry that measures it, never equivalent and never caught; one wholly under a cfg proved false for the host is left out of the count with its predicate.
@row_check n=732 row=decision/host-unreachable-mutants sha=ac0d2a9dc386 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
at: crates/viola-e2e/src/harness/run/mutants.rs:203
quote: doc["host_excluded"] = json!(self.host_excluded);
@row_add n=733 row=decision/peer-admitted-survivor-known-limit area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Peer admission survivor is a known limit
rule: A mutant making a client seam's peer admission always admit survives every committed test and stands as a known limit, read at each epoch-boundary audit.
why: One account can make no foreign peer, and no test seam may be added.
@row_check n=733 row=decision/peer-admitted-survivor-known-limit sha=cf528a6ee475 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
@row_add n=734 row=decision/report-files-per-test-process area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Report files per test process
rule: Test report files carry the test process's id, so two concurrent jobs never share one.
@row_check n=734 row=decision/report-files-per-test-process sha=880a61ef2324 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:520-521
quote: dir.join(format!("{test}.{pid}.report")), dir.join(format!("{test}.{pid}.test.report")),
@row_add n=735 row=decision/mutation-harness-tests-ungated area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3
name: Real cargo-mutants harness tests ungated
rule: The two real-cargo-mutants harness tests run ungated, never ignored, and the CI profile's 120 s kill and zero retries stay unchanged.
@row_check n=735 row=decision/mutation-harness-tests-ungated sha=628d814d04b7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:433
quote: fn run_mutants_passes_when_the_change_is_tested() {
at: crates/viola-e2e/src/harness/run/mutants.rs:410
quote: fn run_mutants_reports_survivors_of_an_untested_change() {
at: .config/nextest.toml:2
quote: retries = 0
at: .config/nextest.toml:4
quote: slow-timeout = { period = "30s", terminate-after = 4 }
@row_add n=736 row=decision/perf-panic-check-before-cleanup area=quality-gates-coverage-targets rec=fct-ftncprbj@2
name: Perf run panic check
rule: After the last perf row and before cleanup, the perf run fails as panic when the session home's role files hold any panic event line, with no seam exemption.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_check n=736 row=decision/perf-panic-check-before-cleanup sha=94f12bc57918 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:192
quote: check(&mut suite, role_panics(&home) == 0, "panic");
at: crates/viola-e2e/src/harness/run/perf.rs:100
quote: .filter(|v| v["event"] == "panic")
@row_add n=737 row=decision/perf-failure-turns-suite-red area=quality-gates-coverage-targets rec=fct-ftncprbj@2
name: Perf failures named, never skipped
rule: A failed build, boot, row, panic check or cleanup turns suite perf red with the failure named, never a missing or skipped row.
@row_check n=737 row=decision/perf-failure-turns-suite-red sha=e2b1a560a966 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:164
quote: fail(&mut suite, "build");
at: crates/viola-e2e/src/harness/run/perf.rs:187
quote: fail(&mut suite, &format!("boot: {reason}"));
at: crates/viola-e2e/src/harness/run/perf.rs:214
quote: check(suite, staged && code == Some(0), hook);
at: crates/viola-e2e/src/harness/run/perf.rs:192
quote: check(&mut suite, role_panics(&home) == 0, "panic");
at: crates/viola-e2e/src/harness/run/perf.rs:194
quote: check(&mut suite, cleaned.doc["ok"] == true, "cleanup");
@row_add n=738 row=decision/perf-hooks-time-the-real-path area=quality-gates-coverage-targets rec=fct-ftncprbj@2
name: Perf hooks time the real path
rule: Every timed hook runs with an instance name and directory set, so it takes the real logging and channel path, never the no-instance no-op, with its instance, server, socket-directory and peer checks inside the timed path.
@row_check n=738 row=decision/perf-hooks-time-the-real-path sha=5594c384899a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:120-121
quote: .env("VIOLA_NAME", INSTANCE) .env("VIOLA_DIR", dir)
@row_add n=739 row=decision/product-state-never-hand-written area=test-data-fixtures rec=fct-bxlupfu3@2
name: On-disk product state produced by viola
rule: On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written.
except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
@row_check n=739 row=decision/product-state-never-hand-written sha=1f807f95321d v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:386
quote: pub fn stamped_home(home: TestHome, fake_agent_path: PathBuf) -> StampedHome {
at: tests/chaos_snapshot_replay.rs:55
quote: let path = dir.join("snapshot.json");
@row_add n=740 row=decision/stamps-only-from-verify area=test-data-fixtures rec=fct-jg3hj3ma@1,fct-6upjfbpd@2
name: Stamps written only by verify
rule: The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it.
why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
@row_check n=740 row=decision/stamps-only-from-verify sha=b78587bf301b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/stamps.rs:3
quote: //! their shape belongs to viola-agent-claude. `update_stamps` is the one writer path, replacing the
at: tests/security_negatives_strict_modes.rs:251
quote: fn only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps(stamped_home: StampedHome) {
@row_add n=741 row=decision/budget-seeded-through-statusline-hook area=test-data-fixtures rec=fct-4ejdt5qs@1,fct-6upjfbpd@2
name: Budget seeded through the statusline hook
rule: A test's budget file is seeded by piping generated statusline stdin into viola hook statusline.
except: The budget reader's own unit cases pad, replace or re-version bytes the writer wrote, because no writer produces an over-cap, unparseable or v 2 file.
@row_check n=741 row=decision/budget-seeded-through-statusline-hook sha=86e55c8eb2f1 v=holds judge=sonnet head=8834195b0683
at: tests/support/statusline.rs:55
quote: .args(["hook", "statusline"])
at: crates/viola-state/src/budget.rs:250
quote: fn budget_read_returns_what_write_budget_wrote() {
@row_add n=742 row=decision/statusline-source-written-by-test area=test-data-fixtures rec=fct-652w7iir@2
name: Statusline source planted by the test
rule: The statusline source is written by the test, never by viola, which only reads it.
@row_check n=742 row=decision/statusline-source-written-by-test sha=4d6bc2f8f1d9 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:298
quote: pub fn plant_statusline_source(home: &Path, command: &str) -> PathBuf {
at: src/cmd/run.rs:532
quote: fn statusline_source(home: &Path, user_home: Option<&Path>) -> Option<PathBuf> {
@row_add n=743 row=decision/socket-dir-check-leaves-dir-as-found area=test-data-fixtures rec=fct-2mxqmdgt@1,fct-n2arkmru@2
name: Socket-directory check never repairs
rule: The socket-directory check never chmods the directory it checks; a refused socket directory is left as found.
@row_check n=743 row=decision/socket-dir-check-leaves-dir-as-found sha=77f8171ddd71 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:518
quote: (!absent || create_private_dir(dir).is_ok()) && check_endpoint_dir(endpoint).is_ok()
at: crates/viola-state/src/strict.rs:154
quote: pub fn check_endpoint_dir(endpoint: &str) -> Result<(), Refused> {
@row_add n=744 row=decision/foreign-owner-by-injected-uid area=test-data-fixtures rec=fct-2mxqmdgt@1
name: Foreign socket-directory owner by injected uid
rule: A socket directory with a foreign owner is not driven in tests; its verdict is taken by an injected uid.
@row_check n=744 row=decision/foreign-owner-by-injected-uid sha=43d24fff4101 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:161
quote: pub fn socket_dir_verdict(
at: tests/security_negatives_socket_dir.rs:6
quote: //! loosened; a foreign owner is `socket_dir_verdict`'s injected case, one account making none.
@row_add n=745 row=decision/agents-stub-from-files-no-env area=test-data-fixtures rec=fct-g6w3aucc@1,fct-6upjfbpd@2
name: Agents list answered by a stub on PATH
rule: claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH.
why: The no-env answer was ratified.
@row_check n=745 row=decision/agents-stub-from-files-no-env sha=5a3535ce3132 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:55
quote: const AGENTS_MODE: &str = "agents-mode";
at: tests/support/fake.rs:36
quote: pub fn claude_bin(scratch: &Path, name: &str, mode: Option<&str>, rows: Option<&[u8]>) -> PathBuf {
at: tests/support/cli.rs:127
quote: command.env("PATH", bin);
@row_add n=746 row=decision/hook-payloads-recorded-by-verify area=test-data-fixtures rec=fct-myaghpwm@2,fct-6upjfbpd@2
name: Hook payloads recorded by verify, replayed by CI
rule: Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
@row_check n=746 row=decision/hook-payloads-recorded-by-verify sha=6685262b356a v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:585-587
quote: let scrubbed = ledger::scrub(&payload, home, user); if let Some(why) = ledger::unclean(&scrubbed, user) { return Err(format!("{name} {}", why.code()));
at: tests/contract_fake_agent_drift.rs:575
quote: let planted: Vec<(String, TestHome, PathBuf, Vec<u8>)> = recorded_versions(&root)
@row_add n=747 row=contract/claude-fixtures/payload-path area=test-data-fixtures rec=fct-myaghpwm@2
boundary: Recorded Claude fixtures
item: payload path
promise: A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
@row_check n=747 row=contract/claude-fixtures/payload-path sha=2c929650ad7d v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:505
quote: self.set_file(&format!("{event}.{variant}.json"))
at: src/bin/viola-fake-agent.rs:509-511
quote: fn set_file(&self, name: &str) -> Option<Vec<u8>> { let dir = self.opts.fixtures.as_ref()?.join(self.opts.cli_version()); fs::read(dir.join(name)).ok()
@row_add n=748 row=contract/mcp-fixture-scrub-refusal/home-path area=test-data-fixtures rec=fct-beozd4ox@1
boundary: MCP fixture scrub refusal codes
item: home-path
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_check n=748 row=contract/mcp-fixture-scrub-refusal/home-path sha=dbcc64041dc0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1306
quote: Self::HomePath => "home-path",
at: crates/viola-agent-claude/src/ledger.rs:1431
quote: return Some(Unclean::HomePath);
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:214
quote: eprintln!("viola-mcp-tap: {name} {code}");
@row_add n=749 row=contract/mcp-fixture-scrub-refusal/absolute-path area=test-data-fixtures rec=fct-beozd4ox@1
boundary: MCP fixture scrub refusal codes
item: absolute-path
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_check n=749 row=contract/mcp-fixture-scrub-refusal/absolute-path sha=dfbd0fa610ba v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1307
quote: Self::AbsolutePath => "absolute-path",
at: crates/viola-agent-claude/src/ledger.rs:1329
quote: Some(Unclean::AbsolutePath)
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:214
quote: eprintln!("viola-mcp-tap: {name} {code}");
@row_add n=750 row=contract/mcp-fixture-scrub-refusal/username area=test-data-fixtures rec=fct-beozd4ox@1
boundary: MCP fixture scrub refusal codes
item: username
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_check n=750 row=contract/mcp-fixture-scrub-refusal/username sha=c98d789dd856 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1308
quote: Self::Username => "username",
at: crates/viola-agent-claude/src/ledger.rs:1331
quote: Some(Unclean::Username)
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:214
quote: eprintln!("viola-mcp-tap: {name} {code}");
@row_add n=751 row=contract/mcp-fixture-scrub-refusal/email area=test-data-fixtures rec=fct-beozd4ox@1
boundary: MCP fixture scrub refusal codes
item: email
promise: A closed refusal code of the MCP fixture scrub, naming the file only.
@row_check n=751 row=contract/mcp-fixture-scrub-refusal/email sha=eddca2f90288 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1309
quote: Self::Email => "email",
at: crates/viola-agent-claude/src/ledger.rs:1433
quote: text_fault(text, user).or_else(|| has_email(text).then_some(Unclean::Email))
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:214
quote: eprintln!("viola-mcp-tap: {name} {code}");
@row_add n=752 row=decision/human-output-expected-as-literals area=test-data-fixtures rec=fct-6wjlyq3w@2
name: Human-mode expected output as literals
rule: Human-mode expected output is held as literals in the test files that check it.
@row_check n=752 row=decision/human-output-expected-as-literals sha=51353136ef13 v=holds judge=sonnet head=8834195b0683
at: tests/cli_output_plain.rs:55
quote: const HELP: &str = concat!(
at: tests/cross_exit_causes.rs:72
quote: const CAUSES: &[Cause] = &[
@row_add n=753 row=decision/decision-bodies-as-insta-snapshots area=test-data-fixtures rec=fct-dvphj5bz@1
name: Decision bodies as insta snapshots
rule: Decision bodies are checked against insta snapshots, regenerated only by the local fixture refresh.
@row_check n=753 row=decision/decision-bodies-as-insta-snapshots sha=0ccc21d6eae9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:687
quote: insta::assert_snapshot!(
@row_add n=754 row=decision/fake-transcripts-compared-to-recordings area=test-data-fixtures rec=fct-moqhwhlq@1
name: Fake-agent transcripts compared byte for byte
rule: Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
@row_check n=754 row=decision/fake-transcripts-compared-to-recordings sha=adb9fbcfa738 v=holds judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:328
quote: .filter(|(_, h)| h["stdin_hex"].as_str().map(unhex).as_ref() == Some(&bytes))
@row_add n=755 row=decision/proptest-for-randomized-data area=test-data-fixtures rec=fct-tuwzhtch@1
name: proptest for randomized data
rule: Randomized test data comes from proptest strategies, with seeds persisted in committed regression files.
@row_check n=755 row=decision/proptest-for-randomized-data sha=ee3e5add7490 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:427
quote: proptest! {
at: crates/viola-core/proptest-regressions/lib.txt:1
quote: # Seeds for failure cases proptest has generated in the past. It is
@row_add n=756 row=decision/link-since-asserted-by-shape area=test-data-fixtures rec=fct-ovluqlfu@1
name: Link time asserted by shape
rule: The link line's since time is asserted by its shape only.
why: It is stamped outside the clock seam.
@row_check n=756 row=decision/link-since-asserted-by-shape sha=25bc9eb73043 v=holds judge=sonnet head=8834195b0683
at: tests/cli_links.rs:315
quote: fn is_clock_time(text: &str) -> bool {
@row_add n=757 row=contract/mcp-wait-tool/timeout-bound area=test-data-fixtures rec=fct-ovluqlfu@1
boundary: MCP wait tool
item: wait bound
promise: The wait bound is the given timeout capped at 50 000 ms, min(given, 50 000).
@row_check n=757 row=contract/mcp-wait-tool/timeout-bound sha=aac45225bc30 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:46
quote: given.map_or(WAIT_CAP, |given| given.min(WAIT_CAP))
at: crates/viola-mcp/src/tools.rs:42
quote: pub const WAIT_CAP: u64 = 50_000;
@row_add n=758 row=decision/test-endpoint-dirs-made-private area=test-data-fixtures rec=fct-fvd26vjh@1,fct-6upjfbpd@2
name: Test endpoint directories are private
rule: A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir.
why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
@row_check n=758 row=decision/test-endpoint-dirs-made-private sha=c10ef0ec6122 v=holds judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:204
quote: viola_state::fs::create_private_dir(socket_dir.path()).expect("the socket dir, private");
at: tests/security_negatives_server_identity.rs:94
quote: viola_state::fs::create_private_dir(socket_dir.path()).expect("the socket dir, private");
@row_add n=759 row=decision/fake-agent-copies-planted-first area=test-data-fixtures rec=fct-q4qwgsv5@1
name: Fake-agent copies planted before any child
rule: A test that plants more than one fake-agent copy plants every copy one after another before any thread starts a child.
why: On tmpfs each copy is written rather than hard-linked, and a child forked by one thread inherits another thread's open write handle (ETXTBSY).
@row_check n=759 row=decision/fake-agent-copies-planted-first sha=a98cf91505f7 v=holds judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:575
quote: let planted: Vec<(String, TestHome, PathBuf, Vec<u8>)> = recorded_versions(&root)
at: tests/contract_fake_agent_drift.rs:580
quote: let bin = fake::claude_bin(home.scratch(), "claude-bin", None, Some(&recorded));
@row_add n=760 row=contract/fake-agent-receipt/line-format area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: line format
promise: Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
@row_check n=760 row=contract/fake-agent-receipt/line-format sha=34a5c419b4c3 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:250
quote: let mut line = json!({"v": 1, "kind": kind});
at: src/bin/viola-fake-agent.rs:257
quote: let _ = f.write_all(&bytes);
@row_add n=761 row=contract/fake-agent-receipt/start area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: start
promise: Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
@row_check n=761 row=contract/fake-agent-receipt/start sha=3950124625a7 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:1162-1163
quote: let _terminal = viola_pty::HostTerminal::enter(); start_receipts(&agent);
at: src/bin/viola-fake-agent.rs:1078
quote: start_receipts(agent);
@row_add n=762 row=contract/fake-agent-receipt/size area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: size
promise: Carries cols and rows, written at start and on every terminal size change with no key needed after it; nothing while stdin/stdout is not a terminal, and none in print mode.
@row_check n=762 row=contract/fake-agent-receipt/size sha=ecf44c7955b3 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:1055
quote: if let Some(size) = now.filter(|s| Some(*s) != *last) {
at: src/bin/viola-fake-agent.rs:1056-1058
quote: agent .receipt .write("size", json!({"cols": size.cols, "rows": size.rows}));
@row_add n=763 row=contract/fake-agent-receipt/cwd area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: cwd
promise: Carries cwd.
@row_check n=763 row=contract/fake-agent-receipt/cwd sha=88cecc9af02e v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:925
quote: agent.receipt.write("cwd", json!({"cwd": cwd.ok()}));
@row_add n=764 row=contract/fake-agent-receipt/hold area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: hold
promise: Carries pid, written by the --exit-no-eof holder at its start.
@row_check n=764 row=contract/fake-agent-receipt/hold sha=79df86e7d68f v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:1142
quote: Receipt::open(opts.receipt.as_deref()).write("hold", json!({"pid": std::process::id()}));
@row_add n=765 row=contract/fake-agent-receipt/fds area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: fds
promise: Carries fds.
@row_check n=765 row=contract/fake-agent-receipt/fds sha=f84c12970c4d v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:940
quote: agent.receipt.write("fds", json!({"fds": fds}));
@row_add n=766 row=contract/fake-agent-receipt/key area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: key
promise: Carries hex, one line per byte received outside a paste.
@row_check n=766 row=contract/fake-agent-receipt/key sha=be453d0bc0d8 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:1112
quote: Action::Key(b) => agent.receipt.write("key", json!({"hex": hex(&[b])})),
@row_add n=767 row=contract/fake-agent-receipt/prompt area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: prompt
promise: Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
@row_check n=767 row=contract/fake-agent-receipt/prompt sha=08329a1e1cbe v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:718
quote: "prompt",
at: src/bin/viola-fake-agent.rs:720
quote: "text": text,
at: src/bin/viola-fake-agent.rs:721
quote: "hex": hex(bytes),
at: src/bin/viola-fake-agent.rs:722
quote: "bare_esc": bytes.contains(&0x1b),
at: src/bin/viola-fake-agent.rs:723
quote: "origin": origin,
at: src/bin/viola-fake-agent.rs:724
quote: "submit": submit,
at: src/bin/viola-fake-agent.rs:1113
quote: Action::Submit(bytes) => agent.submit(&bytes, "human"),
at: src/bin/viola-fake-agent.rs:784
quote: self.submit(HARNESS_TURN.as_bytes(), "harness");
at: src/bin/viola-fake-agent.rs:487-489
quote: Fired::NoHooks => "no-hooks", Fired::NoFixture => "no-fixture", Fired::Ran { .. } => "fired",
at: src/bin/viola-fake-agent.rs:686
quote: "suppressed"
at: src/bin/viola-fake-agent.rs:688
quote: "local-command"
@row_add n=768 row=contract/fake-agent-receipt/hook area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: hook
promise: Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
@row_check n=768 row=contract/fake-agent-receipt/hook sha=f947153d836b v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:621
quote: fields.insert("command_absolute".to_owned(), json!(absolute));
at: src/bin/viola-fake-agent.rs:643
quote: fields.insert("spawn_os_error".to_owned(), json!(e.raw_os_error()));
at: src/bin/viola-fake-agent.rs:651
quote: fields.insert("ran".to_owned(), json!(output.is_ok()));
at: src/bin/viola-fake-agent.rs:653
quote: fields.insert("exit_code".to_owned(), json!(out.status.code()));
at: src/bin/viola-fake-agent.rs:654
quote: fields.insert("stderr_len".to_owned(), json!(out.stderr.len()));
at: src/bin/viola-fake-agent.rs:655
quote: fields.insert("stdout_hex".to_owned(), json!(hex(&out.stdout)));
at: src/bin/viola-fake-agent.rs:656
quote: fields.insert("stdin_hex".to_owned(), json!(hex(body)));
@row_add n=769 row=contract/fake-agent-receipt/statusline area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: statusline
promise: Carries the hook kind's fields without event.
@row_check n=769 row=contract/fake-agent-receipt/statusline sha=9746a2cac058 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:602
quote: self.run_receipted("statusline", None, &command, &shell_expanded(args), &body);
@row_add n=770 row=contract/fake-agent-receipt/step area=test-data-fixtures rec=fct-mw3qahin@1
boundary: Fake-agent receipt
item: step
promise: Carries index and event.
@row_check n=770 row=contract/fake-agent-receipt/step sha=e34dec6cb1c7 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:782
quote: .write("step", json!({"index": index, "event": step.event}));
@row_add n=771 row=decision/fake-agent-argv-only area=test-data-fixtures rec=fct-vrevms4b@1
name: Fake agent configured by argv alone
rule: The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments.
@row_check n=771 row=decision/fake-agent-argv-only sha=beee374c4feb v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:107
quote: /// Unknown arguments are ignored, as the real CLI's other flags would be.
at: src/bin/viola-fake-agent.rs:150
quote: _ => {}
at: src/bin/viola-fake-agent.rs:926
quote: let mut names: Vec<String> = std::env::vars_os()
@row_add n=772 row=decision/harness-cli-version-matches-fake area=test-data-fixtures rec=fct-vrevms4b@1
name: One default CLI version
rule: The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
@row_check n=772 row=decision/harness-cli-version-matches-fake sha=7794e872b282 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:26
quote: const DEFAULT_CLI_VERSION: &str = "2.1.287";
at: crates/viola-e2e/src/harness/boot.rs:25
quote: pub const DEFAULT_CLI_VERSION: &str = "2.1.287";
at: src/bin/viola-fake-agent.rs:510
quote: let dir = self.opts.fixtures.as_ref()?.join(self.opts.cli_version());
@row_add n=773 row=decision/wrapper-boots-pass-trusted-screens area=test-data-fixtures rec=fct-vrevms4b@1
name: Test wrapper boots run trusted with screens
rule: Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd.
except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
@row_check n=773 row=decision/wrapper-boots-pass-trusted-screens sha=1b7f29fdd8b9 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:673-676
quote: args.push("--screens".into()); if trusted { args.push("--trusted-root".into()); args.push(cwd.clone().into());
at: crates/viola-e2e/src/harness/supervise.rs:65
quote: args.extend(["--screens", "--trusted-root"].map(OsString::from));
@row_add n=774 row=contract/process-exit-detail/unclean-fixture area=test-data-fixtures rec=fct-hc6txkos@1
boundary: process-exit detail
item: unclean-fixture
promise: verify refusing a recording as unclean writes, under a parsed VIOLA_NAME, a process-exit line with detail unclean-fixture, and the fixture's file name enters no code.
@row_check n=774 row=contract/process-exit-detail/unclean-fixture sha=486a4bdf9c08 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
at: src/cmd/verify.rs:81
quote: Ended::Refused(detail) => (1, Some(detail)),
at: src/cmd/verify.rs:91
quote: run::log_self_exit(code, detail);
@row_add n=775 row=decision/cross-user-at-peer-check-boundary area=test-scope-summary rec=fct-jnkzde5v@2
name: Cross-user rejection asserted at the boundary
rule: Rejection of a peer with a different euid is asserted at the peer-credential check boundary, over injected euids and an injected decision, with no seam added to the client wiring.
why: Hosted CI runners have no second OS account, so a foreign peer cannot be driven.
@row_check n=775 row=decision/cross-user-at-peer-check-boundary sha=27157c6d2801 v=holds guard=false judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:139
quote: pub fn peer_euid_admitted(observed: Option<u32>, own: u32) -> bool {
at: crates/viola-channel/src/client.rs:513-516
quote: #[case::same_user(Some(1000), 1000, true)] #[case::foreign_user(Some(0), 1000, false)] #[case::foreign_when_root(Some(1000), 0, false)] #[case::unreadable_peer(None, 1000, false)]
@row_add n=776 row=contract/wrapper-channel/max-frame area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Wrapper channel
item: MAX_FRAME
promise: A frame is at most 16 MiB.
@row_check n=776 row=contract/wrapper-channel/max-frame sha=8775aa1765b2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:11
quote: pub const MAX_FRAME: u64 = 16 * 1024 * 1024;
at: crates/viola-channel/src/frame.rs:18
quote: read_frame_within(reader, MAX_FRAME)
@row_add n=777 row=contract/cli-frame-verbs/server-verify-failed area=test-scope-summary rec=fct-jnkzde5v@2
boundary: CLI frame verbs
item: server-verify-failed
promise: A frame verb whose server is not the recorded wrapper sends no frame and exits 21, with detail server-verify-failed under --json.
@row_check n=777 row=contract/cli-frame-verbs/server-verify-failed sha=3c0557064b8c v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:194
quote: Err(CallError::ServerUnverified) => return Ok(Err(Unreachable::ServerUnverified)),
at: src/cmd/client.rs:252
quote: unreachable_document(cause.cause());
@row_add n=778 row=contract/hook/impostor-server area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Hook
item: impostor server
promise: A hook whose server is not the recorded wrapper sends no frame and exits 0 with no body.
@row_check n=778 row=contract/hook/impostor-server sha=3fa40df5a8dd v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:242
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(NotSent::ServerUnverified),
at: src/cmd/hook.rs:318
quote: Err(NotSent::ServerUnverified) => return Asked::ServerUnverified,
@row_add n=779 row=contract/channel-fault-screen/known-code area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Channel fault screen
item: known code
promise: Each of the five known codes takes its fixed message, with data null save the kept -32602 data.
@row_check n=779 row=contract/channel-fault-screen/known-code sha=639dc018a598 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:124-127
quote: -32700 => (ProtocolError::Parse.message(), Value::Null), -32600 => (ProtocolError::InvalidRequest.message(), Value::Null), -32601 => (ProtocolError::MethodNotFound.message(), Value::Null), -32603 => (ProtocolError::Internal.message(), Value::Null),
at: crates/viola-channel/src/lib.rs:141
quote: Some(json!({"code": code, "message": message, "data": data}))
@row_add n=780 row=contract/channel-fault-screen/-32602 area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Channel fault screen
item: -32602
promise: Keeps only an integer data.supported with a version-text data.wrapper, or a data.reason exactly release-from-driver; any other -32602 reads invalid params with data null.
@row_check n=780 row=contract/channel-fault-screen/-32602 sha=49e541568544 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:128-137
quote: -32602 => match (data["supported"].as_i64(), wrapper) { (Some(supported), Some(wrapper)) => ( ProtocolError::UnsupportedVersion.message(), json!({"supported": supported, "wrapper": wrapper}), ), _ if data["reason"] == "release-from-driver" => ( ProtocolError::ReleaseFromDriver.message(), json!({"reason": "release-from-driver"}), ),
at: crates/viola-channel/src/lib.rs:137
quote: _ => (ProtocolError::InvalidParams.message(), Value::Null),
@row_add n=781 row=contract/channel-fault-screen/unknown-code area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Channel fault screen
item: code outside the five
promise: Reads wrapper fault with data null.
@row_check n=781 row=contract/channel-fault-screen/unknown-code sha=1329f89d2e3b v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:139
quote: _ => ("wrapper fault", Value::Null),
@row_add n=782 row=contract/channel-fault-screen/no-integer-code area=test-scope-summary rec=fct-jnkzde5v@2
boundary: Channel fault screen
item: error without an integer code
promise: Reads nothing.
@row_check n=782 row=contract/channel-fault-screen/no-integer-code sha=1bab17211fca v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:118
quote: let code = error["code"].as_i64()?;
@row_add n=783 row=decision/real-claude-never-in-ci area=test-scope-summary rec=fct-zgqp2syf@2,fct-67zmkx76@3
name: Real claude CLI only locally
rule: The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
@row_check n=783 row=decision/real-claude-never-in-ci sha=ecef730eb58d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:56
quote: /// `--mutants` is the epoch-boundary audit's, and `--local-live` drives the real CLI, so none of
searched: verify · in .github/workflows/ci.yml
searched: claude · in .github/workflows/ci.yml
@row_add n=784 row=contract/revive-refusal-code/strict-modes-failed area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: strict-modes-failed
promise: A closed code of the revive --json refusal document.
@row_check n=784 row=contract/revive-refusal-code/strict-modes-failed sha=ce78839ae666 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:70
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=785 row=contract/revive-refusal-code/already-live area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: already-live
promise: A closed code of the revive --json refusal document.
@row_check n=785 row=contract/revive-refusal-code/already-live sha=375414a91844 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=786 row=contract/revive-refusal-code/no-session area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: no-session
promise: A closed code of the revive --json refusal document; also the document of an empty --list chain.
@row_check n=786 row=contract/revive-refusal-code/no-session sha=2e525f7d6978 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
at: src/cmd/revive.rs:283
quote: Refusal::NoSession.write(name, json);
@row_add n=787 row=contract/revive-refusal-code/session-live area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: session-live
promise: A closed code of the revive --json refusal document.
@row_check n=787 row=contract/revive-refusal-code/session-live sha=67cba8d38522 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=788 row=contract/revive-refusal-code/cwd-missing area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: cwd-missing
promise: A closed code of the revive --json refusal document.
@row_check n=788 row=contract/revive-refusal-code/cwd-missing sha=1ab18a488ad6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=789 row=contract/revive-refusal-code/stale-heartbeat area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: stale-heartbeat
promise: A closed code of the revive --json refusal document.
@row_check n=789 row=contract/revive-refusal-code/stale-heartbeat sha=7969f981ae05 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=790 row=contract/revive-refusal-code/pinned-hash-mismatch area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: pinned-hash-mismatch
promise: A closed code of the revive --json refusal document.
@row_check n=790 row=contract/revive-refusal-code/pinned-hash-mismatch sha=1a87287a8122 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=791 row=contract/revive-refusal-code/squatted-name area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: squatted-name
promise: A closed code of the revive --json refusal document.
@row_check n=791 row=contract/revive-refusal-code/squatted-name sha=13c70f5f3786 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=792 row=contract/revive-refusal-code/socket-dir-refused area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: socket-dir-refused
promise: A closed code of the revive --json refusal document.
@row_check n=792 row=contract/revive-refusal-code/socket-dir-refused sha=ad46c4ae0c8b v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=793 row=contract/revive-refusal-code/internal-error area=test-scope-summary rec=fct-xttglbra@2
boundary: revive refusal codes
item: internal-error
promise: A closed code of the revive --json refusal document.
@row_check n=793 row=contract/revive-refusal-code/internal-error sha=a6dbeb646622 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:124
quote: client::refused_document("internal-error");
@row_add n=794 row=contract/revive-json/list-document area=test-scope-summary rec=fct-xttglbra@2
boundary: revive --json
item: --list document
promise: revive --list --json prints {"v":1,"ok":{"sessions":[…]}} in log order.
@row_check n=794 row=contract/revive-json/list-document sha=f19b0c4dfa13 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:292
quote: client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
@row_add n=795 row=contract/revive-json/pump-child-error area=test-scope-summary rec=fct-xttglbra@2
boundary: revive --json
item: child pump error
promise: An error while pumping the child prints no document, the child having held the terminal.
@row_check n=795 row=contract/revive-json/pump-child-error sha=0a1d37e698eb v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:122-128
quote: let started = started(home, args).inspect_err(|_| { if json { client::refused_document("internal-error"); } })?; match started { Started::Launched(launched) => pump_child(*launched),
@row_add n=796 row=contract/revive-exit-codes/0 area=test-scope-summary rec=fct-xttglbra@2
boundary: revive exit codes
item: 0
promise: Success.
@row_check n=796 row=contract/revive-exit-codes/0 sha=4b69bbc4b0a0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:299
quote: Ok(ExitCode::SUCCESS)
@row_add n=797 row=contract/revive-exit-codes/1 area=test-scope-summary rec=fct-xttglbra@2
boundary: revive exit codes
item: 1
promise: A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start.
@row_check n=797 row=contract/revive-exit-codes/1 sha=41a835835ab3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:139
quote: return Ok(Started::Refused(ExitCode::from(1)));
@row_add n=798 row=contract/revive-exit-codes/2 area=test-scope-summary rec=fct-xttglbra@2
boundary: revive exit codes
item: 2
promise: A clap usage error, a malformed --id included.
@row_check n=798 row=contract/revive-exit-codes/2 sha=56d463077059 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:52
quote: Err(usage) => usage.exit(),
at: src/cmd/revive.rs:50
quote: Err("invalid session id".to_owned())
@row_add n=799 row=decision/frame-verbs-check-instance-first area=test-scope-summary rec=fct-xttglbra@2
name: Frame verbs check the instance first
rule: All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
@row_check n=799 row=decision/frame-verbs-check-instance-first sha=adbf3875c927 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/wait.rs:43
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/last.rs:30
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/answer.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/pause.rs:32
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/release.rs:35
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/link.rs:87-92
quote: pub(super) fn link_call( home: &Path, args: &LinkArgs, verb: Verb, request: fn(&LiveEndpoint, Map<String, Value>) -> Result<Value, CallError>, ) -> anyhow::Result<u8> {
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=800 row=decision/self-link-refused-by-wrapper area=test-scope-summary rec=fct-xttglbra@2,fct-5qkdel63@2
name: Self-link refused by the wrapper
rule: A link whose driver equals its driven is refused by the wrapper alone, never client-side: -32602 with data null on the channel and CLI exit 20.
@row_check n=800 row=decision/self-link-refused-by-wrapper sha=84afd6ae8259 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:95
quote: if driven != self.name || driver == driven {
at: crates/viola-channel/src/lib.rs:97
quote: _ => Value::Null,
@row_add n=801 row=absence/windows-and-macos-support area=test-scope-summary rec=fct-ze6ljnvw@1
capability: Windows and macOS support: code, tests, fixtures and CI jobs
@row_check n=801 row=absence/windows-and-macos-support sha=6712269d1526 v=holds judge=sonnet head=8834195b0683
searched: windows-latest · in .github
searched: macos-latest · in .github
searched: cfg(windows) · in src/**, crates/**, tests/** (only tests/contract_os_gated_tests.rs, a lint that forbids such gates)
searched: os: [ubuntu-latest] · in .github/workflows/ci.yml (every matrix, lines 18, 130, 263, 325)
@row_add n=802 row=decision/cli-json-one-document area=test-scope-summary rec=fct-fnukifof@4
name: One JSON document per verb
rule: Under --json a CLI verb signals by its exit code plus one JSON document on stdout.
except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
guard: NOT_ERROR_BODIES
@row_check n=802 row=decision/cli-json-one-document sha=daf7c2915f3d v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/answer.rs:67
quote: return client::document(&json!({"v": 1, "ok": ok}));
at: src/cmd/answer.rs:74
quote: return client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));
at: src/cmd/send.rs:164
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
at: src/cmd/answer.rs:91
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
guard_at: tests/contract_error_bodies.rs:365
@row_add n=803 row=decision/no-sgr-without-tty area=test-scope-summary rec=fct-fnukifof@4
name: No SGR without a terminal
rule: CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb.
@row_check n=803 row=decision/no-sgr-without-tty sha=500bd4a0f689 v=holds judge=sonnet head=8834195b0683
at: src/human/style.rs:62
quote: if facts.json
at: src/human/style.rs:65
quote: || facts.term.as_deref() == Some("dumb")
at: src/human/style.rs:66
quote: || !facts.stdout_tty
at: src/human/style.rs:68
quote: return None;
@row_add n=804 row=decision/human-tty-through-outer-pty area=test-scope-summary rec=fct-fnukifof@4
name: Human TTY rendering through the outer PTY
rule: The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent.
guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
@row_check n=804 row=decision/human-tty-through-outer-pty sha=d8218bf588a8 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cross_list_rows.rs:99
quote: let mut pty = OuterPty::spawn_sized(
at: tests/cross_list_rows.rs:20
quote: use support::outer_pty::{EXIT_WITHIN, OuterPty};
guard_at: tests/cross_list_rows.rs:266
@row_add n=805 row=contract/cli-human-lines/read-back area=test-scope-summary rec=fct-fnukifof@4
boundary: CLI human lines
item: read back
promise: [RB] read back … cursor <n>
@row_check n=805 row=contract/cli-human-lines/read-back sha=9c18cc656d17 v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:25-26
quote: let rest = format!("{submitted}  cursor {cursor}"); out.write_all(mirror_line("[RB]", "read back", name, &rest).as_bytes())
@row_add n=806 row=contract/cli-human-lines/unable area=test-scope-summary rec=fct-fnukifof@4
boundary: CLI human lines
item: unable
promise: unable  <name>  <reason>  <detail>, followed by a last-line hint:.
@row_check n=806 row=contract/cli-human-lines/unable sha=d8b2fac6d6cd v=holds judge=sonnet head=8834195b0683
at: src/human/mirror.rs:64
quote: let mut text = format!("unable  {name}  {reason}\n");
at: src/human/mirror.rs:66
quote: text.push_str(&format!("hint: {hint}\n"));
@row_add n=807 row=contract/cli-human-lines/stamped area=test-scope-summary rec=fct-fnukifof@4
boundary: CLI human lines
item: stamped
promise: stamped <ver>  <n> pass  <n> fail
@row_check n=807 row=contract/cli-human-lines/stamped sha=e6110c5bd94f v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:294
quote: "stamped {version}  {} pass  {failed} fail",
@row_add n=808 row=contract/revive-human/refusal area=test-scope-summary rec=fct-fnukifof@4
boundary: revive human output
item: refusal
promise: Without --json a refusal writes one unable: line and a last-line hint: on stderr, with stdout empty.
@row_check n=808 row=contract/revive-human/refusal sha=4489667a4ca4 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
@row_add n=809 row=contract/revive-human/list-row area=test-scope-summary rec=fct-fnukifof@4
boundary: revive human output
item: --list row
promise: One <ts>  <cause>  <id> row per logged session on stdout.
@row_check n=809 row=contract/revive-human/list-row sha=7b44f8cc63c6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:313
quote: format!("{}  {}  {}", self.ts, self.cause, self.id)
@row_add n=810 row=decision/never-parse-rendered-child-screen area=test-scope-summary rec=fct-b2j3wr36@3
name: Rendered child screen never parsed
rule: Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search.
why: Named a test anti-pattern by the creator.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_check n=810 row=decision/never-parse-rendered-child-screen sha=33c6c3901c74 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/chaos_feed_panic.rs:116
quote: while !wrapper.shown().windows(3).any(|w| w == [0xe4, 0xb8, 0xad]) {
guard_at: tests/chaos_feed_panic.rs:44
@row_add n=811 row=contract/wrapper-channel/success area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: success
promise: {"result":{"ok":…}}
@row_check n=811 row=contract/wrapper-channel/success sha=05d44e6167c1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:95
quote: json!({"jsonrpc": "2.0", "id": id, "result": result})
at: src/run/links.rs:75
quote: Ok(json!({"ok": {"since": since, "already_linked": already_linked}}))
@row_add n=812 row=contract/wrapper-channel/refusal area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: refusal
promise: {"result":{"refusal":…,"detail":…}}, the first applicable refusal in order.
@row_check n=812 row=contract/wrapper-channel/refusal sha=bbe60e685dfb v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:271
quote: let mut data = json!({"refusal": refusal.as_str(), "detail": detail});
at: src/run/send.rs:96-101
quote: if let Some(detail) = slot.wheel.human_typing() { return ctx.refuse( None, RefusalReason::HumanTyping, detail.map(HumanTyping::as_str), );
@row_add n=813 row=contract/wrapper-channel/protocol-fault area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: protocol fault
promise: {"error":{code,message,data}}.
@row_check n=813 row=contract/wrapper-channel/protocol-fault sha=e831bf7a76cc v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:100
quote: json!({"jsonrpc": "2.0", "id": id, "error": fault.body()})
at: crates/viola-channel/src/lib.rs:99
quote: json!({"code": self.code(), "message": self.message(), "data": data})
@row_add n=814 row=contract/wrapper-channel/-32603 area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: -32603
promise: internal error with data null.
@row_check n=814 row=contract/wrapper-channel/-32603 sha=67dba99899ab v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
at: crates/viola-channel/src/lib.rs:97
quote: _ => Value::Null,
@row_add n=815 row=contract/wrapper-channel/link area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: link
promise: Served by the driven instance's wrapper; answers ok {since, already_linked}; a missing, non-string or invalid name or a driven that is not the serving wrapper's name is -32602 with data null and no line; a failed record is -32603.
@row_check n=815 row=contract/wrapper-channel/link sha=b8f9aeb5b83d v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:74-75
quote: let (since, already_linked) = self.record(&driver).map_err(|_| ProtocolError::Internal)?; Ok(json!({"ok": {"since": since, "already_linked": already_linked}}))
at: src/run/links.rs:92
quote: .ok_or(ProtocolError::InvalidParams)
at: src/run/links.rs:95-96
quote: if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
@row_add n=816 row=contract/wrapper-channel/unlink area=test-scope-summary rec=fct-5qkdel63@2
boundary: Wrapper channel
item: unlink
promise: Served by the driven instance's wrapper; answers an empty ok object; invalid names are -32602 with data null and no line; a failed remove is -32603.
@row_check n=816 row=contract/wrapper-channel/unlink sha=265ca7247973 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:81-82
quote: self.remove(&driver).map_err(|_| ProtocolError::Internal)?; Ok(json!({"ok": {}}))
at: src/run/links.rs:92
quote: .ok_or(ProtocolError::InvalidParams)
@row_add n=817 row=contract/channel-response-line/detail area=test-scope-summary rec=fct-5qkdel63@2
boundary: channel-response log line
item: detail
promise: On the server's and the client's WARN line, closed to unsupported-version, invalid-params or release-from-driver for the three -32602 faults and absent on every other line, with no name or param.
@row_check n=817 row=contract/channel-response-line/detail sha=ead17a7eac54 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
at: crates/viola-channel/src/lib.rs:105-107
quote: Self::UnsupportedVersion => Some("unsupported-version"), Self::InvalidParams => Some("invalid-params"), Self::ReleaseFromDriver => Some("release-from-driver"),
at: crates/viola-channel/src/client.rs:77
quote: let mut logged = ResponseLine::client(started, id, &self.conn, label);
@row_add n=818 row=decision/channel-error-bodies-literal area=test-scope-summary rec=fct-5qkdel63@2
name: Channel error bodies are literals
rule: The channel's protocol error bodies are fixed literal wire objects with v 1, holding no path or upstream text.
guard: channel_error_bodies_hold_no_path_or_upstream_text
@row_check n=818 row=decision/channel-error-bodies-literal sha=d1cfed6f0186 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:95
quote: Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
at: crates/viola-channel/src/lib.rs:99
quote: json!({"code": self.code(), "message": self.message(), "data": data})
guard_at: tests/channel_error_bodies.rs:98
@row_add n=819 row=contract/mcp-tool-result/ok area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: ok
promise: isError false with structuredContent equal to the ok payload.
@row_check n=819 row=contract/mcp-tool-result/ok sha=1caebb7c2de8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:16
quote: CallToolResult::structured(payload)
@row_add n=820 row=contract/mcp-tool-result/refusal area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: refusal
promise: isError true with {refusal, detail} and text "refused: <reason>".
@row_check n=820 row=contract/mcp-tool-result/refusal sha=fc46e4bf0087 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=821 row=contract/mcp-tool-result/instance-unreachable area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: instance-unreachable
promise: isError true with {"error":"instance-unreachable","detail":"<cause>"}, the CLI exit-21 document less v, text refused: <code>.
@row_check n=821 row=contract/mcp-tool-result/instance-unreachable sha=cc38e171028e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
@row_add n=822 row=contract/instance-unreachable-cause/strict-modes-failed area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: strict-modes-failed
promise: A closed cause word of instance-unreachable's detail.
@row_check n=822 row=contract/instance-unreachable-cause/strict-modes-failed sha=7477fa013f6c v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
@row_add n=823 row=contract/instance-unreachable-cause/instance-dead area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: instance-dead
promise: A closed cause word of instance-unreachable's detail.
@row_check n=823 row=contract/instance-unreachable-cause/instance-dead sha=4ddc6d6f751a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
@row_add n=824 row=contract/instance-unreachable-cause/unwrapped-name area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: unwrapped-name
promise: A closed cause word of instance-unreachable's detail.
@row_check n=824 row=contract/instance-unreachable-cause/unwrapped-name sha=53a454b54107 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
@row_add n=825 row=contract/instance-unreachable-cause/endpoint-lost area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: endpoint-lost
promise: A closed cause word of instance-unreachable's detail.
@row_check n=825 row=contract/instance-unreachable-cause/endpoint-lost sha=b67e9f71ddc8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
@row_add n=826 row=contract/instance-unreachable-cause/server-verify-failed area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: server-verify-failed
promise: A closed cause word of instance-unreachable's detail.
@row_check n=826 row=contract/instance-unreachable-cause/server-verify-failed sha=aa3a990fbda4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
@row_add n=827 row=contract/instance-unreachable-cause/socket-dir-refused area=test-scope-summary rec=fct-elurmfjv@1
boundary: instance-unreachable causes
item: socket-dir-refused
promise: A closed cause word of instance-unreachable's detail.
@row_check n=827 row=contract/instance-unreachable-cause/socket-dir-refused sha=3804e40e1d5f v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
@row_add n=828 row=contract/mcp-tool-result/wrapper-fault area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: wrapper-fault
promise: isError true with {"error":"wrapper-fault","detail":{code,message,data}}, the detail being the channel fault screen's.
@row_check n=828 row=contract/mcp-tool-result/wrapper-fault sha=5d00c6686d95 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:60-61
quote: return sanitised_fault(fault)             .map(|detail| error("wrapper-fault", detail))
@row_add n=829 row=contract/mcp-tool-result/invalid-params area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP tool result
item: invalid-params
promise: isError true with {"error":"invalid-params","detail":"<field>"}, no frame written.
@row_check n=829 row=contract/mcp-tool-result/invalid-params sha=ec8e62cc8624 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
at: crates/viola-mcp/src/tools.rs:372
quote: (Err(field), _) | (_, Err(field)) => return Ok(result::invalid(field)),
@row_add n=830 row=contract/mcp-invalid-params-field/target area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: target
promise: A closed field word of invalid-params' detail.
@row_check n=830 row=contract/mcp-invalid-params-field/target sha=94825e5d7d79 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:266
quote: .ok_or("target")
@row_add n=831 row=contract/mcp-invalid-params-field/text area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: text
promise: A closed field word of invalid-params' detail.
@row_check n=831 row=contract/mcp-invalid-params-field/text sha=3d0035306e82 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:270
quote: args.get("text").and_then(Value::as_str).ok_or("text")
@row_add n=832 row=contract/mcp-invalid-params-field/after area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: after
promise: A closed field word of invalid-params' detail.
@row_check n=832 row=contract/mcp-invalid-params-field/after sha=dcca7e5d9433 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:386
quote: optional_u64(args, "after")?,
@row_add n=833 row=contract/mcp-invalid-params-field/timeout-ms area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: timeout_ms
promise: A closed field word of invalid-params' detail.
@row_check n=833 row=contract/mcp-invalid-params-field/timeout-ms sha=3a9a0c069cbb v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:387
quote: optional_u64(args, "timeout_ms")?,
@row_add n=834 row=contract/mcp-invalid-params-field/dialog-id area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: dialog_id
promise: A closed field word of invalid-params' detail.
@row_check n=834 row=contract/mcp-invalid-params-field/dialog-id sha=164611ef9d71 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:284
quote: .ok_or("dialog_id")
@row_add n=835 row=contract/mcp-invalid-params-field/response area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP invalid-params fields
item: response
promise: A closed field word of invalid-params' detail.
@row_check n=835 row=contract/mcp-invalid-params-field/response sha=77b98a01fd88 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:291
quote: .ok_or("response")
@row_add n=836 row=contract/mcp-server/internal-fault area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP server
item: internal handler fault
promise: Any other channel error, a reply of no known shape, a wait or last refusal or a panicked call is the JSON-RPC error -32603 internal error with data null.
@row_check n=836 row=contract/mcp-server/internal-fault sha=e54ca79488dc v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:243
quote: Ok(Err(McpError::Internal)) | Err(_) => Err(internal_fault()),
at: crates/viola-mcp/src/tools.rs:194-196
quote: ErrorCode::INTERNAL_ERROR, McpError::Internal.to_string(), Some(Value::Null),
@row_add n=837 row=contract/mcp-server/unknown-tool area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP server
item: unknown tool
promise: The JSON-RPC error -32602.
@row_check n=837 row=contract/mcp-server/unknown-tool sha=ca2c88422bc4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:234
quote: return Err(ErrorData::invalid_params(
@row_add n=838 row=contract/mcp-server/tools-list area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP server
item: tools/list
promise: Exactly the five tools send, wait, last, answer and list, its result carrying ttlMs 0 and cacheScope private.
@row_check n=838 row=contract/mcp-server/tools-list sha=497e3bf71743 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
at: crates/viola-mcp/src/tools.rs:167-168
quote: .with_ttl_ms(0) .with_cache_scope(CacheScope::Private)
@row_add n=839 row=decision/mcp-no-control-tools area=test-scope-summary rec=fct-elurmfjv@1
name: No control tools over MCP
rule: The MCP server offers no release, pause, link or unlink tool.
@row_check n=839 row=decision/mcp-no-control-tools sha=25f947ae6584 v=holds judge=sonnet head=8834195b0683
searched: "release" · in crates/viola-mcp/src (only tools.rs:483 and :652, tests asserting the words are absent)
searched: "pause" · in crates/viola-mcp/src (same)
searched: "link" · in crates/viola-mcp/src (same)
searched: "unlink" · in crates/viola-mcp/src (same)
searched: TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"] · in crates/viola-mcp/src/tools.rs:37
@row_add n=840 row=decision/mcp-negatives-violate-schema area=test-scope-summary rec=fct-elurmfjv@1
name: MCP negatives violate the advertised schema
rule: MCP tests send negative inputs that violate the advertised tool schemas.
why: The schemas are advisory.
@row_check n=840 row=decision/mcp-negatives-violate-schema sha=dc2352a8efa9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/tests/mcp_tools.rs:313
quote: let violations = [
at: crates/viola-e2e/tests/mcp_tools.rs:330
quote: json!({"error": "invalid-params", "detail": "target"}),
@row_add n=841 row=decision/mcp-frame-tools-check-instance-first area=test-scope-summary rec=fct-elurmfjv@1
name: MCP frame tools check the instance first
rule: The MCP frame tools send, wait, last and answer run the instance strict-modes check before the liveness pre-check, a refused instance getting no frame and no line.
guard: handlers_frame_tools_refuse_a_widened_instance_before_any_frame
@row_check n=841 row=decision/mcp-frame-tools-check-instance-first sha=d028eec84669 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339
quote: if check_instance(&ctx.home, &instance_dir).is_err() {
at: crates/viola-mcp/src/tools.rs:342
quote: let Some(live) = live_endpoint(&instance_dir) else {
guard_at: crates/viola-mcp/tests/handlers.rs:444
@row_add n=842 row=contract/mcp-reply-screen/refusal area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP reply screen
item: refusal
promise: A refusal that is no code word reads unknown.
@row_check n=842 row=contract/mcp-reply-screen/refusal sha=1e79366493cd v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:73-77
quote: let reason = if is_code_word(reason) { reason } else { RefusalReason::Unknown.as_str() };
@row_add n=843 row=contract/mcp-reply-screen/detail area=test-scope-summary rec=fct-elurmfjv@1
boundary: MCP reply screen
item: detail
promise: A string detail that is no code word reads null, and a non-string detail is no known reply.
@row_check n=843 row=contract/mcp-reply-screen/detail sha=b10461b2a0e0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
at: crates/viola-mcp/src/result.rs:81
quote: Some(_) => return Err(McpError::Internal),
@row_add n=844 row=contract/mcp-stdin/oversize-before-initialize area=test-scope-summary rec=fct-elurmfjv@1
boundary: viola mcp stdin
item: line past MAX_FRAME before initialize
promise: viola mcp exits 1 with both streams empty and one parse-rejected mcp-stdin oversize line.
@row_check n=844 row=contract/mcp-stdin/oversize-before-initialize sha=ac6451feb2e8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:80
quote: Err(_) => Err(ServeError::Initialize),
at: crates/viola-mcp/src/lib.rs:86-87
quote: if cut.load(Ordering::Relaxed) { reject_oversize_line();
at: src/main.rs:136-138
quote: Role::Mcp | Role::Cli | Role::Other, Outcome::Unparsed | Outcome::Failed | Outcome::Panicked, ) => ExitCode::from(1),
@row_add n=845 row=absence/browser-suite area=test-scope-summary rec=fct-67zmkx76@3
capability: A browser test suite
@row_check n=845 row=absence/browser-suite sha=994b2c75697d v=holds judge=sonnet head=8834195b0683
searched: playwright · in src/**, crates/**, tests/**, .github/**, scripts/** (only crates/viola-e2e/src/harness/gate.rs:416-420 and run.rs:764, tests that refuse it as a suite value)
searched: selenium|cypress|webdriver|chromium|puppeteer · in src/**, crates/**, tests/**, .github/**, scripts/**
@row_add n=846 row=decision/mutation-testing-on-demand area=test-scope-summary rec=fct-67zmkx76@3
name: Mutation testing on demand
rule: Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
@row_check n=846 row=decision/mutation-testing-on-demand sha=1ace402dde2a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:307
quote: if sel.mutants && refusal.is_none() {
at: crates/viola-e2e/src/harness/run.rs:61-63
quote: unit: named.unit || all || none, integration: named.integration || all || none, e2e: named.e2e || all || none,
at: crates/viola-e2e/src/harness/run.rs:56
quote: /// `--mutants` is the epoch-boundary audit's, and `--local-live` drives the real CLI, so none of
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
@row_add n=847 row=contract/harness-run/exit area=test-scope-summary rec=fct-67zmkx76@3
boundary: Harness run command
item: exit
promise: Exits 0 only if every suite passes and no mutant survives; non-zero otherwise, with a JSON summary on stdout of suite, passed, failed and survived.
@row_check n=847 row=contract/harness-run/exit sha=62a1b7f402a5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:88-90
quote: fn green(&self) -> bool { self.failed == 0 && self.survived == 0 }
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
at: crates/viola-e2e/src/harness/run.rs:485
quote: let ok = merged.iter().all(Suite::green);
@row_add n=848 row=contract/harness-cleanup/idempotent area=test-scope-summary rec=fct-67zmkx76@3
boundary: Harness cleanup command
item: second run
promise: A second cleanup exits 0 and reports nothing to clean.
@row_check n=848 row=contract/harness-cleanup/idempotent sha=efc934106e51 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:54
quote: json!({"v": 1, "cmd": "cleanup", "ok": true, "cleaned": []}),
@row_add n=849 row=contract/harness-cleanup/endpoint-gone area=test-scope-summary rec=fct-67zmkx76@3
boundary: Harness cleanup command
item: endpoint check
promise: Stops every booted wrapper, confirming exit on the process handle, deletes the temp home and verifies the endpoint is gone; it removes no leftover socket file.
@row_check n=849 row=contract/harness-cleanup/endpoint-gone sha=b9b97d809ae4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:107
quote: let processes_gone = targets.iter().all(|(_, id)| id.wait_gone(deadline));
at: crates/viola-e2e/src/harness/cleanup.rs:110
quote: recorded.as_deref().is_some_and(unconnectable)
at: crates/viola-e2e/src/harness/cleanup.rs:116
quote: json!(remove_session_home(ws, &record.home))
@row_add n=850 row=decision/events-never-truncated area=test-scope-summary rec=fct-67zmkx76@3
name: Events never truncated
rule: Instance events are never truncated; their retention in tests is the lifetime of the temp home.
@row_check n=850 row=decision/events-never-truncated sha=f89aa6217168 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:2
quote: //! exclusive lock on the `events.ndjson.lock` sibling, never truncated or rewritten (architecture
at: crates/viola-state/src/events.rs:92
quote: write_line(instance_dir, &tail, &bytes)?;
searched: truncate|set_len · in crates/viola-state/src/events.rs and src/** (no events.ndjson truncation; hits are unrelated prompt/body/slot truncation and test-only set_len)
@row_add n=851 row=decision/per-test-fresh-home area=test-scope-summary rec=fct-6upjfbpd@2
name: Self-bootstrapping per-test home
rule: Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
@row_check n=851 row=decision/per-test-fresh-home sha=22ea5e898f2d v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:620
quote: let mut args: Vec<OsString> = vec!["--home".into(), home.clone().into()];
at: tests/support/home.rs:500
quote: viola_state::fs::create_private_dir(&self.dir()).expect("the socket directory");
@row_add n=852 row=decision/fuzz-the-parser-surfaces area=test-scope-summary rec=fct-6upjfbpd@2
name: Property and fuzz generators on parsers
rule: Property and fuzz generators cover the parser surfaces the security plan lists.
@row_check n=852 row=decision/fuzz-the-parser-surfaces sha=35327d78f561 v=holds judge=sonnet head=8834195b0683
at: fuzz/Cargo.toml:39
quote: path = "fuzz_targets/hook_stdin.rs"
at: fuzz/Cargo.toml:53
quote: path = "fuzz_targets/paste_text.rs"
at: fuzz/Cargo.toml:60
quote: path = "fuzz_targets/claude_agents_json.rs"
at: fuzz/Cargo.toml:46
quote: path = "fuzz_targets/vt100_feed.rs"
@row_add n=853 row=decision/fixtures-never-from-real-prompts area=test-scope-summary rec=fct-6upjfbpd@2
name: Fixtures never from real prompts
rule: Fixtures must not come from real prompts and must be checked for absolute paths and usernames.
why: A security anti-pattern.
@row_check n=853 row=decision/fixtures-never-from-real-prompts sha=2b095535c0bc v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:586
quote: if let Some(why) = ledger::unclean(&scrubbed, user) {
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
@row_add n=854 row=decision/run-checks-before-child-spawn area=test-scope-summary rec=fct-n2arkmru@2
name: viola run checks before the child
rule: viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child.
guard: run_refuses_a_widened_home_tree_before_its_log
@row_check n=854 row=decision/run-checks-before-child-spawn sha=bf2cb7cb35ad v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:203
quote: Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
at: src/cmd/run.rs:278
quote: if let Some(refusal) = collision_check(name, &instance_dir, recovered.as_ref(), launch.json) {
at: src/cmd/run.rs:281
quote: let Some((pinned, plugin_dir)) = pin_and_plugin(&home, exe)? else {
at: src/cmd/run.rs:290
quote: let gate = version_gate(&home, &program, &cwd, &strip);
at: src/cmd/run.rs:497
quote: if !socket_dir_ready(&endpoint) {
at: src/cmd/run.rs:500
quote: match Server::bind(&endpoint) {
at: src/cmd/run.rs:366
quote: let (beat, snapshot) = start_state(
at: src/cmd/run.rs:390
quote: let spawned = spawn_child(
guard_at: tests/run_cli.rs:611
@row_add n=855 row=contract/run-start-events/order area=test-scope-summary rec=fct-n2arkmru@2
boundary: run start events
item: order
promise: The events log reads wheel with cause start, then budget-gate, then the agent's session-start with source hook.
@row_check n=855 row=contract/run-start-events/order sha=3f893d991d03 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:620
quote: json!({"holder": Wheel::Driver.as_str(), "cause": WheelCause::Start.as_str()}),
at: src/cmd/run.rs:622
quote: (EventKind::BudgetGate, budget.data()),
at: src/cmd/run.rs:111
quote: Source::Hook,
@row_add n=856 row=contract/run-refusal/socket-dir-refused area=test-scope-summary rec=fct-n2arkmru@2
boundary: run and revive start refusals
item: socket-dir-refused
promise: A refused socket directory exits 1 with no bind and no child, the directory left as found, and one process-exit line of detail socket-dir-refused.
@row_check n=856 row=contract/run-refusal/socket-dir-refused sha=ef1e4a42994c v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:497-498
quote: if !socket_dir_ready(&endpoint) { return Ok(Bound::SocketDirRefused);
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:412-413
quote: run::log_self_exit(1, Some(detail)); Started::Refused(ExitCode::from(1))
@row_add n=857 row=contract/run-refusal/stale-heartbeat area=test-scope-summary rec=fct-n2arkmru@2
boundary: run and revive start refusals
item: stale-heartbeat
promise: A stale holder refuses the start with exit 1.
@row_check n=857 row=contract/run-refusal/stale-heartbeat sha=4889183eb46b v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
at: src/cmd/run.rs:412-413
quote: run::log_self_exit(1, Some(detail)); Started::Refused(ExitCode::from(1))
@row_add n=858 row=contract/run-refusal/already-live area=test-scope-summary rec=fct-n2arkmru@2
boundary: run and revive start refusals
item: already-live
promise: A live holder or a fresh replayed beat refuses the start with exit 1.
@row_check n=858 row=contract/run-refusal/already-live sha=25ef2d45eb9a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
@row_add n=859 row=contract/instance-settings/statusline-override area=test-scope-summary rec=fct-n2arkmru@2
boundary: Instance settings file
item: statusline override
promise: Written 0600 with the absolute pinned path, only for a pinned path of plain characters.
@row_check n=859 row=contract/instance-settings/statusline-override sha=a5df341df659 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:568
quote: replace_private(&path, document.as_bytes(), FILE_MODE)?;
at: crates/viola-state/src/fs.rs:11
quote: pub const FILE_MODE: u32 = 0o600;
at: crates/viola-agent-claude/src/statusline.rs:95
quote: if !pinned_bin_fwd.chars().all(is_shell_plain) {
@row_add n=860 row=contract/send-event-log/send-issued area=test-scope-summary rec=fct-lhvxskln@1
boundary: the send event log
item: send-issued
promise: An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written.
@row_check n=860 row=contract/send-event-log/send-issued sha=6e21debbe6e4 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:147
quote: let cursor = ctx.issue(text.len())?;
at: src/run/send.rs:180
quote: let mut data = json!({"cursor": cursor});
at: src/run/send.rs:182
quote: data["from"] = json!(from);
@row_add n=861 row=contract/send-confirmation/prompt-submitted-driver area=test-scope-summary rec=fct-lhvxskln@1
boundary: send confirmation
item: prompt-submitted with origin driver
promise: A driver send is confirmed only by the matching prompt-submitted event with origin driver inside the confirmation window.
@row_check n=861 row=contract/send-confirmation/prompt-submitted-driver sha=d063f5d5db1a v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:203
quote: line.data["origin"] = json!("driver");
at: src/run/send/slot.rs:149
quote: let deadline = opened + self.window();
@row_add n=862 row=contract/cli-send-json/exit-0 area=test-scope-summary rec=fct-lhvxskln@1
boundary: CLI send --json
item: exit 0
promise: A confirmed send exits 0 with submitted_at and cursor, where cursor equals the events.ndjson end offset before the paste.
@row_check n=862 row=contract/cli-send-json/exit-0 sha=b6b638305b15 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
at: src/run/send.rs:179
quote: let cursor = append_event_at(self.instance_dir, |cursor| {
@row_add n=863 row=contract/cli-send-json/no-prompt-submitted area=test-scope-summary rec=fct-lhvxskln@1
boundary: CLI send --json
item: not-delivered / no-prompt-submitted
promise: A send with no matching prompt-submitted in its window exits 13 as not-delivered with detail no-prompt-submitted, and send-refused is logged.
@row_check n=863 row=contract/cli-send-json/no-prompt-submitted sha=5331dccebc68 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/run/send.rs:279
quote: self.record(EventKind::SendRefused, data)?;
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=864 row=contract/cli-send-json/empty-text area=test-scope-summary rec=fct-lhvxskln@1
boundary: CLI send --json
item: not-delivered / empty-text
promise: A text whose typed text is empty is refused not-delivered with detail empty-text (exit 13), right after control-character and before anything is issued, by the client before any frame and by the wrapper.
@row_check n=864 row=contract/cli-send-json/empty-text sha=d6082974116b v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:187
quote: .then_some(NotDelivered::EmptyText)
at: src/cmd/send.rs:194-195
quote: run::log_self_exit(13, None); return Ok(13);
at: src/run/send.rs:93-94
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText);
@row_add n=865 row=decision/sent-text-typed-form area=test-scope-summary rec=fct-lhvxskln@1,fct-hpqcf7r7@3
name: Sent text is typed in normalised form
rule: A sent text is typed with every CR LF pair and every other CR as one LF and with every trailing CR and LF removed; the typed text holds no CR, nothing else changes, and the typed text is what is classified, pasted and matched.
guard: typed_text_drops_every_trailing_newline, typed_text_every_inner_cr_is_one_lf
@row_check n=865 row=decision/sent-text-typed-form sha=54891d19da7d v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232-234
quote: let text = text.trim_end_matches(['\r', '\n']); if text.contains('\r') { Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: src/run/send.rs:91
quote: let typed = typed_text(text);
at: src/run/send.rs:111
quote: let confirms = classify(text, slot.cli_verified);
at: src/run/send.rs:148
quote: if paste(text).is_err() {
guard_at: crates/viola-agent-claude/src/hook.rs:763
@row_add n=866 row=contract/send-result/unconfirmable area=test-scope-summary rec=fct-lhvxskln@1
boundary: the send result
item: unconfirmable
promise: A listed local command with no measured post-condition, or any listed command on an unverified CLI, yields ok with confirmed false, detail unconfirmable and the cursor at once, with no confirmation window.
@row_check n=866 row=contract/send-result/unconfirmable sha=7b5b33bc8ed2 v=holds judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:30-31
quote: Some((_, Some(post))) if cli_verified => Confirms::Post(*post), Some(_) => Confirms::Nothing,
at: src/run/send.rs:151-152
quote: if matches!(confirms, Confirms::Nothing) { return ctx.unconfirmable(cursor, slot.clock.now());
at: src/run/send.rs:224
quote: Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
@row_add n=867 row=decision/local-command-post-condition area=test-scope-summary rec=fct-lhvxskln@1
name: Local commands confirmed by post-condition only
rule: On a verified CLI /clear is confirmed only by a session-start of cause clear with a new session id, and /compact only by a session-start of cause compact with the unchanged id; neither is claimed by a prompt-submitted, a missed one is not-delivered, and a local command is never presumed delivered.
@row_check n=867 row=decision/local-command-post-condition sha=014c77be5e33 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:111
quote: Some("clear") if prior.as_deref() != Some(id) => Some(PostCondition::NewSession),
at: src/run/send/slot.rs:112
quote: Some("compact") if prior.as_deref() == Some(id) => {
at: src/run/send/classify.rs:30
quote: Some((_, Some(post))) if cli_verified => Confirms::Post(*post),
at: src/run/send/slot.rs:90
quote: if matches!((f.confirms, &f.state), (Confirms::Prompt, Match::Waiting))
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
@row_add n=868 row=decision/multi-line-send-one-prompt area=test-scope-summary rec=fct-lhvxskln@1
name: Multi-line text is one prompt
rule: A multi-line sent text arrives at the agent as one prompt with its inner line feeds intact.
@row_check n=868 row=decision/multi-line-send-one-prompt sha=fa5aae691775 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:79
quote: bytes.extend_from_slice(b"\x1b[200~");
at: crates/viola-agent-claude/src/hook.rs:234
quote: Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
@row_add n=869 row=contract/mcp-send/success area=test-scope-summary rec=fct-lhvxskln@1
boundary: MCP send
item: success result
promise: A confirmed MCP send gives isError false with the same structuredContent as the CLI's --json result.
@row_check n=869 row=contract/mcp-send/success sha=414a9a62e8ba v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:16
quote: CallToolResult::structured(payload)
at: src/cmd/client.rs:74
quote: return Some(Reply::Ok(ok.clone()));
@row_add n=870 row=decision/human-input-takes-the-wheel area=test-scope-summary rec=fct-3oyxjfyz@1
name: Human input takes the wheel
rule: A human editing key in the run terminal since the last turn boundary moves the wheel to the human, and the wheel returns to the driver only via CLI viola release.
@row_check n=870 row=decision/human-input-takes-the-wheel sha=7887d5c174f1 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
at: src/run/wheel.rs:323
quote: self.wheel.human_input();
at: src/run/wheel.rs:45
quote: (Input::Release, Wheel::Human, _) => Some((Wheel::Driver, WheelCause::Release)),
@row_add n=871 row=contract/event-log/wheel-human-input area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: the event log
item: wheel
promise: A human taking the wheel logs wheel with holder human and cause human-input.
@row_check n=871 row=contract/event-log/wheel-human-input sha=a716efadb244 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
at: src/run/wheel.rs:289
quote: json!({"holder": step.to.as_str(), "cause": step.cause.as_str()}),
@row_add n=872 row=contract/cli-send-json/human-typing area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: CLI send --json
item: exit 10
promise: A driver send while the human holds the wheel exits 10 with detail null, or detail manual-pause after pause.
@row_check n=872 row=contract/cli-send-json/human-typing sha=7a6f2febddbf v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:113-114
quote: (held.holder == Wheel::Human) .then(|| (held.cause == WheelCause::ManualPause).then_some(HumanTyping::ManualPause))
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
@row_add n=873 row=contract/mcp-send/refused-human-typing area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: MCP send
item: refused: human-typing
promise: An MCP send while the human holds the wheel gives isError true with the text refused: human-typing.
@row_check n=873 row=contract/mcp-send/refused-human-typing sha=dab194e9128b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
at: crates/viola-mcp/src/result.rs:21
quote: failed(reason, json!({"refusal": reason, "detail": detail}))
@row_add n=874 row=contract/cli-release/wheel-driver area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: CLI release
item: wheel driver
promise: Release gives wheel driver, and the next send is accepted.
@row_check n=874 row=contract/cli-release/wheel-driver sha=b27af05bd42c v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:45
quote: (Input::Release, Wheel::Human, _) => Some((Wheel::Driver, WheelCause::Release)),
at: src/run/wheel.rs:222
quote: Ok(json!({"ok": {"wheel": self.holder().as_str(), "budget_paused": gate.paused()}}))
@row_add n=875 row=contract/cli-release/release-from-driver area=test-scope-summary rec=fct-3oyxjfyz@1
boundary: CLI release
item: release-from-driver
promise: A release carrying from is rejected with -32602 release-from-driver, exit 20.
@row_check n=875 row=contract/cli-release/release-from-driver sha=7127b4b82004 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:206
quote: return Err(ProtocolError::ReleaseFromDriver);
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
at: crates/viola-channel/src/lib.rs:96
quote: Self::ReleaseFromDriver => json!({"reason": "release-from-driver"}),
@row_add n=876 row=decision/harness-turn-keeps-the-wheel area=test-scope-summary rec=fct-3oyxjfyz@1
name: Harness-injected turns keep the wheel
rule: A harness-injected turn (a prompt starting with an agent-message, task-notification or cross-session-message tag) does not flip the wheel.
@row_check n=876 row=decision/harness-turn-keeps-the-wheel sha=45a27408fb36 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:110-114
quote: const HARNESS_PREFIXES: [&str; 4] = [ "<agent-message from=", "<task-notification>", "<\\cross-session-message", "<cross-session-message",
at: src/run/send/slot.rs:219
quote: if line.kind == EventKind::PromptSubmitted && line.data["origin"] == "human" {
@row_add n=877 row=decision/statusline-passthrough area=test-scope-summary rec=fct-qobe5v26@2
name: Statusline passthrough
rule: The statusline hook passes the user's statusline stdout through unchanged (empty on failure) and exits 0.
@row_check n=877 row=decision/statusline-passthrough sha=1ff84c058a64 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:147
quote: Some(Shell::Exited { .. } | Shell::NotStarted) | None => None,
at: src/cmd/hook/statusline.rs:62
quote: Ok(()) => Ok(ExitCode::SUCCESS),
@row_add n=878 row=contract/budget-json/rate-limits area=test-scope-summary rec=fct-qobe5v26@2,fct-hpqcf7r7@3
boundary: budget.json
item: rate_limits
promise: The statusline hook writes rate_limits to budget.json only when the payload holds a rate_limits object; a malformed resets_at becomes "unknown".
@row_check n=878 row=contract/budget-json/rate-limits sha=0b2a509ce9a7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:126-127
quote: let budget_written = reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
at: crates/viola-agent-claude/src/statusline.rs:74
quote: _ => Reading::UNKNOWN,
@row_add n=879 row=decision/statusline-shell-out-frame-cap area=test-scope-summary rec=fct-qobe5v26@2
name: Statusline shell-out frame cap
rule: The statusline shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command.
guard: statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap
@row_check n=879 row=decision/statusline-shell-out-frame-cap sha=33803d02f373 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:264
quote: let _ = pipe.take(MAX_FRAME).read_to_end(&mut bytes);
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:147
quote: Some(Shell::Exited { .. } | Shell::NotStarted) | None => None,
guard_at: src/cmd/hook/statusline.rs:656
@row_add n=880 row=contract/event-log/budget-gate area=test-scope-summary rec=fct-qobe5v26@2
boundary: the event log
item: budget-gate
promise: A budget pause logs budget-gate with paused true and its window.
@row_check n=880 row=contract/event-log/budget-gate sha=829afb58d193 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:125
quote: let mut data = json!({"paused": self.paused.is_some()});
at: src/run/budget.rs:127
quote: data["window"] = json!(crossed.window.as_str());
at: src/run/budget.rs:327
quote: EventKind::BudgetGate,
@row_add n=881 row=contract/cli-send-json/budget-paused area=test-scope-summary rec=fct-qobe5v26@2
boundary: CLI send --json
item: exit 11
promise: A send refused by the budget gate exits 11 with detail five-hour or seven-day.
@row_check n=881 row=contract/cli-send-json/budget-paused sha=b7f451540b36 v=holds judge=sonnet head=8834195b0683
at: src/run/send/budget.rs:12
quote: Some(crossed.window.as_str()),
at: src/cmd/send.rs:68
quote: RefusalReason::BudgetPaused => 11,
at: crates/viola-core/src/lib.rs:259
quote: Self::FiveHour => "five-hour",
at: crates/viola-core/src/lib.rs:260
quote: Self::SevenDay => "seven-day",
@row_add n=882 row=contract/cli-release/budget area=test-scope-summary rec=fct-qobe5v26@2
boundary: CLI release --budget
item: budget_paused false
promise: Release --budget gives budget_paused false, with override_until when a pause stood.
@row_check n=882 row=contract/cli-release/budget sha=6d43d2d81908 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:215
quote: let mut ok = json!({"wheel": self.holder().as_str(), "budget_paused": false});
at: src/run/wheel.rs:217
quote: ok["override_until"] = json!(end);
@row_add n=883 row=decision/torn-tail-healed-by-append area=test-scope-summary rec=fct-yxyv2kzp@2,fct-vbcc7bad@2
name: Torn log tail healed by the next append
rule: A reader counts a torn last line in torn_lines and rewrites nothing; the next append heals it by writing one LF and its line in a single write, and logs one state-recovered line per heal.
guard: torn_tail
@row_check n=883 row=decision/torn-tail-healed-by-append sha=bd4b5eced78f v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:323
quote: torn: byte[0] != b'\n',
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
at: crates/viola-state/src/events.rs:338
quote: ObsEvent::StateRecovered,
at: crates/viola-state/src/events.rs:238
quote: if line.last() != Some(&b'\n') {
guard_at: crates/viola-state/src/events.rs:420
@row_add n=884 row=decision/snapshot-replay-fields area=test-scope-summary rec=fct-yxyv2kzp@2,fct-vbcc7bad@2
name: Snapshot replay recovers a fixed field set
rule: A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel, budget_paused and budget_override_until with dialog_pending false; the replay writes no file and gives no cwd.
guard: replay_recovers, chaos_snapshot_replay
@row_check n=884 row=decision/snapshot-replay-fields sha=42bf7e8e0449 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:157
quote: let state = replay(instance_dir)?;
at: crates/viola-state/src/replay.rs:18-24
quote: pub struct Replayed { pub wheel: Option<Wheel>, pub budget_paused: Option<bool>, pub budget_override_until: Option<String>, pub agent_session_id: Option<String>, pub links: Vec<Value>, pub dialog_pending: bool,
guard_at: crates/viola-state/src/replay.rs:279
@row_add n=885 row=decision/snapshot-read-through-replay area=test-scope-summary rec=fct-yxyv2kzp@2,fct-vbcc7bad@2
name: Every product snapshot reader goes through read-or-replay
rule: Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
@row_check n=885 row=decision/snapshot-read-through-replay sha=6d0bcdf2b676 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:273
quote: match read_snapshot_or_replay(dir) {
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
at: src/cmd/run.rs:277
quote: let recovered = read_snapshot_or_replay(&instance_dir).ok();
at: src/cmd/revive.rs:210
quote: let read = read_snapshot_or_replay(&instance_dir);
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
at: src/cmd/hook/statusline.rs:129
quote: let command = match read_snapshot_or_replay(&instance.dir) {
searched: read_snapshot\( · in src/ and crates/*/src (only test-module callers remain: src/run/*.rs and src/cmd/run.rs:1259 inside cfg(test); product callers all use read_snapshot_or_replay)
@row_add n=886 row=contract/cli-wait/endpoint-lost area=test-scope-summary rec=fct-yxyv2kzp@2
boundary: CLI wait
item: endpoint-lost
promise: An endpoint vanishing during wait gives instance-unreachable with detail endpoint-lost.
@row_check n=886 row=contract/cli-wait/endpoint-lost sha=0c6e4aea8241 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=887 row=decision/fake-agent-contract-suite area=test-scope-summary rec=fct-3wc25tvq@1,fct-vekwchud@2
name: Fake agent held to recorded fixtures
rule: A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, status line, resume and compact variants, including forwarded annotations.
why: guards against the fake agent drifting from the real CLI
guard: contract_fake_agent_drift
@row_check n=887 row=decision/fake-agent-contract-suite sha=fe7c2f34f8ea v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_fake_agent_drift.rs:91
quote: let sent = hook["stdin_hex"].as_str().map(unhex);
at: tests/contract_fake_agent_drift.rs:231
quote: h["stdin_hex"].as_str().map(unhex).unwrap_or_default(),
at: tests/contract_fake_agent_drift.rs:328
quote: .filter(|(_, h)| h["stdin_hex"].as_str().map(unhex).as_ref() == Some(&bytes))
at: tests/contract_fake_agent_drift.rs:661
quote: .map(|l| l["stdin_hex"].as_str().map(unhex).unwrap_or_default())
at: tests/contract_fake_agent_drift.rs:455
quote: fn fake_agent_resume_replay_matches_every_recorded_resume_set() {
at: tests/contract_fake_agent_drift.rs:520
quote: fn fake_agent_compact_replay_matches_every_recorded_compact_set() {
guard_at: tests/contract_fake_agent_drift.rs:139
@row_add n=888 row=decision/nextest-is-the-runner area=test-strategy rec=fct-7osf2h3d@2
name: nextest is the runner
rule: nextest is the test runner; a plain cargo test run is not supported.
@row_check n=888 row=decision/nextest-is-the-runner sha=6f3c6968f880 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:33
quote: tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1
at: .config/nextest.toml:1
quote: [profile.ci]
@row_add n=889 row=decision/e2e-over-fake-agent area=test-strategy rec=fct-pe3lhjgn@1
name: E2E over the fake agent
rule: Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
@row_check n=889 row=decision/e2e-over-fake-agent sha=c3080a4073f9 v=unverifiable judge=sonnet head=8834195b0683
searched: path[0-9]*_ · in tests/*.rs and crates/viola-e2e/tests/*.rs
searched: viola-fake-agent · in crates/viola-e2e/tests/*.rs
why: no evidence found that every one of the 7 critical paths and 4 expansion scenarios has an E2E; only the numbered binaries tests/path1_run_start.rs and crates/viola-e2e/tests/path6_budget_harness.rs stand, the rest could not be mapped by a bounded search
@row_add n=890 row=decision/heartbeat-staleness-pure-classifier area=test-strategy rec=fct-zx23mmpw@2,fct-vbcc7bad@2
name: Heartbeat staleness by a pure classifier
rule: Liveness is classified live, stale or gone by a pure function fed the beat's age with no clock: a dead pid or one of another start time is gone, and for the same process a beat older than 5 s or absent is stale.
@row_check n=890 row=decision/heartbeat-staleness-pure-classifier sha=22f5752c49da v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:27
quote: pub fn classify(beat_age: Option<Duration>, same_process: bool) -> Liveness {
at: crates/viola-state/src/liveness.rs:29
quote: return Liveness::Gone;
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
at: crates/viola-state/src/liveness.rs:33
quote: _ => Liveness::Stale,
at: crates/viola-state/src/liveness.rs:16
quote: pub const STALE_AFTER: Duration = Duration::from_secs(5);
@row_add n=891 row=decision/chaos-fault-sources area=test-strategy rec=fct-zinazvq2@1
name: Chaos fault sources
rule: Chaos faults come from real process and file operations (log truncation, snapshot corruption, a heartbeat freeze by SIGSTOP, a no-EOF child) plus mockall faults on the PTY seam.
@row_check n=891 row=decision/chaos-fault-sources sha=1543d7ccefc3 v=holds judge=sonnet head=8834195b0683
at: tests/chaos_torn_append.rs:66
quote: .set_len(cut)
at: tests/chaos_snapshot_replay.rs:64
quote: std::fs::write(&path, faulted).expect("faulted");
at: tests/support/frozen.rs:14
quote: frozen.signal("-STOP");
at: tests/tui_pty_seam.rs:32
quote: let mut pty = fake_in_pty(&receipt, &["--exit-no-eof"], Size::DEFAULT);
at: crates/viola-pty/src/lib.rs:107
quote: #[cfg_attr(test, mockall::automock)]
@row_add n=892 row=decision/mutation-run-verdict area=test-strategy rec=fct-cz5jn5fs@2
name: Mutation run verdict
rule: A mutation run fails on any missed outcome not left out as host-excluded, on any timeout, or when unviable outcomes exceed caught ones.
except: a mutant replacing a client seam's peer-admitted read with an always-admit survives by construction and stands as the version's known limit
@row_check n=892 row=decision/mutation-run-verdict sha=e17093a105d6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
at: crates/viola-e2e/src/harness/run/mutants.rs:61-63
quote: let swamped = unviable > caught; if swamped { failures.push("unviable-exceeds-caught".to_owned());
@row_add n=893 row=decision/bounded-remove-the-guard-witness area=test-strategy rec=fct-cz5jn5fs@2
name: Bounded remove-the-guard witness
rule: A control whose neutralised guard lets a product process start needs a witness bounded by a time limit.
why: nextest's default profile has no kill line, and an unbounded witness hung
@row_check n=893 row=decision/bounded-remove-the-guard-witness sha=90ec261fa6db v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=894 row=decision/dialog-bodies-pinned-by-insta area=test-strategy rec=fct-vekwchud@2
name: Dialog decision bodies pinned by insta
rule: insta in check mode pins the dialog decision bodies, while hook sequences are compared byte for byte with recorded fixtures, never as snapshots.
@row_check n=894 row=decision/dialog-bodies-pinned-by-insta sha=9cfa1c288912 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:687
quote: insta::assert_snapshot!(
at: Cargo.toml:210
quote: insta = { version = "=1.48.0", default-features = false }
@row_add n=895 row=decision/relayed-fixtures-kept-superseded area=test-strategy rec=fct-vekwchud@2
name: Relayed fixtures kept as superseded
rule: Fixtures relayed from the prototype are superseded by live recordings but stay committed with their provenance under a Superseded section.
@row_check n=895 row=decision/relayed-fixtures-kept-superseded sha=9cd06f3135b3 v=holds judge=sonnet head=8834195b0683
at: fixtures/claude/2.1.287/RELAYED.md:44
quote: ## Superseded (2026-10-05, chunk 2026-10-05-dialog-rows-and-re-probe)
@row_add n=896 row=decision/test-only-e2e-crate area=test-strategy rec=fct-hckuzazj@4
name: Test-only E2E crate
rule: One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root.
why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
@row_check n=896 row=decision/test-only-e2e-crate sha=e793ebcc1d68 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:14-16
quote: [[bin]] name = "viola-harness" path = "src/bin/viola-harness.rs"
at: crates/viola-e2e/Cargo.toml:45
quote: tokio = { workspace = true, features = ["rt", "process"] }
at: .github/workflows/ci.yml:382
quote: cargo deny --config deny-sync.toml --manifest-path "crates/$crate/Cargo.toml" check bans
at: scripts/sync-crates.txt:4
quote: viola-state
@row_add n=897 row=decision/one-shared-test-helper area=test-strategy rec=fct-hckuzazj@4
name: One shared copy of each test helper
rule: Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
@row_check n=897 row=decision/one-shared-test-helper sha=cf5391b50734 v=holds judge=sonnet head=8834195b0683
at: tests/support/events.rs:74
quote: pub fn events(instance_dir: &Path) -> Vec<Value> {
at: tests/support/events.rs:79
quote: pub fn wait_events(instance_dir: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {
at: tests/support/cli.rs:24
quote: pub struct Running(Option<Child>);
at: tests/support/frozen.rs:7
quote: pub struct Frozen(String);
at: tests/support/cli.rs:16
quote: pub struct Ran {
@row_add n=898 row=decision/e2e-layer-by-binary-prefix area=test-strategy rec=fct-hckuzazj@4
name: E2E layer selected by binary prefix
rule: The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
@row_check n=898 row=decision/e2e-layer-by-binary-prefix sha=c5234b115bde v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/nextest.rs:15
quote: pub(super) const E2E: &str = "binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)";
@row_add n=899 row=decision/ipc-vector-test-layers area=test-strategy rec=fct-34amt7nu@2
name: IPC vector test layers
rule: The IPC vector is tested at integration and E2E, including an impostor server and a loosened socket directory.
@row_check n=899 row=decision/ipc-vector-test-layers sha=ccb1868f9eb6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/tests/handlers.rs:489
quote: fn handlers_frame_tools_refuse_a_loosened_socket_directory() {
at: tests/security_negatives_server_identity.rs:200
quote: fn security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper(
at: tests/cross_exit_causes.rs:405
quote: fn a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused(#[case] json: bool) {
@row_add n=900 row=decision/socket-directory-refusal area=test-strategy rec=fct-34amt7nu@2
name: Loosened socket directory refused
rule: A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
@row_check n=900 row=decision/socket-directory-refusal sha=e9beaad83c89 v=holds judge=sonnet head=8834195b0683
at: tests/security_negatives_socket_dir.rs:54
quote: fs::set_permissions(dir, fs::Permissions::from_mode(0o755)).expect("chmod");
at: tests/security_negatives_socket_dir.rs:59
quote: std::os::unix::fs::symlink(&target, dir).expect("symlink");
at: tests/security_negatives_socket_dir.rs:205
quote: fn security_negative_a_start_refuses_the_socket_directory_before_any_bind(#[case] form: Form) {
at: tests/security_negatives_socket_dir.rs:241
quote: fn security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame(
at: tests/security_negatives_socket_dir.rs:355
quote: fn security_negative_a_hook_fails_open_on_the_socket_directory(#[case] form: Form) {
@row_add n=901 row=contract/channel-links/invalid-pair area=test-strategy rec=fct-34amt7nu@2
boundary: the wrapper channel
item: link and unlink with a bad name or pair
promise: A raw link or unlink frame with a bad name or pair gets -32602 and no line is written.
@row_check n=901 row=contract/channel-links/invalid-pair sha=57b619a15316 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:73
quote: let driver = self.driver_of(params)?;
at: src/run/links.rs:92
quote: .ok_or(ProtocolError::InvalidParams)
at: src/run/links.rs:95-96
quote: if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
@row_add n=902 row=decision/hook-stdin-test-layers area=test-strategy rec=fct-yhevpdmp@2
name: Hook stdin test layer
rule: Hook stdin is tested at integration, through the hook as a subprocess.
@row_check n=902 row=decision/hook-stdin-test-layers sha=940c6c1801da v=holds judge=sonnet head=8834195b0683
at: tests/hook_events.rs:416
quote: let mut child = Command::new(VIOLA)
@row_add n=903 row=decision/mcp-test-layer area=test-strategy rec=fct-wzml6tug@1
name: MCP test layer
rule: MCP is tested at E2E through an rmcp client.
@row_check n=903 row=decision/mcp-test-layer sha=a8aea55d8ebe v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/tests/mcp_tools.rs:101
quote: let client = ().serve((stdout, stdin)).await.expect("initialized");
@row_add n=904 row=decision/cli-env-test-layers area=test-strategy rec=fct-3o7lfwre@1
name: CLI and env test layers
rule: CLI and environment input is tested at unit and integration.
@row_check n=904 row=decision/cli-env-test-layers sha=eeeedb5869ee v=holds judge=sonnet head=8834195b0683
at: src/main.rs:448
quote: #[test]
at: tests/security_negatives_strict_modes.rs:340
quote: let mut command = Command::new("/bin/sh");
@row_add n=905 row=decision/filesystem-test-layers area=test-strategy rec=fct-mqdaf5bf@2
name: Filesystem test layers
rule: The filesystem vector is tested at integration and chaos, including strict-mode refusals.
@row_check n=905 row=decision/filesystem-test-layers sha=1f18d39101b0 v=holds judge=sonnet head=8834195b0683
at: tests/security_negatives_strict_modes.rs:82-85
quote: fn frame_verb_refuses_a_widened_instance_before_any_log( stamped_home: StampedHome, #[case] name: &str, ) {
at: tests/chaos_snapshot_replay.rs:30
quote: fn first_life(stamped: StampedHome) -> (StampedHome, Value) {
@row_add n=906 row=decision/home-another-user-may-read-refused area=test-strategy rec=fct-mqdaf5bf@2
name: Readable home refused
rule: A home another user may read refuses run and viola list, and a start under umask 000 makes every home path owner-only.
@row_check n=906 row=decision/home-another-user-may-read-refused sha=7e8239f77fa7 v=holds judge=sonnet head=8834195b0683
at: tests/security_negatives_strict_modes.rs:385
quote: fn a_home_another_user_may_read_refuses_run(#[from(support::home::home)] tmp: TestHome) {
at: tests/cli_list.rs:555
quote: fn list_refuses_a_home_another_user_may_read_with_and_without_an_instance() {
at: tests/security_negatives_strict_modes.rs:335
quote: fn a_start_under_umask_000_makes_every_home_path_owner_only(
@row_add n=907 row=decision/child-spawn-test-layers area=test-strategy rec=fct-svqwosl6@1
name: Child spawn test layers
rule: Child spawn is tested at integration and E2E.
@row_check n=907 row=decision/child-spawn-test-layers sha=0b07ee37523c v=holds judge=sonnet head=8834195b0683
at: tests/tui_pty_seam.rs:22
quote: OuterPty::spawn_sized(Path::new(FAKE), &args, &[], size)
at: tests/path1_run_start.rs:1
quote: //! Critical Path 1, the `run` start sequence, as one ordered flow over one stamped home
@row_add n=908 row=decision/supply-chain-ci-gates area=test-strategy rec=fct-peucakqx@1
name: Supply chain held by CI gates
rule: The supply chain is held by CI gates: a sync-crate check, an orphans check, cargo deny including the fuzz lockfile, the sole-root tokio ban with ban probes, zizmor, a release check and weekly advisories on both lockfiles.
@row_check n=908 row=decision/supply-chain-ci-gates sha=25c03ab95835 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:275
quote: cargo check $(sed 's/^/-p /' scripts/sync-crates.txt)
at: .github/workflows/ci.yml:314
quote: run: bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
at: .github/workflows/ci.yml:382
quote: cargo deny --config deny-sync.toml --manifest-path "crates/$crate/Cargo.toml" check bans
at: .github/workflows/ci.yml:387
quote: run: bash scripts/deny-probes.sh
at: .github/workflows/ci.yml:392
quote: zizmor --format=json .github/workflows/ > target/supply-chain/zizmor.json || { cat target/supply-chain/zizmor.json; exit 1; }
at: .github/workflows/ci.yml:338
quote: run: bash scripts/release-check.sh
at: .github/workflows/nightly.yml:29-30
quote: cargo deny check advisories cargo deny --manifest-path fuzz/Cargo.toml check advisories
@row_add n=909 row=decision/property-trigger-layer area=test-strategy rec=fct-l3h5275l@1
name: Property-test trigger layer
rule: The property-test trigger is met by proptest and the fuzz job.
@row_check n=909 row=decision/property-trigger-layer sha=10b4bbf1dffc v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:427
quote: proptest! {
at: .github/workflows/nightly.yml:48
quote: - name: Time-boxed fuzz (120 s per target)
@row_add n=910 row=decision/chaos-trigger-layer area=test-strategy rec=fct-x65537y4@1
name: Chaos-test trigger layer
rule: The chaos-test trigger is met by a chaos suite.
@row_check n=910 row=decision/chaos-trigger-layer sha=34e7d71f3c7c v=holds judge=sonnet head=8834195b0683
at: tests/chaos_snapshot_replay.rs:30
quote: fn first_life(stamped: StampedHome) -> (StampedHome, Value) {
@row_add n=911 row=decision/performance-trigger-layer area=test-strategy rec=fct-b6yqyazs@1
name: Performance-budget trigger layer
rule: The performance-budget trigger is met by perf budget tests.
@row_check n=911 row=decision/performance-trigger-layer sha=7012f199e466 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:147
quote: run: bash scripts/agent-run.sh run --perf
@row_add n=912 row=decision/cross-surface-trigger-layer area=test-strategy rec=fct-s3ozhmcp@1
name: Cross-surface trigger layer
rule: Cross-surface coordination is tested at E2E, with cross-surface parity checks.
@row_check n=912 row=decision/cross-surface-trigger-layer sha=7529220a1f9d v=holds judge=sonnet head=8834195b0683
at: tests/cross_list_rows.rs:49
quote: fn assert_board(board: &str) {
at: tests/cross_exit_causes.rs:115
quote: #[test]
@row_add n=913 row=decision/single-os-ci-leg area=test-strategy rec=fct-gldxhydz@3
name: One CI leg on Linux
rule: The CI matrix builds on one leg, ubuntu-latest.
@row_check n=913 row=decision/single-os-ci-leg sha=f63969d2d928 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
@row_add n=914 row=decision/os-gated-tests-registered area=test-strategy rec=fct-gldxhydz@3
name: OS-gated tests registered with a reason
rule: Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it.
guard: contract_os_gated_tests
@row_check n=914 row=decision/os-gated-tests-registered sha=7c051f60ad7e v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_os_gated_tests.rs:529-534
quote: out.extend( found .iter() .filter(|(_, _, gate)| names_another_os(gate)) .map(|(file, test, gate)| format!("another OS: {file} {test} cfg({gate})")), );
at: tests/contract_os_gated_tests.rs:520-523
quote: let mut out: Vec<String> = found .difference(&named) .map(|(file, test, gate)| format!("unnamed: {file} {test} cfg({gate})")) .collect();
guard_at: tests/contract_os_gated_tests.rs:539
@row_add n=915 row=decision/multi-version-trigger-layers area=test-strategy rec=fct-iwh5tatz@1
name: Multi-version compatibility layers
rule: Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
@row_check n=915 row=decision/multi-version-trigger-layers sha=7d3a4b53475c v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:893
quote: proptest! {
at: fixtures/claude/2.1.302/Mcp.tools-list-1.json:1
quote: {
@row_add n=916 row=decision/sandbox-contract-trigger-layer area=test-strategy rec=fct-ffgiydwd@1
name: Sandbox contract trigger layer
rule: The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
@row_check n=916 row=decision/sandbox-contract-trigger-layer sha=d93ada29f27e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/tests/mcp_recorded_exchange.rs:181
quote: let walked = recorded(&Workspace::from_build().root.join("fixtures/claude"));
@row_add n=917 row=decision/mutation-trigger-layer area=test-strategy rec=fct-2aejh4x2@1
name: Mutation trigger layer
rule: Mutation testing is met by the mutation run.
@row_check n=917 row=decision/mutation-trigger-layer sha=09ba10106fed v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:33
quote: tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1
@row_add n=918 row=decision/discipline-at-every-layer area=test-strategy rec=fct-brzfiy4i@1
name: Discipline held at every layer
rule: No screen parsing, no blanket approval and no flaky tests hold at every test layer.
@row_check n=918 row=decision/discipline-at-every-layer sha=b996092f1c60 v=unverifiable judge=sonnet head=8834195b0683
searched: screen.?scrap|blanket|flak · in *.rs, *.toml, *.yml
why: not a code claim
@row_add n=919 row=contract/code-word/format area=unit-test-strategy rec=fct-iqktwdfa@1
boundary: code words
item: format
promise: A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
@row_check n=919 row=contract/code-word/format sha=dec5fb615525 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:377
quote: bytes.len() <= 64
at: crates/viola-core/src/lib.rs:378
quote: && bytes.first().is_some_and(u8::is_ascii_lowercase)
at: crates/viola-core/src/lib.rs:379-381
quote: && bytes .iter() .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
@row_add n=920 row=contract/unreachable-cause/socket-dir-refused area=unit-test-strategy rec=fct-iqktwdfa@1
boundary: instance-unreachable causes
item: socket-dir-refused
promise: A refused socket directory is reported with the cause word socket-dir-refused, each cause its own code word.
@row_check n=920 row=contract/unreachable-cause/socket-dir-refused sha=f7082b423526 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: crates/viola-core/src/lib.rs:196-200
quote: Self::StrictModes => "strict-modes-failed", Self::InstanceDead => "instance-dead", Self::UnwrappedName => "unwrapped-name", Self::EndpointLost => "endpoint-lost", Self::ServerVerify => "server-verify-failed",
@row_add n=921 row=contract/json-rpc-errors/-32700 area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: -32700
promise: Parse error is reported as -32700.
@row_check n=921 row=contract/json-rpc-errors/-32700 sha=9731a05e0ae6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:71
quote: Self::Parse => -32700,
@row_add n=922 row=contract/json-rpc-errors/-32600 area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: -32600
promise: Invalid request is reported as -32600.
@row_check n=922 row=contract/json-rpc-errors/-32600 sha=42340e386247 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:72
quote: Self::InvalidRequest => -32600,
@row_add n=923 row=contract/json-rpc-errors/-32601 area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: -32601
promise: Unknown method is reported as -32601.
@row_check n=923 row=contract/json-rpc-errors/-32601 sha=d9809a13b89a v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:73
quote: Self::MethodNotFound => -32601,
@row_add n=924 row=contract/json-rpc-errors/-32602 area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: -32602
promise: Invalid params is reported as -32602 with data holding supported and wrapper, its cause telling the three invalid-params faults apart.
@row_check n=924 row=contract/json-rpc-errors/-32602 sha=f91e6369ee55 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
at: crates/viola-channel/src/lib.rs:95
quote: Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
at: crates/viola-channel/src/lib.rs:103-109
quote: pub const fn cause(self) -> Option<&'static str> { match self { Self::UnsupportedVersion => Some("unsupported-version"), Self::InvalidParams => Some("invalid-params"), Self::ReleaseFromDriver => Some("release-from-driver"),
@row_add n=925 row=contract/json-rpc-errors/wrapper-fault area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: wrapper channel JSON-RPC errors
item: a code outside the five
promise: A fault code outside the five reads code, message "wrapper fault" and data null on --json and MCP, and the human line reads error: wrapper fault followed by the code.
@row_check n=925 row=contract/json-rpc-errors/wrapper-fault sha=9056739e50f2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:139
quote: _ => ("wrapper fault", Value::Null),
at: src/human/mirror.rs:53
quote: out.write_all(format!("error: wrapper fault  {code}\n").as_bytes())
@row_add n=926 row=contract/channel-endpoint/socket-path area=unit-test-strategy rec=fct-o57nkgzg@2
boundary: the wrapper channel endpoint
item: socket path
promise: The endpoint is viola- and 12 hex of the FNV-1a hash of the name, a NUL and the absolute home, as a socket in the per-user 0700 directory under XDG_RUNTIME_DIR/viola, with /tmp/viola-<uid> as fallback.
@row_check n=926 row=contract/channel-endpoint/socket-path sha=1d1122442170 v=holds judge=sonnet head=8834195b0683 masked=true
at: crates/viola-channel/src/endpoint.rs:30
quote: let hash = fnv1a(name.bytes().chain([0]).chain(home.bytes()));
at: crates/viola-channel/src/endpoint.rs:32
quote: Ok(format!("viola-{}", &hex[..12]))
at: crates/viola-channel/src/endpoint.rs:39
quote: Some(xdg) => Path::new(xdg).join("viola"),
at: crates/viola-channel/src/endpoint.rs:40
quote: None => PathBuf::from(format!("{host path}{uid}")),
@row_add n=927 row=decision/defensive-log-reader area=unit-test-strategy rec=fct-vbcc7bad@2
name: Event log reader counts and never panics
rule: The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned.
guard: three_counts
@row_check n=927 row=decision/defensive-log-reader sha=424d0263d1c8 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
at: crates/viola-state/src/events.rs:266
quote: self.skipped.unknown_fields += 1;
guard_at: crates/viola-state/src/events.rs:830
@row_add n=928 row=decision/server-identity-check area=unit-test-strategy rec=fct-vbcc7bad@2
name: Server identity check
rule: A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint.
guard: server_verdict_rows
@row_check n=928 row=decision/server-identity-check sha=2e1b0436cc63 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:129-134
quote: pub fn server_verdict( recorded_pid: u32, started_at: &str, observed_pid: Option<u32>, observed_start: Option<u64>, ) -> ServerVerdict {
at: crates/viola-state/src/liveness.rs:93-95
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else { return None; };
guard_at: crates/viola-state/src/liveness.rs:337
@row_add n=929 row=decision/strict-modes-two-rules area=unit-test-strategy rec=fct-vbcc7bad@2
name: Strict modes write and read rules
rule: Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule.
guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
@row_check n=929 row=decision/strict-modes-two-rules sha=510acbd66ef7 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:227-229
quote: pub fn unix_read_verdict(mode: u32, uid: u32, euid: u32) -> Result<(), Refused> { unix_verdict(mode, uid, euid)?; if mode & 0o077 != 0 {
at: crates/viola-state/src/strict.rs:211
quote: unix_read_verdict(mode, uid, unix::euid())
guard_at: crates/viola-state/src/strict.rs:529
@row_add n=930 row=decision/socket-directory-check area=unit-test-strategy rec=fct-vbcc7bad@2
name: Socket directory check
rule: The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes.
guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
@row_check n=930 row=decision/socket-directory-check sha=4916ff0e6376 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:168-174
quote: if is_symlink { return Err(Refused::Symlink); } if !is_dir { return Err(Refused::NotADirectory); } unix_read_verdict(mode, uid, euid)
at: crates/viola-state/src/strict.rs:141
quote: Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
guard_at: crates/viola-state/src/strict.rs:310
@row_add n=931 row=decision/stamps-refuse-widened-ledger area=unit-test-strategy rec=fct-vbcc7bad@2
name: Stamps refuse a widened ledger
rule: Updating stamps refuses a widened ledger before creating it and again under the lock, reading and writing nothing.
guard: update_stamps_refuses_a_widened_ledger_and_leaves_it
@row_check n=931 row=decision/stamps-refuse-widened-ledger sha=965da06b6a59 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/stamps.rs:61
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?;
at: crates/viola-state/src/stamps.rs:63
quote: create_private_dir(&dir)?;
at: crates/viola-state/src/stamps.rs:66
quote: strict::check_stamps(home).map_err(StampsError::StrictModes)?;
guard_at: crates/viola-state/src/stamps.rs:286
@row_add n=932 row=decision/private-placement-mode-first area=unit-test-strategy rec=fct-vbcc7bad@2
name: Private copies placed mode-first
rule: A pinned copy is placed once at an owner-only mode set before any byte is written, leaving no temporary file and never overwriting a present name.
guard: place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp
@row_check n=932 row=decision/private-placement-mode-first sha=ab1a6fb5cbca v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:115-116
quote: restrict(tmp.path(), mode)?; tmp.write_all(bytes)?;
at: crates/viola-state/src/fs.rs:118
quote: match tmp.into_temp_path().persist_noclobber(path) {
at: crates/viola-state/src/fs.rs:123
quote: Err(e) if e.error.kind() == io::ErrorKind::AlreadyExists => Ok(Placed::Present),
guard_at: crates/viola-state/src/fs.rs:208
@row_add n=933 row=decision/bounded-compare area=unit-test-strategy rec=fct-vbcc7bad@2
name: Bounded compare of a shared target
rule: Comparing a target to known bytes reads no more than one byte past their length, never the whole file.
guard: replace_private_shared_reads_no_more_than_one_byte_past_the_bytes
@row_check n=933 row=decision/bounded-compare sha=c0e97b13cced v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:143-145
quote: let cap = u64::try_from(bytes.len()).map_or(u64::MAX, |len| len.saturating_add(1)); let mut read = Vec::new(); file.take(cap).read_to_end(&mut read).is_ok() && read == bytes
guard_at: crates/viola-state/src/fs.rs:167
@row_add n=934 row=decision/dialog-answer-mapping area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Dialog answer mapping
rule: Questions are answered by PreToolUse allow with updatedInput answers and annotations; permissions by PermissionRequest allow or deny with a message; a plan is approved only via PreToolUse allow with the tool's own input unchanged, and revised via PermissionRequest deny.
@row_check n=934 row=decision/dialog-answer-mapping sha=1f58757bfdda v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:360-364
quote: updated.insert("answers".to_owned(), json!(answers)); if let Some(annotations) = annotations { updated.insert("annotations".to_owned(), Value::Object(annotations.clone())); } pre_tool_use_allow(Some(Value::Object(updated)))
at: crates/viola-agent-claude/src/dialog.rs:372-374
quote: ) => pre_tool_use_allow(Some(Value::Object( input.as_object().cloned().unwrap_or_default(), ))),
at: crates/viola-agent-claude/src/dialog.rs:381
quote: ) => permission_decision("deny", message.as_deref()),
at: crates/viola-agent-claude/src/dialog.rs:384-387
quote: Permit::Allow => "allow", Permit::Deny => "deny", }; permission_decision(behavior, message.as_deref())
@row_add n=935 row=decision/claude-env-strip area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Agent identity environment stripped
rule: Every inherited CLAUDE-prefixed variable is removed by exact name unless in the persistent keep set, and the eleven-name identity floor is removed whatever the keep set says; the floor stays eleven names.
guard: an R8 test against a literal copy of the floor, never imported from the product
@row_check n=935 row=decision/claude-env-strip sha=c344bc52c26e v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:17
quote: pub const IDENTITY_FLOOR: [&str; 11] = [
at: crates/viola-agent-claude/src/lib.rs:72-76
quote: let floor = IDENTITY_FLOOR.contains(&text.as_str()); if !floor && persistent.contains(&text) { plan.kept.push(text); } else { plan.remove.push(name);
guard_at: tests/tui_env_strip.rs:19
@row_add n=936 row=contract/dialog-data/null-details area=unit-test-strategy rec=fct-hpqcf7r7@3
boundary: dialog and Stop event data
item: description, preview and background_tasks
promise: A missing or non-string question description or preview is null, never a refusal; background_tasks carries only the array's length, or null when absent or not an array.
@row_check n=936 row=contract/dialog-data/null-details sha=b283d0f3d6c1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/dialog.rs:154
quote: |o| json!({"description": o["description"].as_str(), "preview": o["preview"].as_str()}),
at: crates/viola-agent-claude/src/hook.rs:172
quote: "background_tasks": known.background_tasks.map(|tasks| tasks.len()),
@row_add n=937 row=decision/version-gate-needs-all-relied-rows area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Version gate needs every relied row
rule: A CLI version reads verified only when its stamp holds every relied ledger row as pass; a stamp of only older row ids reads unverified.
guard: stamps_verdict_needs_the_dialog_rows
@row_check n=937 row=decision/version-gate-needs-all-relied-rows sha=9dda4cd35841 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/ledger.rs:1237-1239
quote: Ok(LedgerRow::relied() .iter() .all(|row| rows[row.id()] == "pass"))
guard_at: src/run/version_gate.rs:289
@row_add n=938 row=decision/agents-parser-fails-to-unknown area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Agents list parse failure reads unknown
rule: An oversize or malformed claude agents --json payload gives unknown and leaves the board as read, and a row's name is never taken as a viola name.
@row_check n=938 row=decision/agents-parser-fails-to-unknown sha=667f97379746 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/agents.rs:90-91
quote: if bytes.len() as u64 > MAX_FRAME { return Err(AgentsFailure::Oversize);
at: crates/viola-agent-claude/src/agents.rs:93-94
quote: let Ok(Value::Array(elements)) = serde_json::from_slice::<Value>(bytes) else { return Err(AgentsFailure::Malformed);
at: crates/viola-agent-claude/src/agents.rs:127
quote: name: text("name"),
@row_add n=939 row=contract/prompt-submitted/origin-harness area=unit-test-strategy rec=fct-hpqcf7r7@3
boundary: the prompt-submitted event
item: origin harness
promise: A prompt whose raw start, untrimmed, is an agent-message, task-notification or cross-session-message tag (escaped or not) gets origin harness; the tag mid-prompt or after a leading space stays human.
@row_check n=939 row=contract/prompt-submitted/origin-harness sha=0d76f1c06edd v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:193
quote: if HARNESS_PREFIXES.iter().any(|p| raw.starts_with(p)) {
at: crates/viola-agent-claude/src/hook.rs:110-115
quote: const HARNESS_PREFIXES: [&str; 4] = [ "<agent-message from=", "<task-notification>", "<\\cross-session-message", "<cross-session-message", ];
@row_add n=940 row=decision/screen-signatures-only area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Screen read only for compiled signatures
rule: The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.
@row_check n=940 row=decision/screen-signatures-only sha=26dac3c0972e v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:152-156
quote: for row in parser.screen().rows(0, self.cols) { if sigs.modals.iter().any(|m| row.contains(m)) { return GateStep::Done(Readiness::InputNotReady); } input_box = input_box || sigs.input_box.iter().any(|s| row.contains(s));
at: crates/viola-agent-claude/src/screen.rs:40-43
quote: pub const SIGNATURES: Signatures = Signatures { input_box: &["for agents", "(shift+tab to cycle)"], modals: &["Yes, I trust this folder", "Yes, allow external imports"], };
@row_add n=941 row=contract/readiness-gate/gate-max-wait area=unit-test-strategy rec=fct-hpqcf7r7@3
boundary: the readiness gate
item: GATE_MAX_WAIT
promise: On a verified CLI a quiet screen with no input-box literal waits up to 8 500 ms and reads input-not-ready from that bound on.
@row_check n=941 row=contract/readiness-gate/gate-max-wait sha=1799a88998d5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:16
quote: pub const GATE_MAX_WAIT: Duration = Duration::from_millis(8500);
at: crates/viola-agent-claude/src/screen.rs:160-163
quote: } else if now.saturating_duration_since(waiting_since) < GATE_MAX_WAIT { GateStep::Wait } else { GateStep::Done(Readiness::InputNotReady)
@row_add n=942 row=decision/vt100-feed-panic-safe area=unit-test-strategy rec=fct-hpqcf7r7@3
name: Screen feed panic-safe
rule: The vt100 feed runs under catch_unwind, and a panic in it gives input-not-ready.
@row_check n=942 row=decision/vt100-feed-panic-safe sha=f7e2a406baf9 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() {
at: crates/viola-agent-claude/src/screen.rs:139-140
quote: let Some(parser) = &self.parser else { return GateStep::Done(Readiness::InputNotReady);
@row_add n=943 row=decision/mcp-stdin-line-cap area=unit-test-strategy rec=fct-vbsipq2o@1
name: MCP stdin line cap
rule: The MCP server's stdin yields whole lines of at most MAX_FRAME bytes; a longer line releases no byte, sets the cut flag and reads as end of stream, ending a session before initialize as the initialize error.
guard: capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more
@row_check n=943 row=decision/mcp-stdin-line-cap sha=272a1c6a0bda v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/stdin.rs:60-64
quote: if !self.fits(self.held.len() + segment.len()) { self.held.clear(); self.done = true; self.cut.store(true, Ordering::Relaxed); return;
at: crates/viola-mcp/src/stdin.rs:93-95
quote: if this.done { return Poll::Ready(Ok(())); }
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
guard_at: crates/viola-mcp/src/stdin.rs:230
@row_add n=944 row=contract/mcp-reply/screened-words area=unit-test-strategy rec=fct-vbsipq2o@1
boundary: MCP tool replies
item: refusal and detail words
promise: A refusal that is no code word reads unknown, a string detail that is no code word reads null, and a peer fault reads through the sanitised fault.
@row_check n=944 row=contract/mcp-reply/screened-words sha=ee48ea973f6c v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:76
quote: RefusalReason::Unknown.as_str()
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
at: crates/viola-mcp/src/result.rs:60
quote: return sanitised_fault(fault)
@row_add n=945 row=decision/unknown-refusal-word-stays-visible area=unit-test-strategy rec=fct-id225cf7@2
name: Unknown refusal word stays visible
rule: A refusal code word outside the closed refusal list stays visible in the JSON document and in the human refused line, with the unknown refusal's exit.
guard: reply_of_reads_the_three_shapes
@row_check n=945 row=decision/unknown-refusal-word-stays-visible sha=ab5e84a2f121 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:77-81
quote: let reason = if is_code_word(reason) { reason.to_owned() } else { RefusalReason::Unknown.as_str().to_owned() };
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
guard_at: src/cmd/client.rs:440
@row_add n=946 row=contract/cli-arguments/json-flag area=unit-test-strategy rec=fct-id225cf7@2
boundary: CLI arguments
item: --json
promise: The --json flag is read from the arguments only before a `--` separator.
@row_check n=946 row=contract/cli-arguments/json-flag sha=f4eaabac250c v=holds judge=sonnet head=8834195b0683
at: src/main.rs:153
quote: .take_while(|arg| arg != "--")
@row_add n=947 row=decision/budget-decision-takes-the-instant area=unit-test-strategy rec=fct-id225cf7@2
name: Budget decision takes the instant as an argument
rule: The budget decision is a pure function that takes the wall-clock instant as an argument, so its thresholds and override expiry are unit-tested with no clock seam and no mocked instant.
guard: gate_decide_*
@row_check n=947 row=decision/budget-decision-takes-the-instant sha=f6f95b9785d8 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:167-172
quote: pub(crate) fn decide( reading: Option<&BudgetReading>, thresholds: Thresholds, override_until: Option<&str>, now: DateTime<Utc>, ) -> Standing {
guard_at: src/run/budget.rs:508
@row_add n=948 row=decision/send-confirmation-exact-match area=unit-test-strategy rec=fct-id225cf7@2
name: Send confirmation by exact normalised match
rule: A send is confirmed only when the submitted prompt text equals the typed text, every CR LF and lone CR read as one LF and the trailing CR and LF removed; a submitted prompt that still holds a CR or the trailing newline claims nothing.
guard: send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing
@row_check n=948 row=decision/send-confirmation-exact-match sha=48c9fd7de326 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232-234
quote: let text = text.trim_end_matches(['\r', '\n']); if text.contains('\r') { Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
at: src/run/send/slot.rs:90-91
quote: if matches!((f.confirms, &f.state), (Confirms::Prompt, Match::Waiting)) && f.text == text =>
guard_at: src/run/send/relabel.rs:76
@row_add n=949 row=decision/send-confirmation-window-expiry area=unit-test-strategy rec=fct-id225cf7@2
name: Send confirmation window expiry
rule: A confirmed send whose window expires without a matching submitted prompt is refused no-prompt-submitted.
@row_check n=949 row=decision/send-confirmation-window-expiry sha=474da918fcd6 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:158-160
quote: Some(Match::Waiting) if self.clock.now() >= deadline => { close(&mut flight.send); return None;
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
@row_add n=950 row=contract/channel-wait/wake-kinds area=unit-test-strategy rec=fct-id225cf7@2
boundary: the channel wait method
item: wake kinds
promise: wait wakes only on turn-ended, question, permission, plan, session-end and dialog-expired, and returns at once when a matching record is at or after `after`.
@row_check n=950 row=contract/channel-wait/wake-kinds sha=9adf9d705b2d v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:99-106
quote: pub const WAIT_WAKE: [Self; 6] = [ Self::TurnEnded, Self::Question, Self::Permission, Self::Plan, Self::SessionEnd, Self::DialogExpired, ];
at: src/run/wait.rs:201-203
quote: if EventKind::WAIT_WAKE .iter() .any(|k| Some(k.as_str()) == kind)
@row_add n=951 row=contract/channel-wait/timed-out area=unit-test-strategy rec=fct-id225cf7@2
boundary: the channel wait method
item: timed_out
promise: wait returns {timed_out:true} only once the clock passes timeout_ms, never before; the timeout runs in the wrapper process.
@row_check n=951 row=contract/channel-wait/timed-out sha=4c96b9c10936 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:159-161
quote: if deadline.is_some_and(|d| self.clock.now() >= d) { span.record("timed_out", true); return Ok(json!({"ok": {"timed_out": true}}));
@row_add n=952 row=decision/send-refusal-order area=unit-test-strategy rec=fct-id225cf7@2
name: send refusal order
rule: send refuses in the order control-character, empty-text, human-typing, budget-paused, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
@row_check n=952 row=decision/send-refusal-order sha=d4f5818d749e v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/send.rs:93
quote: if text.is_empty() {
at: src/run/send.rs:96
quote: if let Some(detail) = slot.wheel.human_typing() {
at: src/run/send.rs:103
quote: if let Some(budget) = slot.budget.get() {
at: src/run/send.rs:114
quote: if flight.send.is_some() || slot.wheel.turn_running() {
at: src/run/send.rs:126
quote: return ctx.not_delivered(None, NotDelivered::InputNotReady);
at: src/run/send.rs:132
quote: if let Some(detail) = slot.wheel.human_typing() {
at: src/run/send.rs:139
quote: if slot.wheel.turn_running() {
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
@row_add n=953 row=decision/empty-text-refused-before-the-wheel area=unit-test-strategy rec=fct-id225cf7@2
name: Empty text refused before the wheel is read
rule: An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read.
guard: send_empty_text_is_refused_first_after_control_character
@row_check n=953 row=decision/empty-text-refused-before-the-wheel sha=e1d29ce220b1 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send.rs:91-96
quote: let typed = typed_text(text); let text: &str = &typed; if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText); } if let Some(detail) = slot.wheel.human_typing() {
guard_at: src/run/send.rs:524
@row_add n=954 row=decision/answer-refusal-order area=unit-test-strategy rec=fct-id225cf7@2
name: answer refusal order
rule: answer refuses in the order control-character, human-typing, unverified-cli, unknown-dialog.
@row_check n=954 row=decision/answer-refusal-order sha=39712744e9ef v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
at: src/run/dialog.rs:429
quote: if let Some(detail) = self.wheel.human_typing() {
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/run/dialog.rs:446
quote: Some(NotDelivered::UnknownDialog.as_str()),
@row_add n=955 row=decision/human-output-control-escaping area=unit-test-strategy rec=fct-id225cf7@2
name: Human output escapes control characters
rule: Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab.
@row_check n=955 row=decision/human-output-control-escaping sha=70aa0cebf6e3 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:8-9
quote: if c.is_control() && c != '\n' && c != '\t' { shown.push_str(&format!("\\x{:02X}", u32::from(c)));
at: src/human/wait.rs:21
quote: out.write_all(escape_message(&line).as_bytes())
@row_add n=956 row=decision/list-row-control-escaping area=unit-test-strategy rec=fct-id225cf7@2
name: list rows show every control character as hex
rule: In list rows every control character, newline, tab, DEL and C1 included, prints as \xHH text.
guard: escape_row_shows_every_control_character_as_hex_text
@row_check n=956 row=decision/list-row-control-escaping sha=3816ccd190bf v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/escape.rs:23-24
quote: if c.is_control() { shown.push_str(&format!("\\x{:02X}", u32::from(c)));
guard_at: src/human/escape.rs:61
@row_add n=957 row=decision/dialog-hold-read-once area=unit-test-strategy rec=fct-id225cf7@2
name: Dialog hold read once at start
rule: The dialog hold is read once at start from the config's dialog_hold_s through a bounded read, a whole number of seconds in range, falling back to sixty with one rejection.
guard: read_hold_falls_back_to_sixty_with_one_rejection
@row_check n=957 row=decision/dialog-hold-read-once sha=acd6847a4dad v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/dialog.rs:162
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
at: src/run/dialog.rs:185-186
quote: let whole = u16::try_from(value.as_u64()?).ok()?; DialogHold::try_new(whole).ok()
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
guard_at: src/run/dialog.rs:819
@row_add n=958 row=decision/dialog-hold-expiry area=unit-test-strategy rec=fct-id225cf7@2
name: Dialog hold expiry
rule: An expired dialog hold appends exactly one dialog-expired line and logs one dialog-answered with deadline_hit true and no from; a late answer is unknown-dialog, and a hand-back or an answer writes no dialog-expired line.
guard: hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line
@row_check n=958 row=decision/dialog-hold-expiry sha=55d126930291 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/dialog.rs:378
quote: EventKind::DialogExpired,
at: src/run/dialog.rs:380
quote: json!({"dialog_id": pending.dialog_id, "dialog_kind": pending.kind.as_str()}),
at: src/run/dialog.rs:386-391
quote: obs_event!( INFO, ObsEvent::DialogAnswered, corr = pending.dialog_id, dialog_kind = pending.kind.as_str(), deadline_hit = true,
at: src/run/dialog.rs:358
quote: let expired = state.pending.take().filter(|_| deadline_hit);
guard_at: src/run/dialog.rs:929
@row_add n=959 row=decision/pending-dialog-before-its-line area=unit-test-strategy rec=fct-id225cf7@2
name: Pending dialog on the snapshot before its line
rule: A held dialog is on the snapshot before its line is logged, and is cleared when the append fails.
guard: hook_dialog_append_that_failed_clears_the_pending_dialog
@row_check n=959 row=decision/pending-dialog-before-its-line sha=d10b913ce46f v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/dialog.rs:297-300
quote: self.write_pending(Some(PendingDialog { dialog_id, kind: raise.kind.as_str().to_owned(), }));
at: src/run/dialog.rs:302-304
quote: let appended = self.feed.appending_dialog(hold, || { append_event_at(&self.instance_dir, |_| line.clone()) });
at: src/run/dialog.rs:305-307
quote: if appended.is_err() { if hold { self.write_pending(None);
guard_at: src/run/dialog.rs:1246
@row_add n=960 row=decision/dialog-timeouts-outlast-the-hold area=unit-test-strategy rec=fct-id225cf7@2
name: Dialog timeouts outlast the longest hold
rule: The hook's dialog bound is the longest hold plus the reply grace, and the embedded dialog hook timeouts outlast the longest hold.
guard: hooks_json_dialog_timeouts_outlast_the_longest_hold
@row_check n=960 row=decision/dialog-timeouts-outlast-the-hold sha=6bab38830439 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:72
quote: started + DIALOG_HOLD_MAX + REPLY_GRACE
at: plugin/hooks/hooks.json:10
quote: {"matcher": "AskUserQuestion|ExitPlanMode", "hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "pre-tool-use"], "timeout": 3615}]}
at: plugin/hooks/hooks.json:13
quote: {"hooks": [{"type": "command", "command": "@@VIOLA_BIN@@", "args": ["hook", "permission-request"], "timeout": 3615}]}
guard_at: crates/viola-agent-claude/src/lib.rs:434
@row_add n=961 row=decision/scoped-capture-pinned-dispatch area=unit-test-strategy rec=fct-id225cf7@2
name: Scoped log captures through a pinned dispatch
rule: Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default.
why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost
guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first
except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner.
@row_check n=961 row=decision/scoped-capture-pinned-dispatch sha=47adc232dbd4 v=holds guard=true judge=sonnet head=8834195b0683
at: src/main.rs:292-294
quote: static PINNED: OnceLock<tracing::Dispatch> = OnceLock::new(); PINNED.get_or_init(|| tracing::Dispatch::new(tracing_subscriber::registry())); tracing::subscriber::with_default(subscriber, f)
guard_at: src/main.rs:657
@row_add n=962 row=decision/server-checked-before-any-frame area=unit-test-strategy rec=fct-id225cf7@2
name: Server checked before any frame
rule: Every client verb and the hook refuse an unverified server inside their own span with no frame written.
guard: request_refuses_an_unverified_server_inside_its_dispatch_span
@row_check n=962 row=decision/server-checked-before-any-frame sha=78ff48e11db1 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:180-183
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid()); match verdict { ServerVerdict::Matches => Ok(client), ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/hook.rs:239-242
quote: let verdict = verify_server(snapshot.pid, &snapshot.started_at, client.server_pid()); match verdict { ServerVerdict::Matches => Ok(client), ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(NotSent::ServerUnverified),
guard_at: src/cmd/send.rs:253
@row_add n=963 row=contract/unreachable-log-cause/instance-dead area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: instance-dead
promise: Logged during connect.
@row_check n=963 row=contract/unreachable-log-cause/instance-dead sha=70a476c23ad9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:119
quote: Self::Dead => UnreachableCause::InstanceDead,
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
at: src/cmd/client.rs:256-262
quote: obs_event!( WARN, ObsEvent::ProcessExit, subject = "self", exit_code = 21u8, detail = cause.detail(), during = cause.during(),
@row_add n=964 row=contract/unreachable-log-cause/unwrapped-name area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: unwrapped-name
promise: Logged during connect.
@row_check n=964 row=contract/unreachable-log-cause/unwrapped-name sha=d67c6f16164f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:120
quote: Self::Unwrapped => UnreachableCause::UnwrappedName,
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=965 row=contract/unreachable-log-cause/endpoint-lost area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: endpoint-lost
promise: Logged during call.
@row_check n=965 row=contract/unreachable-log-cause/endpoint-lost sha=695f7a239031 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: src/cmd/client.rs:136
quote: Self::Lost => "call",
@row_add n=966 row=contract/unreachable-log-cause/server-verify-failed area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: server-verify-failed
promise: Logged during connect.
@row_check n=966 row=contract/unreachable-log-cause/server-verify-failed sha=7591d950fdc1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:122
quote: Self::ServerUnverified => UnreachableCause::ServerVerify,
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=967 row=contract/unreachable-log-cause/socket-dir-refused area=unit-test-strategy rec=fct-id225cf7@2
boundary: the unreachable log cause
item: socket-dir-refused
promise: Logged during connect when the socket directory is refused.
@row_check n=967 row=contract/unreachable-log-cause/socket-dir-refused sha=9f87daea3995 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:123
quote: Self::SocketDir => UnreachableCause::SocketDir,
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=968 row=decision/socket-dir-checked-before-any-frame area=unit-test-strategy rec=fct-id225cf7@2
name: Socket directory checked before any frame
rule: A loosened socket directory is refused inside the verb's or the hook's span before any frame is written.
guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
@row_check n=968 row=decision/socket-dir-checked-before-any-frame sha=b7948796a593 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Err(CallError::SocketDirRefused);
at: src/cmd/hook.rs:231-232
quote: if check_endpoint_dir(endpoint).is_err() { return Err(NotSent::SocketDir);
guard_at: src/cmd/client.rs:560
@row_add n=969 row=contract/human-hint-cause/not-running area=unit-test-strategy rec=fct-id225cf7@2
boundary: the human hint cause
item: not-running
promise: The hint cause of an instance-dead unreachable is not-running, a word apart from the log's.
@row_check n=969 row=contract/human-hint-cause/not-running sha=d347bdf9c4cc v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:144
quote: Self::Dead => "not-running",
@row_add n=970 row=contract/human-hint-cause/socket-dir area=unit-test-strategy rec=fct-id225cf7@2
boundary: the human hint cause
item: socket-dir
promise: The hint cause of a refused socket directory is socket-dir, a word apart from the log's socket-dir-refused.
@row_check n=970 row=contract/human-hint-cause/socket-dir sha=570d30961de3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:148
quote: Self::SocketDir => "socket-dir",
@row_add n=971 row=decision/hook-dialog-no-frame-to-impostor area=unit-test-strategy rec=fct-id225cf7@2
name: Hook writes no frame to an impostor server
rule: The hook's dialog path writes no frame and emits no decision to a server that is not the recorded wrapper.
guard: ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
@row_check n=971 row=decision/hook-dialog-no-frame-to-impostor sha=6b065ca2d1ea v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:318
quote: Err(NotSent::ServerUnverified) => return Asked::ServerUnverified,
at: src/cmd/hook.rs:359
quote: return None;
guard_at: src/cmd/hook.rs:1023
@row_add n=972 row=decision/session-end-direct-append area=unit-test-strategy rec=fct-id225cf7@2
name: SessionEnd falls back to a direct append
rule: A SessionEnd hook whose server check fails appends the session end directly.
@row_check n=972 row=decision/session-end-direct-append sha=c7c2818b73fa v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:193-194
quote: if event == HookEvent::SessionEnd { append_session_end(instance, normalised);
@row_add n=973 row=contract/link-method/link-twice area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: link of a linked pair
promise: Linking a pair twice writes one line and answers the same since.
@row_check n=973 row=contract/link-method/link-twice sha=ee1f28a2df37 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62-63
quote: if let Some(since) = self.since(&set, driver) { return Ok((since, true));
@row_add n=974 row=contract/link-method/unlink-then-link area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: unlink then link
promise: An unlink then a link takes a later since.
@row_check n=974 row=contract/link-method/unlink-then-link sha=99c0eac4f4f1 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
at: src/run/links.rs:124
quote: EventLine::new(&self.name, kind, Source::Wrapper, data, Utc::now())
@row_add n=975 row=contract/link-method/unlink-absent-pair area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: unlink of an absent pair
promise: Unlinking an absent pair writes nothing.
@row_check n=975 row=contract/link-method/unlink-absent-pair sha=00065d8c4ed7 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:105-106
quote: if self.since(&set, driver).is_none() { return Ok(false);
@row_add n=976 row=contract/link-method/bad-pair area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: bad pair
promise: A bad pair is refused with invalid_params.
@row_check n=976 row=contract/link-method/bad-pair sha=e1bbf446bd6c v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:95-96
quote: if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
@row_add n=977 row=contract/link-method/from area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: from
promise: A from on a link or unlink changes nothing.
@row_check n=977 row=contract/link-method/from sha=34dd37de1791 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:94
quote: let (driver, driven) = (named("driver")?, named("driven")?);
@row_add n=978 row=contract/link-method/append-failure area=unit-test-strategy rec=fct-id225cf7@2
boundary: the served link methods
item: failed append
promise: An append that fails answers internal with the link set unchanged.
@row_check n=978 row=contract/link-method/append-failure sha=ac8da7b41cc2 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:74
quote: let (since, already_linked) = self.record(&driver).map_err(|_| ProtocolError::Internal)?;
at: src/run/links.rs:132-133
quote: append_event(&self.instance_dir, line)?; apply_link_line(set, &logged);
@row_add n=979 row=contract/link-verb-line/since area=unit-test-strategy rec=fct-id225cf7@2
boundary: the link verb line
item: since
promise: The line's time is the reply's since as UTC clock time; a since that is no RFC 3339 instant prints escaped.
@row_check n=979 row=contract/link-verb-line/since sha=461d1f7c9735 v=holds judge=sonnet head=8834195b0683
at: src/human/verbs.rs:85-87
quote: match DateTime::parse_from_rfc3339(since) { Ok(at) => at.with_timezone(&Utc).format("%H:%M:%SZ").to_string(), Err(_) => escape_message(since),
@row_add n=980 row=decision/bounded-readers-keep-the-cap area=unit-test-strategy rec=fct-id225cf7@2
name: Bounded readers keep the frame cap
rule: The status line shell-out, the version gate drain and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command.
guard: the BOUNDARIES rows of contract_bounded_readers
@row_check n=980 row=decision/bounded-readers-keep-the-cap sha=ee3916480ddd v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:264
quote: let _ = pipe.take(MAX_FRAME).read_to_end(&mut bytes);
at: src/run/version_gate.rs:127
quote: let _ = pipe.take(cap).read_to_end(&mut bytes);
at: src/run/budget.rs:60
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
at: src/run/dialog.rs:162
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
at: src/run/env.rs:25
quote: Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
guard_at: tests/contract_bounded_readers.rs:29
@row_add n=981 row=contract/e2e-home-refusal/message area=unit-test-strategy rec=fct-msk3nz6p@1
boundary: the e2e-home headroom refusal
item: message
promise: e2e-home backing: {free} MiB free, below the {floor} MiB floor
@row_check n=981 row=contract/e2e-home-refusal/message sha=83f884344c57 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:157
quote: .then(|| format!("e2e-home backing: {free_mib} MiB free, below the {floor_mib} MiB floor"))
@row_add n=982 row=contract/harness-boot/default-instances area=5-command-implementation rec=fct-zxaumenb@5,fct-w6iaall2@1
boundary: harness boot
item: --instance
promise: With no --instance flag the instances are overseer and builder whatever other flags are given; one or more --instance flags replace that list.
@row_check n=982 row=contract/harness-boot/default-instances sha=d1b0d1ffc594 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:148-152
quote: let raw = if instances.is_empty() { vec!["overseer".to_owned(), "builder".to_owned()] } else { instances };
@row_add n=983 row=contract/harness-boot/agents-mode area=5-command-implementation rec=fct-zxaumenb@5,fct-w6iaall2@1
boundary: harness boot
item: --agents-mode
promise: --agents-mode takes recorded, oversize or malformed, default recorded, an unknown value a usage error; it sets no env var and is recorded as agents_mode in the session records.
@row_check n=983 row=contract/harness-boot/agents-mode sha=33706cf13cd8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:44
quote: #[arg(long, value_enum, default_value_t = AgentsMode::Recorded)]
at: crates/viola-e2e/src/harness/boot.rs:193
quote: agents_mode: opts.agents_mode,
@row_add n=984 row=contract/harness-boot/statusline-echo area=5-command-implementation rec=fct-zxaumenb@5,fct-w6iaall2@1
boundary: harness boot
item: --statusline-echo
promise: --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var and writes no budget file.
@row_check n=984 row=contract/harness-boot/statusline-echo sha=0624db9eac15 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:41
quote: #[arg(long, conflicts_with = "unstamped")]
at: crates/viola-e2e/src/harness/boot.rs:253
quote: &home.join("statusline-source.json"),
at: crates/viola-e2e/src/harness/boot.rs:255
quote: 0o600,
@row_add n=985 row=decision/harness-builds-in-its-own-target area=5-command-implementation rec=fct-zxaumenb@5
name: Harness builds into its own target dir
rule: The harness build always uses its own target dir, never the target dir the shim's cargo run uses.
why: a running harness binary could not be relinked, and a workspace-wide build re-fingerprints it
@row_check n=985 row=decision/harness-builds-in-its-own-target sha=899d46ccc16c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:121
quote: .env("CARGO_TARGET_DIR", ws.cargo_target())
at: crates/viola-e2e/src/harness/mod.rs:137
quote: self.root.join("target").join("harness")
@row_add n=986 row=decision/e2e-homes-under-target-e2e-home area=5-command-implementation rec=fct-zxaumenb@5
name: E2E homes under target/e2e-home
rule: Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only.
why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
@row_check n=986 row=decision/e2e-homes-under-target-e2e-home sha=cd65d9f64f82 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:165-169
quote: let parent = tempfile::Builder::new() .prefix("viola-session-") .tempdir_in(opts.ws.e2e_home())? .keep(); let home = parent.join("home");
@row_add n=987 row=decision/e2e-home-link-base area=5-command-implementation rec=fct-zxaumenb@5
name: E2E home base may be a link
rule: A link as the e2e home base must have an absolute target, created 0700 when gone with one non-recursive create, and is accepted only when it reads a real directory with no group or other bit; a refused base reads build-failed.
@row_check n=987 row=decision/e2e-home-link-base sha=93200a4690da v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:108
quote: if !target.is_absolute() {
at: crates/viola-e2e/src/harness/mod.rs:113
quote: let _ = fs::DirBuilder::new().mode(0o700).create(&target);
at: crates/viola-e2e/src/harness/mod.rs:120
quote: if meta.permissions().mode() & 0o077 != 0 {
at: crates/viola-e2e/src/harness/boot.rs:146
quote: start(opts).unwrap_or_else(|_| failure("build-failed", None, None, &[]))
@row_add n=988 row=decision/harness-scoped-path area=5-command-implementation rec=fct-zxaumenb@5
name: Harness-scoped PATH
rule: The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
@row_check n=988 row=decision/harness-scoped-path sha=ee4761a84cbe v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:156-159
quote: fs::copy( exe(&opts.bin_dir, "viola-fake-agent"), exe(&session_bin, "claude"), )?;
at: crates/viola-e2e/src/harness/boot.rs:281
quote: .env("PATH", session_path(session_bin))
at: crates/viola-e2e/src/harness/boot.rs:499
quote: let dirs = std::iter::once(session_bin.to_path_buf()).chain(std::env::split_paths(&inherited));
@row_add n=989 row=decision/verify-sole-stamp-writer area=5-command-implementation rec=fct-zxaumenb@5
name: verify is the only stamp writer
rule: viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
@row_check n=989 row=decision/verify-sole-stamp-writer sha=d7b3ffd4af93 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:286
quote: if out.status.success() && stamped_line(&String::from_utf8_lossy(&out.stdout)) {
at: crates/viola-e2e/src/harness/boot.rs:170-171
quote: if opts.stamp && let Err(code) = stamp(opts, &home, &session_bin)
@row_add n=990 row=decision/supervisor-ordinary-child area=5-command-implementation rec=fct-zxaumenb@5
name: Supervisor is an ordinary child
rule: The supervisor is spawned as an ordinary child with no detach flags, outlives boot, and stops only through its stop request.
@row_check n=990 row=decision/supervisor-ordinary-child sha=687471995c5d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:317-322
quote: let child = Command::new(exe(bin_dir, "viola-harness")) .args(["supervise", "--session", session]) .stdin(Stdio::null()) .stdout(Stdio::null()) .stderr(Stdio::null()) .spawn()?;
at: crates/viola-e2e/src/harness/supervise.rs:115
quote: dir.join("stop.request").exists() || !dir.join("supervise.json").exists()
@row_add n=991 row=decision/harness-never-reads-cargo-bin-exe area=5-command-implementation rec=fct-zxaumenb@5
name: Harness never reads CARGO_BIN_EXE
rule: The harness takes its binaries from a passed binary dir and never reads CARGO_BIN_EXE_*.
why: Cargo sets those only for the root package's own integration tests
@row_check n=991 row=decision/harness-never-reads-cargo-bin-exe sha=98de5b1b0ebf v=holds judge=sonnet head=8834195b0683
searched: CARGO_BIN_EXE · in crates/viola-e2e/src (only a comment in harness/run/mutants.rs:29; no env! or var read)
searched: bin_dir · in crates/viola-e2e/src/harness/boot.rs (binaries come from opts.bin_dir via exe(&opts.bin_dir, ...))
@row_add n=992 row=decision/workspace-root-absolute area=5-command-implementation rec=fct-zxaumenb@5
name: Workspace root passed absolute
rule: The harness resolves the workspace root and passes it as an absolute path.
why: nextest runs the e2e crate's tests with that crate as the cwd
@row_check n=992 row=decision/workspace-root-absolute sha=3b0c27728a8a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:64
quote: let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
at: crates/viola-e2e/src/harness/boot.rs:280
quote: .arg(&opts.ws.root)
@row_add n=993 row=decision/boot-readiness-file-probe area=5-command-implementation rec=fct-hyltxe6w@1
name: Boot readiness is a file-state probe
rule: boot polls file state at a bounded 100 ms interval against a deadline, in stages, never by sleep-based synchronisation.
@row_check n=993 row=decision/boot-readiness-file-probe sha=cd3ef5ce1e0b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:23
quote: pub const POLL: Duration = Duration::from_millis(100);
at: crates/viola-e2e/src/harness/boot.rs:483-491
quote: Readiness::Pending(missing) if expired(deadline) => { return Err(failure( "readiness-timeout", Some(&inst.name), None, &missing, )); } Readiness::Pending(_) => std::thread::sleep(POLL),
@row_add n=994 row=contract/boot-readiness/process-start area=5-command-implementation rec=fct-hyltxe6w@1
boundary: boot readiness
item: process-start
promise: The instance's run log holds a process-start for self and for claude-child, both pids alive by pid and start time; missing reads run-process-start or child-process-start, and a self process-exit first is run-exited.
@row_check n=994 row=contract/boot-readiness/process-start sha=7e2fad57499b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:440
quote: missing.push(format!("{}:run-process-start", inst.name));
at: crates/viola-e2e/src/harness/boot.rs:443
quote: missing.push(format!("{}:child-process-start", inst.name));
at: crates/viola-e2e/src/harness/boot.rs:434
quote: return Readiness::Exited(code);
@row_add n=995 row=contract/boot-readiness/snapshot area=5-command-implementation rec=fct-hyltxe6w@1
boundary: boot readiness
item: snapshot
promise: The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint.
@row_check n=995 row=contract/boot-readiness/snapshot sha=7ca99c33dbbb v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:380
quote: d["pid"].is_u64() && d["started_at"].is_string() && d["child_pid"].is_u64()
at: crates/viola-e2e/src/harness/boot.rs:450
quote: missing.push(format!("{}:endpoint", inst.name));
@row_add n=996 row=contract/boot-readiness/heartbeat area=5-command-implementation rec=fct-hyltxe6w@1
boundary: boot readiness
item: heartbeat
promise: The heartbeat mtime is less than 5 s old; missing reads heartbeat.
@row_check n=996 row=contract/boot-readiness/heartbeat sha=533ad8c98a3e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:403
quote: age < BEAT_WITHIN
at: crates/viola-e2e/src/harness/boot.rs:453
quote: missing.push(format!("{}:heartbeat", inst.name));
@row_add n=997 row=contract/boot-readiness/events area=5-command-implementation rec=fct-hyltxe6w@1
boundary: boot readiness
item: events
promise: Event lines 1 to 3 are a wheel line with cause start, then budget-gate, then session-start with source hook; missing reads events.
@row_check n=997 row=contract/boot-readiness/events sha=dfed7c76fb3a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:418-422
quote: wheel["kind"] == "wheel" && wheel["data"]["cause"] == "start" && gate["kind"] == "budget-gate" && start["kind"] == "session-start" && start["source"] == "hook"
at: crates/viola-e2e/src/harness/boot.rs:456
quote: missing.push(format!("{}:events", inst.name));
@row_add n=998 row=contract/harness-boot-exit/0 area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot exit codes
item: 0
promise: Every instance is ready within the timeout.
@row_check n=998 row=contract/harness-boot-exit/0 sha=000e6ff37505 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:224-226
quote: Ok(Outcome::new(         json!({"v": 1, "cmd": "boot", "ok": true, "session": opts.session, "home": home, "instances": listed}),         true,
@row_add n=999 row=contract/harness-boot-exit/1 area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot exit codes
item: 1
promise: Not ready, with a document of v 1, cmd boot, ok false, a reason, the instance or null, exit_code or null, and the unmet readiness checks in missing.
@row_check n=999 row=contract/harness-boot-exit/1 sha=d70d5fa75d93 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:109-110
quote: "v": 1, "cmd": "boot", "ok": false, "reason": reason,             "instance": instance, "exit_code": exit_code, "missing": missing,
@row_add n=1000 row=contract/harness-boot-reason/build-failed area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot refusal reason
item: build-failed
promise: One of the four boot refusal reasons.
@row_check n=1000 row=contract/harness-boot-reason/build-failed sha=ad139f113d48 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:144
quote: return failure("build-failed", None, code.map(i64::from), &[]);
@row_add n=1001 row=contract/harness-boot-reason/verify-failed area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot refusal reason
item: verify-failed
promise: One of the four boot refusal reasons.
@row_check n=1001 row=contract/harness-boot-reason/verify-failed sha=cdef5978e39d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:173
quote: return Ok(failure("verify-failed", None, code.map(i64::from), &[]));
@row_add n=1002 row=contract/harness-boot-reason/run-exited area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot refusal reason
item: run-exited
promise: The wrapper exited; exit_code is the wrapper's exit code verbatim.
@row_check n=1002 row=contract/harness-boot-reason/run-exited sha=88e7ffa70a52 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:481
quote: return Err(failure("run-exited", Some(&inst.name), Some(code), &[]));
@row_add n=1003 row=contract/harness-boot-reason/readiness-timeout area=5-command-implementation rec=fct-nwpljjvq@1
boundary: harness boot refusal reason
item: readiness-timeout
promise: One of the four boot refusal reasons.
@row_check n=1003 row=contract/harness-boot-reason/readiness-timeout sha=2b502e377632 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:485
quote: "readiness-timeout",
@row_add n=1004 row=contract/harness-boot/timeout area=5-command-implementation rec=fct-ybixbnbx@1
boundary: harness boot
item: timeout
promise: Readiness is bounded at 20 s per instance after the build.
@row_check n=1004 row=contract/harness-boot/timeout sha=3da49a1a7b62 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:26
quote: const INSTANCE_DEADLINE: Duration = Duration::from_secs(20);
@row_add n=1005 row=contract/harness-run/default-selection area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness run
item: default selection
promise: With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
@row_check n=1005 row=contract/harness-run/default-selection sha=554ba7f300c2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:61-64
quote: unit: named.unit || all || none,             integration: named.integration || all || none,             e2e: named.e2e || all || none,             ..named
@row_add n=1006 row=contract/harness-run/mutants-only-flags area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness run
item: --file and --package
promise: --file and --package each require --mutants.
@row_check n=1006 row=contract/harness-run/mutants-only-flags sha=9f2e6eaa9437 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:107
quote: #[arg(long = "file", requires = "mutants")]
at: crates/viola-e2e/src/bin/viola-harness.rs:110
quote: #[arg(long, requires = "mutants")]
@row_add n=1007 row=decision/test-layer-filtersets area=5-command-implementation rec=fct-hgewdmj2@6
name: Test layers by nextest filterset
rule: The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
@row_check n=1007 row=decision/test-layer-filtersets sha=d7d45f5d7055 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/nextest.rs:9
quote: pub(super) const UNIT: &str = "kind(lib) | kind(bin)";
at: crates/viola-e2e/src/harness/run/nextest.rs:14-15
quote: "kind(test) & !binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)"; pub(super) const E2E: &str = "binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)";
@row_add n=1008 row=decision/e2e-boot-model-by-cleanup area=5-command-implementation rec=fct-hgewdmj2@6
name: E2E boot model chosen by cleanup
rule: A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
@row_check n=1008 row=decision/e2e-boot-model-by-cleanup sha=923d4e0b77f8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/session.rs:64
quote: let bin_dir = bin_dir_from_exe(&std::env::current_exe().expect("test exe"));
at: crates/viola-e2e/src/harness/session.rs:131
quote: let _ = cleanup(&self.ws, Target::Session(&self.session), true);
at: crates/viola-e2e/src/harness/boot.rs:134
quote: pub fn boot(opts: &BootOptions) -> Outcome {
@row_add n=1009 row=decision/root-wait-bound area=5-command-implementation rec=fct-hgewdmj2@6
name: Root waits bounded below the kills
rule: Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first.
why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
@row_check n=1009 row=decision/root-wait-bound sha=157be8d7ad9b v=holds judge=sonnet head=8834195b0683
at: tests/support/watch.rs:14
quote: pub const WITHIN: Duration = Duration::from_secs(7);
at: tests/support/home.rs:738-739
quote: if let Some(code) = self.pty.try_wait() { panic!("wrapper {} exited before ready: {code}", self.name);
@row_add n=1010 row=decision/killed-wrapper-endpoint-wait area=5-command-implementation rec=fct-hgewdmj2@6
name: A killed wrapper's endpoint is gone when unconnectable
rule: After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone.
why: a killed wrapper removes nothing, so its socket file stays
@row_check n=1010 row=decision/killed-wrapper-endpoint-wait sha=d337343cfc18 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:917-918
quote: std::os::unix::net::UnixStream::connect(endpoint) .is_err_and(|e| matches!(e.kind(), ConnectionRefused | NotFound))
at: crates/viola-e2e/src/harness/cleanup.rs:139
quote: .is_err_and(|e| matches!(e.kind(), ConnectionRefused | NotFound))
@row_add n=1011 row=contract/harness-session/session-id area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness session
item: session id
promise: The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
@row_check n=1011 row=contract/harness-session/session-id sha=366f51fe609a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/session.rs:65
quote: let session = format!("{test}-{label}-{}", std::process::id());
at: crates/viola-e2e/src/harness/session.rs:88
quote: home: record.home,
at: crates/viola-e2e/src/harness/mod.rs:77
quote: pub fn session_dir(&self, session: &str) -> PathBuf {
at: crates/viola-e2e/tests/harness_lifecycle.rs:103
quote: let session = session_id("main");
@row_add n=1012 row=contract/harness-session/cleanup-report area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness session
item: cleanup report
promise: A harness cleanup report has no field false, an interim null allowed, the same shape as the cleanup command.
@row_check n=1012 row=contract/harness-session/cleanup-report sha=9e0b792d7040 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/session.rs:118
quote: cleanup(&self.ws, Target::Session(&self.session), true)
at: crates/viola-e2e/tests/path6_budget_harness.rs:266-267
quote: assert!( fields.values().all(|v| *v != Value::Bool(false)),
@row_add n=1013 row=decision/mutation-base-whole-chunk area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutation base is the whole chunk
rule: The mutation base is derived from history as the last master flip before the oldest pre-CI commit of a pending record, so every run of a pass mutates the whole chunk; an unresolved base exits 1 base-missing and no empty diff is written.
why: an unbounded search moves mid-pass, and an empty diff reads as a false zero-mutant pass
@row_check n=1013 row=decision/mutation-base-whole-chunk sha=9829c6dc1220 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants/base.rs:55
quote: let bound = pre_ci_parent(repo).unwrap_or_else(|| "HEAD".to_owned());
at: crates/viola-e2e/src/harness/run/mutants.rs:159-161
quote: let missing = || "base-missing".to_owned(); let base = resolve_base(&ws.root, chunk_base).ok_or_else(missing)?; let diff = chunk_diff(&ws.root, &base).ok_or_else(missing)?;
@row_add n=1014 row=decision/chunk-diff-pins-prefixes area=5-command-implementation rec=fct-hgewdmj2@6
name: Chunk diff pins its prefixes
rule: The chunk diff covers the working tree and untracked files and always pins the a/ and b/ prefixes.
why: a host's diff prefix config would write headers the classification cannot read
guard: chunk_diff_pins_its_prefixes_under_a_hostile_config
@row_check n=1014 row=decision/chunk-diff-pins-prefixes sha=a2f8b95cc702 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants/base.rs:132-133
quote: "--src-prefix=a/", "--dst-prefix=b/",
at: crates/viola-e2e/src/harness/run/mutants/base.rs:137
quote: let untracked = git(repo, &["ls-files", "--others", "--exclude-standard", "-z"])?;
guard_at: crates/viola-e2e/src/harness/run/mutants/base.rs:417
@row_add n=1015 row=decision/mutation-own-target-dir area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutation run builds in its own target dir
rule: The mutation run pre-builds the root package and builds every mutant in its own target dir, copied into each scratch tree.
why: a copied default target keeps the original tree's binary paths, so tests drove the unmutated binary
@row_check n=1015 row=decision/mutation-own-target-dir sha=31a926954ec9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:227
quote: .env("CARGO_TARGET_DIR", ws.root.join(MUTANTS_TARGET))
at: crates/viola-e2e/src/harness/run/mutants.rs:248
quote: .env("CARGO_TARGET_DIR", MUTANTS_TARGET)
@row_add n=1016 row=decision/mutation-keeps-no-test-home area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutation run keeps no test home
rule: A mutation run keeps no test home, overriding the workflow's keep-homes setting.
why: every home carries a pinned copy of the viola binary
@row_check n=1016 row=decision/mutation-keeps-no-test-home sha=2aed957c53bc v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:246-247
quote: .env("AGENT_RUN_KEEP_HOMES", "0") .env("AGENT_RUN_KEEP_FAILED", "0")
@row_add n=1017 row=decision/mutants-output-live area=5-command-implementation rec=fct-hgewdmj2@6
name: cargo-mutants output streams live
rule: cargo-mutants' output goes live to the harness's stderr with every outcome line printed, never captured.
why: a stalled or cancelled run names the mutant it stopped at
@row_check n=1017 row=decision/mutants-output-live sha=715033af42a3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:242
quote: .stdout(Stdio::from(std::io::stderr()))
at: crates/viola-e2e/src/harness/run/mutants.rs:24
quote: const MUTANTS_PROGRESS: [&str; 3] = ["--caught", "--unviable", "--build-timeout=400"];
@row_add n=1018 row=decision/mutant-build-timeout area=5-command-implementation rec=fct-hgewdmj2@6
name: Fixed mutant build timeout
rule: Each mutant's build is bounded by a fixed 400 s, five times the largest measured baseline, not a multiplier.
why: a multiplier derives a sub-second bound from a sub-second baseline
@row_check n=1018 row=decision/mutant-build-timeout sha=adb625b19c33 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:24
quote: const MUTANTS_PROGRESS: [&str; 3] = ["--caught", "--unviable", "--build-timeout=400"];
@row_add n=1019 row=decision/mutation-diff-classification area=5-command-implementation rec=fct-hgewdmj2@6
name: Diffs with no mutable Rust skip cargo-mutants
rule: A diff naming no Rust path passes as no-rust-delta and one whose Rust paths are all test targets passes as test-only-rust-delta, both without reaching cargo-mutants; any other deletes the old outcomes first and is red with none.
why: cargo-mutants exits 0 and writes no outcomes on such diffs
@row_check n=1019 row=decision/mutation-diff-classification sha=2a13d1eeb4fb v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:165
quote: let (suite, doc) = unmutated("no-rust-delta", &diff, &base);
at: crates/viola-e2e/src/harness/run/mutants.rs:170
quote: let (suite, mut doc) = unmutated("test-only-rust-delta", &diff, &base);
at: crates/viola-e2e/src/harness/run/mutants.rs:219
quote: let _ = fs::remove_file(out_dir.join("outcomes.json"));
at: crates/viola-e2e/src/harness/run/mutants.rs:268
quote: failures: vec!["outcomes-missing".to_owned()],
@row_add n=1020 row=decision/mutation-verdict-by-counts area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutation verdict by counts
rule: A mutation run passes only when missed minus host-excluded is zero, no mutant timed out and unviable mutants do not outnumber caught; only exits 0, 2 or 3 can pass, then the counts decide.
why: exit 3 masks exit 2, and more unviable than caught means the run tested almost nothing
@row_check n=1020 row=decision/mutation-verdict-by-counts sha=3c7aae6cfc0f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:36
quote: Some(0 | 2 | 3) => None,
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
at: crates/viola-e2e/src/harness/run/mutants.rs:61-63
quote: let swamped = unviable > caught; if swamped { failures.push("unviable-exceeds-caught".to_owned());
@row_add n=1021 row=decision/host-excluded-mutants area=5-command-implementation rec=fct-hgewdmj2@6
name: Host-excluded missed mutants
rule: A missed mutant whose whole span sits under a cfg proved false for the build host, by compile-time facts, leaves the missed count; anything the reader cannot read stays counted.
@row_check n=1021 row=decision/host-excluded-mutants sha=64c3cd0ceee0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants/host.rs:28
quote: pub(super) const HOST: Host = Host {
at: crates/viola-e2e/src/harness/run/mutants/host.rs:53
quote: .filter(|record| record["summary"] == "MissedMutant")
at: crates/viola-e2e/src/harness/run/mutants.rs:259
quote: failures.retain(|name| !excluded.iter().any(|e| e.name == *name));
@row_add n=1022 row=contract/harness-run/mutants-exit-reason area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness run
item: mutants-exit-<code>
promise: cargo-mutants exit 1, 4, 5, 6 or 70 fails run --mutants with reason mutants-exit-<code>.
@row_check n=1022 row=contract/harness-run/mutants-exit-reason sha=c4989dd14b57 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:36-37
quote: Some(0 | 2 | 3) => None,         Some(c) => Some(format!("mutants-exit-{c}")),
@row_add n=1023 row=decision/mutants-tested-by-owning-package area=5-command-implementation rec=fct-hgewdmj2@6
name: Mutants tested by their own package
rule: Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
@row_check n=1023 row=decision/mutants-tested-by-owning-package sha=b5fc082ac8bb v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:239
quote: .args(["--test-tool=nextest", "--copy-target=true"])
searched: test-workspace · in crates/, src/, scripts/, .github/ (no match: cargo-mutants default scope kept)
@row_add n=1024 row=contract/harness-run/package-refused area=5-command-implementation rec=fct-hgewdmj2@6
boundary: harness run
item: package-refused
promise: A --package that is not viola or a member declaring fake-agent, named in lowercase ASCII, digits, - and _, exits 2 package-refused before any cargo runs.
@row_check n=1024 row=contract/harness-run/package-refused sha=fcbbba453653 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:144-145
quote: let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "package-refused"});         return Outcome { doc, code: 2 };
at: crates/viola-e2e/src/harness/run/mutants.rs:115
quote: .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_');
@row_add n=1025 row=contract/coverage-failure/llvm-cov-exit area=5-command-implementation rec=fct-zxoe3ae7@2
boundary: coverage suite failure codes
item: llvm-cov-exit-N
promise: A red llvm-cov exit over green tests, such as a floor breach.
@row_check n=1025 row=contract/coverage-failure/llvm-cov-exit sha=d8718e2e3305 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:46
quote: let mut suite = junit_suite(ws, "coverage", code, "llvm-cov");
at: crates/viola-e2e/src/harness/run/nextest.rs:76-79
quote: if code != Some(0) && suite.failed == 0 { suite.failed = 1; let code = code.map_or("signal".to_owned(), |c| c.to_string()); suite.failures.push(format!("{tool}-exit-{code}"));
@row_add n=1026 row=contract/coverage-failure/llvm-cov-summary-missing area=5-command-implementation rec=fct-zxoe3ae7@2
boundary: coverage suite failure codes
item: llvm-cov-summary-missing
promise: The coverage summary was not written.
@row_check n=1026 row=contract/coverage-failure/llvm-cov-summary-missing sha=dcf345539617 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:58-60
quote: if !summary.is_file() { suite.failed += 1; suite.failures.push("llvm-cov-summary-missing".to_owned());
@row_add n=1027 row=decision/perf-verdict-belongs-to-gate area=5-command-implementation rec=fct-hr3ukgwq@1
name: Perf verdict belongs to the gate
rule: The perf arm runs only when named and reports its suite; the deadline verdict belongs to the gate's perf requirement, never to run.
@row_check n=1027 row=decision/perf-verdict-belongs-to-gate sha=515f1051a713 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:298
quote: if sel.perf && refusal.is_none() {
at: crates/viola-e2e/src/harness/gate.rs:62-63
quote: if suite == "perf" { perf(artifacts, &mut breaches);
@row_add n=1028 row=decision/fuzz-replay-linux-only area=5-command-implementation rec=fct-ciziz6dy@1
name: Fuzz replay on Linux only
rule: Fuzz replay runs on Linux only; elsewhere it exits 2 fuzz-linux-only.
@row_check n=1028 row=decision/fuzz-replay-linux-only sha=7b04608612a3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:135-136
quote: if sel.fuzz_replay && !FUZZ_HOST_SUPPORTED { let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "fuzz-linux-only"});
@row_add n=1029 row=decision/fuzz-toolchain-single-source area=5-command-implementation rec=fct-ciziz6dy@1
name: Fuzz toolchain from one source
rule: The fuzz toolchain channel is read from the fuzz workspace's own toolchain file, the one source, and the fuzz dir is a separate cargo workspace excluded from the root.
@row_check n=1029 row=decision/fuzz-toolchain-single-source sha=1ab50fbf5263 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/fuzz.rs:52
quote: let channel = fs::read_to_string(fuzz.join("rust-toolchain.toml"))
at: Cargo.toml:182
quote: exclude = ["fuzz"]
@row_add n=1030 row=decision/fuzz-corpus-never-vacuous area=5-command-implementation rec=fct-ciziz6dy@1
name: Fuzz replay never passes vacuously
rule: A missing or empty corpus, or no target at all, fails corpus-empty, never a vacuous pass.
@row_check n=1030 row=decision/fuzz-corpus-never-vacuous sha=b332a6fbaea1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/fuzz.rs:57-58
quote: if targets.is_empty() { return Err(Refusal::new("corpus-empty", None));
at: crates/viola-e2e/src/harness/run/fuzz.rs:66-68
quote: if !has_entries(&corpus) { suite.failed += 1; suite.failures.push(format!("{target}: corpus-empty"));
@row_add n=1031 row=decision/fuzz-reproducers-committed area=5-command-implementation rec=fct-ciziz6dy@1
name: Fuzz corpus committed with reproducers
rule: The replay corpus is committed, seeded from recorded payloads or synthetic seeds, and every reproducer is committed with its fix; nothing is cached between CI runs.
@row_check n=1031 row=decision/fuzz-reproducers-committed sha=58ff08794e65 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/fuzz.rs:65
quote: let corpus = fuzz.join("corpus").join(&target);
at: .github/workflows/ci.yml:247-249
quote: - name: Corpus replay shell: bash run: bash scripts/agent-run.sh run --fuzz-replay
@row_add n=1032 row=decision/nightly-fuzz-fails-on-crash area=5-command-implementation rec=fct-ciziz6dy@1
name: Nightly fuzz fails on any crash
rule: The nightly time-boxed fuzz run uploads its artifacts and fails the job on any crash.
@row_check n=1032 row=decision/nightly-fuzz-fails-on-crash sha=9dc0836cea94 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/nightly.yml:55
quote: cargo "+$channel" fuzz run --fuzz-dir fuzz "$t" "fuzz/corpus/$t" -- -max_total_time=120
at: .github/workflows/nightly.yml:59
quote: if: failure()
@row_add n=1033 row=contract/harness-run/tool-missing area=5-command-implementation rec=fct-ciziz6dy@1
boundary: harness run
item: tool-missing
promise: Exits 1 with a detail naming the missing tool or file, such as cargo-fuzz.
@row_check n=1033 row=contract/harness-run/tool-missing sha=f0b0e87d739c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/fuzz.rs:49
quote: return Err(Refusal::new("tool-missing", Some("cargo-fuzz")));
@row_add n=1034 row=decision/live-verify-never-in-ci area=5-command-implementation rec=fct-tuv4h7zt@3
name: Live verify never in CI
rule: verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
@row_check n=1034 row=decision/live-verify-never-in-ci sha=a8d12a90bb29 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:131-133
quote: if sel.local_live && ci { let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "live-in-ci"}); return Outcome { doc, code: 2 };
@row_add n=1035 row=decision/local-live-runner-seam area=5-command-implementation rec=fct-tuv4h7zt@3
name: Local-live through a runner seam
rule: The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
@row_check n=1035 row=decision/local-live-runner-seam sha=d3b2e27e33f5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:378
quote: fn local_live(ws: &Workspace, runner: &mut Runner<'_>) -> Suite {
at: crates/viola-e2e/src/harness/run.rs:123-129
quote: pub fn run_with( ws: &Workspace, sel: Selection, filter: Option<&str>, chunk_base: Option<String>, ci: bool, runner: &mut Runner<'_>,
@row_add n=1036 row=contract/local-live-suite/pass area=5-command-implementation rec=fct-tuv4h7zt@3
boundary: the local-live suite
item: pass
promise: Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
@row_check n=1036 row=contract/local-live-suite/pass sha=9bc39c9b6982 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:408
quote: Some(0) if live_rows_pass(&stdout) => None,
at: crates/viola-e2e/src/harness/run.rs:431-432
quote: matches!(lines.as_slice(), [line] if line.ends_with("  pass")) }) && super::boot::stamped_line(stdout)
@row_add n=1037 row=contract/local-live-suite/failure-codes area=5-command-implementation rec=fct-tuv4h7zt@3
boundary: the local-live suite
item: failure codes
promise: A failure carries one code: build, verify-exit-<n>, verify-exit-none or row-missing.
@row_check n=1037 row=contract/local-live-suite/failure-codes sha=96ab68774812 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:388
quote: suite.failures.push("build".to_owned());
at: crates/viola-e2e/src/harness/run.rs:409
quote: Some(0) => Some("row-missing".to_owned()),
at: crates/viola-e2e/src/harness/run.rs:410
quote: Some(n) => Some(format!("verify-exit-{n}")),
at: crates/viola-e2e/src/harness/run.rs:411
quote: None => Some("verify-exit-none".to_owned()),
@row_add n=1038 row=contract/harness-run-exit/0 area=5-command-implementation rec=fct-dwjsmtpg@1
boundary: harness run exit codes
item: 0
promise: Every selected suite passes, with no timed-out mutants, no missed mutants beyond the host-excluded, and unviable not outnumbering caught.
@row_check n=1038 row=contract/harness-run-exit/0 sha=d3f7d3f5902f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:152
quote: let ok = refusal.is_none() && suites.iter().all(Suite::green);
at: crates/viola-e2e/src/harness/run.rs:89
quote: self.failed == 0 && self.survived == 0
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
at: crates/viola-e2e/src/harness/run/mutants.rs:61
quote: let swamped = unviable > caught;
@row_add n=1039 row=contract/harness-run-exit/non-zero area=5-command-implementation rec=fct-dwjsmtpg@1
boundary: harness run exit codes
item: non-zero
promise: Anything else; survivors, outcomes-missing and unviable-exceeds-caught each make a --mutants run red.
@row_check n=1039 row=contract/harness-run-exit/non-zero sha=ba912d66c01f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
at: crates/viola-e2e/src/harness/run/mutants.rs:68
quote: failed: survived + u64::from(swamped),
at: crates/viola-e2e/src/harness/run/mutants.rs:268
quote: failures: vec!["outcomes-missing".to_owned()],
at: crates/viola-e2e/src/harness/run/mutants.rs:63
quote: failures.push("unviable-exceeds-caught".to_owned());
@row_add n=1040 row=contract/run-document/suites area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: suites
promise: Each entry names its suite and carries passed, failed, skipped, survived, artifact, and failures listing failing test names.
@row_check n=1040 row=contract/run-document/suites sha=306b7591a580 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:71-77
quote: pub suite: String, pub passed: u64, pub failed: u64, pub skipped: u64, pub survived: u64, pub artifact: Option<String>, pub failures: Vec<String>,
@row_add n=1041 row=contract/run-document/mutants area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: mutants
promise: A --mutants run adds a mutants object with tested and a verdict of counted, no-rust-delta, test-only-rust-delta, scoped or package, with the full base sha except on package.
@row_check n=1041 row=contract/run-document/mutants sha=4537a4b73207 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:184
quote: let doc = json!({"tested": run.tested, "verdict": verdict, "base": base});
at: crates/viola-e2e/src/harness/run/mutants.rs:165
quote: let (suite, doc) = unmutated("no-rust-delta", &diff, &base);
at: crates/viola-e2e/src/harness/run/mutants.rs:170
quote: let (suite, mut doc) = unmutated("test-only-rust-delta", &diff, &base);
at: crates/viola-e2e/src/harness/run/mutants.rs:154
quote: let doc = json!({"tested": run.tested, "verdict": "package", "package": member});
@row_add n=1042 row=contract/run-document/host-excluded area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: host_excluded
promise: Present only when a mutant was left out: an array of name and cfg as written in source, repo-relative.
@row_check n=1042 row=contract/run-document/host-excluded sha=903b424f2e83 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:202-203
quote: if !self.host_excluded.is_empty() { doc["host_excluded"] = json!(self.host_excluded);
at: crates/viola-e2e/src/harness/run/mutants/host.rs:38-39
quote: pub(super) name: String, pub(super) cfg: String,
@row_add n=1043 row=contract/run-document/archived area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: archived
promise: The repo-relative run-archive dir holding this run's JUnit files and read outcomes, the newest 10 kept and never uploaded; an absent source is skipped and named on stderr.
@row_check n=1043 row=contract/run-document/archived sha=51c2f40da9ea v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:158
quote: eprintln!("run-archive: skipped {name} (absent)");
at: crates/viola-e2e/src/harness/run.rs:160
quote: doc["archived"] = json!(dir);
at: crates/viola-e2e/src/harness/run.rs:184-185
quote: const ARCHIVE: &str = "target/run-archive"; const ARCHIVE_KEEP: u64 = 10;
@row_add n=1044 row=contract/run-document/reason area=5-command-implementation rec=fct-zujg4opw@3
boundary: the harness run document
item: reason
promise: A document refused by a tool arm carries reason and, where one applies, detail.
@row_check n=1044 row=contract/run-document/reason sha=07117a983196 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:445-448
quote: doc["reason"] = json!(refusal.reason); if let Some(detail) = refusal.detail { doc["detail"] = json!(detail); }
@row_add n=1045 row=decision/skipped-counts-real-skips-only area=5-command-implementation rec=fct-zujg4opw@3
name: skipped counts real skips only
rule: skipped counts only selected tests not run; tests excluded by a layer filterset or compiled out by an OS cfg are never counted.
why: so the gate's suite-skipped breach fires only on a real skip
@row_check n=1045 row=decision/skipped-counts-real-skips-only sha=afb8aaee4840 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/nextest.rs:110-111
quote: } else if body.contains("<skipped") { suite.skipped += 1;
@row_add n=1046 row=decision/doc-examples-never-ignore area=5-command-implementation rec=fct-zujg4opw@3
name: Doc examples never use ignore
rule: A doc example that must not run uses no_run or a text fence, never ignore.
why: ignored doctests count as skipped
@row_check n=1046 row=decision/doc-examples-never-ignore sha=98f9e08a6aa4 v=holds judge=sonnet head=8834195b0683
searched: ```ignore · in **/*.rs (doc-comment fences, `///` and `//!`)
searched: rust,ignore · in **/*.rs
@row_add n=1047 row=decision/scoped-mutants-not-the-verdict area=5-command-implementation rec=fct-wglylwog@1
name: Scoped mutation is the inner fix loop
rule: A file-scoped mutation run is the inner fix loop; the counted chunk mutation run stays the verdict.
@row_check n=1047 row=decision/scoped-mutants-not-the-verdict sha=c66b62fc82af v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:179-183
quote: let verdict = if files.is_empty() { "counted" } else { "scoped" };
@row_add n=1048 row=decision/status-strips-bridge-env area=5-command-implementation rec=fct-aajvmd2t@1
name: Harness status strips the bridge env
rule: Harness status runs viola list with the session PATH and with VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed.
why: so the read files no role line under a host bridge's name
@row_check n=1048 row=decision/status-strips-bridge-env sha=a7c2477444c0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:58-61
quote: .env("PATH", session_path(session_bin)) .env_remove("VIOLA_NAME") .env_remove("VIOLA_DIR") .env_remove("VIOLA_BIN")
@row_add n=1049 row=decision/status-bounded-read area=5-command-implementation rec=fct-aajvmd2t@1
name: Harness status reads a bounded stdout
rule: Harness status reads the list output through a bound of one frame past the cap and parses it as one JSON object; with no supervisor record it spawns nothing.
@row_check n=1049 row=decision/status-bounded-read sha=47f5f788d434 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:78
quote: let _ = out.take(MAX_FRAME + 1).read_to_end(&mut bytes);
at: crates/viola-e2e/src/harness/status.rs:97
quote: Err(_) => (None, None),
@row_add n=1050 row=contract/agent-run-status-polled-fields/name area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: name
promise: A per-item field of the status document that the agent polls.
@row_check n=1050 row=contract/agent-run-status-polled-fields/name sha=1056f51c5161 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:168
quote: pub name: String,
@row_add n=1051 row=contract/agent-run-status-polled-fields/wrapped area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: wrapped
promise: A per-item field of the status document that the agent polls.
@row_check n=1051 row=contract/agent-run-status-polled-fields/wrapped sha=53a782228f7c v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:169
quote: pub wrapped: bool,
@row_add n=1052 row=contract/agent-run-status-polled-fields/liveness area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: liveness
promise: A per-item field of the status document that the agent polls.
@row_check n=1052 row=contract/agent-run-status-polled-fields/liveness sha=2d2b7826247f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:170
quote: pub liveness: RowLiveness,
@row_add n=1053 row=contract/agent-run-status-polled-fields/status area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: status
promise: A per-item field of the status document that the agent polls.
@row_check n=1053 row=contract/agent-run-status-polled-fields/status sha=40636e14fbb1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:171
quote: pub status: SessionStatus,
@row_add n=1054 row=contract/agent-run-status-polled-fields/wheel area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: wheel
promise: A per-item field of the status document that the agent polls.
@row_check n=1054 row=contract/agent-run-status-polled-fields/wheel sha=b61ef4c9ecad v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:173
quote: pub wheel: Option<Wheel>,
@row_add n=1055 row=contract/agent-run-status-polled-fields/budget-paused area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: budget_paused
promise: A per-item field of the status document that the agent polls.
@row_check n=1055 row=contract/agent-run-status-polled-fields/budget-paused sha=c7b8b0fde519 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:175
quote: pub budget_paused: Option<bool>,
@row_add n=1056 row=contract/agent-run-status-polled-fields/dialog-pending area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: dialog_pending
promise: A per-item field of the status document that the agent polls.
@row_check n=1056 row=contract/agent-run-status-polled-fields/dialog-pending sha=637008950bb0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:176
quote: pub dialog_pending: bool,
@row_add n=1057 row=contract/agent-run-status-polled-fields/cli-verified area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: cli_verified
promise: A per-item field of the status document that the agent polls.
@row_check n=1057 row=contract/agent-run-status-polled-fields/cli-verified sha=6ecc11aa3cd7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:180
quote: pub cli_verified: Option<bool>,
@row_add n=1058 row=contract/agent-run-status-polled-fields/skipped area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: skipped
promise: An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
@row_check n=1058 row=contract/agent-run-status-polled-fields/skipped sha=cd2268380f4b v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
at: crates/viola-state/src/board.rs:211-213
quote: pub unknown_kinds: u64,     pub unknown_fields: u64,     pub torn_lines: u64,
@row_add n=1059 row=contract/agent-run-status-polled-fields/budget area=5-command-implementation rec=fct-ipfkedoi@1
boundary: agent-run status polled fields
item: budget
promise: An envelope field of the status document that the agent polls.
@row_check n=1059 row=contract/agent-run-status-polled-fields/budget sha=212e9567b4a3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
@row_add n=1060 row=contract/agent-run-status-exit-codes/0 area=5-command-implementation rec=fct-oui2ztbh@1,fct-kr53tti7@1
boundary: agent-run status exit codes
item: 0
promise: Exit 0 when ok is true, which is only for state ready.
@row_check n=1060 row=contract/agent-run-status-exit-codes/0 sha=1a7b582b7ac7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:108
quote: let ready = derived.state == "ready";
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=1061 row=contract/agent-run-status-exit-codes/1 area=5-command-implementation rec=fct-oui2ztbh@1,fct-kr53tti7@1
boundary: agent-run status exit codes
item: 1
promise: Exit 1 for degraded or down; the full status document is still printed, so an agent waiting for a stale transition reads exit 1 as a state, not a harness fault.
@row_check n=1061 row=contract/agent-run-status-exit-codes/1 sha=56fe26ea2f67 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
at: crates/viola-e2e/src/harness/status.rs:111
quote: "v": 1, "cmd": "status", "ok": ready, "state": derived.state,
@row_add n=1062 row=contract/agent-run-status-exit-codes/2 area=5-command-implementation rec=fct-oui2ztbh@1,fct-kr53tti7@1
boundary: agent-run status exit codes
item: 2
promise: Exit 2 on a usage error, or when no session.json exists for the id, printing {"v":1,"cmd":"status","ok":false,"reason":"unknown-session"}.
@row_check n=1062 row=contract/agent-run-status-exit-codes/2 sha=12b63bde3905 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:326-327
quote: doc: json!({"v": 1, "cmd": cmd, "ok": false, "reason": "unknown-session"}),         code: 2,
at: crates/viola-e2e/src/harness/mod.rs:51
quote: code: 2,
@row_add n=1063 row=decision/cleanup-ctrl-c-repeated-until-exit area=5-command-implementation rec=fct-beydn7zm@2
name: Cleanup stops agents with a repeated Ctrl-C
rule: Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline.
why: A Ctrl-C written into a child that has not set raw mode yet is swallowed.
@row_check n=1063 row=decision/cleanup-ctrl-c-repeated-until-exit sha=17e8971c4237 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/supervise.rs:120
quote: const CTRL_C_EVERY: Duration = Duration::from_millis(500);
at: crates/viola-e2e/src/harness/supervise.rs:153-155
quote: if expired(next_ctrl_c) { ctrl_c(input); next_ctrl_c = Instant::now() + CTRL_C_EVERY;
at: crates/viola-e2e/src/harness/supervise.rs:157-158
quote: if expired(deadline) { let _ = pty.kill();
at: crates/viola-e2e/src/harness/supervise.rs:232
quote: let exits = stop(&mut wrappers, Instant::now() + Duration::from_secs(10));
@row_add n=1064 row=decision/e2e-home-deletion-through-link area=5-command-implementation rec=fct-beydn7zm@2,fct-x3qmgvn2@2
name: Test-home deletion through the dev-host link
rule: On the Linux dev host, where target/e2e-home is a link, harness cleanup and a TestHome's drop and owner sweep delete outside the working directory, but only dirs the harness or the test side itself created.
why: A boundary widening, ratified.
@row_check n=1064 row=decision/e2e-home-deletion-through-link sha=29a90b2eccf9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:158-165
quote: let is_session_dir = parent .file_name() .and_then(|n| n.to_str()) .is_some_and(|n| n.starts_with("viola-session-")); if !is_session_dir || parent.parent() != Some(ws.e2e_home().as_path()) { return false; } let _ = fs::remove_dir_all(parent);
at: tests/support/home.rs:86
quote: pub fn remove_owned(dir: &Path) -> bool {
@row_add n=1065 row=decision/cleanup-is-idempotent area=5-command-implementation rec=fct-rttkzq3u@1
name: Idempotent cleanup
rule: Harness cleanup must be idempotent: with no session record it prints {"v":1,"cmd":"cleanup","ok":true,"cleaned":[]} and exits 0.
@row_check n=1065 row=decision/cleanup-is-idempotent sha=d25d1a6dcaa5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:54
quote: json!({"v": 1, "cmd": "cleanup", "ok": true, "cleaned": []}),
@row_add n=1066 row=contract/cleanup-document/processes-gone area=5-command-implementation rec=fct-tqogk4ey@1
boundary: cleanup document
item: processes_gone
promise: True when the supervisor, every wrapper and every child are gone, checked by pid and start time; survivors are force-killed first.
@row_check n=1066 row=contract/cleanup-document/processes-gone sha=23c95c037890 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:96-100
quote: let mut targets: Vec<(String, ProcessId)> = vec![("supervisor".to_owned(), supervisor)]; for inst in &record.instances { targets.push((inst.name.clone(), inst.wrapper())); targets.push((inst.name.clone(), inst.child())); }
at: crates/viola-e2e/src/harness/cleanup.rs:102
quote: if id.kill() && !killed.contains(name) {
at: crates/viola-e2e/src/harness/cleanup.rs:107
quote: let processes_gone = targets.iter().all(|(_, id)| id.wait_gone(deadline));
@row_add n=1067 row=contract/cleanup-document/endpoint-gone area=5-command-implementation rec=fct-tqogk4ey@1,fct-beydn7zm@2
boundary: cleanup document
item: endpoint_gone
promise: True when the recorded wrapper channel endpoint is gone: a connect is refused or the socket path is absent; a leftover socket file no one listens on after a force-killed wrapper reads true, and cleanup removes nothing of it.
@row_check n=1067 row=contract/cleanup-document/endpoint-gone sha=9708e6b9bf6e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:138-139
quote: std::os::unix::net::UnixStream::connect(endpoint) .is_err_and(|e| matches!(e.kind(), ConnectionRefused | NotFound))
at: crates/viola-e2e/src/harness/cleanup.rs:108-111
quote: let endpoint_gone = record.instances.iter().all(|inst| { let recorded = snapshot_endpoint(&instance_dir(&record.home, &inst.name)); recorded.as_deref().is_some_and(unconnectable) });
@row_add n=1068 row=contract/cleanup-document/home-removed area=5-command-implementation rec=fct-tqogk4ey@1,fct-beydn7zm@2
boundary: cleanup document
item: home_removed
promise: True when the session home was removed; reports "kept" when homes are kept, which is not a failure.
@row_check n=1068 row=contract/cleanup-document/home-removed sha=734d49232a14 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:113-117
quote: let home_removed = if keep_homes { json!("kept") } else { json!(remove_session_home(ws, &record.home)) };
at: crates/viola-e2e/src/harness/cleanup.rs:166
quote: !parent.exists()
@row_add n=1069 row=contract/cleanup-document/killed area=5-command-implementation rec=fct-tqogk4ey@1
boundary: cleanup document
item: killed
promise: Lists the names force-killed; a force-kill is reported but does not by itself fail cleanup.
@row_check n=1069 row=contract/cleanup-document/killed sha=76d00538a89b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:71
quote: "home_removed": home_removed, "killed": killed,
at: crates/viola-e2e/src/harness/cleanup.rs:66
quote: let ok = processes_gone && endpoint_gone && home_removed != false;
@row_add n=1070 row=contract/cleanup-exit-codes/1 area=5-command-implementation rec=fct-tqogk4ey@1
boundary: cleanup exit codes
item: 1
promise: Any false field in the cleanup document means exit 1.
@row_check n=1070 row=contract/cleanup-exit-codes/1 sha=264b7e85baed v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:66
quote: let ok = processes_gone && endpoint_gone && home_removed != false;
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=1071 row=contract/agent-run-logs-stream/events-line area=5-command-implementation rec=fct-mfhdbyoe@1
boundary: agent-run logs ndjson stream
item: events line
promise: Every events.ndjson line of an instance, wrapped as {"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}.
@row_check n=1071 row=contract/agent-run-logs-stream/events-line sha=a3f73091ede3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:133-135
quote: head.insert("src".to_owned(), "events".into()); head.insert("instance".to_owned(), instance.into()); head.insert("offset".to_owned(), at.into());
at: crates/viola-e2e/src/harness/logs.rs:144
quote: wrapped.insert("record".to_owned(), record);
@row_add n=1072 row=contract/agent-run-logs-stream/diag-line area=5-command-implementation rec=fct-mfhdbyoe@1
boundary: agent-run logs ndjson stream
item: diag line
promise: Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.
@row_check n=1072 row=contract/agent-run-logs-stream/diag-line sha=c8b6cb7c9f29 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:177-178
quote: head.insert("src".to_owned(), "diag".into()); head.insert("file".to_owned(), file.into());
at: crates/viola-e2e/src/harness/logs.rs:78
quote: files_in(dir, |n| n.ends_with(".ndjson"))
@row_add n=1073 row=contract/agent-run-logs-stream/detail-line area=5-command-implementation rec=fct-mfhdbyoe@1
boundary: agent-run logs ndjson stream
item: detail line
promise: Every content-bearing detail line of an instance's detail files, wrapped as {"src":"diag","file":<basename>,"instance":<name>,"record":<line>}; instance disambiguates the same basename across instances.
@row_check n=1073 row=contract/agent-run-logs-stream/detail-line sha=acaf9881624f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:106-108
quote: let files = files_in(&dir, |n| n.starts_with("detail-") && n.ends_with(".ndjson")); for (file, bytes) in files { out.extend(wrap_in(&file, Some(&name), &bytes, filter));
at: crates/viola-e2e/src/harness/logs.rs:179-181
quote: if let Some(instance) = instance { head.insert("instance".to_owned(), instance.into()); }
@row_add n=1074 row=contract/agent-run-logs-stream/torn-line area=5-command-implementation rec=fct-mfhdbyoe@1
boundary: agent-run logs ndjson stream
item: torn line
promise: A torn or unparseable line is emitted as {"src":…,"torn":true,"offset":n} and never dropped.
@row_check n=1074 row=contract/agent-run-logs-stream/torn-line sha=42bddf4f2245 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:186-188
quote: out.insert("torn".to_owned(), true.into()); out.insert("offset".to_owned(), at.into()); Value::Object(out)
at: crates/viola-e2e/src/harness/logs.rs:204
quote: _ => out.push(torn(at)),
@row_add n=1075 row=decision/panic-counts-read-role-files-only area=5-command-implementation rec=fct-mfhdbyoe@1
name: Panic counts read role files only
rule: Panic counts over the logs stream use the role files only, excluding detail files.
why: A panic line appears both in a role file and in its detail file.
@row_check n=1075 row=decision/panic-counts-read-role-files-only sha=7c8e2311f8cb v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:92-95
quote: .filter(|p| { p.file_name() .is_some_and(|n| !n.to_string_lossy().starts_with("detail-")) })
@row_add n=1076 row=decision/log-assertions-via-jq-or-jaq area=5-command-implementation rec=fct-mfhdbyoe@1,fct-z43sni6d@2
name: Log assertions through jq or jaq
rule: Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
@row_check n=1076 row=decision/log-assertions-via-jq-or-jaq sha=22e7fa8c2df4 v=unverifiable judge=sonnet head=8834195b0683
searched: jq -e · in scripts/**, .github/**, crates/**/*.rs, tests/**
searched: jaq · in scripts/**, .github/**, crates/**/*.rs, tests/**
searched: "jq" · in scripts/**, .github/**, crates/**/*.rs, tests/**
why: no evidence found
@row_add n=1077 row=decision/session-home-retention area=5-command-implementation rec=fct-xifaibmw@1
name: Session home retention
rule: Logs are retained for the whole lifetime of the session home; events are never truncated, and cleanup deletes the home except where homes are kept.
@row_check n=1077 row=decision/session-home-retention sha=6404bb2d3c7e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/cleanup.rs:60-61
quote: let home_removed = if keep_homes { json!("kept")
@row_add n=1078 row=decision/e2e-homes-under-one-root area=5-command-implementation rec=fct-xifaibmw@1
name: All test homes under target/e2e-home
rule: Every harness and rstest home lives under target/e2e-home.
why: It is the root obs-plan's CI gates and upload read.
@row_check n=1078 row=decision/e2e-homes-under-one-root sha=caeda455ff53 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:165-167
quote: let parent = tempfile::Builder::new() .prefix("viola-session-") .tempdir_in(opts.ws.e2e_home())?
at: tests/support/home.rs:193
quote: let base = workspace_path("target/e2e-home");
@row_add n=1079 row=decision/dev-host-e2e-home-on-tmpfs area=5-command-implementation rec=fct-xifaibmw@1,fct-p6be5x2e@1,fct-x3qmgvn2@2
name: Dev-host test homes on tmpfs
rule: On the Linux dev host target/e2e-home is an operator-set link to an owner-only tmpfs directory, so a kept home lives in memory, does not survive a reboot and ages out of /tmp after ten days untouched; CI is unchanged.
@row_check n=1079 row=decision/dev-host-e2e-home-on-tmpfs sha=2cbd8c0a9257 v=unverifiable judge=sonnet head=8834195b0683
searched: tmpfs · in tests/support/home.rs, crates/viola-e2e/src/harness/mod.rs (the code checks only an absolute, real, owner-only link target at home.rs:129-136; tmpfs, reboot and ten-day ageing are host facts no code shows)
why: not a code claim
@row_add n=1080 row=decision/internal-harness-subcommands-off-agent-surface area=5-command-implementation rec=fct-p5mffmqj@1
name: Internal harness subcommands
rule: The internal harness subcommands are implemented in viola-harness, forwarded unchanged by the shims, and are not part of the agent's 5-command surface.
@row_check n=1080 row=decision/internal-harness-subcommands-off-agent-surface sha=820d66af18d8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:120-122
quote: "supervise", "schema-check", "secret-scan",
at: scripts/agent-run.sh:43
quote: exec cargo run -q -p viola-e2e --bin viola-harness -- "$@"
@row_add n=1081 row=decision/supervisor-spawned-only-by-boot area=5-command-implementation rec=fct-r55p5ks7@1
name: Supervisor spawned only by boot
rule: The supervise subcommand is spawned only by boot, as an ordinary child process that outlives boot, never by an agent or directly by a test; it owns the outer PTYs and runs the stop steps when a stop request appears.
@row_check n=1081 row=decision/supervisor-spawned-only-by-boot sha=97deffd0076d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:316-318
quote: fn spawn_supervisor(bin_dir: &Path, session: &str) -> Result<ProcessId, HarnessError> { let child = Command::new(exe(bin_dir, "viola-harness")) .args(["supervise", "--session", session])
at: crates/viola-e2e/src/harness/supervise.rs:96
quote: while !stop_requested(&dir) {
@row_add n=1082 row=contract/supervisor-exit-file/exits area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervisor-exit.json
item: exits
promise: One entry per instance: {"name":<instance>,"exit_code":<int|null>,"killed":<bool>}, inside {"v":1,"cmd":"supervise","ok":<bool>,"exits":[…]}.
@row_check n=1082 row=contract/supervisor-exit-file/exits sha=21443cc93d47 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/supervise.rs:137
quote: json!({"name": w.name, "exit_code": code, "killed": killed})
at: crates/viola-e2e/src/harness/supervise.rs:108
quote: json!({"v": 1, "cmd": "supervise", "ok": ok, "exits": exits}),
@row_add n=1083 row=contract/supervisor-exit-file/stdout area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervisor-exit.json
item: stdout
promise: The supervisor's stdout is discarded; the JSON document is the file.
@row_check n=1083 row=contract/supervisor-exit-file/stdout sha=5c48f220371b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:318-321
quote: .args(["supervise", "--session", session]) .stdin(Stdio::null()) .stdout(Stdio::null()) .stderr(Stdio::null())
at: crates/viola-e2e/src/harness/supervise.rs:100
quote: let _ = write_json(&dir.join("supervisor-exit.json"), &outcome.doc);
@row_add n=1084 row=contract/supervise-exit-codes/0 area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervise exit codes
item: 0
promise: Exit 0 when no child needed kill().
@row_check n=1084 row=contract/supervise-exit-codes/0 sha=3a5353b64a30 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/supervise.rs:106
quote: let ok = exits.iter().all(|e| e["killed"] == false);
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=1085 row=contract/supervise-exit-codes/1 area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervise exit codes
item: 1
promise: Exit 1 when any child needed kill().
@row_check n=1085 row=contract/supervise-exit-codes/1 sha=2a597c1610ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/supervise.rs:106
quote: let ok = exits.iter().all(|e| e["killed"] == false);
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=1086 row=contract/supervise-exit-codes/2 area=5-command-implementation rec=fct-r55p5ks7@1
boundary: supervise exit codes
item: 2
promise: Exit 2 on a usage error.
@row_check n=1086 row=contract/supervise-exit-codes/2 sha=1e4eaa5eadc2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:51
quote: code: 2,
at: crates/viola-e2e/src/harness/supervise.rs:87
quote: return Outcome::usage(Some("supervise"), "no-supervise-spec");
@row_add n=1087 row=decision/diagnostics-schema-validation area=5-command-implementation rec=fct-7hikjies@1
name: Schema conformance check of diagnostics
rule: Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
@row_check n=1087 row=decision/diagnostics-schema-validation sha=e47aa7cdfb1c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:110-113
quote: let schema = match kind { DiagKind::Role => &line_schema, DiagKind::Detail => &detail_schema, };
at: Cargo.toml:211
quote: jsonschema = { version = "=0.57.0", default-features = false }
at: .github/workflows/ci.yml:64
quote: run: bash scripts/agent-run.sh schema-check
@row_add n=1088 row=contract/schema-check-output/failures area=5-command-implementation rec=fct-7hikjies@1
boundary: schema-check output
item: failures
promise: Each failure is {file, line, keyword}, file relative to target/e2e-home; no line content is ever printed.
@row_check n=1088 row=contract/schema-check-output/failures sha=87fc0221e649 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:126
quote: failures.push(json!({"file": file, "line": n + 1, "keyword": keyword}));
@row_add n=1089 row=contract/schema-check-output/torn area=5-command-implementation rec=fct-7hikjies@1
boundary: schema-check output
item: torn
promise: The count of non-JSON lines.
@row_check n=1089 row=contract/schema-check-output/torn sha=0a2c82a8787c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:116
quote: torn += 1;
@row_add n=1090 row=contract/schema-check-output/skipped-non-file area=5-command-implementation rec=fct-7hikjies@1
boundary: schema-check output
item: skipped_non_file
promise: The count of directories at a diagnostics file path, which are never read.
@row_check n=1090 row=contract/schema-check-output/skipped-non-file sha=57e1a23952f8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:47
quote: out.skipped_non_file += 1;
@row_add n=1091 row=contract/schema-check-reason/empty-scope area=5-command-implementation rec=fct-7hikjies@1,fct-mihqkrxe@2
boundary: schema-check reason
item: empty-scope
promise: No diagnostics file was found.
@row_check n=1091 row=contract/schema-check-reason/empty-scope sha=bc180a3dd990 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:144
quote: doc["reason"] = json!("empty-scope");
@row_add n=1092 row=contract/schema-check-reason/schema-unreadable area=5-command-implementation rec=fct-7hikjies@1,fct-mihqkrxe@2
boundary: schema-check reason
item: schema-unreadable
promise: A schema could not be read.
@row_check n=1092 row=contract/schema-check-reason/schema-unreadable sha=5d41476a9550 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:96
quote: json!({"v": 1, "cmd": "schema-check", "ok": false, "reason": "schema-unreadable"}),
@row_add n=1093 row=contract/schema-check-exit-codes/2 area=5-command-implementation rec=fct-7hikjies@1
boundary: schema-check exit codes
item: 2
promise: Exit 2 on a usage error.
@row_check n=1093 row=contract/schema-check-exit-codes/2 sha=c33ebe377f0d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:50-51
quote: doc: json!({"v": 1, "cmd": cmd, "ok": false, "reason": "usage", "detail": detail}), code: 2,
@row_add n=1094 row=decision/secret-scan-scope area=5-command-implementation rec=fct-5z3qj22n@2
name: Secret scan scope
rule: The secret scan covers every diagnostics ndjson under the test homes (detail files included), every file of the harness capture and the nextest CI junit report, and runs in CI before any upload.
why: The run --mutants chunk diff is repository source text that hits the scan's own needles.
except: The run --mutants chunk.diff at the capture root is never scanned and never uploaded; a chunk.diff anywhere else is still scanned.
@row_check n=1094 row=decision/secret-scan-scope sha=823cd8a18f11 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:63
quote: let diag = diag_files(&roots.e2e_home);
at: crates/viola-e2e/src/harness/secret_scan.rs:79-80
quote: let mutation_diff = roots.agent_run.join("chunk.diff"); capture.retain(|path| *path != mutation_diff);
at: crates/viola-e2e/src/harness/secret_scan.rs:81-82
quote: if roots.junit.is_file() { capture.push(roots.junit.clone());
at: .github/workflows/ci.yml:76
quote: run: bash scripts/agent-run.sh secret-scan
@row_add n=1095 row=decision/canary-list-single-sourced area=5-command-implementation rec=fct-5z3qj22n@2
name: Single-sourced canary list
rule: Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table.
guard: crates/viola-e2e/tests/scan_patterns.rs
@row_check n=1095 row=decision/canary-list-single-sourced sha=b64e67391f20 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/tests/scan_patterns.rs:62-65
quote: let missing: Vec<&String> = planted .iter() .filter(|c| !scanned.contains(&c.as_str())) .collect();
guard_at: crates/viola-e2e/tests/scan_patterns.rs:43
@row_add n=1096 row=contract/secret-scan-class/claude-stripped area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: claude-stripped
promise: A hit of the fixed CLAUDE* canaries the root tests plant.
@row_check n=1096 row=contract/secret-scan-class/claude-stripped sha=7b21f724ce72 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:17
quote: ("claude-stripped", "canary-token-value-7f3a"),
@row_add n=1097 row=contract/secret-scan-class/cookie area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: cookie
promise: A hit of the cookie: header, case-insensitive.
@row_check n=1097 row=contract/secret-scan-class/cookie sha=f6799e420969 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:129-130
quote: for offset in find_all(&lower, b"cookie:") { report(offset, "cookie");
@row_add n=1098 row=contract/secret-scan-class/content-canary area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: content-canary
promise: A content canary, failing only in a home-level role file.
@row_check n=1098 row=contract/secret-scan-class/content-canary sha=a387b07ac1c8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:132-133
quote: if role { for (class, needle) in CONTENT {
@row_add n=1099 row=contract/secret-scan-class/mode area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: mode
promise: Unix only: a diagnostics file that is not 0600.
@row_check n=1099 row=contract/secret-scan-class/mode sha=5b49fda79778 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:180
quote: mode & 0o777 == 0o600
@row_add n=1100 row=contract/secret-scan-class/unreadable area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan hit class
item: unreadable
promise: A file in scope that could not be read.
@row_check n=1100 row=contract/secret-scan-class/unreadable sha=981e1db72302 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:116
quote: hits.push(hit(roots, path, 0, 0, "unreadable"));
@row_add n=1101 row=contract/secret-scan-output/hits area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan output
item: hits
promise: Each hit is {file, line, offset, class}; the matched bytes are never printed.
@row_check n=1101 row=contract/secret-scan-output/hits sha=15ef212c1152 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
@row_add n=1102 row=contract/secret-scan-output/report area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan output
item: report
promise: "report":"unwritten" is added when the hit file cannot be written.
@row_check n=1102 row=contract/secret-scan-output/report sha=2c668b009572 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:107
quote: doc["report"] = json!("unwritten");
@row_add n=1103 row=contract/secret-scan-hits-file/hits-json area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan hits file
item: target/secret-scan/hits.json
promise: Holds the same document as stdout, written only when there are hits; a stale one is deleted on every run.
@row_check n=1103 row=contract/secret-scan-hits-file/hits-json sha=8a7442de7de4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:62
quote: let _ = fs::remove_file(report_dir.join("hits.json"));
at: crates/viola-e2e/src/harness/secret_scan.rs:103-105
quote: if !hits.is_empty() { let written = fs::create_dir_all(report_dir) .is_ok_and(|()| write_json(&report_dir.join("hits.json"), &doc).is_ok());
@row_add n=1104 row=contract/secret-scan-reason/empty-scope area=5-command-implementation rec=fct-5z3qj22n@2,fct-mihqkrxe@2
boundary: secret-scan reason
item: empty-scope
promise: Nothing in scope was found.
@row_check n=1104 row=contract/secret-scan-reason/empty-scope sha=53a3f362e0d9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:101
quote: doc["reason"] = json!("empty-scope");
@row_add n=1105 row=contract/secret-scan-exit-codes/1 area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan exit codes
item: 1
promise: Exit 1 on any hit.
@row_check n=1105 row=contract/secret-scan-exit-codes/1 sha=f72f5ddce983 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:90
quote: let ok = !empty && hits.is_empty();
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=1106 row=contract/secret-scan-exit-codes/2 area=5-command-implementation rec=fct-5z3qj22n@2
boundary: secret-scan exit codes
item: 2
promise: Exit 2 on a usage error.
@row_check n=1106 row=contract/secret-scan-exit-codes/2 sha=dd82ac2bb406 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:241
quote: return emit(Outcome::usage(cmd, "arguments"));
at: crates/viola-e2e/src/harness/mod.rs:51
quote: code: 2,
@row_add n=1107 row=contract/gate-require/suites area=5-command-implementation rec=fct-47aec6c3@3,fct-mihqkrxe@2
boundary: gate --require
item: suites
promise: Takes only the eight closed suite values other than local-live; an empty list or any other value (a11y included) is usage unknown-suite.
@row_check n=1107 row=contract/gate-require/suites sha=92feb0f5cf06 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:14-23
quote: pub const SUITES: [&str; 8] = [ "nextest-unit", "nextest-integration", "doctest", "nextest-e2e", "mutants", "coverage", "perf", "fuzz-replay", ];
at: crates/viola-e2e/src/harness/gate.rs:38
quote: .all(|s| SUITES.contains(&s.as_str()))
at: crates/viola-e2e/src/bin/viola-harness.rs:225
quote: return emit(Outcome::usage(Some("gate"), "unknown-suite"));
@row_add n=1108 row=decision/junit-copied-after-each-invocation area=5-command-implementation rec=fct-47aec6c3@3
name: JUnit report copied after each nextest invocation
rule: The run command deletes the nextest junit report before each invocation and copies it per suite right after; a report missing afterwards is an artifact-missing breach, never a copy of an earlier run's file.
why: nextest overwrites that path on every invocation.
@row_check n=1108 row=decision/junit-copied-after-each-invocation sha=d1573c69a0d8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/nextest.rs:53
quote: let _ = fs::remove_file(junit_source(ws));
at: crates/viola-e2e/src/harness/run/nextest.rs:38
quote: let _ = fs::write(&artifact, &xml);
at: crates/viola-e2e/src/harness/run/nextest.rs:30-33
quote: let Ok(xml) = fs::read_to_string(junit_source(ws)) else { return Suite { failed: 1, failures: vec!["artifact-missing".to_owned()],
@row_add n=1109 row=decision/mutants-survived-count area=5-command-implementation rec=fct-47aec6c3@3
name: Mutants survived count
rule: The mutants suite's survived count is missed minus host-excluded plus timeout, from the run's own cargo-mutants outcomes.
@row_check n=1109 row=decision/mutants-survived-count sha=62a471c32624 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
@row_add n=1110 row=decision/missing-artifact-is-breach area=5-command-implementation rec=fct-47aec6c3@3
name: A missing artifact is a breach
rule: A gate input that is missing or unreadable is a breach, never a pass.
@row_check n=1110 row=decision/missing-artifact-is-breach sha=c6b93151723c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:52-53
quote: if JUNIT.contains(&suite) && !artifacts.join(format!("junit-{suite}.xml")).is_file() { breaches.push(breach(
at: crates/viola-e2e/src/harness/gate.rs:102-104
quote: let Ok(summary) = read_json::<Value>(&artifacts.join("llvm-cov-summary.json")) else { breaches.push(breach( "artifact-missing",
at: crates/viola-e2e/src/harness/gate.rs:133-134
quote: if !artifacts.join(&name).is_file() { breaches.push(breach("artifact-missing", "perf", name));
@row_add n=1111 row=contract/gate-breach/suite-missing area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: suite-missing
promise: A required suite is absent from the run summary.
@row_check n=1111 row=contract/gate-breach/suite-missing sha=e3e9ec51c1ea v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:86
quote: breaches.push(breach("suite-missing", suite, "absent".to_owned()));
@row_add n=1112 row=contract/gate-breach/suite-failed area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: suite-failed
promise: A required suite has failed > 0.
@row_check n=1112 row=contract/gate-breach/suite-failed sha=bcbc60da1c61 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:89-92
quote: for (gate, key) in [("suite-failed", "failed"), ("suite-skipped", "skipped")] { let n = count(entry, key); if n > 0 { breaches.push(breach(gate, suite, format!("{key} {n}")));
@row_add n=1113 row=contract/gate-breach/suite-skipped area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: suite-skipped
promise: A required suite has skipped > 0.
@row_check n=1113 row=contract/gate-breach/suite-skipped sha=2ff1f6f7814d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:89-92
quote: for (gate, key) in [("suite-failed", "failed"), ("suite-skipped", "skipped")] { let n = count(entry, key); if n > 0 { breaches.push(breach(gate, suite, format!("{key} {n}")));
@row_add n=1114 row=contract/gate-breach/coverage area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: coverage
promise: Coverage below a floor, each floor its own breach.
@row_check n=1114 row=contract/gate-breach/coverage sha=80148d3fcd05 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:111-118
quote: for (metric, floor) in COVERAGE_FLOORS { match totals[metric]["percent"].as_f64() { Some(pct) if pct >= floor => {} Some(pct) => breaches.push(breach( "coverage", "coverage", format!("{metric} {pct:.2} < {floor}"),
@row_add n=1115 row=contract/gate-breach/mutants area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: mutants
promise: The mutants suite has survived > 0.
@row_check n=1115 row=contract/gate-breach/mutants sha=d87b216d2745 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:96-97
quote: if suite == "mutants" && survived > 0 { breaches.push(breach("mutants", suite, format!("survived {survived}")));
@row_add n=1116 row=contract/gate-breach/perf area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: perf
promise: A perf result's max is at or over its gate; detail reads {file} max {max} >= {bound}.
@row_check n=1116 row=contract/gate-breach/perf sha=aa960704ecf4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:151-155
quote: Some(max) if max < bound => {} Some(max) => breaches.push(breach( "perf", "perf", format!("{file} max {max} >= {bound}"),
@row_add n=1117 row=contract/gate-breach/artifact-missing area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate breach
item: artifact-missing
promise: A required artifact is missing, each absent perf row its own breach naming that file.
@row_check n=1117 row=contract/gate-breach/artifact-missing sha=8d712527c569 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:134
quote: breaches.push(breach("artifact-missing", "perf", name));
@row_add n=1118 row=contract/gate-output/detail area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate output
item: detail
promise: A fixed code or a count, never file content.
@row_check n=1118 row=contract/gate-output/detail sha=3d4a94d4fcfa v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:42-44
quote: fn breach(gate: &str, suite: &str, detail: String) -> Value { json!({"gate": gate, "suite": suite, "detail": detail}) }
at: crates/viola-e2e/src/harness/gate.rs:92
quote: breaches.push(breach(gate, suite, format!("{key} {n}")));
@row_add n=1119 row=contract/gate-exit-codes/0 area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate exit codes
item: 0
promise: Exit 0 with no breaches.
@row_check n=1119 row=contract/gate-exit-codes/0 sha=89d775a160fc v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:66
quote: let ok = breaches.is_empty();
at: crates/viola-e2e/src/harness/gate.rs:68
quote: json!({"v": 1, "cmd": "gate", "ok": ok, "breaches": breaches}),
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=1120 row=contract/gate-exit-codes/1 area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate exit codes
item: 1
promise: Exit 1 on any breach.
@row_check n=1120 row=contract/gate-exit-codes/1 sha=1cd3e9853486 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:66
quote: let ok = breaches.is_empty();
at: crates/viola-e2e/src/harness/mod.rs:44
quote: code: if ok { 0 } else { 1 },
@row_add n=1121 row=contract/gate-exit-codes/2 area=5-command-implementation rec=fct-47aec6c3@3
boundary: gate exit codes
item: 2
promise: Exit 2 on a usage error.
@row_check n=1121 row=contract/gate-exit-codes/2 sha=7407dadb8011 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:225
quote: return emit(Outcome::usage(Some("gate"), "unknown-suite"));
at: crates/viola-e2e/src/harness/mod.rs:51
quote: code: 2,
@row_add n=1122 row=decision/ci-gate-last-step area=5-command-implementation rec=fct-47aec6c3@3,fct-da3wgwes@2
name: Gate as the last step of every test job
rule: CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.
@row_check n=1122 row=decision/ci-gate-last-step sha=d7fdcfe986f1 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:117
quote: run: bash scripts/agent-run.sh gate --require coverage,doctest
at: .github/workflows/ci.yml:189
quote: run: bash scripts/agent-run.sh gate --require perf
at: .github/workflows/ci.yml:229
quote: run: bash scripts/agent-run.sh gate --require nextest-unit
at: .github/workflows/ci.yml:253
quote: run: bash scripts/agent-run.sh gate --require fuzz-replay
@row_add n=1123 row=decision/no-ci-mutants-gate area=5-command-implementation rec=fct-47aec6c3@3
name: No CI mutants gate
rule: No CI job gates the mutants suite.
@row_check n=1123 row=decision/no-ci-mutants-gate sha=5c4e72cc807b v=holds judge=sonnet head=8834195b0683
searched: gate --require · in .github/workflows/ci.yml (4 hits: coverage,doctest · perf · nextest-unit · fuzz-replay, none names mutants)
searched: mutants · in .github/workflows/ci.yml (only tool pins and comments at lines 32, 33, 91, 209)
@row_add n=1124 row=decision/pre-push-local-linux-gate area=5-command-implementation rec=fct-gvvrytf4@2
name: pre-push local Linux gate
rule: The operator pass runs pre-push, Linux host only, on the uncommitted working tree before the pre-CI commit, and a red stage stops the pass; pre-push is not an agent command.
@row_check n=1124 row=decision/pre-push-local-linux-gate sha=ce7b73d4e9fd v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:52-54
quote: if !supported { doc.stage = "host"; return doc.finish(Err(Stop::new("pre-push-linux-only", "")), 2);
at: crates/viola-e2e/src/harness/pre_push.rs:89-92
quote: doc.stage = "tools"; let linux = Linux::new(&passwd_home(runner)?, &ws.root); tools(ws, &linux, runner)?; doc.stage = "linux-tests";
at: crates/viola-e2e/src/bin/viola-harness.rs:124
quote: "pre-push",
@row_add n=1125 row=decision/pre-push-clean-environment area=5-command-implementation rec=fct-gvvrytf4@2
name: pre-push children run in a clean environment
rule: Every pre-push child runs under env -i with HOME from the passwd entry and a PATH of constants only, inheriting no host environment; the harness's own HOME is never read.
why: No CLAUDE* value crosses.
@row_check n=1125 row=decision/pre-push-clean-environment sha=3e2efbf0daaf v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:39-40
quote: cmd.arg("-i") .arg(format!("HOME={}", self.home))
at: crates/viola-e2e/src/harness/pre_push/linux.rs:84
quote: cmd.args(["-i", SYSTEM_PATH]).args(argv);
at: crates/viola-e2e/src/harness/pre_push/linux.rs:92
quote: let entry = out(runner, probe(&["getent", "passwd", uid])).ok_or_else(missing)?;
@row_add n=1126 row=contract/pre-push-stage/host area=5-command-implementation rec=fct-gvvrytf4@2,fct-mihqkrxe@2
boundary: pre-push stage
item: host
promise: Off Linux, exit 2 with pre-push-linux-only.
@row_check n=1126 row=contract/pre-push-stage/host sha=3735363f040f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:54
quote: return doc.finish(Err(Stop::new("pre-push-linux-only", "")), 2);
@row_add n=1127 row=contract/pre-push-stage/tools area=5-command-implementation rec=fct-gvvrytf4@2,fct-mihqkrxe@2
boundary: pre-push stage
item: tools
promise: Checks the passwd home, cc, and rustc, cargo-nextest, cargo-mutants and cargo-llvm-cov at their pinned versions.
@row_check n=1127 row=contract/pre-push-stage/tools sha=eae703c53cc3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:81
quote: let missing = || Stop::new("tool-missing", "passwd-home");
at: crates/viola-e2e/src/harness/pre_push/linux.rs:106
quote: return Err(Stop::new("tool-missing", "cc"));
at: crates/viola-e2e/src/harness/pre_push/linux.rs:117-119
quote: let mut wanted = vec![("rustc".to_owned(), channel.clone())]; wanted.extend(pins); for (name, pin) in &wanted {
at: crates/viola-e2e/src/harness/pre_push/linux.rs:131-132
quote: if version != Some(pin.as_str()) { return Err(Stop::new("tool-pin-mismatch", name));
@row_add n=1128 row=contract/pre-push-stage/linux-tests area=5-command-implementation rec=fct-gvvrytf4@2,fct-mihqkrxe@2
boundary: pre-push stage
item: linux-tests
promise: Runs run --coverage then gate --require coverage,doctest; the last stage, ok:true when green.
@row_check n=1128 row=contract/pre-push-stage/linux-tests sha=47124561971b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:181
quote: let run = linux_harness(linux, &["run", "--coverage"], runner)?;
at: crates/viola-e2e/src/harness/pre_push/linux.rs:186
quote: let gate = linux_harness(linux, &["gate", "--require", "coverage,doctest"], runner)?;
at: crates/viola-e2e/src/harness/pre_push.rs:92-93
quote: doc.stage = "linux-tests"; linux_tests(&linux, runner, doc)
@row_add n=1129 row=decision/pre-push-runs-no-mutation area=5-command-implementation rec=fct-gvvrytf4@2
name: pre-push runs no mutation
rule: No pre-push stage runs mutation testing.
why: The cargo-mutants pin stays in the tools stage because run --coverage runs the real-cargo-mutants harness tests.
@row_check n=1129 row=decision/pre-push-runs-no-mutation sha=76d321f9b608 v=holds judge=sonnet head=8834195b0683
searched: mutants · in crates/viola-e2e/src/harness/pre_push.rs and crates/viola-e2e/src/harness/pre_push/linux.rs (only the cargo-mutants pin at linux.rs tools check and tests asserting no "--mutants" call; stages run only "run --coverage" at linux.rs:181 and "gate --require coverage,doctest" at linux.rs:186)
@row_add n=1130 row=contract/pre-push-output/paths area=5-command-implementation rec=fct-gvvrytf4@2
boundary: pre-push output
item: paths
promise: Codes, counts and repo-relative names only, never a home or repository path.
@row_check n=1130 row=contract/pre-push-output/paths sha=3a4d7e78043b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:150-153
quote: /// A run or gate document reduced to codes and counts: the suites' `artifact` paths are absolute, /// and neither the home nor the repository path reaches this document. pub(super) fn summary(doc: &Value) -> Value { let mut out = json!({"ok": doc["ok"]});
@row_add n=1131 row=contract/pre-push-exit-codes/0 area=5-command-implementation rec=fct-gvvrytf4@2
boundary: pre-push exit codes
item: 0
promise: Exit 0 when every stage is green.
@row_check n=1131 row=contract/pre-push-exit-codes/0 sha=3322ac04b305 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:69-71
quote: let ok = verdict == Ok(true); let mut doc = json!({"v": 1, "cmd": "pre-push", "ok": ok}); let mut code = u8::from(!ok);
@row_add n=1132 row=contract/pre-push-exit-codes/1 area=5-command-implementation rec=fct-gvvrytf4@2
boundary: pre-push exit codes
item: 1
promise: Exit 1 on a red stage or a stop reason.
@row_check n=1132 row=contract/pre-push-exit-codes/1 sha=5c36f58df985 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:57
quote: doc.finish(verdict, 1)
at: crates/viola-e2e/src/harness/pre_push.rs:77
quote: code = refused;
@row_add n=1133 row=contract/pre-push-exit-codes/2 area=5-command-implementation rec=fct-gvvrytf4@2
boundary: pre-push exit codes
item: 2
promise: Exit 2 on pre-push-linux-only or a usage error.
@row_check n=1133 row=contract/pre-push-exit-codes/2 sha=0708fca10811 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:54
quote: return doc.finish(Err(Stop::new("pre-push-linux-only", "")), 2);
@row_add n=1134 row=decision/closed-enums-need-decisions-log area=5-command-implementation rec=fct-mihqkrxe@2,fct-k3mrcwuf@1
name: Closed enums change only by decision
rule: A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
@row_check n=1134 row=decision/closed-enums-need-decisions-log sha=d74401ab9846 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=1135 row=contract/run-suite/local-live area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run suite
item: local-live
promise: Appears only under --local-live and is not accepted by gate --require.
@row_check n=1135 row=contract/run-suite/local-live sha=a8ab94208fec v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:149-150
quote: if sel.local_live && refusal.is_none() { suites.push(local_live(ws, runner));
at: crates/viola-e2e/src/harness/gate.rs:14-23
quote: pub const SUITES: [&str; 8] = [ "nextest-unit", "nextest-integration", "doctest", "nextest-e2e", "mutants", "coverage", "perf", "fuzz-replay", ];
@row_add n=1136 row=contract/local-live-failure/build area=5-command-implementation rec=fct-mihqkrxe@2
boundary: local-live failure code
item: build
promise: A closed local-live failure code.
@row_check n=1136 row=contract/local-live-failure/build sha=3dd56106a22e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:388
quote: suite.failures.push("build".to_owned());
@row_add n=1137 row=contract/local-live-failure/verify-exit-n area=5-command-implementation rec=fct-mihqkrxe@2
boundary: local-live failure code
item: verify-exit-<n>
promise: A closed local-live failure code.
@row_check n=1137 row=contract/local-live-failure/verify-exit-n sha=7aa8529775aa v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:410
quote: Some(n) => Some(format!("verify-exit-{n}")),
@row_add n=1138 row=contract/local-live-failure/verify-exit-none area=5-command-implementation rec=fct-mihqkrxe@2
boundary: local-live failure code
item: verify-exit-none
promise: A closed local-live failure code.
@row_check n=1138 row=contract/local-live-failure/verify-exit-none sha=8a4cc2d9fcb3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:411
quote: None => Some("verify-exit-none".to_owned()),
@row_add n=1139 row=contract/local-live-failure/row-missing area=5-command-implementation rec=fct-mihqkrxe@2
boundary: local-live failure code
item: row-missing
promise: A closed local-live failure code.
@row_check n=1139 row=contract/local-live-failure/row-missing sha=bf7ec87fdcec v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:409
quote: Some(0) => Some("row-missing".to_owned()),
@row_add n=1140 row=contract/boot-readiness-missing/run-process-start area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:run-process-start
promise: The run process start record of an instance is missing.
@row_check n=1140 row=contract/boot-readiness-missing/run-process-start sha=2a084448d7a8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:440
quote: missing.push(format!("{}:run-process-start", inst.name));
@row_add n=1141 row=contract/boot-readiness-missing/child-process-start area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:child-process-start
promise: The child process start record of an instance is missing.
@row_check n=1141 row=contract/boot-readiness-missing/child-process-start sha=9a39f63af947 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:443
quote: missing.push(format!("{}:child-process-start", inst.name));
@row_add n=1142 row=contract/boot-readiness-missing/snapshot area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:snapshot
promise: The snapshot of an instance is missing.
@row_check n=1142 row=contract/boot-readiness-missing/snapshot sha=e0ae0a4ba49f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:448
quote: missing.push(format!("{}:snapshot", inst.name));
@row_add n=1143 row=contract/boot-readiness-missing/endpoint area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:endpoint
promise: The endpoint of an instance is missing.
@row_check n=1143 row=contract/boot-readiness-missing/endpoint sha=01f44c53159e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:450
quote: missing.push(format!("{}:endpoint", inst.name));
@row_add n=1144 row=contract/boot-readiness-missing/heartbeat area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:heartbeat
promise: The heartbeat of an instance is missing.
@row_check n=1144 row=contract/boot-readiness-missing/heartbeat sha=b7450afdc255 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:453
quote: missing.push(format!("{}:heartbeat", inst.name));
@row_add n=1145 row=contract/boot-readiness-missing/events area=5-command-implementation rec=fct-mihqkrxe@2
boundary: boot readiness missing code
item: <name>:events
promise: The events of an instance are missing.
@row_check n=1145 row=contract/boot-readiness-missing/events sha=3c07eb5879fe v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:456
quote: missing.push(format!("{}:events", inst.name));
@row_add n=1146 row=contract/status-last-error/null area=5-command-implementation rec=fct-mihqkrxe@2,fct-cz5fdkae@1
boundary: status last_error
item: null
promise: No error.
@row_check n=1146 row=contract/status-last-error/null sha=158c867e38b5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:47
quote: last_error: None,
@row_add n=1147 row=contract/status-last-error/list-exit-code area=5-command-implementation rec=fct-mihqkrxe@2,fct-cz5fdkae@1,fct-kr53tti7@1
boundary: status last_error
item: list-exit-<code>
promise: viola list exited with that code (for example list-exit-21); list-exit-0 when it exited 0 with no JSON object on stdout.
@row_check n=1147 row=contract/status-last-error/list-exit-code sha=c4a3374f959d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:33
quote: (Some(c), _) => return down(&c.to_string()),
@row_add n=1148 row=contract/status-last-error/list-exit-none area=5-command-implementation rec=fct-mihqkrxe@2,fct-kr53tti7@1
boundary: status last_error
item: list-exit-none
promise: viola list gave no exit code, could not be spawned, or had no supervise record to run from.
@row_check n=1148 row=contract/status-last-error/list-exit-none sha=357502ccd5bb v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:31
quote: (None, _) => return down("none"),
@row_add n=1149 row=contract/mutants-verdict/scoped area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run --mutants verdict
item: scoped
promise: The run was scoped by --file.
@row_check n=1149 row=contract/mutants-verdict/scoped sha=8ad8ce90dd1c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:179-183
quote: let verdict = if files.is_empty() { "counted" } else { "scoped" };
@row_add n=1150 row=contract/run-reason/fuzz-linux-only area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run reason
item: fuzz-linux-only
promise: Exit 2.
@row_check n=1150 row=contract/run-reason/fuzz-linux-only sha=f4c0d2ba2191 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:136
quote: let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "fuzz-linux-only"});
at: crates/viola-e2e/src/harness/run.rs:137
quote: return Outcome { doc, code: 2 };
@row_add n=1151 row=contract/run-reason/tool-missing area=5-command-implementation rec=fct-mihqkrxe@2,fct-z43sni6d@2
boundary: run reason
item: tool-missing
promise: run --perf and --fuzz-replay check their tool is on PATH first; when missing they exit 1 with this reason and the tool name, never a silent pass.
@row_check n=1151 row=contract/run-reason/tool-missing sha=fada575e36db v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:151
quote: return Err(Refusal::new("tool-missing", Some("hyperfine")));
at: crates/viola-e2e/src/harness/run/fuzz.rs:49
quote: return Err(Refusal::new("tool-missing", Some("cargo-fuzz")));
@row_add n=1152 row=contract/run-reason/corpus-empty area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run reason
item: corpus-empty
promise: A closed run reason.
@row_check n=1152 row=contract/run-reason/corpus-empty sha=d00498ca882f v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/fuzz.rs:58
quote: return Err(Refusal::new("corpus-empty", None));
@row_add n=1153 row=contract/run-reason/live-in-ci area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run reason
item: live-in-ci
promise: Exit 2.
@row_check n=1153 row=contract/run-reason/live-in-ci sha=86310e85b277 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:132-133
quote: let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "live-in-ci"}); return Outcome { doc, code: 2 };
@row_add n=1154 row=contract/run-reason/package-refused area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run reason
item: package-refused
promise: Exit 2.
@row_check n=1154 row=contract/run-reason/package-refused sha=bb1482eb43b6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:144-145
quote: let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "package-refused"}); return Outcome { doc, code: 2 };
@row_add n=1155 row=contract/run-usage-detail/unknown-suite area=5-command-implementation rec=fct-mihqkrxe@2
boundary: run usage detail
item: unknown-suite
promise: The closed usage detail for a suite value outside the closed list.
@row_check n=1155 row=contract/run-usage-detail/unknown-suite sha=c4c409b90254 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:225
quote: return emit(Outcome::usage(Some("gate"), "unknown-suite"));
@row_add n=1156 row=contract/pre-push-reason/pre-push-linux-only area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push reason
item: pre-push-linux-only
promise: Exit 2.
@row_check n=1156 row=contract/pre-push-reason/pre-push-linux-only sha=eab737db604a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:54
quote: return doc.finish(Err(Stop::new("pre-push-linux-only", "")), 2);
@row_add n=1157 row=contract/pre-push-reason/tool-missing area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push reason
item: tool-missing
promise: Reused from run; its detail names the missing tool or passwd-home.
@row_check n=1157 row=contract/pre-push-reason/tool-missing sha=bfcf3b7415b8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:81
quote: let missing = || Stop::new("tool-missing", "passwd-home");
at: crates/viola-e2e/src/harness/pre_push/linux.rs:125
quote: return Err(Stop::new("tool-missing", name));
@row_add n=1158 row=contract/pre-push-reason/tool-pin-mismatch area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push reason
item: tool-pin-mismatch
promise: A closed pre-push reason.
@row_check n=1158 row=contract/pre-push-reason/tool-pin-mismatch sha=5b72ff687da7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:132
quote: return Err(Stop::new("tool-pin-mismatch", name));
@row_add n=1159 row=contract/pre-push-reason/linux-document-unreadable area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push reason
item: linux-document-unreadable
promise: The detail, run or gate, names the harness command whose document was unreadable.
@row_check n=1159 row=contract/pre-push-reason/linux-document-unreadable sha=20facf511866 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:147
quote: last_document(&stdout).ok_or_else(|| Stop::new("linux-document-unreadable", args[0]))
@row_add n=1160 row=contract/pre-push-detail/passwd-home area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: passwd-home
promise: Detail of tool-missing.
@row_check n=1160 row=contract/pre-push-detail/passwd-home sha=9e3c96426a69 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:81
quote: let missing = || Stop::new("tool-missing", "passwd-home");
@row_add n=1161 row=contract/pre-push-detail/cc area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: cc
promise: A closed pre-push detail.
@row_check n=1161 row=contract/pre-push-detail/cc sha=3c2b57f86422 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:106
quote: return Err(Stop::new("tool-missing", "cc"));
@row_add n=1162 row=contract/pre-push-detail/rustc area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: rustc
promise: A closed pre-push detail.
@row_check n=1162 row=contract/pre-push-detail/rustc sha=7ff093e3e2ba v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:117
quote: let mut wanted = vec![("rustc".to_owned(), channel.clone())];
at: crates/viola-e2e/src/harness/pre_push/linux.rs:125
quote: return Err(Stop::new("tool-missing", name));
@row_add n=1163 row=contract/pre-push-detail/cargo-nextest area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: cargo-nextest
promise: A closed pre-push detail.
@row_check n=1163 row=contract/pre-push-detail/cargo-nextest sha=990be566d4df v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:33
quote: tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1
at: crates/viola-e2e/src/harness/pre_push/linux.rs:132
quote: return Err(Stop::new("tool-pin-mismatch", name));
@row_add n=1164 row=contract/pre-push-detail/cargo-mutants area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: cargo-mutants
promise: A closed pre-push detail.
@row_check n=1164 row=contract/pre-push-detail/cargo-mutants sha=1954fcdf2cff v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:33
quote: tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1
at: crates/viola-e2e/src/harness/pre_push/linux.rs:125
quote: return Err(Stop::new("tool-missing", name));
@row_add n=1165 row=contract/pre-push-detail/cargo-llvm-cov area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: cargo-llvm-cov
promise: A closed pre-push detail.
@row_check n=1165 row=contract/pre-push-detail/cargo-llvm-cov sha=fcea5f821bd2 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:33
quote: tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1
at: crates/viola-e2e/src/harness/pre_push/linux.rs:125
quote: return Err(Stop::new("tool-missing", name));
@row_add n=1166 row=contract/pre-push-detail/pins-unreadable area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: pins-unreadable
promise: A closed pre-push detail.
@row_check n=1166 row=contract/pre-push-detail/pins-unreadable sha=47b3bc8cca3b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:108
quote: let unreadable = || Stop::new("tool-pin-mismatch", "pins-unreadable");
@row_add n=1167 row=contract/pre-push-detail/run area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: run
promise: The run document was unreadable.
@row_check n=1167 row=contract/pre-push-detail/run sha=1949c19bdf19 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:147
quote: last_document(&stdout).ok_or_else(|| Stop::new("linux-document-unreadable", args[0]))
@row_add n=1168 row=contract/pre-push-detail/gate area=5-command-implementation rec=fct-mihqkrxe@2
boundary: pre-push detail
item: gate
promise: The gate document was unreadable.
@row_check n=1168 row=contract/pre-push-detail/gate sha=8a9fd1a7019a v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push/linux.rs:186
quote: let gate = linux_harness(linux, &["gate", "--require", "coverage,doctest"], runner)?;
at: crates/viola-e2e/src/harness/pre_push/linux.rs:147
quote: last_document(&stdout).ok_or_else(|| Stop::new("linux-document-unreadable", args[0]))
@row_add n=1169 row=decision/ci-profile-no-retries area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5,fct-da3wgwes@2
name: CI test profile without retries
rule: The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
@row_check n=1169 row=decision/ci-profile-no-retries sha=33ca9326d2db v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:2
quote: retries = 0
at: .config/nextest.toml:3
quote: fail-fast = false
at: .config/nextest.toml:4
quote: slow-timeout = { period = "30s", terminate-after = 4 }
at: .config/nextest.toml:6-7
quote: [profile.ci.junit] path = "junit.xml"
@row_add n=1170 row=decision/verify-window-60s-kill area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: 60 s kill for the verify_window_ class
rule: Tests named verify_window_ take a 60 s per-test kill, in both the ci and mutants profiles, through an override that stands first.
why: Their cases wait out the gate's 8.5 s maximum several times by design.
guard: mutants_verify_window_override_stands_before_the_harness_package_override
@row_check n=1170 row=decision/verify-window-60s-kill sha=3ed3a3350944 v=holds guard=true judge=sonnet head=8834195b0683
at: .config/nextest.toml:18-19
quote: filter = 'test(/verify_window_/)' slow-timeout = { period = "20s", terminate-after = 3 }
at: .config/nextest.toml:42-43
quote: filter = 'test(/verify_window_/)' slow-timeout = { period = "20s", terminate-after = 3 }
guard_at: tests/contract_lints.rs:285
@row_add n=1171 row=decision/verify-driven-tests-20s-kill area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: Per-test kill bounds verify-driven tests
rule: A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound.
why: Ratified; sized from the measured verify floor plus a 3x runner tail.
@row_check n=1171 row=decision/verify-driven-tests-20s-kill sha=27cce20565cd v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:24-25
quote: filter = 'binary(channel_endpoint) | binary(channel_error_bodies) | binary(chaos_forward_compat) | binary(channel_links) | binary(chaos_torn_append) | binary(cli_answer) | binary(cli_budget) | binary(cli_error_bodies) | ... slow-timeout = { period = "10s", terminate-after = 2 }
@row_add n=1172 row=decision/within-7s-wait-bound area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: WITHIN bound for every other wait
rule: WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.
@row_check n=1172 row=decision/within-7s-wait-bound sha=23f63b5400a8 v=holds judge=sonnet head=8834195b0683
at: tests/support/watch.rs:14
quote: pub const WITHIN: Duration = Duration::from_secs(7);
@row_add n=1173 row=decision/mutants-fail-fast-wait area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: Mutants profile stops after one failure and waits
rule: The mutants nextest profile stops scheduling at the first failure and lets the tests already running finish (terminate = wait).
why: Running tests then drop their temp dirs and session guards; immediate termination left leftover temp dirs.
@row_check n=1173 row=decision/mutants-fail-fast-wait sha=c555c5311eeb v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:32
quote: fail-fast = { max-fail = 1, terminate = "wait" }
@row_add n=1174 row=decision/mutants-kill-below-cargo-mutants-timeout area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: Kill lines below cargo-mutants' timeout
rule: Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout.
why: At or above that floor a hang grades Timeout instead of caught.
guard: test_deadlines_sit_below_the_nextest_kill_line
@row_check n=1174 row=decision/mutants-kill-below-cargo-mutants-timeout sha=8008e55223f0 v=holds guard=true judge=sonnet head=8834195b0683
at: .config/nextest.toml:33
quote: slow-timeout = { period = "5s", terminate-after = 2 }
at: .config/nextest.toml:29-30
quote: # still end as a failed (caught) test, not a cargo-mutants timeout: cargo-mutants' auto timeout floors # at 20 s, so every in-test wait and this kill (10 s) stay below it — at or above it, a hang grades Timeout.
guard_at: tests/contract_lints.rs:243
@row_add n=1175 row=decision/viola-e2e-30s-mutants-kill area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: 30 s mutants kill for the harness package
rule: Under the mutants profile the viola-e2e package takes a 30 s per-test kill.
why: The harness's own tests wait out its 20 s boot deadline by design.
@row_check n=1175 row=decision/viola-e2e-30s-mutants-kill sha=e4b4db64fc8d v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:48-49
quote: filter = 'package(viola-e2e)' slow-timeout = { period = "15s", terminate-after = 2 }
@row_add n=1176 row=decision/send-window-paste-hint-30s-kill area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: 30 s mutants kill for send-window classes
rule: Under the mutants profile the send_window_ and send_under_the_paste_hint_ test classes take a 30 s per-test kill.
why: The paste-hint cases wait out the product's 8.5 s gate maximum under a held paste hint, within a second of the 10 s default kill.
guard: mutants_paste_hint_override_holds_the_class_above_the_default_kill
@row_check n=1176 row=decision/send-window-paste-hint-30s-kill sha=13b11d1fd5a3 v=holds guard=true judge=sonnet head=8834195b0683
at: .config/nextest.toml:53-54
quote: filter = 'test(/send_window_/)' slow-timeout = { period = "15s", terminate-after = 2 }
at: .config/nextest.toml:60-61
quote: filter = 'test(/send_under_the_paste_hint_/)' slow-timeout = { period = "15s", terminate-after = 2 }
guard_at: tests/contract_lints.rs:307
@row_add n=1177 row=decision/fixture-git-no-auto-maintenance area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: Fixture git repos without auto maintenance
rule: Fixture git repositories in tests run git with maintenance.auto=false.
why: The detached auto maintenance each fixture commit spawned wrote into .git while the temp dir's drop walked it, leaving half-removed dirs.
@row_check n=1177 row=decision/fixture-git-no-auto-maintenance sha=0efd70b68a46 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:510
quote: "maintenance.auto=false",
at: crates/viola-e2e/src/harness/run/mutants/base.rs:265
quote: "maintenance.auto=false",
@row_add n=1178 row=decision/rmcp-client-only area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: rmcp client only in the harness
rule: The harness uses rmcp with the client feature only, without transport-child-process.
why: The test owns the viola mcp child process.
@row_check n=1178 row=decision/rmcp-client-only sha=12b69848a464 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:44
quote: rmcp = { workspace = true, features = ["client"] }
@row_add n=1179 row=decision/syn-cfg-reader-for-host-exclusion area=bootstrap-phases-derive-for-route-setup-project rec=fct-2j6rht5l@5
name: syn as the cfg reader
rule: syn and proc-macro2 are ordinary harness dependencies that read #[cfg] attributes for run --mutants' host exclusion.
@row_check n=1179 row=decision/syn-cfg-reader-for-host-exclusion sha=7ba4144b91d4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/Cargo.toml:36
quote: proc-macro2.workspace = true
at: crates/viola-e2e/Cargo.toml:39
quote: syn.workspace = true
@row_add n=1180 row=decision/agent-run-shim-over-harness area=bootstrap-phases-derive-for-route-setup-project rec=fct-piiirk6o@2,fct-p5mffmqj@1
name: agent-run shim over viola-harness
rule: The agent-run script is a shim over viola-harness for the five commands boot, run, status, cleanup and logs.
@row_check n=1180 row=decision/agent-run-shim-over-harness sha=d52565766f5f v=holds judge=sonnet head=8834195b0683
at: scripts/agent-run.sh:43
quote: exec cargo run -q -p viola-e2e --bin viola-harness -- "$@"
at: scripts/agent-run.sh:48
quote: echo "Usage: $0 {boot|run|status|cleanup|logs} [flags]" >&2
@row_add n=1181 row=decision/status-shape-ownership area=bootstrap-phases-derive-for-route-setup-project rec=fct-ftvtdczd@1,fct-weskv4ql@1,fct-4npcxfnu@1,fct-ahlkrnje@1
name: Ownership of the status shape
rule: The test plan is the binding source for the agent-run status shape; the nested product shapes inside it, viola list --json, come from the arch CLI contract; obs-plan reads it and may extend but not redefine it.
@row_check n=1181 row=decision/status-shape-ownership sha=a0c61886fe36 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=1182 row=decision/harness-log-fields-owned-by-test-plan area=bootstrap-phases-derive-for-route-setup-project rec=fct-yu57xy4j@1,fct-weskv4ql@1,fct-ahlkrnje@1,fct-k3mrcwuf@1
name: Ownership of the harness-grepped log fields
rule: The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first.
why: The harness greps logs for assertions, so a format break is a harness break.
@row_check n=1182 row=decision/harness-log-fields-owned-by-test-plan sha=daaad95e2f4b v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=1183 row=decision/cargo-llvm-cov-coverage area=bootstrap-phases-derive-for-route-setup-project rec=fct-qacavs3x@1
name: cargo-llvm-cov for coverage
rule: cargo-llvm-cov 0.9.1, installed through taiki-e/install-action, produces coverage as lcov and a JSON summary.
@row_check n=1183 row=decision/cargo-llvm-cov-coverage sha=c6e83d8574b7 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:33
quote: tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0,cargo-llvm-cov@0.9.1
at: crates/viola-e2e/src/harness/run/coverage.rs:34
quote: "target/lcov.info",
at: crates/viola-e2e/src/harness/run/coverage.rs:53
quote: .args(["llvm-cov", "report", "--json", "--summary-only"])
@row_add n=1184 row=decision/g2-zero-panics area=bootstrap-phases-derive-for-route-setup-project rec=fct-z43sni6d@2
name: G2 zero panics
rule: CI's log gate G2 fails closed on any event panic role line under the test homes, run as a probe then the check in the test and perf jobs.
except: A panic_location of exactly src/cmd/hook/seam.rs:<digits>.
@row_check n=1184 row=decision/g2-zero-panics sha=e61eeff8389b v=holds judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:28
quote: and (.panic_location | sub(":[0-9]+$"; "")) == $seam;
at: .github/workflows/ci.yml:59
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
at: .github/workflows/ci.yml:151
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
@row_add n=1185 row=decision/runner-provided-jq area=bootstrap-phases-derive-for-route-setup-project rec=fct-z43sni6d@2
name: Runner-provided jq in CI scripts
rule: CI scripts use the runner-provided jq, presence-checked and never installed, and refuse with tool-missing: jq without it.
@row_check n=1185 row=decision/runner-provided-jq sha=7de8341bea67 v=holds judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:12-13
quote: if ! jq --version >/dev/null 2>&1; then   echo "tool-missing: jq"
at: scripts/orphans-check.sh:18-19
quote: if ! jq --version >/dev/null 2>&1; then   echo "tool-missing: jq"
at: scripts/release-check.sh:8-9
quote: if ! jq --version >/dev/null 2>&1; then   echo "tool-missing: jq"
@row_add n=1186 row=decision/ripgrep-verified-install area=bootstrap-phases-derive-for-route-setup-project rec=fct-z43sni6d@2
name: ripgrep for G1 and G3, checksum-verified
rule: ripgrep 15.2.0 with PCRE2 is the G1/G3 gate tool, installed by a script that downloads the official release asset and checks it against its published sha256.
why: taiki-e/install-action has no ripgrep manifest.
@row_check n=1186 row=decision/ripgrep-verified-install sha=dc67d4d2bdb1 v=holds judge=sonnet head=8834195b0683
at: scripts/install-ripgrep.sh:10
quote: version=15.2.0
at: scripts/install-ripgrep.sh:73
quote: verify "$work/$asset" "$sha" "$asset" || exit 1
@row_add n=1187 row=decision/ci-actions-sha-pinned area=bootstrap-phases-derive-for-route-setup-project rec=fct-da3wgwes@2
name: SHA-pinned actions, no default permissions
rule: Every CI action is SHA-pinned, with permissions: {} at the workflow's top level.
@row_check n=1187 row=decision/ci-actions-sha-pinned sha=4013b76b483e v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:7
quote: permissions: {}
at: .github/workflows/nightly.yml:8
quote: permissions: {}
at: .github/workflows/ci.yml:22
quote: - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
@row_add n=1188 row=decision/ci-exit-code-gates area=bootstrap-phases-derive-for-route-setup-project rec=fct-da3wgwes@2
name: CI exit-code gates
rule: CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.
@row_check n=1188 row=decision/ci-exit-code-gates sha=24634ab422d6 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: .github/workflows/ci.yml:374
quote: cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources 2> target/supply-chain/deny-fuzz.json || { cat target/supply-chain/deny-fuzz.json; exit 1; }
at: .github/workflows/ci.yml:392
quote: zizmor --format=json .github/workflows/ > target/supply-chain/zizmor.json || { cat target/supply-chain/zizmor.json; exit 1; }
at: .github/workflows/ci.yml:314
quote: run: bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh
at: .github/workflows/ci.yml:338
quote: run: bash scripts/release-check.sh
@row_add n=1189 row=contract/log-format/json-per-line area=log-format rec=fct-ehp4mr57@1,fct-gxb3q2be@1
boundary: log format
item: line format
promise: JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
@row_check n=1189 row=contract/log-format/json-per-line sha=88d33bb28259 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:121
quote: .json()
@row_add n=1190 row=contract/log-streams/event-stream area=log-format rec=fct-ehp4mr57@1
boundary: log streams
item: event stream
promise: Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
@row_check n=1190 row=contract/log-streams/event-stream sha=df9c2cc33675 v=stale judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:25-28
quote: pub enum Source {     Wrapper,     Hook, }
shows: The event line's source enum has only wrapper and hook; no code emits source "cli".
cell: promise
value: Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
@row_add n=1191 row=decision/process-logs-via-tracing-subscriber area=log-format rec=fct-ehp4mr57@1
name: tracing-subscriber JSON process logs
rule: Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
@row_check n=1191 row=decision/process-logs-via-tracing-subscriber sha=13da609f5bee v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:243
quote: tracing-subscriber = { version = "=0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }
at: src/obs.rs:122-123
quote: .flatten_event(true)         .with_current_span(false)
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => {             format!("{}-{}", process.as_str(), name_str(name))
@row_add n=1192 row=decision/codes-only-role-logs area=log-format rec=fct-ehp4mr57@1
name: Codes-only role logs, detail apart
rule: Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
@row_check n=1192 row=decision/codes-only-role-logs sha=f4576bd4cbf6 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:292
quote: .join(format!("detail-{}.ndjson", process.as_str()))
at: src/main.rs:220
quote: fields.insert("panic_payload".to_owned(), payload.into());
@row_add n=1193 row=contract/log-files/modes area=log-format rec=fct-ehp4mr57@1
boundary: log files
item: modes and writes
promise: Files are 0600, dirs 0700, with one write per line.
@row_check n=1193 row=contract/log-files/modes sha=659d13b5eb8d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:48
quote: std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
at: crates/viola-state/src/fs.rs:27
quote: std::os::unix::fs::DirBuilderExt::mode(&mut builder, DIR_MODE);
at: src/obs.rs:115
quote: /// One JSON object per line, one `write` per line, into `writer` only (obs-plan §3 Logging stack).
@row_add n=1194 row=contract/log-line/timestamp area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_check n=1194 row=contract/log-line/timestamp sha=bd84fac9ff6a v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:96
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=1195 row=contract/log-line/level area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.
@row_check n=1195 row=contract/log-line/level sha=1cf22cf19437 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:144
quote: ::tracing::Level::$level,
@row_add n=1196 row=contract/log-line/target area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: target
promise: A required field.
@row_check n=1196 row=contract/log-line/target sha=ab3dd12f9fc1 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:121
quote: .json()
@row_add n=1197 row=contract/log-line/message area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: message
promise: fields.message, or the flattened message; required.
@row_check n=1197 row=contract/log-line/message sha=e6b18bf8c4d9 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:122
quote: .flatten_event(true)
at: crates/viola-core/src/obs.rs:150
quote: event.as_str()
@row_add n=1198 row=contract/log-line/event area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: event
promise: A closed kebab-case enum; each value fixes what its corr holds.
@row_check n=1198 row=contract/log-line/event sha=c28a0f13a60a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:51
quote: Self::ChannelRequest => "channel-request",
at: src/run/send.rs:196
quote: corr = cursor,
@row_add n=1199 row=contract/log-line/process area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: process
promise: One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.
@row_check n=1199 row=contract/log-line/process sha=7de524179d88 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:78-83
quote: pub enum ObsProcess {     Run,     Hook,     Mcp,     Cli, }
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name))
@row_add n=1200 row=contract/log-line/instance area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: instance
promise: A ViolaName or null.
@row_check n=1200 row=contract/log-line/instance sha=5e7fdfeec881 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:108
quote: pub instance: Option<ViolaName>,
@row_add n=1201 row=contract/log-line/corr area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
@row_check n=1201 row=contract/log-line/corr sha=b8049e072fef v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:196
quote: corr = cursor,
at: src/run/send.rs:283
quote: corr = corr,
@row_add n=1202 row=contract/log-line/null-encoding area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: null encoding
promise: A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
@row_check n=1202 row=contract/log-line/null-encoding sha=dafd7d13f668 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:147
quote: instance = $crate::obs::ctx_instance(),
at: schemas/diag-line.v1.json:15
quote: "instance": { "type": "string", "pattern": "^[a-z][a-z0-9-]{0,31}$" }
@row_add n=1203 row=contract/log-event-corr/channel-request area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: channel-request
promise: corr is the JSON-RPC request id; null for an id-less hook.event notification.
@row_check n=1203 row=contract/log-event-corr/channel-request sha=3f4198e84091 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:368
quote: corr = corr,
at: crates/viola-channel/src/server.rs:206
quote: log_request(request.id, peer, method, &params, &request.params);
@row_add n=1204 row=contract/log-event-corr/channel-response area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: channel-response
promise: corr is the JSON-RPC request id; null on a response to a frame that yielded no id (error_code -32700 or -32600).
@row_check n=1204 row=contract/log-event-corr/channel-response sha=b09e986cf081 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:476
quote: corr = self.corr,
at: crates/viola-channel/src/server.rs:265
quote: let mut logged = ResponseLine::new(Instant::now(), None, Peer::Server(srv_conn), None);
@row_add n=1205 row=contract/log-event-corr/dialog-raised area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: dialog-raised
promise: corr is the dialog_id.
@row_check n=1205 row=contract/log-event-corr/dialog-raised sha=a89700b19e03 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:315
quote: corr = dialog_id,
@row_add n=1206 row=contract/log-event-corr/dialog-answered area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: dialog-answered
promise: corr is the dialog_id.
@row_check n=1206 row=contract/log-event-corr/dialog-answered sha=4af4183304f4 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:461
quote: corr = dialog_id,
at: src/run/dialog.rs:389
quote: corr = pending.dialog_id,
@row_add n=1207 row=contract/log-event-corr/hook-invoked area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: hook-invoked
promise: corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked may still be null when it fails open before the dialog_id is known.
@row_check n=1207 row=contract/log-event-corr/hook-invoked sha=5c40493e378d v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:369
quote: corr = dialog_id,
at: src/cmd/hook.rs:285
quote: invoked(event, &invoked_at, None);
@row_add n=1208 row=contract/log-event-corr/hook-decision area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null; a hook-decision that carries detail may be null.
@row_check n=1208 row=contract/log-event-corr/hook-decision sha=53318bd4ca00 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:401
quote: corr = dialog_id,
at: src/cmd/hook.rs:596
quote: detail = detail,
@row_add n=1209 row=contract/log-event-corr/send-issued area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: send-issued
promise: corr is the send cursor.
@row_check n=1209 row=contract/log-event-corr/send-issued sha=617c529f266a v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:196
quote: corr = cursor,
@row_add n=1210 row=contract/log-event-corr/send-confirmed area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: send-confirmed
promise: corr is the send cursor.
@row_check n=1210 row=contract/log-event-corr/send-confirmed sha=c649045a6ea2 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:233
quote: corr = cursor,
@row_add n=1211 row=contract/log-event-corr/send-refused area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: send-refused
promise: corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.
@row_check n=1211 row=contract/log-event-corr/send-refused sha=396417754e64 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:277
quote: None => end_offset(self.instance_dir).ok(),
at: src/run/send.rs:283
quote: corr = corr,
@row_add n=1212 row=contract/log-line/conn-and-rpc-id area=log-format rec=fct-k3mrcwuf@1
boundary: process log line
item: conn, rpc_id
promise: Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines.
@row_check n=1212 row=contract/log-line/conn-and-rpc-id sha=1d613a7fdc80 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:197-198
quote: rpc_id = self.call.id, conn = self.call.conn,
at: src/run/send.rs:234-235
quote: rpc_id = self.call.id, conn = self.call.conn,
at: src/run/send.rs:284-285
quote: rpc_id = self.call.id, conn = self.call.conn,
@row_add n=1213 row=contract/log-event-corr/release-from-driver area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: release-from-driver
promise: corr is the JSON-RPC request id of a release the wrapper refused with -32602 for a string from; a from of another type is a plain -32602 with no such line.
@row_check n=1213 row=contract/log-event-corr/release-from-driver sha=4c8c6e822748 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:241
quote: corr = call.id,
at: src/run/wheel.rs:204
quote: if let Some(Value::String(from)) = params.get("from") {
@row_add n=1214 row=contract/log-event-corr/process-start area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: process-start
promise: corr is null.
@row_check n=1214 row=contract/log-event-corr/process-start sha=eee22d8d180a v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:149
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "version-probe");
@row_add n=1215 row=contract/log-event-corr/process-exit area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: process-exit
promise: corr is null.
@row_check n=1215 row=contract/log-event-corr/process-exit sha=29937b2fe566 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:258
quote: ObsEvent::ProcessExit,
@row_add n=1216 row=contract/log-event-corr/panic area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: panic
promise: corr is null.
@row_check n=1216 row=contract/log-event-corr/panic sha=d3172a7b6102 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:245
quote: "event": "panic",
@row_add n=1217 row=contract/log-event-corr/state-recovered area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: state-recovered
promise: corr is null.
@row_check n=1217 row=contract/log-event-corr/state-recovered sha=cc9e9d20989c v=holds judge=sonnet head=8834195b0683
at: src/main.rs:653
quote: viola_core::obs_event!(INFO, ObsEvent::StateRecovered, detail = "torn-line-healed");
@row_add n=1218 row=contract/log-event-corr/parse-rejected area=log-format rec=fct-k3mrcwuf@1
boundary: log event corr
item: parse-rejected
promise: corr is null.
@row_check n=1218 row=contract/log-event-corr/parse-rejected sha=65dbfe8a5cb8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:256
quote: ObsEvent::ParseRejected,
@row_add n=1219 row=decision/diag-line-schema-requires-corr area=log-format rec=fct-k3mrcwuf@1
name: Schema requires corr where always defined
rule: The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
@row_check n=1219 row=decision/diag-line-schema-requires-corr sha=0b3674b5d4b5 v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:91-92
quote: "if": { "properties": { "event": { "enum": ["dialog-raised", "dialog-answered", "release-from-driver"] } } },       "then": { "required": ["corr"] } },
at: schemas/diag-line.v1.json:94-95
quote: "if": { "properties": { "event": { "const": "channel-request" }, "method": { "not": { "const": "hook.event" } } } },       "then": { "required": ["corr"] } },
at: schemas/diag-line.v1.json:97-98
quote: "if": { "properties": { "event": { "const": "channel-response" }, "error_code": { "not": { "enum": [-32700, -32600] } } } },       "then": { "required": ["corr"] } },
at: schemas/diag-line.v1.json:100-101
quote: "if": { "properties": { "event": { "enum": ["send-issued", "send-confirmed", "send-refused"] }, "side": { "const": "wrapper" } }, "required": ["side"] },       "then": { "required": ["corr"] } },
at: schemas/diag-line.v1.json:103-104
quote: "if": { "properties": { "event": { "const": "hook-decision" }, "hook_event": { "enum": ["pre-tool-use", "permission-request"] } }, "required": ["hook_event"], "not": { "required": ["detail"] } },       "then": { "required": ["corr"] } }
@row_add n=1220 row=decision/hook-trace-write-failure-swallowed area=log-format rec=fct-fqmaxw56@1
name: Hook trace write failures swallowed
rule: A failed write to the hook trace file is swallowed, so the hook still exits 0 and never writes to stderr.
@row_check n=1220 row=decision/hook-trace-write-failure-swallowed sha=1babc97c9852 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:126
quote: .log_internal_errors(false)
@row_add n=1221 row=contract/log-line/hook-panic area=log-format rec=fct-gxb3q2be@1
boundary: process log line
item: hook panic
promise: A panic in a hook is caught and logged as one level ERROR, event panic line.
@row_check n=1221 row=contract/log-line/hook-panic sha=4344cce012ac v=holds judge=sonnet head=8834195b0683
at: src/main.rs:48
quote: let outcome = std::panic::catch_unwind(|| {
at: src/main.rs:242-245
quote: "level": "ERROR", "target": "viola::panic", "message": "panic", "event": "panic",
@row_add n=1222 row=decision/no-product-pid-file area=pid-file rec=fct-pskra4yu@1
name: No product pid file
rule: The product has no pid file: product pids live in each instance's snapshot with liveness in its heartbeat, and the harness's own process record is its per-session session.json.
@row_check n=1222 row=decision/no-product-pid-file sha=8e92909324c2 v=holds judge=sonnet head=8834195b0683
searched: \.pid"|pid_file|pidfile|PIDFILE|PID_FILE|"pid\.|lockfile · in **/*.rs
searched: pub pid: u32 (product pid lives in the snapshot) · in crates/viola-state/src/snapshot.rs
@row_add n=1223 row=contract/session-json/supervisor-pid area=pid-file rec=fct-pskra4yu@1
boundary: harness session.json
item: supervisor_pid
promise: The supervisor's pid.
@row_check n=1223 row=contract/session-json/supervisor-pid sha=be3b9d032db4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:214
quote: supervisor_pid: supervisor.pid,
@row_add n=1224 row=contract/session-json/wrapper-pid area=pid-file rec=fct-pskra4yu@1
boundary: harness session.json
item: wrapper_pid
promise: The wrapper's pid.
@row_check n=1224 row=contract/session-json/wrapper-pid sha=cfbcb6451be6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:271
quote: pub wrapper_pid: u32,
@row_add n=1225 row=contract/session-json/lifecycle area=pid-file rec=fct-ooubxlv3@1,fct-iwx6e4zn@1
boundary: harness session.json
item: lifecycle
promise: Written by boot once readiness passes, read by status, logs and cleanup, removed by cleanup.
@row_check n=1225 row=contract/session-json/lifecycle sha=1ebf141bf783 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:218
quote: write_json(&session_record_path(&opts.ws, &opts.session), &record)?;
at: crates/viola-e2e/src/harness/status.rs:87
quote: let record = match load_session(ws, "status", session) {
at: crates/viola-e2e/src/harness/logs.rs:47
quote: let record = load_session(ws, "logs", session)?;
at: crates/viola-e2e/src/harness/cleanup.rs:42
quote: let Ok(record) = read_json::<SessionRecord>(&session_record_path(ws, &id)) else {
at: crates/viola-e2e/src/harness/cleanup.rs:119
quote: let _ = fs::remove_file(session_record_path(ws, id));
@row_add n=1226 row=decision/pid-start-time-verified-kill area=pid-file rec=fct-ooubxlv3@1,fct-iwx6e4zn@1
name: Pid and start time verified before a kill
rule: Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling.
why: A reused pid is never killed.
@row_check n=1226 row=decision/pid-start-time-verified-kill sha=ef84b471f5f1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:250
quote: .filter(|p| p.status() != ProcessStatus::Zombie && p.start_time() == self.started_at)
at: Cargo.lock:1491
quote: version = "0.39.6"
@row_add n=1227 row=contract/status-document/list area=status-endpoint-shape rec=fct-wemcdo6l@1,fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: list
promise: The parsed viola list --json document verbatim, the exit-21 error document included; null when list exits 0 with no JSON object on stdout.
@row_check n=1227 row=contract/status-document/list sha=8693b4b0da2d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:113
quote: "list": list,
at: crates/viola-e2e/src/harness/status.rs:83
quote: .filter(Value::is_object)
@row_add n=1228 row=contract/status-document/ok area=status-endpoint-shape rec=fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: ok
promise: True only for state ready.
@row_check n=1228 row=contract/status-document/ok sha=19e876bbb143 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:108-111
quote: let ready = derived.state == "ready"; Outcome::new( json!({ "v": 1, "cmd": "status", "ok": ready, "state": derived.state,
@row_add n=1229 row=contract/status-document/pid area=status-endpoint-shape rec=fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: pid
promise: Stays null.
@row_check n=1229 row=contract/status-document/pid sha=279d73962b0d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:112
quote: "pid": null, "uptime_ms": null, "last_error": derived.last_error,
@row_add n=1230 row=contract/status-document/uptime-ms area=status-endpoint-shape rec=fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: uptime_ms
promise: Stays null.
@row_check n=1230 row=contract/status-document/uptime-ms sha=5f72a7a4106b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:112
quote: "pid": null, "uptime_ms": null, "last_error": derived.last_error,
@row_add n=1231 row=contract/status-document/instances area=status-endpoint-shape rec=fct-cz5fdkae@1,fct-kr53tti7@1
boundary: agent-run status document
item: instances
promise: One entry per booted instance: name, wrapper_pid, and alive by pid and start-time liveness.
@row_check n=1231 row=contract/status-document/instances sha=5165044b74de v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:105
quote: |i| json!({"name": i.name, "wrapper_pid": i.wrapper_pid, "alive": i.wrapper().alive()}),
@row_add n=1232 row=contract/status-state/ready area=status-endpoint-shape rec=fct-kr53tti7@1,fct-cz5fdkae@1
boundary: agent-run status state
item: ready
promise: viola list exits 0 and every booted instance is an item wrapped:true, liveness:"live"; an unwrapped item is ignored, and a session that booted none reads ready.
@row_check n=1232 row=contract/status-state/ready sha=7245ffd0e5fc v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:38
quote: item["name"] == name.as_str() && item["wrapped"] == true && item["liveness"] == "live"
at: crates/viola-e2e/src/harness/status.rs:42
quote: state: if booted.iter().all(live) {
@row_add n=1233 row=contract/status-state/degraded area=status-endpoint-shape rec=fct-kr53tti7@1,fct-cz5fdkae@1
boundary: agent-run status state
item: degraded
promise: A booted instance is stale or absent from the list, or the document holds no items.
@row_check n=1233 row=contract/status-state/degraded sha=a64a449e741e v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:35
quote: let items = doc["ok"]["items"].as_array().map_or(&[][..], Vec::as_slice);
at: crates/viola-e2e/src/harness/status.rs:44-46
quote: } else { "degraded" },
@row_add n=1234 row=contract/status-state/down area=status-endpoint-shape rec=fct-kr53tti7@1,fct-cz5fdkae@1
boundary: agent-run status state
item: down
promise: viola list exits non-zero, gives no exit code, cannot be spawned, has no supervise record to run from, or exits 0 with no JSON object on stdout.
@row_check n=1234 row=contract/status-state/down sha=cd332655b8ef v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:31-33
quote: (None, _) => return down("none"), (Some(0), Some(doc)) => doc, (Some(c), _) => return down(&c.to_string()),
at: crates/viola-e2e/src/harness/status.rs:97
quote: Err(_) => (None, None),
@row_add n=1235 row=decision/rstest-fixture-chain area=test-data-bootstrap rec=fct-p6be5x2e@1
name: rstest fixture chain
rule: Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
@row_check n=1235 row=decision/rstest-fixture-chain sha=59eb76cda091 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:354
quote: pub fn home() -> TestHome {
at: tests/support/home.rs:359
quote: pub fn fake_agent_path() -> PathBuf {
at: tests/support/home.rs:386
quote: pub fn stamped_home(home: TestHome, fake_agent_path: PathBuf) -> StampedHome {
at: tests/support/home.rs:1063
quote: pub fn booted_wrapper(stamped_home: StampedHome) -> Wrapper {
@row_add n=1236 row=decision/fake-agent-replay-for-stamps area=test-data-bootstrap rec=fct-p6be5x2e@1
name: Fake-agent replay for stamps
rule: Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
@row_check n=1236 row=decision/fake-agent-replay-for-stamps sha=591a2a78264c v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:387-390
quote: let ran = super::verify::verify( home.path(), &workspace_path("fixtures/claude"), fake::RECORDED_CLI_VERSION,
at: tests/support/verify.rs:527
quote: let mut args: Vec<OsString> = vec!["--home".into(), home.into(), "verify".into()];
@row_add n=1237 row=decision/proptest-committed-seeds area=test-data-bootstrap rec=fct-p6be5x2e@1
name: proptest with committed seeds
rule: Randomized inputs come from proptest 1.11.0 strategies with committed regression seeds.
@row_check n=1237 row=decision/proptest-committed-seeds sha=19cb91fab131 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:220
quote: proptest ={ version = "=1.11.0", default-features = false, features = ["std"] }
at: crates/viola-agent-claude/proptest-regressions/hook.txt:1
quote: # Seeds for failure cases proptest has generated in the past. It is
@row_add n=1238 row=decision/chaos-home-outside-scan area=test-data-bootstrap rec=fct-p6be5x2e@1,fct-x3qmgvn2@2
name: One chaos home outside the scans
rule: Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design.
why: A ratified carve-out.
@row_check n=1238 row=decision/chaos-home-outside-scan sha=9385c786854f v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:214-216
quote: pub fn outside_scan() -> Self { let dir = tempfile::Builder::new() .prefix("viola-chaos-")
at: tests/chaos_feed_panic.rs:47
quote: StampedHome::unstamped(TestHome::outside_scan()),
@row_add n=1239 row=decision/e2e-home-base-checks area=test-data-bootstrap rec=fct-p6be5x2e@1
name: Checked base of the test homes
rule: The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
@row_check n=1239 row=decision/e2e-home-base-checks sha=ad58b3c6c447 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:120-125
quote: if fs::symlink_metadata(base).is_ok_and(|m| m.file_type().is_symlink()) { let target = fs::read_link(base).expect("e2e-home link"); assert!( target.is_absolute(), "e2e-home backing: the link's target is relative"
at: tests/support/home.rs:135
quote: meta.permissions().mode() & 0o077 == 0,
@row_add n=1240 row=decision/test-home-owner-record-sweep area=test-data-bootstrap rec=fct-x3qmgvn2@2
name: Owner-record sweep of test homes
rule: Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record.
why: A test the runner kills never drops its home.
@row_check n=1240 row=decision/test-home-owner-record-sweep sha=ba7c250e4fd6 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:107
quote: let record = json!({"pid": pid, "started_at": started_at});
at: tests/support/home.rs:195-196
quote: if !flag("AGENT_RUN_KEEP_HOMES") && !flag("AGENT_RUN_KEEP_FAILED") { sweep_gone_owners(&base);
at: tests/support/home.rs:72
quote: if process_start(pid) == Some(started_at) {
@row_add n=1241 row=decision/owner-record-removed-last area=test-data-bootstrap rec=fct-x3qmgvn2@2
name: Owner record removed last
rule: Removing a test home deletes its owner record last.
why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
@row_check n=1241 row=decision/owner-record-removed-last sha=5afca081c129 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:89-90
quote: if entry.file_name() == OWNER { continue;
at: tests/support/home.rs:100
quote: whole && fs::remove_file(dir.join(OWNER)).is_ok() && fs::remove_dir(dir).is_ok()
@row_add n=1242 row=decision/keep-failed-home-opt-in area=test-data-bootstrap rec=fct-x3qmgvn2@2
name: Failed test homes kept only on request
rule: A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1.
why: So that logs can inspect the home.
@row_check n=1242 row=decision/keep-failed-home-opt-in sha=da90eb2480cc v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:29
quote: keep_homes || (keep_failed && panicking)
@row_add n=1243 row=decision/real-claude-local-only area=1-test-scope-summary rec=fct-wcpph3y4@2,fct-upolcsa3@1,fct-tfd2itkl@1,fct-dpjzmvrb@1,fct-6ugas3oa@1,fct-7bwho3ei@1
name: Real claude CLI tested local-only
rule: Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
@row_check n=1243 row=decision/real-claude-local-only sha=532ff0d58811 v=holds judge=sonnet head=8834195b0683
searched: \bclaude\b · in .github/workflows/ci.yml
searched: claude · in .github/workflows/nightly.yml
@row_add n=1244 row=contract/viola-hook/stdout area=1-test-scope-summary rec=fct-paewrdc5@2,fct-hyr7b53t@2,fct-627s4o4i@1
boundary: viola hook exec-form entry points
item: stdout
promise: stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body; viola hook statusline prints the user's status-line command output unchanged.
@row_check n=1244 row=contract/viola-hook/stdout sha=5e955997bae9 v=stale judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
shows: viola hook statusline prints the user's own status-line command stdout unchanged, which is neither empty nor a decision body.
cell: promise
value: stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body; viola hook statusline prints the user's status-line command output unchanged.
@row_add n=1245 row=decision/sync-crates-without-tokio area=1-test-scope-summary rec=fct-elcdxxgr@2,fct-hlx2zkbt@2,fct-7h3hdkfi@1,fct-ys3szg34@1,fct-u7ux54l5@1
name: Sync crates build without tokio
rule: Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build.
guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban
@row_check n=1245 row=decision/sync-crates-without-tokio sha=4f00afff7d14 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:275
quote: cargo check $(sed 's/^/-p /' scripts/sync-crates.txt)
at: .github/workflows/ci.yml:382
quote: cargo deny --config deny-sync.toml --manifest-path "crates/$crate/Cargo.toml" check bans
guard_at: .github/workflows/ci.yml:275
@row_add n=1246 row=decision/release-build-viola-only area=1-test-scope-summary rec=fct-elcdxxgr@2,fct-see5mjkn@1,fct-thnlptgc@1,fct-2qyicniz@2
name: Release build carries viola only
rule: The release build carries viola alone and fails on any test-only feature or test-only binary in it, with no upload and no gate step.
guard: release-check.sh and its probe
@row_check n=1246 row=decision/release-build-viola-only sha=73564689921f v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/release-check.sh:88
quote: json=$(cargo build --release --locked --bin viola --message-format=json) || rc=$?
at: scripts/release-check.sh:32
quote: echo "release-check: FAILED — test-only binary $name"
guard_at: scripts/release-check.sh:60
@row_add n=1247 row=decision/sha-pinned-actions area=1-test-scope-summary rec=fct-elcdxxgr@2,fct-r5xptmoc@1,fct-qgvb7uk6@1
name: SHA-pinned Actions
rule: Every Action a workflow uses is SHA-pinned, never referenced by a mutable tag.
guard: zizmor unpinned-uses
@row_check n=1247 row=decision/sha-pinned-actions sha=e4f3ebd98d03 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:22
quote: - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
at: .github/workflows/ci.yml:80
quote: uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
guard_at: .github/workflows/ci.yml:388
@row_add n=1248 row=contract/viola-list-json/budget area=1-test-scope-summary rec=fct-hlmuruc2@1
boundary: viola list --json
item: budget
promise: The ok payload's budget is {five_hour:{used_percentage,resets_at}, seven_day:{...}, read_at, paused} or "unknown".
@row_check n=1248 row=contract/viola-list-json/budget sha=d898cb930561 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:170-177
quote: Some(budget) => json!({ "five_hour": budget.stamped.reading.five_hour, "seven_day": budget.stamped.reading.seven_day, "read_at": budget.stamped.read_at, "paused": budget.paused, }), None => json!("unknown"),
@row_add n=1249 row=contract/viola-list-json/skipped area=1-test-scope-summary rec=fct-hlmuruc2@1
boundary: viola list --json
item: skipped
promise: The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
@row_check n=1249 row=contract/viola-list-json/skipped sha=7dc7382ccdf8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:211-213
quote: pub unknown_kinds: u64, pub unknown_fields: u64, pub torn_lines: u64,
@row_add n=1250 row=decision/logs-carry-no-secrets area=1-test-scope-summary rec=fct-yp3slbux@1,fct-gjahavf7@1
name: Logs carry no secrets
rule: No log contains the Cookie header or a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line.
guard: viola-harness secret-scan
@row_check n=1250 row=decision/logs-carry-no-secrets sha=42e9dba4d05e v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:129
quote: for offset in find_all(&lower, b"cookie:") {
at: crates/viola-e2e/src/harness/secret_scan.rs:17
quote: ("claude-stripped", "canary-token-value-7f3a"),
at: crates/viola-e2e/src/harness/secret_scan.rs:33
quote: pub const CONTENT: [(&str, &str); 1] = [("content-canary", "canary-chain-value-5c1e")];
guard_at: crates/viola-e2e/src/bin/viola-harness.rs:122
@row_add n=1251 row=contract/instance-snapshot/pid area=1-test-scope-summary rec=fct-w7y4wg56@1,fct-4khqx2uf@2
boundary: instance snapshot.json
item: pid
promise: The wrapper pid lives in the instance's snapshot.json, with started_at and child_pid; there is no standalone pid file.
@row_check n=1251 row=contract/instance-snapshot/pid sha=33d8e878f6d5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:42-43
quote: pub pid: u32, pub started_at: String,
at: crates/viola-state/src/snapshot.rs:56
quote: pub child_pid: Option<u32>,
searched: pidfile · in src/ and crates/*/src (no match)
searched: .pid" · in src/ and crates/*/src (no match)
@row_add n=1252 row=decision/self-bootstrapping-test-data area=1-test-scope-summary rec=fct-yeykk4zl@1,fct-s424obzs@1
name: Self-bootstrapping test data
rule: Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, budget.json is seeded through viola hook statusline, stamps come from viola verify against the fake agent, and no state is developer-seeded.
@row_check n=1252 row=decision/self-bootstrapping-test-data sha=66289998a2d3 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:387
quote: let ran = super::verify::verify(
at: src/cmd/hook/statusline.rs:127
quote: reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
@row_add n=1253 row=decision/human-never-blocked area=1-test-scope-summary rec=fct-nzaflfcl@1,fct-imixgk6n@1,fct-3oyxjfyz@1
name: The human is never blocked
rule: The human is never blocked: human typing still reaches the agent while automation is refused.
@row_check n=1253 row=decision/human-never-blocked sha=cffe0d330d99 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
at: src/run/send.rs:96
quote: if let Some(detail) = slot.wheel.human_typing() {
@row_add n=1254 row=contract/viola-verify/stamped-line area=1-test-scope-summary rec=fct-yl2sew2w@1,fct-v74hzzdp@1
boundary: viola verify
item: stamped line
promise: viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
@row_check n=1254 row=contract/viola-verify/stamped-line sha=07a116867ff3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:293-296
quote: human::result(&format!( "stamped {version}  {} pass  {failed} fail", results.len() - failed ));
at: src/cmd/verify.rs:302
quote: None => Ok(Ended::Exit(0)),
@row_add n=1255 row=contract/mcp-stdin/oversize-line area=1-test-scope-summary rec=fct-bia7axqu@1,fct-fbkvffnw@2,fct-elurmfjv@1
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: A stdin line past MAX_FRAME is refused whole, passes no byte, and ends the session: earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, and one parse-rejected mcp-stdin oversize WARN line holding no byte of it.
@row_check n=1255 row=contract/mcp-stdin/oversize-line sha=c90a2e78412a v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/lib.rs:98-100
quote: parser = "mcp-stdin", detail = "oversize", count = 1u64,
at: src/cmd/mcp.rs:37-38
quote: run::log_self_exit(0, None); Ok(ExitCode::SUCCESS)
@row_add n=1256 row=decision/leading-slash-never-a-prompt area=1-test-scope-summary rec=fct-engo5ir7@2,fct-lzdm6e7w@6
name: Leading-slash argument never a prompt
rule: A leading-slash argument in the target or text position of send or answer is never taken as a prompt: it is a usage error, exit 2, and nothing is delivered.
@row_check n=1256 row=decision/leading-slash-never-a-prompt sha=3903aad36ea3 v=holds guard=false judge=sonnet head=8834195b0683
at: src/cmd/send.rs:44
quote: ViolaName::try_new(raw.to_owned()).map_err(|_| "invalid instance name".to_owned())
at: src/cmd/send.rs:47-48
quote: fn refuse_text_argument(_raw: &str) -> Result<String, String> { Err("the text is read from stdin or --file, never from an argument".to_owned())
@row_add n=1257 row=decision/tailing-ignores-symlinks area=1-test-scope-summary rec=fct-55p24js3@1,fct-bt3npe7x@2
name: Tailing ignores symlinks and invalid names
rule: Tailing ignores symlinks and invalid names, and an over-long ndjson line is counted in skipped.torn_lines.
@row_check n=1257 row=decision/tailing-ignores-symlinks sha=38eea6d69aba v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:77-78
quote: if kind.is_symlink() { scan.links_ignored += 1;
at: crates/viola-state/src/board.rs:91
quote: _ => scan.malformed += 1,
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
@row_add n=1258 row=decision/vt100-panic-contained area=1-test-scope-summary rec=fct-vzvcqlaz@1,fct-47fi4ugg@2
name: vt100 panic contained
rule: A forced vt100 panic refuses send with not-delivered/input-not-ready while passthrough continues on the outer terminal: the child's own bytes reach it and no viola literal appears on it.
@row_check n=1258 row=decision/vt100-panic-contained sha=fdf69ea6a6d8 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() {
at: src/run/send.rs:129-130
quote: if readiness != Readiness::Ready { return ctx.not_delivered(None, NotDelivered::InputNotReady);
at: tests/chaos_feed_panic.rs:123
quote: for literal in ["panic", "parse-rejected", "vt100", "unable", "hint:"] {
@row_add n=1259 row=decision/linux-only area=1-test-scope-summary rec=fct-5akis2u7@3,fct-ze6ljnvw@1
name: Linux only
rule: The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it.
guard: contract_os_gated_tests
@row_check n=1259 row=decision/linux-only sha=805326b2dc23 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: tests/contract_os_gated_tests.rs:425
quote: names_a_word(predicate, &[WINDOWS]) || spaced.contains("target_os=\"macos\"")
at: tests/contract_os_gated_tests.rs:533
quote: .map(|(file, test, gate)| format!("another OS: {file} {test} cfg({gate})")),
guard_at: tests/contract_os_gated_tests.rs:533
@row_add n=1260 row=contract/cli-exit-codes/14 area=1-test-scope-summary rec=fct-bpxqtijx@1,fct-fn3ybwy5@2,fct-id225cf7@2
boundary: CLI exit codes
item: 14
promise: A refusal whose code word is outside the known list falls back to RefusalReason unknown and exits 14 with a --json refusal document; it never panics and never maps to 20.
@row_check n=1260 row=contract/cli-exit-codes/14 sha=6e0d804313a8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:165-166
quote: #[serde(other)] Unknown,
at: src/cmd/send.rs:75-77
quote: pub(super) fn reason_of(reason: &str) -> RefusalReason { serde_json::from_value(Value::from(reason)).unwrap_or(RefusalReason::Unknown) }
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
@row_add n=1261 row=decision/mutation-at-epoch-boundary area=1-test-scope-summary rec=fct-4ngbveqo@1,fct-cv73iuuc@1,fct-cz5jn5fs@2
name: Mutation testing at the epoch boundary
rule: cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
@row_check n=1261 row=decision/mutation-at-epoch-boundary sha=22c13201565b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:61-63
quote: unit: named.unit || all || none, integration: named.integration || all || none, e2e: named.e2e || all || none,
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
@row_add n=1262 row=decision/rstest-case-tables area=2-test-strategy rec=fct-njccdctv@2,fct-74ah5yxk@1,fct-gbcioyfo@1,fct-7osf2h3d@2
name: Table-driven unit cases in rstest
rule: Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label.
except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
why: so no manifest edge is added for rstest alone
@row_check n=1262 row=decision/rstest-case-tables sha=55cff4fff022 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:168
quote: #[case::fresh(Some(4_900), true, Liveness::Live)]
at: tests/run_cli.rs:608
quote: #[case::instance_dir("instances/builder", INSTANCE_PAIR)]
at: tests/cross_exit_causes.rs:496
quote: #[case::run_instance(false, true, INSTANCE_STATE)]
@row_add n=1263 row=decision/unit-tests-inline area=2-test-strategy rec=fct-njccdctv@2,fct-ks7lvtf7@1,fct-7osf2h3d@2,fct-hckuzazj@4
name: Inline unit tests
rule: Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.
@row_check n=1263 row=decision/unit-tests-inline sha=357f7a36ceb7 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:278
quote: #[cfg(test)]
at: crates/viola-core/src/lib.rs:396
quote: #[cfg(test)]
at: crates/viola-state/src/lib.rs:93
quote: #[cfg(test)]
at: crates/viola-channel/src/lib.rs:154
quote: #[cfg(test)]
at: crates/viola-mcp/src/lib.rs:104
quote: #[cfg(test)]
at: crates/viola-pty/src/lib.rs:267
quote: #[cfg(test)]
at: crates/viola-agent-claude/src/lib.rs:209
quote: #[cfg(test)]
@row_add n=1264 row=decision/integration-suite-per-contract area=2-test-strategy rec=fct-4ignlk4w@2,fct-hlf5klew@2
name: Integration suite for every standard contract
rule: Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
@row_check n=1264 row=decision/integration-suite-per-contract sha=87816571a2a0 v=holds judge=sonnet head=8834195b0683
at: tests/channel_endpoint.rs:12
quote: use interprocess::local_socket::traits::Stream as _;
at: tests/support/home.rs:20
quote: pub const VIOLA: &str = env!("CARGO_BIN_EXE_viola");
at: crates/viola-mcp/tests/handlers.rs:1
quote: //! The tool handlers against a test wrapper, with no stdio: this test process stands in for the
@row_add n=1265 row=decision/hook-deadlines-gated-by-hyperfine area=2-test-strategy rec=fct-egpbbwom@1,fct-zx23mmpw@2
name: Hook deadlines gated by hyperfine
rule: hyperfine gates the hook deadlines on max.
@row_check n=1265 row=decision/hook-deadlines-gated-by-hyperfine sha=2a98936baf83 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:149
quote: .and_then(|v| v["results"][0]["max"].as_f64());
@row_add n=1266 row=decision/fixtures-schema-checked area=2-test-strategy rec=fct-xagti75h@1,fct-vekwchud@2
name: Recorded fixtures checked against their schema
rule: jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
@row_check n=1266 row=decision/fixtures-schema-checked sha=be88fa116864 v=holds judge=sonnet head=8834195b0683
at: tests/contract_fixture_hygiene.rs:394
quote: check(bytes, &mcp_schema(), user).map_err(McpViolation::Hygiene)?;
at: tests/contract_fixture_hygiene.rs:523
quote: let validator = jsonschema::validator_for(&screen_schema()).expect("valid schema");
at: tests/cli_verify.rs:709
quote: assert_eq!(check(&agents, &agents_schema, Some(&user)), Ok(()));
@row_add n=1267 row=decision/complete-lines-reader area=2-test-strategy rec=fct-vvnlh7sf@1,fct-hckuzazj@4
name: Live-appended files read by complete lines
rule: Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader.
why: a Linux reader can observe an append part-way
@row_check n=1267 row=decision/complete-lines-reader sha=85ddf68f8b5f v=holds judge=sonnet head=8834195b0683
at: tests/support/ndjson.rs:14
quote: let Some(end) = bytes.iter().rposition(|b| *b == b'\n') else {
@row_add n=1268 row=decision/fixture-named-by-hook-event area=2-test-strategy rec=fct-cyzxt2op@1
name: Fixtures named by hook event
rule: Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
@row_check n=1268 row=decision/fixture-named-by-hook-event sha=79151605217c v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:505
quote: self.set_file(&format!("{event}.{variant}.json"))
@row_add n=1269 row=decision/test-function-naming area=2-test-strategy rec=fct-ux4bzdth@1,fct-jzgpms4i@1
name: Test function naming
rule: Test functions are named <subject>_<condition>_<expected> in snake_case.
except: critical-path scenarios are named path<N>_<slug>
@row_check n=1269 row=decision/test-function-naming sha=55861cba4822 v=holds judge=sonnet head=8834195b0683
at: tests/cli_send.rs:186
quote: fn path2_send_confirms_with_cl1_events() {
at: crates/viola-e2e/src/harness/run/perf.rs:331
quote: fn perf_without_hyperfine_is_tool_missing() {
@row_add n=1270 row=decision/nextest-test-runner area=4-unit-test-strategy rec=fct-uptme7ma@1,fct-2j6rht5l@5
name: cargo-nextest runs the tests
rule: Tests run on libtest through cargo-nextest, process per test, with JUnit output.
except: Doctests run through cargo test, because nextest cannot run them.
@row_check n=1270 row=decision/nextest-test-runner sha=5bf138ee09d0 v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:6-7
quote: [profile.ci.junit] path = "junit.xml"
@row_add n=1271 row=absence/database area=5-integration-test-strategy rec=fct-rhuz6eg7@1,fct-hctsd5xl@2
capability: A database
@row_check n=1271 row=absence/database sha=3172c29a0c8e v=holds judge=sonnet head=8834195b0683
searched: sqlite|rusqlite|sled|redb|diesel|sqlx|rocksdb · in **/Cargo.toml
@row_add n=1272 row=absence/outbound-http area=5-integration-test-strategy rec=fct-jnilykpz@2,fct-yqmk3k5s@1,fct-qta73xkp@1,fct-b4mgg72y@2,fct-iyneo4vq@1,fct-pwscsozw@2
capability: Outbound HTTP
@row_check n=1272 row=absence/outbound-http sha=6e093dd0a5a3 v=holds judge=sonnet head=8834195b0683
searched: reqwest|hyper|ureq|isahc|surf|http · in **/Cargo.toml (only comments)
searched: reqwest|hyper::|ureq::|TcpStream|https?:// · in src/**/*.rs and crates/*/src/**/*.rs (only a secret-scan test string)
@row_add n=1273 row=decision/fake-agent-as-claude area=5-integration-test-strategy rec=fct-jnilykpz@2,fct-b4mgg72y@2
name: Fake agent serves the claude boundary
rule: The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH.
@row_check n=1273 row=decision/fake-agent-as-claude sha=f800e10a9992 v=holds judge=sonnet head=8834195b0683
at: tests/support/fake.rs:40
quote: if !claude.is_file() && fs::hard_link(FAKE, &claude).is_err() {
@row_add n=1274 row=decision/fresh-home-per-test area=5-integration-test-strategy rec=fct-wwnk7u4j@2,fct-qmrx4mno@2,fct-p6be5x2e@1,fct-ea2dd5jv@1
name: Fresh home per test
rule: Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests.
why: so obs-plan's CI gates, the secret scan and the upload cover integration runs
except: The chaos feed panic test alone boots outside the scans.
@row_check n=1274 row=decision/fresh-home-per-test sha=70fa711341df v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:193
quote: let base = workspace_path("target/e2e-home");
at: tests/support/home.rs:200
quote: .tempdir_in(base)
at: tests/support/home.rs:203
quote: let home = dir.path().join("home");
at: tests/support/cli.rs:63-64
quote: let _ = child.kill(); let _ = child.wait();
at: tests/chaos_feed_panic.rs:47
quote: StampedHome::unstamped(TestHome::outside_scan()),
@row_add n=1275 row=contract/wrapper-channel/cursor area=5-integration-test-strategy rec=fct-r2yuicrg@1,fct-ksdwqwkf@2
boundary: the wrapper channel
item: cursor
promise: A result's cursor equals the events.ndjson length at acceptance.
@row_check n=1275 row=contract/wrapper-channel/cursor sha=ff3c225fc0ec v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:179
quote: let cursor = append_event_at(self.instance_dir, |cursor| {
at: src/run/send.rs:215
quote: Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
@row_add n=1276 row=contract/wrapper-channel/-32602 area=5-integration-test-strategy rec=fct-r2yuicrg@1,fct-5qkdel63@2
boundary: the wrapper channel
item: -32602
promise: A request with a higher v is too new and is answered -32602 with data {supported, wrapper}.
@row_check n=1276 row=contract/wrapper-channel/-32602 sha=eab0a8f18059 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:238-239
quote: if params.v > PROTOCOL_V { return Err(ProtocolError::UnsupportedVersion);
at: crates/viola-channel/src/lib.rs:95
quote: Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
@row_add n=1277 row=contract/wrapper-channel/control-character area=5-integration-test-strategy rec=fct-r2yuicrg@1,fct-ksdwqwkf@2
boundary: the wrapper channel
item: control-character
promise: In wrapper-side paste validation, send or answer text holding ESC, DEL or U+0085 is refused {"refusal":"not-delivered","detail":"control-character"}, and control-character wins over every other refusal.
@row_check n=1277 row=contract/wrapper-channel/control-character sha=e567bac58639 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:88
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/run/dialog.rs:425
quote: .find_map(|t| viola_core::validate_paste_text(t).err())
at: crates/viola-core/src/lib.rs:367
quote: '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' => true,
@row_add n=1278 row=contract/event-line/ts area=5-integration-test-strategy rec=fct-zpi22xot@1,fct-bt3npe7x@2
boundary: the event line
item: ts
promise: ts is RFC 3339 UTC with ms and Z.
@row_check n=1278 row=contract/event-line/ts sha=5d4a511bddf4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/lib.rs:31
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=1279 row=contract/strict-modes-refusal/hook area=5-integration-test-strategy rec=fct-zpi22xot@1,fct-bt3npe7x@2
boundary: strict-modes refusal
item: hook
promise: A strict-modes refusal in the hook exits 0 with no output, reading no config.json or stdin, opening no role file and writing no frame, no line and no SessionEnd append.
@row_check n=1279 row=contract/strict-modes-refusal/hook sha=12dce15852bd v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
at: src/cmd/hook/statusline.rs:42-43
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=1280 row=contract/viola-hook/no-viola-name area=5-integration-test-strategy rec=fct-627s4o4i@1,fct-nzkgjegi@1,fct-srb3tv7d@1
boundary: viola hook
item: no VIOLA_NAME
promise: With no VIOLA_NAME every hook exits 0 immediately with empty stdout and stderr, and no instances directory is created.
@row_check n=1280 row=contract/viola-hook/no-viola-name sha=a457e74bef55 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:256
quote: let name = ViolaName::try_new(name?.into_string().ok()?).ok()?;
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=1281 row=decision/controls-not-disableable area=5-integration-test-strategy rec=fct-w5xcngvx@3,fct-lzdm6e7w@6
name: Controls not disableable
rule: No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them.
guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
@row_check n=1281 row=decision/controls-not-disableable sha=14cd66c01d74 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/cli_controls_not_disableable.rs:424
quote: const SETTINGS: [(&str, &str); 10] = [
at: tests/cli_controls_not_disableable.rs:648
quote: fn setting_does_not_disable_the_socket_dir_check() {
searched: VIOLA_ variable literals read in the product (only VIOLA_NAME, VIOLA_DIR, VIOLA_BIN found, none a disabling switch) · in src/**/*.rs
guard_at: tests/cli_controls_not_disableable.rs:648
@row_add n=1282 row=decision/tty-asserted-only-on-verb-sgr area=6-e2e-test-strategy rec=fct-gnzxb6ex@2
name: TTY rendering asserted only on verb SGR
rule: CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
@row_check n=1282 row=decision/tty-asserted-only-on-verb-sgr sha=a822c8c21deb v=holds judge=sonnet head=8834195b0683
at: tests/cli_output_plain.rs:16
quote: use support::sgr::{sets_a_foreground, sgr_attributes};
at: tests/cli_wait_last.rs:381
quote: let mut pty = OuterPty::spawn(Path::new(VIOLA), &args, &[]);
@row_add n=1283 row=decision/hook-deadline-driver area=6-e2e-test-strategy rec=fct-iqthlzz5@2
name: viola hook driven under an Instant deadline
rule: viola hook and hook statusline are driven by std::process::Command with an Instant deadline over recorded fixture stdin.
@row_check n=1283 row=decision/hook-deadline-driver sha=e73388dc6cdf v=holds judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:39
quote: let mut cmd = Command::new(VIOLA);
at: tests/hook_fail_open.rs:49
quote: let started = Instant::now();
@row_add n=1284 row=decision/env-clear-keeps-llvm-profile-file area=6-e2e-test-strategy rec=fct-iqthlzz5@2,fct-qacavs3x@1
name: env_clear tests re-add LLVM_PROFILE_FILE
rule: Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
@row_check n=1284 row=decision/env-clear-keeps-llvm-profile-file sha=809dd7b8a5dc v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=1285 row=decision/tui-passthrough-never-parses-screen area=6-e2e-test-strategy rec=fct-ktlsbc3h@2
name: Passthrough verdicts never parse the rendered screen
rule: viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen.
except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
@row_check n=1285 row=decision/tui-passthrough-never-parses-screen sha=eec72b52a882 v=holds judge=sonnet head=8834195b0683
at: tests/tui_passthrough.rs:93
quote: assert_eq!(exit["exit_source"], "handle-wait");
at: tests/chaos_feed_panic.rs:116
quote: while !wrapper.shown().windows(3).any(|w| w == [0xe4, 0xb8, 0xad]) {
@row_add n=1286 row=decision/endpoint-read-from-snapshot area=6-e2e-test-strategy rec=fct-uz5vlm6m@1
name: Endpoint read only from the snapshot
rule: Wrapper channel tests drive the sync client, a raw interprocess stream and jsonschema, and read the endpoint only from snapshot.json.
@row_check n=1286 row=decision/endpoint-read-from-snapshot sha=e42a41ed7865 v=holds judge=sonnet head=8834195b0683
at: tests/channel_endpoint.rs:192
quote: let endpoint = snapshot["endpoint"].as_str().expect("endpoint").to_owned();
@row_add n=1287 row=decision/mcp-stdio-test-owns-child area=6-e2e-test-strategy rec=fct-xwumruas@1
name: MCP stdio test owns the child
rule: MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
@row_check n=1287 row=decision/mcp-stdio-test-owns-child sha=6cfc93b0d8d2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/tests/mcp_tools.rs:101
quote: let client = ().serve((stdout, stdin)).await.expect("initialized");
at: crates/viola-e2e/tests/mcp_recorded_exchange.rs:180
quote: fn mcp_recorded_exchange_replays_the_client_frames_and_keeps_the_accepted_result_shape() {
@row_add n=1288 row=decision/cross-user-peers-untestable area=6-e2e-test-strategy rec=fct-lkgqv65p@3,fct-7bwho3ei@1
name: Cross-OS-user peers skipped as untestable
rule: Cross-OS-user peers are skipped as untestable, covered by the peer-credential boundary unit test and same-user socket modes read-back; the euid wiring at the three client seams is held by no committed test.
@row_check n=1288 row=decision/cross-user-peers-untestable sha=7202d02ea10e v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:534
quote: assert!(!peer_admitted(Some(own.wrapping_add(1))));
@row_add n=1289 row=contract/viola-send/empty-text area=6-e2e-test-strategy rec=fct-waypbx6k@1,fct-qbytqn26@3
boundary: viola send
item: empty-text
promise: A text whose typed text is empty once its trailing newlines are removed is refused at once, exit 13 not-delivered/empty-text, and never issued.
@row_check n=1289 row=contract/viola-send/empty-text sha=3710bd305a63 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:91-95
quote: let typed = typed_text(text); let text: &str = &typed; if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText); }
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=1290 row=contract/viola-answer/unknown-dialog area=6-e2e-test-strategy rec=fct-zjfsmr2z@1,fct-2zxurlqa@1
boundary: viola answer
item: 13 unknown-dialog
promise: An answer to an unknown dialog id exits 13 {"refusal":"not-delivered","detail":"unknown-dialog"}.
@row_check n=1290 row=contract/viola-answer/unknown-dialog sha=65ab6d88ecf2 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:446
quote: Some(NotDelivered::UnknownDialog.as_str()),
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=1291 row=contract/viola-answer/unverified-cli area=6-e2e-test-strategy rec=fct-v74hzzdp@1,fct-p2i2onck@2
boundary: viola answer
item: 12 unverified-cli
promise: An answer on an unverified or unstamped CLI is refused exit 12 unverified-cli with no decision body, the dialog left to the human; MCP answer refuses alike.
@row_check n=1291 row=contract/viola-answer/unverified-cli sha=8f1efe90e34c v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
at: src/run/dialog.rs:251-252
quote: let hold = self.cli_verified && state.pending.is_none()
@row_add n=1292 row=decision/snapshot-replay-from-log area=6-e2e-test-strategy rec=fct-uq5phmo2@1,fct-am65hify@1,fct-47fi4ugg@2,fct-3svwp7h4@1
name: Unreadable snapshot replayed from the log
rule: Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the budget pause, override end and the agent session id included, with no dialog pending, and write no file.
guard: a_start_over_an_unreadable_snapshot_replays_the_log, e5_the_replay_reads_what_the_wrapper_wrote
@row_check n=1292 row=decision/snapshot-replay-from-log sha=183453b847ed v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:441
quote: // A replayed snapshot records no pid: the beat alone says whether a wrapper still runs.
guard_at: tests/chaos_snapshot_replay.rs:88
@row_add n=1293 row=decision/start-collision-by-heartbeat area=6-e2e-test-strategy rec=fct-uq5phmo2@1,fct-47fi4ugg@2,fct-h6d4aowm@1
name: Start collision read from the heartbeat
rule: Over a replayed snapshot the start collision check reads liveness from the heartbeat alone: a fresh beat refuses the start already-live (exit 1), a beat older than STALE_AFTER does not.
guard: a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
@row_check n=1293 row=decision/start-collision-by-heartbeat sha=581063b64841 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
guard_at: tests/chaos_snapshot_replay.rs:113
@row_add n=1294 row=decision/hook-fails-open area=6-e2e-test-strategy rec=fct-4aawmv2h@2,fct-fbkvffnw@2
name: Hook fails open silently
rule: The hook exits 0 with empty streams and no frame on oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic, a missing VIOLA_NAME, no instance directory, a refused instance or socket directory, or a server that is not the recorded wrapper.
guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
@row_check n=1294 row=decision/hook-fails-open sha=7001d5086df3 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/hook_fail_open.rs:273-274
quote: #[case::oversize_stdin(&["hook", "stop"], Env::Made, Stdin::Oversize, Some("oversize-stdin"))] #[case::malformed_json(&["hook", "stop"], Env::Made, Stdin::Malformed, Some("malformed-json"))]
at: tests/hook_fail_open.rs:123
quote: vars.push(("FAKE_AGENT_HOOK_PANIC", OsString::from("1")));
at: tests/hook_fail_open.rs:285
quote: #[case::impostor(&["hook", "stop"], Env::Impostor, Stdin::Payload, Some("server-verify-failed"))]
guard_at: tests/hook_fail_open.rs:286
@row_add n=1295 row=contract/json-rpc-errors/-32603 area=6-e2e-test-strategy rec=fct-gjahavf7@1,fct-o57nkgzg@2
boundary: JSON-RPC errors
item: -32603
promise: Every -32603 carries exactly the message "internal error" and data null.
@row_check n=1295 row=contract/json-rpc-errors/-32603 sha=abff2766d2b2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
at: crates/viola-channel/src/lib.rs:127
quote: -32603 => (ProtocolError::Internal.message(), Value::Null),
at: crates/viola-mcp/src/tools.rs:673
quote: json!({"code": -32603, "message": "internal error", "data": null})
@row_add n=1296 row=decision/umask-independent-modes area=6-e2e-test-strategy rec=fct-4uc5ehox@2,fct-hhwc4gim@2
name: Umask-independent modes
rule: Under any umask, home files are 0600, directories 0700 and the pinned viola binary 0700, never group- or world-writable, and a clean start keeps the socket private.
guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
@row_check n=1296 row=decision/umask-independent-modes sha=897ad5d40bec v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:10
quote: pub const DIR_MODE: u32 = 0o700;
at: crates/viola-state/src/fs.rs:11
quote: pub const FILE_MODE: u32 = 0o600;
at: crates/viola-state/src/pin.rs:105
quote: match place_private_new(path, bytes, DIR_MODE)? {
at: crates/viola-channel/src/server.rs:134
quote: fs::set_permissions(&guard.socket, Permissions::from_mode(0o600))
guard_at: tests/security_negatives_socket_dir.rs:408
@row_add n=1297 row=decision/single-writer-state-files area=6-e2e-test-strategy rec=fct-4uc5ehox@2,fct-mqdaf5bf@2
name: Single writers of snapshot and stamps
rule: Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
@row_check n=1297 row=decision/single-writer-state-files sha=75b04f888a8f v=holds judge=sonnet head=8834195b0683
at: src/run/snapshot.rs:31
quote: write_snapshot(&self.instance_dir, &snapshot)?;
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
@row_add n=1298 row=decision/frame-verbs-reach-only-the-recorded-wrapper area=6-e2e-test-strategy rec=fct-4uc5ehox@2
name: Frame verbs reach only the recorded wrapper
rule: A CLI frame verb writes no frame to a server that is not the recorded wrapper.
guard: security_negatives_server_identity
@row_check n=1298 row=decision/frame-verbs-reach-only-the-recorded-wrapper sha=08935eb8ae05 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
guard_at: tests/security_negatives_server_identity.rs:200
@row_add n=1299 row=contract/list-json/name area=6-e2e-test-strategy rec=fct-nsd7seww@1,fct-hdfefjry@2
boundary: list --json
item: name
promise: list --json carries a session's original name string, escaped as JSON, never the escaped form the table prints.
@row_check n=1299 row=contract/list-json/name sha=02589a20cbb0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:168
quote: pub name: String,
at: src/cmd/list.rs:178
quote: json!({"v": 1, "ok": {"items": board.items, "budget": budget, "skipped": board.skipped}})
@row_add n=1300 row=decision/property-tests-on-parsers area=6-e2e-test-strategy rec=fct-advgjcvl@1,fct-akm552hs@1
name: Property tests on every parser surface
rule: proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline and agents parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
@row_check n=1300 row=decision/property-tests-on-parsers sha=9f1668318155 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:969
quote: proptest! {
at: crates/viola-channel/src/frame.rs:172
quote: cases: 512,
at: crates/viola-channel/src/frame.rs:182
quote: proptest::proptest! {
at: crates/viola-agent-claude/src/hook.rs:893
quote: proptest! {
at: crates/viola-agent-claude/src/statusline.rs:485
quote: proptest! {
at: crates/viola-agent-claude/src/agents.rs:361
quote: proptest! {
at: crates/viola-agent-claude/src/screen.rs:579
quote: proptest! {
at: crates/viola-agent-claude/src/hook.rs:985
quote: fn prompt_text_prop_round_trips_a_wrapped_paste(
at: crates/viola-core/proptest-regressions/lib.txt:1
quote: # Seeds for failure cases proptest has generated in the past. It is
@row_add n=1301 row=decision/fixtures-match-tolerant-schema area=6-e2e-test-strategy rec=fct-q6ltykaz@1,fct-sn6e6lxw@2
name: Fixtures checked against the tolerant schema
rule: jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
@row_check n=1301 row=decision/fixtures-match-tolerant-schema sha=599933c82874 v=holds judge=sonnet head=8834195b0683
at: tests/support/hygiene.rs:87
quote: let validator = jsonschema::validator_for(schema).map_err(|_| Violation::Schema)?;
at: tests/contract_fixture_hygiene.rs:129
quote: for path in claude_fixtures(&workspace_path("fixtures/claude")) {
at: schemas/claude-fixture.v1.json:10
quote: "enum": ["SessionStart", "UserPromptSubmit", "PreToolUse", "PermissionRequest", "Stop", "SessionEnd", "Notification", "PostToolUse", "PostToolUseFailure"]
@row_add n=1302 row=decision/bounded-readers area=6-e2e-test-strategy rec=fct-q6ltykaz@1,fct-sn6e6lxw@2
name: Every external reader is bounded
rule: Every product reader of external input is bounded in-statement or named in the enumeration with a witness test.
except: Reads of viola's own executable take no external input and are listed as exceptions.
guard: product_readers_are_each_bounded_or_named
@row_check n=1302 row=decision/bounded-readers sha=a38f1610e45f v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_bounded_readers.rs:29
quote: const BOUNDARIES: &[Boundary] = &[
at: tests/contract_bounded_readers.rs:394
quote: const EXCEPTIONS: &[(&str, &str, &str)] = &[
at: tests/contract_bounded_readers.rs:690
quote: fn product_readers_are_each_bounded_or_named() {
guard_at: tests/contract_bounded_readers.rs:690
@row_add n=1303 row=decision/test-homes-made-private area=7-test-data-fixtures rec=fct-54gkcbby@2,fct-ubb4ig4c@2,fct-6upjfbpd@2
name: Test-made homes are private directories
rule: A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation.
why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
@row_check n=1303 row=decision/test-homes-made-private sha=55bceae626fd v=holds judge=sonnet head=8834195b0683
at: tests/run_cli.rs:212
quote: viola_state::fs::create_private_dir(home).expect("home");
@row_add n=1304 row=decision/mcp-exchange-recorded-by-tap area=7-test-data-fixtures rec=fct-auxau5hd@1,fct-beozd4ox@1
name: MCP exchange recorded by a test-only tap
rule: The recorded MCP exchange is taken by the test-only viola-mcp-tap through the same scrub and refusal, its raw capture 0600 and never committed, and the kept fixture is schema-validated and replayed.
@row_check n=1304 row=decision/mcp-exchange-recorded-by-tap sha=7a7b5a14f6e8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:52
quote: options.mode(0o600);
at: crates/viola-e2e/src/bin/viola-mcp-tap.rs:212
quote: let scrubbed = ledger::scrub(&doc, &home, &user);
at: crates/viola-e2e/tests/mcp_recorded_exchange.rs:20
quote: /// `<root>/*/Mcp.*.json`, sorted.
@row_add n=1305 row=contract/fake-agent-receipt/env area=7-test-data-fixtures rec=fct-3es552c5@1,fct-gbrktuzh@1,fct-mw3qahin@1
boundary: viola-fake-agent receipt
item: env
promise: An env receipt holds variable names, sorted, names only, never a value.
@row_check n=1305 row=contract/fake-agent-receipt/env sha=52f43fcd83e1 v=holds judge=sonnet head=8834195b0683
at: src/bin/viola-fake-agent.rs:926-930
quote: let mut names: Vec<String> = std::env::vars_os() .map(|(k, _)| k.to_string_lossy().into_owned()) .collect(); names.sort(); agent.receipt.write("env", json!({"names": names}));
@row_add n=1306 row=decision/mockall-on-seam-traits-only area=8-mocking-stubbing-discipline rec=fct-76bthv7a@1,fct-3a3m4jll@1,fct-sik7tvk4@2
name: mockall doubles on seam traits only
rule: Test doubles are mockall automock, under test only, on traits viola already defines at its seams, with dependency injection preferred over runtime patching of production code; external processes are real stub executables.
@row_check n=1306 row=decision/mockall-on-seam-traits-only sha=54bc60604c3e v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:107
quote: #[cfg_attr(test, mockall::automock)]
at: Cargo.toml:214
quote: mockall = "=0.15.0"
@row_add n=1307 row=decision/no-orphan-modules area=9-ci-integration rec=fct-7h3hdkfi@1,fct-u7ux54l5@1,fct-pk32flwi@2
name: No orphan modules
rule: Any cargo-modules orphan in any library or binary target fails the build.
guard: orphans-check.sh with its probe of two planted orphans
@row_check n=1307 row=decision/no-orphan-modules sha=3cc50d15f6d5 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/orphans-check.sh:30
quote: log=$(cargo modules orphans "$@" --deny --target "$triple" 2>&1) || rc=$?
at: .github/workflows/ci.yml:314
quote: run: bash scripts/orphans-check.sh --probe && bash scripts/orphans-check.sh
guard_at: scripts/orphans-check.sh:131
@row_add n=1308 row=decision/mutation-gate-thresholds area=10-quality-gates-coverage-targets rec=fct-xnmc6vwq@1,fct-ocdr3xdn@3
name: Mutation gate thresholds
rule: A mutation run passes only with missed == 0, timeout == 0 and unviable <= caught, missed read after host-excluded records are left out.
why: A run whose mutant builds mostly failed is no evidence of test strength; the unviable threshold was picked by measurement.
@row_check n=1308 row=decision/mutation-gate-thresholds sha=81a80fa08ab8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
at: crates/viola-e2e/src/harness/run/mutants.rs:61
quote: let swamped = unviable > caught;
@row_add n=1309 row=decision/every-surface-asserted area=11-test-anti-patterns-never-do-these rec=fct-3wg6nwel@1
name: Every surface of a critical path asserted
rule: A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
@row_check n=1309 row=decision/every-surface-asserted sha=81a513178988 v=unverifiable judge=sonnet head=8834195b0683
searched: surface|Surface · in tests/path*.rs, tests/cross_*.rs
why: no evidence found
@row_add n=1310 row=decision/env-clear-keeps-profile-file area=11-test-anti-patterns-never-do-these rec=fct-wxsgpjht@1,fct-ennvbmje@2
name: Cleared environment keeps LLVM_PROFILE_FILE
rule: A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`.
why: Without it the run silently drops out of coverage.
@row_check n=1310 row=decision/env-clear-keeps-profile-file sha=924ef3553fc4 v=unverifiable judge=sonnet head=8834195b0683
searched: env_clear · in tests/**/*.rs, crates/*/tests, crates/*/src, src/ (no `.env_clear()` call anywhere, only a comment at tests/support/cli.rs:3 "with no `env_clear`", so no line re-adds `LLVM_PROFILE_FILE` after a clear)
searched: LLVM_PROFILE_FILE · in tests/**/*.rs, crates/*/tests, crates/*/src, src/ (only crates/viola-e2e/src/harness/run.rs:520 in a list of names removed with `env_remove`, and a comment at crates/viola-e2e/src/harness/pre_push/linux.rs:35)
why: no evidence found
@row_add n=1311 row=decision/compact-confirm-window area=e2e-test-strategy rec=fct-qbytqn26@3,fct-lhvxskln@1,fct-id225cf7@2
name: Compact confirmation window
rule: `/compact` on a verified CLI awaits its post-condition inside its own window, `COMPACT_CONFIRM_WINDOW`, of 40 s, clock-driven; a missed post-condition is `not-delivered`/`no-prompt-submitted` with the cursor.
guard: send_compact_confirm_window_is_its_own_bound
except: `CONFIRM_WINDOW_FALLBACK` stays 10 s for every other confirmed send.
state: provisional
@row_check n=1311 row=decision/compact-confirm-window sha=fb9ee7eccacd v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:26
quote: pub const COMPACT_CONFIRM_WINDOW: Duration = Duration::from_secs(40);
at: crates/viola-agent-claude/src/screen.rs:20
quote: pub const CONFIRM_WINDOW_FALLBACK: Duration = Duration::from_secs(10);
at: src/run/send/slot.rs:177-178
quote: Some(Confirms::Post(PostCondition::CompactedSession)) => COMPACT_CONFIRM_WINDOW, _ => CONFIRM_WINDOW_FALLBACK,
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
guard_at: src/run/send/slot.rs:589
@row_add n=1312 row=contract/events/human-input-wheel area=e2e-test-strategy rec=fct-imixgk6n@1,fct-iyklz2mz@1
boundary: events.ndjson
item: human-input
promise: Human editing input logs a `wheel{holder:"human",cause:"human-input"}` record.
@row_check n=1312 row=contract/events/human-input-wheel sha=a0d570bd442c v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:42
quote: (Input::Human, Wheel::Driver, _) => Some((Wheel::Human, WheelCause::HumanInput)),
@row_add n=1313 row=contract/events/harness-prompt area=e2e-test-strategy rec=fct-imixgk6n@1,fct-iyklz2mz@1
boundary: events.ndjson
item: harness-injected turn
promise: A harness-injected turn logs `prompt-submitted{origin:"harness"}` and no `wheel` record.
@row_check n=1313 row=contract/events/harness-prompt sha=7637b4c6ca66 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:193-194
quote: if HARNESS_PREFIXES.iter().any(|p| raw.starts_with(p)) { "harness"
at: src/cmd/run.rs:103-104
quote: let prompt_ok = data["text"].is_string() && matches!(data["origin"].as_str(), Some("harness" | "human"));
at: src/run/wheel.rs:128
quote: self.apply(Input::Human, false);
@row_add n=1314 row=decision/five-hour-pause-threshold area=e2e-test-strategy rec=fct-kn7ryhs7@2,fct-kylcwwrw@2
name: Five-hour pause threshold at 90
rule: A `five_hour` reading of exactly 90 pauses `send` (exit 11, `detail:"five-hour"`), and 89 does not.
guard: path6_the_boundaries_and_the_window_word
@row_check n=1314 row=decision/five-hour-pause-threshold sha=1a0582b656d1 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:157
quote: (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
at: src/run/budget.rs:30
quote: const FIVE_HOUR_DEFAULT: u8 = 90;
at: src/run/send/budget.rs:12
quote: Some(crossed.window.as_str()),
guard_at: tests/cli_budget.rs:191
@row_add n=1315 row=decision/driver-hints-never-name-release area=e2e-test-strategy rec=fct-jaqoqtdh@1,fct-kn7ryhs7@2,fct-kylcwwrw@2
name: Driver-facing hints never name release
rule: A driver-facing refusal hint (human-typing, budget-paused) never contains release.
why: Drivers are refused release.
guard: no_hint_names_release, path6_the_refusal_hint_never_names_release
@row_check n=1315 row=decision/driver-hints-never-name-release sha=3d365712637e v=holds guard=true judge=sonnet head=8834195b0683
at: src/human/hints.rs:162
quote: assert!(!hint.contains("release"), "{hint}");
at: tests/cli_budget.rs:346
quote: last.starts_with("hint: ") && !last.contains("release"),
guard_at: tests/cross_exit_causes.rs:129
@row_add n=1316 row=decision/error-bodies-hold-no-path area=e2e-test-strategy rec=fct-iplgfuqy@2,fct-jbbspgvm@1
name: Error bodies hold no path or upstream text
rule: CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers.
guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
@row_check n=1316 row=decision/error-bodies-hold-no-path sha=22e00d2710ed v=holds guard=true judge=sonnet head=8834195b0683
at: tests/support/events.rs:30-31
quote: } else if text.contains("Caused by") { Some("caused-by")
at: tests/cli_error_bodies.rs:106
quote: assert_error_body_clean(label, &body, home, &[STRIPPED_VALUE, UPSTREAM]);
guard_at: tests/cli_error_bodies.rs:122
@row_add n=1317 row=contract/revive-json/refusal-document area=e2e-test-strategy rec=fct-jaqoqtdh@1,fct-xttglbra@2
boundary: revive --json
item: refusal document
promise: An exit-1 refusal of the strict-modes check, the preflight or the start it shares with run prints {"v":1,"error":"<code>","detail":null} on stdout with empty stderr; without --json the human pair is unchanged.
@row_check n=1317 row=contract/revive-json/refusal-document sha=3e61e7ff3ccb v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:107
quote: client::refused_document(self.detail());
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=1318 row=decision/peer-euid-admission area=mocking-stubbing-discipline rec=fct-6xaoh2kl@2,fct-o57nkgzg@2
name: Peer admitted only on the owning user's euid
rule: A channel peer is admitted only when its observed euid equals the server's own; an unreadable euid is refused, and a refused peer is dropped before any frame, with no line written.
@row_check n=1318 row=decision/peer-euid-admission sha=aceacdc8d662 v=holds guard=false judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:140
quote: observed == Some(own)
at: crates/viola-channel/src/server.rs:1031
quote: #[case::refused(|_| false, false)]
@row_add n=1319 row=contract/mutants-verdict/no-rust-delta area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3,fct-mihqkrxe@2
boundary: run --mutants verdict
item: no-rust-delta
promise: A closed mutants.verdict value: a diff naming no .rs path never reaches cargo-mutants and passes with tested 0, the diff and its file count.
@row_check n=1319 row=contract/mutants-verdict/no-rust-delta sha=0b3b9a8b6544 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:165
quote: let (suite, doc) = unmutated("no-rust-delta", &diff, &base);
@row_add n=1320 row=contract/mutants-verdict/test-only-rust-delta area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3,fct-mihqkrxe@2
boundary: run --mutants verdict
item: test-only-rust-delta
promise: A closed mutants.verdict value: a diff whose .rs paths are all test targets passes only with this verdict, naming the diff, its file count and rust_files.
@row_check n=1320 row=contract/mutants-verdict/test-only-rust-delta sha=3795642e0aea v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:170
quote: let (suite, mut doc) = unmutated("test-only-rust-delta", &diff, &base);
@row_add n=1321 row=contract/mutants-verdict/counted area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3,fct-mihqkrxe@2
boundary: run --mutants verdict
item: counted
promise: A closed mutants.verdict value: any other diff naming a .rs path, a mixed one included, is counted; a counted run that produces no fresh outcomes.json is red.
@row_check n=1321 row=contract/mutants-verdict/counted sha=e2b8cd299375 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:180
quote: "counted"
at: crates/viola-e2e/src/harness/run/mutants.rs:268
quote: failures: vec!["outcomes-missing".to_owned()],
@row_add n=1322 row=contract/mutants-verdict/package area=quality-gates-coverage-targets rec=fct-ocdr3xdn@3,fct-mihqkrxe@2
boundary: run --mutants verdict
item: package
promise: A closed mutants.verdict value: a whole member scored by run --mutants --package <member>, under the same missed == 0, timeout == 0 and unviable <= caught.
@row_check n=1322 row=contract/mutants-verdict/package sha=e70eca703c69 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:154
quote: let doc = json!({"tested": run.tested, "verdict": "package", "package": member});
@row_add n=1323 row=decision/perf-gate-spine-deadline area=quality-gates-coverage-targets rec=fct-ftncprbj@2,fct-47aec6c3@3,fct-da3wgwes@2
name: Hook perf gate at the spine deadline
rule: The five hook perf rows are required by name, and every timed hook row's hyperfine max must be below the product's SPINE_DEADLINE (1.0 s); a missing row is a breach, never a pass.
@row_check n=1323 row=decision/perf-gate-spine-deadline sha=b762b24ccea0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:21-27
quote: pub const ROWS: [&str; 5] = [ "session-start", "user-prompt-submit", "stop", "session-end", "pre-tool-use", ];
at: crates/viola-e2e/src/harness/gate.rs:133-134
quote: if !artifacts.join(&name).is_file() { breaches.push(breach("artifact-missing", "perf", name));
at: crates/viola-e2e/src/harness/gate.rs:145
quote: let bound = viola_core::SPINE_DEADLINE.as_secs_f64();
at: crates/viola-e2e/src/harness/gate.rs:151
quote: Some(max) if max < bound => {}
@row_add n=1324 row=contract/cli-frame-verbs/strict-modes-failed area=test-scope-summary rec=fct-xttglbra@2,fct-mqdaf5bf@2
boundary: CLI frame verbs
item: strict-modes-failed
promise: A CLI frame verb refuses a widened instance before any role file or log opens: exit 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, with no frame and no line written.
@row_check n=1324 row=contract/cli-frame-verbs/strict-modes-failed sha=3ef8e3ec9a58 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=1325 row=decision/local-command-exact-match area=test-scope-summary rec=fct-lhvxskln@1,fct-id225cf7@2
name: Local commands matched by exact equality
rule: Send recognises a local command only by exact equality of the typed text (without its trailing CR and LF characters) against the compiled list: nothing else trimmed, no case folding, never a leading-slash test.
@row_check n=1325 row=decision/local-command-exact-match sha=9cfb630fbc7b v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/hook.rs:232
quote: let text = text.trim_end_matches(['\r', '\n']);
at: src/run/send/classify.rs:28
quote: match LOCAL_COMMANDS.iter().find(|(command, _)| *command == text) {
at: src/run/send.rs:111
quote: let confirms = classify(text, slot.cli_verified);
@row_add n=1326 row=decision/budget-governor-thresholds area=unit-test-strategy rec=fct-id225cf7@2,fct-qobe5v26@2
name: Budget governor thresholds
rule: Automated send is refused with budget-paused once five_hour reaches 90 % or seven_day reaches 85 % (89 and 84 stay open), and release --budget overrides the pause until its override instant has passed.
guard: gate_decide_an_expired_override_is_dropped
@row_check n=1326 row=decision/budget-governor-thresholds sha=6da8364c5588 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/budget.rs:157
quote: (*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {
at: src/run/budget.rs:30-31
quote: const FIVE_HOUR_DEFAULT: u8 = 90; const SEVEN_DAY_DEFAULT: u8 = 85;
at: src/run/budget.rs:173
quote: if let Some(end) = override_until.filter(|end| later(end, now)) {
guard_at: src/run/budget.rs:615
@row_add n=1327 row=decision/local-command-unconfirmable area=unit-test-strategy rec=fct-id225cf7@2
name: Local command without a post-condition is unconfirmable
rule: A local command with no measured post-condition, or sent to an unverified CLI, gives unconfirmable with no confirmation window.
guard: …_without_a_measured_post_condition_is_unconfirmable
@row_check n=1327 row=decision/local-command-unconfirmable sha=82b3c5533fb8 v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send/classify.rs:31
quote: Some(_) => Confirms::Nothing,
at: src/run/send.rs:151-152
quote: if matches!(confirms, Confirms::Nothing) { return ctx.unconfirmable(cursor, slot.clock.now());
guard_at: src/run/send/classify.rs:51
@row_add n=1328 row=decision/clear-confirmed-by-new-session area=unit-test-strategy rec=fct-id225cf7@2
name: /clear confirmed by a new session
rule: /clear on a verified CLI is confirmed only by a session-start with cause clear and a new session id, never by a prompt-submitted or another session-start; its window expiry gives no-prompt-submitted with the cursor.
guard: …_clear_is_confirmed_by_a_new_session
@row_check n=1328 row=decision/clear-confirmed-by-new-session sha=2de7ae50b99e v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:111
quote: Some("clear") if prior.as_deref() != Some(id) => Some(PostCondition::NewSession),
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
guard_at: src/run/send/classify.rs:78
@row_add n=1329 row=decision/compact-confirmed-by-same-session area=unit-test-strategy rec=fct-id225cf7@2
name: /compact confirmed by its same session
rule: /compact on a verified CLI is confirmed only by a session-start with cause compact and the remembered same session id; another cause, another id, no id, nothing remembered or a prompt-submitted claims nothing.
guard: …_compact_is_confirmed_by_its_same_session
@row_check n=1329 row=decision/compact-confirmed-by-same-session sha=ecc7fca9435f v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:112
quote: Some("compact") if prior.as_deref() == Some(id) => {
guard_at: src/run/send/classify.rs:329
@row_add n=1330 row=decision/e2e-home-headroom-floor area=test-data-bootstrap rec=fct-p6be5x2e@1,fct-msk3nz6p@1,fct-zxaumenb@5
name: Headroom floor of the e2e home base
rule: A linked e2e home base whose filesystem reads less than the 9 GiB headroom floor free is refused; the floor is one compiled constant shared by the harness keeper and the test home, set by no env, variable, flag, feature or config key; a plain base reads no headroom.
why: The tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB.
guard: harness::tests::e2e_home_*
@row_check n=1330 row=decision/e2e-home-headroom-floor sha=84413ed9d3c3 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/mod.rs:151
quote: pub const HEADROOM_FLOOR_MIB: u64 = 9 * 1024;
at: tests/support/home.rs:152
quote: pub const HEADROOM_FLOOR_MIB: u64 = 9 * 1024;
at: crates/viola-e2e/src/harness/mod.rs:125
quote: if let Some(low) = headroom_refusal(free(&target)?, HEADROOM_FLOOR_MIB) {
at: crates/viola-e2e/src/harness/mod.rs:131
quote: fs::create_dir_all(base)
guard_at: crates/viola-e2e/src/harness/mod.rs:384
@row_add n=1331 row=decision/coverage-floors area=5-command-implementation rec=fct-zxoe3ae7@2,fct-da3wgwes@2,fct-47aec6c3@3
name: Coverage floors
rule: CI enforces coverage floors of 85 lines, 95 functions and 80 regions, each floor its own breach, measured with the fake agent, viola-e2e, test support and fuzz trees ignored.
@row_check n=1331 row=decision/coverage-floors sha=daee05b47843 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/coverage.rs:15-16
quote: pub const COVERAGE_FLOORS: [(&str, f64); 3] = [("lines", 85.0), ("functions", 95.0), ("regions", 80.0)];
at: crates/viola-e2e/src/harness/run/coverage.rs:11-12
quote: pub const COVERAGE_IGNORE: &str = r"(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])";
at: crates/viola-e2e/src/harness/gate.rs:111
quote: for (metric, floor) in COVERAGE_FLOORS {
@row_none n=1 rec=fct-dqhtamw2@1 type=S
@row_none n=2 rec=fct-l5lhrwtf@1 type=0
what: an editing note about when the document itself may be changed
@row_none n=3 rec=fct-uzc5osyb@1 type=S
@row_none n=4 rec=fct-r5flmuox@1 type=S
@row_none n=5 rec=fct-bu2wibzw@1 type=5
@row_none n=6 rec=fct-pebopeci@1 type=5
@row_none n=7 rec=fct-njspb5nv@3 type=4
relied: The child's exit is detected on the process handle and never on EOF.
@row_none n=8 rec=fct-ocrbiixg@1 type=5
@row_none n=9 rec=fct-4rrynj5z@1 type=5
@row_none n=10 rec=fct-23i45jvh@1 type=5
@row_none n=11 rec=fct-np3dgpi7@1 type=5
@row_none n=12 rec=fct-5uplt6g5@1 type=5
@row_none n=13 rec=fct-ydbukved@1 type=5
@row_none n=14 rec=fct-hmbzhbf7@1 type=S
@row_none n=15 rec=fct-5fsff64v@1 type=S
@row_none n=16 rec=fct-vdbzoexn@1 type=S
@row_none n=17 rec=fct-xk4vhilu@1 type=9
@row_none n=18 rec=fct-momocfjp@1 type=9
@row_none n=19 rec=fct-qyfqvwpo@1 type=9
@row_none n=20 rec=fct-udamboof@1 type=S
@row_none n=21 rec=fct-afnl6nvb@1 type=S
@row_none n=22 rec=fct-vbygawhs@1 type=S
@row_none n=23 rec=fct-nymxcmgt@1 type=S
@row_none n=24 rec=fct-3p5vn3ea@1 type=S
@row_none n=25 rec=fct-m6h5pcif@1 type=S
@row_none n=26 rec=fct-ts43nhos@1 type=S
@row_none n=27 rec=fct-k2yzqcx6@1 type=S
@row_none n=28 rec=fct-tqclwfnu@2 type=S
@row_none n=29 rec=fct-oiewcpcp@1 type=S
@row_none n=30 rec=fct-oyk2rij4@1 type=S
@row_none n=31 rec=fct-ymia7ofg@1 type=5
@row_none n=32 rec=fct-2hayha6u@2 type=5
@row_none n=33 rec=fct-7vfmbe3m@2 type=5
@row_none n=34 rec=fct-xcvr2ftc@1 type=5
@row_none n=35 rec=fct-fljf6obl@1 type=S
@row_none n=36 rec=fct-ict4jqoa@1 type=S
@row_none n=37 rec=fct-gr2ozii7@1 type=S
@row_none n=38 rec=fct-h5z4y4ot@1 type=S
@row_none n=39 rec=fct-vmaqxk5t@1 type=S
@row_none n=40 rec=fct-obagkhyb@1 type=S
@row_none n=41 rec=fct-nmpwd4ab@1 type=S
@row_none n=42 rec=fct-xq6vpu6l@1 type=4
relied: The watch report carries a stalled wrapper's state at the readiness deadline, before the panic, and a passing boot adds nothing to it.
@row_none n=43 rec=fct-cl3ld5uh@1 type=S
@row_none n=44 rec=fct-znwunjjb@1 type=S
@row_none n=45 rec=fct-nywojhlo@1 type=S
@row_none n=46 rec=fct-odgitjio@1 type=S
@row_none n=47 rec=fct-yo2sxb4f@1 type=5
@row_none n=48 rec=fct-at6p4owu@1 type=S
@row_none n=49 rec=fct-43gyt7hv@1 type=5
@row_none n=50 rec=fct-57rdoxhz@1 type=5
@row_none n=51 rec=fct-yckihvdn@1 type=5
@row_none n=52 rec=fct-zjvldsf3@1 type=5
@row_none n=53 rec=fct-h73p3622@1 type=5
@row_none n=54 rec=fct-g6cib4bm@1 type=5
@row_none n=55 rec=fct-f6lswl4x@1 type=5
@row_none n=56 rec=fct-irpt6eev@1 type=5
@row_none n=57 rec=fct-j7upv7f4@1 type=5
@row_none n=58 rec=fct-5ylrta5q@1 type=5
@row_none n=59 rec=fct-tdook54p@1 type=5
@row_none n=60 rec=fct-qyyocm3q@1 type=5
@row_none n=61 rec=fct-giovpuyr@1 type=5
@row_none n=62 rec=fct-jiq73dx3@1 type=5
@row_none n=63 rec=fct-r3lcwsm7@1 type=S
@row_none n=64 rec=fct-mmqn6p7s@1 type=S
@row_none n=65 rec=fct-mx7xzaet@1 type=S
@row_none n=66 rec=fct-zinytcdj@1 type=S
@row_none n=67 rec=fct-les7ny3j@1 type=S
@row_none n=68 rec=fct-pb23i77m@1 type=4
relied: The boot command creates the home, stamps it via viola verify and starts the wrappers, which tests rely on to seed their own data.
@row_none n=69 rec=fct-qwtscuv3@1 type=S
@row_none n=70 rec=fct-wok64pug@1 type=4
relied: Per session, cleanup removes the harness home, session.json and the supervisor.
@row_none n=71 rec=fct-6nl4t77j@1 type=S
@row_none n=72 rec=fct-galwepqe@1 type=S
@row_none n=73 rec=fct-m3e2zfxx@1 type=S
@row_none n=74 rec=fct-ahdpqenp@1 type=S
@row_none n=75 rec=fct-jw45xvx5@1 type=S
@row_none n=76 rec=fct-5hz2d655@1 type=S
@row_none n=77 rec=fct-vj56oiq2@1 type=S
@row_none n=78 rec=fct-ocmnyevd@1 type=S
@row_none n=79 rec=fct-nc7rbvnm@1 type=5
@row_none n=80 rec=fct-gs6kxojs@1 type=5
@row_none n=81 rec=fct-oz6bj3oy@1 type=5
@row_none n=82 rec=fct-3g23erif@1 type=S
@row_none n=83 rec=fct-kohfqcyc@1 type=S
@row_none n=84 rec=fct-vua4yfw6@1 type=S
@row_none n=85 rec=fct-xugh2lfn@1 type=S
@row_none n=86 rec=fct-mmlpb66v@1 type=0
what: the coverage floors of the Minimal tier, a tier this project does not use
@row_none n=87 rec=fct-4jvy3xbt@1 type=0
what: the coverage floors of the Standard tier, a tier this project does not use
@row_none n=88 rec=fct-pzyggn3l@1 type=S
@row_none n=89 rec=fct-wcemoat6@1 type=S
@row_none n=90 rec=fct-hdwvrmvb@1 type=S
@row_none n=91 rec=fct-xsdmwdxz@1 type=S
@row_none n=92 rec=fct-suizqs5j@1 type=S
@row_none n=93 rec=fct-tli2yavh@1 type=S
@row_none n=94 rec=fct-e3n7qfim@1 type=S
@row_none n=95 rec=fct-bg247fzm@1 type=S
@row_none n=96 rec=fct-4otx4vxd@2 type=6
@row_none n=97 rec=fct-a43jfmhh@1 type=6
@row_none n=98 rec=fct-l7jcwkqi@1 type=5
@row_none n=99 rec=fct-qdldsdqk@1 type=8
@row_none n=100 rec=fct-4n3xwm4s@1 type=5
@row_none n=101 rec=fct-kh5kjsnv@1 type=5
@row_none n=102 rec=fct-lmiov3q7@1 type=5
@row_none n=103 rec=fct-sefkfuph@1 type=5
@row_none n=104 rec=fct-lqbzryiq@1 type=5
@row_none n=105 rec=fct-oecictpm@1 type=5
@row_none n=106 rec=fct-v76i6l5l@1 type=5
@row_none n=107 rec=fct-pyb4ohg2@1 type=5
@row_none n=108 rec=fct-nbvt5ht3@1 type=5
@row_none n=109 rec=fct-lwjugv35@1 type=5
@row_none n=110 rec=fct-5eczp6gf@2 type=5
@row_none n=111 rec=fct-2rmlc6ik@1 type=5
@row_none n=112 rec=fct-tmgp6fr5@1 type=5
@row_none n=113 rec=fct-nwgyqyjs@2 type=5
@row_none n=114 rec=fct-bio6aaf2@1 type=5
@row_none n=115 rec=fct-vgds3skp@2 type=5
@row_none n=116 rec=fct-5qxwlgko@3 type=5
@row_none n=117 rec=fct-jfjmhkwu@2 type=5
@row_none n=118 rec=fct-t5v77dkt@2 type=5
@row_none n=119 rec=fct-7tiauhj7@1 type=4
relied: The error-body check reads a hit of a planted value as class planted and reports place and class, never bytes.
@row_none n=120 rec=fct-rgx56qfp@2 type=5
@row_none n=121 rec=fct-wmieczsn@2 type=5
@row_none n=122 rec=fct-df3r2j7c@1 type=5
@row_none n=123 rec=fct-56kgfapd@1 type=S
@row_none n=124 rec=fct-zs4cl5pl@1 type=S
@row_none n=125 rec=fct-aet5iavj@1 type=S
@row_none n=126 rec=fct-aqbt7wnm@1 type=S
@row_none n=127 rec=fct-n3ryxatt@1 type=S
@row_none n=128 rec=fct-b5skp7pn@1 type=S
@row_none n=129 rec=fct-6ihplwew@1 type=5
@row_none n=130 rec=fct-jjhahrts@1 type=S
@row_none n=131 rec=fct-fje2sz6m@1 type=S
