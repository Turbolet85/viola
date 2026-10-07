CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + host-win32.md: "On the Linux dev host a process named `viola` is not this repository's by its name … decide
      whose a process is by `/proc/<pid>/exe` against the repository root." (confidence 0.8)
      Proof: implement step 5, 2026-10-07T07:47Z: `pgrep -x viola` read 9 processes, 0 with an executable under this
      repository's root (`evidence/backing.md`, "The quiet-host precondition, as it read"); the plan's check by name
      could not read empty. Scored 0.4 measured (it falsified a plan step) + 0.2 specific detail = 0.6, + 0.2
      load-bearing for the next entry. The file has no `paths:`, so the entry is held to Tier 1's bar: one sentence,
      279 B (measured). The two extended entries read 1 025 B and 1 088 B after the write, under the 1.5 KB cap.
    + verification-harness.md (extended): the 2026-09-29 entry gains "Extended 2026-10-07: on the Linux dev host the
      homes sit on tmpfs behind the `target/e2e-home` link, so a stalled-start red is first a finding about the
      backing … stop and report it; never re-run for a green." (confidence 0.9)
      Proof: the overseer's disposition (4) with the wrap invocation ("from now on"), and the contended reading:
      window 08:08:52Z, control write median 10.999 s on the shared volume, 0.285 s at most through the link,
      `binary(cli_verify)` 29 passed, 0 failed (`evidence/contended-reading.ndjson`). Scored 0.3 from-now-on + 0.4
      measured + 0.2 specific detail.
    + testing.md (extended): the 2026-09-25 entry gains "Extended 2026-10-07: when the new cases call the guarded
      function directly, remove the guard inside that function …" (confidence 0.8)
      Proof: implement step 3: the plan's call-site swap could not reach cases that call the keeper over their own
      temp dirs; with the link arm switched off inside each keeper, case 2 read `AlreadyExists` red, run archives 651
      and 653, and green restored, 652 and 654 (`evidence/keeper-control.md`). Scored 0.4 measured (it falsified a
      plan step) + 0.2 specific detail = 0.6, + 0.2 no other durable home.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 1 task-specific · 0 conflict · 0 deferred · 1 below threshold · 3 recurrences (→ handoff)
    - task-specific: "a natural window is found by its trigger, not by its first line" (one chunk's driver).
    - below threshold: "`find` over a link needs a trailing slash on the start point" scored 0.6 (measured + specific
      detail) and takes neither conditional signal: this wrap amended it into obs-plan §9 (the G2 comment) and the
      gotchas leaf carries it.
    - recurrence-despite-learning: `host-win32.md 2026-09-28/29` (a heredoc with a file target, refused by the Bash
      guard; the Write tool, then run by path) · `host-win32.md` Compound commands (`rm -rf` in a compound, the whole
      call denied) · a time written ahead of the clock into an evidence file (the stamp hook refused it; `date -u`
      is read, never estimated).
  Load-bearing: "a process named `viola` is not this repository's by its name" → Live rows and paste shapes on the dev host
  No-other-home: "when the new cases call the guarded function directly, remove the guard inside that function"
  Extended: T2/verification-harness.md: "2026-09-29: On this host the local suite grades an identical tree
    differently run to run …" + the backing facet · T2/testing.md: "2026-09-25: Every new guard test carries its
    remove-the-guard run …" + the direct-call facet
