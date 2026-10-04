## 2026-10-04-confirmed-send-with-cl-1-records — the second test-data carve-out: the chaos home (F4)
**Section:** §5 Integration (Setup / teardown lifecycle) · §3 → `test-data-bootstrap` (Mechanism; Cleanup) · §7 Test Data (Test data lifecycle, CI)
**Change:** homes were under `target/e2e-home` with one carve-out (`seed_conpty`); now a second: `tests/chaos_feed_panic.rs` alone boots in `TestHome::outside_scan()`, a `viola-chaos-*` home under the system temp dir, because its forced vt100 feed panic writes a G2-counted `event:"panic"` line. That home is outside G2 (zero panics), G4 (schema conformance) and the secret scan; the test asserts its panic line and its `parse-rejected{vt100-feed, panicked}` line present itself.
**Why:** the founder's ruling, live, 2026-10-04, relayed by the overseer, naming all three scans.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/
