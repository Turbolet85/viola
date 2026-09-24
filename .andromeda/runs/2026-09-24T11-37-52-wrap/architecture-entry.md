
## 2026-09-24-log-redaction-and-never-log-floor — anyhow's scope includes the catch-site reporter
**Section:** §Stack and Technologies (Error types row, :27) · §Established Decisions [Error Handling] (:95)
**Change:** anyhow 1.0.104 stays in the `viola` bin only, now named as `main`, dispatch (`cmd::dispatch` returns the error with the resolved home and instance) and the catch-site reporter `viola::obs::report_internal_error`. Context chains are "built at dispatch and recorded only in the owner-only instance detail file" (`chain` in `instances/<name>/diagnostics/detail-<role>.ndjson`), never a role line, stdout or stderr. The rationale now reads "only useful where a person reads them: the owner's post-mortem".
**Why:** the chunk routed the dispatch error's `chain()` through `src/obs.rs` into the detail file (report Changes → Symbols / APIs, Crates / modules), which obs-plan §7 scrubbing layer 3 already required. The old "only at dispatch" / "`main` and dispatch" wording left the reporter outside the decision. Still inside the root bin crate; no dependency added.
**Sweep (cascade step 2):**
- **Patterns:** `only at dispatch|main\` and dispatch|main and dispatch|only useful where errors reach|anyhow[^|]{0,80}dispatch|context chains`, over the seven masters, CLAUDE.md, `.claude/rules`, `.claude/docs`, playbook and drift-base.
- **Amended:** arch :27, :95; obs :54 (cited arch Stack's retired wording verbatim) and obs :1117 (anyhow "at the root-bin dispatch edge only") — see obs-plan-amendments.
- **No change:** arch :413 (root bin → anyhow; no dispatch-only claim); security :447 (external errors carry no anyhow chains — still true); CLAUDE.md :28 ("the only crate with anyhow" — still true) and :37 (external errors — still true); `rules/observability.md:19` (chain only in detail files — already true); `docs/services/viola.md:11` (dependency list), :28 (chain to detail file — already true).
- **Leaves re-derived:** `.claude/docs/stack.md:23`, `.claude/docs/conventions.md:34`, `.claude/docs/services/viola.md:6`.
- **Result:** 0 retired-claim hits remain (post-sweep grep over the same set). **Control:** the pattern fired 4 hits on `.raw-fanout-arch.md`.
