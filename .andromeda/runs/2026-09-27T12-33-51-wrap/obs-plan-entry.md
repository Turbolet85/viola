
## 2026-09-27-wrapper-channel — corr required rules in the line schema; mis-shaped conn logged as srv_conn
**Section:** §3 Log format (Required fields: the `corr` rules; Null encoding) · §3 IPC boundaries (additive `conn`)
**Change:**
- `channel-*` corr is null for an id-less `hook.event` and on a response to a frame with no id (-32700 / -32600); a dialog `hook-invoked` (fail-open before `dialog_id`, D-07) and a `hook-decision` with `detail` may be null.
- `schemas/diag-line.v1.json` requires `corr` on `dialog-raised`, `dialog-answered`, `release-from-driver`, `channel-request` unless `hook.event`, `channel-response` unless -32700/-32600, `send-*` when `side` is `wrapper`, and `hook-decision` for `pre-tool-use`/`permission-request` without `detail`; never on `hook-invoked`. Was "optional keys" for `corr` everywhere.
- A peer `conn` outside `^[a-z]+-\d+-\d+-\d+$` or over 64 bytes is logged as `srv_conn` too; the server strips `conn` before dispatch.
**Why:** the chunk made `corr` required where §3 always defines it and disproved the plan's unconditional rule on dialog hooks; the test-plan §3 Log format is the bound twin and carries the same text.
**Kept:** §1 (the verbatim obs-scope copy) and the D-10 / D-26 Decisions Log entries.
**Ref:** .andromeda/runs/2026-09-27T12-33-51-wrap/
