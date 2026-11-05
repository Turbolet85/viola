@history_item n=1 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — (per-job gate, coverage JUnit and regex) made the coverage ignore regex separator-agnostic (`crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`, harness `COVERAGE_IGNORE`), measured on Windows and recorded as "not a widening"; it also set `fuzz-linux-only` for `--fuzz-replay` off Linux. Why it matters: the Windows separator class is the precedent the anti-pattern rests on, so it stays as it is. The Linux coverage gate must hold 85/95/80 after the `cfg(windows)` arms leave, and the regex may not be widened.

@history_item n=2 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — retired the unscanned `agent-run-<os>` upload and added the scan-gated per-OS uploads `harness-`, `diag-`, `junit-<os>` and `secret-scan-<os>`. Why it matters: these `${{ matrix.os }}`-named uploads are the obs binding the chunk must leave whole on ubuntu when the matrix shrinks.

@history_item n=3 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — added the `release` job (`scripts/release-check.sh --probe`, then the default mode) and per-target `scripts/orphans-check.sh` as plain exit-code jobs, with their fixed last lines. Why it matters: the release and lint jobs lose their Windows and macOS legs, but each ubuntu leg must still end on `release-check: viola only` and `orphans-check: N/N targets clean` after the manifests drop their Windows dependencies.

@history_item n=4 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — set the 11-name identity floor as measured on the Windows host. It also measured the no-EOF case per OS (output outlives the leader natively on ConPTY, on Linux only for a holder in its own process group, never on macOS), so the held-output half is asserted on Windows + Linux. It gave `--exit-no-eof` its own process group on Unix and landed the `.cmd` child case in `tests/cli_program_resolution.rs`. Why it matters: these are the origins of the R8 strip's Windows wording, the chaos suite's per-OS no-EOF legs, the holder's `cfg(unix)` process group (to be made unconditional) and the `.cmd`/`.bat` fork that P4 decides.

@history_item n=5 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — made `cleanup` report `endpoint_gone` (Windows: connect NotFound; Unix: socket path absent). Under §5 it set the pipe DACL from the GA SDDL with a canonical-FA read-back, Unix 0600 by chmod after the bind, and the per-OS oversize close: 0 bytes on Windows, a possible reset on Linux, EPIPE on macOS. Why it matters: the Windows DACL read-back and the Windows/macOS close readings leave with their arms, while the Unix socket mode read-back and the Linux reset reading stay.

@history_item n=6 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — introduced the Windows host mutation scratch `<repo parent>/viola-mutants-scratch` (`scratch-refused`, `scratch-wipe-failed`, `scratch_bytes`) and set the release-check probe to `5/5 refused, control clean`. Why it matters: the Windows-host scratch arm (and `HOST_SCRATCH = cfg!(windows)`) is code the chunk removes, while the release job's probe and verdict lines must stand on ubuntu.

@history_item n=7 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (`run --perf` built, the per-OS `perf` job) gave perf its own `perf (${{ matrix.os }})` job with G2 and scan-gated uploads. Why it matters: the chunk drops the job's Windows and macOS legs, and the ubuntu leg must keep `gate --require perf`, G2 and the `perf-`/`diag-perf-` uploads.

@history_item n=8 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — compiled the two real-cargo-mutants harness self-tests out on macOS (`#[cfg(not(target_os = "macos"))]`, never `#[ignore]`) after measuring a ~77 s baseline build in the copied tree, and added the ENOTCONN macOS reading to the oversize-frame test. Why it matters: these are `target_os = "macos"` gates the chunk removes, so both tests then run on Linux unconditionally, and the §10 text naming the carve-out becomes an expected amendment.

@history_item n=9 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** — (Module ↔ PTY) recorded the ConPTY key-after-resize loss (13 of 200 on windows-2025) and its limit, and made the red test wait for the child's reported size. Why it matters: this is ConPTY-only measurement text in §5. Its owner moved to the retired "Windows-only live measurements" entry, one of the hits the masters' CARRY re-points.

@history_item n=10 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — (stop waits for the endpoint) made `Wrapper::stop` / `stop_keep` wait for the endpoint to be unconnectable, citing a Windows stopped-wrapper pipe that took a hook 25 ms after exit (ci#36532038635). Why it matters: that Windows reading leaves with the pipe arm, while the Unix unconnectable rule stays.

@history_item n=11 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (the H2 pair, the sideloaded backend and its boundary tests; the Windows x64 seeded-home carve-out) added the ConPTY sideload to Critical Path 1, the `conpty_sideload` binary and `restrict_dll_search_…`, and the `windows-2025` `ConPTY vendor verification` step ahead of the coverage run. It also added the first test-data carve-out: `seed_conpty` seeds `<home>/bin/<key>/conpty/` on Windows x64. Why it matters: `tests/conpty_sideload.rs`, `vendor/conpty/`, the vendor step and `seed_conpty` all leave in this chunk. §5, §7, §9 and the start sequence lose the carve-out and the step, and §7's "second exception" count changes.

@history_item n=12 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (pre-push native on the Linux host) made `pre-push` Linux-host only and retired every Windows/WSL stage, reason and document field (`pre-push-windows-only`, `windows-tests`, `vm-release` …). Why it matters: this is the precedent of a Windows harness path retired whole, with its closed enums recorded in place of a Decisions Log entry. The pre-push the chunk keeps is already Linux-only and parses the `test` job's `tool:` line.

@history_item n=13 marker=2026-10-04-confirmed-send-with-cl-1-records
**2026-10-04-confirmed-send-with-cl-1-records** — (the second test-data carve-out: the chaos home (F4)) added `TestHome::outside_scan()` for `tests/chaos_feed_panic.rs` as the second carve-out beside `seed_conpty`. Why it matters: once `seed_conpty` leaves, the carve-out count in §5/§7 falls to this one. The chaos case's Windows byte reading is a CARRY to re-point.

@history_item n=14 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — (Security sweep) landed the Windows `--home` creation half (`create_private_dir`, unit on both OSes plus `tests/cli_version_gate.rs`), with a Windows DACL test home under `target/e2e-home`, never `%TEMP%`. Why it matters: it is Windows-arm test text the chunk's removal leaves naming gone code, and `create_private_dir`'s Unix half stays.

@history_item n=15 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** — (the Windows leg of the boundary audit) added `windows-mutants.yml` (dispatch-only, `windows-2025`, `scripts/agent-run.ps1 run --mutants --package … --file …`). Its `tool:` line is asserted equal to the `test` job's by `tests/contract_windows_mutation_scope.rs`, and the workflow count went to three (zizmor `concurrency-limits` 2 low → 3 low). Why it matters: the chunk removes the workflow, the contract test and the `.ps1` caller together. §9's Platform, Mutation row, workflow paragraph and concurrency count go back to two workflows (an expected amendment).

@history_item n=16 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — (Path 5 as landed) wrote the `windows-2025` focus-case platform fact (founder ruling F-W3) beside Unix's full assertion. Why it matters: the Windows arm of the focus case leaves, and the Unix full assertion becomes its only, ungated assertion.

@history_item n=17 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — added `Workspace::ensure_e2e_home` / `prepare_home_base`. A linked base needs an absolute target, made 0700 when gone and accepted only on one `lstat` of a real owner-only directory; Windows is the plain create. The dev host's base is a tmpfs link. Why it matters: the msrv headroom fix lives in this keeper. Its Windows plain-create arm leaves, and every home must stay under the `target/e2e-home` path (G2, G4 and the secret scan).

@history_item n=18 marker=2026-10-07-t05-47-07-wrap
**2026-10-07-t05-47-07-wrap** — re-pointed H2's real-`claude` key-after-resize question to the route entry "Windows-only live measurements". Why it matters: that entry is now retired, and this is one of the master hits the scope's masters CARRY re-points through the wrap's amendments, which the chunk's plan names as expected.

@history_item n=19 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (the floor's Linux reading) placed a differing Linux dev-host reading on 2.1.287 (12 names, four outside the eleven, three absent) beside the floor's Windows origin, and kept the floor and the literal oracle at the eleven. Why it matters: the R8 strip's case-insensitive Windows arm leaves. The eleven-name floor literal stays the oracle, never the product's `IDENTITY_FLOOR`.

@history_item n=20 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (revive in the test plan; the kill path's endpoint wait) added `strict::check_instance` with `cfg(unix)` widened-mode cases and two `cfg(unix)` root cases. It also made `Wrapper::kill` wait on `holder_gone` (Unix: connect refused or path absent; Windows: pipe not found). Why it matters: these `cfg(unix)` cases are census rows that lose their gate once their Windows siblings leave, and `holder_gone`'s Windows arm leaves.

@history_item n=21 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (Path 6 and Path 1 on Unix only) made the statusline marker equality and the `settings.json` override rewrite Unix-only: on Windows `run` writes no override and the arm runs no command. The Windows half was owed to "Windows-only live measurements". The same chunk (the stamped-home rule on Windows) recorded that a `seed_conpty` home was refused by `hook statusline`'s instance check. Why it matters: the Unix arms become unconditional with their Linux assertions word for word, and the Windows no-override branch, its `cfg!(windows)` test branch and the owner text leave.

@history_item n=22 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (the Windows scratch lay-down and the Windows-target clippy) pinned the scratch's `create_private_dir` lay-down with a `#[cfg(windows)]` unit on `test (windows-2025)`. It also let a chunk plan list a check-only Windows-target clippy (`target/wincheck`, `x86_64-pc-windows-msvc`) as a dev-host gate entry. Why it matters: both the Windows unit and the dev-host Windows-clippy recipe leave with the code they checked.

@history_item n=23 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — (`host_excluded`; nine Windows mutation jobs) taught `run --mutants` to drop a missed mutant whose span sits under a `cfg` proved false for the host into `host_excluded`. It split `windows-mutants.yml` into per-file `matrix.include` jobs, and run 38036448183 read green. Why it matters: once every `cfg(windows)` arm is gone, no Windows mutant remains to exclude or dispatch. §10's mutation-gate text about Windows dispatch and host-excluded `cfg(windows)` mutants becomes an expected amendment, while the verdict rule stays.

@history_item n=24 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added the `cfg(unix)` signal-stop case `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1`. On Windows the console routine is called directly (three cases on `windows-2025`), with real console delivery owed to "Windows-only live measurements". The Windows mutation matrix grew to ten jobs, a change forced by `tests/contract_windows_mutation_scope.rs`. Why it matters: the Windows console cases and the owed CARRY leave or are re-pointed, the `cfg(unix)` case is a census row, and the ten-job matrix line leaves with the workflow.

@history_item n=25 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — said the `rule-inner-lf` control is stamped live on Linux and owed on Windows to "Windows-only live measurements". Why it matters: another owner hit on the retired route entry, for the wrap's amendment to re-point.

@history_item n=26 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — made the relied rows per-OS (21 on Unix, 18 on Windows) with per-OS `cli_verify` literals. `LEDGER_ROWS` became `[&str; 21]` under `cfg(unix)` and the 18-id form under `cfg(not(unix))`, "keeping it out of the Windows mutation scope". Why it matters: this is the origin of the `cfg(unix)`/`cfg(windows)` literal pairs the chunk collapses to the Unix values. The stated reason for the gate (the Windows mutation scope) goes away with the contract.

@history_item n=27 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — (the GUI surfaces and their tests leave 0.1.0) cut the web surfaces on the founder's ruling, but kept the masters true to the tree: each still-standing leftover was owned by the removal route entry. Why it matters: this is the precedent for an OS or surface cut. The masters change only by the wrap's amendments matching what the removal chunk actually removed, which the plan names as expected amendments.

@history_item n=28 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (§2, §5, §7, §9–§11: no browser runner, deny probes 12/12) removed a whole test leg with its CI job, harness surface, closed enums and failure conditions. It re-counted `deny-probes` by its rule (one probe per ban) when a ban left with its dependency. Why it matters: this is the precedent for retiring a leg in one chunk. When `deny.toml` loses the Windows entries, the deny-probes line (now 14/14) moves only by its rule if a ban or probe leaves.

@history_item n=29 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (agents-join at 23 / 20 rows; the board's witnesses) landed row integrity: the Windows terminal case reads escaped names as text under a 200-column terminal, and the no-ESC assertion is Unix-only. Why it matters: the Windows arm of `cross_list_rows_print_escaped_…` leaves, and the Unix no-ESC assertion must keep holding on Linux once ungated.

@history_item n=30 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — (boot's two flags; Path 6's harness-session leg) built `boot --statusline-echo` with the command `<bin dir>/viola-fake-agent[.exe] statusline-echo …`, and landed Path 6's step-7 leg asserting "the Unix / Windows statusline branches". Why it matters: the `.exe` and Windows-branch text in the harness and in `path6_budget_harness` leaves, and the Unix branch stays.

@history_item n=31 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — added `tests/cli_machine_contract.rs` with the `resolve_home_*` / `canonical_home_*` / `plain_drive_*` units. Why it matters: `plain_drive_*` is a home-path canonicalisation case P3 should check for a Windows-only arm. Any that leave must keep the Linux resolution order unchanged.

@history_item n=32 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — pinned `viola --help` by the `HELP` literal, with `help_lists_only_served_verbs` accepting usage program `viola` or `viola.exe` and Unix-only no-lone-CR. It also recorded the bold/amber case's parsed attribute set on windows-2025. Why it matters: the `viola.exe` and windows-2025 arms leave. Any help-string change must move the `HELP` literal in the same commit.

@history_item n=33 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the CI gates: the channel tree form, rmcp features, deny-probes 13/13) added the per-OS `rmcp features` lint step (last line `rmcp-features: server transport-io`) and `scripts/channel-tokio-free.sh` (`channel-tokio-free: ok`). Why it matters: these fixed last lines must survive on the ubuntu lint and supply-chain jobs once the per-OS legs leave.

@history_item n=34 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — set relied 24 on Unix and 21 on Windows, `STEPS_PASS`/`ROW_IDS` `[&str; 24]`/`[&str; 21]`, and the older-stamp oracle with three ids that verify on Windows only. Why it matters: the Windows 21 forms, the `cfg(windows)` literals and the Windows-only verdict leave. The Linux relied set must stay the 24 rows.

@history_item n=35 marker=2026-10-19-newer-claude-cli-rows
**2026-10-19-newer-claude-cli-rows** — (the long-turn screen; the readiness stall report in `wait_ready`) added the long-turn modal case at `19 pass  5 fail` on Unix and `16 pass  5 fail` on Windows. It also added the stall report, whose macOS arm (`/usr/bin/sample`) sits beside Linux's `/proc/<pid>/wchan` and `stat`. Why it matters: the Windows literal and the macOS `sample` arm of `process_state` leave, and the Linux arm stays.

@history_item n=36 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the headroom floor) refused a linked base under `HEADROOM_FLOOR_MIB` (9 GiB) free by `statvfs`. The decision is the plain `headroom_refusal(free, floor_mib)`, unit-tested on every OS, one compiled constant set by no variable, flag or config key. The same entry put the fake agent's spawned commands in their own process group on Unix and granted the scratch control's sibling Users write via `icacls`. The companion entry (the mutants profile's paste-hint class) added libc `=0.2.189` under `cfg(unix)` to viola-e2e for the `statvfs` read. Why it matters: the msrv CARRY's two failing cases sit on this floor. The fix must use a test-controlled free figure with no new seam and keep the plain function and its constant. The `cfg(unix)` libc dev-dep and process group lose their gates, and the `icacls` control leaves.

@history_item n=37 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (`prepare` killed on every host) made `prepare` take its host flag from `scratch::HOST_SCRATCH` (a `cfg!(windows)` const) so both arms are reached on every host. Why it matters: with the Windows host scratch removed, `HOST_SCRATCH` and the Windows-scratch arm leave, and the §10 note on its mutants becomes an expected amendment.

@history_item n=38 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** — (the Windows listener's admission negatives; the control pipes and the anonymous principal as fake servers; the viola-channel untestable zone narrowed) added the five `#[cfg(windows)]` cases of `tests/security_negatives_channel.rs` with `CreateNamedPipeW` control pipes and `ImpersonateAnonymousToken` in §8. It narrowed §1's untestable zone to the other-euid peer. Why it matters: these cases, §8's control-pipe row and §1's Windows-admission clause leave together. The other-euid zone (no foreign peer, no seam) stays.

@history_item n=39 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the macOS-target check-only clippy) added the dev-host `target/maccheck` / `aarch64-apple-darwin` clippy recipe after an unused import in a macOS-gated module reddened `lint (macos-latest)`. Why it matters: the recipe leaves with the macOS lint leg, and the §9 Lint line naming both dev-host recipes is an expected amendment.

@history_item n=40 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the server check's captured-span cases compiled out on macOS; the root bin's server-check unit cases) gated `request_refuses_…`, `hook_refuses_…`, the `refused_inside` helper (`#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`) and the `last.rs`/`pause.rs` modules off macOS. The helper's two-attribute form was chosen because the Windows mutation-scope contract read `cfg!(windows)` as Windows-gated product code. Why it matters: these gates drop and every captured-span case runs on Linux. With the contract gone, the two-attribute rationale no longer binds.

@history_item n=41 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the server pid and the pure server verdict) added `Client::server_pid` per OS (Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId`, macOS none), `SERVER_PID_REPORTED` false on macOS, and `server_verdict_rows` with the no-peer-pid `Matches` row. Why it matters: these are the closed words P4 decides (`SERVER_PID_REPORTED`, the macOS row), with the Linux `SO_PEERCRED` cases kept.

@history_item n=42 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the hook fail-open impostor rows; the exit-cause matrix's server-verification cause; the server-identity security negative; the Wrapper channel's server-mismatch cases; where the server verification is asserted) put the impostor rows under `cfg_attr(not(target_os = "macos"), …)` and `tests/security_negatives_server_identity.rs` under `#![cfg(not(target_os = "macos"))]`, all "on Linux and Windows". It put the exiting-Windows-pipe question as a CARRY on "Windows-only live measurements". Why it matters: these gates are census rows that drop, leaving the Linux cases ungated. The "Linux and Windows" wording and the exiting-pipe CARRY are expected amendments to re-point.

@history_item n=43 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the server-identity impostor: its held process, its listener, its test-built snapshot; the fake agent's exit ignores SIGHUP) made the impostor's held process a real `--hold-stdout-internal` holder, never a double. It gave the fake agent's exit a Unix SIGHUP ignore. Why it matters: when the Windows arm leaves, the Linux impostor must keep the real holder and its test-built snapshot. The SIGHUP-ignoring `cfg(unix)` exit becomes unconditional with Linux behaviour unchanged.

@history_item n=44 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (eight CLI frame verbs under the server check; the link verbs' instance check) gated `unlink.rs`'s test module whole off macOS and `link.rs`'s server case alone, by the rule "a module is gated whole only when its every case is gated". It added the `cfg(unix)` `link_verbs_refuse_a_widened_instance_before_any_log`. Why it matters: these macOS gates drop, and the `cfg(unix)` link case is a census row that loses its gate.

@history_item n=45 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the seeded home made private; the seed's hard-link grant hazard) made `seed_conpty` create the home through `create_private_dir`. It measured an `icacls` `(OI)(CI)` grant reaching every seeded home through hard links, so `run_refuses_a_widened_home_tree_before_its_log` starts unseeded. Why it matters: the seed, its §7 widened-seeded-home row and the unseeded carve-out leave together. The Linux refusal case keeps its assertion.

@history_item n=46 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the security control negatives: Windows `--home` DACL, read-granted home, the ConPTY companion; a home a test makes before viola) added `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (`icacls /save`, `LA`), the ConPTY companion fallback case and the per-OS regular-file oracle (Windows `NotFound`). Why it matters: the `cfg(windows)` negatives leave. The Unix 0750 read-rule refusal and the Unix strict-modes oracle of `verify_under_a_regular_file_refuses_the_home` stay unconditional.

@history_item n=47 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-widening helpers and the Filesystem faults row; the read rule and the instance check's widened set; the tree and diagnostics checks; the six CLI frame verbs under the instance check) added `widen`/`grant_read`/`unwiden`/`still_widened` (Unix modes; Windows `icacls`) and `windows_read_verdict` with the `cfg(windows)` `read_rights_are_the_named_flags`. It added real windows-2025 grant cases, `events.ndjson` and diagnostics files on the read rule on Windows, and the exit-21 home row's Windows `icacls` cause. Why it matters: the Windows arms of the helpers, verdicts and grants leave. The helpers keep their names and Unix behaviour, and `unix_read_verdict` and the Unix rule arms stay.

@history_item n=48 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — (the pinned copy's no-clobber placement; the plain home's refused Windows entry; runner-inherited grants) added the forced-window witnesses `place_private_new_placed_copy_execs_at_once` (Unix, red-green on Linux) and its Windows form. It added the Windows plain-home oracle `strict::win::tests::inherited_untrusted_writes` and the rule to read a runner-inherited grant by its right. Why it matters: the Windows form and its oracle leave, and the Unix witness stays green on ubuntu-latest alone.

@history_item n=49 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the serde_json `unbounded_depth` ban and deny-probes 14/14; the feed-panic passthrough on the outer terminal; the bounded-reader enumeration; the bounded compare read) set `deny-probes: 14/14 banned, control clean` by its rule (the `probes=(` rows plus the channel probe). It owed the chaos case's Windows byte oracle to "Windows-only live measurements". It added `tests/contract_bounded_readers.rs` with literal `BOUNDARIES`/`EXCEPTIONS` rows that must stand. It ran the compare's longer-target refusal on Unix only (Windows using a read-only file). Why it matters: the deny count holds unless a removed ban changes it by rule, and the Windows byte CARRY is re-pointed. Any boundary row naming a removed Windows file must leave, and the Windows read-only-file arm leaves.

@history_item n=50 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the error bodies' surface witnesses; the error-body enumeration; `resolve_program`'s error after `Refusal`) kept §7's `Wrapper::boot_as_user` wording ("`HOME` (on Windows `USERPROFILE`)"). It added `tests/contract_error_bodies.rs`, whose tables name writers that must stand, and folded `BatchScriptChild`/`ProgramNotFound` into `AgentError`, held by `refusal_messages_are_fixed`. Why it matters: the `USERPROFILE` arm leaves. The error-body table rows must still stand on Linux, and the `BatchScriptChild` fork moves its pinned messages if decided.

@history_item n=51 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a stale holder reads `stale-heartbeat`) made `Frozen` a guard on both Windows and Unix so stale cases read on Linux and macOS. Why it matters: its Windows debugger-attach arm and `run_refuses_a_frozen_wrapper_as_stale_on_windows` leave, and the Unix `SIGSTOP` guard stays the real-wrapper stale holder.

@history_item n=52 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`viola revive --json`'s ten codes; the exit-cause table is built; `viola verify`'s refusal codes) added `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document` as `v1-39`'s Windows witness. It named the `.cmd` human pair in `cross_exit_causes.rs`'s top comment and made `batch-script-child` a `verify` refusal code. Why it matters: the `.cmd`/`.bat` refusal and its closed word `batch-script-child` are the fork P3 lists and P4 decides. Any change moves `CAUSES` and its literals in the same commit.

@history_item n=53 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (MCP's lost-cause witness and the unserved row; MCP's `instance-unreachable` body names its cause) justified `record_live`'s never-bound endpoint by "a Windows `Serving` has no `Drop`". It mapped a failed `server_verified` to `server-verify-failed` "(off macOS)". Why it matters: the Windows reason text and the off-macOS qualifier are OS text naming removed code, for P3 to list and P4 to decide. The unserved row keeps its Linux behaviour.

@history_item n=54 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`cleanup`'s `endpoint_gone` on Unix) changed the Unix rule to "connect refused or path absent", with Windows "the pipe no longer exists", and witnessed it on ubuntu with `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone`. Why it matters: the Windows pipe arm of the cleanup check leaves, and the Unix check keeps holding.

@history_item n=55 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket directory's security negatives, unit cases, fixtures and helpers; the IPC boundary's directory check) made `tests/security_negatives_socket_dir.rs` `#![cfg(unix)]`, with off-Unix `check_socket_dir` returning `Ok(())` (`check_socket_dir_off_unix_reads_nothing`). It added `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`). It moved `Seen` into `refused_seen` outside the macOS-gated `refused_inside` so the two socket-dir span cases run on macOS. Why it matters: the off-Unix arm and its case leave, the gates drop, `SOCKET_ROOT_VAR` reads `XDG_RUNTIME_DIR` alone, and the `refused_seen` split stops being an OS carve-out.

@history_item n=56 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`server-verify-failed`'s reach and macOS's carve-out; the peer euid decision; the euid wiring, untestable and a known survivor) named macOS's absent pid comparison "the named carve-out, owned by no route entry". It added `peer_euid_admitted(observed, own, reported)` with `PEER_EUID_REPORTED = cfg!(unix)`, held by injected euids with no env, flag or feature seam. It recorded each client seam's euid wiring as held by no committed test. Why it matters: the macOS carve-out text leaves. Where `PEER_EUID_REPORTED`'s sibling goes, the decision keeps refusing an unreadable peer on Linux through injected arguments only, never a new seam.

@history_item n=57 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a test's endpoint directory is made private; `viola list` on the read rule; the timed hook path's two checks; the hook's two new fail-open causes) required test endpoint directories through `create_private_dir`, never a bare `tempfile::tempdir()`. It added the `windows-2025` `board_scan_holds_a_read_granted_home_…` case and put `check_endpoint_dir` and `peer_admitted` in the timed hook path as "a real check on Unix only". Why it matters: edited impostor, fail-open and handler tests keep private endpoint directories. The Windows board case leaves, and the two checks lose their Unix-only qualifier while staying under `SPINE_DEADLINE`.

@history_item n=58 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the census of OS-gated tests) added `tests/contract_os_gated_tests.rs`, a root `contract_` binary in `nextest-e2e`. It reads faults both ways and has the self-check `census_finds_an_unnamed_gate_and_passes_a_named_one`. Its count is 52 gated functions, 7 gated files and 59 `GATES` rows (rule: the rows of `GATES`, `cfg_attr` case rows included). It moved §2's multi-os-compat row to `CI matrix + contract`. Why it matters: this is the census the chunk re-points or retires with the gates it counts, never left red or vacuous. The 52/7/59 count and the multi-os-compat row move with it.

@history_item n=59 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the per-OS JUnit reader and its reading) added the operator-pass script `scripts/junit-os-sets.py` over `junit-windows-2025`/`junit-macos-latest`/`junit-ubuntu-latest`. It recorded the reading at ci#41918694243: 3386/3412/3458 cases, `ubuntu-not-macos` 48 (43 carve-out + 5 Linux-only). Why it matters: the three artifacts vanish with the legs, so the reader and its reading are retired or re-pointed together with the census (P3 reads, P4 decides).

@history_item n=60 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the ci kill line at 27 binaries) set the `ci` profile's stamped-home 20 s kill over 27 binaries (`.config/nextest.toml:24`, rule: the `binary(…)` names), `binary(conpty_sideload)` among them. Why it matters: removing `tests/conpty_sideload.rs` drops it from that filter line, and the count in §9 and §3 Bootstrap moves by the rule.

@history_item n=61 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the hangup case on Linux only) made `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` `cfg(target_os = "linux")` because macOS `ps` cannot print the ignored-signal mask (ci#41915621814). Why it matters: the gate exists only to keep the case off macos-latest and is a census row. With macOS gone it can drop, while the `ps -o sigignore=` reader and the Linux witness stay.

@history_item n=62 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the v1-40 stale-row dim witness landed; the force-killed-wrapper cleanup on both Unix runners) landed `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`cfg(unix)`, green on ubuntu-latest and macos-latest). It moved `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone` (`#[cfg(unix)]`) to both Unix runners. Why it matters: both are `cfg(unix)` census rows whose macOS leg leaves. They lose their gate where the only sibling left and stay green on ubuntu.

@history_item n=63 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (a terminated wrapper's child on every OS) removed the `#[cfg(windows)]` gate from `run_child_ends_when_its_wrapper_is_terminated` and wrote the terminate step per OS (Windows `OpenProcess` + `TerminateProcess`; Unix `kill -KILL`). Why it matters: the Windows terminate arm leaves and the Unix `kill -KILL` arm becomes the case's only, ungated path.

@history_item n=64 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the Frozen guard's one home; the root waits' count at 32 sites in 24 files) moved `Frozen` to `tests/support/frozen.rs`, its sole home (Unix `kill -STOP`/`ps -o stat=`/`kill -CONT`; Windows debugger attach), with four callers including `run_refuses_a_frozen_wrapper_as_stale_on_windows`. It recounted `Instant::now() + WITHIN` at 32 sites in 24 files under `tests/`. Why it matters: the Windows attach arm and its caller leave, and the `WITHIN` site count moves by its grep rule if removed tests held waits.
