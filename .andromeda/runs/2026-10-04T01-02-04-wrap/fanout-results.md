# Fan-out results — 2026-10-03-mutation-scoring-completion

Seven Explore doc-agents, one batch, the prompt sent verbatim. Returns were YAML. Stripping removed only `#` commentary
lines (each restating "no drift" reasoning, or noting that the orchestrator owns the Decisions Log and the optional
C3 pin); no return failed the parse, so no raw twin was saved.

| doc | verdict |
|---|---|
| design-system | `proposals: []`: no UI surface (both new surfaces `tokens n/a`) |
| layout-templates | `proposals: []`: no UI surface; the WSL sweep found no hit |
| a11y-plan | `proposals: []`: no UI element, no schema change. `a11y-plan.md:933` (`run --mutants`, test-plan §10) still holds |
| obs-plan | 2 proposals |
| security-plan | 3 proposals (1 primary + 2 dependents) |
| architecture | 9 proposals (2 primaries + 7 dependents) |
| test-plan | 11 proposals (2 primaries + 9 dependents) |

Playbook rule applied to every proposal below unless stated: **"Accurate this-chunk addition"** (`routine`, apply).
Each names a symbol or value in the report's Changes, and the invariant holds. Applied text is re-derived from the
report, never pasted from `change`.

## obs-plan
1. D-obs-pii · §8 PII Scrubbing → Integration points item 6, the pre-push document bullet (basis `obs-plan.md:967`).
   The pre-push document carries no absolute path. The native in-place Linux gate prints neither the working-tree root
   nor the passwd home. The Windows repo path and Linux clone wording are retired. **apply**: the detector's
   severity is `escalate`, but the playbook rule is routine and no PII class is introduced (the report's coverage row:
   codes and counts only).
2. D-obs-pii · §8 item 6, unscanned uploads (`mutants.out/` bullet) (basis `:969`). The harness scratch is
   Windows-host-only; the Linux host's `<repo parent>/viola-mutants-scratch` is an unprinted operator `TMPDIR`.
   **apply**, as above.

## security-plan
1. D-security-auth · Secret Management → Storage → Development (basis `security-plan.md:445`). The native
   `env -i HOME=<passwd home> PATH=<constants>` crossing, the PATH-only passwd probes, `wsl-exec.sh` /
   `wsl-provision.sh` deleted, CARRY 4 retired, no uid-0 launch. **apply**. Boundary-widening check: what crosses
   `env -i` is still exactly HOME and PATH. HOME's source moved from the distro's `printenv HOME` to the host's passwd
   entry; neither is the harness's environment. The probes carry less (PATH only). Shown to the founder at P4 as no
   widening. **Not the Boundary widening class** (nothing new crosses).
2. D-security-auth · Dependency Security → Pinning, the toolchain bullet (`:332`), dependent-of D-security-auth. The
   WSL2 distro is dropped as an install site. **apply**.
3. D-security-auth · Dependency Security → Pinning, the WSL2 distro install-site bullet (`:337`), dependent-of
   D-security-auth. Replaced by the native `tools` check; `wsl-provision.sh` and its uid-0 `--install-deps` retired
   (CARRY 4). **apply**.
- **Raised by the orchestrator (check 5, plan list):** Security Anti-Patterns → Code Patterns, the scratch bullet
  worded Windows-host-only, inert elsewhere (site `scratch-refused` 1). **apply** (routine; the report substantiates it:
  Harness / gate surface).

## architecture
1. D-arch-decisions · §Stack CI/CD row (`architecture.md:36`). Pre-push native Linux; WSL2 / `wsl-provision.sh`
   clauses dropped. **apply**.
2. dependent · §Infrastructure Patterns → CI/CD approach (key file `ci-cd-approach.md:21`). The pre-push bullet
   rewritten: `tools → linux-tests`; the WSL clone, cache, vm-release, windows-tests and `CARGO_BUILD_JOBS=16` gone.
   **apply**.
3. dependent · CI/CD approach, hyperfine clause (`ci-cd-approach.md:4`). "the WSL provisioning replays nothing new" →
   the `tools` stage gains no check. **apply**.
4. dependent · Project directory structure (`project-directory-structure.md:63-69`). Drop the two script rows.
   **apply**.
5. dependent · §Occupied Resources, `target/pre-push/` (`:424`). Retired. **apply**.
6. dependent · §Occupied Resources, test-side install sites (`:403`). The WSL distro home becomes the passwd home;
   `~/.cache/viola-provision/` and `--install-deps` retired. **apply** (the `viola-provision` cache writer was
   `wsl-provision.sh`, now deleted: Files).
7. dependent · §Occupied Resources, CI workflow data lines (`:383`). `wsl-provision.sh` dropped from the NODE_PIN
   parsers. **apply**.
8. dependent · §Stack Browser e2e row (`:37`). "pre-push Linux leg" → native Linux `pre-push`. **apply**.
9. D-arch-resources · §Occupied Resources, `<repo parent>/viola-mutants-scratch/` (`:434`). The Linux host's operator
   NOCOW `TMPDIR` use is registered; the harness arm stays Windows-only. **apply**, with the measured status in the
   body (`as measured at` evidence/m3.md). Boundary-widening check: mutation runs are not under `env -i`, and the value
   is a constant path the operator sets in the invocation. **Not the Boundary widening class.**

## test-plan
1. D-tests-obs-harness · §3 → 5-command implementation, the `pre-push` block (`5-command-implementation.md:151-156`).
   Native Linux rewrite. **apply**.
2. dependent · §3 → 5-command implementation, Closed enums `pre-push` (`:164`). New reasons, details and stages; retired
   values. **apply**: the report's Changes now carries the closed list (orchestrator-confirmed in the code; added to
   the report before validation, so the proposal rests on the report).
3. dependent · §9 CI Integration, tool-pin paragraph (`test-plan.md:1153`). The WSL provisioning sentence is replaced.
   **apply**.
4. dependent · §10 Performance budgets → Status (`:1221`). The WSL provisioning is dropped from the no-perf-step list.
   **apply**.
5. dependent · §3 → Bootstrap phases, `ci-tool-install` Node bullet (`bootstrap-…md:48`). The WSL distro Node/Chromium
   sentence is retired. **apply**.
6. D-tests-obs-harness · §3 → 5-command implementation, `run` step 4 (`:42-48`). The `--package` arm. **apply**.
7. dependent · §3 `run` Output format, `mutants` object (`:56`). The `verdict:"package"` form. **apply**.
8. dependent · §3 Closed enums, `run --mutants` verdict + `run` reason (`:161-162`). `package` and `package-refused`.
   **apply**.
9. dependent · §10 Mutation gate (`test-plan.md:1209`). The boundary form `run --mutants --package viola-e2e`; the NOCOW
   `TMPDIR` fact; unmeasurable mutants owed by coordinate, and `missed == 0` over the measurable set. **apply**, with the
   measured status for the TMPDIR fact. The vocabulary rule is the overseer's correction, so it is written as a rule.
10. dependent · §2 Test Strategy, the Mutation row (`:437`). The `--package` form named beside the diff form.
    **apply**.
11. dependent · §3 `run` step 4 → Diff (`:44`). `chunk_diff` pins `--src-prefix=a/ --dst-prefix=b/`. **apply** (a
    this-chunk harness behaviour in Changes; the plan's list omits it, and including it keeps the classification's
    premise current).

## Raised by the orchestrator beyond the fan-out
- **obs-plan §1's closing note** (~`:485`, "keeps its pending wording"), brought current under the founder ruling of
  2026-10-04 (directive 1: verbatim upstream copies are kept current). **apply** (directive-settled; the playbook
  supersession is proposed at the wrap card below).
- **Judgment base: `playbook.md` rules `:40` and `:44` are superseded** by the founder ruling (directive 1). They are
  kept verbatim, their `note:` lines open `SUPERSEDED 2026-10-04 (by the founder ruling …) —`, and a new rule is
  appended. **apply** (founder-ruled = approved).

## Validate summary
- **Check 1, playbook:** 25 + 3 orchestrator-raised → routine apply. No collision. Two boundary-widening reads were
  made and neither is the class (above).
- **Check 2, cross-contradiction:** none. The obs §8 item 6 and test §3 pre-push wording agree (test-plan §3 ↔ obs-plan
  §8 states the same no-path document).
- **Check 3, intent-consistency:** every deviation is justified in the report. Scope record: none.
- **Check 4, absence:** test #2's retired values were confirmed in code by grep and carried into the report. The
  cascade sweep follows.
- **Check 5, expected amendments:** all 11 plan entries are covered (security Anti-Patterns raised by the
  orchestrator). The route note goes to P5; the matrix is not carried.
- **Check 6, disproved claims:**
  - 1 → test §10 #9 + arch #9;
  - 2 → test §10 #9;
  - 3 → security #1 + test #1;
  - 4 → the cascade re-derives `.claude/rules/security.md` (a distillation).

  All disposed.
- **Escalations:** 0.
