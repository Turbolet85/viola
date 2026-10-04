
## 2026-10-04-the-wheel — pause and release served; release-from-driver landed
**Section:** §Standard Contracts → Channel methods · Established Decisions → [MCP] · §Conventions → CLI exit codes (`1`) · §Occupied Resources → Filesystem (`diagnostics/`) · §Infrastructure Patterns → Project directory structure
**Change:**
- Channel methods: was `pause` `{}` and `release` `{budget?:bool}`; now `pause` `{from?}` → `{wheel:"human"}` and `release` `{budget?:bool, from?}` → `{wheel, budget_paused}`. A string `from` on `release` is `-32602` "invalid params" with `data: {"reason":"release-from-driver"}` plus one obs line; another `from` type or a non-bool `budget` (`null` included) is `-32602` `data: null`; `budget:true` leaves the wheel. Both reply only after their `wheel` record lands (`-32603` if not). The `from` readers add `pause`.
- [MCP]: was "whether a wrapper should refuse a `release` whose `params` carry `from` is left to the security specialist"; now the wrapper refuses a string `from` (`release-from-driver`), so CLI `viola release` inside a wrapped session is refused — self-reported, a deterrent, not enforcement.
- The `cli` role's verbs add `pause` and `release` (exit 1 on an internal error); `cli-<name>.ndjson` producers add `pause` / `release`.
- Directory structure: `src/run/` adds `wheel.rs` and `snapshot.rs`; `human.rs` adds the `pause` / `release` lines and callers; viola-core adds `HumanTyping`, `WheelCause`; viola-pty adds `host_stdin()`.
**Why:** the chunk served the two wheel methods (were `-32601`), the security plan's `release-from-driver` guard landing with them.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/
