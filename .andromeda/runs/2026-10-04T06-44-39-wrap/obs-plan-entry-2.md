## 2026-10-04-confirmed-send-with-cl-1-records — the G2 question answered: the chaos home outside the scans (F4)
**Section:** §7 Error Capture (the vt100 bullet) · §9 CI Integration (the Integration tests row; Step order, step 1)
**Change:**
- §7 was "recorded, not witnessed at run level … whether G2 counts the contained panic line is that chunk's question". Now witnessed by `tests/chaos_feed_panic.rs`: exactly one `parse-rejected{vt100-feed, panicked}` and one `event:"panic"` line, both asserted present, then `viola send` refused `input-not-ready` (exit 13) while a typed key still reaches the child. G2's single seam exemption is unchanged.
- §9: integration homes live under `target/e2e-home/` but one — that test's `TestHome::outside_scan()` home under the system temp dir, outside G2, G4, the secret scan and the upload; the test asserts its own panic and `parse-rejected` lines.
**Why:** the founder's ruling, live, 2026-10-04, relayed by the overseer: the second named test-data carve-out, all three scans named, rather than a second G2 exemption.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/
