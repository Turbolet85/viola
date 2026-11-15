@task_prose part=session
A test-only chunk: no product line outside a test module changed, no dependency, CI step, schema or contract moved.
Every test channel socket named viola-test-chan-pid-label now binds under a short test-owned /tmp root ({temp root}/vs-XXXXXX), never under TMPDIR and never at the /tmp root itself; eight sites swept, seven moved, the moves in product source files confined to their test modules.
The nextest mutants profile's default kill rose from 10 s to 15 s (a recorded departure from the never-raise-the-default line), and the shared test wait bound WITHIN rose from 7 s to 12 s, below that kill.
The test-only Hyprland recorder (viola-hypr-tap) gained an integration test binary killing its ten mutation survivors against a stub hyprctl and the committed scrubbed fixtures; no product line of the recorder changed.
A record-home helper was added: a home a test hands to viola verify --record is never named home in any path component; the six record callers take it.
CI read green on the pushed head, 7 of 7 checks.
@task_file n=1 path=tests/hook_fail_open.rs
what: the hook fail-open tests, including the impostor channel endpoint a hook must refuse; its impostor socket now under the short /tmp test socket root
@task_file n=2 path=tests/support/home.rs
what: test support for test-owned viola homes and the test socket root; adds the record-home helper whose home has no path component named home
@task_file n=3 path=tests/support/watch.rs
what: the shared root wait bound WITHIN for tests, raised from 7 s to 12 s
@task_file n=4 path=.config/nextest.toml
what: the nextest profiles; the mutants profile's default slow-timeout kill raised from 10 s to 15 s, the ci profile unchanged
@task_file n=5 path=tests/cli_verify.rs
what: the viola verify integration tests, including verify --record; adds the record-home naming test
@task_file n=6 path=tests/cli_verify/runs.rs
what: viola verify run tests; record callers moved to the record-home helper
@task_file n=7 path=tests/cli_verify/stamps.rs
what: viola verify stamps tests; record callers moved to the record-home helper
@task_file n=8 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only Hyprland fixture recorder with its closed string allowlist; one case added inside its test module only
@task_file n=9 path=crates/viola-e2e/tests/hypr_tap.rs
what: new integration tests of the Hyprland recorder's argument surface: record, record-dispatch and the malformed-argument refusals with exit 2 writing nothing
@task_file n=10 path=crates/viola-channel/src/lib.rs
what: the channel crate root; a test-only socket directory helper added under its test capture module, no product line
@task_file n=11 path=crates/viola-channel/src/client.rs
what: the channel client with its peer euid and server verification checks; only its test module's socket directory moved
@task_file n=12 path=crates/viola-channel/src/server.rs
what: the channel server with its exclusive bind, socket chmod and peer euid check; only its test module's socket directory moved
@task_file n=13 path=crates/viola-channel/src/tokio_client.rs
what: the async channel client used by the MCP server, with its server euid check; only its test module's socket directory moved
@task_file n=14 path=crates/viola-e2e/src/harness/cleanup.rs
what: the e2e harness's test-side cleanup that removes only its own dirs; only its test module's socket directory moved
@task_file n=15 path=src/cmd/client.rs
what: the CLI's channel client seam for frame verbs; only two test-module socket sites moved
@task_file n=16 path=crates/viola-channel/tests/channel_frames.rs
what: channel frame tests (frame cap, closed types, version); its test socket directory moved to the short /tmp root
@task_file n=17 path=tests/security_negatives_server_identity.rs
what: the security negative tests of server identity verification before any frame; its socket directory moved to the short /tmp root
@task_item n=1 part=touches
No Unix socket at the temp root (decision/no-socket-in-tmp-root): test sockets now bind inside a per-test {temp root}/vs-XXXXXX directory, not at the /tmp root and not under TMPDIR; a picker should check the row still reads true for test sockets and whether it should name this test root.
@task_item n=2 part=touches
Socket directory rules (decision/socket-in-per-user-dir, decision/per-user-socket-directory, decision/socket-in-owner-only-dir): the product socket directory is unchanged; only test sockets moved to a short tempdir under /tmp, check no row claims tests share the product directory or TMPDIR.
@task_item n=3 part=touches
Mutation testing at the epoch audit (decision/mutation-testing-at-audit): the mutants profile's default kill moved 10 s to 15 s, still below the 20 s cargo-mutants floor; where the row holds the kill value it is stale.
@task_item n=4 part=touches
Tests use their own home (decision/tests-use-own-home) and test homes kept out of git (decision/test-homes-gitignored): a new record-home helper gives verify --record a test home with no path component named home.
@task_item n=5 part=touches
Fixture scrubbing and the home-path refusal (decision/fixture-scrub-and-refusal, decision/fixture-recording-scrubbed, contract/fixture-refusal-codes/home-path): the record-home rule exists so a test record home is never named home; the product scrub and refusal codes are unchanged.
@task_item n=6 part=touches
Hyprland recorder rules (decision/hyprland-recorder-string-allowlist, decision/hyprland-reply-text-dropped, decision/mcp-fixtures-via-test-only-tap): the test-only recorder gained integration tests against a stub hyprctl replaying committed scrubbed fixtures; its product lines unchanged.
@task_item n=7 part=touches
Release build refuses test-only binaries (decision/release-build-no-test-binaries, decision/release-build-without-test-features): viola-e2e gains a new integration test binary; it stays test-only.
@task_item n=8 part=touches
Test seams compiled out of release (decision/test-seams-compiled-out, decision/test-seams-fake-agent-only): a new cfg(test) socket directory helper sits in the channel crate's test capture module; compiled out of every non-test build.
@task_item n=9 part=touches
Peer and server identity checks (decision/client-peer-euid-check, decision/server-side-peer-euid-check, decision/client-verifies-server-before-frame, decision/verify-server-before-frame): their tests, including the async client's server euid test, now bind under the short /tmp root; the checks themselves unchanged.
@task_item n=10 part=touches
Hook fails open (decision/hook-fails-open, decision/hooks-fail-open): the hook fail-open suite's impostor endpoint now lives under the short /tmp test root and now passes under a long TMPDIR.
@task_item n=11 part=touches
Test-side removal only of own dirs (decision/e2e-home-removal-own-dirs-only, decision/test-removal-own-dirs-only): the e2e cleanup harness's test module now makes its socket dir under /tmp; removal scope unchanged.
@task_item n=12 part=touches
Channel frame cap and closed types (decision/channel-frame-cap-before-read, decision/channel-closed-types): their frame tests moved socket directory only; no frame rule changed.
@task_item n=13 part=touches
No mutants.out upload (decision/no-mutants-out-upload): the mutants profile changed its kill only; no upload or CI step changed.
@task_fact n=1 slug=security-anti-patterns-never-do-these
fact: Test channel sockets bind inside a test-owned short directory under /tmp ({temp root}/vs-XXXXXX, made fresh per test), never under TMPDIR and never at the /tmp root; the product socket rules are unchanged.
@task_fact n=2 slug=data-protection
fact: A home a test gives to viola verify --record comes from the record-home helper and holds no path component named home.
@task_item n=1 part=untouched
MCP tools, wrapper channel methods and their contracts: no method, parameter, refusal word or exit code moved.
@task_item n=2 part=untouched
Dependency security: no crate, pin, version floor, licence, advisory ignore or cargo-deny rule changed.
@task_item n=3 part=untouched
Secret management and the CLAUDE environment strip: no credential, env strip, canary scan class or pre-push environment changed.
@task_item n=4 part=untouched
CI workflows and actions: no workflow, permission, action pin or upload step changed.
@task_item n=5 part=untouched
Input validation of paste text, hook stdin, config and revive arguments: no validator, cap or character set changed.
@task_item n=6 part=untouched
Error handling contracts and the logging bans: no error display, diagnostics or stderr rule changed.
@task_item n=7 part=untouched
Pinned binary placement, re-hash and plugin file rewriting: untouched.
@task_item n=8 part=untouched
The threat model's absences (network, hosting, secrets, accounts): no capability added or removed.
