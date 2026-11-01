proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 33
    section: "## Conventions"
    change: "- **Unreachable cause:** `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify` · `SocketDir`, each word through `as_str`, `SocketDir`'s `socket-dir-refused`) is exit 21's closed cause set. It has no serde path: the two clients write every word. It is no `RefusalReason`."
    sidecar: "conventions: `UnreachableCause` gains `SocketDir`, the sixth cause, `socket-dir-refused`."
    rationale: "report.md:10 adds the sixth cause with no serde path and no `RefusalReason`; report.md:28 moves exit 21's closed causes from five to six, a row listing the five extending."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:10
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 42
    section: "## Conventions"
    change: "In the `1` row's list of `viola revive --json` codes, after `squatted-name` read ` · socket-dir-refused` before ` · internal-error`, the list now eleven; the line then passes 2 000 chars, so break it before `(`viola revive --list` has two refusals` into a continuation line opening with one space."
    sidecar: "conventions: `viola revive --json`'s exit-1 codes extend to eleven with `socket-dir-refused`."
    rationale: "report.md:29 moves `revive --json`'s exit-1 codes from ten to eleven, each list holding all ten extending; report.md:14 has `start` call `refused_with` with `socket-dir-refused`, and `revive`'s start shares it."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 64
    section: "## Conventions"
    change: "- Exit `1` also covers `viola run` and `viola revive`'s start refusing the socket directory (`socket_dir_ready` in `src/cmd/run.rs`, before any snapshot, child or `Server::bind`): the stderr pair `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` (a planning form, the founder's wording owed at Epoch 7's boundary batch), or under `revive --json` the document with code `socket-dir-refused`, and one `process-exit{subject:\"self\", exit_code:1, detail:\"socket-dir-refused\"}`; no path, pid or uid. A failed create and a refused check read this one code; a `Server::bind` failure stays what it was."
    sidecar: "conventions: the socket-directory refusal's absence and its OPEN owner retire; exit 1's refusal and its code stand in the row."
    rationale: "report.md:26 retires the socket-directory check and code owed to the route entry; report.md:14 and report.md:70 and report.md:76 hold the built refusal, and report.md:95 has a failed create and a refused check read one code."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 88
    section: "## Conventions"
    change: "In the first parenthesis read: (on Unix a connected peer whose euid is not this user's or cannot be read, `peer_admitted`; on Linux and Windows the connected peer's pid or OS start time not the snapshot's, or that check not made). In the last sentence, after the strict-modes instance check's parenthesis and before `come first`, add: `and then the socket-directory check on the recorded endpoint (`check_endpoint_dir`, exit 21, `socket-dir-refused`)`. The rest of the row stands."
    sidecar: "conventions: exit 21's server-verify row widens to the peer euid on Unix; the socket-directory check joins the order before the connect."
    rationale: "report.md:15 gives `connect` the order `check_endpoint_dir`, `Client::connect`, `peer_admitted(client.server_euid())`, `verify_server`, a refused euid being `CallError::ServerUnverified`; report.md:31 moves the word's reach."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 94
    section: "## Conventions"
    change: "- Exit `21`'s human hints: `src/human/hints.rs` holds the unwrapped-name hint, the endpoint-lost hint and the socket-directory hint, each last on stderr and plain ASCII (their wording: design-system, surface cli-send). `src/cmd/client.rs` `hint_cause` keys the causes `not-running` · `unwrapped-name` · `endpoint-lost` · `server-verify` · `socket-dir`; `not-running` is the key of the cause the log reads `instance-dead`."
    sidecar: "conventions: exit 21's hints gain the socket directory's hint and the hint cause `socket-dir`."
    rationale: "report.md:15 names hint cause `socket-dir` and report.md:19 adds `send_hint` cause `socket-dir` in `src/human/hints.rs`; report.md:76 holds it plain ASCII with `hint:` last."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:19
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 100
    section: "## Conventions"
    change: "- An exception: `server-verify-failed` is claimed on Linux, Windows and macOS, where on macOS it means an euid mismatch alone; macOS compares no peer pid, `v1-18`'s named carve-out, owned by no route entry."
    sidecar: "conventions: `server-verify-failed` reaches macOS as an euid mismatch; macOS's pid gap stays named, owned by no route entry."
    rationale: "report.md:31 moves the word's reach to Linux, Windows and macOS; report.md:26 retires the dated owner and report.md:33 leaves macOS's pid gap named with no route entry; report.md:68 keeps macOS's server check the named carve-out."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 112
    section: "## Conventions"
    change: "- Exit `21`'s refused socket directory: on `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, `src/cmd/client.rs` `connect`'s refused check is `CallError::SocketDirRefused` to `Unreachable::SocketDir`, writing `process-exit` at WARN with `exit_code` 21, `detail` `socket-dir-refused` and `during` `connect`, and no `channel-request`; the start's is `process-exit` with `subject` `self` and `exit_code` 1, a step with no span and one line."
    sidecar: "conventions: a frame verb's refused socket directory, its mapping and its one `process-exit` line."
    rationale: "F8 carried beside the CLI mapping; report.md:15 holds the mapping and report.md:70 and report.md:71 hold the two lines."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 118
    section: "## Conventions"
    change: "In the `instance-unreachable` cause list read `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`. The rest of the row stands."
    sidecar: "conventions: MCP's `instance-unreachable` causes extend with `socket-dir-refused`."
    rationale: "report.md:28 extends each list of the five; report.md:17 has MCP's `frame` answer `result::unreachable(UnreachableCause::SocketDir)`, and report.md:111 witnesses the word on MCP less `v`."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 125
    section: "## Conventions"
    change: "In the parenthesis read: (on Unix its peer euid not this user's or unreadable, `peer_admitted`; else its pid or OS start time not the snapshot's, or the check cannot be made; the start time read on `tokio::task::spawn_blocking`). The rest of the row stands, a refused euid reading `UnreachableCause::ServerVerify`."
    sidecar: "conventions: MCP's frame tools widen the server check to the peer euid on Unix."
    rationale: "report.md:17 has `!peer_admitted(client.server_euid()) || !server_verified(…)` to `UnreachableCause::ServerVerify`, with no frame and no line."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 125
    section: "## Conventions"
    change: "- A frame tool (`send`, `wait`, `last`, `answer`) meeting a refused socket directory: MCP's `frame`, after `live_endpoint`, answers `result::unreachable(UnreachableCause::SocketDir)` with no frame and no line, `isError` being the record, since MCP's result has no diagnostic line for this."
    sidecar: "conventions: MCP's `frame` on a refused socket directory, its `isError` the record."
    rationale: "F7 carried beside the frame tools' server row; report.md:17 and report.md:73 hold it."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
    fact: F7
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 57
    section: "## Standard Contracts"
    change: "- Before its first frame, the client of `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.event` and `hook.dialog`, and of the MCP `send` / `wait` / `last` / `answer` tools, reads the connected stream's peer euid (`Client::server_euid`, `TokioClient::server_euid`, each writing and logging nothing) and where it is reported (`PEER_EUID_REPORTED`, Unix) refuses another euid or one it cannot read (`peer_admitted`); it then compares the peer pid and that process's OS start time with the snapshot's `pid` and `started_at` from its one snapshot read (Session liveness, below); a refusal, a mismatch, or a check that cannot be made, writes no frame. macOS reports no peer pid and holds the euid check alone, the named carve-out; Windows reads no euid, its DACL and the pid comparison the controls."
    sidecar: "standard-contracts: the client's check before its first frame gains the peer euid on Unix; macOS's gap is the named carve-out."
    rationale: "report.md:12 registers `Client::server_euid`, `TokioClient::server_euid`, `peer_admitted` and `PEER_EUID_REPORTED`; report.md:15, report.md:16 and report.md:17 wire them at the three seams; report.md:33 keeps macOS's no-peer-pid claim as the named carve-out."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 63
    section: "## Standard Contracts"
    change: "Add at the row's end: Between that read and the connect each client checks the recorded endpoint's directory (`viola_state::strict::check_endpoint_dir`), and a refused directory writes no frame: `src/cmd/client.rs` `connect` (`CallError::SocketDirRefused`, one `process-exit`), the hook's `connect` (`NotSent::SocketDir`, one `hook-decision`) and MCP's `frame` (`UnreachableCause::SocketDir`, no line)."
    sidecar: "standard-contracts: the check on the recorded endpoint's directory joins each client's order, with each seam's record."
    rationale: "report.md:11 adds `check_endpoint_dir`; report.md:15, report.md:16 and report.md:17 place it at the three seams before the connect; report.md:71, report.md:72 and report.md:73 give each seam's record."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 93
    section: "## Standard Contracts"
    change: "In the row's opening read: `hook.event`, sent id-less by `hook` once its instance check (`trusted`), its socket-directory check and its server check, the peer euid's included, have passed (a notification, no reply; a refused or absent instance, a replayed or absent snapshot, a refused socket directory or a failed server check sends nothing and the hook fails open). The rest of the row stands."
    sidecar: "standard-contracts: `hook.event` also needs the socket-directory check and the peer euid passed."
    rationale: "report.md:16 gives the hook's `connect` the same three checks in the same order, exit 0, empty streams and no frame, a refused euid being `NotSent::ServerUnverified`."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 111
    section: "## Standard Contracts"
    change: "Add at the row's end: A refused socket directory at either verb, met in `connect` after that check, prints the same document with `detail` `socket-dir-refused` (exit 21), with one `process-exit` at WARN."
    sidecar: "standard-contracts: `viola link` and `viola unlink` gain the socket-directory document."
    rationale: "report.md:15 keeps `link` and `unlink` among `connect`'s 8 callers, and report.md:71 holds the line at the eight CLI frame verbs."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 112
    section: "## Standard Contracts"
    change: "- The exit-21 document's `detail` is one of six closed causes: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`; its `v` is 1 and its `error` `instance-unreachable` on all six."
    sidecar: "standard-contracts: the exit-21 document's closed causes move from five to six."
    rationale: "report.md:28 moves exit 21's closed causes from five to six, each row reading `five closed` changing; report.md:10 adds the word."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 118
    section: "## Standard Contracts"
    change: "Add at the row's end: A refused socket directory at any of the six, met in `connect` after that check, prints the same document with `detail` `socket-dir-refused` (exit 21), with one `process-exit` at WARN and no `channel-request`."
    sidecar: "standard-contracts: the six CLI frame verbs gain the socket-directory document."
    rationale: "report.md:15 places the check in `connect`, whose callers these six are, and report.md:71 holds the line at the eight CLI frame verbs."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 122
    section: "## Standard Contracts"
    change: "- `Launch` carries `json: bool` (`run` passes false, `revive` passes `args.json`). With it true, `refused_with` (`src/cmd/run.rs`) prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and a program not found, which has no human pair, prints `internal-error`; each returns before `spawn_child`."
    sidecar: "standard-contracts: `refused_with` also prints `socket-dir-refused`."
    rationale: "report.md:14 has `start` call `refused_with(launch.json, \"socket-dir-refused\", …)` before any snapshot, child or `Server::bind`, and `revive`'s start shares it."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture.md
    near: 100
    section: "## Conventions"
    change: "In the **CLI exit codes** line, in `1`'s parenthesis read: (a refused home-tree check of `run`, `viola revive`'s start or `verify`, and a refused socket directory of `run` or `viola revive`'s start, among them); at `21`'s end, after the frame verbs' parenthesis, add: , and a refused socket directory of a CLI frame verb, `viola link` or `viola unlink`."
    sidecar: "architecture: the exit-code line names the refused socket directory under exits 1 and 21; the exit set unchanged."
    rationale: "report.md:14 refuses the start at exit 1 and report.md:71 the eight CLI frame verbs at exit 21; report.md:37 leaves the exit set unchanged."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 226
    section: "## Standard Contracts"
    change: "In the `instance-unreachable` line read `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused` (`viola_core::UnreachableCause`). The rest of the line stands."
    sidecar: "architecture: the exit-21 document's `<cause>` list extends with `socket-dir-refused`."
    rationale: "report.md:28 extends each list of the five, with the six list words; report.md:10 adds the word to `UnreachableCause`."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture.md
    near: 229
    section: "## Standard Contracts"
    change: "In the `viola revive --json` line read `<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`. The rest of the line stands."
    sidecar: "architecture: `viola revive --json`'s `<code>` list extends with `socket-dir-refused`."
    rationale: "report.md:29 moves `revive --json`'s exit-1 codes from ten to eleven, each list holding all ten extending; report.md:14 has `revive`'s start share the refusal."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
note: "conventions#5.1 stands at 7955 bytes with its line 42 at 1997 chars, so only the revive code goes in it and the exit-1 and exit-21 socket-directory clauses stand in the rows at lines 64 and 112; body line 158 (contract-instance-snapshot, no picked entity here) stays true of macOS's pid and its «recorded gap» reads as the named carve-out; F1, F2, F3, F4, F5, F6, F9 and F11 are established-decisions, F10, F13, F14 and F15 occupied-resources, F12 cross-cutting-patterns, none carried here."
