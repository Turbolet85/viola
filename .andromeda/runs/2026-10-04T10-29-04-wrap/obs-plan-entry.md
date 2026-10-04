
## 2026-10-04-wait-and-last — wait channel lines scoped; wait/last exit 21 is instance-dead only
**Section:** §4 Span / Trace Coverage → Scenario `wait` / `last` event-driven readback
**Change:**
- Was `channel-request{… method:"wait"|"last", after, timeout_ms}` and `channel-response{… outcome …}` for both methods; now `after` / `timeout_ms` (`u64` only) and `outcome` ride `wait` only, on both sides; `outcome` is the waking kind or `timed-out`, derived once in `viola-channel` over the shared wake set; the client alone adds `instance-unreachable` when the reply never arrived; `last` carries none.
- Was exit 21 `detail` ∈ `instance-dead|strict-modes-failed|server-verify-failed`; now `instance-dead` + `during` at WARN, the other two impossible until client-side server verification lands (Epoch 6, `:109` / `:111`).
**Why:** what the chunk landed; `send`'s exit 21 moving to WARN brings the code to §6's log-level mapping, no body change.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/
