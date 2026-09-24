# Codebase Research — 2026-09-24-workspace-tree-and-code-graph-planes

## Scope
- **Depth:** moderate (CI, manifests, tooling probes; no product source change in view) · **Reads:** 8 · **Globs/Greps:** 9 · **Measurement probes:** 14
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`, read in full (1 Session Addition applied: `--in-diff` mutation scope).
  - `.claude/rules/testing.md`, read in full (8 Session Additions; the one applied is to drive cargo against throwaway temp projects, never a nested workspace build).
  - `.claude/rules/host-win32.md` (always loaded): the separate `CARGO_TARGET_DIR` rule was applied to every probe build.
- **Platform issues consulted:** No runner-only bullet was folded: CI run 36019646063 is green (below). The plan reads CI runs, and its release check needs jq on every runner, so the runner-image readmes were fetched (`gh api repos/actions/runner-images/contents/images/…`). They list `jq 1.8.1` (Windows2025-Readme.md:71), `jq 1.8.2` (macos-15-arm64-Readme.md:62) and `jq 1.7` (Ubuntu2404-Readme.md:84).

## CI witness — 2026-09-24-quality-gates (PREREQ)
All values below were read from the API, not from the relay:
- **Check-runs:** 12 of 12 `completed success` on `3f385ddf55da967cd0a39eda67eaf5f8d07556a2`, run 36019646063 (`gh api …/commits/{sha}/check-runs`, the full sha).
- **`mutants-verdict-ubuntu-latest`** (`gh run download 36019646063 -n …`): `verdict: counted`, 118 mutants (114 caught, 3 unviable, 1 missed). The 3 `crates/viola-e2e/src/harness/secret_scan.rs:225:5: replace file_mode -> Option<u32> with {None, Some(0), Some(1)}` are all `caught`. The one miss is `run.rs:106:5 fuzz_host_supported -> bool with true`, which is equivalent on Linux.
- **`mutants-verdict-windows-2025`:**
  - `file_mode -> None` missed; `Some(0)` and `Some(1)` caught.
  - `fuzz_host_supported -> true` caught; `-> false` missed. Ubuntu catches `-> false` (it does not appear among ubuntu's non-caught).
  - So each leg's miss is caught by the other, and the `mutants-verdict` union job is `success`.
- **`msrv` job log** (`gh api …/actions/jobs/{id}/logs`): line 295 reads `rustc 1.96.1 (31fca3adb 2026-06-26)`.
- **Verdict:** green; nothing to fold. The overseer relayed the same at 17:55 and directed that it be recorded as this chunk's wrap witness.

## Files inspected
- `.github/workflows/ci.yml` (full, 366 lines). It has 7 jobs: `test`, `mutants`, `mutants-verdict`, `msrv`, `fuzz-replay`, `lint`, `supply-chain`.
  - **No release build.** `grep -n 'release' .github/workflows/ci.yml` finds no `cargo build --release`, so target job 6 is unwired.
  - **`supply-chain` (315–366):** ubuntu, rust-cache and taiki-e `cargo-deny@0.20.2`. Steps: `cargo deny --format json check 2> target/supply-chain/deny.json`, then the sole-root tokio loop over `scripts/sync-crates.txt`, then `deny-probes.sh`, then zizmor. It uploads `target/supply-chain/` with `if: always()`.
  - **`lint` (266–313):** a 3-OS matrix. Steps: the sync-crate `cargo check`, fmt, clippy, ripgrep install, G1, G3, then `lint-probes.sh` (Linux only).
- `Cargo.toml` (full):
  - `[[bin]] viola-fake-agent` has `required-features = ["fake-agent"]`.
  - `[workspace] members = ["crates/*"], exclude = ["fuzz"]`, and there is no `default-members`.
  - `[profile.dev]` and `[profile.release]` both set `panic = "unwind"`.
- `crates/viola-e2e/Cargo.toml` (grep): `[[bin]] viola-harness`, plus a no-op `fake-agent` feature.
- `scripts/sync-crates.txt`: `viola-core`, the only sync crate at HEAD.
- `scripts/deny-probes.sh` (full): the pattern of a throwaway project per ban, with an `[workspace]` escape and a control project. `bans` probes only; there is no sources or advisories probe.
- `scripts/code-graph.py` (grep over the plane registry, `detect_planes` at lines 55–68):
  - rust = root `Cargo.toml`;
  - ts = any tracked-or-untracked-unignored `tsconfig.json` (`git ls-files --cached --others --exclude-standard -- tsconfig.json */tsconfig.json`).
- `scripts/code-graph-cookbook.md` (full): the schema and query shapes.
- `.andromeda/architecture.md` §Infrastructure Patterns (lines 395–494, read) and §Occupied Resources → Repository (lines 376–391, grep).

## Measured facts (each with its derivation)
- **Release outputs.** Each build ran with `CARGO_TARGET_DIR=target/release-probe` and `--message-format=json`; executables were read from the `compiler-artifact` records:
  - `cargo build --release` gives `viola` only (the root package is the sole default member).
  - `cargo build --release --workspace` gives `viola` **and `viola-harness`**.
  - `cargo build --release --bin viola` gives `viola` only.
  - `viola-fake-agent` appears in none of the three (`required-features`).
  - The shared dir `target/release-probe/release/` still held `viola-harness.exe` after the `--bin viola` build (`ls`). **A check that lists the directory would therefore read a stale exe.** The check must read that build's own artifact records.
- **The fuzz lock audit passes at HEAD from the repo root.**
  - `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` exits 0 and prints `advisories ok, sources ok`. Its only diagnostic is the `advisory-not-detected` warning for the root `deny.toml:12` ignore RUSTSEC-2017-0008. So the root `deny.toml` is resolved by walking up to parent dirs.
  - `--config deny.toml` given explicitly also exits 0.
  - `--format json … 2> file` also exits 0.
  - The call runs under the root toolchain because the working directory is the repo root. Run under `working-directory: fuzz`, it would need `nightly-2026-09-20` (from `fuzz/rust-toolchain.toml`) on the supply-chain runner.
- **rmcp is absent from the graph.** `cargo tree -e features -p viola --edges normal | grep -c rmcp` returns `0`. The Lint-row `cargo tree` assertion would pass vacuously until `viola-mcp` lands.
- **cargo-modules 0.27.0** (installed on this host at the CI-pinned version, replacing 0.26.0; `cargo modules --version`):
  - **`orphans --deny` exits 0** on all four targets: `-p viola-core --lib`, `-p viola --bin viola`, `-p viola-e2e --lib`, `-p viola-e2e --bin viola-harness`.
  - **`dependencies --acyclic` exits 1 on 3 of the 4.** Every diagnostic names a cycle between a type and its own inherent method: `viola_core::__nutype_ViolaName__::ViolaName` ↔ `…::try_new`, `viola_core::obs::ObsProcess` ↔ `::as_str`, `viola::obs::MillisUtc` ↔ `::format_time`, `viola_e2e::harness::Outcome` ↔ `::new`.
  - The same targets still exit 1 under `--no-fns --no-types --no-traits --no-owns --no-externs --no-sysroot`: impl methods stay in the graph whatever the filter set.
  - Only the thin `viola-harness` main passes.
  - So **`--acyclic` cannot pass on any crate with an inherent method that names `Self`**: it fails by construction, not because the code has a fault.
- **No cfg-gated module files exist.** A Grep for `#[cfg(..)] mod x;` over `{src,crates,tests}/**/*.rs` found 0 matches. An orphans run on one OS therefore sees the same module set as on the others at HEAD.
- **The taiki-e pin has no cargo-modules manifest.** `gh api repos/taiki-e/install-action/contents/manifests/cargo-modules.json?ref=7623a79c…` returns 404, while `cargo-deny.json` exists. A CI install is `cargo install --locked cargo-modules@0.27.0`, which took 1 min 30 s as a release compile on this host.
- **The TS plane is dormant.** `git ls-files '*tsconfig.json'` is empty and no `.ts`/`.js` file is tracked. `scip-typescript` is on this host's PATH (`which` → `/d/dev/node/npm/scip-typescript`). A tracked `e2e-web/tsconfig.json` would switch the ts plane on at the next refresh (`detect_planes`).

## Graph impact
The code-graph was queried on the rust plane; trace at `tree-query-2026-09-24-workspace-tree-and-code-graph-planes.json`, 3 queries, all `db_state: fresh`.
- **Plane coverage.** `SELECT crate, COUNT(*), COUNT(DISTINCT file) FROM symbol GROUP BY crate` gives viola 341 symbols over 15 files, viola-core 70 over 2, and viola-e2e 501 over 16. This equals the tracked `.rs` counts, re-derived with `git ls-files` per crate: 15, 2 and 16.
- **`fuzz/` is not on the plane.** `SELECT COUNT(*) FROM symbol WHERE file LIKE 'fuzz/%'` gives 0; it is a separate workspace, excluded.
- **Crate edges:** `viola → viola-core` and `viola-e2e → viola-core`.
- The plan changes no Rust symbol, so no caller threading is needed.

## Patterns detected
- **Probe-proves-the-gate** (`scripts/deny-probes.sh:1-5`, `:52-87`): each gate is paired with a throwaway fixture that must fail with the gate's own diagnostic, plus a clean control; the verdict is one summary line. The release-output check should follow the same shape.
- **Fail-closed bash gate steps** (`ci.yml:305`, `:309`): `set +e; … ; test $? -eq 1` for rg gates; `test -s` precedes any loop over a list (`:285`, `:346`).
- **Reports land in `target/supply-chain/`** (`ci.yml:341-342`, `:356-357`): JSON on stderr is redirected there, with `cat` on failure.

## Conventions to follow
- **Action pins:** every `uses:` is a full SHA with a `# vX.Y.Z` comment, `permissions: {}` sits at the top and `contents: read` per job (`ci.yml:7`, `:13-14`); toolchains come only from `rustup toolchain install`.
- **rust-cache** is allowed in `ci.yml` only (`ci.yml:29`).
- **`panic = "unwind"`** in every profile (`Cargo.toml` `[profile.*]`); G3 (`ci.yml:309`) must stay exit 1.

## New files to create
- `scripts/release-check.sh`: builds the release `viola` and asserts from the JSON artifacts that the set of executables is non-empty and contains no `viola-fake-agent` or `viola-harness`. A probe mode feeds synthetic artifact records that must fail. (The shape is decided at P4.)

## Files to modify
- `.github/workflows/ci.yml`: a new per-OS `release` job (target job 6); a fuzz-lock audit step in `supply-chain`; and, per the P4 decision, a cargo-modules `orphans --deny` step.
- No product crate, no `.rs` file and no manifest changes are expected. The mutation verdict should therefore be `no-rust-delta`.
- Companion sweep: a `release-check` name grep over the whole tree (`git grep -n release-check`) finds 0 hits, so nothing else pins the new name.

## Open questions
- Should the cargo-modules / cargo-tree Lint row be wired, given that `--acyclic` fails by construction and the rmcp assertion is vacuous today? → blocks: plan-decision (P4 fork).
- Should the TypeScript plane exist? That is, does `e2e-web/` carry a tracked `tsconfig.json` (which switches the ts plane on), or does v1 stay rust-plane-only? → blocks: plan-decision (P4 fork).
- Should the fuzz lock join the weekly `nightly.yml` `advisories` run? The security extract flags this as open, and the advisory DB moves without code changes. → blocks: plan-decision (P4 lean).
