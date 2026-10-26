# arch · cascade read · group 1 of 2

## Carried
- (.andromeda/architecture-detail/stack-and-technologies.md:11) The workspace's Tokio must move to a version RUSTSEC-2026-0386 lists as patched (`>=1.53.3`) and stay built only inside `mcp`; the row's version is the wrap's amendment, never the chunk's edit.
  > | Backend framework (concurrency model) | Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.1 built only inside `mcp` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp requires it |
  (.andromeda/architecture.md:13)
  > std threads with Tokio built only inside `mcp`
- (.andromeda/architecture.md:442) The Inherited Defaults line carries the same Tokio version beside the rmcp and clap pins: the new Tokio must hold the same placement, rmcp and clap unmoved, and this line is amended with the Stack row at the wrap.
  > Framework: Rust 1.98.1, pinned by `rust-toolchain.toml` (MSRV 1.96, edition 2024). A sync std-thread core; Tokio 1.53.1 only in `mcp`; rmcp 3.4.1 for MCP; clap 4.6.7 for the CLI.
- (.andromeda/architecture-detail/stack-and-technologies.md:10) The bumped tokio (and any companion it moves) must declare a rust-version no higher than the 1.96 floor, so CI's `msrv` job still builds the workspace.
  > 1.96 is the declared floor (the security toolchain floor; sysinfo 0.39.6 needs 1.95, std `File::lock` 1.89, rmcp 1.88). CI checks the floor in the `msrv` job: 1.96 is installed by `rustup toolchain install 1.96` and selected with `RUSTUP_TOOLCHAIN=1.96`, which outranks `rust-toolchain.toml`, so the pin is untouched.
- (.andromeda/architecture-detail/stack-and-technologies.md:13) The new tokio must resolve under the exact libc pin and the locked windows-sys version its own graph shares, moving neither, and portable-pty `=0.8.1` must stay as pinned.
  > portable-pty `=0.8.1` (pinned) behind viola's own `pty` seam; windows-sys 0.61.2 for the `TerminateProcess` kill fallback, the host console's raw/VT modes (`Win32_System_Console`) and its input read (`ReadConsoleW`, behind `viola_pty::host_stdin()`); libc `=0.2.189` (Unix only) for the host tty's raw mode
- (.andromeda/architecture-detail/established-decisions.md:14) The portable-pty `=0.8.1` pin is the source of RUSTSEC-2017-0008, which must remain the only advisory ignore after the bump; the pin does not move.
  > **[PTY] portable-pty `=0.8.1` pinned behind viola's own `pty` seam (spawn · read · write · resize · wait · kill).**
  (.andromeda/architecture.md:25)
  > Rationale: 0.8.1 is spike-proven; 0.9.0 has a Windows garbage-read bug.
- (.andromeda/architecture-detail/stack-and-technologies.md:15) interprocess 2.4.4 must stay at its version while its Tokio flavour builds against the new tokio, still reached only through `viola-channel`'s feature-gated client used by `viola-mcp` alone.
  > interprocess 2.4.4 `local_socket` (sync API; the Tokio flavour, `interprocess/tokio`, only in `viola-channel`'s feature-gated `tokio_client::TokioClient`, used by `viola-mcp` alone, which opens a Windows pipe through the same SQOS + overlapped `client::open_handle`)
  (.andromeda/architecture-detail/established-decisions.md:61)
  > Rationale: one API over named pipes and Unix sockets, sync and Tokio flavours, and no daemon.
  (.andromeda/architecture.md:39)
  > **[Message Broker / IPC] interprocess 2.4.4 `local_socket`, one endpoint per `viola run`, behind viola's own `channel` seam.**
  (.andromeda/architecture.md:444)
  > IPC: interprocess 2.4.4 local sockets, one endpoint per `viola run`, JSON-RPC 2.0 over ndjson with `v` + sender version in every `params`.
- (.andromeda/architecture-detail/stack-and-technologies.md:27) The new tokio must resolve under rmcp 3.4.1 with rmcp unchanged, and the release graph's rmcp features must still read exactly `server` + `transport-io` when `scripts/rmcp-features.sh` is re-run.
  > rmcp 3.4.1 (features `server`, `transport-io`), minor pinned; schemars 1.2.2 (tool input schemas) | stdio MCP server implementing spec 2026-07-28 (compatible with 2025-11-25): tools `send · wait · last · answer · list`. The release graph's rmcp features are exactly `server` + `transport-io` (`scripts/rmcp-features.sh`); the test-only `viola-e2e` takes rmcp `client` alone, never `transport-child-process`
  (.andromeda/architecture-detail/established-decisions.md:67)
  > **[MCP] rmcp 3.4.1, stdio, tools `send · wait · last · answer · list`. `release` is CLI-only.** Refusals are `isError: true` tool results mapped from `result.refusal`. Rationale: it is the official SDK, tracks spec 2026-07-28, and the pinned minor limits churn from its near-weekly releases.
- (.andromeda/architecture-detail/stack-and-technologies.md:32) `cargo deny check` must be re-run at CI's pinned cargo-deny 0.20.2 with the per-sync-crate tokio ban, and clippy's `-D warnings` must stay green, so viola's own code may call none of the `AsyncFd` constructors the patched tokio deprecates.
  > cargo-deny 0.20.2 (advisories, licences, sources, bans: C crates, telemetry crates, feature bans; the tokio ban via `deny-sync.toml` per sync crate)
  (.andromeda/architecture-detail/stack-and-technologies.md:32)
  > rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`
  (.andromeda/architecture.md:378)
  > `target/supply-chain/`: `deny.json` (the root graph), `deny-fuzz.json` (the `fuzz/Cargo.lock` advisories and sources audit) and `zizmor.json` from the CI `supply-chain` job, uploaded as artifact `supply-chain` (7 days, admissible by content: obs-plan §8 item 6), gitignored.
  (.andromeda/architecture.md:373)
  > `target/deny-probes/run-<utc>-<pid>/`: the throwaway Cargo projects `scripts/deny-probes.sh` writes per run, gitignored.
- (.andromeda/architecture-detail/established-decisions.md:54) The bumped crates must stay inside `deny.toml`'s licence allow list, which the re-run `cargo deny check` licences family reads; no exception is added.
  > Rationale: atomic-write-file 0.3.1 was the first choice, but its licence is BSD-3-Clause, outside `deny.toml`'s allow list (operator ruling at the 2026-09-27-instance-state-and-start-order plan); tempfile was already locked, MIT OR Apache-2.0.
  (.andromeda/architecture.md:37)
  > Rationale: atomic-write-file's BSD-3-Clause licence is outside `deny.toml`'s allow list, and tempfile was already locked.
- (.andromeda/architecture-detail/established-decisions.md:8) The keystone's containment and its CI enforcement — the sole-root tokio ban per crate of `scripts/sync-crates.txt` on three target triples, the `cargo check` of the sync crates on three OSes, and `scripts/channel-tokio-free.sh` — must hold unchanged after the bump and be re-run by the chunk.
  > - **[Concurrency / Backend Framework] Hybrid sync core + Tokio only in `mcp` (KEYSTONE).** Rationale: portable-pty's handles are blocking, so the pump needs threads either way. `hook` spawns once per hook event and must not pay for runtime startup. rmcp and interprocess-async require Tokio, so it is confined to the subcommand that uses them. CI enforces this with a sole-root `cargo deny` `tokio` ban (`deny-sync.toml`, run once per crate listed in `scripts/sync-crates.txt`), evaluated for the Windows, macOS and Linux target triples, plus a `cargo check` of the listed sync crates on all three OSes (CI job 3). `viola-channel`, whose optional `tokio` feature `viola-mcp` enables, is not on that list (since chunk 2026-10-17-mcp-server-for-drivers): its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh` in the same CI supply-chain step.
  (.andromeda/architecture.md:23)
  > CI enforces it with a sole-root `cargo deny` `tokio` ban and a `cargo check` of the sync crates on all three OSes.
  (.andromeda/architecture.md:291)
  > `viola-pty`, `viola-agent-claude` and `viola-state` are sync crates (listed in `scripts/sync-crates.txt`, beside `viola-core`: four crates since chunk 2026-10-17-mcp-server-for-drivers)
- (.andromeda/architecture-detail/established-decisions.md:190) The bump must add no tokio edge to any product crate but `viola-mcp` (and `viola-channel`'s optional feature), and must not reach `fuzz/`, its own workspace whose lockfile's separate audit stays as it is.
  > `fuzz/` (the `viola-fuzz` cargo-fuzz targets) is NOT a member. It is its own cargo workspace, excluded by the root's `exclude = ["fuzz"]`, so its nightly toolchain and C++-building libfuzzer-sys never enter the root graph. Among the product crates only `viola-mcp` enables `tokio` (`rt`); `viola-channel` lists it optional behind its `tokio` feature (`["dep:tokio", "interprocess/tokio"]`, the `TokioClient`), which `viola-mcp` enables and, in a package-scoped test build, the channel's own test-only `fake-agent = ["tokio"]` (chunk 2026-10-17-mcp-server-for-drivers); the test-only `viola-e2e` takes `rt` + `process`. No other product member's normal graph reaches tokio (the sole-root loop and `scripts/channel-tokio-free.sh`).
  (.andromeda/architecture.md:67)
  > among the product crates only `viola-mcp` enables `tokio`.
  (.andromeda/architecture.md:293)
  > Not a member: `viola-fuzz` (`fuzz/`, test-only, `publish = false`). It is its own cargo workspace with its own `Cargo.lock`, excluded from the root, and depends on `viola-core` and `viola-agent-claude` by path.
- (.andromeda/architecture-detail/stack-and-technologies.md:31) The chunk's green must be read on the three-OS CI matrix, the verdict of record, its `supply-chain` job included; the local pre-push gate run before the operator push is a filter only.
  > Build, lint and test on all three OSes against the fake agent, on native runners. Before the operator push, the local `viola-harness pre-push` gate runs the ubuntu test job's suites natively on the Linux dev host, in the working tree itself (no clone), every child through `/usr/bin/env -i HOME=<passwd home> PATH=<home>/.cargo/bin:/usr/local/bin:/usr/bin:/bin`; it refuses `pre-push-linux-only` off Linux, and its `tools` stage checks `cc`, the `rust-toolchain.toml` channel and ci.yml's `test`-job pins (it installs nothing) — a pre-push filter, never the verdict of record
  (.andromeda/architecture.md:75)
  > **[CI/CD] GitHub Actions native matrix on three OSes, tests against the fake agent, the real CLI only in local live tests.**
- (.andromeda/architecture-detail/occupied-resources.md:14) After the lock moves, the re-run `scripts/release-check.sh` must still find no test-only binary in the release build.
  > Test-only binaries, never in a release build: `viola-fake-agent` (a `[[bin]]` of the root package behind feature `fake-agent`, the stand-in `claude`;
  (.andromeda/architecture.md:268)
  > Test-only binaries, never in a release build: `viola-fake-agent` (a `[[bin]]` of the root package behind feature `fake-agent`, the stand-in `claude`), `viola-harness` (in `viola-e2e`; its command contract is test-plan §3) and `viola-mcp-tap` (a `[[bin]]` of `viola-e2e` that records a live MCP exchange for the `Mcp.` fixtures).

## Bindings
- **Stack Tokio row ↔ established decisions (keystone, module boundaries)**: the version moves while the containment it names stays — only `viola-mcp` (and `viola-channel`'s optional feature) builds tokio, enforced by the sole-root ban and `scripts/channel-tokio-free.sh` (.andromeda/architecture-detail/stack-and-technologies.md:11) (.andromeda/architecture-detail/established-decisions.md:8) (.andromeda/architecture-detail/established-decisions.md:190).
- **Stack Tokio row ↔ Inherited Defaults (body)**: both carry the one Tokio version and are amended together at the wrap, neither edited by the chunk (.andromeda/architecture-detail/stack-and-technologies.md:11) (.andromeda/architecture.md:442).
- **Stack rmcp / interprocess rows ↔ established decisions (MCP, IPC)**: the new tokio must resolve under rmcp 3.4.1 and interprocess 2.4.4 with both unmoved, the pin form of each kept (.andromeda/architecture-detail/stack-and-technologies.md:27) (.andromeda/architecture-detail/established-decisions.md:67) (.andromeda/architecture-detail/stack-and-technologies.md:15) (.andromeda/architecture-detail/established-decisions.md:60).
- **Stack code-quality row ↔ PTY decision**: the re-run cargo-deny 0.20.2 must read RUSTSEC-2017-0008 (via portable-pty `=0.8.1`) as the only ignore and no RUSTSEC-2026-0386 finding (.andromeda/architecture-detail/stack-and-technologies.md:32) (.andromeda/architecture-detail/established-decisions.md:14).
- **Stack code-quality row ↔ snapshot-writer decision**: the licences family the re-run reads is the allow list that refused a BSD-3-Clause crate; the bumped crates must sit inside it (.andromeda/architecture-detail/stack-and-technologies.md:32) (.andromeda/architecture-detail/established-decisions.md:54).
- **Stack language row ↔ Stack Tokio row**: the new tokio's rust-version must not exceed the 1.96 floor the `msrv` job builds at (.andromeda/architecture-detail/stack-and-technologies.md:10) (.andromeda/architecture-detail/stack-and-technologies.md:11).
- **Stack PTY row ↔ Stack Tokio row**: libc `=0.2.189` and windows-sys 0.61.2 are shared by tokio's graph and must not move under the bump (.andromeda/architecture-detail/stack-and-technologies.md:13).
- **Established decisions (keystone) ↔ cross-cutting patterns (group 2)**: the Tokio containment pattern keeps any forced source change on the `mcp` side, and the config rule bars a dependency's new environment read (.andromeda/architecture.md:410) (.andromeda/architecture.md:401).
- **Occupied resources (test-only binaries) ↔ Stack CI row**: `scripts/release-check.sh` re-asserts the release graph after the lock moves, read on the CI matrix that is the verdict of record (.andromeda/architecture-detail/occupied-resources.md:14) (.andromeda/architecture-detail/stack-and-technologies.md:31).
