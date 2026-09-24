
## 2026-09-24-observability-gates — raw-tracing ban: level-macro path ban plus a fail-closed raw-`event!` grep
**Section:** §3 Bootstrap phases (logger-stack-install exemption bullets; obs-ci-gate-wire bullet 1) · §8 PII Scrubbing integration point 1 · §9 Pipeline integration Lint / typecheck row · §10 Build / deploy failure conditions · §11 Logs (raw-tracing rule enforcement) · §12 Decisions Log (new D-33)
**Change:**
- The inner-`#[allow(clippy::disallowed_macros)]` exemption is retired.
- `clippy.toml` `disallowed-macros` bans `tracing::{info,warn,error,debug,trace}` by path.
- Raw `event!` is caught by a fail-closed grep in `scripts/lint-probes.sh`, which is proven both ways and runs on the Linux lint leg.
- `obs_event!`'s inner allow stays but has no effect.
- A `lint-probes.sh` failure is now a build failure.
- D-33 records the supersession of D-25's clause; D-25 is left as written.
**Why:** as measured on clippy 1.98.1 at this chunk (`.andromeda/runs/2026-09-24T12-21-11-implement/clippy-disallowed-macros-measurement.md`), no allow placement inside the macro, at the call site or on the calling fn exempts the inner `event!`. Only a caller-crate `#![allow]` does, and with `tracing::event` listed, all 16 `obs_event!` sites failed. The operator ratified the replacement at /implement P1 (overseer, founder-delegated). Report: "Spec claims disproved" 1, Deviations 1–2.
**Sweep (cascade step 2):**
- Masters, 7 of 7:
  - `tracing::{event,`: 1 hit, obs `:1378`, the §11 rule; no change, since the ban on all six stays true and only its enforcement was amended;
  - `allow attribute inside`: 2 hits, obs `:1641` (D-25, historical, no change) and `:1779` (the new D-33);
  - `inner \`::tracing::event!\``: 1 hit, the amended `:783`;
  - `exemption mechanism`: 0 hits;
  - `disallowed-macros`: every hit is amended text or D-25/D-33.
- Leaves: `.claude/rules/observability.md:24` (the rule stated clippy as the whole enforcement) was re-derived. `.claude/docs/commands.md:45` was re-derived. `obs-summary.md:54` (print bans only) needs no change.
- Curation homes, playbook and drift-base: 0 hits.
