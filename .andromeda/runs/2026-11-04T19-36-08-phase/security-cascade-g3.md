@carried n=1 label=security-plan/authentication-authorization fact=fct-ghctc7rk
why: the scope keeps macOS's recorded gap, so the parity suite must keep the server-identity negatives compiled out on macOS and read no macOS server check as a pid or start-time comparison.
quote: The absent pid and start-time comparison of macOS is a named carve-out owned by no route entry: `server_verdict_rows` keeps its row and the server-identity negatives stay compiled out there.
@carried n=1 run=2 label=security-plan fact=fct-phspeqis
quote: macOS compares no peer pid or start time, the named carve-out, owned by no route entry, so there `server-verify-failed` means an euid mismatch alone.
@carried n=2 label=security-plan/authentication-authorization fact=fct-ka73bihm
why: the macos-latest leg the chunk turns green must read the socket-directory refusal and the client euid read there, with the server-identity negatives compiled out.
quote: On `macos-latest` a per-user socket directory at 0755 or a symlink to a 0700 one gives exit 21 on a frame verb, with `detail` `socket-dir-refused`, and the client euid read holds there. The server-identity negatives compile out on macOS, the named carve-out (below).
@carried n=3 label=security-plan/authentication-authorization fact=fct-ezmg3kua
why: the chunk runs the Unix IPC access-control negatives on both Unix OSes, and they must stay cfg(unix) with none on Windows.
quote: `tests/security_negatives_socket_dir.rs`, `#![cfg(unix)]`, holds the start, frame-verb and hook negatives over a 0755 directory and a symlink to a 0700 one, on `ubuntu-latest` and `macos-latest`, none on Windows.
@carried n=3 run=2 label=security-plan fact=fct-dthgjfbs
quote: IPC access control (Unix) — the socket lives in a per-user 0700 directory, checked before bind and before any client connect (owner, mode, not a symlink), and is chmod 0600 right after the bind; a failed check refuses `run`, reports `instance-unreachable` to clients and fails `hook` open; `try_overwrite` stays off and the Linux abstract namespace is never used.
@carried n=4 label=security-plan/authentication-authorization fact=fct-gfc5nchi
why: the parity suite reads client-side server verification per OS, and the peer-pid read must be Linux SO_PEERCRED and none on macOS, writing and logging nothing.
quote: The client reads the connected stream's peer pid through `Client::server_pid` / `TokioClient::server_pid` (`server_pid_of`, interprocess 2.4.4 `StreamCommon` `peer_creds().pid()`): on Linux `SO_PEERCRED`, on Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, on macOS none. `SERVER_PID_REPORTED`, a `const bool` re-exported by `viola-channel`, is false on macOS. Reading the pid writes and logs nothing.
@carried n=5 label=security-plan/authentication-authorization fact=fct-zpaxdj2d
why: the chunk exercises the client's euid admission on Linux and macOS, where an unreadable peer euid must be refused.
quote: `peer_euid_admitted(observed, own, reported)` admits when not reported or when observed equals own (`!reported || observed == Some(own)`); `PEER_EUID_REPORTED` is `cfg!(unix)`, so a peer whose euid cannot be read is refused on Unix, and on Windows no euid is read and the DACL and the pid comparison stay the controls.
@carried n=6 label=security-plan/authentication-authorization fact=fct-5ryfhirx
why: the verdict decides whether any frame is written on each OS, and macOS's no-comparison Matches is the carve-out the scope keeps.
quote: Only `Matches` lets a frame be written. Where no peer pid is reported (`SERVER_PID_REPORTED` false, macOS) the verdict is `Matches` with no comparison.
@carried n=7 label=security-plan/authentication-authorization fact=fct-ybwsdbo3
why: one diagnostics schema and no new exit code require these exit-21 refusals and their --json documents to read the same on every OS.
quote: A CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) meeting a mismatch, or a check that cannot be made, writes no frame and exits 21: under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}` on stdout with stderr empty; in human mode the `unable  <name>  instance-unreachable` line with the hint `the process on <name>'s endpoint is not its recorded wrapper; it was not contacted` last on stderr. A refused peer euid is this same mismatch, with the same document and hint. A socket-directory refusal, before the connect, writes no frame and exits 21: under `--json` the document `{"v":1,"error":"instance-unreachable","detail":"socket-dir-refused"}` on stdout with stderr empty; in human mode the same `unable` line with the hint of cause `socket-dir` last on stderr.
@carried n=8 label=security-plan/authentication-authorization fact=fct-6ntn52d2
why: the one-schema rule requires MCP isError results to carry the same closed instance-unreachable causes on every OS.
quote: `viola mcp`'s `frame` maps a refused `check_instance` to `strict-modes-failed`, no `live_endpoint` to `without_endpoint`'s reading, a refused `check_endpoint_dir` after `live_endpoint` to `socket-dir-refused`, and a refused `peer_admitted` or a failed `server_verified` to `server-verify-failed`, each the `instance-unreachable` result with its cause in `detail`, with no frame and no line on any of them, `isError` the only record.
@carried n=9 label=security-plan/authentication-authorization fact=fct-6lhjfvvj
why: the one-schema rule requires the no-endpoint closed words to read the same on every OS, with the strict-modes check first.
quote: With no `LiveEndpoint`, `viola_state::liveness::without_endpoint` tells a dead instance from an unwrapped name by `symlink_metadata` on the instance directory alone: a directory reads `instance-dead`, anything else (absent, a regular file, a symlink) `unwrapped-name`. It opens nothing and writes no line, and the strict-modes check stays first at every caller.
@carried n=10 label=security-plan/authentication-authorization fact=fct-p7bllbag
why: link and unlink run on the Unix suite, and their connect must take the socket-directory and euid checks on both Unix OSes and the server check on Linux, macOS keeping the carve-out.
quote: then `cmd::client::connect` before their one `link` / `unlink` frame: the socket-directory check on the recorded endpoint, the connect, the peer euid decision on Unix and, on Linux and Windows, the server check (macOS compares no pid or start time, the named carve-out).
@carried n=11 label=security-plan/authentication-authorization fact=fct-h63tjgiv
why: the chunk's cfg audit must keep each link and unlink test on the OSes its control holds, each gate naming its reason.
quote: `link_verbs_refuse_a_widened_instance_before_any_log` (Unix only) reads a group-writable driven instance refused before any log, its mode as found; the server-identity table's `link` / `unlink` rows read no frame and exit 21 in both modes against an impostor server on Linux and Windows, compiled out on macOS with the file; `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` reads the eight frame verbs, `link` and `unlink` among them, refused with no frame over a 0755 directory and a symlink to a 0700 one, on Linux and macOS.
@carried n=12 label=security-plan/authentication-authorization fact=fct-efyzkncd
why: the scope forbids any seam forcing a refusing peer-euid decision, so the Unix suite must hold foreign-euid and foreign-owner decisions by injected values only.
quote: A foreign euid and a foreign owner are not driven by any test: one account can make no foreign peer, so those decisions are held by injected values, and no seam may be added to drive them.
@carried n=12 run=2 label=security-plan fact=fct-unwy7exn
quote: IPC server-side peer check (Unix) — on every accept, a peer whose euid differs from the server's is dropped before a frame is read.
@carried n=13 label=security-plan/authentication-authorization fact=fct-o3bl5eop
why: no Windows test moves, so this Windows-only witness must stay on windows-2025 with its cfg(windows) reason named.
quote: The remote-client rejection is asserted on CI's `test (windows-2025)` by `security_negative_viola_listener_rejects_remote_clients`: it opens a client end of a bound `viola_channel::Server::bind` listener and reads its `NamedPipeType` through `NtQueryInformationFile` / `FilePipeLocalInformation`, the `FILE_PIPE_REJECT_REMOTE_CLIENTS` bit (2) set.
@carried n=13 run=2 label=security-plan fact=fct-kny6sapm
quote: IPC access control (Windows) — the listener gets an explicit protected DACL granting the user SID and SYSTEM only; the logon SID is deliberately not added, since the boundary v1 promises is the OS user; remote clients stay rejected, the first-pipe-instance behaviour is kept and the handle is not inheritable.
@carried n=14 label=security-plan/authentication-authorization fact=fct-6fdpugfk
why: no Windows test moves, so the DACL-effect witness must stay on windows-2025 with its cfg(windows) reason named.
quote: The protected user + SYSTEM DACL's effect, beyond its text read-back, is asserted on CI's `test (windows-2025)` by `security_negative_viola_listener_denies_an_anonymous_open`: with the listener bound first, a `GENERIC_READ` open under `ImpersonateAnonymousToken` is denied `ERROR_ACCESS_DENIED` (5).
@binding n=1
**authentication-authorization ↔ security-anti-patterns**: the macOS carve-out (security-plan/authentication-authorization fct-ghctc7rk) must stay the only frame-before-verification exception, its euid and socket-directory checks standing and no other frame or process borrowing it (security-plan fct-6cauxi6n).
@binding n=2
**authentication-authorization ↔ error-handling**: the exit-21 refusals and MCP isError causes (security-plan/authentication-authorization fct-ybwsdbo3, fct-6ntn52d2) must carry only the closed instance-unreachable details on every OS (security-plan fct-kulo4ws7).
@binding n=3
**authentication-authorization ↔ security-anti-patterns**: the Unix socket-directory negatives (security-plan/authentication-authorization fct-ezmg3kua) must run against a socket in a verified 0700 per-user directory, chmod 0600 after bind and never through ListenerOptionsExt::mode, which fails the bind on macOS (security-plan fct-zyth7okk).
@binding n=4
**authentication-authorization ↔ security-anti-patterns**: no Unix parity seam may drive or force a refusing peer-euid decision (security-plan/authentication-authorization fct-efyzkncd); any new seam or env read needs its own Decisions Log entry (security-plan fct-7ackeyph).
@binding n=5
**authentication-authorization ↔ error-handling**: the hook negatives over a refused socket directory (security-plan/authentication-authorization fct-ezmg3kua) must still read exit 0, no body and no stderr on Linux and macOS (security-plan fct-r24nqlwe).
@binding n=6
**authentication-authorization ↔ security-anti-patterns**: the Windows witnesses (security-plan/authentication-authorization fct-o3bl5eop, fct-6fdpugfk) must keep holding when a pty-seam or wrapper change is shared with Unix, the listener keeping its security descriptor and the client its SQOS open (security-plan fct-4vfalbv2) (security-plan fct-gqlrkxte).
