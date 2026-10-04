# arch extract

## Relevance
partial — a test-tooling chunk: no product crate, wire contract or runtime surface changes, but it reshapes the harness (`viola-e2e`), the pre-push contract and several registered resources that arch owns.

## Constraints
- All of the chunk's code (the viola-e2e mutation form, the C3 prefix pin, the pre-push reshaping) belongs in the test-only `viola-e2e` (`harness::run::mutants`, `harness::pre_push`) or in test files. No product crate may come to depend on `viola-e2e` (per architecture §Established Decisions [Module Boundaries]; §Infrastructure Patterns → Crate dependency direction).
- A viola-e2e boundary form that needs the root bins in each copied tree has to keep `viola-fake-agent` a test-only root `[[bin]]` behind `fake-agent`, and it must not make `fake-agent` reach a release build. The `fake-agent = []` no-op features exist only so that package-scoped cargo-mutants builds resolve (per architecture §Occupied Resources → Binary, subcommands and exit codes / Workspace crates; §Infrastructure Patterns → CI/CD approach, target job 6).
- The `target/mutants/` contract requires a relative `CARGO_TARGET_DIR=target/mutants` with `--copy-target=true`, so that each copied tree builds its own test binaries. A copied `target/` keeps the original tree's `CARGO_BIN_EXE_*` paths, so tests would drive the unmutated binary. Any viola-e2e form has to keep that property (per architecture §Occupied Resources → Repository, `target/mutants/`).
- The host mutation scratch is specified as `HOST_SCRATCH = cfg!(windows)`, and off Windows `mutants.out/` stays at the repository root. On the Linux host the scratch arm should therefore be inert. Whether the code keys it that way is research's question (per architecture §Occupied Resources → Repository, `<repo parent>/viola-mutants-scratch/`).
- CI runs no mutation job, and pre-push has no mutation stage. Mutation scoring runs only through `agent-run run --mutants` for the epoch-boundary audit. `run --mutants` sets `AGENT_RUN_KEEP_HOMES=0` / `AGENT_RUN_KEEP_FAILED=0` on the `cargo mutants` command (per architecture §Infrastructure Patterns → CI/CD approach).
- The pre-push contract is today's target text: Windows-host only (`pre-push-windows-only` elsewhere); stages `tools → sync → cache → linux-tests → vm-release → windows-tests`; every WSL call `env -i HOME=… PATH=…` with no further assignment; tool pins checked against ci.yml, including `node --version` = `v<NODE_PIN_VERSION>`; a filter before the push, with CI's run as the verdict of record. The chunk reshapes the stages, and that bullet is amended at wrap. The invariants it must keep are the HOME+PATH-only `env -i`, the pin checks, the absence of a mutation or perf stage, and CI as the verdict of record (per architecture §Infrastructure Patterns → CI/CD approach).
- Every OS-specific branch must compile and be tested on its CI runner, and the matrix is `windows-2025` (x64) / `macos-latest` / `ubuntu-latest`. The 13th mutant's `cfg(all(windows, not(target_arch = "x86_64")))` branch has no runner, so its not-measurable record is a gap against this discipline and should be named as one, not hidden (per architecture §Cross-cutting Patterns → Cross-platform discipline; §Stack and Technologies, CI/CD row).

## Patterns to follow
- `run --mutants`' own prebuild (`viola --features fake-agent` into `<repo>/target/mutants`, then cargo-mutants with relative `CARGO_TARGET_DIR` and `--copy-target=true`) is the registered shape to extend to viola-e2e (per architecture §Occupied Resources → Repository, `target/mutants/`).
- The harness builds into its own `target/harness/` (`CARGO_TARGET_DIR`). Any new harness build or test run keeps a separate target dir rather than sharing `target/` (per architecture §Occupied Resources → Repository, `target/harness/`).
- Pin data is parsed from file text, never from the environment. `pre_push::linux::node_pin` reads ci.yml's `NODE_PIN_*` lines from the file, and a native Linux stage keeps that reader (per architecture §Occupied Resources → Environment variables, CI workflow data lines).
- Scratch that carries repository source text (`target/pre-push/` index + `tree.patch`, `target/agent-run/chunk.diff`) sits where `secret-scan` and the uploads treat it by exact path. If the sync stage is retired or re-shaped, its scratch follows the same placement rule or is retired with it (per architecture §Occupied Resources → Repository, `target/pre-push/` and `target/agent-run/`).
- The mutation arm's `outcomes.json` is archived under `target/run-archive/<n>/` and never uploaded, because it carries absolute argv paths (per architecture §Occupied Resources → Repository, `target/run-archive/`).

## Anti-patterns to avoid
- Adding a mutation job to `ci.yml` or a mutation stage to pre-push (per architecture §Infrastructure Patterns → CI/CD approach).
- Letting a host value cross the native stage's `env -i` beyond HOME and PATH, or reading any env var as configuration. A new harness-only variable takes the `AGENT_RUN_` prefix and is never read by `viola` (per architecture §Conventions → Environment variables; §Cross-cutting Patterns → Config management). The C3 prefix pin belongs in `git` argv (`--src-prefix`/`--dst-prefix`), not in an environment or config override.
- A viola-e2e seam or feature path that would put `fake-agent` / `test-support` into a product or release artifact (per architecture §Infrastructure Patterns → CI/CD approach, target job 6).

## Contract bindings
- arch §Infrastructure Patterns → CI/CD approach (pre-push stages, `run --mutants`) ↔ tests test-plan §3 (the harness command contract, including the internal `pre-push` subcommand) and §10 (Mutation gate). The stage list and refusal codes have to change in lockstep.
- arch pre-push `env -i` boundary and the `wsl-provision.sh --install-deps` root install ↔ security (`.claude/rules/security.md` WSL2 bullet; security-plan §Secret Management, Development). Retiring the WSL tooling retires the root-install CARRY. Keeping any WSL path keeps it.
- arch `target/pre-push/` and `target/agent-run/chunk.diff` ↔ obs/CI `secret-scan` (exact-path skip; `harness-<os>` upload exclusion).
- Wrap-time arch amendments this chunk feeds (no spec edit in phase):
  - the §Infrastructure Patterns → CI/CD approach pre-push bullet;
  - §Occupied Resources → Filesystem's WSL test-side install sites;
  - the `scripts/wsl-provision.sh` mention in the CI workflow data lines;
  - the `target/pre-push/` row (the "WSL clone");
  - §Infrastructure Patterns → Project directory structure's `wsl-exec.sh` / `wsl-provision.sh` rows;
  - any new resource the viola-e2e mutation form lands, such as a target dir or an env var.

## Acceptance criteria contributions
- The chunk's diff touches only `crates/viola-e2e/`, test files, `scripts/` and `.config/`. No product crate gains a dependency on `viola-e2e`, a new env-var read or a `fake-agent`-reachable release path, and `scripts/release-check.sh --probe` still reads `5/5 refused, control clean` (per architecture §Established Decisions [Module Boundaries]; §Infrastructure Patterns → CI/CD approach, target job 6).
- `ci.yml` gains no mutation job, and the reshaped pre-push has no mutation or perf stage. Every command the native stage runs carries an environment of exactly HOME and PATH, and any extra PATH entry is shown to the founder (per architecture §Infrastructure Patterns → CI/CD approach).
- The viola-e2e mutation form builds each copied tree's test binaries in that copy: one witnessed mutant in a spawned bin's path is caught, or the `CARGO_BIN_EXE_*` paths are shown to resolve inside the copy (per architecture §Occupied Resources → Repository, `target/mutants/`).
- Every registered resource the chunk adds, retires or re-shapes (the WSL install sites, `target/pre-push/`, `wsl-*.sh`, any new `AGENT_RUN_*` variable or target dir) is listed in the chunk record for wrap's §Occupied Resources / Project directory structure amendment (per architecture §Occupied Resources; §Infrastructure Patterns → Project directory structure).
