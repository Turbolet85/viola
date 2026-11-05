@carried n=1 label=security-plan/dependency-security fact=fct-gezignrj
why: The chunk removes `vendor/conpty/`, so the own audit whose subject these binaries are, and the `windows-2025` CI step that runs it, must leave with them and no gate may be left over paths that no longer exist.
quote: The vendored Microsoft ConPTY binaries (`vendor/conpty/1.24.260710001/x64/{conpty.dll,OpenConsole.exe}`, nuget `Microsoft.Windows.Console.ConPTY` 1.24.260710001, licence MIT, embedded into the Windows x64 `viola` by `include_bytes!`) sit outside every cargo graph, so no lockfile audit reaches them. Their gate is `scripts/conpty-vendor.sh`:
@carried n=1 run=2 label=security-plan fact=fct-y5ir4euu
quote: The vendored Microsoft ConPTY binaries sit outside every cargo graph, so no lockfile audit reaches them: their gate is `scripts/conpty-vendor.sh`
@carried n=1 run=3 label=security-plan fact=fct-v76r7jfp
quote: The per-OS `test` job's `windows-2025`-only step `ConPTY vendor verification` runs `bash scripts/conpty-vendor.sh --verify` and `--probe` before the coverage run
@carried n=1 run=4 label=security-plan fact=fct-n3fibuep
quote: For the vendored ConPTY binaries: `scripts/conpty-vendor.sh` (re-vendor by default; `--verify`: the nupkg and per-file SHA-256 plus the Authenticode signer; `--probe`).
@carried n=1 run=5 label=security-plan fact=fct-reffq76w
quote: The `test` job's `windows-2025` step `ConPTY vendor verification` (`conpty-vendor.sh --verify`, `--probe`).
@carried n=2 label=security-plan/dependency-security fact=fct-mqyejp35
why: `src/conpty.rs`, the four pins' one textual home, is removed, so the script that parses them and the `.gitattributes` binary rule for `vendor/conpty/**` lose their subject and must not survive as live rules over removed code.
quote: The vendored ConPTY package is pinned exactly by four pins with one textual home, `src/conpty.rs`: the package version (`1.24.260710001`), the nupkg SHA-256 (`175640566a3b59c4b132070ee96c2c77e5ab7edd2e92732a5eb3610bbf63d90e`), `conpty.dll`'s SHA-256 (`39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8`) and `OpenConsole.exe`'s (`b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160`); `scripts/conpty-vendor.sh` parses them from that text. The files are committed as binary (`.gitattributes` `vendor/conpty/** binary`).
@carried n=2 run=2 label=security-plan fact=fct-hzqtl6ih
quote: The vendored ConPTY package is pinned exactly by four pins with one textual home, `src/conpty.rs`, from which `scripts/conpty-vendor.sh` parses them
@carried n=3 label=security-plan/dependency-security fact=fct-pgy5bb7v
why: The chunk edits `deny.toml` to drop the removed Windows dependencies' entries and the task re-asserts `cargo deny check` green with the `[[bans.features]]` entries live, so this ban, its probe row and its test must survive the edit untouched.
quote: `deny.toml`'s `[[bans.features]]` also bans serde_json's `unbounded_depth` feature, its reason that every parse of external bytes keeps serde_json's default recursion limit; the `scripts/deny-probes.sh` row `serde_json-unbounded-depth` proves the ban fires, and the test `serde_json_unbounded_depth_is_banned_in_deny_toml` reads it in `deny.toml`.
@carried n=3 run=2 label=security-plan fact=fct-mrnvctku
quote: `rmcp` denies `transport-streamable-http-server` and `auth`.
@carried n=3 run=3 label=security-plan fact=fct-32d4ty7r
quote: `[licenses]`: the `allow` list stays exactly `["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]` (arch §Build system). `0BSD` is admitted only per crate, by `[[licenses.exceptions]]` for `doctest-file` and `recvmsg`, the two 0BSD crates interprocess 2.4.4 pulls in
@carried n=3 run=4 label=security-plan fact=fct-serp72ds
quote: This is the only expected ignore; re-check it whenever the portable-pty pin moves.
@carried n=3 run=5 label=security-plan fact=fct-mi73d5qc
quote: Job 4 (ubuntu) runs `cargo deny check` over all four check families, and fails the build on any advisory, yanked crate or unknown source.
@carried n=3 run=6 label=security-plan fact=fct-uydjlfhj
quote: `Cargo.lock` and `fuzz/Cargo.lock` are committed. `[workspace.dependencies]` pins every third-party version of the root graph (portable-pty `=0.8.1`)
@carried n=3 run=7 label=security-plan fact=fct-q5ipmlaa
quote: `Cargo.lock` must resolve bytes `>=1.11.1` (RUSTSEC-2026-0007).
@carried n=4 label=security-plan/dependency-security fact=fct-o63eny7e
why: The task keeps the C-crate bans, `deny-sync.toml`, `channel-tokio-free.sh` and `deny-probes.sh` live after the manifest edits, so the per-triple sync-crate `tokio` ban and the count `deny-probes.sh` proves must be re-read against the Linux-only tree.
quote: The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system). `viola-channel`, whose Tokio client (tokio `=1.53.3`, `io-util`) sits behind its `tokio` feature, left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features tokio` control one; CI supply-chain step `Tokio ban per sync crate (sole root)`). Every ban, the tree form included (probe `channel-tokio-edge` and its channel control; `14/14 banned, control clean`, counted as the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe, as measured at viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md), is proven live by `scripts/deny-probes.sh`.
@carried n=4 run=2 label=security-plan fact=fct-cavsczz7
quote: every ban is proven live by `scripts/deny-probes.sh`.
@carried n=5 label=security-plan/dependency-security fact=fct-i5izpzxc
why: The chunk edits `nightly.yml` and `ci.yml`, and the task keeps on Linux both separate audits of `fuzz/Cargo.lock` and the unchanged `fuzz/` exemption, so the exemption's conditions and its two audit steps must survive the job removals.
quote: It holds only while the crate is never linked into a shipped binary and `fuzz` never joins the root `[workspace]`. The lockfile is audited separately: the ci.yml `supply-chain` step `Fuzz lockfile audit (advisories, sources)` runs `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` (into `target/supply-chain/deny-fuzz.json`), and the weekly `nightly.yml` `advisories` job runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories`.
@carried n=5 run=2 label=security-plan fact=fct-eugkq5vm
quote: A separate workflow, `nightly.yml` (weekly `schedule:` cron + `workflow_dispatch`, no cache), runs `cargo deny check advisories` and `cargo deny --manifest-path fuzz/Cargo.toml check advisories`
@carried n=5 run=3 label=security-plan fact=fct-blte3oxa
quote: its lockfile is audited separately for advisories and sources, and the C-build ban does not apply to it.
@carried n=6 label=security-plan/dependency-security fact=fct-glgoi2kc
why: The chunk deletes `windows-mutants.yml` whole, so every rule over it retires with the file, no `ci.yml` job may be left depending on it, and the unscanned-upload list stays as it is.
quote: A third workflow, `windows-mutants.yml` (founder ruling C2, 2026-10-04), runs only on `workflow_dispatch`, with no `inputs:` (no event-payload input class), at the epoch-boundary audit. It is never a gate and never a dependency of `ci.yml`.
@carried n=6 run=2 label=security-plan/dependency-security fact=fct-glgoi2kc
quote: It uses no secret, cache or `needs:` and uploads nothing: the harness document and outcome lines stay in the job log, carrying repo-relative paths only. The unscanned-upload list is unchanged.
@carried n=6 run=3 label=security-plan fact=fct-lpbl5pyp
quote: `windows-mutants.yml` runs only on `workflow_dispatch`, with no `inputs:`, at the epoch-boundary audit; it is never a gate and never a dependency of `ci.yml`
@carried n=6 run=4 label=security-plan fact=fct-4ggr436n
quote: **CI integration** (`.github/workflows/ci.yml`, `.github/workflows/nightly.yml`, `.github/workflows/windows-mutants.yml`):
@carried n=6 run=5 label=security-plan fact=fct-npplilir
quote: The dispatch-only `windows-mutants.yml` runs the `--package` arm, which reads no base.
@carried n=6 run=6 label=security-plan fact=fct-pxt7iprv
quote: zizmor's pedantic `concurrency-limits` (3 low findings, one per workflow, as read with zizmor 1.30.1 on the dev host 2026-10-04) was declined on 2026-09-24 and stays visible
@carried n=6 run=7 label=security-plan fact=fct-ivw6qaje
quote: `zizmor .github/workflows/` runs on the ubuntu leg and fails on unpinned actions, `excessive-permissions`, template injection and cache poisoning.
@carried n=7 label=security-plan/dependency-security fact=fct-fguujmtu
why: The `test` and `perf` jobs that run G2 lose their Windows and macOS legs, and the task re-asserts G2's single exemption and its probe, so the surviving Linux legs must keep both exactly.
quote: CI's G2 zero-panics gate (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs) has exactly one exemption. It does not count a role-file `panic` line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` (the `FAKE_AGENT_HOOK_PANIC` seam's forced panic). The location is compared as a whole string, never as a prefix, suffix, regex or home path. The perf arm's own zero-panic check exempts nothing. G2's `--probe` runs before every G2 check.
@carried n=7 run=2 label=security-plan fact=fct-wsxv34pw
quote: CI's G2 zero-panics gate has exactly one exemption, a role-file `panic` line whose `panic_location` is exactly the `FAKE_AGENT_HOOK_PANIC` seam's, compared as a whole string, its `--probe` run before every G2 check
@carried n=7 run=3 label=security-plan fact=fct-2c53xvmz
quote: its `tools` stage checks the host against CI's own pins — `cc`, the `rust-toolchain.toml` channel, and the cargo-nextest, cargo-mutants and cargo-llvm-cov versions it parses from ci.yml's `test`-job `tool:` line — and installs nothing.
@carried n=7 run=4 label=security-plan fact=fct-h3byb6pq
quote: It refuses any executable other than `viola`, and any artifact whose `features` include a test-only feature (`test-support`, `fake-agent`): `release-check: FAILED — test-only feature {feature} in {target}`. `--probe` runs first and must read 5/5 refused with a clean control.
@carried n=7 run=5 label=security-plan fact=fct-lagqorre
quote: The workspace `rust-version` floor is 1.96, and the CI `msrv` job checks it with `rustup toolchain install 1.96 --profile minimal` selected by `RUSTUP_TOOLCHAIN=1.96`.
@carried n=8 label=security-plan/bootstrap-phases fact=fct-sawbqqxr
why: The chunk removes the Windows arms of the listener hardening, the client server verification and the `viola-state` strict-modes checks, so the Unix arms named here must stay whole and act on Linux exactly as before while the off-Unix fallbacks leave with their arms.
quote: `viola-channel`: per-OS listener hardening (Windows SDDL converted by windows-sys into interprocess `security_descriptor` + windows-sys SID lookup; Unix 0700 per-user socket directory + chmod 0600 after the bind + `peer_creds` euid check).
@carried n=8 run=2 label=security-plan/bootstrap-phases fact=fct-sawbqqxr
quote: the verdict, pid + sysinfo start time against the snapshot's `pid` and `started_at`, lives in `viola-state` liveness (`verify_server`, `ServerVerdict`), called by the root bin and `viola-mcp`, built on Linux and Windows;
@carried n=8 run=3 label=security-plan/bootstrap-phases fact=fct-sawbqqxr
quote: then makes the liveness pre-check, the socket-directory check on the parent of the recorded endpoint (`check_endpoint_dir`), the connect, the peer euid decision and the server check.
@carried n=8 run=4 label=security-plan/bootstrap-phases fact=fct-sawbqqxr
quote: `check_present` (the `conpty/` companions); `check_socket_dir` and `check_endpoint_dir` (the socket directory, one `symlink_metadata` read, never a chmod, off Unix `Ok(())`); and the `Readable`, `Symlink` and `NotADirectory` refusals; on Windows, the protected user + SYSTEM DACL for a `--home` outside `%USERPROFILE%` and the owner + DACL check (windows-sys).
@carried n=8 run=5 label=security-plan/bootstrap-phases fact=fct-sawbqqxr
quote: The `settings.json` override is rewritten on Unix when the pinned path passes the plain-character rule; it is not written on Windows or for a pinned path a shell could misread (§ Data Protection, Code-bearing artefacts).
@carried n=8 run=6 label=security-plan fact=fct-pz6zubwd
quote: per-OS listener hardening and client server verification in `viola-channel`, owner-only home creation, the strict-modes checks and the pinned binary's SHA-256 re-hash in `viola-state`, and code-bearing files rewritten at every start, every exec-form `command` the absolute pinned path
@carried n=9 label=security-plan/bootstrap-phases fact=fct-f326yvjk
why: The chunk shrinks the `test` and `perf` jobs' per-OS uploads to the Linux leg and keeps the nightly fuzz job, so every scan-gated upload, `harness-<os>` with its `chunk.diff` exclusion included, must still wait on the canary scan and the fuzz crash inputs stay the one unscanned upload.
quote: so it is skipped by exact path and the `harness-<os>` upload drops it (`!target/agent-run/chunk.diff`): it is never scanned and never uploaded, and a `chunk.diff` anywhere else under the capture is still scanned. It never prints or writes the matched bytes, and every scan-gated upload (`diag-`, `junit-`, `harness-<os>`) waits for it to pass.
@carried n=9 run=2 label=security-plan/bootstrap-phases fact=fct-f326yvjk
quote: the nightly `fuzz` job's `fuzz/artifacts/` on failure: crash inputs the fuzzer mutated from the committed synthetic corpus.
@carried n=9 run=3 label=security-plan fact=fct-rwpm3oj4
quote: every scan-gated upload waits for it to pass; one upload leaves CI unscanned, admissible by content: the nightly fuzz job's crash inputs, mutated from the committed synthetic corpus, which a non-synthetic seed must first drop.
@carried n=9 run=4 label=security-plan fact=fct-zhdn6mmq
quote: Its uploads `perf-<os>` (hyperfine exports) and `diag-perf-<os>` (the perf home's diagnostics) go up only when that scan succeeds, and `secret-scan-perf-<os>` only when it fails; every input is synthetic (obs-plan §8 item 6).
@carried n=9 run=5 label=security-plan fact=fct-h2ql6c6w
quote: It reads every file under `target/agent-run/` except `run --mutants`' `target/agent-run/chunk.diff`, which is skipped by exact path and excluded from the `harness-<os>` upload, so it is never uploaded.
@carried n=9 run=6 label=security-plan fact=fct-umh22lbq
quote: a non-`0600` diagnostics file, Unix only.
@carried n=10 label=security-plan/secret-management fact=fct-fdqq3i6o
why: The chunk removes the Windows registry reader of persistent-environment names, so R8 must keep on Linux its exact-match strip with `config.json` `claude_env_keep` as the persistent set and the identity floor stripped whatever that set says, names only and never a value.
quote: R8 (operator ruling 2026-09-25, Decisions Log): every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) is removed except a name in the persistent set — Windows registry `Environment` value names, Unix `config.json` `claude_env_keep` — and the identity floor (`IDENTITY_FLOOR`, the 11 names measured on the Windows host, both messaging variables among them) is removed whatever the persistent set says. The strip, the registry reader and the logs handle names only and never read a value;
@carried n=10 run=2 label=security-plan/secret-management fact=fct-fdqq3i6o
quote: the variables outside `VIOLA_*` plumbing are the test-only seams `FAKE_AGENT_PUMP_DELAY_MS` and `FAKE_AGENT_HOOK_PANIC`, compiled out of release builds
@carried n=10 run=3 label=security-plan/secret-management fact=fct-fdqq3i6o
quote: The floor stays the eleven: the four are not identity names (the founder's live ruling of 2026-10-07T11:37Z, relayed by the overseer), and the prefix rule already removes them whenever they are inherited outside the persistent set.
@carried n=10 run=4 label=security-plan fact=fct-ugohyrrh
quote: the inherited `CLAUDE*` credentials stay only in the `viola run` wrapper's environment, removed from the child by the R8 rule (the identity floor whatever the persistent set says) and never serialised into events, snapshots, diagnostics or the ledger; the strip handles names only and never reads a value.
@binding n=1
**dependency-security (the vendored ConPTY gate and pins) ↔ bootstrap-phases (dep-audit-tooling-install, dep-security-ci-gate)**: removing `src/conpty.rs` and `vendor/conpty/` must retire in the same change the four pins, the `windows-2025` `ConPTY vendor verification` step and the `.gitattributes` `vendor/conpty/** binary` rule, and `scripts/conpty-vendor.sh` must not stay a live gate over removed paths; the plan's ConPTY rows are named as expected wrap amendments, not edited here (security-plan/dependency-security fct-gezignrj) (security-plan/dependency-security fct-mqyejp35) (security-plan fct-reffq76w)
@binding n=2
**dependency-security (deny.toml, licences, pinning) ↔ the manifest edits**: the `deny.toml` edit drops only the removed Windows dependencies' entries; after it `cargo deny check` must be green over all four families, the licence `allow` list stays exactly the four, `0BSD` stays per-crate exceptions only, the `rmcp` and `serde_json` feature bans and their probe rows stay live, the RUSTSEC-2017-0008 ignore stays tied to portable-pty `=0.8.1`, `Cargo.lock` stays committed and every build `--locked`, with the rmcp and bytes floors in force (security-plan/dependency-security fct-pgy5bb7v) (security-plan fct-32d4ty7r) (security-plan fct-serp72ds) (security-plan fct-uydjlfhj)
@binding n=3
**dependency-security (sync-crate tokio ban) ↔ the Linux-only target**: the `tokio` ban per sync crate must hold for every triple `deny-sync.toml` names; dropping a non-Linux triple is a change this chunk owns, and `scripts/deny-probes.sh` must still prove every ban live with its count re-read, not carried over from `14/14` (security-plan/dependency-security fct-o63eny7e) (security-plan fct-cavsczz7)
@binding n=4
**dependency-security (windows-mutants.yml, zizmor) ↔ bootstrap-phases (secret-scanning-ci-gate)**: deleting `windows-mutants.yml` removes no scan gate and adds no unscanned upload, the nightly fuzz crash inputs staying the one; `zizmor .github/workflows/` must pass on the ubuntu leg over the remaining workflows with no `concurrency:` block added, its per-workflow finding count re-read; the `--package` arm keeps the `run --mutants` ban wherever that code survives (security-plan/dependency-security fct-glgoi2kc) (security-plan fct-pxt7iprv) (security-plan/bootstrap-phases fct-f326yvjk)
@binding n=5
**bootstrap-phases (secret-scanning-ci-gate) ↔ dependency-security (the per-OS test, perf and release jobs)**: shrinking the jobs to the Linux leg keeps each scan-gated upload (`diag-`, `junit-`, `harness-`, `perf-`, `diag-perf-`) behind its passing scan, `secret-scan-perf-` only on failure, `target/agent-run/chunk.diff` skipped and never uploaded; G2's single exemption and `--probe`, the release `--probe` 5/5 refused, the `test`-job `tool:` line the pre-push tools stage parses, and the `msrv` job on 1.96 all survive on Linux (security-plan/bootstrap-phases fct-f326yvjk) (security-plan fct-zhdn6mmq) (security-plan/dependency-security fct-fguujmtu) (security-plan fct-h3byb6pq) (security-plan fct-2c53xvmz)
@binding n=6
**bootstrap-phases (auth-scaffolding-baseline) ↔ secret-management (R8)**: the Windows arms leave (the SDDL listener, the SQOS client open and named-pipe server pid, the owner + DACL strict-modes arm, `check_present` over `conpty/` companions, the registry reader of persistent-environment names); the Unix arms stay whole and, where made unconditional, refuse exactly what they refused before, an off-Unix `Ok(())` leaving with its sibling and never becoming a skipped check; on Linux R8 strips by exact name with `config.json` `claude_env_keep` as the persistent set and the eleven-name identity floor stripped whatever it says (security-plan/bootstrap-phases fct-sawbqqxr) (security-plan/secret-management fct-fdqq3i6o) (security-plan fct-ugohyrrh)
@binding n=7
**bootstrap-phases (the canary scan's non-0600 class) ↔ secret-management (the test-only seams)**: the scan class for non-`0600` diagnostics files, made unconditional, keeps its Linux behaviour and gains no reach; the two fake-agent seams stay compiled out of release builds and no new seam or environment read appears without its own Decisions Log entry (security-plan fct-umh22lbq) (security-plan/secret-management fct-fdqq3i6o)
