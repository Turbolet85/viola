proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 8
    section: "## 8. Mocking & Stubbing Discipline"
    change: "In the Time item, replace the clause `chrono::Utc::now()` is read only through the clock seam, with one exception as landed: … with: `chrono::Utc::now()` is read only through the clock seam, with two exceptions as landed: the budget gate's three evaluation points read it directly and hand the instant to the pure decision (as read in `src/run/send.rs` and `src/cmd/run.rs` at the chunk 2026-10-10-budget-governor wrap), and `LinkSet::line` stamps a link line's `since` with it directly (`src/run/links.rs:92`, as reported at chunk 2026-10-26-explicit-session-links), so no clock double reaches `since` and its root cases assert it by shape only (`is_clock_time` in `tests/cli_links.rs`); neither read was a census of `Utc::now()`."
    sidecar: "test-plan mocking-stubbing-discipline Time item: the link line's `since` (`LinkSet::line`, `src/run/links.rs:92`) named as a second direct `Utc::now()` read outside the clock seam, its root cases asserting `since` by shape only (`is_clock_time`) — chunk 2026-10-26-explicit-session-links."
    rationale: "The report's Decisions & corrections states `LinkSet::line` stamps `since` with `Utc::now()` directly (`src/run/links.rs:92`), outside the clock seam, and root cases assert `since` by shape only (`is_clock_time`); the file's Time item still says the clock seam has exactly one exception (the budget gate), which no longer holds now that the served `link` method answers that `since` (Symbols / APIs: `link` answers `{since, already_linked}`)."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:278
note: the body's §8 Time bullet (.andromeda/test-plan.md:705) states the same single exception ("except at the budget gate's three evaluation points") and §7's Time-sensitive data line (.andromeda/test-plan.md:653) is the report's named home for this clock-seam fact — both are outside this category file and are the body / §7 detectors' to amend.
