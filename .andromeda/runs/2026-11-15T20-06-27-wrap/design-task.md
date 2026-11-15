@task_prose part=session
A test-only session: no product line, no CLI output, no verb, no exit code, no refusal text and no hint changed.
Every test socket named viola-test-chan- now binds under a short /tmp root ({temp root}/vs-XXXXXX), never under TMPDIR; the only edits to product source files are inside their test modules.
The nextest mutants profile's default kill moved 10 s to 15 s and the shared root test wait bound WITHIN moved 7 s to 12 s.
The test-only Hyprland recorder viola-hypr-tap gained an integration test binary covering its argument surface and its ten mutation survivors.
A test home given to viola verify --record now comes from a record-home helper whose path names no component called home.
CI ran green on the pushed head; no commit after the pre-CI one.
@task_file n=1 path=tests/hook_fail_open.rs
what: the hook verb's fail-open integration tests, with an impostor channel endpoint; its socket now under the short /tmp test root, plus a test that the impostor socket path fits sun_path
@task_file n=2 path=tests/support/home.rs
what: shared test support for test homes and the test socket root; gains the record-home helper and a check that a path names no home component
@task_file n=3 path=tests/support/watch.rs
what: shared test wait helpers holding the root wait bound WITHIN (now 12 s)
@task_file n=4 path=.config/nextest.toml
what: the nextest test-runner profiles; the mutants profile's default slow-timeout kill (now 15 s) and its override comments
@task_file n=5 path=tests/cli_verify.rs
what: viola verify integration tests; imports the record-home helper and adds the test that a record home names no home component
@task_file n=6 path=tests/cli_verify/runs.rs
what: viola verify run tests; its --record callers take the record-home helper
@task_file n=7 path=tests/cli_verify/stamps.rs
what: viola verify stamps.json tests; its --record callers take the record-home helper
@task_file n=8 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the test-only Hyprland reply recorder; one inline unit test added, no recorder line changed
@task_file n=9 path=crates/viola-e2e/tests/hypr_tap.rs
what: new integration tests for the recorder's argument surface: record, record dispatch, and exit 2 on each malformed argument form
@task_file n=10 path=crates/viola-channel/src/lib.rs
what: the channel crate root; a test-only socket_dir helper giving the short /tmp test socket root
@task_file n=11 path=crates/viola-channel/src/client.rs
what: the channel client; test-module lines only, its test sockets moved to the short /tmp root
@task_file n=12 path=crates/viola-channel/src/server.rs
what: the channel server; test-module lines only, its test sockets moved to the short /tmp root
@task_file n=13 path=crates/viola-channel/src/tokio_client.rs
what: the async channel client; test-module lines only, its test sockets moved to the short /tmp root
@task_file n=14 path=crates/viola-e2e/src/harness/cleanup.rs
what: the end-to-end harness cleanup; test-module lines only, a test socket_dir under the short /tmp root
@task_file n=15 path=src/cmd/client.rs
what: the CLI's channel client glue for frame verbs; test-module lines only, two test socket sites moved to the short /tmp root
@task_file n=16 path=crates/viola-channel/tests/channel_frames.rs
what: channel frame integration tests; a file-level socket_dir under the short /tmp root
@task_file n=17 path=tests/security_negatives_server_identity.rs
what: security negative tests for server identity verification (the server-verify refusal path); its sockets moved to the short /tmp test root
@task_item n=1 part=touches
Rows whose guard names a test in the hook verb's fail-open tests (the hook verb's exit code and stderr): the tests stand under the same names, only their socket directory moved; a picker checks the guard still reads true.
@task_item n=2 part=touches
Rows on the server-verify-failed cause (exit 21 cause, its --json cause, its log detail code, its refusal hint) whose guard names a server-identity security negative test: the tests stand, only their socket directory moved.
@task_item n=3 part=touches
Rows on the wrapper socket's refused peer, guarded by the impostor-endpoint tests: unchanged behaviour, the impostor socket now under the short /tmp test root, with a new sun_path-fit test beside it.
@task_item n=4 part=touches
Rows on viola verify output and refusals (step line, summary, stamps.json update refused, home-tree check) whose guard names a viola verify test: the --record tests now take their home from the record-home helper; test names unchanged, one test added.
@task_item n=5 part=touches
Rows on frame-verb and socket-directory refusals (socket-dir-refused cause, frame verb refusals, run and revive socket-directory refusals) guarded by channel client or server tests: those tests now bind under the short /tmp root; no refusal text moved.
@task_item n=6 part=touches
Any row whose cell holds a test wait bound of 7 s or a 10 s mutants kill: those values are now 12 s and 15 s; a design-system row holding them is stale.
@task_item n=7 part=touches
Rows on viola wait output timed out, only where a guard or cell ties it to the shared root test wait bound WITHIN rather than the product's own wait timeout.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, iconography and depth: no glyph, colour token, column, width or style moved.
@task_item n=2 part=untouched
The CLI exit-code table, refusal lines, hints and fixed refusal pairs: no code, text or ordering changed; viola-hypr-tap's exit 2 is a test tool's, not a viola verb's.
@task_item n=3 part=untouched
viola list board, readback box, send readback mirror, link and unlink output, revive output and refusal codes: no test or product line of these moved.
@task_item n=4 part=untouched
The command names and the help output's verb groups: no verb added, renamed or hidden.
@task_item n=5 part=untouched
The web-page absence, the rendered-page absence and the self-validation protocol checks: nothing rendered changed.
@task_item n=6 part=untouched
The product's own channel socket placement (never at the /tmp or TMPDIR root): unchanged; only test sockets moved.
