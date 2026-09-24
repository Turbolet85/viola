
## 2026-09-24-log-redaction-and-never-log-floor — arch citation and anyhow scope follow the catch-site reporter
**Section:** §2 root bin coverage row, "Reason if not fully instrumentable" (:54) · §7 Error Reporting, Platform pick (:1117)
**Change:** :54 now cites arch Stack as saying context chains are "built at dispatch and recorded only in the owner-only instance detail file". :1117 now says anyhow 1.0.104 is used "at the root-bin dispatch edge and its catch-site reporter (`viola::obs::report_internal_error`) only".
**Why:** cascade step 2 of the arch amendment in this pass (architecture-amendments, same marker). :54 quoted arch's retired "context chains only at dispatch" verbatim, and :1117 restated the dispatch-only scope while the chunk's reporter in `src/obs.rs` now renders the chain (report Changes → Symbols / APIs). §7 scrubbing layer 3 (`chain:[...]` only in `detail-<process>.ndjson`, only when an instance resolves) already matched the shipped behaviour and is unchanged.
**Sweep:** the arch entry's patterns and dispositions; in obs: amended :54, :1117. No other obs hit.
**Leaves re-derived:** none needed — `.claude/docs/obs-summary.md` and `.claude/rules/observability.md` carry neither sentence (grep `anyhow|dispatch edge|platform pick|local error capture` → 0 hits in obs-summary; observability.md :19 already states the chain goes only to detail files).
