proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 39
    section: "## Stack and Technologies"
    change: "Keep the row and end it: on Unix both clients also read the serving process's euid from the same creds, `peer_creds().ok()?.euid()` (`Client::server_euid`, `TokioClient::server_euid`), none on Windows, where the DACL and the pid comparison stay the controls."
    sidecar: "Stack, Wrapper IPC: the peer euid read joins the peer pid read on Unix."
    rationale: "report.md:12 adds the euid read through interprocess's `peer_creds`, no dependency moved (report.md:23); the row names only the pid read."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: "## Established Decisions"
    change: "- No check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge, and `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design; the socket-directory check stands at the channel seam's callers."
    sidecar: "Message Broker / IPC: no check inside `Client::connect`, recorded with its reason."
    rationale: "report.md:53 records the negative fact and its reason."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:53"
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "- `viola_channel` exports `peer_euid_admitted`, `peer_admitted` and `PEER_EUID_REPORTED`, the last `cfg!(unix)`: the decision is `!reported || observed == Some(own)`, so a peer whose euid cannot be read is refused where it is reported, and on Windows the DACL and the pid comparison stay the controls. `Client::server_euid` and `TokioClient::server_euid` supply the observed euid, writing and logging nothing."
    sidecar: "Message Broker / IPC: the peer euid decision and its exports."
    rationale: "report.md:12 lands the three exports, the decision and both readers."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 81
    section: "## Established Decisions"
    change: "- The wrapper's `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))`, so a refused stream is dropped on the accept thread, before its worker thread and any frame, reading end-of-file, writing no line, nothing on the terminal and moving no wheel; the decision is injected as `type Admission`, through private `serve_admitting`."
    sidecar: "Message Broker / IPC: the wrapper's drop of a refused peer on its accept thread."
    rationale: "report.md:13 and report.md:75 land the filter, its injected decision and its silence."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13"
    fact: F5
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "- On macOS no peer pid is reported, so `viola_state::liveness::server_verdict` returns `Matches` with no comparison: the recorded gap, named and owned by no route entry. The macOS client euid and socket-directory checks and the 0755-directory case stand built, so on macOS `server-verify-failed` means an euid mismatch alone."
    sidecar: "Message Broker / IPC: the owed macOS clause discharged, the pid gap kept and owned by no route entry."
    rationale: "report.md:26 retires the dated gaps and the owed code; report.md:33 keeps the pid gap named but unowned; report.md:31 moves the word's reach; report.md:107 and report.md:110 read the macOS case and the absent comparison as met."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "- `viola_state::strict::check_socket_dir` takes one `symlink_metadata`: an absent path passes, an unstattable one is `Refused::Unreadable`, a symlink `Refused::Symlink`, a non-directory the new fixed-message `Refused::NotADirectory`, else the read rule by `socket_dir_verdict`; it never chmods, a refused directory stays as found, and off Unix it returns `Ok(())`."
    sidecar: "Message Broker / IPC: the socket-directory check's verdicts."
    rationale: "report.md:11 lands the check, its verdicts and its no-chmod rule."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "- Standing authority: a refused socket directory's new word and the client euid reading `server-verify-failed` on Linux and macOS are the founder's own word (`inputs#I2` Q1, Q2); the board's rule on every OS is the overseer's as delegate, PROVISIONAL (`inputs#I2` Q3), the founder's own word owed at Epoch 7's boundary batch."
    sidecar: "Message Broker / IPC: the authority behind the new word, the euid reading and the board's rule."
    rationale: "report.md:93 and report.md:94 state whose word each is, and report.md:50 owes the founder's word on Q3 at that batch."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:93"
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 125
    section: "## Established Decisions"
    change: "- `viola_mcp`'s `result::unreachable(cause)` builds the `instance-unreachable` result from a `viola_core::UnreachableCause`: `of_reply` reads `Connect` as `InstanceDead` and `Closed | Io` as `EndpointLost`, `of_board` reads the board's refusal as `StrictModes`, and `tools.rs` `frame` reads a refused `check_instance` as `StrictModes`, no `live_endpoint` by `without_endpoint` ([Session Liveness]), a refused `check_endpoint_dir` after `live_endpoint` as `SocketDir`, and a refused `peer_admitted(client.server_euid())` or a failed `server_verified` as `ServerVerify`, each with no frame and no line, `isError` the record."
    sidecar: "MCP: `frame` reads a refused socket directory as `SocketDir` and a refused euid as `ServerVerify`."
    rationale: "report.md:17 lands both readings in `frame`."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17"
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 156
    section: "## Established Decisions"
    change: "- The event and dialog paths, once their instance check has passed ([Hook Contract]), share one connect: `replay::read_snapshot_or_replay` (a replayed or absent snapshot is an endpoint not reached: the hook fails open with `channel-unreachable`), then `check_endpoint_dir` on the recorded endpoint, then `Client::connect_by`, then `peer_admitted(client.server_euid())`, then `verify_server` (§Standard Contracts, Session liveness). A refused directory gets no frame: the hook records `socket-dir-refused` and exits 0 with no body. A serving process of another euid, or whose pid or OS start time is not the snapshot's, or a check that cannot be made, gets no frame: the hook records `server-verify-failed` and exits 0 with no body, the event lost as for an unreachable endpoint."
    sidecar: "Hook Transport: the hook's connect takes the directory and euid checks."
    rationale: "report.md:16 lands the checks in the hook's `connect` with their words, and report.md:74 reads the euid refusal as `server-verify-failed` on each seam."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 171
    section: "## Established Decisions"
    change: "- `viola hook` meeting a refused socket directory exits 0 with empty streams and no frame and writes one `hook-decision` at WARN with `detail` `socket-dir-refused`, on the dialog path after `hook-invoked{corr:null}` with `decision_emitted:false`; a refused euid is `NotSent::ServerUnverified` in the same fail-open form."
    sidecar: "Hook Contract: a refused socket directory fails open and writes one line."
    rationale: "report.md:16 and report.md:72 land the fail-open form and its one line."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72"
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 198
    section: "## Established Decisions"
    change: "End the row: Then come the liveness-only pre-check (`live_endpoint`), then through `cmd::client::connect` the socket-directory check (`check_endpoint_dir`, a refusal exit 21, `socket-dir-refused`), the connect, the peer euid check and the server identity check (either refusal exit 21, `server-verify-failed`), and one request."
    sidecar: "Links: the explicit verbs' `connect` takes the directory and euid checks."
    rationale: "report.md:15 names `link` and `unlink` among `connect`'s 8 callers and lands the checks and their words in `connect`."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15"
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 258
    section: "## Established Decisions"
    change: "In the start order, read: exclusive bind of the endpoint (`socket_dir_ready` first, a refused or uncreatable directory the exit-1 refusal `socket-dir-refused` before any snapshot, child or `Server::bind`; a bind that finds the name taken is an exit-1 refusal, `squatted-name`, so two concurrent starts cannot both win)."
    sidecar: "Session Liveness: the start order's bind step opens with the socket-directory check."
    rationale: "report.md:14 lands the check ahead of `Server::bind`, `revive`'s start sharing it; report.md:95 gives a failed create and a refused check one code."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14"
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 259
    section: "## Established Decisions"
    change: "In the as-landed chain, read: endpoint bind (`socket_dir_ready` first, a refusal `socket-dir-refused`, exit 1, with no bind) in place of the bare step endpoint bind."
    sidecar: "Session Liveness: the as-landed chain's bind step opens with the socket-directory check."
    rationale: "report.md:14, the second statement of the same step in this entity."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14"
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 262
    section: "## Established Decisions"
    change: "- Each frame-writing client takes three checks in one order: `check_endpoint_dir` on the recorded endpoint, then connect and `peer_admitted(client.server_euid())`, then the server verify; the seams are `src/cmd/client.rs` `connect` with its 8 callers, `src/cmd/hook.rs` `connect` and `crates/viola-mcp/src/tools.rs` `frame`."
    sidecar: "Session Liveness: the three checks of each frame-writing client, in order."
    rationale: "report.md:15, report.md:16 and report.md:17 land the order on each of the three seams."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15"
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 266
    section: "## Established Decisions"
    change: "- Before any snapshot, child or `Server::bind`, `viola run` and `viola revive`'s start run `socket_dir_ready`: `check_endpoint_dir`, create the parent only when absent, check again, so a race's loser reads the same; a failed create and a refused check read one code, `socket-dir-refused`, exit 1, and a `Server::bind` failure stays what it was."
    sidecar: "Session Liveness: the start's socket-directory check and its one code."
    rationale: "report.md:14 lands the check on both starts; report.md:95 records the one code."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14"
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 309
    section: "## Established Decisions"
    change: "In the code list, read: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`, the rest of the row unchanged."
    sidecar: "CLI Conventions: `viola revive --json`'s exit-1 codes ten to eleven."
    rationale: "report.md:29 moves the codes from ten to eleven, each list holding all ten extending; report.md:14 has `revive`'s start share the refusal."
    basis: ".andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29"
note: F7, F8 (conventions), F10, F13, F14, F15 (occupied-resources) and F12 (cross-cutting-patterns) are other categories' facts; no broad line of architecture.md §Stack and Technologies or §Established Decisions moved, while its §Conventions, §Standard Contracts and §Occupied Resources lines on the causes, the ten codes, the verified directory and the board's rule belong to those categories' passes.
