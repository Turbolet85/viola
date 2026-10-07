## 2026-10-06-local-command-send-outcomes — send consumes the local-command list
**Section:** §Established Decisions [Delivery Confirmation] · [CLI Version Compatibility] (the closing "owed" clause) · §Standard Contracts → Channel methods (`hook.event`) · Event `data` per kind (`send-confirmed`)
**Change:** was "`send` does not consume them yet … every local command still ends `not-delivered` / `no-prompt-submitted`, never `ok`"; now `send` consumes `LOCAL_COMMANDS`.
- The text as sent is classified by exact equality with a list entry when the slot is reserved: no trim, no case folding, never a leading-slash test. Every refusal rung applies to a listed command.
- A listed command with post-condition "none", and any listed command on an unverified CLI version (`/remote-control` on every version, `/clear` with no passing stamp): `send-issued`, the paste, then `send-confirmed {cursor, confirmed:false}` and `ok` `unconfirmable` at once, with no window. The verified bit is the version gate's `cli_verified` through `SendSlot::new`; the send path reads no `ledger/stamps.json`.
- `/clear` on a verified version waits the window for its post-condition only; no `prompt-submitted` claims it. The tap claims it on a `session-start` with `cause` `clear` and a string `agent_session_id` differing from the remembered id, appends the line unchanged, then settles with that line's `ts` as `submitted_at`.
- The remembered id is the last `session-start`'s `agent_session_id`, whatever its cause, in memory under the in-flight lock: no string id clears it, nothing remembered makes any string id new, nothing persists it.
- A window that closes first is `not-delivered` / `no-prompt-submitted` with the cursor; no new detail.
- `send-confirmed` data is `{cursor, confirmed?}` (was `{cursor}`): additive, no `v` bump, no product reader.
- The `hook.event` relabel sentence gains its one exception, the send waiting for a post-condition.
- [CLI Version Compatibility]: `send`'s use of the list "landed" (was "owed to the "Local-command send outcomes" route entry").
**Why:** the chunk landed the two clauses the matrix capability `v1-29` still owed. Limit kept in the body: the `/clear` confirmation is proven on the recorded 2.1.287 `clear-1` variants replayed by the fake agent; the live proof is owed to "First live test and self-drive".
**Kept:** one detail for a missed post-condition (`no-prompt-submitted`), no new `NotDelivered` value; the entry's opening sentences stay and are scoped to a send that is not a listed command.
**Ref:** .andromeda/runs/2026-10-06T23-49-33-wrap/
