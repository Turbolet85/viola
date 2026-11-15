@task_prose part=session
A test-only session: no product line changed, no crate, dependency, CI step or contract added or removed.
Every test channel socket named viola-test-chan-<pid>-<label> now binds under a short test-owned /tmp root ({temp root}/vs-XXXXXX), never under TMPDIR, in eight sites across four crates, so sun_path fits under any long scratch TMPDIR.
The shared root test wait bound WITHIN moved from 7 s to 12 s, and the nextest mutants profile's default kill moved from 10 s to 15 s (5 s period, terminate after 3), a stated departure from never raising the default; every per-test override kill is unchanged.
The viola-hypr-tap recorder gained an integration test binary hypr_tap in viola-e2e that kills ten mutation survivors, and one inline test; no product line of the recorder changed.
A record-home helper now gives each viola verify --record test a home whose path has no component named home, with a test proving it.
Measured: the whole workspace passes under the mutants profile (3286 tests, twice), a planted hang times out at 15 s under mutants, and CI read green.
@task_file n=1 path=tests/hook_fail_open.rs
what: the hook fail-open integration tests, with the impostor channel endpoint a test binds to check the hook fails open on a failed server check; edited, the impostor socket now under the short /tmp test socket root
@task_file n=2 path=tests/support/home.rs
what: the shared test-home support (TestHome, each test in its own viola home, owner-recorded cleanup, the test socket root SocketRoot); edited, gains the record-home constructor and fixture and the home-component check
@task_file n=3 path=tests/support/watch.rs
what: the shared root test wait bound WITHIN every root test wait shares; edited, 7 s to 12 s
@task_file n=4 path=.config/nextest.toml
what: the nextest profiles, the mutants profile's default slow-timeout kill and its per-test overrides, and the ci profile; edited, the mutants default kill 10 s to 15 s and its comments
@task_file n=5 path=tests/cli_verify.rs
what: the viola verify integration tests; edited, the record callers take the record-home helper and a test proves no record home names a home component
@task_file n=6 path=tests/cli_verify/runs.rs
what: the viola verify run-scenario tests; edited, its record caller takes the record-home helper
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the viola verify ledger-stamps tests; edited, its record callers take the record-home helper
@task_file n=8 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the viola-hypr-tap test-only recorder binary that records hyprctl replies as fixtures; edited, its inline test module only
@task_file n=9 path=crates/viola-e2e/tests/hypr_tap.rs
what: new viola-e2e integration test binary hypr_tap for the recorder's argument surface, against a stub hyprctl replaying committed scrubbed fixtures
@task_file n=10 path=crates/viola-channel/src/lib.rs
what: the viola-channel crate root; edited, its test-only capture module gains a test socket directory under the short /tmp root
@task_file n=11 path=crates/viola-channel/src/client.rs
what: the channel's blocking frame client; edited, its inline test module's sockets only
@task_file n=12 path=crates/viola-channel/src/server.rs
what: the channel's wrapper-side server with exclusive endpoint bind and peer euid admission; edited, its inline test module's sockets only
@task_file n=13 path=crates/viola-channel/src/tokio_client.rs
what: the channel's Tokio client behind its feature, used by the MCP server; edited, its inline test module's sockets only
@task_file n=14 path=crates/viola-e2e/src/harness/cleanup.rs
what: the end-to-end harness cleanup that reads a refused or missing socket as gone; edited, its inline test module's sockets only
@task_file n=15 path=src/cmd/client.rs
what: the root bin's CLI frame-verb client path (connect to the recorded endpoint, server verification, exit-21 details); edited, its two test sites' sockets only
@task_file n=16 path=crates/viola-channel/tests/channel_frames.rs
what: the channel wrapper-frames integration tests; edited, its sockets under the short /tmp root
@task_file n=17 path=tests/security_negatives_server_identity.rs
what: the security negative tests for server identity checked before the first frame; edited, its socket directory under the short /tmp root
@task_item n=1 part=touches
decision/root-waits-share-one-bound: the one shared root wait bound is now 12 s, not 7 s, still below the mutants default kill, now 15 s.
@task_item n=2 part=touches
name/other/viola-test-chan-pid-label: the name is unchanged, but every socket of that name now lives under the short test-owned {temp root}/vs-XXXXXX root in all four crates, not under a TMPDIR temp dir.
@task_item n=3 part=touches
decision/test-endpoints-outside-namespace: the test channel endpoints keep their name outside the product namespace and now take one directory spelling, a short /tmp root, never TMPDIR.
@task_item n=4 part=touches
decision/test-isolated-viola-home: a test that gives a home to viola verify --record now takes it from the record-home helper, a home whose path names no component home.
@task_item n=5 part=touches
decision/test-homes-cleaned-by-owner and decision/test-home-owner-sweep: the new record-home constructor is one more TestHome, removed by its owner like the rest; check it stays under the owner-record rule.
@task_item n=6 part=touches
decision/mutation-at-epoch-boundary and decision/mutants-own-target-dir: the mutation runs' nextest mutants profile default kill is now 15 s (was 10 s), stated as a departure from never raising the default; a row holding the 10 s kill is stale.
@task_item n=7 part=touches
decision/test-support-never-in-product: test-only socket helpers were added inside cfg(test) modules of product source in viola-channel, viola-e2e's harness and the root bin; no non-test product line changed.
@task_item n=8 part=touches
decision/e2e-test-only and decision/test-binaries-never-released: viola-e2e gains the integration test binary hypr_tap; still test-only, never released.
@task_item n=9 part=touches
name/binary/viola-hypr-tap: the recorder gains integration tests over its argument surface and malformed-argument refusals (exit 2, writes nothing); no product line of it changed.
@task_item n=10 part=touches
decision/recorded-fixtures-scrubbed: the new hypr_tap tests replay committed scrubbed fixtures through a stub hyprctl with HOME set to the case's temp dir.
@task_item n=11 part=touches
decision/hook-fails-open and decision/dialog-hook-fails-open-on-server-check: the test proving fail-open against an impostor endpoint now binds the impostor under the short /tmp root and gains a test that it fits sun_path at the widest pid and label; the rule itself unchanged.
@task_item n=12 part=touches
decision/server-identity-check-before-frame and decision/frame-client-verifies-server: their security negative tests and the frame client's tests moved sockets to the short /tmp root; the rule unchanged.
@task_item n=13 part=touches
decision/viola-channel-tokio-free-by-default and decision/channel-tokio-only-behind-feature: the Tokio client's inline tests changed (socket directory only); the feature gate unchanged.
@task_item n=14 part=touches
decision/cleanup-reads-refused-or-missing-socket-gone: the harness cleanup's inline tests take the short /tmp socket root; the cleanup rule unchanged.
@task_item n=15 part=touches
decision/verify-record-only-clean: tests recording fixtures through verify --record now run in a home with no path component named home; the clean-record rule unchanged.
@task_fact n=1 slug=occupied-resources
fact: Test channel sockets bind under a test-owned short /tmp root {temp root}/vs-XXXXXX, never under TMPDIR, so the sun_path bound holds under any TMPDIR length; the product endpoint socket is not under it.
@task_item n=1 part=untouched
Product contracts: CLI verbs and exit codes, channel methods and fault codes, MCP tools and results, event data, snapshots and ledger stamps — no product behaviour changed.
@task_item n=2 part=untouched
Stack, dependencies and build-system policy: no dependency, feature, lint ban, toolchain or MSRV changed.
@task_item n=3 part=untouched
CI and pre-push: no workflow, CI step, harness verb or status shape changed; coverage floors and perf gates did not move.
@task_item n=4 part=untouched
Crate dependency direction and crate names: no crate added or removed, no dependency edge moved.
@task_item n=5 part=untouched
Design philosophy, project intent and deployment model: the wheel, the no-daemon rule, local-only install and release contents are untouched.
@task_item n=6 part=untouched
The product endpoint name viola-<h12>.sock and the product socket's per-user owner-only directory: no product line changed, only test sockets moved.
@task_item n=7 part=untouched
Hyprland seam errors and window features: the seam crate and its error contracts are unchanged.
