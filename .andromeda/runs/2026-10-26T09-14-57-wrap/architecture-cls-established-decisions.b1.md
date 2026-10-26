# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · the report reads Tokio containment unmoved: no sync crate gains tokio, viola-channel's normal graph stays tokio-free, and the MCP start-time read stays inside viola-mcp
established-decisions#2.1 · no · the portable-pty pin, the pty seam and the child spawn and revive paths are untouched (revive is unchanged per the report), and no reached file is PTY code
established-decisions#3.1 · no · the screen model and the wrapper's readiness gate are not in the reached files or the change, and send.rs changes only the CLI verb's pre-frame check
established-decisions#4.1 · no · delivery confirmation is unchanged per the report, and refusal_of in send.rs still refuses before any frame because the client's own refusals come first
established-decisions#5.1 · yes · add-to · F9: verified clients keep every snapshot reader on read_snapshot (fork 1, provisional, option 1), with the replay move re-pinned to Home and code-bearing file integrity
established-decisions#6.1 · no · the snapshot writer and replace_private are untouched, and the change adds snapshot reads in liveness but no write path
established-decisions#7.1 · yes · add-to · F3 and F4: the channel clients now read the server's pid through interprocess peer_creds (SO_PEERCRED, GetNamedPipeServerProcessId on the SQOS-adopted end, none on macOS), and frame clients compare it before their first frame
established-decisions#8.1 · no · tools.rs keeps its tool inputs, result.refusal mapping, from rule and timeout cap, and the new mismatch result is an unreachable outcome, not a refusal
established-decisions#9.1 · yes · add-to · F7: the hook's event and dialog paths verify the wrapper before writing, a SessionEnd whose check fails takes the direct append, and the Windows exiting-wrapper window is owed to Windows-only live measurements
established-decisions#10.1 · yes · add-to · F7: a failed server check is one more fail-open cause, exit 0 with no body and no frame
established-decisions#11.1 · no · the wrapper's dialog-expired append at the hold's end is untouched, and the new check runs in the hook's connect before its hook.dialog frame
established-decisions#11.2 · no · the 671 s figure is a recorded measurement the session does not re-run, and the check runs before the hold starts
established-decisions#12.1 · no · the hold ceiling and its two route owners stay as they are, and the session only adds an exiting-wrapper item to Windows-only live measurements
