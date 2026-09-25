# Scope — 2026-09-25-security-prerequisites

**Working entry (verbatim):** Security prerequisites — SQOS pipe-handle spike and pure-Rust SHA-256 pick, both recorded ahead of the channel client and bin/ writes; no unprotected-connect fallback

**Epoch:** Epoch 2 — Windows slice I: wrapper, events, ledger · **Version:** viola-0.1.0

## What this chunk builds

It settles the two **open questions** in the security-plan Decisions Log (`security-plan.md` §Decisions Log → Open questions,
the SQOS spike and the SHA-256 crate). Each blocks a later chunk under the Bootstrap `auth-scaffolding-baseline` rules
(`security-plan.md` ~:365-375): the `viola-channel` client needs the SQOS answer, and the `viola-state` `bin/` writer needs
the hash crate. Settling a question means measuring it at HEAD, leaving evidence in the repo, and having the Decisions Log
entry turned from an open question into a decision. Phase and implement never amend the spec; the chunk's wrap records it
from the evidence.

### 1. SQOS pipe-handle spike (Windows)
- The question as the plan states it (`security-plan.md` ~:609): does interprocess 2.4.4's
  `interprocess::os::windows::named_pipe::local_socket::Stream::try_from(OwnedHandle)` accept a handle that windows-sys
  0.61.2 `CreateFileW` opened with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED`?
- The answer is a **measurement**, not a reading. Open the handle that way against a test-owned, same-user pipe server,
  adopt it, and exchange bytes in both directions. The server side calls `ImpersonateNamedPipeClient` and reads the thread
  token's `TokenImpersonationLevel`: it must be `SecurityIdentification`, never `SecurityImpersonation` or higher. This is
  the shape of test-plan's "Windows client SQOS" negative (`test-plan.md` ~:1282), used here as the spike's pass
  criterion.
- **Fail path:** if the adoption refuses the handle, or the level is wrong, the chunk records a working replacement SQOS
  open instead, measured the same way. The Windows client chunk stays blocked until a working open is recorded.
- **No unprotected-connect fallback:** interprocess's default connect (`connect_without_waiting`, which opens without SQOS)
  is never an accepted outcome, a fallback or a test double for the client open. The evidence must show the protected open
  itself.
- Windows-only (`#[cfg(windows)]`). Unix has no SQOS; its server verification (`peer_creds`) belongs to the channel chunk.

### 2. Pure-Rust SHA-256 pick
- Pick the crate that computes the truncated SHA-256 `<hash>` (first 16 hex characters of the digest) of the exe bytes for
  `bin/<version>-<hash>/` and `plugin/<version>-<hash>/` (`security-plan.md` ~:265, :599, :611).
- The pick must pass what the repo already enforces:
  - the cargo-deny C-build ban (no `cc` or other C-building crate in its graph under the features chosen);
  - the permissive licence allowlist (`deny.toml` `[licenses] allow`);
  - the per-sync-crate tokio ban (`deny-sync.toml`), if it lands in a sync crate;
  - `cargo deny check` green, with `scripts/deny-probes.sh` still proving every ban fires.
- The pick is shown to compute SHA-256 correctly: a published known-answer vector and the 16-hex truncation.
- **Never FNV-1a or `DefaultHasher`** (`security-plan.md` ~:519).

### 3. Boundaries (not this chunk)
- Not the `viola-channel` crate, its listener (SDDL, `accept_remote(false)`), its client verb path, or the pid +
  start-time server verification (that is the Wrapper channel entry).
- Not the `bin/` / `plugin/` writer, the re-hash-before-reuse check, or strict-modes (the Instance state entry).
- The spike and the pick are evidence and a pinned dependency. They are not the production API.

## Inferred scope (not stated by the working entry — closed at P3 against research.md)
- [premise-corrected: measured at P3 — interprocess 2.4.4 pulls `doctest-file` 1.1.1 + `recvmsg` 1.0.0 (both `0BSD`),
  which the repo `deny.toml` rejects (`licenses FAILED`); windows-sys 0.61.2 links via `windows-link` 0.2.1, not
  `windows-targets`]
  - interprocess `=2.4.4` and windows-sys `=0.61.2` are pinned in `[workspace.dependencies]` and consumed as the root
    `viola` package's `[target.'cfg(windows)'.dev-dependencies]`. No product crate is created to hold them.
  - Their graph has no `cc` on any `[graph]` target.
  - `cargo deny check` stays green only once `deny.toml` disposes of the two 0BSD crates. How is P4's decision.
- The spike is a durable `#[cfg(windows)]` witness, `tests/channel_sqos_open.rs`. It pins the open the channel client will
  reuse, runs on the `windows-2025` `test` leg, and includes a discriminating no-SQOS control. (Verified lean.)
- Measured at P3 (research.md §Measured facts):
  - The spec's path passes: `CreateFileW(SQOS | IDENTIFICATION | OVERLAPPED)` → `Stream::try_from` → server reads
    `SecurityIdentification`. So no replacement open is needed.
  - `FILE_FLAG_OVERLAPPED` is load-bearing: a non-overlapped handle adopted by `try_from` hangs.
  - std `security_qos_flags` + `custom_flags(FILE_FLAG_OVERLAPPED)` is a measured safe-Rust equivalent. It is recorded
    for the channel chunk, not adopted here.
- The hash crate is RustCrypto `sha2 =0.11.0`, `default-features = false` (MIT OR Apache-2.0, rust-version 1.85). Its graph
  (cfg-if, cpufeatures, digest, block-buffer, crypto-common, hybrid-array, typenum, and libc on aarch64-apple) has no
  `cc`, passes the licences and checks on 1.96. It is consumed as a root `[dev-dependencies]` by a KAT test, so
  `cargo deny` actually sees it. (Verified.)
- **Licence (founder ruling 2026-09-25, relayed by the overseer; verified buildable at P3).** The MIT copyright-holder
  line is P4's open question. The repo is now PUBLIC and has no licence. The chunk licenses it **MIT OR Apache-2.0**:
  - `LICENSE-MIT` and `LICENSE-APACHE` at the repo root;
  - `license = "MIT OR Apache-2.0"` in the root `Cargo.toml` `[workspace.package]`, inherited by every workspace package
    (the root `viola` package and `crates/*`);
  - the same `license` in `fuzz/Cargo.toml`, which is its own workspace;
  - a README line stating the licence (no `README.md` exists at HEAD).

  `cargo deny check` must stay green: `deny.toml` allows MIT, Apache-2.0, Zlib and Unicode-3.0, and sets
  `private = { ignore = true }`. The same holds for `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`.
- **CI fold (overseer direction): closed, nothing to fold.** CI run 36133114541 on the wrap sha 874420d re-read at P3
  as 15/15 success; `lint (windows-2025)` completed `success` at 2026-09-25T12:16:33Z.
- **Standing test discipline (overseer direction; verified applicable).** Hypothesis, measured at /implement: the new
  code is test-target-only, so `run --mutants --in-diff` likely generates 0 mutants, and the remove-the-guard readings
  are the witnesses.
  - Every new guard test carries a remove-the-guard run: the guard is removed and the test is shown red.
  - The mutation gate holds 0 missed and 0 timeout.
  - The unviable-swamp rule applies: `unviable <= caught`, else red.
  - These are folded, not carried.
