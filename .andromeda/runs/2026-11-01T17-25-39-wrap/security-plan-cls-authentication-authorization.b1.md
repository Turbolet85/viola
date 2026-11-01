# Classified — security-plan · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · no accounts or passwords, so the session's checks have no password to touch
authentication-authorization#1.2 · no · the only principal is still the local OS user and the new peer drop does not change the all-or-nothing surface, so the claim holds
authentication-authorization#1.3 · no · the Windows listener DACL is unchanged and the session's checks are cfg(unix), so the Windows row's claim holds on the changed server.rs
authentication-authorization#1.4 · yes · contradict · the socket-dir check is built before bind and before every connect; a hook's refused directory writes one WARN hook-decision line, not a diagnostics note, and the bind's directory failure now reads socket-dir-refused (F4)
authentication-authorization#1.5 · yes · contradict · the stack reference "sync + Tokio server" is disproved, one sync server is what the code holds, while the peer drop in accept_loop is built before any frame (F7)
authentication-authorization#1.6 · yes · contradict · the client euid check is built at the three client seams (F5, F6), so the clause "the client-side euid check is not built" and the euid-only macOS verification are no longer true
authentication-authorization#1.7 · no · the release-from-driver refusal and its line are unchanged, and the new peer drop runs before any frame without reaching them
authentication-authorization#1.8 · yes · stale · scan holds the home to check_read on every OS (F9), not check_path as the list reading says, and the 0750 board inconsistency CARRY is discharged
authentication-authorization#1.9 · no · the home-tree check and its refusal shape (exit 1, no log, no line, no child) are unchanged; the socket-dir check is a separate check at its own directory
authentication-authorization#2.1 · no · the Windows remote-client rejection and its witness are unchanged, with no Windows listener change
authentication-authorization#2.2 · no · the Windows anonymous-open DACL witness is unchanged, with no Windows listener change
authentication-authorization#3.1 · yes · add-to · peer_creds() now also serves an euid read on Unix (F6), beside the pid read this row states; the pid claim, none on macOS, stands
authentication-authorization#3.2 · no · the verdict and its Matches-only pid and start-time comparison are unchanged; the euid admission is a separate gate and "only Matches" still holds
authentication-authorization#3.3 · no · without_endpoint's reading and its two causes are unchanged
authentication-authorization#3.4 · yes · stale · the link/unlink server check runs through client::connect, which now has the directory check before connect and the euid decision on macOS too (F5), so "on Linux and Windows" is no longer its reach
authentication-authorization#3.5 · no · the CLI mismatch refusal (exit 21, no frame, server-verify-failed) holds as written; the new refusal is a directory one, not a server-verification one
authentication-authorization#3.6 · yes · add-to · frame now runs the directory check after live_endpoint and the euid check beside server_verified, both on the one helper shared by the three client seams (F3), each refusal writing no frame and no line
authentication-authorization#3.7 · no · a Windows-only pipe case, and no euid or directory check changes on Windows
authentication-authorization#3.8 · yes · contradict · the CARRY owed to "Unix endpoint and home hardening" is discharged: the client euid check is built (F6) and the 0755 directory case and socket-dir code now run on macOS (F13)
authentication-authorization#3.9 · yes · add-to · channel_link_names_are_validated_by_the_wrapper writes through Client::connect past every check, since no check sits inside connect and the directory and euid checks sit at the seams it passes (F8)
