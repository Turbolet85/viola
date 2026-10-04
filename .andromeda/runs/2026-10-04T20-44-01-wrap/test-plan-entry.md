
## 2026-10-04-the-wheel — Path 5 as landed; the human-wheel controls row
**Section:** §6 E2E → Scenario: Path 5 (Surfaces involved) · §5 Integration → `tests/cli_controls_not_disableable.rs` · §3 → Log format
**Change:**
- Path 5: as landed, `tests/tui_wheel.rs` (the outer-PTY steps, the harness-turn case, the focus/mouse/resize case, the `^Z` case) and `tests/cli_wheel.rs` (`pause` / `release`, the `human-typing` refusals with detail `null` | `manual-pause`, hints without `release`, `release-from-driver` exit 20) cover tui, cli and the wrapper channel on all three CI OSes; on `windows-2025` the focus case asserts the platform fact (an injected mouse report takes the wheel, focus reports are swallowed), Unix the full assertion. The MCP `send` step and its refusal checks are owed to `:102`, the Playwright WHEEL cell to `:139`.
- The controls table: was "the four verb negatives … join when `send` / `answer` land"; now the human-wheel negative (`send` exit 10) has joined, the rest still to join.
- Log format: the `release-from-driver` corr rule names a `release` carrying a string `from` (another type is a plain `-32602`), matching obs-plan §3.
**Why:** the chunk landed Path 5's CLI and outer-PTY halves; the Windows clause is the founder's live ruling F-W3, relayed by the overseer; the Log format line keeps the tests↔obs §3 bind.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/
