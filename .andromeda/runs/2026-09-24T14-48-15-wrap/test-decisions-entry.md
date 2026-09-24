`2026-09-24`: The Quality gates chunk wired the per-job `gate`, per-OS coverage, MSRV, a two-leg mutation verdict and the seeded fuzz replay.
- **Decision:**
  - Mutation runs as two legs (`ubuntu-latest`, `windows-2025`). Each writes `mutants-verdict-<leg>.json` (`run --mutants --leg`), and the `mutants-verdict` job gates the union: a mutant is red only when no leg caught it and some leg missed it or timed out. `mutants.out/` is no longer uploaded.
  - The fuzz pipeline is seeded with a `viola_name` target (`ViolaName::try_new` against an independent oracle, 18 synthetic seeds) ahead of the seven parser targets. `fuzz/` is its own cargo workspace, excluded from the root, with its own lockfile and `fuzz/rust-toolchain.toml` channel `nightly-2026-09-20`.
  - No `concurrency:` block in `ci.yml` or `nightly.yml`. zizmor's pedantic `concurrency-limits` is declined and stays visible (2 low).
  - The MSRV and fuzz toolchains come from `rustup toolchain install`, never a toolchain action.
  - New closed values: usage `detail` `invalid-leg` and `unknown-suite`; `run` reasons `fuzz-linux-only` (exit 2), `tool-missing` and `corpus-empty`; the `mutants.leg` field; the leg-verdict `outcome` set.
- **Rationale:**
  - cargo-mutants "does not yet understand conditional compilation … will report functions for other platforms as missed" (mutants.rs/limitations.html). CI run `36005608858` on `39c3d2b` MISSED the `#[cfg(not(unix))]` `file_mode` stub (`secret_scan.rs:230:5`), which Linux never compiles. Operator P4: "fix the gate now, not in Epoch 2".
  - Operator P4: "ViolaName is a real consumer, so the fuzz pipeline is infrastructure with a target, not an abstraction over emptiness".
  - A concurrency group cancels pending runs even with `cancel-in-progress: false`, which would drop a push's `--in-diff` mutation diff and its `always()` gate and upload chain.
  - Architecture and security forbid a toolchain action. `RUSTUP_TOOLCHAIN=1.96` outranks `rust-toolchain.toml` (research M4).
  - `mutants.out/outcomes.json` holds absolute argv paths and `log/` holds test output; the mutants job has no homes to scan.
- **Impact:** §2 (Property-based row), §3 (preamble usage details, `run` body / `--coverage` / `--fuzz-replay` / exit semantics / Output format, `gate`, Bootstrap `ci-tool-install`, Closed enums), §6 (Property suite), §9 (Coverage report, Mutation, MSRV, Fuzz replay and Quality gates rows, tool install, Matrix builds, Test report format), §10 (Stack adjustments, Mutation gate), §11 (CI).
- **By:** `/andromeda-wrap-session`, chunk `2026-09-24-quality-gates`.

