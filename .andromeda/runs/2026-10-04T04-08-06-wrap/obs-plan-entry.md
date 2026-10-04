
## 2026-10-04-windows-boundary-mutation-workflow — the boundary run's Windows form
**Section:** §9 Platform · §9 Pipeline integration (Mutation row)
**Change:**
- Platform: the dispatch-only `windows-mutants.yml` (the boundary audit's Windows mutation leg, uploads nothing) is named beside `ci.yml` and `nightly.yml`.
- Mutation row: was "no CI job since 2026-09-28"; now no push or pull-request job. The audit's Windows form is the dispatch-only, report-only `windows-mutants.yml` (`run --mutants --package <member> --file …` on `windows-2025`), which scored the `cfg(windows)` obs code in `src/panic_frames.rs`: 9 of 9 caught in run 37174673472.
**Why:** founder ruling C2 (2026-10-04); obs-code mutation for this boundary closes on that run.
**Ref:** .andromeda/runs/2026-10-04T04-08-06-wrap/
