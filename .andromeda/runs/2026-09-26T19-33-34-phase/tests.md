# tests extract

## Relevance
Relevant. All three parts of the chunk sit on surfaces that test-plan owns: the `run --mutants` base (§3), the `gate --mutants-legs` union (§3 / §9 / §10), and the `secret-scan` scope (§3 / §6). Each also needs a wrap amendment.

## Constraints
- **The base rule changes, and the base-missing contract stays.** test-plan §3 `run` step 4 "Base" currently says the base is `AGENT_RUN_CHUNK_BASE`, else `merge-base HEAD origin/main`. It says CI sets that variable to the PR base sha, else `github.event.before`. §9 Pipeline structure "Mutation" row says the same. This chunk replaces that CI derivation, so both sites need a wrap amendment (the scope names test-plan as the rule's home).
  - The same §3 Base bullet requires an unreachable base to exit 1 with `reason:"base-missing"`.
  - It also forbids writing an empty `chunk.diff`, because that would read as a false `tested:0` pass.
  - The new rule must keep both behaviours, and must give a defined answer before any pre-CI commit exists. §3 preamble says "never a vacuous pass".
  - Whether `resolve_base` or ci.yml computes the new base, and what ci.yml:166 passes today, is for research to settle.
- **The union rule is closed, and the new rule has to be written against it.** test-plan §3 `gate` "A breach is any of" (union bullet) defines the union as red when no leg reports `caught` and some leg reports `missed` or `timeout`.
  - In that rule, unviable on every leg is reported but not red. A missing leg file is `artifact-missing`, and a partial union judges no mutant.
  - §10 Mutation gate and §9 Mutation row restate this rule. Both justify it by the cargo-mutants `#[cfg]` limitation.
  - "Unviable on the compiling leg is not a survivor" is a new rule. It must be added at all three sites.
  - The "missed on the compiling leg stays a survivor" half must hold.
- **The leg-file shape and the verdict/outcome values are closed.** test-plan §3 `run` Output format fixes the leg file as `{"v":1,"leg","verdict","mutants":[{"name","outcome"}]}`, with outcomes only: no argv, log path or test output.
  - §3 Closed enums fix `mutants.verdict` and the leg `outcome` set, and say "A new value needs a Decisions Log entry" (§12).
  - Any new field or value that tells the gate which leg compiles a mutant counts as a contract change. It needs a §3 edit plus a §12 entry.
  - The field must stay free of argv and paths, because §9 Test report format explains why `mutants.out/` is never uploaded.
- **Per-leg verdicts stay fixed.** test-plan §3 `run` step 4 Verdict and Exit code semantics require `missed == 0 && timeout == 0 && unviable <= caught`.
  - A leg with `unviable > caught` is red at its own run and is never deferred to the union. The union change must not weaken this.
  - §11 Quality says NEVER trust the cargo-mutants exit code alone.
- **Secret-scan scope is fixed in two places.** test-plan §3 Internal harness subcommands `secret-scan` "Scope" names every file under `target/agent-run/`. §6 "Error sanitization and secret scan" (Canary bullet) names "the harness capture in `target/agent-run/`".
  - Narrowing the scan so it skips stale mutation-leg residue (`target/agent-run/chunk.diff`) amends both sites.
  - Per §9 Test report format, the per-OS `test` job's `harness-<os>` upload leaves CI only behind this scan. The narrowed scope must still cover every file that upload ships.
  - The scan must still never print matched bytes, keep the `empty-scope` reason, and keep the closed hit `class` set (§3 Closed enums).
- **Topology and gate placement stay as they are.** test-plan §9 Pipeline structure / Matrix builds and §11 CI define mutation as a fixed two-leg matrix (`ubuntu-latest`, `windows-2025`) plus one ubuntu `mutants-verdict` job running `gate --require mutants --mutants-legs ubuntu-latest,windows-2025` (§3 `gate` CI paragraph).
  - §10 Mutation gate records that a macOS-only body has no leg yet. That matches the scope boundary: no macOS leg in this chunk.
- **No retries or flake parking.** test-plan §10 Zero-flakiness budget and §11 CI / Quality: `retries = 0`, no `#[ignore]` parking, and no retrying a red CI run away. A red CI run in the operator pass is fixed, not rerun.

## Patterns to follow
- **Literal-oracle negative next to each new rule.** test-plan §12 (2026-09-25 security-prerequisites entry) kept the mixed-diff red under a named literal-oracle test. The union's "missed on the compiling leg stays red" negative, and the probe's mid-pass `· complete ·` edit case, follow the same shape (§11 Unit: no product constants as the oracle).
- **Measured witness on a recorded CI run.** test-plan §10 Mutation gate and §12 entries anchor each rule change to a CI run id and its counts. The union witness should replay run 36165685381's `HostTerminal::enter -> Some(Default::default())` shape.
- **Probe plus control for a gate.** test-plan §9 Lint and Supply-chain rows (orphans-check `--probe`, `deny-probes.sh` "banned, control clean") and the Release build row. The base-stability probe and the secret-scan planted-secret red test follow the fired-plus-clean-control form.
- **Every run gets a fresh artifact, never a stale one.** test-plan §3 `run` step 4 Diff / Classification and §3 `gate` Inputs.
  - Stale `chunk.diff`, `outcomes.json` and leg files are deleted first.
  - A missing artifact is a breach, never a copy of an earlier run's file.
  - The secret-scan residue fix works in the same stale-artifact territory.
- **Fixed-code breach details.** test-plan §3 `gate` Output requires `detail` to be a fixed code, a count, or a union's mutant name, never file content. Any new union breach or detail must follow this.

## Anti-patterns to avoid
- **A vacuous green.** test-plan §3 preamble ("never a vacuous pass") and §3 `run` step 4 Base (never an empty `chunk.diff`). Neither the base rule, nor the union's new unviable exemption, nor the narrowed scan may turn a real survivor or a real secret into a pass. Examples: a base equal to HEAD that yields `no-rust-delta`, or unviable-everywhere read as "compiling leg unviable".
- **Trusting cargo-mutants' `#[cfg]` view, or its exit code.** test-plan §11 Quality and §10 Mutation gate. The per-OS body is judged only on its compiling leg, from counted outcomes.
- **Widening a gate's exclusions to get green.** test-plan §11 CI (the coverage-regex rule applies by analogy to narrowing the secret-scan scope) and §11 Quality ("NEVER skip quality gates"). The scan's narrowing must be scoped to named mutation residue, with the planted-secret red test kept.

## Contract bindings
- **tests ↔ obs:** the `secret-scan` step order and scan-gated uploads bind to obs-plan §9 (step 3, `id: secret-scan`). The canary rule binds to obs-plan §8 (High-class verification), per test-plan §6 Error sanitization and §9 Test report format.
- **tests ↔ security:** `mutants.out/` stays un-uploaded under security-plan §Bootstrap `secret-scanning-ci-gate` (test-plan §12, 2026-09-25 Epoch 1 cleanup entry). Any leg-file field added for the union inherits this ban.
- **tests ↔ arch / obs restatements:** the amendment history records that architecture.md:515 and obs-plan.md:1253 restate the union rule. A union change needs their sidecars swept in the same wrap.
- **tests ↔ /andromeda-phase process:** the base is the last master-flip commit, found by the `.andromeda/master-route.md` pickaxe (phase Setup 5a) and bound to V15/V17 fix-commit discipline. test-plan states the rule; the tooling owns the pickaxe.

## Acceptance criteria contributions
- **(tests) Base probe.** A simulated pass (flip commit → pre-CI commit → fix commit, including one that edits a `· complete ·` master line) resolves to the same chunk base on every commit. An unreachable base still exits 1 `base-missing` with no `chunk.diff` written. (per test-plan §3 `run` step 4 Base)
- **(tests) Union witness and negative.**
  - Witness: `gate --require mutants --mutants-legs ubuntu-latest,windows-2025` over leg files in the 36165685381 shape (unviable on the compiling leg, missed on the other) has no `mutants` breach.
  - Negative: a mutant `missed` on its compiling leg still breaches, with `gate:"mutants"` and the mutant name as `detail`.
  - A missing leg file stays `artifact-missing`.
  - (per test-plan §3 `gate` union bullet; §10 Mutation gate)
- **(tests) Secret-scan scope.** With a stale `target/agent-run/chunk.diff` containing a `?t=` pattern line, `secret-scan` does not report it. A planted secret in a scoped file still exits 1 with its class, and the output never prints the matched bytes. (per test-plan §3 Internal harness subcommands `secret-scan`; §6 Error sanitization and secret scan)
- **(tests) Mutation verdict.** `cargo nextest run -p viola-e2e` passes. The CI mutation legs end `missed == 0`, `timeout == 0`, `unviable <= caught`, and `mutants-verdict` succeeds. Every new guard has a remove-the-guard run that goes red. (per test-plan §10 Mutation gate; §3 `run` step 4 Verdict)

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton, "integration filterset, mutation diff, base and trigger"**
  - Set the CI base to PR base sha, else `github.event.before`, through `env:`. This is the rule this chunk replaces.
  - Set the diff to working tree plus untracked files from `merge-base(<base>, HEAD)`.
  - Recorded that the `origin/main` fallback never resolves on the single build branch (measured with `git ls-remote`).
- **2026-09-24-quality-gates**
  - Introduced `--leg`, `gate --mutants-legs` and the two-leg union, because of the cargo-mutants `#[cfg]` limitation (run 36005608858 MISSED a `#[cfg(not(unix))]` stub).
  - This is the union this chunk refines.
- **2026-09-24-epoch-1-cleanup**
  - Added the force-push `base-missing` case (run 36117447745; remedy: rewind to the chunk base, then fast-forward).
  - Made `unviable > caught` red per leg and never deferred.
  - Scoped the union's "unviable on every leg is reported, not red" parenthetical to the union.
  - Swept architecture.md:515 and obs-plan.md:1253 as restatements of the union.
  - Declined partial-verdict uploads, so `mutants.out/` stays un-uploaded.
- **2026-09-25-security-prerequisites**
  - Added the `test-only-rust-delta` verdict. The leg file carries it with `"mutants":[]`, and the closed verdict set went from 2 to 3.
  - This is the precedent for how a new leg/verdict value is added: §3 + Closed enums + §10 + §12.
- **2026-09-24-observability-gates**
  - Declared `secret-scan` as an internal subcommand, with the scope that includes every file under `target/agent-run/`.
  - Replaced the unscanned `agent-run-<os>` upload with scan-gated uploads.
  - This is the scope this chunk narrows.
