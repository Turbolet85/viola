# Cascade compare — architecture · phase · 2026-10-25-windows-endpoint-admission
E items 63 · held 17 · lost at classify 3 · lost at strong 3 · beside a placed fact - · cascade only 5 · body 23 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### established-decisions#8.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:67
- (.andromeda/architecture-detail/established-decisions.md:67) Admission must keep the v1 trust boundary as it stands: every process of the same OS user is admitted (the user SID ACE), so the DACL neither widens beyond nor narrows within that user.
  > This is an affordance, not an enforcement: an agent's Bash tool can still run CLI `viola release` (for example through `VIOLA_BIN`), and the Local endpoint trust boundary accepts it because v1 trusts every process of the same OS user.

### established-decisions#14.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:125
- (.andromeda/architecture-detail/established-decisions.md:125) This chunk must not serve `link` / `unlink` or build their verbs: they are held for the later entry "Explicit session links", which follows this one.
  > The Epoch 6 working-route entry "Explicit session links", after "Windows endpoint admission" and "Server verification before any frame", carries the hold

### stack-and-technologies#1.4 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:13
- (.andromeda/architecture-detail/stack-and-technologies.md:13) Any new Win32 call the chunk adds (e.g. reading the pipe's mode or its DACL back, or an open by another principal in a test) must come from the pinned windows-sys 0.61.2, its features named per module, with no new crate.
  > windows-sys 0.61.2 for the `TerminateProcess` kill fallback, the host console's raw/VT modes (`Win32_System_Console`) and its input read (`ReadConsoleW`, behind `viola_pty::host_stdin()`)

### stack-and-technologies#1.1 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:10
- (.andromeda/architecture-detail/stack-and-technologies.md:10) Code and tests the chunk adds must build on the declared floor 1.96 (checked by the CI `msrv` job) as well as the pinned 1.98.1, so no std API newer than 1.96 may be used.
  > Rust 1.98.1, pinned exactly by `rust-toolchain.toml` (rustfmt + clippy); workspace `rust-version = "1.96"`, edition 2024 | One native binary `viola` / `viola.exe`. 1.96 is the declared floor (the security toolchain floor; sysinfo 0.39.6 needs 1.95, std `File::lock` 1.89, rmcp 1.88). CI checks the floor in the `msrv` job

### stack-and-technologies#1.23 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:32
- (.andromeda/architecture-detail/stack-and-technologies.md:32) The chunk's code must pass clippy `-D warnings` with the print and tracing-macro bans, and the cargo-deny bans (C crates, the per-sync-crate tokio ban); the `supply-chain` advisories red is owned by the next route entry, never silenced here.
  > rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`

### stack-and-technologies#1.23 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:32
- (.andromeda/architecture-detail/stack-and-technologies.md:32) The deny gate the chunk's CI runs read.
  > cargo-deny 0.20.2 (advisories, licences, sources, bans: C crates, telemetry crates, feature bans; the tokio ban via `deny-sync.toml` per sync crate)

### standard-contracts#2.0 · lost at strong · .andromeda/architecture-detail/standard-contracts.md:25
- (.andromeda/architecture-detail/standard-contracts.md:25) If `src/cmd/run.rs`'s start refusal is touched, `run`'s collision check stays one of the five `read_snapshot(` product callers. It must still read an unreadable or unsupported snapshot as absent, and the count is kept by its stated rule.
  > `read_snapshot` keeps its results, and its five product callers (`src/cmd/client.rs`, `run`'s collision check, `src/cmd/hook.rs` twice, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still read every such snapshot as absent.

## Held

- conventions#1.1
- conventions#2.1
- cross-cutting-patterns#1.1
- established-decisions#1.1
- established-decisions#15.1
- established-decisions#17.1
- established-decisions#18.1
- established-decisions#22.1
- established-decisions#7.1
- occupied-resources#1.1
- occupied-resources#19.1
- stack-and-technologies#1.12
- stack-and-technologies#1.15
- stack-and-technologies#1.2
- stack-and-technologies#1.22
- stack-and-technologies#1.6
- standard-contracts#5.1

## Cascade only

- cross-cutting-patterns#2.1
- established-decisions#19.1
- occupied-resources#13.1
- occupied-resources#15.1
- occupied-resources#23.1
