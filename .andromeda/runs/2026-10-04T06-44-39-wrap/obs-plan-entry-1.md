## 2026-10-04-confirmed-send-with-cl-1-records — feed overflow, send.client exits, the -32602 metric
**Section:** §6 Log Coverage (`parse-rejected` details) · §7 Error Capture (the vt100 bullet) · §4 Scenario Confirmed `send` (CL-1) — Cleanup · §5 Metric Coverage (`unknown` fallback / higher-`v` row) · §1 Obs Scope Summary (higher-`v` rejections)
**Change:**
- `oversize` now also covers `vt100-feed`: a copy the bounded feed queue drops poisons the model, one line per poisoning episode. The §7 bullet was "each poisoning gives exactly one `{vt100-feed, panicked}`"; now the detail is set by the cause (`panicked` or `oversize`), and a size message carries the drop count at its offer.
- `send.client`'s exits were 0 / 10 / 11 / 13 / 14 / 20 / 21; now also 12, and 1 `internal-error` on an unparseable reply; exit 20 carries `detail:"wrapper-fault"`, exit 21 `instance-dead` with `during` `connect` or `call`.
- `viola.compat.v_rejected` over-counts: `-32602` also answers `send`'s invalid params and `release` carrying `from`, and the client's `fault_of` labels any `-32602` unsupported-version, until the line tells them apart. §1 no longer equates `-32602` with higher-`v`.
**Why:** what confirmed `send` landed; `-32602` gained a cause with `ProtocolError::InvalidParams`.
**Kept:** the wrapper `send-refused`'s `wheel` field stays required; the line ships without it until the wheel lands (the `:80` entry's CARRY).
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/
