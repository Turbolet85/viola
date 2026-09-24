# Scope — 2026-09-24-workspace-tree-and-code-graph-planes

**Working entry (verbatim title + hint):** Workspace tree and code-graph planes — arch tree lists test/obs/a11y
artifacts, release build free of test binaries, Rust plane ok, TypeScript plane decided

**Epoch:** Epoch 1 — Foundation (its last entry; the epoch closes with this chunk's wrap)

_Premise closure ran at P3 (research.md). Every `[inferred]` tag is now either dropped (verified) or replaced by
`[premise-corrected: …]`._

## What this chunk builds

1. **PREREQ, first action: the CI witness for 2026-09-24-quality-gates. It is closed green.**
   - Run 36019646063 on `3f385ddf55da967cd0a39eda67eaf5f8d07556a2`: 12 of 12 check-runs are `success`.
   - `mutants-verdict-ubuntu-latest`: all 3 `replace file_mode` mutants are `caught`.
   - `mutants-verdict-windows-2025`: `file_mode -> None` missed (expected on Windows), `Some(0)`/`Some(1)` caught.
   - The union job is `success`, and the `msrv` log carries `rustc 1.96.1`.
   - Nothing is folded. The overseer directed that this be recorded as the 3f385dd CI witness at this chunk's wrap
     (research.md §CI witness).
2. **The architecture lists every test, observability and accessibility artifact**
   (`verification-matrix.json#v1-23`). The architecture's workspace, directory tree and dependency policy must all
   name them. The matrix's observed gap names these artifacts:
   - tests: `crates/viola-e2e`, the feature-gated `viola-fake-agent`, `e2e-web/`, `scripts/agent-run.*`, `fuzz/`;
   - obs: `schemas/`, the workspace `clippy.toml`, the `target/secret-scan/` report dir;
   - a11y: `e2e-web/*`, `a11y/sr-pass/`, `viola-ui/assets/index.html`.

   Verified: prior amendments already added part of that list to the tree. Grep counts over `architecture.md`:
   `viola-e2e` 5, `viola-fake-agent` 2, `fuzz/` 10, `schemas/` 7, `clippy.toml` 4, `agent-run` 6,
   `target/secret-scan` 1. The "No separate test crate" statement is gone (0). The residual gap, at 0 hits each:
   - `e2e-web/` and its members (`a11y-row`, `playwright`, `test-results`);
   - `a11y/sr-pass/`;
   - `crates/viola-ui/assets/` (`index.html`, `app.css`);
   - `tests/cmd/` and `tests/snapshots/`;
   - the report dirs `target/perf/` and `target/nextest/`.
3. **The tokio ban still holds for every sync crate.** This re-confirms the existing gate; it does not rebuild it.
   Verified: the `supply-chain` job is green on 3f385dd, and `scripts/sync-crates.txt` lists `viola-core`.
4. **The release build contains no test-only binary.**
   - Verified: per-OS target job 6 is not wired at HEAD (ci.yml has no `cargo build --release`).
   - [premise-corrected: a bare root `cargo build --release` produces only `viola`, and so does `--bin viola`;
     `--workspace` also produces `viola-harness`, and `viola-fake-agent` never builds without its feature
     (measured with `--message-format=json`, research.md §Measured facts)]. The risk is a `--workspace` or
     `--features fake-agent` release build, plus stale exes in a shared `target/release/`. So the check reads the
     build's own JSON artifact records, never a directory listing, and is proven to fail on a record naming a
     test-only binary.
5. **The Rust code-graph plane is ok.** Verified:
   - the `rust` plane indexes all three workspace members (viola 15 files, viola-core 2, viola-e2e 16, equal to the
     tracked `.rs` counts), with `db_state: fresh`;
   - `fuzz/`, a separate workspace, is outside the plane (0 symbols), which is by design.
6. **The TypeScript plane is decided.**
   - Verified: HEAD has no tracked `tsconfig.json` and no `.ts`/`.js`.
   - The plane switches on automatically once a tracked or unignored `tsconfig.json` exists
     (`scripts/code-graph.py` `detect_planes`).
   - The test and a11y plans put TypeScript only in `e2e-web/`. The viola-ui page is Lit 3.3.3 with no JS build
     step, so no tsconfig may cover it.
   - The decision is whether `e2e-web/` carries a tracked `tsconfig.json` (a ts plane scoped to e2e-web) or not
     (rust-only in v1). It is recorded in the architecture at wrap.
7. **CARRY: `fuzz/Cargo.lock` audit** (from 2026-09-24-quality-gates; operator-ratified test-only exemption). Add
   `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources` to the `supply-chain` job, per the
   overseer's fold-into-next rule. Verified: it passes at HEAD from the repo root, where it resolves the root
   `deny.toml`.
8. **CARRY: test-plan §9 Lint-row dependency checks** (from 2026-09-24-quality-gates).
   - `cargo tree -e features`: rmcp must resolve to `server` + `transport-io` only.
   - `cargo modules dependencies --acyclic` and `cargo modules orphans --deny`.
   - The architecture calls cargo-modules "on demand"; test-plan puts it in CI. Decide and wire, or amend.
   - [premise-corrected: measured at cargo-modules 0.27.0, the CI pin. `orphans --deny` passes on all 4 targets.
     `--acyclic` fails by construction on every crate with an inherent method naming `Self` (type ↔ method cycle,
     whatever the filters). The rmcp assertion is vacuous until `viola-mcp` lands (rmcp count 0 in `cargo tree`).]

## Boundaries (out of scope)
- Verified: creating `e2e-web/`, the viola-ui crate or its page, or `a11y/sr-pass/` content is out of scope. Those
  arrive with their Epoch 8 / Web UI chunks; this chunk only lists them in the architecture's tree.
- Verified: no product-crate behaviour changes; research found no product touchpoint. The expected touchpoints are
  `.github/workflows/ci.yml` and a new `scripts/` gate script. The spec amendments (architecture, test-plan,
  security-plan) happen at wrap.

## Surfaces and contracts touched
- `.andromeda/architecture.md` §Infrastructure Patterns (Build system, Project directory structure, CI/CD target
  jobs) and §Occupied Resources (report dirs).
- `.github/workflows/ci.yml`: the `supply-chain` job (fuzz audit), a new per-OS `release` job (target job 6), and
  any cargo-modules step.
- The `viola-harness gate` verdict: no new suite; the release and lint gates are plain exit-code steps.
- The code-graph pipeline (`scripts/code-graph.py`, `.andromeda/cache/{plane}/`): read-only here.
- The verification matrix: `v1-23`.
