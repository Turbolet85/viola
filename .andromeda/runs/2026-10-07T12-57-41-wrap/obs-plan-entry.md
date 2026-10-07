
## 2026-10-07-send-waits-out-the-paste-hint — `send-confirmed.duration_ms` spans the gate's wait
**Section:** §4 Scenario: Confirmed `send` (the `send-confirmed` field line) · §5 (the `viola.send.confirm_ms` row)
**Change:** Both sites now say what `duration_ms` spans: it runs from the send's arrival at the wrapper, so a sample includes the readiness gate's wait, up to architecture's `GATE_MAX_WAIT`, ahead of the confirmation window. The bound is named by its constant, not copied. No field, site or detail was added.
**Why:** The field and its start instant are as they were; what moved is the gate's share, which can now reach 8.5 s on a delivered send (the founder's live ruling of 2026-10-07T10:29Z on the gate's wait, relayed by the overseer). A reader of the readback-latency metric would otherwise take a sample as the `open` to `read back` interval, which starts later, at `send-issued`.
**Ref:** .andromeda/runs/2026-10-07T12-57-41-wrap/
