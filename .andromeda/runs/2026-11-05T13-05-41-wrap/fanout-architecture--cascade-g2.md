@proposal n=1 detector=D-retired severity=warning file=established-decisions anchor=fct-gmzv3mtg basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: ## Established Decisions
change: In [Concurrency / Backend Framework] replace «evaluated for the Windows, macOS and Linux target triples, plus a `cargo check` of the listed sync crates on all three OSes (CI job 3)» with «evaluated for the one target triple `x86_64-unknown-linux-gnu`, plus a `cargo check` of the listed sync crates on `ubuntu-latest` (CI job 3)»; the sole-root tokio ban, `scripts/sync-crates.txt` and the `channel-tokio-free.sh` clause stay.
sidecar: [Concurrency / Backend Framework] — the deny-sync tokio ban is evaluated for one Linux triple and the sync-crate check runs on ubuntu-latest alone (Linux-only ruling, 2026-11-04).
rationale: The report's Schema / config says `deny-sync.toml` reads `targets = ["x86_64-unknown-linux-gnu"]` (was three triples), and its Harness / gate surface says CI keeps `strategy.matrix.os: [ubuntu-latest]` with the windows-2025 and macos-latest legs retired; the sole-root tokio ban passed (Outcome, security).
@proposal n=2 detector=D-retired severity=warning file=established-decisions anchor=fct-ad44nn2e basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: In [PTY] drop every Windows-only clause: the `TerminateProcess` / windows-sys kill fallback; the `ReadConsoleW` host-stdin reader sentence (now «`viola_pty::host_stdin()` is `std::io::stdin()`; its callers are `run`'s pump and the fake agent»); the swap trigger «An own ConPTY on windows-sys 0.61.2 + nix 0.31.3»; the Windows console modes of `HostTerminal` (it keeps the Unix termios raw mode and restores it on Drop); the `%USERPROFILE%` and Windows-search reasons (viola still sets the child's cwd and resolves the child program to an absolute path itself); the ConPTY input-writer / `CTRL_CLOSE` reason; the H2 windows-2025 key-loss measurement as current fact (re-word as history recoverable at tag `multi-os-final` or drop, keeping «viola never holds, queues or re-orders a human key behind a resize» and «a test that writes a key after a resize waits for the child to observe the new size first»); and the whole sideloaded-ConPTY passage (`restrict_dll_search`, `pin_companions` / `bin/<version>-<hash>/conpty/`, the pre-load, the `conpty-sideload` / `conpty` words, `sideload_fallback`, the inbox spawners, the sideload H2 run and the DA1 stall); state instead «`viola_pty::pty_backend()` is always `openpty`; `host_size()` reads `TIOCGWINSZ`; `PtyError::Load` is gone»; in the rationale replace «0.9.0 is excluded because of the Windows garbage-read bug wezterm#6783» with «the Windows garbage-read bug of 0.9.0 (wezterm#6783) that justified the `=0.8.1` pin left with the Windows code; moving off 0.8.1 is owned by the CARRY on «Version done-check»».
sidecar: [PTY] — the ConPTY sideload, the System32 DLL restriction, the ReadConsoleW reader, the console host_size arm and the TerminateProcess fallback retired with the Windows code; pty_backend is always openpty; the 0.9.0 Windows exclusion no longer justifies the pin (Linux-only ruling, 2026-11-04).
rationale: The report's Symbols / APIs removes `mod sideload` (`restrict_dll_search`, the pre-load), the `conpty-sideload` · `conpty` words (`pty_backend()` now always `openpty`), `host_stdin()`'s `ReadConsoleW` reader, `host_size()`'s `GetConsoleScreenBufferInfo` arm, the windows-sys `TerminateProcess` fallback and `PtyError::Load`, child exit still detected on the handle wait; its Retired bullet retires the ConPTY sideload and its CARRY fact (fct-swb6jipj) says the 0.9.0 Windows bug that justified `=0.8.1` leaves with this chunk.
@proposal n=3 detector=D-new-detail severity=warning file=established-decisions anchor=fct-ad44nn2e fact=F1 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-swb6jipj
section: ## Established Decisions
change: Add after the [PTY] entry: «- RUSTSEC-2026-0441 (`pin-utils` unmaintained, via `portable-pty =0.8.1` → `nix 0.25.1`) reads red in supply-chain's advisories family; a CARRY on «Version done-check» owns moving `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`, with no ignore-list widening.»
sidecar: [PTY] — RUSTSEC-2026-0441 via the portable-pty =0.8.1 pin recorded red, owned by a CARRY on «Version done-check» (inputs#I3, inputs#I4).
rationale: The report's Cross-project claims record ci#42030025050's supply-chain failure on RUSTSEC-2026-0441 (`pin-utils 0.1.0` ← `nix 0.25.1` ← `portable-pty 0.8.1` ← `viola-pty`) and the approved CARRY on «Version done-check» with no ignore-list widening.
@proposal n=4 detector=D-retired severity=warning file=established-decisions anchor=fct-7xqqphkl basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: Retire the OPEN item on the Windows-host rendering of the child's wide character under a 1-column ConPTY (owner "Windows-only live measurements"): it is a Windows claim retired with the OS target, the code recoverable at tag `multi-os-final`.
sidecar: [PTY] OPEN — the Windows 1-column ConPTY wide-character reading retired with the Windows code (Linux-only ruling, 2026-11-04).
rationale: The report's Retired bullet retires the Windows target and says the working-route entry «Windows-only live measurements» was retired at the 2026-11-04T23-11-43-wrap, its master sites re-pointed or retired with the Windows claim they owe (Expected amendments, `wolm`).
@proposal n=5 detector=D-retired severity=warning file=established-decisions anchor=fct-va43dnf2 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: In [Delivery Confirmation] replace «This is proved under the fake agent, at the unit and cross-process layers on the three CI OSes.» with «This is proved under the fake agent, at the unit and cross-process layers on the Linux CI runner (`ubuntu-latest`).», and from the «Not measured:» list drop «an LF typed inside a paste on Windows (owed to "Windows-only live measurements")».
sidecar: [Delivery Confirmation] — the empty-text proof runs on the one Linux CI leg; the Windows paste-LF measurement retired with the Windows target.
rationale: The report's Harness / gate surface keeps CI on `[ubuntu-latest]` with the windows-2025 and macos-latest legs retired, and its Retired bullet retires the Windows target and the «Windows-only live measurements» entry.
@proposal n=6 detector=D-cascade severity=warning file=established-decisions anchor=fct-ak4xjyyx basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: In [Database / State Store] replace «Lock files are separate from logs because append + exclusive lock fails on Windows (rust-lang/rust#54118).» with «Lock files are separate from logs: chosen while Windows was a target, where append + exclusive lock fails (rust-lang/rust#54118), and kept on the Linux-only product.»
sidecar: [Database / State Store] — the separate-lock-file rationale re-worded as history of the retired Windows target; the layout stands.
rationale: The report's Retired bullet makes the OS target Linux only, and its Counts class `os-target` re-words a Windows claim to the Linux statement, the claim it carries kept; no change to the lock layout is reported.
@proposal n=7 detector=D-arch-decisions severity=warning file=established-decisions anchor=fct-kfz3cseb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: In [Snapshot writer] replace the sentences from «On Windows a `persist` refused with raw OS error 5» through «(chunk 2026-10-13-epoch-4-survivors)» with «`replace_private` is one attempt: a failed `persist` returns at once, the target as it was; `write_snapshot` calls it.»; in the `replace_private_shared` sentence drop «, the Windows x64 ConPTY companions under `bin/<version>-<hash>/conpty/`» (its files the plugin files); drop the companions' held-open sentence (`FILE_SHARE_READ`, `retry_open`, `open_held_waiting`).
sidecar: [Snapshot writer] — replace_private is one attempt; the Windows retry loop (retry_replace, REPLACE_ATTEMPTS, replace_private_retrying, replace_private_with, write_snapshot_with) and the companions' held open removed.
rationale: The report's Symbols / APIs removes `fs.rs`'s `HOST_IS_WINDOWS`, `retry_replace`, `REPLACE_ATTEMPTS`, `replace_private_retrying`, `replace_private_with` (`replace_private` is one attempt; `write_snapshot` calls it; `write_snapshot_with` gone; `replace_private_shared` stays) and `pin.rs`'s `pin_companions`, the companions' held open, `retry_open` and `open_held_waiting`.
@proposal n=8 detector=D-arch-decisions severity=warning file=established-decisions anchor=fct-k7elqwqc basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-baujzndh
section: ## Established Decisions
change: Drop «, and `place_private_new_placed_copy_runs_at_once` runs it on Windows» so the item ends «`place_private_new_placed_copy_execs_at_once` execs the placed copy from it on Unix.»
sidecar: [Snapshot writer] — the Windows run of the placed-copy test left with the Windows code.
rationale: The report's Outcome says the carry pattern (`cfg!?\(.*(windows|target_os = "macos")`) reads no `.rs` line, and its Retired bullet retires the Windows target, so no Windows test arm remains.
@proposal n=9 detector=D-retired severity=warning file=established-decisions anchor=fct-wfmyj5kz basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: In [Message Broker / IPC] drop the sentence «On Windows the client connects only through the SQOS Identification open … (as measured at chunk 2026-09-25-security-prerequisites).» and the sentence «macOS caps socket paths at about 104 bytes.»; replace the rationale «one API over named pipes and Unix sockets, sync and Tokio flavours, and no daemon» with «one API over Unix sockets, sync and Tokio flavours, and no daemon».
sidecar: [Message Broker / IPC] — the SQOS client open, the named-pipe half of the rationale and the macOS socket-path note retired; the endpoint is a Unix socket only.
rationale: The report's Retired bullet retires the Windows named-pipe endpoint with its SDDL / SQOS controls and the macOS `$TMPDIR/viola/` socket dir; Symbols / APIs removes the SQOS open and named-pipe arms of `client.rs` · `endpoint.rs` · `tokio_client.rs` · `server.rs` and `server/win.rs`.
@proposal n=10 detector=D-retired severity=warning file=established-decisions anchor=fct-rrvv5rrr basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: Re-word to «`viola_channel::Client::server_pid` and `tokio_client::TokioClient::server_pid` return the connected stream's peer pid through interprocess's `peer_creds().pid()` (`SO_PEERCRED`); they write and log nothing. Every frame-writing client compares that pid and the process's OS start time with the snapshot's before its first frame (§Standard Contracts, Channel methods), so a squatted or rogue server is never sent a frame.» — the Windows `GetNamedPipeServerProcessId` arm, the macOS no-pid arm and `SERVER_PID_REPORTED` gone.
sidecar: [Message Broker / IPC] — server_pid reads SO_PEERCRED alone; SERVER_PID_REPORTED folded; the Windows and macOS arms retired.
rationale: The report's Symbols / APIs removes the constant `SERVER_PID_REPORTED` (and its argument in `src/cmd/hook.rs`, `src/cmd/client.rs` and `viola-mcp`'s `tools.rs`) and the named-pipe arms; its Retired bullet retires the macOS no-peer-pid carve-out.
@proposal n=11 detector=D-arch-decisions severity=warning file=established-decisions anchor=fct-wqjpkbtb basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: Re-word to «`viola_channel` exports `peer_euid_admitted` and `peer_admitted`: `peer_euid_admitted(observed, own)` is `observed == Some(own)`, so a peer whose euid cannot be read is refused. `Client::server_euid` and `TokioClient::server_euid` supply the observed euid, writing and logging nothing.» — `PEER_EUID_REPORTED`, the `reported` parameter and the Windows DACL clause gone.
sidecar: [Message Broker / IPC] — PEER_EUID_REPORTED folded: peer_euid_admitted is observed == Some(own), an unreadable euid still refused.
rationale: The report's Symbols / APIs folds `PEER_EUID_REPORTED`'s Linux value: `peer_euid_admitted(observed, own)` is `observed == Some(own)`, an unreadable euid still refused, `peer_admitted` / `serve_admitting` keeping their roles, no seam added.
@proposal n=12 detector=D-retired severity=warning file=established-decisions anchor=fct-y7p4rnnj basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: Retire the macOS no-peer-pid item and put in its place «`viola_state::liveness::server_verdict` and `verify_server` take no `peer_pid_reported` parameter: every verdict compares the peer pid and start time (`server_verdict_rows` holds no no-pid row).»
sidecar: [Message Broker / IPC] — the macOS no-peer-pid carve-out and server_verdict's macOS Matches arm retired.
rationale: The report's Symbols / APIs says `liveness::server_verdict` / `verify_server` lose the `peer_pid_reported` parameter and the macOS `Matches` arm (`server_verdict_rows` loses its no-pid row); its Retired bullet retires the macOS no-peer-pid carve-out.
@proposal n=13 detector=D-arch-decisions severity=warning file=established-decisions anchor=fct-kbitf2mh basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: Drop «, and off Unix it returns `Ok(())`» so the sentence ends «it never chmods, a refused directory stays as found.»
sidecar: [Message Broker / IPC] — check_socket_dir's off-Unix Ok(()) arm removed; its Unix body unconditional.
rationale: The report's Symbols / APIs removes the off-Unix `Ok(())` arm of `check_socket_dir` (its Unix body unconditional; `NotADirectory` · `Symlink` · `Unreadable` unchanged).
@proposal n=14 detector=D-retired severity=warning file=established-decisions anchor=fct-vxml3b56 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: Re-word to «A hold near the 3600 s ceiling is first read live at "Version done-check"; the ceiling is PROVISIONAL.» — the Windows leg owed to "Windows-only live measurements" retired.
sidecar: [Hook Contract] — the long hold's Windows leg retired with the Windows target; the ceiling's «Version done-check» owner stays.
rationale: The report's Retired bullet retires the Windows target and the «Windows-only live measurements» entry, its master sites retired with the Windows claim they owe.
@proposal n=15 detector=D-retired severity=warning file=established-decisions anchor=fct-4coxmu76 basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: In [Human Takeover / Wheel] drop the passage from «On Windows the platform decides what arrives (the founder's live ruling F-W3, 2026-10-04, relayed by the overseer)» through «The error only ever favours the human.»; the paragraph then continues «A bridge send is written as one `ESC[200~…ESC[201~` + CR write.»
sidecar: [Human Takeover / Wheel] — the Windows input passage (inbox and sideloaded ConPTY, F-W3) retired with the Windows code.
rationale: The report's Retired bullet retires the Windows target and the ConPTY sideload (vendored binaries, `sideload_fallback`), and Symbols / APIs removes `host_stdin()`'s console reader, so no Windows input path remains.
@proposal n=16 detector=D-retired severity=warning file=body anchor=fct-l5jpezvw dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: ## Established Decisions
change: Replace «a `cargo check` of the sync crates on all three OSes» with «a `cargo check` of the sync crates on `ubuntu-latest`».
sidecar: [Concurrency / Backend Framework] body line — the sync-crate check runs on the one Linux CI leg.
rationale: Same drift as the group's [Concurrency / Backend Framework] entity: CI keeps `[ubuntu-latest]` alone and `deny-sync.toml` one Linux triple.
@proposal n=17 detector=D-retired severity=warning file=body anchor=fct-omwrdnlc dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: ## Established Decisions
change: Re-word to «[PTY] portable-pty `=0.8.1` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill). Child exit is detected on the process handle, never on EOF; `pty_backend()` is `openpty`; viola never holds, queues or re-orders a human key behind a resize and never answers DA1 itself. Rationale: 0.8.1 is spike-proven; moving off it is owned by the CARRY on «Version done-check».» — the Windows sideload, the System32 restriction and the 0.9.0 Windows rationale retired.
sidecar: [PTY] body line — the ConPTY sideload and DLL-search clauses and the Windows pin rationale retired.
rationale: Same drift as the group's [PTY] entity: the report removes `mod sideload`, `restrict_dll_search` and the conpty backend words, and its CARRY fact says the 0.9.0 Windows bug leaves with this chunk.
@proposal n=18 detector=D-retired severity=warning file=body anchor=fct-bq65okf4 dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: Retire the OPEN line (the real `claude` key loss after a resize on Windows and the sideloaded ConPTY's DA1 start stall, owner "Windows-only live measurements"): both are Windows claims retired with the OS target.
sidecar: [PTY] OPEN body line — the Windows resize key-loss and sideload DA1 stall items retired with the Windows code.
rationale: The report's Retired bullet retires the ConPTY sideload and the Windows target, and records the «Windows-only live measurements» entry retired at the 2026-11-04T23-11-43-wrap.
@proposal n=19 detector=D-cascade severity=warning file=body anchor=fct-73drmylz dependent_of=D-cascade basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: Replace «lock files are separate because append + exclusive lock fails on Windows» with «lock files are separate, chosen while Windows (where append + exclusive lock fails) was a target, and kept».
sidecar: [Database / State Store] body line — the lock-file rationale re-worded as history of the retired Windows target.
rationale: Same drift as the group's [Database / State Store] entity: the OS target is Linux only per the report's Retired bullet.
@proposal n=20 detector=D-retired severity=warning file=body anchor=fct-w2igrcse dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: Drop «; on Windows the client connects only through the SQOS Identification open, never interprocess's default connect» and replace the rationale «one API over named pipes and Unix sockets, sync and Tokio flavours, and no daemon» with «one API over Unix sockets, sync and Tokio flavours, and no daemon».
sidecar: [Message Broker / IPC] body line — the SQOS clause and the named-pipe rationale retired.
rationale: Same drift as the group's [Message Broker / IPC] entity: the report retires the Windows named-pipe endpoint and its SQOS controls.
@proposal n=21 detector=D-retired severity=warning file=body anchor=fct-4nt6qtyp dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: ## Established Decisions
change: Drop «; its Windows leg, owner: "Windows-only live measurements"» so the parenthesis reads «(`dialog_hold_s`, 60–3600 s, default 60; ceiling PROVISIONAL, owner: "Version done-check")».
sidecar: [Hook Contract] body line — the dialog hold's Windows leg retired; the ceiling's owner stays.
rationale: Same drift as the group's [Hook Contract] item: the report retires the Windows target and the «Windows-only live measurements» entry.
@remark
F2 (the Linux-only OS target) has no picked entity to sit beside; the body's [Scale / Product] fct-hcu7z4ke, Design Philosophy fct-igbvtfv6, Project Intent fct-iigxhka7/fct-qndsom6a, [Agent Coverage] fct-7tdo3mcl, [CI/CD] fct-bvzxxcy7, [Error Handling] fct-sha7o35z and the §Inherited Defaults/§Occupied Resources lines also go stale but no picked entity of this group details them.
