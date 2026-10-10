
## 2026-10-10-statusline-pass-through — the capability ledger: three relied-on dependences with no row, and where each row is owed
**Section:** §Cross-cutting Patterns (Capability ledger as the single gate) · §Established Decisions ([CLI Version Compatibility]: the Statusline rows bullet, the closing paragraph)
**Change:**
- Capability ledger: "Two relied-on shapes have no row yet" (per "2026-10-10-viola-revive — root wait count 26 sites in 19 files, the kill path's endpoint wait, ten fake-agent options, a second relied-on shape with no row") is now three relied-on dependences: `send`'s LF inside a paste, `viola revive`'s `claude --resume <id>`, and the statusline pass-through's three shapes (the payload's `rate_limits` object, the `statusLine` the child takes through `--settings`, `/bin/sh` as the shell the CLI runs a statusline command string through). The three were read by hand in three live sessions on 2.1.287 on the Linux dev host with a project-scope status line; CI reads them under the fake agent only. The ledger stays at seventeen rows.
- The rows are owed: `rate_limits` on the working-route entry "Budget governor", the settings override and the Unix shell on "Paste newline ledger row", the Windows legs of both on "Windows-only live measurements".
- [CLI Version Compatibility]: the Statusline bullet of the row list says none of the three has a row yet; the closing paragraph no longer says the statusline rows land with Epochs 4 and 5.
**Why:** the founder chose a hand reading of three live starts with no row, no probe and no fixture for this chunk (live, 2026-10-10, relayed verbatim by the operator); the operator's direction at this wrap, 2026-10-10, named the entry each row is owed to.
**Kept:** the pattern's rule stands: a new dependence on an undocumented `claude` behaviour is a ledger row with a probe. These three are named exceptions with an owner each, not a change to the rule.
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/
