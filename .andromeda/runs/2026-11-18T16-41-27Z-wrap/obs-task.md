@task_prose part=session
The session made the CLI's browser-tools prompt (Yes, use my browser) a third compiled modal literal, beside the trust and imports literals, and a sixth first-screen closed word browser-tools.
It added a 24th relied capability-ledger row browser-tools-prompt with three words pass, absent and fail; the relied row count moved 23 to 24 and the verify step counter to [NN/24].
The verified rule moved: a CLI version is verified when every relied row counts, that is reads pass, or absent on browser-tools-prompt alone; verify's step line can now end absent and its summary counts an absent row in n pass.
Run's pre-send gate and verify's settle read the prompt as a modal and type nothing into it; a send on a verified CLI is refused while the prompt stands.
The fake agent gained an argv-only option --start-screen and the live rig two arms VP and VK with summary lines cache-a and browser-tools-prompt; the rig reads only the key name of the CLI's extension cache in its own private config, by the founder's recorded answer.
Measured live on 2.1.325: BP showed the prompt and settled 8/24 with verify exit 1, BK read 24/24 with the row absent; no log line, span, detail code, env var or IPC method was added.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures of the claude adapter: the modal literals and the first-screen classification of an interactive start
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: its relied rows, each row's word, the stamp merge and the verified-version rule
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the capability ledger rows and their readings
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of the ledger row checks over verify's typed runs
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the ledger stamps and the verified-version reading
@task_file n=6 path=src/cmd/verify.rs
what: the viola verify command: its step lines, its summary line and its exit
@task_file n=7 path=src/cmd/verify/typed/pty.rs
what: verify's interactive PTY probe runs and their settle on a first screen
@task_file n=8 path=src/cmd/verify/typed/turns.rs
what: verify's typed turns over the interactive probe
@task_file n=9 path=src/run/gate.rs
what: the run readiness gate before a send: input-box readiness from the screen signatures
@task_file n=10 path=src/run/version_gate.rs
what: the run version gate: the ledger stamps verdict that makes a CLI verified or unverified
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent binary and its argv-only options
@task_file n=12 path=tests/cli_send.rs
what: integration tests of viola send and its refusals
@task_file n=13 path=tests/cli_verify.rs
what: integration tests of viola verify output: step lines, summary line, exit codes
@task_file n=14 path=tests/cli_verify/runs.rs
what: integration tests of verify's probe runs against the fake agent
@task_file n=15 path=tests/cli_verify/stamps.rs
what: integration tests of the stamps verify writes
@task_file n=16 path=tests/contract_ledger_probes.rs
what: the contract test of the ledger's relied row ids and probes
@task_file n=17 path=crates/viola-e2e/src/harness/run.rs
what: the agent-run harness: its local-live row list and the boot that stamps the ledger
@task_file n=18 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its arms, their labels and its summary lines
@task_file n=19 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated scratch home: its seeded markers and its private CLI config read
@task_file n=20 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsal tests against the fake agent
@task_item n=1 part=touches
contract/verify-stdout/step-line: a step line's verdict word can now be absent (on the browser-tools-prompt row), beside pass and fail, and the counter reads [NN/24].
@task_item n=2 part=touches
contract/verify-stdout/summary-line: n in n pass now counts the rows that count, an absent browser-tools-prompt row included; the all-pass line reads 24 pass 0 fail.
@task_item n=3 part=touches
contract/verify-exit-codes/0 and contract/verify-exit-codes/1: exit 0 now follows every relied row counting rather than every row passing; an unanswered browser-tools prompt leaves verify at exit 1.
@task_item n=4 part=touches
decision/unreadable-ledger-is-unverified and decision/unreadable-ledger-means-unverified: the verified reading over the stamp moved to every relied row counting, and a stamp of the 23 pre-chunk ids now reads unverified.
@task_item n=5 part=touches
decision/verify-sole-stamps-writer and decision/ledger-contents-never-logged: the stamp verify writes gains the row key browser-tools-prompt and the first-screen word browser-tools.
@task_item n=6 part=touches
contract/run-log-parse-rejected/ledger-stamps: the ledger-stamps parser now accepts the new row key and the closed word browser-tools; a stamp without the 24th row parses but reads unverified.
@task_item n=7 part=touches
contract/run-log/cli-verified and contract/run-span-attributes/run-version-gate: the version gate's verdict behind cli_verified now reads the 24-row rule.
@task_item n=8 part=touches
decision/manual-readiness-gate-span, contract/send-span-attributes/run-readiness-gate and contract/send-refused-detail/input-not-ready: the readiness gate now refuses at once on the browser-tools prompt as a modal; the refusal is an existing send refusal, no new detail word.
@task_item n=9 part=touches
decision/no-telemetry-from-screen and decision/no-telemetry-from-screen-content: the third modal literal and the new first-screen word are read from screen content into the stamp only, never into a log line.
@task_item n=10 part=touches
decision/verify-lines-carry-no-content and decision/verify-lines-redacted: verify's lines carry the new row id and verdict word only, no screen text.
@task_item n=11 part=touches
decision/no-obs-gate-on-verify-lines: verify's stdout gained a verdict word; no obs gate reads it.
@task_item n=12 part=touches
decision/claude-internals-not-a-contract and decision/transcripts-not-a-contract: the CLI's prompt wording is now a relied literal live on 2.1.325 only, and the test rig reads the presence of the key name cachedChromeExtensionInstalled in its own private CLI config, never a value (the founder's recorded answer).
@task_item n=13 part=touches
decision/ci-verify-fake-agent-only and decision/real-cli-verify-never-in-ci: the browser-tools row is proved in CI by the fake agent's --start-screen and live only by the local rig arms VP and VK.
@task_item n=14 part=touches
decision/no-typed-keys-in-telemetry and decision/typed-keys-never-logged: the fake agent's start screen writes one key receipt line per stdin byte in its test receipts, and the rig arms stop with no key typed.
@task_item n=15 part=touches
contract/agent-run-status/state and contract/agent-run-status/last-error: the harness boot now stamps 24 rows and the local-live row list accepts absent on the 24th row; status reads cli_verified true at 24 rows.
@task_item n=16 part=touches
contract/verify-spawn-subject/verify-pty-probe and contract/verify-process-exit/child-exit-status-and-duration: verify's PTY probe now settles on the browser-tools prompt as a modal, settling at once rather than at its timeout.
@task_fact n=1 slug=4-span-trace-coverage
fact: The browser-tools prompt reading is uninstrumented: it adds no log line, span, event or detail code, and reaches only the ledger stamp and verify's stdout.
@task_item n=1 part=untouched
The MCP stdio server, its tools, frames and refusals: no MCP surface moved.
@task_item n=2 part=untouched
Hooks, their fail-open behaviour, stdout decision bodies and latency budgets: no hook path changed.
@task_item n=3 part=untouched
The channel, JSON-RPC errors, corr and conn correlation: no IPC method, param or join key was added.
@task_item n=4 part=untouched
Panic capture, the panic line, the detail-file schema and error-chain handling: unchanged.
@task_item n=5 part=untouched
PII scrubbing, redaction, the field allow-list and secret scan rules: no logged field was added.
@task_item n=6 part=untouched
CI gates G1 to G4, artifact uploads, perf budgets and hyperfine thresholds: no gate, budget or threshold moved.
@task_item n=7 part=untouched
Diagnostics file paths, modes, home resolution, log levels and the tracing pipeline: no sink or level changed.
@task_item n=8 part=untouched
Metrics derivation, counters and histograms: no counted event was added.
@task_item n=9 part=untouched
Service identity, exit 1 and exit 21 detail codes, state-recovered and parse-rejected detail words other than the ledger stamps: unchanged.
@task_item n=10 part=untouched
The trust-record write and its start log line, the wheel and dialog paths: unchanged by this session.
