# Cascade compare — architecture · phase · 2026-10-26-server-verification-before-any-frame
E items 146 · held 25 · lost at classify 3 · lost at strong 2 · beside a placed fact - · cascade only 6 · listed 2 · body 80 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### cross-cutting-patterns#1.1 · lost at strong · .andromeda/architecture-detail/cross-cutting-patterns.md:9
- (.andromeda/architecture-detail/cross-cutting-patterns.md:9) The snapshot every verifying client reads must be the one under the home `resolve_home` / `canonical_home` resolved, the resolution creating nothing.
  > `resolve_home(flag, name, dir, user_home)` in `src/cmd/mod.rs` resolves `--home` as given, else the `instance_of` home when both `VIOLA_NAME` and `VIOLA_DIR` pass the one shape check (a valid `ViolaName`, an absolute dir ending `instances/<name>`), else `<user home>/.viola`, a failing pair falling through; `resolved_home` reads the two variables and `std::env::home_dir()` once, at every verb arm of `dispatch`; `canonical_home` canonicalises a home that exists (`std::fs::canonicalize`, a Windows `\\?\C:\…` written back as `C:\…`, every other verbatim form kept) and keeps `std::path::absolute`'s spelling for one that does not, creating nothing;

### cross-cutting-patterns#1.1 · lost at strong · .andromeda/architecture-detail/cross-cutting-patterns.md:9
- (.andromeda/architecture-detail/cross-cutting-patterns.md:9) The verification's tests must run each wrapper in its own home, so a test's endpoint and snapshot are never another test's.
  > This is the isolation point for tests: each test runs in its own home, and that home's `ledger/stamps.json` can stamp the fake agent's reported version.

### cross-cutting-patterns#1.1 · lost at strong · .andromeda/architecture-detail/cross-cutting-patterns.md:12
- (.andromeda/architecture-detail/cross-cutting-patterns.md:12) No environment variable may switch the verification off or alter it, and a new dependency or feature brought in for the start-time or peer read (sysinfo, an interprocess feature) must read no environment variable.
  > There are no secrets in v1, and environment variables are not a configuration channel. The `VIOLA_*` variables are wrapper-to-child plumbing set by `viola run` only.

### cross-cutting-patterns#1.1 · lost at strong · .andromeda/architecture-detail/cross-cutting-patterns.md:12
- (.andromeda/architecture-detail/cross-cutting-patterns.md:12) The rule binds a dependency's own environment reads as it binds a direct one.
  > a dependency or feature that reads an environment variable breaks this rule like a direct read does.

### occupied-resources#3.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:23
- (.andromeda/architecture-detail/occupied-resources.md:23) The `viola mcp` client keeps its existing home check and exit shape; a verification failure is a per-tool outcome, never this process-level exit.
  > `mcp` (chunk 2026-10-17-mcp-server-for-drivers) is hidden from `viola --help` (`#[command(hide = true)]`: run by the plugin, never by a person) and takes no argument but the global `--home`; it serves the five tools over stdio until stdin closes, and exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` (each that exists) fails `viola_state::strict::check_path`.

### occupied-resources#19.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:132
- (.andromeda/architecture-detail/occupied-resources.md:132) The test-side endpoint waits the exiting-wrapper CARRY rests on: a verification test of a stopped or killed wrapper must keep these waits' rules (unconnectable endpoint after stop, holder gone after kill).
  > One of the waits is `tests/support/home.rs` `wait_endpoint_gone`, which `Wrapper::stop` / `stop_keep` run after the wrapper's exit until the snapshot's recorded `endpoint` is unconnectable; after a kill, `Wrapper::kill` waits on `holder_gone` instead (on Unix a connect to the path is refused or the path is absent, on Windows the pipe is not found), because a killed wrapper removes nothing and its socket file stays on Unix

### occupied-resources#19.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:132
- (.andromeda/architecture-detail/occupied-resources.md:132) A count with its rule: a root test wait added on the `WITHIN` bound moves this count, read back by its pattern at the wrap.
  > `Instant::now() + WITHIN` reads 28 sites in 20 files under `tests/`, as measured at chunk 2026-10-10-budget-governor's wrap on the Linux dev host's final tree

### stack-and-technologies#1.1 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:10
- (.andromeda/architecture-detail/stack-and-technologies.md:10) Every API the check uses (sysinfo, windows-sys, libc, std) must build on the declared 1.96 floor, which CI's `msrv` job checks.
  > 1.96 is the declared floor (the security toolchain floor; sysinfo 0.39.6 needs 1.95, std `File::lock` 1.89, rmcp 1.88). CI checks the floor in the `msrv` job

### standard-contracts#8.1 · lost at classify · .andromeda/architecture-detail/standard-contracts.md:65
- (.andromeda/architecture-detail/standard-contracts.md:65) `list` (CLI and MCP) contacts no wrapper, so it writes no frame and takes no server verification.
  > `list` is not a channel method. The calling process reads `instances/*/` (heartbeat and snapshot) and `claude agents --json` itself, so `list` needs no live wrapper.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### established-decisions#18.1 · listed (lost at classify) · .andromeda/architecture-detail/established-decisions.md:184
- (.andromeda/architecture-detail/established-decisions.md:184) A new verification-failure error must be a variant of the owning crate's single thiserror enum, not a new enum.
  > **[Error Handling] thiserror 2.0.20, one enum per workspace crate

### established-decisions#18.1 · listed (lost at classify) · .andromeda/architecture-detail/established-decisions.md:184
- (.andromeda/architecture-detail/established-decisions.md:184) The verification failure's anyhow chain, if any, goes only to `detail-<role>.ndjson`, never to a role line, stdout or stderr. The outcome must still map to the three surfaces' typed outcomes.
  > `RefusalReason` lives in `viola-core`. Rationale: callers branch on refusal variants across three surfaces (channel, CLI exit codes, MCP `isError`). anyhow's context chains are only useful where a person reads them: the owner's post-mortem, which gets the chain as `chain` in `instances/<name>/diagnostics/detail-<role>.ndjson`, never a role line, stdout or stderr.

### stack-and-technologies#1.15 · listed (lost at strong) · .andromeda/architecture-detail/stack-and-technologies.md:24
- (.andromeda/architecture-detail/stack-and-technologies.md:24) A verification-failure error type must be a thiserror variant in its own crate's enum (anyhow only at the bin edge), with any context chain kept to the owner-only detail file.
  > thiserror 2.0.20 (per crate; `viola-pty`'s `PtyError` is hand-written); anyhow 1.0.104 (bin edge only) | Typed errors that callers can branch on; context chains built at dispatch and recorded only in the owner-only instance detail file

## Held

- conventions#1.1
- conventions#2.1
- conventions#3.1
- established-decisions#1.1
- established-decisions#10.1
- established-decisions#13.1
- established-decisions#14.1
- established-decisions#17.1
- established-decisions#19.1
- established-decisions#22.1
- established-decisions#7.1
- established-decisions#8.1
- established-decisions#9.1
- occupied-resources#1.1
- occupied-resources#13.1
- occupied-resources#14.1
- occupied-resources#8.1
- stack-and-technologies#1.17
- stack-and-technologies#1.2
- stack-and-technologies#1.23
- stack-and-technologies#1.6
- stack-and-technologies#1.9
- standard-contracts#10.1
- standard-contracts#2.0
- standard-contracts#7.1

## Cascade only

- established-decisions#4.1
- established-decisions#5.1
- established-decisions#6.1
- stack-and-technologies#1.14
- stack-and-technologies#1.18
- stack-and-technologies#1.8
