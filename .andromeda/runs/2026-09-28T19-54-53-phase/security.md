# security extract

## Relevance
partial — no product surface, secret or IPC change. The chunk edits `ci.yml` (supply-chain / zizmor / action-pin surface), the pre-push WSL `env -i` launch, and the secret-scan's scope. Several security-plan lines name the mutation machinery being removed, so they become expected wrap amendments.

## Constraints
- The `ci.yml` edit must keep workflow-level `permissions: {}` and job-level `contents: read`, every remaining `uses:` pinned by full commit SHA with a version comment, and no `github.event` value reaching a `run:` except through `env:`. Per security-plan §Dependency Security (CI integration), the last property currently holds because "the mutation base is derived by the harness from git history". Research must check whether the kept `run --mutants` arm or the test job still reads any event value once the mutation jobs are gone.
- `zizmor .github/workflows/`, `cargo deny check`, the fuzz-lockfile audit, `scripts/npm-audit.sh` and `scripts/deny-probes.sh` must stay green after the workflow edit, and none may be silenced, skipped or widened to get there (per security-plan §Dependency Security, Audit tool / CI integration).
- The `actions/download-artifact` SHA pin is recorded as serving only the `mutants-verdict` job (per security-plan §Dependency Security, CI integration pin list). Research must check whether any other `ci.yml` job still uses that action. If none does, the pin leaves the workflow with the job and the plan line becomes a wrap amendment. The pin must not stay behind as an unused `uses:`.
- The pre-push gate must keep launching every WSL call as `--exec /usr/bin/env -i HOME=… PATH=…`. The only named extra assignment is `TMPDIR=<distro home>/viola-pre-push-scratch`, on the Linux mutation leg. Removing that leg must add no assignment whose value comes from the host, and no gate or harness path may start running a distro command through `wsl-exec.sh` (per security-plan §Secret Management, Development; Decisions Log `2026-09-27` pre-push gate and `wsl-exec.sh`).
- The WSL distro installs only CI's own pins: `wsl-provision.sh` parses cargo-nextest, cargo-mutants and cargo-llvm-cov from `ci.yml`'s `test`-job `tool:` line (per security-plan §Dependency Security, Pinning). If this chunk changes that line or `wsl-provision.sh`, it becomes a re-provision input. The `--install-deps` root hardening (root runs only `apt-get install` over an allowlisted dry-run list) then binds this chunk in full (per Decisions Log `2026-09-27`, browser pipe / operator-only root install, Conditions).
- The kept `run --mutants` arm keeps the host mutation scratch guard unchanged. It refuses unless the final component is exactly `viola-mutants-scratch` and the repository lies outside it, fails with `scratch-refused` / `scratch-wipe-failed`, never falls back to `%TEMP%` or the repository, and never prints the path. Its remove-the-guard test pair also stays (per Decisions Log `2026-09-27`, test-only feature / host mutation scratch).
- The artifact canary scan (`viola-harness secret-scan`) keeps running in the `test` job and gating every scan-gated upload. Its one exact-path skip of `target/agent-run/chunk.diff` must not be widened. Research must check whether the kept arm still writes that file under the capture: if it does, the skip and the `!target/agent-run/chunk.diff` upload exclusion stay; if it does not, they may go but must not become a broader pattern (per security-plan §Bootstrap phases, `secret-scanning-ci-gate`; §Secret Management, Secret scanning in CI).

## Patterns to follow
- An own audit, never an exemption: when an upload or gate disappears, its exemption goes with it (the `mutants-verdict-<os>.json` unscanned-upload admission, per Decisions Log `2026-09-24`). No new unscanned upload takes its place (per security-plan §Bootstrap phases, `secret-scanning-ci-gate`).
- Harness-authored artifacts hold repo-relative locations and outcomes only: never an absolute path, an argv, a log path or test output. This applies to any per-leg verdict artifact the kept arm still writes for the code audit (per security-plan §Bootstrap phases, `secret-scanning-ci-gate`).
- Pins have one home: tool versions stay parsed from `ci.yml`'s text by the provisioning and install scripts, never duplicated elsewhere (per security-plan §Dependency Security, Pinning).
- A widening is recorded, never routine: any new host→distro assignment or new env read is a Decisions Log entry that halts for a live answer (per security-plan §Secret Management, Development).

## Anti-patterns to avoid
- NEVER reference a GitHub Action by a mutable ref, and never widen an ignore, add a `skip`/`allow` or silence zizmor to reach green after the job removal (per security-plan §Security Anti-Patterns → Code Patterns; §Dependency Security, Audit tool).
- NEVER add a host-valued assignment to a WSL launch or a `wsl-exec.sh` caller, and never let the root `--install-deps` launch become a gate, harness or pre-push stage (per security-plan §Secret Management, Development).
- NEVER add another env seam read by a `viola` build (`AGENT_RUN_*` names are "never read by `viola`") while reshaping the harness (per security-plan §Security Anti-Patterns → Universal; Decisions Log `2026-09-27` pre-push gate).

## Contract bindings
- security ↔ tests: test-plan §3 and §10, the expected wrap amendments the scope names. The `test` job keeps installing `cargo-mutants@27.1.0`, which is the provisioning input `wsl-provision.sh` parses (security-plan §Dependency Security, Pinning).
- security ↔ obs: obs-plan §9's `secret-scan` step and the `harness-<os>` upload exclusion of `chunk.diff` (security-plan §Secret Management, Secret scanning in CI).
- The security-plan's own mutation lines are expected wrap amendments, never phase edits:
  - §Threat Model Summary, CI/CD jobs list ("mutation (ubuntu and windows legs plus a union verdict)")
  - §Dependency Security, CI integration: the `download-artifact` pin annotation, the concurrency note's mutation-coverage clause, and the "mutation base derived by the harness" line
  - §Bootstrap phases `secret-scanning-ci-gate`: the `mutants-verdict-<os>.json` unscanned upload
  - §Secret Management, Development: `TMPDIR` "on the Linux mutation leg"
  - a new Decisions Log entry retiring the 2026-09-24 `mutants-verdict` upload admission and the 2026-09-27 pre-push mutation-leg `TMPDIR` assignment

## Acceptance criteria contributions
- (security) After the `ci.yml` edit:
  - `zizmor .github/workflows/`, `cargo deny check` and `bash scripts/deny-probes.sh` are green.
  - Every remaining `uses:` in `ci.yml` is a full SHA with a version comment, and no unused `download-artifact` pin remains.
  - `permissions: {}` and job `contents: read` are unchanged.

  (per security-plan §Dependency Security, CI integration)
- (security) Every WSL launch in `viola-harness pre-push` still carries only `env -i HOME=… PATH=…` plus distro-derived constants, with no host-valued assignment. `grep -rn wsl-exec` over `crates/`, `src/`, `.github/` and `scripts/agent-run.*` finds no caller (per security-plan §Secret Management, Development).
- (security) The `test` job's `secret-scan` still gates the `diag-`, `junit-` and `harness-<os>` uploads. Its exact-path skip is still `target/agent-run/chunk.diff` or removed, never widened, and no new unscanned upload is added (per security-plan §Bootstrap phases, `secret-scanning-ci-gate`).
- (security) Either the `test`-job `tool:` line and `wsl-provision.sh` are byte-unchanged, or the chunk takes the `--install-deps` root-hardening CARRY in full. The host scratch guard's refusal tests stay present and green (per Decisions Log `2026-09-27`, browser pipe Conditions; Decisions Log `2026-09-27`, host mutation scratch).
