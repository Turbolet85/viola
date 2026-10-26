# Task — design · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The chunk
A dependency lock bump: the locked `tokio` moves from 1.53.1 to a patched version (`>=1.53.3`), the workspace's exact `=` pin moved with it, so `cargo deny check`'s advisories family stops failing on the unsound advisory RUSTSEC-2026-0386 and CI's `supply-chain` job reads green again.
No ignore-list widening: RUSTSEC-2017-0008 (via portable-pty `=0.8.1`) stays the only advisory ignore, and `deny.toml`'s policy is untouched.
No product behaviour change: no source file changes unless the bump forces one (the `AsyncFd` constructor deprecation read by clippy `-D warnings`), and no wire contract, exit code, refusal word, log line or human output changes.
It re-runs the gates: `cargo deny check` at CI's pinned version, the tokio bans and their probes, fmt, clippy, the workspace tests, the rmcp-features and release checks, and closes the `supply-chain` red of ci#40257114709 against that run's log.

## Reached files
- Cargo.toml · edits the `[workspace.dependencies]` `tokio` pin
- Cargo.lock · edits the `tokio` package entry (and `tokio-macros`, `tokio-stream`, `tokio-util` only where the new tokio requires)
- deny.toml · re-reads `cargo deny check` (advisories, the ignore list unchanged)
- deny-sync.toml · re-reads the tokio ban per sync crate
- scripts/channel-tokio-free.sh · re-reads `viola-channel`'s tokio-free normal graph
- scripts/deny-probes.sh · re-reads that every ban fires
- scripts/rmcp-features.sh · re-reads the rmcp feature gate
- scripts/release-check.sh · re-reads the release-build artifact check
- fuzz/Cargo.lock · re-reads the fuzz lockfile's own advisories and sources audit (no tokio in it)
- crates/viola-mcp/Cargo.toml · re-reads tokio containment
- crates/viola-channel/Cargo.toml · re-reads tokio containment (its `tokio` feature)
- crates/viola-e2e/Cargo.toml · re-reads tokio containment (test-only)
- .github/workflows/ci.yml · re-reads the `supply-chain` job

## Needs
- The workspace `tokio` pin moves from `=1.53.1` to an exact patched version at or above 1.53.3 (or a later published 1.53.x / 1.54.x), keeping the `=` form and `default-features = false`.
- The locked `tokio` in `Cargo.lock` matches the pin; companion crates `tokio-macros`, `tokio-stream`, `tokio-util` move only where the new tokio requires it; no other dependency is bumped.
- `cargo deny check` at CI's pinned cargo-deny version reads no RUSTSEC-2026-0386 finding and no other advisory, licence, ban or source failure, with no new ignore, `skip`, `allow` or licence exception.
- Tokio stays contained: built only in `viola-mcp`, `viola-channel`'s `tokio` feature and the test-only `viola-e2e`; `deny-sync.toml` per sync crate, `scripts/channel-tokio-free.sh` and `scripts/deny-probes.sh` keep their meaning and pass.
- The patched tokio deprecates the safe `AsyncFd` constructors (`new`, `with_interest`, `try_new`, `try_with_interest`); the lint gate runs clippy with `-D warnings`, so a call in viola's own code must not remain, while a dependency's own use (interprocess's tokio backend) is not linted by viola's gate.
- Any source change the deprecation forces stays behaviour-neutral: no wire contract, exit code, refusal word, log line or printed output of any verb changes.
- The hidden `viola mcp` verb (rmcp stdio server over Tokio) keeps writing only MCP frames on stdout and has no human design surface; the bump must not change what it prints.
- `viola-channel`'s Tokio client behind its `tokio` feature, used by `viola mcp`'s tools, keeps its frames and refusal codes unchanged.
- The rmcp feature gate (exactly `server` + `transport-io` in `viola`'s release graph) and the release check (the release build carries `viola` only) still pass after the lock move.
- The other gates hold: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`, the workspace tests.
- The CI red on ci#40257114709 (`b1249fb1`, job `supply-chain`) is closed against that run's log: its failure is RUSTSEC-2026-0386 alone; CI's `supply-chain` job is green on the chunk's push.
- `fuzz/Cargo.lock` holds no tokio, so its own advisories and sources audit is not moved by this chunk.

## Standing conventions
- (none)

## Not needed
- Color Palette (core colors, surface scale, text hierarchy, semantic and domain status colors, border progression): a 0.2.0 frontend design input; the chunk renders nothing.
- Typography, Spacing, Depth Strategy, Border Radius, Motion and Iconography: 0.2.0 frontend inputs or fixed CLI look; a dependency bump touches none.
- Brand Identity's expression level, domain anchors and the readback-box signature: no surface or state changes.
- The cli colour tokens and depth choice (amber `DIALOG`, dim `stale`, bold NAME, the colour decision order beyond `viola mcp`'s frames-only stdout): no human output changes.
- The `viola list` strip table, BAY line and unwrapped rows: untouched by the bump.
- The `viola send` readback mirror and its hints, `wait` / `last` result lines and the message escaper: unchanged.
- The wheel, link, gate, `answer` and `plugin install` verb phraseology: unchanged.
- The `viola verify` step counter and `run` / `revive` start output: unchanged.
- The exit-code phraseology and its `--json` documents: no exit code or refusal word changes.
- The `viola --help` navigation rows and the flat verb list: unchanged.
- The cli Toolkit's other pinned dependencies (clap 4.6.7 without `color`, windows-sys 0.61.2): only tokio and its companions move.
- The Self-Validation Protocol (swap, squint, signature, token, sameness tests): it applies to UI output, and the chunk presents none.
- The Anti-Patterns' per-surface cli bans: no printed output is added or changed.
