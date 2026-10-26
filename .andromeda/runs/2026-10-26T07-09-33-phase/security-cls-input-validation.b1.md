# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · yes · the CLI send (src/cmd/send.rs, reached whole) and the MCP send tool keep validate_paste_text and the empty-typed-text check before the first frame, so the verification must follow those refusals
input-validation#1.2 · yes · viola answer (src/cmd/answer.rs, reached whole) and the MCP answer tool keep validate_paste_text on free text before the first frame, so the verification must follow it
input-validation#1.3 · yes · the hook.event and hook.dialog sends (edited) and the verbs' and MCP frames must build the closed params the wrapper re-validates, and the viola-channel client reads under take(MAX_FRAME)
input-validation#1.4 · yes · the hook.event and hook.dialog sends sit behind the stdin cap, the one-object rule and the dialog shape check, which must keep failing open (exit 0, no body) ahead of the server check
input-validation#1.5 · yes · the MCP send, wait, last and answer handlers the chunk edits keep their raw-argument checks (ViolaName, u64, validate_paste_text) and invalid-params shape, with no frame on a failure
input-validation#1.6 · yes · the verbs' argument parsing and the strict-modes gaps they read under (send, wait, last, answer, pause, release, src/cmd/client.rs, mcp) sit in reached files and in the snapshot read the verification adds
input-validation#1.7 · no · config.json thresholds, the dialog hold, port and diagnostics level are read in unreached budget, dialog and obs modules, and the chunk reads no config value
input-validation#1.8 · no · the Windows registry name read lives in unreached src/run/env.rs, and run.rs is edited only at its collision check
input-validation#1.9 · no · standing S5 · no new environment read or test seam without its own Decisions Log entry; the two existing fake-agent seams stay compiled out of release
input-validation#1.10 · no · standing S5 · no new environment read or test seam without its own Decisions Log entry; the two existing fake-agent seams stay compiled out of release
input-validation#1.11 · no · the colour facts are read only by unreached src/human/style.rs and src/cmd/list.rs, and the chunk adds no environment change
input-validation#1.12 · yes · the verification's new read of snapshot.json pid and started_at (snapshot.rs, liveness.rs live_endpoint) must follow the strict-modes check or keep the dated gaps, and a reader never writes
input-validation#1.13 · no · no notify tail is reached, and the chunk reads snapshot.json rather than a tailed events.ndjson path
input-validation#1.14 · no · the claude agents, --version and verify PTY reads sit outside the reached files (list, revive, verify, the version probe), and the mcp list tool is an untouched unit of tools.rs
input-validation#1.15 · no · the PTY output tee, gate and vt100 screen model are unreached, and the Not-needed list keeps the vt100 feed out
input-validation#1.16 · no · run.rs is edited only at its collision check, so program resolution, the .cmd refusal and the cwd rule are untouched units, and revive is unreached
input-validation#1.17 · no · the session-id argv path is revive and verify's Run E, neither reached, and resume_args is untouched
input-validation#1.18 · no · the statusline source read at start is a run.rs unit the chunk does not edit (only the collision check is), and the chunk changes no recording of statusline_command
input-validation#1.19 · yes · statusline.rs is reached whole, and its arm order (instance check, log, stdin, budget write, the snapshot read the scope names among its five readers, then the one shell-out) binds
input-validation#1.20 · no · the DLL search restriction and ConPTY sideload are Not-needed and sit in unreached viola-pty sideload and src/conpty.rs, and run's conpty_sideload step is another run.rs unit
input-validation#1.21 · no · the entity is N/A (no upload, webhook or database surface), and the chunk reaches none
