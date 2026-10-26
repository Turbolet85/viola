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
- The bump is a manifest pin move as well as a lock move (verified at P3: `Cargo.toml:242` holds
  `tokio = { version = "=1.53.1", default-features = false }` in `[workspace.dependencies]`; in a scratch worktree the
  pin edit plus `cargo update -p tokio --precise 1.53.3` gave exit 0). The entry's "lock bump" names the outcome, not
  the only file that changes.
- The target version: the entry's floor is `>=1.53.3` and the workspace's pin form (`=`) stays (verified at P3:
  crates.io publishes 1.53.3 (2026-10-13), 1.54.0 yanked, 1.54.1 (rust-version 1.85); the 1.53.3 manifest declares
  rust-version 1.71 and a dependency set identical to 1.53.1's, `inputs#I3`). The exact pin is P4's lean.
- Companion crates `tokio-macros` 2.7.2, `tokio-stream` 0.1.19 and `tokio-util` 0.7.20 do not move, and no other
  dependency is bumped (verified at P3: the scratch worktree's `git diff Cargo.lock` is the `tokio` entry alone, its
  `version` and `checksum` lines).
- The deprecation the advisory names lands in the patched versions: "Starting in Tokio 1.51.6, 1.53.3, and 1.54.0,
  `AsyncFd::new`, `AsyncFd::with_interest`, `AsyncFd::try_new`, and `AsyncFd::try_with_interest` are deprecated in
  favor of the `unsafe` constructors" (`inputs#I2`). It reaches no viola code (verified at P3: `grep -rn AsyncFd`
  over `crates src tests` returns no line; the one user in the graph is interprocess 2.4.4's
  `src/os/unix/unnamed_pipe/tokio.rs`, a dependency viola's lint gate does not read; on the bumped scratch worktree
  `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` exits 0 with no `deprecated` line).
- Tokio's containment stays as it is: Tokio built only in `viola-mcp`, `viola-channel`'s `tokio` feature, and the
  test-only `viola-e2e` (verified at P3 on the bumped scratch worktree: the four `deny-sync.toml` sole-root runs over
  `scripts/sync-crates.txt` exit 0, `scripts/channel-tokio-free.sh` ends `channel-tokio-free: ok`,
  `scripts/deny-probes.sh` ends `deny-probes: 13/13 banned, control clean`).
- `fuzz/Cargo.lock` holds no tokio, so the fuzz lockfile's own audit is not moved by this chunk (verified at P3: no
  `name = "tokio` line in `fuzz/Cargo.lock`; `cargo deny --manifest-path fuzz/Cargo.toml --format json check
  advisories sources` exits 0, one `warning` — `advisory was not encountered`, the RUSTSEC-2017-0008 ignore, which
  that graph does not hold).

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
- That red intersects what this chunk builds: ci#40257114709, sha `b1249fb1a87bed38d167637c2d45e19508db5c3f`, job
  `supply-chain` (failure) — the chunk's own subject by the CARRY above (verified at P3 against that run's log, job
  121361224603: one `"severity":"error"` diagnostic, `"code":"unsound"`, `"id":"RUSTSEC-2026-0386"` on `tokio`, and four
  `duplicate` warnings; the job's last step run is `Dependency policy (cargo deny, all four families)`, then the upload).
- The red hides four steps (found and measured at P3): the job stops at its first deny step, so `Fuzz lockfile audit`,
  `Tokio ban per sync crate (sole root)`, `Ban probes` and `Workflow lint (zizmor)` have not run on CI on that run
  (its step list read from the job log, job 121361224603). Each was read locally on the bumped scratch worktree at
  the CI pins (cargo-deny 0.20.2, zizmor 1.30.1) and passes (the fuzz audit, the bans and probes above; zizmor `[]`);
  CI's own reading of them comes on the chunk's push.

## Acceptance (to be planned at P4)
- The locked tokio is a patched version (`Cargo.lock`), the workspace pin matching it.
- `cargo deny check` at CI's pinned cargo-deny version reads no RUSTSEC-2026-0386 finding and no other failure, with
  the ignore list unchanged; the tokio bans (`deny-sync.toml` per sync crate, `scripts/channel-tokio-free.sh`) and
  `scripts/deny-probes.sh` still pass.
- The other gates hold: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --features fake-agent -- -D
  warnings`, the workspace tests, `scripts/rmcp-features.sh`, `scripts/release-check.sh`.
- CI's `supply-chain` job green on the chunk's push (the full row read at the wrap).
