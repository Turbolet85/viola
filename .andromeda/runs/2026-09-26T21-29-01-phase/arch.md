# arch extract

## Relevance
Partial. The chunk is test-side only: harness, scripts, docs and local WSL provisioning. Arch applies through workspace placement, the harness/env-var/path registries, the toolchain pin sources and the mutation-leg rules. The product-crate, IPC, GUI and hook sections do not apply.

## Constraints
- The gate lives only in test-side homes. That means the test-only `viola-e2e` crate (`viola-harness`) and the `scripts/agent-run.{sh,ps1}` shims. No product crate may depend on `viola-e2e`, and no product crate's behaviour changes. Per architecture §Established Decisions [Module Boundaries] and §Occupied Resources → Workspace crates / Binary, subcommands and exit codes. §Occupied Resources also says the harness command contract is test-plan §3.
- §Project directory structure describes `scripts/agent-run.{sh,ps1}` as "identical shims over viola-harness". A gate added as a harness subcommand or flag keeps that shape. A new sibling script is a tree addition that has to be amended in at wrap. Which form to use is P3/P4's decision.
- Any new harness variable must use the `AGENT_RUN_` prefix and must never be read by `viola`. Per §Conventions (Naming patterns → Environment variables) and §Occupied Resources → Environment variables.
  - `AGENT_RUN_CHUNK_BASE` is registered as an explicit override of the derived base. A gate whose printed `base` must equal CI's has to leave the base to the harness derivation, as CI does. Per §Occupied Resources → Environment variables.
- The Rust toolchain has one pin source: `rust-toolchain.toml` (exact 1.98.1 + rustfmt + clippy). CI installs it with `rustup toolchain install` reading that file, with no toolchain action. Per §Stack and Technologies (Language / runtime) and §Infrastructure Patterns → CI/CD approach (Setup steps).
  - cargo-llvm-cov 0.9.1 + `llvm-tools-preview` is registered in the §Stack Code quality row.
  - cargo-nextest and cargo-mutants versions are NOT registered in the arch Stack. Their pin source is `ci.yml`. Whether the Stack row should gain them is a wrap question.
- The ubuntu mutation leg must follow the same verdict and base rules as CI's `mutants` legs, per §Infrastructure Patterns → CI/CD approach (Jobs wired today):
  - it runs `run --mutants --leg`;
  - the base is derived as the last master flip before the chunk's oldest operator pre-CI commit;
  - a leg is red at its own run when its unviable mutants outnumber its caught ones;
  - the cross-leg union (`harness::cfg_legs`) stays CI's `mutants-verdict`.

  The derived-base algorithm itself is not changed.
- New `viola-e2e` code must pass the workspace gates, per §Infrastructure Patterns → Build system (Lint; Boundary review; Dependency policy):
  - clippy `--workspace --all-targets --features fake-agent -D warnings`. `viola-e2e` opts out of the print bans and denies only `dbg_macro`.
  - the `cargo modules orphans --deny` gate for each lib/bin target.
  - `cargo deny`: any new dependency is pinned in `[workspace.dependencies]`, allowed licences only, and no C-building crates such as `cc`.
- Any new gitignored output path under the repo (for example a gate report dir under `target/`) is a Repository registry entry. A Linux-side clone or cache that lives outside the repo is not an arch resource unless the product reads or sets it (see the registry-over-reach note under amendment history). Per §Occupied Resources → Repository.

## Patterns to follow
- **Pinned, verified tool install.** `scripts/install-ripgrep.sh` installs a pinned release asset, checks it against the published sha256, and has a `--probe` self-test. Tools without a taiki-e manifest are installed with `cargo install --locked <tool>@<ver>` (zizmor, cargo-modules, cargo-fuzz). Both routes suit provisioning inside WSL, where taiki-e/install-action does not exist. Per §Infrastructure Patterns → CI/CD approach (Setup steps).
- **Probe-first gates.** `deny-probes.sh`, `lint-probes.sh`, `orphans-check.sh --probe` and `release-check.sh --probe` each prove a gate fires on a planted fault and passes a clean control. This matches the chunk's planted-Unix-failure witness. Per §Infrastructure Patterns → Build system.
- **Typed tool-missing refusal.** `scripts/release-check.sh` and `scripts/orphans-check.sh` refuse with `tool-missing: jq` instead of skipping. This is the model for a typed failure when the distro or a tool is unprovisioned. Per §Stack Code quality row and §CI/CD approach.
- **Dedicated target dirs.** `target/harness/` and `target/release-check/` each give a gate its own `CARGO_TARGET_DIR`; `release-check` honours an inherited one and never sets it. Relevant to the bounded Linux build cache. Per §Occupied Resources → Repository.
- **Direct child spawning.** "Child processes are spawned directly" and "Nothing shells out through `sh`, `bash` or `cmd`" are written for the product. The same form fits a harness that spawns `wsl -d Ubuntu -- …` as a direct process. Per §Cross-cutting Patterns → Cross-platform discipline.

## Anti-patterns to avoid
- Setting `AGENT_RUN_CHUNK_BASE` from the gate to force a base, or re-implementing the base or verdict logic outside the harness. Either would split the local leg from CI's derived base and verdict. Per §Occupied Resources → Environment variables and §CI/CD approach.
- Letting a floating or unpinned toolchain or tool into the local leg, when the arch pin sources are `rust-toolchain.toml` and CI's pinned installs. Per §Stack and Technologies (Language / runtime) and §CI/CD approach (Setup steps).
- Treating the local gate as a replacement for CI's per-runner testing. §Cross-cutting Patterns → Cross-platform discipline says every OS-specific branch compiles and is tested on its CI runner. The local gate is only a pre-push filter.

## Contract bindings
- **arch ↔ tests:** `viola-harness`'s command contract, and so the gate entry's JSON document and typed exit, is test-plan §3 (per §Occupied Resources → Binary, subcommands and exit codes). The leg-red and union rules cite test-plan §3 `run` step 4 and §10 Mutation gate (per §CI/CD approach).
- **arch ↔ obs:** the harness writes under `target/agent-run/`, including `chunk.diff`, the one file `secret-scan` skips. Any new gate output that CI might upload falls under obs-plan §8/§9 admissibility (per §Occupied Resources → Repository).
- **arch ↔ security:** WSL-side tool provisioning falls under the supply-chain pin discipline: exact versions, `--locked`, and sha256-verified downloads. Per §Infrastructure Patterns → Build system (Dependency policy) and CI/CD approach (Setup steps). The security plan owns the rules.

## Acceptance criteria contributions
- The diff touches only `crates/viola-e2e/**`, `scripts/**` and docs. It changes no product member's manifest or source, and no product crate depends on `viola-e2e` (per architecture §Established Decisions [Module Boundaries]).
- Every new environment variable is `AGENT_RUN_`-prefixed, `viola` reads none of them, and the gate does not set `AGENT_RUN_CHUNK_BASE`. The ubuntu leg's printed `base` comes from the harness derivation (per architecture §Conventions Naming patterns and §Occupied Resources → Environment variables).
- The Linux toolchain resolves from `rust-toolchain.toml` through rustup, the same route as CI, and reports `rustc 1.98.1` with rustfmt and clippy present (per architecture §Stack and Technologies Language / runtime and §CI/CD approach Setup steps).
- New `viola-e2e` modules pass clippy `-D warnings` with `--features fake-agent` and the orphans gate. Any new gitignored repo path is registered under §Occupied Resources → Repository at wrap (per architecture §Infrastructure Patterns → Build system and §Occupied Resources → Repository).

## Relevant amendment history
- **2026-09-26-ci-chunk-base-and-union-verdict:** the chunk right before this one.
  - `AGENT_RUN_CHUNK_BASE` became an explicit override that CI does not set.
  - The mutation legs now derive the base in the harness and name it in the run document.
  - `mutants-verdict` judges each mutant only by the legs whose `#[cfg]`s compile its line (`harness::cfg_legs`), which added syn/proc-macro2 to `viola-e2e`.
  - `chunk.diff` is excluded from the scan and the upload.

  Why: whole-chunk coverage on every push. This is the precondition the Linux clone's git history must satisfy.
- **2026-09-24-epoch-1-cleanup:** a leg is red at its own run when unviable > caught, before the union. Why: this was a cross-master citation of test-plan's mutation verdict. It is the rule the local leg is judged by.
- **2026-09-24-quality-gates:** introduced the two-leg (ubuntu + windows) mutation matrix plus the union, and cargo-llvm-cov 0.9.1 + `llvm-tools-preview`. Why: cargo-mutants reports another OS's `#[cfg]` bodies as missed. This is the asymmetry the local ubuntu leg now catches before the push.
- **2026-09-24-three-os-ci-headless-harness-skeleton:** fixed the Rust 1.98.1 exact pin in `rust-toolchain.toml`, CI's `rustup toolchain install` route with no toolchain action, the `viola-e2e` registration, the `AGENT_RUN_` prefix and `target/harness/`. That wrap also refused to register the harness's child env (`PATH`, `CARGO_TARGET_DIR`, `NEXTEST_PROFILE`) as "registry over-reach". This is the precedent for not registering WSL/child plumbing.
- **2026-09-24-fake-agent-and-test-data-fixtures / 2026-09-24-supply-chain-and-workflow-gates:** registry-over-reach rejections, e.g. `tests/support/` and config files were routed to the tree rather than Occupied Resources. This guides whether a new script, the Linux clone path or a report dir goes in the Repository registry or only in the tree.
- **2026-09-24-observability-gates / 2026-09-24-workspace-tree-and-code-graph-planes:** set up the pinned, sha256-verified `install-ripgrep.sh` with `--probe`, the `cargo install --locked` route for tools without a manifest, and the per-gate target dirs (`target/release-check/` honours an inherited `CARGO_TARGET_DIR`). These are the precedents for WSL tool provisioning and the bounded Linux cache.
