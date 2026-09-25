# Report — 2026-09-25-security-prerequisites

**Chunk:** security prerequisites. It covers:
- the SQOS pipe-handle spike (CreateFileW SQOS + `Stream::try_from`, Identification level measured, no unprotected-connect
  fallback);
- the pure-Rust SHA-256 pick (C-build ban, 16-hex KAT).

Both are recorded ahead of the channel client and the `bin/` writes. The licence (MIT OR Apache-2.0) is folded in.
**Date:** 2026-09-25T13:15:00Z
**Commits:** since `last_wrap` 2026-09-25T11:43:30Z, `8e25ca7 chore(2026-09-25-security-prerequisites): operator pre-CI commit`
(parent `874420d`, the previous chunk's wrap commit). The operator pass ran, so the chunk basis is `874420d` → the working
tree.

## Changes (structured — detectors read this)
- **Files:** `git diff --name-only 874420d`, source and doc files only; run trails excluded:
  - `Cargo.toml`, `Cargo.lock`, `crates/viola-core/Cargo.toml`, `crates/viola-e2e/Cargo.toml`, `fuzz/Cargo.toml`;
  - `deny.toml`;
  - `crates/viola-e2e/src/harness/run/mutants.rs`;
  - new `tests/channel_sqos_open.rs`, new `tests/contract_content_hash.rs`;
  - new `LICENSE-MIT`, new `LICENSE-APACHE`, new `README.md`;
  - `viola-0.1.0/verification-matrix.json`, `viola-0.1.0/working-route.md`, `.andromeda/master-route.md` (the phase
    promotion);
  - the chunk folder (scope, research, plan, evidence).
- **Symbols / APIs:**
  - `crates/viola-e2e/src/harness/run/mutants.rs` (the harness, test-only crate `viola-e2e`):
    - new `pub fn rust_paths(diff: &str) -> Vec<&str>` (each `.rs` path the diff names, once, in diff order);
    - new `pub fn test_target(path: &str) -> bool` (true for `tests/`, `benches/`, `examples/` at the root or under
      `crates/<member>/`);
    - new private `fn unmutated(ws, leg, verdict, diff) -> (Suite, Value)`, which replaces and absorbs the removed private
      `fn no_rust_delta(diff)`. Its sole caller was `mutants()`, measured by `grep -n no_rust_delta` over
      `crates/viola-e2e/src` = the definition + 1 call @874420d.
  - `mutants()` gains a second pre-cargo arm. When every `.rs` path is a `test_target`, the run returns
    `{"tested":0,"verdict":"test-only-rust-delta","diff":"target/agent-run/chunk.diff","files":N,"rust_files":[…]}`,
    writes the leg verdict file with that verdict, and never invokes `cargo build` or `cargo mutants`.
  - A mixed diff, or any other Rust delta, is unchanged: `counted`, and red `outcomes-missing` without a fresh
    `outcomes.json`.
  - No product (`src/`, `crates/viola-core`) symbol changed.
  - New test fns:
    - `tests/channel_sqos_open.rs`: `sqos_identification_open_adopted_reads_identification`,
      `sqos_flags_absent_open_reads_impersonation`;
    - `tests/contract_content_hash.rs`: `sha256_published_vector_matches_digest` (3 cases),
      `sha256_truncated_to_eight_bytes_yields_sixteen_hex` (2 cases);
    - `mutants.rs` tests: `rust_paths_lists_each_rust_path_once`, `test_target_holds_only_test_bench_and_example_dirs`,
      `run_mutants_test_only_delta_passes_by_name_without_running_cargo`,
      `run_mutants_mixed_src_and_test_delta_without_outcomes_stays_outcomes_missing`.
  - The test pipe name is `\\.\pipe\viola-test-sqos-<pid>-<label>`. It is test-only and outside the `viola-<h12>`
    endpoint namespace.
- **Crates / modules:** none added or removed. `viola-channel` and `viola-state` were deliberately NOT created: they
  belong to their first consumers.
- **Dependencies** (all exact pins in root `[workspace.dependencies]`):
  - `interprocess = "=2.4.4"`, default features only (`[]`, never `tokio`);
  - `windows-sys = "=0.61.2"`, features `Win32_Foundation`, `Win32_Security`, `Win32_Storage_FileSystem`,
    `Win32_System_Pipes`, `Win32_System_Threading`;
  - `sha2 = "=0.11.0"`, `default-features = false`.

  Consumers: the root `viola` package only. `sha2` in `[dev-dependencies]`; interprocess and windows-sys in a new
  `[target.'cfg(windows)'.dev-dependencies]` table. No product crate links any of them.

  Resolved transitive crates, measured with `cargo tree` on the three `[graph]` triples:
  - under interprocess: `doctest-file 1.1.1` (0BSD), `recvmsg 1.0.0` (0BSD), `widestring 1.2.1`;
  - under windows-sys: `windows-link 0.2.1`. That is not `windows-targets`: the security extract's "via windows-targets"
    premise is corrected;
  - under sha2: `cfg-if 1.0.5`, `cpufeatures 0.3.1` (+ `libc 0.2.189` on aarch64-apple), `digest 0.11.3`,
    `block-buffer 0.12.1`, `crypto-common 0.2.2`, `hybrid-array 0.4.15`, `typenum 1.20.1`.
  - No `cc` on any triple.
  - sha2 is `MIT OR Apache-2.0`, `rust-version 1.85`. interprocess is `0BSD OR Apache-2.0`, `rust-version 1.75`.
- **Schema / config:**
  - `deny.toml` `[licenses]` gains two `[[licenses.exceptions]]`, `crate = "doctest-file"` and `crate = "recvmsg"`, each
    `allow = ["0BSD"]`, with a comment. `allow = ["MIT", "Apache-2.0", "Zlib", "Unicode-3.0"]` is unchanged. There is no
    new ignore, skip or ban change.
  - Licence metadata, per the founder's ruling of 2026-09-25 relayed by the overseer:
    - `license = "MIT OR Apache-2.0"` in root `[workspace.package]`;
    - `license.workspace = true` in `viola`, `viola-core` and `viola-e2e`;
    - `license = "MIT OR Apache-2.0"` literal in `fuzz/Cargo.toml` (its own workspace);
    - `publish = false` unchanged everywhere.
  - `LICENSE-MIT` (`Copyright (c) 2026 Turbolet85`, founder ruling at P4) and `LICENSE-APACHE` (the standard Apache 2.0
    text, byte-copied from the registry's rust-lang copy, sha256 prefix `a60eea81…`). `README.md` has a `## License`
    section naming both.
- **Spec-master edits:** none by /phase or /implement. All go through this wrap's P2.
- **Counts / qualifiers moved:**
  - The `run --mutants` `mutants.verdict` closed set goes from 2 to 3 values: `counted`, `no-rust-delta`,
    `test-only-rust-delta`. Stated at test-plan.md:566, :663, :1780 (the `grep -n no-rust-delta .andromeda/*.md` hits in
    test-plan).
  - The workspace licence goes from none to `MIT OR Apache-2.0`.
  - Root integration test files: +2 (`channel_*`, `contract_*`). The arch tree lists `tests/` generically, so no count
    there moves.
  - `deny-probes` stays `13/13` and orphans stay `5/5`: measured unchanged by gates 15 and 23.
- **Dev-tool versions:** none. cargo-deny re-read at 0.20.2, cargo-mutants at 27.1.0, toolchains 1.98.1-msvc / 1.96.
- **Harness / gate surface:**
  - `viola-harness run --mutants` gains the `test-only-rust-delta` verdict (above): the verdict JSON shape gains a
    `rust_files` array for that verdict only. `mutants-verdict-<leg>.json` can carry `"verdict":"test-only-rust-delta"`
    with `"mutants":[]`.
  - `gate --mutants-legs` is unchanged: the union needs only each leg file to exist and never reads the verdict value
    (`crates/viola-e2e/src/harness/gate.rs:117-150`).
  - No CI workflow change.
- **Cross-project / external claims:**
  - **CI run 36138441784** on sha `8e25ca70f6232eb3e55bf4710d86814fb2c9e865` (the pre-CI commit): 15/15 `success`,
    re-read by the overseer too. The windows `test` leg PASSed both `channel_sqos_open` tests and all 5
    `contract_content_hash` cases (job 108082084137). Both mutants legs read `counted`, `9 mutants tested … 9 caught`.
  - CI run 36133114541 on `874420d` (Setup read): 15/15 success, so nothing was folded.
  - Measured in a scratch probe outside the repo (research.md §Measured facts), on 1.98.1-msvc and stable-gnu:
    - `CreateFileW(SQOS|IDENTIFICATION|OVERLAPPED)` → `Stream::try_from` → server level 1;
    - without OVERLAPPED, the exchange hangs;
    - std `OpenOptions::security_qos_flags` + `custom_flags(OVERLAPPED)` → level 1;
    - no SQOS, or interprocess's default connect → level 2.
  - cargo-mutants 27.1.0 `--list-files` over a package with `src/`, `tests/`, `benches/` and `examples/` files lists
    `src/lib.rs` only.
  - interprocess 2.4.4 source: `local_socket::Stream::try_from` → `PipeStream::try_from`, which re-opens the handle via
    `ReOpenFile(…, FILE_FLAG_OVERLAPPED)` with no SQOS flags (`named_pipe/c_wrappers.rs:176-182`). The measured level
    shows the re-open keeps the connect-time level.
- **Reverted / negative API facts:** none.
- **Insufficient fixes:** none.
- **Spec claims disproved by measurement:**
  1. The plan (step 10, the mutation acceptance) forecast a green `counted` verdict for a tests-only Rust delta. The harness
     and the operator ruling at test-plan.md:1790 ("a diff WITH .rs files and no outcomes stays red") made it red
     `outcomes-missing`. Measured at gate run 1 (`.andromeda/runs/2026-09-25T12-41-13-implement/gate-run-1.txt`, entry 24).
     Resolved by the operator's fold (option A): the harness verdict above. The test-plan wording is owed (Expected
     amendments).
  2. Plan step 4's NIST 448-bit message literal was mistyped (`…jklmmnopnopq`). The published message
     `abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq` gives `248d6a61…` (verified with Python `hashlib`). No
     master states it, so this is a plan-only fact.
  3. The security-plan open question's implied premise that the spike's pass is still undetermined: the spike **passed**,
     so no replacement open is needed. The Decisions Log is owed.
- **Expected amendments (from plan):**
  - security-plan §Security Decisions Log: SQOS + SHA-256 decisions. **Carried**: Dependencies, Cross-project claims and
    Spec claims #3. Sites: `grep -n -E "Open questions|Spike: does|SHA-256 crate:" .andromeda/security-plan.md` →
    608/609/611; the dependent lines 205, 373, 375 name the open spike / open pick.
  - security-plan §Dependency Security: the per-crate 0BSD exceptions. **Carried**: Schema / config. Sites:
    `grep -n -i -E "0BSD|licen" .andromeda/security-plan.md` → 134, 304, 308 (0 hits for `0BSD`).
  - architecture §Infrastructure Patterns → Build system (Dependency policy) + §Stack and Technologies (`sha2` for the
    `<hash>`). **Carried**: Dependencies and Schema / config. Sites: `grep -n -i licen .andromeda/architecture.md` → 36,
    409, 441; `grep -n sha2 .andromeda/*.md` → 0 hits.
  - architecture §Infrastructure Patterns → Project directory structure: the tree gains `LICENSE-MIT`, `LICENSE-APACHE`,
    `README.md`. **Carried**: Files. Site: `architecture.md` tree 436-470 lists no root README or LICENSE; `tests/` is
    listed generically, so the two new test files need no tree line.
  - (Overseer, wrap P2) architecture project metadata: the licence wherever arch names it. **Carried**: Schema / config.
    Sites: `grep -n -i -E "publishab|workspace\.package|rust-version|publish = false" .andromeda/architecture.md` → 13,
    350, 351, 403, 418, 589.
  - test-plan §6 Security control negatives → Windows client SQOS: the recipe is pinned by `tests/channel_sqos_open.rs`.
    **Carried**: Symbols. Site: test-plan.md:1282.
  - (Added by the operator fold) test-plan §10 Mutation gate + §3 `run` step 4 classification + the closed value list:
    `test-only-rust-delta`, refining the ruling at :1790. **Carried**: Harness / gate surface, Counts moved, Spec claims
    #1. Sites: `grep -n no-rust-delta .andromeda/test-plan.md` → 556, 566, 663, 1780, 1781, 1790; §10 Mutation gate at
    1496.
- **Coverage of new surfaces:**
  - `tests/channel_sqos_open.rs` (test-only Windows FFI) → validation n/a · instrumentation n/a (tests assert, no print) ·
    PII n/a · tests integ (Windows host + CI windows-2025) · a11y n/a · tokens n/a.
  - `tests/contract_content_hash.rs` → validation n/a · instrumentation n/a · PII n/a · tests integ (3 OS) · a11y n/a ·
    tokens n/a.
  - `viola-harness run --mutants` `test-only-rust-delta` arm → validation (path classifier, unit-tested with 6 positive
    and 8 negative literals)✓ · instrumentation n/a (the harness prints one JSON document by contract) · PII n/a (repo-relative
    paths only) · tests unit + integ (stand-in runner) + mutation (9/9 caught on both CI legs) · a11y n/a · tokens n/a.

## Deviations from intent
- **Scope widened on the operator's word (option A)** to `crates/viola-e2e/src/harness/run/mutants.rs` and its inline
  tests, not in the plan's modify-set. Justification: the plan's mutation acceptance could not pass under the standing
  ruling. The operator chose to fold the fix, with 4 conditions, all met:
  1. only-test-target diffs; benches/ and examples/ included by measurement;
  2. the mixed-diff red kept, as a literal-oracle test;
  3. the new code's own mutation reading: counted 9/9 caught, 0 missed, 0 timeout, 0 unviable, locally and on both CI
     legs;
  4. the test-plan §10 amendment is routed here.
- **KAT message corrected** from the plan's typo to the published NIST message (Spec claims #2).
- **Test server design:** it uses `FlushFileBuffers` before closing (instead of the probe's drain read), so the reply
  reaches the client deterministically. This is within plan step 3.
- **`LICENSE-APACHE` sourced by byte-copy** from the registry (two independent rust-lang copies identical), not retyped.
- **v1-43 method** refined from `manual` to `integration`, with its acceptance concretised at the phase P5 review
  (operator-approved).

## Decisions & corrections
- Founder ruling (2026-09-25, via overseer): the public repo is licensed **MIT OR Apache-2.0**, and the MIT holder is
  `Turbolet85`.
- Operator P4 ruling: interprocess's 0BSD transitive crates are admitted by **per-crate `[[licenses.exceptions]]`**, the
  narrowest widening, never by adding `0BSD` to `allow`.
- Operator implement ruling (option A): a Rust delta confined to test/bench/example targets passes the mutation gate only
  with the explicit `test-only-rust-delta` verdict. A mixed diff with no outcomes stays `outcomes-missing`. Fold, do not
  carry.
- Standing discipline applied: every guard test carried its remove-the-guard run, 5 readings (a–e), all red, all restored
  (`evidence/remove-the-guard.md`).
- **Sweep / measurement hazards found:**
  - cargo-mutants `--in-diff` over a tests-only Rust diff prints `INFO No mutants to filter` and writes no `outcomes.json`.
    It is not an error exit, so only the harness's `outcomes-missing` arm catches it.
  - An rstest `#[case]` line reflowed by the rustfmt PostToolUse hook breaks an anchored Edit on the pre-format text: a
    one-shot mutation edit silently missed, and the run read green on the unmodified file. Re-Read after every hook
    reformat before a remove-the-guard edit.
  - The scratch probe first built on the host default toolchain (stable-gnu), not the repo pin: re-run on
    1.98.1-msvc.
  - `cargo info` downloads go to the non-default `CARGO_HOME` `D:/dev/rust/cargo`, not `~/.cargo`.
  - A non-overlapped SQOS handle adopted by interprocess hangs indefinitely: tests must never rely on a bounded wait for
    this, and the nextest ci profile's 120 s termination is the backstop (measured in run b).

## Outcome
**Acceptance criteria, re-asserted against the diff:**
- **SQOS witness.** Met on this host (7/7) and on CI windows-2025 (run 36138441784): Identification for the recipe,
  Impersonation for the control. The witness has no default connect: the grep entry reads 0.
- **sha2.** Pinned once. `cargo tree -i sha2` resolves `sha2 v0.11.0`. The KAT and truncation pass on 3 OS.
- **cargo deny.** `advisories ok, bans ok, licenses ok, sources ok`. The allowlist line is unchanged, and there are
  exactly 2 exceptions. The fuzz audit, the deny-sync viola-core ban and `deny-probes: 13/13 banned, control clean` all
  hold.
- **Dependency placement.** interprocess and windows-sys are pinned once and consumed only as cfg(windows)
  dev-dependencies, with no tokio feature. No `viola-channel` or `viola-state` crate was created. Orphans `5/5 targets
  clean`. Met.
- **MSRV 1.96.** `cargo check --workspace --all-targets` passes, and CI `msrv` succeeded. Met.
- **Licence.** Met: `['MIT OR Apache-2.0']` for the root workspace and for `viola-fuzz`; `LICENSE-MIT` holder line
  present; `LICENSE-APACHE` text present; README names both; `publish = false` unchanged.
- **Mutation.** Met: the windows leg is `counted`, 9 caught, 0 missed, 0 timeout, 0 unviable, and both CI legs plus the
  union are `success`.
- **fmt / clippy.** Met: both green; no print, `dbg!` or new print-lint allow.

**Gates** (implement's final run, `gate-run-4.txt`) are all green:
- `cargo fmt --all --check`, `cargo clippy …`, `run --unit`;
- `run --integration --filter 'binary(channel_sqos_open) | binary(contract_content_hash)'` (7 passed);
- `run --integration`;
- the SQOS-flags grep (last line 1), the no-connect grep (exit 1, last line 0), the allow-line grep (1), the exceptions
  grep (2);
- `cargo deny check`, `cargo tree -i sha2`, `cargo tree -i interprocess`, the fuzz `cargo deny`, the `deny-sync` viola-core
  ban, `deny-probes.sh`;
- `git diff --exit-code --quiet -- fuzz/Cargo.lock`, both `cargo metadata` licence reads, the LICENSE-MIT, LICENSE-APACHE
  and README greps;
- the MSRV `cargo check`, `orphans-check.sh`, `AGENT_RUN_CHUNK_BASE=874420d… run --mutants` (counted, 9/9).

The operator entries:
- the clean-tree-guarded push: fired on the operator's word, 874420d..8e25ca7 fast-forward;
- the two check-runs reads: `success` and `success,success,success`, recorded in `evidence/operator-pass.md`.

Smoke: skipped. There was no boot-path change; the one harness path the fold touched (`run --mutants`) ran end to end as a
gate.

**Outcome basis:** implement's P4 report as given in this session, plus the operator's option-A directive (the scope fold)
and the operator-pass evidence recorded after it.

**Process hygiene:** implement's census found every process this chunk started terminated. The hung (b) test was killed
by nextest at 120 s. Two `viola.exe` processes belong to `viola-lab/prototype` and were not started by this chunk.
