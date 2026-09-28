# Scope — 2026-09-28-mutation-testing-to-the-epoch-boundary · Mutation testing to the epoch boundary

**Source:** `viola-0.1.0/working-route.md:55` (Epoch 2b — Windows slice I b: events and ledger), taken up 2026-09-28.
**Working entry (verbatim title + hint):** Mutation testing to the epoch boundary — per-chunk mutation gate retired:
pre-push mutation leg, CI mutation jobs and mutants-verdict removed; cargo-mutants kept for code-audit

## Take-up direction (overseer relay, 2026-09-28, at this chunk's `/andromeda-phase` invocation)
- **Founder live ruling (~21:50): NO epoch split.** Epoch 2b stays one epoch; this entry is not preceded by a boundary.
  (Relay: the Viola overseer. Recorded here as relayed; the wrap carries it as the founder's word, with no artifact on
  disk behind it beyond this relay.)
- **Keep:** cargo-mutants itself and the harness mutants arm (`agent-run run --mutants`, its scratch/base/leg modules)
  WORKING, because `/andromeda-code-audit` runs mutation testing at the epoch boundary through them.
- **Remove:** only the per-chunk gate, the pre-push mutation leg and the CI mutation legs (with `mutants-verdict`).

## CI verdict read at Setup (Setup 5a)
- `537ac36` (the last wrap's flip = HEAD) · **green** · checks 18/18 · wall 309 s · ci#36473870288 completed/success.
  Agrees with the overseer's relay. No red to disposition.

## What it builds
Moves mutation testing out of every chunk and out of CI, to the epoch-boundary code audit (founder ruling
2026-09-28 17:59, the entry's first CARRY). The mutation MACHINERY stays; only its per-chunk and per-push USE goes.

- **CI:** remove the `mutants (${{ matrix.os }})` matrix job and the `mutants-verdict` union job from
  `.github/workflows/ci.yml`, and anything that `needs:` them or names them (a required-check list, a summary job).
- **Pre-push:** remove the mutation leg from `viola-harness pre-push` (the local ubuntu/windows union: the Linux leg in
  WSL, the host windows leg, the copy-back of `mutants-verdict-<leg>.json`, the `gate --require mutants` judgement over
  them) — the remaining pre-push stages keep running unchanged.
- **Per-chunk gate:** retire `gate --require mutants` / `--mutants-legs` as a per-chunk requirement, and whatever in the
  harness exists ONLY to feed that gate (the union judge in `harness/gate.rs`, the `cfg_legs` line-to-leg map) —
  each such piece is either deleted or kept because the code-audit use still needs it. [verified: the graph's callers
  (research.md §Graph impact) put every production caller of `union` / `compiled_legs` / `leg_verdict_path` /
  `parse_legs` / `run --leg` in `gate_cmd --mutants-legs`, pre-push `stages`/`linux_leg` and the `run --mutants` leg
  write; `/andromeda-code-audit` C1 runs `cargo mutants -p {unit}` itself and names a project union only by pointer
  (collectors.md §Host-excluded mutants) — so no audit caller exists; whether to delete them is P4's fork]
- **Default selection** (validation-1, intent-incomplete — added at P5): `run` with no selector or `--all` selects
  mutants today (`crates/viola-e2e/src/harness/run.rs:51-61`), which makes every chunk's default `run` a mutation gate.
  Mutants leave the default and `--all`; they run only when `--mutants` is named.
- **Leg/union machinery — P4 fork, answered:** delete (`run --leg`, the `mutants-verdict-<leg>.json` writer,
  `gate --mutants-legs` and its union, `harness/cfg_legs.rs`, `syn` / `proc-macro2` in `viola-e2e`). The overseer,
  2026-09-28: "It has no caller left, and the founder rule is no code without a real consumer … the specs must stop
  describing it."
- **Kept working:** `agent-run run --mutants` (scoped `--file` and whole-workspace), its scratch dir, base handling,
  the per-leg verdict artifact if the audit reads it, and the harness's own tests of that arm — still run in the `test`
  job, which keeps installing `cargo-mutants@27.1.0` for them [verified: ci.yml:40-41; the `msrv` job's install at
  ci.yml:343 serves the same tests under `run --unit`].
- **Spec + docs:** test-plan §3 (the harness command set) and §10 (Quality Gates) no longer name a per-chunk or CI
  mutation gate; they name the epoch-boundary audit as mutation's home. These are spec masters: phase does not amend
  them — they are **expected wrap amendments** the plan lists. The project docs and rules that describe the per-chunk
  gate (`.claude/rules/testing.md`, `verification-harness.md`, `.claude/docs/commands.md`, `workflow.md`,
  `tests-summary.md`, `security-summary.md`, `obs-summary.md`) are brought to the new truth
  [premise-corrected: by the WRAP's cascade, not by /implement — prior chunks' research lists these files as
  wrap-cascade leaves, never implement touchpoints (e.g. chunk 2026-09-24-quality-gates research.md:51); the plan
  names them as expected wrap re-derivations].

## Folded freight (the entry's five CARRYs — hypotheses until P3 re-verifies each named artifact)
1. **Founder ruling 2026-09-28 17:59** (overseer relay; placed at the capability-ledger wrap): mutation testing leaves
   chunks and CI for the epoch-boundary audit — remove the pre-push mutation leg, the CI mutation jobs and
   `mutants-verdict`, amend test-plan §3 and §10 Quality Gates; cargo-mutants stays for code-audit. → the body above.
2. **The macOS 73 s** (chunk 2026-09-28-capability-ledger-and-viola-verify): the `test (macos-latest)` harness mutants
   tests `run_mutants_passes_when_the_change_is_tested` and `run_mutants_reports_survivors_of_an_untested_change` (kill
   line 120 s) still spend ~73 s on the cold compile of the one-file throwaway crate after a private `CARGO_HOME`
   removed the package-cache lock wait (ci#36448654074, 82 s per test; that chunk's
   `evidence/macos-mutants-baseline-carry.md`). **This entry owns the 73 s.**
   - *hypothesis, unmeasured (verbatim from the CARRY):* "the nested cargo inherits the outer `cargo llvm-cov
     nextest` jobserver or coverage `RUSTFLAGS`". [verified as a runner-only observation, WORSE at HEAD: ci#36473870288
     on `537ac36`, `test (macos-latest)` JUnit — `run_mutants_reports_survivors_of_an_untested_change` 108.112 s,
     `run_mutants_passes_when_the_change_is_tested` 108.376 s (12 s under the 120 s kill); the same run's ubuntu leg
     1.349 / 1.382 s and windows leg 5.787 / 5.850 s under the same `run --coverage`. The mechanism stays a HYPOTHESIS:
     the ubuntu leg inherits the same coverage environment and is fast, so any cause is macOS-specific; a second
     candidate from the platform sources — macOS XProtect scanning each newly built executable on first launch, one at
     a time (research.md §Platform issues consulted) — fits the measured 0.02 s co-finish of the two baselines. The plan
     measures the phases on the macOS runner before it changes the work (testing.md 2026-09-28).]
   - Since the harness arm is KEPT, these two tests stay; the 73 s is fixed at its cause or measured and recorded, not
     removed with the gate.
   - **Operator bound at the P5 review (2026-09-28):** at most ONE measurement push. A cause the repo can remove is
     removed. A runner-side cause (XProtect or similar) is not looped on. Those two tests are excluded on macOS with the
     measured reason, because the mutants arm's only consumer is the code audit on the Windows host. The kill bound
     stays unchanged.
3. **WSL `--install-deps` root hardening** (chunk 2026-09-27-browser-verdict-reachability, overseer live ratification,
   operator-only): binds the chunk that first RE-PROVISIONS the WSL distro. This chunk re-provisions nothing:
   `wsl-provision.sh` installs `cargo install --locked` of ci.yml's `test`-job line, and that line keeps
   `cargo-mutants@27.1.0` (the harness tests need it), so no provisioning input changes [verified: ci.yml:41 is the
   `test`-job line; `wsl-provision.sh` is not in the modify set; the pre-push `tools` stage keeps its cargo-mutants pin
   check because the distro's `run --coverage` runs the same real-cargo-mutants harness tests]. If the test-job line or
   `wsl-provision.sh` does change after all, this CARRY is this chunk's, whole. Otherwise it moves on with the next
   markerless entry (the wrap re-places it).
4. **Host-scoped `run --mutants --file` expectations** (chunk 2026-09-28-cli-output-tokens, operator ruling E1): a
   plan's host-scoped mutants entry expects 0 missed only when every regenerated mutant's killer compiles on the host.
   With per-chunk mutants entries gone, this rule's subject moves to the code audit's reading of the union. → this chunk
   settles it: retire it, or rewrite it as code-audit guidance in `verification-harness.md`.
5. **Epoch 7 "Unix endpoint and home hardening" mutation-union CARRYs** (working-route :112 — a `macos-latest` mutants
   leg for `mutants-verdict`; the pre-push union's missing macOS leg): with `mutants-verdict` and the pre-push leg gone
   they retire or move to the code audit. → this chunk SETTLES which (a plan decision); the wrap's route-resolve
   rewrites the :112 text (phase edits no markerless entry).

## Boundaries
- Not in scope: `/andromeda-code-audit` itself or any Andromeda skill/template (outside this repository); the audit's
  own mutation procedure. [verified: the pipeline seeds no per-chunk mutants `[[gate]]` entry — `grep -n -i mutant`
  over the phase skill's `gate-contract.md` / `plan-template.md` hits only a scratch-path example; prior plans' scoped
  `run --mutants --file` entries were authored per this project's own rules (testing.md :48,
  verification-harness.md 2026-09-25), which the wrap re-derives]
- Not in scope: `fuzz`, `perf`, the Playwright or coverage legs of pre-push/CI — they stay byte-for-byte except where a
  `needs:`/ordering edge pointed at a removed mutation job.
- No product-crate (`viola`, `viola-*` non-e2e) behaviour change is expected. Product-side references to mutation are
  comments or feature stubs that serve the kept arm or the nextest `mutants` profile, and all stay [verified: each hit
  read — `src/cmd/hook/seam.rs:23`, `viola-pty/src/lib.rs:236,566`, `viola-channel/src/client.rs:144`, the four crate
  `Cargo.toml:11` feature notes, `tests/support/{outer_pty,watch}.rs`, `tests/tui_passthrough.rs:96`,
  `crates/viola-e2e/tests/zero_retries.rs:69`].
- Deny/supply-chain gates (`cargo deny`, `deny-probes.sh`, zizmor) must stay green after the workflow edit.
