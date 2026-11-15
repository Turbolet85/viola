@task_prose part=session
A test-only chunk: no product line changed outside test modules, no CLI output, refusal, exit code, colour or terminal byte moved.
The nextest mutants profile's default kill rose from 10 s to 15 s, and the shared root wait bound every test waits under (WITHIN) rose from 7 s to 12 s, still below the kill; per-case overrides unchanged.
Every test channel socket (viola-test-chan-pid-label) now binds under a short /tmp root ({temp root}/vs-XXXXXX), never under TMPDIR, so a long scratch TMPDIR no longer overflows sun_path; the swept sites span the channel crate's test modules, the e2e harness and the root tests.
Tests now give viola verify --record a home from a record-home helper whose path never holds a component named home, with a test proving it.
The Hyprland recorder tool (viola-hypr-tap) gained an integration test binary killing its ten mutation survivors, including its exit-2 refusal of each malformed argument form.
Measured: the whole workspace passes under the mutants profile (3286 tests), a planted hang times out at 15 s under it, and CI read green 7/7.
@task_file n=1 path=tests/hook_fail_open.rs
what: the hook fail-open suite — viola hook event and dialog calls failing open against an impostor channel endpoint; edited: the impostor socket now under the short /tmp root, plus a test that its path fits sun_path
@task_file n=2 path=tests/support/home.rs
what: the shared test home and socket-root support for the integration suites; edited: added the record-home helper and its home-component check
@task_file n=3 path=tests/support/watch.rs
what: the shared root wait bound every integration test's deadline uses; edited: 7 s to 12 s
@task_file n=4 path=.config/nextest.toml
what: the nextest profiles (ci and mutants) the coverage suite and mutation audit run under; edited: the mutants default kill 10 s to 15 s and its comments
@task_file n=5 path=tests/cli_verify.rs
what: the viola verify output suite (step lines, record mode); edited: record-mode cases take the record home, new test that the home names no home component
@task_file n=6 path=tests/cli_verify/runs.rs
what: viola verify run cases in the verify suite; edited: record-mode callers take the record home
@task_file n=7 path=tests/cli_verify/stamps.rs
what: viola verify stamped-home cases in the verify suite; edited: record-mode callers take the record home
@task_file n=8 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: the Hyprland reply recorder used to capture fixtures for the TUI boundary rig; edited: its inline test module only
@task_file n=9 path=crates/viola-e2e/tests/hypr_tap.rs
what: new integration test binary for the Hyprland reply recorder — recording from a stub hyprctl and exit-2 refusal of malformed arguments writing nothing
@task_file n=10 path=crates/viola-channel/src/lib.rs
what: the IPC channel crate (plugin and IPC channel, not assertable); edited: test-module socket directory helper only
@task_file n=11 path=crates/viola-channel/src/client.rs
what: the IPC channel client; edited: its test module's socket directory only
@task_file n=12 path=crates/viola-channel/src/server.rs
what: the IPC channel server with its peer identity check; edited: its test module's socket directory only
@task_file n=13 path=crates/viola-channel/src/tokio_client.rs
what: the async IPC channel client and its server euid check; edited: its test module's socket directory only
@task_file n=14 path=crates/viola-e2e/src/harness/cleanup.rs
what: the e2e harness's cleanup of sessions and sockets; edited: its test module's socket directory only
@task_file n=15 path=src/cmd/client.rs
what: the CLI's frame-verb client to a wrapped session (frame verb refusals); edited: its test module's socket directories only
@task_file n=16 path=crates/viola-channel/tests/channel_frames.rs
what: the IPC channel frame tests; edited: their socket directory moved to the short /tmp root
@task_file n=17 path=tests/security_negatives_server_identity.rs
what: the server-identity negative suite — refusal when the server is not the recorded wrapper (server-verify-failed); edited: its socket directory moved to the short /tmp root
@task_item n=1 part=touches
Mutation audit covers invariants held in crate tests: the mutants profile's default kill is now 15 s (was 10 s), and ten recorder survivors are now killed; any row holding the 10 s kill is stale.
@task_item n=2 part=touches
Zero flakiness, no retries: the shared test wait bound rose 7 s to 12 s and the mutants kill 10 s to 15 s to stop load-driven timeouts, with no retry added; a row holding either value is stale.
@task_item n=3 part=touches
Time bounds fall on the driver only / never on the human: the test-side wait bound (12 s) and kill (15 s) moved; still driver-side bounds, no human-facing time limit added.
@task_item n=4 part=touches
A11y cases run inside the coverage suite / a11y failures are nextest failures / a11y runs in the existing test suites: the nextest profiles those suites run under changed (mutants default kill), the ci profile unchanged.
@task_item n=5 part=touches
CI test gate allows no skips / gate requires coverage and doctest / gate breach fails the build: the full workspace passed under the mutants profile and CI read green 7/7; no gate step changed.
@task_item n=6 part=touches
Client euid refusal test strategy: the async client's server-euid test now binds its socket under the short /tmp root (it was red under a long TMPDIR); the strategy itself is unchanged.
@task_item n=7 part=touches
Server not the recorded wrapper (server-verify-failed) across hook calls, frame verbs and MCP frame tools: the impostor-endpoint and server-identity negative tests moved their socket directory; the refusal contract is unchanged.
@task_item n=8 part=touches
Socket-directory refusal before any frame and the socket-dir-refused contracts: test sockets no longer bind under TMPDIR; the product socket and its refusals are unchanged.
@task_item n=9 part=touches
viola verify output (step line, interrupted pair) and the verify suite: record-mode tests now take a home whose path never names a home component; no output line changed.
@task_item n=10 part=touches
TUI boundary driver / boundary tests sequenced on records / live readings never gate: the Hyprland reply recorder that captures the rig's records gained tests; the live rig's half of the record-home rule is carried to the standing live rig with a screen model.
@task_item n=11 part=touches
Plugin and IPC channel not assertable / internal crates not assertable: the channel crate's test modules changed (socket directory only), no a11y assertion added there.
@task_item n=12 part=touches
Hook event and dialog calls failing open (claude agents read fails open, hook-call contracts): the hook fail-open suite now passes under a long TMPDIR; behaviour unchanged.
@task_item n=1 part=untouched
Colour and SGR rows (amber DIALOG, bold NAME, dim stale rows, word beside every SGR, no SGR off a terminal): no output styling moved.
@task_item n=2 part=untouched
viola list board, caption, separator and NAME truncation rows: unchanged.
@task_item n=3 part=untouched
viola --help rows, CLI exit-code contracts and refusal layout (unable, hint, reason code, fixed copy pinned by literal): no string or exit code moved.
@task_item n=4 part=untouched
viola revive --json refusals and --list, link/unlink and plugin install contracts: not reached.
@task_item n=5 part=untouched
Screen-reader targets and manual Orca or keyboard passes: no manual pass run or changed.
@task_item n=6 part=untouched
TUI passthrough contracts (human keystrokes, viola terminal bytes, focus/mouse/resize, vt100 feed panic): no passthrough behaviour changed.
@task_item n=7 part=untouched
The absences (web or graphical frontend, touch surface, internationalization, rendered page, user time limit): still absent.
@task_item n=8 part=untouched
MCP wait tool timeout_ms and config dialog_hold_s: product bounds, unchanged by the test wait bound.
