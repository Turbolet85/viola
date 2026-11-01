proposals:
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 43
    section: "## Threat Model Summary"
    change: "In line 43 replace only the Unix clause, socket file `$TMPDIR/viola-<h12>.sock` (else `/tmp`) on Unix, with: socket file in a verified per-user 0700 directory on Unix. The Windows named-pipe clause stays."
    sidecar: "Threat Model Summary, boundary-ipc entry point: the Unix socket stands in a verified per-user 0700 directory; the `$TMPDIR` / `/tmp` socket path is retired as a disproved claim."
    rationale: "report.md:58 measures this claim false, the endpoint standing in a per-user 0700 directory since chunk 2026-09-27-wrapper-channel, and says to retire that one line; §Authentication's IPC access control (Unix) says the same."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:58
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 53
    section: "## Threat Model Summary"
    change: "Line 53 reads: On Unix the socket sits in a per-user 0700 directory; a path there that is a symlink, is no directory, has another owner or holds any group or other bit is refused and left as found, so no client connects through a path another local user made."
    sidecar: "Threat Model Summary, boundary-ipc consequences: the world-writable `/tmp` consequence is retired with the socket path it rested on; the verified per-user directory replaces it."
    rationale: "A second occurrence of the claim report.md:58 retires, which pattern `threat-sock` did not count: the socket in `/tmp` with `TMPDIR` unset. The directory check is report.md:11 and its refusals report.md:107."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
    dependent-of: D-security-auth
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 50
    section: "## Threat Model Summary"
    change: "Add after line 50, at its level: On Unix the wrapper's `accept_loop` filters `incoming()` by `peer_admitted(peer_euid_of(stream))`: a stream whose euid differs from the server's, or whose euid cannot be read, is dropped on the accept thread, before its worker thread and before any frame, the peer reading end-of-file, with no line written and nothing on the terminal, and no env, flag or feature reaches the decision. It checks the OS user only and names no caller within that user. Windows reports no euid; the DACL and the pid comparison stay the controls there. No test drives a foreign euid: injected values hold the decision (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    sidecar: "Threat Model Summary, boundary-ipc trust boundary: the wrapper's built peer euid drop at accept on Unix, what it checks and what it does not."
    rationale: "New server-side control built this chunk (report.md:12, report.md:13), beside a line stating no per-connection authentication; the limit of its proof is report.md:130."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 56
    section: "## Threat Model Summary"
    change: "In line 56 replace only the last clause, macOS, where the peer reports no pid, stays a recorded gap (owed to "Unix endpoint and home hardening"), with: macOS, where the peer reports no pid, compares no peer pid or start time, a recorded gap and named carve-out owned by no route entry."
    sidecar: "Threat Model Summary, boundary-ipc: the owed clause on "Unix endpoint and home hardening" is discharged; macOS's pid gap stays named, owned by no route entry."
    rationale: "report.md:26 retires the dated gaps owed to that route entry, and report.md:33 keeps macOS's pid gap as a true claim owned by no route entry."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 56
    section: "## Threat Model Summary"
    change: "Add after line 56, at its level: On Unix each of those clients, once it holds the recorded endpoint, runs `check_endpoint_dir` on its parent before connecting (one `symlink_metadata`; a symlink, a non-directory, another owner or any group or other bit refused, an absent path passing, nothing ever chmodded), then after connecting reads `peer_admitted(client.server_euid())` ahead of the pid and start-time check, macOS included, and a peer whose euid it cannot read is refused. A refused directory reads `socket-dir-refused` and a refused euid `server-verify-failed`, with no frame on either: CLI exit 21, `viola mcp` `isError` with no line, `viola hook` exit 0 with empty streams and one `hook-decision` at WARN. Off Unix the directory check passes and no euid is compared. The seams' euid wiring was read only by this chunk's forced refusals; no committed test repeats it (chunk 2026-11-01-unix-endpoint-and-home-hardening; the founder's own answers, `inputs#I2` Q1 and Q2)."
    sidecar: "Threat Model Summary, boundary-ipc consequences: the built socket-directory and client euid checks at the CLI, hook and MCP seams, their closed words and the wiring's limit of proof."
    rationale: "Report the client seams at report.md:15, report.md:16 and report.md:17, the check at report.md:11, the decision at report.md:12, the founder's answers at report.md:93 and the wiring's limit at report.md:54."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 57
    section: "## Threat Model Summary"
    change: "Add after line 57, at its level: `run` and `revive`'s start hold the socket directory to the same check through `socket_dir_ready`: check the endpoint's parent, create it only when absent, then check again, so a race's loser reads the same. A refusal is exit 1 with the closed word `socket-dir-refused`, before any snapshot, child or `Server::bind`, the directory left as found; a `Server::bind` failure stays what it was."
    sidecar: "Threat Model Summary, boundary-ipc consequences: the start's built socket-directory refusal ahead of any snapshot, child or bind."
    rationale: "report.md:14 and report.md:107: a new refusal at the start beside the line on what blocks a start."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 60
    section: "## Threat Model Summary"
    change: "In line 60: one of five closed words becomes one of six closed words; `server-verify-failed` (off macOS) becomes `server-verify-failed` (Linux, Windows and macOS, on macOS an euid mismatch alone), and after it, before the full stop, add a sixth item: `socket-dir-refused` (the socket directory failed its check). The rest of the line stays."
    sidecar: "Threat Model Summary, boundary-ipc: `UnreachableCause` five to six, the sixth `socket-dir-refused`; `server-verify-failed` now reaches macOS as an euid mismatch alone."
    rationale: "report.md:28 moves the causes five to six, report.md:31 moves the word's reach, and report.md:10 names the variant, with no serde path."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 123
    section: "## Threat Model Summary"
    change: "Add after line 123, at its level: `viola list`'s board scan holds the home to the read rule on every OS (`check_read`), `instances/` staying `check_path`, so a home another user may read is refused whole, with or without an instance; its order and no-open are unchanged, and `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home. PROVISIONAL: the overseer's answer as founder's delegate, the founder's own word owed (`inputs#I2` Q3)."
    sidecar: "Threat Model Summary, boundary-cli trust boundary: `viola list` holds the home to the read rule on every OS, PROVISIONAL on the delegate's answer."
    rationale: "report.md:18 and report.md:112 change the board's rule on a state tree `--home` or `VIOLA_DIR` chooses; report.md:50 and report.md:94 carry its PROVISIONAL standing."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
note: "Outside this file: the body .andromeda/security-plan.md:70 still places Unix sockets in `$TMPDIR` or `/tmp`, :23 names `/tmp` as a shared namespace of the endpoint, and :114 and :442 still read owed to the discharged route entry; threat-model-summary.md:53 sits beyond pattern `threat-sock`'s count of report.md:58. D-security-input and D-security-deps read no drift here."
