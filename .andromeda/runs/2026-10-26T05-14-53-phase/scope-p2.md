# Scope — tokio RUSTSEC-2026-0386 lock bump (2026-10-26-tokio-rustsec-2026-0386-lock-bump)

**Working entry (viola-0.1.0/working-route.md, Epoch 6 — Security hardening):** tokio RUSTSEC-2026-0386 lock bump — a
lock bump of `tokio` to `>=1.53.3`: the `unsound` advisory RUSTSEC-2026-0386 against the locked tokio 1.53.1 fails CI's
`supply-chain` job, with no ignore-list widening (per security-plan §Dependency Security).

**Invocation:** the operator's, relaying overseer1's call (`inputs#I1`): an ordinary chunk with no forced sample at P2;
the stub run dir `.andromeda/runs/2026-10-26T02-06-15-phase/` (a phase start that stopped after its CI read) is left as
it is. Standing (same input): no epoch boundary inside Epoch 6; read `df -h /tmp` before a stamped-home gate; a
boundary widening halts for the founder's live word.

## What it builds
The locked `tokio` moves from 1.53.1 to a version the advisory lists as patched, so `cargo deny check`'s advisories
family stops failing on RUSTSEC-2026-0386 and CI's `supply-chain` job reads green again, with RUSTSEC-2017-0008 (via
portable-pty `=0.8.1`) still the only advisory ignore.

- The advisory (`inputs#I2`, the advisory DB at `21d909e4`): RUSTSEC-2026-0386, `tokio`, dated 2026-10-13,
  `informational = "unsound"`, the safe `AsyncFd` constructors (`new`, `with_interest`, `try_new`,
  `try_with_interest`) violate I/O safety; patched `>= 1.51.6, < 1.52.0` and `>= 1.53.3`.
- [inferred] The bump is a manifest pin move as well as a lock move: the workspace pins tokio exactly,
  `tokio = { version = "=1.53.1", default-features = false }` (`Cargo.toml:242`, `[workspace.dependencies]`), so no
  `cargo update` can lift the lock past it while that pin stands. The entry's "lock bump" names the outcome, not
  the only file that changes.
- [inferred] The target version: the entry's floor is `>=1.53.3`, and the workspace's pin form (`=`) stays. Whether
  the pin is `=1.53.3` or a later published 1.53.x or 1.54.x is P4's lean, read against what crates.io publishes
  and what the lock then pulls (P3).
- [inferred] Companion crates `tokio-macros` 2.7.2, `tokio-stream` 0.1.19 and `tokio-util` 0.7.20 (`Cargo.lock:1648-1671`)
  move only where the new tokio requires it; no other dependency is bumped by this chunk.
- [inferred] The deprecation the advisory names lands in the patched versions: "Starting in Tokio 1.51.6, 1.53.3, and
  1.54.0, `AsyncFd::new`, `AsyncFd::with_interest`, `AsyncFd::try_new`, and `AsyncFd::try_with_interest` are
  deprecated in favor of the `unsafe` constructors" (`inputs#I2`). The lint gate runs clippy with `-D warnings`, so a
  call in viola's own code would turn the bump red; a dependency's own use (interprocess's `tokio` backend) is not
  linted by viola's gate. P3 measures both.
- [inferred] Tokio's containment stays as it is: Tokio built only in `viola-mcp`, `viola-channel`'s `tokio` feature,
  and the test-only `viola-e2e` (`crates/*/Cargo.toml`); `deny-sync.toml` per sync crate and
  `scripts/channel-tokio-free.sh` keep their meaning, and `scripts/deny-probes.sh` still proves every ban fires.
- [inferred] `fuzz/Cargo.lock` holds no tokio (no `name = "tokio` line at the take-up), so the fuzz lockfile's own
  audit (`cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`) is not moved by this chunk.

## Boundaries
- No ignore-list widening, no `skip`, no `allow`, no new licence exception, no change to `deny.toml`'s policy
  (security-plan §Dependencies and CI: "Never widen an ignore, add a `skip`/`allow` or silence zizmor to reach green").
- No product behaviour change: no source file changes unless the bump forces one (a deprecation `-D warnings` reads,
  P3), and none of viola's wire contracts, exit codes, refusal words or log lines change.
- No other route entry's work: the Epoch 6 entries after this one are untouched.
- The spec masters that name tokio 1.53.1 — architecture's Stack row (Tokio 1.53.1 built only inside `mcp`) and
  security-plan Dependency Security's open-advisory line, which names this entry as its owner
  (`.andromeda/security-plan-detail/dependency-security.md:58`, the `deps-supply-chain` group) — are the wrap's
  amendments, not this chunk's edits.

## Freight folded
- CARRY (verbatim): chunk 2026-10-25-windows-endpoint-admission (its P5 review card's row 1,
  `.andromeda/runs/2026-10-25T21-08-26-phase/p5-card-1.md`, approved by the founder's own yes, live by dialog,
  `viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission/inputs/I6-relay-5.md.txt`; placed by the overseer's
  founder-delegated answer, `viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission/inputs/I2-relay-1.md.txt`): this
  entry owns the red read on every CI run since the 2026-10-20 wrap, `supply-chain` alone (ci#40247500584 at
  `e2bb59c8` the last), and security-plan Dependency Security's open-advisory line names it as its owner.

## CI since the last wrap (Setup 5a)
- `b1249fb1` red · checks 15/15 · wall 898 s · first-fail +24 s `supply-chain` · ci#40257114709 completed/failure.
- [inferred] That red intersects what this chunk builds: ci#40257114709, sha `b1249fb1a87bed38d167637c2d45e19508db5c3f`,
  job `supply-chain` (failure) — the chunk's own subject by the CARRY above. P3 closes it against that run's log
  (`gh api repos/Turbolet85/viola/actions/jobs/121361224603/logs`): its failure is RUSTSEC-2026-0386 alone and no other
  advisory, licence, ban or source finding stands behind it.

## Acceptance (to be planned at P4)
- The locked tokio is a patched version (`Cargo.lock`), the workspace pin matching it.
- `cargo deny check` at CI's pinned cargo-deny version reads no RUSTSEC-2026-0386 finding and no other failure, with
  the ignore list unchanged; the tokio bans (`deny-sync.toml` per sync crate, `scripts/channel-tokio-free.sh`) and
  `scripts/deny-probes.sh` still pass.
- The other gates hold: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --features fake-agent -- -D
  warnings`, the workspace tests, `scripts/rmcp-features.sh`, `scripts/release-check.sh`.
- CI's `supply-chain` job green on the chunk's push (the full row read at the wrap).
