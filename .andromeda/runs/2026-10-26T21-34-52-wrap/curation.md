# Curation — 2026-10-26-home-and-code-bearing-file-integrity (wrap 2026-10-26T21-34-52-wrap, P3 at the 2a resume)

Source: the report's *Decisions & corrections* (14 entries; the implementing windows are gone, P1 carried them) + this
resumed window (two `bash-guard.sh` `cd` refusals, each re-issued in the allowed form — hook-held, not a learning).

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "Read a process listing whole before naming what runs or hangs: a `| head -N` cut hides every row past N, so a status built on it misses live processes." (confidence 0.7)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 1 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 4 below threshold · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "a rebuilt binary's /proc/<pid>/exe reads `<path> (deleted)`, so strip that suffix before comparing"
  Extended: T2/host-linux.md: "2026-10-07: On the Linux dev host a process named `viola` is not this repository's by its name …" + "a rebuilt binary's `/proc/<pid>/exe` reads `<path> (deleted)` — strip the suffix before comparing" (confidence 0.8)
  CLAUDE.md size: 128/200 · T1 3.1 KB, 0 over 600 B
```

host-linux.md has no `paths:` (always loaded): both writes judged by Tier 1's bar (tiebreaker 2) — one sentence, the
extended entry 575 B and the new entry 165 B after the write.

## Applied
- host-linux.md, new entry (truncated listing).
  Proof: report *Decisions & corrections* «Correction (implement): a status named two hung pids from a listing cut at 30
  rows (`| head -30`); two more were hidden»; the implement journal `.andromeda/runs/2026-10-26T19-02-07-implement/journal.md`.
  Signals: explicit correction +0.4 · "never" language (the correction quotes «never read a result summary through
  truncation») +0.3 = 0.7. Filter 1: no corpus entry holds the facet (host-linux.md's `| head` line is about `$?`, not
  hidden rows); the bash guard refuses a pipeline tool's stdout into `head`, not a `ps` listing, so no hook holds it.
- host-linux.md 2026-10-07 entry, extended (`(deleted)` suffix).
  Proof: report *Decisions & corrections* «Measured: a `/proc/<pid>/exe` reading `… (deleted)` after a rebuild was matched
  with that suffix removed». Signals: verified by measurement +0.4 · specific technical detail +0.2 · no-other-home +0.2
  (no master, route annotation, ledger note or contract document carries it; a `grep -rn '(deleted)'` over the masters, their
  `-detail/` files, `.claude/rules/` and `.claude/docs/` read none) = 0.8. Filter 1: additive facet (a failure mode the matched entry lacks), same tier
  and location.

## Every *Decisions & corrections* entry
1. The founder's P4 answers (`inputs#I3`) — not a learning: rulings, homed in the masters (P2) and the route (P5).
2. The founder's P5 «yes» (`inputs#I4`) — not a learning: a ruling.
3. The wrap directive (`inputs#I5`) — not a learning: this wrap's operator direction.
4. The overseer's process answers (kill only pids whose `/proc/<pid>/exe` and home prove them this run's) — rejected,
   Filter 1 duplicate of host-linux.md 2026-10-07 (decide whose a process is by `/proc/<pid>/exe`).
5. Correction: listing cut at 30 rows hid two hung pids — applied (host-linux.md, new entry).
6. Correction: the controls script stopped inside G10, not before it — rejected, Filter 2 task-specific.
7. Measured: `/proc/<pid>/exe` reads `… (deleted)` after a rebuild — extended (host-linux.md 2026-10-07).
8. Guard refusals (implement and this wrap) — not a learning: each act is held by its hook and was re-issued in the
   allowed form (telemetry, the friction stream).
9. Measured: an inheritable `icacls` grant propagates through `seed_conpty`'s hard links; a widening test starts its home
   unseeded — rejected, Filter 4 at 0.6 (measurement +0.4, detail +0.2): P2 amended it into the test-plan
   (`test-data-fixtures` / `integration-test-strategy`, test-plan-amendments.md), so no conditional signal applies.
10. Measured: canonical SDDL spells RID 500 `LA` — rejected, Filter 4 at 0.6: P2 amended it (test-plan-amendments.md, the
    `LA` spelling).
11. Measured: `Get-Acl` silent, `icacls /save` read the protected DACL back — rejected, Filter 4 at 0.6: P2 amended it
    (test-plan-amendments.md, the same entry).
12. Measured: a path under a regular file stats `NotFound` on Windows — → handoff, `recurrence-despite-learning`: testing.md
    2026-09-24 («a path under a regular file … reads as `NotFound` on Windows — so cfg-scope the case per OS») records this
    defect, and ci#40398601594 went red on it (fix `f65a4ae8`).
13. Remove-the-guard control hung 27 minutes under nextest's default profile (no kill line) — → handoff,
    `recurrence-despite-learning`: testing.md 2026-09-25, Extended 2026-10-04 («without that profile's kill line the run
    hangs»); the entry already stands at 1 536 B, over the Tier-2 cap, so no facet is added.
14. Sweep hazard: `git grep -c 'human::refuse('` vs `grep -rcE 'human::refuse\('`, per-file zero counts — rejected,
    Filter 4 at 0.2 (specific detail +0.2; no measurement changed a design, a claim or a gate).
