# Classified — security-plan · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · no account or password is added, and the row's N/A still holds
authentication-authorization#1.2 · no · the nine methods stay all-or-nothing behind the OS check, which the session strengthens without adding a method or role
authentication-authorization#1.3 · no · the Windows DACL row's claim holds on the changed server file, since the session's server change (the peer drop and euid) is cfg(unix) and the DACL is untouched
authentication-authorization#1.4 · yes · contradict · the owed bind-time socket-dir code is now built (socket-dir-refused, F4), and a hook's refused directory leaves one hook-decision WARN line, not a note in diagnostics/
authentication-authorization#1.5 · yes · contradict · the stack reference "sync + Tokio server" is disproved, one sync server being what the code holds, while the euid drop in accept_loop is built (F7)
authentication-authorization#1.6 · yes · contradict · the client euid check is built at the three client seams (F5), so the clause owing it is discharged and macOS's euid and verified directory checks now hold
authentication-authorization#1.7 · no · the release-from-driver refusal and the release verb are not in the session's changes, so the claim holds
authentication-authorization#1.8 · yes · contradict · viola list's home is now checked on the read rule on every OS (F9), so the check_path clause and the owed 0750 CARRY no longer hold
authentication-authorization#1.9 · no · check_home_tree and TreeRefused are unchanged, and the session's strict.rs addition is the separate check_socket_dir, so the start tree rule holds
authentication-authorization#2.1 · no · the Windows remote-client witness is in the channel tests, which are not reached, and the reached server change is Unix-only
authentication-authorization#2.2 · no · the Windows anonymous-open witness is in the channel tests, which are not reached, and the DACL is untouched
authentication-authorization#3.1 · yes · add-to · the sibling PEER_EUID_REPORTED const (F6), cfg!(unix), joins it, so an unreadable euid is refused on Unix, while the macOS pid none stays
authentication-authorization#3.2 · no · the verdict and its macOS Matches-without-comparison are unchanged, and the euid decision is a separate gate at the seam
authentication-authorization#3.3 · no · without_endpoint is unchanged, and the session's directory check is a separate helper beside it
authentication-authorization#3.4 · yes · add-to · link and unlink are frame verbs whose connect now runs the endpoint-directory check before the euid and server checks (F3), and the macOS parenthetical is the pid carve-out that stays
authentication-authorization#3.5 · yes · add-to · the endpoint-directory check shared at the client seams (F3) also refuses a CLI frame verb with exit 21 and no frame, under the closed cause socket-dir-refused, beside server-verify-failed
authentication-authorization#3.6 · yes · add-to · MCP's frame gains the endpoint-directory check after live_endpoint (F3), a new refusal mapped to instance-unreachable with no frame and no line, isError the record
authentication-authorization#3.7 · no · the Windows exiting-pipe claim holds, since the hook connect's new directory and euid checks are Unix-only
authentication-authorization#3.8 · yes · contradict · the euid check, the socket-directory refusal's code and the macOS socket-dir cases are built and witnessed (F13), so their CARRY is discharged, and only the pid gap stays, owned by no route entry (F11)
authentication-authorization#3.9 · yes · add-to · channel_link_names_are_validated_by_the_wrapper writes through the viola-channel client past every check, since no check sits inside Client::connect (F8)
