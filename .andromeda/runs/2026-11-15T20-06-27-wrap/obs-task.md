@task_prose part=session
A test-only session: no product symbol, behaviour, log line, event, detail code or schema changed; every product-source edit sits inside a test module.
The mutants nextest profile's default per-test kill moved from 10 s to 15 s (a recorded departure from the never-raise-the-default line); every per-test override kept its own kill and the ci profile is unchanged.
The shared root wait bound for tests (WITHIN) moved from 7 s to 12 s, still below the 15 s kill; the count of root-wait sites is unchanged.
Every test socket named viola-test-chan-pid-label now binds under a short /tmp root ({temp root}/vs-XXXXXX), never under TMPDIR, so a long TMPDIR no longer overflows sun_path; the product socket placement is unchanged.
Ten mutation survivors in the hyprctl reply recorder (viola-hypr-tap) gained killing tests in a new viola-e2e integration test binary, and tests that give viola verify --record a home now take a record-home helper whose path never has a component named home.
Measured: the whole workspace passed under the mutants profile (3286 tests); CI green 7/7 on the pushed commit.
@task_file n=1 path=tests/hook_fail_open.rs
what: the integration tests of the hook fail-open path, including the impostor channel endpoint that a hook must not trust; edited so the impostor socket sits under the short /tmp test socket root, with a test that it fits sun_path
@task_file n=2 path=tests/support/home.rs
what: the shared test-home support (TestHome, the test socket root); edited to add the record-home helper and fixture for viola verify --record and a check that a path names no home component
@task_file n=3 path=tests/support/watch.rs
what: the shared test wait bound WITHIN used by the root waits and fake::wait_for; edited 7 s to 12 s
@task_file n=4 path=.config/nextest.toml
what: the nextest profiles, including the mutants profile's default slow-timeout kill and per-test overrides; edited the default kill 10 s to 15 s and its comments
@task_file n=5 path=tests/cli_verify.rs
what: the integration tests of viola verify; edited to add the record-home test and import the helper
@task_file n=6 path=tests/cli_verify/runs.rs
what: the viola verify run tests; edited so their --record callers take the record-home helper
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the viola verify capability-stamp tests; edited so their --record callers take the record-home helper
@task_file n=8 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the hyprctl reply recorder used to capture scrubbed Hyprland fixtures; edited its inline test module only
@task_file n=9 path=crates/viola-e2e/tests/hypr_tap.rs
what: a new integration test binary for the hyprctl reply recorder's argument surface, run against a stub hyprctl; edited (new file)
@task_file n=10 path=crates/viola-channel/src/lib.rs
what: the viola-channel IPC crate root; edited its test-only capture module to add a short /tmp test socket directory
@task_file n=11 path=crates/viola-channel/src/client.rs
what: the viola-channel synchronous client; edited its test module only (test sockets under the short /tmp root)
@task_file n=12 path=crates/viola-channel/src/server.rs
what: the viola-channel server with peer-credential checks; edited its test module only (test sockets under the short /tmp root)
@task_file n=13 path=crates/viola-channel/src/tokio_client.rs
what: the viola-channel Tokio client used by viola mcp; edited its test module only (test sockets under the short /tmp root)
@task_file n=14 path=crates/viola-e2e/src/harness/cleanup.rs
what: the e2e harness cleanup of test instances; edited its test module only (test sockets under the short /tmp root)
@task_file n=15 path=src/cmd/client.rs
what: the CLI client verbs' connection to an instance channel; edited its test module only (test sockets under the short /tmp root)
@task_file n=16 path=crates/viola-channel/tests/channel_frames.rs
what: the channel frame integration tests; edited to bind their sockets under a short /tmp root
@task_file n=17 path=tests/security_negatives_server_identity.rs
what: the security negative tests of channel server identity verification; edited to bind their sockets under the short /tmp test socket root
@task_item n=1 part=touches
Obs code mutation-tested like product code: the mutants profile that runs it now kills a test at 15 s by default, not 10 s; any row citing the mutants kill or its value reads stale.
@task_item n=2 part=touches
Surviving obs mutant is a failure: the hyprctl reply recorder's ten survivors are now killed by tests; a row counting or listing survivors there is affected.
@task_item n=3 part=touches
Rows naming the cargo-mutants 20 s floor: the default kill stays below it (15 s), the floor itself unchanged; a row stating the kill sits at 10 s under that floor is stale.
@task_item n=4 part=touches
No sleep-based log waits: tests wait for log lines on a deadline bounded by WITHIN, now 12 s; a row holding that bound's value is touched.
@task_item n=5 part=touches
Test homes under one root and under the scanned root, kept until the gates run: a new test home kind (the record home for viola verify --record, never named home) joins them; check the rule still holds for it.
@task_item n=6 part=touches
Test socket placement: every viola-test-chan- test socket now under a short /tmp root and never under TMPDIR or a test home; a row placing test sockets, or tying test socket paths to the scanned root or TMPDIR length, is touched.
@task_item n=7 part=touches
Hook fail-open logged and hook silent rows: their integration tests (the impostor endpoint case) were changed in where the impostor socket lives, behaviour asserted unchanged.
@task_item n=8 part=touches
Server check by peer credentials and server euid must match: their security negative tests and the IPC crate's inline tests moved only their socket directory; behaviour unchanged.
@task_item n=9 part=touches
CI verify against the fake agent only and real-CLI verify never in CI: the verify test suite gained the record-home helper and test; still fake-agent only.
@task_item n=10 part=touches
The hyprctl-probe subject and hyprctl-json parser rows: the recorder that produces the hyprctl fixtures gained tests; fixtures not a telemetry source still holds.
@task_item n=11 part=touches
Mutation outputs and mutation diff never uploaded: the mutants scratch homes left on disk stay local, nothing uploaded.
@task_item n=12 part=touches
OS-gated test census: a new integration test binary (hypr_tap) adds three tests to the workspace count.
@task_item n=1 part=untouched
Process log line fields, event enum, corr and conn correlation, detail codes and exit codes: no product line, event or code changed.
@task_item n=2 part=untouched
Span naming, span attributes, span kinds and span chains: no span declared or changed.
@task_item n=3 part=untouched
OTel, exporters, error reporters, Sentry, network egress and their absences: nothing added.
@task_item n=4 part=untouched
PII scrubbing, redaction, veil, the NEVER-log floor and secret scanning of uploads: unchanged.
@task_item n=5 part=untouched
Panic capture and panic outcome by role: unchanged.
@task_item n=6 part=untouched
viola mcp tool contracts and MCP stdin framing: unchanged.
@task_item n=7 part=untouched
Hook performance budgets, hyperfine gates and heartbeat staleness thresholds: unchanged; the 12 s and 15 s values are test-harness bounds, not product or perf budgets.
@task_item n=8 part=untouched
Diagnostics file paths, modes, home resolution and log rotation: unchanged.
@task_item n=9 part=untouched
CI workflow steps, artifact retention and pinned actions: no CI step changed; the ci nextest profile is unchanged.
