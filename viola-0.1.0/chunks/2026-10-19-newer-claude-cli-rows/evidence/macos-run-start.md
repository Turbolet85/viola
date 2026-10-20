# The macOS readiness flake — phase measurement (plan steps 6–7)

Written by /implement (run dir `.andromeda/runs/2026-10-19T23-48-08-implement/`). The operator pass's CI runs append
below.

## 6(a) — the stall report, installed

`tests/support/home.rs` `Wrapper::wait_ready`: at the `WITHIN` deadline, just before the `wrapper <name> not ready`
panic, `note_stall` adds these lines to the watch report:
- `stall last event <event>`: the wrapper's own last `run-<name>.ndjson` event, or `none`;
- `stall pid <pid>`: the `viola` whose arguments hold the boot's fake-agent control file, found through one sysinfo
  read. When none is found the line reads `stall pid unknown`;
- on macOS, `stall sample` and then `stack <line>` for each line of `/usr/bin/sample <pid> 1 -mayDie`'s
  `Call graph:` section, at most 120 lines. The file goes under the temp dir's `viola-root-watch/`. `sample` gets 4 s
  and is killed past that (`stall sample timed out`). It is a child the test started, and it reads the task without
  signalling it;
- on Linux, `stall wchan <wchan> state <state field of /proc/<pid>/stat>`.

The report adds nothing to a passing boot: `note_stall` runs only once the deadline has passed. Nothing signals the
wrapper. `WITHIN`, the kill lines, `retries` and the thread count are unchanged.

## 6(b) — the gap from `process-start{self}` to `process-start{version-probe}`, from recorded runs

**Source:** ci#39321117796 (sha `b6ddc19856e55fd9d4083db4400a8196f24f6488`), its diagnostics artifacts, downloaded
2026-10-19 ~23:53Z:
- `diag-macos-latest` attempt 1, artifact 12219784289 (20:47:16Z);
- `diag-macos-latest` attempt 2, artifact 12221562534 (21:00:54Z);
- for the two-sided read, `diag-ubuntu-latest` 12220487228 and `diag-windows-2025` 12220391778.

The attempt-1 JUnit (`junit-macos-latest`, artifact 12219874158) places the two failures:
- `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json`: started 20:44:50.784Z, 16.698 s;
- `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode`: started 20:44:50.770Z, 16.835 s.

Both watch reports read in full `starts w0 c0 r0 snapshot false beat false` / `starts w1 c0 r0 snapshot false beat
false`. So each wrapper's `process-start{self}` was seen, and no `claude-child` start ever followed it.

**Reader:** `viola-0.1.0/chunks/2026-10-19-newer-claude-cli-rows/macos-run-start.py`, run with `python3 -I`, one
`<label>=<dir>` per extracted artifact. Its output, verbatim, except that only the rows with a gap over 1 s and the
two `none` stalls are kept. The other `no version-probe` rows all end `last event process-exit`: those are starts the
tests refuse on purpose.

```
== ci#39321117796 macos attempt 1
self starts 264 · with a version-probe start 244 · without 20
gap ms min 29 · p50 48 · p90 93 · p99 5826 · max 5833
near, gap 1 s or less: median 5 · max 22
over 1 s: home viola-test-bRUKwy file run-builder.ndjson pid 3656 self at 2026-10-19T20:41:41.949Z gap 5833 ms near 2
over 1 s: home viola-test-coRo4C file run-builder.ndjson pid 17651 self at 2026-10-19T20:45:21.209Z gap 2482 ms near 4
over 1 s: home viola-test-rTYBu5 file run-builder.ndjson pid 3655 self at 2026-10-19T20:41:41.949Z gap 5826 ms near 2
over 1 s: home viola-test-wP9hoj file run-builder.ndjson pid 3658 self at 2026-10-19T20:41:41.949Z gap 5830 ms near 2
no version-probe: home viola-test-7lGEmW file run-builder.ndjson pid 16377 self at 2026-10-19T20:44:56.328Z last event none near 1
no version-probe: home viola-test-yyRGqm file run-builder.ndjson pid 16375 self at 2026-10-19T20:44:56.288Z last event none near 1
== ci#39321117796 macos attempt 2
self starts 265 · with a version-probe start 247 · without 18
gap ms min 32 · p50 58 · p90 98 · p99 232 · max 355
near, gap 1 s or less: median 2 · max 23
== ci#39321117796 ubuntu
self starts 265 · with a version-probe start 247 · without 18
gap ms min 157 · p50 203 · p90 345 · p99 666 · max 806
near, gap 1 s or less: median 4 · max 12
== ci#39321117796 windows
self starts 254 · with a version-probe start 232 · without 22
gap ms min 27 · p50 34 · p90 96 · p99 1080 · max 1088
near, gap 1 s or less: median 4 · max 20
over 1 s: home viola-session-Kp0lU9 file run-builder.ndjson pid 4784 self at 2026-10-19T20:48:16.593Z gap 1080 ms near 1
over 1 s: home viola-session-MdrlRD file run-builder.ndjson pid 8272 self at 2026-10-19T20:48:13.409Z gap 1084 ms near 3
over 1 s: home viola-session-tXL7SW file run-builder.ndjson pid 6496 self at 2026-10-19T20:48:06.823Z gap 1088 ms near 3
over 1 s: home viola-test-VDNemy file run-second.ndjson pid 4812 self at 2026-10-19T20:45:58.602Z gap 1075 ms near 1
```

**What it shows:**
- Only macOS attempt 1 has multi-second gaps. Three of them, 5826–5833 ms, are the job's first three wrapper starts,
  all at 20:41:41.949Z. The fourth, 2482 ms, is at 20:45:21Z, 25 s after the stall.
- The two stalls (pids 16375 and 16377, started 40 ms apart at 20:44:56.3Z) logged nothing after
  `process-start{self}`. Attempt 2 is clean, with a 355 ms maximum. Its first starts (20:51:54Z) were fast, so a slow
  first start is not a fixed cost of every job.
- Concurrency does not explain the gap (`near` = the other self starts within 1 s). The slow and stalled starts had
  1–4 neighbours, while the fast starts had a median of 5 and up to 22. A hypothesis that contention among
  simultaneous starts drives it is not supported by this run.
- The role file holds no line between `process-start{self}` and `process-start{version-probe}`: `run.start`'s steps
  1–6 (`resolve_program`, `canonical_home`, `collision_check`, `pin_and_plugin`, `statusline_command`, `plan_strip`)
  log nothing on success. So these readings bound the stalled span but cannot name a phase inside it.

## 6(c) — the stalled phase: NOT NAMED (as of this run)

6(b) does not locate the phase. 6(a)'s report fires only on a CI run that stalls, and no run with it installed exists
yet: the operator pass's CI runs are the first. Until one of them stalls and its report names a phase, the cause is
unknown, and `pin_exe`'s whole-exe read, hash and copy stays a hypothesis.

## 7 — the fix: NOT RUN (as of this run)

By plan step 7's own rule, step 7 does not run while neither (a) nor (b) locates the phase. This criterion reads
unmet, and the red stays open until its cause is known (CLAUDE.md Tier 1). A green `test (macos-latest)` on a
first attempt would not close it.

## Operator pass — CI readings

(none yet)
