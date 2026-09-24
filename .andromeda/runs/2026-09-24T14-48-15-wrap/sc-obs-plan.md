
## 2026-09-24-quality-gates — mutation consumer, unscanned uploads admissible by content, nightly fuzz
**Section:** §8 PII Scrubbing integration point 6 (new sub-bullet) · §9 Platform · §9 Pipeline integration (Mutation row)
**Change:**
- §8 item 6 records two uploads outside the secret scan, admissible by content: `mutants-verdict-<os>.json` (repo-relative source locations and outcomes only; `mutants.out/` never uploaded) and the nightly `fuzz/artifacts/` on failure (from the synthetic corpus; a non-synthetic seed drops that upload first).
- §9 Platform: `nightly.yml` runs advisories and the fuzz time-box, no longer "only" advisories.
- §9 Mutation consumer: the per-leg verdicts merged by `mutants-verdict`'s union gate.
**Why:** chunk 2026-09-24-quality-gates. The D-obs-pii escalations were resolved at wrap P2 by the overseer's ruling "Ratify both by content", with the verdict file stated as repo-relative only.
**Sweep:** `mutants\.out|outcomes\.json` 2 hits after the apply (`:1209` new text; the Mutation row's new consumer text). `runs only the weekly` 0. `nightly` 1 (the amended Platform). `.claude/docs/obs-summary.md` and `.claude/rules/observability.md` recomputed with no change (neither names the upload inventory, the nightly workflow or the mutation consumer).
