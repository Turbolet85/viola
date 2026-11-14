CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A `gate.py control` case names the test as its runner prints the NAME … never with nextest's binary id before it" (confidence 0.8)
                                              + testing.md: "A remove-the-guard control whose entry runs a walk that refuses an empty set reads UNREPRODUCED until that set exists" (confidence 0.8)
                                              + verification-harness.md: "A test-only recorder that replaces every off-list reply with a placeholder also hides why a live step failed …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 3 task-specific · 0 conflict (→ handoff) · 2 deferred (→ handoff)
  No-other-home: "a gate.py control case is the test name alone" · "a control over an empty-refusing walk reads UNREPRODUCED until the set exists" · "a placeholder recorder hides a live step's failure cause"
  CLAUDE.md size: 133/200 · T1 3.4 KB, 0 over 600 B

## Applied
- testing.md (tx 1087): the `gate.py control` case form.
  Proof: gate v1.33 refused all four cases written `<binary-id> <name>` at exit 2 («`case` is a test name, or an array of them, each as its runner prints it»); the name alone fired and held (`evidence/control-1.txt`). Signals: measured +0.4 · specific detail +0.2 · no other durable home +0.2 = 0.8.
- testing.md (tx 1087): UNREPRODUCED until the set exists.
  Proof: control firing 1 read `hygiene-title · UNREPRODUCED — in place red` on the empty Hyprland walk, firing 2 after L1 read `holds` (`evidence/control-1.txt`, `-2.txt`). 0.8, the same three signals.
- verification-harness.md (tx 1088): the placeholder recorder hides a live step's cause.
  Proof: L2 firing 1 recorded both dispatches exit 7 with the placeholder; the raw reply, read once by hand, named the Lua dispatch form (`evidence/operator-4-5-L2.txt`). 0.8, the same three signals.

## Each Decisions & corrections entry
1. The operator's directive at implement — not a learning (this chunk's directive; snapped as inputs).
2. L1 and L2 words — not a learning (records of this chunk's words).
3. The placeholder recorder hides the failure cause — applied (verification-harness.md).
4. Hyprland 0.56.2 runs dispatch as Lua — rejected, Filter 4: 0.6 (measured + specific); its home is the master row architecture `decision/hyprctl-direct-spawn` landed at P2, so neither conditional signal applies.
5. `gate.py control` takes the case name alone — applied (testing.md).
6. A proptest property comparing a parsed value's text with the input fails on non-canonical input — deferred, Filter 5 (0.8; the cap took three).
7. dash defaults an unset `PATH` — deferred, Filter 5 (0.8).
8. A control over an empty-refusing walk reads UNREPRODUCED — applied (testing.md).
9. Guard refusals re-issued once — not a learning (the guard put each call back on its documented path; `guard-refused`).
10. `fuzz/Cargo.lock` stale; `run --fuzz-replay` rewrites it in the tree — rejected, Filter 2 / Filter 4: 0.6, carried by this chunk's `--locked` gate entry.
11. The three plan defects — rejected, Filter 2: task-specific to this plan (routed to P5).
