# Cascade compare — architecture · phase · 2026-10-26-home-and-code-bearing-file-integrity
E items 121 · held 46 · lost at classify 1 · lost at strong 4 · beside a placed fact - · cascade only 11 · listed 3 · body 32 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### established-decisions#9.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:73
- (.andromeda/architecture-detail/established-decisions.md:73) Boundary: the macOS server-verify gap, client euid and socket-directory checks belong to "Unix endpoint and home hardening", not this chunk.
  > On macOS no peer pid is reported, so `viola_state::liveness::server_verdict` returns `Matches` with no comparison: the recorded gap. The macOS client euid and socket-directory checks, and the 0755-directory exit-cause case, are owed to the working-route entry "Unix endpoint and home hardening".

### established-decisions#17.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:142
- (.andromeda/architecture-detail/established-decisions.md:142) Single snapshot writer: every snapshot write must stay inside the wrapper's one in-process holder.
  > Every snapshot write goes through the one in-process holder `src/run/snapshot.rs` (`Snapshots`).

### occupied-resources#6.1 · lost at strong · .andromeda/architecture-detail/occupied-resources.md:41
- (.andromeda/architecture-detail/occupied-resources.md:41) The hidden `hook --capture` arm is outside the hook's entry-point set: the hook's new strict-modes check must not reach it (it reads no `VIOLA_*` and no snapshot).
  > no `VIOLA_*` read, no obs init, no channel; always exit 0 with empty stderr.

### stack-and-technologies#1.12 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:21
- (.andromeda/architecture-detail/stack-and-technologies.md:21) The DACL check before any diagnostics file must precede the opening of these home-level role files and the instance detail files they sit beside.
  > One-line JSON process logs into the viola home's `diagnostics/`; the line format and levels are owned by the obs plan

### stack-and-technologies#1.23 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:32
- (.andromeda/architecture-detail/stack-and-technologies.md:32) Any dependency the chunk adds for owner/DACL reads must pass cargo-deny's bans (no C crates) and the per-sync-crate tokio ban, and the code must pass clippy `-D warnings` with the print and dbg bans.
  > rustfmt, clippy (`-D warnings`, with the workspace `print_stdout` / `print_stderr` / `dbg_macro` bans and `clippy.toml` `disallowed-macros` on the tracing level macros), `cargo check`;

### stack-and-technologies#1.23 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:32
- (.andromeda/architecture-detail/stack-and-technologies.md:32) The supply-chain gate the chunk's dependency graph must stay green on.
  > cargo-deny 0.20.2 (advisories, licences, sources, bans: C crates, telemetry crates, feature bans; the tokio ban via `deny-sync.toml` per sync crate)

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### established-decisions#1.1 · listed (lost at classify) · .andromeda/architecture-detail/established-decisions.md:8
- (.andromeda/architecture-detail/established-decisions.md:8) The check code lands in a sync crate (`viola-state`) and is called from `viola-mcp` too; it must not pull Tokio into a sync crate.
  > CI enforces this with a sole-root `cargo deny` `tokio` ban (`deny-sync.toml`, run once per crate listed in `scripts/sync-crates.txt`), evaluated for the Windows, macOS and Linux target triples, plus a `cargo check` of the listed sync crates on all three OSes (CI job 3).

### stack-and-technologies#1.2 · listed (lost at classify) · .andromeda/architecture-detail/stack-and-technologies.md:11
- (.andromeda/architecture-detail/stack-and-technologies.md:11) The strict-modes check added to the CLI verbs, `hook` and `verify` must run on std threads with no async runtime; only `viola mcp`'s frame tools (CARRY 15) run it inside Tokio.
  > Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.3 built only inside `mcp` | No async runtime on the hot `hook` path.

### stack-and-technologies#1.15 · listed (lost at classify) · .andromeda/architecture-detail/stack-and-technologies.md:24
- (.andromeda/architecture-detail/stack-and-technologies.md:24) A new strict-modes refusal must be a typed thiserror variant in its crate, with anyhow only at the bin edge and context chains only in the owner-only detail file.
  > thiserror 2.0.20 (per crate; `viola-pty`'s `PtyError` is hand-written); anyhow 1.0.104 (bin edge only) | Typed errors that callers can branch on; context chains built at dispatch and recorded only in the owner-only instance detail file

## Held

- conventions#2.1
- conventions#3.1
- conventions#4.1
- conventions#5.1
- cross-cutting-patterns#1.1
- established-decisions#10.1
- established-decisions#12.1
- established-decisions#13.1
- established-decisions#14.1
- established-decisions#15.3
- established-decisions#18.2
- established-decisions#2.1
- established-decisions#20.1
- established-decisions#21.1
- established-decisions#21.2
- established-decisions#24.1
- established-decisions#25.1
- established-decisions#26.1
- established-decisions#6.1
- established-decisions#8.1
- occupied-resources#10.1
- occupied-resources#15.1
- occupied-resources#16.1
- occupied-resources#17.1
- occupied-resources#18.1
- occupied-resources#2.1
- occupied-resources#27.1
- occupied-resources#4.1
- occupied-resources#5.1
- occupied-resources#7.1
- occupied-resources#9.1
- stack-and-technologies#1.1
- stack-and-technologies#1.10
- stack-and-technologies#1.21
- stack-and-technologies#1.22
- stack-and-technologies#1.4
- stack-and-technologies#1.8
- stack-and-technologies#1.9
- stack-and-technologies#3.1
- standard-contracts#12.1
- standard-contracts#13.1
- standard-contracts#14.1
- standard-contracts#2.0
- standard-contracts#3.0
- standard-contracts#5.1
- standard-contracts#9.1

## Cascade only

- conventions#1.1
- established-decisions#11.1
- established-decisions#19.1
- established-decisions#5.1
- occupied-resources#1.1
- occupied-resources#13.1
- occupied-resources#14.1
- occupied-resources#25.1
- stack-and-technologies#1.17
- stack-and-technologies#2.1
- standard-contracts#4.1
