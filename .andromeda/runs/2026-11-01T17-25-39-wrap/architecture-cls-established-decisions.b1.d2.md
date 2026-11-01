# Classified — architecture · established-decisions.b1

## Answers
established-decisions#1.1 · no · the Tokio containment claim holds: no crate or dependency moved, the normal graph stays Tokio-free and scripts/channel-tokio-free.sh stays green, and TokioClient::server_euid sits behind the optional feature
established-decisions#2.1 · no · the PTY and conpty claims hold on the reached run.rs: the socket-dir check precedes the snapshot, the revive cwd is still read only after the strict-modes check, and viola-pty and src/conpty.rs are not reached
established-decisions#3.1 · no · the open Windows wide-character reading is untouched and no pty or feed path is reached by the session
established-decisions#4.1 · no · the screen model, its signatures and the gate start are unchanged, and the gate and feed thread are not reached
established-decisions#5.1 · no · send confirmation, the windows and the not-delivered reasons are unchanged, and the new check refuses before any frame, a case this claim does not cover
established-decisions#6.1 · no · the append and read counts and the per-instance log pass are unchanged, and board.rs moves only the home's read rule
established-decisions#7.1 · no · viola_state::fs is not edited, create_private_dir gains test callers only, and replace_private is unchanged
established-decisions#8.1 · no · place_private_new is unchanged and viola_state::fs is not edited
established-decisions#8.2 · no · the placed hook and its tests are unchanged and viola_state::fs is not edited
established-decisions#8.3 · no · the closed-handle order of the pinned copy is unchanged and viola_state::fs is not edited
established-decisions#9.1 · yes · add-to · F1 the run and revive start runs socket_dir_ready before Server::bind, F9 no check sits inside Client::connect (the checks sit at the callers), and F5 the wrapper's accept_loop admission filter on the one endpoint
established-decisions#10.1 · yes · add-to · F4 Client::server_euid and TokioClient::server_euid join server_pid beside it through peer_admitted, and F3 every frame-writing client takes the directory check, then connect with the euid check, then the pid and start-time verify, before its first frame
established-decisions#11.1 · yes · contradict · it says the macOS client euid check, the socket-directory checks and the 0755 exit-cause case are owed to the Unix endpoint route entry, but the session built all of them and retired that entry, leaving only the macOS pid gap, which keeps its name
established-decisions#12.1 · no · the replay reader and the rule that a replayed snapshot gives no endpoint and no frame are unchanged, and the new directory check reads only the recorded endpoint
established-decisions#13.1 · no · statusline's serve reads a replayed snapshot that holds no statusline_command, so it reaches no connect and the new checks in hook.rs are not on its path
established-decisions#14.1 · yes · add-to · F11 the founder's Q1 and Q2 word covers the new socket-dir-refused word and the euid server-verify-failed reading, and the board's read rule on every OS is the overseer's delegate, PROVISIONAL (inputs#I2 Q3), beside the list's Q1 authority
established-decisions#15.1 · no · the CappedLines stdin reader is not reached and tools.rs changes only the frame path
established-decisions#16.1 · yes · add-to · F2 viola_state::strict::check_socket_dir is the sibling strict check, and the frame tools now run the endpoint directory check at connect beside check_instance, whose strict-modes refusal stays as it was
established-decisions#16.2 · yes · add-to · tools.rs frame now also reads a refused socket directory as UnreachableCause::SocketDir, the new viola-core variant, beside its StrictModes and ServerVerify reads, each still with no frame and no line
established-decisions#17.1 · no · the stdin cap and the oversize line are unchanged and tools.rs changes only the frame path
established-decisions#17.2 · no · the exit-1 oversize case before initialize is unchanged and src/main.rs is not reached
established-decisions#18.1 · no · the bound stays in viola's reader, and rmcp's codec and stdio read are untouched
