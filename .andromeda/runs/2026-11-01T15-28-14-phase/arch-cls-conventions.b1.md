# Classified — arch · conventions.b1

## Answers
conventions#1.1 · no · RefusalReason and its detail sets: the chunk adds no RefusalReason or detail value; its new refusals are exit-21 causes and exit-1 codes that come before any frame, so the wrapper's refusal order is unchanged
conventions#2.1 · yes · the new closed codes (socket-directory refusal, euid mismatch, any added UnreachableCause word) and their hint keys must pass is_code_word, since the --json documents and reply readers call it
conventions#3.1 · no · the reading of a peer's reply (reply_of, of_reply) is untouched; the chunk's refusals are local and add no reply shape
conventions#4.1 · yes · UnreachableCause is exit 21's closed cause set; the socket-directory and euid-mismatch refusals must fit it or extend it in viola-core
conventions#5.1 · yes · the typed exit set (0, 1, 2, 10–14, 20, 21) and which refusals map to exit 1 versus 21, including the new socket-directory and euid refusals
conventions#6.1 · yes · the catch-site internal-error rule and the silent run, revive start, mcp and hook paths: the socket-directory refusal leaves anyhow for a typed refusal, and the run and hook terminal rules sit on it
conventions#7.1 · yes · the open absence is this chunk's own work: the socket-directory refusal needs a code in place of the anyhow error from bind_endpoint's create_private_dir
conventions#8.1 · yes · run's and revive's start refuse the home tree before any log opens; the chunk's Unix strict modes and the socket-directory refusal at a wrapper's start run ahead of that log
conventions#9.1 · yes · verify's home-tree check runs through check_home_tree, the call the chunk's Unix strict-modes rule goes through; a refusal it now makes must exit 1 with no line
conventions#10.1 · no · verify's process-exit codes: the chunk adds no verify refusal and does not reach the update_stamps measuring step
conventions#11.1 · yes · the existing server check (peer pid and OS start time, server-verify-failed, no frame) is kept beside the euid check and carved out for macOS
conventions#12.1 · yes · hint_cause keys and the exit-21 hints in src/human/hints.rs: the new causes need their keys and hints
conventions#13.1 · yes · the macOS carve-out of server-verify-failed, which the chunk records by name as a named carve-out of the server check
conventions#14.1 · yes · the client's strict-modes instance check (check_instance on home and instances/<name>) runs before any connect; the new client directory and euid checks must follow it
conventions#15.1 · yes · the Connect versus Closed|Io mapping to Dead and Lost: the new client connect refusals and the root kill path's connect probe must map through it
conventions#16.1 · yes · the MCP isError result and structuredContent for an unreachable instance: the new refusals reach viola mcp's frame tools as the same document less v
conventions#17.1 · yes · the frame tools' server check (start time on spawn_blocking, no frame, no log line) binds the euid check on viola mcp's Tokio client path
conventions#18.1 · yes · frame tools run check_instance before live_endpoint, so the client directory verification on mcp's frame tools must follow that order
