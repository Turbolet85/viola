# arch extract

## Relevance
partial — no new code, crate, contract or resource; the chunk moves a pinned Stack version and must keep the Tokio containment, the workspace pin form and the dependency-policy gates intact.

## Constraints
- architecture §Established Decisions → [Concurrency / Backend Framework] (KEYSTONE) requires Tokio confined to the `mcp` path: the sole-root `deny-sync.toml` tokio ban per crate of `scripts/sync-crates.txt` (three target triples) and `scripts/channel-tokio-free.sh` keep their meaning across the bump; the new tokio (or a companion crate it pulls) must open no tokio edge into `viola-core`, `viola-pty`, `viola-state` or `viola-agent-claude`.
- architecture §Established Decisions → [Module Boundaries] requires that among product crates only `viola-mcp` enables `tokio` (`rt`), `viola-channel` lists it optional behind its `tokio` feature, and the test-only `viola-e2e` takes `rt` + `process`; the bump changes no feature set on any of these edges (whether the new tokio's feature names or defaults differ from 1.53.1 is research's question).
- architecture §Infrastructure Patterns → Build system requires `[workspace.dependencies]` to pin every third-party version in one place, so the tokio move is made in the root `Cargo.toml` pin (the exact `=` form kept) with `Cargo.lock` following it, never a per-crate override.
- architecture §Infrastructure Patterns → Build system notes `resolver = "3"` prefers versions compatible with `rust-version = "1.96"`; the chosen tokio (and any companion it forces) must build on the 1.96 floor checked by CI's `msrv` job (whether the patched tokio's own MSRV fits 1.96 is research's question).
- architecture §Infrastructure Patterns → Build system requires `cargo deny check` over `deny.toml` (advisories, licences `allow` exactly MIT · Apache-2.0 · Zlib · Unicode-3.0, sources crates.io only, the C-building-crate, telemetry and feature bans) to pass for the three target triples; a new tokio or companion pulling a banned crate or an off-list licence is a red, never a policy edit.
- architecture §Stack and Technologies → stack and §Inherited Defaults name Tokio 1.53.1 (built only inside `mcp`); the new version is recorded there by the wrap's amendment, not by this chunk's edits (per the scope's Boundaries).
- architecture §Cross-cutting Patterns → Tokio containment requires that only `viola-mcp` builds a runtime; the bump adds no runtime use on the `run`, `hook`, `send` or channel-server paths.

## Patterns to follow
- Exact-pin form in `[workspace.dependencies]` as for portable-pty `=0.8.1` and sha2 `=0.11.0` (per architecture §Infrastructure Patterns → Build system; §Stack and Technologies → stack).
- CI's `supply-chain` sequence as the proof of record: target job 4 `cargo deny --format json check`, the fuzz lockfile audit, the sole-root tokio ban loop then `scripts/channel-tokio-free.sh`, then `scripts/deny-probes.sh` (per architecture §Infrastructure Patterns → CI/CD approach).
- The tree-form tokio check reads absence from the printed line (`no tokio v` line without `--features tokio`, one with it), never the exit code (per architecture §Infrastructure Patterns → Build system).
- A Cargo feature or lock change that only re-unifies leaves `Cargo.lock` otherwise unmoved; companion crates (`tokio-macros`, `tokio-stream`, `tokio-util`) move only where the new tokio requires it (per architecture §Infrastructure Patterns → Crate dependency direction, the windows-sys feature precedent of a lock that does not move).

## Anti-patterns to avoid
- A direct-parent `wrappers` allowlist or a workspace `--exclude` to make the tokio ban pass (per architecture §Infrastructure Patterns → Build system).
- Relaxing the pin to a range or adding tokio to a sync crate's normal graph to reach a patched version (per architecture §Established Decisions → [Concurrency / Backend Framework]).

## Contract bindings
- arch ↔ security: the advisory/ignore policy (RUSTSEC-2017-0008 the only ignore, no `skip`/`allow` widening) is security-plan §Dependency Security's; arch supplies the `deny.toml` / `deny-sync.toml` gate shape it runs through (architecture §Infrastructure Patterns → Build system).
- arch ↔ tests: the `msrv` job (`RUSTUP_TOOLCHAIN=1.96` `cargo check --workspace`, `agent-run run --unit`) and the lint job's `rmcp-features` check are test-harness gates that read the bumped graph (architecture §Infrastructure Patterns → CI/CD approach).

## Acceptance criteria contributions
- `cargo deny --config deny-sync.toml --manifest-path crates/<crate>/Cargo.toml check bans` passes for every crate of `scripts/sync-crates.txt`, `scripts/channel-tokio-free.sh` prints `channel-tokio-free: ok`, and `scripts/deny-probes.sh` reads all bans banned with a clean control (per architecture §Infrastructure Patterns → Build system).
- The root `Cargo.toml` `[workspace.dependencies]` tokio entry is an exact `=` pin equal to the version `Cargo.lock` resolves, and no crate's `Cargo.toml` changes its tokio feature set (per architecture §Established Decisions → [Module Boundaries]).
- `cargo check --workspace` under `RUSTUP_TOOLCHAIN=1.96` passes with the bumped lock (per architecture §Stack and Technologies → stack, the 1.96 floor row).
