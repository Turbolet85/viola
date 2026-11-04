@answer n=1 entity=a11y-scope-summary#1.1 v=no
why: the session changed no CLI copy, exit code, refusal or SGR rule, and its test-only OS-gate reason lines leave these refusal claims unmoved
@answer n=2 entity=a11y-scope-summary#2.1 v=no
why: link and unlink output and refusals sit in src/cmd and tests/cli_links.rs, which the session did not reach, and no line text or exit code moved
@answer n=3 entity=a11y-scope-summary#3.1 v=no
why: the strict-modes refusal of the frame verbs is witnessed in tests/security_negatives_strict_modes.rs, which the session did not reach, and the refusal and its exit code are unchanged
@answer n=4 entity=a11y-scope-summary#4.1 v=no
why: the socket-directory refusal is unchanged, and the session's test-only change to tests/cross_exit_causes.rs leaves the Unix-only reading of the cited case intact
@answer n=5 entity=a11y-scope-summary#5.1 v=no
why: the run, revive-start and verify refusals of a widened home tree are unchanged, and the session's test-only changes to tests/run_cli.rs and tests/cli_verify.rs leave these cited cases and their outcomes intact
@answer n=6 entity=a11y-scope-summary#6.1 v=no
why: the socket-directory refusal on a start is unchanged, and the session's test-only change to tests/cross_exit_causes.rs leaves the cited case and its Unix-only reading intact
@answer n=7 entity=a11y-scope-summary#7.1 v=no
why: the keystroke and wheel boundary is untouched, since run_child_ends_when_its_wrapper_is_terminated running on every OS concerns child lifetime, not keystroke delivery
@answer n=8 entity=a11y-scope-summary#8.1 v=no
why: the chaos feed-panic case in tests/chaos_feed_panic.rs is not reached, and the refused-send and passthrough boundary is unchanged
@answer n=9 entity=a11y-scope-summary#9.1 v=no
why: the owed Windows outer-terminal reading of the child's own bytes is a live measurement that the session neither took nor moved
@answer n=10 entity=a11y-scope-summary#10.1 v=no
why: the impostor-endpoint fail-open behaviour in the hook and MCP tests is unchanged, and the macOS no-peer-pid carve-out stays the same named gap, the new census row only naming it
@answer n=11 entity=a11y-scope-summary#11.1 v=no
why: the strict-modes refusal of the hook and MCP frame tools is unchanged, and tests/hook_fail_open.rs and crates/viola-mcp/tests/handlers.rs are not reached by the session
@answer n=12 entity=a11y-scope-summary#12.1 v=no
why: the hook socket-directory fail-open sits in tests/security_negatives_socket_dir.rs, which the session did not reach, and its no-frame, exit 0 behaviour is unchanged
@answer n=13 entity=a11y-scope-summary#13.1 v=no
why: the MCP frame tools' socket-directory refusal is unchanged, and its witness in crates/viola-mcp/tests/handlers.rs is not reached by the session
@answer n=14 entity=a11y-scope-summary#14.1 v=yes mode=stale
why: the session's stale-row dim SGR witness is cfg(unix) and runs on ubuntu-latest and macos-latest only, so the all-three-OS reading of the portable-pty outer-PTY SGR checks needs an exception for it
@answer n=15 entity=a11y-scope-summary#15.1 v=no
why: the link since-line escaping through escape_message lives in src/cmd and src/human, which the session did not touch, and no line text changed
@answer n=16 entity=a11y-scope-summary#16.1 v=no
why: the tui passthrough boundary claims (zero bytes written, keystrokes never blocked, wheel unmoved) are unchanged, since the run_child_ends change concerns child lifetime and not keystroke delivery
@answer n=17 entity=a11y-scope-summary#17.1 v=no
why: the board caption line, the UNWRAPPED separator and the help group labels are unchanged, as the session changed no CLI string, caption or hint
