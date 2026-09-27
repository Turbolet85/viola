# Fan-out results — 2026-09-27-epoch-2-cleanup wrap

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim (substituted, self-checked). Entity probe over the
three raw twins: `entities=0`.

| doc | verdict | proposals | twin |
|---|---|---|---|
| architecture | proposals | 10 | `.raw-fanout-architecture.md` |
| security-plan | `proposals: []` (notes: the release-check refusal and the scratch guard are expected amendments left to the orchestrator) | 0 | — |
| design-system | `proposals: []` | 0 | — |
| layout-templates | `proposals: []` | 0 | — |
| test-plan | proposals (+ note: :1450/:1484 still say `3/3`) | 17 | `.raw-fanout-test-plan.md` (condensed) |
| obs-plan | proposals | 2 | `.raw-fanout-obs-plan.md` (condensed) |
| a11y-plan | `proposals: []` | 0 | — |

## Validate (orchestrator)

Re-derivation tell: none (every rationale cites the report or the doc).

| # | proposal | playbook | disposition |
|---|---|---|---|
| A1 | arch Repository: host mutation scratch | Accurate this-chunk addition → routine | apply |
| A2 | arch CI/CD: Windows TMP/TEMP + `--output` beside the Linux TMPDIR | Accurate this-chunk addition → routine | apply (see E1: the TMP/TEMP assignment is host→host, no boundary crossed — named in E1 for the operator) |
| A3 | arch CI/CD: drop "stop rust-analyzer →" from the operator pass order | Accurate addition (mechanism reconcile) → routine; intent: plan Test Commands state the stop is needed only until step 7 | apply, scoped to what was measured (both 2026-09-27 pre-pushes + the scoped runs green with it running) |
| A4 | arch Repository: `target/run-archive/<n>/` | routine | apply |
| A5 | arch Filesystem: test-only `<temp dir>/viola-pty-watch/` | routine (arch registers test-only resources) | apply |
| A6 | arch Env vars: `AGENT_RUN_KEEP_FAILED` second reader | routine (retires "Read only by the root test chain", :368) | apply |
| A7 | arch Workspace crates: `test-support` feature | routine | apply |
| A8 | arch CI/CD job 6: release-check refuses test-only features, probe 5/5 | routine | apply |
| A9 | arch directory structure: `scripts/wsl-exec.sh` + the split submodules | submodules routine; the wsl-exec.sh line waits on **E1** | submodules apply; wsl-exec.sh per E1's answer |
| A10 | arch Stack/Code quality: name cargo-machete | Registry over-reach / not this chunk's stack decision → reject | reject: cargo-machete is the code audit's instrument (`.andromeda/code-metrics.ndjson` `commands.dead`), runs only inside the audit, not CI (scope item 6); the chunk annotated a false positive, it did not adopt a tool |
| T1–T16 | test-plan §2/§3/§10/§12 harness contract (scoped, scratch, archived, pre-push fields, Booted reader, closed enums, Decisions Log) | Accurate this-chunk addition → routine | apply as one dependent-of group |
| T17 | test-plan §4: labelled case table accepted in viola-e2e | Accurate addition (the overseer accepted deviation 2) → routine | apply, narrowly (viola-e2e only, no dev-deps) |
| O1 | obs §8 item 6: pre-push cache fields path-free | Accurate this-chunk addition → routine (a registration, not a PII violation; D-obs-pii's escalate is its default severity) | apply |
| O2 | obs §8 item 6: host scratch + `target/run-archive/` never uploaded | routine | apply (re-derived: "absolute argv paths" only, per §8's own wording; no "captured test output" claim the report does not carry) |

- Check 2 (cross-contradiction): none.
- Check 3 (intent): consistent with the working-route entry and the plan; the deviations carry justifications, accepted by the
  overseer.
- Check 4 (absence): A6's "no further occurrence" was self-swept by the agent; the cascade sweep re-checks.
- Check 5 (expected amendments), orchestrator-raised where no detector proposed:
  - **S1** security-plan: the release-check feature refusal (Decisions Log 2026-09-27 Conditions, :689). The report substantiates
    it → routine, apply.
  - **S2** security-plan: the scratch wipe outside the repo and its guard. The report substantiates it → routine, apply beside
    §Secret Management :426's WSL/TMPDIR text.
  - arch, test-plan §3 and obs §8 item 6 are covered by A1–A9, T1–T16 and O1–O2.
- Check 6 (disproved claims):
  - (a) the clones 3 → 4, (b) ~165 → 200 mutants, and (c) step 6's list: plan-only claims with 0 master hits → disposed:
    recorded in evidence; (a) and (c) route to curation.
  - (d) `3/3 refused`: orchestrator-raised **T18** (test-plan :1450, :1484 → `5/5`, routine); the `.claude/docs/commands.md:73`
    leaf is re-derived in cascade step 3.

## Escalation (HALT)

- **E1 — boundary widening: `scripts/wsl-exec.sh` is a second launcher into the WSL2 distro.** security-plan §Secret Management
  :426 states the pre-push gate launches every WSL call with `env -i` and no host value. `wsl-exec.sh` keeps `env -i` (HOME and
  PATH from the distro; `--probe` proves no `CLAUDE*` name crosses and argv arrives unconverted). But it carries operator-typed
  argv and a `--cd DIR` from the host into the distro: a new crossing. Class: playbook "Boundary widening" (never routine). A
  direction written before the widening was shown (the P4/P11 plan item) does not ratify it. It needs the founder's live answer.
  - **Resolved 2026-09-27: ratified as an operator aid — the overseer's live answer, given under the founder's 2026-09-27 ruling
    (recorded as the overseer's, not the founder's own).** Merits (overseer): only what the operator types crosses, which is the
    tool's purpose; `env -i` keeps every host environment value out. Checkable invariant written into security-plan: no gate,
    harness or plan entry may invoke `wsl-exec.sh`; a future gate use is a new widening that halts again. A9's wsl-exec.sh line
    → apply.
  - **Clarified live by the overseer (second question at this wrap):** the invariant covers RUNNING a distro command through it; its
    own `wsl-exec.sh --probe` self-test is exempt and stays in the light gate (this chunk's plan entry), like `release-check.sh
    --probe`. Callers measured: 0 in `crates/`, `src/`, `.github/`, `scripts/agent-run.*`; 1 plan hit, the probe entry.
  - Considered and NOT a widening (named for the operator): the host `cargo mutants` child's `TMP`/`TEMP` (host → host, no
    boundary crossed); the scratch wipe (a guarded host-local path, S2); the `test-support` feature (release-check proves it absent).
