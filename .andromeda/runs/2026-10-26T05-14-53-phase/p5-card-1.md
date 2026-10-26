# P5 review card 1 — 2026-10-26-tokio-rustsec-2026-0386-lock-bump

**Chunk:** tokio RUSTSEC-2026-0386 lock bump · viola-0.1.0 · Epoch 6 — Security hardening · an ordinary chunk, no
forced sample (`inputs#I1`; the cascade drew `sample: no`)
**Plan:** `viola-0.1.0/chunks/2026-10-26-tokio-rustsec-2026-0386-lock-bump/plan.md` (337 lines, 153 of prose, 27 gate
entries, 10 acceptance criteria) · research `…/research.md` · scope `…/scope.md`

## Goal
Move tokio from 1.53.1 to 1.53.3, the 1.53 line's first patched release (`inputs#I2`), so `cargo deny check` stops
failing on RUSTSEC-2026-0386 and CI's `supply-chain` job reads green again. This closes the CARRY: `supply-chain`
alone has been red on every run since the 2026-10-20 wrap, last at ci#40257114709 on `b1249fb1`. No ignore is widened,
and RUSTSEC-2017-0008 stays the only ignore.

## Touchpoints
- `Cargo.toml:242` — the workspace pin `=1.53.1` → `=1.53.3`, with the `=` form and `default-features = false` kept
- `Cargo.lock` — the `tokio` entry's `version` and `checksum`, written by `cargo update -p tokio --precise 1.53.3`
- No source file, no crate manifest, no `deny.toml` / `deny-sync.toml` / lint or toolchain file

## Measured at P3 (scratch worktree of HEAD, removed after)
- The lock delta is the tokio entry alone (4 lines). tokio 1.53.3's dependencies are identical to 1.53.1's (`inputs#I3`).
- Every supply-chain step passes on the bumped lock at the CI pins: root deny (0 errors), the fuzz audit, the four
  sync-crate tokio bans, `channel-tokio-free: ok`, `deny-probes: 13/13`, zizmor `[]`. clippy `-D warnings` exits 0. No
  viola call site uses the deprecated `AsyncFd` constructors (the graph's one user is interprocess 2.4.4, a
  dependency).
- The red hides four steps: CI's `supply-chain` stops at its first deny step, so the fuzz audit, the ban loop, the probes
  and zizmor have not run on CI since 2026-10-20. This chunk's push is their first CI reading since then.
- The move carries tokio fixes that touch code viola runs: `DuplexStream` wakers, `current_thread` paused time, the
  Windows child-handle wait, the blocking-pool mutex (`inputs#I4`). Their witnesses are the Tokio-bearing suites in
  `pre-push` (Linux) and every OS's CI `test` row.

## Top acceptance criteria
1. `cargo deny check` exits 0, its output holds no `RUSTSEC-2026-0386`, and the ignore list is unchanged (the guard over
   `deny.toml`).
2. The pin reads `=1.53.3`, the lock resolves `1.53.3`, and the lock delta against `b1249fb1` is the tokio entry's 4 lines.
3. Tokio containment holds: the sole-root bans, `channel-tokio-free: ok`, `deny-probes: 13/13 banned, control clean`.
4. The fuzz audit, zizmor, `rmcp-features: server transport-io`, fmt, clippy on the host and the Windows target, and MSRV
   1.96 `cargo check` all pass; `run --unit` and `pre-push` read `"ok":true`; G2 and G4 pass on the smoke home.
5. The guard reads `guard: clean`: nothing outside `Cargo.toml` and `Cargo.lock` changed.
6. CI on the pushed HEAD reads `verdict: green` over the full row, `supply-chain` included, and the run id is named.

## Test commands (27 entries; baselines on the untouched tree)
- **Red by design, each proven to turn green by a must-pass control over a bumped scratch clone**
  (`.andromeda/runs/2026-10-26T05-14-53-phase/p5-controls.md`):
  - entry 6, the pin: 0 → 1
  - entry 7, the lock version: 1.53.1 → 1.53.3
  - entry 8, the lock delta: 0 → 4, and an extra moved line reads 6
  - entry 9, `cargo deny check`: advisories FAILED on RUSTSEC-2026-0386 → exit 0
  - entry 17, the guard: clean → clean, and an out-of-scope edit reads `outside 2 files`
- **Green now:**
  - the `/tmp` reading (25G free)
  - MSRV
  - the fuzz audit
  - the ban loop
  - `channel-tokio-free`
  - `deny-probes`
  - `rmcp-features`
  - zizmor
  - the smoke session
- **Seen green at the last wrap:** fmt, clippy, the Windows-target clippy, `run --unit`, G2, G4, `pre-push`
- **Operator pass:** hygiene, the guarded push, then `ci.py conclusion --sha HEAD --wait 2400` asserting
  `verdict: green` over the full row

## Verification matrix
No capability is claimed. The only cap that names the dependency policy is `v1-01`, the whole three-OS pipeline, and
this chunk proves only its advisories leg. It stays pooled.

## Expected amendments (wrap)
- architecture: the Stack row and Inherited Defaults move to tokio 1.53.3.
- security-plan: the `deps-supply-chain` open-advisory line is retired, and the `deps` tokio pin moves to `=1.53.3`.
- test-plan: §8 (twice) and §7 move to 1.53.3, and so does the §3 Bootstrap phases key file.
- The Tier-3 docs (`stack.md`, `services/viola-mcp.md`) are updated at the wrap commit.

## Notes for the reviewer
- **The version lean is `=1.53.3`.** It is decisive, not a fork: the same-minor patch, with identical dependencies.
  The other candidates were rejected:
  - `=1.54.1`: a new minor (rust-version 1.85; 1.54.0 is yanked)
  - the 1.51.6 band: a downgrade
  - a range pin
  - an ignore
- **No playbook escalate pattern applies, and no boundary is widened.**
- **P2:**
  - design, layouts and a11y have no domain coverage (3, under the warn band)
  - 1 task writer and 4 strong readers were re-spawned once each; all passed on retry
  - history read: arch, security and tests by index, obs whole

## Beyond the plan
none
