
## 2026-10-10-self-healing-state — the torn append as landed: a stop and a shortened log, the appender heals, the reader counts three names
**Section:** §1 Test Scope Summary (Coverage triggers, chaos-test) · §2 Test Strategy (Test pyramid, Chaos / Fault row) · §4 Unit Test Strategy (What unit tests cover, viola-state) · §6 E2E Test Strategy (Chaos suite)
**Change:**
- §6 Chaos suite, torn append: the wrapper is stopped and its log shortened with `File::set_len`, in place of a kill mid-append with `Process::kill_with(Signal::Kill)`; the count is read through the product reader, 1 before the heal and 1 after; the next start's first record starts on a fresh line with every earlier byte unchanged; the role file holds one `state-recovered` line, which the case holds to the diag-line schema itself because a local run removes its test homes (`tests/chaos_torn_append.rs`). The `skipped.torn_lines` reading on `list --json` and `/api/sessions` is owed to "The board: viola list" and, for the page, to Epoch 8.
- §1 chaos-test trigger and §2 Chaos / Fault row: the same fault, a stopped wrapper's log shortened with `File::set_len` (was "kill the wrapper mid-write" / "kill mid-write"); the reader counts and rewrites nothing, the next append heals (was "readers count `torn_lines` and heal").
- §4 viola-state: the reader's three counts and what each takes (`three_counts`); the heal is the appender's, with `append_event_at`'s L + 1, `try_append_event` under a held lock, and one `state-recovered` line per heal (`torn_tail`). Was "Torn-line healing counts the line in `torn_lines` and never panics" and "An over-long line is counted as torn".
**Why:** the chunk built the fresh-line half of the chaos case and the counts. A stop leaves the same file on disk as a kill and no short coverage profile.
**Kept:** the observable, "the next append starts on a fresh line", is unchanged; the other chaos bullets stand.
**Ref:** .andromeda/runs/2026-10-10T11-03-09-wrap/
