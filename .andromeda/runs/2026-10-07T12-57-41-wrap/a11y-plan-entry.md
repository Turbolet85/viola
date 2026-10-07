
## 2026-10-07-send-waits-out-the-paste-hint — the gate's bound falls on the driver; a key during the wait wins
**Section:** §3 → Keyboard test harness (the key file, the tui sentence) · §4 P4 (the tui bullet) · §8 Timeout extensions (the CLI clause)
**Change:**
- §8: a `viola send` on a verified CLI waits in the readiness gate for the input box for at most `GATE_MAX_WAIT` (8.5 s) and is then refused `not-delivered` / `input-not-ready` with nothing typed. The bound falls on the driver only: a key pressed during the wait goes to the child and takes the wheel at once, and the waiting send is refused `human-typing`.
- §3 key file and §4 P4: a further outer-PTY case, `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, under boundary clause (2). It is sequenced on the wrapper's `channel-request` line and the `wheel` record, never a timer. Its stated limit: it cannot tell `send`'s first wheel read from its second, whose own proof is the unit tier. The three boundary cases stand as they were.
**Why:** The gate's wait and its 8.5 s bound are the founder's live ruling of 2026-10-07T10:29Z, relayed by the overseer. The refusal of a send whose wait a human key fell into is the overseer's technical answer of 2026-10-07T12:10Z, not the founder's: "the human always wins" has to hold through a wait of seconds. The case's limit is stated on the overseer's disposition at the wrap.
**Ref:** .andromeda/runs/2026-10-07T12-57-41-wrap/
