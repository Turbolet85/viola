@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (CI setup and wired jobs) set CI to `rustup toolchain install` from `rust-toolchain.toml`, the SHA-pinned checkout / rust-cache / install-action / upload-artifact, `permissions: {}` at the top with `contents: read` per job, and the 3-OS `test` job; the msrv and fuzz toolchains install their own. This is the least-privilege and pin baseline every surviving Linux job must keep while the `windows-2025` / `macos-latest` legs leave.

@history_item n=2 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (test-only crate, bins, env vars, paths) registered `viola-e2e`, `viola-harness` and the `scripts/agent-run.{sh,ps1}` pair in the tree. This is where the pwsh twin enters the masters, so whether `scripts/agent-run.ps1` leaves as a Windows-only harness step also touches this registration.

@history_item n=3 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — made the tokio ban the sole-root `deny-sync.toml` loop over `scripts/sync-crates.txt` (an excluded-graph check was falsified under feature unification), gave `deny.toml` its four families, and added `nightly.yml`. The chunk must keep this loop and every deny family green on Linux after it removes the Windows deny entries; whether the deny target triples narrow is the plan's fork.

@history_item n=4 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — wired the `test` job's obs-plan §9 gate order with scan-gated per-OS uploads, and the `lint` job's G1/G3 and lint probes on Linux. The `diag-<os>` / `junit-<os>` / `secret-scan-<os>` uploads and the gate order are what the shrunk ubuntu-only `test` job must keep intact.

@history_item n=5 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — added the `msrv` job (rustup 1.96 selected with `RUSTUP_TOOLCHAIN`, which outranks the pin), `fuzz-replay` and the fuzz workspace outside the root. The chunk owns the red `msrv` job (the e2e-home backing cases), so this is the job's mechanism, and it must stay green without the pin being touched.

@history_item n=6 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — made `scripts/orphans-check.sh` (+ `--probe`) a CI gate in `lint`, added the `release` job, and fixed CI at 8 jobs / 15 check-runs per push. The orphans probe's `cfg(windows)`/`cfg(unix)` file-pair control and the check-run count both move or must be re-read when the Windows/macOS legs leave.

@history_item n=7 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** — recorded the Windows SQOS client open (windows-sys `CreateFileW` adopted by `Stream::try_from`), the test-only pipe `viola-test-sqos-<pid>-<label>`, and windows-sys as `viola-channel`'s Windows-only dependency. These are among the Windows arms, the test pipe and the crate edge the chunk deletes, and their rows are expected amendments.

@history_item n=8 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — landed the R8 strip with the Windows registry reader beside the Unix `claude_env_keep`, viola-side program resolution with the `batch-script-child` exit-1 refusal, `viola-pty`'s deps (portable-pty, windows-sys on Windows, libc on Unix), the root bin's windows-sys registry edge, and `PtyError` as the one ratified hand-written exception. The registry reader and the windows-sys edges leave; the `.cmd`/`.bat` refusal is the closed word P4 decides; `PtyError`'s exception stays.

@history_item n=9 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — moved the Unix endpoint to `<socket dir>/viola-<h12>.sock` in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/` on macOS, a uid fallback), chmod 0600 after the bind, with a `.lock` arbiter; the server DACL came from windows-sys and libc served the socket dir. The macOS `$TMPDIR` arm and the DACL go, and the Linux socket rules must come out unchanged.

@history_item n=10 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — added `viola-channel`'s test-only `test-support` feature and registered the host mutation scratch `<repo parent>/viola-mutants-scratch/`. The feature's Windows helpers (`canonical_sddl`, `dacl_of`, `user_sid`) and the Windows scratch arm are among what the chunk removes.

@history_item n=11 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — added [Snapshot writer]'s Windows retry: a `persist` refused with raw OS error 5 is retried every 10 ms, at most `REPLACE_ATTEMPTS` = 100. This is the Windows decision the plan must remove or keep inert, while `replace_private` on Linux keeps its no-retry behaviour.

@history_item n=12 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (perf arm registry, G2 script, the `perf` CI job) created the per-OS `perf` job with its scan-gated `perf-<os>` / `diag-perf-<os>` uploads and `gate --require perf`, at 9 jobs / 18 check-runs. Its Windows and macOS legs leave, and the Linux `perf` job must keep its steps and gate order.

@history_item n=13 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the version gate placed and split by crate) gave the root bin windows-sys `Win32_System_Diagnostics_Debug` + `Win32_System_LibraryLoader` and the Unix-only libc `=0.2.189` for `src/panic_frames.rs`. That windows-sys edge leaves, and the libc edge becomes unconditional with no new edge added.

@history_item n=14 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — took mutation out of CI (7 jobs / 15 check-runs) and out of pre-push. It also scoped the host mutation scratch to `run --mutants` on a Windows host and removed syn/proc-macro2. This is the baseline for the CI job set the chunk shrinks, and the origin of the Windows-only scratch arm.

@history_item n=15 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** — recorded under [PTY] the measured H2 loss (a key written right after a ConPTY resize never reaching a test child) and the rule that viola never holds or re-orders a human key. That OPEN's real-`claude` half ends under the retired owner «Windows-only live measurements», so it is one of the [PTY] lines the plan names as an expected amendment.

@history_item n=16 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (the Windows x64 child is hosted in Microsoft's sideloaded ConPTY) added `restrict_dll_search` as `main`'s second statement, the pre-load of the pinned `conpty.dll`, `pty_backend()`'s words `conpty-sideload` · `conpty` · `openpty`, and `src/conpty.rs`, `crates/viola-pty/src/sideload.rs`, `scripts/conpty-vendor.sh` and `vendor/conpty/` in the tree. These are the Windows-only files and closed words the chunk removes or lists, while `openpty` stays Linux's report.

@history_item n=17 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (the ConPTY sideload step in the start order) put `run.conpty_sideload` between the pinned copy and the strip plan, and added the `windows-2025`-only `ConPTY vendor verification` step to the `test` job. The start-order step and that CI step both leave, and every other start step keeps its place.

@history_item n=18 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (the embedded ConPTY companions on disk and in the repository) registered `bin/<version>-<hash>/conpty/`, the companions' write through `replace_private_shared`, the error-32 held-open retry, `vendor/conpty/<version>/x64/` and `target/conpty-seed/<key>/`. Every one of these Occupied Resources rows loses its subject with the removal.

@history_item n=19 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (pre-push native on the Linux host) replaced the WSL/Windows pre-push with a Linux-only gate in the working tree (`pre-push-linux-only` elsewhere, stages `tools → linux-tests`, children under a stripped HOME/PATH env, it installs nothing). This is the template for "every gate green on Linux", and its `tools` stage reads ci.yml's `test`-job pins, which must survive the matrix shrink.

@history_item n=20 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (the mutation scratch dir on the Linux host) kept the harness arm Windows-only (`HOST_SCRATCH = cfg!(windows)`), with the Linux scratch an operator `TMPDIR` environment fact. The Windows arm leaves, and the off-Windows inert behaviour (and `target/mutants`) stays.

@history_item n=21 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** — added the dispatch-only `windows-mutants.yml` (founder ruling C2) and `tests/contract_windows_mutation_scope.rs`, which keeps its file lists equal to the sources and to ci.yml, and made "no workflow has a `concurrency:` block". Both files leave together whole, and the three-workflow count returns to two.

@history_item n=22 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (fuzz targets, the chaos test home, the send layout) registered the `viola-chaos-*` home (`tests/chaos_feed_panic.rs`) as the second test-data carve-out beside `seed_conpty`. When `seed_conpty` leaves, the chaos home becomes the only carve-out, and the carve-out text must say so.

@history_item n=23 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (decisions on the six-row stamp, the strict stamps read, the 2.1.287 fixtures) gave `viola-state` libc for the Unix strict owner + mode check and windows-sys for the owner + DACL check and `create_private_dir`'s protected DACL (`GetUserProfileDirectoryW`, `Win32_UI_Shell`). The windows-sys half and its features leave, and the libc half becomes unconditional.

@history_item n=24 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (the wheel as landed: the closed non-editing list and the Windows platform fact) recorded the Windows platform fact (the inbox ConPTY swallows focus reports, and win32-input-mode turns an injected mouse report into key-down records). That fact names removed code and is [Human Takeover / Wheel] text for an expected amendment; the closed list itself stays.

@history_item n=25 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (viola's own Windows console stdin reader) made `viola_pty::host_stdin()` viola's own `ReadConsoleW` reader on Windows and `std::io::stdin()` elsewhere, adding the console input read to windows-sys's roles. The Windows arm leaves, and `host_stdin()` stays the pump's and the fake agent's seam on Linux.

@history_item n=26 marker=2026-10-07-t05-47-07-wrap
**2026-10-07-t05-47-07-wrap** — created the route entry «Windows-only live measurements» and moved H2's real-`claude` key loss and the DA1 start stall to it. This is the retired owner whose 39 master hits the wrap re-points, and the plan names them as expected amendments.

@history_item n=27 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — put the two keepers (`prepare_home_base`, `Workspace::ensure_e2e_home`) under one contract, with the dev host's `target/e2e-home` a link to a tmpfs dir and no CI runner having a link ("On Windows the keepers are the plain create"). The chunk's msrv fix rests on this contract, and the Windows plain-create arm leaves.

@history_item n=28 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — (first live on the Linux dev host) reworded §Design Philosophy to "Windows is the first target" and the Scale path to Windows/macOS/Linux CI-tested, with Windows live awaiting an interactive host. These are the exact truths the wrap amends to the Linux-only OS target and the plan names as expected amendments.

@history_item n=29 marker=2026-10-09-inner-cr-and-crlf-in-a-sent-text
**2026-10-09-inner-cr-and-crlf-in-a-sent-text** — recorded "an LF inside a paste on Windows" among the unmeasured shapes under [Delivery Confirmation]. This is a per-OS line owed to the retired owner, which the plan lists as an expected amendment while Linux delivery stays unchanged.

@history_item n=30 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — split `windows-mutants.yml` into nine labelled items, moved host-exclusion classification into `harness::run::mutants::host` (the `#[cfg]` reader), and revived syn `=2.0.119` / proc-macro2 `=1.0.107` as `viola-e2e`-only dependencies. When the workflow leaves, the plan must decide whether the host-exclusion reader and its syn/proc-macro2 edges stay or go.

@history_item n=31 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (root wait count 26 sites in 19 files, the kill path's endpoint wait, ten fake-agent options) made `Wrapper::kill` wait on `holder_gone`, with a Unix arm (connect refused or path absent) and a Windows arm (pipe not found). The Windows arm leaves and the Unix arm loses its gate. This entry also measured that a file-absent wait could never end on Unix.

@history_item n=32 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (the statusline pass-through as landed) made `hook statusline` run through `sh -c` with "none on Windows", and wrote the settings override Unix-only. The Windows override and shell-out were cut on the founder's word and owed to «Windows-only live measurements». The Unix gates become unconditional with no Linux change, and the owed Windows half is an expected amendment.

@history_item n=33 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (four crossings ratified by the founder) ratified the settings override and the `sh -c` shell-out as viola's only shell-out. This bounds the chunk's anti-pattern: collapsing the cfg arms adds no other shell-out.

@history_item n=34 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (the mutation scratch re-created owner-only) re-created the wiped scratch through `create_private_dir`, so on Windows it takes the protected DACL (witnessed only on the `windows-2025` test leg). The case and its Windows rationale leave with the mutation arm.

@history_item n=35 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — moved the replace loop into the private `replace_private_retrying` (decision passed as `retry_replace(_, HOST_IS_WINDOWS)` so an every-host unit case reaches the bound). It also registered `target/viola-answers-*/`, which exists because a temp dir on another Windows drive has no relative path. The plan's fork on whether the retry loop and its unit case stay starts here, and the answers dir's Windows rationale loses its subject.

@history_item n=36 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added `src/cmd/verify/stop.rs` with Unix `SIGINT`/`SIGTERM` (libc `sigaction`) and Windows `CTRL_C_EVENT`/`CTRL_BREAK_EVENT` (windows-sys `SetConsoleCtrlHandler`), and grew `windows-mutants.yml` to ten items. Its standing rule is that "a later chunk adding a `cfg(windows)` product file owes that workflow its line". The Windows console arm and the matrix line leave, and the libc arm becomes unconditional.

@history_item n=37 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — (three statusline ledger rows and the per-OS relied set) introduced `LedgerRow::relied()` filtered by `relied_on(row, statusline::WRAPPED_HERE)`, with the statusline rows relied on Unix only, Windows legs owed. This is the per-OS mechanism whose Windows arm leaves, and Linux must keep the full relied set.

@history_item n=38 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — (Run B's probe --settings, the statusline capture arm) gave Run B one probe `--settings` on Unix only for shell-plain paths ("Windows passes none"). That gate becomes unconditional and verify's Linux probe must stay as built.

@history_item n=39 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (the twenty-fourth ledger row, local-command-compact) set the relied set at 24 on Unix and 21 on Windows, with step lines `[NN/24]` / `[NN/21]` and the Windows reading owed. This is the current count: the Unix 24 must stand after the Windows arm and its count leave.

@history_item n=40 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — (0.1.0 is a pure CLI; the frontend moves to 0.2.0) kept the masters true to the tree while a removal was still owed, marking the browser rows "owned by" the removal entry rather than editing ahead of the code. It is the precedent for this chunk naming its OS-target changes as expected amendments for the wrap instead of editing the masters itself.

@history_item n=41 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (the Browser e2e row and the leg's occupied resources leave) is the precedent for a whole-leg removal. Its Stack row and every install site and output left the registry in the wrap, and the pre-push `tools` stage was narrowed to `cc`, the channel and ci.yml's `test`-job pins. It also kept the npm-shim → `claude.exe` resolution text, which this chunk now lists for P4.

@history_item n=42 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (Infrastructure Patterns: the browser pipe, the ts plane and the axum ban leave) recorded `nightly.yml` as two ubuntu jobs (`advisories`, `fuzz`), set the `test` job's `gate --require coverage,doctest`, and removed a `deny.toml` ban and its probe with the leg (the probe count moving with it). This is the precedent for removing deny entries and probe rows in the same change, and the source of "nightly holds no Windows/macOS job".

@history_item n=43 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (Standard Contracts: the agents read, the join and the unwrapped rack land) made `list`, `revive`'s preflight and verify's Run B read `claude` through the one `run::resolve_program`. Whatever P4 decides for the npm-shim and `.cmd`/`.bat` arms, the three callers stay on that one resolver.

@history_item n=44 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — gave the root bin's `Win32_System_Console` `GetConsoleMode`/`SetConsoleMode` for `src/human/style.rs`'s stdout VT enable, with no dependency added. This is one of the root-bin windows-sys uses the scope names (`src/human/style.rs`) that leave with their cfg arm, while the `NO_COLOR` / `TERM` / `COLORTERM` reads stay.

@history_item n=45 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (the CLI verbs take the VIOLA_DIR step and canonicalise the home) defined `canonical_home` with its Windows verbatim-path rewrite (`\\?\C:\…` written back as `C:\…`). That arm leaves while every other spelling rule and the ratified `VIOLA_DIR` step stay.

@history_item n=46 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the Tokio client and viola-channel's tree-form ban) cut the sync list to four crates, proved `viola-channel`'s no-feature graph tokio-free with `scripts/channel-tokio-free.sh` in supply-chain, set `deny-probes.sh`'s `channel-tokio-edge`, and had `lint` run `rmcp features` on every OS. The Tokio confinement and its two enforcers must keep running on the Linux CI once the per-OS `cargo check` of the sync crates shrinks.

@history_item n=47 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — (plugin install's exits, document, names and files) put a Windows-gated source in `src/human/verbs.rs` (`command_word`), listed in `windows-mutants.yml`'s `viola-main` job. It is one more `cfg(windows)` arm whose workflow line leaves with the workflow.

@history_item n=48 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — made the dialog hold `config.json`'s `dialog_hold_s` and owed its long hold's Windows leg to «Windows-only live measurements»; its order fix answered a macOS CI red. The Windows leg is a [Hook Contract] line for an expected amendment, and the Linux hold stays.

@history_item n=49 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the keepers' headroom floor) added the 9 GiB `HEADROOM_FLOOR_MIB` refusal by `statvfs`, one compiled constant per keeper set by no variable, flag or config key, with the fixed message `e2e-home backing: {free} MiB free, below the {floor} MiB floor`; a plain-directory base reads no headroom. This is the floor whose two backing cases fail on the msrv runner, so the fix must give them a test-controlled figure without making the floor configurable.

@history_item n=50 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (the split modules, `relied_here` and the `viola-mcp` serde edge retired) folded `relied_here` into `relied()` over `relied_on(WRAPPED_HERE)`. That is the one site where the per-OS relied set lives, and where the Windows arm leaves.

@history_item n=51 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (the test-only child marker and the orphans probe's two cases) recorded `scripts/orphans-check.sh --probe` firing on two planted orphans (`2/2 fired`), and named the unregistered test marker `PTY_SIDELOAD_TEST_CASE`. The probe's Windows/Unix file-pair control must still fire with CI on Linux alone, and the sideload's test marker leaves with `sideload.rs`.

@history_item n=52 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** — added the root's `[target.'cfg(windows)'.dev-dependencies]` windows-sys (`Wdk_Storage_FileSystem`, `Win32_System_IO`) for `tests/security_negatives_channel.rs`'s pipe-type read-back. It leaves with that Windows test. Trap: a windows-sys feature can compile through unification, so check the lock and deny after the removal.

@history_item n=53 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (both clients read the serving process's pid; the macOS recorded gap) made `server_pid` read `peer_creds().pid()` (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId`, none on macOS where `SERVER_PID_REPORTED` is false) and `server_verdict` return `Matches` on macOS. The Windows arm and the macOS carve-out lose their code, and the Linux pid and start-time check stays.

@history_item n=54 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the snapshot's pid and started_at as the server identity) wrote the snapshot contract's "on Linux and Windows … (macOS reports no peer pid, the recorded gap)" and the pure `server_verdict(…, peer_pid_reported)`. These per-OS words are expected amendments, and the `peer_pid_reported` parameter's macOS-only false arm is the carve-out P4 decides.

@history_item n=55 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the ConPTY companions' owner/DACL check) added `pin_companions`' `check_present` on `conpty/` with the `strict-modes-failed` fallback word in `sideload_fallback`'s schema enum. It leaves with the sideload, and the schema's OS word is text the plan lists.

@history_item n=56 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the read rule and check_instance's widened set; the Windows instance-tree cases built) defined the read rule's Unix `mode & 0o077` half and Windows `READ_RIGHTS` half (`windows_read_verdict`), the `events.ndjson` write-rule-on-Unix / read-rule-on-Windows split, and the `cfg(windows)` proving cases. The Windows halves and cases leave, and the Unix halves become the only rule, ungated.

@history_item n=57 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (three Windows readings for tests) recorded the per-OS oracle for a path under a regular file, the `icacls` grant reaching through `seed_conpty`'s hard links, and the protected DACL reading `LA`. All three rows lose their subject, and a test that planted a regular file keeps only its Unix oracle.

@history_item n=58 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree check before the start's and verify's log) wrote `check_home_tree` checking the pinned `viola` (`viola.exe` on Windows). The Windows name is text that names removed code, and the Unix write-rule check over the same entries stays.

@history_item n=59 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — ([Snapshot writer]: the pinned copy's no-clobber placement) added `place_private_new` with the forced-window parameter, witnessed by `place_private_new_placed_copy_execs_at_once` on Unix and `…_runs_at_once` on Windows (no env, no feature). The Windows case leaves and the Unix case stays the witness, still with no seam.

@history_item n=60 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — (the Windows plain-home refusal read by its inherited untrusted write grant) recorded the runner-dependent ACL and `strict::win::tests::inherited_untrusted_writes`. These are Windows-only test rows the removal deletes.

@history_item n=61 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (bounded readers, a cross-cutting pattern held by a census) made `tests/contract_bounded_readers.rs` hold every product reader bounded or named (`BOUNDARIES`, `EXCEPTIONS`, a stands-check). The ConPTY companions' re-hash reads leave, so their rows must leave in the same change or the stands-check goes red.

@history_item n=62 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — ([PTY] OPEN: the Windows reading of the wide character after a feed panic) made `tests/chaos_feed_panic.rs` assert bytes on Unix and only the absent literal on Windows, owed to «Windows-only live measurements». The Windows arm leaves, and the OPEN is an expected amendment.

@history_item n=63 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the serde_json unbounded_depth ban and the deny probe count) set `deny-probes: 14/14 banned, control clean`, the count's rule being one per `probes=(` row plus `channel-tokio-edge`. If a Windows deny entry carries a probe row, the count moves with its rule.

@history_item n=64 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the enum census) folded `Refusal` into `AgentError` (`BatchScriptChild`, `ProgramNotFound`) and fixed the census at 9 `thiserror::Error` derives by `command grep -rn "thiserror::Error" src crates --include=*.rs`. The count is re-measured by that grep after the removed files go, and `BatchScriptChild` is the `.cmd`/`.bat` refusal P4 decides.

@history_item n=65 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the sanitised error surfaces pattern and its contract binary) made `tests/contract_error_bodies.rs` check each error body's writer and witness and census the product's error writers. Any row whose writer lived only in a Windows arm must leave with it, and Linux's closed codes stay rows.

@history_item n=66 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (`v1-47`'s reading over the homes CI keeps) set `AGENT_RUN_KEEP_HOMES` `"1"` and `if: always()` on the end-to-end job in `.github/workflows/ci.yml`. That job step stays with the surviving Linux job.

@history_item n=67 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`viola revive --json` documents every refusal of its start) made `batch-script-child` one of the closed exit-1 codes `refused_with` prints. Retiring or keeping the `.cmd`/`.bat` refusal touches this closed set, and no Linux code may move.

@history_item n=68 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the socket-directory and macOS carve-outs) claimed `server-verify-failed` off macOS only, as `v1-18`'s named carve-out. A later entry widened it to macOS, and this carve-out text now names code the chunk removes.

@history_item n=69 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the code-word rule and the exit-cause table) added `tests/cross_exit_causes.rs` and the rule that every exit-21 cause and exit-1 code is an `is_code_word` row of `tests/contract_error_bodies.rs`. These are the witnesses that the typed exit causes come out of the removal unchanged.

@history_item n=70 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the peer euid on both sides) exported `PEER_EUID_REPORTED` as `cfg!(unix)` and `peer_euid_admitted(observed, own, reported)`, and injected `peer_admitted` into `accept_loop` through the private `serve_admitting` "with no env, flag or feature seam"; Windows reads no euid. Making the gate unconditional must keep that no-seam rule and refuse an unreadable peer.

@history_item n=71 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (macOS's server check, an euid mismatch alone) claimed `server-verify-failed` on Linux, Windows and macOS (on macOS an euid mismatch alone), with the no-peer-pid gap owned by no route entry. That carve-out is the macOS code that leaves, and the conventions text is listed.

@history_item n=72 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory check's read and its home in the code) defined `check_socket_dir` (one `symlink_metadata`, never chmods) with "off Unix it returns `Ok(())`", and kept the check out of `viola_channel::Client::connect`. The off-Unix arm leaves and the Unix half loses its gate with its Linux verdicts unchanged.

@history_item n=73 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a force-killed wrapper's socket file) made harness `cleanup`'s `endpoint_gone` read through `unconnectable` on Unix. Its Windows sibling leaves, and the Unix reading loses its gate with `ok:true` kept.

@history_item n=74 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the test-owned socket roots) registered `SOCKET_ROOT_VAR`, `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere, for `tests/security_negatives_socket_dir.rs`. The macOS arm leaves and `XDG_RUNTIME_DIR` becomes the one variable.

@history_item n=75 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the OS-gated tests census) landed `tests/contract_os_gated_tests.rs` (`GATES`, a census over `windows` / `unix` / `target_os` predicates that fails both ways, the uniqueness check, the eight-case self-check). It also rewrote the cross-platform discipline line to name it. This is the census the chunk re-points or retires in the same change as the gates, and that body line is an expected amendment.

@history_item n=76 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the root waits on the WITHIN bound) set `Instant::now() + WITHIN` at 32 sites in 24 files by its grep rule, and named Unix `Frozen` as a user. Removing Windows/macOS tests or helpers that use the bound moves this count, which is re-measured by its rule.

@history_item n=77 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the shared Frozen guard) put `tests/support/frozen.rs` `Frozen` as "one twin per OS under its own gate" (Unix `kill -STOP`/`ps`, Windows debugger attach). The Windows twin leaves and the Unix twin becomes the guard's one ungated home, with no Windows-gated file left for orphans-check.

@history_item n=78 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the per-OS JUnit script) added `scripts/junit-os-sets.py`, which downloads `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest`. Those artifacts stop existing once the jobs leave, so the plan must re-point or retire the script.
