CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   (extension only, below)
  Tier 3 (.claude/docs/session-learnings.md): + "Architecture holds verify's process-exit codes twice under one boundary name"
    Proof: this wrap's `rec.py rows-land --doc architecture --file ops-main-architecture.md --dry-run` refused `contract/verify-exit-detail/untrusted-folder · stands — stands as contract/verify-process-exit/untrusted-folder — the same name under another key` after tx 5038 landed the latter; the two families' rows read back by `rec.py rows --master architecture --boundary verify-exit-detail` (6) and `--boundary verify-process-exit` (7). Score: measured +0.4 · specific technical detail +0.2 · no other durable home +0.2 (the run dir's cascade-dispositions.md only) = 0.8.
  Filters: 0 dup · 2 task-specific · 0 conflict · 0 deferred
  No-other-home: "Architecture holds verify's process-exit codes twice under one boundary name"
  Extended: 2/verification-harness.md: "2026-10-04: A test that binds a Unix socket binds it under a short test-owned root made in `/tmp`, never under `TMPDIR` …" + "a live-reading script fired by hand under `--rehearse` points the wrapper's runtime dir under its scratch root, so that root is a short `mktemp -d` too, never a folder under the session scratchpad"
    Proof: implement's by-hand R7 rehearsal rooted under the session scratchpad read red with «local socket name length exceeds capacity of sun_path of sockaddr_un»; entry 8's own `mktemp -d` root read green (report Decisions & corrections; `scripts/r7-place.sh`'s `started` sets `XDG_RUNTIME_DIR={root}/run` under `--rehearse` only). Score: measured +0.4 · specific +0.2 · no other home +0.2 = 0.8. The matched entry records a defect, so this is also a recurrence: `recurrence-despite-learning` → handoff Deferred learnings.
  CLAUDE.md size: 136/200 · T1 4.8 KB, 0 over 600 B

Dispositions of the report's Decisions & corrections:
- implement · the overseer's implement directive inputs#I6 — not a learning (a one-chunk directive)
- implement · sweep hazard: a verify test failing rows through `"--trusted-root", elsewhere` now reads the untrusted-folder refusal — rejected, Filter 4 at 0.6 (measured +0.4, specific +0.2): its rule landed in test-plan decision/verify-failing-row-from-fixture-set this wrap, so no no-other-home signal
- implement · the by-hand rehearsal's sun_path red — extended (verification-harness.md, the 2026-10-04 entry) + recurrence-despite-learning → handoff
- implement · the Bash guard's refused `cd` — not a learning (the guard holds it; guard-refused)
- close · inputs#I6 operator pass without R7 — not a learning (one-chunk directive)
- close · inputs#I7 whose-word map — not a learning (recorded in the sidecars and the answer card)
- close · inputs#I7 item 3, jq / fake-agent preconditions ruled in — rejected, Filter 2 (task-specific: one script's words)
- close · inputs#I7 item 4, the v2-13 ledger note — not a learning (its home is the matrix note at P7.3)
- close · inputs#I7 item 5, route-resolve shape — not a learning (P5's)
- close · site-sweep hazard: `CAUSES` stands in no master — rejected, Filter 2 (a test table's name; no recurring sweep use)
- wrap conversation · the architecture double family — applied, Tier 3 (above)
