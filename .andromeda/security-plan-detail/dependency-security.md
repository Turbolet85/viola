# security-plan · Dependency Security
Read whole. The master's broad view is .andromeda/security-plan.md; this file holds its Dependency Security detail.

## Dependency Security

<!-- detail: values · dep-conpty -->

- The vendored Microsoft ConPTY binaries (`vendor/conpty/1.24.260710001/x64/{conpty.dll,OpenConsole.exe}`, nuget `Microsoft.Windows.Console.ConPTY` 1.24.260710001, licence MIT, embedded into the Windows x64 `viola` by `include_bytes!`) sit outside every cargo graph, so no lockfile audit reaches them. Their gate is `scripts/conpty-vendor.sh`: `--verify` fetches the nupkg outside the tree, checks its SHA-256 before extraction, byte-compares each file against the committed one and its SHA-256, and checks the Authenticode signer (Microsoft), verdict `conpty-vendor: verified <version>`; `--probe` shows its four refusals (a nupkg SHA-256 mismatch before extraction, a per-file mismatch, an unsigned file, a TLS failure on an unreachable host) and a clean control, verdict `conpty-vendor probe: 4/4 refused, control clean`. The signer is checked there and in CI only: at run time the SHA-256 pin carries it (a hash-pinned blob cannot carry another signature). An own audit, never an exemption (Decisions Log `2026-09-29`).

<!-- /detail -->

<!-- detail: values · deps -->

- `deny.toml`'s `[[bans.features]]` also bans serde_json's `unbounded_depth` feature, its reason that every parse of external bytes keeps serde_json's default recursion limit; the `scripts/deny-probes.sh` row `serde_json-unbounded-depth` proves the ban fires, and the test `serde_json_unbounded_depth_is_banned_in_deny_toml` reads it in `deny.toml`.

<!-- /detail -->

<!-- detail: values · deps -->

- The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system). `viola-channel`, whose Tokio client (tokio `=1.53.3`, `io-util`) sits behind its `tokio` feature, left that list at chunk 2026-10-17-mcp-server-for-drivers and is covered instead by the `cargo tree` form `scripts/channel-tokio-free.sh` (its normal graph without the feature must print no `tokio v` line, its `--features tokio` control one; CI supply-chain step `Tokio ban per sync crate (sole root)`). Every ban, the tree form included (probe `channel-tokio-edge` and its channel control; `14/14 banned, control clean`, counted as the `probes=(` rows of `scripts/deny-probes.sh` plus the planted channel probe, as measured at viola-0.1.0/chunks/2026-10-27-bounded-inputs-at-every-boundary/report.md), is proven live by `scripts/deny-probes.sh`.

<!-- /detail -->

<!-- detail: exception · deps -->

- `fuzz/` is a separate cargo workspace (`viola-fuzz`, `publish = false`, excluded from the root by `exclude = ["fuzz"]`) with its own committed `fuzz/Cargo.lock`: `libfuzzer-sys =0.4.13` (builds C++ via `cc`), `arbitrary =1.4.2` and a path dep on `viola-core`. That lockfile is outside `cargo deny`'s root graph, so the root run's families (the C-build ban included) do not reach it. The operator ratified this on 2026-09-24 as a test-only exemption (Decisions Log). It holds only while the crate is never linked into a shipped binary and `fuzz` never joins the root `[workspace]`. The lockfile is audited separately: the ci.yml `supply-chain` step `Fuzz lockfile audit (advisories, sources)` runs `cargo deny --manifest-path fuzz/Cargo.toml --format json check advisories sources` (into `target/supply-chain/deny-fuzz.json`), and the weekly `nightly.yml` `advisories` job runs `cargo deny --manifest-path fuzz/Cargo.toml check advisories`. Both run from the repo root, where cargo-deny resolves the root `deny.toml` by walking up from the manifest (measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes). The C-build ban still does not apply to this graph.

<!-- /detail -->

<!-- detail: values · dep-conpty -->

- The vendored ConPTY package is pinned exactly by four pins with one textual home, `src/conpty.rs`: the package version (`1.24.260710001`), the nupkg SHA-256 (`175640566a3b59c4b132070ee96c2c77e5ab7edd2e92732a5eb3610bbf63d90e`), `conpty.dll`'s SHA-256 (`39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8`) and `OpenConsole.exe`'s (`b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160`); `scripts/conpty-vendor.sh` parses them from that text. The files are committed as binary (`.gitattributes` `vendor/conpty/** binary`). A version move is a re-vendor through the script's default mode, never a hand copy.

<!-- /detail -->

<!-- detail: values · deps-supply-chain -->

- A third workflow, `windows-mutants.yml` (founder ruling C2, 2026-10-04), runs only on `workflow_dispatch`, with no `inputs:` (no event-payload input class), at the epoch-boundary audit. It is never a gate and never a dependency of `ci.yml`. One dispatch was made outside that audit, on 2026-10-10, for chunk 2026-10-10-windows-mutation-grade alone and on the founder's own word for it (run 38036448183); the ruling stands as written.
  - Permissions: workflow `permissions: {}`; its one job definition (`windows-2025`, a ten-item matrix, counted as the `- package:` items under `matrix.include`; the root `viola` package is five of them, split by file, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`; job name `mutants (<label>)`) has `contents: read`.
  - Pins: checkout by SHA with `persist-credentials: false`, and install-action by SHA with ci.yml's cargo-nextest / cargo-mutants pins.
  - `matrix.*` reaches its pwsh step only through `env:`.
  - It uses no secret, cache or `needs:` and uploads nothing: the harness document and outcome lines stay in the job log, carrying repo-relative paths only. The unscanned-upload list is unchanged.

<!-- /detail -->

<!-- detail: exception · deps-supply-chain -->

- CI's G2 zero-panics gate (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs) has exactly one exemption. It does not count a role-file `panic` line whose `panic_location` is exactly `src/cmd/hook/seam.rs:<digits>` (the `FAKE_AGENT_HOOK_PANIC` seam's forced panic). The location is compared as a whole string, never as a prefix, suffix, regex or home path. The perf arm's own zero-panic check exempts nothing. G2's `--probe` runs before every G2 check. It proves that each of these reads red: another file, a look-alike path (`x/…/seam.rs`, `seam.rs.bak`, `seam.rsx`), the seam path without a line number, a missing location and an empty scope. Any further G2 exemption needs its own Decisions Log entry (§Security Anti-Patterns → Universal).

<!-- /detail -->

<!-- detail: values · deps-supply-chain -->

- Actions are pinned by full commit SHA (the set `ci.yml` uses; SHAs resolved 2026-09-24 via `gh api repos/{repo}/commits/{tag}`):
  - `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1` (with `persist-credentials: false`)
  - `Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2.9.2`
  - `taiki-e/install-action@7623a79cdfecb99d681017af368ca353d9f49bb5 # v2.87.19`
  - `actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1`

<!-- /detail -->
