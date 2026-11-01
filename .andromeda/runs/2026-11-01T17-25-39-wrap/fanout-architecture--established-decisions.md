proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "Line reads: - On macOS no peer pid is reported, so `viola_state::liveness::server_verdict` returns `Matches` with no comparison: the recorded gap, a named carve-out owned by no route entry, so `server-verify-failed` on macOS means an euid mismatch alone. The macOS client euid and socket-directory checks are built, and the 0755-directory case runs on `macos-latest` (`tests/security_negatives_socket_dir.rs`), since chunk 2026-11-01-unix-endpoint-and-home-hardening."
    sidecar: "[Message Broker / IPC]: the macOS euid, socket-directory and 0755-case clause no longer owed to \"Unix endpoint and home hardening\"; macOS's pid gap stays named, owned by no route entry."
    rationale: "The report's Retired bullet names these dated gaps as built; its owner-title row keeps only macOS's pid gap, named and owned by no route entry, and its verify-word row narrows the word's macOS meaning to an euid mismatch (report.md:26, :31, :33, :64, :107)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 262
    section: "## Established Decisions"
    change: "Replace «then the liveness-only pre-check, then, on Linux and Windows, compares» with «then the liveness-only pre-check, then `check_endpoint_dir` on the recorded endpoint's parent (a refusal reads `socket-dir-refused`), then connects, then reads `peer_admitted(client.server_euid())` (a refusal reads `server-verify-failed`), then, on Linux and Windows, compares»."
    sidecar: "[Session Liveness]: every frame-writing client's order gains the socket-directory check before its connect and the euid decision after it."
    rationale: "The line states the clients' whole check order, and the CLI, hook and MCP seams now run two more checks before any frame (report.md:15, :16, :17)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 156
    section: "## Established Decisions"
    change: "Replace «then `Client::connect_by`, then `verify_server` (§Standard Contracts, Session liveness).» with «then `check_endpoint_dir` on the recorded endpoint (a refused socket directory, `NotSent::SocketDir` or `Asked::SocketDir`, gets no frame: exit 0, empty streams and one `hook-decision` at WARN with `detail` `socket-dir-refused`, that one line a lean of the P5 word, the overseer's as delegate, PROVISIONAL, chunk 2026-11-01-unix-endpoint-and-home-hardening's `inputs#I3`), then `Client::connect_by`, then `peer_admitted(client.server_euid())` (a refused euid is `NotSent::ServerUnverified`), then `verify_server` (§Standard Contracts, Session liveness).»"
    sidecar: "[Hook Transport]: the hook's one connect gains the socket-directory check and the euid decision, each failing open."
    rationale: "The line lists the hook's connect step by step; `src/cmd/hook.rs` `connect` now runs the same three checks as the CLI, with its own closed outcomes (report.md:16, :72, :95)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 198
    section: "## Established Decisions"
    change: "Replace «the server identity check through `cmd::client::connect`, and one request.» with «`cmd::client::connect`'s socket-directory check, euid decision and server identity check, and one request.»"
    sidecar: "[Links]: the explicit verbs' `connect` now holds the socket-directory check and the euid decision too."
    rationale: "`link` and `unlink` are among `connect`'s 8 callers, unedited, so they take its new checks (report.md:15)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 125
    section: "## Established Decisions"
    change: "Replace «no `live_endpoint` by `without_endpoint` ([Session Liveness]) and a failed `server_verified` as `ServerVerify`» with «no `live_endpoint` by `without_endpoint` ([Session Liveness]), a refused `check_endpoint_dir` after `live_endpoint` as `SocketDir`, and a refused `peer_admitted` or a failed `server_verified` as `ServerVerify`»."
    sidecar: "[MCP]: `frame` reads the new `SocketDir` cause and a refused euid as `ServerVerify`."
    rationale: "The line maps each of `frame`'s refusals to its cause, and `frame` gained both readings with no frame and no line (report.md:10, :17)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 171
    section: "## Established Decisions"
    change: "Replace «A dialog hook whose server check fails (the serving process is not the recorded wrapper, or the check cannot be made) sends» with «A dialog hook whose socket directory is refused or whose server check fails (the serving process is not the recorded wrapper, the peer's euid is refused, or the check cannot be made) sends», and append after the sentence: On a refused socket directory `hook-invoked{corr:null}` comes first, and the `hook-decision` reads `decision_emitted:false`."
    sidecar: "[Hook Contract]: a dialog hook's fail-open now also covers a refused socket directory and a refused euid."
    rationale: "The dialog path meets both new checks and fails open the same way (report.md:16, :72)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 258
    section: "## Established Decisions"
    change: "Replace «version gate, exclusive bind of the endpoint» with «version gate, the socket directory's check (`socket_dir_ready`: `check_endpoint_dir`, the parent created only when absent, then checked again, so a race's loser reads the same; a refused check or a failed create is exit 1 `socket-dir-refused`, before any snapshot, child or bind, the one code a lean of the P5 word, the overseer's as delegate, PROVISIONAL, chunk 2026-11-01-unix-endpoint-and-home-hardening's `inputs#I3`), exclusive bind of the endpoint»."
    sidecar: "[Session Liveness]: the start order gains the socket directory's check ahead of the bind, shared with `revive`'s start."
    rationale: "The line gives `run`'s start order; `start` now refuses `socket-dir-refused` before any snapshot, child or `Server::bind`, and `revive`'s start shares it (report.md:14, :95)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 259
    section: "## Established Decisions"
    change: "Replace «→ endpoint bind →» with «→ the socket directory's check (`socket_dir_ready`, a step with no span, its refusal one `process-exit` line with `detail` `socket-dir-refused`) → endpoint bind →»."
    sidecar: "[Session Liveness]: the as-landed step list gains the socket directory's check."
    rationale: "The line says every step has its place, and the new step sits on that chain with no span and one line (report.md:14, :65, :70)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 309
    section: "## Established Decisions"
    change: "Replace «`pinned-hash-mismatch` · `squatted-name` · `internal-error`» with «`pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»."
    sidecar: "[CLI Conventions]: `viola revive --json`'s exit-1 codes ten → eleven, `socket-dir-refused` added."
    rationale: "The report's `revive-codes` row moves the codes ten → eleven and says each list holding all ten extends; this list holds all ten (report.md:14, :29)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "New item: - `viola_state::strict::check_socket_dir` takes one `symlink_metadata` of the socket directory: an absent path passes, an unstattable one is `Refused::Unreadable`, a symlink `Refused::Symlink`, not a directory `Refused::NotADirectory`, else the read rule (`socket_dir_verdict`); off Unix it returns `Ok(())`. It never chmods: a refused directory stays as found. `check_endpoint_dir` runs it on a recorded endpoint's parent, at the start's bind ([Session Liveness]) and at the three client seams. A refusal reads the closed word `socket-dir-refused`, `viola_core::UnreachableCause::SocketDir`, the sixth cause, with no serde path and no `RefusalReason` (the founder's own word, live, chunk 2026-11-01-unix-endpoint-and-home-hardening's `inputs#I2` Q1). No check sits inside `viola_channel::Client::connect`: `viola-channel` has no `viola-state` edge."
    sidecar: "[Message Broker / IPC]: the verified per-user socket directory recorded."
    rationale: "The report names [Message Broker / IPC] among the expected amendments and lands this check and its word (report.md:10, :11, :53, :63, :93)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "New item: - `viola_channel::Client::server_euid` and `TokioClient::server_euid` return the connected stream's peer euid, writing and logging nothing: `peer_creds().ok()?.euid()` on Unix, none on Windows, where the DACL and the pid comparison stay the controls. `peer_admitted` decides through `peer_euid_admitted(observed, own, reported)`, `!reported || observed == Some(own)`, with `PEER_EUID_REPORTED` = `cfg!(unix)`, so a peer it cannot read is refused where reported. A client's refusal reads `server-verify-failed` on Linux and macOS (the founder's own word, live, chunk 2026-11-01-unix-endpoint-and-home-hardening's `inputs#I2` Q2). The decision and the read are tested; each seam's wiring was read only through that chunk's forced refusals, since one account can make no foreign peer and no seam may be added."
    sidecar: "[Message Broker / IPC]: the client euid check on Unix recorded, with its weakest proof."
    rationale: "New symbols and a founder decision in this category's IPC seam, with the report's own limit on the wiring's proof (report.md:12, :54, :93, :109)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 87
    section: "## Established Decisions"
    change: "New item: - The wrapper takes the same decision on each client: `Server::serve` calls the private `serve_admitting(dispatch, peer_admitted)`, and `accept_loop(listener, dispatch, admitted)` filters `incoming()` by `admitted(peer_euid_of(stream))`, so a refused stream is dropped on the accept thread, before its worker thread and before any frame. The peer reads end-of-file, and the wrapper writes no line and nothing on the terminal (no line is a lean of the P5 word, the overseer's as delegate, PROVISIONAL, chunk 2026-11-01-unix-endpoint-and-home-hardening's `inputs#I3`). The decision is injected (`type Admission = fn(Option<u32>) -> bool`), with no env, flag or feature seam."
    sidecar: "[Message Broker / IPC]: the wrapper's drop of a refused peer recorded."
    rationale: "New server-side behaviour on the channel seam, its no-line choice among the P5 word's leans (report.md:13, :75, :95, :108)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
note: "Line 147's SessionEnd direct append is not stated by the report for a refused socket directory, so no proposal is written there; D-arch-decisions found no new library or runtime (report.md:23)."
