# Classified — security · authentication-authorization.b1

## Answers
authentication-authorization#1.1 · no · N/A: no accounts or passwords, so the chunk has no hash to make
authentication-authorization#1.2 · no · the all-or-nothing capability model is unchanged (no roles, the OS user the only principal); its frame gates bind through #1.6, #1.8 and #3.4 instead
authentication-authorization#1.3 · yes · the TokenUser user SID, never the logon SID, is the principal the Windows owner and DACL leg and the protected home DACL at creation take (the listener SDDL and accept_remote are Not needed)
authentication-authorization#1.4 · no · the Unix socket-directory, socket-file and abstract-namespace rules are "Unix endpoint and home hardening"'s freight (Not needed); its owner rule for the home is #1.8's
authentication-authorization#1.5 · no · the server-side peer euid check on accept is "Unix endpoint and home hardening"'s freight (Not needed)
authentication-authorization#1.6 · yes · the Needs' dated-gap list (hook.event, hook.dialog, CLI send/wait/last/answer/pause/release, mcp frames) closes in this chunk, so the strict-modes check must run before the server verification and the first frame
authentication-authorization#1.7 · yes · release.rs, reached whole, sends `from` from VIOLA_NAME and exits 20 from a driver session, and the new check must leave that path and its frame intact
authentication-authorization#1.8 · yes · the owner-only home, the strict-modes check at every entry point, the Windows owner and DACL rules and the refusal surfaces are named by the Needs
authentication-authorization#2.1 · no · the remote-client witness of the pipe listener, whose SDDL and accept_remote the chunk does not change (Not needed)
authentication-authorization#2.2 · no · the anonymous-open witness of the pipe listener's DACL, a pipe the chunk does not change (Not needed); the chunk's own windows-2025 proof is of the home's DACL leg
authentication-authorization#3.1 · yes · the peer pid read (Linux SO_PEERCRED, Windows GetNamedPipeServerProcessId) is the value the Needs' server check compares before each new frame, and reading it logs nothing
authentication-authorization#3.2 · yes · the ServerVerdict over live_endpoint's LiveEndpoint, where only Matches writes a frame, is the check the Needs keep before every new frame, and live_endpoint is edited
authentication-authorization#3.3 · yes · link/unlink's check-before-client::start, liveness and server order is the every-verb order the Needs require, and the edited start and live_endpoint must keep it
authentication-authorization#3.4 · yes · the CLI frame-verb refusal (exit 21 instance-unreachable, no frame, no pid, path or endpoint in any line) is the refusal surface the Needs set for the new strict-modes refusals
authentication-authorization#3.5 · no · the exiting-wrapper pipe verdict is a server-verification fact the chunk does not change, and its CARRY is owed to "Windows-only live measurements"
authentication-authorization#3.6 · no · the macOS server-verification cases and the client euid check are "Unix endpoint and home hardening"'s freight (Not needed), CARRY owed there
authentication-authorization#3.7 · yes · the widened-instance-before-any-log and impostor exit-21 rows for link/unlink run through the edited start and live_endpoint, so they must keep holding
