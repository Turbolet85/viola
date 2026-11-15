@task_prose part=session
A test-only chunk: no product line changed, and every product-file edit sits inside a test module.
The mutants nextest profile's default kill moved from 10 s to 15 s, a stated departure from the rule never to meet a slow class by raising the default; every per-class override kill (60 s, 30 s, 30 s, 30 s) and the ci profile are unchanged.
The shared root wait bound WITHIN moved from 7 s to 12 s, still below the 15 s kill; the wait sites it bounds (Instant::now() + WITHIN, EXIT_WITHIN, fake::wait_for) take it unchanged, and none was added or removed.
Every test socket named viola-test-chan-{pid}-{label}.sock now binds under a short /tmp root ({temp root}/vs-XXXXXX, tempfile tempdir_in /tmp) and never under TMPDIR, so a long scratch TMPDIR no longer overflows sun_path; the sweep covered eight sites in four crates.
A test home given to viola verify --record now comes from a record-home helper (TestHome::for_record, rstest fixture record_home) whose path has no component named home, checked by record_home_names_no_path_component_home.
The ten surviving mutants of the test-only Hyprland reply recorder (viola-hypr-tap) are now killed by a new viola-e2e integration binary hypr_tap (three cases over a stub hyprctl) and one inline case; workspace run under the mutants profile: 3286 passed.
@task_file n=1 path=tests/hook_fail_open.rs
what: edited — the hook fail-open integration suite, its impostor server endpoint (the impostor listener's socket now under the short /tmp socket root) and a case that the endpoint fits sun_path at the widest pid and label
@task_file n=2 path=tests/support/home.rs
what: edited — the shared test home support: the per-test home, the test-owned socket root, and the new record-home helper and its rstest fixture for homes given to viola verify --record
@task_file n=3 path=tests/support/watch.rs
what: edited — the shared root wait bound WITHIN (and EXIT_WITHIN) that every root integration test's event wait is bounded by
@task_file n=4 path=.config/nextest.toml
what: edited — the nextest profiles: the mutants profile's default slow-timeout kill and its per-class kill overrides, with their comments
@task_file n=5 path=tests/cli_verify.rs
what: edited — the viola verify integration suite, now with the record-home rule case
@task_file n=6 path=tests/cli_verify/runs.rs
what: edited — the viola verify run cases, whose --record callers take the record-home fixture
@task_file n=7 path=tests/cli_verify/stamps.rs
what: edited — the viola verify stamp and fixture-recording cases, whose --record callers take the record-home fixture
@task_file n=8 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: edited — the test-only Hyprland reply recorder in the test-only E2E crate; one case added in its inline test module only
@task_file n=9 path=crates/viola-e2e/tests/hypr_tap.rs
what: added — a new integration test binary of the test-only E2E crate over the Hyprland reply recorder's argument surface, run against a stub hyprctl replaying committed scrubbed fixtures
@task_file n=10 path=crates/viola-channel/src/lib.rs
what: edited — the wrapper channel crate root; its test-only capture support gains the short /tmp socket directory helper for the crate's inline tests
@task_file n=11 path=crates/viola-channel/src/client.rs
what: edited — the sync wrapper channel client; its inline tests bind their sockets under the short /tmp root
@task_file n=12 path=crates/viola-channel/src/server.rs
what: edited — the wrapper channel server; its inline tests bind their sockets under the short /tmp root
@task_file n=13 path=crates/viola-channel/src/tokio_client.rs
what: edited — the Tokio wrapper channel client used by the MCP handlers; its inline tests (among them the server euid check) bind under the short /tmp root
@task_file n=14 path=crates/viola-e2e/src/harness/cleanup.rs
what: edited — the harness cleanup command; its inline tests' socket directory moved to the short /tmp root
@task_file n=15 path=src/cmd/client.rs
what: edited — the CLI's channel client glue for the frame verbs; two inline test sites' socket directory moved to the short /tmp root
@task_file n=16 path=crates/viola-channel/tests/channel_frames.rs
what: edited — the wrapper channel frame integration tests on a real endpoint; their socket directory moved to the short /tmp root
@task_file n=17 path=tests/security_negatives_server_identity.rs
what: edited — the security negative suite for the server identity check before any frame; its impostor socket directory moved to the short /tmp root
@task_item n=1 part=touches
decision/root-wait-bound: the root wait bound is now 12 s, not 7 s, and it now sits below a 15 s mutants default kill, not a 10 s one; both values in the row are stale.
@task_item n=2 part=touches
decision/within-7s-wait-bound: WITHIN is now 12 s, so the row's 7 s (in its name and its text) is stale.
@task_item n=3 part=touches
decision/mutants-kill-below-cargo-mutants-timeout: the mutants profile's default kill is now 15 s, not 10 s. It still sits below the 20 s cargo-mutants floor, and the row must record the departure from the rule never to raise the default, made on the operator's word (the stamped-home class is now the default population).
@task_item n=4 part=touches
decision/send-window-paste-hint-30s-kill: its reason was measured against the old 10 s default kill, and the default is now 15 s; the 30 s override kill itself is unchanged.
@task_item n=5 part=touches
decision/verify-window-60s-kill, decision/viola-e2e-30s-mutants-kill and decision/verify-driven-tests-20s-kill: their override kills are unchanged, but they now sit above a 15 s default, not a 10 s one. Check any wording that compares them to the default.
@task_item n=6 part=touches
decision/ci-stamped-home-kill-line and decision/mutants-fail-fast-wait: the mutants profile's default now kills stamped-home cases at 15 s. The ci profile and its kill line are unchanged.
@task_item n=7 part=touches
decision/waits-key-on-events, decision/no-test-retries-or-sleeps and decision/no-sleep-synchronization: test waits keyed on logged events keep their rule, and their shared deadline is now the 12 s WITHIN.
@task_item n=8 part=touches
decision/tempfile-test-dirs: test socket directories are now made with tempfile under a fixed short /tmp root (tempdir_in /tmp), not the default TMPDIR-relative tempdir. This is an exception to plain tempfile directories.
@task_item n=9 part=touches
decision/private-dir-for-test-endpoints and decision/test-endpoint-dirs-made-private: test endpoint directories now live under {temp root}/vs-XXXXXX. Check that the rule still names where they live and that they are private.
@task_item n=10 part=touches
decision/one-shared-test-helper and decision/shared-e2e-helpers: the short socket-directory helper now exists as separate copies in the channel crate's test capture, the harness cleanup tests, the channel frame tests and the root test support. Copies of one helper may contradict the one-shared-copy rule.
@task_item n=11 part=touches
contract/hook/impostor-server, decision/impostor-server-real-listener and decision/hook-dialog-no-frame-to-impostor: the impostor listener's socket directory moved to the short /tmp root. Its name viola-test-chan-{pid}-{label}.sock is unchanged.
@task_item n=12 part=touches
decision/hooks-fail-open and decision/hook-fails-open: the fail-open suite now passes under a long scratch TMPDIR, and a case pins the impostor endpoint to fit sun_path at the widest pid and label.
@task_item n=13 part=touches
decision/server-identity-verified, decision/client-verifies-server-identity and decision/peer-euid-checked: their negative tests and the Tokio client's server-euid case bind under the short /tmp root. Before, the euid case failed red under a 52 B scratch TMPDIR.
@task_item n=14 part=touches
decision/channel-tests-real-endpoint, decision/ipc-tested-on-real-endpoint and decision/wrapper-channel-integration-driver: the channel tests still run on a real endpoint, whose directory is now the short /tmp root.
@task_item n=15 part=touches
decision/per-test-home, decision/tests-create-own-home, decision/fresh-home-per-test and decision/per-test-fresh-home: a home that a test gives to viola verify --record now comes from a dedicated record-home constructor whose path has no component named home.
@task_item n=16 part=touches
decision/rstest-fixtures-for-constructors, decision/rstest-fixtures and decision/rstest-fixture-chain: a new rstest fixture record_home joins the shared test-home fixtures.
@task_item n=17 part=touches
decision/fixtures-recorded-by-live-verify, decision/record-only-at-zero-failing-rows and decision/recorder-scrub: the verify --record test callers now record into a record-home whose path has no component named home. The scrub itself is unchanged.
@task_item n=18 part=touches
decision/test-only-e2e-crate and decision/single-test-only-crate: the test-only E2E crate gains an integration test binary hypr_tap, and the crate count is unchanged.
@task_item n=19 part=touches
decision/e2e-layer-by-binary-prefix, decision/test-layer-filtersets and decision/integration-binary-surface-prefix: a new test binary hypr_tap in the E2E crate must fall into a layer under the binary-prefix and filterset rules.
@task_item n=20 part=touches
decision/external-processes-real-stubs and decision/agents-stub-from-files-no-env: the recorder's tests drive a real stub hyprctl executable on PATH that replays committed fixtures, with HOME set to the case's temp dir.
@task_item n=21 part=touches
decision/mutants-tested-by-owning-package and decision/mutation-run-verdict: the recorder's ten survivors are now caught by tests in its owning package, the E2E crate.
@task_item n=22 part=touches
decision/no-test-binary-in-release and decision/release-build-viola-only: the recorder binary changed only inside its test module, and it stays out of the release build.
@task_fact n=1 slug=test-data-fixtures
fact: A home that a test gives to viola verify --record comes from the record-home helper (TestHome::for_record, rstest fixture record_home), and no component of its path is named home; the live rig's half of the rule is carried to the standing live rig entry.
@task_fact n=2 slug=test-data-fixtures
fact: Every test-made channel socket (viola-test-chan-{pid}-{label}.sock) binds under a short test-owned socket root {temp root}/vs-XXXXXX and never under TMPDIR, so a long scratch TMPDIR cannot overflow sun_path. The convention holds in every crate that binds one.
@task_item n=1 part=untouched
Product contracts of the CLI, MCP tools, channel JSON-RPC errors, exit codes, event lines and JSON documents: no product line changed, and every edit to product source sits inside a test module.
@task_item n=2 part=untouched
CI integration: the workflows, jobs, artifacts, Actions pinning, supply-chain gates and the nextest ci profile are unchanged.
@task_item n=3 part=untouched
Coverage floors, coverage thresholds, coverage exclusions and the mutation gate thresholds and cadence: no threshold moved.
@task_item n=4 part=untouched
Performance budgets and the hyperfine perf gate, and the hook spine and connect deadlines: product deadlines are unchanged.
@task_item n=5 part=untouched
Agent-run harness verbs, the status shape, the pre-push gate, the gate breach codes and the run document: no harness verb, status shape or CI step changed.
@task_item n=6 part=untouched
The fake agent's command line, receipts, turn scripts and hook execution contracts: the fake agent is unchanged.
@task_item n=7 part=untouched
Log format, log line fields and corr rules: no log field or line changed.
@task_item n=8 part=untouched
E2E test-home location and its base checks (target/e2e-home, tmpfs on the dev host, headroom floor, owner-record sweep): homes did not move, only test sockets did.
@task_item n=9 part=untouched
Dependency, toolchain and MSRV decisions: no dependency or tool version changed.
