
## 2026-10-04-wait-and-last — Path 3 as landed
**Section:** §6 E2E Test Strategy → Scenario: Path 3 — `wait` / `last` (Surfaces involved)
**Change:** Path 3 now records its landed half: `tests/cli_wait_last.rs` (`path3_wait_parks_until_turn_ended_then_last_reads_it`, `wait_after_a_send_cursor_returns_the_turn`, `last_survives_a_wrapper_restart`) over `fixtures/fake-scripts/path3.json`, and `tests/chaos_wait_vanish.rs`, cli + wrapper channel on all three CI OSes; the MCP steps owed to `:102`, `/api/sessions` to `:129`, the page STATUS to `:139`, the dialog kinds' end-to-end witness to `:78`.
**Why:** the scenario named four surfaces with no landed or owed status, overstating what this chunk covered.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/
