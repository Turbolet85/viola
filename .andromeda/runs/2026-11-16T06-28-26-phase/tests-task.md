@task_prose part=chunk
A driver's send of a local slash command that the CLI runs reads as delivered (ok), confirmed by an echo of that command the product can observe, never presumed; a local command the CLI did not run is never confirmed by it.
Today a local command off the compiled list waits for a prompt-submitted the CLI never fires and ends not-delivered / no-prompt-submitted although it ran; a listed command with no measured post-condition, or on an unverified CLI, reads ok with confirmed false, detail unconfirmable.
The echo is measured, not assumed: P3 re-measures from the code and the recorded per-version fixtures what echo the CLI gives and which commands it covers; an echo the product relies on becomes a capability-ledger row with a viola verify probe.
The chunk re-reads what a send of a local command reads today (the listed post-conditions, unconfirmable, the off-list no-prompt-submitted) across the send path, the CLI send human mirror and the MCP send result.
Capability v2-23; the plan stands on the fake agent and its local-command mode, with no live claude start and no mutation run.
@task_file n=1 path=src/run/send.rs
what: the wrapper's driver send path — typing the text, the confirmation wait and the send outcome it replies with
@task_file n=2 path=src/run/send/classify.rs mark=edits
what: the send classifier deciding what confirms a send (prompt-submitted, a local command's post-condition, unconfirmable) — edits: the classification of a local command, today Confirms::Prompt for a command off the compiled list
@task_file n=3 path=src/run/send/slot.rs
what: the send confirmation slot the send path waits on for its confirming fact
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude capability ledger and the compiled local-command list with each command's post-condition — edits: the local-command list, a new ledger row if the echo is relied on
@task_file n=5 path=src/cmd/verify/ mark=edits
what: viola verify, the ledger rows' probes against the CLI and the stamp it writes — edits: the probe of the echo's ledger row
@task_file n=6 path=src/run/version_gate.rs
what: the unverified CLI version gate, reading which relied ledger rows a stamp holds
@task_file n=7 path=src/human/mirror.rs
what: the CLI send human-mode mirror of the send result, with unconfirmable's fixed note
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent serving the claude boundary — edits: its --local-command-mode
@task_file n=9 path=fixtures/fake-scripts/
what: the fake agent turn scripts
@task_item n=1 part=needs
A driver send of a local slash command the CLI runs reads ok, confirmed by that command's own observed echo; a local command the CLI did not run is never confirmed by an echo — the driver send local-command outcome and the viola send confirmed result.
@task_item n=2 part=needs
A slash text off the compiled local-command list that the CLI runs no longer ends not-delivered / no-prompt-submitted after its window — the changed reading of the off-list slash send and of the unconfirmed-send outcome.
@task_item n=3 part=needs
The unconfirmable reading (ok, confirmed false) of a listed local command with no measured post-condition or on an unverified CLI: what it reads after the change, in the send result, the CLI send --json document, the human-mode unconfirmable line with its fixed note, and the MCP send result.
@task_item n=4 part=needs
Local commands are matched against the compiled list by exact equality, and a leading-slash text is never taken as an ordinary prompt; the post-condition rule (local commands confirmed by a post-condition only) is what the echo extends or joins.
@task_item n=5 part=needs
Every dependence on an undocumented claude behaviour is a capability-ledger row with a viola verify probe: an echo the send relies on is a ledger row, measured and stamped; the version gate needs every relied row, and a stamp refuses a widened ledger.
@task_item n=6 part=needs
Claude shapes stay in the Claude agent crate: what counts as a local command's echo is read there and the send path consumes a normalised fact; if the echo is a screen line, it is read only as a compiled signature of the readiness screen model, never by parsing the rendered child screen.
@task_item n=7 part=needs
Upstream text (a command's output) is content, never a viola command or config, and crosses no log line, event or error body — logs carry no secrets, error bodies hold no upstream text.
@task_item n=8 part=needs
The send confirmation window and its expiry bound the wait for the echo the same way they bound a prompt-submitted wait; the send-issued and send-confirmed records and their log lines (with corr) carry the local-command send.
@task_item n=9 part=needs
The change to local-command classification leaves the prompt-submitted match for ordinary sends (with its trailing-whitespace trim tolerance) and the /clear and /compact post-conditions as measured exactly as they are.
@task_item n=10 part=needs
The CLI send and the MCP send read the wrapper's reply as they already do and return identical payloads; the MCP tool surface's shape does not change.
@task_item n=11 part=needs
The fake agent's local-command mode and its input handling of local commands (/clear, /compact, other slash commands) carry the echo, held to the recorded per-version Claude fixtures byte for byte; the fake agent is configured by argv alone.
@task_item n=12 part=needs
No live claude start in the plan: real claude is local-only and never in CI, a live reading is not a test layer; if one is needed the plan counts the starts, runs each under a private CLAUDE_CONFIG_DIR inside a started session's turn, and fixtures are recorded only by live verify.
@task_item n=13 part=needs
No mutation run: each control is a bounded remove-the-guard witness with its --timeout named in the plan, on an entry filtered to the guard's own cases, a control id within 16 characters, holding when those cases ran and failed with no TIMEOUT line.
@task_item n=14 part=needs
Every acceptance threshold is checked against the class's own CI kill line and mutants kill (the send-window class's mutants kill, the stamped-home kill line, verify-driven per-test kill).
@task_item n=15 part=needs
A test waiting on a channel or a thread takes a bound and never hangs past a failure; waits key on logged events, never sleeps or retries.
@task_item n=16 part=needs
A test socket goes under the short /tmp root; a stamped home comes from the shared stamp's copy, stamps written only by verify against the fake agent.
@task_item n=17 part=needs
The new ledger row's probe joins the ledger probe contract suite, and verify's step and summary lines report it as for any row.
@task_item n=18 part=needs
A fold is decided at the phase: implement meeting a red that moves a plan entry stops and says so before the operator pass.
@task_standing n=1
convention: each new test is named by the test function naming rule, table cases in rstest, unit tests inline
home: 2-test-strategy
@task_standing n=2
convention: each new integration or E2E binary is prefixed by its surface and runs over the fake agent, asserting on events, never on screen content
home: test-strategy
@task_standing n=3
convention: each new test creates its own private home, product state produced by viola, never hand-written
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=4
convention: each new fixture or fake-agent script carries synthetic text only, passes the fixture hygiene walk and is scrubbed
home: 7-test-data-fixtures
@task_standing n=5
convention: each new test avoids sleeps, retries, process-wide env mutation and shared mutable state, asserts literal oracles on public behaviour
home: 11-test-anti-patterns-never-do-these
@task_standing n=6
convention: each new log line is a structured JSON line with its corr, codes only, detail apart
home: log-format
@task_standing n=7
convention: each new test class sits under its kill line, below cargo-mutants' timeout
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=8
convention: human-mode output a new test asserts is pinned by literals, no SGR without a terminal
home: integration-test-strategy
@task_item n=1 part=not_needed
Dialog hook decision bodies, dialog events, answer refusals and dialog holds: the dialog paths are untouched.
@task_item n=2 part=not_needed
The wait verb's wake kinds, timeouts and cursor rows: wait's wake rule is untouched.
@task_item n=3 part=not_needed
Revive refusal codes, revive exit codes and revive documents: the chunk changes no start or revive path.
@task_item n=4 part=not_needed
Harness boot, cleanup, supervisor, session record, status document and pre-push rows: the chunk changes no harness command.
@task_item n=5 part=not_needed
The list board, instance liveness and heartbeat staleness rows: nothing the chunk changes reads or writes them.
@task_item n=6 part=not_needed
Link and unlink methods and their records: a local-command send changes no linking.
@task_item n=7 part=not_needed
Release verb and wheel-return rows, and human-input wheel taking: the wheel's rules are untouched.
@task_item n=8 part=not_needed
Child environment strip, process-start records and pinned-exe hash checks: the chunk spawns nothing new.
@task_item n=9 part=not_needed
Hook performance budgets and hyperfine perf gates: send confirmation latency is not timed.
@task_item n=10 part=not_needed
Instance-unreachable causes and exit-21 details (socket-dir, strict-modes, server-verify): the chunk changes no reachability check.
