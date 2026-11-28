CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a lone ESC written right before the next key decodes with it as one Alt sequence — a case that presses Escape writes its next key only after the `escape` key line lands"
  Proof: implement run 2026-11-28T14-52-37Z-implement's outcome (Decisions & corrections, «Sweep hazard (test authoring)»; Deviations «Unit escape case»): `ESC v` read `unbound`; the E2E escape case waits for the `escape` key line before its next key (`tests/tui_console.rs::console_editing_line_left_by_escape_runs_nothing`, its wait at lines 1056-1058 of the report's New text, by line).
  Tier 3 (.claude/docs/session-learnings.md): extended — «Naming the process behind a truncated coverage profile» + "the writer can be a fake agent; name a cut file's binary by `llvm-profdata show --binary-ids` on a complete sibling matched against `readelf -n`"
  Proof: `.andromeda/cache/chunks/2026-11-28-console-actions/evidence/prepush-red-1.md` §What the files show: a complete sibling's binary id `c9125db1…` is the build id of `target/llvm-cov-target/debug/viola-fake-agent` (readelf).
  Filters: 0 dup · 4 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  Extended: T3/session-learnings.md: "Naming the process behind a truncated coverage profile" + the fake-agent writer and the binary-id read
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

Dispositions, every Decisions & corrections entry (implement's two runs, then After implement):
- Part 1 · the invocation's directive (Part 1 only, operator away, fakes only) — not a learning: a run's own directive.
- Part 1 · guard refusals re-issued once (a `cd`, a `cat` heredoc) — not a learning: the guard working; host-linux.md and the guard already carry both.
- Part 1 · sweep hazard, a lone ESC then a key decodes as one Alt sequence — applied, Tier 2 testing.md.
- Part 1 · measured, the two-column gutter cuts an 80-column row's LAST cell — rejected, Filter 2 (task-specific; the code and its pinned case hold it).
- Part 1 · measured, a gone instance draws no row — rejected, Filter 2 (the board's own rule, held by the code; reached P2 as check 6's disposed claim).
- Part 2 · the invocation's directive — not a learning: a run's own directive.
- Part 2 · guard refusal, a `cat` heredoc re-issued as Edit — not a learning: the guard working.
- Part 2 · own slip, a call beginning `cat > /dev/null;` blocked on stdin to the tool's limit — rejected, Filter 4 (one occurrence, a typo-class slip, no measured rule beyond «a command reading stdin with none waits»).
- Part 2 · measured, the key line cut after `m swi` at 80 columns — not a learning: his open fork (founder-forks.md F-K).
- Part 2 · measured, a 200-byte paste in at most four reads — rejected, Filter 2 (a figure the code's test states).
- Part 2 · measured, a CR inside a read ends the line and drops the rest — rejected, Filter 2 (the code's behaviour, pinned by `editing_a_cr_ends_the_read_and_drops_the_rest`).
- After implement · inputs#I9, the wrap directive — not a learning: this wrap's own directive.
- After implement · the key line to founder-forks — not a learning: his fork.
- After implement · the operator pass, no ruling (he was away) — not a learning.
- After implement · the three E2E verdicts that read other records — not a learning: dispositions of this chunk's acceptance, carried in the report.
- After implement · the pre-push red's mechanism (a truncated fake-agent profile) — extended, Tier 3 (above).
