@answer n=1 entity=authentication-authorization#3.7 v=no
why: the four frame refusals map to strict-modes-failed, socket-dir-refused and server-verify-failed on Linux as before, and the session removed only the SERVER_PID_REPORTED argument, which this mapping does not name
@answer n=2 entity=authentication-authorization#3.8 v=yes mode=contradict
why: the Windows pipe this row covers is removed, and the route entry "Windows-only live measurements" it owes its CARRY to is retired
@answer n=3 entity=authentication-authorization#3.9 v=yes mode=contradict
why: the macos-latest leg is gone from CI and the server-identity negatives now run ungated, so the macOS compile-out this row states no longer holds
@answer n=4 entity=authentication-authorization#3.10 v=yes mode=contradict
why: the macOS carve-out it names is removed with the client-side macOS arm, and the server-identity negatives are no longer compiled out there
@answer n=5 entity=authentication-authorization#3.11 v=yes mode=contradict
why: the macos-latest leg of the socket-dir negatives is removed with the CI matrix, now [ubuntu-latest]
@answer n=6 entity=authentication-authorization#3.12 v=yes mode=stale
why: the cited test name carries the where_it_is_reported qualifier of the removed PEER_EUID_REPORTED, since peer_euid_admitted is now observed == Some(own) on every build
@answer n=7 entity=authentication-authorization#3.13 v=yes mode=contradict
why: the link and unlink server-identity rows and the socket-dir negative claim Windows and macOS runs that the session removed
@answer n=8 entity=authentication-authorization#3.14 v=no
why: the session removed only the Windows SQOS arm and the macOS carve-out from client.rs and added no check, so no check sits inside Client::connect still holds and the report names no new viola-state edge
