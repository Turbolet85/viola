# Fan-out results — 2026-09-26-local-linux-pre-push-gate (wrap 2026-09-27T00-51-08)

Returns were YAML only, 0 entities, no stripping needed except the detectors' trailing `#` comment lines (kept in the
record below as the detectors' own notes).

| doc | verdict | proposals |
|---|---|---|
| architecture | 10 proposals | D-arch-resources ×5, D-arch-decisions ×5 |
| security-plan | 4 proposals | D-security-input ×4 (1 primary + 3 dependent-of) |
| design-system | `proposals: []` | D-design-tokens: no UI surface (all `tokens n/a`) |
| layout-templates | `proposals: []` | D-layout-surface: no UI surface; pre-push is internal, not a product verb |
| test-plan | 7 proposals | D-tests-obs-harness ×5, D-tests-coverage ×1, D-tests-framework ×1 |
| obs-plan | `proposals: []` | no hot path, stack or PII drift; flagged expected amendments #12, #13, #17 as outside its detectors |
| a11y-plan | `proposals: []` | no interactive element; no schema change |

## Validate
- **Accepted (19):**
  - arch: env registry (seam) · Conventions naming (:171) · Config management (:546) · [Naming] decision (:106) ·
    Repository `target/pre-push/` · project tree (`wsl-provision.sh`, `harness::pre_push`) · Stack CI/CD row ·
    CI/CD approach.
  - security: Input Validation row · Anti-Patterns Universal carve-out (:577) · Secret Management Storage (:420) ·
    Decisions Log entry.
  - test-plan: §3 Internal harness subcommands · §3 Closed enums · §3 run step 4 Base · §10 Mutation gate · §12 entry ·
    §5 Module ↔ PTY · §9 tool install.
- **Playbook:** "Accurate this-chunk addition" (routine) for the harness / tree / registry / test-plan items. The seam's
  items fall in "Boundary widening" (never-routine: a new input read by `viola`, test builds only) and touch a locked
  decision ([Naming]) — escalations RESOLVED by the operator's recorded ratification in the wrap directive ("Record
  the test-only `FAKE_AGENT_PUMP_DELAY_MS` seam as the architecture + security-plan amendments … a carve-out behind
  cfg(feature=\"fake-agent\"), capped at 5 s, absent from release builds"); the ratification is cited in each sidecar.
- **Rejected (2) — Registry over-reach:** arch host-side WSL state (`~/viola-pre-push`, `~/.rustup`, `~/.cargo`: outside
  the repo, never read or set by the product — the P2 arch extract's own note) · arch temp trail
  `viola-resize-<pid>.ndjson` (arch registers OS-namespace IPC objects such as the test pipe at :334; there is no
  filesystem registry of test temp files).
- **Raised by the orchestrator (check 5, routine — report substantiates each):** #10 security-plan §Dependency Security
  (WSL toolchain install) · #11 security-plan §Secret Management (`env -i` into WSL; no `CLAUDE*` crossing) · #12
  obs-plan §9 Mutation row (local union) · #13 obs-plan §8 integration point 6 (Linux clone paths never in a document) ·
  #17 obs-plan restatements of the env rule (dispositioned by the cascade sweep).
- **Check 6:** disproved claim 1 → test-plan §3 Base proposal · 2 → test-plan §5 proposal · 3 (route CARRY, not a master)
  → spent freight, archived at P7 flip-compaction · 4 (plan-level research M3) → curation.
- Cross-contradiction: none (no two proposals edit one section in opposite directions). Intent-consistency: the
  product-code widening is a justified divergence already amended into scope/plan on the operator's word.
