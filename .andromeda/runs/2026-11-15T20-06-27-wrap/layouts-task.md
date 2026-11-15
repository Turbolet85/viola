@task_prose part=session
A test-only session: no product symbol, no CLI verb, no output line, no exit code, no refusal and no hint changed.
It moved every test channel socket named viola-test-chan- to a short /tmp root ({temp root}/vs-XXXXXX), never under TMPDIR, through cfg(test) lines only in product source files.
It raised the nextest mutants profile default kill 10 s to 15 s and the shared test wait bound WITHIN 7 s to 12 s.
It added tests that kill the ten surviving mutants of the viola-hypr-tap recorder, and a record-home helper so a home given to viola verify --record in a test is never named home.
Measured: the whole workspace green under the mutants profile (3286 passed), a planted hang killed at 15 s, CI green 7/7; nothing retired.
@task_file n=1 path=tests/hook_fail_open.rs
what: the tests of the hook fail-open path, holding the impostor channel endpoint test helper and its sun_path length test
@task_file n=2 path=tests/support/home.rs
what: the shared test home support, the test-owned socket root and the new record-home helper for viola verify --record tests
@task_file n=3 path=tests/support/watch.rs
what: the shared test wait bound WITHIN the root test waits use, now 12 s
@task_file n=4 path=.config/nextest.toml
what: the nextest test-runner profiles, the mutants profile default kill now 15 s and its per-test overrides
@task_file n=5 path=tests/cli_verify.rs
what: the integration tests of viola verify output and --record, gaining the record-home naming test
@task_file n=6 path=tests/cli_verify/runs.rs
what: the viola verify --record run tests, now taking the record-home helper
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the viola verify stamp tests, now taking the record-home helper
@task_file n=8 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the e2e Hyprland reply recorder test tool, its inline test module only
@task_file n=9 path=crates/viola-e2e/tests/hypr_tap.rs
what: new integration tests of the Hyprland reply recorder argument forms and written replies, test-only
@task_file n=10 path=crates/viola-channel/src/lib.rs
what: the channel crate root, its test-only socket directory helper (cfg(test) lines only)
@task_file n=11 path=crates/viola-channel/src/client.rs
what: the channel client, its inline test module socket directories only
@task_file n=12 path=crates/viola-channel/src/server.rs
what: the channel server, its inline test module socket directories only
@task_file n=13 path=crates/viola-channel/src/tokio_client.rs
what: the async channel client, its inline test module socket directories only
@task_file n=14 path=crates/viola-e2e/src/harness/cleanup.rs
what: the e2e harness cleanup, its inline test module socket directory only
@task_file n=15 path=src/cmd/client.rs
what: the CLI side of the frame verbs reaching a wrapper over the channel, its inline test module socket directories only
@task_file n=16 path=crates/viola-channel/tests/channel_frames.rs
what: the channel frame integration tests, their socket directory
@task_file n=17 path=tests/security_negatives_server_identity.rs
what: the security negative tests of the wrapper server identity check, their socket directory
@task_item n=1 part=touches
contract/viola-verify-output/record: the tests that drive viola verify --record now take a home from a record-home helper never named home; the contract's output is unchanged, check whether the row names how its home is chosen in tests.
@task_item n=2 part=touches
contract/cli-global-flags/home: tests pass a --home to viola verify --record built by the new record-home helper; the flag itself is unchanged.
@task_item n=3 part=touches
contract/viola-verify-refusals/home-not-private: indirect, through the verify tests whose homes moved to the record-home helper; the refusal is unchanged.
@task_item n=4 part=touches
contract/frame-verb-hints/server-verify-failed and contract/instance-unreachable-detail/server-verify-failed: indirect, the server identity negative tests that read these now bind their sockets under the short /tmp root; the hint and detail are unchanged.
@task_item n=5 part=touches
contract/frame-verb-hints/socket-dir-refused, contract/instance-unreachable-detail/socket-dir-refused and contract/run-revive-start/socket-dir-refused: indirect, test socket directories moved; the product socket directory and its refusal are unchanged.
@task_item n=6 part=touches
contract/viola-wait-output/timed-out and contract/mcp-tools/wait: a picker may read the WITHIN 12 s move as a wait bound; WITHIN is a test deadline, the product wait bound is unchanged — flag only where a row's cell holds the 7 s test bound.
@task_item n=7 part=touches
contract/viola-send-output rows (input-not-ready-hint, issue-line): indirect, the send-under-the-paste-hint test override keeps its own 30 s kill while the default kill moved to 15 s; the output is unchanged.
@task_item n=1 part=untouched
The surface-web-spa area and absence/web-surface: no page or web surface is reached.
@task_item n=2 part=untouched
The board and list styling decisions and the viola list board contracts: no column, colour, truncation, ordering or styling changed.
@task_item n=3 part=untouched
The CLI exit codes, the --json document forms and the refusal line form: no code, field or line changed.
@task_item n=4 part=untouched
The hint wording decisions and the readback mirror and readback vocabulary rows: no hint or word changed.
@task_item n=5 part=untouched
The viola revive, viola run, link and plugin install contracts: their verbs, preflight and output are unchanged.
@task_item n=6 part=untouched
The token rendering contracts and the CLI parsing, TTY detection and SGR styling decisions: no token, dependency or detection changed.
@task_item n=7 part=untouched
The MCP verb and MCP tool result rows: no tool, result shape or hidden verb changed.
@task_item n=8 part=untouched
absence/cli-deep-links: no link or URL coupling added.
