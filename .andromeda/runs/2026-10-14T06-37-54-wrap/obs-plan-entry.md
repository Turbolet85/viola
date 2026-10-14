
## 2026-10-13-owed-ledger-rows — the statusline capture arm, uninstrumented, and verify's relied step lines
**Section:** §1 Obs Scope Summary (hook surface Exporter); §2 Agent-readable invariants (panic-line exemptions); §3 (hook stdout); §4 Edge flows (`verify`, the capture arms), Instrumentation per surface (hook row), Scenario: Budget governor; §10 bounded exemption 5
**Change:**
- A second hidden capture arm, `hook statusline --capture <DIR> [--shell <WORD>]` (sole caller Run B's probe settings, Unix only), is uninstrumented by design: no `VIOLA_*`, no obs init, no role or detail file, no `hook-invoked` / `hook-decision`, no channel, snapshot or `budget.json`, nothing printed and exit 0. It files `StatusLine.<k>.json` and its `bin-sh` | `other` verdict.
- Exemption 5 and the §2 exemption list were "the `hook --capture` arm"; they now name both capture arms. The hook row of the per-surface table, the §3 stdout bullet, the §1 Exporter line and the Budget governor scenario name it too.
- `verify`'s stdout: one `[NN/MM]` step line per relied row, `MM` the relied count (21 on Unix, 18 on Windows), with the summary summing to it. Was per ledger row.
- The six spawn pairs stay six. Run B's `verify-pty-probe` pair carries one probe `--settings` on Unix, and no line carries the probe command, a payload or a path.
**Why:** The capture arm is the founder's own answer to P4 fork 3 of this chunk, relayed verbatim by the operator (`inputs#I1`). The wrap's card confirm is PROVISIONAL (delegate viola-overseer), superseded by his own word at the Epoch 5 boundary. §1 is a verbatim copy kept current (the 2026-10-04 ruling).
**Ref:** .andromeda/runs/2026-10-14T06-37-54-wrap/
