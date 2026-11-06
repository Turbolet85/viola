@proposal n=1 detector=D-retired severity=warning file=authentication-authorization anchor=fct-o3bl5eop basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Authentication & Authorization
change: retire the entity "IPC access control (Windows), the remote-client rejection's witness" whole: the Windows named-pipe endpoint and its accept_remote(false) control are removed, tests/security_negatives_channel.rs is deleted and windows-sys leaves every manifest; the code stays recoverable at the tag multi-os-final (ecbe7ab3).
sidecar: authentication-authorization#2.1 retired — the Windows pipe remote-client rejection witness left with the named-pipe endpoint (Linux-only ruling, 2026-11-04).
rationale: the report's Retired bullet names the Windows named-pipe endpoint with its SDDL / SQOS / accept_remote(false) controls, and its Files line deletes tests/security_negatives_channel.rs.
@proposal n=2 detector=D-retired severity=warning file=authentication-authorization anchor=fct-6fdpugfk basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Authentication & Authorization
change: retire the entity "IPC access control (Windows), the DACL's effect's witness" whole: the protected user + SYSTEM DACL on the Windows pipe listener is removed with the endpoint, and its test file tests/security_negatives_channel.rs is deleted.
sidecar: authentication-authorization#2.2 retired — the Windows pipe DACL effect witness left with the named-pipe endpoint (Linux-only ruling, 2026-11-04).
rationale: the report retires the Windows named-pipe endpoint and its SDDL controls (Retired) and deletes tests/security_negatives_channel.rs (Files), removing windows-sys (Dependencies).
@proposal n=3 detector=D-cascade severity=warning file=authentication-authorization anchor=fct-gfc5nchi basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Authentication & Authorization
change: Implementation reads: The client reads the connected stream's peer pid through `Client::server_pid` / `TokioClient::server_pid` (`server_pid_of`, interprocess 2.4.4 `StreamCommon` `peer_creds().pid()`, Linux `SO_PEERCRED`). Reading the pid writes and logs nothing. (The Windows `GetNamedPipeServerProcessId` arm, the macOS none case and the `SERVER_PID_REPORTED` constant are dropped.)
sidecar: authentication-authorization#3.1 — the server pid read is SO_PEERCRED alone; the Windows and macOS arms and SERVER_PID_REPORTED removed.
rationale: the report's Symbols line removes viola-channel's SQOS and named-pipe arms of client.rs / tokio_client.rs and the constant SERVER_PID_REPORTED, and Retired names the macOS no-peer-pid carve-out.
@proposal n=4 detector=D-cascade severity=warning file=authentication-authorization anchor=fct-zpaxdj2d basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Authentication & Authorization
change: Implementation reads: `peer_euid_admitted(observed, own)` admits only when observed equals own (`observed == Some(own)`), so a peer whose euid cannot be read is refused. `Client::server_euid` / `TokioClient::server_euid` read it (`peer_creds().ok()?.euid()`), writing and logging nothing. (The `reported` parameter, `PEER_EUID_REPORTED` and the Windows no-euid clause are dropped.)
sidecar: authentication-authorization#3.2 — peer_euid_admitted takes (observed, own), PEER_EUID_REPORTED removed, the Windows DACL clause dropped.
rationale: the report states the constants SERVER_PID_REPORTED and PEER_EUID_REPORTED are removed, their Linux value folded: peer_euid_admitted(observed, own) is observed == Some(own), an unreadable euid still refused, peer_admitted / serve_admitting keeping their roles.
@proposal n=5 detector=D-cascade severity=warning file=authentication-authorization anchor=fct-5ryfhirx basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Authentication & Authorization
change: drop the last sentence ("Where no peer pid is reported (`SERVER_PID_REPORTED` false, macOS) the verdict is `Matches` with no comparison."): `server_verdict` and `verify_server` always compare the peer pid and its start time; the rest of the row stands.
sidecar: authentication-authorization#3.3 — the verdict's macOS no-pid Matches arm removed; server_verdict / verify_server lose peer_pid_reported.
rationale: the report's Symbols line: liveness::server_verdict / verify_server lose the peer_pid_reported parameter and the macOS Matches arm (server_verdict_rows loses its no-pid row).
@proposal n=6 detector=D-cascade severity=warning file=authentication-authorization anchor=fct-p7bllbag basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Authentication & Authorization
change: the connect clause reads "then `cmd::client::connect` before their one `link` / `unlink` frame: the socket-directory check on the recorded endpoint, the connect, the peer euid decision and the server check (peer pid and start time)." — the "on Unix", "on Linux and Windows" and "(macOS compares no pid or start time, the named carve-out)" qualifiers dropped; the rest of the row stands.
sidecar: authentication-authorization#3.5 — link / unlink's connect clause reads the one Linux path; the Windows and macOS qualifiers dropped.
rationale: the report retires the Windows named-pipe endpoint and the macOS no-peer-pid carve-out (Retired) and removes SERVER_PID_REPORTED and the macOS Matches arm (Symbols / APIs).
@proposal n=7 detector=D-retired severity=warning file=authentication-authorization anchor=fct-dyanhg3f basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Authentication & Authorization
change: retire the entity "IPC client-side server verification, an exiting wrapper's Windows pipe" whole, with its Windows claim: the Windows pipe it covers is removed and its owner, the route entry "Windows-only live measurements", was retired at the 2026-11-04T23-11-43-wrap (residuals target another-os).
sidecar: authentication-authorization#3.8 retired — the exiting wrapper's Windows pipe case left with the named-pipe endpoint; its route owner retired earlier.
rationale: the report's Retired bullet removes the Windows named-pipe endpoint and records the working-route entry "Windows-only live measurements" retired at the 2026-11-04T23-11-43-wrap, its sites re-pointed or retired with the Windows claim they owe.
@proposal n=8 detector=D-retired severity=warning file=authentication-authorization anchor=fct-ka73bihm basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Authentication & Authorization
change: retire the entity "IPC client-side server verification, the macOS cases" whole: the macos-latest CI leg and the macOS carve-out are removed, and tests/security_negatives_server_identity.rs now runs ungated (the socket-directory negatives stay in authentication-authorization#3.11).
sidecar: authentication-authorization#3.9 retired — the macOS cases left with the macos-latest leg and the macOS carve-out.
rationale: the report retires the macos-latest CI leg and the macOS no-peer-pid carve-out (Retired), and deviation 11 makes security_negatives_server_identity.rs unconditional.
@proposal n=9 detector=D-retired severity=warning file=authentication-authorization anchor=fct-ghctc7rk basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Authentication & Authorization
change: retire the entity "IPC client-side server verification, the macOS carve-out" whole: server_verdict_rows loses its no-pid row, the server-identity negatives run ungated, and the census tests/contract_os_gated_tests.rs reads a gate naming target_os macos as a fault (another OS), so no NO_PEER_PID row stands.
sidecar: authentication-authorization#3.10 retired — the macOS no-peer-pid carve-out left with the macOS code (Linux-only ruling, 2026-11-04).
rationale: the report's Retired bullet names the macOS no-peer-pid carve-out; Symbols / APIs removes server_verdict_rows' no-pid row; Harness / gate surface makes the census read a macOS gate as a fault.
@proposal n=10 detector=D-cascade severity=warning file=authentication-authorization anchor=fct-ezmg3kua basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: ## Authentication & Authorization
change: Implementation reads: `tests/security_negatives_socket_dir.rs`, `#![cfg(unix)]`, holds the start, frame-verb and hook negatives over a 0755 directory and a symlink to a 0700 one, on CI's `ubuntu-latest` leg. (The `macos-latest` leg and the "none on Windows" clause are dropped.)
sidecar: authentication-authorization#3.11 — the socket-directory negatives run on the one ubuntu-latest leg.
rationale: the report sets ci.yml's matrix to [ubuntu-latest] and retires the windows-2025 and macos-latest legs; deviation 11 keeps every other cfg(unix) gate.
@proposal n=11 detector=D-cascade severity=warning file=authentication-authorization anchor=fct-h63tjgiv basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-iol4iyl6
section: ## Authentication & Authorization
change: the server-identity clause reads "the server-identity table's `link` / `unlink` rows read no frame and exit 21 in both modes against an impostor server, the file ungated" and the socket-directory clause ends "over a 0755 directory and a symlink to a 0700 one" — "on Linux and Windows, compiled out on macOS with the file" and "on Linux and macOS" dropped; the rest of the row stands.
sidecar: authentication-authorization#3.13 — the link / unlink impostor and socket-directory test clauses lose their Windows and macOS qualifiers; the server-identity file runs ungated.
rationale: deviation 11 makes security_negatives_server_identity.rs and the impostor cases unconditional, and the report's Retired bullet removes the Windows endpoint and the macOS carve-out.
@remark
not proposed (body, not this detail group): §Authentication & Authorization body facts fct-qkuk7pih (enforced per OS), fct-7nb5glkx (Library row: Windows/macOS pid arms, windows-sys), fct-kny6sapm (IPC access control Windows), fct-phspeqis (SQOS / macOS carve-out) and fct-jya3vgt6 (Windows DACL) are stale per task items 5-8; #3.12's stack-reference test name ends where_it_is_reported, which the report does not say was renamed after PEER_EUID_REPORTED left — worth a check.
