# Curation — wrap of 2026-09-26-local-linux-pre-push-gate

Scope: this session (the resumed wrap window) + the report's *Decisions & corrections*. The implement window's
conversation is gone (the wrap paused across a session boundary), so a correction only that conversation held and
the report did not carry is not curated here.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "To test a timing window, force it open with a test-only hold …" (confidence 1.0)
                                              + verification-harness.md: "A nextest `test(=name)` filter needs the test's full path …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Extended: T2/testing.md: "A test wait that a mutant can reach must … be bounded below … the nextest mutants kill" + "a test's own assertion deadline must also sit below the nextest profile's kill line" (confidence 0.8)
  No-other-home: "nextest `test(=name)` needs the full `tests::` path" · "a test's deadline must sit below the runner's kill line"
  Filters: 3 dup/homed (the `wsl --` vs `--exec` hazard and the distro-PATH fact: amended into test-plan §3 and `docs/gotchas.md` this wrap; `MSYS2_ARG_CONV_EXCL` for `/mnt/…`: covered by host-win32.md §Paths) · 2 below threshold (apt lists stale → the chunk plan's standing entry + `docs/commands.md` carry it, 0.6; the ~850 s vs 1301 s wall-clock pair: a metric, not a learning) · 0 conflict · 1 deferred (→ handoff)
  CLAUDE.md size: 122/200 · T1 1.5 KB, 0 over 600 B

## Proofs
- testing.md "force the window": the overseer's correction ("the cause must be MEASURED, not inferred from 'it went
  green'; a variation that does not depend on luck — force the window — before sampling") (+0.4 user correction,
  +0.3 never/must language); measured at `evidence/linux-red-investigation.md` §Experiment A: the gate went red 3/3 on
  `tui_host_resize_reaches_the_child` while 68 isolated reproductions stayed green; a 1 s hold forced the window,
  6/6 red with the exact signature, control 6/6 green, 6/6 green after the fix (+0.4 measurement). Score 1.0 (capped).
- verification-harness.md "`test(=name)` full path": the window-test gate entry authored with `test(=…)` selected
  nothing for an inline unit test; baseline read `nextest-exit-4` (report §Deviations, §Decisions & corrections)
  (+0.4 real gate failure, +0.2 specific technical detail, +0.2 no-other-home: carried by no master, route line or
  ledger note). Score 0.8.
- testing.md extension "deadline below the kill line": nextest's `mutants` profile killed the resize test at 10.006 s,
  before its 10 s assertion could print the dump; the test's deadline moved to 8 s (`RESIZE_WITHIN`) below the
  profile's 10 s kill (report §Harness / gate surface, §Decisions & corrections) (+0.4 measurement, +0.2 detail,
  +0.2 no-other-home). Score 0.8.

## Deferred (Filter 5 cap)
- `git reset --hard` already drops files the clone's own `add -A` staged, so a `clean` guard's red half needs a stray
  clone-side file — the first g8 pair was vacuous (confidence 0.8; candidate facet of testing.md's remove-the-guard entry).
