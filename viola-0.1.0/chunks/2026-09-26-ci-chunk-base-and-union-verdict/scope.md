# Scope — CI chunk base and union verdict

**Marker:** `2026-09-26-ci-chunk-base-and-union-verdict` · **Epoch:** Epoch 2 — Windows slice I: wrapper, events, ledger
**Working entry:** "CI chunk base and union verdict — whole-chunk CI base surviving the operator pass, per-leg union
mutation verdict, owned secret-scan residue scope"

## Why (founder ruling, 2026-09-25)
CI round-trips cost more than the code. After this chunk, every CI run of an operator pass reads the WHOLE chunk
(V15, instance half), so a fix push re-reads everything rather than its own delta. V17 is live in the pipeline: fix
commits after the operator pre-CI commit are sanctioned as `fix({marker}): operator fix after CI run {id}, …`,
fast-forward only.

## What this chunk builds

### (a) A CI chunk base that survives the operator pass
- The mutation leg's chunk base is derived from the repository's own history, not from the push event.
  Today `.github/workflows/ci.yml:166` sets `AGENT_RUN_CHUNK_BASE: ${{ github.event.pull_request.base.sha ||
  github.event.before }}`, so a fix push's leg reads only that push's delta. (CARRY, operator adaptation 2026-09-25.)
- The base is the last master-flip commit, the pickaxe /andromeda-phase Setup 5a uses (`git log -1 --format=%H -G
  ' · complete · ' -- .andromeda/master-route.md`; it read fcca1ce at the adaptation). The CI checkout already
  fetches full history (`fetch-depth: 0`, ci.yml:155). (CARRY.)
- The CARRY's hypothesis ("`-G` selects any commit whose diff adds or removes a `· complete ·` line, so a mid-pass edit
  of a complete master record would move the base") is measured true at this phase (research.md §Measured, scratch
  probe `base_probe.sh`: the HEAD pickaxe moved to the fix commit that edited a complete line).
- The chosen rule cannot move mid-pass: the last master-flip commit reachable from the OLDEST operator pre-CI commit's
  parent (overseer direction 2). The same probe measured it holding the flip through every commit of the pass. Before
  any pre-CI commit exists (no pending record, or a pending chunk not yet pushed), the rule reads the pickaxe from HEAD.
  On the wrap push, whose own commit is the flip, the base is that commit's parent, so the push reads its own delta
  and never an empty diff (val-1 fold from plan step 1c; test-plan §3 Base forbids a vacuous pass).
- The `run --mutants` document names the base it read (`"base"`, a commit id), so every run of a pass can be shown to
  read the same whole chunk (val-1 fold from plan step 2; overseer direction 1).
- A probe proves it: a simulated pass (flip commit → pre-CI commit → a fix commit, including one that edits a
  `· complete ·` master line) resolves to the same base on every commit of the pass.
- The base is resolved in ONE place, the harness's `resolve_base` (`crates/viola-e2e/src/harness/run/mutants.rs:34`;
  today `AGENT_RUN_CHUNK_BASE`, else the merge-base with `origin/main`), so a local run and CI read the same rule;
  ci.yml:166 stops passing the event-derived value (research.md §Files to modify).
- test-plan states the rule (a wrap amendment, listed in the plan's Expected amendments). (CARRY.)

### (b) The per-leg union mutation verdict
- The union rule (CARRY, chunk 2026-09-25-pty-wrapper-on-windows, overseer direction 2): a mutant unviable on the leg
  that compiles its code is not a survivor, whatever the other leg reads (`gate --mutants-legs`). The per-leg union
  is the project harness's to define (overseer1).
- It closes run 36165685381's breach: `HostTerminal::enter -> Some(Default::default())`, reported at
  `crates/viola-pty/src/lib.rs:395` / `:420` (the two cfg variants; `fn enter` at :394 / :418 on HEAD fcca1ce),
  ratified with no product change at that wrap. (CARRY.)
- A witness on the recorded 36165685381 breach shape (overseer direction 3). The shape is measured from that run's own
  leg files: `lib.rs:395:9` windows unviable / ubuntu missed; `lib.rs:420:9` ubuntu unviable / windows missed
  (research.md §Measured).
- A negative (overseer direction 3): a mutant MISSED on a leg that compiles its line stays a survivor.
- [premise-corrected: outcomes alone cannot separate "compiled out" from "missed on a compiling leg" — research.md
  §Measured] The gate needs to know which legs COMPILE each mutant's line; the leg files carry names and outcomes only.
  The `mutants-verdict` job checks out the same sha, so the source is at hand where the union runs. The workspace's
  lib/bin code uses five cfg predicates (`test`, `unix`, `windows`, `not(unix)`, `not(windows)`), at fn, field and
  statement level, with no cfg-gated `mod` declaration. The mechanism is P4's decision.

### (c) secret-scan residue scope
- CARRY, chunk 2026-09-25-pty-wrapper-on-windows: `secret-scan` (`scripts/agent-run.sh secret-scan`, ci.yml:100) read
  `target/agent-run/chunk.diff` residue left by a prior `run --mutants`, where it hit its own `?t=` pattern line (that
  chunk's report.md:124). It had no owner; this chunk sets the scan's scope.
- [premise-corrected: the hit is not staleness — `chunk.diff` is repo source text by construction, so a fresh diff that
  touches `secret_scan.rs` hits too; and the `harness-<os>` upload ships the whole `target/agent-run/` (ci.yml:115) —
  research.md §Files inspected] The scan stops reading the mutation leg's `chunk.diff` (overseer direction 4), and the
  `harness-<os>` upload stops shipping it, so the scan still covers every file that upload ships. The planted-secret
  red tests stay. Today `scan` collects every file under `target/agent-run` (`secret_scan.rs:78`).

## Operator pass (overseer direction 1)
- Planned from the start under V17 (overseer direction 1; the pty chunk's pass ran this shape — its evidence/operator-pass.md): the pre-CI commit, then any fixes as `fix({marker}): operator fix after
  CI run {id}, …`, fast-forward only; with (a) landed, each run of the pass reads the whole chunk.

## Standing (overseer direction 5)
- Every guard gets a remove-the-guard run; the mutation leg ends 0 missed / 0 timeout; the swamp and test-only
  verdicts hold. Fold, do not carry.

## Boundaries
- NOT this chunk: the local Linux pre-push gate (the next entry, "Local Linux pre-push gate").
- No product-crate behaviour change; the subject is CI, the harness (`viola-e2e`) and test-plan.
- The macOS mutants leg stays out (the "Unix endpoint and home hardening" CARRY owns adding one).

## CI verdict at take-up (Setup 5a)
- Base fcca1ce (the last flip), shas fcca1ce and e486544: both green, 15/15 checks (runs 36171941029, 36172787056).
  Nothing to disposition.
