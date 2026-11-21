@task_prose part=session
The session added the 29th relied capability-ledger row, cross-session-prefix: verify's Run B, started with a crossSessionInbound accept settings literal, receives a message from a new print-mode sender child (verify-sender) and the row reads pass only when the arrival's prompt opens with the CLI's cross-session harness prefix.
The sender addresses Run B by its CLI inbox socket, a uds address verify composes from XDG_RUNTIME_DIR (read once, held in memory) and Run B's child pid; the earlier bare-session-id addressing was measured red live (the 2.1.328 send tool resolves no local session by a bare id) and retired.
viola hook gained a hidden --names-only mode of the --capture arm that files only closed-word Tool.k files; the stamp gained measured.cross_session_probe; the diag-line subject enum gained verify-sender.
Relied rows moved 28 to 29 (step lines [NN/29]); the recorded 2.1.328 fixture set was re-recorded whole, 32 to 33 files, one new cross-session UserPromptSubmit fixture.
Every test and harness verify now runs with a short owner-only test-owned runtime dir, because pre-push's env -i leaves XDG_RUNTIME_DIR unset; measured live on 2.1.328 as 29/29 with the message delivered.
Measured by the wrap: the new fixture holds, inside the CLI's own prefix, the sender's socket address (the recording host's runtime-dir path with its uid, and the sender pid); the fixture scrub and hygiene walk do not normalise it.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: edited · the capability ledger: the relied rows and their readings, the probe plugins, the stamp merge, the recorded-fixture scrub and hygiene walk; gains the cross-session-prefix row, the uds address composer, the sender prompt and the closed tool and send-result words
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited · the agent crate's compiled claude flag and literal set; gains the uds scheme, the CLI socket dir, suffix and 103-byte bound, and the --allowedTools flag
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: edited · the claude hook payload shapes and the harness prompt prefix classification; gains a harness-prefix lookup and its id list, prompt origin rewritten over it with the same result
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited · the ledger's unit tests: the tool and send-result words, the names-only sender plugin hooks
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited · the ledger's row-check unit tests: the cross-session-prefix pass and fail table, the uds address and sender prompt
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: edited · the ledger's framing unit tests over the relied row set
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited · the ledger's stamp-merge unit tests: the cross-session stamp block
@task_file n=8 path=src/cmd/hook.rs
what: edited · the viola hook command and its probe capture arm; gains the hidden --names-only capture mode
@task_file n=9 path=src/cmd/verify.rs
what: edited · viola verify, the one stamps writer, its measure and child set; reads XDG_RUNTIME_DIR once for the sender's address
@task_file n=10 path=src/cmd/verify/typed.rs
what: edited · verify's typed-run inputs and readings; gains the runtime dir input and the cross-session run
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: edited · verify's typed runs and how each ends
@task_file n=12 path=src/cmd/verify/typed/turns.rs
what: edited · verify's Run B turns; gains the cross-session turn that spawns the print-mode sender, gated on Run B's child pid and a composed uds address
@task_file n=13 path=src/run/version_gate.rs
what: edited (tests only) · the start's version gate over the stamps, its relied-row count in tests
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: edited · the fake agent test binary; gains the fake sender and the cross-session receiver
@task_file n=15 path=src/bin/viola-fake-agent/agents.rs
what: edited · the fake agent's claude agents listing; sibling lookup by uds address replaces lookup by session id
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited · the fake agent's mode tests: the sender and the receiver
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited · the fake agent's option tests over the recorded set
@task_file n=18 path=schemas/diag-line.v1.json
what: edited · the diagnostics line schema and its subject enum; gains verify-sender
@task_file n=19 path=crates/viola-e2e/src/harness/boot.rs
what: edited · the test harness boot stamp; runs verify with a short owner-only runtime dir it owns
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs
what: edited · the test harness run and its expected verify step lines at 29
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: edited · the standing live rig: its floor rows, kept-stamp summary lines and labels
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited · the live rig's rehearsals; each case's own runtime dir, the command env carried over
@task_file n=23 path=tests/cli_verify.rs
what: edited · the verify integration tests; gains the cross-session-prefix cases
@task_file n=24 path=tests/cli_verify/runs.rs
what: edited · the verify run integration tests
@task_file n=25 path=tests/cli_verify/stamps.rs
what: edited · the verify stamp integration tests
@task_file n=26 path=tests/cli_fake_agent.rs
what: edited · the fake agent receipt tests; the start receipt key pin at nine keys
@task_file n=27 path=tests/cli_send.rs
what: edited · the send integration tests; a recorded session id literal
@task_file n=28 path=tests/chaos_revive.rs
what: edited · the revive chaos tests; a recorded session id literal
@task_file n=29 path=tests/contract_diag_schema.rs
what: edited · the diagnostics schema contract test
@task_file n=30 path=tests/contract_fake_agent_drift.rs
what: edited · the fake agent drift contract against the recorded dialog sets
@task_file n=31 path=tests/contract_fixture_hygiene.rs
what: edited · the recorded fixture hygiene contract; walks the cross-session variant
@task_file n=32 path=tests/contract_ledger_probes.rs
what: edited · the ledger probes contract over every stamped set
@task_file n=33 path=tests/security_negatives_env_capture.rs
what: edited · the capture arm security negatives; the names-only canary case
@task_file n=34 path=tests/support/home.rs
what: edited · the root test support's home and wrapper start helpers; a test-owned socket root as runtime dir
@task_file n=35 path=tests/support/start.rs
what: edited · the root test support's start helpers; a recorded session id literal
@task_file n=36 path=tests/support/verify.rs
what: edited · the root test support's verify helpers; verify with and without a runtime dir
@task_file n=37 path=fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json
what: added · the recorded claude hook fixture of the cross-session arrival, the CLI's own prefix around the sender's message
@task_file n=38 path=fixtures/claude/2.1.328
what: replaced · the recorded 2.1.328 claude hook fixture set, re-recorded whole by the live rig, recorded session ids moved
@task_item n=1 part=touches
The capability ledger's relied set gains a 29th row, cross-session-prefix; rows on the per-behaviour ledger and on the ledger as the single gate, and the per-row ledger decisions beside identity-floor, trust-record and the local-command rows.
@task_item n=2 part=touches
Verify's bounded child set gains a child: the print-mode sender verify-sender, spawned from Run B's cross-session turn with a 120 s deadline and a 30 s window; Run B's own args gain --settings with the crossSessionInbound accept literal.
@task_item n=3 part=touches
The relied count moved 28 to 29: any row whose cell holds the figure (verify's step line [NN/MM], the summary line, a version verified only when every relied row counts, the version gate's stamp read) is touched only where it states 28; rows stating it by rule are not.
@task_item n=4 part=touches
Probe capture arms inert and the viola hook --capture arm's payload file and probe-capture payload-file items: a hidden --names-only mode, valid only with --capture, files only Tool.k files of closed words, 0600, no payload byte.
@task_item n=5 part=touches
Environment variables carry the VIOLA_ prefix and environment variables are no configuration channel: verify now reads XDG_RUNTIME_DIR once, in memory, solely to compose the sender's address; the socket-dir rows that name XDG_RUNTIME_DIR as the socket dir's reader alone go stale.
@task_item n=6 part=touches
The ledger stamps envelope's measured items: measured.cross_session_probe added beside env_probe, first_screen and first_settle_ms (outcome, prefix, latency_ms, sender_tools, send_result).
@task_item n=7 part=touches
Diagnostics line subject set: verify-sender added to the subject enum of the diag-line schema; the composed address enters no line.
@task_item n=8 part=touches
Start verifies a new CLI version when keyed (and the automatic verify under its attempt lock): the automatic verify runs the same measure, so it runs the sender too and inherits the start's XDG_RUNTIME_DIR.
@task_item n=9 part=touches
Verify's CLI transcripts are an accepted residual: a new residual lands beside it (the address reaching the model provider through the sender's prompt, the pid-reuse window, closed-word files left on SIGKILL).
@task_item n=10 part=touches
Harness prompts classified by raw prefix and harness prompts are not a human take: a harness-prefix lookup with its id list now names which literal matched; prompt origin rewritten over it with the same result; the cross-session reading rests on that prefix.
@task_item n=11 part=touches
Claude flag literals in the agent crate: new compiled literals --allowedTools, the uds scheme, the cc-socks dir, the .sock suffix and the 103-byte path bound.
@task_item n=12 part=touches
viola-agent-claude does no file I/O: the uds address composer is pure; verify reads, lists and stats no directory to compose it.
@task_item n=13 part=touches
Recorded fixtures scrubbed and hygiene-walked, and the unclean-fixture absolute-path refusal: the new cross-session fixture keeps a runtime-dir path (with the uid) and the sender pid inside the CLI's prefix; the scrub rewrites only home and user, the walk reads only home-style absolute paths, so both pass it.
@task_item n=14 part=touches
Verify's Run E resumes Run B's first session: Run B's cross-session turn is no longer gated on the resume target, which Run E alone keeps.
@task_item n=15 part=touches
Status-line interception absence (no --settings at start): verify's Run B now passes --settings with the accept literal; the wrapped start still passes none.
@task_item n=16 part=touches
Test channel sockets under a short root in the OS temp dir and each test in its own viola home: every test and harness verify, and the wrapper start helper, now take a short owner-only test-owned runtime dir (vr-*, vs-* in the OS temp dir).
@task_item n=17 part=touches
Pre-push children in a clean environment: env -i leaves XDG_RUNTIME_DIR unset, so the harness and rig rehearsals supply one, and a rebuilt command carries its env over.
@task_item n=18 part=touches
Fake-agent test seams and tests against the fake agent: the fake sender resolves its sibling by uds address, no longer by session id; the start receipt gains pid and prompt.
@task_item n=19 part=touches
Verify removes only the probe folders it created and the drop-guarded probe dirs: the sender's plugin dir and its Tool capture dir join verify's probe folders.
@task_item n=20 part=touches
Standing live rig and its floor rows: FLOOR_ROWS gained the row under its rule every relied row but four; labels XS, XS-2 and the cross-session summary lines added.
@task_fact n=1 slug=established-decisions
fact: Cross-session-prefix ledger row, the 29th relied row: verify's Run B, started with the crossSessionInbound accept settings, receives a message from a print-mode sender addressed to Run B's CLI inbox socket; it reads pass only when the arrival's prompt opens with the CLI's cross-session harness prefix, fail otherwise, never from the sender's own tool or result words, and not-run with no runtime dir.
@task_fact n=2 slug=established-decisions
fact: A local claude session is addressed for a cross-session send by its inbox socket uds address, uds:<runtime>/cc-socks/<pid>.sock, never by a bare session id, which the CLI resolves to no local session; the address is composed only when the runtime dir is absolute, has no . or .. component, holds only [A-Za-z0-9/._-] and the path stays within 103 bytes.
@task_fact n=3 slug=cross-cutting-patterns
fact: Exception to the VIOLA_ prefix and no-environment-configuration rules: viola verify reads XDG_RUNTIME_DIR once, held in memory, used only to compose the sender's address; its value reaches the sender's prompt argv (and through it the model provider) alone, never a log line, stamp, error or step line.
@task_fact n=4 slug=occupied-resources
fact: viola hook --capture --names-only is a hidden mode valid only with --capture: on PreToolUse or PostToolUse it files only Tool.k files of closed tool and send-result words, owner-only, no payload byte; spine events and a relative dir file nothing.
@task_fact n=5 slug=standard-contracts
fact: The ledger stamp's measured.cross_session_probe holds outcome, prefix, latency_ms, sender_tools and send_result, closed words and one number.
@task_fact n=6 slug=occupied-resources
fact: verify-sender is verify's print-mode sender child, claude -p with --model haiku, --no-session-persistence, --allowedTools SendMessage and a sender plugin dir, under a 120 s deadline; it is a diag-line subject and its address enters no line.
@task_fact n=7 slug=occupied-resources
fact: Exception to recorded fixtures scrubbed: the runtime dir's value reaches no file verify writes of its own, but a recorded cross-session arrival keeps, inside the CLI's own prefix, the sender's socket address (the recording host's runtime-dir path with its uid, and the sender pid) and a from-name number; the scrub and the hygiene walk pass it.
@task_fact n=8 slug=occupied-resources
fact: Every test or harness start that stamps the relied rows gives verify a short owner-only test-owned runtime dir, because pre-push's env -i leaves XDG_RUNTIME_DIR unset; the rig rehearsal sets its own only where the host sets none.
@task_item n=1 part=untouched
The wheel, the send path, dialog holds and answers, and links: no rule on taking, releasing or confirming changed.
@task_item n=2 part=untouched
The MCP server, its tools and results, and the wrapper channel's methods, frames and fault codes.
@task_item n=3 part=untouched
The events log, instance snapshots, list and the board, and revive.
@task_item n=4 part=untouched
The wrapped child's argv and environment contracts and the start order: the wrapped run passes no new flag and keeps its env strip.
@task_item n=5 part=untouched
Dependencies, toolchain, cargo-deny policy, CI workflows and release: nothing added or removed; the CLI tool version proven stays 2.1.328.
@task_item n=6 part=untouched
CLI exit codes and refusal codes of every verb, verify's included, and config.json keys: no code, refusal word or config key added.
@task_item n=7 part=untouched
The PTY seam, the vt100 screen model and readiness gate, and the Hyprland seam.
