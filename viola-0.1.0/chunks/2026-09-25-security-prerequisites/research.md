# Codebase Research — 2026-09-25-security-prerequisites

## Scope
- **Depth:** moderate. The chunk adds no product code, but its two premises (the SQOS adoption and the dependency graph)
  were **measured** in a throwaway scratch project outside the repo, per the mechanism re-derivation rule.
- **Reads:** 14 (4 manifests, `deny.toml`, `deny-sync.toml`, `scripts/deny-probes.sh`, `.config/nextest.toml`, the
  `ci.yml` test/mutants jobs, `crates/viola-e2e/src/harness/run/coverage.rs:11`, 3 interprocess 2.4.4 source files, the
  cookbook head).
- **Globs/Greps:** 9.
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`: read in full, 1 Session Addition applied (`--in-diff` never regenerates
    mutants outside the diff, so the witness is the test's own PASS line).
  - `.claude/rules/testing.md`: read in full, 10 Session Additions applied (remove-the-guard run 2026-09-25; waits bounded
    below 20 s and `try_wait`-aware; no `sleep` sync).
- **Platform issues consulted** (filled at P5 check 9: the plan reads CI runs). Queries on `gh search issues --repo
  kotauskas/interprocess`:
  - `ReOpenFile`, `SQOS`, `from_raw_handle hang`, `overlapped hang`, `try_from handle` → 0 issues each.
  - `impersonation` → #4 "UNIX Socket/Win Pipe authentication support", CLOSED 2021-01-27. The fetched body asks for
    peer authentication (Windows impersonation "given proper flags was passed … from client"). It reports no defect in
    the handle-adoption path and nothing about SQOS or overlapped adoption.

  No runner-only bullet stands: the CI fold closed green (§Scope premise closure).

## Files inspected
- `Cargo.toml` (full):
  - `[workspace.package]` has `version/edition/rust-version/publish`, and no `license` (`Cargo.toml@874420d`).
  - The root `viola` package inherits each key with `.workspace = true`.
  - `[dev-dependencies]` holds jsonschema, rstest, serde_json and tempfile. There are no target-gated tables yet.
  - `[workspace.lints]` has no `unsafe_code` lint, so FFI in a test target is allowed.
- `crates/viola-core/Cargo.toml`, `crates/viola-e2e/Cargo.toml` (full): both inherit `publish.workspace = true`, and
  neither has `license`.
- `fuzz/Cargo.toml` (full): its own `[workspace]`, `publish = false`, and no `license`. It cannot inherit from the
  root `[workspace.package]`.
- `deny.toml` (full):
  - `[graph] targets` = windows-msvc, aarch64-apple-darwin, linux-gnu, and no `exclude-dev`, so dev-dependencies are
    checked.
  - `[licenses] allow = ["MIT","Apache-2.0","Zlib","Unicode-3.0"]`, `private = { ignore = true }`.
  - There is no `[[licenses.exceptions]]`.
- `deny-sync.toml` (full): `exclude-dev = true`, tokio ban. `scripts/sync-crates.txt` = `viola-core` only.
- `scripts/deny-probes.sh`: 13 probes (`sed -n 19,33p scripts/deny-probes.sh | grep -c '|deny'` = 13). It prints
  `deny-probes: 13/13 banned, control clean` on success (line 83).
- `.config/nextest.toml` (full): `ci` profile `retries = 0`, slow-timeout 30 s × 4. The `mutants` profile kills at
  5 s × 2.
- `.github/workflows/ci.yml`:
  - `test` runs `agent-run run --coverage` on `[windows-2025, macos-latest, ubuntu-latest]`.
  - `mutants` runs `run --mutants --leg` on `[ubuntu-latest, windows-2025]`, then `gate --mutants-legs`.
- `crates/viola-e2e/src/harness/run/coverage.rs:11`: `COVERAGE_IGNORE` = `(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])`.
  Root `tests/*.rs` files are NOT ignored, so a new test file's own functions count toward the per-OS function floor
  (95) on the OS that compiles them.
- interprocess 2.4.4 source (`D:/dev/rust/cargo/registry/src/index.crates.io-*/interprocess-2.4.4/src/os/windows/`):
  - `named_pipe/local_socket/stream.rs:21`: `type StreamImpl = DuplexPipeStream<Bytes>`.
  - `stream.rs:111` `impl TryFrom<OwnedHandle> for Stream` delegates to
    `named_pipe/stream/impl/handle.rs:68` `impl TryFrom<OwnedHandle> for PipeStream<Rm, Sm>`. That impl:
    - checks `get_flags`;
    - **re-opens the handle through `np_wrappers::reopen_overlapped`** (`named_pipe/c_wrappers.rs:176-182`:
      `ReOpenFile(h, access, share, FILE_FLAG_OVERLAPPED)`, which passes **no SQOS flags**);
    - keeps the original handle if the re-open fails.
  - interprocess's own client open is `named_pipe/c_wrappers.rs:152-170` `connect_without_waiting`, a `CreateFileW` with
    only `FILE_FLAG_OVERLAPPED`: no SQOS, which confirms the spec's warning.

## Measured facts (scratch probe, not repo code)
Probe: `{scratchpad}/p3probe/`, a standalone Cargo project with an empty `[workspace]` and its own target dirs. Each case
creates a unique `\\.\pipe\p3probe-<pid>-<case>` server (`CreateNamedPipeW`, byte mode, 1 instance). The server:
1. `ConnectNamedPipe`, then `ReadFile` 4 bytes;
2. `ImpersonateNamedPipeClient` → `OpenThreadToken(TOKEN_QUERY, OpenAsSelf=TRUE)` →
   `GetTokenInformation(TokenImpersonationLevel)` → `RevertToSelf`;
3. writes `pong`.

The client writes `ping` and reads `pong`. Each case runs in its own process under `timeout 20`.

The probe ran on toolchains `1.98.1-x86_64-pc-windows-msvc` (the repo pin, CI's target) and stable-gnu, with identical
results. Case A ran 3× with identical results.

| case | client open | adopted via `Stream::try_from` | server level | bytes both ways |
|---|---|---|---|---|
| A | windows-sys `CreateFileW(SECURITY_SQOS_PRESENT \| SECURITY_IDENTIFICATION \| FILE_FLAG_OVERLAPPED)` | yes | **1 = SecurityIdentification** | yes |
| B | same, **without** `FILE_FLAG_OVERLAPPED` | yes | — (**hangs**, killed at 20 s) | — |
| C | std `OpenOptions::security_qos_flags(SECURITY_IDENTIFICATION)`, no overlapped | yes | — (**hangs**, 20 s) | — |
| D | std `OpenOptions` + `custom_flags(FILE_FLAG_OVERLAPPED)` + `security_qos_flags(SECURITY_IDENTIFICATION)` | yes | **1** | yes |
| E | case A's flags minus overlapped, used as a plain `std::fs::File` (no adoption) | no | 1 | yes |
| F | `CreateFileW(FILE_FLAG_OVERLAPPED)` only, no SQOS | yes | **2 = SecurityImpersonation** | yes |
| G | interprocess default `LocalSocketStream::connect` | n/a | **2** | yes |

- **The spec's spike passes:** case A yields `SecurityIdentification` after adoption. The `ReOpenFile` inside `try_from`
  does not lose the level: the level is fixed by the client's connect-time QoS, and the re-open keeps the same
  connection.
- **`FILE_FLAG_OVERLAPPED` is load-bearing:** a non-overlapped handle adopted by `try_from` hangs the exchange (cases B
  and C). The measured fact is the hang. Its cause is unmeasured (hypothesis: the re-open and the sync-over-overlapped
  I/O interplay).
- **The measurement discriminates:** F and G read level 2. A test asserting `== SecurityIdentification` goes red when the
  SQOS flags are removed, which is exactly the remove-the-guard reading.
- **A safe-Rust equivalent exists** (case D, no `unsafe` on the client side). The spec names the windows-sys path, and it
  passes, so D is recorded as a fact for the channel chunk, not as a replacement.

### Dependency graph (same probe, `cargo tree --target <triple> -e normal,build`)
- **windows-msvc:**
  - interprocess 2.4.4 → doctest-file 1.1.1 (proc-macro), recvmsg 1.0.0, widestring 1.2.1, windows-sys 0.61.2 →
    windows-link 0.2.1.
  - sha2 0.11.0 → cfg-if 1.0.5, cpufeatures 0.3.1, digest 0.11.3 → block-buffer 0.12.1, crypto-common 0.2.2,
    hybrid-array 0.4.15, typenum 1.20.1.
- **linux-gnu:** sha2's tree as above.
- **aarch64-apple-darwin:** sha2's tree plus `cpufeatures → libc 0.2.189`.
- **No `cc` on any triple.** windows-sys 0.61.2 links through `windows-link`, not `windows-targets` import libs.

### cargo-deny 0.20.2 with the repo's `deny.toml` on that graph
`cargo deny --manifest-path p3probe/Cargo.toml --config D:/dev/projects/viola/deny.toml check` →
**`advisories ok, bans ok, licenses FAILED, sources ok`** (rc 4). The two rejections are `doctest-file 1.1.1` and
`recvmsg 1.0.0`, both `license = "0BSD"`, both through interprocess 2.4.4. interprocess itself is `0BSD OR Apache-2.0`,
which passes via Apache-2.0.

The same run with two `[[licenses.exceptions]]` (`crate = "doctest-file"` and `crate = "recvmsg"`, `allow = ["0BSD"]`)
→ **all four ok** (rc 0). Each scratch output is `p3probe/deny.out` / `deny-exc.out`.

### Other measured facts
- **sha2 0.11.0:** `license = "MIT OR Apache-2.0"`, `rust-version = 1.85`. Default features are `[alloc, oid]`; with
  `default-features = false`, `cargo check` passes under `RUSTUP_TOOLCHAIN=1.96`, the MSRV floor.
- **interprocess 2.4.4:** `rust-version = 1.75`, `default = []`. `tokio` is an opt-in feature.

## Graph impact
- **crate_edges** (`rust` plane, `tree-query-2026-09-25-security-prerequisites.json`): `viola → viola-core` is the root
  package's only edge. No symbol in scope exists at HEAD.
  - Derivation: `grep -rln -E 'interprocess|windows_sys|windows-sys|sha2|Sha256|SECURITY_SQOS|CreateFileW' --include=*.rs --include=Cargo.toml src crates tests fuzz Cargo.toml`
    returned 0 files.
  - So there is no caller set to thread, and the change is additive (new test targets plus manifest keys).

## Patterns detected
- **Target-gated dependency tables:** none exist yet. This chunk introduces the first,
  `[target.'cfg(windows)'.dev-dependencies]`, the shape arch §Crate dependency direction names for windows-sys.
- **Exact pins in `[workspace.dependencies]`, inherited with `.workspace = true`** (`Cargo.toml` e.g.
  `rstest = { version = "=0.27.0", default-features = false }`). Features are trimmed at the workspace entry, with a
  comment when the default features pull something banned (the `jsonschema` comment).
- **Probe-proven gates:** `scripts/deny-probes.sh` proves every ban fires with a planted crate. A licence exception
  needs no new probe, because the licence family already fails loudly (measured above).
- **Root integration tests without `required-features`** are auto-discovered, since `autotests` is not disabled. Root
  suites use `{cli,hook,tui,channel,chaos,contract}_<topic>.rs` (testing.md §Framework).

## Conventions to follow
- **Naming:** `<subject>_<condition>_<expected>`, with readable rstest case labels (testing.md §Naming).
- **No sleep sync:**
  - the server thread signals readiness through the pipe's own creation order: the server instance exists before the
    client opens it (probe pattern);
  - the joins are bounded;
  - no wait may reach 20 s (testing.md Session Additions 2026-09-24).
- **Print bans:** the root package inherits `print_stdout`/`print_stderr`/`dbg_macro = "deny"`, so the tests assert and
  never print (`Cargo.toml` `[workspace.lints.clippy]`).
- **Oracles are literals:** the SHA-256 KAT digests are written out, never computed by a second code path (testing.md
  §What to assert).
- **Pipe names:** outside the `viola-<h12>` namespace (arch §Occupied Resources), unique per test (pid + test label), and
  never `try_overwrite`.

## New files to create
- `tests/channel_sqos_open.rs`: `#![cfg(windows)]` integration test that pins the SQOS open recipe (probe case A), with:
  - a same-user test-owned pipe server that measures `TokenImpersonationLevel`;
  - a round-trip byte exchange;
  - a discriminating control case: no SQOS flags → `SecurityImpersonation`.
- `tests/contract_content_hash.rs`: rstest KAT table over `sha2::Sha256` with published FIPS 180-2 vectors as literal
  digests, plus the 16-hex truncation (the first 8 digest bytes as lowercase hex).
- `LICENSE-MIT`, `LICENSE-APACHE`: repo root, the standard texts.
- `README.md`: repo root, with at least a one-line description and the licence line (dual MIT OR Apache-2.0, pointing
  at both files).

## Files to modify
- `Cargo.toml`:
  - `[workspace.package]` gains `license = "MIT OR Apache-2.0"`, and the root `[package]` gains
    `license.workspace = true`;
  - `[workspace.dependencies]` gains `interprocess = "=2.4.4"`, `windows-sys = { version = "=0.61.2", features = [...] }`
    (Win32_Foundation, Win32_Security, Win32_Storage_FileSystem, Win32_System_Pipes, Win32_System_Threading) and
    `sha2 = { version = "=0.11.0", default-features = false }`;
  - the root package gains `[target.'cfg(windows)'.dev-dependencies]` interprocess + windows-sys and `[dev-dependencies]`
    sha2.
- `crates/viola-core/Cargo.toml`, `crates/viola-e2e/Cargo.toml`: `license.workspace = true`.
- `fuzz/Cargo.toml`: `license = "MIT OR Apache-2.0"`.
- `deny.toml`: `[licenses]` gains the 0BSD disposition. **The plan decides the shape** (open question 1).
- `Cargo.lock`: regenerated by cargo with the new dev-dependency graph. `fuzz/Cargo.lock` is unchanged, because the
  fuzz package gains no dependency.
- **Companion sweep:**
  - `license`: 0 hits in any manifest (`grep -rn '^license' Cargo.toml crates/*/Cargo.toml fuzz/Cargo.toml`), so there
    are 4 manifests to change and no other sites.
  - `contract_lints.rs` asserts `[lints] workspace = true` inheritance, not package keys, so it needs no change: its
    assertion is on `[lints]`, and `license` is outside it.

## Scope premise closure
- `[inferred]` interprocess `=2.4.4` + windows-sys `=0.61.2` enter `[workspace.dependencies]` → **premise-corrected**, on two
  counts:
  1. They land as the root package's `cfg(windows)` **dev**-dependencies (no product crate exists to take them, per arch
     [Module Boundaries]).
  2. `cargo deny check` does **not** stay green unchanged: interprocess brings two `0BSD` crates that the allowlist
     rejects (measured). The mechanism "windows-sys links import libs through windows-targets" is also corrected: the
     0.61.2 tree is `windows-link` 0.2.1.
- `[inferred]` spike location/durability → **VERIFIED as the lean:** a durable `tests/channel_sqos_open.rs` running on the
  `windows-2025` `test` leg (`ci.yml` matrix).
- `[inferred]` hypothesis: the std `security_qos_flags` + `custom_flags(FILE_FLAG_OVERLAPPED)` open → **VERIFIED as an
  equivalent** (case D, level 1). Corrected detail: it works **only** with the overlapped flag (case C hangs). It is not
  needed as a replacement, since case A passes.
- `[inferred]` candidate sha2 → **VERIFIED:** sha2 `=0.11.0` with `default-features = false`. Its graph has no `cc` on
  the three `[graph]` targets, its licences pass, and it checks on 1.96.
- `[inferred]` licence fold → **VERIFIED as buildable:**
  - no README or LICENSE is tracked (`git ls-files | grep -i -E '^(readme|license|copying)'` → none);
  - 3 workspace manifests + `fuzz/Cargo.toml` lack `license`;
  - `private = { ignore = true }` plus `publish = false` keep own-crate licences out of the check.

  The MIT copyright holder line is an open question.
- `[inferred]` CI fold → **VERIFIED closed:** run 36133114541 on 874420d re-read at P3 as 15/15 success
  (`lint (windows-2025)` completed `success` at 2026-09-25T12:16:33Z). Nothing to fold.
- `[inferred]` standing test discipline → **VERIFIED applicable.** Hypothesis, to be measured at /implement: the new code
  is test-target-only (`tests/*.rs`), so `run --mutants --in-diff` likely generates 0 mutants. The remove-the-guard
  readings are the witnesses:
  - with the SQOS flags removed, the level reads 2 and the test goes red;
  - with a KAT digest altered, the test goes red.

## Open questions
1. How should the root graph accept 0BSD (`doctest-file`, `recvmsg`, via interprocess)? As a per-crate
   `[[licenses.exceptions]]` (measured green), as `"0BSD"` added to `allow`, or with interprocess deferred? → blocks:
   plan-decision.
2. The MIT copyright holder line in `LICENSE-MIT` (git author and GitHub owner = `Turbolet85`). → blocks:
   plan-decision.
