# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · no dependency, version or crate changed and the link verbs run on the sync client, so Tokio stays confined to viola-mcp
established-decisions#2.1 · no · the wrapper change only serves link and unlink on the channel, and the PTY seam, pump, cwd and ConPTY claims live in viola-pty and src/run, which the session does not move
established-decisions#3.1 · no · the screen model and gate live in viola-agent-claude and src/run/gate.rs, and the wrapper change adds only the link and unlink arms, so the readiness gate claims stand
established-decisions#4.1 · no · the send, slot and confirm paths live in src/run/send and src/cmd/send.rs, which the session does not reach, and the wrapper change adds only the link and unlink arms
established-decisions#5.1 · no · the session adds writers of the existing link and unlink kinds and no reader, and the task says the link set still follows the log, so the appender and one-pass restore claims stand
established-decisions#6.1 · no · the link and unlink answers wait on the wrapper's existing snapshot write, and the task names no new write path or temp-and-rename code
established-decisions#7.1 · no · the link and unlink methods are two more arms on the same endpoint and channel seam, and the task changes no wait, last, answer or hook.dialog behaviour
established-decisions#8.1 · yes · add-to · F6: the new link and unlink verbs are frame-writing clients, so they run check_instance on the home and instances/<driven> before client::start, then the liveness pre-check and the server identity check
established-decisions#9.1 · no · the link verbs reach server_verdict's macOS path unchanged (Matches with no comparison), and 'Unix endpoint and home hardening' is not among the session's route moves
established-decisions#10.1 · no · the link verbs read the snapshot only through the liveness pre-check and the server check, so an unreadable snapshot stays absent, and 'Home and code-bearing file integrity' is not among the route moves
established-decisions#11.1 · no · the task says pause, link and unlink stay CLI-only and the MCP tool set (send, wait, last, answer, list) gains no tool
established-decisions#12.1 · no · the hook transport is untouched (the task's Untouched section)
established-decisions#13.1 · no · the event and dialog paths keep their one hook connect, and the two new callers of cmd::client::connect are CLI verbs
established-decisions#14.1 · no · the hook contract is untouched (the task's Untouched section)
established-decisions#15.1 · no · the dialog hold is untouched (the task's Untouched section), so the dialog-expired append is unchanged
established-decisions#15.2 · no · a recorded live measurement of the dialog hold that the session neither re-runs nor moves
established-decisions#15.3 · no · the hook's server-check failure path is untouched (the task's Untouched section), and its log-line pointer still names the right section
established-decisions#16.1 · no · 'Version done-check' gains the MCP and board halves but still owns the live read of the hold, which the task leaves untouched
