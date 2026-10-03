# Fan-out results — 2026-10-02-epoch-2b-cleanup

Seven Explore doc-agents, one batch. Each returned text was stripped of commentary only. No entity-escaped character
appeared in any return (no `&lt;` `&gt;` `&amp;`), so no decode was needed (probe: `entities=0`). No raw twin was warranted:
no `proposals: []` return was changed by stripping beyond the agents' `#` notes, and no return failed the parse.

| doc | verdict | stripped |
|---|---|---|
| architecture | 1 proposal | a `proposals_note` (D-arch-decisions: no drift; other candidates checked: test-only OS env reads unregistered by precedent, the lint reads an existing file, `target/agent-run/p5/` fits `target/agent-run/<session>/`; no other home-removal site) |
| security-plan | `proposals: []` | notes: no new external input, auth or dependency. FYI: report disproved claim 4 vs security-plan.md:207 (home DACL "at creation"), possibly an unimplemented control |
| design-system | `proposals: []` | note: no UI |
| layout-templates | `proposals: []` | note: no surface; 0 hits for the renamed flags |
| test-plan | 1 proposal | notes: D-tests-coverage / D-tests-framework no drift; the §6 "TempDir drop" lines (730 · 828 · 875 · 889 · 902 · 945) are path homes, not the root `TestHome` |
| obs-plan | `proposals: []` | notes: no hot path, logger or PII. FYI: obs-plan repeats "(Windows: home DACL)" at :288 · :393 · :439 · :538 · :1102 |
| a11y-plan | `proposals: []` | note: no UI, no schema change |

## Proposals

### P-1 — architecture · D-arch-resources · warning
- section: §Occupied Resources → Repository (`target/e2e-home/` entry, architecture.md:416)
- change: both removal paths (the drop and the gone-owner sweep) go through `tests/support/home.rs` `remove_owned`, which
  deletes `owner.json` last, so a removal that stops part-way keeps the record; D:'s ownerless remnants stay open under M2
- basis: architecture.md:416
- **disposition: APPLY (routine).**
  - Check 1: playbook "Accurate this-chunk addition"; its named symbol is in the report's Changes, and it lands inside an
    existing entry that already describes the owner-record lifecycle (not "Registry over-reach": the entry states the
    removal mechanism at the grain it changed).
  - Check 2: no opposing edit.
  - Check 3: plan step 5 sanctioned home-lifetime changes; consistent.
  - Check 4: the "no other site" claim was re-read. `grep -c TempDir` architecture.md = 0, its five key files = 0;
    `owner.json` 1 hit (:416, read in full at its 892-char line).
  - Check 5: expected amendment 1.
  - Applied text re-derived from the report, without the proposal's "deadline-failing runs" wording: the remnants'
    mechanism is not established (report, Insufficient fixes).

### P-2 — test-plan · D-tests-obs-harness · warning
- section: §3 → Test data bootstrap (`registries/contracts/test-plan/test-data-bootstrap.md:15`, the Cleanup bullet)
- change: root `TestHome` drop and the sweep remove through `remove_owned` (record last), replacing "`TempDir` drop
  removes per-test homes"; the rest of the bullet kept
- basis: test-data-bootstrap.md:15
- **disposition: APPLY (routine).**
  - Check 1: playbook "Accurate this-chunk addition".
  - Check 2: same fact as P-1 in another master, no contradiction.
  - Check 3: consistent with plan step 5.
  - Check 4: the six §6 "TempDir drop" cells were read (Paths' H / H2 / H3 homes, not root `TestHome`); the
    `5-command-implementation.md:33` hit was read (the `viola_e2e::fixtures` chain); no dependents.
  - Check 5: expected amendment 1.
  - Orchestrator addition in the same bullet: the line's two `TempDir::keep()` statements ("a failing test calls
    `TempDir::keep()` only when …", "makes every rstest home call `TempDir::keep()`") restate as the keep *behaviour*. The
    new drop calls `keep()` in both branches to take the path, so the mechanism words went stale while the behaviour
    stands.
  - obs-plan carries no removal claim (agent grep: 0), so the §3 ↔ obs §3 bind is unaffected.

## Validate — the orchestrator's checks over the whole set
- **Check 5 (expected amendments):**
  1. carried by P-1 + P-2 (the git-fixture read-only half not carried: premise disproved);
  2. the deadline lint / P6 witness: judged **not contract-level** and not raised. Both enforce rules that already live
     in testing.md (2026-09-24 deadline-below-kill; 2026-09-28 retarget the unbuilt selector), and no master states either
     (`kill line` / `unbuilt selector` / `never-planned`: 0 hits in arch, test-plan, obs and security);
  3. superseded (no `m1-*` dir in the repository; the report);
  4. route → P5.
- **Check 6 (disproved claims), each DISPOSED:**
  1. The git read-only premise: no master states it (test-plan "read-only" 4 hits = fixture-lifetime cells, read;
     test-data-bootstrap 0). Chunk docs only → **routed to curation** (the measured std behaviour as a learning).
  2. The audit's resize/drop grading: the audit record is not a master → **recorded in `evidence/m1.md`**, no amendment.
  3. The P5 study hypothesis (`refs/`, not a master) → **routed to P5**: the revive entry reads the measured result;
     the tab-close leg is owed to the founder.
  4. The home DACL "at creation" (security-plan.md:207; obs-plan's "(Windows: home DACL)" ×5; testing.md §Test data):
     the masters state the target design, and the gap is **already owned** by `working-route.md:107` (Epoch 6, "Home and
     code-bearing file integrity", whose CARRY names "no Windows protected user+SYSTEM DACL for a `--home` outside
     `%USERPROFILE%`"). Sequencing deferral (playbook rule 1): no amendment; the basis (a grep) stays the report's.
- **Escalations:** none. No playbook escalate-class proposal; no boundary widening.
