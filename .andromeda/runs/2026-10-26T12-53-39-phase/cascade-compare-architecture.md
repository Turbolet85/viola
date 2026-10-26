# Cascade compare — architecture · phase · 2026-10-26-explicit-session-links
E items 143 · held 42 · lost at classify 0 · lost at strong 3 · beside a placed fact - · cascade only 4 · listed 3 · body 76 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### established-decisions#17.1 · lost at strong · .andromeda/architecture-detail/established-decisions.md:142
- (.andromeda/architecture-detail/established-decisions.md:142) The snapshot rewrite of `links` after a served `link` / `unlink` must go through the one in-process holder.
  > Every snapshot write goes through the one in-process holder `src/run/snapshot.rs` (`Snapshots`).

### stack-and-technologies#1.1 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:10
- (.andromeda/architecture-detail/stack-and-technologies.md:10) New code must build on the pinned 1.98.1 toolchain and on the declared 1.96 floor the `msrv` job checks.
  > Rust 1.98.1, pinned exactly by `rust-toolchain.toml` (rustfmt + clippy); workspace `rust-version = "1.96"`, edition 2024 | One native binary `viola` / `viola.exe`.

### standard-contracts#6.1 · lost at strong · .andromeda/architecture-detail/standard-contracts.md:53
- (.andromeda/architecture-detail/standard-contracts.md:53) The explicit methods share the `LinkSet` with the implicit `send` link, whose ordering and "a held pair adds nothing" rule must stay intact.
  > Since chunk 2026-10-10-session-links a `send` whose `from` is a name other than the wrapper's own records the link `from → this instance` ([Links]) after the second wheel and running-turn reads and directly before `send-issued`: a new pair's `link` line stands directly before that send's `send-issued`, whose `cursor` is still the offset its own line starts at, and a pair the set holds adds nothing. A link that cannot be recorded answers `-32603` with nothing issued and nothing typed.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### stack-and-technologies#1.8 · listed (lost at strong) · .andromeda/architecture-detail/stack-and-technologies.md:17
- (.andromeda/architecture-detail/stack-and-technologies.md:17) The served `link` / `unlink` lines and the two methods' `params` must carry `v`, and readers must keep skipping unknown kinds and fields; no migration step replaces this.
  > N/A. Every record carries `v`, readers skip unknown kinds and fields, and a snapshot with an unsupported `v` is rebuilt by replaying the log.

### stack-and-technologies#1.15 · listed (lost at classify) · .andromeda/architecture-detail/stack-and-technologies.md:24
- (.andromeda/architecture-detail/stack-and-technologies.md:24) New error paths must be thiserror enums in the library crates, with anyhow confined to the root bin's dispatch edge and its chains only in the owner-only detail file.
  > thiserror 2.0.20 (per crate; `viola-pty`'s `PtyError` is hand-written); anyhow 1.0.104 (bin edge only) | Typed errors that callers can branch on; context chains built at dispatch and recorded only in the owner-only instance detail file

### stack-and-technologies#1.22 · listed (lost at strong) · .andromeda/architecture-detail/stack-and-technologies.md:31
- (.andromeda/architecture-detail/stack-and-technologies.md:31) The verbs and methods must build, lint and test on all three CI OSes against the fake agent, with the local `pre-push` gate a filter only.
  > | Build, lint and test on all three OSes against the fake agent, on native runners. Before the operator push, the local `viola-harness pre-push` gate runs the ubuntu test job's suites natively on the Linux dev host, in the working tree itself (no clone),

## Held

- conventions#2.1
- conventions#3.1
- conventions#4.1
- cross-cutting-patterns#1.1
- cross-cutting-patterns#2.1
- established-decisions#10.1
- established-decisions#11.1
- established-decisions#18.1
- established-decisions#21.1
- established-decisions#21.2
- established-decisions#26.1
- established-decisions#5.1
- established-decisions#6.1
- established-decisions#7.1
- established-decisions#8.1
- established-decisions#9.1
- occupied-resources#1.1
- occupied-resources#14.1
- occupied-resources#15.1
- occupied-resources#2.1
- occupied-resources#4.1
- occupied-resources#9.1
- stack-and-technologies#1.11
- stack-and-technologies#1.12
- stack-and-technologies#1.13
- stack-and-technologies#1.14
- stack-and-technologies#1.16
- stack-and-technologies#1.17
- stack-and-technologies#1.18
- stack-and-technologies#1.2
- stack-and-technologies#1.23
- stack-and-technologies#1.3
- stack-and-technologies#1.6
- stack-and-technologies#1.7
- stack-and-technologies#1.9
- stack-and-technologies#2.1
- stack-and-technologies#3.1
- standard-contracts#12.1
- standard-contracts#2.0
- standard-contracts#4.1
- standard-contracts#5.1
- standard-contracts#7.1

## Cascade only

- established-decisions#1.1
- established-decisions#20.1
- established-decisions#23.1
- occupied-resources#22.1
