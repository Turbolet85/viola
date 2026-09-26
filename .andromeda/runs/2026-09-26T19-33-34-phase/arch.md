# arch extract

## Relevance
Partial. The chunk is CI, harness and test-plan work. Architecture governs where the code lives, workflow hygiene, the harness env-var and path registry, and the §CI/CD approach text this chunk will make stale.

## Constraints
- architecture §Established Decisions [Module Boundaries] requires `viola-e2e` to stay test-only, with no product crate depending on it. The scope's boundary is "no product-crate behaviour change". Base resolution, the union verdict and the secret-scan scope therefore belong in `crates/viola-e2e`, `.github/workflows/ci.yml` and `scripts/`, not in `viola-pty` or any other product crate.
- architecture §Conventions (Environment variables) and §Occupied Resources → Environment variables require harness-only variables to use the `AGENT_RUN_` prefix, to be registered, and never to be read by `viola`. `AGENT_RUN_CHUNK_BASE` is registered as "the mutation gate's diff base, read by `viola-harness`". If the new rule changes what that variable means (for example, it becomes optional or an override only), or if the chunk adds a variable, the registry must be amended at wrap. Whether `resolve_base` keeps reading it is research's question.
- architecture §Infrastructure Patterns → CI/CD approach (Least privilege) requires:
  - `permissions: {}` at the workflow top and `contents: read` per job;
  - every `uses:` pinned by SHA;
  - event-payload values reaching a step only through `env:`, with zizmor asserting it.
  This still applies to any `github.event.*` value that stays in the mutants leg, for example as a fallback.
- architecture §Infrastructure Patterns → CI/CD approach (Jobs wired today) fixes the mutation topology:
  - two `fail-fast: false` legs, `ubuntu-latest` and `windows-2025`; macOS stays out, per the scope;
  - each leg uploads only `mutants-verdict-<os>.json`, and `mutants.out/` is never uploaded;
  - `mutants-verdict` runs on ubuntu with `needs: mutants`, `if: always()`, actions/download-artifact 8.0.1 and `gate --require mutants --mutants-legs …`;
  - every gate step is fail-closed `shell: bash`.
- architecture §Occupied Resources → Repository (the `target/agent-run/` row) requires leg verdict files to carry "repo-relative mutant names and outcomes only". If the union needs to know which leg compiles a mutant's code and puts that in the verdict file, the row must be amended. Whether today's names plus outcomes are enough to derive the compiling leg is a P3/P4 question.
- architecture §Occupied Resources → Repository fixes the secret-scan surfaces:
  - `target/agent-run/chunk.diff` and `target/agent-run/artifacts/` are harness-written registered paths;
  - `target/secret-scan/hits.json` is written only on hits and removed on every clean run.
  architecture §CI/CD approach (the `test` job) gates the `diag-`, `junit-` and `harness-<os>` uploads on `steps.secret-scan.outcome`. The narrowed scan scope must still cover every path those uploads ship.
- architecture §CI/CD approach says `ci.yml` has no `concurrency:` block. The stated reason is that a group would cancel pending runs and drop a push's `--in-diff` mutation diff and its `always()` gate chain. The chunk must not add a concurrency block.

## Patterns to follow
- Every gate is proven live by a probe, per architecture §Infrastructure Patterns → Build system: `deny-probes.sh`, `lint-probes.sh`, `orphans-check.sh --probe` and `install-ripgrep.sh --probe` each make a planted case fire and a control pass. Do the same for the base-rule probe (simulated flip → pre-CI → fix pass), the union witness and negative, and the kept planted-secret red test. This fits the chunk's remove-the-guard standing.
- The plan records measured facts as "measured at chunk {marker}", as in the Build system `--exclude` and cargo-modules notes. Use that form when stating the `-G ' · complete · '` edge that overseer direction 2 asks to have measured.
- The legs exist because cargo-mutants reports another OS's `#[cfg]` bodies as missed (architecture §CI/CD approach). The per-leg union extends that rationale, so the rule should be put in those terms: a mutant unviable on its compiling leg.
- Uploads are gated on the secret-scan outcome, and hits go only to `target/secret-scan/hits.json`, uploaded as `secret-scan-<os>` on failure (architecture §CI/CD approach, `test` job). Keep this shape when the scan scope changes.
- The harness command contract belongs to test-plan §3 (architecture §Occupied Resources → Binary). Arch registers resources and topology only.

## Anti-patterns to avoid
- Using `${{ github.event.* }}` directly in a `run:` script, bypassing `env:` (architecture §CI/CD approach, Least privilege). This matters because the chunk moves the base away from event data.
- Registry over-reach: do not register child env the harness sets (`PATH`, `CARGO_TARGET_DIR`, `NEXTEST_PROFILE`) or helper files the tree already carries. The arch registry tracks only variables and paths the harness or product owns (amendments: three-os-ci-headless-harness-skeleton, fake-agent-and-test-data-fixtures).
- Adding a `concurrency:` block, or a macOS mutants leg (the latter is owned by the "Unix endpoint and home hardening" CARRY).

## Contract bindings
- **arch ↔ tests:** The union rule and the per-leg "unviable > caught" red are test-plan §3 `run` step 4 and §10 Mutation gate. architecture §CI/CD approach cites them, and the chunk-base rule is to be stated in test-plan, an expected wrap amendment. Arch §CI/CD approach must mirror the as-built rule, not define it.
- **arch ↔ obs:** The `test` job's gate order and the secret-scan step follow obs-plan §9. Upload admissibility is obs-plan §8. A secret-scan scope change binds to both, and to the registered `target/agent-run/` and `target/secret-scan/` paths.
- **arch ↔ security:** Workflow least privilege (`env:`-only event data, SHA pins, zizmor) binds to the security-plan CI/CD threat-model line.

## Acceptance criteria contributions
- The diff touches only `crates/viola-e2e/**`, `.github/workflows/ci.yml`, `scripts/**` and test-plan. No product crate (`viola`, `viola-core`, `viola-pty`, `viola-agent-claude`) changes (per architecture §Established Decisions [Module Boundaries]).
- In `ci.yml`, no `github.event.*` expression appears inside a `run:` body. Any value that stays event-derived reaches the step only through `env:`. zizmor passes in `supply-chain`, and there is still no `concurrency:` block (per architecture §Infrastructure Patterns → CI/CD approach).
- Every harness env var the chunk reads or adds is `AGENT_RUN_`-prefixed and never read by `viola`. The leg verdict file either stays names plus outcomes only, or the `target/agent-run/` Repository row is amended at wrap to its new content (per architecture §Conventions (Environment variables) and §Occupied Resources).
- At wrap, architecture §CI/CD approach is amended, with a sweep, in three places:
  - the mutants-leg sentence "base = the PR base sha or `github.event.before`";
  - the `mutants-verdict` union sentence, updated to the unviable-on-compiling-leg rule;
  - the no-concurrency rationale, re-checked against a whole-chunk base.
  The `AGENT_RUN_CHUNK_BASE` registry line is also amended if its meaning changed (per architecture §Infrastructure Patterns → CI/CD approach, §Occupied Resources → Environment variables).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton:** registered `viola-e2e`/`viola-harness`, `AGENT_RUN_CHUNK_BASE` and the `AGENT_RUN_` prefix, and `target/agent-run/`. It wired `mutants` with the base "via `env:`", which is the event-derived base this chunk replaces. It rejected registering the harness child env as registry over-reach.
- **2026-09-24-observability-gates:** registered `target/secret-scan/hits.json` and replaced the unscanned `agent-run-<os>` upload with scan-gated uploads in obs-plan §9 order. This is the scan whose scope this chunk narrows.
- **2026-09-24-quality-gates:** introduced the two-leg `mutants` matrix, the `mutants-verdict` union job and download-artifact 8.0.1, and registered `mutants-verdict-<leg>.json`. It also recorded the declined `concurrency:` block and why.
- **2026-09-24-epoch-1-cleanup:** amended the union sentence so it is no longer the only red path: a leg is red on its own when its unviable mutants outnumber its caught ones, citing test-plan §3/§10. This is the closest prior edit to the per-leg union text this chunk changes.
- **2026-09-25-pty-wrapper-on-windows:** landed `viola-pty` and `HostTerminal` with cfg variants, the source of the 36165685381 `HostTerminal::enter` breach this chunk's union rule closes. No arch text about mutation was amended there.
