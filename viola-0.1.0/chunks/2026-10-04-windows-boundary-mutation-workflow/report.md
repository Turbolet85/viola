# Report — 2026-10-04-windows-boundary-mutation-workflow

**Chunk:** Windows boundary mutation workflow — cfg(windows) mutants scored on the windows-2025 runner by a dispatch-only, non-blocking workflow; the 34 owed coordinates graded; the mutation-run temp-dir leak
**Date:** 2026-10-04
**Commits:** `60c569b chore(2026-10-04-windows-boundary-mutation-workflow): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-04T01:25:50Z besides the prior wrap `7aca558`)

## Changes (structured — detectors read this)
- **Files:** (basis: `git diff --name-only 7aca558` + untracked, excluding `.andromeda/runs/`)
  - new `.github/workflows/windows-mutants.yml`;
  - new `tests/contract_windows_mutation_scope.rs`;
  - modified `.config/nextest.toml`;
  - modified `.claude/docs/commands.md`;
  - chunk folder `viola-0.1.0/chunks/2026-10-04-windows-boundary-mutation-workflow/{scope,research,plan,report}.md` +
    `evidence/{guard,leak,operator-pass,windows-dispatch}.md`;
  - pipeline bookkeeping: `.andromeda/{friction-log.ndjson,master-route.md}`, `viola-0.1.0/working-route.md`,
    `.claude/session-handoff.md`.
- **Symbols / APIs:** no product symbol, IPC method, endpoint, port, socket or env var added or changed. Two new
  test-side and CI surfaces:
  - **The new GitHub Actions workflow `windows-mutants`** (`.github/workflows/windows-mutants.yml`).
    - Trigger: `on: workflow_dispatch` with no `inputs:` (the only trigger).
    - Permissions: workflow `permissions: {}`; job `mutants` with display name `mutants (${{ matrix.package }})`,
      `runs-on: windows-2025`, job `permissions: contents: read`, `timeout-minutes: 120`.
    - Strategy: `fail-fast: false`, six `matrix.include` items (`package` + one-line space-separated `files`):
      - viola-pty: `lib.rs`, `sideload.rs`;
      - viola-channel: `client.rs`, `endpoint.rs`, `server.rs`, `server/win.rs`, `test_support.rs`;
      - viola-state: `pin.rs`, `fs.rs`;
      - viola-agent-claude: `lib.rs`;
      - viola: `src/cmd/run.rs`, `src/run/env.rs`, `src/panic_frames.rs`, `src/main.rs`, `src/conpty.rs`;
      - viola-e2e: `harness/run/mutants/scratch.rs`, `harness/cleanup.rs`, `harness/run/browser.rs`.
    - Steps: checkout `3d3c42e5…` (v7.0.1, `persist-credentials: false`) → `rustup toolchain install` →
      install-action `7623a79c…` (v2.87.19) with `tool: cargo-nextest@0.9.146,cargo-mutants@27.1.0` → one
      `shell: pwsh` step reading `PACKAGE` / `FILES` from step `env:` (set from `matrix.*`) that runs
      `./scripts/agent-run.ps1 run --mutants --package $env:PACKAGE` with one `--file` per entry, then
      `if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }`.
    - Absent: no cache, `concurrency:`, `secrets.`, upload or `needs:`.
    - Its checks appear on the dispatched sha as `mutants (<package>)` × 6.
  - **The new root integration test binary `contract_windows_mutation_scope`** (sync, no new dependency,
    auto-discovered: no `[[test]]` entry, no `required-features`). Three tests:
    - `windows_gated_sources_equal_the_workflow_scope_both_ways`:
      - walks every `*.rs` under `src/` and `crates/*/src/`;
      - a file is gated by a product line (before the first column-0 `#[cfg(test)]`) carrying
        `cfg(windows` / `cfg(all(windows` / `cfg(any(windows` / `cfg!(windows)` / `target_os = "windows"`;
      - or by a gated `mod x;` line (resolved to `x.rs` / `x/mod.rs` beside `lib.rs` / `main.rs` / `mod.rs`, else
        under the declarer's stem dir), transitively through modules such a module declares;
      - asserts set equality with the workflow's matrix both ways, plus own-package membership.
    - `workflow_tool_pins_equal_the_ci_test_job_line`: the workflow's `tool:` pins (cargo-nextest, cargo-mutants)
      equal ci.yml's test-job line, the one `ci_pins` in `crates/viola-e2e/src/harness/pre_push/linux.rs` also reads.
    - `workflow_trigger_is_dispatch_only_without_inputs`.
    - Each test runs synthetic known positives first.
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none — no `Cargo.toml` or `Cargo.lock` change (`git diff --quiet 7aca558 -- Cargo.toml Cargo.lock`:
  exit 0). The workflow installs cargo-nextest 0.9.146 and cargo-mutants 27.1.0, the existing ci.yml pins.
- **Schema / config:** `.config/nextest.toml` `[profile.mutants]`:
  - `fail-fast = { max-fail = 1, terminate = "immediate" }` → `{ max-fail = 1, terminate = "wait" }`, and the comment
    above it rewritten to the new reason;
  - `slow-timeout` 5 s × 2, the `package(viola-e2e)` override 15 s × 2 and `[profile.ci]` byte-unchanged (gate entry
    `git diff -U0 7aca558 -- .config/nextest.toml | … | grep -vc …`: last line 0).
- **Spec-master edits:** none (implement and the operator pass edit no master).
- **Counts / qualifiers moved:**
  - GitHub workflows: 2 → **3** (`ci.yml`, `nightly.yml`, `windows-mutants.yml`; `ls .github/workflows`). Stated as
    two at `registries/contracts/architecture/ci-cd-approach.md:2` ("two workflows"), test-plan.md:1153 ("Every
    workflow (`ci.yml`, `nightly.yml`)"), security-plan.md:341 ("**CI integration** (`ci.yml`, `nightly.yml`)") and
    :344, obs-plan.md:993.
  - zizmor pedantic `concurrency-limits` findings: 2 low → **3 low** (`zizmor --persona=pedantic` over the three files:
    one per workflow). Stated as "(2 low findings)" at security-plan.md:344 and "(2 low)" at test-plan.md:1154. The
    default-persona run prints `No findings to report. Good job! (3 suppressed)` (was 2). No ignore comment was
    added: the suppression is zizmor's default persona.
  - The CI check count of a push run is unchanged at 15 (ci#37174418732: `checks 15/15`). A dispatched run adds six
    `mutants (…)` checks to its sha (`ci.py conclusion --sha 60c569b --name mutants`: `checks 6/21`).
  - Linux-host full `run --mutants --package viola-e2e` wall: 22 m 49 s (m3.md run 4, `immediate`) → 78 m 06 s
    (`evidence/leak.md`, `wait`), with identical counts (711 · 649 caught · 2 missed · 0 timeout · 60 unviable).
- **Dev-tool versions:** none — zizmor re-read at 1.30.1, cargo-mutants 27.1.0 and cargo-nextest 0.9.146 unchanged.
  Read on the dev host for diagnosis only: git 2.55.0, whose `git commit` spawns `git maintenance run --auto --quiet
  --detach`. Not a change.
- **Harness / gate surface:**
  - no harness code changed;
  - the new workflow is a CI surface (above) that drives the existing harness verb `run --mutants --package <member>
    --file …` through the pwsh shim;
  - the nextest `mutants` profile's failure semantics changed (above);
  - `.claude/docs/commands.md` gained the dispatch and read recipe;
  - CI gains no job on push or pull_request, and `ci.yml` / `nightly.yml` are byte-unchanged (`git diff --quiet
    7aca558 -- .github/workflows/ci.yml .github/workflows/nightly.yml`: exit 0).
- **Cross-project / external claims:**
  - CI ci#37174418732 on `60c569b`: `verdict: green · checks 15/15 · wall 309 s`.
  - windows-mutants#37174673472 (`workflow_dispatch`, `headSha` `60c569b40ff5…`): run `completed/failure`; verdict
    row `red · checks 6/21 · failed 5` (report-only, per plan). Per job (`gh api …/actions/jobs/<id>/logs`; the
    harness document + outcome lines):

    | job | tested | caught | unviable | missed | timeout | wall |
    |---|---|---|---|---|---|---|
    | viola-pty | 79 | 64 | 10 | 5 | 0 | 10 m 38 s |
    | viola-channel | 125 | 104 | 17 | 4 | 0 | 9 m 47 s |
    | viola-state | 65 | 57 | 7 | 1 | 0 | 9 m 15 s |
    | viola-agent-claude | 40 | 36 | 4 | 0 | 0 | 5 m 08 s |
    | viola | 131 | 94 | 24 | 13 | 0 | 26 m 49 s |
    | viola-e2e | 68 | 59 | 7 | 2 | 0 | 16 m 12 s |

    Total 508 tested, equal to P4's forecast job for job.
  - Default branch: `build/viola-0.1.0` (research.md, `gh repo view`), so a pushed workflow is dispatchable there
    (`gh workflow list`: `windows-mutants active 374339639`).
- **Reverted / negative API facts:** none. The workflow upload, `inputs:` selector, `concurrency:` block and
  `rust-cache` were never written (plan §Constraints).
- **Insufficient fixes (written, kept, not the remedy):** `terminate = "wait"` (`.config/nextest.toml`).
  - Defect: the mutation-run temp-dir leak (CARRY 3: 25 275 `.tmp*` + 62 nested `cargo-mutants-ws-*` after one
    viola-e2e boundary run).
  - Resolved: the nested-copy half (0) and 99.85 % of the `.tmp*` half (38 left, `evidence/leak.md`).
  - Remainder:
    - 17 dirs from nextest TIMEOUT / mutant-made SIGKILL kills, the one path `wait` cannot drain (by design;
      accepted);
    - 21 half-removed throwaway git repos of the `Pass` fixture (`crates/viola-e2e/src/harness/run/mutants/base.rs:234`)
      with no kill. A `terminate`-independent class: the previous `immediate`-mode run left 26 of them among 461
      repos in the host `/tmp`.
  - Owner of the remainder: `working-route.md:72` (Readiness gate and timing constants) as an `[inferred]`
    HYPOTHESIS — the overseer's disposition, folded by this wrap's route-resolve.
- **Spec claims disproved by measurement:**
  1. The plan's leak acceptance "leaves 0 `.tmp*` dirs" (plan.md §Acceptance Criteria, the leak bullet; its step-5
     prediction 0 · 0 · 0) measured **38 · 0 · 0** (`evidence/leak.md`). DISPOSED by the overseer
     (founder-delegated, verbatim in `leak.md` §Disposition): "ACCEPT the leak acceptance as measured", the 21-repo
     remainder owned by :72.
  2. The 2026-09-24 locked reason for `terminate = "immediate"` — "a caught mutant graded Timeout while running tests
     waited out" — is stated at `registries/contracts/test-plan/test-data-bootstrap.md:15` and
     `…/bootstrap-phases-derive-for-route-setup-project.md:11`. It no longer holds under the current slow-timeout
     kill.
     - Under `terminate = "wait"`, 0 Timeout grades in 711 Linux viola-e2e mutants (`leak.md`) and 0 in 508 Windows
       mutants (`windows-dispatch.md`).
     - Two-sided witness (research.md): 170 leftovers under `immediate` vs 0 under `--max-fail=1:wait`, same 10/10
       caught.
     - The plan names the amendment that follows a REVERSAL of a locked decision (playbook.md:30). Owner of the
       disposition: P2 escalation, routed to the overseer by pre-direction (1).
- **Expected amendments (from plan):** (search: `grep -rnF` over `.andromeda/*.md` + `.andromeda/registries/`,
  sidecars and archives excluded)
  - architecture §CI/CD approach — carried (Counts: workflows 2 → 3; Harness / gate: CI gains no push job). Sites:
    `registries/contracts/architecture/ci-cd-approach.md:2` ("two workflows", "runs no mutation job") and `:5`;
    `architecture.md:379` ("runs no mutation job": 1 hit).
  - architecture §Project directory structure — carried (Files: the two new paths). Site:
    `registries/contracts/architecture/project-directory-structure.md:94` (the `nightly.yml` line: 1 hit; the
    `.github/workflows/` and `tests/` listings live there).
  - architecture §Occupied Resources — carried.
    - The `viola-mutants-scratch` row: `architecture.md:433` (1 hit) gains the runner's checkout parent (the workflow
      runs the Windows `HOST_SCRATCH` arm there; `scratch_bytes: 0` in all six documents).
    - The `AGENT_RUN_CHUNK_BASE` row: `architecture.md:379` (1 hit, "CI runs no mutation job and never sets it"),
      reworded so it stays true: the workflow's `--package` arm reads no base.
  - test-plan §9 Mutation row (`test-plan.md:1146`), §10 Mutation gate (`:1209`) and the §9 tool-pin paragraph
    (`:1153`: "Every workflow (`ci.yml`, `nightly.yml`)", plus the `tool:` lists) — carried (Symbols: the workflow and
    its pins; the contract test single-sources them).
  - test-plan §3 `run` step 2 / Test data bootstrap Cleanup — carried (Schema / config: `terminate = "wait"`; Spec
    claims disproved 2). Sites: `registries/contracts/test-plan/test-data-bootstrap.md:15`,
    `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:11` (`terminate = "immediate"`:
    1 hit each). Escalation, per playbook.md:30 and pre-direction (1).
  - security-plan §Dependency Security (CI integration) — carried (Counts: workflows 2 → 3, `concurrency-limits` 2 → 3
    low). Sites: `security-plan.md:341` (the workflow list), `:344` ("(2 low findings)", "CI runs no mutation job"),
    `:356` ("runs no mutation job"). The unscanned-upload list is unchanged: the workflow uploads nothing.
  - obs-plan §9 Mutation row (`obs-plan.md:1013`) and §9 Platform (`:993`, "the single push/PR workflow … beside the
    scheduled `nightly.yml`") — carried (Symbols: the dispatch workflow is the boundary run's Windows form).
  - Found beyond the plan's list (same search): `test-plan.md:1154` "(2 low)" — carried under the security-plan
    entry's count fact.
- **Coverage of new surfaces:**
  - `windows-mutants.yml` (a CI surface, dispatch-only) → validation: no inputs; `matrix.*` reaches `run:` only via
    `env:` (zizmor template-injection clean) · instrumentation: the job log carries the harness JSON document +
    cargo-mutants outcome lines · PII n/a (repo-relative paths only, absolute-path probe 0 in all six documents) ·
    tests: integ (`contract_windows_mutation_scope`; zizmor; the three grep probes) · a11y n/a · tokens n/a.
  - `contract_windows_mutation_scope` (a test binary) → validation n/a · instrumentation n/a · PII n/a · tests: itself,
    with synthetic known positives + a remove-the-guard pair (`evidence/guard.md`) · a11y n/a · tokens n/a.

## Deviations from intent
- **The scope guard propagates transitively.** Plan step 2's `mod`-line rule did not say whether modules declared
  inside a whole-file Windows module are gated too. Implemented: yes, transitively. That is Windows-only code by
  construction, and the C2 ruling asks for "later ones" to come in without a hand edit. It carries its own synthetic
  known positive (`side.rs` → `side/inner.rs`). No such file exists today, so the equality reads the same 18
  files: 15 gated by an attribute and 3 by a `mod` line (the matrix's 2 + 5 + 2 + 1 + 5 + 3).
- **The leak witness's first attempt failed on setup.** Its 24-character `TMPDIR` subdir pushed a viola-e2e test's
  Unix socket path to 116 B, past `sun_path`'s 107 B, so cargo-mutants' baseline failed (`mutants-exit-4`). Re-run in
  a fresh NOCOW `<repo parent>/viola-mutants-scratch/lw`. Both are recorded in `leak.md`.
- **The remove-the-guard restore re-adds a token to an existing line,** so the "insertions-only diff" is proven by
  `git diff --no-index --word-diff=porcelain` (one `+` token, no `-`) rather than a line diff.
- **Unviable reasons were read from the types, not from build logs.** The 4 unviable owed coordinates' build logs
  stay in the runner's `mutants.out/log/` and the workflow uploads nothing by design. Each replacement needs a
  `Default` its type does not implement (`windows-dispatch.md`).
- **The leak acceptance was unmet (38 ≠ 0),** surfaced by /implement and dispositioned by the overseer (Spec claims
  disproved 1).
- **The workflow's jobs read red by construction:** four of the six file lists carry `#[cfg(unix)]` twins, which the
  Windows build compiles out (19 of 25 misses). That is consistent with C2 (survivors go to the audit; the workflow is
  no gate). The note goes on the Epoch 3 boundary by pre-direction (3).
- scope record: none — `gate.py scope` clean (`changed 3 · listed 3 · recorded 0`), 0 recorded.

## Decisions & corrections
- **Overseer, founder-delegated (after /implement's report):** "ACCEPT the leak acceptance as measured … The 17
  by-design kills are recorded as such. The 21 half-removed fixture git repos are an [inferred] item with a named
  owner: route-resolve folds it into the next chunk (:72), as a hypothesis (git maintenance --auto --detach racing the
  TempDir drop; fix candidate -c maintenance.auto=false) with its own two-sided acceptance." It also asked: "watch each
  job against its 120-minute timeout and record any timeout as such, not as survivors" (0 timeouts; every job under
  27 min).
- **Wrap pre-directions (overseer):**
  - (1) the `terminate = "wait"` escalation goes to the overseer: it reverses a chunk-level 2026-09-24 decision, not
    a founder ruling, and the measured basis (170 vs 0 leftovers; no timeout misgrade) carries it;
  - (2) route-resolve folds the 21-repo item into :72 as an `[inferred]` hypothesis;
  - (3) the audit-classification note goes on the Epoch 3 boundary: unix-only twins make every dispatch read red
    until classified;
  - any widening is held until morning.
- **Decision (implement):** no `-c maintenance.auto=false` fix was taken. It is new behaviour in files outside the
  lists, and its cause is a hypothesis.
- **Sweep / tool hazards found this chunk:**
  - **`gh run view --job <id> --log` blocks the whole run.** It prints "run … is still in progress; logs will be
    available when it is complete" until every job ends. A finished job's log reads through `gh api
    repos/<o>/<r>/actions/jobs/<id>/logs`, which refuses ANSI output without `--allow-escape-sequences`. Strip the
    escapes before grepping.
  - **cargo-mutants on Windows mutates the `#[cfg(unix)]` twins of a scoped file** and grades them MISSED: the
    compiled-out body leaves the binary unchanged. Read every MISSED line's cfg from source before calling it a
    survivor.
  - **A mutation `TMPDIR` has a length ceiling** (≈ 60 B before `.tmpXXXXXX/`). A test binding
    `$TMPDIR/.tmp*/viola-test-chan-<pid>-<label>.sock` hits `sun_path` otherwise.
  - **`ci.py conclusion --sha HEAD` after a dispatch includes the dispatched run** (checks 15 → 21). A CI read of a
    sha must precede its dispatch, as the plan ordered.
  - **The repo's PreToolUse hook blocks a `cat <<EOF >> file` append** (host-win32.md 2026-09-29 class). Use the
    Edit tool.
  - **`gate.py run` redirect paths:** a relative redirect climbing out of the run dir failed before the tool ran.
    Use absolute scratch paths.

## Outcome
Acceptance, re-asserted against the diff:
- (arch, security) `windows-mutants.yml` exists, dispatch-only with no inputs, `permissions: {}` / job
  `contents: read`, `windows-2025`, six-package matrix, `fail-fast: false`, `timeout-minutes: 120`, every `uses:`
  SHA-pinned with its version comment, `persist-credentials: false`; no secret, cache, upload, `concurrency:` or
  `needs:`. ci.yml / nightly.yml are byte-unchanged from `7aca558` — **met**.
- (tests) The workflow drives only `scripts/agent-run.ps1 run --mutants --package <member> --file …`; there is no bare
  `cargo mutants` step — **met**.
- (tests) `binary(contract_windows_mutation_scope)` green with its known positives; remove-the-guard red named
  `crates/viola-pty/src/sideload.rs`, then green after the restore (`evidence/guard.md`) — **met**.
- (tests) The leak — **unmet as written, ACCEPTED as measured** by the overseer:
  - `.config/nextest.toml`'s only non-comment change is `terminate = "wait"` (met);
  - verdict `missed == 0` over the measurable set, `timeout == 0`, `unviable` 60 ≤ `caught` 649 (met);
  - 0 nested copies (met); 38 `.tmp*`, not 0 (unmet → disposed, remainder owned by :72);
  - no sweep or cleanup step added (met).
- (tests, obs) The 34 coordinates (run 37174673472) — **met**:
  - 30 caught · 4 unviable (`viola-pty lib.rs:290:9`; `viola-channel client.rs:177:5`, `:182:5`, `server.rs:83:5`;
    reasons read from the types) · 0 missed;
  - `src/cmd/run.rs:318:5` still not measurable;
  - the nine `src/panic_frames.rs` grades (all caught) close obs-code mutation for this boundary.
- (tests) Survivors outside the 34 are listed by coordinate as Epoch 3 audit items (`windows-dispatch.md`): 4 not
  measurable · 19 host-excluded `cfg(unix)` · 1 Windows-equivalent (`viola-state fs.rs:17:5`) · 1 shared-body Windows
  survivor (`viola-e2e cleanup.rs:106:35`) — **met**.
- (obs, security) The absolute-path probe reads 0 in all six documents; 0 uploads — **met**.
- (tests) `pre-push` `"ok":true` at `linux-tests`; the final HEAD `60c569b`'s CI run reads `verdict: green` 15/15;
  no `#[ignore]`, retry or skip added — **met**.
- (tests) `.claude/docs/commands.md` carries the dispatch and read recipe — **met**.
- No verification-matrix capability linked (`matrix.py show --chunk …`: claimed 0) — **met**.

Gates (/implement P2, run dir `2026-10-04T02-02-33-implement`, all on the first call):
- `cargo fmt --all --check` — green (exit 0)
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green
- `bash scripts/agent-run.sh run --integration --filter 'binary(contract_windows_mutation_scope)'` — green (exit 0,
  `"ok":true`, 3 passed)
- `bash scripts/agent-run.sh run --integration --filter 'binary(contract_lints)'` — green
- `bash scripts/agent-run.sh run --unit` — green
- `bash scripts/agent-run.sh run` — green
- `zizmor .github/workflows/` — green (`No findings to report. Good job! (3 suppressed)`)
- `! grep -nE 'secrets\.|upload-artifact|…' .github/workflows/windows-mutants.yml` — green (no output)
- `test -f … && grep -E '…uses:' … | grep -cvE '@[0-9a-f]{40} # v[0-9]'` — green (exit 1, last line 0)
- `grep -cE '^permissions: \{\}$|…' .github/workflows/windows-mutants.yml` — green (last line 4)
- `git diff --quiet 7aca5587bf93 -- .github/workflows/ci.yml .github/workflows/nightly.yml` — green
- `grep -c 'fail-fast = { max-fail = 1, terminate = "wait" }' .config/nextest.toml` — green (last line 1)
- `git diff -U0 7aca5587bf93 -- .config/nextest.toml | … | grep -vcE …` — green (exit 1, last line 0)
- `! (git diff 7aca5587bf93 -- '*.rs' .config/nextest.toml | grep -E '^\+.*(#\[ignore|retries *=|test\.skip)')` —
  green
- `bash scripts/agent-run.sh pre-push` — green (`"ok":true`, `"stage":"linux-tests"`; re-run in the operator pass:
  coverage 962 · 0, playwright 1 · 0, gate `breaches:[]`)
- `python -X utf8 …/gate.py hygiene` — leg operator: fired by the operator pass, `hygiene: clean — read 35 (runs 32 ·
  evidence 3)` (`evidence/operator-pass.md`)
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` — leg operator: `7aca558..60c569b`
  (`operator-pass.md`)
- `python -X utf8 …/ci.py conclusion --sha HEAD --wait 1800` — leg operator: `60c569b40ff5 verdict: green · checks
  15/15 · wall 309 s · ci#37174418732` (`operator-pass.md`)
- Smoke: skipped — no boot-path or UI-surface change; the plan lists no smoke or self-verify entry.
- No `defer` entry; no deferral.

Watches: none folded.

Outcome basis: the operator pass ran.
- Setup 4's commit list: `60c569b` (the pre-CI commit) and no fix commit. Step 9 did not fire: 0 of the 34 missed.
- The final HEAD's CI run: ci#37174418732 green, in `evidence/operator-pass.md`.
- The dispatch: `evidence/windows-dispatch.md`. The leak witness and disposition: `evidence/leak.md`.
- Implement's P4 report (this conversation) for the gate block. The overseer's disposition between implement and
  this report changed the leak acceptance's status to accepted, with an owner.

Process hygiene (implement P4 census plus the operator pass, re-measured now via `pgrep`):
- the gate run's children, the two cargo-mutants runs and their nextest children, the scratch `git` traces (and
  their detached maintenance), the `gh run watch` waiter and the job monitors — all terminated;
- none left running from this chunk.
