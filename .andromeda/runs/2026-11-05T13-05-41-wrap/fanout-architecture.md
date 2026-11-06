@proposal n=1 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Design Philosophy
change: fct-igbvtfv6 becomes «**Linux only.** viola targets Linux: the core on any current distribution, built and tested on every commit in CI on `ubuntu-latest` against the fake agent, the window features of 0.2.0 on Hyprland; the Windows and macOS code was removed at chunk 2026-11-05-linux-only-code-and-ci and is recoverable at the tag `multi-os-final` (`ecbe7ab3`). No design choice may depend on tmux or a shell.» then the first-live-run sentence (R-L1, chunk 2026-10-08-first-live-test-and-self-drive) as written; drop «Cross-platform from the first commit, Windows first», «Windows is the first target», «The Windows, macOS and Linux paths build and pass CI» and «Windows live behaviour stays proven on the CI runner … until an interactive Windows host exists».
sidecar: §Design Philosophy — the «Cross-platform, Windows first» principle replaced by «Linux only» (founder rulings 2026-11-04 21:43Z/21:50Z/21:53Z, inputs#I1, inputs#I5 item 6; code recoverable at tag multi-os-final).
rationale: The report's Retired bullet (fct-tulramlm) retires the OS target «Windows first, macOS and Linux» → «Linux only: the core on any current distribution (CI on `ubuntu-latest`), the window features of 0.2.0 on Hyprland; the Windows and macOS code removed, recoverable at the tag `multi-os-final`»; Counts (fct-tjt6dmfu) marks the os-target class («cross-platform», «Windows first», «three OSes») stale.
@proposal n=2 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Stack and Technologies
change: fct-zegjs5rw reads «One native Rust binary `viola` (…)» (drop «/ `viola.exe`») and ends «GitHub Actions on `ubuntu-latest` against the fake agent» (was «on three OSes»).
sidecar: §Stack and Technologies — the one-line stack names the Linux binary and the one CI leg `ubuntu-latest`.
rationale: Retired (fct-tulramlm): the OS target, the `windows-2025` and `macos-latest` CI legs; Counts (fct-tjt6dmfu): the CI matrix 3 OS legs → `[ubuntu-latest]`.
@proposal n=3 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Established Decisions
change: [Scale / Product] fct-hcu7z4ke becomes «Personal v1, Linux only.» — no accounts, no hosting; built and CI-tested on Linux (`ubuntu-latest`) from the first commit, on-disk state parsed defensively; Rationale: the founder's rulings of 2026-11-04 cut the Windows and macOS support carried since the first commit (recoverable at the tag `multi-os-final`), and the later public version (individual Claude Code subscribers) must not need a rewrite; drop «cross-platform from day one», «on Windows, macOS and Linux» and «retrofitting cross-platform support … from the start».
sidecar: [Scale / Product] — «cross-platform from day one» retired for «Linux only» (founder rulings 2026-11-04, inputs#I1).
rationale: Retired (fct-tulramlm) names the OS target; Decisions & corrections fct-g4iyzuf4: «Viola is Linux-only; the Windows and macOS code is cut, recoverable at the tag `multi-os-final`».
@proposal n=4 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Established Decisions
change: [Agent Coverage] fct-7tdo3mcl lists «child program resolution (`resolve_program`: a given path as given, a bare name on `PATH`)» in place of «npm-shim → `claude.exe` resolution».
sidecar: [Agent Coverage] — the npm-shim → claude.exe resolution retired; resolve_program takes a given path as given and a bare name on PATH.
rationale: Symbols / APIs (fct-xxutheix): `resolve_program`'s `Search::windows`, `pathext`, `is_script`, `SHIM_TARGET`, `AgentError::BatchScriptChild` removed («a given path as given, a bare name on `PATH`; `ProgramNotFound` unchanged»); Retired (fct-tulramlm): the `.cmd`/`.bat` refusal.
@proposal n=5 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §Established Decisions
change: [Concurrency / Backend Framework] fct-l5jpezvw reads «… a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on `ubuntu-latest`» (was «on all three OSes»).
sidecar: [Concurrency / Backend Framework] — the sync-crate check runs on the one CI leg ubuntu-latest.
rationale: Harness / gate surface (fct-sy4jxsfp): `lint` keeps `strategy.matrix.os: [ubuntu-latest]`; Retired (fct-tulramlm): the `windows-2025` and `macos-latest` CI legs.
@proposal n=6 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Established Decisions
change: [PTY] fct-omwrdnlc drops «on Windows x64 the ConPTY is sideloaded, failing open to the inbox one, and every `viola` process restricts its DLL search to System32»; its Rationale reads «0.8.1 is spike-proven; the 0.9.0 Windows garbage-read bug that justified the `=0.8.1` pin left with the Windows code, and moving `portable-pty` off 0.8.1 to a release whose graph drops `pin-utils` (RUSTSEC-2026-0441) is the CARRY on «Version done-check»».
sidecar: [PTY] — the ConPTY sideload and System32 DLL-search restriction retired; the =0.8.1 pin's Windows rationale gone, its move owned by the CARRY on «Version done-check» (inputs#I3, inputs#I4).
rationale: Retired (fct-tulramlm): «the ConPTY sideload (… `sideload_fallback`, `run.conpty_sideload`)»; Symbols / APIs (fct-xxutheix): `mod sideload` (`restrict_dll_search`, the pre-load) removed, `pty_backend()` always `openpty`; Cross-project (fct-swb6jipj): «The 0.9.0 Windows garbage-read bug that justified the `=0.8.1` pin leaves with this chunk»; Dependencies (fct-www7rgz5): `portable-pty` stays `=0.8.1`.
@proposal n=7 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-pdxuthnr
section: §Established Decisions
change: the [PTY] OPEN fct-bq65okf4 reads «OPEN — whether the real `claude` loses a key typed right after a resize · owner: the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`)»; drop «the sideloaded ConPTY's DA1 start stall with no terminal on viola's stdio» and the owner «"Windows-only live measurements"».
sidecar: [PTY] OPEN — the sideloaded ConPTY DA1 stall retired with the sideload; the resize-key question re-pointed off the retired «Windows-only live measurements» to the 2026-11-04T23-11-43-wrap residuals (target another-os).
rationale: Retired (fct-tulramlm): the ConPTY sideload, and «The working-route entry «Windows-only live measurements» was retired at the 2026-11-04T23-11-43-wrap (its 17 residuals → origin `2026-11-04T23-11-43-wrap`, target `another-os`)»; Expected amendments (fct-pdxuthnr): the `wolm` sites «are re-pointed to the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`) or retired with the Windows claim they owe».
@proposal n=8 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Established Decisions
change: [Message Broker / IPC] fct-w2igrcse drops «on Windows the client connects only through the SQOS Identification open, never interprocess's default connect»; its Rationale reads «one API over Unix sockets, sync and Tokio flavours, and no daemon» (was «over named pipes and Unix sockets»).
sidecar: [Message Broker / IPC] — the Windows named pipe and its SQOS Identification open retired.
rationale: Retired (fct-tulramlm): «the Windows named-pipe endpoint (`\\.\pipe\viola-<h12>`), its SDDL / SQOS / `accept_remote(false)` controls»; Symbols / APIs (fct-xxutheix): `server/win.rs`, «the SQOS open and named-pipe arms of `client.rs` · `endpoint.rs` · `tokio_client.rs` · `server.rs`» removed.
@proposal n=9 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-pdxuthnr
section: §Established Decisions
change: [Hook Contract] fct-4nt6qtyp's hold clause reads «(`dialog_hold_s`, 60–3600 s, default 60; ceiling PROVISIONAL, owner: "Version done-check")»; drop «; its Windows leg, owner: "Windows-only live measurements"».
sidecar: [Hook Contract] — the dialog hold's Windows leg retired with the Windows target and its retired working-route owner.
rationale: Retired (fct-tulramlm): the OS target and the working-route entry «Windows-only live measurements»; Expected amendments (fct-pdxuthnr) list [Hook Contract] among the `wolm` sites, «retired with the Windows claim they owe».
@proposal n=10 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Established Decisions
change: [CI/CD] fct-bvzxxcy7 reads «GitHub Actions on `ubuntu-latest` (`strategy.matrix.os: [ubuntu-latest]`), tests against the fake agent, the real CLI only in local live tests.» and «viola resolves the child program to an absolute path itself (a given path as given, a bare name searched on `PATH`) and spawns that.»; drop «native matrix on three OSes», «(on Windows a PATH search over PATHEXT only, never the bare name)» and the whole sentence «A `.cmd`/`.bat` child is refused (exit 1, `batch-script-child`), except the `claude` npm shim … the shim is never read»; the per-test home sentence and the Rationale stay.
sidecar: [CI/CD] — the three-OS matrix, the PATHEXT search and the .cmd/.bat refusal with its npm-shim exception retired.
rationale: Retired (fct-tulramlm): the `windows-2025` and `macos-latest` CI legs and «the `.cmd`/`.bat` refusal (`batch-script-child`, `AgentError::BatchScriptChild` …)»; Symbols / APIs (fct-xxutheix): `resolve_program`'s `Search::windows`, `pathext`, `SHIM_TARGET` removed, «a given path as given, a bare name on `PATH`»; `src/run/mod.rs`'s `PATHEXT` removed.
@proposal n=11 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: §Standard Contracts
change: the revive `--json` refusal bullet fct-54jxzo44 lists ten codes, `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error` (drop `batch-script-child`).
sidecar: §Standard Contracts CLI --json — revive's closed refusal codes 11 → 10, batch-script-child retired.
rationale: Counts (fct-tjt6dmfu): «revive's closed `--json` refusal codes 11 → 10 (`batch-script-child` gone)»; Outcome fct-25zti4qi: «revive's codes ten … pinned in `tests/cross_exit_causes.rs`».
@proposal n=12 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Standard Contracts
change: the instance-snapshot bullet fct-ld365mzq reads «`pid` and `started_at` identify the wrapper process for the liveness check and for the server check every frame-writing client makes before its first frame: the connected peer's pid and its OS start time against these two fields, a mismatch writing no frame.»; drop «on Linux and Windows,» and «(macOS reports no peer pid, the recorded gap)».
sidecar: §Standard Contracts instance snapshot — the macOS no-peer-pid gap retired; the server check is unconditional.
rationale: Retired (fct-tulramlm): «the macOS no-peer-pid carve-out»; Symbols / APIs (fct-xxutheix): the constant `SERVER_PID_REPORTED` removed, `liveness::server_verdict` / `verify_server` lose the `peer_pid_reported` parameter and the macOS `Matches` arm.
@proposal n=13 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Standard Contracts
change: the `hook statusline` bullet fct-horlp4ay reads «… through `/bin/sh -c` with the same stdin (the only shell-out in viola), printing its stdout unchanged …» (drop «; none on Windows»).
sidecar: §Standard Contracts Hook contract — the statusline shell-out's Windows exception retired.
rationale: Symbols / APIs (fct-xxutheix): `WRAPPED_HERE` folded to `true`, `statusline::shell_argv` returns `[&str; 3]` (was `Option`); Retired (fct-tulramlm): the OS target.
@proposal n=14 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: delete the IPC-endpoint bullet fct-vjccfh2l («**Windows:** named pipe `\\.\pipe\viola-<h12>`, one per `viola run`»); the Unix domain socket is the one endpoint form.
sidecar: §Occupied Resources IPC endpoints — the Windows named pipe \\.\pipe\viola-<h12> retired.
rationale: Retired (fct-tulramlm): «the Windows named-pipe endpoint (`\\.\pipe\viola-<h12>`)»; Symbols / APIs (fct-xxutheix): `server/win.rs` and the named-pipe arms removed.
@proposal n=15 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: the test-only endpoint bullet fct-s2vyyjop reads «**Test-only:** endpoints named `viola-test-chan-<pid>-<label>` (a socket of that base name), created by the channel tests (`tests/channel_endpoint.rs`, the `viola-channel` crate tests and the `viola-e2e` cleanup tests). They are outside the `viola-<h12>` namespace …» as written; drop the SQOS pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` with `tests/channel_sqos_open.rs`, «a named pipe on Windows», and the deleted `tests/security_negatives_channel.rs`.
sidecar: §Occupied Resources IPC endpoints — the test-only SQOS pipe and the deleted channel test files retired.
rationale: Retired (fct-tulramlm): «the test-only SQOS pipe»; Files (fct-yq5vc7zj): `tests/channel_sqos_open.rs` and `tests/security_negatives_channel.rs` deleted whole.
@proposal n=16 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Occupied Resources
change: the socket bullet fct-wular7qm is labelled «**Linux:**» (was «Linux / macOS») and reads «Unix domain socket file `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root, …», the rest as written; drop «or `$TMPDIR`».
sidecar: §Occupied Resources IPC endpoints — the macOS $TMPDIR/viola/ socket-directory form retired; the forms are $XDG_RUNTIME_DIR/viola/ and /tmp/viola-<uid>/.
rationale: Symbols / APIs (fct-xxutheix): «the macOS `$TMPDIR/viola/` socket-directory arm (forms now `$XDG_RUNTIME_DIR/viola/` and `/tmp/viola-<uid>/`)»; Retired (fct-tulramlm): «the macOS `$TMPDIR/viola/` socket dir».
@proposal n=17 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: the binary bullet fct-lvi3drxp reads «Binary: `viola`.» (drop «(`viola.exe`)»).
sidecar: §Occupied Resources — the viola.exe binary name retired with the Windows target.
rationale: Retired (fct-tulramlm): the OS target «Linux only … the Windows and macOS code removed».
@proposal n=18 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Occupied Resources
change: the test-support feature bullet fct-4whhx2hw reads «… gates `pub mod test_support` (the `JsonFields` tracing field visitor).» (drop «; on Windows `canonical_sddl`, `dacl_of` and an independent `user_sid`»).
sidecar: §Occupied Resources workspace crates — viola-channel test_support's Windows SDDL helpers retired.
rationale: Symbols / APIs (fct-xxutheix): `viola-channel`'s «`test_support`'s `canonical_sddl` · `dacl_of` · `user_sid`» removed; Retired (fct-tulramlm): the SDDL controls.
@proposal n=19 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Occupied Resources
change: the R8 strip bullet fct-jujxlicd ends «Names only: the strip and the logs never read a value.» (drop «the registry reader»).
sidecar: §Occupied Resources environment variables — the Windows registry environment reader retired.
rationale: Retired (fct-tulramlm): «the Windows registry environment reader»; Symbols / APIs (fct-xxutheix): «`src/run/env.rs`'s registry reader (`HKCU\Environment`) and case-insensitive match (the exact strip, `claude_env_keep` and the eleven-name identity floor stay)».
@proposal n=20 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: the `VIOLA_BIN` bullet fct-7qo4h5jo names the pinned copy `<viola home>/bin/<version>-<hash>/viola` (drop «(.exe)»).
sidecar: §Occupied Resources environment variables — VIOLA_BIN's .exe form retired.
rationale: Retired (fct-tulramlm): the OS target, the Windows code removed.
@proposal n=21 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: the pinned-copy bullet fct-hnee7xdr reads «`bin/<version>-<hash>/viola`: the pinned copy … (0700), …» (drop «(.exe)» and «on Unix»).
sidecar: §Occupied Resources filesystem — the pinned copy's .exe form and Unix-only qualifier retired.
rationale: Retired (fct-tulramlm): the OS target; Symbols / APIs (fct-xxutheix): the DACL arm of `create_private_dir` removed.
@proposal n=22 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: delete the filesystem bullet fct-ekftm72e (`bin/<version>-<hash>/conpty/OpenConsole.exe` and `conpty.dll`, the ConPTY companions, `pin_companions`, `sideload_fallback`).
sidecar: §Occupied Resources filesystem — the pinned ConPTY companions under bin/<version>-<hash>/conpty/ retired.
rationale: Retired (fct-tulramlm): «the ConPTY sideload (… `bin/<version>-<hash>/conpty/` …, `sideload_fallback`, `run.conpty_sideload`)»; Symbols / APIs (fct-xxutheix): `pin.rs`'s `pin_companions`, the companions' held open removed.
@proposal n=23 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Occupied Resources
change: the `diagnostics/` bullet fct-5jecrew3 reads «dir 0700, files 0600» (drop «on Unix») and «… the folder to the read rule, the file to the read rule, an absent path passing» (drop «on Unix and the write rule on Windows»).
sidecar: §Occupied Resources filesystem — diagnostics/ checks lose the Windows owner/DACL arm.
rationale: Retired (fct-tulramlm): «the Windows home owner/DACL strict-modes arm»; Symbols / APIs (fct-xxutheix): `strict.rs`'s Windows owner/DACL arm, `windows_read_verdict`, `READ_RIGHTS` removed.
@proposal n=24 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Occupied Resources
change: the `instances/<ViolaName>/` bullet fct-cicfj3bq reads «… to the read rule (no group or other bit), `instances/` and `snapshot.json` to the write rule, and `events.ndjson` to the write rule, each that exists …» (drop «Unix» before «no group», «; Windows no read right granted to another SID» and «on Unix and the read rule on Windows»).
sidecar: §Occupied Resources filesystem — the instance-file checks lose the Windows SID/read-rule arm.
rationale: Retired (fct-tulramlm): the Windows home owner/DACL strict-modes arm; Symbols / APIs (fct-xxutheix): `windows_read_verdict`, `READ_RIGHTS` removed, `check_socket_dir`'s Unix body unconditional.
@proposal n=25 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Occupied Resources
change: the settings-override bullet fct-klpb7auf reads «… rewritten whole at every start before the spawn; written only for a pinned path of plain characters, else no override is written and no `--settings` is passed» (drop «written only on Unix, and»).
sidecar: §Occupied Resources Claude Code integration — the settings override is no longer Unix-gated (WRAPPED_HERE folded true).
rationale: Symbols / APIs (fct-xxutheix): «`WRAPPED_HERE` (its `true` folded …)»; `statusline::shell_argv` returns `[&str; 3]` («with `WRAPPED_HERE` folded it was always `Some`», deviation fct-mjkivgjx).
@proposal n=26 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Occupied Resources
change: the `viola-chaos-*` bullet fct-swegdoif reads «It is the one named carve-out to the test-data rule (its other, `seed_conpty`, left with the ConPTY sideload).» in place of «It is the second named carve-out to the test-data rule, beside `seed_conpty` (the founder's ruling, 2026-10-04).».
sidecar: §Occupied Resources filesystem — seed_conpty, the other test-data carve-out, retired.
rationale: Symbols / APIs (fct-xxutheix): «`tests/support/home.rs` loses `seed_conpty`»; Retired (fct-tulramlm): the ConPTY sideload.
@proposal n=27 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: delete the repository bullet fct-rryrskwd (`vendor/conpty/<version>/x64/{conpty.dll,OpenConsole.exe}`, `.gitattributes` `vendor/conpty/** binary`, `scripts/conpty-vendor.sh`).
sidecar: §Occupied Resources repository — vendor/conpty/ and scripts/conpty-vendor.sh retired.
rationale: Retired (fct-tulramlm): «vendored `vendor/conpty/<version>/x64/` … `scripts/conpty-vendor.sh`»; Files (fct-yq5vc7zj): the whole `vendor/conpty/` tree and `scripts/conpty-vendor.sh` deleted; Schema / config (fct-wrfywvkp): `.gitattributes` loses `vendor/conpty/** binary`.
@proposal n=28 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: delete the repository bullet fct-qe46hlhe (`target/conpty-seed/<key>/`, the `seed_conpty` per-run copy).
sidecar: §Occupied Resources repository — target/conpty-seed/ retired.
rationale: Retired (fct-tulramlm): «`target/conpty-seed/`»; Symbols / APIs (fct-xxutheix): `tests/support/home.rs` loses `seed_conpty`.
@proposal n=29 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Occupied Resources
change: delete the repository bullet fct-7wuxnmfc (`<repo parent>/viola-mutants-scratch/`, the Windows host mutation scratch).
sidecar: §Occupied Resources repository — viola-mutants-scratch/ and the harness's Windows scratch arm retired.
rationale: Retired (fct-tulramlm): «`windows-mutants.yml` … with `viola-mutants-scratch` / `HOST_SCRATCH` / `scratch_bytes`»; Symbols / APIs (fct-xxutheix): `scratch.rs` whole (`HOST_SCRATCH`, the Windows scratch wipe, `prepare`) removed.
@proposal n=30 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Infrastructure Patterns → build-system
change: in fct-jbq7vmcz the callers list reads «the `viola run` start refusals (live or stale name, tampered pinned copy, squatted endpoint, a refused home tree)» (drop «`.cmd`/`.bat` child,»).
sidecar: Build system — the .cmd/.bat start refusal left the human writer's callers.
rationale: Retired (fct-tulramlm): the `.cmd`/`.bat` refusal and its human pair; Symbols / APIs (fct-xxutheix): `src/cmd/run.rs`'s `BatchScriptChild` start refusal removed.
@proposal n=31 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: §Infrastructure Patterns → build-system
change: in fct-cotc6dxt the licence clause reads «`0BSD` only through the per-crate `[[licenses.exceptions]]` for `doctest-file`, the one 0BSD crate interprocess 2.4.4 pulls into the Linux graph, never through `allow`» (drop `recvmsg` and «the two»); the evaluation reads «evaluated for the target triple `x86_64-unknown-linux-gnu`» (was «the Windows, macOS and Linux target triples so `cfg`-gated dependencies are covered»), and `deny-sync.toml` «the same one triple» (was «the same three triples»).
sidecar: Build system — deny targets 3 triples → x86_64-unknown-linux-gnu; licence exceptions 2 → 1 (recvmsg, interprocess's Windows-only dependency, dropped; inputs#I5 item 7).
rationale: Schema / config (fct-wrfywvkp): «`deny.toml` / `deny-sync.toml`: `targets = ["x86_64-unknown-linux-gnu"]` (was three triples); `deny.toml`'s licence exceptions now `doctest-file` alone (the `recvmsg` exception dropped … its crate is interprocess's Windows-only dependency)»; Counts (fct-tjt6dmfu) name this key's `deny-triples` and `recvmsg` sites stale.
@proposal n=32 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Infrastructure Patterns → deployment-model
change: fct-4jixev2q reads «… pins a copy of itself in `~/.viola/bin/<version>-<hash>/`, writes the embedded plugin …» (drop «(on Windows x64 also the embedded ConPTY companions in its `conpty/` subdirectory, with `conpty.dll` pre-loaded from there before the spawn)»).
sidecar: Deployment model — the pinned ConPTY companions and the conpty.dll pre-load retired.
rationale: Retired (fct-tulramlm): the ConPTY sideload incl. `bin/<version>-<hash>/conpty/`; Symbols / APIs (fct-xxutheix): `src/main.rs`'s `restrict_dll_search` and sideload pre-load removed.
@proposal n=33 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-www7rgz5
section: §Infrastructure Patterns → crate-dependency-direction
change: fct-ggzishfk's third-party list reads «portable-pty, tracing (the `pty.spawn` span), libc (termios raw mode and the terminal size)» (drop «windows-sys (Windows only)» and libc's «Unix only»).
sidecar: Crate dependency direction — viola-pty loses windows-sys; libc a plain dependency.
rationale: Dependencies (fct-www7rgz5): «removed — `windows-sys` (… `viola-pty` …); `libc` moved from `[target.'cfg(unix)'.dependencies]` to plain `[dependencies]` in … `viola-pty`»; Symbols / APIs (fct-xxutheix): the windows-sys `TerminateProcess` kill fallback removed.
@proposal n=34 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-www7rgz5
section: §Infrastructure Patterns → crate-dependency-direction
change: fct-vavh762f's list reads «`viola-channel` → `viola-core`, interprocess, serde, serde_json, thiserror, tracing, veil, libc (the uid for the socket directory)» (drop the windows-sys entry with its SQOS / DACL roles and libc's «Unix only»), and the helpers list drops «and on Windows `client::open_handle`, the SQOS + overlapped open».
sidecar: Crate dependency direction — viola-channel loses windows-sys and the SQOS client open.
rationale: Dependencies (fct-www7rgz5): windows-sys removed from `viola-channel`, libc plain; Symbols / APIs (fct-xxutheix): the SQOS open and named-pipe arms of `client.rs` · `tokio_client.rs` removed.
@proposal n=35 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-www7rgz5
section: §Infrastructure Patterns → crate-dependency-direction
change: fct-a4ctqcf6 reads «…; libc (the `strict` module's owner + mode check), workspace-pinned.» (drop «Unix only» and the whole windows-sys clause: the owner + DACL check, the protected DACL of `fs::create_private_dir`, `GetUserProfileDirectoryW` and `Win32_UI_Shell`).
sidecar: Crate dependency direction — viola-state loses windows-sys and the Windows owner/DACL arm.
rationale: Dependencies (fct-www7rgz5): windows-sys removed from `viola-state`, libc plain; Symbols / APIs (fct-xxutheix): `strict.rs`'s Windows owner/DACL arm, `outside_profile`, the DACL arm of `create_private_dir` removed.
@proposal n=36 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Infrastructure Patterns → crate-dependency-direction
change: fct-sfkaxxdn drops `WRAPPED_HERE` from the `statusline` module's list, reads «the relied set (`relied()`: all twenty-four rows on every build)» in place of «the per-OS relied set (`relied()`, filtering on `relied_on(WRAPPED_HERE)`)», and drops «`CASE_INSENSITIVE` and `is_script` are `pub` for `verify`;».
sidecar: Crate dependency direction — viola-agent-claude's WRAPPED_HERE, CASE_INSENSITIVE and is_script retired; relied() all twenty-four rows.
rationale: Symbols / APIs (fct-xxutheix): `is_script`, `CASE_INSENSITIVE`, `WRAPPED_HERE` removed, «`LedgerRow::relied()` is all twenty-four rows on every build»; Counts (fct-tjt6dmfu): relied set «Windows twenty-one → none».
@proposal n=37 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-www7rgz5
section: §Infrastructure Patterns → crate-dependency-direction
change: fct-p5s5gpab reads «The `viola` root bin → all members, clap, anyhow, tempfile (…), libc `=0.2.189` (`backtrace` + `dladdr` for the raw, never-symbolised panic frames, and `sigaction` for the same module's `SIGINT` / `SIGTERM` handler, `SA_RESTART`); …»: drop the whole windows-sys clause (the registry reader, `RtlCaptureStackBackTrace`, `SetConsoleCtrlHandler`, the VT enable) and libc's «Unix only».
sidecar: Crate dependency direction — the root bin loses windows-sys (registry reader, Windows panic frames, console-control stop, VT enable); libc plain.
rationale: Dependencies (fct-www7rgz5): windows-sys removed from the root, libc plain; Symbols / APIs (fct-xxutheix): the registry reader, `stop.rs`'s console-control arm, `style.rs`'s VT enable and the OS arms of `src/panic_frames.rs` removed.
@proposal n=38 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-yq5vc7zj
section: §Infrastructure Patterns → project-directory-structure
change: in the tree fct-irhgihut: `main.rs` drops «(Windows: the System32 DLL-search restriction is its second statement)»; the `conpty.rs` line goes; `style.rs` drops «the Windows VT enable and its restoring guard»; `verbs.rs` drops «: a Windows-gated source, in `windows-mutants.yml`'s `viola-main` job»; `verify/stop.rs` reads «(SIGINT / SIGTERM)»; `frozen.rs` reads «`kill -STOP` polled …, `kill -CONT` on drop» without the Windows debugger attach; `viola-pty/` drops «+ windows-sys kill fallback», «windows-sys Console /», reads «host_stdin(): `std::io::stdin()`» and loses the `sideload` line; `viola-channel/` drops `PEER_EUID_REPORTED`; `viola-state/` drops `windows_read_verdict`, `READ_RIGHTS`; `viola-agent-claude/` reads «program resolution» for «shim resolution»; `scripts/agent-run.sh # the shim over viola-harness` for `agent-run.{sh,ps1}`; the `conpty-vendor.sh` and `vendor/conpty/<version>/x64/` lines go; `ci.yml` reads «test/perf/lint/release on `ubuntu-latest`» for «3-OS»; the `windows-mutants.yml` entry goes.
sidecar: Project directory structure — the ConPTY, Windows-mutants, ps1-shim and Windows/macOS arms left the tree.
rationale: Files (fct-yq5vc7zj): `src/conpty.rs`, `crates/viola-pty/src/sideload.rs`, `scripts/agent-run.ps1`, `scripts/conpty-vendor.sh`, `.github/workflows/windows-mutants.yml`, `vendor/conpty/` deleted; Symbols / APIs (fct-xxutheix) list the removed arms; Harness / gate surface (fct-sy4jxsfp): matrix `[ubuntu-latest]`, `windows-mutants.yml` deleted.
@proposal n=39 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §Infrastructure Patterns → ci-cd-approach
change: fct-vf6cfvel reads «GitHub Actions, two workflows. `ci.yml`, triggered on push and pull request, with matrix `os: [ubuntu-latest]`, is the only push/PR pipeline.» then the `nightly.yml` text as written; every `windows-mutants.yml` sentence (ruling C2, the 2026-10-10 dispatch, the `mutants (<label>)` job, `tests/contract_windows_mutation_scope.rs`, the pwsh step, `host_excluded`, run 38036448183) is replaced by «`windows-mutants.yml`, the boundary audit's Windows leg, was deleted at chunk 2026-11-05-linux-only-code-and-ci (recoverable at the tag `multi-os-final`).»; the concurrency text stays, its parenthetical without «and `windows-mutants.yml` uploads nothing».
sidecar: CI/CD approach — workflows 3 → 2, matrix → [ubuntu-latest], windows-mutants.yml retired.
rationale: Counts (fct-tjt6dmfu): «workflows 3 → 2 (`ci.yml`, `nightly.yml`) and the CI matrix 3 OS legs → `[ubuntu-latest]`»; Retired (fct-tulramlm): «`windows-mutants.yml` (the boundary audit's Windows leg)»; Files (fct-yq5vc7zj): `tests/contract_windows_mutation_scope.rs` deleted.
@proposal n=40 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §Infrastructure Patterns → ci-cd-approach
change: fct-3nc6guvb: «Jobs wired today in `ci.yml` (7; ci#42030025050 on `d3690534`: test, lint, msrv, perf, release, fuzz-replay green, supply-chain red on RUSTSEC-2026-0441)» for the three-OS check-run count; `test` (on `ubuntu-latest`) loses the `ConPTY vendor verification` step clause, runs `run --coverage` «through the `sh` shim (`scripts/agent-run.sh`)», its per-OS floors re-worded to the one `ubuntu-latest` floor (its figure as the harness holds it; the report states none); drop «whose Windows leg is the dispatch-only `windows-mutants.yml` … through the pwsh shim»; `lint` (on `ubuntu-latest`) runs `scripts/lint-probes.sh` (drop «on Linux») and `rmcp features` (drop «on every OS»); `perf` / `release` read «on `ubuntu-latest`» for «per OS».
sidecar: CI/CD approach — the ConPTY vendor verification step, the pwsh shim steps, the per-OS legs and the Windows mutation leg retired from the ci.yml job list.
rationale: Harness / gate surface (fct-sy4jxsfp): «`test`, `perf`, `lint`, `release` keep `strategy.matrix.os: [ubuntu-latest]` …; the `ConPTY vendor verification` step and both `pwsh shim` steps leave; the `sh shim` steps and `Lint probes` lose their `if: runner.os` conditions»; Cross-project (fct-swb6jipj): ci#42030025050's job verdicts.
@proposal n=41 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §Infrastructure Patterns → ci-cd-approach
change: fct-jykzrth4 reads «Target jobs, on `ubuntu-latest`, …» (was «per OS») and job 3 «— proves the sync crates, including the `hook` path, compile with no tokio» (drop «on each OS»).
sidecar: CI/CD approach — target jobs run on the one ubuntu-latest leg.
rationale: Harness / gate surface (fct-sy4jxsfp): every job keeps `strategy.matrix.os: [ubuntu-latest]`; Retired (fct-tulramlm): the `windows-2025` and `macos-latest` CI legs.
@proposal n=42 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Infrastructure Patterns → ci-cd-approach
change: fct-oqzfmod4 reads «… and the relied ledger rows (`LedgerRow::relied()`: all twenty-four on every build; `tests/cli_verify.rs`, harness `boot` step 4) …» (drop «on the Unix runners, twenty-one on `windows-2025`, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`»).
sidecar: CI/CD approach — the Windows relied set of twenty-one retired; relied() all twenty-four on every build.
rationale: Symbols / APIs (fct-xxutheix): «`LedgerRow::relied()` is all twenty-four rows on every build»; Retired (fct-tulramlm): «the Windows relied set of twenty-one»; Outcome fct-2n5dws2q: `[NN/24]` on Linux.
@proposal n=43 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Infrastructure Patterns → ci-cd-approach
change: fct-ni3gbuvj drops the sentence «The other OSes are CI's alone.».
sidecar: CI/CD approach — no other-OS CI legs remain behind the Linux pre-push.
rationale: Retired (fct-tulramlm): the `windows-2025` and `macos-latest` CI legs; Counts (fct-tjt6dmfu): the CI matrix → `[ubuntu-latest]`.
@proposal n=44 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §Cross-cutting Patterns
change: fct-fa2klbw5's heading reads «**Process and path discipline.**» (was «Cross-platform discipline»); its shell bullet reads «Nothing shells out through `sh` or `bash`, except …» (drop `cmd`); its last bullet reads «Every OS-gated test names its reason in the census binary `tests/contract_os_gated_tests.rs`, and a gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`)» in place of «Every OS-specific branch compiles and is tested on its CI runner, and …».
sidecar: §Cross-cutting Patterns — cross-platform discipline re-worded for Linux only; the census faults any gate naming another OS.
rationale: Harness / gate surface (fct-sy4jxsfp): «`GATES` names exactly the remaining gates …; a gate whose predicate names `windows` or `target_os = "macos"` reads as a fault (`another OS: …`) even when a row names it»; Counts (fct-tjt6dmfu): the os-target class «cross-platform» stale.
@proposal n=45 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Cross-cutting Patterns
change: fct-ao5mmzh3 reads «Each endpoint, a Unix socket file, must not be connectable by another OS user. The security specialist owns how this is enforced and any per-connection proof beyond it.» (drop «whether a named pipe or» and «per OS»).
sidecar: §Cross-cutting Patterns local endpoint trust boundary — the named-pipe endpoint retired.
rationale: Retired (fct-tulramlm): the Windows named-pipe endpoint and its SDDL / SQOS controls.
@proposal n=46 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Project Intent
change: fct-iigxhka7's first part reads «a native Linux CLI binary;» (was «a native cross-platform CLI binary;»).
sidecar: §Project Intent — the product's CLI binary is Linux-only.
rationale: Retired (fct-tulramlm): the OS target → «Linux only»; Counts (fct-tjt6dmfu): «cross-platform» in the os-target class stale.
@proposal n=47 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Project Intent
change: Scale path fct-qndsom6a's v1 bullet reads «v1 is personal and Linux only (first live on the Linux dev host, by the founder's ruling R-L1; CI-tested on `ubuntu-latest` against the fake agent; the Windows and macOS code removed, recoverable at the tag `multi-os-final`).» (drop «Windows, macOS and Linux CI-tested against the fake agent; Windows live awaits an interactive Windows host»).
sidecar: §Project Intent Scale path — v1 is Linux only; the Windows live wait retired.
rationale: Retired (fct-tulramlm): the OS target; Expected amendments (fct-pdxuthnr) name «Scale path» among architecture's sites.
@proposal n=48 detector=D-retired severity=warning dependent_of=D-retired basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Inherited Defaults
change: fct-f53f3qjo reads «CI: GitHub Actions on `ubuntu-latest` against the fake agent.» (was «on windows-2025 / macos-latest / ubuntu-latest»).
sidecar: §Inherited Defaults — the CI row is the one ubuntu-latest leg.
rationale: Retired (fct-tulramlm): «the `windows-2025` and `macos-latest` CI legs»; Expected amendments (fct-pdxuthnr) name «§Inherited Defaults CI row».
@remark
D-arch-resources and D-arch-decisions find no drift (the chunk adds no resource, crate or library; libc only moved from a cfg(unix) table to plain dependencies). Left to the cascade because they sit in standing detail groups: occupied-resources fct-jv5gqhbl (within-waits 32/24 → 31/23), the crate-dependency-direction key's detail group fct-ahjpxago (the root windows-sys dev-dependency for the deleted tests/security_negatives_channel.rs), and the established-decisions / stack / conventions / standard-contracts detail sites. Not proposed, for the orchestrator to judge: conventions fct-nhsg4k7g says «nine thiserror derives» where the report measured 10 at HEAD and after (fct-pws3g656, a drift older than this chunk); the v1.x winget/Homebrew/Scoop channels in Scale path fct-qndsom6a and [Deferred] fct-jqic6r22; and the Windows-only rationales kept on standing items (fct-73drmylz lock files, fct-5q3bjmyd target/harness/, fct-4u2vf5lk target/viola-answers-*).
