# Cascade dispositions — 2026-09-28-mutation-testing-to-the-epoch-boundary

**Search:** `cascade.py sweep --patterns-file cascade-patterns.toml` (21 patterns, baseline `537ac366`, every control
fired on the pre-pass masters): the retired names (`mutants-verdict`, `mutants-legs`, `invalid-leg`, `scoped-leg`,
`linux-leg|windows-leg`, `verdict-missing|base-mismatch`, `viola-pre-push-scratch`, `download-artifact`, `cfg_legs`,
`syn|proc-macro2`, `windows_scratch_bytes|scratch_bytes_after`, `TMPDIR=<distro home>`, `cmd_env`, `--leg`) and the
retired MECHANISM's phrasings (`mutation leg(s)|mutants leg(s)|leg verdict(s)`; `union of … legs|the union|union
verdict|same union`; `per-chunk mutation|mutation gate`; `mutates the whole chunk|whole chunk`; `18 check-runs|18/18|(9;
18`; `steps 1, 2 and 4`). Sections read beyond the rows: test-plan §6 around :1273 (the one stale standing row).

## Masters
- **new / edited rows** (this pass's own text, each re-read): architecture :572 · security-plan :393 :430 :750 · test-plan
  :431 :437 :1448 :1509 :1942–:1947 · obs-plan :1212 :1256 :1348 — amended; no swept text stands as a claim (the new
  text names the retirement). test-plan :552 (`Base: the whole chunk…`) — the base derivation `run --mutants` keeps;
  true, no change.
- **STALE, fixed in this pass:** test-plan :1273 (§6 E2E, "except the mutation leg's `chunk.diff`" → `run --mutants`' `chunk.diff`),
  a dependent occurrence the detector missed — amended.
- **Rejected (verbatim upstream copy):** security-plan :131 (`download-artifact` in the Threat Model supply-chain list),
  :161 (the Threat Model CI jobs line "mutation (ubuntu and windows legs plus a union verdict)") — the section stays the
  threat-assessment copy; the facts live in §Dependency Security (S6 S9 S10) and architecture :572.
- **Dated history, no change:** security-plan :654 :690 :736 :746 (Decisions Log entries and witnesses — the new
  2026-09-28 entry supersedes :654 :690) · test-plan :1807 :1812 :1816 :1823 :1841 :1844 :1851 :1853 :1854 :1874 :1875
  :1879 :1882 :1885 :1888 :1889 :1902 :1903 :1912 :1920 :1921 :1925 :1926 (§12 entries and their Impact lines).
- **A true claim sharing a token:** a11y-plan :1145 ("the union of the per-state axe verdicts").
- `cmdenv`, `defaultsel`: 0 rows after the pass (controls fired) — both retired phrasings are gone from the masters.

## Leaves (re-derived at step 3)
`.claude/rules/verification-harness.md` :24 :44 · `.claude/rules/security.md` :34 · `.claude/rules/testing.md` :48 ·
`.claude/docs/commands.md` :32 :36 :38 :47 :48 :74 · `.claude/docs/workflow.md` :10 :11 :47 ·
`.claude/docs/tests-summary.md` :42 · `.claude/docs/obs-summary.md` :51 · `.claude/docs/security-summary.md` :66 ·
`.claude/docs/stack.md` :32 :34.

## Curation homes (never edited by the cascade → P3 as in-place extensions)
`.claude/rules/verification-harness.md` Session Additions :54 (2026-09-25 "run it as `run --mutants --leg windows-2025`
and take the verdict from the CI union") and :56 (2026-09-26 "judge a mutation leg by its verdict") ·
`.claude/rules/testing.md` Session Additions :58 (2026-09-24 "CI gates the union of the ubuntu and windows legs") —
stale → P3. `.claude/rules/testing.md` :52 ("Under the zero-missed mutation gate …") — still true of the audit's gate;
no change. `.claude/docs/session-learnings.md` :27 (a dated measurement of the old gate) — history, no change.

## Judgment bases
`playbook.md`, `drift-base.md`: 0 rows.
