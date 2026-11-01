proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 8
    section: "## Occupied Resources"
    change: Add after this line, in the same res-ipc-endpoints block, a new bullet reading «- A killed wrapper's leftover socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it and no owner is proposed, the choice recorded as a harmless file in a 0700 directory (the P5 word on chunk 2026-11-01-unix-endpoint-and-home-hardening, `inputs#I3`, the overseer's as the founder's delegate, PROVISIONAL).»
    sidecar: Occupied Resources gains the leftover socket file's standing, no remover and no owner, PROVISIONAL on the delegate's word.
    rationale: The report records this as the card's third Beyond-the-plan row (report.md:51) and in the harness surface (report.md:42); the file's endpoint group holds no such record for the product's own directory.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:51
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 14
    section: "## Occupied Resources"
    change: Add after this line, in the same res-ipc-endpoints block, a new bullet reading «- The socket directory is verified before any bind and any connect (chunk 2026-11-01-unix-endpoint-and-home-hardening). `viola_state::strict::check_socket_dir(dir)` takes one `symlink_metadata`, an absent path passing and an unstattable one `Refused::Unreadable`, then the plain `socket_dir_verdict(is_symlink, is_dir, mode, uid, euid)`, a symlink `Refused::Symlink`, not a directory `Refused::NotADirectory`, else `unix_read_verdict`; off Unix it returns `Ok(())`; `check_endpoint_dir(endpoint)` is the check on a recorded endpoint's parent. It never chmods, so a refused directory stays as found. `run` and `revive`'s start (`socket_dir_ready`) check, create the parent only when absent and check again, so a race's loser reads the same; a refused check or a failed create is exit 1, code `socket-dir-refused`, before any snapshot, child or `Server::bind`, with one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`. The eight CLI frame verbs (`client::connect`), `hook` `connect` and `mcp`'s `frame` check the recorded endpoint's parent before they connect, and none sends a frame on a refusal, the verbs exit 21 `instance-unreachable` with `detail` `socket-dir-refused`, MCP the same less `v` and no line, the hook exit 0 with empty streams and one `hook-decision` at WARN. `viola_channel::Client::connect` holds no check, because `viola-channel` has no `viola-state` edge. The word is the sixth `viola_core::UnreachableCause` (`SocketDir`), no `RefusalReason` (the founder's own answer, `inputs#I2` Q1). The `unable:` / `hint:` pair is a planning form, the founder's wording owed at Epoch 7's boundary batch.»
    sidecar: Occupied Resources registers the verified socket directory, its checks, its three exits and the sixth closed cause.
    rationale: The report lands check_socket_dir, check_endpoint_dir, socket_dir_verdict, socket_dir_ready and the three client seams (report.md:10-17, report.md:70-73); the endpoint group states the 0700 directory but no check of it.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 14
    section: "## Occupied Resources"
    change: Add after this line, in the same res-ipc-endpoints block, a new bullet reading «- The wrapper admits only its own euid (chunk 2026-11-01-unix-endpoint-and-home-hardening). `Server::serve` calls the private `serve_admitting(dispatch, peer_admitted)`, and `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))`, so a refused stream is dropped on the accept thread, before its worker thread and any frame, reading end-of-file, with no line written and nothing on the terminal. The decision is injected (`type Admission = fn(Option<u32>) -> bool`), with no env, flag or feature seam. `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)` and `PEER_EUID_REPORTED` is `cfg!(unix)`, so where the euid is reported a peer it cannot read is refused; on Windows none is read, the DACL and the pid comparison staying the controls.»
    sidecar: Occupied Resources registers the wrapper's peer-euid drop at accept and the public decision functions of viola-channel.
    rationale: New channel symbols and a new endpoint behaviour with no row in the file (report.md:12-13, report.md:75).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 14
    section: "## Occupied Resources"
    change: Replace the last sentence «macOS reports no peer pid and stays on the recorded gap.» with «Before that comparison each of them reads the connected peer's euid on Unix (`Client::server_euid` / `TokioClient::server_euid` through `peer_admitted`), and a foreign or unreadable euid is the same refusal, `server-verify-failed` (the founder's own answer, `inputs#I2` Q2 of chunk 2026-11-01-unix-endpoint-and-home-hardening). macOS reports no peer pid, so there the word means an euid mismatch alone, and the missing pid comparison stays a named carve-out owned by no route entry.»
    sidecar: The server check gains the peer euid read on Linux and macOS; macOS's pid comparison stays a named carve-out with no route owner.
    rationale: The word's reach moved to Linux, Windows and macOS, on macOS an euid mismatch alone (report.md:31), the order being connect then peer_admitted then verify_server (report.md:15-17); macOS's pid gap stays named but owned by no route entry (report.md:33).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 30
    section: "## Occupied Resources"
    change: Reword «(one of the ten codes, §Standard Contracts)» as «(one of the eleven codes, §Standard Contracts)».
    sidecar: revive --json's exit-1 codes ten to eleven, the eleventh socket-dir-refused.
    rationale: The report names this site, revive --json's exit-1 codes ten to eleven (report.md:29).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 122
    section: "## Occupied Resources"
    change: After «or `detail:"server-verify-failed"` + `during:"connect"` when the serving process is not the recorded wrapper (then no `channel-request` line)» insert «, or `detail:"socket-dir-refused"` + `during:"connect"` when the recorded endpoint's socket directory is refused (no `channel-request` line either)», the following «, at WARN» covering it.
    sidecar: The cli role file's exit 21 words gain socket-dir-refused during connect.
    rationale: Exit 21's closed causes five to six; the eight frame verbs write this process-exit at WARN with no channel-request (report.md:28, report.md:71).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 36
    section: "## Occupied Resources"
    change: Reword «21 when the instance check refuses, no live endpoint is recorded or the serving process is not the recorded wrapper» as «21 when the instance check refuses, no live endpoint is recorded, the recorded endpoint's socket directory is refused or the serving process is not the recorded wrapper».
    sidecar: link and unlink's exit 21 gains the refused socket directory.
    rationale: link and unlink are among connect's eight callers, a refused directory reading exit 21 (report.md:15).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 124
    section: "## Occupied Resources"
    change: In the `hook-<name>.ndjson` clause reword «a failed server check writes `hook-decision{detail:"server-verify-failed"}` at WARN» as «a failed server check writes `hook-decision{detail:"server-verify-failed"}` and a refused socket directory `hook-decision{detail:"socket-dir-refused"}`, each at WARN», the rest of the clause standing.
    sidecar: The hook role file's WARN decision gains socket-dir-refused.
    rationale: One hook-decision at WARN, on the dialog path hook-invoked with corr null first and decision_emitted false (report.md:16, report.md:72).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 124
    section: "## Occupied Resources"
    change: In the `mcp.ndjson` clause reword «(none for a frame tool whose instance check or server check fails, and a refused instance check writes no line at all)» as «(none for a frame tool whose instance check, socket-directory check or server check fails, and a refused instance check or socket directory writes no line at all)».
    sidecar: mcp's frame tools write no line on a refused socket directory.
    rationale: At MCP's frame a refused directory is no frame and no line, isError the record (report.md:17, report.md:73).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:73
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 125
    section: "## Occupied Resources"
    change: Reword the list as «holds `stale-heartbeat`, `unwrapped-name`, `endpoint-lost`, `program-not-found`, `version-unreadable`, `unclean-fixture` and `socket-dir-refused` among its words».
    sidecar: The process-exit detail words named here gain socket-dir-refused.
    rationale: socket-dir-refused lands on process-exit's detail enum (report.md:24, report.md:30); the row lists the enum's endpoint and start words.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:24
    dependent-of: D-arch-resources
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 163
    section: "## Occupied Resources"
    change: Reword the last sentence as «`Refused` holds `Readable` for a read-rule refusal, `Symlink` for a symbolic link `check_diagnostics` or `socket_dir_verdict` meets and `NotADirectory` for a socket directory path that is no directory, each with a fixed message.», the two rules standing.
    sidecar: Refused gains NotADirectory and Symlink a second caller; strict still holds two rules.
    rationale: New fixed-message variant and socket_dir_verdict's symlink arm (report.md:11); the two rules stay true, the write rule and the read rule still standing (report.md:32).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 187
    section: "## Occupied Resources"
    change: Replace the bullet with «- `viola list` holds the home to the read rule alone, on every OS, with or without an instance. `board::scan`'s `check_if_present(path, check)` takes the home through `check_read` and `instances/` through `check_path`, so a 0750 home refuses the whole board (`ScanRefused::Home`) with nothing opened. `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home. The overseer's technical answer as the founder's delegate, PROVISIONAL (chunk 2026-11-01-unix-endpoint-and-home-hardening, `inputs#I2` Q3), the founder's own word owed at Epoch 7's boundary batch.»
    sidecar: viola list's two rules on the home retired for the read rule on every OS, PROVISIONAL; the CARRY on "Unix endpoint and home hardening" discharged.
    rationale: The Retired bullet names viola list's two rules on the home and the owed code of that route entry (report.md:26); the scan's new rule and the keepers of the write rule (report.md:18, report.md:32); Q3's standing (report.md:45, report.md:50).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 155
    section: "## Occupied Resources"
    change: Reword «`viola_state::board::scan` runs `check_path` on the home and on `instances/`, each that exists» as «`viola_state::board::scan` runs `check_read` on the home and `check_path` on `instances/`, each that exists», the rest of the line standing.
    sidecar: The list check's home rule moves to check_read at the instance-files row.
    rationale: The scan takes the home through check_read and instances/ through check_path on every OS, order and no-open unchanged (report.md:18); this row still states the retired write rule on the home (report.md:26).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 218
    section: "## Occupied Resources"
    change: Add after this line, in the same res-filesystem block, a new bullet reading «- Harness `cleanup` holds the same reading as `holder_gone` on Unix. `crates/viola-e2e/src/harness/cleanup.rs` `unconnectable` reads a connect refused or `NotFound` as gone, in place of `!Path::exists`, so `scripts/agent-run.sh cleanup`'s `endpoint_gone` is true and `ok:true` after a force-killed wrapper, whose file stays until a next bind of that name takes it over; it removes no leftover file. As measured at the phase of chunk 2026-11-01-unix-endpoint-and-home-hardening (`false` and exit 1 before it, `.andromeda/runs/2026-11-01T15-28-14-phase/p3-cleanup-measure.md`); witness `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone`.»
    sidecar: The harness cleanup's unconnectable recorded beside holder_gone, a connect refused or a path absent reading gone.
    rationale: The plan's expected amendment asks holder_gone's record beside the measured cleanup (report.md:63); the changed rule and its measurement (report.md:20, report.md:42, report.md:57).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:57
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 231
    section: "## Occupied Resources"
    change: Add after this line, in the same res-filesystem block, a new bullet reading «- Test-only, outside the viola home, the socket roots under `/tmp` that `tests/support/home.rs` `SocketRoot` owns, given to a test wrapper through `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`) by `Wrapper::boot_on_socket_root` and `first_life_on`, so a test owns the socket directory it loosens (`tests/security_negatives_socket_dir.rs`). `viola` knows them only as its ordinary socket root.»
    sidecar: The test-owned socket roots under /tmp registered.
    rationale: The plan's expected amendment names their row in this section (report.md:63); the helpers (report.md:21, report.md:288).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:21
note: D-arch-decisions found no drift (no dependency added, Tokio containment held, report.md:22-23); the body's own broad lines on these facts stay outside this file's proposals, and lines 122 and 155 already print in pieces, so their one-clause growth is left unsplit.
