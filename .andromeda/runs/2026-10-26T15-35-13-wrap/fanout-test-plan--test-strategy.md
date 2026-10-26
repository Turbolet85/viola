proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 10
    section: "## 2. Test Strategy"
    change: "In the Unit tier's macOS compiled-out list, the whole-module gates read `the last.rs, pause.rs and unlink.rs test modules, gated whole` (unlink.rs's one case is its server-check case); link.rs's module is not gated whole: only its `request_refuses_an_unverified_server_inside_its_dispatch_span` is compiled out on macOS (already held by `each frame verb's`), its reply-shape table and line case still running there (a whole-module gate only where every case is gated)."
    sidecar: "test-strategy Unit tier: unlink.rs joins last.rs and pause.rs as a test module gated whole off macOS; link.rs gates its server-check case only (chunk 2026-10-26-explicit-session-links, implement deviation 1)."
    rationale: "The report's Deviations (implement 1) states only link.rs's server-check case is gated off macOS while unlink.rs's module, whose one case is gated, is gated whole; the Outcome (obs) has the two request_refuses_an_unverified_server_inside_its_dispatch_span cases compiled out on macOS. The file's named list of whole-gated modules (last.rs, pause.rs) now misses unlink.rs."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:251
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 58
    section: "## 2. Test Strategy"
    change: "In the IPC vector (V1) row's Security control negatives parenthetical, add: root `tests/channel_links.rs` `channel_link_names_are_validated_by_the_wrapper` (all OSes): raw `link` / `unlink` frames with a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven`, each answered `-32602` with `data: null` and no line written, beside its control."
    sidecar: "test-strategy trigger map V1 row: the raw-channel link / unlink name negative, landed in tests/channel_links.rs (chunk 2026-10-26-explicit-session-links)."
    rationale: "The report's Changes (Files: new tests/channel_links.rs; Symbols: the served link / unlink methods' -32602 causes) and Outcome (security: raw bad-name / foreign-driven / self-pair frames -> -32602, no line, met by channel_link_names_are_validated_by_the_wrapper, 14 passed) land the raw-channel negative the V1 row's plan location names only by file for the other security control negatives."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:301
note: "Outside this category: the body's §8 Time line (test-plan.md:705, `chrono::Utc::now()` read only through the clock seam, except at the budget gate's three evaluation points) now misses `LinkSet::line`'s `since` stamp read directly (src/run/links.rs:92, report Decisions & corrections), and the body's §6 Security control negatives line (test-plan.md:606) files the raw-channel link / unlink negative under `security_negatives_*.rs` though it landed in tests/channel_links.rs: for the body / mocking-stubbing-discipline / e2e-test-strategy detectors."
