# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · tokio stays confined to viola-mcp and viola-core stays pure; Cargo.toml gains only a [[test]] entry, no dependency moved
established-decisions#2.1 · no · the portable-pty pin and the viola_pty seam are untouched; run's pty path and the revive cwd rule hold, and Launch gains only a json flag
established-decisions#3.1 · no · the Windows wide-character OPEN concerns the vt100 feed under ConPTY, which the session does not touch
established-decisions#4.1 · no · the vt100 readiness gate, its signatures and the input-not-ready verdict are unchanged; run passes the same gate
established-decisions#5.1 · no · the confirmation window, typed_text and the empty-text refusal in send.rs hold; the send.rs change is only the no-endpoint arms reading without_endpoint
established-decisions#6.1 · no · no writer, lock or append path changed and without_endpoint writes nothing; list's log_pass reader is unchanged
established-decisions#7.1 · no · replace_private and the shared helpers are not in the diff; the plugin files are still written the same way
established-decisions#8.1 · no · place_private_new and place_absent are not in the diff; run's pinned-copy start is unchanged
established-decisions#8.2 · no · the placed hook and its exec tests are unchanged
established-decisions#8.3 · no · the close-before-placement order is untouched and the ETXTBSY race evidence stands
established-decisions#9.1 · no · wait still returns instance-unreachable when the endpoint vanishes; the new cause rides in the document detail, which this decision does not state
established-decisions#10.1 · no · viola-channel and server_pid are not in the diff; liveness.rs gains without_endpoint only
established-decisions#11.1 · no · server_verdict still returns Matches on macOS, and the socket-directory case stays owed to Unix endpoint and home hardening with no code built
established-decisions#12.1 · yes · add-to · F3: the no-endpoint case of a replayed snapshot now splits through without_endpoint into InstanceDead or UnwrappedName, opening nothing and writing no line
established-decisions#13.1 · no · hook.rs is not in the diff, so the statusline serve through replay is unchanged
established-decisions#14.1 · no · the isError refusal mapping and the five tool inputs hold on the changed tools.rs and result.rs; the new cause only fills the unreachable body
established-decisions#15.1 · no · CappedLines and the oversize cut live in viola_mcp::stdin, which the session does not touch
established-decisions#16.1 · yes · contradict · F6: a refused check_instance on a frame tool now reads StrictModes in the unreachable result, so its detail is no longer null
established-decisions#17.1 · no · the oversize line's WARN event and the end of stream are untouched
established-decisions#17.2 · no · mcp still exits 1 with both streams empty on an oversize line; main.rs gains only the internal-error path, which mcp stays silent at
established-decisions#18.1 · no · the bound lives in the reader viola hands rmcp, which the session does not change
established-decisions#19.1 · no · hook.rs is not in the diff; the hook tiers, the spine deadline and exit 0 hold
