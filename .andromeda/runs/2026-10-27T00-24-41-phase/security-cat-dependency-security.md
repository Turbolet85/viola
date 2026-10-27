# security · Dependency Security — category read

## Carried
- (.andromeda/security-plan.md:281) Part B's two clean release builds must be the release job's own build through `scripts/release-check.sh`, with its artifact-record judgement, its test-only-feature refusal and its `--probe` read unchanged by the chunk.
  > The per-OS `release` job builds `cargo build --release --locked --bin viola` through `scripts/release-check.sh`. The script judges that build's own `compiler-artifact` records, never a `target/release/` listing. It refuses any executable other than `viola`, and any artifact whose `features` include a test-only feature (`test-support`, `fake-agent`): `release-check: FAILED — test-only feature {feature} in {target}`. `--probe` runs first and must read 5/5 refused with a clean control.
- (.andromeda/security-plan.md:264) The reproducibility measurement must build on the exactly pinned product toolchain, never a host default, so that both builds of one tree share one compiler.
  > The product toolchain is pinned exactly by `rust-toolchain.toml` (`channel = "1.98.1"`, rustfmt + clippy); the native Linux dev host (where `pre-push` runs) and CI install it through rustup.
- (.andromeda/security-plan.md:261) Both builds must resolve the committed lockfile (`--locked`), and any crate the race fix adds must be pinned exactly in `[workspace.dependencies]`.
  > `Cargo.lock` and `fuzz/Cargo.lock` are committed. `[workspace.dependencies]` pins every third-party version of the root graph (portable-pty `=0.8.1`)
- (.andromeda/security-plan.md:274) Any dependency change the fix makes must pass the full `cargo deny check` gate on the root lockfile.
  > Job 4 (ubuntu) runs `cargo deny check` over all four check families, and fails the build on any advisory, yanked crate or unknown source.
- (.andromeda/security-plan-detail/dependency-security.md:14) The race fix lands in `viola-state` (a sync crate) and so must pull no `tokio` and no C-building crate; the bans and their live probes stay as they are.
  > The arch bans stay as they are: C-building crates in `deny.toml`, and `tokio` in the sync crates' graph for three target triples in `deny-sync.toml`, run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt` (arch §Build system).
- (.andromeda/security-plan.md:252) A new dependency the fix adds must hold a licence on the exact allow list; a new exception needs its own Decisions Log entry.
  > the `allow` list stays exactly `["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]` (arch §Build system).
- (.andromeda/security-plan-detail/dependency-security.md:26) If P3 changes `pin_companions`' wait-out on Windows, the ConPTY pins and their one textual home must stay untouched; the chunk moves no version.
  > The vendored ConPTY package is pinned exactly by four pins with one textual home, `src/conpty.rs`: the package version (`1.24.260710001`), the nupkg SHA-256 (`175640566a3b59c4b132070ee96c2c77e5ab7edd2e92732a5eb3610bbf63d90e`), `conpty.dll`'s SHA-256 (`39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8`) and `OpenConsole.exe`'s (`b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160`); `scripts/conpty-vendor.sh` parses them from that text.
- (.andromeda/security-plan-detail/dependency-security.md:48) Any CI step P4 adds to measure the builds on CI's three OSes must use only the SHA-pinned action set.
  > Actions are pinned by full commit SHA (the set `ci.yml` uses; SHAs resolved 2026-09-24 via `gh api repos/{repo}/commits/{tag}`):
- (.andromeda/security-plan.md:290) A CI measurement step must take no event-payload value except through `env:`, and must keep the workflow and job permissions.
  > A value from the event payload reaches a step only through `env:`, never through `${{ }}` inside `run:`. Today no workflow reads a `github.event` value at all.
- (.andromeda/security-plan.md:291) The permissions floor any edited workflow must keep.
  > Workflow-level `permissions: {}` and job-level `contents: read`.
- (.andromeda/security-plan.md:282) Any workflow edit must pass the zizmor step.
  > `zizmor .github/workflows/` runs on the ubuntu leg and fails on unpinned actions, `excessive-permissions`, template injection and cache poisoning.

## Bindings
- **The release-check build ↔ architecture §Build system (every Cargo profile keeps `panic = "unwind"`)**: a reproducibility change to a profile key, a linker flag or a path remap is a P4 fork for the founder and must leave the release-check build and the profile's unwind rule as they stand (.andromeda/security-plan.md:281)
- **The exact toolchain pin ↔ the build-identity question of CARRY 11**: the two clean builds are compared on the one pinned toolchain, so a difference cannot come from the toolchain (.andromeda/security-plan.md:264)
- **The sync-crate bans ↔ architecture §Build system and `scripts/deny-probes.sh`**: the `viola-state` race fix keeps `viola-state` tokio-free and C-build-free, proven by the per-crate deny run (.andromeda/security-plan-detail/dependency-security.md:14)
- **The ConPTY pins ↔ §Security Anti-Patterns → Data Protection (the companions' re-hash)**: a Windows concurrency change in `pin_companions` changes neither the pins nor the re-hash under a `FILE_SHARE_READ`-only handle (.andromeda/security-plan-detail/dependency-security.md:26)
- **The SHA-pinned actions and permissions ↔ test-plan §CI Integration**: a three-OS CI measurement, if P4 chooses one, joins `ci.yml` under the same pins, `env:`-only values and permissions floor (.andromeda/security-plan-detail/dependency-security.md:48)
