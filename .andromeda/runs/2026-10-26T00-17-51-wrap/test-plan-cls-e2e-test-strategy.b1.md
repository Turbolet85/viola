# Classified — test-plan · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · no · the viola list rendering, the viola --help literal and their test files hold since no product behaviour moved, and Cargo.lock is unchanged so the assert_cmd, predicates and trycmd pins hold
e2e-test-strategy#1.2 · no · the assert_cmd driver, its Instant deadline and its env_clear are unchanged, and the reached statusline file moved only unit log captures while no product behaviour moved
e2e-test-strategy#1.3 · no · the outer-PTY driver and the viola run passthrough are unchanged, the portable-pty pin holds on the unchanged Cargo.lock, and no product behaviour moved
e2e-test-strategy#1.4 · yes · add-to · F1, F2, F3, F4 and F10 add a Windows-only driver to this wrapper-channel row: the new cases bind its pipe through viola_channel's server and open it with raw CreateFileW clients, CreateNamedPipeW controls and viola-test-chan names
e2e-test-strategy#1.5 · no · the rmcp client over viola mcp and its crates/viola-e2e test files sit in no reached file, and no product behaviour moved
e2e-test-strategy#1.6 · no · the library-only runner (cargo-nextest and cargo test --doc) is unchanged, since the session's ruling against plain cargo test names the root bin's unit runner only
e2e-test-strategy#2.1 · no · the Path 1 steps drive viola run, viola list and the channel squat, and the session report shows no product behaviour moved
e2e-test-strategy#2.2 · yes · stale · src/cmd/run.rs is reached and its unit captures moved onto with_scoped_default, and this entry cites the unit test it holds there, which the session report does not show still holds
e2e-test-strategy#2.3 · yes · stale · Cargo.toml is reached and gained a Windows dev-dependency table, and this entry's [[test]] required-features line lives in that file, which the session report does not show still holds
e2e-test-strategy#2.4 · no · the Path 1 cleanup (drop guard, TempDir and squatter listener) sits in no reached file, and no product behaviour moved
e2e-test-strategy#3.1 · no · the path2 coverage names tests in tests/cli_send.rs and crates/viola-e2e, neither reached, and no product behaviour moved
e2e-test-strategy#3.2 · no · the send steps drive viola send, the fake agent and rmcp, none of them a reached file, and no product behaviour moved
e2e-test-strategy#3.3 · no · the gate's 3 000 ms hold and 8.5 s refusal are product behaviour the session report shows unchanged, and no end-to-end case was added for the unit-tier claim
e2e-test-strategy#3.4 · no · the path2 cleanup is the agent-run cleanup command, and the session report shows no harness command was edited
e2e-test-strategy#4.1 · no · the wait and last coverage names tests in tests/cli_wait_last.rs, tests/chaos_wait_vanish.rs and viola-e2e, none reached, and no product behaviour moved
e2e-test-strategy#4.2 · no · the wait and last steps drive viola wait, viola last and the chaos helper, and the session report shows no product behaviour moved
e2e-test-strategy#4.3 · no · the wait and last signals are product behaviour the session report shows unchanged
e2e-test-strategy#4.4 · no · the wait and last cleanup is the harness cleanup and its sysinfo checks, which the session report shows unedited
e2e-test-strategy#5.1 · no · the answer coverage names tests in tests/cli_answer.rs and tests/hook_events.rs, neither reached, and no product behaviour moved
e2e-test-strategy#5.2 · no · the dialog steps drive the hook, wait, answer and pause verbs, and the session report shows no product behaviour moved
e2e-test-strategy#5.3 · no · the dialog decision bodies and refusals are product behaviour the session report shows unchanged
e2e-test-strategy#5.4 · no · the dialog cleanup is the harness cleanup, which the session report shows unedited
e2e-test-strategy#6.1 · no · the wheel coverage names tests in tests/tui_wheel.rs, tests/cli_wheel.rs and src/run/wheel.rs, none reached, and no product behaviour moved
e2e-test-strategy#6.2 · no · the wheel steps drive run, send, release and pause, and the session report shows no product behaviour moved
e2e-test-strategy#6.3 · no · the wheel signals and refusals are product behaviour the session report shows unchanged
