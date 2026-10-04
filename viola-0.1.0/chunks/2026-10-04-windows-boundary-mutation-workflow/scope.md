# Scope — Windows boundary mutation workflow

**Marker:** `2026-10-04-windows-boundary-mutation-workflow` · **Version:** viola-0.1.0 · **Epoch:** Epoch 3 — Windows slice II: driving verbs and live proof
**Working entry:** `viola-0.1.0/working-route.md:70` — "Windows boundary mutation workflow — cfg(windows) mutants scored on the windows runner by a manually dispatched, non-blocking workflow at each epoch boundary", with three CARRY blocks (all three folded below; `route.py pins` indexed 3 freight blocks on :70, 712 · 499 · 572 chars).
**Host:** Linux (Omarchy, btrfs) since 2026-10-03; Windows is witnessed only by the `windows-2025` CI runner.
**Mode:** autonomous (operator directive at take-up): a boundary widening is shown at P4 and answered HOLD by the overseer, never ratified; the founder decides in the morning.
**P3 closure:** every `[inferred]` bullet below is closed against `research.md` (verified, tag dropped and evidence named; or `[premise-corrected: …]`).

## What this chunk builds

A manually dispatched, non-blocking GitHub Actions workflow that scores the project's `cfg(windows)` code under
cargo-mutants on the `windows-2025` runner, run at each epoch boundary audit; its first run grades the 34 mutants the
previous chunk could not measure on the Linux host; and the fix for the mutation run's temp-dir leak.

### 1. The workflow (CARRY 1 — founder ruling C2)
- Source: founder ruling 2026-10-04, relayed by the overseer (`additional/viola-overseer/linux-route-adaptation.md`
  :31-35, item C2 — re-read at P1, wording matches the CARRY), placed at `:70` by the 2026-10-04 0-pending wrap
  (overseer founder-delegated) so `Mutation scoring completion` stayed within one window; the Epoch 3 boundary audit is
  its first consumer.
- Trigger: `workflow_dispatch` ONLY — run only during the boundary audit. No `push`, `pull_request` or `schedule`
  trigger.
- It keeps the 2026-09-28 ruling (testing.md): no mutation entry in a chunk's `[[gate]]` block, no mutation leg in
  `pre-push`, no blocking mutation job in CI. The workflow is never a required check and never a dependency of `ci.yml`.
- Runner: `windows-2025` (CI's only Windows runner).
- Scope of what it mutates: the `cfg(windows)` code — the sideload DLL search, the pin/ACL paths, the frozen-wrapper
  stale check, and later ones. "And later ones" means a `cfg(windows)` item added after this chunk must come under the
  workflow without a hand edit being forgotten. VERIFIED as an open design question with its inventory
  (research.md §Measured facts): 18 `src/` files carry a Windows gate in an attribute, and three more are WHOLE-FILE
  Windows modules gated at their `mod` line (`crates/viola-pty/src/sideload.rs`, `crates/viola-channel/src/server/win.rs`,
  `src/conpty.rs`) that a per-file grep cannot see; file-scoped they hold 562 mutants, whole packages 1 626. The
  selection form is a P4 fork.
- Its survivors become corrective-chunk items like any boundary survivor (the boundary audit dispositions them; this
  workflow decides nothing).
- Cost: about one long runner job per epoch (measured per-mutant cost on the retired windows-2025 leg: 5.1–12.6 s).
  Val-1 (intent-incomplete, P5): the overseer ruled at P4 that "one long job" was its cost estimate, not part of the
  founder's ruling; the shape is a per-package matrix with `fail-fast: false`, about +30 runner-minutes for isolation
  and per-package logs.
- It drives the project's own `run --mutants` harness verb, not a hand-rolled cargo-mutants invocation. VERIFIED:
  `run --mutants --package <member> [--file …]` exists (`mutants.rs:134-176`); every member with Windows code declares
  `fake-agent` (the `package_member` precondition); on Windows `HOST_SCRATCH` routes cargo-mutants' copy and `--output`
  to `<repo parent>/viola-mutants-scratch`, wiped before each run; the retired windows-2025 leg ran the same prebuild +
  `--copy-target=true` form, so runner disk and the path limit held.
- Workflow hardening per security.md §Dependencies and CI: actions pinned by full commit SHA, workflow
  `permissions: {}`, job `contents: read`, `zizmor` clean, no secret, no credential on the runner. VERIFIED: ci.yml's
  `supply-chain` zizmor step globs `.github/workflows/` (`ci.yml:481`), so the new file is covered with no ci.yml edit;
  the pinned zizmor 1.30.1 is on this host. The cargo-mutants / nextest pins it installs must equal ci.yml's test-job
  `tool:` line (their single source, `pre_push/linux.rs:115-128`).
- The result must be readable by the auditor after the run. VERIFIED, in a form that needs no artifact: the harness's
  stdout run document (`mutants.tested`, `suites[].survived`, `suites[].failures` = the repo-relative missed/timeout
  lines) lands in the job log, and cargo-mutants' outcome lines stream to stderr there too; no upload, so no new
  disclosure crossing.
- `workflow_dispatch` can be dispatched only for a workflow file GitHub knows on the repository's default branch.
  VERIFIED: the default branch is `build/viola-0.1.0` (public repo), so the operator pass's push of this chunk's tree
  makes the workflow dispatchable from that branch.
- The dispatch recipe (`gh workflow run …`, reading the run back) is documented where the boundary audit reads it.
  VERIFIED as a two-part landing: `.claude/docs/commands.md` (implement); the spec text — architecture §Infrastructure
  Patterns → CI/CD approach ("two workflows", "CI runs no mutation job"), the directory tree, test-plan §9 Mutation row,
  security-plan §Dependency Security CI integration, obs-plan §9 Mutation row — is wrap amendment.

### 2. The 34 owed coordinates (CARRY 2 — overseer disposition at the previous chunk)
- Owed by `2026-10-03-mutation-scoring-completion`, by coordinate; re-verified at P1 against its evidence:
  - viola-e2e, 2 (`evidence/m3.md` :73-76): `crates/viola-e2e/src/harness/run/mutants/scratch.rs:48:5`
    (`prepare → Ok(None)`) and `:54:8` (delete `!` in `prepare`), both behind `HOST_SCRATCH = cfg!(windows)`; their
    named Windows witnesses are `prepare_empties_the_scratch_and_reports_what_it_held` and
    `prepare_refuses_a_repo_whose_scratch_is_an_ancestor` (m3.md :82-83).
  - viola-pty, 10 (`evidence/cfg-unix.md` :52-56): `lib.rs:234:5` ×2, `:242:46`, `:290:9` ×2, `:304:54`, `:304:59`,
    `:304:95`, `:309:10`, `:355:87`.
  - viola-channel, 13 (cfg-unix.md :75-80): `client.rs:163` ×7, `client.rs:177:5`, `:182:5`, `:211:19`, `:215:12`,
    `server.rs:83:5`, `:94:25`.
  - `src/panic_frames.rs`, 9 (cfg-unix.md :98-100): `:37:5` ×3, `:50:5` ×5, `:55:79`.
  - Count: 2 + 10 + 13 + 9 = 34, matching the CARRY.
- Each is graded by a run of THIS chunk's workflow on the windows runner: caught, unviable (with its build-log reason),
  or missed → killed by a new/strengthened test, or recorded equivalent with its argument (the M1 disposal form). Never
  counted as caught until measured there.
- Line coordinates are as of `7aca558`. VERIFIED at HEAD (= `7aca558`): each site read carries the named body or
  operator; the four files last changed in `80b69cd`, the tree the previous chunk measured.
- Out of scope, recorded: the 13th audit coordinate `src/cmd/run.rs:318:5` (`cfg(all(windows, not(target_arch =
  "x86_64")))`) stays NOT MEASURABLE — `windows-2025` is x86_64, so this workflow does not compile it either
  (cfg-unix.md :111-123). It is not owed here and is never counted as caught.
- The first dispatch also grades Windows code beyond the 34 (viola-state, viola-agent-claude, the sideload, conpty, the
  rest of the gated files). Per C2 those survivors are Epoch 3 boundary-audit items, not this chunk's kills.

### 3. The mutation-run temp-dir leak (CARRY 3 — [inferred] at the previous chunk, owner named at its wrap)
- Mechanism, verbatim from the CARRY: "the mutants nextest profile terminates every running test at once on the first
  failure, so `TempDir` drops and the `Booted` guard never run — one viola-e2e boundary run left ~25k test temp dirs
  plus 62 nested `cargo-mutants-ws-*` copies in `TMPDIR` (pre-existing)." VERIFIED for the temp-dir half by a two-sided
  local witness (research.md §Measured facts): the same 10-mutant viola-e2e run left 170 `.tmp*` dirs under the
  committed `terminate = "immediate"` and 0 under `--max-fail=1:wait`, 10/10 caught on both sides, +5 s per caught
  mutant. Counts re-derived: 25 275 `.tmp*` + 62 `cargo-mutants-ws-*` in the scratch, all in the runs' time window.
  The nested-copy half: its producer is the two real-cargo-mutants self-tests (`mutants.rs:389-430`); not reproduced in
  the 10-mutant witness (0 on both sides), so the full-run witness below is what proves it.
- Acceptance (from the CARRY): a before/after count of `TMPDIR` across one boundary run reads no leftover test temp dir
  and no nested copy, with the mechanism fixed rather than a cleanup step bolted on.
- The boundary run that witnesses it is a Linux-host `run --mutants --package viola-e2e` run with a fresh NOCOW
  `TMPDIR`. VERIFIED as the right host: the leak was measured there; on Windows the harness's own scratch wipe before
  every run already bounds it to one run on an ephemeral runner.

## Boundaries
- No change to `ci.yml`'s blocking jobs (the zizmor gate already covers a new workflow file).
- No mutation entry in this chunk's `[[gate]]` block and no `pre-push` leg (the 2026-09-28 ruling). The workflow's
  own dispatch is an operator act (it needs a push and a dispatch), not a gate.
- No reshaping of `cfg(windows)` code to let a Linux test reach a mutant (testing.md 2026-09-28).
- No Claude credential and no secret on any runner.
- No artifact upload from the new workflow (an unscanned mutation upload would be a disclosure widening).

## CI verdict since the last wrap (Setup 5a)
- `7aca5587bf93` (the 2026-10-03-mutation-scoring-completion wrap commit; the only sha from the last flip through HEAD):
  read 2026-10-04T01:33Z `verdict: in progress` (ci#37168195381, oldest running `test (windows-2025)` 132 s); re-read
  at P3: **`verdict: green` · checks 15/15 · wall 315 s** · ci#37168195381 completed/success. Nothing to disposition.
