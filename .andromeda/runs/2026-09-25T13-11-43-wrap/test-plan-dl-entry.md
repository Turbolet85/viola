
`2026-09-25`: The Security prerequisites chunk added the `test-only-rust-delta` mutation verdict and pinned the SQOS open and the content hash.
- **Decision:**
  - New closed value: `run --mutants` `mutants.verdict` ∈ {`counted`, `no-rust-delta`, `test-only-rust-delta`}.
    - A diff whose `.rs` paths are all test targets (`tests/`, `benches/`, `examples/`, at the root or under `crates/<member>/`) never builds or runs cargo-mutants. It passes only with that explicit verdict, which names the diff, its file count and `rust_files`, and the leg file carries it with `"mutants":[]`.
    - A mixed diff (any other `.rs` path) stays `counted`, and red `outcomes-missing` without a fresh `outcomes.json`.
    - This refines the chunk-2 operator ruling ("a diff WITH .rs files and no outcomes stays red"). The ruling still holds for every diff that names a `.rs` path cargo-mutants can mutate.
  - The Windows client SQOS open is pinned ahead of `viola-channel` by `tests/channel_sqos_open.rs`, with a discriminating no-SQOS control.
  - The pinned-exe content hash's library is pinned by `tests/contract_content_hash.rs` (FIPS 180-2 vectors + the 16-hex truncation).
- **Rationale:**
  - cargo-mutants 27.1.0 over this chunk's tests-only Rust diff printed `INFO No mutants to filter` and wrote no `outcomes.json`, so the gate read `outcomes-missing` (chunk implement gate run 1, entry 24).
  - `--list-files` over a package with `src/`, `tests/`, `benches/` and `examples/` files lists `src/lib.rs` alone, so all three target directories are unmutatable (measured on 27.1.0).
  - Operator (founder-delegated) ruling "option A", 2026-09-25: fold the fix. Conditions:
    - only test-target diffs;
    - the mixed red kept, under a literal-oracle test (`run_mutants_mixed_src_and_test_delta_without_outcomes_stays_outcomes_missing`);
    - the new classifier's own mutation reading;
    - this entry.
  - Witness: the local windows leg `counted`, 9 mutants, 9 caught. CI run 36138441784 on `8e25ca7`: ubuntu and windows `9 mutants tested … 9 caught`, `mutants-verdict` success.
- **Impact:** §3 (`run` step 4 Classification, Output format `mutants` object and leg file, Closed enums); §6 (Security control negatives → Windows client SQOS; Contract suite); §10 (Mutation gate).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-25-security-prerequisites`.
