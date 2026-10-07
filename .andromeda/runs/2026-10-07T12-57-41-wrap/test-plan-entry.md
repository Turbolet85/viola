
## 2026-10-07-send-waits-out-the-paste-hint — kills, hold and cap follow the 8.5 s gate; Path 2 delivered under the hint
**Section:** §3 → Bootstrap phases (the key file, `[profile.ci]` and `[profile.mutants]`) · §4 (Screen signatures; Refusal ordering) · §5 (the `cli_verify` bullet) · §6 Path 2 (Verification signal) · §7 (Fake agent, Modes)
**Change:**
- §3 key file: `test(/verify_window_/)` is 20 s × 3, a 60 s kill, under `[profile.ci]` (was 15 s × 3, 45 s) and 15 s × 3, a 45 s kill, under `[profile.mutants]` (was 15 s × 2, 30 s; the three `mutants` overrides no longer share one line). The no-screen case waits out the gate's 8.5 s maximum four times: 34.3 s on the dev host, up to 35.3 s on CI (was "about 21 s" at 5 s). The paste-hint case holds 9 s (was 6 s) and reads 13.1 s to 13.7 s on CI (was "about 10 s").
- §4: the readiness-gate verdict is ready / wait / `input-not-ready`; on a verified CLI a quiet screen with no input-box literal waits until `GATE_MAX_WAIT`, 8 500 ms, pinned by a unit case. `send` reads human-typing and then turn-running a second time after the gate returns ready (four clock-driven unit cases).
- §5: the `verify_window_` hint case holds 9 s, past the 8.5 s maximum.
- §6 Path 2: `send_under_the_paste_hint_on_a_verified_cli` is delivered in both cases (was `input-not-ready` under the hold). New `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`: exit 10 `human-typing`, nothing typed, the key reaches the child. It cannot tell `send`'s first wheel read from its second, so its green is a floor. The refusal at the bound is unit-tier only.
- §7: `--paste-hint-ms` is capped at 10 000 ms (was 8 000 ms).
**Why:** The gate's bound moved to 8.5 s on the founder's live ruling of 2026-10-07T10:29Z (relayed by the overseer), and the `verify_window_` class waits that bound four times by design. Both kills moved in proportion on the overseer's answer of 2026-10-07T12:10Z, with a planted hang shown killed under each moved bound and passing with none. Standing rule: a kill line moves only for a floor that is by design, with that control recorded both ways.
**Kept:** `WITHIN` (7 s), the `mutants` profile's own 10 s kill and every other override; no end-to-end case blocks to the gate's bound.
**Ref:** .andromeda/runs/2026-10-07T12-57-41-wrap/
