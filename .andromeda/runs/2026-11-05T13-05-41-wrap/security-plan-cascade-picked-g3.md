@account n=1 entity=authentication-authorization#2.1 v=proposed
why: the Windows named-pipe endpoint, accept_remote(false) and tests/security_negatives_channel.rs are retired and deleted by the report
@account n=2 entity=authentication-authorization#2.2 v=proposed
why: the Windows protected DACL witness lived in tests/security_negatives_channel.rs, deleted with the retired Windows pipe controls
@account n=3 entity=authentication-authorization#3.1 v=proposed
why: the report removes the Windows and macOS pid arms and the SERVER_PID_REPORTED constant this row names
@account n=4 entity=authentication-authorization#3.2 v=proposed
why: peer_euid_admitted lost its reported parameter and PEER_EUID_REPORTED is removed, so the row's signature and Windows clause are stale
@account n=5 entity=authentication-authorization#3.3 v=proposed
why: server_verdict / verify_server lose peer_pid_reported and the macOS Matches arm the row's last sentence states
@account n=6 entity=authentication-authorization#3.4 v=not_touched
why: the report changes in liveness.rs only server_verdict / verify_server and in client.rs / tools.rs only the SERVER_PID_REPORTED argument, so without_endpoint's claim still holds
@account n=7 entity=authentication-authorization#3.5 v=proposed
why: its Linux-and-Windows server check and macOS carve-out clause name the retired Windows pipe and macOS no-peer-pid carve-out
@account n=8 entity=authentication-authorization#3.8 v=proposed
why: the exiting wrapper's Windows pipe is retired with the named-pipe endpoint and its owner, Windows-only live measurements, was retired at the prior wrap
@account n=9 entity=authentication-authorization#3.9 v=proposed
why: the macos-latest leg and the macOS carve-out are retired and the server-identity negatives now run ungated
@account n=10 entity=authentication-authorization#3.10 v=proposed
why: the macOS no-peer-pid carve-out is retired, server_verdict_rows loses its no-pid row and the census now reads a macOS gate as a fault
@account n=11 entity=authentication-authorization#3.11 v=proposed
why: its macos-latest leg and none-on-Windows clause are stale on a CI matrix of ubuntu-latest alone
@account n=12 entity=authentication-authorization#3.12 v=not_touched
why: the report states no seam was added and an unreadable euid is still refused, so the untested foreign-euid and foreign-owner claim still holds
@account n=13 entity=authentication-authorization#3.13 v=proposed
why: its Linux-and-Windows impostor rows, compiled-out-on-macOS file and Linux-and-macOS clauses are stale with security_negatives_server_identity.rs now ungated
