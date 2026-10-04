
## 2026-10-04-running-turn-refusal — the turn-running control negative
**Section:** §5 CLI → `tests/cli_controls_not_disableable.rs`
**Change:** was "four control negatives"; now five: ESC in `send` text → 13 `control-character`; `answer` on an unstamped home → 12; human wheel → `send` 10; a running turn → `send` 13 `turn-running`; a 0770 `--home` → 21. As landed, the turn-running row (`send` during a running harness turn → exit 13 under every `FAKE_AGENT_HOOK_PANIC` setting) joined with this chunk.
**Why:** no setting may disable a control, and `turn-running` is now a control on a running turn, not only on a second send in flight.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/
