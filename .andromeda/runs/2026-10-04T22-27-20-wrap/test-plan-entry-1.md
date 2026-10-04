
## 2026-10-04-running-turn-refusal — Path 5 and Path 2 span the running turn
**Section:** §6 E2E → Scenario: Path 5 (As landed · step 8 · Verification signal) · Scenario: Path 2 (As landed)
**Change:**
- Path 5 step 8 boots the harness turn over the gated `fixtures/fake-scripts/path3.json`: a `send` while the harness turn runs, then its scripted `PostToolUse` and `Stop` released and a `send` after `turn-ended`.
- Path 5 signal: was "the harness-injected turns log `prompt-submitted{origin:"harness"}` and no `wheel` record" alone; now also, during the turn, `send` exits 13 `{"v":1,"refusal":"not-delivered","detail":"turn-running"}`, nothing typed, one `send-refused{refusal, detail}` with no `cursor` and no `send-issued`, and `send` exits 0 after `turn-ended`; with no Stop a bare `release` leaves the turn (13) and `pause` then `release` clears it.
- Path 5 As landed adds the harness-turn refusal and `tests/cli_wheel.rs` `path5_a_turn_left_running_is_cleared_by_pause_then_release`.
- Path 2 As landed: `path2_send_confirms_with_cl1_events` boots over `path3.json` and ends its first send's turn before the second; `send_after_a_confirmed_send_is_turn_running_until_turn_ended` covers the driver's own turn (exit 13, the `[/ ] unable … turn-running` line + `hint: a turn is running; viola wait builder first`, then 0 after `turn-ended`).
**Why:** a running turn now refuses `send`, so every test that sends twice must end the first turn with a scripted Stop (the fake agent's interactive submit fires none), and the closed hypothesis is witnessed on three CI OSes.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/
