# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · the chunk builds no password or account surface; the row is N/A (no accounts, Personal v1)
authentication-authorization#1.2 · yes · any caller passing the IPC OS check gets the full surface, and the chunk builds that gate (the peer euid check and the verified socket directory)
authentication-authorization#1.3 · yes · the Windows pipe listener's SDDL is named as keeping its arm as it stands, and the shared bind code must not move it
authentication-authorization#1.4 · yes · the Unix primary control (0700 per-user directory, lstat owner, mode and symlink check before bind and connect), the 0600 chmod after bind, and the bans on ListenerOptionsExt::mode, try_overwrite and the abstract namespace are the chunk's own work
authentication-authorization#1.5 · yes · the wrapper's peer euid check on every accept, before a frame is read, is a named need
authentication-authorization#1.6 · yes · the Unix client euid check on every client, before the first frame of any method, and the macOS gap (no peer pid) are the chunk's work; the Windows client open and identity arm stay as they stand
authentication-authorization#1.7 · no · the release-from-driver guard runs after the frame read and the chunk changes no frame content (Not needed)
authentication-authorization#1.8 · yes · the home strict-modes read rule of check_instance and write rule of check_path, run before any first read under the home, and board::scan's home check, are named needs
authentication-authorization#1.9 · yes · check_home_tree's home read rule with the instance set is the home-level Unix strict-modes rule that the chunk's board::scan home check must share
authentication-authorization#2.1 · yes · the scope names the Windows pipe listener's arm (SDDL) as staying put, and this test witnesses the reject-remote bit of that listener
authentication-authorization#2.2 · yes · the scope keeps the shared pipe listener's Windows arm as it stands, and this test witnesses its protected DACL
authentication-authorization#3.1 · yes · the scope names peer_creds().pid() reading none on macOS and the euid source on Linux and macOS
authentication-authorization#3.2 · yes · the scope names the macOS carve-out as a match with no comparison, and the Unix server check the chunk extends is this verdict
authentication-authorization#3.3 · yes · the chunk's client euid check must run in order, with the strict-modes check first at every caller; the without_endpoint reading itself is untouched
authentication-authorization#3.4 · yes · link and unlink are Unix clients, so the new client euid check runs on them, and the macOS recorded gap is named
authentication-authorization#3.5 · yes · a peer euid mismatch takes the CLI frame-verb refusal surface (exit 21, instance-unreachable, --json and human forms)
authentication-authorization#3.6 · yes · a peer euid mismatch takes the MCP frame refusal shape (instance-unreachable with a detail, isError only)
authentication-authorization#3.7 · no · the exiting-wrapper pass on Windows is the Windows identity arm, not a unit the scope names as kept, and is owed to another route entry
authentication-authorization#3.8 · yes · the scope adds the macOS 0755 exit-21 case, and this entity owes the macOS socket-directory refusal code to this entry
authentication-authorization#3.9 · yes · the link and unlink impostor-server rows are the witness shape for the new client euid refusal on the link and unlink verbs
