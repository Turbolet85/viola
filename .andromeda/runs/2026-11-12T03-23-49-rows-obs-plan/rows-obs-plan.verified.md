@row_add n=1 row=decision/agent-readable-telemetry area=obs-plan-viola rec=fct-gty7muwj@1
name: Agent-readable telemetry only
rule: Every telemetry signal must be readable by an agent from a snapshot or pasted log excerpt: no human-only dashboards, no proprietary APM lock-in, no human-review-gated log analysis.
except: A surface that cannot be agent-readable is flagged not-instrumentable and handled through boundary instrumentation only.
@row_check n=1 row=decision/agent-readable-telemetry sha=130bb7f95b9c v=unverifiable judge=sonnet head=8834195b0683
searched: not-instrumentable · in tracked source outside pipeline records
searched: datadog|newrelic|opentelemetry|sentry|grafana|prometheus · in tracked source outside pipeline records (only deny.toml:32-34 bans opentelemetry-otlp, opentelemetry-stdout, sentry)
why: not a code claim
@row_add n=2 row=decision/obs-tier-standard area=1-obs-scope-summary rec=fct-y4ogetzn@1,fct-6eqwgp2k@1
name: Observability tier Standard
rule: The observability tier is Standard (1), with Minimal-tier exporter carve-outs.
why: The tests tier is Comprehensive (2) and obs sits one step below it within the plus-or-minus-one band; the Minimal security tier gives no Comprehensive driver.
@row_check n=2 row=decision/obs-tier-standard sha=b3117ee9de31 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=3 row=absence/audit-log-compliance-trail area=1-obs-scope-summary rec=fct-6eqwgp2k@1
capability: An audit-log compliance trail
@row_check n=3 row=absence/audit-log-compliance-trail sha=1971401cc33d v=holds judge=sonnet head=8834195b0683
searched: (?i)audit · in src/**/*.rs and crates/*/src/**/*.rs
@row_add n=4 row=decision/no-network-otlp-exporter area=1-obs-scope-summary rec=fct-4v6f5pir@1,fct-uv7mc5hr@1,fct-fxg4qazp@1
name: No network OTLP exporter
rule: Telemetry has no network OTLP exporter, loopback included; the Standard default is replaced by a Minimal-tier shape.
why: Upstream bans outbound network calls and new listeners without a Security Decisions Log entry; viola makes no outbound network calls.
@row_check n=4 row=decision/no-network-otlp-exporter sha=4406a8330440 v=holds judge=sonnet head=8834195b0683
searched: (?i)otlp|opentelemetry|TcpListener|TcpStream|reqwest|hyper · in src/**/*.rs and crates/*/src/**/*.rs
searched: opentelemetry|otlp · in **/Cargo.toml
@row_add n=5 row=decision/no-error-reporter area=1-obs-scope-summary rec=fct-4v6f5pir@1,fct-uv7mc5hr@1
name: No opt-in error reporter
rule: The product has no opt-in error reporter; the Standard default is replaced by a Minimal-tier shape.
why: Upstream bans outbound network calls, reserves stdout on 3 of 4 roles, and bans Tokio and C crates in the sync graph.
@row_check n=5 row=decision/no-error-reporter sha=ee0e7679a4e6 v=holds judge=sonnet head=8834195b0683
searched: (?i)sentry · in src/**/*.rs and crates/*/src/**/*.rs
searched: sentry|opentelemetry · in **/Cargo.toml
@row_add n=6 row=decision/no-env-log-levels area=1-obs-scope-summary rec=fct-ljftf7ri@1,fct-uv7mc5hr@1
name: No per-module log levels from the environment
rule: Per-module log levels are never read from environment variables.
why: Environment variables are not a configuration channel.
@row_check n=6 row=decision/no-env-log-levels sha=2827f801ac68 v=holds judge=sonnet head=8834195b0683
searched: EnvFilter|from_default_env|with_env_filter · in src/**/*.rs and crates/*/src/**/*.rs
searched: env::var · in src/obs.rs and crates/*/src/**/*.rs (no log-level variable read; only a comment names RUST_LOG at src/obs.rs:180)
@row_add n=7 row=decision/error-chain-fixed-stderr-message area=1-obs-scope-summary rec=fct-cjx2g7cf@1,fct-3yxzr6p7@1
name: Error chains reach stderr as fixed messages
rule: In the viola root binary, the only crate that uses anyhow, an error chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to the diagnostics logs only.
@row_check n=7 row=decision/error-chain-fixed-stderr-message sha=cc2fab3b111d v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/obs.rs:271-276
quote: let chain: Vec<Value> = error .chain() .map(|cause| Value::from(cause.to_string())) .collect(); let mut fields = Map::new(); fields.insert("chain".to_owned(), Value::Array(chain));
@row_add n=8 row=decision/viola-core-emits-no-telemetry area=1-obs-scope-summary rec=fct-gy7sj7m2@1,fct-yt4lnr2b@1
name: viola-core emits no telemetry
rule: The viola-core crate supplies log-field values but emits nothing and is not instrumented.
why: A pure type crate with no I/O.
@row_check n=8 row=decision/viola-core-emits-no-telemetry sha=0e241359079c v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:143
quote: ::tracing::event!(
@row_add n=9 row=decision/channel-json-rpc-over-ndjson area=1-obs-scope-summary rec=fct-lz2jkmkt@1,fct-p4bwzizt@2
name: Channel is JSON-RPC 2.0 over ndjson
rule: The viola-channel crate speaks JSON-RPC 2.0 over ndjson on local Unix sockets, one endpoint per viola run, with a sync client and server and a Tokio client behind the tokio feature.
@row_check n=9 row=decision/channel-json-rpc-over-ndjson sha=8313140bee32 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:83
quote: if request.jsonrpc != "2.0" {
at: crates/viola-channel/src/server.rs:123
quote: let listener = ListenerOptions::new()
at: crates/viola-channel/src/lib.rs:12-13
quote: #[cfg(feature = "tokio")] pub mod tokio_client;
@row_add n=10 row=decision/channel-sync-instrumentation area=1-obs-scope-summary rec=fct-hezc5s2v@1,fct-lz2jkmkt@1
name: Sync instrumentation in viola-channel
rule: Instrumentation in viola-channel must be sync, because Tokio is forbidden in that crate's graph.
except: The tokio feature, when enabled.
@row_check n=10 row=decision/channel-sync-instrumentation sha=b7fc6d53b72a v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/Cargo.toml:17
quote: tokio = ["dep:tokio", "interprocess/tokio"]
at: crates/viola-channel/Cargo.toml:27
quote: tracing.workspace = true
@row_add n=11 row=decision/state-sync-instrumentation area=1-obs-scope-summary rec=fct-ewk35na4@1,fct-4rqiexan@1
name: Sync instrumentation in viola-state
rule: Instrumentation in viola-state must be sync.
why: The Stack bans Tokio in viola-state.
@row_check n=11 row=decision/state-sync-instrumentation sha=b21ba96b2591 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/Cargo.toml:26
quote: tracing.workspace = true
@row_add n=12 row=decision/stripped-claude-env-never-logged area=1-obs-scope-summary rec=fct-gira4fst@1,fct-73ei4s3r@2
name: R8-stripped CLAUDE values never logged
rule: R8-stripped CLAUDE* environment values are never logged anywhere.
why: The viola-agent-claude outputs are the most sensitive in the product.
@row_check n=12 row=decision/stripped-claude-env-never-logged sha=e0389a849d92 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:49-50
quote: let stripped = strip.removed_names(); let kept = strip.kept_names();
@row_add n=13 row=decision/rmcp-stdio-mcp-server area=1-obs-scope-summary rec=fct-zry4bqzw@1
name: rmcp stdio MCP server
rule: The viola-mcp crate is an rmcp stdio MCP server.
@row_check n=13 row=decision/rmcp-stdio-mcp-server sha=bcc5a30bbccd v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:223
quote: rmcp = { version = ">=3.4.1, <3.5", default-features = false, features = ["server", "transport-io"] }
@row_add n=14 row=contract/viola-mcp-tools/send area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: send
promise: The viola-mcp stdio server offers a send tool.
@row_check n=14 row=contract/viola-mcp-tools/send sha=783762abc2e0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=15 row=contract/viola-mcp-tools/wait area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: wait
promise: The viola-mcp stdio server offers a wait tool.
@row_check n=15 row=contract/viola-mcp-tools/wait sha=5446fc559de2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=16 row=contract/viola-mcp-tools/last area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: last
promise: The viola-mcp stdio server offers a last tool.
@row_check n=16 row=contract/viola-mcp-tools/last sha=3685ac3cd9e0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=17 row=contract/viola-mcp-tools/answer area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: answer
promise: The viola-mcp stdio server offers an answer tool.
@row_check n=17 row=contract/viola-mcp-tools/answer sha=2329d206f70e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=18 row=contract/viola-mcp-tools/list area=1-obs-scope-summary rec=fct-zry4bqzw@1
boundary: viola-mcp stdio server tools
item: list
promise: The viola-mcp stdio server offers a list tool.
@row_check n=18 row=contract/viola-mcp-tools/list sha=a03b10a8293e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:37
quote: pub const TOOL_NAMES: [&str; 5] = ["send", "wait", "last", "answer", "list"];
@row_add n=19 row=decision/mcp-stdout-frames-only area=1-obs-scope-summary rec=fct-uulw22xw@1,fct-zry4bqzw@1
name: MCP stdout carries frames only
rule: No telemetry from the viola-mcp server may go to stdout.
why: Stdout is reserved for MCP frames.
@row_check n=19 row=decision/mcp-stdout-frames-only sha=fef1d927b28f v=holds judge=sonnet head=8834195b0683
searched: println!|print!\(|stdout\(\) · in crates/viola-mcp/src
@row_add n=20 row=decision/child-exit-from-process-handle area=1-obs-scope-summary rec=fct-avrouaxx@1,fct-ctz7espz@1
name: Child exit from the process handle
rule: The wrapped child's exit and its process-exit event come from the wait on the process handle, never from reader EOF.
@row_check n=20 row=decision/child-exit-from-process-handle sha=fca70052669c v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:4-5
quote: //! The line is portable-pty 0.8.1. Exit is read on the process handle ([`Pty::try_wait`]), never on //! the end of output.
at: src/cmd/run.rs:702
quote: run::log_child_exit(exit);
@row_add n=21 row=decision/agents-probe-output-untrusted area=1-obs-scope-summary rec=fct-lpshe4lx@1,fct-ubqrs7oo@1
name: Agents probe output untrusted
rule: The output rows of the claude agents --json probe are untrusted.
@row_check n=21 row=decision/agents-probe-output-untrusted sha=e4d6dd1776d4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/agents.rs:90-91
quote: if bytes.len() as u64 > MAX_FRAME {         return Err(AgentsFailure::Oversize);
@row_add n=22 row=decision/statusline-command-duration-exit-only area=1-obs-scope-summary rec=fct-hfqkop25@1,fct-4hfgtuu2@1
name: Statusline command observed by duration and exit
rule: The user's statusline_command shell-out is observed by its duration and exit status only.
why: A user-written shell command.
@row_check n=22 row=decision/statusline-command-duration-exit-only sha=c79b67e86efb v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:207-212
quote: obs_event!( INFO, ObsEvent::ProcessExit, subject = SUBJECT, shell_exit_status = code, duration_ms = duration_ms,
@row_add n=23 row=decision/statusline-command-string-diagnostics-only area=1-obs-scope-summary rec=fct-hfqkop25@1,fct-4hfgtuu2@1
name: Statusline command string kept in diagnostics
rule: The user's statusline_command string is never logged outside the diagnostics logs.
@row_check n=23 row=decision/statusline-command-string-diagnostics-only sha=919d859f9bdc v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:207-212
quote: obs_event!( INFO, ObsEvent::ProcessExit, subject = SUBJECT, shell_exit_status = code, duration_ms = duration_ms,
@row_add n=24 row=decision/os-apis-at-call-sites area=1-obs-scope-summary rec=fct-eokuiijb@1,fct-zce67g4u@2
name: OS APIs observed at call sites
rule: OS APIs (openpty, Unix socket, filesystem locks and atomic rename, sysinfo) are instrumented at viola's call sites only, never in the OS internals.
@row_check n=24 row=decision/os-apis-at-call-sites sha=517fdc9ec924 v=unverifiable judge=sonnet head=8834195b0683
searched: openpty · in src/**/*.rs
why: not a code claim
@row_add n=25 row=decision/claude-fixtures-scrub-required area=1-obs-scope-summary rec=fct-k7mp4nhp@1,fct-cnsk5zxg@1
name: Claude fixtures are scrub-required
rule: Build and CI infrastructure is not instrumented, and the claude fixtures are scrub-required data, never a telemetry source.
why: Not product runtime.
@row_check n=25 row=decision/claude-fixtures-scrub-required sha=aa2537417599 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=26 row=decision/cli-file-sink-only area=1-obs-scope-summary rec=fct-fxg4qazp@1,fct-wf4ruhe4@1
name: Short-lived CLI logs to a file sink only
rule: Short-lived CLI verbs log only to a JSON-per-line file sink in diagnostics when an instance resolves, never to stdout.
why: Stdout carries the --json results.
@row_check n=26 row=decision/cli-file-sink-only sha=94648aecc4b2 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:145
quote: let _ = tracing::subscriber::set_global_default(subscriber(file, level));
@row_add n=27 row=decision/instance-never-from-working-directory area=1-obs-scope-summary rec=fct-fxg4qazp@1
name: Instance never from the working directory
rule: The instance comes from VIOLA_NAME or an argument, never from the working directory.
@row_check n=27 row=decision/instance-never-from-working-directory sha=380c5de20712 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:277
quote: std::env::var_os("VIOLA_NAME"),
@row_add n=28 row=decision/stderr-fixed-strings area=1-obs-scope-summary rec=fct-fxg4qazp@1
name: Fixed strings on stderr
rule: Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
@row_check n=28 row=decision/stderr-fixed-strings sha=05267e609d84 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:54
quote: let _ = write_internal_error(&mut io::stderr().lock());
@row_add n=29 row=contract/run-log-file/path area=1-obs-scope-summary rec=fct-ctz7espz@1,fct-bgcxnqje@1
boundary: the viola run log file
item: path
promise: viola run logs only to a file sink at diagnostics/run-<name>.ndjson.
@row_check n=29 row=contract/run-log-file/path sha=31144b09495a v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:58
quote: format!("{}-{}", process.as_str(), name_str(name))
at: src/obs.rs:62
quote: Some(format!("{stem}.ndjson"))
@row_add n=30 row=decision/run-silent-terminal area=1-obs-scope-summary rec=fct-ctz7espz@1,fct-bgcxnqje@1
name: viola run silent on the terminal
rule: viola run writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns.
@row_check n=30 row=decision/run-silent-terminal sha=f3f44e3b9102 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
@row_add n=31 row=decision/run-sync-instrumentation area=1-obs-scope-summary rec=fct-ctz7espz@1,fct-bgcxnqje@1
name: Sync instrumentation in viola run
rule: Instrumentation in viola run is sync, with no Tokio.
@row_check n=31 row=decision/run-sync-instrumentation sha=bbc314bcb81a v=holds judge=sonnet head=8834195b0683
searched: tokio · in src/**/*.rs
searched: tokio · in Cargo.toml [dependencies] (root package)
@row_add n=32 row=contract/hook-log-file/path area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
boundary: the viola hook log file
item: path
promise: viola hook logs only to a file sink at diagnostics/hook-<name>.ndjson.
@row_check n=32 row=contract/hook-log-file/path sha=8f5365e7d58f v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:58
quote: format!("{}-{}", process.as_str(), name_str(name))
at: src/obs.rs:62
quote: Some(format!("{stem}.ndjson"))
@row_add n=33 row=decision/hook-stderr-forbidden area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: No stderr from hooks
rule: viola hook writes nothing to stderr.
@row_check n=33 row=decision/hook-stderr-forbidden sha=5fa18fe7a78d v=holds judge=sonnet head=8834195b0683
searched: eprintln! · in src/cmd/hook.rs and src/cmd/hook/*.rs (product code)
searched: stderr(Stdio::null) is the child's stderr · in src/cmd/hook/statusline.rs:228
@row_add n=34 row=decision/hook-stdout-decision-body-only area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: Hook stdout carries the decision body only
rule: viola hook stdout carries only the decision body; for viola hook statusline, the output of the user's status-line command.
@row_check n=34 row=decision/hook-stdout-decision-body-only sha=ad004a259faa v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:298
quote: out.write_all(body.as_bytes())?;
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
@row_add n=35 row=decision/hook-silent-no-op-without-instance area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: Hook silent no-op without an instance
rule: Without VIOLA_NAME, on an instance the instance check refuses, or on a VIOLA_DIR naming an instance directory that does not exist, viola hook is a silent no-op: exit 0, no file opened, no line, no frame.
@row_check n=35 row=decision/hook-silent-no-op-without-instance sha=3bca05ed656d v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
@row_add n=36 row=decision/hook-log-line-single-append-write area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1
name: One write per hook log line
rule: Each viola hook log line is one write on an append-mode handle.
@row_check n=36 row=decision/hook-log-line-single-append-write sha=d5de7c139d1a v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:46
quote: options.create(true).append(true);
at: src/obs.rs:128
quote: .with_writer(writer);
@row_add n=37 row=decision/corr-field-propagation area=1-obs-scope-summary rec=fct-m7cbk627@1,fct-p4bwzizt@2
name: Propagation through the corr field
rule: Correlation across the viola-channel is carried by the corr field in log lines of the process on each side.
why: Channel params declare no trace-context field.
@row_check n=37 row=decision/corr-field-propagation sha=d6bed7dd685a v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:241
quote: corr = call.id,
at: src/cmd/hook.rs:369
quote: corr = dialog_id,
@row_add n=38 row=contract/json-rpc-errors/32603 area=1-obs-scope-summary rec=fct-m7cbk627@1,fct-p4bwzizt@2
boundary: viola-channel JSON-RPC errors
item: -32603
promise: Error -32603 is "internal error" with data:null.
@row_check n=38 row=contract/json-rpc-errors/32603 sha=7ebc81952107 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
at: crates/viola-channel/src/lib.rs:127
quote: -32603 => (ProtocolError::Internal.message(), Value::Null),
@row_add n=39 row=decision/self-reported-from-not-identity area=1-obs-scope-summary rec=fct-m7cbk627@1,fct-p4bwzizt@2
name: Self-reported from is not an identity
rule: The self-reported from on the viola-channel is never logged as an authenticated identity.
@row_check n=39 row=decision/self-reported-from-not-identity sha=158b7da2d2af v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:201
quote: from_trust = self.trust(),
at: src/run/wheel.rs:244
quote: from_trust = "self-reported",
@row_add n=40 row=decision/mcp-stdio-server-tokio-allowed area=1-obs-scope-summary rec=fct-p7pvzia6@1
name: MCP server on stdio with Tokio allowed
rule: The `viola mcp` surface is an internal stdio MCP server, and Tokio is allowed in it.
@row_check n=40 row=decision/mcp-stdio-server-tokio-allowed sha=e8c2edcd763c v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/Cargo.toml:23
quote: tokio = { workspace = true, features = ["rt"] }
@row_add n=41 row=contract/mcp-stdin/max-frame area=1-obs-scope-summary rec=fct-p7pvzia6@1
boundary: viola mcp stdin
item: MAX_FRAME
promise: Stdin is read in lines of at most `MAX_FRAME`; a longer line is refused whole and the session ends as at a closed stdin.
@row_check n=41 row=contract/mcp-stdin/max-frame sha=471f2c825712 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:77
quote: let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);
at: crates/viola-mcp/src/stdin.rs:60
quote: if !self.fits(self.held.len() + segment.len()) {
@row_add n=42 row=decision/no-otel-sdk-exporter area=1-obs-scope-summary rec=fct-hx3c7qre@1
name: No OTel SDK exporter
rule: No OTel SDK exporter is initialised; any OTel bridge may only feed an in-process or file sink and must be Tokio-free.
why: outbound network calls are banned, a new listener needs a Security Decisions Log entry, stdout is reserved on hook, mcp and run, and cargo deny bans Tokio in sync crates
except: an optional Tokio-free OTel bridge feeding an in-process or file sink
@row_check n=42 row=decision/no-otel-sdk-exporter sha=ce08a3fb5947 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry · in Cargo.toml, src/**/*.rs, crates/*/Cargo.toml, crates/*/src/**/*.rs (case-insensitive)
searched: otel · in Cargo.toml, src/**/*.rs, crates/*/Cargo.toml, crates/*/src/**/*.rs (case-insensitive)
@row_add n=43 row=decision/tracing-json-pipeline area=1-obs-scope-summary rec=fct-hx3c7qre@1,fct-ccfaed7l@1
name: tracing with the tracing-subscriber JSON formatter
rule: The `tracing` facade plus the tracing-subscriber JSON formatter is the binding logging pipeline, and JSON-to-file is the only logging mode.
@row_check n=43 row=decision/tracing-json-pipeline sha=7a073bbf26df v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:121
quote: .json()
at: src/obs.rs:128
quote: .with_writer(writer);
@row_add n=44 row=contract/diag-line/service-name area=1-obs-scope-summary rec=fct-uwfffcsu@1
boundary: diagnostic log line
item: service.name
promise: Hardcoded to "viola".
@row_check n=44 row=contract/diag-line/service-name sha=f9171b27070d v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:7
quote: pub const SERVICE_NAME: &str = "viola";
@row_add n=45 row=contract/diag-line/service-version area=1-obs-scope-summary rec=fct-uwfffcsu@1
boundary: diagnostic log line
item: service.version
promise: The crate version, fixed at compile time.
@row_check n=45 row=contract/diag-line/service-version sha=6c823b46c479 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:8
quote: pub const VERSION: &str = env!("CARGO_PKG_VERSION");
@row_add n=46 row=contract/diag-line/process area=1-obs-scope-summary rec=fct-uwfffcsu@1
boundary: diagnostic log line
item: process
promise: The role of the writing process, one of run, hook, mcp, cli; required on every line and matching `sender` / `writer`.
@row_check n=46 row=contract/diag-line/process sha=277c5278e69d v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:146
quote: process = $crate::obs::ctx_process(),
at: crates/viola-core/src/obs.rs:93
quote: Self::Cli => "cli",
@row_add n=47 row=contract/diag-line/instance area=1-obs-scope-summary rec=fct-uwfffcsu@1
boundary: diagnostic log line
item: instance
promise: The instance's `ViolaName`; the key is absent when the process has no instance.
@row_check n=47 row=contract/diag-line/instance sha=0bd3bceade05 v=stale judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:147
quote: instance = $crate::obs::ctx_instance(),
shows: An absent instance is an absent key (doc comment at obs.rs:135 "an absent value is an absent key"), never a null, and the line is not required to carry it.
cell: promise
value: The instance's `ViolaName`; the key is absent when the process has no instance.
@row_add n=48 row=decision/level-filter-not-from-env area=1-obs-scope-summary rec=fct-ccfaed7l@1
name: Level filter never read from RUST_LOG
rule: The log level filter is never read from `RUST_LOG`.
@row_check n=48 row=decision/level-filter-not-from-env sha=cc5ff2644c1c v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:182
quote: pub(crate) fn read_diagnostics_level(home: &Path) -> (Level, Option<ConfigRejection>) {
searched: RUST_LOG|EnvFilter|from_env · in src/obs.rs (only a comment names RUST_LOG, none reads it)
@row_add n=49 row=decision/redaction-not-disableable area=1-obs-scope-summary rec=fct-ccfaed7l@1,fct-xvmeqbmc@1
name: Redaction cannot be disabled
rule: No verbosity control, env, flag or config may disable or bypass redaction.
@row_check n=49 row=decision/redaction-not-disableable sha=23cd2f0aa55f v=holds judge=sonnet head=8834195b0683
searched: (?i)redact|no_redact|VIOLA_DEBUG|VIOLA_LOG|verbose · in src/**/*.rs, crates/**/*.rs (only veil Redact derive on channel Request)
searched: CONFIG_KEYS · in src/obs.rs (v, diagnostics_level, claude_env_keep, five_hour, seven_day, dialog_hold_s: no redaction switch)
@row_add n=50 row=decision/diag-schema-additive-only area=1-obs-scope-summary rec=fct-zhb762yr@1
name: Diagnostic log schema is additive only
rule: The log format JSON schema is binding: observability may add fields or extend the event enum through a Decisions Log entry, and must not rename or remove any field.
@row_check n=50 row=decision/diag-schema-additive-only sha=fa92439dbf89 v=unverifiable judge=sonnet head=8834195b0683
searched: diag-line · in schemas/
why: no evidence found; a never-rename/remove rule is a process rule over a schema history, not shown by a code line
@row_add n=51 row=decision/diagnostics-owner-only-modes area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-wrd7y3z2@2
name: Diagnostics files are owner-only
rule: Every diagnostics file is mode 0600 inside a 0700 directory.
@row_check n=51 row=decision/diagnostics-owner-only-modes sha=bf7a3cf53e54 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:10-11
quote: pub const DIR_MODE: u32 = 0o700; pub const FILE_MODE: u32 = 0o600;
at: src/obs.rs:91-92
quote: ensure_private_dir(&dir)?; Ok(Arc::new(open_private_append(&path)?))
@row_add n=52 row=contract/harness-logs/merge area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: harness logs command
item: merged sources
promise: Merges `events.ndjson` lines (`src:"events"`, with byte `offset`) with `diagnostics/*.ndjson` lines (`src:"diag"`).
@row_check n=52 row=contract/harness-logs/merge sha=e1324567d8b4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:133
quote: head.insert("src".to_owned(), "events".into());
at: crates/viola-e2e/src/harness/logs.rs:135
quote: head.insert("offset".to_owned(), at.into());
at: crates/viola-e2e/src/harness/logs.rs:177
quote: head.insert("src".to_owned(), "diag".into());
@row_add n=53 row=contract/harness-logs/torn-line area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: harness logs command
item: torn lines
promise: A torn line is emitted with its `src` and `offset` and `"torn":true`, never dropped.
@row_check n=53 row=contract/harness-logs/torn-line sha=cf67133bc7c3 v=stale judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:150
quote: torn.insert("torn".to_owned(), true.into());
shows: A torn line is emitted as an object keeping its `src`, `offset` (and `file`/`instance`) with `"torn":true`, not as the bare `{"torn":true}`.
cell: promise
value: A torn line is emitted with its `src` and `offset` and `"torn":true`, never dropped.
@row_add n=54 row=contract/harness-logs/filters area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: harness logs command
item: filters
promise: Output can be filtered by `--instance`, `--kind` and `--process`.
@row_check n=54 row=contract/harness-logs/filters sha=2321e1626c69 v=stale judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:58-67
quote: Logs { #[arg(long, default_value = "default")] session: String, #[arg(long)] instance: Option<String>, #[arg(long)] process: Option<String>, #[arg(long)] kind: Option<String>, },
shows: The logs command takes `--instance`, `--process` and `--kind` only; there is no `--after` filter.
cell: promise
value: Output can be filtered by `--instance`, `--kind` and `--process`.
@row_add n=55 row=contract/harness-logs/output area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: harness logs command
item: output
promise: Output is ndjson on stdout, ready for `jq` or pasting to an agent.
@row_check n=55 row=contract/harness-logs/output sha=56b57c8ce92b v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/bin/viola-harness.rs:215
quote: println!("{line}");
@row_add n=56 row=contract/agent-run-status/state-last-error area=1-obs-scope-summary rec=fct-q2zh72zo@2
boundary: agent-run status
item: state and last_error
promise: The aggregate health snapshot gives `state` (`ready`, `degraded` or `down`) and `last_error` (null, or `list-exit-<code>` built from the list exit code).
@row_check n=56 row=contract/agent-run-status/state-last-error sha=564ab0e8bdbb v=stale judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:28
quote: last_error: Some(format!("list-exit-{code}")),
shows: `state` is one of ready, degraded, down, but `last_error` is null or a `list-exit-<code>` string built from the exit code, not a closed enum.
cell: promise
value: The aggregate health snapshot gives `state` (`ready`, `degraded` or `down`) and `last_error` (null, or `list-exit-<code>` built from the list exit code).
@row_add n=57 row=decision/no-traceparent-on-local-socket area=1-obs-scope-summary rec=fct-pcotmuct@1
name: No W3C traceparent on the local socket
rule: Calls from CLI, MCP and hook to the wrapper carry no W3C `traceparent`; correlation uses `corr` (JSON-RPC `id`, `dialog_id`, send `cursor`) plus `process` and `instance`.
why: channel `params` declare no trace-context field
@row_check n=57 row=decision/no-traceparent-on-local-socket sha=182abd0c73d7 v=holds judge=sonnet head=8834195b0683
searched: traceparent · in src/**/*.rs, crates/**/*.rs
searched: trace_context · in src/**/*.rs, crates/**/*.rs
@row_add n=58 row=decision/hook-stdin-correlation area=1-obs-scope-summary rec=fct-pcotmuct@1
name: Hook stdin correlated by instance and dialog
rule: Hook stdin from Claude Code carries no inbound trace context, so it is correlated by `instance` and `dialog_id`.
@row_check n=58 row=decision/hook-stdin-correlation sha=35a84f4613c8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:401
quote: corr = dialog_id,
searched: traceparent · in src/**/*.rs, crates/**/*.rs
@row_add n=59 row=decision/spans-not-wire-contract area=1-obs-scope-summary rec=fct-o6ysgnna@1
name: Tracing spans are not part of the wire contract
rule: Span names are internal `tracing` spans in `<area>.<operation>` snake_case form; the agent-facing contract is the closed `event` enum plus `corr`.
why: the config sets `with_current_span(false)`, so spans do not reach the line
@row_check n=59 row=decision/spans-not-wire-contract sha=8dd0dd32b3f2 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:123
quote: .with_current_span(false)
at: crates/viola-core/src/obs.rs:12
quote: pub enum ObsEvent {
@row_add n=60 row=decision/exit-1-detail-codes area=1-obs-scope-summary rec=fct-i6rpx5jf@1,fct-qixjo2w7@1
name: Distinct detail codes for exit-1 causes
rule: On the `viola run` start sequence to child spawn, each exit-1 cause logs a distinct `detail` code with no path or pid in it.
@row_check n=60 row=decision/exit-1-detail-codes sha=176dc0a508a1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=61 row=decision/assistant-message-never-logged area=1-obs-scope-summary rec=fct-xsyteni6@1,fct-mutbrgup@1
name: Assistant message never logged on readback
rule: On `wait` / `last` readback, the `last_assistant_message` content never appears in log fields.
@row_check n=61 row=decision/assistant-message-never-logged sha=5e6d6922e19d v=holds judge=sonnet head=8834195b0683
searched: last_assistant_message · in src/run/wait.rs, src/cmd/last.rs, src/run/send/slot.rs (appears only in json! reply bodies and tests, never as an obs_event! field)
searched: obs_event! field list · in crates/viola-channel/src/server.rs:365-376,473-485 (corr, conn, srv_conn, method, sender, v, after, timeout_ms, result_class, error_code, detail, outcome, duration_ms)
@row_add n=62 row=decision/dialog-text-never-logged area=1-obs-scope-summary rec=fct-t4iplhxy@1,fct-berta6ar@1
name: Dialog text never logged on answer
rule: On the dialog to `answer` path, the question, answer and tool `input` text never appear in log fields.
@row_check n=62 row=decision/dialog-text-never-logged sha=1f57b7635c45 v=holds judge=sonnet head=8834195b0683
searched: obs_event!(INFO, ObsEvent::DialogAnswered · in src/run/dialog.rs:458-467 (fields: corr, dialog_kind, from, from_trust, deadline_hit, duration_ms; no question, answer or input)
searched: question|tool_input as obs_event field · in src/run/dialog.rs
@row_add n=63 row=decision/statusline-command-confined area=1-obs-scope-summary rec=fct-z24xvxbu@1,fct-tlje6rwd@1
name: Statusline command confined to diagnostics
rule: On the budget governor path, the `statusline_command` string never appears outside `diagnostics/`.
@row_check n=63 row=decision/statusline-command-confined sha=1bcc3082b4b0 v=holds judge=sonnet head=8834195b0683
searched: statusline_command · in src/ (only snapshot field, statusline.rs:130 read, and run.rs:288 computation; no obs_event! field carries it)
searched: statusline_command · in src/cmd/hook/statusline.rs
searched: obs_event! · in src/cmd/hook/statusline.rs
@row_add n=64 row=decision/channel-lines-carry-no-bodies area=1-obs-scope-summary rec=fct-ernmap45@1
name: Channel log lines carry no bodies
rule: IPC channel log lines never carry `params` or result bodies.
@row_check n=64 row=decision/channel-lines-carry-no-bodies sha=279c5301034e v=holds judge=sonnet head=8834195b0683
searched: obs_event!(INFO, ObsEvent::ChannelRequest · in crates/viola-channel/src/server.rs:365-376 (no params or result field; wait_bounds passes only after/timeout_ms)
searched: ObsEvent::ChannelResponse · in crates/viola-channel/src/server.rs:473-485 (result_class and outcome only)
@row_add n=65 row=decision/channel-from-unauthenticated area=1-obs-scope-summary rec=fct-ernmap45@1
name: Channel sender logged as unauthenticated
rule: The `from` of an IPC channel call is logged as unauthenticated.
@row_check n=65 row=decision/channel-from-unauthenticated sha=415636a0cbd3 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:166
quote: self.from.as_ref().map(|_| "self-reported")
at: src/run/dialog.rs:464
quote: from_trust = from.map(|_| "self-reported"),
@row_add n=66 row=decision/send-text-confined area=1-obs-scope-summary rec=fct-w5cnwd2z@1
name: Send text confined to instance diagnostics
rule: `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
@row_check n=66 row=decision/send-text-confined sha=a378b18c293c v=holds judge=sonnet head=8834195b0683
searched: ObsEvent::SendRefused · in src/run/send.rs:280-292 (wrapper side: no text field)
searched: ObsEvent::SendRefused · in src/cmd/send.rs:239-242 and crates/viola-mcp/src/tools.rs:308-311 (client sides: side, refusal, detail only)
searched: ObsEvent::SendIssued · in src/run/send.rs:193-203 (text_bytes count only)
@row_add n=67 row=decision/hook-fail-open-silent area=1-obs-scope-summary rec=fct-s5xlshqe@1
name: Hook fails open silently
rule: Hook fail-open paths log only to `hook-<name>.ndjson` and exit 0 with stderr empty.
@row_check n=67 row=decision/hook-fail-open-silent sha=94c709a4ba9f v=holds judge=sonnet head=8834195b0683
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/obs.rs:57
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => {
@row_add n=68 row=decision/drift-reports-to-diagnostics area=1-obs-scope-summary rec=fct-s5xlshqe@1
name: Drift reports go to diagnostics only
rule: Hook drift reports are written to `diagnostics/` only.
@row_check n=68 row=decision/drift-reports-to-diagnostics sha=5373284e8605 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:557
quote: obs::write_detail(&instance.home, &instance.name, ObsProcess::Hook, &line);
@row_add n=69 row=decision/stripped-env-never-logged area=1-obs-scope-summary rec=fct-xvmeqbmc@1
name: Stripped CLAUDE env values never logged
rule: No R8-stripped `CLAUDE*` value appears in any sink or fixture; the strip is logged as a count or variable names only.
@row_check n=69 row=decision/stripped-env-never-logged sha=7db836e51c6d v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:57-58
quote: env_stripped_count = strip.count(), env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
searched: removed_names · in src/run/mod.rs:49 (names only)
@row_add n=70 row=decision/serde-errors-fixed-message area=1-obs-scope-summary rec=fct-xvmeqbmc@1
name: Serde error chains become a fixed message
rule: A serde-sourced anyhow error chain is replaced by a fixed message before it reaches stderr.
@row_check n=70 row=decision/serde-errors-fixed-message sha=bcff971f446a v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=71 row=decision/refusal-telemetry area=1-obs-scope-summary rec=fct-wrd7y3z2@2
name: Refusal telemetry for filesystem refusals
rule: Pinned-exe hash mismatches and ignored symlinks produce refusal telemetry.
@row_check n=71 row=decision/refusal-telemetry sha=e25e6704b144 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
at: crates/viola-state/src/board.rs:310
quote: ("symlink-ignored", scan.links_ignored),
@row_add n=72 row=decision/strict-modes-failure-lines area=1-obs-scope-summary rec=fct-wrd7y3z2@2
name: Where a strict-modes failure is logged
rule: A strict-modes failure has a log line only on the `ledger-stamps` read in the `run` version gate and on `verify`'s own `process-exit` for a refused stamp write.
@row_check n=72 row=decision/strict-modes-failure-lines sha=50ffb964b547 v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:204
quote: Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")),
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=73 row=decision/refused-diagnostics-drops-line area=1-obs-scope-summary rec=fct-wrd7y3z2@2
name: Refused diagnostics file drops its line
rule: A refused `diagnostics/` folder or file is treated as a failed open and its log line is dropped.
@row_check n=73 row=decision/refused-diagnostics-drops-line sha=7ec2187714a1 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:328
quote: if check_diagnostics(dir, &path).is_err() || ensure_private_dir(dir).is_err() {
@row_add n=74 row=contract/process-exit-detail/strict-modes-failed area=1-obs-scope-summary rec=fct-wrd7y3z2@2
boundary: process-exit detail codes
item: strict-modes-failed
promise: Carried by `verify`'s `process-exit` for a refused stamp write, with exit 1.
@row_check n=74 row=contract/process-exit-detail/strict-modes-failed sha=48b789f8f956 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
at: src/cmd/verify.rs:81
quote: Ended::Refused(detail) => (1, Some(detail)),
@row_add n=75 row=contract/process-exit-detail/socket-dir-refused area=1-obs-scope-summary rec=fct-wrd7y3z2@2
boundary: process-exit detail codes
item: socket-dir-refused
promise: Carried by the line of a refused socket directory, where it has one; a refused socket directory is no strict-modes failure.
@row_check n=75 row=contract/process-exit-detail/socket-dir-refused sha=d9f1785b82ac v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:195
quote: Err(CallError::SocketDirRefused) => return Ok(Err(Unreachable::SocketDir)),
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=76 row=decision/parse-failures-unknown-counter area=1-obs-scope-summary rec=fct-6ciuuorn@1
name: Parse failures surface as unknown plus a counter
rule: A child-output parse failure surfaces as `unknown` plus a counter.
@row_check n=76 row=decision/parse-failures-unknown-counter sha=8e7ce7401c34 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:78
quote: let list = agents::parse(&ran.stdout).map_err(|failure| {
at: src/run/agents.rs:81
quote: ObsEvent::ParseRejected,
at: crates/viola-state/src/board.rs:291
quote: status: SessionStatus::Unknown,
@row_add n=77 row=decision/escape-controls-in-terminal area=1-obs-scope-summary rec=fct-6ciuuorn@1
name: Control characters escaped in terminal rendering
rule: C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
@row_check n=77 row=decision/escape-controls-in-terminal sha=a1af6ec63df2 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:23
quote: if c.is_control() {
at: src/human/escape.rs:8
quote: if c.is_control() && c != '\n' && c != '\t' {
@row_add n=78 row=decision/single-file-sink-no-os-exporter area=1-obs-scope-summary rec=fct-dao4nifd@2
name: One file sink and one line schema
rule: Telemetry on Linux goes to one file sink with one line schema, with no OS-specific exporter.
@row_check n=78 row=decision/single-file-sink-no-os-exporter sha=8e677f62f080 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry|syslog|journald|eventlog|oslog|tracing-appender · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs (no match)
searched: set_global_default · in src/obs.rs:145 (the one subscriber, writing to the role file only)
@row_add n=79 row=decision/event-log-never-rotated area=1-obs-scope-summary rec=fct-6xh2aqem@1
name: Event log never rotated
rule: The events.ndjson product event log is never rotated; its retention is the whole lifetime of the session home.
why: byte offsets into the log are cursors
@row_check n=79 row=decision/event-log-never-rotated sha=ca150c2db6fc v=holds judge=sonnet head=8834195b0683
searched: rotat · in src/**/*.rs
searched: rotat · in crates/**/*.rs
@row_add n=80 row=decision/ndjson-logs-on-disk area=1-obs-scope-summary rec=fct-rjdprqrn@1
name: Bridge logs as ndjson on disk
rule: The bridge's logs are ndjson on disk, and the event log serves as the audit trail.
why: logs on disk survive restarts on either side
@row_check n=80 row=decision/ndjson-logs-on-disk sha=a3ab9ee46e50 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:92
quote: Ok(Arc::new(open_private_append(&path)?))
at: crates/viola-state/src/fs.rs:46
quote: options.create(true).append(true);
@row_add n=81 row=contract/process-log-event/release-from-driver area=1-obs-scope-summary rec=fct-y6qmqcu2@1
boundary: the process log event field
item: release-from-driver
promise: Release from the driver is logged as its own distinct event.
@row_check n=81 row=contract/process-log-event/release-from-driver sha=9e33957b1830 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:240
quote: ObsEvent::ReleaseFromDriver,
@row_add n=82 row=decision/no-compliance-audit-trail area=1-obs-scope-summary rec=fct-vqjna3cn@1,fct-i24bcxka@1
name: No compliance audit trail
rule: Observability keeps no compliance audit trail; events.ndjson is the product audit stream, not a compliance trail.
why: the security tier is Minimal and no compliance triggers apply
@row_check n=82 row=decision/no-compliance-audit-trail sha=ee4ce4c57225 v=holds judge=sonnet head=8834195b0683
searched: compliance · in src/**/*.rs
searched: compliance · in crates/**/*.rs
@row_add n=83 row=decision/normal-mode-not-inverted area=1-obs-scope-summary rec=fct-x2cjy5mn@1
name: Normal observability mode, not inverted
rule: Viola is not a telemetry product: observability runs in normal mode, and events.ndjson is a product audit log, not a telemetry-collector function.
@row_check n=83 row=decision/normal-mode-not-inverted sha=a9e3c89eeb2d v=holds judge=sonnet head=8834195b0683
searched: opentelemetry|otlp · in Cargo.toml
searched: opentelemetry|otlp|tonic|prost · in src/**/*.rs and crates/**/*.rs
@row_add n=84 row=decision/standard-obs-tier area=2-telemetry-strategy rec=fct-mxljj435@1
name: Standard telemetry tier with Minimal exporter carve-outs
rule: Telemetry follows the Standard signal pyramid, with Minimal-tier carve-outs for exporters.
@row_check n=84 row=decision/standard-obs-tier sha=36bf6ec08bc2 v=holds judge=sonnet head=8834195b0683
at: deny.toml:32
quote: { crate = "opentelemetry-otlp", reason = "no telemetry exporter or egress in v1" },
at: src/obs.rs:120-121
quote: let layer = tracing_subscriber::fmt::layer() .json()
@row_add n=85 row=overview/2-telemetry-strategy area=2-telemetry-strategy rec=fct-i24bcxka@1
text: Structured logs are the primary signal, one JSON-per-line file per process role; spans are declared in code but not serialized in v1, and cross-process joins use corr and conn; errors have no external reporter; metrics have no in-process instruments and no exporter and are derived by jq over the ndjson; events.ndjson is the product audit stream, not a compliance trail, and obs never copies event payloads into process logs.
@row_check n=85 row=overview/2-telemetry-strategy sha=717b1d155454 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:120-121
quote: let layer = tracing_subscriber::fmt::layer() .json()
at: src/obs.rs:123-124
quote: .with_current_span(false) .with_span_list(false)
at: src/run/send.rs:198
quote: conn = self.call.conn,
at: deny.toml:34
quote: { crate = "sentry", reason = "no error reporter or egress in v1" },
@row_add n=86 row=decision/spans-not-serialized-v1 area=2-telemetry-strategy rec=fct-i24bcxka@1
name: Spans declared but not serialized
rule: Spans are declared in code but not serialized in v1.
@row_check n=86 row=decision/spans-not-serialized-v1 sha=52b1c202daaa v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:123-124
quote: .with_current_span(false) .with_span_list(false)
@row_add n=87 row=decision/metrics-derived-by-jq area=2-telemetry-strategy rec=fct-i24bcxka@1
name: Metrics derived by jq, no instruments
rule: Metrics have no in-process instruments and no exporter; they are derived by jq over the ndjson logs.
@row_check n=87 row=decision/metrics-derived-by-jq sha=471f7813875f v=holds judge=sonnet head=8834195b0683
searched: metrics · in Cargo.toml
searched: opentelemetry|prometheus · in {src,crates,tests,scripts,schemas,.github,.config}/**
searched: opentelemetry-otlp banned · in deny.toml:32
@row_add n=88 row=decision/machine-parseable-signals area=2-telemetry-strategy rec=fct-irnyaxwz@1
name: Every signal machine-parseable
rule: Every signal produces machine-parseable output: JSON-per-line process logs, the ndjson product event stream, JSON harness documents, hyperfine JSON for perf gates and nextest JUnit XML in CI.
@row_check n=88 row=decision/machine-parseable-signals sha=eb488909f832 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:121
quote: .json()
at: .config/nextest.toml:7
quote: path = "junit.xml"
at: crates/viola-e2e/src/harness/run/perf.rs:117
quote: .arg("--export-json")
at: crates/viola-e2e/src/harness/status.rs:57
quote: .args(["list", "--json"])
@row_add n=89 row=decision/no-human-only-dashboards area=2-telemetry-strategy rec=fct-dpql65fn@1
name: No human-only dashboards
rule: No Grafana, Kibana, Datadog or other human-only dashboard UI exists in the stack, and none may be added as a consumption surface.
@row_check n=89 row=decision/no-human-only-dashboards sha=955d0cca6b3e v=holds judge=sonnet head=8834195b0683
searched: grafana|kibana|datadog|prometheus · in {src,crates,tests,scripts,schemas,.github,.config}/**
@row_add n=90 row=decision/no-proprietary-apm-sdk area=2-telemetry-strategy rec=fct-7dfhzxv4@1
name: No proprietary APM SDK
rule: No proprietary APM SDK is used, as primary or at all; the v1 exporter role is filled by the local file sink.
@row_check n=90 row=decision/no-proprietary-apm-sdk sha=dab2860e4326 v=holds judge=sonnet head=8834195b0683
searched: datadog|newrelic|new_relic|appdynamics|sentry · in {src,crates,tests,scripts,schemas,.github,.config}/** and Cargo.toml
searched: sentry banned · in deny.toml:34
@row_add n=91 row=decision/no-human-gated-log-analysis area=2-telemetry-strategy rec=fct-i3sjkwgk@1
name: No human-review-gated log analysis
rule: Every log verdict is a jq or harness assertion that sets an exit code, never a human review.
@row_check n=91 row=decision/no-human-gated-log-analysis sha=2952d04f9b6a v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=92 row=decision/one-line-panic-record area=2-telemetry-strategy rec=fct-z3yuahd4@1
name: One structured line per panic
rule: Every panic after init step 4 produces exactly one single-line structured record in its home-level role file with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file.
except: the bounded panic exemptions listed in the plan's Section 10
@row_check n=92 row=decision/one-line-panic-record sha=ac3dab8872e5 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:191
quote: let _ = (&*sink.file).write_all(line.as_bytes());
at: src/main.rs:201
quote: obs::write_detail(&sink.home, instance, sink.process, &detail);
@row_add n=93 row=decision/corr-conn-correlation area=2-telemetry-strategy rec=fct-pcsvls55@1
name: Correlation through corr and conn
rule: Every cross-surface call carries correlation through the corr field plus the additive conn discriminator.
@row_check n=93 row=decision/corr-conn-correlation sha=e6e1fc1fe793 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:198
quote: conn = self.call.conn,
at: src/run/wheel.rs:242
quote: conn = call.conn,
@row_add n=94 row=decision/span-names-not-wire-contract area=2-telemetry-strategy rec=fct-gyyrakol@1
name: Span names are code-internal
rule: Span names are code-internal and not part of any wire contract.
@row_check n=94 row=decision/span-names-not-wire-contract sha=bfea765c6f79 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:124
quote: .with_span_list(false)
@row_add n=95 row=contract/process-log-line/field-names area=2-telemetry-strategy rec=fct-rcqby6f3@1
boundary: the process log line
item: field names
promise: Field names are snake_case.
@row_check n=95 row=contract/process-log-line/field-names sha=beaf95e8f0fb v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:33
quote: service_name = SERVICE_NAME,
@row_add n=96 row=contract/process-log-line/enum-values area=2-telemetry-strategy rec=fct-rcqby6f3@1
boundary: the process log line
item: enum values
promise: Enum values are kebab-case.
@row_check n=96 row=contract/process-log-line/enum-values sha=247417e65bdf v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:51
quote: Self::ChannelRequest => "channel-request",
@row_add n=97 row=contract/process-log-line/at-instants area=2-telemetry-strategy rec=fct-rcqby6f3@1
boundary: the process log line
item: *_at
promise: Instants are named *_at and written in RFC 3339 UTC with milliseconds and a Z suffix.
@row_check n=97 row=contract/process-log-line/at-instants sha=0d0776a9978c v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:96
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
at: src/cmd/hook.rs:175
quote: invoked_at = invoked_at.as_str(),
@row_add n=98 row=contract/process-log-line/duration-ms area=2-telemetry-strategy rec=fct-rcqby6f3@1
boundary: the process log line
item: duration_ms
promise: Durations are duration_ms as an integer.
@row_check n=98 row=contract/process-log-line/duration-ms sha=2a68e6d030c1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:465
quote: let duration_ms = u64::try_from(self.started.elapsed().as_millis()).unwrap_or(u64::MAX);
@row_add n=99 row=contract/process-start-event/service-name area=2-telemetry-strategy rec=fct-xd7arejm@1
boundary: the process-start event
item: service_name
promise: Emitted once per process as the fixed value "viola", the binary name.
@row_check n=99 row=contract/process-start-event/service-name sha=66db660b0ec7 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:33
quote: service_name = SERVICE_NAME,
at: crates/viola-core/src/lib.rs:7
quote: pub const SERVICE_NAME: &str = "viola";
@row_add n=100 row=contract/process-start-event/version area=2-telemetry-strategy rec=fct-xd7arejm@1
boundary: the process-start event
item: version
promise: Emitted once per process as the crate package version.
@row_check n=100 row=contract/process-start-event/version sha=e93ea78b7f95 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:34
quote: version = VERSION,
at: crates/viola-core/src/lib.rs:8
quote: pub const VERSION: &str = env!("CARGO_PKG_VERSION");
@row_add n=101 row=contract/hook-output/stdout area=3-observability-harness-contract rec=fct-bemloa4f@1
boundary: hook output streams
item: stdout
promise: A hook writes only its decision body to stdout, and no telemetry reaches stdout.
@row_check n=101 row=contract/hook-output/stdout sha=79dc04766af8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:298
quote: out.write_all(body.as_bytes())?;
at: src/cmd/hook/statusline.rs:55
quote: let out = &mut std::io::stdout().lock();
@row_add n=102 row=contract/hook-output/stderr area=3-observability-harness-contract rec=fct-bemloa4f@1
boundary: hook output streams
item: stderr
promise: A hook writes nothing to stderr.
@row_check n=102 row=contract/hook-output/stderr sha=18850fb68c07 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
at: src/cmd/hook/statusline.rs:228
quote: .stderr(Stdio::null())
@row_add n=103 row=contract/hook-output/statusline-stdout area=3-observability-harness-contract rec=fct-bemloa4f@1
boundary: hook output streams
item: viola hook statusline stdout
promise: Carries the user's status-line command output byte for byte; nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden --capture arm.
@row_check n=103 row=contract/hook-output/statusline-stdout sha=6777db3bacbc v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:43
quote: return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:91-96
quote: let Some(dir) = &args.capture else { return statusline::statusline(started); }; let shell = args.shell.as_deref(); capture_statusline(dir, shell, &mut std::io::stdin().lock()); return Ok(ExitCode::SUCCESS);
@row_add n=104 row=contract/run-terminal-output/child-output area=3-observability-harness-contract rec=fct-26ufyipy@2
boundary: run terminal output
item: child output
promise: run writes nothing to the terminal except the child's own output; while the child runs, neither run nor a revived start writes anything of its own.
@row_check n=104 row=contract/run-terminal-output/child-output sha=21f53a36f705 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
at: src/cmd/run.rs:716
quote: human::refuse(&format!("{name} is already live"), "viola list");
@row_add n=105 row=contract/run-terminal-output/start-refusal area=3-observability-harness-contract rec=fct-26ufyipy@2
boundary: run terminal output
item: start refusal
promise: A start refusal before any child is spawned writes exactly two fixed stderr lines with no path or pid: the home pair for a home-level refusal, the instance pair for an instance-level one.
@row_check n=105 row=contract/run-terminal-output/start-refusal sha=e86a8fc2716a v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
at: src/cmd/run.rs:212
quote: human::refuse(unable, hint);
at: src/cmd/run.rs:216
quote: human::refuse(&unable, &hint);
@row_add n=106 row=contract/revive-list-stderr/refusal-pair area=3-observability-harness-contract rec=fct-26ufyipy@2
boundary: viola revive --list stderr
item: refusal pair
promise: viola revive --list writes the refusal pair for its two refusals and nothing else on stderr.
@row_check n=106 row=contract/revive-list-stderr/refusal-pair sha=bbfa3764fd78 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:273
quote: Refusal::StrictModes.write(name, json);
at: src/cmd/revive.rs:283
quote: Refusal::NoSession.write(name, json);
at: src/cmd/revive.rs:110
quote: human::refuse(&unable, &hint);
@row_add n=107 row=contract/list-stderr/unable-hint-pair area=3-observability-harness-contract rec=fct-26ufyipy@2
boundary: viola list stderr
item: unable / hint pair
promise: The unable / hint: pair carries no SGR colour codes.
@row_check n=107 row=contract/list-stderr/unable-hint-pair sha=09f390329012 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
at: src/cmd/list.rs:106
quote: human::refuse(unable, hint);
@row_add n=108 row=decision/codes-only-process-logs area=3-observability-harness-contract rec=fct-k3pbv5ix@1
name: Codes-only process logs, content in detail files
rule: Every role's codes-only process log goes to its home-level role file under diagnostics, and content-bearing detail goes only to the instance's per-role detail file.
@row_check n=108 row=decision/codes-only-process-logs sha=bb5737d0c485 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:87-88
quote: let dir = home.join("diagnostics"); let path = dir.join(name);
at: src/obs.rs:291-292
quote: .join("diagnostics") .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=109 row=contract/mcp-output/stdout area=3-observability-harness-contract rec=fct-jujyptfd@1
boundary: mcp output streams
item: stdout
promise: mcp writes only MCP frames to stdout.
@row_check n=109 row=contract/mcp-output/stdout sha=f3973974cc67 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:76
quote: let (stdin, stdout) = rmcp::transport::stdio();
searched: println!|print!|io::stdout|stdout() · in src/cmd/mcp.rs and crates/viola-mcp/src
@row_add n=110 row=decision/short-lived-verbs-stderr area=3-observability-harness-contract rec=fct-3hlujd4h@1
name: Short-lived verbs use stderr for humans
rule: Short-lived CLI verbs may use stderr for their human output.
@row_check n=110 row=decision/short-lived-verbs-stderr sha=aa09ca0747a9 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:43
quote: let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
@row_add n=111 row=decision/obs-owns-logger-and-format area=3-observability-harness-contract rec=fct-adr52q64@1
name: Obs owns the logger and format
rule: The logger and the log format are owned by observability.
@row_check n=111 row=decision/obs-owns-logger-and-format sha=fe28e22e9572 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:122
quote: .flatten_event(true)
at: src/obs.rs:135
quote: pub(crate) fn viola_obs_init(
@row_add n=112 row=decision/static-span-names area=4-span-trace-coverage rec=fct-osius4ib@1
name: Static span names
rule: Span names are static strings; instance names, dialog ids and paths never appear in a span name.
@row_check n=112 row=decision/static-span-names sha=15f5f46bad32 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:116
quote: #[instrument(skip_all, name = "cli.list", fields(json = json))]
at: crates/viola-channel/src/server.rs:230
quote: #[instrument(skip_all, name = "channel.dispatch", fields(method = method, conn = peer.label()))]
@row_add n=113 row=decision/span-field-allow-list area=4-span-trace-coverage rec=fct-osius4ib@1
name: Spans declared with explicit field allow-list
rule: Every span is declared skipping all arguments with an explicit field allow-list.
@row_check n=113 row=decision/span-field-allow-list sha=4ca63cd7735c v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:163
quote: #[instrument(skip_all, name = "hook.handle", fields(hook_event = event.as_str()))]
at: crates/viola-pty/src/lib.rs:125-129
quote: #[tracing::instrument(     skip_all,     name = "pty.spawn",     fields(pty_backend = pty_backend(), env_stripped_count = spec.env_remove.len()) )]
@row_add n=114 row=decision/no-err-ret-span-options area=4-span-trace-coverage rec=fct-osius4ib@1
name: err and ret span options banned
rule: The err and ret instrumentation options are banned on spans.
@row_check n=114 row=decision/no-err-ret-span-options sha=217f08697192 v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:116
quote: #[instrument(skip_all, name = "cli.list", fields(json = json))]
at: crates/viola-channel/src/server.rs:230
quote: #[instrument(skip_all, name = "channel.dispatch", fields(method = method, conn = peer.label()))]
@row_add n=115 row=decision/span-kinds-not-emitted-v1 area=4-span-trace-coverage rec=fct-7n4mjpbn@1
name: Span kinds recorded but not emitted
rule: Span kinds are recorded in the plan per span but not emitted in v1; if the deferred tracing-opentelemetry bridge is adopted, they map to the otel.kind span field.
@row_check n=115 row=decision/span-kinds-not-emitted-v1 sha=22c3f89cc904 v=holds judge=sonnet head=8834195b0683
searched: otel.kind · in src/**, crates/**, schemas/**
searched: tracing-opentelemetry · in every Cargo.toml
searched: SpanKind · in src/**, crates/**
@row_add n=116 row=decision/manual-spans-only area=4-span-trace-coverage rec=fct-4sjmn35v@1
name: Manual spans only
rule: Every span is written by hand; no surface uses an auto-instrumentation library.
@row_check n=116 row=decision/manual-spans-only sha=70cc85c47eb8 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:242
quote: tracing = { version = "=0.1.44", default-features = false, features = ["attributes", "std"] }
searched: ^(tracing|opentelemetry)[a-z_-]*\s*= · in every Cargo.toml (only tracing and tracing-subscriber found)
@row_add n=117 row=decision/dependency-targets-off-in-sink area=4-span-trace-coverage rec=fct-4sjmn35v@1
name: rmcp and portable-pty targets off
rule: The rmcp and portable-pty log targets are switched OFF in the sink.
@row_check n=117 row=decision/dependency-targets-off-in-sink sha=0c6479370639 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:110
quote: .fold(Targets::new().with_default(LevelFilter::OFF), |t, name| {
at: src/obs.rs:22-30
quote: const VIOLA_TARGETS: [&str; 7] = [     "viola",     "viola_core",     "viola_pty",     "viola_channel",     "viola_state",     "viola_agent_claude",     "viola_mcp", ];
@row_add n=118 row=decision/channel-lines-no-params-or-results area=4-span-trace-coverage rec=fct-4sjmn35v@1
name: Channel lines carry no params or results
rule: Channel log lines never record request params or result bodies.
@row_check n=118 row=decision/channel-lines-no-params-or-results sha=3542b1f7d7aa v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:367
quote: ObsEvent::ChannelRequest,
at: crates/viola-channel/src/server.rs:475
quote: ObsEvent::ChannelResponse,
@row_add n=119 row=decision/uninstrumented-components area=4-span-trace-coverage rec=fct-vjo2uyhn@1
name: Components left uninstrumented
rule: viola-core, config.json and plugin files, and build/CI infrastructure are not instrumented.
why: viola-core only supplies ObsEvent and RefusalReason values; the rest are not instrumentable.
@row_check n=119 row=decision/uninstrumented-components sha=347d6aa8a547 v=holds judge=sonnet head=8834195b0683
searched: instrument|info_span|span!( · in crates/viola-core/src/**
searched: instrument · in config.json handling is src/obs.rs read only (no span in read_diagnostics_level), build files .github/workflows/ci.yml carry no span
@row_add n=120 row=decision/no-trace-and-span-ids area=4-span-trace-coverage rec=fct-qh2p5n6e@2,fct-wlvn44lx@1,fct-xbpjnox3@1
name: No trace_id or span_id
rule: Log lines never emit trace_id or span_id; correlation is by corr plus conn.
why: No W3C trace context exists, and spans are not attached to lines.
@row_check n=120 row=decision/no-trace-and-span-ids sha=790858befebd v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:123
quote: .with_current_span(false)
searched: trace_id|span_id · in src/**, crates/**, schemas/**
@row_add n=121 row=absence/web-or-graphical-frontend area=4-span-trace-coverage rec=fct-qh2p5n6e@2
capability: A web or graphical frontend
@row_check n=121 row=absence/web-or-graphical-frontend sha=6fce34e0d025 v=holds judge=sonnet head=8834195b0683
searched: ^(egui|eframe|iced|tauri|axum|actix|hyper|warp|rocket|leptos|yew|winit|wry|slint|gtk|druid)[a-z_-]*\s*= · in every Cargo.toml
@row_add n=122 row=decision/no-endpoint-path-or-env-in-spans area=4-span-trace-coverage rec=fct-itztmsse@2
name: No endpoint, binary path or env values in spans
rule: A span never carries the endpoint name, the pinned_bin path or environment values.
@row_check n=122 row=decision/no-endpoint-path-or-env-in-spans sha=ddae5944f27e v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:61
quote: #[instrument(skip_all, name = "channel.bind", fields(endpoint_kind = ENDPOINT_KIND))]
searched: fields( with endpoint|pinned_bin|env values · in all #[instrument] and info_span sites under src/** and crates/**
@row_add n=123 row=decision/child-settings-never-logged area=4-span-trace-coverage rec=fct-itztmsse@2
name: Child settings argument never logged
rule: The claude child's --settings argument appears in no log line or span.
@row_check n=123 row=decision/child-settings-never-logged sha=c3a771fb4e3e v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:260
quote: OsString::from("--settings"),
searched: "--settings"|settings_arg · in src/**, crates/** (only the launch_args argv build, no obs_event! or span field)
@row_add n=124 row=decision/statusline-command-never-logged area=4-span-trace-coverage rec=fct-itztmsse@2,fct-rwzldnx4@1
name: Statusline command never logged
rule: The statusline command and its source path appear in no log line or span.
@row_check n=124 row=decision/statusline-command-never-logged sha=54e9618ea250 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:165
quote: detail = detail,
searched: statusline_command · in obs_event!/span field lists in src/**, crates/** (only struct fields and the snapshot read at statusline.rs:130, never a log field)
@row_add n=125 row=decision/detail-codes-only area=4-span-trace-coverage rec=fct-itztmsse@2,fct-7pburmsm@1
name: detail is a closed code
rule: The detail field is a closed kebab-case code catalog and never carries a path or pid.
@row_check n=125 row=decision/detail-codes-only sha=480a001ebb4b v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:77
quote: "detail": { "enum": ["already-live", "squatted-name", "pinned-hash-mismatch", "internal-error", "instance-dead", "strict-modes-failed", "server-verify-failed", "socket-dir-refused", "wrapper-fault", "cwd-missing", "no-session", "interrupted", "session-live", "stale-heartbea
at: crates/viola-core/src/lib.rs:375
quote: pub fn is_code_word(word: &str) -> bool {
@row_add n=126 row=decision/sent-text-never-logged area=4-span-trace-coverage rec=fct-o5zzhzdu@1
name: Sent text never logged
rule: A sent message's text is never logged; only its size in text_bytes is.
@row_check n=126 row=decision/sent-text-never-logged sha=1cb748705587 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:202
quote: text_bytes = u64::try_from(text_bytes).unwrap_or(u64::MAX),
@row_add n=127 row=decision/no-assistant-message-in-logs area=4-span-trace-coverage rec=fct-q67chfay@2
name: No assistant message or result body in logs
rule: last_assistant_message never appears in a log, and no result body reaches a log line.
@row_check n=127 row=decision/no-assistant-message-in-logs sha=cb67727be00a v=holds judge=sonnet head=8834195b0683
searched: last_assistant_message · in obs_event! calls under src/** and crates/** (occurrences are only in json! reply bodies, src/run/wait.rs:177 and src/run/send/slot.rs:393, never a log field)
searched: last_assistant_message · in crates/viola-channel/src/server.rs (the ChannelResponse obs_event! at lines 473-485 carries only result_class, error_code, detail, outcome)
@row_add n=128 row=decision/mid-call-disconnect-logged-once area=4-span-trace-coverage rec=fct-q67chfay@2
name: Mid-call disconnect logged once
rule: A disconnect in the middle of a call is logged once, by the client.
@row_check n=128 row=decision/mid-call-disconnect-logged-once sha=ed5be8967a8a v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => { return Ok(Err(Unreachable::Lost));
at: src/cmd/client.rs:136
quote: Self::Lost => "call",
at: crates/viola-channel/src/client.rs:217
quote: (method == Some("wait") && vanished).then_some("instance-unreachable")
@row_add n=129 row=contract/cli-wait-last-connect/server-verify-failed area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: CLI wait / last connect refusal
item: server-verify-failed
promise: A serving process that is not the recorded wrapper or runs under another euid ends the call before any frame with exit 21 and one WARN process-exit line (subject self, during connect) inside the dispatch span, and no channel-request line.
@row_check n=129 row=contract/cli-wait-last-connect/server-verify-failed sha=40e67618efec v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:256-264
quote: obs_event!( WARN, ObsEvent::ProcessExit, subject = "self", exit_code = 21u8, detail = cause.detail(), during = cause.during(), duration_ms = obs::duration_ms(), );
@row_add n=130 row=contract/cli-wait-last-connect/socket-dir-refused area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: CLI wait / last connect refusal
item: socket-dir-refused
promise: A refused socket directory, checked before the server comparison, ends the call before any frame with exit 21 and one WARN process-exit line (during connect), and no channel-request line.
@row_check n=130 row=contract/cli-wait-last-connect/socket-dir-refused sha=11958b0445f1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=131 row=contract/cli-wait-last-connect/strict-modes-failed area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: CLI wait / last connect refusal
item: strict-modes-failed
promise: A home or instance tree refused by the instance check, checked first, ends the call before any role file opens with exit 21 and no line; under --json the output is exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_check n=131 row=contract/cli-wait-last-connect/strict-modes-failed sha=8cb58f7e063f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=132 row=contract/mcp-wait-last-connect/server-verify-failed area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: MCP wait / last connect refusal
item: server-verify-failed
promise: A serving process that is not the recorded wrapper or runs under another euid returns instance-unreachable with detail server-verify-failed and writes no line.
@row_check n=132 row=contract/mcp-wait-last-connect/server-verify-failed sha=c4d31ee301f8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:352-353
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? { return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=133 row=contract/mcp-wait-last-connect/socket-dir-refused area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: MCP wait / last connect refusal
item: socket-dir-refused
promise: A refused socket directory returns instance-unreachable with detail socket-dir-refused and writes no line.
@row_check n=133 row=contract/mcp-wait-last-connect/socket-dir-refused sha=d7ad9904e0a0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=134 row=contract/mcp-wait-last-connect/strict-modes-failed area=4-span-trace-coverage rec=fct-q67chfay@2
boundary: MCP wait / last connect refusal
item: strict-modes-failed
promise: A home or instance tree refused by the instance check returns instance-unreachable with detail strict-modes-failed and writes no line.
@row_check n=134 row=contract/mcp-wait-last-connect/strict-modes-failed sha=d917eb63bfb3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-340
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=135 row=decision/unwrapped-hook-no-op area=4-span-trace-coverage rec=fct-hxgn7hhb@1,fct-wa67dogf@1
name: Unwrapped hook is a no-op
rule: A hook run without VIOLA_NAME writes no file and no line, exits 0 and leaves stdout and stderr empty.
guard: edge test E1, asserted by absence
@row_check n=135 row=decision/unwrapped-hook-no-op sha=1f22bb112420 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
at: src/cmd/mod.rs:256
quote: let name = ViolaName::try_new(name?.into_string().ok()?).ok()?;
guard_at: tests/hook_fail_open.rs:283
@row_add n=136 row=contract/claude-child-process-start/env-stripped-count area=4-span-trace-coverage rec=fct-xnfonqro@1,fct-wa67dogf@1
boundary: process-start line of the claude child
item: env_stripped_count
promise: Reports the R8 environment strip on process-start{subject:"claude-child"}; never an environment value.
@row_check n=136 row=contract/claude-child-process-start/env-stripped-count sha=a8fbe373c3ab v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:57
quote: env_stripped_count = strip.count(),
@row_add n=137 row=contract/claude-child-process-start/env-stripped-known area=4-span-trace-coverage rec=fct-xnfonqro@1,fct-wa67dogf@1
boundary: process-start line of the claude child
item: env_stripped_known
promise: Reports the R8 environment strip on process-start{subject:"claude-child"} by variable names only, never values.
@row_check n=137 row=contract/claude-child-process-start/env-stripped-known sha=448e244a3afa v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:58
quote: env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
@row_add n=138 row=contract/claude-child-process-start/env-kept area=4-span-trace-coverage rec=fct-xnfonqro@1,fct-wa67dogf@1
boundary: process-start line of the claude child
item: env_kept
promise: Reports the variables kept after the R8 environment strip on process-start{subject:"claude-child"} by names only, never values.
@row_check n=138 row=contract/claude-child-process-start/env-kept sha=5180969f0741 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:59
quote: env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()),
@row_add n=139 row=decision/link-has-no-own-trace area=4-span-trace-coverage rec=fct-3o7hzpx4@1,fct-wa67dogf@1
name: Link and unlink have no trace of their own
rule: Implicit and explicit link / unlink have no process-log line, event value or span of their own; their trace is the call's channel-request / channel-response pair, the product link / unlink line in events.ndjson and the snapshot's links.
guard: edge test E3
@row_check n=139 row=decision/link-has-no-own-trace sha=e9cf7402636d v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:147-148
quote: "link" => self.links.link(call.params), "unlink" => self.links.unlink(call.params),
searched: link · in crates/viola-core/src/obs.rs (no ObsEvent variant for link or unlink)
searched: instrument|info_span|obs_event|span! · in src/run/links.rs (none)
guard_at: tests/cli_links.rs:329
@row_add n=140 row=decision/link-verbs-log-to-driven-instance area=4-span-trace-coverage rec=fct-3o7hzpx4@1,fct-wa67dogf@1
name: Link verbs log to the driven instance
rule: The viola link and viola unlink verbs log into the driven instance's cli-<driven>.ndjson, their request inside the cli.link / cli.unlink span.
@row_check n=140 row=decision/link-verbs-log-to-driven-instance sha=347f0ba3b4f7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:97
quote: let home = client::start(home, &args.driven)?;
at: src/cmd/client.rs:47
quote: obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
at: src/cmd/link.rs:129
quote: #[instrument(skip_all, name = "cli.link")]
at: src/cmd/unlink.rs:19
quote: #[instrument(skip_all, name = "cli.unlink")]
@row_add n=141 row=contract/state-recovered-detail/snapshot-replayed area=4-span-trace-coverage rec=fct-iyk74prg@1,fct-wa67dogf@1
boundary: state-recovered detail
item: snapshot-replayed
promise: A state-recovered line at WARN carries this detail, one line per snapshot read call.
@row_check n=141 row=contract/state-recovered-detail/snapshot-replayed sha=8814fd94e7b0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:159
quote: ReplayCause::Unreadable => ("snapshot-replayed", None),
@row_add n=142 row=contract/state-recovered-detail/snapshot-unsupported-v area=4-span-trace-coverage rec=fct-iyk74prg@1,fct-wa67dogf@1
boundary: state-recovered detail
item: snapshot-unsupported-v
promise: A state-recovered line at WARN carries this detail, one line per snapshot read call.
@row_check n=142 row=contract/state-recovered-detail/snapshot-unsupported-v sha=b3537fdc2f8d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:160
quote: ReplayCause::Unsupported { v_seen } => ("snapshot-unsupported-v", Some(v_seen)),
@row_add n=143 row=decision/no-endpoint-over-replayed-snapshot area=4-span-trace-coverage rec=fct-iyk74prg@1,fct-wa67dogf@1
name: No endpoint over a replayed snapshot
rule: Over a replayed or absent snapshot read, endpoint lookup and the hook connect give no endpoint, so no frame is sent, and the statusline runs no command.
guard: edge test E5
@row_check n=143 row=decision/no-endpoint-over-replayed-snapshot sha=e98d1417d0e4 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
at: src/cmd/hook/statusline.rs:129-132
quote: let command = match read_snapshot_or_replay(&instance.dir) { Ok(Recovered::Snapshot(snapshot)) => snapshot.statusline_command, _ => None, };
guard_at: crates/viola-state/src/liveness.rs:267
@row_add n=144 row=contract/state-recovered-detail/torn-line-healed area=4-span-trace-coverage rec=fct-ghtvc6rx@1
boundary: state-recovered detail
item: torn-line-healed
promise: The appender that heals a torn line writes one state-recovered line with this detail per heal.
@row_check n=144 row=contract/state-recovered-detail/torn-line-healed sha=586673a87cf5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:339
quote: detail = "torn-line-healed",
@row_add n=145 row=decision/reader-never-heals-torn-lines area=4-span-trace-coverage rec=fct-ghtvc6rx@1
name: Reader never heals torn lines
rule: A reader of an append log neither heals a torn line nor logs one; only the appender heals.
@row_check n=145 row=decision/reader-never-heals-torn-lines sha=f3b21349f78b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:145
quote: /// torn, neither returned nor rewritten, and an absent log reads as no lines.
at: crates/viola-state/src/events.rs:335-339
quote: file.write_all(&[b"\n", bytes].concat())?; obs_event!( WARN, ObsEvent::StateRecovered, detail = "torn-line-healed",
@row_add n=146 row=decision/unreadable-ledger-is-unverified area=4-span-trace-coverage rec=fct-g4daqt4h@1
name: Unreadable ledger counts as unverified
rule: A missing stamp, or an unreadable, malformed or strict-modes-refused capability ledger, counts as unverified (cli_verified false).
@row_check n=146 row=decision/unreadable-ledger-is-unverified sha=bd947004f7f8 v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:203-208
quote: Ok(None) => (false, None), Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")), Err(StampsReadError::State(_)) => (false, Some("unreadable")), Ok(Some(bytes)) => match verified(&bytes, version) { Ok(verified) => (verified, None), Err(_) => (false, Some("malformed")),
@row_add n=147 row=decision/verify-logs-only-with-instance area=4-span-trace-coverage rec=fct-bharnesn@1
name: verify and plugin install log only inside an instance
rule: With VIOLA_NAME set, verify and plugin install log process-start / process-exit to cli-<name>.ndjson; from a plain terminal with no instance they write no process-log file.
@row_check n=147 row=decision/verify-logs-only-with-instance sha=fb5c92f84d91 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:71-74
quote: if let Some(name) = instance { let (level, rejection) = obs::read_diagnostics_level(&home); obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?; run::log_self_start();
at: src/cmd/verify.rs:90-91
quote: if instance.is_some() { run::log_self_exit(code, detail);
at: src/cmd/plugin.rs:62-63
quote: if let Some(name) = client::own_name() { client::start(&home, &name)?;
@row_add n=148 row=decision/capture-arms-uninstrumented area=4-span-trace-coverage rec=fct-bharnesn@1
name: Hidden capture arms uninstrumented
rule: The two hidden capture arms of verify / plugin install are uninstrumented by design.
@row_check n=148 row=decision/capture-arms-uninstrumented sha=b3baae9e5531 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:91-96
quote: let Some(dir) = &args.capture else { return statusline::statusline(started); }; let shell = args.shell.as_deref(); capture_statusline(dir, shell, &mut std::io::stdin().lock()); return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:101-107
quote: if let Some(dir) = &args.capture { let filed = capture(event, dir, &mut std::io::stdin().lock()); if let (Some(filed), Some(answers)) = (filed, &args.answers) { answer(event, dir, answers, &filed, &mut std::io::stdout().lock()); } return Ok(ExitCode::SUCCESS); }
at: src/cmd/hook.rs:122
quote: let _ = obs::viola_obs_init(
@row_add n=149 row=decision/verify-lines-redacted area=4-span-trace-coverage rec=fct-bharnesn@1
name: verify lines carry no content
rule: No verify or plugin install line carries row text, a prompt, a session id or a path.
@row_check n=149 row=decision/verify-lines-redacted sha=d5f6ced10461 v=holds judge=sonnet head=8834195b0683
searched: obs_event! · in src/cmd/verify.rs (fields: subject, child_exit_status, duration_ms; no row text, prompt, session id or path)
searched: obs_event! · in src/cmd/verify/typed/run.rs (fields: subject, child_exit_status, duration_ms)
searched: obs_event! · in src/cmd/plugin.rs (none; only run::log_self_start / log_self_exit, whose fields are codes and counts)
@row_add n=150 row=decision/ci-verify-fake-agent-only area=4-span-trace-coverage rec=fct-bharnesn@1
name: CI verify against the fake agent only
rule: In CI, viola verify runs only against the fake agent.
@row_check n=150 row=decision/ci-verify-fake-agent-only sha=7bd9d306a946 v=holds judge=sonnet head=8834195b0683
at: tests/support/verify.rs:529
quote: args.extend(["--", FAKE, "--cli-version", version, "--fixtures"].map(OsString::from));
at: crates/viola-e2e/src/harness/boot.rs:269-270
quote: .args(["verify", "--"]) .arg(exe(session_bin, "claude"))
searched: verify · in .github/workflows (no workflow runs viola verify against a real claude)
@row_add n=151 row=contract/viola-verify-refusal/home-tree-refused area=4-span-trace-coverage rec=fct-bharnesn@1
boundary: viola verify strict-modes refusal
item: home tree refused
promise: Before logging, verify holds the home, ledger/, bin/ and plugin/ to the strict-modes check; a refused tree exits 1 with the home pair and writes no line.
@row_check n=151 row=contract/viola-verify-refusal/home-tree-refused sha=eb1254c9d1b0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67
quote: if check_home_tree(&home, None, None).is_err() {
at: src/cmd/verify.rs:68-69
quote: refuse_home(); return Ok(ExitCode::from(1));
@row_add n=152 row=contract/viola-verify-refusal/stamp-write-refused area=4-span-trace-coverage rec=fct-bharnesn@1
boundary: viola verify strict-modes refusal
item: stamp write refused
promise: A ledger/stamps.json the stamp write refuses prints the home pair and exits 1 through verify's own process-exit line.
@row_check n=152 row=contract/viola-verify-refusal/stamp-write-refused sha=77d2269a55ca v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:286-288
quote: Err(StampsError::StrictModes(_)) => { refuse_home(); return Ok(Ended::Refused("strict-modes-failed"));
at: src/cmd/verify.rs:90-91
quote: if instance.is_some() { run::log_self_exit(code, detail);
@row_add n=153 row=decision/metrics-derived-from-logs area=5-metric-coverage rec=fct-kvmjumn6@1,fct-x6ueqkr2@1
name: Metrics derived from logs
rule: Every metric is logical, computed by jq or the harness over the JSON-per-line logs or from hyperfine output, never by OTel or metrics-crate instruments.
why: There is no reader or exporter to flush instruments.
@row_check n=153 row=decision/metrics-derived-from-logs sha=c16aad8738d6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:114
quote: let mut cmd = Command::new("hyperfine");
searched: opentelemetry|metrics|prometheus|statsd (case-insensitive) · in Cargo.toml, **/Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=154 row=decision/metric-grouping-closed-enums area=5-metric-coverage rec=fct-5mkprlwu@1
name: Metric grouping by closed enums only
rule: Aggregate reports group only by closed enums (event, detail, refusal, method, hook_event, subject, parser, process); corr, conn, pid, child_pid and instance are join keys, never grouping keys.
@row_check n=154 row=decision/metric-grouping-closed-enums sha=12191fc498ca v=unverifiable judge=sonnet head=8834195b0683
searched: group_by|GROUP BY|aggregate · in src/**, crates/**, scripts/**, tests/**
why: no evidence found
@row_add n=155 row=decision/log-schema-additive-only area=6-log-coverage rec=fct-wk2jyutc@1
name: Log schema additive to the tests' format
rule: The observability log schema only adds fields and enum values to the tests' log format; it renames and removes nothing.
why: The tests' log format is the binding text.
@row_check n=155 row=decision/log-schema-additive-only sha=9cd57427db33 v=unverifiable judge=sonnet head=8834195b0683
searched: additive · in src/obs.rs and schemas/diag-line.v1.json
why: no evidence found; additivity against the tests' log format is a comparison of two documents, not a line of code
@row_add n=156 row=contract/log-line/timestamp area=6-log-coverage rec=fct-qwey6fxj@1,fct-3i3k7olh@1
boundary: every log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_check n=156 row=contract/log-line/timestamp sha=847d3b6c718a v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:96
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
at: src/obs.rs:127
quote: .with_timer(MillisUtc)
@row_add n=157 row=contract/log-line/level area=6-log-coverage rec=fct-vagpwmim@1,fct-3i3k7olh@1
boundary: every log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR; TRACE is never emitted.
@row_check n=157 row=contract/log-line/level sha=5b0639a5f0cd v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:203
quote: Some(Some("debug")) => (Level::DEBUG, false),
searched: obs_event!(TRACE · in src/**/*.rs and crates/*/src/**/*.rs (none; only src/obs.rs:434 asserts TRACE is not enabled)
@row_add n=158 row=contract/log-line/target area=6-log-coverage rec=fct-324giizr@1,fct-3i3k7olh@1
boundary: every log line
item: target
promise: The Rust module path, for example viola_channel::server.
@row_check n=158 row=contract/log-line/target sha=19f7d7a73ee7 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:120-122
quote: let layer = tracing_subscriber::fmt::layer()         .json()         .flatten_event(true)
@row_add n=159 row=contract/log-line/message area=6-log-coverage rec=fct-gmpqxlxj@1,fct-3i3k7olh@1
boundary: every log line
item: message
promise: Flattened, a static literal equal to the event name.
@row_check n=159 row=contract/log-line/message sha=599bc9420b64 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:149-150
quote: "{}",                 event.as_str()
@row_add n=160 row=contract/log-line/event area=6-log-coverage rec=fct-6pnuxsb3@1,fct-3i3k7olh@1
boundary: every log line
item: event
promise: A value of the closed kebab-case event enum.
@row_check n=160 row=contract/log-line/event sha=8fd225745954 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:145
quote: event = event.as_str(),
@row_add n=161 row=contract/log-line/instance area=6-log-coverage rec=fct-ev4mirl5@1,fct-3i3k7olh@1
boundary: every log line
item: instance
promise: The ViolaName; the key is absent when there is none.
@row_check n=161 row=contract/log-line/instance sha=ac1ad848918d v=stale judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:147
quote: instance = $crate::obs::ctx_instance(),
shows: instance is an Option; when there is no name the key is absent (the macro doc says "an absent value is an absent key"), never a null value.
cell: promise
value: The ViolaName; the key is absent when there is none.
@row_add n=162 row=contract/process-start-line/service-name area=6-log-coverage rec=fct-iee6b2de@1,fct-xbpjnox3@1
boundary: process-start line
item: service_name
promise: A constant service name, emitted once on process-start rather than on every line.
@row_check n=162 row=contract/process-start-line/service-name sha=ad9c6a4338dc v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:33
quote: service_name = SERVICE_NAME,
@row_add n=163 row=decision/no-event-type-field area=6-log-coverage rec=fct-w3dtohlr@1,fct-xbpjnox3@1
name: No event_type field
rule: Log lines carry no event_type field.
why: event already does that job, and a duplicate would amount to a rename.
@row_check n=163 row=decision/no-event-type-field sha=a2c41a58c1a8 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:311
quote: record.insert("event".to_owned(), event.as_str().into());
searched: event_type · in src/**, crates/*/src/**, schemas/**
@row_add n=164 row=decision/additive-field-naming area=6-log-coverage rec=fct-o3exhcyn@1
name: Additive log field naming
rule: Additive log fields are snake_case with kebab-case values, and carry codes only.
@row_check n=164 row=decision/additive-field-naming sha=285dcbdd7f42 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:375
quote: timeout_ms = timeout_ms,
at: crates/viola-core/src/obs.rs:51
quote: Self::ChannelRequest => "channel-request",
@row_add n=165 row=decision/skipped-symlink-never-logged area=6-log-coverage rec=fct-gr4iywnh@1
name: Skipped symlink never named in logs
rule: The entry name and link target of a skipped symlink are never logged.
@row_check n=165 row=decision/skipped-symlink-never-logged sha=8e11937de8df v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:310
quote: ("symlink-ignored", scan.links_ignored),
@row_add n=166 row=contract/log-levels/warn area=6-log-coverage rec=fct-ufkep7p5@1
boundary: log levels
item: warn
promise: WARN carries state-recovered; parse-rejected; hook-decision with a fail-open detail; process-exit with exit 21; channel-response with -32600 or -32602.
@row_check n=166 row=contract/log-levels/warn sha=16d09c49d7e4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:337
quote: WARN,
at: src/cmd/hook.rs:536
quote: ObsEvent::ParseRejected,
at: src/cmd/hook.rs:390
quote: WARN,
at: src/cmd/client.rs:257
quote: WARN,
at: crates/viola-channel/src/server.rs:496
quote: response_line!(WARN)
@row_add n=167 row=contract/log-levels/info area=6-log-coverage rec=fct-remsrxge@1
boundary: log levels
item: info
promise: INFO carries all other boundary, lifecycle and state-transition events, including refusals, which are normal product outcomes.
@row_check n=167 row=contract/log-levels/info sha=6fec1a2ae8bf v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:498
quote: _ => response_line!(INFO),
@row_add n=168 row=contract/log-levels/debug area=6-log-coverage rec=fct-vji36ghd@1
boundary: log levels
item: debug
promise: DEBUG is reserved for per-frame channel framing detail and uses the same field allow-list.
@row_check n=168 row=contract/log-levels/debug sha=4467f8d9ccf2 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:203
quote: Some(Some("debug")) => (Level::DEBUG, false),
searched: obs_event!(DEBUG · in src/**/*.rs and crates/*/src/**/*.rs (only the test at src/obs.rs:797; no product line emits at DEBUG)
@row_add n=169 row=decision/debug-level-by-config-only area=6-log-coverage rec=fct-vji36ghd@1
name: Debug level enabled by config only
rule: The debug log level is enabled only by diagnostics_level "debug" in config.json, never by RUST_LOG.
@row_check n=169 row=decision/debug-level-by-config-only sha=15fa9b061dee v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:203
quote: Some(Some("debug")) => (Level::DEBUG, false),
at: src/obs.rs:180
quote: /// `<home>/config.json` `diagnostics_level` is the only level knob (never `RUST_LOG`); anything
@row_add n=170 row=contract/log-levels/trace area=6-log-coverage rec=fct-c3cqwtsl@1
boundary: log levels
item: trace
promise: TRACE is never emitted; the level exists only in the tests' level enum.
@row_check n=170 row=contract/log-levels/trace sha=e5f71e966e2d v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:434
quote: assert!(!debug.would_enable("viola::run", &Level::TRACE));
searched: obs_event!(TRACE · in src/**/*.rs and crates/*/src/**/*.rs (none)
@row_add n=171 row=decision/viola-crates-log-at-info area=6-log-coverage rec=fct-h3nedfqn@1,fct-ffszh776@1
name: Viola crates log at INFO
rule: The per-module log filter sets the viola, viola_core, viola_pty, viola_channel, viola_state, viola_agent_claude and viola_mcp targets to INFO, or DEBUG via diagnostics_level.
@row_check n=171 row=decision/viola-crates-log-at-info sha=c4ef8046e0ee v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:22-30
quote: const VIOLA_TARGETS: [&str; 7] = [ "viola", "viola_core", "viola_pty", "viola_channel", "viola_state", "viola_agent_claude", "viola_mcp", ];
at: src/obs.rs:111
quote: t.with_target(*name, level)
@row_add n=172 row=decision/third-party-crate-logs-off area=6-log-coverage rec=fct-7e4jvssp@1,fct-ffszh776@1
name: Third-party crate logs off
rule: The per-module log filter turns logging OFF for the rmcp, portable_pty, sysinfo and interprocess targets.
why: Their failures are captured at the viola seam as viola events.
@row_check n=172 row=decision/third-party-crate-logs-off sha=759d04c193ca v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:110
quote: .fold(Targets::new().with_default(LevelFilter::OFF), |t, name| {
@row_add n=173 row=decision/unlisted-targets-log-off area=6-log-coverage rec=fct-qberstet@1,fct-ffszh776@1
name: Unlisted log targets off
rule: The per-module log filter turns logging OFF for every target it does not list.
@row_check n=173 row=decision/unlisted-targets-log-off sha=a8c838727351 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:110
quote: .fold(Targets::new().with_default(LevelFilter::OFF), |t, name| {
@row_add n=174 row=decision/file-only-per-role-log-sinks area=6-log-coverage rec=fct-fdakcnvs@1,fct-xeoybvbw@1
name: File-only log sinks per role
rule: Diagnostics are written to files only, one sink per role, under the home's diagnostics directory.
@row_check n=174 row=decision/file-only-per-role-log-sinks sha=a692cc8725c0 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:87
quote: let dir = home.join("diagnostics");
@row_add n=175 row=decision/content-detail-in-instance-file area=6-log-coverage rec=fct-fdakcnvs@1,fct-xeoybvbw@1
name: Content-bearing detail in the instance detail file
rule: Content-bearing detail goes only to the per-process detail file inside the instance's diagnostics directory.
@row_check n=175 row=decision/content-detail-in-instance-file sha=bbf020ad832a v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:292
quote: .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=176 row=decision/json-to-file-only-log-mode area=6-log-coverage rec=fct-p2v4kjoq@1
name: JSON-to-file as the only log mode
rule: JSON to file is the only log mode: there is no agent-mode flag and no stdout pretty sink on any role.
@row_check n=176 row=decision/json-to-file-only-log-mode sha=ddd274378722 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:128
quote: .with_writer(writer);
searched: agent_mode · in src/**, crates/*/src/**
searched: .pretty() · in src/obs.rs
@row_add n=177 row=decision/ipc-calls-logged-without-arguments area=6-log-coverage rec=fct-qmzbmwv2@1
name: IPC calls logged without arguments
rule: IPC params and result bodies are never recorded, not even as a hash or digest.
@row_check n=177 row=decision/ipc-calls-logged-without-arguments sha=062ec4ce2006 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:371
quote: method = method,
at: crates/viola-channel/src/server.rs:372
quote: sender = params.sender.as_deref().and_then(version_label),
at: crates/viola-channel/src/server.rs:381
quote: pub(crate) fn wait_bounds(method: Option<&str>, params: &Value) -> (Option<u64>, Option<u64>) {
at: crates/viola-channel/src/server.rs:382
quote: if method != Some("wait") {
@row_add n=178 row=decision/euid-refused-peer-writes-no-line area=6-log-coverage rec=fct-qmzbmwv2@1
name: Euid-refused peer writes no line
rule: A channel peer refused by euid is dropped before its worker thread and before any frame, writes no log line, and the client reads end-of-file.
why: There is no frame to key a line on, and channel-response's detail is closed.
@row_check n=178 row=decision/euid-refused-peer-writes-no-line sha=360af4bbb3bc v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:139
quote: /// A refused peer's stream is dropped here, before its thread and before any frame is read: it reads
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
@row_add n=179 row=contract/pty-seam-log/spawn area=6-log-coverage rec=fct-3epowryi@1
boundary: PTY seam log events
item: spawn
promise: A spawn gives process-start{subject:"claude-child"}.
@row_check n=179 row=contract/pty-seam-log/spawn sha=a66a9d6e618d v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:54
quote: subject = "claude-child",
@row_add n=180 row=contract/pty-seam-log/kill-fallback area=6-log-coverage rec=fct-3epowryi@1
boundary: PTY seam log events
item: kill-fallback
promise: An exit through the kill fallback is logged with exit_source:"kill-fallback".
@row_check n=180 row=contract/pty-seam-log/kill-fallback sha=bcb998b84e56 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:54
quote: Self::KillFallback => "kill-fallback",
at: src/run/mod.rs:71
quote: exit_source = exit.source.as_str(),
@row_add n=181 row=contract/pty-seam-log/paste area=6-log-coverage rec=fct-3epowryi@1
boundary: PTY seam log events
item: paste
promise: A paste is logged with text_bytes only.
@row_check n=181 row=contract/pty-seam-log/paste sha=258efe15205a v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:75
quote: fields(text_bytes = text.len(), paste_mode = "bracketed")
@row_add n=182 row=contract/child-spawn-log/subject area=6-log-coverage rec=fct-7eai74no@1
boundary: child and shell spawn log events
item: subject
promise: process-start and process-exit lines for child and shell spawns carry a subject from a closed set.
@row_check n=182 row=contract/child-spawn-log/subject sha=e3305f154f83 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/run/version_gate.rs:149
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "version-probe");
at: src/cmd/hook/statusline.rs:29
quote: const SUBJECT: &str = "statusline-shell";
at: src/cmd/verify/typed/run.rs:21
quote: const SUBJECT: &str = "verify-pty-probe";
@row_add n=183 row=contract/child-spawn-log/claude-agents-json-parse-failure area=6-log-coverage rec=fct-7eai74no@1
boundary: child and shell spawn log events
item: claude agents --json parse failure
promise: A claude agents --json parse failure gives one parse-rejected{parser:"claude-agents-json"} line and the value "unknown".
@row_check n=183 row=contract/child-spawn-log/claude-agents-json-parse-failure sha=0d64fd8583c4 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:78-82
quote: let list = agents::parse(&ran.stdout).map_err(|failure| {         obs_event!(             WARN,             ObsEvent::ParseRejected,             parser = "claude-agents-json",
at: src/cmd/list.rs:124-126
quote: .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))     {         board::join(&mut board, observations(list));
@row_add n=184 row=absence/database area=6-log-coverage rec=fct-oqqvdowc@1
capability: A database
@row_check n=184 row=absence/database sha=3172c29a0c8e v=holds judge=sonnet head=8834195b0683
searched: rusqlite|sqlx|diesel|sled|redb|sqlite|postgres · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/*/src/**
searched: libsqlite3-sys · in deny.toml (listed only as a denied crate)
@row_add n=185 row=decision/local-error-capture-only area=7-error-capture-reporting rec=fct-nwrbbhk2@1,fct-bx64zmwr@1
name: Local error capture only
rule: Errors are captured locally only; there is no external error-reporting platform and no reporter to opt into.
@row_check n=185 row=decision/local-error-capture-only sha=a5b9fabaef81 v=holds judge=sonnet head=8834195b0683
searched: sentry|ureq|rustls|reporter · in Cargo.toml and crates/*/Cargo.toml (only deny.toml:34 bans sentry)
searched: sentry|ureq|rustls · in src/**/*.rs and crates/*/src/**/*.rs (no match)
@row_add n=186 row=decision/typed-errors-fixed-display area=7-error-capture-reporting rec=fct-nwrbbhk2@1,fct-i5zqforo@1
name: Typed errors with fixed Display messages
rule: Every typed error has a fixed-message Display, written with thiserror, and holds no fields with paths or payloads.
except: PtyError's Display is hand-written, as viola-pty takes no thiserror.
@row_check n=186 row=decision/typed-errors-fixed-display sha=90c8e340a174 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:81
quote: impl fmt::Display for PtyError {
at: crates/viola-mcp/src/lib.rs:31
quote: #[derive(Debug, thiserror::Error)]
searched: #[error(".*{ · in crates/*/src/**/*.rs (no error attribute interpolates a field)
@row_add n=187 row=decision/anyhow-at-dispatch-edge-only area=7-error-capture-reporting rec=fct-nwrbbhk2@1
name: anyhow at the dispatch edge only
rule: anyhow is used only at the root binary's dispatch edge and its catch-site reporter.
@row_check n=187 row=decision/anyhow-at-dispatch-edge-only sha=8afef1ace9c7 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:153
quote: anyhow.workspace = true
at: src/cmd/list.rs:39
quote: pub(crate) fn list(home: &Path, args: &ListArgs) -> anyhow::Result<ExitCode> {
searched: anyhow · in crates/*/src/**/*.rs and crates/*/Cargo.toml (no match; only the root bin uses anyhow)
@row_add n=188 row=decision/hand-written-panic-capture area=7-error-capture-reporting rec=fct-nwrbbhk2@1
name: Hand-written panic capture
rule: Panic capture is a hand-written std panic hook plus std catch_unwind, with no panic-reporting crate.
@row_check n=188 row=decision/hand-written-panic-capture sha=d9a1c2eb908c v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
at: src/main.rs:48
quote: let outcome = std::panic::catch_unwind(|| {
searched: sentry|human-panic|tracing-panic|color-eyre · in **/Cargo.toml (no match)
@row_add n=189 row=decision/sentry-not-admissible area=7-error-capture-reporting rec=fct-s7d7nvvm@1
name: Sentry not admissible
rule: sentry and sentry-tracing are not admissible.
why: They break the egress ban, and their default reqwest, native-tls and tokio transport breaks the cargo-deny C-crate and Tokio-in-sync-crates bans.
@row_check n=189 row=decision/sentry-not-admissible sha=9ff80f23d8ff v=holds judge=sonnet head=8834195b0683
at: deny.toml:34
quote: { crate = "sentry", reason = "no error reporter or egress in v1" },
@row_add n=190 row=decision/tracing-panic-rejected area=7-error-capture-reporting rec=fct-hbeo3r33@1
name: tracing-panic rejected
rule: tracing-panic is not used.
why: The crate is stale.
@row_check n=190 row=decision/tracing-panic-rejected sha=ee0d4d2f9e00 v=holds judge=sonnet head=8834195b0683
searched: tracing-panic|tracing_panic · in **/Cargo.toml and src/**/*.rs and crates/*/src/**/*.rs (no match)
@row_add n=191 row=decision/human-panic-rejected area=7-error-capture-reporting rec=fct-hbeo3r33@1
name: human-panic rejected
rule: human-panic is not used.
why: It writes a human report to stderr.
@row_check n=191 row=decision/human-panic-rejected sha=539ff95feefe v=holds judge=sonnet head=8834195b0683
searched: human-panic|human_panic · in **/Cargo.toml and src/**/*.rs and crates/*/src/**/*.rs (no match)
@row_add n=192 row=decision/future-reporter-ureq-rustls area=7-error-capture-reporting rec=fct-s5b2ujf4@1,fct-bx64zmwr@1
name: Future reporter uses ureq and rustls
rule: Any future error reporter must use the ureq and rustls transport.
@row_check n=192 row=decision/future-reporter-ureq-rustls sha=780024b451e5 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=193 row=decision/future-reporter-never-log-floor area=7-error-capture-reporting rec=fct-l46fiqal@1,fct-bx64zmwr@1
name: Future reporter applies the NEVER-log floor
rule: Any future error reporter must apply the NEVER-log floor in before_send.
@row_check n=193 row=decision/future-reporter-never-log-floor sha=8e8d826231eb v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=194 row=decision/future-reporter-off-by-default area=7-error-capture-reporting rec=fct-5tiqkcpp@1,fct-bx64zmwr@1
name: Future reporter off by default
rule: Any future error reporter must be off by default.
@row_check n=194 row=decision/future-reporter-off-by-default sha=73b757408324 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=195 row=decision/future-reporter-not-env-enabled area=7-error-capture-reporting rec=fct-i665y2l6@1,fct-bx64zmwr@1
name: Future reporter never enabled by env var
rule: Any future error reporter must never be enabled by an environment variable.
@row_check n=195 row=decision/future-reporter-not-env-enabled sha=45da152ba78f v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=196 row=decision/future-reporter-needs-egress-entry area=7-error-capture-reporting rec=fct-uzwm6rhy@1,fct-bx64zmwr@1
name: Future reporter needs a security egress entry
rule: Any future error reporter also needs a security Decisions Log entry for its egress.
@row_check n=196 row=decision/future-reporter-needs-egress-entry sha=15c581478e96 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=197 row=decision/scrubbing-at-source area=7-error-capture-reporting rec=fct-ugpuuxhx@1
name: Scrubbing at the source
rule: Error scrubbing sits at the source, not in a reporter's before_send hook.
why: No reporter exists.
@row_check n=197 row=decision/scrubbing-at-source sha=85acedf365ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:66-67
quote: /// Fixed messages only: portable-pty's own error text names the command line and cwd, so it is /// kept as the source for the detail file and never shown.
searched: before_send · in src/**/*.rs and crates/*/src/**/*.rs (no match)
@row_add n=198 row=decision/serde-errors-become-internal-error area=7-error-capture-reporting rec=fct-nmxgzm4p@1,fct-ugpuuxhx@1
name: Serde error chains replaced by internal error
rule: At dispatch, an error chain whose source is a serde_json or serde_path_to_error error is replaced by the fixed message "internal error" before it reaches stderr, --json, MCP isError or channel error.data.
why: A scrubbing layer at the source.
@row_check n=198 row=decision/serde-errors-become-internal-error sha=c69babd7f9e0 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:57
quote: obs::report_internal_error(&failure.error, failure.sink.as_ref());
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=199 row=decision/full-error-chain-in-detail-file-only area=7-error-capture-reporting rec=fct-hesjwzzx@1,fct-ugpuuxhx@1
name: Full error chain in the detail file only
rule: The full error chain is written as a one-line JSON array chain:[...] only to the instance's per-process detail file, and only when an instance resolves.
@row_check n=199 row=decision/full-error-chain-in-detail-file-only sha=0e307f38e338 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:258-261
quote: if let Some(sink) = sink { let line = chain_detail_line(&timestamp(Utc::now()), sink.process, &sink.instance, error); write_detail(&sink.home, &sink.instance, sink.process, &line); }
at: src/obs.rs:292
quote: .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=200 row=decision/panic-hook-silent-on-stderr area=7-error-capture-reporting rec=fct-iis47bt2@1
name: Panic hook installed first, silent on stderr
rule: Viola's panic hook is installed by main's first statement, never calls the default hook and never writes to stderr.
why: The hook contract and the run terminal both forbid it.
@row_check n=200 row=decision/panic-hook-silent-on-stderr sha=5ae7d81b5972 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
at: src/main.rs:160-163
quote: fn viola_panic_hook(info: &PanicHookInfo<'_>) { let Some(sink) = PANIC_SINK.get() else { return; };
@row_add n=201 row=decision/panic-line-bypasses-tracing area=7-error-capture-reporting rec=fct-7mvwq4nm@1
name: Panic line bypasses tracing
rule: The panic hook writes its one JSON line with a single write to a pre-opened file, not through the tracing subscriber.
why: No re-entrancy if the panic happened inside formatting.
@row_check n=201 row=decision/panic-line-bypasses-tracing sha=436da0b640b6 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:191
quote: let _ = (&*sink.file).write_all(line.as_bytes());
@row_add n=202 row=contract/panic-line/keys area=7-error-capture-reporting rec=fct-7mvwq4nm@1
boundary: the panic line
item: keys
promise: The panic line holds timestamp, level "ERROR", target "viola::panic", message "panic", event "panic", process, instance, panic_location and thread.
@row_check n=202 row=contract/panic-line/keys sha=1fde2c3c5320 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:241-248
quote: "timestamp": timestamp, "level": "ERROR", "target": "viola::panic", "message": "panic", "event": "panic", "process": process, "panic_location": location, "thread": thread,
at: src/main.rs:251
quote: record["instance"] = serde_json::Value::from(instance);
@row_add n=203 row=contract/panic-line/corr area=7-error-capture-reporting rec=fct-7mvwq4nm@1
boundary: the panic line
item: corr
promise: corr is never written on the panic line.
@row_check n=203 row=contract/panic-line/corr sha=4d7f0bd95632 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:240-249
quote: let mut record = serde_json::json!({ "timestamp": timestamp, "level": "ERROR", "target": "viola::panic", "message": "panic", "event": "panic", "process": process, "panic_location": location, "thread": thread, });
searched: "corr" · in src/main.rs panic_line (lines 233-256), no corr key written
@row_add n=204 row=contract/panic-line/instance area=7-error-capture-reporting rec=fct-7mvwq4nm@1
boundary: the panic line
item: instance
promise: instance is omitted when none resolved, the same key-absence null encoding obs_event! uses.
@row_check n=204 row=contract/panic-line/instance sha=84296d2aad42 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:250-251
quote: if let Some(instance) = instance { record["instance"] = serde_json::Value::from(instance);
@row_add n=205 row=decision/panic-backtrace-in-detail-file area=7-error-capture-reporting rec=fct-yn6razq4@1
name: Panic backtrace in the detail file
rule: When an instance resolves, the panic hook also writes the payload and a raw, never-symbolised backtrace as one line to the process's detail file, not gated by RUST_BACKTRACE.
why: Env vars are not a configuration channel.
@row_check n=205 row=decision/panic-backtrace-in-detail-file sha=187af7b3726c v=holds judge=sonnet head=8834195b0683
at: src/main.rs:213
quote: let backtrace: Vec<Value> = panic_frames::capture()
at: src/main.rs:201
quote: obs::write_detail(&sink.home, instance, sink.process, &detail);
@row_add n=206 row=decision/panic-exit-decided-at-catch-site area=7-error-capture-reporting rec=fct-sue3fxzz@1
name: Panic exit decided at a catch site
rule: The panic hook only writes; the exit is decided by one catch_unwind in the root binary's main around everything after the hook is set, so no role ends through an uncaught unwind.
@row_check n=206 row=decision/panic-exit-decided-at-catch-site sha=ab7514639c94 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46-48
quote: std::panic::set_hook(Box::new(viola_panic_hook)); let role = role_of(std::env::args_os()); let outcome = std::panic::catch_unwind(|| {
@row_add n=207 row=decision/panic-unwind-in-all-profiles area=7-error-capture-reporting rec=fct-sue3fxzz@1
name: panic = unwind in every profile
rule: Every Cargo profile keeps panic = "unwind".
guard: G3
@row_check n=207 row=decision/panic-unwind-in-all-profiles sha=8e5a315ac5e5 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:261
quote: panic = "unwind"
at: Cargo.toml:269
quote: panic = "unwind"
guard_at: .github/workflows/ci.yml:298
@row_add n=208 row=contract/panic-outcome-by-role/hook area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: hook
promise: After a panic, hook exits 0 with empty stdout.
@row_check n=208 row=contract/panic-outcome-by-role/hook sha=3a3c6d0029bf v=holds judge=sonnet head=8834195b0683
at: src/main.rs:48
quote: let outcome = std::panic::catch_unwind(|| {
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=209 row=contract/panic-outcome-by-role/cli area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: cli
promise: After a panic, cli logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} when an instance resolves, puts the chain in the detail file only, writes exactly "error: internal error\n" on stderr in one write with no hint, and exits 1.
@row_check n=209 row=contract/panic-outcome-by-role/cli sha=d5fea0e9d254 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:234
quote: .is_some_and(|c| matches!(c.process, ObsProcess::Run | ObsProcess::Cli))
at: src/obs.rs:241
quote: detail = "internal-error",
at: src/main.rs:201
quote: obs::write_detail(&sink.home, instance, sink.process, &detail);
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/main.rs:138
quote: ) => ExitCode::from(1),
@row_add n=210 row=contract/panic-outcome-by-role/cli-json area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: cli --json
promise: After a panic under --json, cli writes {"v":1,"error":"internal-error","detail":null} on stdout with stderr empty, and exits 1.
@row_check n=210 row=contract/panic-outcome-by-role/cli-json sha=9910e2f31424 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:67-68
quote: if json_asked(std::env::args_os()) { cmd::internal_error_document();
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/main.rs:138
quote: ) => ExitCode::from(1),
@row_add n=211 row=contract/panic-outcome-by-role/run area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: run
promise: After a panic, run logs process-exit{subject:"self", exit_code:1, detail:"internal-error"} and exits 1, writing nothing on the terminal while the child runs.
@row_check n=211 row=contract/panic-outcome-by-role/run sha=4897bbb138c5 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:63
quote: obs::internal_error_exit_line();
at: src/obs.rs:240-241
quote: exit_code = 1u8, detail = "internal-error",
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=212 row=contract/panic-outcome-by-role/mcp-task-panic area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: mcp spawned-task panic
promise: A panic inside an mcp spawned Tokio task is logged by the hook, contained at the task boundary, and the process continues.
@row_check n=212 row=contract/panic-outcome-by-role/mcp-task-panic sha=2b0b93f9c9cd v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:240
quote: let task = tokio::spawn(async move { call(&ctx, tool, args).await });
at: crates/viola-mcp/src/tools.rs:243
quote: Ok(Err(McpError::Internal)) | Err(_) => Err(internal_fault()),
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
@row_add n=213 row=contract/panic-outcome-by-role/mcp-main-thread-panic area=7-error-capture-reporting rec=fct-x3mpnm6q@1
boundary: panic outcome by role
item: mcp main-thread panic
promise: An mcp main-thread panic writes no process-exit line and nothing to stdout, and the process exits 1.
@row_check n=213 row=contract/panic-outcome-by-role/mcp-main-thread-panic sha=10e3cd4316c6 v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=214 row=decision/unhandled-panics-always-captured area=7-error-capture-reporting rec=fct-236mfl4p@1,fct-u2647lgq@1
name: Unhandled panics always captured
rule: Every unhandled panic is captured as the one-line event "panic".
@row_check n=214 row=decision/unhandled-panics-always-captured sha=ae715ccbd72b v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
at: src/main.rs:245
quote: "event": "panic",
@row_add n=215 row=decision/boundary-errors-end-in-logged-line area=7-error-capture-reporting rec=fct-yc4xljp5@1,fct-u2647lgq@1
name: Crate-seam errors end in a logged line
rule: Every typed error that crosses a crate seam ends in a logged process-exit detail, channel-response error, hook-decision detail or send-refused line, carrying codes only at WARN or ERROR.
@row_check n=215 row=decision/boundary-errors-end-in-logged-line sha=d2f887070aec v=unverifiable judge=sonnet head=8834195b0683
searched: every typed error crossing a crate seam · in src/**/*.rs and crates/**/src/*.rs (too broad to confirm each seam; no single doing line)
why: no evidence found
@row_add n=216 row=decision/vt100-panics-contained area=7-error-capture-reporting rec=fct-gxdjrwob@1,fct-u2647lgq@1
name: vt100 panics contained
rule: vt100 panics are caught around each feed and resize on run's feed thread; each poisoning episode gives exactly one parse-rejected{parser:"vt100-feed"} at WARN, and run continues.
@row_check n=216 row=decision/vt100-panics-contained sha=163f70f0b564 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() {
at: src/run/gate.rs:81
quote: parser = "vt100-feed",
@row_add n=217 row=decision/mcp-rejects-codes-only area=7-error-capture-reporting rec=fct-pqz5mgd5@2
name: MCP rejects are codes only
rule: MCP rejects are isError codes only, and a logged reject's line holds codes only.
@row_check n=217 row=decision/mcp-rejects-codes-only sha=c6b27c44b20b v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:340
quote: return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=218 row=decision/instance-unreachable-not-logged area=7-error-capture-reporting rec=fct-pqz5mgd5@2
name: instance-unreachable logged by no line
rule: An MCP instance-unreachable reject, from any instance, liveness, socket-directory, euid or server-verification check, is logged by no line; the isError code is its only record.
@row_check n=218 row=decision/instance-unreachable-not-logged sha=47b3441b723a v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:340
quote: return Ok(result::unreachable(UnreachableCause::StrictModes));
at: crates/viola-mcp/src/tools.rs:343
quote: return Ok(result::unreachable(without_endpoint(&instance_dir)));
at: crates/viola-mcp/src/tools.rs:346
quote: return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=219 row=decision/error-correlation-without-trace-ids area=7-error-capture-reporting rec=fct-kd3a4grv@1
name: Error correlation without trace ids
rule: Errors correlate by corr, conn, process and instance, plus timestamp, not by trace_id or span_id.
@row_check n=219 row=decision/error-correlation-without-trace-ids sha=04b6fc4fbcf5 v=holds judge=sonnet head=8834195b0683
searched: trace_id|span_id|TraceId · in src/**/*.rs and crates/**/*.rs
@row_add n=220 row=contract/detail-file-line/correlation-keys area=7-error-capture-reporting rec=fct-kd3a4grv@1
boundary: the detail-file error line
item: correlation keys
promise: The detail-file line for an error carries the same event, corr and conn as its home-level counterpart, so jq joins them.
@row_check n=220 row=contract/detail-file-line/correlation-keys sha=97c295a39ba2 v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=221 row=decision/secrets-never-logged area=8-pii-scrubbing-compliance rec=fct-btgnwmtl@1
name: Secrets never logged
rule: The Cookie header, signing credentials and R8-stripped CLAUDE* values are never logged.
@row_check n=221 row=decision/secrets-never-logged sha=5be40d174fe4 v=holds judge=sonnet head=8834195b0683
searched: Cookie|cookie|signing · in src/**/*.rs and crates/**/*.rs (only crates/viola-e2e/src/harness/secret_scan.rs, the scanner that flags a leaked cookie)
searched: Cookie|CLAUDE_ inside obs_event! calls · in crates/viola-core/src/obs.rs
@row_add n=222 row=decision/ci-secrets-not-in-artifacts area=8-pii-scrubbing-compliance rec=fct-btgnwmtl@1
name: CI secrets never in artifacts
rule: CI secrets are never echoed into artifacts.
@row_check n=222 row=decision/ci-secrets-not-in-artifacts sha=ea137476b9ff v=holds judge=sonnet head=8834195b0683
searched: echo.*secret · in .github/workflows/ci.yml (no echo of a secret; the secret scan at line 76 runs before the artifact uploads)
searched: secrets\. · in .github/workflows/ci.yml
@row_add n=223 row=decision/conversation-text-only-in-detail-file area=8-pii-scrubbing-compliance rec=fct-btgnwmtl@1
name: Conversation text only in the instance detail file
rule: Prompt, assistant, tool-input, plan and dialog text never reach a home-level line, only an instance's per-process detail file inside a chain or drift report.
@row_check n=223 row=decision/conversation-text-only-in-detail-file sha=5bbfe47e4eb4 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:260
quote: write_detail(&sink.home, &sink.instance, sink.process, &line);
at: src/cmd/hook.rs:557
quote: obs::write_detail(&instance.home, &instance.name, ObsProcess::Hook, &line);
@row_add n=224 row=decision/statusline-command-out-of-home-logs area=8-pii-scrubbing-compliance rec=fct-btgnwmtl@1
name: statusline_command out of home-level logs
rule: statusline_command stays out of home-level logs.
@row_check n=224 row=decision/statusline-command-out-of-home-logs sha=4d4333ef8717 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:375
quote: statusline_command,
searched: obs_event!\([^;]*statusline_command · in src/**/*.rs and crates/**/*.rs (no match)
searched: statusline_command · in src/**/*.rs and crates/**/*.rs (snapshot field and its builder only, no log call)
@row_add n=225 row=decision/endpoint-and-pinned-bin-never-logged area=8-pii-scrubbing-compliance rec=fct-qk7hj63u@1
name: endpoint and pinned_bin never logged
rule: The endpoint and pinned_bin values are never logged.
@row_check n=225 row=decision/endpoint-and-pinned-bin-never-logged sha=f56aebc9485e v=holds judge=sonnet head=8834195b0683
searched: obs_event!\([^;]*(endpoint|pinned_bin) · in src/**/*.rs and crates/**/*.rs (only endpoint_kind at src/run/mod.rs:37)
searched: pinned_bin · in src/**/*.rs and crates/**/*.rs (snapshot field only)
@row_add n=226 row=decision/claude-agents-rows-never-logged area=8-pii-scrubbing-compliance rec=fct-qk7hj63u@1
name: claude agents rows never logged verbatim
rule: claude agents --json rows are never logged verbatim, only parse-rejected counts.
@row_check n=226 row=decision/claude-agents-rows-never-logged sha=c99f784d7ab9 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:82-84
quote: parser = "claude-agents-json", detail = failure.detail(), count = 1u64,
@row_add n=227 row=decision/low-class-metadata-as-typed-fields area=8-pii-scrubbing-compliance rec=fct-qk7hj63u@1
name: Low-class metadata as typed fields
rule: Low-class metadata is logged as typed fields, never inside detail codes.
@row_check n=227 row=decision/low-class-metadata-as-typed-fields sha=742436009c9d v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:36
quote: pid = std::process::id(),
at: src/run/agents.rs:82
quote: parser = "claude-agents-json",
@row_add n=228 row=decision/veil-redact-second-layer area=8-pii-scrubbing-compliance rec=fct-nfaxfmcg@1
name: veil Redact as the second redaction layer
rule: veil's derive Redact is the second redaction layer, so a stray value capture prints [REDACTED] instead of content.
@row_check n=228 row=decision/veil-redact-second-layer sha=74ae9a91837f v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:51
quote: #[derive(Deserialize, Redact)]
@row_add n=229 row=decision/veil-toggle-feature-banned area=8-pii-scrubbing-compliance rec=fct-nfaxfmcg@1
name: veil toggle feature banned
rule: veil's non-default toggle feature, which lets redaction be disabled at runtime, is never enabled.
guard: cargo-deny bans.features entry denying toggle on veil
@row_check n=229 row=decision/veil-toggle-feature-banned sha=206d991ff43b v=holds guard=true judge=sonnet head=8834195b0683
searched: toggle · in Cargo.toml (veil = "=0.3.0" at line 245, no features)
guard_at: deny.toml:38-40
@row_add n=230 row=decision/no-regex-scrubbers area=8-pii-scrubbing-compliance rec=fct-z6ygfhpz@1
name: No regex scrubbers
rule: Sensitive data is never scrubbed by regular expressions.
why: viola's sensitive data is known by type
@row_check n=230 row=decision/no-regex-scrubbers sha=da0c524c570a v=holds judge=sonnet head=8834195b0683
searched: Regex::new|use regex|regex:: · in src/**/*.rs
searched: regex\s*= · in **/Cargo.toml (no match)
@row_add n=231 row=decision/redact-crate-not-adopted area=8-pii-scrubbing-compliance rec=fct-34ylsxhv@1
name: redact crate not adopted
rule: The redact crate, the researched lighter alternative to veil, is not adopted.
@row_check n=231 row=decision/redact-crate-not-adopted sha=6b17de49fb15 v=holds judge=sonnet head=8834195b0683
searched: redact\s*= · in **/Cargo.toml (no match; only veil at Cargo.toml:245)
@row_add n=232 row=absence/sentry-error-reporting area=8-pii-scrubbing-compliance rec=fct-xpgyosmo@1
capability: Sentry error reporting
@row_check n=232 row=absence/sentry-error-reporting sha=7d038ea15f69 v=holds judge=sonnet head=8834195b0683
searched: sentry|Sentry|opentelemetry · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs (no match)
searched: sentry · in deny.toml (line 34 bans it)
@row_add n=233 row=decision/source-allow-list-logging area=8-pii-scrubbing-compliance rec=fct-n2ewvgzr@1
name: Source allow-list for logged fields
rule: Spans skip all arguments and declare fields explicitly, and events go through obs_event! with explicit typed fields only; this source allow-list is the primary PII control.
@row_check n=233 row=decision/source-allow-list-logging sha=5023f1b31c63 v=holds guard=false judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:142
quote: #[instrument(
at: crates/viola-mcp/src/tools.rs:248
quote: #[instrument(skip_all, name = "mcp.tool_call", fields(tool = tool.as_str()))]
@row_add n=234 row=decision/fixed-error-messages area=8-pii-scrubbing-compliance rec=fct-6wyihaqe@1
name: Fixed error messages
rule: Error Display texts are fixed, and serde-sourced error chains are mapped to fixed messages at dispatch.
@row_check n=234 row=decision/fixed-error-messages sha=7107259961d6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:42
quote: #[error("channel frame could not be parsed")]
at: crates/viola-channel/src/frame.rs:79
quote: let value: Value = serde_json::from_slice(line).map_err(|_| (None, ProtocolError::Parse))?;
@row_add n=235 row=decision/content-only-in-detail-file area=8-pii-scrubbing-compliance rec=fct-42flcsyl@1
name: Content-bearing records only in the detail file
rule: Content-bearing records go only to the instance-scoped detail-<process>.ndjson file.
@row_check n=235 row=decision/content-only-in-detail-file sha=53ab648a07cc v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:292
quote: .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=236 row=decision/content-canary-never-home-level area=8-pii-scrubbing-compliance rec=fct-tcjwyf2p@1
name: Content canary never in home-level files
rule: The content canary never appears in a home-level diagnostics ndjson file.
@row_check n=236 row=decision/content-canary-never-home-level sha=6e494e1dcddf v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:132-134
quote: if role { for (class, needle) in CONTENT { for offset in find_all(&bytes, needle.as_bytes()) {
@row_add n=237 row=decision/detail-upload-synthetic-inputs-only area=8-pii-scrubbing-compliance rec=fct-tcjwyf2p@1
name: Detail-file upload only with synthetic inputs
rule: Detail files and the harness capture are uploaded only while every input is synthetic; a non-synthetic source first drops them from the upload paths.
why: every input is synthetic, so detail content carries no real data
@row_check n=237 row=decision/detail-upload-synthetic-inputs-only sha=2dc36a757863 v=unverifiable judge=sonnet head=8834195b0683
searched: synthetic · in .github/workflows/ci.yml
why: not a code claim (a conditional process rule on the input source; the workflow uploads the diagnostics files behind the scan gate, no code tests for synthetic inputs)
@row_add n=238 row=decision/scan-failure-uploads-hit-report-only area=8-pii-scrubbing-compliance rec=fct-tcjwyf2p@1,fct-hmk2eyfw@1
name: Only the hit report uploaded on scan failure
rule: When the secret scan fails, only its hit report is uploaded, as the secret-scan-<os> artifact.
@row_check n=238 row=decision/scan-failure-uploads-hit-report-only sha=16abe0aaa03b v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:106
quote: if: always() && steps.secret-scan.outcome == 'failure'
at: .github/workflows/ci.yml:109
quote: name: secret-scan-${{ matrix.os }}
@row_add n=239 row=contract/secret-scan-hit-report/file area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
boundary: the secret-scan hit report
item: file
promise: Each hit names the file it was found in.
@row_check n=239 row=contract/secret-scan-hit-report/file sha=b31aec53ffaf v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
@row_add n=240 row=contract/secret-scan-hit-report/line area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
boundary: the secret-scan hit report
item: line
promise: Each hit names the line it was found on.
@row_check n=240 row=contract/secret-scan-hit-report/line sha=2fbef93c2b20 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
@row_add n=241 row=contract/secret-scan-hit-report/byte-offset area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
boundary: the secret-scan hit report
item: byte offset
promise: Each hit names the byte offset of the match.
@row_check n=241 row=contract/secret-scan-hit-report/byte-offset sha=0fe1f06ebe36 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
@row_add n=242 row=contract/secret-scan-hit-report/pattern-class area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
boundary: the secret-scan hit report
item: pattern class
promise: Each hit names the class of pattern that matched.
@row_check n=242 row=contract/secret-scan-hit-report/pattern-class sha=c49a315e9539 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
@row_add n=243 row=decision/hit-report-never-matched-bytes area=8-pii-scrubbing-compliance rec=fct-fxeyaiva@1
name: Hit report never carries matched bytes
rule: The secret-scan hit report never contains the matched bytes.
@row_check n=243 row=decision/hit-report-never-matched-bytes sha=5705a65cc1e5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:142
quote: json!({"file": relative(&roots.base, path), "line": line, "offset": offset, "class": class})
@row_add n=244 row=decision/upload-members-scanned-or-dropped area=8-pii-scrubbing-compliance rec=fct-onsbpcol@1
name: Upload members scanned or dropped
rule: An uploaded member that could carry a host path or log content is scanned before upload or dropped; unscanned uploads are admissible by content only.
@row_check n=244 row=decision/upload-members-scanned-or-dropped sha=c3cab9955662 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
@row_add n=245 row=decision/local-run-outputs-never-uploaded area=8-pii-scrubbing-compliance rec=fct-onsbpcol@1
name: Local run outputs never uploaded
rule: mutants.out, the run archive and the local viola-harness pre-push document are never uploaded.
@row_check n=245 row=decision/local-run-outputs-never-uploaded sha=6309448d7534 v=holds judge=sonnet head=8834195b0683
searched: mutants.out · in .github/workflows/*.yml
searched: run-archive · in .github/workflows/*.yml
searched: pre-push · in .github/workflows/*.yml
@row_add n=246 row=decision/fuzz-artifacts-synthetic-seeds-only area=8-pii-scrubbing-compliance rec=fct-onsbpcol@1
name: Fuzz artifacts uploaded only with synthetic seeds
rule: A non-synthetic fuzz corpus seed first drops the fuzz artifacts upload.
@row_check n=246 row=decision/fuzz-artifacts-synthetic-seeds-only sha=159e78c9915a v=unverifiable judge=sonnet head=8834195b0683
searched: synthetic · in .github/workflows/nightly.yml
why: not a code claim (a conditional rule on a future non-synthetic seed; the nightly workflow uploads fuzz/artifacts/ on failure, no code switches on seed provenance)
@row_add n=247 row=decision/default-deny-log-fields area=8-pii-scrubbing-compliance rec=fct-llmksgvm@1
name: Default-deny for logged fields
rule: A field is logged only if it is named in the obs plan's event catalog; an unknown field is a build failure.
guard: gate G4 against schemas/diag-line.v1.json
@row_check n=247 row=decision/default-deny-log-fields sha=632f2f6312d2 v=holds guard=true judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:106
quote: "unevaluatedProperties": false,
guard_at: .github/workflows/ci.yml:60
@row_add n=248 row=decision/schema-top-level-unevaluated-properties area=8-pii-scrubbing-compliance rec=fct-llmksgvm@1
name: Default-deny schema via top-level unevaluatedProperties
rule: The diagnostics line schema enforces default-deny with a top-level unevaluatedProperties false over per-event field lists.
why: in JSON Schema 2020-12 additionalProperties does not see properties declared inside allOf or if-then
guard: contract_diag_schema
@row_check n=248 row=decision/schema-top-level-unevaluated-properties sha=72697656d9a7 v=holds guard=true judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:106
quote: "unevaluatedProperties": false,
guard_at: tests/contract_diag_schema.rs:1
@row_add n=249 row=decision/no-allow-list-widening area=8-pii-scrubbing-compliance rec=fct-llmksgvm@1
name: Allow-list cannot be widened
rule: No config key, flag or env var can widen the logged-field allow-list or disable redaction.
@row_check n=249 row=decision/no-allow-list-widening sha=f29e5e7ccc78 v=holds judge=sonnet head=8834195b0683
searched: RUST_LOG|EnvFilter · in src/** and crates/*/src/**
searched: with_target\(|diagnostics_level (the only level knob, not an allow-list) · in src/obs.rs
@row_add n=250 row=decision/detail-schema-self-contained area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
name: Separate self-contained detail-line schema
rule: Detail-file lines are validated by their own schema with a top-level unevaluatedProperties false, which inlines the event enum and uses no cross-file reference.
@row_check n=250 row=decision/detail-schema-self-contained sha=c996df367c4a v=holds judge=sonnet head=8834195b0683
at: schemas/diag-detail.v1.json:29
quote: "unevaluatedProperties": false
at: schemas/diag-detail.v1.json:14
quote: "enum": ["channel-request", "channel-response", "dialog-raised", "dialog-answered", "hook-invoked", "hook-decision", "send-issued", "send-confirmed", "send-refused", "release-from-driver", "process-st
@row_add n=251 row=contract/detail-line/timestamp area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: timestamp
promise: Required on every detail-file line, as on home-level lines.
@row_check n=251 row=contract/detail-line/timestamp sha=c9ba22c4b3cb v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:307
quote: record.insert("timestamp".to_owned(), timestamp.into());
@row_add n=252 row=contract/detail-line/level area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: level
promise: Required on every detail-file line, as on home-level lines.
@row_check n=252 row=contract/detail-line/level sha=f1d4cc1829c3 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:308
quote: record.insert("level".to_owned(), level.into());
@row_add n=253 row=contract/detail-line/target area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: target
promise: Required on every detail-file line, as on home-level lines.
@row_check n=253 row=contract/detail-line/target sha=310c4873cc40 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:309
quote: record.insert("target".to_owned(), target.into());
@row_add n=254 row=contract/detail-line/message area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: message
promise: Required on every detail-file line, as on home-level lines.
@row_check n=254 row=contract/detail-line/message sha=9aeded5c597b v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:310
quote: record.insert("message".to_owned(), event.as_str().into());
@row_add n=255 row=contract/detail-line/event area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: event
promise: Required on every detail-file line, as on home-level lines.
@row_check n=255 row=contract/detail-line/event sha=58c34e1c8476 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:311
quote: record.insert("event".to_owned(), event.as_str().into());
@row_add n=256 row=contract/detail-line/process area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: process
promise: Required on every detail-file line, as on home-level lines.
@row_check n=256 row=contract/detail-line/process sha=143079407e01 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:312
quote: record.insert("process".to_owned(), process.as_str().into());
@row_add n=257 row=contract/detail-line/instance area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: instance
promise: Required on every detail-file line, as on home-level lines.
@row_check n=257 row=contract/detail-line/instance sha=6f80c4d3e245 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:313
quote: record.insert("instance".to_owned(), name_str(instance).into());
@row_add n=258 row=contract/detail-line/panic-location area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: panic_location
promise: Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
@row_check n=258 row=contract/detail-line/panic-location sha=dcf45a4b3bcf v=holds judge=sonnet head=8834195b0683
at: src/main.rs:218
quote: fields.insert("panic_location".to_owned(), location.into());
@row_add n=259 row=contract/detail-line/thread area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: thread
promise: Allowed on panic detail lines only, copied from the home-level panic line so both lines join without the payload.
@row_check n=259 row=contract/detail-line/thread sha=42396469b3d2 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:219
quote: fields.insert("thread".to_owned(), thread.into());
@row_add n=260 row=contract/detail-line/panic-payload area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: panic_payload
promise: One of exactly four content fields a detail-file line may carry: the panic's payload.
@row_check n=260 row=contract/detail-line/panic-payload sha=45bb8f0ee8e6 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:220
quote: fields.insert("panic_payload".to_owned(), payload.into());
@row_add n=261 row=contract/detail-line/backtrace area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: backtrace
promise: One of exactly four content fields a detail-file line may carry: the panic's backtrace.
@row_check n=261 row=contract/detail-line/backtrace sha=f503bb80ca08 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:221
quote: fields.insert("backtrace".to_owned(), Value::Array(backtrace));
@row_add n=262 row=contract/detail-line/chain area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: chain
promise: One of exactly four content fields a detail-file line may carry: the full error chain.
@row_check n=262 row=contract/detail-line/chain sha=64719851f34f v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:276
quote: fields.insert("chain".to_owned(), Value::Array(chain));
@row_add n=263 row=contract/detail-line/drift-report area=8-pii-scrubbing-compliance rec=fct-o26lv7c7@1
boundary: the detail-file line
item: drift_report
promise: One of exactly four content fields a detail-file line may carry: the drift report.
@row_check n=263 row=contract/detail-line/drift-report sha=4c77cfc507a5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:547
quote: fields.insert("drift_report".to_owned(), Value::Array(drift.to_vec()));
@row_add n=264 row=decision/ci-on-github-actions area=9-ci-integration rec=fct-xyexk37w@2
name: CI on GitHub Actions
rule: CI runs on GitHub Actions in one push and pull-request workflow on an ubuntu-latest matrix, beside a scheduled nightly workflow for advisory checks and fuzzing; there is no OTLP collector job.
@row_check n=264 row=decision/ci-on-github-actions sha=cb4c9ba3522d v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:3-5
quote: on:   push:   pull_request:
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: .github/workflows/nightly.yml:5
quote: - cron: "17 4 * * 1"
@row_add n=265 row=decision/test-homes-under-e2e-home area=9-ci-integration rec=fct-xtfnzpyu@1
name: Integration-test homes under one root
rule: Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it.
why: a hook panic exits 0, so a home outside that root would hide it
except: the one chaos_feed_panic test home
@row_check n=265 row=decision/test-homes-under-e2e-home sha=fbd126d875e3 v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:193
quote: let base = workspace_path("target/e2e-home");
at: tests/support/home.rs:210
quote: /// A home under the system temp dir, NOT under `target/e2e-home`: outside the CI scans (G2
@row_add n=266 row=decision/g2-zero-panics-gate area=9-ci-integration rec=fct-eodqccd6@1
name: Zero-panics gate fails closed
rule: The zero-panics gate probes itself first, fails closed, runs in the test and perf jobs, and counts panics over home-level role files only.
guard: G2
except: the fake-agent seam's forced panic at exactly its seam location, compared as a whole string
@row_check n=266 row=decision/g2-zero-panics-gate sha=5b7d6a9c5413 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:59
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
at: .github/workflows/ci.yml:151
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
at: scripts/g2-zero-panics.sh:23
quote: find "$scope/" -path '*/diagnostics/*.ndjson' ! -name 'detail-*' -exec awk 1 {} + \
at: scripts/g2-zero-panics.sh:28
quote: and (.panic_location | sub(":[0-9]+$"; "")) == $seam;
guard_at: .github/workflows/ci.yml:56
@row_add n=267 row=decision/g4-schema-conformance-gate area=9-ci-integration rec=fct-htkftgtl@1
name: Schema conformance gate on every line
rule: Every home-level line is validated against the diagnostics line schema and every detail line against the detail schema, in a step that always runs.
guard: G4
@row_check n=267 row=decision/g4-schema-conformance-gate sha=124b8a5b8330 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:92-93
quote: validator("diag-line.v1.json"),         validator("diag-detail.v1.json"),
at: .github/workflows/ci.yml:60-62
quote: - name: G4 schema conformance         id: schema-conformance         if: always()
guard_at: .github/workflows/ci.yml:60
@row_add n=268 row=decision/gate-output-never-line-content area=9-ci-integration rec=fct-htkftgtl@1
name: Schema gate output never shows line content
rule: The schema conformance gate prints file, line and keyword of a failure, never the line's content.
guard: G4
@row_check n=268 row=decision/gate-output-never-line-content sha=ce015d4d4c11 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:126
quote: failures.push(json!({"file": file, "line": n + 1, "keyword": keyword}));
guard_at: .github/workflows/ci.yml:60
@row_add n=269 row=decision/no-abort-panic-strategy area=9-ci-integration rec=fct-2rxslar4@1
name: No abort panic strategy
rule: No panic strategy is set to abort in Cargo.toml, cargo config or workflows; the gate passes only on a clean no-match.
guard: G3
@row_check n=269 row=decision/no-abort-panic-strategy sha=8aeab17e725d v=holds guard=true judge=sonnet head=8834195b0683
searched: abort · in Cargo.toml, .cargo/*, crates/*/Cargo.toml (no match)
searched: abort · in .github/workflows/*.yml (only the G3 pattern itself, not a setting)
guard_at: .github/workflows/ci.yml:301
@row_add n=270 row=decision/no-ci-attributes-in-log-lines area=9-ci-integration rec=fct-z7z7o2va@1
name: No CI resource attributes in log lines
rule: deployment.environment, ci.run.id and git.commit.sha are never injected into viola log lines; CI identity lives in artifact names and GitHub run metadata.
why: env vars are not a configuration channel
@row_check n=270 row=decision/no-ci-attributes-in-log-lines sha=d9675d5245bd v=holds judge=sonnet head=8834195b0683
searched: deployment\.environment|ci\.run\.id|git\.commit\.sha · in **/*.{rs,yml,sh,toml,json} (no match)
@row_add n=271 row=decision/artifact-retention-seven-days area=9-ci-integration rec=fct-gc3zcqcq@1
name: Seven-day artifact retention
rule: CI artifacts are retained for 7 days, below the 90-day default.
@row_check n=271 row=decision/artifact-retention-seven-days sha=fa199d0701e5 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:84
quote: retention-days: 7
@row_add n=272 row=decision/panic-and-schema-gates-always-run area=9-ci-integration rec=fct-4573rvoy@2
name: Panic and schema gates always run
rule: The zero-panics and schema gates run after the integration and E2E tests even when a test step failed.
why: a failed test step must not skip the zero-panics gate
@row_check n=272 row=decision/panic-and-schema-gates-always-run sha=c96f96ad6421 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:56-57
quote: - name: G2 zero panics         if: always()
at: .github/workflows/ci.yml:60-62
quote: - name: G4 schema conformance         id: schema-conformance         if: always()
@row_add n=273 row=decision/perf-gates-own-job area=9-ci-integration rec=fct-4573rvoy@2
name: Perf gates in their own job
rule: The hyperfine performance gates run in their own perf job.
@row_check n=273 row=decision/perf-gates-own-job sha=7fca514c0322 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:122
quote: perf:
at: .github/workflows/ci.yml:147
quote: run: bash scripts/agent-run.sh run --perf
@row_add n=274 row=decision/homes-kept-until-gates-run area=9-ci-integration rec=fct-4573rvoy@2
name: Test homes kept until the gates run
rule: Integration-test homes are kept until the gate steps have run.
@row_check n=274 row=decision/homes-kept-until-gates-run sha=03f0c847fc35 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:19-20
quote: env:       AGENT_RUN_KEEP_HOMES: "1"
at: tests/support/home.rs:195
quote: if !flag("AGENT_RUN_KEEP_HOMES") && !flag("AGENT_RUN_KEEP_FAILED") {
@row_add n=275 row=decision/secret-scan-always-runs area=9-ci-integration rec=fct-xwehypb2@1
name: Secret scan always runs
rule: The secret scan step runs even when an earlier step failed.
why: otherwise a failed E2E step skips the scan and the gated uploads, which loses the failing job's diagnostics
@row_check n=275 row=decision/secret-scan-always-runs sha=c94362f3df51 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:72-74
quote: - name: Secret scan         id: secret-scan         if: always()
@row_add n=276 row=decision/secret-scan-scope area=9-ci-integration rec=fct-xwehypb2@1
name: Secret scan scope
rule: The secret scan covers every test-home diagnostics file, the harness capture and the nextest JUnit file.
why: nextest stores failing tests' captured output in the JUnit file
except: the mutants run's chunk diff, skipped by exact path and excluded from the harness upload
@row_check n=276 row=decision/secret-scan-scope sha=4607a4ef44a4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:63
quote: let diag = diag_files(&roots.e2e_home);
at: crates/viola-e2e/src/harness/secret_scan.rs:76
quote: collect_files(&roots.agent_run, &mut capture);
at: crates/viola-e2e/src/harness/secret_scan.rs:79-80
quote: let mutation_diff = roots.agent_run.join("chunk.diff");     capture.retain(|path| *path != mutation_diff);
at: crates/viola-e2e/src/harness/secret_scan.rs:81-82
quote: if roots.junit.is_file() {         capture.push(roots.junit.clone());
@row_add n=277 row=decision/no-retry-policies area=10-slo-invariants-telemetry-budgets rec=fct-3qkbv2ca@1,fct-uhtssi46@1
name: Retry policies forbidden
rule: Retry-once policies are forbidden, for tests and for panics alike.
why: retry policies mask real signals
@row_check n=277 row=decision/no-retry-policies sha=ca0625dc2482 v=holds judge=sonnet head=8834195b0683
at: .config/nextest.toml:2
quote: retries = 0
@row_add n=278 row=decision/hook-panic-without-name-unobservable area=10-slo-invariants-telemetry-budgets rec=fct-73zqjpsk@1
name: Hook panic without VIOLA_NAME unobservable
rule: A hook panic without VIOLA_NAME set is unobservable by design.
@row_check n=278 row=decision/hook-panic-without-name-unobservable sha=14d97b05c082 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/main.rs:161
quote: let Some(sink) = PANIC_SINK.get() else {
@row_add n=279 row=decision/heartbeat-liveness area=10-slo-invariants-telemetry-budgets rec=fct-gqc4mh6b@1,fct-onnnq5uj@1
name: Heartbeat file as the liveness signal
rule: run touches the heartbeat every 1 s and no process logs a transition; liveness flips to stale at more than 5 s.
why: replaces periodic tick lines
@row_check n=279 row=decision/heartbeat-liveness sha=d26d97733b67 v=holds guard=false judge=sonnet head=8834195b0683
at: crates/viola-state/src/heartbeat.rs:13
quote: pub const BEAT_EVERY: Duration = Duration::from_secs(1);
at: crates/viola-state/src/heartbeat.rs:42
quote: let _ = touch_heartbeat(&instance_dir);
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
@row_add n=280 row=decision/module-boundary-error-logging area=10-slo-invariants-telemetry-budgets rec=fct-db3jadur@1
name: Errors logged once at module boundaries
rule: Every typed error that crosses a crate seam is logged once at WARN or ERROR with a closed detail or error_code; the full chain goes only to the instance detail file.
@row_check n=280 row=decision/module-boundary-error-logging sha=c605de83f0fd v=unverifiable judge=sonnet head=8834195b0683
searched: error_code · in src/**, crates/*/src/**, schemas/** (only channel-response -32xxx codes)
searched: chain|DetailSink · in src/obs.rs
why: no evidence found for the universal claim that every typed error crossing a crate seam is logged once; only the catch site (src/obs.rs report_internal_error, chain to the detail file only) was seen
@row_add n=281 row=decision/hook-perf-budgets-timed area=10-slo-invariants-telemetry-budgets rec=fct-fruln4ot@2
name: Hook performance budgets timed with hyperfine
rule: The session-start, user-prompt-submit, stop, session-end and pre-tool-use hooks are timed with hyperfine on an unstamped session in the perf job, and the perf gate requires all five; the async tier is untimed.
guard: gate --require perf
@row_check n=281 row=decision/hook-perf-budgets-timed sha=863d9e119bb4 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:21-27
quote: pub const ROWS: [&str; 5] = [ "session-start", "user-prompt-submit", "stop", "session-end", "pre-tool-use", ];
at: crates/viola-e2e/src/harness/run/perf.rs:180
quote: stamp: false,
at: crates/viola-e2e/src/harness/gate.rs:131
quote: for row in PERF_ROWS {
guard_at: crates/viola-e2e/src/harness/gate.rs:130
@row_add n=282 row=decision/perf-gate-max-sample area=10-slo-invariants-telemetry-budgets rec=fct-onnnq5uj@1,fct-xa6ldddm@1
name: Performance budgets gate the max sample
rule: Every performance budget gates hyperfine's max sample; p99 is not computed.
why: max is an upper bound on every quantile
guard: gate --require perf
@row_check n=282 row=decision/perf-gate-max-sample sha=7e89961900b5 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:149
quote: .and_then(|v| v["results"][0]["max"].as_f64());
searched: p99|percentile · in *.rs (no match in the tree)
guard_at: crates/viola-e2e/src/harness/gate.rs:130
@row_add n=283 row=decision/session-end-hook-budget area=10-slo-invariants-telemetry-budgets rec=fct-onnnq5uj@1
name: session-end hook budget
rule: viola hook session-end completes in under 1.0 s.
guard: gate --require perf
@row_check n=283 row=decision/session-end-hook-budget sha=ec98485b4c30 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:151
quote: Some(max) if max < bound => {}
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
guard_at: crates/viola-e2e/src/harness/gate.rs:130
@row_add n=284 row=decision/spine-hook-budget area=10-slo-invariants-telemetry-budgets rec=fct-onnnq5uj@1
name: Spine hook budget
rule: The spine hooks and pre-tool-use on an unverified CLI complete under the tests-owned spine deadline, provisionally 1.0 s.
guard: gate --require perf
state: provisional
@row_check n=284 row=decision/spine-hook-budget sha=d7706f6d9b0e v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:145
quote: let bound = viola_core::SPINE_DEADLINE.as_secs_f64();
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
guard_at: crates/viola-e2e/src/harness/gate.rs:130
@row_add n=285 row=decision/no-request-ratio-slo area=10-slo-invariants-telemetry-budgets rec=fct-d4bn7qgd@1
name: No request-ratio SLO
rule: There is no request-ratio SLO; the error budget is a set of zero-tolerance counts.
why: viola is a local tool with no deploys
@row_check n=285 row=decision/no-request-ratio-slo sha=611c748a2435 v=holds judge=sonnet head=8834195b0683
searched: (?i)\bslo\b|error.ratio|request.ratio|burn.rate|error.budget · in src/**, crates/*/src/**, scripts/**, .github/**, .config/**
@row_add n=286 row=decision/error-budget-zero-tolerance area=10-slo-invariants-telemetry-budgets rec=fct-zze2ezbr@1
name: Zero-tolerance error budget
rule: The error budget tolerates zero panic lines in any CI artifact, zero secret-scan hits and zero schema-conformance failures.
except: the fake-agent seam's deliberate panic, as the zero-panics gate counts it
@row_check n=286 row=decision/error-budget-zero-tolerance sha=a2fb76d77603 v=holds judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:10
quote: seam_file="src/cmd/hook/seam.rs"
at: .github/workflows/ci.yml:64
quote: run: bash scripts/agent-run.sh schema-check
at: .github/workflows/ci.yml:76
quote: run: bash scripts/agent-run.sh secret-scan
@row_add n=287 row=decision/error-budget-measured-by-gate-exits area=10-slo-invariants-telemetry-budgets rec=fct-2v5pxi7x@1
name: Error budget measured by in-job gate exit codes
rule: The error budget is measured by in-job gate exit codes, not by the uploaded artifact: panics by G2, schema failures by G4, secret hits by the secret-scan step.
why: the diagnostics artifact is not uploaded when the scan fails, so it is the diagnosis input, not the measurement surface
@row_check n=287 row=decision/error-budget-measured-by-gate-exits sha=e168756c2e97 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:59
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
at: .github/workflows/ci.yml:64
quote: run: bash scripts/agent-run.sh schema-check
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
@row_add n=288 row=decision/artifact-panic-count-excludes-detail area=10-slo-invariants-telemetry-budgets rec=fct-2v5pxi7x@1
name: Artifact panic count excludes detail files
rule: A panic count over a downloaded diagnostics artifact excludes detail files.
why: detail files repeat the panic line
@row_check n=288 row=decision/artifact-panic-count-excludes-detail sha=1499c04f6f17 v=holds judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:23
quote: find "$scope/" -path '*/diagnostics/*.ndjson' ! -name 'detail-*' -exec awk 1 {} + \
@row_add n=289 row=decision/nonzero-count-blocks-chunk area=10-slo-invariants-telemetry-budgets rec=fct-e2fzkaej@1
name: Nonzero error count blocks the chunk
rule: Any nonzero error-budget count makes the chunk red until the root cause is fixed within that chunk.
why: the tests quarantine rule
@row_check n=289 row=decision/nonzero-count-blocks-chunk sha=2d895605e4e2 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=290 row=absence/on-call-paging area=10-slo-invariants-telemetry-budgets rec=fct-e2fzkaej@1
capability: On-call or paging
@row_check n=290 row=absence/on-call-paging sha=987dfa1b89fc v=holds judge=sonnet head=8834195b0683
searched: on-call|oncall|pagerduty|paging (case-insensitive) · in **/*.{rs,toml,sh,yml,yaml}
@row_add n=291 row=decision/hyperfine-max-regression-fails area=10-slo-invariants-telemetry-budgets rec=fct-y4gvhk7p@1
name: Hyperfine max regression is a failure
rule: A hyperfine max regression beyond the telemetry latency budgets counts as an SLO invariant failure.
@row_check n=291 row=decision/hyperfine-max-regression-fails sha=04b697b41447 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:152-155
quote: Some(max) => breaches.push(breach( "perf", "perf", format!("{file} max {max} >= {bound}"),
@row_add n=292 row=decision/secret-scan-hit-fails area=10-slo-invariants-telemetry-budgets rec=fct-ydzhack5@2
name: Secret-scan hit is a failure
rule: A secret-scan hit counts as an SLO invariant failure.
@row_check n=292 row=decision/secret-scan-hit-fails sha=b5a22272b0ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:90
quote: let ok = !empty && hits.is_empty();
@row_add n=293 row=decision/diagnostics-files-mode-0600 area=10-slo-invariants-telemetry-budgets rec=fct-ydzhack5@2
name: Diagnostics files are 0600
rule: A diagnostics file whose mode is not 0600 counts as an SLO invariant failure.
@row_check n=293 row=decision/diagnostics-files-mode-0600 sha=0778928523cf v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:180
quote: mode & 0o777 == 0o600
@row_add n=294 row=decision/schema-conformance-failure-fails area=10-slo-invariants-telemetry-budgets rec=fct-j3ksxxc2@1
name: Schema-conformance failure is a failure
rule: An unknown event value, a missing required field or a field outside the catalog counts as an SLO invariant failure.
@row_check n=294 row=decision/schema-conformance-failure-fails sha=b3be707e29ab v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:132
quote: let ok = !empty && failures.is_empty();
at: schemas/diag-line.v1.json:106
quote: "unevaluatedProperties": false,
@row_add n=295 row=decision/print-and-macro-lint-failure-fails area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: Print and macro lint failure is a failure
rule: A clippy disallowed-macros, print_stdout, print_stderr or dbg_macro failure counts as an SLO invariant failure.
@row_check n=295 row=decision/print-and-macro-lint-failure-fails sha=9041b433df60 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:281
quote: run: cargo clippy --workspace --all-targets --features fake-agent -- -D warnings
at: Cargo.toml:256
quote: print_stdout = "deny"
at: Cargo.toml:258
quote: dbg_macro = "deny"
@row_add n=296 row=decision/lint-probes-failure-fails area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: Lint-probes failure is a failure
rule: A lint-probes failure (a ban that did not fire, a control that did not pass, or a raw event! outside the core obs module) counts as an SLO invariant failure.
@row_check n=296 row=decision/lint-probes-failure-fails sha=23c1ffa6bb94 v=holds judge=sonnet head=8834195b0683
at: scripts/lint-probes.sh:163-164
quote: echo "lint-probes: FAILED — $fired/${#bans[@]} bans fired, $clean/${#controls[@]} controls clean, raw-event grep $([ "$grep_ok" -eq 1 ] && echo clean || echo red); failing: ${failing[*]:-none}" exit 1
at: .github/workflows/ci.yml:304
quote: run: bash scripts/lint-probes.sh
@row_add n=297 row=decision/g3-panic-abort-gate area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: G3 panic-abort gate
rule: A G3 hit on panic = "abort" counts as an SLO invariant failure.
guard: G3
@row_check n=297 row=decision/g3-panic-abort-gate sha=6bc8bc6de11d v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:301
quote: set +e; rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' "(panic|_PANIC)\s*[:=]\s*[\"']?abort" .; test $? -eq 1
guard_at: .github/workflows/ci.yml:298
@row_add n=298 row=decision/grep-gates-fail-closed area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: Grep gates fail closed
rule: The G1 and G3 gates fail on any rg exit other than 1, including exit 2 on a regex or I/O error and 127 on a missing rg.
@row_check n=298 row=decision/grep-gates-fail-closed sha=c0d7cbcf164d v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:297
quote: set +e; rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .; test $? -eq 1
at: .github/workflows/ci.yml:301
quote: set +e; rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' "(panic|_PANIC)\s*[:=]\s*[\"']?abort" .; test $? -eq 1
@row_add n=299 row=decision/cargo-deny-failure-fails area=10-slo-invariants-telemetry-budgets rec=fct-5kn2jpbt@1
name: cargo deny failure is a failure
rule: A cargo deny failure (Tokio, a C crate, veil toggle, banned obs crates) counts as an SLO invariant failure.
guard: cargo deny
@row_check n=299 row=decision/cargo-deny-failure-fails sha=7d0272db57bf v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:367
quote: cargo deny --format json check 2> target/supply-chain/deny.json || { cat target/supply-chain/deny.json; exit 1; }
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
at: deny.toml:40
quote: deny = ["toggle"]
at: deny.toml:32
quote: { crate = "opentelemetry-otlp", reason = "no telemetry exporter or egress in v1" },
guard_at: .github/workflows/ci.yml:363
@row_add n=300 row=decision/surviving-obs-mutant-fails area=10-slo-invariants-telemetry-budgets rec=fct-o5c3uxju@1
name: Surviving obs mutant is a failure
rule: A surviving cargo-mutants mutant in obs code at a run --mutants (the epoch-boundary audit or on demand) counts as an SLO invariant failure.
except: A missed mutant the host never compiles is not counted as surviving.
@row_check n=300 row=decision/surviving-obs-mutant-fails sha=f312a9b4eb40 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:96-97
quote: if suite == "mutants" && survived > 0 { breaches.push(breach("mutants", suite, format!("survived {survived}")));
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
@row_add n=301 row=decision/no-hot-path-instrumentation area=11-obs-anti-patterns-never-do-these rec=fct-75yvf2rf@1
name: No hot-path instrumentation
rule: The PTY pump read loop and the vt100 feed get no per-byte or per-chunk span or event; only gate outcomes are logged.
@row_check n=301 row=decision/no-hot-path-instrumentation sha=9127bd8a8bbd v=holds judge=sonnet head=8834195b0683
searched: tracing::(info|debug|trace|warn|error)!|\b(info|debug|trace|warn|error)!\( · in crates/viola-pty/src
searched: obs_event! · in crates/viola-pty/src (the only span there is the paste write, not the read loop: crates/viola-pty/src/pump.rs:72)
@row_add n=302 row=decision/must-trace-paths-always-traced area=11-obs-anti-patterns-never-do-these rec=fct-3si6piva@1
name: Must-trace paths always traced
rule: Spans and events are never skipped on the 6 must-trace paths.
why: The agent loses its ability to debug without them.
@row_check n=302 row=decision/must-trace-paths-always-traced sha=40cf90f13a57 v=unverifiable judge=sonnet head=8834195b0683
searched: must-trace · in src/**/*.rs crates/**/*.rs
why: no evidence found (the 6 must-trace paths are named only in the plan, not in code)
@row_add n=303 row=decision/no-otel-without-decisions-log area=11-obs-anti-patterns-never-do-these rec=fct-n47oemak@1
name: No OTel without a Decisions Log entry
rule: No OTel SDK, OTLP exporter or opentelemetry-stdout is added without a Decisions Log entry.
why: opentelemetry-otlp's default reqwest-blocking-client spins up a Tokio runtime.
@row_check n=303 row=decision/no-otel-without-decisions-log sha=c596953d5c97 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry|otlp · in Cargo.toml and crates/*/Cargo.toml
searched: opentelemetry|otlp|Meter · in src/**/*.rs crates/**/*.rs
@row_add n=304 row=decision/no-otel-stdout-exporter area=11-obs-anti-patterns-never-do-these rec=fct-n47oemak@1
name: OTel stdout exporter inadmissible
rule: The OTel stdout exporter is inadmissible on every role.
why: stdout is reserved.
@row_check n=304 row=decision/no-otel-stdout-exporter sha=29db3688f527 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:128
quote: .with_writer(writer);
searched: opentelemetry-stdout|opentelemetry · in Cargo.toml and crates/*/Cargo.toml
@row_add n=305 row=decision/no-telemetry-from-screen-content area=11-obs-anti-patterns-never-do-these rec=fct-gr6xdpkj@1
name: No telemetry from screen content
rule: Telemetry is never derived from screen content; vt100 is used for readiness and modal gate outcomes only.
@row_check n=305 row=decision/no-telemetry-from-screen-content sha=faba8882931c v=holds judge=sonnet head=8834195b0683
searched: obs_event!|instrument|tracing:: · in crates/viola-agent-claude/src/screen.rs
@row_add n=306 row=decision/transcripts-not-a-contract area=11-obs-anti-patterns-never-do-these rec=fct-gr6xdpkj@1
name: Claude Code transcripts are not a contract
rule: Claude Code transcripts are never read as a contract.
@row_check n=306 row=decision/transcripts-not-a-contract sha=1d68711bf4ae v=holds judge=sonnet head=8834195b0683
searched: transcript · in src/**/*.rs crates/**/*.rs (only a fake-agent payload key and a test fixture)
@row_add n=307 row=decision/no-pii-in-span-fields area=11-obs-anti-patterns-never-do-these rec=fct-om4ml4hz@1
name: No PII or content in span fields
rule: Span fields never include PII or content.
why: Scrubbing at an exporter stage is too late, and v1 has no exporter stage at all.
@row_check n=307 row=decision/no-pii-in-span-fields sha=d925ff4c07c5 v=unverifiable judge=sonnet head=8834195b0683
searched: #\[(tracing::)?instrument · in src/**/*.rs crates/**/*.rs
why: no evidence found (fields seen, e.g. text_bytes, carry no content, but not every span field was checked)
@row_add n=308 row=absence/telemetry-exporter-stage area=11-obs-anti-patterns-never-do-these rec=fct-om4ml4hz@1
capability: A telemetry exporter stage
@row_check n=308 row=absence/telemetry-exporter-stage sha=68f26cac6a23 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry|otlp|exporter · in Cargo.toml and crates/*/Cargo.toml
searched: tracing-appender|opentelemetry · in src/**/*.rs crates/**/*.rs
@row_add n=309 row=decision/no-bare-instrument area=11-obs-anti-patterns-never-do-these rec=fct-3n7snwrw@1
name: No bare #[instrument]
rule: Instrumentation always uses #[instrument(skip_all, fields(...))], never a bare #[instrument].
why: A bare #[instrument] records every argument with Debug and leaks send text, hook.dialog payloads and tool input.
@row_check n=309 row=decision/no-bare-instrument sha=f3bb1e730c2b v=holds judge=sonnet head=8834195b0683
searched: #\[(tracing::)?instrument\] · in src/**/*.rs crates/**/*.rs
searched: instrument\] · in src/**/*.rs crates/**/*.rs
@row_add n=310 row=decision/no-instrument-err-ret-on-sensitive area=11-obs-anti-patterns-never-do-these rec=fct-odnuwe7o@1
name: No instrument err or ret on sensitive results
rule: #[instrument(err)] and #[instrument(ret)] are never used where the error or return can carry a serde or serde_path_to_error source or user content.
@row_check n=310 row=decision/no-instrument-err-ret-on-sensitive sha=e94384e8f2fd v=holds judge=sonnet head=8834195b0683
searched: instrument\((err|ret)|, ?(err|ret)\b · in src/**/*.rs crates/**/*.rs
@row_add n=311 row=decision/low-cardinality-span-names area=11-obs-anti-patterns-never-do-these rec=fct-zgyunnhf@1
name: Low-cardinality span names
rule: Span names never contain instance names, dialog ids or paths.
@row_check n=311 row=decision/low-cardinality-span-names sha=79591c2e2dec v=holds judge=sonnet head=8834195b0683
searched: name = format|name = &|name = concat · in src/**/*.rs crates/**/*.rs
@row_add n=312 row=decision/explicit-corr-conn-instance area=11-obs-anti-patterns-never-do-these rec=fct-stl2zgws@1
name: corr, conn and instance passed explicitly
rule: corr, conn and instance are passed as explicit values across threads or tasks, never carried by span context.
why: Spans are not serialized.
@row_check n=312 row=decision/explicit-corr-conn-instance sha=088b3d90fc1c v=unverifiable judge=sonnet head=8834195b0683
searched: corr · in src/obs.rs crates/viola-core/src/obs.rs
why: no evidence found
@row_add n=313 row=decision/no-in-process-metrics area=11-obs-anti-patterns-never-do-these rec=fct-npbjgxck@1
name: No in-process metrics instruments
rule: No in-process metrics instruments (opentelemetry meters, metrics crate) are created.
why: There is no reader, exporter or listener to flush them.
@row_check n=313 row=decision/no-in-process-metrics sha=81bb8c88bbd4 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry|metrics · in Cargo.toml and crates/*/Cargo.toml
searched: opentelemetry|metrics::|counter!|histogram!|gauge!|Meter · in src/**/*.rs crates/**/*.rs
@row_add n=314 row=decision/no-telemetry-endpoint area=11-obs-anti-patterns-never-do-these rec=fct-khkx5tob@1
name: No telemetry endpoint without a security decision
rule: No /metrics or other telemetry endpoint is added without a Security Decisions Log entry.
why: A telemetry endpoint is a new listener.
@row_check n=314 row=decision/no-telemetry-endpoint sha=6225846b9f15 v=holds judge=sonnet head=8834195b0683
searched: /metrics|axum|hyper|tiny_http|reqwest · in src/**/*.rs crates/**/*.rs Cargo.toml
@row_add n=315 row=decision/derived-metrics-by-closed-enums area=11-obs-anti-patterns-never-do-these rec=fct-gy4gdls6@1
name: Derived metrics grouped by closed enums
rule: Derived metrics are grouped by the closed enums only, never by unbounded keys such as path, corr, conn, pid or instance.
@row_check n=315 row=decision/derived-metrics-by-closed-enums sha=5f45add5edb9 v=unverifiable judge=sonnet head=8834195b0683
searched: derived metric|group_by · in src/**/*.rs crates/**/*.rs
why: no evidence found
@row_add n=316 row=decision/latency-with-gate-statistic area=11-obs-anti-patterns-never-do-these rec=fct-733e2isu@1
name: Latency reported with the gate statistic
rule: Latency is never reported without the gate statistic; p50 and p95 are report-only.
@row_check n=316 row=decision/latency-with-gate-statistic sha=bef2725bb9bd v=unverifiable judge=sonnet head=8834195b0683
searched: p50|p95|percentile · in src/**, crates/**, scripts/**, tests/**
why: a reporting rule; no code path shows latency being reported without the statistic (only the gate on max is visible, and no p50/p95 string exists)
@row_add n=317 row=decision/hyperfine-max-is-enforced area=11-obs-anti-patterns-never-do-these rec=fct-733e2isu@1
name: Hyperfine max is the enforced number
rule: The max from hyperfine is the enforced latency number.
@row_check n=317 row=decision/hyperfine-max-is-enforced sha=afbc12e158ef v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:149
quote: .and_then(|v| v["results"][0]["max"].as_f64());
at: crates/viola-e2e/src/harness/gate.rs:151
quote: Some(max) if max < bound => {}
@row_add n=318 row=decision/no-telemetry-on-stdout area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
name: No telemetry on stdout
rule: Telemetry is never written to stdout on any role.
why: stdout carries each role's own output and run's terminal carries the child's screen.
@row_check n=318 row=decision/no-telemetry-on-stdout sha=52d15d4c81e8 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:128
quote: .with_writer(writer);
at: Cargo.toml:256
quote: print_stdout = "deny"
@row_add n=319 row=contract/stdout-by-role/cli area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
boundary: stdout by role
item: cli
promise: stdout carries the --json results.
@row_check n=319 row=contract/stdout-by-role/cli sha=60d4ddf18355 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:221
quote: let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
@row_add n=320 row=contract/stdout-by-role/hook area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
boundary: stdout by role
item: hook
promise: stdout carries the decision body or, for hook statusline, the user's status-line output.
@row_check n=320 row=contract/stdout-by-role/hook sha=9474644d1d6f v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:104
quote: answer(event, dir, answers, &filed, &mut std::io::stdout().lock());
at: src/cmd/hook/statusline.rs:55
quote: let out = &mut std::io::stdout().lock();
@row_add n=321 row=contract/stdout-by-role/mcp area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
boundary: stdout by role
item: mcp
promise: stdout carries MCP frames.
@row_check n=321 row=contract/stdout-by-role/mcp sha=e0068eccc3a0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:76
quote: let (stdin, stdout) = rmcp::transport::stdio();
@row_add n=322 row=contract/stdout-by-role/run area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
boundary: stdout by role
item: run
promise: The terminal of run carries the child's screen.
@row_check n=322 row=contract/stdout-by-role/run sha=d3b088c78742 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
@row_add n=323 row=decision/explicit-subscriber-writer area=11-obs-anti-patterns-never-do-these rec=fct-va5nveqe@1
name: Explicit tracing-subscriber writer
rule: with_writer is always set explicitly on the tracing-subscriber.
why: The default tracing-subscriber writer is stdout.
@row_check n=323 row=decision/explicit-subscriber-writer sha=897b462591ab v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:128
quote: .with_writer(writer);
@row_add n=324 row=decision/log-internal-errors-off area=11-obs-anti-patterns-never-do-these rec=fct-rd5bgb55@1
name: log_internal_errors turned off
rule: log_internal_errors is never left at its default true.
why: Writer failures would eprintln to stderr and break the Hook contract and the run terminal.
@row_check n=324 row=decision/log-internal-errors-off sha=955e439e8991 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:126
quote: .log_internal_errors(false)
@row_add n=325 row=decision/obs-event-macro-only area=11-obs-anti-patterns-never-do-these rec=fct-aemh5hkn@1
name: obs_event! is the only event macro
rule: tracing's event and level macros are never called directly outside viola_core::obs; events go through obs_event!.
why: Only obs_event! guarantees event, process and instance.
guard: clippy disallowed-macros; the raw-event! grep in lint-probes
@row_check n=325 row=decision/obs-event-macro-only sha=71b6a539e341 v=holds guard=true judge=sonnet head=8834195b0683
at: clippy.toml:5
quote: { path = "tracing::info", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: scripts/lint-probes.sh:128
quote: '\bevent!\s*[({[]' .) || rc=$?
guard_at: clippy.toml:4
@row_add n=326 row=decision/corr-set-explicitly-by-caller area=11-obs-anti-patterns-never-do-these rec=fct-aemh5hkn@1
name: corr set explicitly by the caller
rule: The caller sets corr explicitly as a typed field on the events that carry one.
@row_check n=326 row=decision/corr-set-explicitly-by-caller sha=a1d168def50e v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:148
quote: $($key = $value,)*
@row_add n=327 row=decision/no-stringified-corr area=11-obs-anti-patterns-never-do-these rec=fct-dhc5cdwv@1
name: corr never recorded with ? or %
rule: corr is never recorded with ?corr or %corr.
why: Those stringify numbers.
@row_check n=327 row=decision/no-stringified-corr sha=b5dde2a0b825 v=holds judge=sonnet head=8834195b0683
searched: \?corr|%corr|corr = [?%] · in src/**, crates/**, scripts/**, tests/**
@row_add n=328 row=decision/no-env-log-level area=11-obs-anti-patterns-never-do-these rec=fct-mk3zdfg7@1
name: Log level not read from the environment
rule: The level is never read from RUST_LOG, the env-filter feature is never enabled and EnvFilter::from_default_env is never called.
why: Env vars are not a configuration channel.
@row_check n=328 row=decision/no-env-log-level sha=0ad7493e93ef v=holds judge=sonnet head=8834195b0683
searched: from_default_env · in **/*.rs, **/Cargo.toml, **/*.sh
searched: env-filter · in **/Cargo.toml (workspace tracing-subscriber features are fmt, json, registry, std only; Cargo.toml:243)
searched: RUST_LOG · in **/*.rs (only a test env set at tests/run_cli.rs:338 and a comment at src/obs.rs:180 saying never RUST_LOG)
@row_add n=329 row=decision/one-line-panics area=11-obs-anti-patterns-never-do-these rec=fct-fgn7d524@1
name: Panics are one line
rule: Multi-line stack traces are never written; panics are logged as one line.
@row_check n=329 row=decision/one-line-panics sha=aaa43b8327df v=holds judge=sonnet head=8834195b0683
at: src/main.rs:191
quote: let _ = (&*sink.file).write_all(line.as_bytes());
@row_add n=330 row=decision/backtraces-in-instance-detail-only area=11-obs-anti-patterns-never-do-these rec=fct-fgn7d524@1
name: Backtraces in the instance detail file only
rule: Backtraces are written as one-line JSON arrays in the instance detail file only.
@row_check n=330 row=decision/backtraces-in-instance-detail-only sha=6eca0d38899a v=holds judge=sonnet head=8834195b0683
at: src/main.rs:221
quote: fields.insert("backtrace".to_owned(), Value::Array(backtrace));
@row_add n=331 row=decision/no-info-in-hot-loops area=11-obs-anti-patterns-never-do-these rec=fct-thq5vza6@1
name: No info logging in hot loops
rule: Nothing is logged at info inside hot loops such as the PTY pump; counts go on the boundary event instead.
@row_check n=331 row=decision/no-info-in-hot-loops sha=6486a0252888 v=holds judge=sonnet head=8834195b0683
searched: obs_event!|Level::INFO|INFO, · in crates/viola-pty (the PTY pump crate; no match)
@row_add n=332 row=decision/no-non-blocking-appender area=11-obs-anti-patterns-never-do-these rec=fct-v6m4e2mq@1
name: No non_blocking appender
rule: tracing-appender non_blocking is never used.
why: It drops lines when full, needs a WorkerGuard flush, and breaks the hook's exit-0 / max < 1.0 s gate and the zero-unlogged-panics invariant.
@row_check n=332 row=decision/no-non-blocking-appender sha=61023b625b89 v=holds judge=sonnet head=8834195b0683
searched: non_blocking · in **/*.rs, **/Cargo.toml, **/*.sh
searched: tracing-appender · in **/Cargo.toml
@row_add n=333 row=decision/log-field-names-frozen area=11-obs-anti-patterns-never-do-these rec=fct-v7dbdmio@1
name: Log field names frozen
rule: Log fields such as corr and event are never renamed without a Decisions Log entry.
@row_check n=333 row=decision/log-field-names-frozen sha=d7ae92d46c1d v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim; a Decisions Log process rule
@row_add n=334 row=decision/event-values-closed area=11-obs-anti-patterns-never-do-these rec=fct-v7dbdmio@1
name: event values closed
rule: No event value is added without a Decisions Log entry.
@row_check n=334 row=decision/event-values-closed sha=b23574f0fff1 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim; a Decisions Log process rule (the code only holds the closed ObsEvent enum)
@row_add n=335 row=decision/no-log-bridge area=11-obs-anti-patterns-never-do-these rec=fct-vdlxpzho@1
name: No log bridge
rule: tracing_log::LogTracer or any log bridge is never installed by hand, and tracing-log stays off.
why: Bridged records lack event, corr, process and instance.
@row_check n=335 row=decision/no-log-bridge sha=0dba8f0f3490 v=holds judge=sonnet head=8834195b0683
searched: LogTracer · in **/*.rs, **/Cargo.toml
searched: tracing-log · in **/Cargo.toml (tracing-subscriber at Cargo.toml:243 has default-features = false, features fmt, json, registry, std)
@row_add n=336 row=decision/no-error-reporter-in-v1 area=11-obs-anti-patterns-never-do-these rec=fct-fvlnws45@1
name: No error reporter in v1
rule: sentry or any other reporter is never added in v1.
why: Egress is banned, and sentry's default native-tls / tokio transport breaks cargo-deny.
@row_check n=336 row=decision/no-error-reporter-in-v1 sha=4c123989bae9 v=holds judge=sonnet head=8834195b0683
at: deny.toml:34
quote: { crate = "sentry", reason = "no error reporter or egress in v1" },
searched: sentry · in Cargo.toml and crates/**/Cargo.toml (no dependency)
@row_add n=337 row=decision/no-serde-source-in-error-output area=11-obs-anti-patterns-never-do-these rec=fct-qeuz4fkn@1
name: No serde source in printed errors
rule: An anyhow chain that still contains a serde_json or serde_path_to_error source is never printed to stderr, --json, MCP isError or channel error.data.
@row_check n=337 row=decision/no-serde-source-in-error-output sha=0da5dfe01d14 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:254-255
quote: /// The catch site's records for a dispatch error: the codes-only role line, and the chain only in /// the instance detail file (obs-plan §7); nothing reaches stdout or stderr.
searched: {:#} · in src/**/*.rs
searched: isError · in src/**/*.rs
searched: (eprintln|println)!\(.*(err|error|e)\b · in src/**/*.rs
@row_add n=338 row=decision/default-panic-hook-never-runs area=11-obs-anti-patterns-never-do-these rec=fct-murrb5aj@1
name: Default panic hook never runs
rule: The default panic hook is never called or allowed to run.
why: Its stderr output breaks hook and the run terminal.
@row_check n=338 row=decision/default-panic-hook-never-runs sha=c2542fdf543e v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
searched: take_hook · in src/**/*.rs (only a test at src/main.rs:593)
@row_add n=339 row=decision/reporter-opt-in-only area=11-obs-anti-patterns-never-do-these rec=fct-fb7hrxpx@1
name: Any future reporter is opt-in
rule: Any future reporter is opt-in, never enabled by env var or by default.
@row_check n=339 row=decision/reporter-opt-in-only sha=9736ed65b006 v=holds judge=sonnet head=8834195b0683
searched: sentry · in **/*.toml and src/**/*.rs (only the ban at deny.toml:34)
searched: before_send · in src/**/*.rs, crates/**/*.rs
@row_add n=340 row=decision/reporter-ureq-rustls area=11-obs-anti-patterns-never-do-these rec=fct-fb7hrxpx@1
name: Future reporter transport is ureq and rustls
rule: Any future reporter uses ureq with rustls.
@row_check n=340 row=decision/reporter-ureq-rustls sha=963fcb5d027e v=unverifiable judge=sonnet head=8834195b0683
searched: ureq · in **/*.toml and src/**/*.rs (no match)
searched: rustls · in **/*.toml and src/**/*.rs (no match)
why: not a code claim; the rule concerns a future reporter, and no reporter, ureq or rustls exists in the tree to show it either way
@row_add n=341 row=decision/reporter-never-log-floor area=11-obs-anti-patterns-never-do-these rec=fct-fb7hrxpx@1
name: Future reporter applies the NEVER-log floor
rule: Any future reporter applies the NEVER-log floor in before_send.
@row_check n=341 row=decision/reporter-never-log-floor sha=df8de9b27135 v=holds judge=sonnet head=8834195b0683
searched: before_send · in src/**/*.rs, crates/**/*.rs (no match; no reporter exists)
@row_add n=342 row=decision/file-sink-only-error-sink area=11-obs-anti-patterns-never-do-these rec=fct-5ak67rzk@1
name: File sink is the only error sink
rule: Local logs are never optional: the file sink is the only error sink and works with every other feature off.
@row_check n=342 row=decision/file-sink-only-error-sink sha=c54ee1486a6f v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:260
quote: write_detail(&sink.home, &sink.instance, sink.process, &line);
@row_add n=343 row=decision/field-allow-list-default-deny area=11-obs-anti-patterns-never-do-these rec=fct-vq5djl4b@1
name: Field allow-list is default-deny
rule: No field is logged unless named in the field catalog.
@row_check n=343 row=decision/field-allow-list-default-deny sha=d7383300543e v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:106
quote: "unevaluatedProperties": false,
@row_add n=344 row=decision/veil-toggle-banned area=11-obs-anti-patterns-never-do-these rec=fct-u3w4pedj@1
name: veil toggle feature banned
rule: veil's toggle feature (VEIL_DISABLE_REDACTION / veil::disable()) is never enabled.
guard: cargo-deny
@row_check n=344 row=decision/veil-toggle-banned sha=58e590b76ab9 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:40
quote: deny = ["toggle"]
guard_at: deny.toml:39
@row_add n=345 row=decision/allow-list-not-widenable area=11-obs-anti-patterns-never-do-these rec=fct-fruyyip2@1
name: Allow-list and redaction not configurable
rule: config.json, a VIOLA_* env var or a CLI flag never widens the field allow-list or disables redaction.
@row_check n=345 row=decision/allow-list-not-widenable sha=4e80cf6ea486 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:32-39
quote: const CONFIG_KEYS: [&str; 6] = [ "v", "diagnostics_level", "claude_env_keep", "five_hour", "seven_day", "dialog_hold_s", ];
searched: VIOLA_ · in src/obs.rs (only VIOLA_TARGETS, a target list)
@row_add n=346 row=decision/verbosity-changes-volume-only area=11-obs-anti-patterns-never-do-these rec=fct-fruyyip2@1
name: Verbosity changes volume only
rule: diagnostics_level changes the volume of logging only.
@row_check n=346 row=decision/verbosity-changes-volume-only sha=2314222b49c5 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:202-204
quote: None | Some(Some("info")) => (Level::INFO, false), Some(Some("debug")) => (Level::DEBUG, false), Some(_) => (Level::INFO, true),
@row_add n=347 row=decision/claude-env-and-cookie-never-logged area=11-obs-anti-patterns-never-do-these rec=fct-vnsyc5om@1
name: CLAUDE* values and Cookie never logged
rule: CLAUDE* values and a Cookie header are never logged, at any level, in any file, detail files included.
@row_check n=347 row=decision/claude-env-and-cookie-never-logged sha=84b09b394a47 v=holds judge=sonnet head=8834195b0683
searched: Cookie|cookie · in **/*.rs (only the secret-scan harness)
searched: CLAUDE · in src/obs.rs (no match)
searched: CLAUDE · in src/run/env.rs (names only filtered, never logged)
@row_add n=348 row=decision/sensitive-content-detail-file-only area=11-obs-anti-patterns-never-do-these rec=fct-vnsyc5om@1
name: Sensitive content only in the instance detail file
rule: statusline_command, send text, tool input and last_assistant_message never reach a home-level file; they may reach only the instance-scoped detail-<process>.ndjson, inside a chain or drift report.
@row_check n=348 row=decision/sensitive-content-detail-file-only sha=5dc935dda538 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:557
quote: obs::write_detail(&instance.home, &instance.name, ObsProcess::Hook, &line);
searched: last_assistant_message|statusline_command · in src/obs.rs (no match)
@row_add n=349 row=decision/no-paths-or-pids-in-detail-codes area=11-obs-anti-patterns-never-do-these rec=fct-hl4ehpjw@1
name: No paths or pids in detail codes
rule: detail codes never contain paths or pids.
@row_check n=349 row=decision/no-paths-or-pids-in-detail-codes sha=85ce542013e7 v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:77
quote: "detail": { "enum": ["already-live", "squatted-name", "pinned-hash-mismatch", "internal-error", "instance-dead", "strict-modes-failed", "server-verify-failed", "socket-dir-refused", "wrapper-fault", "cwd-
@row_add n=350 row=decision/paths-only-in-instance-detail-files area=11-obs-anti-patterns-never-do-these rec=fct-hl4ehpjw@1
name: Paths only in instance detail files
rule: Paths appear only in instance detail files.
@row_check n=350 row=decision/paths-only-in-instance-detail-files sha=b40d5b9e468f v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:288
quote: pub(crate) fn detail_path(home: &Path, instance: &ViolaName, process: ObsProcess) -> PathBuf {
at: src/obs.rs:292
quote: .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=351 row=decision/upload-only-after-secret-scan area=11-obs-anti-patterns-never-do-these rec=fct-qteviymq@1,fct-ntdd4mdw@1
name: Diagnostics uploaded only after the secret scan
rule: diagnostics artifacts are never uploaded before the secret-scan step passes; uploads run if: always() / failure() combined with the secret-scan step's success, never bare.
@row_check n=351 row=decision/upload-only-after-secret-scan sha=3b07bc9c38eb v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:87
quote: if: failure() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:106
quote: if: always() && steps.secret-scan.outcome == 'failure'
@row_add n=352 row=decision/failing-job-diagnostics-kept area=11-obs-anti-patterns-never-do-these rec=fct-ntdd4mdw@1
name: Failing job's diagnostics kept
rule: A failing job's diagnostics are never lost: G2, G4 and the secret scan run if: always().
@row_check n=352 row=decision/failing-job-diagnostics-kept sha=f2a3d0c54db6 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:57
quote: if: always()
at: .github/workflows/ci.yml:62
quote: if: always()
at: .github/workflows/ci.yml:74
quote: if: always()
@row_add n=353 row=decision/failed-scan-uploads-hit-report area=11-obs-anti-patterns-never-do-these rec=fct-ntdd4mdw@1
name: Failed scan uploads the hit report
rule: On a failed secret scan the hit report is uploaded instead of the diagnostics.
@row_check n=353 row=decision/failed-scan-uploads-hit-report sha=49447b9e3fef v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:106
quote: if: always() && steps.secret-scan.outcome == 'failure'
@row_add n=354 row=decision/rerun-no-substitute area=11-obs-anti-patterns-never-do-these rec=fct-ntdd4mdw@1
name: Re-run is no substitute for diagnostics
rule: A re-run is not a substitute for the failing job's diagnostics.
why: CI runs under retries = 0.
@row_check n=354 row=decision/rerun-no-substitute sha=d65e1d6d9e4a v=unverifiable judge=sonnet head=8834195b0683
searched: retries · in .config/nextest.toml (line 2 reads `retries = 0`, the why only)
why: not a code claim
@row_add n=355 row=decision/machine-asserted-telemetry-verdicts area=11-obs-anti-patterns-never-do-these rec=fct-on5vltm5@1,fct-feqirlyj@1
name: Machine-asserted telemetry verdicts
rule: Log analysis is never human-review-gated; every CI telemetry verdict is a jq or hyperfine assertion that sets an exit code.
@row_check n=355 row=decision/machine-asserted-telemetry-verdicts sha=73f6ef30aca5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:149
quote: .and_then(|v| v["results"][0]["max"].as_f64());
at: .github/workflows/ci.yml:59
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
@row_add n=356 row=decision/no-ci-env-metadata-in-logs area=11-obs-anti-patterns-never-do-these rec=fct-zbnejvt2@1
name: No CI env metadata in product logs
rule: CI env metadata is never injected into product log lines.
why: The product never reads env as configuration.
@row_check n=356 row=decision/no-ci-env-metadata-in-logs sha=d5ac1fa62d3c v=holds judge=sonnet head=8834195b0683
searched: GITHUB_|RUNNER_|std::env::var · in src/**/*.rs (no CI variable read; only VIOLA_NAME, VIOLA_DIR, PATH, NO_COLOR and fake-agent switches)
searched: GITHUB_|RUNNER_ · in crates/viola-core/src
@row_add n=357 row=decision/no-retry-once area=11-obs-anti-patterns-never-do-these rec=fct-pardphhn@1
name: No retry-once policies
rule: No retry-once policy is added for panics or flaky telemetry assertions.
@row_check n=357 row=decision/no-retry-once sha=ff525c27a3bf v=holds judge=sonnet head=8834195b0683
searched: retries|retry · in .config/ , .github/ , scripts/ (only `retries = 0` in .config/nextest.toml:2 and the ripgrep download retry in scripts/install-ripgrep.sh)
@row_add n=358 row=decision/budgets-asserted-by-exit-code area=11-obs-anti-patterns-never-do-these rec=fct-soiwyjd4@1
name: Budgets asserted by exit code
rule: Every budget has an exit-code-setting assertion (hyperfine max via jq).
@row_check n=358 row=decision/budgets-asserted-by-exit-code sha=87a3ce08cfe5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:151
quote: Some(max) if max < bound => {}
at: crates/viola-e2e/src/harness/gate.rs:152
quote: Some(max) => breaches.push(breach(
@row_add n=359 row=decision/no-per-heartbeat-logging area=11-obs-anti-patterns-never-do-these rec=fct-ie2ur45l@1
name: No log line per heartbeat tick
rule: Heartbeats log transitions and lifecycle only, never a line per tick.
why: The log files are never rotated.
@row_check n=359 row=decision/no-per-heartbeat-logging sha=d86d0efad2de v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/heartbeat.rs:42
quote: let _ = touch_heartbeat(&instance_dir);
@row_add n=360 row=decision/no-proprietary-apm-only area=11-obs-anti-patterns-never-do-these rec=fct-df7l3zmj@1
name: No proprietary APM as the only exporter
rule: Proprietary APM (Datadog APM SDK, NewRelic agent, AppDynamics) is never the only exporter.
why: Vendor schemas break paste-to-AI workflows, and viola has no egress at all.
@row_check n=360 row=decision/no-proprietary-apm-only sha=bb4be2f2ac7c v=holds judge=sonnet head=8834195b0683
searched: datadog|newrelic|appdynamics|ddtrace · in Cargo.toml, Cargo.lock, src/**, crates/**, scripts/**, .github/**
@row_add n=361 row=absence/network-egress area=11-obs-anti-patterns-never-do-these rec=fct-df7l3zmj@1
capability: Network egress of any kind
@row_check n=361 row=absence/network-egress sha=773272721c9d v=holds judge=sonnet head=8834195b0683
searched: TcpStream|TcpListener|UdpSocket|reqwest|ureq · in src/**/*.rs, crates/**/*.rs, Cargo.lock
searched: opentelemetry-otlp|sentry (banned) · in deny.toml:32,34
@row_add n=362 row=decision/no-dashboards-only-obs area=11-obs-anti-patterns-never-do-these rec=fct-h4xijqlt@1
name: No dashboards-only observability
rule: Every signal is consumed through agent-run logs / status and jq, never through dashboards as the sole surface.
@row_check n=362 row=decision/no-dashboards-only-obs sha=5979201d526c v=holds judge=sonnet head=8834195b0683
searched: grafana|kibana|datadog · in Cargo.toml, src/**, crates/**, scripts/**, .github/**
@row_add n=363 row=decision/product-is-not-a-telemetry-collector area=11-obs-anti-patterns-never-do-these rec=fct-gd3tf5wl@1
name: Product is not a telemetry collector
rule: viola never self-instruments as if the product were telemetry: events.ndjson is not turned into a collector and no OTLP receiver is added.
why: viola is Normal mode (Founder Direction 3)
@row_check n=363 row=decision/product-is-not-a-telemetry-collector sha=b6490760d940 v=holds judge=sonnet head=8834195b0683
searched: otlp|OtlpReceiver|opentelemetry · in src/**, crates/**/*.rs, Cargo.toml (only deny.toml bans and scripts/deny-probes.sh)
@row_add n=364 row=decision/service-identity-from-constants area=11-obs-anti-patterns-never-do-these rec=fct-jzb7kb3q@1
name: Service identity from compiled constants
rule: Service name and version come from viola-core constants, never from env vars such as OTEL_SERVICE_NAME or OTEL_RESOURCE_ATTRIBUTES; any future OTel Resource starts from an empty builder.
@row_check n=364 row=decision/service-identity-from-constants sha=33a6efb1ffd4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:7
quote: pub const SERVICE_NAME: &str = "viola";
at: crates/viola-core/src/lib.rs:8
quote: pub const VERSION: &str = env!("CARGO_PKG_VERSION");
@row_add n=365 row=decision/scrub-before-bytes-leave-process area=11-obs-anti-patterns-never-do-these rec=fct-cegd5nw3@1
name: Scrubbing before any byte leaves the process
rule: Every byte is scrubbed of secrets before it leaves the process; in v1 the only destination is local files.
guard: the secret-scan test
@row_check n=365 row=decision/scrub-before-bytes-leave-process sha=9fbb77ef8c00 v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:41
quote: reason = "toggle lets VEIL_DISABLE_REDACTION / veil::disable() switch redaction off"
guard_at: crates/viola-e2e/src/harness/secret_scan.rs:242
@row_add n=366 row=decision/ci-artifacts-gated-on-secret-scan area=ci-integration rec=fct-auyn3syc@1,fct-u6lpuily@1,fct-t3izd7ip@1,fct-gsrzhsh7@2
name: CI artifacts uploaded only after a passing secret scan
rule: Diagnostics, harness logs, JUnit XML and perf samples are uploaded on failing runs too, but only when the job's secret scan succeeded; a failed scan withholds every artifact.
why: a bare always() or failure() would also upload after a failed scan
@row_check n=366 row=decision/ci-artifacts-gated-on-secret-scan sha=fd80eefb614f v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:87
quote: if: failure() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:98
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:162
quote: if: always() && steps.secret-scan.outcome == 'success'
@row_add n=367 row=decision/harness-logs-captured-before-scan area=ci-integration rec=fct-u6lpuily@1
name: Harness logs captured before the secret scan
rule: On test failure the agent-run logs and status are captured before the secret scan, so the scan covers them, and they go in an artifact of their own, not the diagnostics artifact.
why: upload artifacts are immutable, so a second upload under the same name conflicts, and overwriting would replace the raw diagnostics
@row_check n=367 row=decision/harness-logs-captured-before-scan sha=d46d25056d7d v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:66
quote: if: failure()
at: .github/workflows/ci.yml:73-76
quote: id: secret-scan if: always() shell: bash run: bash scripts/agent-run.sh secret-scan
at: .github/workflows/ci.yml:90
quote: name: harness-${{ matrix.os }}
@row_add n=368 row=decision/no-raw-tracing-macros area=ci-integration rec=fct-xpak76xj@2
name: No raw tracing macros outside the obs module
rule: Raw tracing info, warn, error, debug and trace macros and raw event! are refused outside viola_core::obs.
guard: clippy -D warnings disallowed-macros; raw-event! grep and ban probes in scripts/lint-probes.sh
@row_check n=368 row=decision/no-raw-tracing-macros sha=51310a232caf v=holds guard=true judge=sonnet head=8834195b0683
at: clippy.toml:5
quote: { path = "tracing::info", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:6
quote: { path = "tracing::warn", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:7
quote: { path = "tracing::error", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:8
quote: { path = "tracing::debug", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:9
quote: { path = "tracing::trace", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: scripts/lint-probes.sh:133
quote: printf '%s\n' "$out" | grep -v '^\./crates/viola-core/src/obs\.rs:' | grep -v '^$' || true
guard_at: .github/workflows/ci.yml:281
@row_add n=369 row=decision/sync-crates-without-tokio area=ci-integration rec=fct-xpak76xj@2
name: Sync crates build without Tokio
rule: The synchronous crates must compile without Tokio.
guard: cargo check of the sync crates without Tokio
@row_check n=369 row=decision/sync-crates-without-tokio sha=9cb1de53955c v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:275
quote: cargo check $(sed 's/^/-p /' scripts/sync-crates.txt)
guard_at: .github/workflows/ci.yml:275
@row_add n=370 row=decision/banned-telemetry-dependencies area=ci-integration rec=fct-xpak76xj@2
name: Banned telemetry and logging dependencies
rule: opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender, the tracing-subscriber env-filter feature and the veil toggle feature are not taken as dependencies.
guard: cargo deny check
@row_check n=370 row=decision/banned-telemetry-dependencies sha=12c2be5c3a3c v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:32
quote: { crate = "opentelemetry-otlp", reason = "no telemetry exporter or egress in v1" },
at: deny.toml:33
quote: { crate = "opentelemetry-stdout", reason = "no telemetry exporter in v1" },
at: deny.toml:34
quote: { crate = "sentry", reason = "no error reporter or egress in v1" },
at: deny.toml:35
quote: { crate = "tracing-appender", reason = "its non_blocking writer drops lines and pulls a vulnerable time range" },
at: deny.toml:50
quote: deny = ["env-filter"]
at: deny.toml:40
quote: deny = ["toggle"]
guard_at: .github/workflows/ci.yml:367
@row_add n=371 row=decision/instrument-requires-skip-all area=ci-integration rec=fct-xpak76xj@2,fct-do3y4vos@1
name: Instrument attribute must skip all arguments
rule: Every #[instrument] in the workspace takes skip_all as its first argument; a bare #[instrument] fails the build.
guard: gate G1
@row_check n=371 row=decision/instrument-requires-skip-all sha=85ee9a296641 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:297
quote: set +e; rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .; test $? -eq 1
at: src/cmd/list.rs:116
quote: #[instrument(skip_all, name = "cli.list", fields(json = json))]
guard_at: .github/workflows/ci.yml:297
@row_add n=372 row=decision/ci-gates-fail-closed area=ci-integration rec=fct-do3y4vos@1,fct-v4xtuswt@1
name: CI gates fail closed
rule: A gate whose tool is missing, errors or meets an empty scope fails the step rather than passing.
why: an erroring search with a must-be-empty check would pass on every commit
guard: G1, G3, scripts/g2-zero-panics.sh --probe
@row_check n=372 row=decision/ci-gates-fail-closed sha=24d39cd7466a v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:297
quote: set +e; rg -n -U --pcre2 --type rust '#\[(tracing::)?instrument\b(?!\(\s*skip_all\b)' .; test $? -eq 1
at: scripts/g2-zero-panics.sh:13
quote: echo "tool-missing: jq"
at: scripts/g2-zero-panics.sh:37
quote: echo "g2: empty scope — no role file under target/e2e-home"
guard_at: .github/workflows/ci.yml:59
@row_add n=373 row=decision/test-homes-under-scan-root area=ci-integration rec=fct-uqr3kd37@1,fct-eilqg5wz@2,fct-pto2t3ps@1
name: Test homes live under the scanned root
rule: Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it.
why: a hook panic exits 0, so a home outside that root would hide it
except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
@row_check n=373 row=decision/test-homes-under-scan-root sha=d13a1533295a v=holds judge=sonnet head=8834195b0683
at: tests/support/home.rs:193
quote: let base = workspace_path("target/e2e-home");
at: tests/support/home.rs:214
quote: pub fn outside_scan() -> Self {
@row_add n=374 row=decision/perf-budget-asserted-on-max area=ci-integration rec=fct-uqr3kd37@1,fct-gsrzhsh7@2
name: Perf budgets asserted on the maximum sample
rule: Performance budgets are asserted on the max of hyperfine samples, and a failing perf gate still leaves its samples.
@row_check n=374 row=decision/perf-budget-asserted-on-max sha=3bf0421c2466 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:149
quote: .and_then(|v| v["results"][0]["max"].as_f64());
at: .github/workflows/ci.yml:162
quote: if: always() && steps.secret-scan.outcome == 'success'
@row_add n=375 row=decision/zero-panics-gate area=ci-integration rec=fct-k3fxcuz7@1,fct-v4xtuswt@1
name: Zero panics gate
rule: No panic line may appear in the home-level role files of test runs; detail files repeat the panic line and are excluded.
why: ratified
guard: gate G2 (scripts/g2-zero-panics.sh, --probe first)
except: a panic whose panic_location is exactly src/cmd/hook/seam.rs:<digits>, the fake-agent seam's forced panic, compared as a whole string
@row_check n=375 row=decision/zero-panics-gate sha=51e7c7366f34 v=holds guard=true judge=sonnet head=8834195b0683
at: scripts/g2-zero-panics.sh:23
quote: find "$scope/" -path '*/diagnostics/*.ndjson' ! -name 'detail-*' -exec awk 1 {} + \
at: scripts/g2-zero-panics.sh:29
quote: [inputs | fromjson? | select(.event == "panic") | select(exempt | not)] | length' \
at: scripts/g2-zero-panics.sh:28
quote: and (.panic_location | sub(":[0-9]+$"; "")) == $seam;
guard_at: .github/workflows/ci.yml:59
@row_add n=376 row=decision/os-gated-test-census area=ci-integration rec=fct-t2ak7i7a@2
name: OS-gated test census
rule: Every OS-gated test function or file must be named in the census with a non-empty reason, and a gate predicate naming windows or macos is a fault even when named.
guard: contract_os_gated_tests
@row_check n=376 row=decision/os-gated-test-census sha=a5108ee2520c v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_os_gated_tests.rs:544
quote: !g.reason.trim().is_empty(),
at: tests/contract_os_gated_tests.rs:425
quote: names_a_word(predicate, &[WINDOWS]) || spaced.contains("target_os=\"macos\"")
guard_at: tests/contract_os_gated_tests.rs:1
@row_add n=377 row=decision/obs-code-mutation-tested area=ci-integration rec=fct-r4tkrl6z@2
name: Obs code mutation-tested like product code
rule: Obs code is mutation-tested like product code at the epoch boundary and on demand, not on push or pull request; a timed-out or missed obs-code mutant, or unviable outnumbering caught, is red.
guard: test-plan Mutation gate
@row_check n=377 row=decision/obs-code-mutation-tested sha=bb30daeded84 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/mutants.rs:59
quote: let survived = missed.saturating_sub(host_excluded) + timeout;
at: crates/viola-e2e/src/harness/run/mutants.rs:63
quote: failures.push("unviable-exceeds-caught".to_owned());
searched: run --mutants · in .github/workflows/*.yml (only an install of cargo-mutants and a comment; no push or pull_request mutation job)
guard_at: crates/viola-e2e/src/harness/run/mutants.rs:63
@row_add n=378 row=decision/schema-conformance-gate area=ci-integration rec=fct-k3fxcuz7@1,fct-ocs2orze@1
name: Schema conformance gate on real process output
rule: Every role-file line of test homes must validate against schemas/diag-line.v1.json and every detail line against schemas/diag-detail.v1.json; a failing line fails the build, a torn non-JSON line is skipped and counted.
why: schema failures count as a build failure and unit tests check only in-memory lines
guard: gate G4 (schema-conformance step)
@row_check n=378 row=decision/schema-conformance-gate sha=7fbcae90cd8c v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:92
quote: validator("diag-line.v1.json"),
at: crates/viola-e2e/src/harness/schema_check.rs:93
quote: validator("diag-detail.v1.json"),
at: crates/viola-e2e/src/harness/schema_check.rs:116
quote: torn += 1;
at: crates/viola-e2e/src/harness/schema_check.rs:132
quote: let ok = !empty && failures.is_empty();
guard_at: .github/workflows/ci.yml:64
@row_add n=379 row=decision/gate-output-omits-line-content area=ci-integration rec=fct-ocs2orze@1
name: Gate output never prints a line's content
rule: A failing schema check prints file, line number and failing keyword, never the line's content.
why: detail lines carry the canary
@row_check n=379 row=decision/gate-output-omits-line-content sha=9c0c874afba1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/schema_check.rs:126
quote: failures.push(json!({"file": file, "line": n + 1, "keyword": keyword}));
@row_add n=380 row=decision/panic-unwind-required area=ci-integration rec=fct-ocs2orze@1,fct-vfnyc5f5@2
name: Panic strategy is unwind
rule: Every Cargo profile keeps panic = "unwind"; no abort strategy in profiles, rustflags, env or workflows.
why: abort turns every catch_unwind into an abort, so hook would die by SIGABRT instead of exiting 0
guard: gate G3
@row_check n=380 row=decision/panic-unwind-required sha=a029238cb694 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:261
quote: panic = "unwind"
at: Cargo.toml:269
quote: panic = "unwind"
at: .github/workflows/ci.yml:301
quote: set +e; rg -n --hidden -g 'Cargo.toml' -g 'config.toml' -g '*.yml' -g '*.yaml' "(panic|_PANIC)\s*[:=]\s*[\"']?abort" .; test $? -eq 1
guard_at: .github/workflows/ci.yml:301
@row_add n=381 row=decision/gates-run-after-failed-tests area=ci-integration rec=fct-eilqg5wz@2
name: Gates run even after failed tests
rule: G2 and G4 run if: always(), so a failed test step never skips them.
why: failing runs are where panic lines are most likely, and panics are measured only through G2's exit code
@row_check n=381 row=decision/gates-run-after-failed-tests sha=7d02c041c80e v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:57
quote: if: always()
at: .github/workflows/ci.yml:62
quote: if: always()
@row_add n=382 row=decision/test-homes-kept-until-gates area=ci-integration rec=fct-eilqg5wz@2,fct-ulqlzuau@1
name: Test homes kept until gate steps run
rule: CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it.
why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
@row_check n=382 row=decision/test-homes-kept-until-gates sha=dd4123cd56ca v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:20
quote: AGENT_RUN_KEEP_HOMES: "1"
@row_add n=383 row=decision/raw-unsymbolised-backtrace area=error-capture-reporting rec=fct-c5mu6ru4@2
name: Panic backtraces captured raw, never symbolised
rule: A panic's detail line carries raw frames (ip, module path, base, offset), up to 62, for offline resolution against the pinned copy; nothing is symbolised in the panic path.
why: symbolising cost most of a hook run and pushed the forced-panic hook past the 1.0 s spine bound
@row_check n=383 row=decision/raw-unsymbolised-backtrace sha=ac94423a89f6 v=holds judge=sonnet head=8834195b0683
at: src/panic_frames.rs:8
quote: const MAX_FRAMES: usize = 62;
at: src/panic_frames.rs:15
quote: "0x{ip:016x} {path} base=0x{base:016x} +0x{:x}",
@row_add n=384 row=decision/env-vars-not-config-channel area=error-capture-reporting rec=fct-c5mu6ru4@2
name: Env vars are not a configuration channel
rule: Backtrace capture is not gated by RUST_BACKTRACE or any env var.
why: env vars are not a configuration channel
@row_check n=384 row=decision/env-vars-not-config-channel sha=2a8402220c86 v=holds judge=sonnet head=8834195b0683
searched: RUST_BACKTRACE · in src/**/*.rs, crates/**/*.rs
searched: force_capture · in src/**/*.rs, crates/**/*.rs
@row_add n=385 row=decision/exit-decided-at-catch-site area=error-capture-reporting rec=fct-vfnyc5f5@2
name: Exit decided at a catch site, not the panic hook
rule: The panic hook only writes; everything after the hook is set runs inside one catch_unwind in main, so no role ends through an uncaught unwind.
@row_check n=385 row=decision/exit-decided-at-catch-site sha=171c25351a4c v=holds judge=sonnet head=8834195b0683
at: src/main.rs:48
quote: let outcome = std::panic::catch_unwind(|| {
@row_add n=386 row=decision/role-classified-before-clap area=error-capture-reporting rec=fct-vfnyc5f5@2
name: Role taken from argv before clap
rule: The role is the first argument that is neither a global flag nor a flag's value and names a role subcommand, so viola --home <dir> hook stop is still a hook whose pre-init panic exits 0.
why: reading argv[1] alone would misclassify that form as cli and exit 1
@row_check n=386 row=decision/role-classified-before-clap sha=b83a954a8110 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:92
quote: fn role_of(args: impl IntoIterator<Item = OsString>) -> Role {
at: src/main.rs:98
quote: return if arg == "hook" {
@row_add n=387 row=contract/cli-internal-error/human-line area=error-capture-reporting rec=fct-vfnyc5f5@2
boundary: cli internal error
item: error: internal error
promise: Without --json the cli catch site prints exactly error: internal error; run, revive's start, mcp and hook print neither.
@row_check n=387 row=contract/cli-internal-error/human-line sha=d0b6c63371ac v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
@row_add n=388 row=decision/channel-connection-panic-contained area=error-capture-reporting rec=fct-vfnyc5f5@2
name: Channel connection panic contained
rule: A panic in one channel connection worker logs channel-response error -32603, closes that connection, and run continues.
@row_check n=388 row=decision/channel-connection-panic-contained sha=53aacb13cfcf v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:249
quote: .unwrap_or(Err(ProtocolError::Internal))
at: crates/viola-channel/src/server.rs:224
quote: close: logged.class == Class::Error(Some(ProtocolError::Internal)),
@row_add n=389 row=decision/vt100-feed-panic-contained area=error-capture-reporting rec=fct-vfnyc5f5@2,fct-pto2t3ps@1
name: vt100 feed panic contained
rule: A panic in a vt100 feed or resize is caught on the feed thread, poisons the screen model until the host size changes, writes one parse-rejected per episode, and run continues.
@row_check n=389 row=decision/vt100-feed-panic-contained sha=6e09c6dbcda5 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() {
at: src/run/gate.rs:49
quote: parse_rejected("panicked");
at: src/run/gate.rs:57
quote: self.panicked = false;
@row_add n=390 row=contract/mcp-frame-tools/strict-modes-failed area=error-capture-reporting rec=fct-tikck2t5@1
boundary: viola mcp frame tools
item: strict-modes-failed
promise: A refused instance check returns isError instance-unreachable with detail strict-modes-failed, and writes no frame and no line.
@row_check n=390 row=contract/mcp-frame-tools/strict-modes-failed sha=19a74c7dd377 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:340
quote: return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=391 row=contract/mcp-frame-tools/socket-dir-refused area=error-capture-reporting rec=fct-mgjsgjd2@1
boundary: viola mcp frame tools
item: socket-dir-refused
promise: A refused socket directory returns isError instance-unreachable with detail socket-dir-refused, and writes no frame and no line.
@row_check n=391 row=contract/mcp-frame-tools/socket-dir-refused sha=98e414057db9 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:346
quote: return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=392 row=decision/single-sanitising-fault-reader area=error-capture-reporting rec=fct-j2kwa4rj@1
name: One sanitising reader of peer faults
rule: A peer's fault is read only through one sanitiser: five known codes read fixed messages with data admitted only in validated shapes, a code outside them reads wrapper fault, and a fault without an integer code reads nothing.
@row_check n=392 row=decision/single-sanitising-fault-reader sha=e0fb66b5f5a8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:139
quote: _ => ("wrapper fault", Value::Null),
at: crates/viola-channel/src/lib.rs:118
quote: let code = error["code"].as_i64()?;
@row_add n=393 row=decision/peer-replies-screened-as-code-words area=error-capture-reporting rec=fct-6qbwm4yh@1
name: Peer replies screened as code words
rule: A peer reply's refusal and detail are kept only as code words (1-64 bytes of [a-z0-9-], letter first); an unknown code word stays visible and exits 14.
why: by design
@row_check n=393 row=decision/peer-replies-screened-as-code-words sha=b5ff62896fe8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:77
quote: let reason = if is_code_word(reason) {
at: src/cmd/send.rs:76
quote: serde_json::from_value(Value::from(reason)).unwrap_or(RefusalReason::Unknown)
@row_add n=394 row=contract/answer-send-exit-2/stderr-line area=error-capture-reporting rec=fct-cq3l3475@1
boundary: viola answer and viola send exit 2
item: stderr line
promise: Exit 2 writes the fixed unusable-response or unusable-text line to stderr whatever --json says, with no document.
@row_check n=394 row=contract/answer-send-exit-2/stderr-line sha=1b002acbeaf2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/answer.rs:91
quote: let _ = io::stderr().lock().write_all(UNUSABLE_RESPONSE.as_bytes());
at: src/cmd/send.rs:164
quote: let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
@row_add n=395 row=contract/exit-21-document/shape area=error-capture-reporting rec=fct-3qk5e3jo@1
boundary: exit 21 document
item: shape
promise: Exit 21 prints one document per cause: {"v":1,"error":"instance-unreachable","detail":"<cause>"}.
@row_check n=395 row=contract/exit-21-document/shape sha=e3f55a13bac6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=396 row=contract/exit-21-document/detail area=error-capture-reporting rec=fct-3qk5e3jo@1
boundary: exit 21 document
item: detail
promise: One of six closed words: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
@row_check n=396 row=contract/exit-21-document/detail sha=12b60c4683aa v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:196-201
quote: Self::StrictModes => "strict-modes-failed", Self::InstanceDead => "instance-dead", Self::UnwrappedName => "unwrapped-name", Self::EndpointLost => "endpoint-lost", Self::ServerVerify => "server-verify-failed", Self::SocketDir => "socket-dir-refused",
@row_add n=397 row=contract/diag-line-fields/process-start area=log-coverage rec=fct-yb5jpaot@2
boundary: diagnostic line additive fields
item: process-start
promise: subject (self, claude-child, version-probe, verify-probe, verify-pty-probe, agents-probe, statusline-shell), service_name, version, os, pid, child_pid, endpoint_kind, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept (names only).
@row_check n=397 row=contract/diag-line-fields/process-start sha=91dfd0550cbf v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:32
quote: subject = "self",
at: src/run/mod.rs:54
quote: subject = "claude-child",
at: src/run/version_gate.rs:149
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "version-probe");
at: src/cmd/verify.rs:209
quote: "verify-probe",
at: src/cmd/verify/typed/run.rs:21
quote: const SUBJECT: &str = "verify-pty-probe";
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/cmd/hook/statusline.rs:29
quote: const SUBJECT: &str = "statusline-shell";
at: src/run/mod.rs:33-37
quote: service_name = SERVICE_NAME, version = VERSION, os = std::env::consts::OS, pid = std::process::id(), endpoint_kind = viola_channel::ENDPOINT_KIND,
at: src/run/mod.rs:55-61
quote: child_pid = child_pid, pty_backend = viola_pty::pty_backend(), env_stripped_count = strip.count(), env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()), env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()), cli_version = cli_version, cli_verified = cli_verified,
@row_add n=398 row=contract/diag-line-fields/process-exit area=log-coverage rec=fct-umidjbc4@1
boundary: diagnostic line additive fields
item: process-exit
promise: subject, exit_code (self), child_exit_status, shell_exit_status, exit_source (handle-wait or kill-fallback), detail, during (connect or call), duration_ms.
@row_check n=398 row=contract/diag-line-fields/process-exit sha=88ef98355df8 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:81-84
quote: subject = "self", exit_code = exit_code, detail = detail, duration_ms = duration_ms,
at: src/run/mod.rs:70-71
quote: child_exit_status = exit.code, exit_source = exit.source.as_str(),
at: src/cmd/hook/statusline.rs:211
quote: shell_exit_status = code,
at: crates/viola-pty/src/lib.rs:53-54
quote: Self::HandleWait => "handle-wait", Self::KillFallback => "kill-fallback",
at: src/cmd/client.rs:262
quote: during = cause.during(),
@row_add n=399 row=contract/diag-line-fields/channel-request area=log-coverage rec=fct-tnco7mng@1
boundary: diagnostic line additive fields
item: channel-request
promise: method, conn / srv_conn, sender, v, after, timeout_ms.
@row_check n=399 row=contract/diag-line-fields/channel-request sha=704cdbd344d0 v=stale judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:366-375
quote: INFO, ObsEvent::ChannelRequest, corr = corr, conn = peer.conn(), srv_conn = peer.srv_conn(), method = method, sender = params.sender.as_deref().and_then(version_label), v = params.v, after = after, timeout_ms = timeout_ms,
shows: No channel-request line (server log_request, client request_frame, client notify) carries `from` or `from_trust`; only method, conn / srv_conn, sender, v, after, timeout_ms are emitted.
cell: promise
value: method, conn / srv_conn, sender, v, after, timeout_ms.
@row_add n=400 row=contract/diag-line-fields/channel-response area=log-coverage rec=fct-ikvvw35l@1
boundary: diagnostic line additive fields
item: channel-response
promise: method (only a closed-list name), conn / srv_conn, result_class (ok, refusal, error), detail (only the three -32602 faults), error_code (-32700, -32600, -32601, -32602, -32603), outcome, duration_ms.
@row_check n=400 row=contract/diag-line-fields/channel-response sha=7ae1575460a3 v=stale judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:476-485
quote: corr = self.corr, conn = self.peer.conn(), srv_conn = self.peer.srv_conn(), method = self.method, result_class = result_class, error_code = fault.map(ProtocolError::code), detail = fault.and_then(ProtocolError::cause), outcome = self.outcome, duration_ms = duration_ms, )
shows: The channel-response line (the only emitter) has no `refusal` field; a refusal is only result_class "refusal".
cell: promise
value: method (only a closed-list name), conn / srv_conn, result_class (ok, refusal, error), detail (only the three -32602 faults), error_code (-32700, -32600, -32601, -32602, -32603), outcome, duration_ms.
@row_add n=401 row=contract/diag-line-fields/send-events area=log-coverage rec=fct-hb5clvjs@1
boundary: diagnostic line additive fields
item: send-issued / send-confirmed / send-refused
promise: from, from_trust, text_bytes (normalised typed length), confirmed, refusal, detail, side (client or wrapper), wheel, window, used_percentage, resets_at, paused, duration_ms, and wrapper side only conn / srv_conn and rpc_id.
@row_check n=401 row=contract/diag-line-fields/send-events sha=3b584552ce0d v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:200-202
quote: from = from, from_trust = self.trust(), text_bytes = u64::try_from(text_bytes).unwrap_or(u64::MAX),
at: src/run/send.rs:239
quote: confirmed = confirmed,
at: src/run/send.rs:289-296
quote: side = "wrapper", wheel = self.wheel.holder().as_str(), refusal = refusal.as_str(), detail = detail, window = budget.map(|crossed| crossed.window.as_str()), used_percentage = budget.map(|crossed| crossed.used_percentage), resets_at = budget.map(|crossed| crossed.resets_at.as_str()), paused = budget.map(|_| true),
at: src/cmd/send.rs:240
quote: side = "client",
at: src/run/send.rs:197-199
quote: rpc_id = self.call.id, conn = self.call.conn, srv_conn = self.call.srv_conn,
at: src/run/send.rs:240
quote: duration_ms = duration_ms,
@row_add n=402 row=contract/diag-line-fields/dialog-events area=log-coverage rec=fct-qb47tiuw@1
boundary: diagnostic line additive fields
item: dialog-raised / dialog-answered
promise: dialog_kind, hook_event, from, from_trust, deadline_hit, duration_ms.
@row_check n=402 row=contract/diag-line-fields/dialog-events sha=ee6def41532a v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:316-317
quote: dialog_kind = raise.kind.as_str(), hook_event = raise.hook.as_str(),
at: src/run/dialog.rs:463-466
quote: from = from, from_trust = from.map(|_| "self-reported"), deadline_hit = false, duration_ms = millis(self.clock.now().saturating_duration_since(pending.raised)),
@row_add n=403 row=contract/diag-line-fields/hook-events area=log-coverage rec=fct-vrjzzaok@1
boundary: diagnostic line additive fields
item: hook-invoked / hook-decision
promise: hook_event (one of ten closed hook names incl. statusline), invoked_at, decision_emitted, deadline_hit, budget_written, detail, duration_ms.
@row_check n=403 row=contract/diag-line-fields/hook-events sha=cb1777f6236a v=stale judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:173-175
quote: ObsEvent::HookInvoked, hook_event = event.as_str(), invoked_at = invoked_at.as_str(),
shows: The hook-invoked line carries hook_event and invoked_at only; no code in src/ or crates/ emits a `stdin_bytes` field (the hook-decision lines carry decision_emitted, deadline_hit, duration_ms, detail; statusline carries budget_written).
cell: promise
value: hook_event (one of ten closed hook names incl. statusline), invoked_at, decision_emitted, deadline_hit, budget_written, detail, duration_ms.
@row_add n=404 row=contract/diag-line-fields/panic area=log-coverage rec=fct-hq2iiwoa@1
boundary: diagnostic line additive fields
item: panic
promise: panic_location (workspace-relative, or <crate>-<ver>/src/... for dependencies) and thread; payload and backtrace go to the detail file only.
@row_check n=404 row=contract/diag-line-fields/panic sha=76fbfc091fbb v=holds judge=sonnet head=8834195b0683
at: src/main.rs:247-248
quote: "panic_location": location, "thread": thread,
at: src/main.rs:220-221
quote: fields.insert("panic_payload".to_owned(), payload.into()); fields.insert("backtrace".to_owned(), Value::Array(backtrace));
@row_add n=405 row=contract/diag-line-fields/release-from-driver area=log-coverage rec=fct-izpoipm6@1
boundary: diagnostic line additive fields
item: release-from-driver
promise: conn, from (only when a valid ViolaName), from_trust (self-reported).
@row_check n=405 row=contract/diag-line-fields/release-from-driver sha=247007ea721c v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:241-244
quote: corr = call.id, conn = call.conn, from = from.as_ref().map(AsRef::<str>::as_ref), from_trust = "self-reported",
@row_add n=406 row=contract/diag-line-fields/state-recovered area=log-coverage rec=fct-rn7dqtoc@1
boundary: diagnostic line additive fields
item: state-recovered
promise: detail (torn-line-healed, snapshot-replayed, snapshot-unsupported-v), file (basename only), offset (torn-line-healed only), v_seen (snapshot-unsupported-v only), scanned (snapshot details only: the instance name when valid, never a path or null).
@row_check n=406 row=contract/diag-line-fields/state-recovered sha=3671bf55e43f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:166-169
quote: detail = detail, file = SNAPSHOT, v_seen = v_seen, scanned = scanned,
at: crates/viola-state/src/events.rs:339-341
quote: detail = "torn-line-healed", file = EVENTS, offset = tail.len,
at: crates/viola-state/src/replay.rs:176-181
quote: fn scanned_name(instance_dir: &Path) -> Option<&str> { instance_dir .file_name() .and_then(|name| name.to_str()) .filter(|name| ViolaName::try_new((*name).to_owned()).is_ok()) }
@row_add n=407 row=contract/diag-line-fields/parse-rejected area=log-coverage rec=fct-yp3xr6e4@1
boundary: diagnostic line additive fields
item: parse-rejected
promise: parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry), detail, count.
@row_check n=407 row=contract/diag-line-fields/parse-rejected sha=d5f1ee602306 v=stale judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:537
quote: parser = "hook-stdin",
shows: Eight of the ten listed parsers are emitted (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry); no parse-rejected line anywhere carries parser `prompt-submitted` or `paste-text`.
cell: promise
value: parser (claude-agents-json, vt100-feed, config-json, ledger-stamps, hook-stdin, channel-frame, mcp-stdin, state-entry), detail, count.
@row_add n=408 row=contract/process-exit-detail/exit-1 area=log-coverage rec=fct-dsupowm6@2
boundary: process-exit detail
item: exit 1
promise: already-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, internal-error, socket-dir-refused, and from revive's preflight no-session, session-live, cwd-missing; each one process-exit line with no child started.
@row_check n=408 row=contract/process-exit-detail/exit-1 sha=08a1441458f5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
at: src/cmd/run.rs:270
quote: Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
at: src/cmd/run.rs:411-413
quote: pub(super) fn refused(detail: &'static str) -> Started { run::log_self_exit(1, Some(detail)); Started::Refused(ExitCode::from(1))
@row_add n=409 row=decision/revive-preflight-order area=log-coverage rec=fct-dsupowm6@2
name: Revive start preflight order
rule: Revive's start preflight reads the holder, then the session, then session-live, then the directory, and stops at the first refusal.
@row_check n=409 row=decision/revive-preflight-order sha=d0e838467c49 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:176
quote: if let Some(refusal) = holder() {
at: src/cmd/revive.rs:179
quote: let session = match session()? {
at: src/cmd/revive.rs:183
quote: if let Some(refusal) = session_live(&session) {
at: src/cmd/revive.rs:186
quote: Ok(recorded_dir(&session).map(|dir| (session, dir)))
@row_add n=410 row=decision/session-live-check-fails-open area=log-coverage rec=fct-dsupowm6@2
name: Session-live check fails open
rule: A failed, oversize or malformed claude agents read passes the session-live check; the check is skipped under --fork.
@row_check n=410 row=decision/session-live-check-fails-open sha=d19e6c37e901 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:193
quote: .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
at: src/cmd/revive.rs:223
quote: |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
@row_add n=411 row=contract/process-exit-detail/interrupted area=log-coverage rec=fct-dsupowm6@2
boundary: process-exit detail
item: interrupted
promise: A viola verify stopped by SIGINT or SIGTERM before its rows are checked ends with one process-exit exit 1 detail interrupted, after every spawn pair it opened is closed.
@row_check n=411 row=contract/process-exit-detail/interrupted sha=dcee677bfcc9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
at: src/cmd/verify/stop.rs:12
quote: for signal in [libc::SIGINT, libc::SIGTERM] {
@row_add n=412 row=contract/process-exit-detail/verify-refusal area=log-coverage rec=fct-dsupowm6@2
boundary: process-exit detail
item: viola verify refusal
promise: Under a parsed VIOLA_NAME, verify writes its refusal code: program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture or interrupted; a failing row carries no code.
@row_check n=412 row=contract/process-exit-detail/verify-refusal sha=fdf3902f9e80 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:164
quote: return Ok(Ended::Refused("program-not-found"));
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
at: src/cmd/verify.rs:199
quote: return Ok(Ended::Refused("version-unreadable"));
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
at: src/cmd/verify.rs:629
quote: Ended::Refused("unclean-fixture")
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
@row_add n=413 row=decision/unemittable-schema-words-removed area=log-coverage rec=fct-j4k5a564@1
name: Schema words no build can emit are removed
rule: A closed word of the diag-line schema that no build can emit leaves the schema with its emitter; os is closed at linux.
@row_check n=413 row=decision/unemittable-schema-words-removed sha=50973c9fd7e6 v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:67
quote: "os": { "enum": ["linux"] }, "pid": { "$ref": "#/$defs/count" },
@row_add n=414 row=contract/process-exit-detail/exit-21 area=log-coverage rec=fct-qzbqiq2k@2
boundary: process-exit detail
item: exit 21
promise: instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
@row_check n=414 row=contract/process-exit-detail/exit-21 sha=71c2a2a0cc05 v=stale judge=sonnet head=8834195b0683
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:40
quote: Ok(Some(21))
shows: strict-modes-failed on exit 21 is refused with the stderr pair or the document and no process-exit line; the only exit-21 process-exit writer (client.rs:256-262) takes its detail from Unreachable, which has no strict-modes variant, so five of the six words are written and strict-modes-failed is not.
cell: promise
value: instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused.
@row_add n=415 row=contract/process-exit-detail/server-verify-failed area=log-coverage rec=fct-qzbqiq2k@2
boundary: process-exit detail
item: server-verify-failed
promise: The eight CLI frame verbs write it at WARN with exit 21, during connect, on a server of another euid, not the recorded wrapper, or an unverifiable check, with no pid, start time, path or endpoint.
@row_check n=415 row=contract/process-exit-detail/server-verify-failed sha=29bf8cc6f12b v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:178
quote: return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:256-262
quote: obs_event!( WARN, ObsEvent::ProcessExit, subject = "self", exit_code = 21u8, detail = cause.detail(), during = cause.during(),
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=416 row=contract/process-exit-detail/exit-20 area=log-coverage rec=fct-ylj2z2t5@1
boundary: process-exit detail
item: exit 20
promise: wrapper-fault.
@row_check n=416 row=contract/process-exit-detail/exit-20 sha=1aabe5f520c3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:244
quote: run::log_self_exit(20, Some("wrapper-fault"));
@row_add n=417 row=contract/hook-decision-detail/fail-open-codes area=log-coverage rec=fct-qiql34nd@2
boundary: hook-decision detail
item: fail-open codes
promise: oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline; no hook arm writes strict-modes-failed or unverified-cli.
@row_check n=417 row=contract/hook-decision-detail/fail-open-codes sha=3c267774b462 v=stale judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:213-215
quote: Self::Unreachable => "channel-unreachable", Self::ServerUnverified => "server-verify-failed", Self::SocketDir => UnreachableCause::SocketDir.as_str(),
shows: The hook writes six detail codes (oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline); no line in the hook arms writes unverified-cli or strict-modes-failed as a hook-decision detail.
cell: promise
value: oversize-stdin, malformed-json, channel-unreachable, server-verify-failed, socket-dir-refused, deadline; no hook arm writes strict-modes-failed or unverified-cli.
@row_add n=418 row=contract/hook-decision-detail/statusline-codes area=log-coverage rec=fct-qiql34nd@2
boundary: hook-decision detail
item: statusline codes
promise: viola hook statusline writes only oversize-stdin, malformed-json and deadline; a refused instance check writes no line.
@row_check n=418 row=contract/hook-decision-detail/statusline-codes sha=007b45ff7457 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:119
quote: return Ok(Outcome::failed("oversize-stdin"));
at: src/cmd/hook/statusline.rs:123
quote: return Ok(Outcome::failed("malformed-json"));
at: src/cmd/hook/statusline.rs:146
quote: Some(Shell::Deadline) => Some("deadline"),
@row_add n=419 row=decision/statusline-command-deadline area=log-coverage rec=fct-qiql34nd@2
name: Statusline command deadline
rule: The user's statusline command is bounded at 5 s, reported as deadline.
state: provisional
@row_check n=419 row=decision/statusline-command-deadline sha=d3df0449fc02 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:64
quote: const STATUSLINE_DEADLINE: Duration = Duration::from_secs(5);
at: src/cmd/hook/statusline.rs:146
quote: Some(Shell::Deadline) => Some("deadline"),
@row_add n=420 row=contract/parse-rejected-detail/malformed area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: malformed
promise: Any parser.
@row_check n=420 row=contract/parse-rejected-detail/malformed sha=0ffece59e495 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:167
quote: Self::Malformed => "malformed",
at: src/run/version_gate.rs:208
quote: Err(_) => (false, Some("malformed")),
at: src/cmd/hook.rs:184
quote: reject_stdin("malformed");
at: crates/viola-agent-claude/src/agents.rs:83
quote: Self::Malformed => "malformed",
at: crates/viola-state/src/board.rs:311
quote: ("malformed", scan.malformed),
@row_add n=421 row=contract/parse-rejected-detail/unreadable area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: unreadable
promise: config-json, ledger-stamps.
@row_check n=421 row=contract/parse-rejected-detail/unreadable sha=ee7e9c30f88f v=stale judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:309-312
quote: for (detail, count) in [ ("symlink-ignored", scan.links_ignored), ("malformed", scan.malformed), ] {
shows: parser state-entry emits only symlink-ignored and malformed; unreadable is written for config-json (src/obs.rs:166) and ledger-stamps (src/run/version_gate.rs:205) alone.
cell: promise
value: config-json, ledger-stamps.
@row_add n=422 row=contract/parse-rejected-detail/oversize area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: oversize
promise: channel-frame beyond MAX_FRAME, hook-stdin, mcp-stdin past MAX_FRAME, vt100-feed when the bounded queue drops a copy (one line per poisoning episode), claude-agents-json past MAX_FRAME.
@row_check n=422 row=contract/parse-rejected-detail/oversize sha=11dbcffeab45 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:264
quote: reject_frame("oversize");
at: src/cmd/hook.rs:180
quote: reject_stdin("oversize");
at: crates/viola-mcp/src/lib.rs:99
quote: detail = "oversize",
at: src/run/gate.rs:72
quote: parse_rejected("oversize");
at: crates/viola-agent-claude/src/agents.rs:82
quote: Self::Oversize => "oversize",
@row_add n=423 row=contract/parse-rejected-detail/unknown-keys area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: unknown-keys
promise: config-json skipped keys, counted in count.
@row_check n=423 row=contract/parse-rejected-detail/unknown-keys sha=04a7ee04c962 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:168
quote: Self::UnknownKeys(_) => "unknown-keys",
at: src/obs.rs:174
quote: Self::UnknownKeys(n) => n,
@row_add n=424 row=contract/parse-rejected-detail/panicked area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: panicked
promise: vt100-feed under catch_unwind.
@row_check n=424 row=contract/parse-rejected-detail/panicked sha=6ec2fdbb8baf v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:46
quote: if catch_unwind(AssertUnwindSafe(|| step(&mut self.screen))).is_err() {
at: src/run/gate.rs:49
quote: parse_rejected("panicked");
@row_add n=425 row=contract/parse-rejected-detail/symlink-ignored area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: symlink-ignored
promise: state-entry.
@row_check n=425 row=contract/parse-rejected-detail/symlink-ignored sha=a526215a3bc1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:310
quote: ("symlink-ignored", scan.links_ignored),
at: crates/viola-state/src/board.rs:317
quote: parser = "state-entry",
@row_add n=426 row=contract/parse-rejected-detail/strict-modes-failed area=log-coverage rec=fct-vehppv2d@1
boundary: parse-rejected detail
item: strict-modes-failed
promise: ledger-stamps: run's version gate refused the stamps file on the strict-modes check, and the read counts as unverified.
@row_check n=426 row=contract/parse-rejected-detail/strict-modes-failed sha=cd0a7d0189fa v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:204
quote: Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")),
at: src/run/version_gate.rs:177
quote: parser = "ledger-stamps",
@row_add n=427 row=contract/parse-rejected/mcp-stdin-oversize area=log-coverage rec=fct-iy4rx7dp@1
boundary: parse-rejected diagnostic line
item: parser mcp-stdin, detail oversize
promise: Written at WARN into mcp.ndjson once per session, after the session ends, only when the stdin reader cut a line past MAX_FRAME, with count 1; the line holds no byte of the cut line.
@row_check n=427 row=contract/parse-rejected/mcp-stdin-oversize sha=9fc9fd1a4dfb v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:86-87
quote: if cut.load(Ordering::Relaxed) { reject_oversize_line();
at: crates/viola-mcp/src/lib.rs:95-100
quote: obs_event!( WARN, ObsEvent::ParseRejected, parser = "mcp-stdin", detail = "oversize", count = 1u64,
@row_add n=428 row=contract/send-refused-detail/refusal-reason-words area=log-coverage rec=fct-icf6u7jt@1
boundary: send-refused detail
item: RefusalReason detail words
promise: Refusal details are the arch RefusalReason details verbatim: manual-pause, five-hour, seven-day, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text.
@row_check n=428 row=contract/send-refused-detail/refusal-reason-words sha=544bb91e8211 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:242
quote: Self::ManualPause => "manual-pause",
at: crates/viola-core/src/lib.rs:259
quote: Self::FiveHour => "five-hour",
at: crates/viola-core/src/lib.rs:260
quote: Self::SevenDay => "seven-day",
at: crates/viola-core/src/lib.rs:221
quote: Self::InputNotReady => "input-not-ready",
at: crates/viola-core/src/lib.rs:222
quote: Self::NoPromptSubmitted => "no-prompt-submitted",
at: crates/viola-core/src/lib.rs:223
quote: Self::TurnRunning => "turn-running",
at: crates/viola-core/src/lib.rs:224
quote: Self::UnknownDialog => "unknown-dialog",
at: crates/viola-core/src/lib.rs:225
quote: Self::ControlCharacter => "control-character",
at: crates/viola-core/src/lib.rs:226
quote: Self::EmptyText => "empty-text",
@row_add n=429 row=contract/send-refused-detail/empty-text area=log-coverage rec=fct-icf6u7jt@1
boundary: send-refused detail
item: empty-text
promise: Written for an empty typed text on send; the word adds no field, and send-refused detail stays an open string in the diag-line schema.
@row_check n=429 row=contract/send-refused-detail/empty-text sha=c2c8c310ed72 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:94
quote: return ctx.not_delivered(None, NotDelivered::EmptyText);
at: schemas/diag-line.v1.json:58
quote: "detail": { "type": "string" }, "side": { "enum": ["client", "wrapper"] }, "wheel": { "type": "string" },
@row_add n=430 row=contract/diag-detail/strict-modes-failed area=log-coverage rec=fct-mivk5eld@1,fct-tinnk5cr@1
boundary: diagnostic detail words
item: strict-modes-failed
promise: Written on the process-exit exit 1 of viola verify under a parsed VIOLA_NAME and on parse-rejected ledger-stamps from the version gate of run, at warn, with no path; every other refused strict-modes check writes no log line, its exit code or JSON document its only record.
@row_check n=430 row=contract/diag-detail/strict-modes-failed sha=964278cd4f00 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
at: src/cmd/verify.rs:81
quote: Ended::Refused(detail) => (1, Some(detail)),
at: src/run/version_gate.rs:174-178
quote: obs_event!( WARN, ObsEvent::ParseRejected, parser = "ledger-stamps", detail = detail,
@row_add n=431 row=contract/diag-detail/pinned-hash-mismatch area=log-coverage rec=fct-mivk5eld@1
boundary: diagnostic detail words
item: pinned-hash-mismatch
promise: A filesystem refusal word written on process-exit with exit 1.
@row_check n=431 row=contract/diag-detail/pinned-hash-mismatch sha=3bd644599178 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:171
quote: return Ok(Ended::Refused("pinned-hash-mismatch"));
at: src/cmd/verify.rs:81
quote: Ended::Refused(detail) => (1, Some(detail)),
@row_add n=432 row=decision/symlink-skip-logged-as-count area=log-coverage rec=fct-mivk5eld@1
name: Skipped state entries logged as a parse-rejected count
rule: A scan of instance state that skips a symlinked or malformed entry writes one parse-rejected line with parser state-entry, detail symlink-ignored or malformed and a count, once per scan, at warn, into the scanning process's role file, naming no entry and no link target.
why: A skipped symlink has no exit or decision to carry it.
@row_check n=432 row=decision/symlink-skip-logged-as-count sha=d3a09fee52c3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:310
quote: ("symlink-ignored", scan.links_ignored),
at: crates/viola-state/src/board.rs:311
quote: ("malformed", scan.malformed),
at: crates/viola-state/src/board.rs:314
quote: obs_event!(
at: crates/viola-state/src/board.rs:317
quote: parser = "state-entry",
at: crates/viola-state/src/board.rs:319
quote: count = count,
@row_add n=433 row=contract/diag-detail/socket-dir-refused area=log-coverage rec=fct-mivk5eld@1,fct-l3f7hrdu@2
boundary: diagnostic detail words
item: socket-dir-refused
promise: A closed detail on process-exit and hook-decision for a socket dir refused as a symlink, no directory, foreign owner or mode bit: run and revive's start write process-exit exit 1, CLI frame verbs process-exit exit 21 during connect at WARN, viola hook a hook-decision at WARN; MCP writes none.
@row_check n=433 row=contract/diag-detail/socket-dir-refused sha=11fd361a76dd v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/run/mod.rs:81-83
quote: subject = "self", exit_code = exit_code, detail = detail,
at: src/cmd/client.rs:256-262
quote: obs_event!( WARN, ObsEvent::ProcessExit, subject = "self", exit_code = 21u8, detail = cause.detail(), during = cause.during(),
at: src/cmd/hook.rs:389-396
quote: Some(detail) => obs_event!( WARN, ObsEvent::HookDecision, hook_event = event.as_str(), decision_emitted = emitted, deadline_hit = deadline_hit, duration_ms = duration_ms, detail = detail,
@row_add n=434 row=contract/diag-subject/self area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: self
promise: One value of the closed subject enum of the diag-line schema.
@row_check n=434 row=contract/diag-subject/self sha=a65aba9ca3ed v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:32
quote: subject = "self",
@row_add n=435 row=contract/diag-subject/claude-child area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: claude-child
promise: One value of the closed subject enum of the diag-line schema.
@row_check n=435 row=contract/diag-subject/claude-child sha=b1eb6b9cc0ea v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:54
quote: subject = "claude-child",
@row_add n=436 row=contract/diag-subject/version-probe area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: version-probe
promise: Subject of the process-start and process-exit pair of a version probe spawn.
@row_check n=436 row=contract/diag-subject/version-probe sha=71d28d1fca26 v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:149
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "version-probe");
at: src/run/version_gate.rs:166
quote: subject = "version-probe",
@row_add n=437 row=contract/diag-subject/verify-probe area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: verify-probe
promise: Subject of the process-start and process-exit pair of the print-mode probe of viola verify.
@row_check n=437 row=contract/diag-subject/verify-probe sha=f6ecf393acdb v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:209
quote: "verify-probe",
at: src/cmd/verify.rs:123
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = subject);
@row_add n=438 row=contract/diag-subject/verify-pty-probe area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: verify-pty-probe
promise: Subject of the process-start and process-exit pair of each of the five interactive PTY runs of viola verify, Run A to Run E.
@row_check n=438 row=contract/diag-subject/verify-pty-probe sha=6e9e91faf3e9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/run.rs:89
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = SUBJECT);
at: src/cmd/verify/typed/run.rs:223
quote: subject = SUBJECT,
@row_add n=439 row=contract/diag-subject/agents-probe area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: agents-probe
promise: Subject of the process-start and process-exit pair of the claude agents read, written by viola list, viola revive's session-live reading, verify's Run B and the list tool of viola mcp into their role files.
@row_check n=439 row=contract/diag-subject/agents-probe sha=ab7c11d3a8cf v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/cmd/list.rs:124
quote: .and_then(|cwd| run::agents::read_here(home, &cwd).map_err(|_| ()))
at: src/cmd/revive.rs:192
quote: crate::run::agents::read_here(home, cwd)
at: src/cmd/verify/typed/turns.rs:112
quote: let read = crate::run::agents::read_logged(inputs.program, dir, inputs.strip, run.stop);
@row_add n=440 row=contract/diag-subject/statusline-shell area=log-coverage rec=fct-lhnmh6dy@1
boundary: diag-line subject enum
item: statusline-shell
promise: Subject of the process-start and process-exit pair of a statusline shell spawn.
@row_check n=440 row=contract/diag-subject/statusline-shell sha=becaf700a58c v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:200
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = SUBJECT);
at: src/cmd/hook/statusline.rs:29
quote: const SUBJECT: &str = "statusline-shell";
@row_add n=441 row=contract/verify-process-exit/child-exit-status-and-duration area=log-coverage rec=fct-lhnmh6dy@1
boundary: viola verify spawn process-exit lines
item: child_exit_status and duration_ms
promise: Each of the eight verify spawn pairs' process-exit carries child_exit_status and duration_ms, including a child of an interactive PTY run killed at a Ctrl-C or TERM stop; a stopped run writes only the pairs it started.
@row_check n=441 row=contract/verify-process-exit/child-exit-status-and-duration sha=79fdcfa60375 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:134
quote: child_exit_status = status,
at: src/cmd/verify.rs:135
quote: duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
at: src/cmd/verify/typed/run.rs:224-225
quote: child_exit_status = status, duration_ms = ms(self.started.elapsed()),
at: src/run/agents.rs:66-67
quote: child_exit_status = status, duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
@row_add n=442 row=contract/parse-rejected/claude-agents-json area=log-coverage rec=fct-lhnmh6dy@1
boundary: parse-rejected diagnostic line
item: parser claude-agents-json
promise: A claude agents --json parse failure, oversize or malformed, gives one parse-rejected line with parser claude-agents-json and the value unknown.
@row_check n=442 row=contract/parse-rejected/claude-agents-json sha=cb216af4ad2b v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:78-84
quote: let list = agents::parse(&ran.stdout).map_err(|failure| { obs_event!( WARN, ObsEvent::ParseRejected, parser = "claude-agents-json", detail = failure.detail(), count = 1u64,
at: crates/viola-agent-claude/src/agents.rs:82-83
quote: Self::Oversize => "oversize", Self::Malformed => "malformed",
@row_add n=443 row=decision/counters-from-home-role-files area=metric-coverage rec=fct-iwloyhyc@1
name: Counters derived from home-level role files
rule: A counter of an event, detail or refusal value is derived with jq over the home-level diagnostics role files only, skipping a torn line.
why: Detail files repeat lines.
@row_check n=443 row=decision/counters-from-home-role-files sha=e2e114bdc810 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=444 row=decision/histograms-as-raw-samples area=metric-coverage rec=fct-zrfczzyu@1
name: Histograms as raw samples
rule: A latency distribution is the raw duration_ms samples per event, with no bucketing; quantiles are taken by sort and index in jq.
@row_check n=444 row=decision/histograms-as-raw-samples sha=f4e96e2992c7 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=445 row=decision/gauges-from-harness-snapshots area=metric-coverage rec=fct-wlstziny@1
name: Gauges from harness snapshots
rule: A current-value gauge is read from harness snapshots: budget.read_at, and liveness via agent-run status.
@row_check n=445 row=decision/gauges-from-harness-snapshots sha=d1ec099a9305 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=446 row=decision/quantiles-report-only area=metric-coverage rec=fct-sqywzwf6@1
name: Quantiles are report-only
rule: Latency quantiles are for reports only; the gates use the max, and the quantile read skips a torn line rather than aborting.
@row_check n=446 row=decision/quantiles-report-only sha=6117466f2027 v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=447 row=decision/linux-only-os-target area=obs-scope-summary rec=fct-ztmbo3l4@1
name: Linux-only OS target
rule: The OS target is Linux only: the core on any current distribution, CI on ubuntu-latest.
@row_check n=447 row=decision/linux-only-os-target sha=47428dc2ed24 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:18
quote: os: [ubuntu-latest]
at: .github/workflows/nightly.yml:13
quote: runs-on: ubuntu-latest
@row_add n=448 row=decision/anyhow-error-capture-boundary area=obs-scope-summary rec=fct-iyfle2l2@1
name: The anyhow crate is the error-capture boundary
rule: The only crate that uses anyhow is the error-capture boundary, with context chains only at dispatch; the full chain goes to diagnostics only.
@row_check n=448 row=decision/anyhow-error-capture-boundary sha=d3569cdf5ff0 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:153
quote: anyhow.workspace = true
at: src/obs.rs:256
quote: pub(crate) fn report_internal_error(error: &anyhow::Error, sink: Option<&DetailSink>) {
at: src/obs.rs:260
quote: write_detail(&sink.home, &sink.instance, sink.process, &line);
@row_add n=449 row=decision/serde-chain-fixed-message area=obs-scope-summary rec=fct-iyfle2l2@1
name: Serde error chains become a fixed message on stderr
rule: An error chain that contains a serde source becomes a fixed message before it reaches stderr.
@row_check n=449 row=decision/serde-chain-fixed-message sha=397edd85d213 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:57
quote: obs::report_internal_error(&failure.error, failure.sink.as_ref());
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=450 row=contract/unreachable-cause/strict-modes-failed area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: strict-modes-failed
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_check n=450 row=contract/unreachable-cause/strict-modes-failed sha=ea57e21f0570 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: crates/viola-mcp/src/tools.rs:340
quote: return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=451 row=contract/unreachable-cause/instance-dead area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: instance-dead
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_check n=451 row=contract/unreachable-cause/instance-dead sha=e1fda62ecd20 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:119
quote: Self::Dead => UnreachableCause::InstanceDead,
at: crates/viola-mcp/src/result.rs:53
quote: Err(ChannelError::Connect(_)) => return Ok(unreachable(UnreachableCause::InstanceDead)),
@row_add n=452 row=contract/unreachable-cause/unwrapped-name area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: unwrapped-name
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_check n=452 row=contract/unreachable-cause/unwrapped-name sha=bb1583b72751 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:120
quote: Self::Unwrapped => UnreachableCause::UnwrappedName,
at: crates/viola-mcp/src/tools.rs:343
quote: return Ok(result::unreachable(without_endpoint(&instance_dir)));
@row_add n=453 row=contract/unreachable-cause/endpoint-lost area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: endpoint-lost
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_check n=453 row=contract/unreachable-cause/endpoint-lost sha=6385ec37b1f8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
at: crates/viola-mcp/src/result.rs:55
quote: return Ok(unreachable(UnreachableCause::EndpointLost));
@row_add n=454 row=contract/unreachable-cause/server-verify-failed area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: server-verify-failed
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_check n=454 row=contract/unreachable-cause/server-verify-failed sha=7125abf746bf v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:122
quote: Self::ServerUnverified => UnreachableCause::ServerVerify,
at: crates/viola-mcp/src/tools.rs:353
quote: return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=455 row=contract/unreachable-cause/socket-dir-refused area=obs-scope-summary rec=fct-y6hfs4wy@1
boundary: exit 21 cause words
item: socket-dir-refused
promise: One of the closed set of six exit 21 cause words, written into the CLI's --json exit-21 document and MCP's isError body.
@row_check n=455 row=contract/unreachable-cause/socket-dir-refused sha=a4bdb7855b86 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:123
quote: Self::SocketDir => UnreachableCause::SocketDir,
at: crates/viola-mcp/src/tools.rs:346
quote: return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=456 row=decision/ipc-crate-sync-instrumentation area=obs-scope-summary rec=fct-aowjxl3a@1,fct-2vabautb@1
name: Synchronous instrumentation in the IPC crate
rule: Instrumentation in the interprocess IPC crate must be synchronous.
why: Stack forbids Tokio in the crate's graph unless its feature is enabled.
@row_check n=456 row=decision/ipc-crate-sync-instrumentation sha=3b9206704d93 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/Cargo.toml:23
quote: tokio = { workspace = true, optional = true, features = ["io-util"] }
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
@row_add n=457 row=decision/state-crate-sync-instrumentation area=obs-scope-summary rec=fct-pssgrgft@1,fct-g6misrd3@1
name: Synchronous instrumentation in the state crate
rule: Instrumentation in the state crate built on ndjson, atomic-write-file and sysinfo must be synchronous.
why: Stack bans Tokio in that crate.
@row_check n=457 row=decision/state-crate-sync-instrumentation sha=778315e3186c v=holds judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
searched: tokio|async fn|\.await · in crates/viola-state (no match)
@row_add n=458 row=decision/stripped-claude-values-never-logged area=obs-scope-summary rec=fct-add24jhw@1
name: Stripped CLAUDE values never logged
rule: R8-stripped CLAUDE* values are never logged anywhere.
why: Its outputs are the most sensitive in the product.
@row_check n=458 row=decision/stripped-claude-values-never-logged sha=c81d81c1658c v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:57-58
quote: env_stripped_count = strip.count(), env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
at: crates/viola-pty/src/lib.rs:128
quote: fields(pty_backend = pty_backend(), env_stripped_count = spec.env_remove.len())
@row_add n=459 row=decision/mcp-no-telemetry-on-stdout area=obs-scope-summary rec=fct-drz74jk7@1
name: No telemetry on the MCP server's stdout
rule: The MCP server sends no telemetry to stdout.
why: stdout is reserved for MCP frames.
@row_check n=459 row=decision/mcp-no-telemetry-on-stdout sha=38d1f5be92d0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:76
quote: let (stdin, stdout) = rmcp::transport::stdio();
searched: println!|print!\(|io::stdout|stdout\(\) · in src/cmd/mcp.rs and src/obs.rs (no match)
@row_add n=460 row=decision/claude-boundary-only-instrumentation area=obs-scope-summary rec=fct-t3jwc6wb@1
name: Boundary-only instrumentation of claude
rule: The hosted claude binary is instrumented only at its boundary: spawn and exit, paste writes, and the vt100 readiness-gate outcome.
why: claude is a third-party binary run unmodified.
@row_check n=460 row=decision/claude-boundary-only-instrumentation sha=9a95f81956fb v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:127
quote: name = "pty.spawn",
at: src/run/mod.rs:66-69
quote: obs_event!( INFO, ObsEvent::ProcessExit, subject = "claude-child",
at: crates/viola-pty/src/pump.rs:74
quote: name = "pty.paste_write",
at: src/run/gate.rs:155-156
quote: name = "run.readiness_gate", fields(outcome = tracing::field::Empty, vt100_panicked = tracing::field::Empty)
@row_add n=461 row=decision/claude-exit-on-process-handle area=obs-scope-summary rec=fct-t3jwc6wb@1
name: Claude exit detected on the process handle
rule: The exit of the hosted claude process is detected on the process handle, never on EOF.
@row_check n=461 row=decision/claude-exit-on-process-handle sha=4bf60753e2dc v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:169
quote: if let Some(code) = pty.try_wait()? {
at: crates/viola-pty/src/pump.rs:176
quote: source: ExitSource::HandleWait,
@row_add n=462 row=decision/hook-mcp-instrumented-at-boundary area=obs-scope-summary rec=fct-lpbmjfvf@1
name: Hook and MCP instrumented at the boundary
rule: Hook and MCP traffic is instrumented only at viola's own boundary: hook-invoked and hook-decision, MCP tool-call handling, and the viola mcp stdin cut.
why: The caller is Claude Code, so viola sees only the inbound stdin payload and its own response, and no trace context arrives from Claude Code.
@row_check n=462 row=decision/hook-mcp-instrumented-at-boundary sha=0451244cdeba v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:173
quote: ObsEvent::HookInvoked,
at: src/cmd/hook.rs:391
quote: ObsEvent::HookDecision,
at: crates/viola-mcp/src/tools.rs:248
quote: #[instrument(skip_all, name = "mcp.tool_call", fields(tool = tool.as_str()))]
at: crates/viola-mcp/src/lib.rs:98
quote: parser = "mcp-stdin",
@row_add n=463 row=decision/agents-cli-instrumented-at-boundary area=obs-scope-summary rec=fct-k7csbv6h@1,fct-drucxpuy@1
name: claude agents CLI instrumented at spawn, exit and parse
rule: The external `claude agents --json` CLI is instrumented only at its spawn, its exit and its parse outcome.
why: An external CLI.
@row_check n=463 row=decision/agents-cli-instrumented-at-boundary sha=9a5fbdf69278 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/run/agents.rs:65
quote: subject = "agents-probe",
at: src/run/agents.rs:82
quote: parser = "claude-agents-json",
@row_add n=464 row=decision/agents-cli-parse-failure-unknown area=obs-scope-summary rec=fct-k7csbv6h@1,fct-drucxpuy@1
name: claude agents parse failure reads unknown
rule: A failed parse of `claude agents --json` output reads "unknown" plus a counter.
@row_check n=464 row=decision/agents-cli-parse-failure-unknown sha=4cd5ff5bf38e v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:82-84
quote: parser = "claude-agents-json", detail = failure.detail(), count = 1u64,
at: crates/viola-state/src/board.rs:236
quote: status: SessionStatus::Unknown,
@row_add n=465 row=decision/agents-cli-output-untrusted area=obs-scope-summary rec=fct-k7csbv6h@1,fct-drucxpuy@1
name: claude agents output untrusted
rule: The output rows of `claude agents --json` are treated as untrusted.
@row_check n=465 row=decision/agents-cli-output-untrusted sha=8e86f6a6a68a v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/agents.rs:36
quote: _ => Self::Unknown,
at: src/run/agents.rs:55
quote: MAX_FRAME + 1,
@row_add n=466 row=decision/statusline-shell-out-instrumentation area=obs-scope-summary rec=fct-nvhb6dqn@1,fct-vr7juzn5@1
name: Status-line shell-out instrumented by duration and exit
rule: The `viola hook statusline` shell-out of the user's command is instrumented by its duration and exit status only.
why: A user-written shell command.
@row_check n=466 row=decision/statusline-shell-out-instrumentation sha=62d9600d3639 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:211-212
quote: shell_exit_status = code, duration_ms = duration_ms,
at: src/cmd/hook/statusline.rs:195
quote: name = "statusline.shell_out",
@row_add n=467 row=decision/statusline-command-only-in-diagnostics area=obs-scope-summary rec=fct-nvhb6dqn@1
name: Status-line command string kept to diagnostics
rule: The statusline_command string is never logged outside diagnostics/.
@row_check n=467 row=decision/statusline-command-only-in-diagnostics sha=d5c9690b920b v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:193-197
quote: #[instrument( skip_all, name = "statusline.shell_out", fields(shell_exit_status = Empty, duration_ms = Empty) )]
@row_add n=468 row=decision/instrument-call-sites-not-os-internals area=obs-scope-summary rec=fct-f3waaflx@2
name: OS operations instrumented at viola's call sites
rule: OS-level operations (bind, connect, lock, rename, kill fallback, the server check's peer reads, the strict-modes stat reads, the socket directory check) are instrumented at viola's own call sites, not inside the OS.
@row_check n=468 row=decision/instrument-call-sites-not-os-internals sha=fa398239f7ac v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:61
quote: #[instrument(skip_all, name = "channel.bind", fields(endpoint_kind = ENDPOINT_KIND))]
at: crates/viola-pty/src/lib.rs:200
quote: #[tracing::instrument(skip_all, name = "pty.kill")]
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=469 row=contract/exit-cause-detail/server-verify-failed area=obs-scope-summary rec=fct-f3waaflx@2
boundary: Exit cause detail codes
item: server-verify-failed
promise: A failed server check of peer euid, peer pid or start time is logged by the CLI and the hook as server-verify-failed; MCP writes no line for it.
@row_check n=469 row=contract/exit-cause-detail/server-verify-failed sha=ea21734caecb v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:261
quote: detail = cause.detail(),
at: src/cmd/hook.rs:214
quote: Self::ServerUnverified => "server-verify-failed",
at: crates/viola-mcp/src/tools.rs:353
quote: return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=470 row=contract/exit-cause-detail/socket-dir-refused area=obs-scope-summary rec=fct-f3waaflx@2,fct-oiibon7o@2,fct-h2wm2goy@2
boundary: Exit cause detail codes
item: socket-dir-refused
promise: A refused socket directory has its own detail code socket-dir-refused and writes its line (MCP writes none); on run and on a start made by viola revive that line is one process-exit{subject:"self", exit_code:1} before any snapshot, child or bind.
@row_check n=470 row=contract/exit-cause-detail/socket-dir-refused sha=856fc3120a2a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:412
quote: run::log_self_exit(1, Some(detail));
at: src/cmd/run.rs:497-498
quote: if !socket_dir_ready(&endpoint) { return Ok(Bound::SocketDirRefused);
at: crates/viola-mcp/src/tools.rs:346
quote: return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=471 row=decision/refused-peer-dropped-without-line area=obs-scope-summary rec=fct-f3waaflx@2,fct-6xpygltd@1
name: Refused channel peer dropped without a log line
rule: A channel stream whose peer euid is refused is dropped on the accept thread before its worker thread and before any frame: the peer reads end-of-file, and the wrapper writes no log line and nothing on the terminal.
why: There is no frame to key a line on, and channel-response's detail is closed.
@row_check n=471 row=decision/refused-peer-dropped-without-line sha=19972a08f940 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:139
quote: /// A refused peer's stream is dropped here, before its thread and before any frame is read: it reads
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
@row_add n=472 row=contract/replayed-snapshot/live-fields area=obs-scope-summary rec=fct-a3522pml@1
boundary: A snapshot.json rebuilt by replay
item: endpoint, pid, started_at, statusline_command
promise: A snapshot.json rebuilt from events.ndjson carries none of these, so the endpoint reads unreachable, hook connect fails open with channel-unreachable, hook statusline prints nothing, and run's collision check reads the heartbeat alone.
@row_check n=472 row=contract/replayed-snapshot/live-fields sha=ef14d6cd3f80 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:18-24
quote: pub struct Replayed { pub wheel: Option<Wheel>, pub budget_paused: Option<bool>, pub budget_override_until: Option<String>, pub agent_session_id: Option<String>, pub links: Vec<Value>, pub dialog_pending: bool,
at: crates/viola-state/src/liveness.rs:93
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(instance_dir) else {
at: src/cmd/hook.rs:227-228
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else { return Err(NotSent::Unreachable);
at: src/cmd/hook/statusline.rs:130-131
quote: Ok(Recovered::Snapshot(snapshot)) => snapshot.statusline_command, _ => None,
at: src/cmd/run.rs:441-442
quote: // A replayed snapshot records no pid: the beat alone says whether a wrapper still runs. Some(Recovered::Replayed { .. }) => age()
@row_add n=473 row=decision/budget-metadata-loggable area=obs-scope-summary rec=fct-a3522pml@1
name: Budget metadata loggable as fields
rule: Operational metadata in budget.json (percentages, resets_at) may be logged as structured fields.
@row_check n=473 row=decision/budget-metadata-loggable sha=be357a1af381 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:294
quote: used_percentage = budget.map(|crossed| crossed.used_percentage),
at: src/run/send.rs:295
quote: resets_at = budget.map(|crossed| crossed.resets_at.as_str()),
@row_add n=474 row=decision/config-observed-at-load area=obs-scope-summary rec=fct-p5bvvhtw@1
name: Configuration observed only at load
rule: Configuration is observed only at its parse at load time, logged as skipped-key counts.
why: Static configuration and data.
@row_check n=474 row=decision/config-observed-at-load sha=8a67f0483ac1 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:213
quote: Some(ConfigRejection::UnknownKeys(unknown))
at: src/obs.rs:226
quote: count = rejection.count(),
@row_add n=475 row=decision/fixtures-not-telemetry area=obs-scope-summary rec=fct-6un2lrdt@1
name: Recorded fixtures are no telemetry source
rule: Recorded fixtures are scrub-required data, not a telemetry source, and are excluded from the observability plan.
why: Not product runtime.
@row_check n=475 row=decision/fixtures-not-telemetry sha=64e3be06a667 v=unverifiable judge=sonnet head=8834195b0683
searched: fixtures? · in src/obs.rs and Cargo.toml
why: not a code claim
@row_add n=476 row=decision/tracing-json-logging-pipeline area=obs-scope-summary rec=fct-noyuw3eo@1
name: tracing with a JSON subscriber for logging
rule: Logging and spans go through the Rust tracing facade with a tracing-subscriber JSON structured logger.
@row_check n=476 row=decision/tracing-json-logging-pipeline sha=bed9615931b4 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:121-122
quote: .json()         .flatten_event(true)
@row_add n=477 row=decision/panic-hook-replaced area=obs-scope-summary rec=fct-noyuw3eo@1,fct-3ftavspi@1,fct-foy4up7k@1
name: Panic hook replaced by a one-line writer
rule: Each process replaces the default panic hook, as the first statement in main and before any child spawns, with one that writes one JSON line and nothing to stderr.
why: The default hook prints to stderr, which breaks the hook contract and is the child's terminal under run.
@row_check n=477 row=decision/panic-hook-replaced sha=2b476c41b90a v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
at: src/main.rs:157
quote: /// Never calls the default hook and never writes stderr: one payload-free JSON line, one
@row_add n=478 row=absence/interactive-terminal-interface area=obs-scope-summary rec=fct-6ujsabka@1
capability: An interactive terminal interface: prompt, TUI or pager
@row_check n=478 row=absence/interactive-terminal-interface sha=6d64a95bc1e9 v=holds judge=sonnet head=8834195b0683
searched: dialoguer|ratatui|crossterm|rustyline|inquire|cursive|termion|pager · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=479 row=decision/process-logs-file-sink-only area=obs-scope-summary rec=fct-oy7tgncm@1,fct-he3oql74@1
name: Process logs to a file sink only
rule: Process logs are written only as JSON lines to one file per process role under diagnostics/; there is no network exporter and no stdout sink.
why: viola makes no outbound network calls, and stdout carries --json results.
@row_check n=479 row=decision/process-logs-file-sink-only sha=18cb35085113 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:128
quote: .with_writer(writer);
at: src/obs.rs:115
quote: /// One JSON object per line, one `write` per line, into `writer` only (obs-plan §3 Logging stack).
@row_add n=480 row=contract/cli-log-file/revive-list area=obs-scope-summary rec=fct-oy7tgncm@1
boundary: CLI process-log file
item: viola revive --list
promise: Logs as cli into the caller's cli-<VIOLA_NAME>.ndjson once its strict-modes check has passed; no file with no VIOLA_NAME.
@row_check n=480 row=contract/cli-log-file/revive-list sha=ce2c77e42ba4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:272-278
quote: if !trusted(home, name)? { Refusal::StrictModes.write(name, json); return Ok(ExitCode::from(1)); } let home = std::path::absolute(home)?; if let Some(own) = client::own_name() { client::start(&home, &own)?;
at: src/cmd/client.rs:47
quote: obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
@row_add n=481 row=contract/cli-log-file/link-unlink area=obs-scope-summary rec=fct-oy7tgncm@1
boundary: CLI process-log file
item: viola link / viola unlink
promise: Log as cli into the driven instance's cli-<driven>.ndjson, never the caller's VIOLA_NAME, once their instance check of it has passed.
@row_check n=481 row=contract/cli-log-file/link-unlink sha=02eb71a8a91f v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/link.rs:97
quote: let home = client::start(home, &args.driven)?;
at: src/cmd/unlink.rs:15
quote: let code = link_call(home, args, Verb::Unlink, request)?;
@row_add n=482 row=contract/cli-log-file/revive-start area=obs-scope-summary rec=fct-oy7tgncm@1
boundary: CLI process-log file
item: viola revive start arm
promise: Logs as the wrapper, process="run", into diagnostics/run-<name>.ndjson.
@row_check n=482 row=contract/cli-log-file/revive-start sha=6ed84f919b8b v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:152
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/run.rs:225
quote: obs::viola_obs_init(home, ObsProcess::Run, Some(name.clone()), level)?;
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name))
@row_add n=483 row=contract/cli-json-error/strict-modes-failed area=obs-scope-summary rec=fct-oy7tgncm@1,fct-oiibon7o@2
boundary: CLI --json error document
item: instance-unreachable strict-modes-failed
promise: A refused instance check of send, wait, last, answer, pause, release, link or unlink exits 21 with the instance pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_check n=483 row=contract/cli-json-error/strict-modes-failed sha=36061d5db035 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:40
quote: Ok(Some(21))
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
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
at: src/cmd/unlink.rs:15
quote: let code = link_call(home, args, Verb::Unlink, request)?;
@row_add n=484 row=contract/service-identity/service-version area=obs-scope-summary rec=fct-uwlbnhdg@1
boundary: Service identity
item: service.version
promise: The compile-time CARGO_PKG_VERSION, the same value used as sender and writer.
@row_check n=484 row=contract/service-identity/service-version sha=26ad59f57f62 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:8
quote: pub const VERSION: &str = env!("CARGO_PKG_VERSION");
@row_add n=485 row=absence/deployment-environments area=obs-scope-summary rec=fct-uwlbnhdg@1
capability: Deployment environments or hosting
@row_check n=485 row=absence/deployment-environments sha=f063d8bb8e47 v=holds judge=sonnet head=8834195b0683
searched: deployment|environment =|staging · in Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
@row_add n=486 row=decision/instance-from-name-not-cwd area=obs-scope-summary rec=fct-oiibon7o@2,fct-uwlbnhdg@1
name: Instance from its name, never the working directory
rule: The instance comes from VIOLA_NAME or an explicit argument, never from the working directory.
@row_check n=486 row=decision/instance-from-name-not-cwd sha=a894a2486ab3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:217
quote: let instance = std::env::var_os("VIOLA_NAME")
@row_add n=487 row=decision/fixed-stderr-messages area=obs-scope-summary rec=fct-oiibon7o@2
name: Fixed stderr refusal and fault lines
rule: Refusal and fault lines on stderr are fixed strings with no paths, pids, error chains or upstream text.
@row_check n=487 row=decision/fixed-stderr-messages sha=7af26ca1af91 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=488 row=contract/cli-internal-error/stderr-line area=obs-scope-summary rec=fct-oiibon7o@2
boundary: CLI internal error
item: stderr line
promise: Without --json a catch-site fault prints the fixed line `error: internal error` to stderr.
@row_check n=488 row=contract/cli-internal-error/stderr-line sha=8590e9466d25 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/main.rs:67-70
quote: if json_asked(std::env::args_os()) { cmd::internal_error_document(); } else { human::internal_error();
@row_add n=489 row=decision/exit-code-log-field area=obs-scope-summary rec=fct-oiibon7o@2
name: Exit code as a log field
rule: The exit code (0, 1, 2, 10–14, 20, 21), the machine-readable outcome, is carried as a log field.
@row_check n=489 row=decision/exit-code-log-field sha=e41cf5e9f966 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:82
quote: exit_code = exit_code,
@row_add n=490 row=decision/sync-instrumentation-no-tokio area=obs-scope-summary rec=fct-cuxf35wq@1,fct-foy4up7k@1
name: Sync instrumentation without Tokio
rule: Instrumentation in the sync crates runs on std threads with no Tokio.
why: Hybrid concurrency.
guard: cargo deny ban
@row_check n=490 row=decision/sync-instrumentation-no-tokio sha=4f47faad85f1 v=holds guard=true judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
guard_at: deny-sync.toml:12
@row_add n=491 row=decision/run-silent-on-terminal area=obs-scope-summary rec=fct-i2uu7hiy@1,fct-giamwcrt@1
name: run silent on the terminal
rule: run writes nothing to the terminal while the child runs; its process log goes only to diagnostics/run-<name>.ndjson.
why: The terminal carries the child's screen.
@row_check n=491 row=decision/run-silent-on-terminal sha=ab0a6f1faf7e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:691
quote: Box::new(Tee::new(io::stdout(), feed)),
searched: println!|eprintln!|print!|eprint! · in src/run/**/*.rs and src/cmd/run.rs (none; stdout above is only the child's screen tee)
@row_add n=492 row=contract/process-log/process area=obs-scope-summary rec=fct-ivwsjbo2@1,fct-7fgifl76@1,fct-uy5gif5o@1,fct-ogkwmfvl@1
boundary: Process log line
item: process
promise: The process role, one of run, hook, mcp, cli.
@row_check n=492 row=contract/process-log/process sha=11d6b2d11ea3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:90
quote: Self::Run => "run",
at: crates/viola-core/src/obs.rs:91
quote: Self::Hook => "hook",
at: crates/viola-core/src/obs.rs:92
quote: Self::Mcp => "mcp",
at: crates/viola-core/src/obs.rs:93
quote: Self::Cli => "cli",
@row_add n=493 row=contract/heartbeat-file/tick area=obs-scope-summary rec=fct-3ftavspi@1
boundary: Heartbeat file
item: tick
promise: The heartbeat file is touched every 1 s as the liveness tick.
@row_check n=493 row=contract/heartbeat-file/tick sha=4565f1efcac5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/heartbeat.rs:13
quote: pub const BEAT_EVERY: Duration = Duration::from_secs(1);
@row_add n=494 row=decision/child-exit-from-handle-wait area=obs-scope-summary rec=fct-3ftavspi@1
name: Child exit from the process handle
rule: The child's process-exit comes from the wait on its process handle, never from reader end-of-file.
why: Child exit is detected on the process handle.
@row_check n=494 row=decision/child-exit-from-handle-wait sha=e7e61da96546 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:169
quote: if let Some(code) = pty.try_wait()? {
at: crates/viola-pty/src/pump.rs:176
quote: source: ExitSource::HandleWait,
@row_add n=495 row=decision/hook-panic-one-line area=obs-scope-summary rec=fct-t4pdf4hq@1,fct-ogkwmfvl@1
name: Hook panic logged as one line
rule: A panic in a hook is caught with catch_unwind and logged as one level:"ERROR", event:"panic" line.
@row_check n=495 row=decision/hook-panic-one-line sha=34a92ed710e7 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:48
quote: let outcome = std::panic::catch_unwind(|| {
at: src/main.rs:242-245
quote: "level": "ERROR",         "target": "viola::panic",         "message": "panic",         "event": "panic",
@row_add n=496 row=decision/hook-never-writes-stderr area=obs-scope-summary rec=fct-d5lpbhzx@1,fct-ogkwmfvl@1
name: Hook never writes stderr
rule: A hook process never writes to stderr.
@row_check n=496 row=decision/hook-never-writes-stderr sha=e1be2f87c6e2 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:51
quote: Err(_) if role == Role::Hook => return Outcome::Unparsed,
at: src/main.rs:145
quote: role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
at: src/main.rs:191
quote: let _ = (&*sink.file).write_all(line.as_bytes());
@row_add n=497 row=contract/hook-stdout/hook-event area=obs-scope-summary rec=fct-d5lpbhzx@1
boundary: Hook stdout
item: viola hook <event>
promise: stdout carries only the decision body.
@row_check n=497 row=contract/hook-stdout/hook-event sha=480fdf58e2a3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:133-134
quote: let out = &mut std::io::stdout().lock(); return match handle_dialog(event, &instance, deadlines, stdin, out) {
@row_add n=498 row=contract/hook-stdout/statusline area=obs-scope-summary rec=fct-d5lpbhzx@1
boundary: Hook stdout
item: viola hook statusline
promise: stdout carries only the user's status-line command's output.
@row_check n=498 row=contract/hook-stdout/statusline sha=630594405ae9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
@row_add n=499 row=contract/hook-stdout/capture-arms area=obs-scope-summary rec=fct-d5lpbhzx@1
boundary: Hook stdout
item: --capture arms
promise: The two hidden --capture arms write no log sink and print nothing, except --answers' probe decision body.
@row_check n=499 row=contract/hook-stdout/capture-arms sha=d2c2eb58790a v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:101-106
quote: if let Some(dir) = &args.capture { let filed = capture(event, dir, &mut std::io::stdin().lock()); if let (Some(filed), Some(answers)) = (filed, &args.answers) { answer(event, dir, answers, &filed, &mut std::io::stdout().lock()); } return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:95-96
quote: capture_statusline(dir, shell, &mut std::io::stdin().lock()); return Ok(ExitCode::SUCCESS);
@row_add n=500 row=decision/hook-log-failure-swallowed area=obs-scope-summary rec=fct-d5lpbhzx@1,fct-ogkwmfvl@1
name: Hook log write failure swallowed
rule: A failed log write in a hook is swallowed and the hook still exits 0.
@row_check n=500 row=decision/hook-log-failure-swallowed sha=a2a7e9cc82f6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:122
quote: let _ = obs::viola_obs_init(
at: src/obs.rs:126
quote: .log_internal_errors(false)
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=501 row=decision/hook-silent-without-trusted-instance area=obs-scope-summary rec=fct-7fgifl76@1,fct-kv6frzyp@2
name: Hook silent without a trusted instance
rule: A hook with no VIOLA_NAME, or whose instance is absent or refused by its strict-modes check, exits 0 with no config read, no stdin read, no log line, no stderr, no frame and no SessionEnd append.
@row_check n=501 row=decision/hook-silent-without-trusted-instance sha=71dd556d50d1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
at: src/cmd/hook/statusline.rs:42-43
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=502 row=decision/hook-instance-check-before-obs-init area=obs-scope-summary rec=fct-7fgifl76@1
name: Hook instance check before logging starts
rule: The hook's event and dialog paths run the instance strict-modes check before reading the diagnostics level and initialising logging.
except: The two --capture arms run ahead of the check.
@row_check n=502 row=decision/hook-instance-check-before-obs-init sha=557d14104d71 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:119
quote: let (level, _) = obs::read_diagnostics_level(&instance.home);
at: src/cmd/hook.rs:101
quote: if let Some(dir) = &args.capture {
@row_add n=503 row=decision/hook-latency-budget area=obs-scope-summary rec=fct-kv6frzyp@2
name: Hook latency budget
rule: Each hook runs within max < 1.0 s, and logger init adds no blocking I/O beyond one file open and append.
guard: run --perf hook rows under SPINE_DEADLINE
except: viola hook statusline is outside the budget.
@row_check n=503 row=decision/hook-latency-budget sha=2bd168610af1 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:56
quote: const CONNECT_DEADLINE: Duration = Duration::from_millis(750);
at: src/cmd/hook/statusline.rs:59
quote: bound: STATUSLINE_DEADLINE,
guard_at: crates/viola-e2e/src/harness/gate.rs:151
@row_add n=504 row=decision/correlation-in-corr-field area=obs-scope-summary rec=fct-kkjide4x@1
name: Correlation in the corr log field
rule: Correlation across a channel call travels in the corr log field; channel params declare no trace-context field.
@row_check n=504 row=decision/correlation-in-corr-field sha=6a335ca2c21e v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:369
quote: corr = dialog_id,
searched: traceparent|tracestate|trace_id|trace.context · in src/**, crates/**, schemas/**
@row_add n=505 row=contract/channel-params/sender area=obs-scope-summary rec=fct-qe2js5tq@1
boundary: Channel params
item: sender
promise: params.sender carries the peer's version, CARGO_PKG_VERSION.
@row_check n=505 row=contract/channel-params/sender sha=d80c303091f7 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:115
quote: params.insert("sender".to_owned(), VERSION.into());
at: crates/viola-core/src/lib.rs:8
quote: pub const VERSION: &str = env!("CARGO_PKG_VERSION");
@row_add n=506 row=contract/json-rpc-frame/id area=obs-scope-summary rec=fct-6xpygltd@1
boundary: JSON-RPC frame
item: id
promise: Ids are integers, monotonic per connection, and so not unique across connections or processes.
@row_check n=506 row=contract/json-rpc-frame/id sha=4e71459ff8ee v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:74-75
quote: let id = self.next_id; self.next_id += 1;
@row_add n=507 row=contract/json-rpc-error/32603 area=obs-scope-summary rec=fct-6xpygltd@1
boundary: JSON-RPC error
item: -32603
promise: The -32603 error carries the message "internal error" with data null.
@row_check n=507 row=contract/json-rpc-error/32603 sha=e7e0e6330e42 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:292
quote: r#"{"code":-32603,"message":"internal error","data":null}"#
at: crates/viola-channel/src/lib.rs:86
quote: Self::Internal => "internal error",
@row_add n=508 row=decision/from-not-authenticated area=obs-scope-summary rec=fct-6xpygltd@1
name: from never logged as authenticated
rule: The self-reported from field is never logged as an authenticated identity.
@row_check n=508 row=decision/from-not-authenticated sha=b2196353f8ae v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:166
quote: self.from.as_ref().map(|_| "self-reported")
at: src/run/dialog.rs:464
quote: from_trust = from.map(|_| "self-reported"),
at: src/run/wheel.rs:244
quote: from_trust = "self-reported",
@row_add n=509 row=decision/subscriber-explicit-writer area=obs-scope-summary rec=fct-g4wb4nmr@1,fct-foy4up7k@1
name: Subscriber installed with an explicit writer
rule: The tracing subscriber is always installed with an explicit writer, never stdout.
why: tracing-subscriber fmt() writes to stdout unless a writer is configured, and stdout carries MCP frames.
@row_check n=509 row=decision/subscriber-explicit-writer sha=617cbad8d8ef v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:128
quote: .with_writer(writer);
at: src/obs.rs:145
quote: let _ = tracing::subscriber::set_global_default(subscriber(file, level));
@row_add n=510 row=contract/mcp-stdin/line-within-max-frame area=obs-scope-summary rec=fct-g4wb4nmr@1
boundary: viola mcp stdin
item: line within MAX_FRAME
promise: A line of at most MAX_FRAME bytes, its newline counted, passes whole.
@row_check n=510 row=contract/mcp-stdin/line-within-max-frame sha=b30c2adf2e45 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/stdin.rs:50
quote: u64::try_from(len).is_ok_and(|len| len <= self.cap)
@row_add n=511 row=contract/mcp-stdin/oversize-before-initialize area=obs-scope-summary rec=fct-g4wb4nmr@1
boundary: viola mcp stdin
item: oversize line before initialize
promise: An oversize line before initialize exits the process 1 with both streams empty and the one parse-rejected record.
@row_check n=511 row=contract/mcp-stdin/oversize-before-initialize sha=96d9123cd121 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:80
quote: Err(_) => Err(ServeError::Initialize),
at: crates/viola-mcp/src/lib.rs:86-88
quote: if cut.load(Ordering::Relaxed) { reject_oversize_line(); }
at: src/main.rs:136-138
quote: Role::Mcp | Role::Cli | Role::Other, Outcome::Unparsed | Outcome::Failed | Outcome::Panicked, ) => ExitCode::from(1),
@row_add n=512 row=contract/mcp-server/instance area=obs-scope-summary rec=fct-uy5gif5o@1
boundary: MCP server identity
item: instance
promise: The server's own VIOLA_NAME when set, which is also the from it adds.
@row_check n=512 row=contract/mcp-server/instance sha=2a3b832a86a9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mcp.rs:27
quote: obs::viola_obs_init(home, ObsProcess::Mcp, own_name.clone(), level)?;
at: src/cmd/client.rs:210
quote: let raw = std::env::var_os("VIOLA_NAME")?.into_string().ok()?;
at: crates/viola-mcp/src/tools.rs:319
quote: params.insert("from".to_owned(), Value::from(AsRef::<str>::as_ref(from)));
@row_add n=513 row=decision/mcp-results-codes-only area=obs-scope-summary rec=fct-k56v7yhe@1
name: MCP rejections carry codes only
rule: MCP isError results and the log lines of rejected calls carry codes only.
@row_check n=513 row=decision/mcp-results-codes-only sha=d29930faf2fc v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:73
quote: let reason = if is_code_word(reason) {
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
@row_add n=514 row=contract/mcp-iserror/instance-unreachable area=obs-scope-summary rec=fct-k56v7yhe@1
boundary: MCP isError result
item: instance-unreachable
promise: The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed or socket-dir-refused; the CLI's document less v, its text `refused: instance-unreachable`.
@row_check n=514 row=contract/mcp-iserror/instance-unreachable sha=8ca2418d5b1c v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: crates/viola-mcp/src/result.rs:31
quote: error("instance-unreachable", Value::from(cause.as_str()))
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=515 row=contract/mcp-iserror/invalid-params area=obs-scope-summary rec=fct-k56v7yhe@1
boundary: MCP isError result
item: invalid-params
promise: The invalid-params detail is a closed field word.
@row_check n=515 row=contract/mcp-iserror/invalid-params sha=51a36098c962 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
@row_add n=516 row=contract/mcp-iserror/refusal area=obs-scope-summary rec=fct-k56v7yhe@1
boundary: MCP isError result
item: refusal
promise: A peer refusal that is no code word reads unknown, and a string detail that is no code word reads null.
@row_check n=516 row=contract/mcp-iserror/refusal sha=267f144500aa v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:73-77
quote: let reason = if is_code_word(reason) { reason } else { RefusalReason::Unknown.as_str() };
at: crates/viola-mcp/src/result.rs:80
quote: Some(Value::String(detail)) => Some(detail.as_str()).filter(|d| is_code_word(d)),
@row_add n=517 row=contract/mcp-iserror/wrapper-fault area=obs-scope-summary rec=fct-k56v7yhe@1
boundary: MCP isError result
item: wrapper-fault
promise: The wrapper-fault detail is a closed sanitised object: the fixed message of the five codes with data null or closed values; a code outside the five reads message "wrapper fault", data null.
@row_check n=517 row=contract/mcp-iserror/wrapper-fault sha=87753abec8c3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:139
quote: _ => ("wrapper fault", Value::Null),
at: crates/viola-channel/src/lib.rs:141
quote: Some(json!({"code": code, "message": message, "data": data}))
at: crates/viola-mcp/src/result.rs:60-61
quote: return sanitised_fault(fault) .map(|detail| error("wrapper-fault", detail))
@row_add n=518 row=decision/no-otel-exporter area=obs-scope-summary rec=fct-foy4up7k@1
name: No OTel SDK exporter
rule: v1 initialises no OTel SDK exporter; the tracing facade plus the tracing-subscriber JSON formatter is the binding pipeline.
why: No outbound network calls, a new listener needs a security decision, stdout is reserved on hook, mcp and run, and cargo deny bans Tokio and C-building crates.
except: An optional OTel bridge layer may feed only an in-process or file sink and stays Tokio-free on the sync crates.
@row_check n=518 row=decision/no-otel-exporter sha=d8997daafb76 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry · in Cargo.toml, crates/*/Cargo.toml (no dependency; only deny.toml:32-33 bans opentelemetry-otlp and opentelemetry-stdout)
searched: tracing-appender|sentry · in Cargo.toml, crates/*/Cargo.toml (no dependency; only deny.toml:34-35 bans them)
@row_add n=519 row=decision/process-init-order area=obs-scope-summary rec=fct-foy4up7k@1
name: Process logging init order
rule: Each process installs its panic hook, parses arguments, resolves home and instance and runs its strict-modes check, opens its diagnostics file append-mode 0600 in a 0700 directory, installs the subscriber, then emits process-start.
why: For run this completes before the start sequence so the collision check and version gate are logged; for mcp, before the stdio transport starts.
@row_check n=519 row=decision/process-init-order sha=a893c452a41c v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
at: src/main.rs:49
quote: let cli = match cmd::Cli::try_parse() {
at: src/obs.rs:141
quote: let file = open_role_file(home, process, instance.as_ref())?;
at: src/obs.rs:145
quote: let _ = tracing::subscriber::set_global_default(subscriber(file, level));
at: src/cmd/mcp.rs:28
quote: run::log_self_start();
@row_add n=520 row=decision/tracing-subscriber-version-floor area=obs-scope-summary rec=fct-he3oql74@1
name: tracing-subscriber version floor
rule: tracing-subscriber is held at version 0.3.20 or later.
why: The security floor for RUSTSEC-2025-0055.
@row_check n=520 row=decision/tracing-subscriber-version-floor sha=64b8b6358018 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:243
quote: tracing-subscriber = { version = "=0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }
@row_add n=521 row=decision/log-level-not-from-env area=obs-scope-summary rec=fct-he3oql74@1
name: Log level never from the environment
rule: The log level filter is never read from RUST_LOG; any verbosity control comes from config.json or a CLI flag and never bypasses redaction.
why: Environment variables are not a configuration channel, and no env, flag or config value may disable a control.
@row_check n=521 row=decision/log-level-not-from-env sha=38f7770de89e v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:184
quote: match File::open(home.join("config.json")) {
searched: RUST_LOG|EnvFilter · in src/**/*.rs, crates/**/*.rs (only comments; deny.toml:50 denies feature env-filter)
@row_add n=522 row=decision/json-to-file-only-mode area=obs-scope-summary rec=fct-he3oql74@1
name: JSON-to-file the only logging mode
rule: JSON-to-file is the only logging mode; there is no separate agent-mode flag.
@row_check n=522 row=decision/json-to-file-only-mode sha=45ea64ca5159 v=holds judge=sonnet head=8834195b0683
searched: agent-mode|agent_mode|log-format|LogFormat · in src/**/*.rs, crates/**/*.rs (no logging-mode flag; hits are only the LOG_FORMAT_EVENTS test constant)
@row_add n=523 row=contract/process-log/timestamp area=obs-scope-summary rec=fct-he3oql74@1,fct-ogkwmfvl@1
boundary: Process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_check n=523 row=contract/process-log/timestamp sha=b546068d00f1 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:96
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=524 row=contract/event-line/shape area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: events.ndjson event line
item: line shape
promise: Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
@row_check n=524 row=contract/event-line/shape sha=e835e9fb28d4 v=stale judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:25-28
quote: pub enum Source { Wrapper, Hook, }
shows: The event line has the six keys v, ts, instance, kind, source, data, but its source type has only wrapper and hook; no code writes source "cli".
cell: promise
value: Exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
@row_add n=525 row=contract/process-log/level area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.
@row_check n=525 row=contract/process-log/level sha=02a5a1239978 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:279
quote: "ERROR",
at: src/obs.rs:801
quote: assert_eq!(lines[0]["level"], "DEBUG");
@row_add n=526 row=contract/process-log/target area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: target
promise: A required field.
@row_check n=526 row=contract/process-log/target sha=08d0ef545dbc v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:309
quote: record.insert("target".to_owned(), target.into());
@row_add n=527 row=contract/process-log/message area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: message
promise: A required field, as fields.message or the flattened message.
@row_check n=527 row=contract/process-log/message sha=308579c1127e v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:149-150
quote: "{}", event.as_str()
@row_add n=528 row=contract/process-log/event area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: event
promise: A closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
@row_check n=528 row=contract/process-log/event sha=63e71e7aa9f0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:12
quote: pub enum ObsEvent {
@row_add n=529 row=contract/process-log/instance area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: instance
promise: A ViolaName or null.
@row_check n=529 row=contract/process-log/instance sha=8c6b319fbf2c v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:147
quote: instance = $crate::obs::ctx_instance(),
@row_add n=530 row=contract/process-log/corr area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed.
@row_check n=530 row=contract/process-log/corr sha=db32ee52bbb7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:369
quote: corr = dialog_id,
at: src/run/send.rs:196
quote: corr = cursor,
at: crates/viola-channel/src/client.rs:183
quote: corr = id,
@row_add n=531 row=contract/process-log-corr/channel-request area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: channel-request
promise: corr is the JSON-RPC request id.
@row_check n=531 row=contract/process-log-corr/channel-request sha=3d5fc14484c0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:206
quote: log_request(request.id, peer, method, &params, &request.params);
at: crates/viola-channel/src/server.rs:368
quote: corr = corr,
at: crates/viola-channel/src/client.rs:183
quote: corr = id,
@row_add n=532 row=contract/process-log-corr/channel-response area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: channel-response
promise: corr is the JSON-RPC request id.
@row_check n=532 row=contract/process-log-corr/channel-response sha=a5aac392a11e v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:476
quote: corr = self.corr,
at: crates/viola-channel/src/server.rs:459
quote: Self::new(started, Some(corr), Peer::Client(conn), method)
@row_add n=533 row=contract/process-log-corr/dialog-raised area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: dialog-raised
promise: corr is the dialog_id.
@row_check n=533 row=contract/process-log-corr/dialog-raised sha=4909afb26b9b v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:315
quote: corr = dialog_id,
@row_add n=534 row=contract/process-log-corr/dialog-answered area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: dialog-answered
promise: corr is the dialog_id.
@row_check n=534 row=contract/process-log-corr/dialog-answered sha=ffa0b835b3ee v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:389
quote: corr = pending.dialog_id,
at: src/run/dialog.rs:461
quote: corr = dialog_id,
@row_add n=535 row=contract/process-log-corr/hook-invoked area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: hook-invoked
promise: corr is the dialog_id for dialog hooks, otherwise null.
@row_check n=535 row=contract/process-log-corr/hook-invoked sha=ca806df4bd56 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:369
quote: corr = dialog_id,
at: src/cmd/hook.rs:173
quote: ObsEvent::HookInvoked,
@row_add n=536 row=contract/process-log-corr/hook-decision area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null.
@row_check n=536 row=contract/process-log-corr/hook-decision sha=5e9f82d41250 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:401
quote: corr = dialog_id,
at: src/cmd/hook.rs:592
quote: ObsEvent::HookDecision,
@row_add n=537 row=contract/process-log-corr/send-issued area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: send-issued
promise: corr is the send cursor.
@row_check n=537 row=contract/process-log-corr/send-issued sha=5af79873b2f7 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:196
quote: corr = cursor,
@row_add n=538 row=contract/process-log-corr/send-confirmed area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: send-confirmed
promise: corr is the send cursor.
@row_check n=538 row=contract/process-log-corr/send-confirmed sha=fe1c0be606fa v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:233
quote: corr = cursor,
@row_add n=539 row=contract/process-log-corr/send-refused area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: send-refused
promise: corr is the send cursor.
@row_check n=539 row=contract/process-log-corr/send-refused sha=cfe532b47045 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:283
quote: corr = corr,
@row_add n=540 row=contract/process-log-corr/process-start area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: process-start
promise: corr is null.
@row_check n=540 row=contract/process-log-corr/process-start sha=0c551265be5d v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:31
quote: ObsEvent::ProcessStart,
@row_add n=541 row=contract/process-log-corr/process-exit area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: process-exit
promise: corr is null.
@row_check n=541 row=contract/process-log-corr/process-exit sha=c02537f08954 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:68
quote: ObsEvent::ProcessExit,
@row_add n=542 row=contract/process-log-corr/panic area=obs-scope-summary rec=fct-ogkwmfvl@1
boundary: Process log corr by event
item: panic
promise: corr is null.
@row_check n=542 row=contract/process-log-corr/panic sha=87e762ee00e0 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:245
quote: "event": "panic",
@row_add n=543 row=decision/log-fields-additive-only area=obs-scope-summary rec=fct-ogkwmfvl@1
name: Required log fields only added to
rule: Fields may be added to process-log lines, but the required ones are never renamed or removed.
why: The harness greps on them.
@row_check n=543 row=decision/log-fields-additive-only sha=7a5bba09b97f v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=544 row=decision/single-line-log-records area=obs-scope-summary rec=fct-ogkwmfvl@1
name: Single-line log records
rule: Every log record is one JSON line parseable by jq; never a multi-line stack trace.
@row_check n=544 row=decision/single-line-log-records sha=613820410ed1 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:121
quote: .json()
at: src/obs.rs:315
quote: let mut line = Value::Object(record).to_string();
@row_add n=545 row=decision/diagnostics-file-modes area=obs-scope-summary rec=fct-ogkwmfvl@1,fct-khwnpdov@2
name: Diagnostics file and directory modes
rule: Diagnostics files are 0600 inside 0700 directories.
guard: secret-scan test
@row_check n=545 row=decision/diagnostics-file-modes sha=641da8f7c15e v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:27
quote: std::os::unix::fs::DirBuilderExt::mode(&mut builder, DIR_MODE);
at: crates/viola-state/src/fs.rs:48
quote: std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
guard_at: tests/run_cli.rs:576
@row_add n=546 row=decision/log-file-basenames-follow-tests area=obs-scope-summary rec=fct-khwnpdov@2
name: Log file names follow tests
rule: Process-log files are <home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson, as tests require.
why: Tests is upstream; arch and security place diagnostics under the instance directory instead.
@row_check n=546 row=decision/log-file-basenames-follow-tests sha=92c763fc326d v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:56
quote: (ObsProcess::Mcp, _) => "mcp".to_owned(),
at: src/obs.rs:58
quote: format!("{}-{}", process.as_str(), name_str(name))
at: src/obs.rs:87
quote: let dir = home.join("diagnostics");
@row_add n=547 row=contract/logs-wrapper/record-envelope area=obs-scope-summary rec=fct-khwnpdov@2
boundary: logs wrapper
item: record envelope
promise: Each process-log line is wrapped as {"src":"diag","file":<basename>,"record":<line>}.
@row_check n=547 row=contract/logs-wrapper/record-envelope sha=60da30b877db v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:177
quote: head.insert("src".to_owned(), "diag".into());
at: crates/viola-e2e/src/harness/logs.rs:178
quote: head.insert("file".to_owned(), file.into());
at: crates/viola-e2e/src/harness/logs.rs:200
quote: wrapped.insert("record".to_owned(), record);
@row_add n=548 row=decision/home-level-mcp-log-no-user-content area=obs-scope-summary rec=fct-khwnpdov@2
name: Home-level MCP log holds no user content
rule: An mcp.ndjson at home level never carries user content.
@row_check n=548 row=decision/home-level-mcp-log-no-user-content sha=14237b90f385 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:289-292
quote: home.join("instances") .join(name_str(instance)) .join("diagnostics") .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=549 row=decision/diagnostics-checked-at-open area=obs-scope-summary rec=fct-khwnpdov@2
name: Diagnostics checked before open
rule: Before a role or detail file is created or narrowed, the existing diagnostics/ folder and the file are checked, a symlink refused and no group or other bit allowed; a refusal is a failed open.
@row_check n=549 row=decision/diagnostics-checked-at-open sha=5f1f15fe1fc1 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:89
quote: check_diagnostics(&dir, &path)?;
at: src/obs.rs:328
quote: if check_diagnostics(dir, &path).is_err() || ensure_private_dir(dir).is_err() {
at: crates/viola-state/src/strict.rs:122-123
quote: if meta.file_type().is_symlink() { return Err(Refused::Symlink);
@row_add n=550 row=contract/run-start-events/order area=obs-scope-summary rec=fct-jqtgp6hh@1
boundary: run start events in events.ndjson
item: order
promise: A run start writes wheel{cause:"start"}, then budget-gate, then session-start.
@row_check n=550 row=contract/run-start-events/order sha=f4ecc29eb2b3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:619-622
quote: EventKind::Wheel, json!({"holder": Wheel::Driver.as_str(), "cause": WheelCause::Start.as_str()}), ), (EventKind::BudgetGate, budget.data()),
@row_add n=551 row=contract/run-log/cli-version area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run log line
item: cli_version
promise: An optional field on run's claude-child process-start line; absent when the CLI version is unreadable.
@row_check n=551 row=contract/run-log/cli-version sha=5f4060fd68a7 v=stale judge=sonnet head=8834195b0683
at: src/run/mod.rs:60
quote: cli_version = cli_version,
shows: cli_version is an Option written only on the claude-child process-start line and is absent from the line when the version is unreadable, so it is not a required field of run's log lines.
cell: promise
value: An optional field on run's claude-child process-start line; absent when the CLI version is unreadable.
@row_add n=552 row=contract/run-log/cli-verified area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run log line
item: cli_verified
promise: A required field on run's log lines.
@row_check n=552 row=contract/run-log/cli-verified sha=06a4e394dd78 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:61
quote: cli_verified = cli_verified,
@row_add n=553 row=contract/run-log/exit-code area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run log line
item: exit_code
promise: A required field on run's log lines.
@row_check n=553 row=contract/run-log/exit-code sha=36861adcf546 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:82
quote: exit_code = exit_code,
@row_add n=554 row=contract/run-exit-1-detail/already-live area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: already-live
promise: A live holder or a fresh replayed beat.
@row_check n=554 row=contract/run-exit-1-detail/already-live sha=b236de619861 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=555 row=contract/run-exit-1-detail/stale-heartbeat area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: stale-heartbeat
promise: A stale holder.
@row_check n=555 row=contract/run-exit-1-detail/stale-heartbeat sha=80626ce46349 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=556 row=contract/run-exit-1-detail/squatted-name area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: squatted-name
promise: A distinct detail code for a squatted name.
@row_check n=556 row=contract/run-exit-1-detail/squatted-name sha=bc85a836c3c8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=557 row=contract/run-exit-1-detail/sha-256-mismatch area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: sha-256 mismatch
promise: A distinct detail code for a sha-256 mismatch.
@row_check n=557 row=contract/run-exit-1-detail/sha-256-mismatch sha=d41434b88274 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=558 row=contract/run-exit-1-detail/no-session area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: no-session
promise: On a start made by viola revive, a distinct detail code.
@row_check n=558 row=contract/run-exit-1-detail/no-session sha=f3a23868d862 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=559 row=contract/run-exit-1-detail/session-live area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: session-live
promise: On a start made by viola revive, a distinct detail code.
@row_check n=559 row=contract/run-exit-1-detail/session-live sha=bb23527d4dc9 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=560 row=contract/run-exit-1-detail/cwd-missing area=obs-scope-summary rec=fct-h2wm2goy@2
boundary: run exit-1 detail codes
item: cwd-missing
promise: On a start made by viola revive, a distinct detail code.
@row_check n=560 row=contract/run-exit-1-detail/cwd-missing sha=f8be52931037 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=561 row=decision/detail-codes-no-paths area=obs-scope-summary rec=fct-h2wm2goy@2
name: Detail codes carry no path or pid
rule: No path or pid appears in a detail code.
@row_check n=561 row=decision/detail-codes-no-paths sha=b4ba1db703f5 v=holds judge=sonnet head=8834195b0683
searched: detail\s*=\s*[^"a-z_]*(path|display\(\)|pid) · in src/**/*.rs and crates/**/*.rs
@row_add n=562 row=contract/send-path-log-events/event-order area=obs-scope-summary rec=fct-zrxuhqku@1
boundary: Send path log events
item: event order
promise: A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none awaited for an unconfirmable send), then send-confirmed or send-refused, then channel-response.
@row_check n=562 row=contract/send-path-log-events/event-order sha=13227f7fe424 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:367
quote: ObsEvent::ChannelRequest,
at: src/run/send.rs:195
quote: ObsEvent::SendIssued,
at: src/cmd/hook.rs:173
quote: ObsEvent::HookInvoked,
at: src/run/send.rs:232
quote: ObsEvent::SendConfirmed,
at: src/run/send.rs:282
quote: ObsEvent::SendRefused,
at: crates/viola-channel/src/server.rs:475
quote: ObsEvent::ChannelResponse,
@row_add n=563 row=contract/send-log-line/required-fields area=obs-scope-summary rec=fct-gpdjinuy@1
boundary: Send log line
item: required fields
promise: A send line carries event, process, instance, corr, from, confirmed, refusal, detail and duration_ms.
@row_check n=563 row=contract/send-log-line/required-fields sha=8e3d11e8a7a2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:145-147
quote: event = event.as_str(), process = $crate::obs::ctx_process(), instance = $crate::obs::ctx_instance(),
at: src/run/send.rs:283
quote: corr = corr,
at: src/run/send.rs:287
quote: from = self.sender(),
at: src/run/send.rs:239
quote: confirmed = confirmed,
at: src/run/send.rs:291-292
quote: refusal = refusal.as_str(), detail = detail,
at: src/run/send.rs:240
quote: duration_ms = duration_ms,
@row_add n=564 row=contract/send-log-line/corr area=obs-scope-summary rec=fct-gpdjinuy@1,fct-exdrb7s2@1
boundary: Send log line
item: corr
promise: On the wrapper's send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused (a refusal before send-issued carries the log's end offset); the CLI's and MCP's client-side send-refused lines carry no corr; on channel lines corr is the JSON-RPC id.
@row_check n=564 row=contract/send-log-line/corr sha=520b1ae419d0 v=stale judge=sonnet head=8834195b0683
at: src/cmd/send.rs:237-243
quote: obs_event!( INFO, ObsEvent::SendRefused, side = "client", refusal = RefusalReason::NotDelivered.as_str(), detail = detail.as_str(), );
shows: Only the wrapper's send-issued, send-confirmed and send-refused lines carry the cursor as corr (send.rs:196, 234, 283); the CLI's and MCP's client-side send-refused lines (also crates/viola-mcp/src/tools.rs:308-312) carry no corr, and a wrapper refusal before send-issued carries the log's end offset, so nothing joins them across CLI and MCP.
cell: promise
value: On the wrapper's send lines corr is the send cursor, which joins send-issued, send-confirmed and send-refused (a refusal before send-issued carries the log's end offset); the CLI's and MCP's client-side send-refused lines carry no corr; on channel lines corr is the JSON-RPC id.
@row_add n=565 row=contract/send-log-line/from area=obs-scope-summary rec=fct-gpdjinuy@1,fct-jlmrrvwc@1
boundary: Send log line
item: from
promise: The from field is self-reported and logged as unauthenticated, never as an identity.
@row_check n=565 row=contract/send-log-line/from sha=bc5a32e7d105 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:166
quote: self.from.as_ref().map(|_| "self-reported")
@row_add n=566 row=contract/send-log-line/detail area=obs-scope-summary rec=fct-gpdjinuy@1
boundary: Send log line
item: detail
promise: The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text or turn-running.
@row_check n=566 row=contract/send-log-line/detail sha=854e7a77b7ce v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:221
quote: Self::InputNotReady => "input-not-ready",
at: crates/viola-core/src/lib.rs:222
quote: Self::NoPromptSubmitted => "no-prompt-submitted",
at: crates/viola-core/src/lib.rs:223
quote: Self::TurnRunning => "turn-running",
at: crates/viola-core/src/lib.rs:225
quote: Self::ControlCharacter => "control-character",
at: crates/viola-core/src/lib.rs:226
quote: Self::EmptyText => "empty-text",
@row_add n=567 row=contract/wait-path-log-events/channel-pair area=obs-scope-summary rec=fct-leqqhyxz@1
boundary: Wait path log events
item: channel pair
promise: A wait logs channel-request and channel-response, the response carrying the outcome.
@row_check n=567 row=contract/wait-path-log-events/channel-pair sha=dd79ec6a99ee v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:367
quote: ObsEvent::ChannelRequest,
at: crates/viola-channel/src/server.rs:475
quote: ObsEvent::ChannelResponse,
at: crates/viola-channel/src/server.rs:483
quote: outcome = self.outcome,
@row_add n=568 row=contract/wait-log-line/required-fields area=obs-scope-summary rec=fct-cetbw4yv@1
boundary: Wait log line
item: required fields
promise: A wait line carries event, process, instance, corr (the JSON-RPC id), after (a byte offset), timeout_ms, outcome, exit_code and duration_ms.
@row_check n=568 row=contract/wait-log-line/required-fields sha=ec9410ffdc02 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:183
quote: corr = id,
at: crates/viola-channel/src/client.rs:188
quote: after = after,
at: crates/viola-channel/src/client.rs:189
quote: timeout_ms = timeout_ms,
at: crates/viola-channel/src/client.rs:209
quote: logged.outcome = wait_outcome(label, &reply["result"]);
at: src/cmd/client.rs:258
quote: ObsEvent::ProcessExit,
at: src/cmd/client.rs:260
quote: exit_code = 21u8,
at: src/cmd/client.rs:263
quote: duration_ms = obs::duration_ms(),
@row_add n=569 row=contract/wait-log-line/outcome area=obs-scope-summary rec=fct-cetbw4yv@1
boundary: Wait log line
item: outcome
promise: The wait outcome is the woken kind, timed-out or instance-unreachable.
@row_check n=569 row=contract/wait-log-line/outcome sha=701b4d906695 v=stale judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:396
quote: return Some("timed-out");
shows: The timeout outcome is written as the hyphenated word timed-out, not timed_out.
cell: promise
value: The wait outcome is the woken kind, timed-out or instance-unreachable.
@row_add n=570 row=contract/wait-log-line/exit-code-21 area=obs-scope-summary rec=fct-cetbw4yv@1
boundary: Wait log line
item: exit_code 21
promise: A wait exits 21 when the wrapper is killed mid-wait (detail endpoint-lost, during call), or when a CLI wait meets a refused socket directory before any frame (detail socket-dir-refused, during connect, no channel-request).
@row_check n=570 row=contract/wait-log-line/exit-code-21 sha=34e6720403f1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:199
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => {
at: src/cmd/client.rs:136
quote: Self::Lost => "call",
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
at: src/cmd/client.rs:265
quote: 21
@row_add n=571 row=contract/dialog-path-log-events/event-order area=obs-scope-summary rec=fct-3dvlg5hs@1
boundary: Dialog path log events
item: event order
promise: A dialog logs hook-invoked, dialog-raised, dialog-answered, then hook-decision, whose decision is null on deadline expiry.
@row_check n=571 row=contract/dialog-path-log-events/event-order sha=abf452b8bff8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:368
quote: ObsEvent::HookInvoked,
at: src/run/dialog.rs:314
quote: ObsEvent::DialogRaised,
at: src/run/dialog.rs:388
quote: ObsEvent::DialogAnswered,
at: src/cmd/hook.rs:391
quote: ObsEvent::HookDecision,
at: src/cmd/hook.rs:359
quote: return None;
@row_add n=572 row=contract/dialog-log-line/required-fields area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: required fields
promise: A dialog line carries event, process, instance, corr, dialog_kind, deadline_hit and duration_ms; refusal or detail does not appear on a dialog-raised or dialog-answered line.
@row_check n=572 row=contract/dialog-log-line/required-fields sha=6d9960f3a912 v=stale judge=sonnet head=8834195b0683
at: src/run/dialog.rs:388
quote: ObsEvent::DialogAnswered,
at: src/run/dialog.rs:389
quote: corr = pending.dialog_id,
at: src/run/dialog.rs:390
quote: dialog_kind = pending.kind.as_str(),
at: src/run/dialog.rs:391
quote: deadline_hit = true,
at: src/run/dialog.rs:392
quote: duration_ms = millis(self.clock.now().saturating_duration_since(pending.raised)),
shows: The dialog lines (dialog-raised, dialog-answered) carry corr, dialog_kind, deadline_hit and duration_ms but no refusal or detail field; the refusals unknown-dialog and unverified-cli are returned in the reply and written to no dialog line.
cell: promise
value: A dialog line carries event, process, instance, corr, dialog_kind, deadline_hit and duration_ms; refusal or detail does not appear on a dialog-raised or dialog-answered line.
@row_add n=573 row=contract/dialog-log-line/corr area=obs-scope-summary rec=fct-ycxx4ij4@2,fct-exdrb7s2@1
boundary: Dialog log line
item: corr
promise: On dialog lines corr is the dialog_id, which joins hook and wrapper lines; the driver's channel lines carry the JSON-RPC id.
@row_check n=573 row=contract/dialog-log-line/corr sha=dfa8cf214c08 v=stale judge=sonnet head=8834195b0683
at: src/run/dialog.rs:315
quote: corr = dialog_id,
at: src/cmd/hook.rs:369
quote: corr = dialog_id,
at: crates/viola-channel/src/client.rs:183
quote: corr = id,
shows: Hook and wrapper lines carry corr = dialog_id, but the driver's answer channel-request line carries the JSON-RPC request id as corr, so dialog_id does not join the driver side.
cell: promise
value: On dialog lines corr is the dialog_id, which joins hook and wrapper lines; the driver's channel lines carry the JSON-RPC id.
@row_add n=574 row=contract/dialog-log-line/unknown-dialog area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: unknown-dialog
promise: The detail unknown-dialog goes with exit 13.
@row_check n=574 row=contract/dialog-log-line/unknown-dialog sha=7fefeb737f27 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:446
quote: Some(NotDelivered::UnknownDialog.as_str()),
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=575 row=contract/dialog-log-line/unverified-cli area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: unverified-cli
promise: The detail unverified-cli goes with exit 12.
@row_check n=575 row=contract/dialog-log-line/unverified-cli sha=d41c60524994 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:436
quote: return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
@row_add n=576 row=contract/dialog-log-line/server-verify-failed area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: server-verify-failed
promise: A server-verify failure exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
@row_check n=576 row=contract/dialog-log-line/server-verify-failed sha=ddbd1e823fc8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
at: src/cmd/client.rs:265
quote: 21
at: src/cmd/hook.rs:305
quote: Asked::ServerUnverified => Some(NotSent::ServerUnverified.detail()),
at: src/cmd/hook.rs:384
quote: Asked::Deadline | Asked::Unreachable | Asked::ServerUnverified | Asked::SocketDir => None,
at: src/cmd/hook.rs:393
quote: decision_emitted = emitted,
@row_add n=577 row=contract/dialog-log-line/socket-dir-refused area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog log line
item: socket-dir-refused
promise: A refused socket directory exits 21 on CLI answer with during connect, and on the dialog hook fails open with corr null and decision_emitted false; the line names no pid, start time, path or endpoint.
@row_check n=577 row=contract/dialog-log-line/socket-dir-refused sha=ef94da971ba0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
at: src/cmd/client.rs:265
quote: 21
at: src/cmd/hook.rs:306
quote: Asked::SocketDir => Some(NotSent::SocketDir.detail()),
at: src/cmd/hook.rs:384
quote: Asked::Deadline | Asked::Unreachable | Asked::ServerUnverified | Asked::SocketDir => None,
at: src/cmd/hook.rs:393
quote: decision_emitted = emitted,
@row_add n=578 row=contract/cli-frame-verbs/strict-modes-failed area=obs-scope-summary rec=fct-ycxx4ij4@2,fct-ydc7ueix@3
boundary: CLI frame verbs
item: strict-modes-failed
promise: A strict-modes refusal of send, wait, last, answer, pause or release writes no line and prints only the fixed instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, exit 21, with no pid, path or endpoint.
@row_check n=578 row=contract/cli-frame-verbs/strict-modes-failed sha=51ef685d260f v=holds judge=sonnet head=8834195b0683
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
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=579 row=contract/dialog-hook/refused-instance-check area=obs-scope-summary rec=fct-ycxx4ij4@2
boundary: Dialog hook
item: refused instance check
promise: A refused instance check on the dialog hook exits 0 at once with empty streams, no frame and no line.
@row_check n=579 row=contract/dialog-hook/refused-instance-check sha=d3a8420c2b45 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=580 row=contract/wheel-path-log-events/pause-and-release area=obs-scope-summary rec=fct-m4ttxbfi@1
boundary: Wheel path log events
item: pause and release
promise: A wheel take-over logs send-refused for the refused send and a channel-request and channel-response pair for pause and for release.
@row_check n=580 row=contract/wheel-path-log-events/pause-and-release sha=c96de923d171 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:282
quote: ObsEvent::SendRefused,
at: crates/viola-channel/src/server.rs:342-343
quote: "pause", "release",
at: crates/viola-channel/src/server.rs:367
quote: ObsEvent::ChannelRequest,
at: crates/viola-channel/src/server.rs:475
quote: ObsEvent::ChannelResponse,
@row_add n=581 row=contract/diag-line-event/release-from-driver area=obs-scope-summary rec=fct-m4ttxbfi@1,fct-n7xftqkr@2
boundary: Diag line event
item: release-from-driver
promise: release-from-driver is its own logged event, admitted by the diag-line schema, not a generic -32602 wrapper fault.
@row_check n=581 row=contract/diag-line-event/release-from-driver sha=dcbc343ba539 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:240
quote: ObsEvent::ReleaseFromDriver,
at: schemas/diag-line.v1.json:62
quote: { "if": { "properties": { "event": { "const": "release-from-driver" } } }, "then": { "properties": {
@row_add n=582 row=contract/wheel-log-line/human-typing area=obs-scope-summary rec=fct-ylcomffg@1
boundary: Wheel log line
item: human-typing
promise: A send refused while a human holds the wheel logs refusal human-typing, detail null or manual-pause, the wheel holder and exit_code 10.
@row_check n=582 row=contract/wheel-log-line/human-typing sha=44ec132b1dea v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:291
quote: refusal = refusal.as_str(),
at: src/run/send.rs:290
quote: wheel = self.wheel.holder().as_str(),
at: crates/viola-core/src/lib.rs:242
quote: Self::ManualPause => "manual-pause",
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
@row_add n=583 row=contract/budget-path-log-events/event-set area=obs-scope-summary rec=fct-te5knmlh@1
boundary: Budget path log events
item: event set
promise: The budget path logs hook-invoked and hook-decision for statusline, and send-refused with budget-paused.
@row_check n=583 row=contract/budget-path-log-events/event-set sha=39e669b7da12 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:104
quote: ObsEvent::HookInvoked,
at: src/cmd/hook/statusline.rs:160
quote: ObsEvent::HookDecision,
at: src/run/send/budget.rs:11
quote: RefusalReason::BudgetPaused,
@row_add n=584 row=contract/budget-log-line/window area=obs-scope-summary rec=fct-kbwooksf@1
boundary: Budget log line
item: window
promise: The budget window is five-hour or seven-day.
@row_check n=584 row=contract/budget-log-line/window sha=6a151ce606ef v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:259
quote: Self::FiveHour => "five-hour",
at: crates/viola-core/src/lib.rs:260
quote: Self::SevenDay => "seven-day",
@row_add n=585 row=decision/statusline-command-never-in-home-logs area=obs-scope-summary rec=fct-kbwooksf@1,fct-3znzbm34@1
name: Statusline command never in home-level logs
rule: The statusline_command string never appears outside diagnostics/, reaching a detail file only as part of an error chain; its shell-out records only shell_exit_status and duration_ms.
why: The statusline command is classed High for integrity.
@row_check n=585 row=decision/statusline-command-never-in-home-logs sha=303cd52ee571 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:196
quote: fields(shell_exit_status = Empty, duration_ms = Empty)
searched: statusline_command · in src/**/*.rs (no obs_event! or span field carries it; only snapshot struct fields)
@row_add n=586 row=decision/channel-log-scrub-rule area=obs-scope-summary rec=fct-jlmrrvwc@1
name: Channel log scrub rule
rule: Channel request and response lines log only the method, id or corr, the result class (ok, refusal or error code), the sender version and the closed detail word, never params or result bodies.
@row_check n=586 row=decision/channel-log-scrub-rule sha=f2060fa0c31e v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:371
quote: method = method,
at: crates/viola-channel/src/server.rs:372
quote: sender = params.sender.as_deref().and_then(version_label),
at: crates/viola-channel/src/server.rs:480-482
quote: result_class = result_class, error_code = fault.map(ProtocolError::code), detail = fault.and_then(ProtocolError::cause),
@row_add n=587 row=decision/send-refused-by-each-deciding-side area=obs-scope-summary rec=fct-gtr2jsdi@1
name: Send refusal logged by each deciding side
rule: A send-refused with detail control-character or empty-text is emitted by each side that decides it, client and wrapper, without the text.
@row_check n=587 row=decision/send-refused-by-each-deciding-side sha=e29b978952ea v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:240
quote: side = "client",
at: src/run/send.rs:289
quote: side = "wrapper",
at: src/run/send.rs:292
quote: detail = detail,
@row_add n=588 row=contract/hook-decision/server-verify-failed area=obs-scope-summary rec=fct-mxwnyc6t@2
boundary: Hook decision line
item: server-verify-failed
promise: A server-verify mismatch is one hook-decision at WARN with detail server-verify-failed and no frame sent; on a dialog hook, hook-invoked with corr null then hook-decision with corr null and decision_emitted false; no pid, start time, path or endpoint.
@row_check n=588 row=contract/hook-decision/server-verify-failed sha=f7480d52b138 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:236-237
quote: if !peer_admitted(client.server_euid()) { return Err(NotSent::ServerUnverified);
at: src/cmd/hook.rs:285-286
quote: invoked(event, &invoked_at, None); dialog_decided(event, &Asked::Unreachable, false, Some(detail), started);
at: src/cmd/hook.rs:389-390
quote: Some(detail) => obs_event!( WARN,
@row_add n=589 row=contract/hook-decision/socket-dir-refused area=obs-scope-summary rec=fct-mxwnyc6t@2,fct-5ckzuzgq@2
boundary: Hook decision line
item: socket-dir-refused
promise: A refused socket directory on viola hook writes one hook-decision in the server-verify form with detail socket-dir-refused, and the hook exits 0.
@row_check n=589 row=contract/hook-decision/socket-dir-refused sha=8666042de495 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:231-232
quote: if check_endpoint_dir(endpoint).is_err() { return Err(NotSent::SocketDir);
at: src/cmd/hook.rs:215
quote: Self::SocketDir => UnreachableCause::SocketDir.as_str(),
at: src/cmd/hook.rs:148-149
quote: decided(event, detail, started); Ok(ExitCode::SUCCESS)
@row_add n=590 row=contract/hook-fail-open/exit area=obs-scope-summary rec=fct-mxwnyc6t@2
boundary: Hook fail-open
item: exit
promise: A hook that fails open exits 0 and leaves stderr empty.
@row_check n=590 row=contract/hook-fail-open/exit sha=e59b0223c68a v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:148-149
quote: decided(event, detail, started); Ok(ExitCode::SUCCESS)
at: src/cmd/hook.rs:135
quote: Ok(()) => Ok(ExitCode::SUCCESS),
@row_add n=591 row=decision/drift-reports-detail-only area=obs-scope-summary rec=fct-mxwnyc6t@2,fct-glcip7ij@1
name: Drift reports in the detail file only
rule: A full serde drift report goes only to the instance-scoped detail file, as its drift_report field.
@row_check n=591 row=decision/drift-reports-detail-only sha=29b2cf68c43b v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:547
quote: fields.insert("drift_report".to_owned(), Value::Array(drift.to_vec()));
@row_add n=592 row=contract/mcp-invalid-params/detail area=obs-scope-summary rec=fct-4bhscgzh@1
boundary: MCP invalid-params
item: detail
promise: The detail of an MCP invalid-params is a closed field word: target, text, after, timeout_ms, dialog_id or response.
@row_check n=592 row=contract/mcp-invalid-params/detail sha=5b483ee52b9c v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:266
quote: .ok_or("target")
at: crates/viola-mcp/src/tools.rs:270
quote: args.get("text").and_then(Value::as_str).ok_or("text")
at: crates/viola-mcp/src/tools.rs:277
quote: Some(value) => value.as_u64().map(Some).ok_or(field),
at: crates/viola-mcp/src/tools.rs:284
quote: .ok_or("dialog_id")
at: crates/viola-mcp/src/tools.rs:291
quote: .ok_or("response")
@row_add n=593 row=contract/mcp-frame-tool-refusal/strict-modes-failed area=obs-scope-summary rec=fct-4bhscgzh@1,fct-ydc7ueix@3
boundary: MCP frame tool refusal
item: strict-modes-failed
promise: A frame tool (send, wait, last, answer) whose instance check refuses returns instance-unreachable with detail strict-modes-failed, writes no frame and no line.
@row_check n=593 row=contract/mcp-frame-tool-refusal/strict-modes-failed sha=06d9f33465cf v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-340
quote: if check_instance(&ctx.home, &instance_dir).is_err() { return Ok(result::unreachable(UnreachableCause::StrictModes));
@row_add n=594 row=contract/mcp-frame-tool-refusal/no-live-endpoint area=obs-scope-summary rec=fct-4bhscgzh@1
boundary: MCP frame tool refusal
item: no live endpoint
promise: A frame tool with no live endpoint returns instance-dead or unwrapped-name and writes no frame.
@row_check n=594 row=contract/mcp-frame-tool-refusal/no-live-endpoint sha=c5faaab5e542 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:342-343
quote: let Some(live) = live_endpoint(&instance_dir) else { return Ok(result::unreachable(without_endpoint(&instance_dir)));
at: crates/viola-state/src/liveness.rs:112-113
quote: Ok(metadata) if metadata.is_dir() => UnreachableCause::InstanceDead, _ => UnreachableCause::UnwrappedName,
@row_add n=595 row=contract/mcp-frame-tool-refusal/socket-dir-refused area=obs-scope-summary rec=fct-4bhscgzh@1,fct-5ckzuzgq@2
boundary: MCP frame tool refusal
item: socket-dir-refused
promise: A frame tool whose socket directory is refused returns socket-dir-refused, writes no frame and no line.
@row_check n=595 row=contract/mcp-frame-tool-refusal/socket-dir-refused sha=986d5c53daa0 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() { return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=596 row=contract/mcp-frame-tool-refusal/server-verify-failed area=obs-scope-summary rec=fct-4bhscgzh@1
boundary: MCP frame tool refusal
item: server-verify-failed
promise: A frame tool whose server check fails or whose server euid is refused returns server-verify-failed and writes no frame.
@row_check n=596 row=contract/mcp-frame-tool-refusal/server-verify-failed sha=430d18f0827c v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:352-353
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? { return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=597 row=decision/messaging-secrets-never-logged area=obs-scope-summary rec=fct-rkannnby@1,fct-ja2bi6wz@1
name: Messaging secrets never logged
rule: CLAUDE_CODE_MESSAGING_TOKEN, CLAUDE_CODE_MESSAGING_SOCKET and every R8-stripped CLAUDE* value are never logged in any sink, detail files and fixtures included.
guard: secret-scan test
@row_check n=597 row=decision/messaging-secrets-never-logged sha=596faf212d72 v=holds guard=true judge=sonnet head=8834195b0683
searched: CLAUDE_CODE_MESSAGING_TOKEN|CLAUDE_CODE_MESSAGING_SOCKET · in src/**/*.rs and crates/**/*.rs (only the strip lists in crates/viola-agent-claude/src/lib.rs:23-24, no log call)
guard_at: tests/run_cli.rs:478
@row_add n=598 row=decision/r8-strip-logged-as-names-only area=obs-scope-summary rec=fct-rkannnby@1,fct-ja2bi6wz@1
name: R8 strip logged as names only
rule: The R8 environment strip is logged as a count and lists of stripped and kept variable names, values never read, and environment maps never pass to a span or event.
@row_check n=598 row=decision/r8-strip-logged-as-names-only sha=0d319b58386e v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:57
quote: env_stripped_count = strip.count(),
at: src/run/mod.rs:58
quote: env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
at: src/run/mod.rs:59
quote: env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()),
@row_add n=599 row=decision/serde-errors-become-fixed-message area=obs-scope-summary rec=fct-rkannnby@1,fct-glcip7ij@1
name: Serde errors become a fixed message
rule: An error chain holding a serde source is replaced by a fixed message before it reaches stderr.
@row_check n=599 row=decision/serde-errors-become-fixed-message sha=6079e368e63d v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
at: src/obs.rs:271-274
quote: let chain: Vec<Value> = error .chain() .map(|cause| Value::from(cause.to_string())) .collect();
@row_add n=600 row=decision/redaction-cannot-be-disabled area=obs-scope-summary rec=fct-rkannnby@1
name: Redaction cannot be disabled
rule: No environment variable, flag or config setting may disable log redaction.
@row_check n=600 row=decision/redaction-cannot-be-disabled sha=2ab52420a8e7 v=holds judge=sonnet head=8834195b0683
searched: RUST_LOG|EnvFilter|VIOLA_LOG|VIOLA_DEBUG|redact · in src/**/*.rs and crates/**/*.rs (only a doc comment "never RUST_LOG" at src/obs.rs:180 and the params Debug redaction in crates/viola-channel/src/frame.rs)
@row_add n=601 row=decision/diagnostics-owner-only-permissions area=obs-scope-summary rec=fct-5ckzuzgq@2
name: Diagnostics owner-only permissions
rule: Files under diagnostics/ are 0600 inside 0700 directories.
guard: secret-scan test
@row_check n=601 row=decision/diagnostics-owner-only-permissions sha=043b4e950da3 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:10
quote: pub const DIR_MODE: u32 = 0o700;
at: crates/viola-state/src/fs.rs:11
quote: pub const FILE_MODE: u32 = 0o600;
at: src/obs.rs:92
quote: Ok(Arc::new(open_private_append(&path)?))
guard_at: crates/viola-e2e/src/harness/secret_scan.rs:179
@row_add n=602 row=contract/process-exit/socket-dir-refused-on-start area=obs-scope-summary rec=fct-5ckzuzgq@2
boundary: Process exit line
item: socket-dir-refused on a start
promise: A refused socket directory on a start writes one process-exit with detail socket-dir-refused, exit 1.
@row_check n=602 row=contract/process-exit/socket-dir-refused-on-start sha=0050ce72d248 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:412-413
quote: run::log_self_exit(1, Some(detail)); Started::Refused(ExitCode::from(1))
@row_add n=603 row=contract/process-exit/socket-dir-refused-on-frame-verb area=obs-scope-summary rec=fct-5ckzuzgq@2
boundary: Process exit line
item: socket-dir-refused on a CLI frame verb
promise: A refused socket directory on a CLI frame verb writes one process-exit with detail socket-dir-refused, exit 21, during connect.
@row_check n=603 row=contract/process-exit/socket-dir-refused-on-frame-verb sha=bc367ea3fae8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:123
quote: Self::SocketDir => UnreachableCause::SocketDir,
at: src/cmd/client.rs:260-262
quote: exit_code = 21u8, detail = cause.detail(), during = cause.during(),
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=604 row=decision/pinned-copy-race-adds-no-line area=obs-scope-summary rec=fct-5ckzuzgq@2
name: Pinned-copy race adds no telemetry
rule: A pinned-exe hash mismatch reached when a concurrent first start wins the no-clobber placement leaves the file as found and adds no line, field or detail code beyond pinned-hash-mismatch.
@row_check n=604 row=decision/pinned-copy-race-adds-no-line sha=a61c20ce854d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:108
quote: Placed::Present => Err(PinError::HashMismatch),
@row_add n=605 row=decision/skipped-counters area=obs-scope-summary rec=fct-5ckzuzgq@2,fct-plg35lpj@1
name: Skipped counters
rule: Unknown event kinds, unknown fields and torn lines are each counted in skipped counters.
@row_check n=605 row=decision/skipped-counters sha=3d84e0addc3e v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:245
quote: self.skipped.torn_lines += 1;
at: crates/viola-state/src/events.rs:262
quote: self.skipped.unknown_kinds += 1;
at: crates/viola-state/src/events.rs:266
quote: self.skipped.unknown_fields += 1;
@row_add n=606 row=contract/process-log/child-and-probe-lifecycle area=obs-scope-summary rec=fct-u2gugwuk@1
boundary: Process log
item: child and probe lifecycle
promise: process-start and process-exit are logged for the claude child and for the claude agents --json and --version probes.
@row_check n=606 row=contract/process-log/child-and-probe-lifecycle sha=748dddcacbe8 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:54
quote: subject = "claude-child",
at: src/run/mod.rs:66-69
quote: obs_event!( INFO, ObsEvent::ProcessExit, subject = "claude-child",
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/run/agents.rs:64
quote: ObsEvent::ProcessExit,
at: src/run/version_gate.rs:149
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "version-probe");
at: src/run/version_gate.rs:165
quote: ObsEvent::ProcessExit,
@row_add n=607 row=decision/parse-failures-surface-as-unknown area=obs-scope-summary rec=fct-u2gugwuk@1
name: Parse failures surface as unknown
rule: A parse failure surfaces as the value unknown plus a counter.
@row_check n=607 row=decision/parse-failures-surface-as-unknown sha=4c05e0fdaec9 v=unverifiable judge=sonnet head=8834195b0683
searched: parse-rejected · in src/**/*.rs (counts emitted as ObsEvent::ParseRejected lines, but no single line shows a parse failure surfacing as the value unknown)
why: no evidence found
@row_add n=608 row=decision/controls-escaped-in-terminal-output area=obs-scope-summary rec=fct-u2gugwuk@1,fct-7eu65g47@1
name: Controls escaped in terminal output
rule: C0 and C1 control characters are escaped in any human terminal rendering of logged rows.
@row_check n=608 row=decision/controls-escaped-in-terminal-output sha=a5f01b27da54 v=holds judge=sonnet head=8834195b0683
at: src/human/escape.rs:23
quote: if c.is_control() {
@row_add n=609 row=decision/tracing-subscriber-logger area=obs-scope-summary rec=fct-3og7ujbn@2
name: tracing-subscriber is the logger
rule: The logger dependency is tracing-subscriber at version 0.3.20 or later.
@row_check n=609 row=decision/tracing-subscriber-logger sha=450d45161e74 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:243
quote: tracing-subscriber = { version = "=0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }
@row_add n=610 row=decision/obs-deps-no-c-building-crates area=obs-scope-summary rec=fct-3og7ujbn@2
name: Obs dependencies build no C
rule: Observability dependencies add no C-building crates.
@row_check n=610 row=decision/obs-deps-no-c-building-crates sha=03ec5addfe3c v=holds judge=sonnet head=8834195b0683
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
@row_add n=611 row=decision/obs-deps-pass-cargo-deny area=obs-scope-summary rec=fct-3og7ujbn@2
name: Obs dependencies pass cargo deny
rule: Observability dependencies pass cargo deny on its one target, x86_64-unknown-linux-gnu.
guard: cargo deny
@row_check n=611 row=decision/obs-deps-pass-cargo-deny sha=3da0c0292c4e v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:5
quote: targets = ["x86_64-unknown-linux-gnu"]
guard_at: deny.toml:5
@row_add n=612 row=contract/hook-log-line/duration-ms area=obs-scope-summary rec=fct-dijwakce@1
boundary: Hook log line
item: duration_ms
promise: duration_ms is on hook-decision, or on process-exit for process hook, for each hook event.
@row_check n=612 row=contract/hook-log-line/duration-ms sha=0a2de7c6699a v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:395
quote: duration_ms = duration_ms,
at: src/cmd/hook/statusline.rs:209-212
quote: ObsEvent::ProcessExit, subject = SUBJECT, shell_exit_status = code, duration_ms = duration_ms,
@row_add n=613 row=decision/heartbeat-staleness-threshold area=obs-scope-summary rec=fct-dijwakce@1
name: Heartbeat staleness threshold
rule: A heartbeat flips to stale after more than 5 s: 4.9 s reads live, 5.1 s reads stale or gone.
@row_check n=613 row=decision/heartbeat-staleness-threshold sha=aef766b7c649 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:16
quote: pub const STALE_AFTER: Duration = Duration::from_secs(5);
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
@row_add n=614 row=decision/latency-aggregation-outside-process area=obs-scope-summary rec=fct-dijwakce@1
name: Latency aggregation outside the process
rule: Percentile aggregation is done by the harness or jq over the ndjson, never by an in-process metrics pipeline.
@row_check n=614 row=decision/latency-aggregation-outside-process sha=66d0c407c63f v=holds judge=sonnet head=8834195b0683
searched: metrics|prometheus|opentelemetry|percentile|p95 · in Cargo.toml, crates/*/Cargo.toml, src/**/*.rs
@row_add n=615 row=decision/fault-telemetry-per-class area=obs-scope-summary rec=fct-yfoyoh37@2
name: Fault telemetry per class
rule: Each fault class emits its own telemetry: a healed torn ndjson line with file and offset, a corrupt or unsupported snapshot recovered by replay, a child exit seen without EOF, an endpoint vanished during wait, and a stale heartbeat against a live pid.
@row_check n=615 row=decision/fault-telemetry-per-class sha=1c2869d47fb5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:339
quote: detail = "torn-line-healed",
at: crates/viola-state/src/replay.rs:165
quote: ObsEvent::StateRecovered,
at: src/run/mod.rs:70
quote: child_exit_status = exit.code,
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=616 row=decision/one-linux-log-sink area=obs-scope-summary rec=fct-7bdi4wod@2
name: One file sink on Linux
rule: One file sink and one line schema serve Linux, with no OS-specific exporter.
@row_check n=616 row=decision/one-linux-log-sink sha=cc3806a2fb3d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:44
quote: pub fn open_private_append(path: &Path) -> io::Result<File> {
at: deny.toml:32
quote: { crate = "opentelemetry-otlp", reason = "no telemetry exporter or egress in v1" },
@row_add n=617 row=decision/log-append-atomicity area=obs-scope-summary rec=fct-7bdi4wod@2
name: Append atomicity for concurrent writers
rule: Concurrent hook writers append to a log atomically through POSIX O_APPEND.
@row_check n=617 row=decision/log-append-atomicity sha=806794582acc v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:46
quote: options.create(true).append(true);
@row_add n=618 row=decision/server-check-peer-credentials area=obs-scope-summary rec=fct-7bdi4wod@2
name: Server check by peer credentials
rule: The server check reads the peer pid by SO_PEERCRED and the peer euid; a mismatch of either reads server-verify-failed and an unreadable euid is refused.
@row_check n=618 row=decision/server-check-peer-credentials sha=baf82f90d921 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:133
quote: let pid = stream.peer_creds().ok()?.pid()?;
at: crates/viola-channel/src/client.rs:156
quote: stream.peer_creds().ok()?.euid()
at: src/cmd/hook.rs:214
quote: Self::ServerUnverified => "server-verify-failed",
@row_add n=619 row=decision/no-formal-slo area=obs-scope-summary rec=fct-rjubzxp3@1
name: No formal SLO
rule: No formal latency or availability SLO is held beyond the tests perf gates.
@row_check n=619 row=decision/no-formal-slo sha=eab6767c74b6 v=holds judge=sonnet head=8834195b0683
searched: \bSLO\b|availability · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs
@row_add n=620 row=decision/no-log-rotation-v1 area=obs-scope-summary rec=fct-lmrmnkyn@1
name: No log rotation in v1
rule: events.ndjson and diagnostics/*.ndjson are not rotated in v1; any future rotation keeps 0600/0700 permissions and keeps wait after and send cursor offsets valid.
why: The trigger is operational, not compliance: the security tier is Minimal with no compliance triggers.
@row_check n=620 row=decision/no-log-rotation-v1 sha=e36114d4519f v=holds judge=sonnet head=8834195b0683
searched: rotate|rotation|logrotate · in src/**/*.rs, crates/**/*.rs
@row_add n=621 row=decision/structured-json-process-logs area=obs-scope-summary rec=fct-urji5is2@1
name: Structured JSON process logs
rule: Every process role (run, hook, mcp, cli) writes structured JSON process logs under diagnostics/, which harness logs merges with events.ndjson.
@row_check n=621 row=decision/structured-json-process-logs sha=c3e56192a74d v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name))
at: src/obs.rs:56
quote: (ObsProcess::Mcp, _) => "mcp".to_owned(),
at: crates/viola-e2e/src/harness/logs.rs:1
quote: //! `logs`: every `<home>/diagnostics/*.ndjson` line as `{"src":"diag","file","record"}`, then
@row_add n=622 row=decision/tests-event-list-adopted area=obs-scope-summary rec=fct-urji5is2@1
name: Tests event list adopted
rule: The tests' field and event list is adopted verbatim, with send-issued and send-refused among the logged events.
@row_check n=622 row=decision/tests-event-list-adopted sha=f68c276f4a37 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:195
quote: ObsEvent::SendIssued,
at: src/run/send.rs:282
quote: ObsEvent::SendRefused,
@row_add n=623 row=contract/exit-21-detail/instance-dead area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: instance-dead
promise: instance-dead is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_check n=623 row=contract/exit-21-detail/instance-dead sha=66e7bfd6ac16 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
at: src/cmd/client.rs:119
quote: Self::Dead => UnreachableCause::InstanceDead,
@row_add n=624 row=contract/exit-21-detail/unwrapped-name area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: unwrapped-name
promise: unwrapped-name is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_check n=624 row=contract/exit-21-detail/unwrapped-name sha=300e2595772d v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:198
quote: Self::UnwrappedName => "unwrapped-name",
at: src/cmd/client.rs:120
quote: Self::Unwrapped => UnreachableCause::UnwrappedName,
@row_add n=625 row=contract/exit-21-detail/endpoint-lost area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: endpoint-lost
promise: endpoint-lost is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_check n=625 row=contract/exit-21-detail/endpoint-lost sha=5e45fabbfb38 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:199
quote: Self::EndpointLost => "endpoint-lost",
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
@row_add n=626 row=contract/exit-21-detail/strict-modes-failed area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: strict-modes-failed
promise: strict-modes-failed is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_check n=626 row=contract/exit-21-detail/strict-modes-failed sha=49bd35dfbf59 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:196
quote: Self::StrictModes => "strict-modes-failed",
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=627 row=contract/exit-21-detail/server-verify-failed area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: server-verify-failed
promise: server-verify-failed is a distinct detail code for an exit-21 cause, distinct from instance-dead, naming no path or pid.
@row_check n=627 row=contract/exit-21-detail/server-verify-failed sha=9ef6ff46e89f v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
at: src/cmd/client.rs:122
quote: Self::ServerUnverified => UnreachableCause::ServerVerify,
@row_add n=628 row=contract/exit-21-detail/socket-dir-refused area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 21 detail codes
item: socket-dir-refused
promise: socket-dir-refused is a distinct detail code for an exit-21 cause, naming no path or pid.
@row_check n=628 row=contract/exit-21-detail/socket-dir-refused sha=4fcbb4c45dc5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
at: src/cmd/client.rs:123
quote: Self::SocketDir => UnreachableCause::SocketDir,
@row_add n=629 row=contract/exit-1-detail/already-live area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: already-live
promise: already-live is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_check n=629 row=contract/exit-1-detail/already-live sha=5cf513922ba7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
@row_add n=630 row=contract/exit-1-detail/stale-heartbeat area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: stale-heartbeat
promise: stale-heartbeat is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_check n=630 row=contract/exit-1-detail/stale-heartbeat sha=d6bfe550fe9a v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=631 row=contract/exit-1-detail/squatted-name area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: squatted-name
promise: squatted-name is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_check n=631 row=contract/exit-1-detail/squatted-name sha=27fce9769d2f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=632 row=contract/exit-1-detail/pinned-hash-mismatch area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: pinned-hash-mismatch
promise: pinned-hash-mismatch is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_check n=632 row=contract/exit-1-detail/pinned-hash-mismatch sha=53891416c0b5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:284
quote: "pinned-hash-mismatch",
@row_add n=633 row=contract/exit-1-detail/socket-dir-refused area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: socket-dir-refused
promise: socket-dir-refused is a distinct detail code for an exit-1 cause, naming no path or pid.
@row_check n=633 row=contract/exit-1-detail/socket-dir-refused sha=0b0d631656eb v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
@row_add n=634 row=contract/exit-1-detail/no-session area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: no-session
promise: no-session is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_check n=634 row=contract/exit-1-detail/no-session sha=c21bb1b36940 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=635 row=contract/exit-1-detail/session-live area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: session-live
promise: session-live is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_check n=635 row=contract/exit-1-detail/session-live sha=6e4f82a6294b v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
@row_add n=636 row=contract/exit-1-detail/cwd-missing area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: Exit 1 detail codes
item: cwd-missing
promise: cwd-missing is a distinct detail code for viola revive's exit-1 cause, naming no path or pid.
@row_check n=636 row=contract/exit-1-detail/cwd-missing sha=bfce4921a72f v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=637 row=contract/viola-revive/strict-modes-failed area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: viola revive output
item: strict-modes-failed
promise: A strict-mode fail of viola revive writes no log line; it prints the stderr pair, or under --json {"v":1,"error":"strict-modes-failed","detail":null}, and exits 1.
@row_check n=637 row=contract/viola-revive/strict-modes-failed sha=aa3288650f50 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:138
quote: Refusal::StrictModes.write(&args.name, args.json);
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
at: src/cmd/revive.rs:139
quote: return Ok(Started::Refused(ExitCode::from(1)));
@row_add n=638 row=contract/viola-link-unlink/strict-modes-failed area=obs-scope-summary rec=fct-n7xftqkr@2
boundary: viola link and unlink output
item: strict-modes-failed
promise: A strict-mode fail of the driven instance's check writes no log line; it prints the fixed instance-state unable and hint: pair on stderr, or under --json {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and exits 21.
@row_check n=638 row=contract/viola-link-unlink/strict-modes-failed sha=fe5180beab88 v=holds judge=sonnet head=8834195b0683
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=639 row=decision/socket-dir-one-code area=obs-scope-summary rec=fct-n7xftqkr@2
name: One code for socket-directory causes
rule: A refused check, a failed create or a lost race on the socket directory all read the one code socket-dir-refused.
@row_check n=639 row=decision/socket-dir-one-code sha=adb0c3370543 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:511
quote: if check_endpoint_dir(endpoint).is_err() {
at: src/cmd/run.rs:518
quote: (!absent || create_private_dir(dir).is_ok()) && check_endpoint_dir(endpoint).is_ok()
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: crates/viola-core/src/lib.rs:201
quote: Self::SocketDir => "socket-dir-refused",
@row_add n=640 row=contract/refusal-reason/unknown-fallback area=obs-scope-summary rec=fct-plg35lpj@1
boundary: Refusal reason
item: unknown fallback
promise: An unrecognised RefusalReason falls back to unknown with exit 14.
@row_check n=640 row=contract/refusal-reason/unknown-fallback sha=3c52c5e0f09b v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:76
quote: serde_json::from_value(Value::from(reason)).unwrap_or(RefusalReason::Unknown)
at: src/cmd/send.rs:71
quote: RefusalReason::Unknown => 14,
@row_add n=641 row=contract/channel-response/invalid-params-causes area=obs-scope-summary rec=fct-plg35lpj@1
boundary: Channel response line
item: invalid-params causes
promise: detail invalid-params covers send's invalid params, and a link or unlink whose driver or driven is missing, not a string or no valid name, whose driven is not the serving wrapper's own name, or whose driver equals driven.
@row_check n=641 row=contract/channel-response/invalid-params-causes sha=4cb141797e3d v=holds judge=sonnet head=8834195b0683
at: src/run/send/params.rs:9
quote: .ok_or(ProtocolError::InvalidParams)?;
at: src/run/links.rs:92
quote: .ok_or(ProtocolError::InvalidParams)
at: src/run/links.rs:95-96
quote: if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
@row_add n=642 row=decision/version-rejections-counted-by-detail area=obs-scope-summary rec=fct-plg35lpj@1
name: Version rejections counted by detail
rule: Higher-v rejections are counted by the detail unsupported-version, not by the -32602 code.
why: The -32602 code also carries invalid-params and release-from-driver.
@row_check n=642 row=decision/version-rejections-counted-by-detail sha=2c11e3dd1cad v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:105
quote: Self::UnsupportedVersion => Some("unsupported-version"),
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
@row_add n=643 row=decision/deterministic-event-emission area=obs-scope-summary rec=fct-jz5zhprx@1
name: Deterministic event emission
rule: Events are emitted deterministically, keyed on hook events and events.ndjson offsets.
@row_check n=643 row=decision/deterministic-event-emission sha=b29bd805bbd6 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:277
quote: None => end_offset(self.instance_dir).ok(),
@row_add n=644 row=decision/no-telemetry-from-screen area=obs-scope-summary rec=fct-jz5zhprx@1
name: No telemetry from screen content
rule: No telemetry is derived from screen content; vt100 serves gate outcomes only.
@row_check n=644 row=decision/no-telemetry-from-screen sha=3c0cbd5579b9 v=holds judge=sonnet head=8834195b0683
searched: obs_event! · in src/run/gate.rs (only line: parse-rejected parser="vt100-feed" detail count, no screen text)
searched: obs_event! with screen contents · in src/run/*.rs
@row_add n=645 row=decision/cookie-header-never-logged area=pii-scrubbing-compliance rec=fct-cthr64ip@1
name: Cookie header never logged
rule: A Cookie header, in its case-insensitive form, is classed Critical and never logged in any diagnostics/ file.
guard: secret-scan test
@row_check n=645 row=decision/cookie-header-never-logged sha=31b5cea93d76 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:129
quote: for offset in find_all(&lower, b"cookie:") {
guard_at: crates/viola-e2e/src/harness/secret_scan.rs:255
@row_add n=646 row=decision/signing-credentials-never-logged area=pii-scrubbing-compliance rec=fct-j3d6gqtu@1
name: Signing credentials never logged
rule: v1.x signing credentials are classed Critical and never logged, and CI secrets are never echoed into artifacts.
@row_check n=646 row=decision/signing-credentials-never-logged sha=04538d738eea v=holds judge=sonnet head=8834195b0683
searched: signing|codesign|secrets\. · in src/**/*.rs, crates/**/*.rs
searched: secrets\.|signing|echo · in .github/
@row_add n=647 row=decision/prompt-and-output-never-in-home-logs area=pii-scrubbing-compliance rec=fct-a6wgb4sc@1,fct-cetbw4yv@1
name: Prompt and assistant output never in home logs
rule: Prompt text and assistant output, last_assistant_message included, never appear in a home-level process-log line; they reach an instance detail file only inside a chain or drift report, and only text_bytes is logged.
@row_check n=647 row=decision/prompt-and-output-never-in-home-logs sha=79449bea78af v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:202
quote: text_bytes = u64::try_from(text_bytes).unwrap_or(u64::MAX),
searched: last_assistant_message · in src/**/*.rs (appears only in reply payloads and parsing, in no log field)
@row_add n=648 row=decision/veil-redaction area=pii-scrubbing-compliance rec=fct-a6wgb4sc@1,fct-jv26h5pj@1
name: veil derives redaction
rule: veil 0.3.0's derived Redact is what redacts the payload fields of contract types, and instrumented spans skip all arguments.
@row_check n=648 row=decision/veil-redaction sha=f17193439c67 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:58
quote: #[redact(fixed = 8)]
at: src/cmd/send.rs:231
quote: #[instrument(skip_all, name = "send.client")]
@row_add n=649 row=decision/endpoint-paths-never-logged area=pii-scrubbing-compliance rec=fct-5jqyxpdr@1
name: Endpoint and pinned binary never logged
rule: The snapshot endpoint and pinned_bin are never logged, only endpoint_kind.
@row_check n=649 row=decision/endpoint-paths-never-logged sha=113bce7050c7 v=holds judge=sonnet head=8834195b0683
searched: \bendpoint\s*=|pinned_bin\s*= · in src/**/*.rs, crates/**/*.rs (no log field; only locals)
searched: endpoint_kind · in src/run/mod.rs:37 (the only endpoint-related log field)
@row_add n=650 row=decision/typed-ids-never-in-detail area=pii-scrubbing-compliance rec=fct-24jsq3a5@1
name: Ids logged as typed fields
rule: pid, child_pid, agent_session_id, started_at, budget percentages and resets_at are logged as typed fields, never inside detail codes.
@row_check n=650 row=decision/typed-ids-never-in-detail sha=a77a199185ae v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:36
quote: pid = std::process::id(),
at: src/run/mod.rs:55
quote: child_pid = child_pid,
@row_add n=651 row=decision/agent-rows-never-verbatim area=pii-scrubbing-compliance rec=fct-7eu65g47@1
name: Agent rows never logged verbatim
rule: claude agents --json rows are untrusted and never logged verbatim, only parse-rejected counts.
@row_check n=651 row=decision/agent-rows-never-verbatim sha=663ebb071b8c v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:82-84
quote: parser = "claude-agents-json", detail = failure.detail(), count = 1u64,
@row_add n=652 row=decision/link-names-never-in-process-log area=pii-scrubbing-compliance rec=fct-wjbkfws5@1
name: Link pair names never in a process log
rule: link and unlink driver and driven names appear only in the wrapper's link and unlink event lines in events.ndjson, never in a process-log line; the channel pair logs the method, never the params.
guard: link_verbs_log_the_channel_pair_and_no_names
@row_check n=652 row=decision/link-names-never-in-process-log sha=f6cf7ac12090 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:230
quote: #[instrument(skip_all, name = "channel.dispatch", fields(method = method, conn = peer.label()))]
guard_at: tests/cli_links.rs:468
@row_add n=653 row=decision/closed-protocol-codes area=pii-scrubbing-compliance rec=fct-exgbnztg@1
name: Closed protocol codes
rule: Protocol codes and refusal details are logged as closed kebab-case enums naming no param or value.
guard: answer_invalid_params_names_its_cause_and_no_param
@row_check n=653 row=decision/closed-protocol-codes sha=c7e41b78969e v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
guard_at: crates/viola-channel/src/server.rs:595
@row_add n=654 row=decision/fixtures-not-a-telemetry-source area=pii-scrubbing-compliance rec=fct-rp7rj7wv@1
name: Fixtures not a telemetry source
rule: Recorded fixtures are not a telemetry source, and observability adds no fixture content.
@row_check n=654 row=decision/fixtures-not-a-telemetry-source sha=77cbe570f341 v=holds judge=sonnet head=8834195b0683
searched: fixtures · in src/obs.rs
searched: fixtures · in src/**/*.rs obs_event! fields (none)
@row_add n=655 row=decision/canary-only-in-detail-files area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Canary only in detail files
rule: Every E2E and integration input is synthetic and embeds a fixed canary, which may appear in detail files but never in a home-level diagnostics file.
guard: mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line
@row_check n=655 row=decision/canary-only-in-detail-files sha=4a6a2473a467 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:132-133
quote: if role { for (class, needle) in CONTENT {
guard_at: crates/viola-e2e/tests/mcp_tools.rs:758
@row_add n=656 row=decision/detail-upload-needs-synthetic-input area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Detail upload needs synthetic input
rule: Detail files and harness captures are uploaded only while every input is synthetic; a non-synthetic input source must first drop detail files and the harness capture from the upload paths.
@row_check n=656 row=decision/detail-upload-needs-synthetic-input sha=b4ec4005e48e v=unverifiable judge=sonnet head=8834195b0683
why: a process rule about future input sources; the workflows upload on secret-scan success only and hold no synthetic-input check
@row_add n=657 row=decision/mutation-diff-never-uploaded area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Mutation diff never uploaded
rule: The mutation run's chunk.diff at the harness capture root is excluded from the harness upload, so it is never scanned and never uploaded.
why: The diff is repository source text by construction and hits the scan's Critical-class patterns.
@row_check n=657 row=decision/mutation-diff-never-uploaded sha=c8e71ed2d0ec v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:94
quote: !target/agent-run/chunk.diff
at: crates/viola-e2e/src/harness/secret_scan.rs:80
quote: capture.retain(|path| *path != mutation_diff);
@row_add n=658 row=decision/pre-push-document-path-free area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Pre-push document path-free
rule: The local pre-push gate document is never uploaded and carries no absolute path, only codes and counts.
guard: pre_push_document_carries_no_absolute_path
@row_check n=658 row=decision/pre-push-document-path-free sha=43f894392d1b v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/pre_push.rs:292-293
quote: assert!(!text.contains(HOME), "{text}"); assert!(!text.contains(&*ws.root.to_string_lossy()), "{text}");
guard_at: crates/viola-e2e/src/harness/pre_push.rs:286
@row_add n=659 row=decision/server-verify-refusal-names-nothing area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Server-verify refusal names nothing
rule: A server-verification refusal carries no pid, start time, path or endpoint in any output or log line.
guard: security_negatives_server_identity
@row_check n=659 row=decision/server-verify-refusal-names-nothing sha=b4e37e594677 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:509
quote: "server-verify-failed",
guard_at: tests/security_negatives_server_identity.rs:1
@row_add n=660 row=decision/socket-dir-refusal-names-nothing area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Socket-directory refusal names nothing
rule: A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed instance name.
guard: security_negatives_socket_dir
@row_check n=660 row=decision/socket-dir-refusal-names-nothing sha=da960f20c35f v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:513
quote: #[case::socket_dir(Unreachable::SocketDir, "socket-dir-refused", "connect", "socket-dir")]
guard_at: tests/security_negatives_socket_dir.rs:1
@row_add n=661 row=decision/error-bodies-clean area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Clean error bodies
rule: Error bodies of CLI --json, MCP results and the channel faults hold no absolute path, upstream caused-by text or serde path, and every product error writer is named.
guard: error_bodies_each_name_a_writer_and_a_witness_that_stand
@row_check n=661 row=decision/error-bodies-clean sha=8f2fe4bcd627 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_error_bodies.rs:771
quote: fn product_error_writers_are_each_named() {
at: tests/contract_error_bodies.rs:718
quote: if !read(&root.join(body.file)).contains(body.needle) {
guard_at: tests/contract_error_bodies.rs:703
@row_add n=662 row=decision/error-messages-interpolate-nothing area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Error messages interpolate nothing
rule: thiserror messages interpolate no value.
guard: thiserror_messages_interpolate_nothing
@row_check n=662 row=decision/error-messages-interpolate-nothing sha=9c72d16a9bb7 v=holds guard=true judge=sonnet head=8834195b0683
at: tests/contract_error_bodies.rs:806
quote: assert!(faults.is_empty(), "messages that interpolate: {faults:#?}");
guard_at: tests/contract_error_bodies.rs:794
@row_add n=663 row=decision/unscanned-uploads-admissible-by-content area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Unscanned uploads admissible by content
rule: An artifact uploaded without a scan must carry no host path or log content; a member that could carry one is scanned before upload or dropped.
why: Ratified.
@row_check n=663 row=decision/unscanned-uploads-admissible-by-content sha=eb8a4959d447 v=unverifiable judge=sonnet head=8834195b0683
searched: supply-chain · in .github/workflows/ci.yml
why: not a code claim
@row_add n=664 row=decision/mutants-out-never-uploaded area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Mutation outputs never uploaded
rule: mutants.out/ and the per-run archive of JUnit and outcomes files are never uploaded.
why: The outcomes file held absolute argv paths and the log folder held test output.
@row_check n=664 row=decision/mutants-out-never-uploaded sha=c6722a337861 v=holds judge=sonnet head=8834195b0683
searched: mutants\.out · in .github
searched: run-archive · in .github
searched: upload-artifact · in .github (paths: target/e2e-home diagnostics, target/agent-run, junit, secret-scan, supply-chain, fuzz/artifacts; none is mutants.out or run-archive)
@row_add n=665 row=decision/fuzz-artifacts-need-synthetic-corpus area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Fuzz artifacts need a synthetic corpus
rule: Fuzz crash artifacts upload only while the committed fuzz corpus is synthetic; a non-synthetic corpus seed must first drop that upload.
@row_check n=665 row=decision/fuzz-artifacts-need-synthetic-corpus sha=d033eca816e3 v=unverifiable judge=sonnet head=8834195b0683
searched: fuzz/artifacts/ · in .github/workflows/nightly.yml
why: no evidence found; a condition on corpus content and a process rule, the workflow only uploads on failure()
@row_add n=666 row=decision/scan-failure-blocks-upload area=pii-scrubbing-compliance rec=fct-ydc7ueix@3
name: Scan failure blocks upload
rule: When the secret scan fails no diagnostics, harness or perf artifact uploads; a hit report with file, line, byte offset and pattern class, never the matched bytes, uploads instead.
why: The failing job still leaves agent-readable evidence.
@row_check n=666 row=decision/scan-failure-blocks-upload sha=791776ef25d5 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:106
quote: if: always() && steps.secret-scan.outcome == 'failure'
at: .github/workflows/ci.yml:110
quote: path: target/secret-scan/
@row_add n=667 row=decision/no-retry-on-panic area=slo-invariants-telemetry-budgets rec=fct-6543go25@1
name: No retry on panic
rule: Retry-once policies for panic handling are forbidden.
why: Parallel to the tests' retries = 0.
@row_check n=667 row=decision/no-retry-on-panic sha=8cc512cafbae v=holds judge=sonnet head=8834195b0683
at: src/main.rs:48
quote: let outcome = std::panic::catch_unwind(|| {
at: .config/nextest.toml:2
quote: retries = 0
searched: retry|retries · in src, crates, tests, scripts, .config (only nextest `retries = 0`, curl retries in scripts/install-ripgrep.sh, no panic retry)
@row_add n=668 row=decision/session-end-perf-gate area=slo-invariants-telemetry-budgets rec=fct-h65wem67@1,fct-dijwakce@1
name: Session-end hook perf gate
rule: viola hook session-end must keep its hyperfine max under 1.0 s; p50 and p95 are report-only.
guard: gate --require perf
@row_check n=668 row=decision/session-end-perf-gate sha=e7318f009b0c v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:151
quote: Some(max) if max < bound => {}
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
guard_at: crates/viola-e2e/src/harness/gate.rs:130
@row_add n=669 row=decision/spine-hook-perf-gate area=slo-invariants-telemetry-budgets rec=fct-4od5l5b2@2,fct-dijwakce@1
name: Spine hook perf gate
rule: Spine hooks (session-start, user-prompt-submit, stop) and pre-tool-use on an unverified CLI keep their hyperfine max under the spine deadline, timed over every check a frame-writing hook makes before its first frame.
guard: gate --require perf
@row_check n=669 row=decision/spine-hook-perf-gate sha=c19e9532f5f6 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:145
quote: let bound = viola_core::SPINE_DEADLINE.as_secs_f64();
at: crates/viola-e2e/src/harness/run/perf.rs:21-27
quote: pub const ROWS: [&str; 5] = [ "session-start", "user-prompt-submit", "stop", "session-end", "pre-tool-use", ];
guard_at: crates/viola-e2e/src/harness/gate.rs:130
@row_add n=670 row=contract/hook-decision/channel-unreachable area=slo-invariants-telemetry-budgets rec=fct-4od5l5b2@2
boundary: Hook decision line
item: channel-unreachable
promise: A hook whose snapshot read is replayed or absent fails open as channel-unreachable.
@row_check n=670 row=contract/hook-decision/channel-unreachable sha=ad8d730e9cc5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:227-228
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else { return Err(NotSent::Unreachable);
@row_add n=671 row=decision/heartbeat-staleness-flip area=slo-invariants-telemetry-budgets rec=fct-bpjqjqbh@1
name: Heartbeat staleness flip
rule: An instance whose heartbeat is 4.9 s old reads live, and one whose heartbeat is 5.1 s old reads stale or gone, as reported in the liveness field of list --json.
@row_check n=671 row=decision/heartbeat-staleness-flip sha=349a549a64ba v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
at: crates/viola-state/src/liveness.rs:33
quote: _ => Liveness::Stale,
at: crates/viola-state/src/board.rs:170
quote: pub liveness: RowLiveness,
@row_add n=672 row=decision/hook-benchmark-on-passing-instance area=slo-invariants-telemetry-budgets rec=fct-bxbg45yr@1
name: Hook benchmark runs against a passing instance
rule: Every hyperfine hook command runs with VIOLA_NAME set, a home under target/e2e-home/ and an instance directory that exists and passes the strict-modes check.
why: Without a name, or with an absent or refused instance, the hook times its no-op path and the gate passes vacuously.
@row_check n=672 row=decision/hook-benchmark-on-passing-instance sha=c9583d8cbe52 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:120
quote: .env("VIOLA_NAME", INSTANCE)
at: crates/viola-e2e/src/harness/run/perf.rs:204
quote: let dir = instance_dir(home, INSTANCE);
at: crates/viola-e2e/src/harness/boot.rs:166
quote: .prefix("viola-session-")
@row_add n=673 row=decision/perf-job-runs-g2 area=slo-invariants-telemetry-budgets rec=fct-bxbg45yr@1
name: Perf job runs G2 over the benchmark home
rule: The perf job runs the G2 check over the hook benchmark home.
why: A hook panic exits 0 and hyperfine cannot see it.
@row_check n=673 row=decision/perf-job-runs-g2 sha=f12f61cf83a4 v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:151
quote: run: bash scripts/g2-zero-panics.sh --probe && bash scripts/g2-zero-panics.sh
@row_add n=674 row=decision/cli-manual-dispatch-span area=span-trace-coverage rec=fct-wac6zl42@1
name: Manual dispatch span per cli verb
rule: Each short-lived cli verb opens one manual dispatch span of its own.
why: clap 4.6.7 has no tracing integration.
@row_check n=674 row=decision/cli-manual-dispatch-span sha=c7df6142ca1e v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:231
quote: #[instrument(skip_all, name = "send.client")]
at: src/cmd/wait.rs:88
quote: #[instrument(skip_all, name = "wait.block", fields(after = after, timeout_ms = timeout_ms))]
at: src/cmd/last.rs:65
quote: #[instrument(skip_all, name = "last.client")]
at: src/cmd/answer.rs:150
quote: #[instrument(skip_all, name = "answer.client")]
at: src/cmd/pause.rs:91
quote: #[instrument(skip_all, name = "pause.client")]
at: src/cmd/release.rs:75
quote: #[instrument(skip_all, name = "release.client")]
at: src/cmd/link.rs:129
quote: #[instrument(skip_all, name = "cli.link")]
at: src/cmd/unlink.rs:19
quote: #[instrument(skip_all, name = "cli.unlink")]
@row_add n=675 row=decision/refused-check-writes-no-line area=span-trace-coverage rec=fct-wac6zl42@1,fct-espo2tl3@1
name: A refused check opens no process log
rule: A cli verb whose strict-modes, tree or instance check refuses (exit 21) opens no process log and writes no line.
@row_check n=675 row=decision/refused-check-writes-no-line sha=90000bae2d2f v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/send.rs:170
quote: let home = client::start(home, &args.name)?;
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=676 row=contract/revive-list-process-exit/no-session area=span-trace-coverage rec=fct-wac6zl42@1
boundary: viola revive --list process log
item: no-session
promise: An empty session chain ends process-exit with exit_code 1 and detail no-session; any other listing ends exit_code 0 with no detail.
@row_check n=676 row=contract/revive-list-process-exit/no-session sha=59f18faea338 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:284
quote: crate::run::log_self_exit(1, Some(Refusal::NoSession.detail()));
at: src/cmd/revive.rs:298
quote: crate::run::log_self_exit(0, None);
@row_add n=677 row=decision/revive-start-logs-as-run area=span-trace-coverage rec=fct-wac6zl42@1
name: viola revive start logs as run
rule: The start arm of viola revive is not a cli verb and logs as process run once its instance check has passed.
@row_check n=677 row=decision/revive-start-logs-as-run sha=e549b311adc1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:137
quote: if !trusted(home, &args.name)? {
at: src/cmd/revive.rs:152
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/mod.rs:140
quote: process: ObsProcess::Run,
@row_add n=678 row=decision/error-chain-in-detail-log-only area=span-trace-coverage rec=fct-wac6zl42@1
name: Full error chain only in the cli detail log
rule: The full anyhow error chain of a cli verb goes to the cli detail log only.
@row_check n=678 row=decision/error-chain-in-detail-log-only sha=d595cccba8ca v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:131
quote: client::own_name().map(|instance| DetailSink {
at: src/obs.rs:323
quote: pub(crate) fn write_detail(home: &Path, instance: &ViolaName, process: ObsProcess, line: &str) {
@row_add n=679 row=decision/portable-pty-log-target-off area=span-trace-coverage rec=fct-xrkfnxjv@2
name: portable-pty log target OFF
rule: The log target of portable-pty is OFF in the sink, and PTY failures are captured at the pty seam instead.
@row_check n=679 row=decision/portable-pty-log-target-off sha=b77f73866e39 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:110
quote: .fold(Targets::new().with_default(LevelFilter::OFF), |t, name| {
at: src/obs.rs:22
quote: const VIOLA_TARGETS: [&str; 7] = [
@row_add n=680 row=decision/manual-readiness-gate-span area=span-trace-coverage rec=fct-xrkfnxjv@2
name: Manual span on the readiness gate
rule: The readiness gate of viola run carries a manual span, run.readiness_gate.
why: vt100 has no tracing.
@row_check n=680 row=decision/manual-readiness-gate-span sha=21340875a176 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:155
quote: name = "run.readiness_gate",
@row_add n=681 row=contract/hook-spans/span-chain area=span-trace-coverage rec=fct-kreejnxq@1
boundary: viola hook spans
item: span chain
promise: viola hook traces hook.handle › channel.request, and writes the hook-decision line inside hook.handle.
@row_check n=681 row=contract/hook-spans/span-chain sha=d031de780245 v=stale judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:163
quote: #[instrument(skip_all, name = "hook.handle", fields(hook_event = event.as_str()))]
shows: hook.handle and channel.request (crates/viola-channel/src/client.rs:67) spans exist, but no hook.decision_emit span exists; the decision is written as a hook-decision line inside hook.handle (hook.rs:389-391).
cell: promise
value: viola hook traces hook.handle › channel.request, and writes the hook-decision line inside hook.handle.
@row_add n=682 row=decision/hook-always-exits-zero area=span-trace-coverage rec=fct-kreejnxq@1
name: viola hook keeps exit 0
rule: viola hook exits 0 even when its dispatch panics: a panic around dispatch is caught.
@row_check n=682 row=decision/hook-always-exits-zero sha=5705af1df558 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:48
quote: let outcome = std::panic::catch_unwind(|| {
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=683 row=decision/hook-refused-instance-silent area=span-trace-coverage rec=fct-kreejnxq@1
name: Refused instance makes the hook silent
rule: On the event and dialog paths the instance check runs before the diagnostics level and log init; a refused or unresolved instance exits 0 at once with no config read, no role file, no stdin read, no line, no frame and no SessionEnd append.
@row_check n=683 row=decision/hook-refused-instance-silent sha=8b1bee99396e v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:119
quote: let (level, _) = obs::read_diagnostics_level(&instance.home);
@row_add n=684 row=decision/hook-capture-arms-unlogged area=span-trace-coverage rec=fct-kreejnxq@1
name: Hidden capture arms write no lines
rule: The hidden capture arms of viola hook (hook <event> --capture and hook statusline --capture) initialise no observability and write no lines.
@row_check n=684 row=decision/hook-capture-arms-unlogged sha=f86777a08e47 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:95
quote: capture_statusline(dir, shell, &mut std::io::stdin().lock());
at: src/cmd/hook.rs:101
quote: if let Some(dir) = &args.capture {
at: src/cmd/hook.rs:102
quote: let filed = capture(event, dir, &mut std::io::stdin().lock());
at: src/cmd/hook.rs:106
quote: return Ok(ExitCode::SUCCESS);
@row_add n=685 row=contract/channel-lines/fields area=span-trace-coverage rec=fct-pt4swoyy@1
boundary: channel-request and channel-response lines
item: fields
promise: channel-request carries method, corr, conn and sender; channel-response carries method, corr, conn, result_class and error_code, plus detail on a -32602.
@row_check n=685 row=contract/channel-lines/fields sha=9a420d9e0137 v=stale judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:480-482
quote: result_class = result_class, error_code = fault.map(ProtocolError::code), detail = fault.and_then(ProtocolError::cause),
at: crates/viola-channel/src/client.rs:183-186
quote: corr = id, conn = conn, method = label, sender = VERSION,
shows: The channel-request line carries method, corr, conn and sender but no result_class or error_code, and the channel-response line carries result_class, error_code and detail but no sender.
cell: promise
value: channel-request carries method, corr, conn and sender; channel-response carries method, corr, conn, result_class and error_code, plus detail on a -32602.
@row_add n=686 row=decision/no-params-in-channel-lines area=span-trace-coverage rec=fct-pt4swoyy@1
name: No params or result bodies in channel lines
rule: channel-request and channel-response lines never carry params or result bodies.
@row_check n=686 row=decision/no-params-in-channel-lines sha=47c609381d19 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:367
quote: ObsEvent::ChannelRequest,
at: crates/viola-channel/src/server.rs:372
quote: sender = params.sender.as_deref().and_then(version_label),
at: crates/viola-channel/src/server.rs:375
quote: timeout_ms = timeout_ms,
at: crates/viola-channel/src/server.rs:475
quote: ObsEvent::ChannelResponse,
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
at: crates/viola-channel/src/server.rs:484
quote: duration_ms = duration_ms,
@row_add n=687 row=decision/accept-drops-foreign-euid area=span-trace-coverage rec=fct-6fdjkv6t@1
name: Unadmitted peers dropped at accept
rule: The wrapper drops an accepted stream whose peer euid is not admitted on the accept thread, before any worker thread or frame, with no span and no line; the peer reads end-of-file and the wheel does not move.
@row_check n=687 row=decision/accept-drops-foreign-euid sha=8eefa2b5f7c4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:145
quote: .filter(|stream| admitted(peer_euid_of(stream)));
at: crates/viola-channel/src/server.rs:150
quote: .spawn(move || serve_conn(&stream, &format!("srv-{n}"), dispatch.as_ref()));
@row_add n=688 row=decision/rmcp-tracing-target-off area=span-trace-coverage rec=fct-felgh2fq@1
name: rmcp tracing target OFF
rule: The native tracing target of rmcp is OFF in the sink.
why: Its records lack required fields and can carry frame content.
@row_check n=688 row=decision/rmcp-tracing-target-off sha=de7aa8622c91 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:110
quote: .fold(Targets::new().with_default(LevelFilter::OFF), |t, name| {
at: src/obs.rs:21
quote: /// Every other target (rmcp, notify, …) is OFF: its records lack the required fields.
@row_add n=689 row=contract/mcp-spans/mcp-tool-call area=span-trace-coverage rec=fct-felgh2fq@1
boundary: viola mcp spans
item: mcp.tool_call
promise: viola mcp opens one mcp.tool_call span per tool (send, wait, last, answer, list); rejects log codes-only send-refused or channel-response lines.
@row_check n=689 row=contract/mcp-spans/mcp-tool-call sha=ff07e3c6c5c1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:248
quote: #[instrument(skip_all, name = "mcp.tool_call", fields(tool = tool.as_str()))]
at: crates/viola-mcp/src/tools.rs:251-255
quote: ToolName::Send => send(ctx, &args).await, ToolName::Wait => wait(ctx, &args).await, ToolName::Last => last(ctx, &args).await, ToolName::Answer => answer(ctx, &args).await, ToolName::List => list(ctx).await,
at: crates/viola-mcp/src/tools.rs:306-311
quote: obs_event!( INFO, ObsEvent::SendRefused, side = "client", refusal = RefusalReason::NotDelivered.as_str(), detail = detail.as_str(),
@row_add n=690 row=decision/verify-server-before-frame area=span-trace-coverage rec=fct-2tnvi4o3@2
name: Server verified before any frame
rule: Each frame verb, after its own client-side refusals and before any frame, checks the socket directory, connects, decides on the peer euid, then compares the peer's pid and OS start time against the one snapshot read.
@row_check n=690 row=decision/verify-server-before-frame sha=d9a61e808767 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/client.rs:176
quote: let client = Client::connect(&live.endpoint, "cli")?;
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/client.rs:180
quote: let verdict = verify_server(live.pid, &live.started_at, client.server_pid());
@row_add n=691 row=contract/cli-exit-21/server-verify-failed area=span-trace-coverage rec=fct-2tnvi4o3@2,fct-u2zbdxyd@2
boundary: cli exit 21 process-exit
item: server-verify-failed
promise: A refused euid, a server that is not the recorded wrapper, or a check that cannot be made opens no channel.request span, writes no channel-request line and ends process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}.
@row_check n=691 row=contract/cli-exit-21/server-verify-failed sha=c047f8968aae v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177-178
quote: if !peer_admitted(client.server_euid()) {         return Err(CallError::ServerUnverified);
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/client.rs:258
quote: ObsEvent::ProcessExit,
at: src/cmd/client.rs:260
quote: exit_code = 21u8,
@row_add n=692 row=decision/peer-euid-must-match area=span-trace-coverage rec=fct-jvpve25b@2
name: Server euid must match the client's
rule: Every client seam refuses a server of another euid, or one whose euid cannot be read, after the connect and before the pid comparison; reading the euid writes and logs nothing.
@row_check n=692 row=decision/peer-euid-must-match sha=4aed3a96d7f4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:177
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
at: crates/viola-mcp/src/tools.rs:352
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {
at: crates/viola-channel/src/client.rs:140
quote: observed == Some(own)
@row_add n=693 row=decision/server-refusal-inside-root-span area=span-trace-coverage rec=fct-7w75bmha@1
name: Server-verification refusal inside the root span
rule: A frame verb's or the hook's server-verification refusal happens inside its scenario's root span.
guard: request_refuses_an_unverified_server_inside_its_dispatch_span; hook_refuses_an_unverified_server_inside_hook_handle
@row_check n=693 row=decision/server-refusal-inside-root-span sha=281b4a69ab3f v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:183
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
at: src/cmd/hook.rs:242
quote: ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(NotSent::ServerUnverified),
at: src/cmd/hook.rs:262
quote: #[instrument(skip_all, name = "hook.handle", fields(hook_event = event.as_str()))]
guard_at: src/cmd/send.rs:253
@row_add n=694 row=decision/socket-dir-refusal-inside-root-span area=span-trace-coverage rec=fct-vivmhmma@2
name: Socket-directory refusal inside the root span
rule: A frame verb's or the hook's socket-directory refusal happens inside its dispatch span or hook span.
guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
@row_check n=694 row=decision/socket-dir-refusal-inside-root-span sha=95c60ee5ade5 v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173
quote: if check_endpoint_dir(&live.endpoint).is_err() {
at: src/cmd/hook.rs:231
quote: if check_endpoint_dir(endpoint).is_err() {
guard_at: src/cmd/client.rs:560
@row_add n=695 row=decision/instance-check-before-role-file area=span-trace-coverage rec=fct-espo2tl3@1
name: Instance check before any role file
rule: The frame verbs run the instance check on their named instance (the driven instance for link and unlink) before any role file opens.
@row_check n=695 row=decision/instance-check-before-role-file sha=d3bcb200564d v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:167
quote: if let Some(code) = client::refuse_untrusted(home, &args.name, args.json)? {
at: src/cmd/link.rs:94
quote: if let Some(code) = client::refuse_untrusted(home, &args.driven, args.json)? {
at: src/cmd/unlink.rs:15
quote: let code = link_call(home, args, Verb::Unlink, request)?;
@row_add n=696 row=contract/instance-unreachable-json/strict-modes-failed area=span-trace-coverage rec=fct-espo2tl3@1,fct-hluu43pb@2
boundary: cli --json refusal document
item: strict-modes-failed
promise: A refused instance check exits 21 with the instance pair on stderr, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, and sends no frame.
@row_check n=696 row=contract/instance-unreachable-json/strict-modes-failed sha=a16dda27ffcd v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
@row_add n=697 row=contract/cli-exit-21/socket-dir-refused area=span-trace-coverage rec=fct-zklcva6d@1
boundary: cli exit 21 process-exit
item: socket-dir-refused
promise: A frame verb meeting a refused socket directory writes one process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"} at WARN inside its dispatch span, with no channel-request line and no frame.
@row_check n=697 row=contract/cli-exit-21/socket-dir-refused sha=0a96f28af517 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:173-174
quote: if check_endpoint_dir(&live.endpoint).is_err() {         return Err(CallError::SocketDirRefused);
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=698 row=decision/socket-dir-check-changes-no-mode area=span-trace-coverage rec=fct-zklcva6d@1
name: Socket-directory check repairs nothing
rule: The socket-directory check on the recorded endpoint's parent changes no mode: a refused directory stays as found.
@row_check n=698 row=decision/socket-dir-check-changes-no-mode sha=6e7d4395774b v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/strict.rs:139
quote: let meta = match std::fs::symlink_metadata(dir) {
at: crates/viola-state/src/strict.rs:154-157
quote: pub fn check_endpoint_dir(endpoint: &str) -> Result<(), Refused> { Path::new(endpoint) .parent() .map_or(Ok(()), check_socket_dir)
@row_add n=699 row=contract/refusal-stderr-pair/socket-dir-refused area=span-trace-coverage rec=fct-ucfyu5c3@1
boundary: cli refusal stderr pair
item: socket-directory refusal
promise: Printed on start and frame-verb refusals in plain ASCII with hint: last and no SGR, naming only the parsed ViolaName.
@row_check n=699 row=contract/refusal-stderr-pair/socket-dir-refused sha=adcdc771b238 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:737-740
quote: human::refuse(         &format!("the socket directory for {name} cannot be trusted"),         &human::send_hint(name, "socket-dir").unwrap_or_default(),     );
at: src/human/hints.rs:59-60
quote: "socket-dir" => "viola's socket directory must be a directory you own, readable only by \                          you; viola does not change its permissions"
@row_add n=700 row=decision/replayed-snapshot-unreachable area=span-trace-coverage rec=fct-kohas2np@2
name: Replayed snapshot is unreachable
rule: Only a snapshot read whole leads to a connect; a replayed or absent snapshot is treated as unreachable and no frame is sent.
why: A replayed snapshot carries no endpoint, pid or started_at.
@row_check n=700 row=decision/replayed-snapshot-unreachable sha=6292a68aa31a v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
at: src/cmd/hook.rs:228
quote: return Err(NotSent::Unreachable);
at: src/cmd/hook.rs:230
quote: let endpoint = snapshot.endpoint.as_deref().ok_or(NotSent::Unreachable)?;
@row_add n=701 row=contract/mcp-instance-unreachable/strict-modes-failed area=span-trace-coverage rec=fct-ts6ky5ry@2
boundary: viola mcp frame tool refusal
item: strict-modes-failed
promise: A refused instance check, run before the live endpoint is read, returns instance-unreachable with detail strict-modes-failed, no frame and no line.
@row_check n=701 row=contract/mcp-instance-unreachable/strict-modes-failed sha=75ce472e7fd5 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:339-342
quote: if check_instance(&ctx.home, &instance_dir).is_err() {         return Ok(result::unreachable(UnreachableCause::StrictModes));     }     let Some(live) = live_endpoint(&instance_dir) else {
@row_add n=702 row=contract/mcp-instance-unreachable/socket-dir-refused area=span-trace-coverage rec=fct-ts6ky5ry@2
boundary: viola mcp frame tool refusal
item: socket-dir-refused
promise: A refused socket directory returns instance-unreachable with detail socket-dir-refused, no frame and no line.
@row_check n=702 row=contract/mcp-instance-unreachable/socket-dir-refused sha=729b2cd153ad v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:345-346
quote: if check_endpoint_dir(&live.endpoint).is_err() {         return Ok(result::unreachable(UnreachableCause::SocketDir));
@row_add n=703 row=contract/mcp-instance-unreachable/server-verify-failed area=span-trace-coverage rec=fct-ts6ky5ry@2
boundary: viola mcp frame tool refusal
item: server-verify-failed
promise: A refused euid, a mismatch or a check that cannot be made returns the codes-only isError instance-unreachable with detail server-verify-failed, no frame and no log line.
@row_check n=703 row=contract/mcp-instance-unreachable/server-verify-failed sha=d65a83140f88 v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:352-353
quote: if !peer_admitted(client.server_euid()) || !server_verified(live, client.server_pid()).await? {         return Ok(result::unreachable(UnreachableCause::ServerVerify));
@row_add n=704 row=contract/run-start-spans/span-chain area=span-trace-coverage rec=fct-e6vtcesf@2
boundary: viola run start spans
item: span chain
promise: run.start (INTERNAL, root) › state.snapshot_recover › run.collision_check › run.pin_copy › run.version_gate › channel.bind › run.budget_eval (trigger start) › state.snapshot_write › state.heartbeat_start › pty.spawn (CLIENT).
@row_check n=704 row=contract/run-start-spans/span-chain sha=85510d9a45d0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:1644
quote: "run.start",
at: src/cmd/run.rs:1649
quote: "channel.bind",
at: src/cmd/run.rs:1653
quote: "pty.spawn",
@row_add n=705 row=decision/statusline-settings-steps-unlogged area=span-trace-coverage rec=fct-e6vtcesf@2
name: Statusline read and settings write unlogged
rule: The start's statusline source read and settings override write carry no span and no log line.
why: Ratified as by design.
@row_check n=705 row=decision/statusline-settings-steps-unlogged sha=98de79adff7b v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:549
quote: fn statusline_command(home: &Path, user_home: Option<&Path>) -> Option<String> {
at: src/cmd/run.rs:550
quote: let source = statusline_source(home, user_home)?;
at: src/cmd/run.rs:557
quote: statusline::user_command(&bytes)
at: src/cmd/run.rs:568
quote: replace_private(&path, document.as_bytes(), FILE_MODE)?;
@row_add n=706 row=decision/no-statusline-settings-in-telemetry area=span-trace-coverage rec=fct-e6vtcesf@2,fct-6peizmat@1
name: No settings or statusline command in telemetry
rule: The child's --settings argument, the statusline source path and the statusline command appear in no log line or span.
@row_check n=706 row=decision/no-statusline-settings-in-telemetry sha=9e948e1af1db v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:259
quote: #[instrument(skip_all, name = "run.start", fields(instance = name_str(&launch.name)))]
at: crates/viola-pty/src/lib.rs:128
quote: fields(pty_backend = pty_backend(), env_stripped_count = spec.env_remove.len())
searched: obs_event!([^;]{0,400}(driver|driven) · in src/**/*.rs crates/*/src/**/*.rs (no start line carries settings or the statusline command)
@row_add n=707 row=contract/run-start-failure-detail/socket-dir-refused area=span-trace-coverage rec=fct-e6vtcesf@2,fct-4g2at3oh@2
boundary: run start process-exit
item: socket-dir-refused
promise: A socket directory the start refuses is one process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"} before any snapshot, child or bind, on viola revive's start too; a bind failure keeps its own outcome.
@row_check n=707 row=contract/run-start-failure-detail/socket-dir-refused sha=6b765b327c0e v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:497
quote: if !socket_dir_ready(&endpoint) {
at: src/cmd/run.rs:412
quote: run::log_self_exit(1, Some(detail));
at: src/cmd/run.rs:500
quote: match Server::bind(&endpoint) {
@row_add n=708 row=contract/run-span-attributes/run-start area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: run.start
promise: run.start carries instance.
@row_check n=708 row=contract/run-span-attributes/run-start sha=3493dc7cb379 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:259
quote: #[instrument(skip_all, name = "run.start", fields(instance = name_str(&launch.name)))]
@row_add n=709 row=contract/run-span-attributes/run-collision-check area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: run.collision_check
promise: outcome is free or already-live; a live or stale holder is already-live, a gone one free; over a replayed snapshot a beat no older than STALE_AFTER is already-live, an older one free; an absent snapshot or failed read is free.
@row_check n=709 row=contract/run-span-attributes/run-collision-check sha=7347fe0211fb v=stale judge=sonnet head=8834195b0683
at: src/cmd/run.rs:447-450
quote: let outcome = match taken { None | Some(Liveness::Gone) => "free", Some(Liveness::Live | Liveness::Stale) => "already-live", };
shows: The collision_check span records only free or already-live; squatted-name is never recorded on it (it arises at the bind, as a process-exit detail only).
cell: promise
value: outcome is free or already-live; a live or stale holder is already-live, a gone one free; over a replayed snapshot a beat no older than STALE_AFTER is already-live, an older one free; an absent snapshot or failed read is free.
@row_add n=710 row=contract/run-span-attributes/run-pin-copy area=span-trace-coverage rec=fct-gdmzm6bw@1,fct-i4d4tneg@1
boundary: viola run start span attributes
item: run.pin_copy
promise: outcome is ok or pinned-hash-mismatch, and the set stays those two.
@row_check n=710 row=contract/run-span-attributes/run-pin-copy sha=0c3db5ed5622 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:468
quote: tracing::Span::current().record("outcome", "pinned-hash-mismatch");
at: src/cmd/run.rs:473
quote: tracing::Span::current().record("outcome", "ok");
@row_add n=711 row=contract/run-span-attributes/run-version-gate area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: run.version_gate
promise: run.version_gate carries cli_version and cli_verified.
@row_check n=711 row=contract/run-span-attributes/run-version-gate sha=53233d915411 v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:186
quote: span.record("cli_version", version.as_str());
at: src/run/version_gate.rs:188
quote: span.record("cli_verified", cli_verified);
@row_add n=712 row=contract/run-span-attributes/channel-bind area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: channel.bind
promise: channel.bind carries endpoint_kind, unix-socket.
@row_check n=712 row=contract/run-span-attributes/channel-bind sha=161046455578 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:61
quote: #[instrument(skip_all, name = "channel.bind", fields(endpoint_kind = ENDPOINT_KIND))]
at: crates/viola-channel/src/endpoint.rs:12
quote: pub const ENDPOINT_KIND: &str = "unix-socket";
@row_add n=713 row=contract/run-span-attributes/state-snapshot-write area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: state.snapshot_write
promise: state.snapshot_write carries v.
@row_check n=713 row=contract/run-span-attributes/state-snapshot-write sha=538c22dd0e9a v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/snapshot.rs:90
quote: #[tracing::instrument(skip_all, name = "state.snapshot_write", fields(v = SNAPSHOT_V))]
@row_add n=714 row=contract/run-span-attributes/pty-spawn area=span-trace-coverage rec=fct-gdmzm6bw@1
boundary: viola run start span attributes
item: pty.spawn
promise: pty.spawn carries pty_backend (openpty) and env_stripped_count.
@row_check n=714 row=contract/run-span-attributes/pty-spawn sha=e9f70e190e17 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/lib.rs:125-129
quote: #[tracing::instrument( skip_all, name = "pty.spawn", fields(pty_backend = pty_backend(), env_stripped_count = spec.env_remove.len()) )]
@row_add n=715 row=decision/no-paths-in-run-spans area=span-trace-coverage rec=fct-gdmzm6bw@1
name: No endpoint, path or env values on start spans
rule: Start spans never carry the endpoint name, the pinned binary path or environment values.
@row_check n=715 row=decision/no-paths-in-run-spans sha=a8cecf01b703 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:431
quote: #[instrument(skip_all, name = "run.collision_check", fields(outcome = tracing::field::Empty))]
at: src/cmd/run.rs:463
quote: #[instrument(skip_all, name = "run.pin_copy", fields(outcome = tracing::field::Empty))]
at: crates/viola-channel/src/server.rs:61
quote: #[instrument(skip_all, name = "channel.bind", fields(endpoint_kind = ENDPOINT_KIND))]
at: crates/viola-pty/src/lib.rs:128
quote: fields(pty_backend = pty_backend(), env_stripped_count = spec.env_remove.len())
@row_add n=716 row=contract/run-process-log/common-fields area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run process log
item: common fields
promise: Every run line carries the common fields with process="run" and corr null.
@row_check n=716 row=contract/run-process-log/common-fields sha=ecdabba405e6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:225
quote: obs::viola_obs_init(home, ObsProcess::Run, Some(name.clone()), level)?;
at: crates/viola-core/src/obs.rs:146
quote: process = $crate::obs::ctx_process(),
@row_add n=717 row=contract/run-process-log/process-start-self area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run process log
item: process-start self
promise: process-start{subject:"self", service_name, version, os, pid, endpoint_kind}.
@row_check n=717 row=contract/run-process-log/process-start-self sha=f3443018c39a v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:32-37
quote: subject = "self", service_name = SERVICE_NAME, version = VERSION, os = std::env::consts::OS, pid = std::process::id(), endpoint_kind = viola_channel::ENDPOINT_KIND,
@row_add n=718 row=contract/run-process-log/version-probe area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run process log
item: version-probe
promise: process-start and process-exit{subject:"version-probe", child_exit_status, duration_ms}.
@row_check n=718 row=contract/run-process-log/version-probe sha=e3a3e7a7cce3 v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:149
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "version-probe");
at: src/run/version_gate.rs:166-168
quote: subject = "version-probe", child_exit_status = status, duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
@row_add n=719 row=contract/run-start-failure-detail/already-live area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run start process-exit
item: already-live
promise: process-exit{subject:"self", exit_code:1, detail:"already-live"} for a live holder or a fresh replayed beat, with no path or pid.
@row_check n=719 row=contract/run-start-failure-detail/already-live sha=8ab7d0b017ea v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:454
quote: Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
at: src/cmd/run.rs:442-444
quote: Some(Recovered::Replayed { .. }) => age() .is_some_and(|age| age <= STALE_AFTER) .then_some(Liveness::Live),
@row_add n=720 row=contract/run-start-failure-detail/stale-heartbeat area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run start process-exit
item: stale-heartbeat
promise: process-exit{subject:"self", exit_code:1, detail:"stale-heartbeat"} for a stale holder, with no path or pid.
@row_check n=720 row=contract/run-start-failure-detail/stale-heartbeat sha=23b285bda37f v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:455
quote: Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
@row_add n=721 row=contract/run-start-failure-detail/squatted-name area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run start process-exit
item: squatted-name
promise: process-exit{subject:"self", exit_code:1, detail:"squatted-name"}, with no path or pid.
@row_check n=721 row=contract/run-start-failure-detail/squatted-name sha=794a27e73d63 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:295
quote: return Ok(refused_with(launch.json, "squatted-name", || {
@row_add n=722 row=contract/run-start-failure-detail/pinned-hash-mismatch area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run start process-exit
item: pinned-hash-mismatch
promise: process-exit{subject:"self", exit_code:1, detail:"pinned-hash-mismatch"}, with no path or pid.
@row_check n=722 row=contract/run-start-failure-detail/pinned-hash-mismatch sha=76fb8f79722b v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:282-285
quote: return Ok(refused_with( launch.json, "pinned-hash-mismatch", refuse_tampered_pin,
@row_add n=723 row=decision/home-tree-check-before-start-log area=span-trace-coverage rec=fct-4g2at3oh@2
name: Home-tree check before the start's log
rule: The start of run and of viola revive checks the home tree (the home, instance set, ledger, bin, plugin, pinned exe and plugin hooks) before its log opens; a refusal exits 1 with the home or instance pair, opens no log, writes no line and starts no child.
@row_check n=723 row=decision/home-tree-check-before-start-log sha=9498d1fce07c v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/revive.rs:141
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/revive.rs:152
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: crates/viola-state/src/strict.rs:84
quote: pub fn check_home_tree(
at: crates/viola-state/src/strict.rs:89
quote: check_existing(&[(home.to_path_buf(), Rule::Read)]).map_err(TreeRefused::HomeLevel)?;
at: crates/viola-state/src/strict.rs:97
quote: (home.join("plugin"), Rule::Write),
at: crates/viola-state/src/strict.rs:107
quote: (plugin.join(".claude-plugin"), Rule::Write),
@row_add n=724 row=contract/revive-preflight-detail/no-session area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: viola revive preflight process-exit
item: no-session
promise: A preflight refusal adds detail no-session before any child is started.
@row_check n=724 row=contract/revive-preflight-detail/no-session sha=459c865c82fa v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:71
quote: Self::NoSession | Self::UnknownId => "no-session",
@row_add n=725 row=contract/revive-preflight-detail/session-live area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: viola revive preflight process-exit
item: session-live
promise: A preflight refusal adds detail session-live before any child is started; the reading is skipped under --fork and passes on a failed read.
@row_check n=725 row=contract/revive-preflight-detail/session-live sha=b8455041aa05 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:72
quote: Self::SessionLive => "session-live",
at: src/cmd/revive.rs:223
quote: |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
at: src/cmd/revive.rs:193
quote: .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
@row_add n=726 row=contract/revive-preflight-detail/cwd-missing area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: viola revive preflight process-exit
item: cwd-missing
promise: A preflight refusal adds detail cwd-missing before any child is started.
@row_check n=726 row=contract/revive-preflight-detail/cwd-missing sha=42ad858f9bb6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/revive.rs:73
quote: Self::CwdMissing => "cwd-missing",
@row_add n=727 row=contract/revive-process-log/agents-probe area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: viola revive run process log
item: agents-probe
promise: The session-live reading writes one process-start and process-exit{subject:"agents-probe"} pair, plus one parse-rejected{parser:"claude-agents-json"} on an oversize or malformed answer.
@row_check n=727 row=contract/revive-process-log/agents-probe sha=af8c70a2bbb6 v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/run/agents.rs:64-65
quote: ObsEvent::ProcessExit, subject = "agents-probe",
at: src/run/agents.rs:82
quote: parser = "claude-agents-json",
at: src/cmd/revive.rs:192
quote: crate::run::agents::read_here(home, cwd)
@row_add n=728 row=contract/run-process-log/process-start-claude-child area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: run process log
item: process-start claude-child
promise: On success, process-start{subject:"claude-child", child_pid, pty_backend, cli_version, cli_verified, env_stripped_count, env_stripped_known, env_kept}.
@row_check n=728 row=contract/run-process-log/process-start-claude-child sha=72618fc9d440 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:54-61
quote: subject = "claude-child", child_pid = child_pid, pty_backend = viola_pty::pty_backend(), env_stripped_count = strip.count(), env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()), env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()), cli_version = cli_version, cli_verified = cli_verified,
@row_add n=729 row=contract/diag-line-schema/pty-backend area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: diag-line v1 schema
item: pty_backend
promise: pty_backend is any string.
@row_check n=729 row=contract/diag-line-schema/pty-backend sha=4ec6563c6832 v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:69
quote: "pty_backend": { "type": "string" },
@row_add n=730 row=contract/diag-line-schema/os area=span-trace-coverage rec=fct-4g2at3oh@2
boundary: diag-line v1 schema
item: os
promise: os is closed to linux.
@row_check n=730 row=contract/diag-line-schema/os sha=d2fd875daab0 v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:67
quote: "os": { "enum": ["linux"] }, "pid": { "$ref": "#/$defs/count" },
@row_add n=731 row=decision/product-order-from-events-log area=span-trace-coverage rec=fct-4g2at3oh@2
name: Start product order read from the events log
rule: The product order wheel{cause:"start"} then budget-gate then session-start{source:"hook"} is verified from the instance's events log, not duplicated into process logs.
@row_check n=731 row=decision/product-order-from-events-log sha=c56b45b6fa43 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/boot.rs:406-409
quote: /// `events.ndjson`'s first three complete lines are `wheel{cause:"start"}`, `budget-gate`, then /// the hook's `session-start` (test-plan §3 Readiness signal). pub fn start_records(instance_dir: &Path) -> bool { let text = fs::read_to_string(instance_dir.join("events.ndjson")).unwrap_or_default();
@row_add n=732 row=contract/run-start-spans/run-start-close area=span-trace-coverage rec=fct-mw7zqeom@2
boundary: viola run start spans
item: run.start close
promise: run.start closes when pty.spawn returns.
@row_check n=732 row=contract/run-start-spans/run-start-close sha=7d310ce98708 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:259
quote: #[instrument(skip_all, name = "run.start", fields(instance = name_str(&launch.name)))]
at: src/cmd/run.rs:390
quote: let spawned = spawn_child(
@row_add n=733 row=contract/run-process-log/process-exit-self area=span-trace-coverage rec=fct-mw7zqeom@2
boundary: run process log
item: process-exit self
promise: After the child's process-exit, run emits process-exit{subject:"self", exit_code, duration_ms}.
@row_check n=733 row=contract/run-process-log/process-exit-self sha=5abbb6dce436 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:702-703
quote: run::log_child_exit(exit); run::log_self_exit(0, None);
at: src/run/mod.rs:81-84
quote: subject = "self", exit_code = exit_code, detail = detail, duration_ms = duration_ms,
@row_add n=734 row=decision/pinned-copy-never-replaced area=span-trace-coverage rec=fct-i4d4tneg@1
name: Pinned copy placed, never replaced
rule: An absent pinned copy is placed, never replaced; a start that loses the placement re-hashes the copy that stands, reusing an equal one and refusing a different one as pinned-hash-mismatch with the file left as found.
@row_check n=734 row=decision/pinned-copy-never-replaced sha=b01054ca0798 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/pin.rs:105-108
quote: match place_private_new(path, bytes, DIR_MODE)? { Placed::Written => Ok(()), Placed::Present if file_key(path)? == hash => Ok(()), Placed::Present => Err(PinError::HashMismatch),
@row_add n=735 row=contract/confirmed-send-spans/span-chain area=span-trace-coverage rec=fct-lohdixpq@2
boundary: confirmed send spans
item: span chain
promise: send.client (CLIENT) › channel.request (CLIENT) → channel.dispatch (SERVER) › run.readiness_gate › pty.paste_write › run.confirm_window; an unconfirmable send ends at pty.paste_write. In the hook: hook.handle › channel.request(hook.event).
@row_check n=735 row=contract/confirmed-send-spans/span-chain sha=6d0c60c68a17 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:231
quote: #[instrument(skip_all, name = "send.client")]
at: crates/viola-channel/src/client.rs:67
quote: #[instrument(skip_all, name = "channel.request", fields(method = method, conn = self.conn.as_str()))]
at: crates/viola-channel/src/server.rs:230
quote: #[instrument(skip_all, name = "channel.dispatch", fields(method = method, conn = peer.label()))]
at: src/run/gate.rs:155
quote: name = "run.readiness_gate",
at: crates/viola-pty/src/pump.rs:74
quote: name = "pty.paste_write",
at: src/run/send/slot.rs:147
quote: #[instrument(skip_all, name = "run.confirm_window", fields(window_ms = window_ms(self.window())))]
at: src/cmd/hook.rs:163
quote: #[instrument(skip_all, name = "hook.handle", fields(hook_event = event.as_str()))]
@row_add n=736 row=decision/implicit-link-on-send-and-answer area=span-trace-coverage rec=fct-lohdixpq@2,fct-46kjozmt@1
name: Implicit link recorded by send and answer
rule: An issued send or a taken answer whose from names an instance other than the wrapper records the link, one link product line for a new pair; a refused call, one with no from, or one under the wrapper's own name records nothing.
@row_check n=736 row=decision/implicit-link-on-send-and-answer sha=650ba1d7bded v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:142-145
quote: if let (Some(links), Some(from)) = (slot.links.get(), ctx.from.as_ref()) && from != name { links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/dialog.rs:452-455
quote: if let (Some(links), Some(from)) = (self.links.get(), from.as_ref()) && *from != self.name { links.record(from).map_err(|_| ProtocolError::Internal)?;
@row_add n=737 row=contract/events-log/link-before-send-issued area=span-trace-coverage rec=fct-lohdixpq@2
boundary: events log
item: implicit link and send-issued
promise: A new pair's link product line stands directly before that send's send-issued, whose cursor is the offset its own line starts at; a link record that fails answers -32603 with nothing issued, nothing typed and no send-issued.
@row_check n=737 row=contract/events-log/link-before-send-issued sha=f09bdde9af57 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:145
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/send.rs:147
quote: let cursor = ctx.issue(text.len())?;
@row_add n=738 row=decision/link-names-not-in-process-log area=span-trace-coverage rec=fct-lohdixpq@2,fct-lnbq3lnm@1
name: Link names stay out of process logs
rule: The driver and driven names reach no process-log line.
@row_check n=738 row=decision/link-names-not-in-process-log sha=69ed2bb7c036 v=holds judge=sonnet head=8834195b0683
searched: obs_event!\([^;]{0,400}(driver|driven) · in src/**/*.rs crates/*/src/**/*.rs
searched: obs_event! · in src/run/links.rs
searched: driver|driven · in src/obs.rs crates/viola-core/src/obs.rs schemas/diag-line.v1.json
@row_add n=739 row=contract/no-endpoint-instance/classification area=span-trace-coverage rec=fct-lohdixpq@2
boundary: no-endpoint instance reading
item: instance-dead or unwrapped-name
promise: An instance with no endpoint whose path is a directory reads instance-dead; anything else (absent, a regular file, a symlink) reads unwrapped-name; the reading writes no line and the instance check stays first.
@row_check n=739 row=contract/no-endpoint-instance/classification sha=eb7ea37cc4e8 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:111-113
quote: match std::fs::symlink_metadata(instance_dir) { Ok(metadata) if metadata.is_dir() => UnreachableCause::InstanceDead, _ => UnreachableCause::UnwrappedName,
@row_add n=740 row=contract/send-span-attributes/run-readiness-gate area=span-trace-coverage rec=fct-xfymn57j@1
boundary: confirmed send span attributes
item: run.readiness_gate
promise: outcome (ready or input-not-ready) and vt100_panicked (bool).
@row_check n=740 row=contract/send-span-attributes/run-readiness-gate sha=7b4c67bead05 v=holds judge=sonnet head=8834195b0683
at: src/run/gate.rs:174
quote: span.record("outcome", readiness.as_str());
at: src/run/gate.rs:175
quote: span.record("vt100_panicked", panicked);
@row_add n=741 row=contract/send-span-attributes/text-bytes area=span-trace-coverage rec=fct-xfymn57j@1,fct-gghzntp4@2
boundary: confirmed send span attributes
item: text_bytes
promise: text_bytes, on pty.paste_write and send-issued alike, is the typed text's length: the sent text with each CR LF pair and each other CR as one LF, trailing CR and LF removed; pty.paste_write also carries paste_mode="bracketed".
@row_check n=741 row=contract/send-span-attributes/text-bytes sha=4c1b5f6d9360 v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:75
quote: fields(text_bytes = text.len(), paste_mode = "bracketed")
at: src/run/send.rs:202
quote: text_bytes = u64::try_from(text_bytes).unwrap_or(u64::MAX),
at: crates/viola-agent-claude/src/hook.rs:232-234
quote: let text = text.trim_end_matches(['\r', '\n']); if text.contains('\r') { Cow::Owned(text.replace("\r\n", "\n").replace('\r', "\n"))
@row_add n=742 row=contract/send-span-attributes/window-ms area=span-trace-coverage rec=fct-xfymn57j@1
boundary: confirmed send span attributes
item: run.confirm_window window_ms
promise: window_ms is the bound the send's own deadline reads: 40 000 for a /compact send waiting for its compacted-session post-condition, else 10 000.
@row_check n=742 row=contract/send-span-attributes/window-ms sha=fdcf9a2c5f90 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:177-178
quote: Some(Confirms::Post(PostCondition::CompactedSession)) => COMPACT_CONFIRM_WINDOW, _ => CONFIRM_WINDOW_FALLBACK,
@row_add n=743 row=decision/no-text-in-telemetry area=span-trace-coverage rec=fct-xfymn57j@1,fct-gghzntp4@2
name: Sent text never in telemetry
rule: The sent text never appears in a span or log line; only its length does.
@row_check n=743 row=decision/no-text-in-telemetry sha=d20a81419ef1 v=holds judge=sonnet head=8834195b0683
searched: obs_event!\([^;]{0,500}(text\b|\btext:|last_assistant_message) · in src/**/*.rs crates/*/src/**/*.rs (only the doc example in crates/viola-core/src/obs.rs matched)
searched: fields\([^)]*\btext\b · in src/**/*.rs crates/*/src/**/*.rs (only text_bytes at crates/viola-pty/src/pump.rs:75 and src/run/send.rs:202)
@row_add n=744 row=contract/send-log-fields/channel-request area=span-trace-coverage rec=fct-gghzntp4@2
boundary: confirmed send log fields
item: channel-request
promise: channel-request{corr:<id>, conn, method, sender}.
@row_check n=744 row=contract/send-log-fields/channel-request sha=6a14648fd486 v=stale judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:367-375
quote: ObsEvent::ChannelRequest, corr = corr, conn = peer.conn(), srv_conn = peer.srv_conn(), method = method, sender = params.sender.as_deref().and_then(version_label), v = params.v, after = after, timeout_ms = timeout_ms,
shows: The channel-request line carries corr, conn, srv_conn, method, sender, v, after and timeout_ms but no from and no from_trust (from_trust is emitted only on wrapper send-*, dialog and wheel lines).
cell: promise
value: channel-request{corr:<id>, conn, method, sender}.
@row_add n=745 row=contract/send-log-fields/send-issued area=span-trace-coverage rec=fct-gghzntp4@2
boundary: confirmed send log fields
item: send-issued
promise: send-issued{corr:<cursor>, from, text_bytes}.
@row_check n=745 row=contract/send-log-fields/send-issued sha=d84a63c3b299 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:196
quote: corr = cursor,
at: src/run/send.rs:200
quote: from = from,
at: src/run/send.rs:202
quote: text_bytes = u64::try_from(text_bytes).unwrap_or(u64::MAX),
@row_add n=746 row=contract/send-log-fields/hook-invoked area=span-trace-coverage rec=fct-gghzntp4@2
boundary: confirmed send log fields
item: hook-invoked
promise: hook-invoked{hook_event:"user-prompt-submit", corr:null} for an ordinary send; hook_event session-start for a /clear or /compact confirmed by its session; an unconfirmable send waits for no hook line.
@row_check n=746 row=contract/send-log-fields/hook-invoked sha=62ba999126c4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:174
quote: hook_event = event.as_str(),
at: crates/viola-agent-claude/src/hook.rs:46
quote: Self::UserPromptSubmit => "user-prompt-submit",
at: crates/viola-agent-claude/src/hook.rs:45
quote: Self::SessionStart => "session-start",
@row_add n=747 row=contract/send-log-fields/send-confirmed area=span-trace-coverage rec=fct-gghzntp4@2
boundary: confirmed send log fields
item: send-confirmed
promise: send-confirmed{corr:<cursor>, confirmed, duration_ms}, confirmed false for unconfirmable, duration_ms from the send's arrival including the readiness gate's wait.
@row_check n=747 row=contract/send-log-fields/send-confirmed sha=b07678f4dda6 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:239-240
quote: confirmed = confirmed, duration_ms = duration_ms,
at: src/run/send.rs:86
quote: started: slot.clock.now(),
at: src/run/send.rs:223
quote: self.log_confirmed(cursor, false, now);
@row_add n=748 row=contract/send-refused-detail/input-not-ready area=span-trace-coverage rec=fct-gghzntp4@2
boundary: wrapper send-refused detail
item: input-not-ready
promise: send-refused{corr:<cursor>, refusal, detail:"input-not-ready", side:"wrapper", wheel}.
@row_check n=748 row=contract/send-refused-detail/input-not-ready sha=123e94858bea v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:290
quote: wheel = self.wheel.holder().as_str(),
at: src/run/send.rs:130
quote: return ctx.not_delivered(None, NotDelivered::InputNotReady);
at: crates/viola-agent-claude/src/screen.rs:65
quote: Self::InputNotReady => "input-not-ready",
@row_add n=749 row=contract/send-refused-detail/no-prompt-submitted area=span-trace-coverage rec=fct-gghzntp4@2,fct-u2zbdxyd@2
boundary: wrapper send-refused detail
item: no-prompt-submitted
promise: Confirm-window expiry is send-refused{detail:"no-prompt-submitted"} for ordinary, /clear and /compact sends alike, with no new detail.
@row_check n=749 row=contract/send-refused-detail/no-prompt-submitted sha=9e5438f44baa v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:156
quote: None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
at: src/run/send/slot.rs:177
quote: Some(Confirms::Post(PostCondition::CompactedSession)) => COMPACT_CONFIRM_WINDOW,
@row_add n=750 row=contract/send-refused-detail/turn-running area=span-trace-coverage rec=fct-gghzntp4@2
boundary: wrapper send-refused detail
item: turn-running
promise: Covers a running turn or another send in flight, with no new field; the refusal has no cursor (corr the end offset at refusal), no send-issued, and the client exits 13.
@row_check n=750 row=contract/send-refused-detail/turn-running sha=43f26cb9e824 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:114
quote: if flight.send.is_some() || slot.wheel.turn_running() {
at: src/run/send.rs:116
quote: return ctx.not_delivered(None, NotDelivered::TurnRunning);
at: src/run/send.rs:277
quote: None => end_offset(self.instance_dir).ok(),
at: src/cmd/send.rs:70
quote: RefusalReason::NotDelivered => 13,
@row_add n=751 row=contract/send-refused-client/control-character area=span-trace-coverage rec=fct-gghzntp4@2
boundary: client-side send validation refusal
item: control-character
promise: Made before any frame: send-refused{corr:null, refusal:"not-delivered", detail:"control-character", side:"client"}, one line per deciding side, never the text.
@row_check n=751 row=contract/send-refused-client/control-character sha=c18ba956b755 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:182
quote: if let Err(detail) = viola_core::validate_paste_text(text) {
at: src/cmd/send.rs:240
quote: side = "client",
@row_add n=752 row=contract/send-refused-client/empty-text area=span-trace-coverage rec=fct-gghzntp4@2
boundary: client-side send validation refusal
item: empty-text
promise: An empty typed text, read after control-character and ahead of both wheel reads; a wrapper deciding it writes its own send-refused{side:"wrapper"} with the same pair, corr, conn and rpc_id, and no send-issued.
@row_check n=752 row=contract/send-refused-client/empty-text sha=21e5f3fe3d7a v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:93-94
quote: if text.is_empty() { return ctx.not_delivered(None, NotDelivered::EmptyText);
at: src/run/send.rs:289
quote: side = "wrapper",
@row_add n=753 row=decision/send-line-correlation area=span-trace-coverage rec=fct-gghzntp4@2
name: Send lines correlate with channel lines
rule: Every wrapper send-* line carries conn (or srv_conn) and rpc_id, so that pair on the send line equals (conn, corr) on the call's channel lines; the client-side send-refused carries neither.
@row_check n=753 row=decision/send-line-correlation sha=9a90032a1e75 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:197-198
quote: rpc_id = self.call.id, conn = self.call.conn,
at: src/run/send.rs:234-235
quote: rpc_id = self.call.id, conn = self.call.conn,
at: src/run/send.rs:284-285
quote: rpc_id = self.call.id, conn = self.call.conn,
at: src/cmd/send.rs:237-242
quote: obs_event!( INFO, ObsEvent::SendRefused, side = "client", refusal = RefusalReason::NotDelivered.as_str(), detail = detail.as_str(),
@row_add n=754 row=contract/confirm-window-close/ordinary-send area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: run.confirm_window close
item: ordinary send
promise: Closes on the matching prompt-submitted{origin:"driver"}, or on window expiry.
@row_check n=754 row=contract/confirm-window-close/ordinary-send sha=814315dc4c2c v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:203
quote: line.data["origin"] = json!("driver");
at: src/run/send/slot.rs:158
quote: Some(Match::Waiting) if self.clock.now() >= deadline => {
@row_add n=755 row=contract/confirm-window-close/clear area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: run.confirm_window close
item: /clear
promise: On a verified CLI, closes on the session-start whose cause is clear and whose agent_session_id differs from the id the send slot remembers; no prompt-submitted claims the send.
@row_check n=755 row=contract/confirm-window-close/clear sha=1d5385aad467 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:111
quote: Some("clear") if prior.as_deref() != Some(id) => Some(PostCondition::NewSession),
@row_add n=756 row=contract/confirm-window-close/compact area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: run.confirm_window close
item: /compact
promise: On a verified CLI, closes on the session-start whose cause is compact and whose agent_session_id equals the id the send slot remembered; nothing remembered claims nothing, and no prompt-submitted claims the send.
@row_check n=756 row=contract/confirm-window-close/compact sha=946310df49e7 v=holds judge=sonnet head=8834195b0683
at: src/run/send/slot.rs:112
quote: Some("compact") if prior.as_deref() == Some(id) => {
@row_add n=757 row=contract/cli-exit/1-internal-error area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: send.client process-exit
item: 1 internal-error
promise: exit_code 1 internal-error on a reply with neither a parseable result nor an error.code.
@row_check n=757 row=contract/cli-exit/1-internal-error sha=1c6dbc3b9abf v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:204
quote: let reply = reply_of(&frame).ok_or_else(|| anyhow::anyhow!("malformed reply"))?;
at: src/obs.rs:240-241
quote: exit_code = 1u8, detail = "internal-error",
@row_add n=758 row=contract/cli-exit/20-wrapper-fault area=span-trace-coverage rec=fct-u2zbdxyd@2
boundary: send.client process-exit
item: 20 wrapper-fault
promise: Exit 20 carries detail wrapper-fault.
@row_check n=758 row=contract/cli-exit/20-wrapper-fault sha=8177e64f8ddf v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:244
quote: run::log_self_exit(20, Some("wrapper-fault"));
@row_add n=759 row=contract/cli-exit-21/instance-dead area=span-trace-coverage rec=fct-u2zbdxyd@2,fct-hluu43pb@2
boundary: cli exit 21 process-exit
item: instance-dead
promise: detail instance-dead with during connect: no live snapshot, a replayed one included, or the connect failed.
@row_check n=759 row=contract/cli-exit-21/instance-dead sha=53d58ae3c6b8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:196-197
quote: Err(CallError::Channel(ChannelError::Connect(_))) => {             return Ok(Err(Unreachable::Dead));
at: src/cmd/client.rs:137
quote: Self::Dead | Self::Unwrapped | Self::ServerUnverified | Self::SocketDir => "connect",
@row_add n=760 row=contract/cli-exit-21/unwrapped-name area=span-trace-coverage rec=fct-u2zbdxyd@2,fct-hluu43pb@2
boundary: cli exit 21 process-exit
item: unwrapped-name
promise: detail unwrapped-name with during connect: no instance directory.
@row_check n=760 row=contract/cli-exit-21/unwrapped-name sha=aad782291173 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:111
quote: UnreachableCause::UnwrappedName => Self::Unwrapped,
at: crates/viola-state/src/liveness.rs:113
quote: _ => UnreachableCause::UnwrappedName,
@row_add n=761 row=contract/cli-exit-21/endpoint-lost area=span-trace-coverage rec=fct-u2zbdxyd@2,fct-hluu43pb@2
boundary: cli exit 21 process-exit
item: endpoint-lost
promise: detail endpoint-lost with during call: the connection was lost after the connect; it is the only exit-21 detail whose during is call.
@row_check n=761 row=contract/cli-exit-21/endpoint-lost sha=54a23975ccb3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:136
quote: Self::Lost => "call",
at: src/cmd/client.rs:199-200
quote: Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => {             return Ok(Err(Unreachable::Lost));
@row_add n=762 row=contract/wait-last-spans/span-chain area=span-trace-coverage rec=fct-l4ierws2@2
boundary: wait and last spans
item: span chain
promise: wait.block (CLIENT) › channel.request → channel.dispatch › run.wait_dispatch (INTERNAL); for last, last.client › channel.request.
@row_check n=762 row=contract/wait-last-spans/span-chain sha=99135662e2c7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:88
quote: #[instrument(skip_all, name = "wait.block", fields(after = after, timeout_ms = timeout_ms))]
at: crates/viola-channel/src/client.rs:67
quote: #[instrument(skip_all, name = "channel.request", fields(method = method, conn = self.conn.as_str()))]
at: crates/viola-channel/src/server.rs:230
quote: #[instrument(skip_all, name = "channel.dispatch", fields(method = method, conn = peer.label()))]
at: src/run/wait.rs:132
quote: name = "run.wait_dispatch",
at: src/cmd/last.rs:65
quote: #[instrument(skip_all, name = "last.client")]
@row_add n=763 row=contract/wait-span-attributes/wait-block area=span-trace-coverage rec=fct-ykdl4lpn@1
boundary: wait span attributes
item: wait.block
promise: wait.block carries after and timeout_ms.
@row_check n=763 row=contract/wait-span-attributes/wait-block sha=6a0ecd6302f5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/wait.rs:88
quote: #[instrument(skip_all, name = "wait.block", fields(after = after, timeout_ms = timeout_ms))]
@row_add n=764 row=contract/wait-span-attributes/run-wait-dispatch area=span-trace-coverage rec=fct-ykdl4lpn@1
boundary: wait span attributes
item: run.wait_dispatch
promise: woken_kind, one of turn-ended, question, permission, plan, session-end, dialog-expired, or timed_out.
@row_check n=764 row=contract/wait-span-attributes/run-wait-dispatch sha=48063dfa27e9 v=holds judge=sonnet head=8834195b0683
at: src/run/wait.rs:154
quote: span.record("woken_kind", line.value["kind"].as_str());
at: src/run/wait.rs:160
quote: span.record("timed_out", true);
at: crates/viola-core/src/lib.rs:99
quote: pub const WAIT_WAKE: [Self; 6] = [
@row_add n=765 row=contract/wait-log-fields/channel-request area=span-trace-coverage rec=fct-hluu43pb@2
boundary: wait and last log fields
item: channel-request
promise: channel-request{corr:<id>, conn, method:"wait"|"last"} on both sides, plus after and timeout_ms for wait only and only as u64 values.
@row_check n=765 row=contract/wait-log-fields/channel-request sha=f51a99b8ac1e v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:367
quote: ObsEvent::ChannelRequest,
at: crates/viola-channel/src/client.rs:182
quote: ObsEvent::ChannelRequest,
at: crates/viola-channel/src/server.rs:385
quote: (params["after"].as_u64(), params["timeout_ms"].as_u64())
@row_add n=766 row=contract/wait-log-fields/channel-response-outcome area=span-trace-coverage rec=fct-hluu43pb@2
boundary: wait and last log fields
item: channel-response outcome
promise: For wait only, outcome is the waking kind or timed-out, derived once over the shared wake set; the client side alone writes instance-unreachable when the reply never arrived; last carries no outcome and no result body reaches either line.
@row_check n=766 row=contract/wait-log-fields/channel-response-outcome sha=76b54a7a236d v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:391
quote: if method != Some("wait") {
at: crates/viola-channel/src/server.rs:396
quote: return Some("timed-out");
at: crates/viola-channel/src/server.rs:399
quote: EventKind::WAIT_WAKE
at: crates/viola-channel/src/client.rs:217
quote: (method == Some("wait") && vanished).then_some("instance-unreachable")
@row_add n=767 row=contract/mcp-refusal-text/instance-unreachable area=span-trace-coverage rec=fct-hluu43pb@2
boundary: viola mcp wait and last refusal
item: strict-modes-failed body
promise: MCP wait and last refused by the instance check return the cli document less v (error instance-unreachable, detail strict-modes-failed), their text refused: instance-unreachable, with no line.
@row_check n=767 row=contract/mcp-refusal-text/instance-unreachable sha=c138b6ffdd5e v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/tools.rs:340
quote: return Ok(result::unreachable(UnreachableCause::StrictModes));
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
@row_add n=768 row=decision/no-last-assistant-message area=span-trace-coverage rec=fct-hluu43pb@2
name: last_assistant_message never logged
rule: last_assistant_message never appears in a log line.
@row_check n=768 row=decision/no-last-assistant-message sha=f32a4a83248c v=holds judge=sonnet head=8834195b0683
searched: last_assistant_message · in src/obs.rs src/obs/ crates/viola-core/src/obs* schemas/diag-line.v1.json
searched: obs_event!\([^;]{0,500}last_assistant_message · in src/**/*.rs crates/*/src/**/*.rs
@row_add n=769 row=decision/disconnect-logged-by-client area=span-trace-coverage rec=fct-3omdhkir@1
name: Mid-call disconnect logged once by the client
rule: A mid-call disconnect is logged once, by the client.
@row_check n=769 row=decision/disconnect-logged-by-client sha=5953fe8fa83b v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:121
quote: Self::Lost => UnreachableCause::EndpointLost,
at: src/cmd/client.rs:262
quote: during = cause.during(),
@row_add n=770 row=contract/answer-link-record/failure area=span-trace-coverage rec=fct-u5crvqku@2
boundary: wrapper answer handling
item: failed link record
promise: A link record that fails answers -32603: the answer is not taken, the dialog stays held and no dialog-answered line is written.
@row_check n=770 row=contract/answer-link-record/failure sha=83121c69084c v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:455
quote: links.record(from).map_err(|_| ProtocolError::Internal)?;
at: src/run/dialog.rs:468
quote: pending.answer = Some(response);
@row_add n=771 row=decision/dialog-hook-fails-open area=span-trace-coverage rec=fct-u5crvqku@2
name: Dialog hook fails open on a failed check
rule: A dialog hook whose instance, socket-directory, euid or server check fails emits no decision and sends no frame, so the dialog falls back to the human.
@row_check n=771 row=decision/dialog-hook-fails-open sha=3eca6fcada6b v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/hook.rs:231
quote: if check_endpoint_dir(endpoint).is_err() {
at: src/cmd/hook.rs:236
quote: if !peer_admitted(client.server_euid()) {
at: src/cmd/hook.rs:318
quote: Err(NotSent::ServerUnverified) => return Asked::ServerUnverified,
@row_add n=772 row=decision/second-dialog-gets-null area=span-trace-coverage rec=fct-uicv6khr@1
name: A concurrent second dialog gets null
rule: A second concurrent dialog gets a null response.
@row_check n=772 row=decision/second-dialog-gets-null sha=68f2d5a5cfe5 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:252
quote: && state.pending.is_none()
at: src/run/dialog.rs:272
quote: return Ok(reply(dialog_id, None));
@row_add n=773 row=contract/dialog-log-fields/dialog-raised area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: dialog log fields
item: dialog-raised
promise: dialog-raised{corr:<dialog_id>, dialog_kind, hook_event}.
@row_check n=773 row=contract/dialog-log-fields/dialog-raised sha=1c98be1d3d90 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:314
quote: ObsEvent::DialogRaised,
at: src/run/dialog.rs:315-317
quote: corr = dialog_id, dialog_kind = raise.kind.as_str(), hook_event = raise.hook.as_str(),
@row_add n=774 row=contract/dialog-log-fields/dialog-answered area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: dialog log fields
item: dialog-answered
promise: dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}; from and from_trust only on an answer that carried a from.
@row_check n=774 row=contract/dialog-log-fields/dialog-answered sha=75e8196dbe9d v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:461-466
quote: corr = dialog_id, dialog_kind = pending.kind.as_str(), from = from, from_trust = from.map(|_| "self-reported"), deadline_hit = false, duration_ms = millis(self.clock.now().saturating_duration_since(pending.raised)),
at: src/run/dialog.rs:389-392
quote: corr = pending.dialog_id, dialog_kind = pending.kind.as_str(), deadline_hit = true, duration_ms = millis(self.clock.now().saturating_duration_since(pending.raised)),
@row_add n=775 row=contract/dialog-log-fields/hook-lines area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: dialog log fields
item: hook-invoked and hook-decision
promise: hook-invoked{corr:<dialog_id>, hook_event, invoked_at}, then hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}.
@row_check n=775 row=contract/dialog-log-fields/hook-lines sha=51effbf8a887 v=stale judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:366-372
quote: obs_event!( INFO, ObsEvent::HookInvoked, corr = dialog_id, hook_event = event.as_str(), invoked_at = invoked_at, );
shows: hook-invoked carries corr, hook_event and invoked_at only; no code emits stdin_bytes (it stands only in schemas/diag-line.v1.json:51).
cell: promise
value: hook-invoked{corr:<dialog_id>, hook_event, invoked_at}, then hook-decision{corr:<dialog_id>, hook_event, decision_emitted, deadline_hit, detail, duration_ms}.
@row_add n=776 row=decision/dialog-hook-invoked-after-response area=span-trace-coverage rec=fct-ffumo3bh@1
name: Dialog hook-invoked written after the response
rule: A dialog hook writes hook-invoked only once dialog_id is known, after the hook.dialog response; non-dialog hooks and fail-open paths before dialog_id write hook-invoked{corr:null} at once.
why: invoked_at still records the hook's true start, and the wrapper's dialog-raised gives live visibility during the wait.
@row_check n=776 row=decision/dialog-hook-invoked-after-response sha=1d9e8dc75ff1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:296
quote: invoked(event, &invoked_at, dialog_id);
at: src/cmd/hook.rs:285
quote: invoked(event, &invoked_at, None);
@row_add n=777 row=contract/answer-refusal/not-delivered area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: driver-side answer refusal
item: not-delivered
promise: channel-response{result_class:"refusal"} with no refusal word and no detail on the line, exit 13.
@row_check n=777 row=contract/answer-refusal/not-delivered sha=aeca74f2de60 v=stale judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:480-482
quote: result_class = result_class, error_code = fault.map(ProtocolError::code), detail = fault.and_then(ProtocolError::cause),
shows: The channel-response line carries result_class "refusal" and no detail, but no `refusal` key at all (the refusal word is on send-refused only); exit 13 is src/cmd/send.rs:70 `RefusalReason::NotDelivered => 13,`.
cell: promise
value: channel-response{result_class:"refusal"} with no refusal word and no detail on the line, exit 13.
@row_add n=778 row=contract/answer-refusal/unverified-cli area=span-trace-coverage rec=fct-ffumo3bh@1
boundary: driver-side answer refusal
item: unverified-cli
promise: refusal unverified-cli, exit 12, the answer withheld.
@row_check n=778 row=contract/answer-refusal/unverified-cli sha=52ad97800bf9 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:435-436
quote: if !self.cli_verified { return Ok(refusal(RefusalReason::UnverifiedCli, None));
at: src/cmd/send.rs:69
quote: RefusalReason::UnverifiedCli => 12,
@row_add n=779 row=decision/no-dialog-content-in-logs area=span-trace-coverage rec=fct-ffumo3bh@1
name: Dialog content never logged
rule: The question, the answer and the tool input never appear in a log line.
@row_check n=779 row=decision/no-dialog-content-in-logs sha=2e79298b00c0 v=holds judge=sonnet head=8834195b0683
searched: `^\s+(input|question|answer|text|plan|tool_input|response|keys|bytes) = ` · in src/**/*.rs and crates/**/*.rs (only hit crates/viola-mcp/src/stdin.rs:70, a local assignment, no log field)
searched: `obs_event!` field lists of the dialog events · in src/run/dialog.rs:312-318, 386-393 and src/cmd/hook.rs:365-407 (only dialog_kind, hook_event, corr, deadline_hit, duration_ms, detail, invoked_at)
@row_add n=780 row=contract/events-log/dialog-expired area=span-trace-coverage rec=fct-k233iq5j@1
boundary: events log
item: dialog-expired
promise: At the hold's end the wrapper appends one dialog-expired{dialog_id, dialog_kind} line (source wrapper, no question, plan or tool text) that wakes a parked wait, and writes dialog-answered with deadline_hit:true and no from.
@row_check n=780 row=contract/events-log/dialog-expired sha=4a041c2a6024 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:378
quote: EventKind::DialogExpired,
at: src/run/dialog.rs:391
quote: deadline_hit = true,
@row_add n=781 row=decision/expired-dialog-renders-for-human area=span-trace-coverage rec=fct-k233iq5j@1
name: Expired dialog returns to the human
rule: At the hold's end the reply stays null, the hook writes hook-decision with decision_emitted false, exits 0 with empty stdout, and the dialog renders for the human.
@row_check n=781 row=decision/expired-dialog-renders-for-human sha=62ef633cb145 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:348-349
quote: if self.clock.now() >= deadline { break (None, true);
at: src/run/dialog.rs:369
quote: reply(dialog_id, answer.as_ref())
at: src/cmd/hook.rs:297
quote: if let Some(body) = &body {
at: src/cmd/hook.rs:393
quote: decision_emitted = emitted,
@row_add n=782 row=contract/hook-decision/deadline-hit area=span-trace-coverage rec=fct-k233iq5j@1
boundary: dialog hook-decision line
item: deadline_hit
promise: The hook writes deadline_hit:true only when its own read times out at its backstop bound, DIALOG_HOLD_MAX + 5 s (3605 s).
@row_check n=782 row=contract/hook-decision/deadline-hit sha=b275b9aec78a v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:72
quote: started + DIALOG_HOLD_MAX + REPLY_GRACE
at: src/cmd/hook.rs:386
quote: let deadline_hit = *asked == Asked::Deadline;
@row_add n=783 row=contract/wheel-spans/span-chain area=span-trace-coverage rec=fct-a7vqqg3h@2
boundary: wheel spans
item: span chain
promise: run.wheel_transition (INTERNAL) › channel.dispatch(send) refused; pause.client or release.client (CLIENT) › channel.request(pause|release) → channel.dispatch(pause|release).
@row_check n=783 row=contract/wheel-spans/span-chain sha=91cf4e87c5f9 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:277
quote: name = "run.wheel_transition",
at: src/cmd/pause.rs:91
quote: #[instrument(skip_all, name = "pause.client")]
at: src/cmd/release.rs:75
quote: #[instrument(skip_all, name = "release.client")]
at: crates/viola-channel/src/client.rs:67
quote: #[instrument(skip_all, name = "channel.request", fields(method = method, conn = self.conn.as_str()))]
at: crates/viola-channel/src/server.rs:230
quote: #[instrument(skip_all, name = "channel.dispatch", fields(method = method, conn = peer.label()))]
@row_add n=784 row=contract/pause-release/answer-after-wheel-record area=span-trace-coverage rec=fct-a7vqqg3h@2
boundary: pause and release methods
item: reply timing
promise: pause and release answer only after the move's wheel record lands, -32603 if it did not.
@row_check n=784 row=contract/pause-release/answer-after-wheel-record sha=c14dc29bb856 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:230-231
quote: Some(outcome) if outcome.recv().unwrap_or(false) => Ok(()), Some(_) => Err(ProtocolError::Internal),
at: src/run/wheel.rs:221
quote: settled(self.apply(Input::Release, true))?;
@row_add n=785 row=contract/wheel-span-attributes/run-wheel-transition area=span-trace-coverage rec=fct-yj7tr26f@1
boundary: wheel span attributes
item: run.wheel_transition
promise: wheel_from, wheel_to (human|driver), cause (human-input|manual-pause|release); a point-in-time span with a static name, opened once per wheel change on the wheel's worker thread.
@row_check n=785 row=contract/wheel-span-attributes/run-wheel-transition sha=dc5bd5f974ca v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:279-281
quote: wheel_from = step.from.as_str(), wheel_to = step.to.as_str(), cause = step.cause.as_str()
at: src/run/wheel.rs:122
quote: std::thread::spawn(move || recorder.run(&rx));
@row_add n=786 row=decision/no-typed-keys-in-telemetry area=span-trace-coverage rec=fct-yj7tr26f@1
name: Typed keys never in telemetry
rule: The keys the human types never appear in a span.
@row_check n=786 row=decision/no-typed-keys-in-telemetry sha=e591c438dae8 v=holds judge=sonnet head=8834195b0683
searched: `fields(` · in src/**/*.rs (every span field list; src/run/wheel.rs:278-282 holds only wheel_from, wheel_to, cause; none holds typed keys or input bytes)
searched: `keys|typed` as a span or obs_event field · in src/run/wheel.rs and src/human/*.rs (none)
@row_add n=787 row=contract/send-refused/human-typing area=span-trace-coverage rec=fct-wovktszp@1
boundary: wrapper send-refused
item: human-typing
promise: send-refused{corr:<cursor>, refusal:"human-typing", detail:null|"manual-pause", wheel:"human", side:"wrapper"}, with the client process-exit{exit_code:10}.
@row_check n=787 row=contract/send-refused/human-typing sha=9e03c5ea9673 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:289-292
quote: side = "wrapper", wheel = self.wheel.holder().as_str(), refusal = refusal.as_str(), detail = detail,
at: src/cmd/send.rs:67
quote: RefusalReason::HumanTyping => 10,
@row_add n=788 row=decision/release-from-driver-refused area=span-trace-coverage rec=fct-wovktszp@1
name: A release from a driver is refused
rule: A release that carries a string from is refused with -32602, and viola release forwards VIOLA_NAME, so a driver session's own viola release is refused too.
@row_check n=788 row=decision/release-from-driver-refused sha=cc1397135b10 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: crates/viola-channel/src/lib.rs:74
quote: Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
at: src/cmd/pause.rs:70-71
quote: if let Some(from) = own_name() { params.insert("from".to_owned(), from.as_ref().into());
@row_add n=789 row=contract/release-refusal/release-from-driver area=span-trace-coverage rec=fct-wovktszp@1
boundary: release refusal lines
item: release-from-driver
promise: One release-from-driver{corr:<rpc id>, conn, from (only a valid name), from_trust:"self-reported"} at INFO beside the -32602 reply with data {"reason":"release-from-driver"}; its channel-response carries detail release-from-driver on both sides at WARN.
@row_check n=789 row=contract/release-refusal/release-from-driver sha=c4be0f32cb86 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:238-244
quote: obs_event!( INFO, ObsEvent::ReleaseFromDriver, corr = call.id, conn = call.conn, from = from.as_ref().map(AsRef::<str>::as_ref), from_trust = "self-reported",
at: crates/viola-channel/src/lib.rs:107
quote: Self::ReleaseFromDriver => Some("release-from-driver"),
at: crates/viola-channel/src/server.rs:490-496
quote: Some( ProtocolError::InvalidRequest | ProtocolError::UnsupportedVersion | ProtocolError::InvalidParams | ProtocolError::ReleaseFromDriver, ) => { response_line!(WARN)
@row_add n=790 row=contract/release-refusal/invalid-params area=span-trace-coverage rec=fct-wovktszp@1
boundary: release refusal lines
item: invalid-params
promise: A from of another type, or a non-bool budget, is a plain -32602 (data null) with no release-from-driver line, its channel-response carrying detail invalid-params on both sides at WARN.
@row_check n=790 row=contract/release-refusal/invalid-params sha=fbbd79a9cc9e v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:211
quote: Some(budget) => budget.as_bool().ok_or(ProtocolError::InvalidParams)?,
at: src/run/wheel.rs:208
quote: parse_from(params)?;
at: crates/viola-channel/src/lib.rs:106
quote: Self::InvalidParams => Some("invalid-params"),
@row_add n=791 row=contract/release-budget/budget-gate area=span-trace-coverage rec=fct-wovktszp@1
boundary: release method
item: budget true
promise: A release with budget true does not touch the wheel and appends one budget-gate record when a pause stood, none otherwise; the string-from refusal and the budget type check come first.
@row_check n=791 row=contract/release-budget/budget-gate sha=ef97cb9a63b9 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:213-219
quote: if budget { let lifted = gate.lift().map_err(|_| ProtocolError::Internal)?; let mut ok = json!({"wheel": self.holder().as_str(), "budget_paused": false}); if let Some(end) = lifted { ok["override_until"] = json!(end); } return Ok(json!({"ok": ok}));
at: src/run/budget.rs:311-312
quote: let Some(crossed) = &recorded.paused else { return Ok(None);
@row_add n=792 row=contract/budget-spans/span-chain area=span-trace-coverage rec=fct-koyhaawm@1
boundary: budget governor spans
item: span chain
promise: hook.statusline (SERVER) › state.budget_write › statusline.shell_out (CLIENT); in run, run.budget_eval (INTERNAL).
@row_check n=792 row=contract/budget-spans/span-chain sha=6178a4b0d325 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:99
quote: #[instrument(skip_all, name = "hook.statusline")]
at: crates/viola-state/src/budget.rs:44
quote: name = "state.budget_write",
at: src/cmd/hook/statusline.rs:195
quote: name = "statusline.shell_out",
at: src/run/budget.rs:260
quote: name = "run.budget_eval",
@row_add n=793 row=contract/budget-span-attributes/state-budget-write area=span-trace-coverage rec=fct-6peizmat@1
boundary: budget span attributes
item: state.budget_write
promise: state.budget_write carries five_hour_pct and seven_day_pct.
@row_check n=793 row=contract/budget-span-attributes/state-budget-write sha=c67d0ab9ffea v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/budget.rs:44-45
quote: name = "state.budget_write", fields(five_hour_pct = used(&reading.five_hour), seven_day_pct = used(&reading.seven_day))
@row_add n=794 row=contract/budget-span-attributes/statusline-shell-out area=span-trace-coverage rec=fct-6peizmat@1
boundary: budget span attributes
item: statusline.shell_out
promise: statusline.shell_out carries shell_exit_status and duration_ms.
@row_check n=794 row=contract/budget-span-attributes/statusline-shell-out sha=d93fba3fe6eb v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:195-196
quote: name = "statusline.shell_out", fields(shell_exit_status = Empty, duration_ms = Empty)
@row_add n=795 row=contract/budget-span-attributes/run-budget-eval area=span-trace-coverage rec=fct-6peizmat@1
boundary: budget span attributes
item: run.budget_eval
promise: trigger (start|send|turn-ended), paused, and window only when paused; opened once per evaluation: at start, at a send's arrival, and after a turn-ended hook line is appended.
@row_check n=795 row=contract/budget-span-attributes/run-budget-eval sha=675802f302be v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:260-261
quote: name = "run.budget_eval", fields(trigger = trigger.as_str(), paused = Empty, window = Empty)
at: src/run/budget.rs:274
quote: span.record("window", crossed.window.as_str());
at: src/cmd/run.rs:131
quote: .evaluate(Trigger::TurnEnded, Utc::now())
@row_add n=796 row=decision/unreadable-budget-reads-as-none area=span-trace-coverage rec=fct-6peizmat@1
name: Bad budget file reads as no reading
rule: A refused, unreadable, over-cap or other-version budget file reads as no reading.
@row_check n=796 row=decision/unreadable-budget-reads-as-none sha=0fc166ba1f1d v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/budget.rs:94-95
quote: check_path(home).ok()?; check_path(&path).ok()?;
at: crates/viola-state/src/budget.rs:104-107
quote: if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME { return None; } let stored: Stored = serde_json::from_slice(&bytes).ok()?;
at: crates/viola-state/src/budget.rs:112
quote: (stored.v == BUDGET_V).then_some(StampedReading {
@row_add n=797 row=contract/events-log/budget-gate area=span-trace-coverage rec=fct-6peizmat@1
boundary: events log
item: budget-gate
promise: A budget-gate record is appended only when the triple (paused, window, override end) changes.
@row_check n=797 row=contract/events-log/budget-gate sha=d73ee4c46600 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:299
quote: if standing.triple() != recorded.triple() {
@row_add n=798 row=contract/statusline-log-fields/hook-invoked area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook log fields
item: hook-invoked
promise: hook-invoked{hook_event:"statusline", corr:null}.
@row_check n=798 row=contract/statusline-log-fields/hook-invoked sha=f1874bb9b38f v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:102-107
quote: obs_event!( INFO, ObsEvent::HookInvoked, hook_event = WORD, invoked_at = invoked_at.as_str(), );
@row_add n=799 row=contract/statusline-log-fields/parse-rejected area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook log fields
item: parse-rejected
promise: parse-rejected{parser:"hook-stdin", detail:"oversize"|"malformed", count:1} for stdin the arm rejects.
@row_check n=799 row=contract/statusline-log-fields/parse-rejected sha=b981383ab0b4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:536-539
quote: ObsEvent::ParseRejected, parser = "hook-stdin", detail = detail, count = 1u64,
@row_add n=800 row=contract/statusline-log-fields/statusline-shell area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook log fields
item: statusline-shell
promise: process-start and process-exit{subject:"statusline-shell", shell_exit_status, duration_ms}, only when the snapshot records a statusline_command.
@row_check n=800 row=contract/statusline-log-fields/statusline-shell sha=dc1a6ee56494 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:200
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = SUBJECT);
at: src/cmd/hook/statusline.rs:207-213
quote: obs_event!( INFO, ObsEvent::ProcessExit, subject = SUBJECT, shell_exit_status = code, duration_ms = duration_ms, );
@row_add n=801 row=contract/statusline-log-fields/hook-decision area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook log fields
item: hook-decision
promise: hook-decision{hook_event:"statusline", budget_written, duration_ms}, deadline_hit only when true; budget_written is true only when the payload held a rate_limits object and the write succeeded.
@row_check n=801 row=contract/statusline-log-fields/hook-decision sha=bf9a77782de6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:167-173
quote: None => obs_event!( INFO, ObsEvent::HookDecision, hook_event = WORD, budget_written = outcome.budget_written, duration_ms = duration_ms, ),
at: src/cmd/hook/statusline.rs:163
quote: deadline_hit = (detail == "deadline").then_some(true),
at: src/cmd/hook/statusline.rs:126-127
quote: let budget_written = reading.is_some_and(|reading| write_budget(&instance.home, &reading, Utc::now()).is_ok());
@row_add n=802 row=contract/statusline-decision-detail/oversize-stdin area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook-decision detail
item: oversize-stdin
promise: The statusline hook-decision carries detail oversize-stdin on that fail-open path.
@row_check n=802 row=contract/statusline-decision-detail/oversize-stdin sha=31d5cccf8ca0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:119
quote: return Ok(Outcome::failed("oversize-stdin"));
@row_add n=803 row=contract/statusline-decision-detail/malformed-json area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: statusline hook-decision detail
item: malformed-json
promise: The statusline hook-decision carries detail malformed-json on that fail-open path.
@row_check n=803 row=contract/statusline-decision-detail/malformed-json sha=354c49c2e69e v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:123
quote: return Ok(Outcome::failed("malformed-json"));
@row_add n=804 row=contract/statusline-decision-detail/deadline area=span-trace-coverage rec=fct-gqw4imf3@2,fct-v76wl5kz@1
boundary: statusline hook-decision detail
item: deadline
promise: The shell-out reaching its bound ends in hook-decision{detail:"deadline", deadline_hit:true}.
@row_check n=804 row=contract/statusline-decision-detail/deadline sha=8e5eb6bd0b67 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:146
quote: Some(Shell::Deadline) => Some("deadline"),
at: src/cmd/hook/statusline.rs:163
quote: deadline_hit = (detail == "deadline").then_some(true),
@row_add n=805 row=decision/statusline-silent-without-instance area=span-trace-coverage rec=fct-gqw4imf3@2
name: Statusline arm silent without a valid instance
rule: The statusline arm writes no line without a valid VIOLA_NAME or when the instance check refuses: it returns exit 0 at once with no role file opened.
@row_check n=805 row=decision/statusline-silent-without-instance sha=34276c2576d6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:42-43
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=806 row=contract/send-refused/budget-paused area=span-trace-coverage rec=fct-gqw4imf3@2
boundary: wrapper send-refused
item: budget-paused
promise: send-refused{corr:<cursor>, refusal:"budget-paused", detail:"five-hour"|"seven-day", window, used_percentage, resets_at, paused:true}, with client exit_code 11.
@row_check n=806 row=contract/send-refused/budget-paused sha=74093ac5411d v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:293-296
quote: window = budget.map(|crossed| crossed.window.as_str()), used_percentage = budget.map(|crossed| crossed.used_percentage), resets_at = budget.map(|crossed| crossed.resets_at.as_str()), paused = budget.map(|_| true),
at: src/cmd/send.rs:68
quote: RefusalReason::BudgetPaused => 11,
@row_add n=807 row=decision/rejected-statusline-stdin-runs-nothing area=span-trace-coverage rec=fct-v76wl5kz@1
name: Rejected statusline stdin runs nothing
rule: Oversize or malformed statusline stdin prints nothing and runs no shell.
@row_check n=807 row=decision/rejected-statusline-stdin-runs-nothing sha=9c1bdc6cf3a4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:117-119
quote: if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME { reject_stdin("oversize"); return Ok(Outcome::failed("oversize-stdin"));
at: src/cmd/hook/statusline.rs:121-123
quote: let Ok(reading) = statusline::reading(&bytes) else { reject_stdin("malformed"); return Ok(Outcome::failed("malformed-json"));
@row_add n=808 row=decision/claude-env-strip area=span-trace-coverage rec=fct-6xa7lf3f@2
name: CLAUDE environment strip
rule: The child loses every inherited CLAUDE* name, matched exactly, except the persistent set kept in the claude_env_keep config key.
@row_check n=808 row=decision/claude-env-strip sha=d21aee051193 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:73
quote: if !floor && persistent.contains(&text) {
at: src/run/env.rs:13
quote: const KEEP_KEY: &str = "claude_env_keep";
@row_add n=809 row=decision/identity-floor-always-stripped area=span-trace-coverage rec=fct-6xa7lf3f@2
name: Identity floor always stripped
rule: The 11 names of the identity floor are always removed whatever the persistent set says; beyond the floor there is no compile-time list of known names.
@row_check n=809 row=decision/identity-floor-always-stripped sha=af5853a0b0e2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/lib.rs:17
quote: pub const IDENTITY_FLOOR: [&str; 11] = [
at: crates/viola-agent-claude/src/lib.rs:72-73
quote: let floor = IDENTITY_FLOOR.contains(&text.as_str()); if !floor && persistent.contains(&text) {
@row_add n=810 row=contract/claude-child-process-start/env-fields area=span-trace-coverage rec=fct-6xa7lf3f@2
boundary: process-start claude-child
item: env fields
promise: env_stripped_count, env_stripped_known (every stripped name, comma-joined) and env_kept (every kept persistent-set name, comma-joined).
@row_check n=810 row=contract/claude-child-process-start/env-fields sha=e84bc29f0cb3 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:57-59
quote: env_stripped_count = strip.count(), env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()), env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()),
at: crates/viola-agent-claude/src/lib.rs:45-46
quote: pub fn removed_names(&self) -> String {         joined(self.remove.iter().map(|n| n.to_string_lossy().into_owned()))
@row_add n=811 row=decision/implicit-link-untraced area=span-trace-coverage rec=fct-46kjozmt@1
name: Implicit link has no line or span
rule: The implicit link record has no process-log line, no event value and no span; its trace is the call's channel pair, the link product line and the snapshot's links.
@row_check n=811 row=decision/implicit-link-untraced sha=3a0db67fdaae v=holds judge=sonnet head=8834195b0683
searched: `obs_event!|instrument|span!` · in src/run/links.rs (none)
searched: `ObsEvent::Link` · in src/**/*.rs and crates/viola-core/src/obs.rs (no event value for link)
@row_add n=812 row=contract/link-methods/invalid-params area=span-trace-coverage rec=fct-lnbq3lnm@1
boundary: link and unlink methods
item: -32602
promise: A missing, non-string or invalid name, a driven that is not the serving wrapper's own name, or a driver equal to driven is -32602 with data null and no line; a record or remove error is -32603; neither method reads from.
@row_check n=812 row=contract/link-methods/invalid-params sha=8b2e28e7ea06 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:94-96
quote: let (driver, driven) = (named("driver")?, named("driven")?); if driven != self.name || driver == driven { return Err(ProtocolError::InvalidParams);
at: src/run/links.rs:74
quote: let (since, already_linked) = self.record(&driver).map_err(|_| ProtocolError::Internal)?;
at: src/run/links.rs:81
quote: self.remove(&driver).map_err(|_| ProtocolError::Internal)?;
@row_add n=813 row=decision/link-methods-untraced area=span-trace-coverage rec=fct-lnbq3lnm@1
name: Explicit link methods open no span
rule: The explicit link and unlink methods open no span and write no process-log line of their own; their channel lines hold no driver, driven or other param.
@row_check n=813 row=decision/link-methods-untraced sha=99ec00cc8f17 v=holds judge=sonnet head=8834195b0683
searched: `obs_event!|instrument|span!` · in src/run/links.rs (none; the link/unlink methods are served at src/cmd/run.rs:147-148)
searched: params logged on channel lines · in crates/viola-channel/src/server.rs:365-376 (only corr, conn, srv_conn, method, sender, v, after, timeout_ms; after/timeout_ms only for wait)
@row_add n=814 row=contract/events-log/link area=span-trace-coverage rec=fct-lnbq3lnm@1
boundary: events log
item: link
promise: link appends one link product line {driver, driven} (source wrapper) for a new pair and nothing for a pair the set holds, and answers after its line and the snapshot write.
@row_check n=814 row=contract/events-log/link sha=3123baebada8 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:62
quote: if let Some(since) = self.since(&set, driver) {
at: src/run/links.rs:65
quote: let line = self.line(EventKind::Link, driver);
at: src/run/links.rs:132
quote: append_event(&self.instance_dir, line)?;
@row_add n=815 row=contract/events-log/unlink area=span-trace-coverage rec=fct-lnbq3lnm@1
boundary: events log
item: unlink
promise: unlink appends one unlink product line {driver, driven} (source wrapper) only when it removes a pair, nothing for an absent one, and answers after its line and the snapshot write.
@row_check n=815 row=contract/events-log/unlink sha=48dc60851fc6 v=holds judge=sonnet head=8834195b0683
at: src/run/links.rs:105
quote: if self.since(&set, driver).is_none() {
at: src/run/links.rs:108
quote: let line = self.line(EventKind::Unlink, driver);
at: src/run/links.rs:82
quote: Ok(json!({"ok": {}}))
@row_add n=816 row=contract/state-recovered/snapshot-replayed area=span-trace-coverage rec=fct-wev2ceaa@1
boundary: state-recovered line
item: snapshot-replayed
promise: state-recovered{detail:"snapshot-replayed", file:"snapshot.json"} at WARN with no corr for an unreadable snapshot, one line per call; none for a present or absent snapshot, and none when the replay itself fails.
@row_check n=816 row=contract/state-recovered/snapshot-replayed sha=c1e0199667cd v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:159
quote: ReplayCause::Unreadable => ("snapshot-replayed", None),
at: crates/viola-state/src/replay.rs:163-167
quote: obs_event!( WARN, ObsEvent::StateRecovered, detail = detail, file = SNAPSHOT,
@row_add n=817 row=contract/state-recovered/snapshot-unsupported-v area=span-trace-coverage rec=fct-wev2ceaa@1
boundary: state-recovered line
item: snapshot-unsupported-v
promise: state-recovered{detail:"snapshot-unsupported-v", file:"snapshot.json", v_seen} at WARN with no corr for a newer snapshot, one line per call.
@row_check n=817 row=contract/state-recovered/snapshot-unsupported-v sha=99ace00bd4d6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:160
quote: ReplayCause::Unsupported { v_seen } => ("snapshot-unsupported-v", Some(v_seen)),
@row_add n=818 row=contract/state-recovered/scanned area=span-trace-coverage rec=fct-wev2ceaa@1
boundary: state-recovered line
item: scanned
promise: From the board read, the line carries instance (the caller) and scanned (the scanned instance's name when it parses, absent otherwise); with no VIOLA_NAME no log is open and the line goes nowhere.
@row_check n=818 row=contract/state-recovered/scanned sha=17c752711299 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/replay.rs:176-180
quote: fn scanned_name(instance_dir: &Path) -> Option<&str> { instance_dir .file_name() .and_then(|name| name.to_str()) .filter(|name| ViolaName::try_new((*name).to_owned()).is_ok())
@row_add n=819 row=decision/no-raw-snapshot-reads area=span-trace-coverage rec=fct-wev2ceaa@1
name: Product readers read the snapshot through replay
rule: Every product reader of the snapshot uses the replaying read; no product reader reads the snapshot raw.
@row_check n=819 row=decision/no-raw-snapshot-reads sha=b367426d6904 v=holds judge=sonnet head=8834195b0683
searched: `read_snapshot\(` · in src/**/*.rs outside test modules (every hit is after the file's #[cfg(test)]: dialog.rs:504, budget.rs:341, wheel.rs:529, links.rs:139)
searched: `read_snapshot\(` · in crates/**/*.rs (only the definition crates/viola-state/src/snapshot.rs:150 and its tests)
searched: `read_snapshot_or_replay` · in src (callers hook.rs:227, revive.rs:210, run.rs:277, statusline.rs:129) and crates/viola-state/src/board.rs:273
@row_add n=820 row=contract/state-recovered/torn-line-healed area=span-trace-coverage rec=fct-wgyuisdz@1
boundary: state-recovered line
item: torn-line-healed
promise: state-recovered{detail:"torn-line-healed", file:"events.ndjson", offset} at WARN with no corr, by the process whose append healed, offset the log's length before the append; one line per heal.
@row_check n=820 row=contract/state-recovered/torn-line-healed sha=3fb8cfe0dd79 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:336-342
quote: obs_event!( WARN, ObsEvent::StateRecovered, detail = "torn-line-healed", file = EVENTS, offset = tail.len, );
@row_add n=821 row=decision/appender-heals-torn-line area=span-trace-coverage rec=fct-wgyuisdz@1
name: The appender heals a torn line
rule: An append that finds the events log's last byte is not LF writes one LF and its line in a single write; the reader neither heals nor logs.
@row_check n=821 row=decision/appender-heals-torn-line sha=a7c217b0eb9f v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:335
quote: file.write_all(&[b"\n", bytes].concat())?;
at: crates/viola-state/src/events.rs:323
quote: torn: byte[0] != b'\n',
at: crates/viola-state/src/events.rs:144-145
quote: /// `wait`: the line STARTS at or after the cursor). A last line without its `\n` is counted as /// torn, neither returned nor rewritten, and an absent log reads as no lines.
@row_add n=822 row=contract/run-log-parse-rejected/ledger-stamps area=span-trace-coverage rec=fct-6z2owz4q@1
boundary: the run process log parse-rejected line
item: parser ledger-stamps
promise: An unreadable, malformed or strict-modes-refused capability ledger gives one parse-rejected line with parser "ledger-stamps", detail "unreadable", "malformed" or "strict-modes-failed", and a count, in the run log.
@row_check n=822 row=contract/run-log-parse-rejected/ledger-stamps sha=1c28cbba8f23 v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:177
quote: parser = "ledger-stamps",
at: src/run/version_gate.rs:204
quote: Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")),
at: src/run/version_gate.rs:205
quote: Err(StampsReadError::State(_)) => (false, Some("unreadable")),
at: src/run/version_gate.rs:208
quote: Err(_) => (false, Some("malformed")),
@row_add n=823 row=decision/unreadable-ledger-means-unverified area=span-trace-coverage rec=fct-6z2owz4q@1
name: Unreadable capability ledger means an unverified CLI
rule: A capability ledger that is missing a stamp, unreadable, malformed or refused by the strict-modes check is treated as an unverified CLI (cli_verified false).
@row_check n=823 row=decision/unreadable-ledger-means-unverified sha=a935fc17158e v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/stamps.rs:96
quote: pub fn read_stamps_strict(home: &Path) -> Result<Option<Vec<u8>>, StampsReadError> {
at: src/run/version_gate.rs:203-208
quote: Ok(None) => (false, None), Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")), Err(StampsReadError::State(_)) => (false, Some("unreadable")), Ok(Some(bytes)) => match verified(&bytes, version) { Ok(verified) => (verified, None), Err(_) => (false, Some("malformed")),
at: src/run/version_gate.rs:172
quote: let (verified, detail) = stamps_verdict(read_stamps_strict(home), version);
@row_add n=824 row=contract/verify-spawn-subject/version-probe area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify child spawn subjects
item: version-probe
promise: The --version read of the agent CLI is logged as one process-start / process-exit pair with subject version-probe.
@row_check n=824 row=contract/verify-spawn-subject/version-probe sha=73ef6b5fa2dc v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:184
quote: "version-probe",
at: src/cmd/verify.rs:123
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = subject);
at: src/cmd/verify.rs:132
quote: ObsEvent::ProcessExit,
@row_add n=825 row=contract/verify-spawn-subject/verify-probe area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify child spawn subjects
item: verify-probe
promise: The print-mode probe is logged as one process-start / process-exit pair with subject verify-probe.
@row_check n=825 row=contract/verify-spawn-subject/verify-probe sha=123bdf2f5b61 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:209
quote: "verify-probe",
at: src/cmd/verify.rs:123
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = subject);
@row_add n=826 row=contract/verify-spawn-subject/agents-probe area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify child spawn subjects
item: agents-probe
promise: Run B's single agents --json read is logged as one process-start / process-exit pair with subject agents-probe.
@row_check n=826 row=contract/verify-spawn-subject/agents-probe sha=33bbfbcc41af v=holds judge=sonnet head=8834195b0683
at: src/run/agents.rs:46
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
at: src/run/agents.rs:65
quote: subject = "agents-probe",
at: src/cmd/verify/typed/turns.rs:112
quote: let read = crate::run::agents::read_logged(inputs.program, dir, inputs.strip, run.stop);
@row_add n=827 row=contract/verify-spawn-subject/verify-pty-probe area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify child spawn subjects
item: verify-pty-probe
promise: Each of verify's five interactive PTY runs is logged as one process-start / process-exit pair with subject verify-pty-probe.
@row_check n=827 row=contract/verify-spawn-subject/verify-pty-probe sha=031d127feabd v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify/typed/run.rs:89
quote: obs_event!(INFO, ObsEvent::ProcessStart, subject = SUBJECT);
at: src/cmd/verify/typed/run.rs:223
quote: subject = SUBJECT,
at: src/cmd/verify/typed.rs:74
quote: let runs: [&RunStep<'_>; 5] = [
@row_add n=828 row=contract/verify-process-exit/interrupted area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify process-exit line
item: detail interrupted
promise: A verify stopped by SIGINT or SIGTERM starts no further spawn, closes every pair it opened and ends with process-exit subject self, exit_code 1, detail "interrupted".
@row_check n=828 row=contract/verify-process-exit/interrupted sha=9752b63aa8b4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
at: src/run/mod.rs:81
quote: subject = "self",
at: src/cmd/verify/typed.rs:85
quote: if (inputs.stop)() {
@row_add n=829 row=contract/verify-exit-codes/0 area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify exit codes
item: 0
promise: Every relied ledger row passed.
@row_check n=829 row=contract/verify-exit-codes/0 sha=a3a6877ad45a v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:302
quote: None => Ok(Ended::Exit(0)),
at: src/cmd/verify.rs:297-298
quote: if failed > 0 { return Ok(Ended::Exit(1));
@row_add n=830 row=contract/verify-exit-codes/1 area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify exit codes
item: 1
promise: A row failed, verify refused, or verify was stopped.
@row_check n=830 row=contract/verify-exit-codes/1 sha=7ff416898a4d v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:298
quote: return Ok(Ended::Exit(1));
at: src/cmd/verify.rs:81
quote: Ended::Refused(detail) => (1, Some(detail)),
at: src/cmd/verify.rs:87
quote: (1, Some("interrupted"))
@row_add n=831 row=contract/verify-stop-stderr/unable-hint-pair area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify stderr on a stop
item: the stop pair
promise: A stopped verify prints exactly "unable: verify was stopped before it finished" and "hint: run viola verify again to stamp this CLI" on stderr, with empty stdout and no stamp written.
@row_check n=831 row=contract/verify-stop-stderr/unable-hint-pair sha=97892c394f32 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:83-86
quote: human::refuse( "verify was stopped before it finished", "run viola verify again to stamp this CLI", );
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
@row_add n=832 row=contract/verify-stdout/step-line area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify stdout
item: step line
promise: One "[NN/MM] <row id> <row words>  pass|fail" line is printed for each relied ledger row, MM being the relied row count.
@row_check n=832 row=contract/verify-stdout/step-line sha=60ad12273d55 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:494
quote: "[{n:02}/{total:02}] {} {}  {verdict}",
at: src/cmd/verify.rs:475
quote: let total = relied.len();
@row_add n=833 row=contract/verify-stdout/summary-line area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify stdout
item: summary line
promise: The last stdout line is "stamped <version>  N pass  N fail", the two counts summing to MM.
@row_check n=833 row=contract/verify-stdout/summary-line sha=2dde1b0eefc0 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:293-296
quote: human::result(&format!( "stamped {version}  {} pass  {failed} fail", results.len() - failed ));
@row_add n=834 row=contract/cli-stderr/refusal-pair area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: CLI stderr
item: unable and hint pair
promise: A refusal is printed to stderr as a fixed "unable:" line followed by a "hint:" line.
@row_check n=834 row=contract/cli-stderr/refusal-pair sha=f3e71743b224 v=holds judge=sonnet head=8834195b0683
at: src/human.rs:37
quote: out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
@row_add n=835 row=contract/cli-stderr/internal-error area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: CLI stderr
item: internal fault message
promise: An internal fault prints exactly "error: internal error".
@row_check n=835 row=contract/cli-stderr/internal-error sha=28798f9ab17f v=holds judge=sonnet head=8834195b0683
at: src/human.rs:49
quote: out.write_all(b"error: internal error\n")
@row_add n=836 row=contract/plugin-install-exit-codes/21 area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: plugin install exit codes
item: 21
promise: The strict-modes tree check refused; no log is opened and no line is written, with the stderr pair or, under --json, the instance-unreachable document.
@row_check n=836 row=contract/plugin-install-exit-codes/21 sha=0d580f66ef5e v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:53-60
quote: if !tree_passes(&home) { if args.json { client::unreachable_document(UnreachableCause::StrictModes); } else { let (unable, hint) = human::home_state_pair(); human::refuse(unable, hint); } return Ok(ExitCode::from(21));
@row_add n=837 row=contract/plugin-install-json/ok area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: plugin install --json document
item: ok
promise: Success prints {"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}, with no path in it.
@row_check n=837 row=contract/plugin-install-json/ok sha=7014a1f14a9a v=holds judge=sonnet head=8834195b0683
at: src/cmd/plugin.rs:67-70
quote: client::document(&json!({ "v": 1, "ok": {"marketplace": marketplace::NAME, "plugin": PLUGIN_REF}, }));
at: crates/viola-agent-claude/src/marketplace.rs:14
quote: pub const PLUGIN_REF: &str = "viola@viola";
@row_add n=838 row=contract/verify-process-exit/strict-modes-failed area=span-trace-coverage rec=fct-f3gi2vhc@2
boundary: verify process-exit line
item: detail strict-modes-failed
promise: A stamps store refused by its strict-modes check, under a parsed VIOLA_NAME, ends with process-exit carrying detail "strict-modes-failed" and the home pair on stderr.
@row_check n=838 row=contract/verify-process-exit/strict-modes-failed sha=746c42cf625e v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:287
quote: refuse_home();
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=839 row=decision/refused-home-tree-writes-no-line area=span-trace-coverage rec=fct-f3gi2vhc@2
name: Refused home tree writes no line
rule: verify and plugin install check the home tree before their log opens, and a refused tree opens no log and writes no line.
@row_check n=839 row=decision/refused-home-tree-writes-no-line sha=57c1491534f5 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:67-73
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1)); } if let Some(name) = instance { let (level, rejection) = obs::read_diagnostics_level(&home); obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
at: src/cmd/plugin.rs:53-55
quote: if !tree_passes(&home) { if args.json { client::unreachable_document(UnreachableCause::StrictModes);
@row_add n=840 row=decision/verify-lines-carry-no-content area=span-trace-coverage rec=fct-f3gi2vhc@2
name: verify log lines carry no content
rule: No verify log line carries row text, a prompt, a session id, the probe command, a status line payload or a path.
@row_check n=840 row=decision/verify-lines-carry-no-content sha=93001f9064a4 v=holds judge=sonnet head=8834195b0683
searched: obs_event!|ObsEvent:: · in src/cmd/verify.rs and src/cmd/verify/ (only subject, child_exit_status, duration_ms and process-start/exit fields at verify.rs:123-136 and verify/typed/run.rs:89,220-226)
searched: session_id|prompt|cwd|path.display · in src/cmd/verify.rs and src/cmd/verify/ obs_event! calls (none)
@row_add n=841 row=decision/hidden-capture-arms-uninstrumented area=span-trace-coverage rec=fct-f3gi2vhc@2
name: Hidden hook capture arms uninstrumented
rule: The hidden hook capture arms used by verify's probes read no VIOLA_* variable, do no obs init, emit no hook lines, keep stderr empty and always exit 0.
why: uninstrumented by design, as ratified by the founder
@row_check n=841 row=decision/hidden-capture-arms-uninstrumented sha=de6a4ab898a8 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:95-96
quote: capture_statusline(dir, shell, &mut std::io::stdin().lock()); return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:101-106
quote: if let Some(dir) = &args.capture { let filed = capture(event, dir, &mut std::io::stdin().lock()); if let (Some(filed), Some(answers)) = (filed, &args.answers) { answer(event, dir, answers, &filed, &mut std::io::stdout().lock()); } return Ok(ExitCode::SUCCESS);
at: src/cmd/hook.rs:417-418
quote: /// The raw payload into the first free `<dir>/<Event>.<k>.json`, and nothing else: no `VIOLA_*` /// read, no log, no channel. `dir` must be an absolute, existing directory; every failure writes
@row_add n=842 row=decision/real-cli-verify-never-in-ci area=span-trace-coverage rec=fct-f3gi2vhc@2
name: Real-CLI verify never runs in CI
rule: In CI verify runs only against the fake agent; the real-CLI verify runs only through the local live run, which refuses under CI.
@row_check n=842 row=decision/real-cli-verify-never-in-ci sha=1ddedeb9d41c v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run.rs:131-132
quote: if sel.local_live && ci { let doc = json!({"v": 1, "cmd": "run", "ok": false, "reason": "live-in-ci"});
at: crates/viola-e2e/src/harness/run.rs:101
quote: let ci = std::env::var_os("CI").is_some();
@row_add n=843 row=decision/verify-sole-stamps-writer area=span-trace-coverage rec=fct-f3gi2vhc@2
name: verify is the only stamps writer
rule: verify is the only writer of the capability ledger's stamps file.
@row_check n=843 row=decision/verify-sole-stamps-writer sha=cf16d2a35614 v=holds judge=sonnet head=8834195b0683
at: src/cmd/verify.rs:273
quote: let written = update_stamps(home, |existing| {
at: crates/viola-state/src/stamps.rs:57
quote: pub fn update_stamps(
@row_add n=844 row=decision/no-obs-gate-on-verify-lines area=span-trace-coverage rec=fct-f3gi2vhc@2
name: No obs gate depends on verify lines
rule: No observability gate depends on verify's own log lines; its home-level lines fall under the same panic, schema and secret-scan gates as any other.
@row_check n=844 row=decision/no-obs-gate-on-verify-lines sha=830157d62dd3 v=holds judge=sonnet head=8834195b0683
searched: verify-pty-probe|version-probe|agents-probe|cli_verify · in crates/viola-e2e/src (no gate body reads verify's log lines; only the invocations at harness/run.rs:404 and harness/boot.rs:269)
searched: verify · in scripts (only install-ripgrep.sh and agent-run.sh, no log-line gate)
@row_add n=845 row=decision/spans-not-serialized-in-v1 area=telemetry-strategy rec=fct-4joil3ca@1
name: Spans declared but not serialized in v1
rule: Spans are declared on the must-trace paths and every crate seam but are not written to output in v1.
why: they give the code its structure and are ready for the deferred OpenTelemetry bridge
@row_check n=845 row=decision/spans-not-serialized-in-v1 sha=bd3377862525 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:123-124
quote: .with_current_span(false) .with_span_list(false)
@row_add n=846 row=decision/span-naming-skip-all area=telemetry-strategy rec=fct-4joil3ca@1
name: Span declaration convention
rule: Every span is declared with instrument, skip_all and a name of the form "<area>.<operation>".
@row_check n=846 row=decision/span-naming-skip-all sha=92a1f25c7a18 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/stamps.rs:56
quote: #[tracing::instrument(skip_all, name = "state.ledger_write")]
@row_add n=847 row=decision/cross-process-join-corr-conn area=telemetry-strategy rec=fct-4joil3ca@1,fct-474ly2yg@1
name: Cross-process joins by corr and conn
rule: Lines across processes are joined by the corr and conn fields, not by W3C trace IDs.
@row_check n=847 row=decision/cross-process-join-corr-conn sha=cb8c440d3d2a v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:67
quote: #[instrument(skip_all, name = "channel.request", fields(method = method, conn = self.conn.as_str()))]
searched: traceparent|W3C|opentelemetry · in **/*.rs and **/*.toml (only deny.toml bans exporter crates)
@row_add n=848 row=decision/metrics-derived-from-ndjson area=telemetry-strategy rec=fct-6lxvbxmt@1
name: Metrics derived from logs, no instruments
rule: There are no in-process metric instruments and no exporter; metrics are derived by jq over the ndjson logs, and perf gates run on hyperfine JSON exports.
@row_check n=848 row=decision/metrics-derived-from-ndjson sha=b4bb884fc658 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/run/perf.rs:117
quote: .arg("--export-json")
searched: opentelemetry|prometheus|metrics:: · in Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml
@row_add n=849 row=decision/events-stream-never-rotated area=telemetry-strategy rec=fct-mtgjpoqf@1,fct-kytmq3ju@1
name: Event stream never rotated
rule: The instance's events.ndjson is the product audit stream and is never rotated by a generic library.
why: byte offsets in it are the wait-after and send cursors
@row_check n=849 row=decision/events-stream-never-rotated sha=36be13dad19d v=holds judge=sonnet head=8834195b0683
searched: logroller|tracing-appender|rotat · in Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml
@row_add n=850 row=decision/panic-one-line-record area=telemetry-strategy rec=fct-6ukjckx6@1
name: One single-line record a panic
rule: Every panic after obs init produces exactly one single-line structured record in its home-level role file, with location, thread, role and instance; payload and backtrace go only to the instance-scoped detail file.
except: the pre-init window and the paths that refuse or lack an instance or home tree before their role file opens
@row_check n=850 row=decision/panic-one-line-record sha=9918c4f47908 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:191
quote: let _ = (&*sink.file).write_all(line.as_bytes());
at: src/main.rs:201
quote: obs::write_detail(&sink.home, instance, sink.process, &detail);
@row_add n=851 row=decision/hook-silent-without-instance area=telemetry-strategy rec=fct-n6l5o3fm@2
name: Hook silent without an instance
rule: A hook with no VIOLA_NAME, or with a refused or absent instance, exits 0 at once and writes no file, no line and no frame.
@row_check n=851 row=decision/hook-silent-without-instance sha=81fc565e146d v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-117
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS);
@row_add n=852 row=decision/pre-open-strict-modes-refusal-silent area=telemetry-strategy rec=fct-ne5xp5cg@2,fct-4hckhgvs@1
name: Strict-modes refusal before a log opens writes no line
rule: A strict-modes refusal made before any process log opens writes no line and carries no logged detail code; its machine-readable form is the exit code and, under --json, its closed document.
except: a refused socket directory always logs detail socket-dir-refused, except at MCP's frame tool, which writes no line
@row_check n=852 row=decision/pre-open-strict-modes-refusal-silent sha=77dec755717b v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
@row_add n=853 row=contract/strict-modes-refusal-exit/21 area=telemetry-strategy rec=fct-4hckhgvs@1
boundary: strict-modes refusal exit codes
item: 21
promise: The six frame verbs and link / unlink exit 21 on a strict-modes refusal, under --json with the instance-unreachable document carrying detail strict-modes-failed.
@row_check n=853 row=contract/strict-modes-refusal-exit/21 sha=bafaa3559876 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=854 row=contract/strict-modes-refusal-exit/1 area=telemetry-strategy rec=fct-4hckhgvs@1
boundary: strict-modes refusal exit codes
item: 1
promise: run, revive's start and verify exit 1 on a strict-modes refusal; revive under --json prints the strict-modes-failed document.
@row_check n=854 row=contract/strict-modes-refusal-exit/1 sha=e9894042fca7 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:173
quote: return Ok(ExitCode::from(1));
at: src/cmd/verify.rs:69
quote: return Ok(ExitCode::from(1));
at: src/cmd/revive.rs:138
quote: Refusal::StrictModes.write(&args.name, args.json);
@row_add n=855 row=contract/socket-dir-refused-exit/codes area=telemetry-strategy rec=fct-4hckhgvs@1,fct-ne5xp5cg@2
boundary: socket-dir-refused exit codes
item: socket-dir-refused
promise: A refused socket directory logs detail socket-dir-refused with no path, pid or uid, and exits 21 at the eight CLI frame verbs and 1 at run and revive's start.
@row_check n=855 row=contract/socket-dir-refused-exit/codes sha=bd1f4617438f v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:123
quote: Self::SocketDir => UnreachableCause::SocketDir,
at: src/cmd/client.rs:260
quote: exit_code = 21u8,
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/run.rs:412
quote: run::log_self_exit(1, Some(detail));
@row_add n=856 row=decision/no-opentelemetry-in-v1 area=bootstrap-phases-derive-for-route-setup-project rec=fct-7tykupyn@1
name: No OpenTelemetry in v1
rule: No opentelemetry crate is added in v1, and cargo deny bans opentelemetry-otlp, opentelemetry-stdout, sentry and tracing-appender on the workspace.
why: so they cannot arrive transitively without a Decisions Log entry
guard: cargo deny bans
@row_check n=856 row=decision/no-opentelemetry-in-v1 sha=37bef69bd7ed v=holds guard=true judge=sonnet head=8834195b0683
at: deny.toml:32
quote: { crate = "opentelemetry-otlp", reason = "no telemetry exporter or egress in v1" },
at: deny.toml:33
quote: { crate = "opentelemetry-stdout", reason = "no telemetry exporter in v1" },
at: deny.toml:34
quote: { crate = "sentry", reason = "no error reporter or egress in v1" },
at: deny.toml:35
quote: { crate = "tracing-appender", reason = "its non_blocking writer drops lines and pulls a vulnerable time range" },
searched: opentelemetry · in Cargo.toml and crates/*/Cargo.toml (no dependency; only deny.toml names it)
guard_at: deny.toml:28
@row_add n=857 row=decision/subscriber-only-in-root-bin area=bootstrap-phases-derive-for-route-setup-project rec=fct-y2h27pfd@2
name: tracing-subscriber only in the root binary
rule: tracing-subscriber is a product dependency of the root binary only, without its chrono feature; the product crates depend on tracing alone.
except: a library crate may take it as a dev-dependency for in-memory line capture in its own unit tests
@row_check n=857 row=decision/subscriber-only-in-root-bin sha=67283e6f8bd4 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:161
quote: tracing-subscriber.workspace = true
at: Cargo.toml:243
quote: tracing-subscriber = { version = "=0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }
at: crates/viola-channel/Cargo.toml:36
quote: tracing-subscriber.workspace = true
@row_add n=858 row=decision/obs-event-single-macro area=bootstrap-phases-derive-for-route-setup-project rec=fct-y2h27pfd@2,fct-in3nxxob@1
name: One obs_event macro for every line
rule: Every log line is emitted through the one obs_event macro defined in viola-core, which expands to tracing's event at the caller, so viola-core stays I/O-free and tracing-free.
@row_check n=858 row=decision/obs-event-single-macro sha=ba16fa2b98ec v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:138
quote: macro_rules! obs_event {
at: crates/viola-core/src/obs.rs:143
quote: ::tracing::event!(
@row_add n=859 row=decision/direct-chrono-dependency area=bootstrap-phases-derive-for-route-setup-project rec=fct-y2h27pfd@2
name: Direct chrono dependency for timestamps
rule: The root binary takes chrono directly, without default features, to produce the millisecond UTC timestamp.
why: tracing-subscriber's chrono feature does not re-export chrono
@row_check n=859 row=decision/direct-chrono-dependency sha=8aff2158aaca v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:194
quote: chrono = { version = "=0.4.45", default-features = false, features = ["clock", "std"] }
at: Cargo.toml:154
quote: chrono.workspace = true
@row_add n=860 row=decision/raw-tracing-ban area=bootstrap-phases-derive-for-route-setup-project rec=fct-y2h27pfd@2,fct-safgt2vp@1
name: Raw tracing macros banned
rule: The tracing level macros are banned by clippy disallowed-macros and raw event calls by a fail-closed grep outside the obs module, so only obs_event emits lines.
why: clippy reports a disallowed macro expanded inside an exported macro at the caller, so no allow inside obs_event can exempt it
guard: clippy disallowed-macros and scripts/lint-probes.sh
@row_check n=860 row=decision/raw-tracing-ban sha=db4244312a52 v=holds guard=true judge=sonnet head=8834195b0683
at: clippy.toml:5
quote: { path = "tracing::info", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: clippy.toml:6
quote: { path = "tracing::warn", reason = "log through viola_core::obs_event! (obs-plan §3)" },
at: scripts/lint-probes.sh:128
quote: '\bevent!\s*[({[]' .) || rc=$?
guard_at: clippy.toml:4
@row_add n=861 row=decision/single-service-identity-source area=bootstrap-phases-derive-for-route-setup-project rec=fct-lrh7vv4t@1
name: One source for service name and version
rule: The core crate's service name and version constants are the single source for process-start version, sender and writer, with the workspace version shared by every member.
@row_check n=861 row=decision/single-service-identity-source sha=11712c4bf2cf v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:7
quote: pub const SERVICE_NAME: &str = "viola";
at: crates/viola-core/src/lib.rs:8
quote: pub const VERSION: &str = env!("CARGO_PKG_VERSION");
at: src/run/mod.rs:34
quote: version = VERSION,
at: crates/viola-channel/src/client.rs:115
quote: params.insert("sender".to_owned(), VERSION.into());
at: crates/viola-state/src/snapshot.rs:95
quote: writer: VERSION.to_owned(),
at: Cargo.toml:3
quote: version.workspace = true
@row_add n=862 row=decision/diag-schema-matches-event-enum area=bootstrap-phases-derive-for-route-setup-project rec=fct-vwautgwv@1
name: Diagnostic line schema matches the event enum
rule: A hand-maintained JSON Schema of the process-log line is committed, and its event enum must equal the ObsEvent variants, asserted by a tests-owned check.
@row_check n=862 row=decision/diag-schema-matches-event-enum sha=6903b97e90e8 v=holds judge=sonnet head=8834195b0683
at: tests/contract_diag_schema.rs:78-83
quote: fn diag_line_schema_event_enum_equals_the_log_format_literals() { let line = line_schema(); assert_eq!( enum_strings(&line["$defs"]["event"]["enum"]), LOG_FORMAT_EVENTS );
at: crates/viola-core/src/obs.rs:181-182
quote: let shown: Vec<String> = ObsEvent::ALL.iter().map(ToString::to_string).collect(); assert_eq!(shown, LOG_FORMAT_EVENTS);
@row_add n=863 row=decision/liveness-transitions-not-logged area=bootstrap-phases-derive-for-route-setup-project rec=fct-lnkipj6a@1,fct-t7swfgbl@1
name: Liveness transitions not logged
rule: No process-log line carries a liveness transition; liveness is exposed only by readers such as list --json.
why: no process watches another instance's heartbeat
@row_check n=863 row=decision/liveness-transitions-not-logged sha=11495d80a5ff v=holds judge=sonnet head=8834195b0683
searched: liveness|Liveness|heartbeat · in crates/viola-core/src/obs.rs (no ObsEvent variant; the 15 variants listed at lines 13-27 carry none)
@row_add n=864 row=decision/instrument-skip-all-everywhere area=bootstrap-phases-derive-for-route-setup-project rec=fct-vhrcelle@1,fct-gxspxfur@1
name: Spans skip all arguments
rule: Every instrumented function uses skip_all with explicit fields, so no argument is recorded implicitly.
guard: G1
@row_check n=864 row=decision/instrument-skip-all-everywhere sha=9cfad3e7043d v=holds guard=true judge=sonnet head=8834195b0683
at: src/cmd/run.rs:259
quote: #[instrument(skip_all, name = "run.start", fields(instance = name_str(&launch.name)))]
at: src/run/version_gate.rs:142-143
quote: #[instrument( skip_all,
guard_at: .github/workflows/ci.yml:294
@row_add n=865 row=decision/veil-redact-payload-types area=bootstrap-phases-derive-for-route-setup-project rec=fct-vhrcelle@1,fct-gxspxfur@1
name: veil redacts payload types
rule: Payload types derive veil's Redact, and cargo deny bans veil's toggle feature.
guard: cargo deny veil toggle ban
@row_check n=865 row=decision/veil-redact-payload-types sha=3d224f573a66 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-channel/src/frame.rs:51
quote: #[derive(Deserialize, Redact)]
at: deny.toml:39-40
quote: crate = "veil" deny = ["toggle"]
guard_at: deny.toml:38
@row_add n=866 row=decision/fixed-error-display area=bootstrap-phases-derive-for-route-setup-project rec=fct-vhrcelle@1
name: Fixed error Display text
rule: Typed errors have fixed Display text that carries no interpolated content.
@row_check n=866 row=decision/fixed-error-display sha=37ffb53ed5a2 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/lib.rs:34
quote: #[error("channel endpoint is held by another process")]
at: tests/contract_error_bodies.rs:794
quote: fn thiserror_messages_interpolate_nothing() {
@row_add n=867 row=decision/content-only-in-instance-detail-file area=bootstrap-phases-derive-for-route-setup-project rec=fct-vhrcelle@1,fct-r43ltybn@2
name: Content only in instance-scoped detail files
rule: Content-bearing detail goes only to the instance's detail-<process>.ndjson file; home-level log files are codes-only and carry no user content, token, cookie or CLAUDE* value.
why: security allows user-derived content only in instance-scoped diagnostics
@row_check n=867 row=decision/content-only-in-instance-detail-file sha=cd946f2c7c57 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:288-292
quote: pub(crate) fn detail_path(home: &Path, instance: &ViolaName, process: ObsProcess) -> PathBuf { home.join("instances") .join(name_str(instance)) .join("diagnostics") .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=868 row=decision/sha-pinned-ci-actions area=bootstrap-phases-derive-for-route-setup-project rec=fct-safgt2vp@1
name: SHA-pinned CI actions
rule: The ripgrep install step and the upload-artifact action are pinned by SHA.
@row_check n=868 row=decision/sha-pinned-ci-actions sha=4b5649b3864f v=holds judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:80
quote: uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
at: .github/workflows/ci.yml:285
quote: - name: Install ripgrep (PCRE2, checksum-pinned)
at: scripts/install-ripgrep.sh:22
quote: digest() { sha256sum "$1" | cut -d' ' -f1; }
@row_add n=869 row=decision/heartbeat-file-not-logged area=heartbeat-ticks rec=fct-nmtbwule@1
name: Heartbeat file touched, not logged
rule: run touches its instance heartbeat file every second and logs nothing per tick; mcp, hook and short-lived verbs have no tick.
why: a one-second line into a never-rotated file would grow without bound
@row_check n=869 row=decision/heartbeat-file-not-logged sha=9693ad13eded v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/heartbeat.rs:13
quote: pub const BEAT_EVERY: Duration = Duration::from_secs(1);
at: src/cmd/run.rs:616
quote: let beat = Heartbeat::start(instance_dir.to_path_buf());
@row_add n=870 row=contract/list-json/liveness area=heartbeat-ticks rec=fct-t7swfgbl@1
boundary: list --json
item: liveness
promise: list --json exposes each instance's liveness.
@row_check n=870 row=contract/list-json/liveness sha=83f268523d7d v=unverifiable judge=sonnet head=8834195b0683
why: evidence refused
@row_add n=871 row=contract/list-liveness/stale-threshold area=heartbeat-ticks rec=fct-s6vdltl4@1
boundary: list liveness
item: stale threshold
promise: An instance is live while its heartbeat is at most 5 s old and stale or gone beyond that.
@row_check n=871 row=contract/list-liveness/stale-threshold sha=589c9fbae213 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/liveness.rs:16
quote: pub const STALE_AFTER: Duration = Duration::from_secs(5);
at: crates/viola-state/src/liveness.rs:32
quote: Some(age) if age <= STALE_AFTER => Liveness::Live,
@row_add n=872 row=contract/agent-run-status/state-degraded area=heartbeat-ticks rec=fct-s6vdltl4@1
boundary: agent-run status
item: state degraded
promise: The status is degraded when a booted instance's item is stale or absent.
@row_check n=872 row=contract/agent-run-status/state-degraded sha=bf4703a48317 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:37-46
quote: items.iter().any(|item| { item["name"] == name.as_str() && item["wrapped"] == true && item["liveness"] == "live" }) }; Derived { state: if booted.iter().all(live) { "ready" } else { "degraded" },
@row_add n=873 row=contract/diagnostics-files/paths area=log-file-location rec=fct-r43ltybn@2
boundary: diagnostics files
item: process log paths
promise: Process logs are written to <home>/diagnostics/ as run-<name>, hook-<name>, mcp and cli-<name> .ndjson files, and detail to <home>/instances/<name>/diagnostics/detail-<process>.ndjson.
@row_check n=873 row=contract/diagnostics-files/paths sha=3ef91aa341c6 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:87
quote: let dir = home.join("diagnostics");
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name))
at: src/obs.rs:56
quote: (ObsProcess::Mcp, _) => "mcp".to_owned(),
at: src/obs.rs:289-292
quote: home.join("instances") .join(name_str(instance)) .join("diagnostics") .join(format!("detail-{}.ndjson", process.as_str()))
@row_add n=874 row=contract/diagnostics-files/home-resolution area=log-file-location rec=fct-r43ltybn@2
boundary: diagnostics files
item: home resolution
promise: The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
@row_check n=874 row=contract/diagnostics-files/home-resolution sha=fd698a8e7e66 v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:294
quote: if let Some(home) = flag {
at: src/cmd/mod.rs:297
quote: if let Some(instance) = instance_of(name, dir) {
at: src/cmd/mod.rs:301
quote: .map(|user_home| default_home(&user_home))
at: src/cmd/mod.rs:242
quote: user_home.join(".viola")
@row_add n=875 row=decision/log-files-owner-only area=log-file-location rec=fct-r43ltybn@2
name: Owner-only log files
rule: Log files are created 0600 inside 0700 directories.
guard: secret-scan test
@row_check n=875 row=decision/log-files-owner-only sha=3b7d93f408e7 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:48
quote: std::os::unix::fs::OpenOptionsExt::mode(&mut options, FILE_MODE);
at: crates/viola-state/src/fs.rs:10
quote: pub const DIR_MODE: u32 = 0o700;
guard_at: crates/viola-e2e/src/harness/secret_scan.rs:57
@row_add n=876 row=decision/no-log-rotation-in-v1 area=log-file-location rec=fct-kytmq3ju@1
name: No log rotation in v1
rule: Diagnostics are not rotated and no rotation library is used; retention is the lifetime of the session home.
@row_check n=876 row=decision/no-log-rotation-in-v1 sha=cf3566cccee2 v=holds judge=sonnet head=8834195b0683
searched: logroller · in Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml
searched: rotat (case-insensitive) · in Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs, crates/*/Cargo.toml
@row_add n=877 row=decision/tracing-appender-rejected area=log-file-location rec=fct-kytmq3ju@1
name: tracing-appender rejected
rule: tracing-appender is not used as a log writer.
why: its non_blocking writer drops lines, and it pulls in an advisory-affected time crate
@row_check n=877 row=decision/tracing-appender-rejected sha=3b8996cafea6 v=holds judge=sonnet head=8834195b0683
searched: tracing-appender · in Cargo.toml, Cargo.lock, crates/*/Cargo.toml, src/**/*.rs, crates/*/src/**/*.rs
searched: tracing_appender · in src/**/*.rs, crates/*/src/**/*.rs
@row_add n=878 row=decision/obs-aligns-to-tests-log-format area=log-format-json-schema rec=fct-5k3podjh@1,fct-evvm5jqh@1
name: Observability aligns to the tests log format
rule: The log format is bound by the test plan's harness contract; observability may add fields but must not rename or remove its fields.
why: the harness greps on them
@row_check n=878 row=decision/obs-aligns-to-tests-log-format sha=8f6845e2fa8b v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:122-123
quote: .flatten_event(true)         .with_current_span(false)
at: src/obs.rs:96
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=879 row=contract/event-stream/line area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the event stream
item: event line
promise: Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
@row_check n=879 row=contract/event-stream/line sha=cdc462ebbecb v=stale judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:25-28
quote: pub enum Source {     Wrapper,     Hook, }
shows: The event line's source enum has only wrapper and hook; no code produces source "cli".
cell: promise
value: Each line of instances/<name>/events.ndjson is exactly {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
@row_add n=880 row=contract/process-log-line/timestamp area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the process log line
item: timestamp
promise: RFC 3339 UTC with milliseconds and Z.
@row_check n=880 row=contract/process-log-line/timestamp sha=73b788e12250 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:127
quote: .with_timer(MillisUtc)
at: src/obs.rs:96
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
@row_add n=881 row=contract/process-log-line/level area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the process log line
item: level
promise: One of DEBUG, INFO, WARN, ERROR.
@row_check n=881 row=contract/process-log-line/level sha=c3c32f8d7a35 v=holds judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:10
quote: "level": { "enum": ["DEBUG", "INFO", "WARN", "ERROR"] },
at: src/obs.rs:121
quote: .json()
@row_add n=882 row=contract/process-log-line/target area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the process log line
item: target
promise: The target is always present.
@row_check n=882 row=contract/process-log-line/target sha=ffe98a810beb v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:120-123
quote: let layer = tracing_subscriber::fmt::layer() .json() .flatten_event(true) .with_current_span(false)
@row_add n=883 row=contract/process-log-line/message area=log-format-json-schema rec=fct-evvm5jqh@1,fct-in3nxxob@1
boundary: the process log line
item: message
promise: The message is a static literal equal to the event name and never carries interpolated content.
@row_check n=883 row=contract/process-log-line/message sha=a08e9382fcae v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:149-150
quote: "{}", event.as_str()
@row_add n=884 row=contract/process-log-line/process area=log-format-json-schema rec=fct-evvm5jqh@1,fct-m6rejt2n@1
boundary: the process log line
item: process
promise: One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing cli-<name>.ndjson.
@row_check n=884 row=contract/process-log-line/process sha=dda2fa758881 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:86
quote: pub const ALL: [Self; 4] = [Self::Run, Self::Hook, Self::Mcp, Self::Cli];
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name))
@row_add n=885 row=contract/process-log-line/instance area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: the process log line
item: instance
promise: The instance's name, or absent when none resolved.
@row_check n=885 row=contract/process-log-line/instance sha=a73123fc4e36 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:147
quote: instance = $crate::obs::ctx_instance(),
@row_add n=886 row=contract/process-log-line/corr area=log-format-json-schema rec=fct-evvm5jqh@1,fct-in3nxxob@1
boundary: the process log line
item: corr
promise: Copied unchanged as a JSON number or string, never renamed and never debug- or display-formatted; required only on the events whose rules always define it.
@row_check n=886 row=contract/process-log-line/corr sha=3570a5ec7df6 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:148
quote: $($key = $value,)*
at: src/run/dialog.rs:315
quote: corr = dialog_id,
@row_add n=887 row=contract/corr-by-event/channel area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: channel-request and channel-response
promise: corr is the JSON-RPC request id; null for an id-less hook.event notification and on a response to a frame that yielded no id.
@row_check n=887 row=contract/corr-by-event/channel sha=2a16c0062948 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:206
quote: log_request(request.id, peer, method, &params, &request.params);
at: crates/viola-channel/src/server.rs:185
quote: let mut logged = ResponseLine::new(started, id, Peer::Server(srv_conn), None);
at: crates/viola-channel/src/server.rs:476
quote: corr = self.corr,
at: crates/viola-channel/src/client.rs:183
quote: corr = id,
@row_add n=888 row=contract/corr-by-event/dialog area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: dialog-raised and dialog-answered
promise: corr is the dialog_id.
@row_check n=888 row=contract/corr-by-event/dialog sha=e9b96489d18e v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:315
quote: corr = dialog_id,
at: src/run/dialog.rs:461
quote: corr = dialog_id,
@row_add n=889 row=contract/corr-by-event/hook area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: hook-invoked and hook-decision
promise: corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked before the id is known, or a hook-decision carrying detail, may be null.
@row_check n=889 row=contract/corr-by-event/hook sha=f9a22cf8d3a6 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:369
quote: corr = dialog_id,
at: src/cmd/hook.rs:401
quote: corr = dialog_id,
at: src/cmd/hook.rs:391
quote: ObsEvent::HookDecision,
@row_add n=890 row=contract/corr-by-event/send area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: send-issued, send-confirmed, send-refused
promise: corr is the send cursor; a wrapper refusal before send-issued carries the target's event-stream end offset, never returned to the caller; wrapper send lines carry conn and rpc_id; the client-side send-refused has corr null.
@row_check n=890 row=contract/corr-by-event/send sha=7e4dd0a909e2 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:196
quote: corr = cursor,
at: src/run/send.rs:233
quote: corr = cursor,
at: src/run/send.rs:275-278
quote: let corr = match cursor { Some(cursor) => Some(cursor), None => end_offset(self.instance_dir).ok(), };
at: src/run/send.rs:284-285
quote: rpc_id = self.call.id, conn = self.call.conn,
at: src/cmd/send.rs:240
quote: side = "client",
@row_add n=891 row=contract/corr-by-event/release-from-driver area=log-format-json-schema rec=fct-evvm5jqh@1,fct-ufgywcj6@1
boundary: corr by event
item: release-from-driver
promise: corr is the JSON-RPC request id of the release refused with -32602 for carrying a string from.
@row_check n=891 row=contract/corr-by-event/release-from-driver sha=30d425606b77 v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:204-206
quote: if let Some(Value::String(from)) = params.get("from") { log_release_from_driver(call, from); return Err(ProtocolError::ReleaseFromDriver);
at: src/run/wheel.rs:241
quote: corr = call.id,
@row_add n=892 row=contract/corr-by-event/process-and-panic area=log-format-json-schema rec=fct-evvm5jqh@1
boundary: corr by event
item: process-start, process-exit, panic
promise: corr is null.
@row_check n=892 row=contract/corr-by-event/process-and-panic sha=c4d6c621325b v=holds judge=sonnet head=8834195b0683
at: src/main.rs:245
quote: "event": "panic",
at: src/run/mod.rs:31
quote: ObsEvent::ProcessStart,
at: src/obs.rs:238
quote: ObsEvent::ProcessExit,
@row_add n=893 row=contract/corr-by-event/state-recovered area=log-format-json-schema rec=fct-evvm5jqh@1,fct-ufgywcj6@1
boundary: corr by event
item: state-recovered
promise: corr is null.
@row_check n=893 row=contract/corr-by-event/state-recovered sha=cd3c5b39f5fd v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/events.rs:338-339
quote: ObsEvent::StateRecovered, detail = "torn-line-healed",
at: crates/viola-state/src/replay.rs:165-166
quote: ObsEvent::StateRecovered, detail = detail,
@row_add n=894 row=contract/corr-by-event/parse-rejected area=log-format-json-schema rec=fct-evvm5jqh@1,fct-ufgywcj6@1
boundary: corr by event
item: parse-rejected
promise: corr is null.
@row_check n=894 row=contract/corr-by-event/parse-rejected sha=4c3162dd0780 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:223-224
quote: ObsEvent::ParseRejected, parser = "config-json",
at: src/run/gate.rs:80-81
quote: ObsEvent::ParseRejected, parser = "vt100-feed",
@row_add n=895 row=decision/null-as-key-absence area=log-format-json-schema rec=fct-evvm5jqh@1,fct-zf2424ri@1
name: Null written as key absence
rule: A null corr or instance is written as an absent key, readers treat absence and null as equal, and the schema rejects a literal null.
why: tracing has no null field value
@row_check n=895 row=decision/null-as-key-absence sha=145f4f16d6dd v=holds judge=sonnet head=8834195b0683
at: src/main.rs:250-251
quote: if let Some(instance) = instance { record["instance"] = serde_json::Value::from(instance);
at: schemas/diag-line.v1.json:111
quote: "corr": { "type": ["number", "string"] },
at: schemas/diag-line.v1.json:15
quote: "instance": { "type": "string", "pattern": "^[a-z][a-z0-9-]{0,31}$" }
@row_add n=896 row=decision/no-multiline-stack-traces area=log-format-json-schema rec=fct-evvm5jqh@1
name: No multi-line stack traces
rule: Every log line is one physical line parseable by jq; a hook panic is caught and logged as one ERROR line with event panic.
@row_check n=896 row=decision/no-multiline-stack-traces sha=89f8a0b65192 v=holds judge=sonnet head=8834195b0683
at: src/main.rs:253-254
quote: let mut line = record.to_string();     line.push('\n');
at: src/main.rs:245
quote: "event": "panic",
at: src/main.rs:242
quote: "level": "ERROR",
@row_add n=897 row=decision/tracing-logging-library area=logging-stack rec=fct-in3nxxob@1
name: tracing as the logging library
rule: tracing is the logging facade and tracing-subscriber's JSON formatter writes the lines.
why: Tokio-free and no C, and meets the security floor for tracing-subscriber
@row_check n=897 row=decision/tracing-logging-library sha=bc0266847f28 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:242
quote: tracing = { version = "=0.1.44", default-features = false, features = ["attributes", "std"] }
at: Cargo.toml:243
quote: tracing-subscriber = { version = "=0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }
at: src/obs.rs:121
quote: .json()
@row_add n=898 row=contract/obs-event-macro/attached-fields area=logging-stack rec=fct-in3nxxob@1
boundary: the obs_event macro
item: attached fields
promise: Every line carries event from the ObsEvent enum's kebab name, plus process and instance from the process context, each absent when it has no value.
@row_check n=898 row=contract/obs-event-macro/attached-fields sha=2236f9da8e84 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:145
quote: event = event.as_str(),
at: crates/viola-core/src/obs.rs:146
quote: process = $crate::obs::ctx_process(),
at: crates/viola-core/src/obs.rs:147
quote: instance = $crate::obs::ctx_instance(),
@row_add n=899 row=contract/diagnostics-line/format area=logging-stack rec=fct-i4z6jsd7@2
boundary: the diagnostics line
item: format
promise: Each diagnostics line is one structured JSON object on one line, written whole by one write of the complete line on an append-mode handle.
@row_check n=899 row=contract/diagnostics-line/format sha=3a23f2b531ba v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:120-128
quote: let layer = tracing_subscriber::fmt::layer() .json() .flatten_event(true) .with_current_span(false) .with_span_list(false) .with_ansi(false) .log_internal_errors(false) .with_timer(MillisUtc) .with_writer(writer);
at: crates/viola-state/src/fs.rs:46
quote: options.create(true).append(true);
at: src/obs.rs:332
quote: let _ = file.write_all(line.as_bytes());
@row_add n=900 row=contract/diagnostics-line/role-file-line-size area=logging-stack rec=fct-i4z6jsd7@2
boundary: the diagnostics line
item: role-file line size
promise: A line in a role file (run, hook, mcp, cli) is codes-only and stays well under 4 KiB.
@row_check n=900 row=contract/diagnostics-line/role-file-line-size sha=c0022f691591 v=unverifiable judge=sonnet head=8834195b0683
searched: 4096 · in src/obs.rs, src/main.rs, schemas/diag-line.v1.json, tests/contract_diag_schema.rs
searched: maxLength · in schemas/diag-line.v1.json
why: no evidence found
@row_add n=901 row=contract/diagnostics-line/detail-line-size area=logging-stack rec=fct-i4z6jsd7@2
boundary: the diagnostics line
item: detail-file line size
promise: A line in an instance detail file (chains, drift reports, one-line backtraces) is not codes-only and may exceed 4 KiB.
@row_check n=901 row=contract/diagnostics-line/detail-line-size sha=fd94ec5a4134 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:332
quote: let _ = file.write_all(line.as_bytes());
@row_add n=902 row=decision/shared-log-files-concurrent-append area=logging-stack rec=fct-i4z6jsd7@2
name: Concurrent appends to shared diagnostics files
rule: Every diagnostics file may have concurrent writers in separate processes; each line is one whole write on an append-mode handle, readers tolerate a torn line, and a concurrent-append check, including a detail line over 4 KiB, asserts every line parses.
why: Non-interleaving at this size is an assumption, not a POSIX guarantee, because the 4 KiB PIPE_BUF rule applies to pipes.
guard: concurrent-append check; hook_panics_append_whole_lines_over_4_kib_side_by_side
@row_check n=902 row=decision/shared-log-files-concurrent-append sha=bc045716ad2c v=holds guard=true judge=sonnet head=8834195b0683
at: src/main.rs:191
quote: let _ = (&*sink.file).write_all(line.as_bytes());
at: src/obs.rs:332
quote: let _ = file.write_all(line.as_bytes());
at: src/obs.rs:49-51
quote: fn open_private_append(path: &Path) -> io::Result<File> { viola_state::fs::open_private_append(path) }
guard_at: tests/hook_fail_open.rs:792
@row_add n=903 row=contract/agent-run-logs/torn-line area=logging-stack rec=fct-i4z6jsd7@2,fct-v725cwjk@2
boundary: agent-run logs output
item: torn line
promise: A torn line appears as {"torn":true,"offset":n}, and readers skip it rather than fail.
@row_check n=903 row=contract/agent-run-logs/torn-line sha=c8f0263c7f49 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:186-187
quote: out.insert("torn".to_owned(), true.into()); out.insert("offset".to_owned(), at.into());
at: crates/viola-e2e/src/harness/schema_check.rs:115-116
quote: let Ok(line) = serde_json::from_str::<Value>(raw) else { torn += 1;
@row_add n=904 row=decision/no-internal-error-fallback area=logging-stack rec=fct-pbscnpyo@1
name: No stderr fallback for subscriber internal errors
rule: The diagnostics subscriber never reports its own internal errors by printing to stderr.
why: The default falls back to eprintln, which would break the Hook contract and the run terminal rule.
@row_check n=904 row=decision/no-internal-error-fallback sha=484cbbf2311a v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:126
quote: .log_internal_errors(false)
@row_add n=905 row=decision/no-span-list-in-lines area=logging-stack rec=fct-pbscnpyo@1
name: No span list in diagnostics lines
rule: Diagnostics lines never carry the list of entered spans.
why: Closes the spans leak path.
@row_check n=905 row=decision/no-span-list-in-lines sha=5c04c122aa37 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:124
quote: .with_span_list(false)
@row_add n=906 row=decision/explicit-log-writer area=logging-stack rec=fct-pbscnpyo@1
name: Explicit diagnostics writer
rule: The diagnostics subscriber always has its writer set explicitly.
why: The default writer is stdout.
@row_check n=906 row=decision/explicit-log-writer sha=baac233a74e4 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:128
quote: .with_writer(writer);
@row_add n=907 row=decision/no-ansi-in-logs area=logging-stack rec=fct-pbscnpyo@1
name: No ANSI codes in diagnostics lines
rule: Diagnostics lines carry no ANSI colour codes.
@row_check n=907 row=decision/no-ansi-in-logs sha=4e1ef39acd6f v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:125
quote: .with_ansi(false)
@row_add n=908 row=decision/no-source-paths-in-logs area=logging-stack rec=fct-pbscnpyo@1
name: No source paths in diagnostics lines
rule: Diagnostics lines carry no source file names or line numbers.
@row_check n=908 row=decision/no-source-paths-in-logs sha=f0b919b91ba9 v=holds judge=sonnet head=8834195b0683
searched: .with_file( · in src/obs.rs (builder at lines 120-128 sets no file or line option; only unrelated with_file_name elsewhere)
searched: with_line_number · in **/*.rs
@row_add n=909 row=decision/file-only-diagnostics-sinks area=logging-stack rec=fct-zlgd74pd@1
name: File-only diagnostics, one file per role
rule: Every role logs only to its own file under the home's diagnostics directory; no role has a stdout sink, and there is no pretty or dev sink.
@row_check n=909 row=decision/file-only-diagnostics-sinks sha=598adb88bd91 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:92
quote: Ok(Arc::new(open_private_append(&path)?))
at: src/obs.rs:145
quote: let _ = tracing::subscriber::set_global_default(subscriber(file, level));
@row_add n=910 row=contract/diagnostics-files/detail area=logging-stack rec=fct-zlgd74pd@1
boundary: the diagnostics files
item: detail
promise: Content-bearing records (full error chains, drift reports, panic payload and backtrace) go only to <home>/instances/<name>/diagnostics/detail-<process>.ndjson, file 0600 in a 0700 directory, and only when an instance resolves.
@row_check n=910 row=contract/diagnostics-files/detail sha=eb9da6121fb4 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:288-293
quote: pub(crate) fn detail_path(home: &Path, instance: &ViolaName, process: ObsProcess) -> PathBuf { home.join("instances") .join(name_str(instance)) .join("diagnostics") .join(format!("detail-{}.ndjson", process.as_str()))
at: src/obs.rs:328
quote: if check_diagnostics(dir, &path).is_err() || ensure_private_dir(dir).is_err() {
at: src/obs.rs:331
quote: if let Ok(mut file) = open_private_append(&path) {
@row_add n=911 row=decision/content-records-bypass-subscriber area=logging-stack rec=fct-zlgd74pd@1
name: Content-bearing records bypass the subscriber
rule: Content-bearing records never pass through the tracing subscriber, whose one writer holds only the home-level role file; the detail writer formats each detail line itself and writes it with one whole write.
@row_check n=911 row=decision/content-records-bypass-subscriber sha=556564f058ca v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:297
quote: pub(crate) fn detail_line(
at: src/obs.rs:315
quote: let mut line = Value::Object(record).to_string();
at: src/obs.rs:332
quote: let _ = file.write_all(line.as_bytes());
@row_add n=912 row=decision/no-content-in-home-level-lines area=logging-stack rec=fct-zlgd74pd@1
name: No content fields in home-level lines
rule: No home-level diagnostics line carries chain, drift_report, panic_payload or backtrace.
guard: G4 diag-line.v1.json schema
@row_check n=912 row=decision/no-content-in-home-level-lines sha=32b5ca17373b v=holds guard=true judge=sonnet head=8834195b0683
at: schemas/diag-line.v1.json:106
quote: "unevaluatedProperties": false,
guard_at: schemas/diag-line.v1.json:106
@row_add n=913 row=decision/no-agent-mode-flag area=logging-stack rec=fct-brueynpb@3
name: No agent-mode flag
rule: There is no agent-mode flag; JSON to file is the only logging mode.
@row_check n=913 row=decision/no-agent-mode-flag sha=22f45ab4abdd v=holds judge=sonnet head=8834195b0683
searched: agent-mode|agent_mode|--agent · in src/**/*.rs, Cargo.toml, deny.toml
@row_add n=914 row=contract/config-json/diagnostics-level area=logging-stack rec=fct-brueynpb@3,fct-6bbewo2y@2
boundary: config.json
item: diagnostics_level
promise: Key diagnostics_level takes "info" (the default) or "debug"; a missing or unreadable file or an unknown value falls back to "info".
@row_check n=914 row=contract/config-json/diagnostics-level sha=f8ee58df0c44 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:202-204
quote: None | Some(Some("info")) => (Level::INFO, false), Some(Some("debug")) => (Level::DEBUG, false), Some(_) => (Level::INFO, true),
at: src/obs.rs:185-186
quote: Err(e) if e.kind() == io::ErrorKind::NotFound => (Level::INFO, None), Err(_) => (Level::INFO, Some(ConfigRejection::Unreadable)),
@row_add n=915 row=decision/log-level-never-from-env area=logging-stack rec=fct-brueynpb@3
name: Log level never from the environment
rule: The diagnostics level is never read from RUST_LOG and the env-filter feature is never enabled; the level comes only from config.json.
@row_check n=915 row=decision/log-level-never-from-env sha=5a772a75b22e v=holds judge=sonnet head=8834195b0683
at: deny.toml:50
quote: deny = ["env-filter"]
at: src/obs.rs:201
quote: let (level, bad_value) = match map.get("diagnostics_level").map(Value::as_str) {
@row_add n=916 row=decision/level-changes-volume-only area=logging-stack rec=fct-brueynpb@3
name: Level changes volume only
rule: Every diagnostics level uses the same field allow-list, so the level changes only how many lines are written.
why: No setting can disable redaction.
@row_check n=916 row=decision/level-changes-volume-only sha=b91c6faa0c32 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:107
quote: fn targets(level: Level) -> Targets {
at: src/obs.rs:111
quote: t.with_target(*name, level)
@row_add n=917 row=decision/third-party-targets-off area=logging-stack rec=fct-brueynpb@3
name: Third-party log targets off
rule: Records of third-party targets (rmcp, portable_pty, sysinfo, interprocess) are off in every sink; their failures are captured at viola's seam from the returned results.
why: Their records lack the required fields.
@row_check n=917 row=decision/third-party-targets-off sha=58c0e28e442f v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:110
quote: .fold(Targets::new().with_default(LevelFilter::OFF), |t, name| {
@row_add n=918 row=decision/no-log-crate-bridge area=logging-stack rec=fct-brueynpb@3
name: No log-crate bridge
rule: The log-crate bridge (tracing-log, LogTracer) stays off; failures are captured from returned values or handles, and a gap is fixed at the seam or by the researched json-subscriber fallback, never by enabling the bridge.
why: The accepted loss is only third-party log context emitted without a returned error.
@row_check n=918 row=decision/no-log-crate-bridge sha=fa3e952bc261 v=holds guard=false judge=sonnet head=8834195b0683
at: Cargo.toml:243
quote: tracing-subscriber = { version = "=0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }
@row_add n=919 row=decision/no-otel-sdk-v1 area=otel-sdk-init rec=fct-56r4goln@1,fct-rrjwwqcp@2
name: No OpenTelemetry SDK in v1
rule: No OpenTelemetry SDK, bridge or Resource is used in v1.
why: With no exporter they would add only dependency and cargo-mutants surface, and bridged span IDs would have no consumer.
@row_check n=919 row=decision/no-otel-sdk-v1 sha=8ae844a86dc1 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry · in **/*.rs and **/Cargo.toml (only deny.toml bans exporter crates)
searched: Resource::builder · in **/*.rs and **/Cargo.toml
@row_add n=920 row=decision/tracing-is-the-instrumentation-facade area=otel-sdk-init rec=fct-56r4goln@1
name: tracing as the instrumentation facade
rule: tracing is the instrumentation facade in every instrumentable crate, and tracing-subscriber is the formatter and sink, a product dependency of the root bin only.
@row_check n=920 row=decision/tracing-is-the-instrumentation-facade sha=4f01c0784fa0 v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:160
quote: tracing.workspace = true
at: Cargo.toml:161
quote: tracing-subscriber.workspace = true
at: crates/viola-state/Cargo.toml:31
quote: tracing-subscriber.workspace = true
@row_add n=921 row=decision/otel-resource-built-empty area=otel-sdk-init rec=fct-56r4goln@1,fct-rrjwwqcp@2
name: OTel Resource built empty if ever adopted
rule: If an OTel SDK is ever adopted, its Resource is built empty with viola's own identity values, never with the default builder.
why: The default builder's environment detector reads OTEL_* env vars.
@row_check n=921 row=decision/otel-resource-built-empty sha=8fe3629cab5a v=unverifiable judge=sonnet head=8834195b0683
why: not a code claim
@row_add n=922 row=decision/tracing-subscriber-minimal-features area=otel-sdk-init rec=fct-56r4goln@1
name: Minimal tracing-subscriber features
rule: tracing-subscriber is taken with default features off and only fmt, json, registry and std; ansi, tracing-log and chrono stay off.
why: The chrono feature only enables ChronoUtc / ChronoLocal, and the millisecond timer uses the root bin's direct chrono dependency.
@row_check n=922 row=decision/tracing-subscriber-minimal-features sha=43de8dc4ac3c v=holds judge=sonnet head=8834195b0683
at: Cargo.toml:243
quote: tracing-subscriber = { version = "=0.3.23", default-features = false, features = ["fmt", "json", "registry", "std"] }
@row_add n=923 row=decision/panic-hook-first area=otel-sdk-init rec=fct-6bbewo2y@2
name: Panic hook installed first
rule: The panic hook is installed as the first statement of main and writes only to the diagnostics file once that is open, never falling back to stderr.
@row_check n=923 row=decision/panic-hook-first sha=421182e9223b v=holds judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
at: src/main.rs:161-163
quote: let Some(sink) = PANIC_SINK.get() else { return; };
@row_add n=924 row=decision/home-resolution-order area=otel-sdk-init rec=fct-6bbewo2y@2
name: Home resolution order
rule: The home is resolved from --home, then the grandparent of VIOLA_DIR, then ~/.viola/.
@row_check n=924 row=decision/home-resolution-order sha=e1a00a20446d v=holds judge=sonnet head=8834195b0683
at: src/cmd/mod.rs:294
quote: if let Some(home) = flag {
at: src/cmd/mod.rs:297
quote: if let Some(instance) = instance_of(name, dir) {
at: src/cmd/mod.rs:301
quote: .map(|user_home| default_home(&user_home))
@row_add n=925 row=decision/strict-modes-before-diagnostics area=otel-sdk-init rec=fct-6bbewo2y@2
name: Home strict-modes check before diagnostics
rule: Nothing under the home, the diagnostics directory included, is created or opened before the role's strict-modes check on the home passes; a refused home gets no diagnostics line.
@row_check n=925 row=decision/strict-modes-before-diagnostics sha=e8199986f3a1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/mcp.rs:22
quote: if !home_passes(home) {
@row_add n=926 row=decision/lazy-detail-file area=otel-sdk-init rec=fct-6bbewo2y@2
name: Detail file opened lazily
rule: The instance detail file is opened only by the first content-bearing record or by the panic hook, and a failed detail open drops only the detail line, writing nothing to stderr.
why: The error-free hook path keeps exactly one log open.
@row_check n=926 row=decision/lazy-detail-file sha=80aff91e71a2 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:323
quote: pub(crate) fn write_detail(home: &Path, instance: &ViolaName, process: ObsProcess, line: &str) {
at: src/obs.rs:328
quote: if check_diagnostics(dir, &path).is_err() || ensure_private_dir(dir).is_err() {
at: src/obs.rs:331
quote: if let Ok(mut file) = open_private_append(&path) {
at: src/main.rs:201
quote: obs::write_detail(&sink.home, instance, sink.process, &detail);
@row_add n=927 row=contract/config-json/five-hour area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: config.json
item: five_hour
promise: Key five_hour holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.
@row_check n=927 row=contract/config-json/five-hour sha=face658e261f v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:85
quote: five_hour: threshold(FIVE_HOUR_KEY, FIVE_HOUR_DEFAULT),
at: src/run/budget.rs:79-81
quote: Some(None) => { rejections.push(ConfigRejection::Malformed); compiled(default)
at: src/obs.rs:224
quote: parser = "config-json",
@row_add n=928 row=contract/config-json/seven-day area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: config.json
item: seven_day
promise: Key seven_day holds a whole percentage from 0 to 100; any other value is logged as one parse-rejected{parser:"config-json", detail:"malformed"} WARN line.
@row_check n=928 row=contract/config-json/seven-day sha=c901eb1de986 v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:86
quote: seven_day: threshold(SEVEN_DAY_KEY, SEVEN_DAY_DEFAULT),
at: src/run/budget.rs:79-81
quote: Some(None) => { rejections.push(ConfigRejection::Malformed); compiled(default)
at: src/obs.rs:224
quote: parser = "config-json",
@row_add n=929 row=contract/config-json/v area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: config.json
item: v
promise: A document that is no object, or whose v is not 1, gives the threshold and hold defaults and no further line.
@row_check n=929 row=contract/config-json/v sha=9426c0f2f48b v=holds judge=sonnet head=8834195b0683
at: src/run/budget.rs:72-73
quote: if map.get("v").and_then(Value::as_u64) != Some(1) { return (Thresholds::default(), Vec::new());
at: src/run/dialog.rs:174-175
quote: if map.get("v").and_then(Value::as_u64) != Some(1) { return (DialogHold::default(), Vec::new());
@row_add n=930 row=contract/config-json/known-keys area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: config.json
item: known keys
promise: config.json has six known keys: v, diagnostics_level, claude_env_keep, five_hour, seven_day, dialog_hold_s; any other key is counted as skipped.
@row_check n=930 row=contract/config-json/known-keys sha=e4bbec38e58c v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:32-39
quote: const CONFIG_KEYS: [&str; 6] = [ "v", "diagnostics_level", "claude_env_keep", "five_hour", "seven_day", "dialog_hold_s", ];
at: src/obs.rs:206-209
quote: let unknown = map .keys() .filter(|k| !CONFIG_KEYS.contains(&k.as_str())) .count();
@row_add n=931 row=contract/diagnostics-events/config-parse-rejected area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: diagnostics events
item: parse-rejected config-json
promise: Right after process-start, a config.json parse that failed or skipped keys logs parse-rejected{parser:"config-json", detail, count}.
@row_check n=931 row=contract/diagnostics-events/config-parse-rejected sha=d9d45744ced2 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:226-228
quote: run::log_self_start(); if let Some(rejection) = rejection { obs::log_config_rejection(rejection);
at: src/obs.rs:224
quote: parser = "config-json",
@row_add n=932 row=decision/diagnostics-init-before-role-work area=otel-sdk-init rec=fct-6bbewo2y@2
name: Diagnostics init before the role's work
rule: Diagnostics init and the process-start line come before the role's work: in run before its collision check and the child spawn, in mcp before its stdio transport starts.
why: Exit-1 causes are logged.
@row_check n=932 row=decision/diagnostics-init-before-role-work sha=9b2153e6e97d v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:176
quote: let persistent = open_wrapper_log(home, &args.name)?;
at: src/cmd/run.rs:187
quote: match start(home, &launch, &persistent, &exe)? {
at: src/cmd/mcp.rs:27
quote: obs::viola_obs_init(home, ObsProcess::Mcp, own_name.clone(), level)?;
at: src/cmd/mcp.rs:36
quote: viola_mcp::serve(home.to_path_buf(), own_name, board)?;
@row_add n=933 row=decision/instance-check-before-log-init area=otel-sdk-init rec=fct-6bbewo2y@2
name: Instance check before log init
rule: Commands acting on an instance (revive's start arm, the hook arms, link, unlink, send, wait, last, answer, pause, release) run the instance strict-modes check before opening any log; a refusal opens and writes nothing and leaves modes as found.
why: Opening the role file narrows an existing home to 0700, so a check made after it passes a home at 0770.
except: The two hook --capture arms run ahead of the check.
@row_check n=933 row=decision/instance-check-before-log-init sha=e716697fa608 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else {
at: src/cmd/client.rs:31
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
at: src/cmd/revive.rs:137
quote: if !trusted(home, &args.name)? {
at: src/cmd/hook.rs:101
quote: if let Some(dir) = &args.capture {
@row_add n=934 row=decision/home-tree-check-before-log-init area=otel-sdk-init rec=fct-6bbewo2y@2
name: Home tree check before log init
rule: run, revive's start arm and viola verify check the home tree's modes before their log opens; a refusal writes no line, starts no child, leaves modes as found and exits 1.
@row_check n=934 row=decision/home-tree-check-before-log-init sha=e21d0e8e2910 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:169
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/run.rs:172
quote: refuse_tree(&args.name, refused);
at: src/cmd/revive.rs:141
quote: let exe = match checked_exe(home, &args.name)? {
at: src/cmd/verify.rs:67-69
quote: if check_home_tree(&home, None, None).is_err() { refuse_home(); return Ok(ExitCode::from(1));
@row_add n=935 row=decision/hook-single-log-open area=otel-sdk-init rec=fct-6bbewo2y@2
name: One config read and one log open per hook
rule: A hook invocation makes one config.json read and exactly one log open, then only appends.
@row_check n=935 row=decision/hook-single-log-open sha=1b75d1537024 v=holds guard=false judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:119
quote: let (level, _) = obs::read_diagnostics_level(&instance.home);
at: src/cmd/hook.rs:122
quote: let _ = obs::viola_obs_init(
@row_add n=936 row=decision/hook-refusal-exits-zero area=otel-sdk-init rec=fct-6bbewo2y@2
name: Refused hook fails open
rule: A hook whose instance is refused or absent exits 0 at once, reading nothing under the home and no stdin, writing no frame and no log line.
@row_check n=936 row=decision/hook-refusal-exits-zero sha=fa590d5de2f4 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:116-118
quote: let Some(instance) = instance.as_ref().and_then(canonical).filter(trusted) else { return Ok(ExitCode::SUCCESS); };
@row_add n=937 row=contract/cli-exit-codes/21 area=otel-sdk-init rec=fct-6bbewo2y@2
boundary: CLI exit codes
item: 21
promise: An instance strict-modes refusal exits 21 with the fixed instance-state pair, or under --json the document {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}.
@row_check n=937 row=contract/cli-exit-codes/21 sha=73dada93ef4c v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:35
quote: unreachable_document(UnreachableCause::StrictModes);
at: src/cmd/client.rs:233
quote: document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));
at: src/cmd/client.rs:40
quote: Ok(Some(21))
@row_add n=938 row=decision/millisecond-utc-timestamps area=otel-sdk-init rec=fct-xxrgqpdc@1
name: Millisecond UTC timestamps
rule: Every diagnostics timestamp is RFC 3339 in UTC with millisecond precision and a Z suffix.
why: The default timer writes microseconds, and the chrono RFC 3339 timer writes +00:00 with automatic precision.
@row_check n=938 row=decision/millisecond-utc-timestamps sha=34e46e1dbc6d v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:96
quote: at.to_rfc3339_opts(SecondsFormat::Millis, true)
at: src/obs.rs:127
quote: .with_timer(MillisUtc)
@row_add n=939 row=decision/normal-observability-mode area=product-mode-normal-vs-inverted-self-observing rec=fct-2los3fjb@1
name: Normal observability mode
rule: viola runs in Normal observability mode: events.ndjson is a product audit log, not a telemetry collector.
why: viola is not a telemetry product; the self-observing-inversion trigger did not fire.
@row_check n=939 row=decision/normal-observability-mode sha=7e1f3f2e7c9c v=unverifiable judge=sonnet head=8834195b0683
searched: opentelemetry · in Cargo.toml
why: not a code claim
@row_add n=940 row=decision/env-vars-not-configuration area=service-identity rec=fct-plkmesmz@1,fct-ah3dlz7g@1
name: Environment variables are not configuration
rule: Environment variables are not a configuration channel: service identity has no env-var source or fallback.
@row_check n=940 row=decision/env-vars-not-configuration sha=7ef1217359b3 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:7
quote: pub const SERVICE_NAME: &str = "viola";
searched: OTEL_SERVICE_NAME · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs
searched: OTEL_ · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs
@row_add n=941 row=decision/single-version-constant area=service-identity rec=fct-ycmm64u7@1
name: One version constant for every crate
rule: One compile-time version constant taken from the package version serves every crate, and every crate takes the workspace version.
why: The log version, channel sender and snapshot writer cannot drift apart.
@row_check n=941 row=decision/single-version-constant sha=6cf614e1bd89 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:8
quote: pub const VERSION: &str = env!("CARGO_PKG_VERSION");
at: Cargo.toml:3
quote: version.workspace = true
at: crates/viola-core/Cargo.toml:3
quote: version.workspace = true
@row_add n=942 row=absence/hosted-deployment area=service-identity rec=fct-ah3dlz7g@1
capability: A hosted deployment or deployment environment; viola is local-only
@row_check n=942 row=absence/hosted-deployment sha=c5d864621b06 v=holds judge=sonnet head=8834195b0683
searched: deployment · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs
@row_add n=943 row=contract/process-start/service-name area=service-identity rec=fct-rrjwwqcp@2
boundary: the process-start line
item: service_name
promise: process-start carries the service name as field service_name.
@row_check n=943 row=contract/process-start/service-name sha=f0a8624dd339 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:33
quote: service_name = SERVICE_NAME,
@row_add n=944 row=contract/process-start/version area=service-identity rec=fct-rrjwwqcp@2
boundary: the process-start line
item: version
promise: process-start carries the product version as field version.
@row_check n=944 row=contract/process-start/version sha=b7e4b58ed6d3 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:34
quote: version = VERSION,
@row_add n=945 row=contract/process-start/os area=service-identity rec=fct-rrjwwqcp@2
boundary: the process-start line
item: os
promise: process-start carries field os, the one word linux.
@row_check n=945 row=contract/process-start/os sha=292be8968184 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:35
quote: os = std::env::consts::OS,
@row_add n=946 row=contract/process-start/pid area=service-identity rec=fct-rrjwwqcp@2
boundary: the process-start line
item: pid
promise: process-start carries the process id as field pid.
@row_check n=946 row=contract/process-start/pid sha=4fc29b67bfe0 v=holds judge=sonnet head=8834195b0683
at: src/run/mod.rs:36
quote: pid = std::process::id(),
@row_add n=947 row=contract/diagnostics-line/process area=service-identity rec=fct-rrjwwqcp@2
boundary: the diagnostics line
item: process
promise: Every diagnostics line carries the role as field process.
@row_check n=947 row=contract/diagnostics-line/process sha=e1ef77ae3f7a v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:146
quote: process = $crate::obs::ctx_process(),
@row_add n=948 row=absence/pulse-class-receiver area=snapshot-paste-to-ai-integration rec=fct-ehmrhyqq@1
capability: A pulse-class log receiver
@row_check n=948 row=absence/pulse-class-receiver sha=959fa281c4ac v=holds judge=sonnet head=8834195b0683
searched: pulse · in src/**/*.rs, crates/**/*.rs, tests/**/*.rs
@row_add n=949 row=contract/agent-run-logs/events-record area=snapshot-paste-to-ai-integration rec=fct-v725cwjk@2
boundary: agent-run logs output
item: events record
promise: Each events.ndjson line is merged as {"src":"events","instance","offset","record"}.
@row_check n=949 row=contract/agent-run-logs/events-record sha=1b7433c7ae59 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:133-135
quote: head.insert("src".to_owned(), "events".into()); head.insert("instance".to_owned(), instance.into()); head.insert("offset".to_owned(), at.into());
@row_add n=950 row=contract/agent-run-logs/diag-record area=snapshot-paste-to-ai-integration rec=fct-v725cwjk@2
boundary: agent-run logs output
item: diag record
promise: Each line of every diagnostics/*.ndjson is merged as {"src":"diag","file","record"}.
@row_check n=950 row=contract/agent-run-logs/diag-record sha=881ff9675c27 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:177-178
quote: head.insert("src".to_owned(), "diag".into()); head.insert("file".to_owned(), file.into());
@row_add n=951 row=contract/agent-run-logs/detail-record area=snapshot-paste-to-ai-integration rec=fct-v725cwjk@2,fct-45i3uku5@1
boundary: agent-run logs output
item: detail record
promise: Every instance detail line from instances/*/diagnostics/detail-*.ndjson is merged as {"src":"diag","file","instance","record"}, the instance telling apart the same detail file name across instances.
@row_check n=951 row=contract/agent-run-logs/detail-record sha=fb2ae71b3372 v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/logs.rs:106
quote: let files = files_in(&dir, |n| n.starts_with("detail-") && n.ends_with(".ndjson"));
at: crates/viola-e2e/src/harness/logs.rs:179-180
quote: if let Some(instance) = instance { head.insert("instance".to_owned(), instance.into());
@row_add n=952 row=contract/agent-run-status/state area=snapshot-paste-to-ai-integration rec=fct-mmhn4sme@1
boundary: agent-run status
item: state
promise: agent-run status, the aggregate health snapshot, reports a state.
@row_check n=952 row=contract/agent-run-status/state sha=f02316c6602d v=holds judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:111
quote: "v": 1, "cmd": "status", "ok": ready, "state": derived.state,
@row_add n=953 row=contract/agent-run-status/last-error area=snapshot-paste-to-ai-integration rec=fct-mmhn4sme@1
boundary: agent-run status
item: last_error
promise: agent-run status reports last_error as null or a `list-exit-<code>` string built from the list exit code.
@row_check n=953 row=contract/agent-run-status/last-error sha=6c0da3cb7146 v=stale judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/status.rs:28
quote: last_error: Some(format!("list-exit-{code}")),
shows: `last_error` is an open string `list-exit-<code>` (or null), not a value from a closed enum.
cell: promise
value: agent-run status reports last_error as null or a `list-exit-<code>` string built from the list exit code.
@row_add n=954 row=contract/send-line/client-side-refusal area=snapshot-paste-to-ai-integration rec=fct-67w3nt6h@1
boundary: the send line
item: client-side send-refused
promise: The caller-side send-refused{side:"client"} has corr null.
@row_check n=954 row=contract/send-line/client-side-refusal sha=818e4b982ee1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/send.rs:240
quote: side = "client",
@row_add n=955 row=contract/send-line/cursor-uniqueness area=snapshot-paste-to-ai-integration rec=fct-67w3nt6h@1,fct-2geebg2r@2
boundary: the send line
item: cursor corr uniqueness
promise: A send cursor is a byte offset into the target's own events.ndjson, unique per target only; refusals and later sends can share it, so one send is identified by target instance, corr, conn and rpc_id.
@row_check n=955 row=contract/send-line/cursor-uniqueness sha=5a0a41ec1b11 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:179
quote: let cursor = append_event_at(self.instance_dir, |cursor| {
at: src/run/send.rs:277
quote: None => end_offset(self.instance_dir).ok(),
at: src/run/send.rs:197
quote: rpc_id = self.call.id,
at: src/run/send.rs:198
quote: conn = self.call.conn,
@row_add n=956 row=contract/send-line/unconfirmable area=snapshot-paste-to-ai-integration rec=fct-67w3nt6h@1
boundary: the send line
item: unconfirmable send
promise: An unconfirmable send logs one send-issued and one send-confirmed{confirmed:false} under the same corr, with no send-refused and no hook side.
@row_check n=956 row=contract/send-line/unconfirmable sha=c0a0a7a03546 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:147
quote: let cursor = ctx.issue(text.len())?;
at: src/run/send.rs:151-152
quote: if matches!(confirms, Confirms::Nothing) { return ctx.unconfirmable(cursor, slot.clock.now());
at: src/run/send.rs:221-223
quote: let data = json!({"cursor": cursor, "confirmed": false}); self.record(EventKind::SendConfirmed, data)?; self.log_confirmed(cursor, false, now);
@row_add n=957 row=contract/diagnostics-line/corr-on-channel-lines area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the diagnostics line
item: corr on channel lines
promise: On channel lines, corr carries the JSON-RPC id; a notification such as hook.event has no id, so its channel-request has corr null.
@row_check n=957 row=contract/diagnostics-line/corr-on-channel-lines sha=1fb0ba069813 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:183
quote: corr = id,
at: crates/viola-channel/src/server.rs:368
quote: corr = corr,
at: crates/viola-channel/src/server.rs:351
quote: const NOTIFICATION: &str = "hook.event";
@row_add n=958 row=contract/diagnostics-line/corr-on-dialog-lines area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the diagnostics line
item: corr on dialog and hook lines
promise: On dialog and hook lines, corr carries the dialog_id.
@row_check n=958 row=contract/diagnostics-line/corr-on-dialog-lines sha=faf441427d72 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:315
quote: corr = dialog_id,
at: src/cmd/hook.rs:369
quote: corr = dialog_id,
@row_add n=959 row=contract/diagnostics-line/corr-on-send-lines area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the diagnostics line
item: corr on send lines
promise: On send lines, corr carries the send cursor; a send refused before send-issued carries the target's events.ndjson end offset at refusal, a log join key only, never returned to the caller.
@row_check n=959 row=contract/diagnostics-line/corr-on-send-lines sha=e7b0095baf1f v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:196
quote: corr = cursor,
at: src/run/send.rs:275-278
quote: let corr = match cursor { Some(cursor) => Some(cursor), None => end_offset(self.instance_dir).ok(), };
@row_add n=960 row=contract/channel-params/conn area=trace-context-propagation rec=fct-2geebg2r@2
boundary: viola-channel request params
item: conn
promise: The client sends conn = "<process>-<pid>-<t0>-<n>" in params (t0 the process start in epoch milliseconds, n a per-process connection counter); both sides log it, and the server strips it before dispatch, never an input to a method.
@row_check n=960 row=contract/channel-params/conn sha=915399f23fc1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:163
quote: format!("{process}-{}-{}-{n}", std::process::id(), t0_ms())
at: crates/viola-channel/src/client.rs:176
quote: params.insert("conn".to_owned(), conn.into());
at: crates/viola-channel/src/server.rs:197
quote: fields.remove("conn");
@row_add n=961 row=contract/channel-line/srv-conn area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the channel line
item: srv_conn
promise: A server receiving no conn, or one outside ^[a-z]+-\d+-\d+-\d+$ or over 64 bytes, logs srv_conn = "srv-<accept counter>" instead.
@row_check n=961 row=contract/channel-line/srv-conn sha=33be31b0724e v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:150
quote: .spawn(move || serve_conn(&stream, &format!("srv-{n}"), dispatch.as_ref()));
at: crates/viola-channel/src/server.rs:283
quote: match conn.filter(|c| is_conn(c)) {
at: crates/viola-channel/src/server.rs:316
quote: conn.len() <= 64
@row_add n=962 row=decision/channel-join-key-conn-corr area=trace-context-propagation rec=fct-2geebg2r@2
name: Channel join key is conn and corr
rule: The merged-log join key for channel lines is (conn, corr); instance is deliberately not part of it.
why: Each side logs its own instance, so the two sides of one call carry different instance values.
@row_check n=962 row=decision/channel-join-key-conn-corr sha=eee3c4dc1fe1 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:183-184
quote: corr = id, conn = conn,
at: crates/viola-channel/src/server.rs:193
quote: let peer = Peer::of(params.conn.as_deref(), srv_conn);
@row_add n=963 row=decision/conn-includes-start-instant area=trace-context-propagation rec=fct-2geebg2r@2
name: Connection id includes the process start instant
rule: conn includes the process's start instant besides the pid and a counter, so it is unique across the session home.
why: PIDs are recycled, and every hook opens one connection with the same first id, so two hooks from a reused PID would share (conn, corr).
@row_check n=963 row=decision/conn-includes-start-instant sha=3ee943442f29 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/client.rs:163
quote: format!("{process}-{}-{}-{n}", std::process::id(), t0_ms())
@row_add n=964 row=contract/send-line/conn-rpc-id area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the send line
item: conn and rpc_id
promise: Every wrapper send-* line carries conn (or srv_conn) and rpc_id, and (conn, rpc_id) equals (conn, corr) on that call's channel-request and channel-response on both sides.
@row_check n=964 row=contract/send-line/conn-rpc-id sha=7ae1077f38ce v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:197-199
quote: rpc_id = self.call.id, conn = self.call.conn, srv_conn = self.call.srv_conn,
at: src/run/send.rs:234-236
quote: rpc_id = self.call.id, conn = self.call.conn, srv_conn = self.call.srv_conn,
at: src/run/send.rs:284-286
quote: rpc_id = self.call.id, conn = self.call.conn, srv_conn = self.call.srv_conn,
at: crates/viola-channel/src/client.rs:183-184
quote: corr = id, conn = conn,
at: crates/viola-channel/src/server.rs:370
quote: srv_conn = peer.srv_conn(),
@row_add n=965 row=contract/channel-line/from-trust area=trace-context-propagation rec=fct-2geebg2r@2
boundary: the channel line
item: from_trust
promise: from is logged with the constant from_trust:"self-reported" and is never treated as identity.
@row_check n=965 row=contract/channel-line/from-trust sha=7bfc92e34325 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:166
quote: self.from.as_ref().map(|_| "self-reported")
at: src/run/dialog.rs:464
quote: from_trust = from.map(|_| "self-reported"),
@row_add n=966 row=decision/hook-correlation-by-fields area=trace-context-propagation rec=fct-hg74abkb@1
name: Hook lines correlated by fields
rule: Hooks receive no inbound trace context from Claude Code; hook lines are correlated by instance plus dialog_id for dialog hooks, or by instance plus hook_event plus timestamp.
@row_check n=966 row=decision/hook-correlation-by-fields sha=4b1b1ccd4987 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:369-370
quote: corr = dialog_id, hook_event = event.as_str(),
at: src/cmd/hook.rs:174
quote: hook_event = event.as_str(),
@row_add n=967 row=decision/explicit-correlation-values area=trace-context-propagation rec=fct-pfzaam6b@1
name: Correlation values passed explicitly
rule: corr, conn and instance are passed as explicit values into each thread or task and set on each event, never recovered from span context.
why: Spans are not serialized.
@row_check n=967 row=decision/explicit-correlation-values sha=967df61f9b74 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:196-199
quote: corr = cursor, rpc_id = self.call.id, conn = self.call.conn, srv_conn = self.call.srv_conn,
at: crates/viola-channel/src/server.rs:245
quote: conn: peer.conn(),
@row_add n=968 row=decision/pty-byte-stream-never-logged area=1-obs-scope-summary rec=fct-m24p7svx@2,fct-t2iihkln@3,fct-jzzdh6ip@2
name: PTY byte stream never logged
rule: The PTY byte stream is never logged; the viola-pty seam is instrumentable, and the openpty OS API behind it is boundary-only.
why: The PTY byte stream is user content.
@row_check n=968 row=decision/pty-byte-stream-never-logged sha=e8c57575a18c v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:72-75
quote: #[tracing::instrument( skip_all, name = "pty.paste_write", fields(text_bytes = text.len(), paste_mode = "bracketed")
@row_add n=969 row=decision/drift-reports-diagnostics-only area=1-obs-scope-summary rec=fct-gira4fst@1,fct-73ei4s3r@2,fct-add24jhw@1
name: Drift reports go to diagnostics only
rule: Drift reports of viola-agent-claude go to diagnostics/ only and nowhere else.
why: The outputs of viola-agent-claude are the most sensitive in the product.
@row_check n=969 row=decision/drift-reports-diagnostics-only sha=a315dc1302be v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:557
quote: obs::write_detail(&instance.home, &instance.name, ObsProcess::Hook, &line);
@row_add n=970 row=decision/claude-internals-not-a-contract area=1-obs-scope-summary rec=fct-avrouaxx@1,fct-ngecoymf@1,fct-t3jwc6wb@1
name: Wrapped claude internals are not a contract
rule: The wrapped claude CLI is a third-party binary run unmodified and instrumented at its boundary only (spawn and exit, paste writes, the vt100 readiness-gate outcome); its internal state and transcripts are never read as a contract.
@row_check n=970 row=decision/claude-internals-not-a-contract sha=2b589f0f33d1 v=holds judge=sonnet head=8834195b0683
searched: transcript · in src/**/*.rs (only the fake test agent and a test fixture name it)
searched: transcript · in crates/viola-agent-claude/src/**/*.rs (only a test payload at hook.rs:418)
@row_add n=971 row=decision/event-payload-not-in-process-logs area=1-obs-scope-summary rec=fct-5wgriooq@1,fct-a3522pml@1
name: Event payloads kept out of process logs
rule: The payload content of events.ndjson is never copied into process-log lines.
why: events.ndjson is itself the product audit stream.
@row_check n=971 row=decision/event-payload-not-in-process-logs sha=aa4718fd06de v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:193-201
quote: obs_event!( INFO, ObsEvent::SendIssued, corr = cursor, rpc_id = self.call.id, conn = self.call.conn, srv_conn = self.call.srv_conn, from = from, from_trust = self.trust(),
@row_add n=972 row=decision/absolute-paths-scrubbed-externally area=1-obs-scope-summary rec=fct-5wgriooq@1,fct-a3522pml@1
name: Absolute paths scrubbed from external output
rule: Absolute paths in snapshot.json (endpoint, pinned_bin) and in config values are scrubbed from external output and allowed in diagnostics/.
@row_check n=972 row=decision/absolute-paths-scrubbed-externally sha=cd30a072115c v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/board.rs:167
quote: pub struct Item {
@row_add n=973 row=contract/log-line/process area=1-obs-scope-summary rec=fct-df2jtbjl@1,fct-cmvi57xd@1,fct-3i3k7olh@1
boundary: every log line
item: process
promise: Every log line carries a process key naming its role: one of run, hook, mcp, cli.
@row_check n=973 row=contract/log-line/process sha=cf645ada1d75 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:146
quote: process = $crate::obs::ctx_process(),
at: crates/viola-core/src/obs.rs:86
quote: pub const ALL: [Self; 4] = [Self::Run, Self::Hook, Self::Mcp, Self::Cli];
@row_add n=974 row=decision/hook-log-write-failure-swallowed area=1-obs-scope-summary rec=fct-qlfp3c3h@1,fct-z66kg5vk@1,fct-evvm5jqh@1
name: Hook log write failure swallowed
rule: A failed write to the hook log is swallowed, so the hook still exits 0 and never writes to stderr.
@row_check n=974 row=decision/hook-log-write-failure-swallowed sha=d4ddbf4bfa22 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:126
quote: .log_internal_errors(false)
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
@row_add n=975 row=decision/mcp-rejections-codes-only area=1-obs-scope-summary rec=fct-mawfkjfn@1,fct-276b34sw@1,fct-4bhscgzh@1
name: MCP rejections carry codes only
rule: MCP isError results and rejected tool calls carry codes only (refusal, detail, instance-unreachable, wrapper-fault, invalid-params), with no upstream text, absolute paths or type names.
@row_check n=975 row=decision/mcp-rejections-codes-only sha=1df092d4e82f v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/result.rs:41
quote: result.content = vec![ContentBlock::text(format!("refused: {code}"))];
at: crates/viola-mcp/src/result.rs:36
quote: error("invalid-params", Value::from(field))
@row_add n=976 row=decision/one-write-per-log-line area=1-obs-scope-summary rec=fct-ccfaed7l@1,fct-kv6frzyp@2,fct-he3oql74@1
name: One write per log line
rule: Each formatted log line is emitted as a single write on an append-mode handle.
why: Concurrent hook processes share one file.
@row_check n=976 row=decision/one-write-per-log-line sha=deb5635f3b03 v=holds judge=sonnet head=8834195b0683
at: crates/viola-state/src/fs.rs:46
quote: options.create(true).append(true);
@row_add n=977 row=contract/diagnostics-files/run area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-zlgd74pd@1
boundary: the diagnostics files
item: run
promise: Role run writes <home>/diagnostics/run-<name>.ndjson.
@row_check n=977 row=contract/diagnostics-files/run sha=770eae12bf4d v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name))
at: src/obs.rs:87
quote: let dir = home.join("diagnostics");
@row_add n=978 row=contract/diagnostics-files/hook area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-zlgd74pd@1
boundary: the diagnostics files
item: hook
promise: Role hook writes <home>/diagnostics/hook-<name>.ndjson, shared by concurrent hook processes; a write failure is swallowed.
@row_check n=978 row=contract/diagnostics-files/hook sha=affa63650a07 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:57-58
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name))
at: src/cmd/hook.rs:122
quote: let _ = obs::viola_obs_init(
@row_add n=979 row=contract/diagnostics-files/cli area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-zlgd74pd@1
boundary: the diagnostics files
item: cli
promise: Role cli writes <home>/diagnostics/cli-<name>.ndjson, only when an instance resolves.
@row_check n=979 row=contract/diagnostics-files/cli sha=104283f2a509 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:57-60
quote: (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name)) => { format!("{}-{}", process.as_str(), name_str(name)) } _ => return None,
at: src/cmd/client.rs:47
quote: obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
@row_add n=980 row=decision/no-log-rotation area=1-obs-scope-summary rec=fct-igz2gfrn@1,fct-z22o2ytp@1,fct-575hof4h@1,fct-khwnpdov@2
name: No log rotation
rule: There is no rotation of events.ndjson or diagnostics/*.ndjson in v1, retention being the session home's lifetime, and any future diagnostics/ rotation must keep 0600/0700 and never touch events.ndjson offsets.
@row_check n=980 row=decision/no-log-rotation sha=070318be0651 v=holds judge=sonnet head=8834195b0683
searched: rotat · in src/**/*.rs
searched: rotat · in crates/**/*.rs
@row_add n=981 row=absence/otlp-receiver area=1-obs-scope-summary rec=fct-q2zh72zo@2,fct-x2cjy5mn@1
capability: An OTLP or pulse-class telemetry receiver
@row_check n=981 row=absence/otlp-receiver sha=80433f4d2117 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry|otlp|tonic|prost · in src/**/*.rs and crates/**/*.rs
searched: opentelemetry|otlp · in Cargo.toml
@row_add n=982 row=decision/send-text-never-logged area=1-obs-scope-summary rec=fct-hverrr7w@1,fct-xnd434rh@1,fct-gpdjinuy@1,fct-gtr2jsdi@1
name: Send text never logged
rule: Sent text never appears on stderr, in --json output, in MCP results or in any process-log field outside instances/<name>/diagnostics/.
@row_check n=982 row=decision/send-text-never-logged sha=11e59ed5bab6 v=holds judge=sonnet head=8834195b0683
at: src/run/send.rs:147
quote: let cursor = ctx.issue(text.len())?;
@row_add n=983 row=decision/typed-keys-never-logged area=1-obs-scope-summary rec=fct-iks5yrbc@1,fct-3y3q6wvi@1,fct-ybmmsj4h@1,fct-ylcomffg@1
name: Typed keys never logged
rule: Keys a human types never appear in a log line; the wheel product event in events.ndjson is the audit record.
@row_check n=983 row=decision/typed-keys-never-logged sha=b49eb811ddeb v=holds judge=sonnet head=8834195b0683
at: crates/viola-pty/src/pump.rs:94
quote: impl Write for HumanInput {
@row_add n=984 row=decision/obs-deps-tokio-free area=1-obs-scope-summary rec=fct-r2s52jjf@1,fct-3og7ujbn@2
name: Obs dependencies Tokio-free and C-free
rule: Obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass cargo deny.
guard: cargo deny
@row_check n=984 row=decision/obs-deps-tokio-free sha=804e376dc0f1 v=holds guard=true judge=sonnet head=8834195b0683
at: deny-sync.toml:12
quote: { crate = "tokio", reason = "tokio is confined to viola-mcp; the sync crates stay tokio-free" },
at: deny.toml:29
quote: { crate = "cc", reason = "no C in the build" },
guard_at: deny.toml:27
@row_add n=985 row=decision/distinct-detail-code-per-exit-cause area=1-obs-scope-summary rec=fct-y6qmqcu2@1,fct-oiibon7o@2
name: Distinct detail code per exit cause
rule: Each cause behind exit 21 and exit 1 gets a distinct detail code with no path or pid.
except: A strict-modes check refused before any log opens writes no line; its cause is the fixed stderr pair or the --json document.
@row_check n=985 row=decision/distinct-detail-code-per-exit-cause sha=23a9a729e083 v=holds judge=sonnet head=8834195b0683
at: src/cmd/run.rs:300
quote: return Ok(refused_with(launch.json, "socket-dir-refused", || {
at: src/cmd/client.rs:261
quote: detail = cause.detail(),
at: crates/viola-core/src/lib.rs:197
quote: Self::InstanceDead => "instance-dead",
@row_add n=986 row=decision/no-external-error-reporter area=2-telemetry-strategy rec=fct-i24bcxka@1,fct-6c7xwzms@1
name: No external error reporter
rule: Errors have no external reporter.
why: Security says no outbound network calls.
@row_check n=986 row=decision/no-external-error-reporter sha=458a4c5a055c v=holds judge=sonnet head=8834195b0683
at: deny.toml:34
quote: { crate = "sentry", reason = "no error reporter or egress in v1" },
@row_add n=987 row=decision/no-event-payloads-in-process-logs area=2-telemetry-strategy rec=fct-i24bcxka@1,fct-mtgjpoqf@1
name: No event payloads in process logs
rule: Obs never copies event payloads into process logs.
@row_check n=987 row=decision/no-event-payloads-in-process-logs sha=788dea7f45dd v=unverifiable judge=sonnet head=8834195b0683
searched: events.ndjson · in {src,crates/viola-core/src}/**
why: no evidence found
@row_add n=988 row=decision/no-w3c-traceparent area=2-telemetry-strategy rec=fct-pcsvls55@1,fct-dhcjb4zv@1
name: No W3C traceparent in v1
rule: W3C traceparent is not used in v1; correlation goes through corr plus conn.
why: No consumer exists and channel params declare no trace-context field.
@row_check n=988 row=decision/no-w3c-traceparent sha=f1db611826f9 v=holds judge=sonnet head=8834195b0683
searched: traceparent · in {src,crates,tests,scripts,schemas,.github,.config}/** and *.rs, *.toml
@row_add n=989 row=decision/span-naming area=2-telemetry-strategy rec=fct-gyyrakol@1,fct-osius4ib@1
name: Span naming as <area>.<operation>
rule: Span names are <area>.<operation> in snake_case, <area> one of run, pty, channel, state, hook, statusline, send, wait, last, answer, pause, release, cli, mcp; cli.<verb> names the dispatch span of a short-lived verb.
@row_check n=989 row=decision/span-naming sha=0a8bd890816e v=holds judge=sonnet head=8834195b0683
at: src/cmd/list.rs:116
quote: #[instrument(skip_all, name = "cli.list", fields(json = json))]
at: src/cmd/hook.rs:163
quote: #[instrument(skip_all, name = "hook.handle", fields(hook_event = event.as_str()))]
at: src/cmd/pause.rs:91
quote: #[instrument(skip_all, name = "pause.client")]
at: crates/viola-mcp/src/tools.rs:248
quote: #[instrument(skip_all, name = "mcp.tool_call", fields(tool = tool.as_str()))]
@row_add n=990 row=contract/process-log-line/event area=2-telemetry-strategy rec=fct-hliwgb3i@1,fct-evvm5jqh@1
boundary: the process log line
item: event
promise: The event field is a closed kebab-case enum; a new value needs a Decisions Log entry, and each value fixes what its corr holds.
@row_check n=990 row=contract/process-log-line/event sha=d9c8af1d3335 v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/obs.rs:12
quote: pub enum ObsEvent {
@row_add n=991 row=decision/channel-response-always-logged area=4-span-trace-coverage rec=fct-o5zzhzdu@1,fct-u2zbdxyd@2
name: channel-response always logged
rule: Every dispatched channel request logs its channel-response line, an early return or an error included.
why: The line is emitted from a Drop guard, so an early return or error still logs it.
@row_check n=991 row=decision/channel-response-always-logged sha=a9ce580dc06b v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:463
quote: impl Drop for ResponseLine<'_> {
at: crates/viola-channel/src/server.rs:211
quote: let mut logged = ResponseLine::new(started, Some(id), peer, method);
@row_add n=992 row=decision/compact-confirm-window area=4-span-trace-coverage rec=fct-o5zzhzdu@1,fct-xfymn57j@1,fct-u2zbdxyd@2
name: Compact confirmation window
rule: A /compact send waits 40 s for its compacted-session post-condition, while every other confirmed send waits 10 s.
state: provisional
@row_check n=992 row=decision/compact-confirm-window sha=074e7361e588 v=holds judge=sonnet head=8834195b0683
at: crates/viola-agent-claude/src/screen.rs:26
quote: pub const COMPACT_CONFIRM_WINDOW: Duration = Duration::from_secs(40);
at: crates/viola-agent-claude/src/screen.rs:20
quote: pub const CONFIRM_WINDOW_FALLBACK: Duration = Duration::from_secs(10);
at: src/run/send/slot.rs:177-178
quote: Some(Confirms::Post(PostCondition::CompactedSession)) => COMPACT_CONFIRM_WINDOW, _ => CONFIRM_WINDOW_FALLBACK,
@row_add n=993 row=decision/dialog-content-never-logged area=4-span-trace-coverage rec=fct-trei2tnd@1,fct-ycxx4ij4@2,fct-jv26h5pj@1
name: Dialog content never logged
rule: Dialog questions, answers, plan text and tool input never appear in a home-level process-log line; dialog events carry codes only, and the text reaches an instance detail file only inside a chain or drift report.
@row_check n=993 row=decision/dialog-content-never-logged sha=145e12492e27 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:316
quote: dialog_kind = raise.kind.as_str(),
at: src/run/dialog.rs:462
quote: dialog_kind = pending.kind.as_str(),
searched: (question|response|tool_input|plan_text|answer_text) = · in src/run/dialog.rs src/cmd/hook.rs src/cmd/answer.rs src/cmd/mcp.rs (no obs_event field carries them)
@row_add n=994 row=decision/wheel-event-is-audit-record area=4-span-trace-coverage rec=fct-ybmmsj4h@1,fct-k7yhktu7@1
name: Wheel product event is the audit record
rule: The wheel product event in events.ndjson is the audit record of a wheel change; the typed keys are never logged and the wheel transition span is only a point in time.
@row_check n=994 row=decision/wheel-event-is-audit-record sha=c1c5f8b24fdb v=holds judge=sonnet head=8834195b0683
at: src/run/wheel.rs:289
quote: json!({"holder": step.to.as_str(), "cause": step.cause.as_str()}),
at: src/run/wheel.rs:277-281
quote: name = "run.wheel_transition", fields( wheel_from = step.from.as_str(), wheel_to = step.to.as_str(), cause = step.cause.as_str()
@row_add n=995 row=decision/statusline-stdout-passthrough area=4-span-trace-coverage rec=fct-rwzldnx4@1,fct-v76wl5kz@1
name: Statusline stdout passes through unlogged
rule: The user's statusline stdout passes through unchanged and is never logged.
@row_check n=995 row=decision/statusline-stdout-passthrough sha=39e08b9de793 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook/statusline.rs:143
quote: let _ = io.out.write_all(&stdout).and_then(|()| io.out.flush());
at: src/cmd/hook/statusline.rs:190-192
quote: /// The user's command through the shell, as ONE argument, with the payload on its stdin and this /// process's environment and working directory unchanged. The command is user content: neither /// line nor the span carries it.
@row_add n=996 row=decision/statusline-deadline area=4-span-trace-coverage rec=fct-rwzldnx4@1,fct-kv6frzyp@2,fct-v76wl5kz@1
name: Statusline shell-out deadline
rule: The statusline shell-out of the user's command is bounded by STATUSLINE_DEADLINE, 5 s.
state: provisional
@row_check n=996 row=decision/statusline-deadline sha=e43857066ff3 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:64
quote: const STATUSLINE_DEADLINE: Duration = Duration::from_secs(5);
@row_add n=997 row=decision/env-names-only area=4-span-trace-coverage rec=fct-xnfonqro@1,fct-wa67dogf@1,fct-6xa7lf3f@2
name: Environment names only, never values
rule: The strip and every log record environment variable names only, never values, and no env map is passed to a span or event.
guard: E2
@row_check n=997 row=decision/env-names-only sha=1d90cb8ffd3d v=holds guard=true judge=sonnet head=8834195b0683
at: src/run/mod.rs:58
quote: env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
at: crates/viola-agent-claude/src/lib.rs:46
quote: joined(self.remove.iter().map(|n| n.to_string_lossy().into_owned()))
guard_at: tests/tui_env_strip.rs:1
@row_add n=998 row=decision/ledger-contents-never-logged area=4-span-trace-coverage rec=fct-g4daqt4h@1,fct-6z2owz4q@1
name: Capability ledger path and stamps never logged
rule: The capability ledger's path and its stamp contents never appear in a log line.
@row_check n=998 row=decision/ledger-contents-never-logged sha=fe74edf62b6f v=holds judge=sonnet head=8834195b0683
at: src/run/version_gate.rs:174-180
quote: obs_event!( WARN, ObsEvent::ParseRejected, parser = "ledger-stamps", detail = detail, count = 1u64, );
@row_add n=999 row=absence/in-process-metrics-pipeline area=5-metric-coverage rec=fct-kvmjumn6@1,fct-dijwakce@1
capability: An in-process metrics pipeline, with no listener and no exporter
@row_check n=999 row=absence/in-process-metrics-pipeline sha=ab2c34c95b2f v=holds judge=sonnet head=8834195b0683
searched: opentelemetry|metrics|prometheus|statsd · in Cargo.toml, **/Cargo.toml, Cargo.lock, src/**/*.rs, crates/*/src/**/*.rs
searched: metrics-exporter|metrics:: · in Cargo.toml, crates/**/Cargo.toml, **/*.rs
@row_add n=1000 row=contract/log-line/corr area=6-log-coverage rec=fct-qaeihrqm@1,fct-3i3k7olh@1,fct-dhc5cdwv@1
boundary: every log line
item: corr
promise: Set per the tests' table and copied unchanged as a JSON number or string; null is represented by the key's absence.
@row_check n=1000 row=contract/log-line/corr sha=599cf69774ed v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:476
quote: corr = self.corr,
at: crates/viola-channel/src/client.rs:183
quote: corr = id,
@row_add n=1001 row=absence/grpc area=6-log-coverage rec=fct-oqqvdowc@1,fct-nvdezwhb@1
capability: gRPC of any kind
@row_check n=1001 row=absence/grpc sha=a6ef83ab5413 v=holds judge=sonnet head=8834195b0683
searched: tonic|prost|grpcio|grpc · in Cargo.toml, crates/*/Cargo.toml, src/**, crates/*/src/**
@row_add n=1002 row=decision/no-sentry-or-otel-bridge area=7-error-capture-reporting rec=fct-kd3a4grv@1
name: No Sentry or OTel bridge
rule: Errors have no Sentry or OTel bridge; they correlate by corr, conn, process and instance, not by trace_id or span_id.
why: Both were deferred or rejected.
@row_check n=1002 row=decision/no-sentry-or-otel-bridge sha=ca8073b48ed8 v=holds judge=sonnet head=8834195b0683
searched: sentry|opentelemetry · in src/**/*.rs and crates/**/*.rs
searched: trace_id|span_id · in src/**/*.rs and crates/**/*.rs
@row_add n=1003 row=decision/home-level-lines-path-free area=8-pii-scrubbing-compliance rec=fct-qk7hj63u@1,fct-syepqt4n@1
name: Home-level lines path-free
rule: Home-level log lines are path-free: file fields are basenames, panic_location is workspace-relative and source file recording is off; absolute paths appear only in detail files.
why: The OS username inside absolute paths is the only PII in scope.
@row_check n=1003 row=decision/home-level-lines-path-free sha=79d4991f0d3f v=holds judge=sonnet head=8834195b0683
at: src/main.rs:260
quote: fn panic_location(file: &Path) -> String {
@row_add n=1004 row=absence/otel-span-processing area=8-pii-scrubbing-compliance rec=fct-xpgyosmo@1
capability: An OTel SpanProcessor
@row_check n=1004 row=absence/otel-span-processing sha=b165016e02a6 v=holds judge=sonnet head=8834195b0683
searched: SpanProcessor · in src/**, crates/*/src/**, Cargo.toml, crates/*/Cargo.toml, Cargo.lock, deny.toml
searched: opentelemetry · in the same paths (only deny.toml bans opentelemetry-otlp and opentelemetry-stdout)
@row_add n=1005 row=decision/secret-scan-before-upload area=8-pii-scrubbing-compliance rec=fct-tcjwyf2p@1,fct-ydc7ueix@3
name: Secret scan before artifact upload
rule: CI runs the secret scan for the Cookie header, CLAUDE* values, the content canary and 0600 over every diagnostics/ file before any artifact upload.
guard: secret-scan test
@row_check n=1005 row=decision/secret-scan-before-upload sha=d30aa21b1bfd v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/secret_scan.rs:129
quote: for offset in find_all(&lower, b"cookie:") {
at: crates/viola-e2e/src/harness/secret_scan.rs:124
quote: for (class, needle) in CRITICAL {
at: crates/viola-e2e/src/harness/secret_scan.rs:180
quote: mode & 0o777 == 0o600
at: .github/workflows/ci.yml:76
quote: run: bash scripts/agent-run.sh secret-scan
guard_at: crates/viola-e2e/src/harness/secret_scan.rs:242
@row_add n=1006 row=decision/uploads-gated-on-secret-scan area=9-ci-integration rec=fct-cstqzpm2@1,fct-safgt2vp@1
name: CI uploads gated on the secret scan
rule: The diag, harness and junit CI artifact uploads run only when the secret scan, which always runs, succeeded.
guard: secret-scan
@row_check n=1006 row=decision/uploads-gated-on-secret-scan sha=13b9aee752dc v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:79
quote: if: always() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:87
quote: if: failure() && steps.secret-scan.outcome == 'success'
at: .github/workflows/ci.yml:98
quote: if: always() && steps.secret-scan.outcome == 'success'
guard_at: .github/workflows/ci.yml:72
@row_add n=1007 row=decision/zero-unlogged-panics area=10-slo-invariants-telemetry-budgets rec=fct-3qkbv2ca@1,fct-wgmneptw@1,fct-rjubzxp3@1,fct-6543go25@1
name: Zero unlogged panics
rule: Every panic in run, hook, mcp and cli after init step 4 produces exactly one single-line ERROR panic record in its home-level role file, with no multi-line backtrace there, and hook still exits 0; retry-once policies for panic handling are forbidden.
guard: G2
except: the bounded exemptions where no file exists, so no line can be written; G2 alone leaves out the fake-agent seam's deliberate panic from its count.
@row_check n=1007 row=decision/zero-unlogged-panics sha=4d3c9654a321 v=holds guard=true judge=sonnet head=8834195b0683
at: src/main.rs:46
quote: std::panic::set_hook(Box::new(viola_panic_hook));
at: src/main.rs:133
quote: (Role::Hook, _) => ExitCode::SUCCESS,
at: .config/nextest.toml:2
quote: retries = 0
at: scripts/g2-zero-panics.sh:10
quote: seam_file="src/cmd/hook/seam.rs"
guard_at: scripts/g2-zero-panics.sh:1
@row_add n=1008 row=decision/hook-obs-overhead-budget area=10-slo-invariants-telemetry-budgets rec=fct-onnnq5uj@1,fct-bxbg45yr@1
name: Hook observability overhead budget
rule: Obs overhead on hook, logger init being one open plus appends, is held inside the same max < 1.0 s perf gate, which holds only when every hook command runs with VIOLA_NAME set and a home under target/e2e-home.
guard: gate --require perf
@row_check n=1008 row=decision/hook-obs-overhead-budget sha=ad7a0dedd3c8 v=holds guard=true judge=sonnet head=8834195b0683
at: crates/viola-e2e/src/harness/gate.rs:151
quote: Some(max) if max < bound => {}
at: crates/viola-core/src/lib.rs:14
quote: pub const SPINE_DEADLINE: Duration = Duration::from_secs(1);
at: crates/viola-e2e/src/harness/run/perf.rs:120
quote: .env("VIOLA_NAME", INSTANCE)
guard_at: .github/workflows/ci.yml:189
@row_add n=1009 row=decision/no-print-macros area=11-obs-anti-patterns-never-do-these rec=fct-l2iw7ho4@1,fct-xpak76xj@2,fct-hfdtlj6j@2,fct-safgt2vp@1
name: No print macros in product code paths
rule: print!, println!, eprint!, eprintln! and dbg! are never used in the hook, run or mcp code paths of the product crates, and every product member inherits the workspace lints.
why: They bypass the file sink and break the Hook contract, the run terminal rule and MCP framing.
guard: clippy print_stdout, print_stderr and dbg_macro at deny
except: viola-harness, the fake agent and the output modules that own --json, human CLI stderr and the hook decision body.
@row_check n=1009 row=decision/no-print-macros sha=a9d766686321 v=holds guard=true judge=sonnet head=8834195b0683
at: Cargo.toml:256
quote: print_stdout = "deny"
at: Cargo.toml:258
quote: dbg_macro = "deny"
guard_at: Cargo.toml:256
@row_add n=1010 row=decision/pinned-actions area=11-obs-anti-patterns-never-do-these rec=fct-x6iew4rz@1
name: Actions pinned by commit SHA
rule: Actions are never used unpinned; actions/upload-artifact is pinned by commit SHA.
guard: zizmor
@row_check n=1010 row=decision/pinned-actions sha=1a2cad2f5522 v=holds guard=true judge=sonnet head=8834195b0683
at: .github/workflows/ci.yml:80
quote: uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
guard_at: .github/workflows/ci.yml:388
@row_add n=1011 row=absence/otel-trace-export area=ci-integration rec=fct-dajf6x5e@1
capability: OTel trace export: no exporter
@row_check n=1011 row=absence/otel-trace-export sha=35343fb21356 v=holds judge=sonnet head=8834195b0683
searched: opentelemetry · in src/**/*.rs, crates/**/*.rs, Cargo.toml, crates/**/Cargo.toml
searched: otel · in src/**/*.rs, crates/**/*.rs, Cargo.toml, crates/**/Cargo.toml
@row_add n=1012 row=contract/cli-internal-error/json-document area=error-capture-reporting rec=fct-vfnyc5f5@2,fct-oiibon7o@2
boundary: cli internal error
item: --json document
promise: Under --json, read among the arguments before a --, the cli catch site prints {"v":1,"error":"internal-error","detail":null} on stdout and stderr stays empty.
@row_check n=1012 row=contract/cli-internal-error/json-document sha=537600c19a7e v=holds judge=sonnet head=8834195b0683
at: src/main.rs:154
quote: .any(|arg| arg == "--json")
at: src/cmd/mod.rs:101
quote: client::refused_document("internal-error");
at: src/cmd/client.rs:227
quote: document(&json!({"v": 1, "error": code, "detail": null}));
@row_add n=1013 row=contract/mcp-stdin/oversize-line area=error-capture-reporting rec=fct-iy5crjhq@1,fct-44go5tc2@1,fct-lpbmjfvf@1,fct-g4wb4nmr@1
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: A line past MAX_FRAME is refused whole, gives no isError result and ends the session as a closed stdin: exit 0 after initialize, exit 1 before it with both streams empty; its one record is parse-rejected mcp-stdin oversize, no byte of the line.
@row_check n=1013 row=contract/mcp-stdin/oversize-line sha=939b99ec48ea v=holds judge=sonnet head=8834195b0683
at: crates/viola-mcp/src/lib.rs:83
quote: Ok(_) => Ok(()),
at: crates/viola-mcp/src/lib.rs:98-99
quote: parser = "mcp-stdin", detail = "oversize",
at: src/main.rs:134
quote: (Role::Mcp | Role::Cli | Role::Other, Outcome::Done(code)) => code,
@row_add n=1014 row=contract/hook-decision-detail/server-verify-failed area=log-coverage rec=fct-qiql34nd@2,fct-kohas2np@2,fct-ffumo3bh@1,fct-n6l5o3fm@2
boundary: hook-decision detail
item: server-verify-failed
promise: viola hook writes it at WARN on a server of another euid, a server that is not the recorded wrapper, or a check that cannot be made; no frame is sent, the hook fails open, and on the dialog path it follows hook-invoked with decision_emitted false.
@row_check n=1014 row=contract/hook-decision-detail/server-verify-failed sha=a0361b26d273 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:237
quote: return Err(NotSent::ServerUnverified);
at: src/cmd/hook.rs:305
quote: Asked::ServerUnverified => Some(NotSent::ServerUnverified.detail()),
at: src/cmd/hook.rs:308
quote: dialog_decided(event, &asked, body.is_some(), detail, started);
@row_add n=1015 row=contract/hook-decision-detail/channel-unreachable area=log-coverage rec=fct-qiql34nd@2,fct-kohas2np@2
boundary: hook-decision detail
item: channel-unreachable
promise: Written when the channel cannot be reached, a replayed or absent snapshot included; no frame is sent, the hook fails open and SessionEnd takes its direct append.
@row_check n=1015 row=contract/hook-decision-detail/channel-unreachable sha=2866efdbfc27 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:227
quote: let Ok(Recovered::Snapshot(snapshot)) = read_snapshot_or_replay(&instance.dir) else {
at: src/cmd/hook.rs:228
quote: return Err(NotSent::Unreachable);
at: src/cmd/hook.rs:193-195
quote: if event == HookEvent::SessionEnd { append_session_end(instance, normalised); }
@row_add n=1016 row=contract/cli-process-exit/server-verify-failed area=metric-coverage rec=fct-l3f7hrdu@2,fct-n7xftqkr@2,fct-ydc7ueix@3
boundary: CLI process-exit diagnostic line
item: server-verify-failed
promise: The CLI's server-verification refusal writes one process-exit with subject self, exit_code 21, detail server-verify-failed and during connect at WARN in cli-<name>.ndjson, with no pid, start time, path or endpoint; MCP's wait tool writes no line for it.
@row_check n=1016 row=contract/cli-process-exit/server-verify-failed sha=18b312f93e63 v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:260
quote: exit_code = 21u8,
at: crates/viola-core/src/lib.rs:200
quote: Self::ServerVerify => "server-verify-failed",
@row_add n=1017 row=decision/no-process-log-without-instance area=obs-scope-summary rec=fct-oy7tgncm@1,fct-f3gi2vhc@2
name: No process log without an instance
rule: A short-lived CLI verb run with no resolved instance writes no process-log file; its human stderr carries only fixed-message lines, and its outcome is the exit code and stdout.
@row_check n=1017 row=decision/no-process-log-without-instance sha=86fcd5b24991 v=holds judge=sonnet head=8834195b0683
at: src/obs.rs:60
quote: _ => return None,
@row_add n=1018 row=decision/strict-modes-refusal-writes-no-line area=obs-scope-summary rec=fct-oy7tgncm@1,fct-oiibon7o@2,fct-foy4up7k@1,fct-h2wm2goy@2,fct-mxwnyc6t@2,fct-5ckzuzgq@2
name: Strict-modes refusal writes no line
rule: A strict-modes check refused before any log opens ends the process with nothing under the home opened and no log line; its cause is carried only by the fixed stderr pair or the --json document.
why: Ratified as the answer to whether a refused strict-modes check logs.
except: The ledger-stamps read in run's version gate and a refused update_stamps in viola verify each leave a line.
@row_check n=1018 row=decision/strict-modes-refusal-writes-no-line sha=0ad291bd30be v=holds judge=sonnet head=8834195b0683
at: src/cmd/client.rs:31-33
quote: if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() { return Ok(None); }
at: src/run/version_gate.rs:177
quote: parser = "ledger-stamps",
at: src/cmd/verify.rs:288
quote: return Ok(Ended::Refused("strict-modes-failed"));
@row_add n=1019 row=contract/service-identity/service-name area=obs-scope-summary rec=fct-uwlbnhdg@1,fct-plkmesmz@1
boundary: Service identity
item: service.name
promise: Hardcoded "viola", the binary name, never read from OTEL_SERVICE_NAME or SERVICE_NAME.
@row_check n=1019 row=contract/service-identity/service-name sha=c9ef4233c12d v=holds judge=sonnet head=8834195b0683
at: crates/viola-core/src/lib.rs:7
quote: pub const SERVICE_NAME: &str = "viola";
searched: OTEL_SERVICE_NAME · in src/**/*.rs
searched: OTEL_SERVICE_NAME · in crates/**/*.rs
@row_add n=1020 row=decision/no-secrets-in-log-lines area=obs-scope-summary rec=fct-ogkwmfvl@1,fct-evvm5jqh@1
name: No secrets in log lines
rule: No log line may contain a Cookie header or any stripped CLAUDE* value.
guard: secret-scan test
@row_check n=1020 row=decision/no-secrets-in-log-lines sha=6e731e2db5dc v=holds guard=true judge=sonnet head=8834195b0683
searched: Cookie · in src/**/*.rs and crates/**/*.rs
guard_at: crates/viola-e2e/src/harness/secret_scan.rs:358
@row_add n=1021 row=contract/channel-response/detail area=obs-scope-summary rec=fct-m4ttxbfi@1,fct-jlmrrvwc@1,fct-plg35lpj@1,fct-exgbnztg@1,fct-fm6zc2nz@1,fct-gghzntp4@2
boundary: Channel response line
item: detail
promise: On a channel-response answering -32602, detail is one closed word: unsupported-version, invalid-params or release-from-driver, on both the server's and the client's line; it is absent on every other channel-response line and names no param or value.
@row_check n=1021 row=contract/channel-response/detail sha=b4bc360434b4 v=holds judge=sonnet head=8834195b0683
at: crates/viola-channel/src/server.rs:482
quote: detail = fault.and_then(ProtocolError::cause),
at: crates/viola-channel/src/lib.rs:105-108
quote: Self::UnsupportedVersion => Some("unsupported-version"), Self::InvalidParams => Some("invalid-params"), Self::ReleaseFromDriver => Some("release-from-driver"), Self::Parse | Self::InvalidRequest | Self::MethodNotFound | Self::Internal => None,
at: crates/viola-channel/src/client.rs:208
quote: logged.class = reply_class(&reply);
@row_add n=1022 row=contract/hook-decision-detail/socket-dir-refused area=span-trace-coverage rec=fct-elpncyv4@1,fct-ffumo3bh@1,fct-n6l5o3fm@2,fct-ne5xp5cg@2
boundary: hook-decision line
item: socket-dir-refused
promise: A refused socket directory writes one hook-decision{detail:"socket-dir-refused"} at WARN, on the dialog path after hook-invoked{corr:null}, with decision_emitted:false; exit 0, empty streams, no frame.
@row_check n=1022 row=contract/hook-decision-detail/socket-dir-refused sha=90a055b690b1 v=holds judge=sonnet head=8834195b0683
at: src/cmd/hook.rs:306
quote: Asked::SocketDir => Some(NotSent::SocketDir.detail()),
at: src/cmd/hook.rs:389-393
quote: Some(detail) => obs_event!( WARN, ObsEvent::HookDecision, hook_event = event.as_str(), decision_emitted = emitted,
at: src/cmd/hook.rs:384
quote: Asked::Deadline | Asked::Unreachable | Asked::ServerUnverified | Asked::SocketDir => None,
@row_add n=1023 row=contract/config-json/dialog-hold-s area=span-trace-coverage rec=fct-uicv6khr@1,fct-6bbewo2y@2
boundary: config.json
item: dialog_hold_s
promise: The dialog hold, a whole number of seconds from 60 to 3600, default 60; any other value gives one parse-rejected malformed WARN line and the hold takes 60.
@row_check n=1023 row=contract/config-json/dialog-hold-s sha=543ae08b4488 v=holds judge=sonnet head=8834195b0683
at: src/run/dialog.rs:177
quote: match map.get(HOLD_KEY).map(whole_hold) {
at: src/run/dialog.rs:180
quote: Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
at: crates/viola-core/src/lib.rs:22
quote: pub const DIALOG_HOLD_MIN: Duration = Duration::from_secs(60);
at: crates/viola-core/src/lib.rs:27
quote: pub const DIALOG_HOLD_MAX: Duration = Duration::from_secs(3600);
@row_none n=1 rec=fct-puseu3vv@1 type=S
@row_none n=2 rec=fct-7rmyxbi7@1 type=0
what: a note on when the document may be edited
@row_none n=3 rec=fct-cybcxb5n@1 type=S
@row_none n=4 rec=fct-cnzlh5yi@1 type=S
@row_none n=5 rec=fct-2cs5m4zb@1 type=S
@row_none n=6 rec=fct-n5cdr5q7@1 type=S
@row_none n=7 rec=fct-mpt2356v@1 type=9
@row_none n=8 rec=fct-hp7myxef@1 type=S
@row_none n=9 rec=fct-bwsghifi@1 type=5
@row_none n=10 rec=fct-h5763i4j@1 type=0
what: a description of what viola sees at the hook boundary
@row_none n=11 rec=fct-d2ucjvnc@1 type=5
@row_none n=12 rec=fct-4ereqqqf@1 type=5
@row_none n=13 rec=fct-any6tvdo@1 type=0
what: an instrumentability note on static configuration files
@row_none n=14 rec=fct-bui7ij2h@1 type=S
@row_none n=15 rec=fct-q66n3gga@1 type=S
@row_none n=16 rec=fct-r554tsmx@1 type=S
@row_none n=17 rec=fct-nluqjh6q@1 type=9
@row_none n=18 rec=fct-ana74txu@1 type=S
@row_none n=19 rec=fct-c2yfcgzp@1 type=S
@row_none n=20 rec=fct-mlx62e77@1 type=S
@row_none n=21 rec=fct-q4jff4ss@1 type=S
@row_none n=22 rec=fct-ptquzqoz@1 type=S
@row_none n=23 rec=fct-xyisrhrm@1 type=S
@row_none n=24 rec=fct-wpzlres3@1 type=S
@row_none n=25 rec=fct-yrjpw262@1 type=S
@row_none n=26 rec=fct-f5gzcc3w@1 type=S
@row_none n=27 rec=fct-zsea7hp7@1 type=S
@row_none n=28 rec=fct-cyx6phfi@1 type=S
@row_none n=29 rec=fct-v7ii4ik3@1 type=S
@row_none n=30 rec=fct-32l5mpj3@1 type=S
@row_none n=31 rec=fct-tbqtd6ka@1 type=S
@row_none n=32 rec=fct-djrxd23m@1 type=S
@row_none n=33 rec=fct-mhsc2vvi@1 type=S
@row_none n=34 rec=fct-6cgyhmtp@1 type=S
@row_none n=35 rec=fct-4qdca7zm@1 type=S
@row_none n=36 rec=fct-huapunm4@1 type=S
@row_none n=37 rec=fct-hd6w2yfv@1 type=S
@row_none n=38 rec=fct-gx6wpoyv@1 type=S
@row_none n=39 rec=fct-yxfyyyjr@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=40 rec=fct-zsuohxry@1 type=0
what: a source citation for a trigger
@row_none n=41 rec=fct-vox54xyt@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=42 rec=fct-mjdbrsjn@2 type=0
what: a source citation for a trigger
@row_none n=43 rec=fct-as4ibtw3@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=44 rec=fct-coohkvlj@1 type=0
what: a source citation for a trigger
@row_none n=45 rec=fct-metpdpyi@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=46 rec=fct-izqaewfs@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=47 rec=fct-cywoqpsh@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=48 rec=fct-fzcmyye4@1 type=0
what: a source citation for a trigger
@row_none n=49 rec=fct-uzysxmnr@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=50 rec=fct-n5zh5jve@1 type=0
what: a source citation for a trigger
@row_none n=51 rec=fct-6ofy4zhs@1 type=0
what: a trigger-type label naming why the obs scope applies
@row_none n=52 rec=fct-ipp77q5u@1 type=0
what: a source citation for a trigger
@row_none n=53 rec=fct-23yn4mby@1 type=9
@row_none n=54 rec=fct-q5t2vgpo@1 type=S
@row_none n=55 rec=fct-6zohmrdn@1 type=S
@row_none n=56 rec=fct-nieygaaw@1 type=S
@row_none n=57 rec=fct-eqimq7x6@1 type=S
@row_none n=58 rec=fct-pi7x2mau@1 type=S
@row_none n=59 rec=fct-cloqkzeh@1 type=S
@row_none n=60 rec=fct-fova76zr@1 type=S
@row_none n=61 rec=fct-6wlikjns@1 type=S
@row_none n=62 rec=fct-ikyfhy6h@1 type=S
@row_none n=63 rec=fct-qcaucl2u@1 type=S
@row_none n=64 rec=fct-u5bzeuwo@2 type=S
@row_none n=65 rec=fct-rigkqwft@1 type=S
@row_none n=66 rec=fct-ilrtsek3@1 type=S
@row_none n=67 rec=fct-krxskcsw@1 type=5
@row_none n=68 rec=fct-ophpx6bz@1 type=5
@row_none n=69 rec=fct-gtvcxthg@1 type=5
@row_none n=70 rec=fct-mwabsk26@1 type=5
@row_none n=71 rec=fct-qkp2jcsg@1 type=5
@row_none n=72 rec=fct-cbixfjiv@1 type=5
@row_none n=73 rec=fct-rtdpsbcq@1 type=S
@row_none n=74 rec=fct-cbntkles@1 type=S
@row_none n=75 rec=fct-zhhme5jd@1 type=S
@row_none n=76 rec=fct-qsbtfoen@1 type=S
@row_none n=77 rec=fct-546mputj@1 type=S
@row_none n=78 rec=fct-illbksdl@1 type=S
@row_none n=79 rec=fct-42q5kq2o@1 type=S
@row_none n=80 rec=fct-5nu2dy7a@1 type=S
@row_none n=81 rec=fct-ee4kl3w4@1 type=S
@row_none n=82 rec=fct-rv3gmelk@1 type=S
@row_none n=83 rec=fct-tpoc756b@1 type=S
@row_none n=84 rec=fct-rqrxq7s4@1 type=S
@row_none n=85 rec=fct-tf3qxuqd@1 type=S
@row_none n=86 rec=fct-q3jgjpq3@1 type=S
@row_none n=87 rec=fct-xpt2kzwy@1 type=4
relied: A panic before any role file is opened (clap parsing, home and instance checks) leaves no log record.
@row_none n=88 rec=fct-6be5cxpd@1 type=S
@row_none n=89 rec=fct-ytzmkt6r@1 type=S
@row_none n=90 rec=fct-rbwf5ypa@1 type=S
@row_none n=91 rec=fct-s2a63rdo@1 type=S
@row_none n=92 rec=fct-2jywf42w@1 type=S
@row_none n=93 rec=fct-fhqex5we@1 type=S
@row_none n=94 rec=fct-cdgkifg4@1 type=S
@row_none n=95 rec=fct-tzfcwoax@1 type=S
@row_none n=96 rec=fct-7bv4tsaz@1 type=S
@row_none n=97 rec=fct-y73cmz67@1 type=S
@row_none n=98 rec=fct-rzzg2wyr@2 type=6
@row_none n=99 rec=fct-7cctnpvb@1 type=S
@row_none n=100 rec=fct-5ybtic5h@1 type=6
@row_none n=101 rec=fct-d2xottqv@1 type=S
@row_none n=102 rec=fct-oij4u53u@1 type=S
@row_none n=103 rec=fct-7ivmlbcp@1 type=S
@row_none n=104 rec=fct-axh4ampq@1 type=S
@row_none n=105 rec=fct-cgglh3ad@1 type=S
@row_none n=106 rec=fct-pa7xy3qa@1 type=S
@row_none n=107 rec=fct-ah2viroy@1 type=S
@row_none n=108 rec=fct-eid2uzhv@1 type=S
@row_none n=109 rec=fct-q2i64n5w@1 type=S
@row_none n=110 rec=fct-gmkzqfqx@1 type=S
@row_none n=111 rec=fct-bprhan45@2 type=4
relied: A refused diagnostics-path check is exactly a failed open: the role writer ends at a sink and the detail line is dropped.
@row_none n=112 rec=fct-dujo6hv4@1 type=4
relied: Refusals of the instance and home-tree checks that run before a role file opens write no line in any log.
@row_none n=113 rec=fct-l5yuzjd3@2 type=5
@row_none n=114 rec=fct-25dvifnk@1 type=4
relied: Each MCP refusal maps to exactly one UnreachableCause word.
@row_none n=115 rec=fct-j5jj3gux@3 type=5
@row_none n=116 rec=fct-wmupioom@1 type=4
relied: The NotADirectory refusal message is fixed and names no path.
@row_none n=117 rec=fct-tgmj25nt@1 type=5
@row_none n=118 rec=fct-qat4n66a@1 type=5
@row_none n=119 rec=fct-737rcqr6@1 type=5
@row_none n=120 rec=fct-drmiyyaa@2 type=5
@row_none n=121 rec=fct-4jptauao@1 type=9
@row_none n=122 rec=fct-lt3l45qm@1 type=9
@row_none n=123 rec=fct-4gkcm5vc@1 type=9
@row_none n=124 rec=fct-dgq2ctzu@1 type=9
@row_none n=125 rec=fct-m5nv47uz@1 type=S
@row_none n=126 rec=fct-d4mbb5n4@1 type=0
what: an instrumentability tag with no entity named
@row_none n=127 rec=fct-qjje6f7i@1 type=S
@row_none n=128 rec=fct-5xfc4cig@1 type=0
what: an instrumentability tag with no entity named
@row_none n=129 rec=fct-kemgo46r@1 type=S
@row_none n=130 rec=fct-cfbdfwxc@1 type=0
what: an instrumentability tag with no entity named
@row_none n=131 rec=fct-mr2vh66r@1 type=0
what: an instrumentability tag with no entity named
@row_none n=132 rec=fct-qrw2kvvm@1 type=0
what: an instrumentability tag with no entity named
@row_none n=133 rec=fct-gwe6beo3@1 type=S
@row_none n=134 rec=fct-vaolg4ug@1 type=0
what: an instrumentability tag with no entity named
@row_none n=135 rec=fct-crky2gz3@1 type=S
@row_none n=136 rec=fct-yqxfnq5h@1 type=0
what: an instrumentability tag with no entity named
@row_none n=137 rec=fct-3afzfojv@1 type=S
@row_none n=138 rec=fct-2drawtvt@1 type=0
what: an instrumentability tag with no entity named
@row_none n=139 rec=fct-2h4irdjs@1 type=S
@row_none n=140 rec=fct-podhtuoc@1 type=0
what: an instrumentability label naming no entity and no reason
@row_none n=141 rec=fct-xenjwzhp@1 type=0
what: an instrumentability label naming no entity and no reason
@row_none n=142 rec=fct-ly2hzjle@1 type=0
what: an instrumentability label naming no entity and no reason
@row_none n=143 rec=fct-tedqfbbw@1 type=S
@row_none n=144 rec=fct-2v5blby2@1 type=0
what: an instrumentability label naming no entity and no reason
@row_none n=145 rec=fct-kbibhsg7@1 type=S
@row_none n=146 rec=fct-or7ezsji@1 type=0
what: an instrumentability label for the persistent stores
@row_none n=147 rec=fct-ks33cou5@1 type=S
@row_none n=148 rec=fct-tk3iawkf@1 type=0
what: an instrumentability label naming no entity and no reason
@row_none n=149 rec=fct-3bsvrrhe@1 type=S
@row_none n=150 rec=fct-fvn7dgcf@1 type=0
what: an instrumentability label naming no entity and no reason
@row_none n=151 rec=fct-x5lzlwa7@1 type=0
what: a not-applicable frontend line
@row_none n=152 rec=fct-4iim5xgf@1 type=0
what: a not-applicable frontend line
@row_none n=153 rec=fct-rj2a6toh@1 type=0
what: which log file each side of a channel call writes to
@row_none n=154 rec=fct-ff4quohy@1 type=4
@row_none n=155 rec=fct-g6v6qpwu@1 type=0
what: a not-applicable frontend line
@row_none n=156 rec=fct-6yzr723e@1 type=0
what: the list of surfaces a critical path crosses
@row_none n=157 rec=fct-6dojwnci@1 type=S
@row_none n=158 rec=fct-7bjg2xvs@1 type=0
what: the list of surfaces a critical path crosses
@row_none n=159 rec=fct-jztxixk3@1 type=S
@row_none n=160 rec=fct-r2x644q6@1 type=5
@row_none n=161 rec=fct-vtwqamxe@1 type=S
@row_none n=162 rec=fct-ewxn4524@1 type=5
@row_none n=163 rec=fct-qpelf55l@1 type=S
@row_none n=164 rec=fct-7ka2fpsx@1 type=5
@row_none n=165 rec=fct-aajrejox@1 type=S
@row_none n=166 rec=fct-rrskex2m@1 type=5
@row_none n=167 rec=fct-mydtv3ut@1 type=S
@row_none n=168 rec=fct-4gke2rkl@1 type=4
relied: The CLI and MCP clients screen a peer reply to code words and a sanitised fault before using it, adding no span and no line.
@row_none n=169 rec=fct-pl6vsrqd@1 type=S
@row_none n=170 rec=fct-xumv4vtr@1 type=S
@row_none n=171 rec=fct-gqzo5zgv@1 type=S
@row_none n=172 rec=fct-kgjd4mty@1 type=S
@row_none n=173 rec=fct-wkatnlg5@1 type=S
@row_none n=174 rec=fct-pso667is@1 type=S
@row_none n=175 rec=fct-7hbkkb6o@1 type=S
@row_none n=176 rec=fct-olppdytk@1 type=S
@row_none n=177 rec=fct-4vc4xc3n@1 type=S
@row_none n=178 rec=fct-6isebxud@2 type=S
@row_none n=179 rec=fct-shii35kl@1 type=S
@row_none n=180 rec=fct-oqunheh6@1 type=S
@row_none n=181 rec=fct-lrne4qjd@1 type=S
@row_none n=182 rec=fct-tdzuoari@1 type=S
@row_none n=183 rec=fct-go7yectq@1 type=S
@row_none n=184 rec=fct-fwi5gkgt@1 type=S
@row_none n=185 rec=fct-dpkhbaeo@1 type=0
what: the one-time ordering of bootstrap setup phases
@row_none n=186 rec=fct-hwfvcwas@1 type=0
what: the one-time placement of a merged bootstrap setup phase
@row_none n=187 rec=fct-sziec6n5@1 type=S
@row_none n=188 rec=fct-l76pk6pu@1 type=S
@row_none n=189 rec=fct-dd4kcev2@1 type=8
@row_none n=190 rec=fct-vbofen4i@1 type=S
@row_none n=191 rec=fct-imiry6ko@1 type=S
@row_none n=192 rec=fct-odc7kjss@1 type=0
what: an illustrative example log line
@row_none n=193 rec=fct-vguxziuc@1 type=S
@row_none n=194 rec=fct-rar73ens@1 type=4
relied: The subscriber flattens event fields to the top level of each line, and the Targets filter is the only configurable level gate.
