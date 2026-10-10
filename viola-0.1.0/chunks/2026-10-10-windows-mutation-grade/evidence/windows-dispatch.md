# The `windows-mutants` dispatch of this chunk (plan step 11)

Written 2026-10-10, after the run finished. Times are `date -u` or the run's own.

## On whose word

The founder's own, given live in the overseer dialog at 2026-10-10T07:58:27Z after this dispatch was shown to him
with three options priced: «Разрешить 1–2 запуска сейчас». It reached the implementer relayed verbatim by the
operator and is snapshotted as `inputs#I5`; it was cited in `operator-pass.md` (the section "Step 11") before
anything was fired. Its bounds: one or two dispatches, for this chunk only; a third comes back to him.

**This is dispatch 1 of at most 2. No second dispatch was made:** every job read green, so nothing called for one.

## The dispatch

- `gh workflow run windows-mutants.yml --ref build/viola-0.1.0` at 08:01:33Z → exit 0, run **38036448183**.
- Read from the run itself (`gh api …/actions/runs/38036448183`): event `workflow_dispatch`, head
  `dd5161d55743c6b8bf9cf83e7470a37597b7a886` (the pushed pre-CI commit, equal to the remote branch head read live
  before the dispatch), `run_attempt` **1**, created 08:01:34Z, last updated 08:50:59Z, `completed` / `success`.
  Every one of its nine jobs is at attempt 1.
- The push's own CI verdict on this sha (`ci#38021000200`, green, 15/15, attempt 1) was read at step 10, before the
  dispatch joined the sha's checks.

## Entry 26, driven once by hand

- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --name
  mutants --wait 5400`, started 08:01:43Z, returned 08:51:18Z, **exit 0**. Its output, whole (the last field of the
  verdict line is clipped by the tool):

  ```
  ci v1.2 · 881cd498
  repo Turbolet85/viola (the push remote `origin`) · names mutants · polled 97× over 2975 s
  dd5161d55743 verdict: green · checks 9/24 · wall 2961 s · runs windows-mutants#38036448183 completed/success ci#3802100…
  runs: windows-mutants#38036448183 workflow_dispatch completed/success · ci#38021000200 push completed/success
  checks: mutants (viola-agent-claude) · mutants (viola-channel) · mutants (viola-cmd-run) · mutants (viola-e2e)
    mutants (viola-main) · mutants (viola-panic-frames) · mutants (viola-pty) · mutants (viola-run-env)
    mutants (viola-state)
  ```
- Atoms: `exit 0` ✓, `contains verdict: green` ✓. It read the nine `mutants (…)` checks of the sha's 24.
- The entry fired no dispatch and no mutation run and wrote nothing in the tree.

## The nine jobs

Each log was read through `gh api repos/Turbolet85/viola/actions/jobs/<id>/logs --allow-escape-sequences`, the
escapes stripped. The counts are each job's own harness document and its cargo-mutants summary line; the outcome
lines were counted beside them and agree in every job. "Left out" is the document's `host_excluded`; "forecast" is
the audit's pinned recipe (`cover.py` through `forecast.py`), re-read on this tree at 08:02Z: 642 mutants, 30 left
out on Windows, the same as `host-excluded.md` recorded at step 9.

| job (id) | wall | baseline | tested | caught | unviable | left out | missed | timeout | `ok` | `verdict` | forecast: mutants / left out |
|---|---|---|---|---|---|---|---|---|---|---|---|
| viola-pty (114167755367) | 13 m 41 s | 12 s build + 3 s test | 89 | 73 | 11 | 5 | 0 | 0 | true | package | 89 / 5 — equal |
| viola-channel (114167755343) | 11 m 00 s | 30 s + 1 s | 148 | 127 | 17 | 4 | 0 | 0 | true | package | 148 / 4 — equal |
| viola-state (114167755272) | 14 m 43 s | 43 s + 4 s | 156 | 138 | 11 | 7 | 0 | 0 | true | package | 156 / 7 — equal |
| viola-agent-claude (114167755465) | 6 m 36 s | 32 s + 2 s | 40 | 36 | 4 | 0 | 0 | 0 | true | package | 40 / 0 — equal |
| viola-cmd-run (114167755487) | 26 m 11 s | 75 s + 98 s | 50 | 32 | 14 | 4 | 0 | 0 | true | package | 50 / 4 — equal |
| viola-main (114167755387) | 41 m 30 s | 113 s + 103 s | 34 | 33 | 1 | 0 | 0 | 0 | true | package | 34 / 0 — equal |
| viola-run-env (114167755401) | 49 m 21 s | 113 s + 104 s | 33 | 24 | 9 | 0 | 0 | 0 | true | package | 33 / 0 — equal |
| viola-panic-frames (114167755346) | 45 m 07 s | 86 s + 100 s | 23 | 14 | 0 | 9 | 0 | 0 | true | package | 23 / 9 — equal |
| viola-e2e (114167755423) | 18 m 02 s | 72 s + 58 s | 69 | 60 | 8 | 1 | 0 | 0 | true | package | 69 / 1 — equal |
| **total** | | | **642** | 537 | 75 | 30 | **0** | **0** | | | 642 / 30 |

- 537 caught + 75 unviable + 30 left out = 642 tested. "Missed" is the document's `survived` and `failed`, both 0 in
  every job, with `failures` empty.
- cargo-mutants itself still grades the left-out mutants MISSED: its summary lines read 5, 4, 7, 0, 4, 0, 0, 9 and
  1 missed. In every job the MISSED outcome lines are exactly the mutants of that job's `host_excluded`, name for
  name, and no other line reads MISSED. No line reads TIMEOUT in any job.
- **`host_excluded` against the forecast:** equal in all nine jobs, by mutant name and by predicate. 26 rows carry
  `unix`; the four of `viola-cmd-run` carry `all(windows, not(target_arch = "x86_64"))`. The plan predicted 29 at
  its HEAD; the final tree reads 30 (`host-excluded.md` explains the one: `fs.rs:19:5`). No mutant in a body the
  runner compiles is in any list: each row was read against the recipe, which decides from source.
- `unviable ≤ caught` holds in every job (the closest is `viola-cmd-run`, 14 against 32).
- Each document's `files` equals its matrix item's list, and `verdict` is `package` in all nine.
- Absolute-path probe on each document (`C:`, `D:`, `/home/`, `/Users/`): 0 hits in all nine. Upload probe on each
  log (`upload-artifact`, `Uploading artifact`): 0 in all nine.
- **The ceiling:** the longest job ran 49 m 21 s against `timeout-minutes: 120`. Every job printed its document.

## The wall-time forecast, and what was read

The plan estimated about 52 min for `viola-cmd-run`, as a floor, and 16 min or less for every other job. The
total stayed inside the estimate's longest job, but the shape was wrong:

| job | forecast | read |
|---|---|---|
| viola-cmd-run | about 52 min, a floor | 26 m 11 s |
| viola-main | 16 min or less | 41 m 30 s |
| viola-run-env | 16 min or less | 49 m 21 s |
| viola-panic-frames | 16 min or less | 45 m 07 s |
| viola-e2e | 16 min or less | 18 m 02 s |
| the other four | 16 min or less | 6 m 36 s to 14 m 43 s |

Read from the outcome lines: in the three slow jobs a graded mutant costs about one whole pass of the root tests
(the baseline's test time there is 100 to 104 s). `viola-run-env`: 24 caught, median test time 102 s.
`viola-panic-frames`: 14 caught at a median of 95 s and 9 left-out mutants at 101 s each (a left-out mutant is
still built and tested; it is left out of the count afterwards). `viola-main`: 33 caught, median 45 s.
`viola-cmd-run` came in under its estimate because 14 of its 50 are unviable and its 32 caught have a median test
time of 13 s. Why a caught mutant of the root package so often takes the full pass was not read by this chunk.

## What this run graded for the first time

- **The sixteen survivors:** twelve read caught on `windows-2025` and four are no longer generated. Each row's
  outcome line is in `survivors.md`, the Windows column.
- **The six retry-loop mutants and both `fs.rs:301:19` guard mutants** read caught at this source, and so do the
  two `prepare` mutants (`scratch.rs:48:5`, `:54:8`).
- **`src/run/env.rs`:** all 33 mutants are graded, 24 caught and 9 unviable. The last dispatch graded 18 of the 33.
- **`mutants (viola-e2e)`** ran for the first time since its baseline was fixed, and read green.
- **The harness's host reader on a Windows host,** for every job, the three shapes `host-excluded.md` listed as
  unread among them where they occur on this host: the architecture key (`viola-cmd-run`). The `mod`-declaration
  shape and the unknown-key shape do not occur in a Windows host's left-out set and stay unread by a real run.

## The nine harness documents, verbatim

Each is the job's own `{"v":1,"cmd":"run",…}` line, one per job, copied from its log.

`mutants (viola-pty)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":73,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":89,"verdict":"package","package":"viola-pty","files":["crates/viola-pty/src/lib.rs","crates/viola-pty/src/sideload.rs"],"scratch_bytes":0,"host_excluded":[{"name":"crates/viola-pty/src/lib.rs:321:9: replace HostTerminal::enter -> Option<Self> with None","cfg":"unix"},{"name":"crates/viola-pty/src/lib.rs:321:9: replace HostTerminal::enter -> Option<Self> with Some(Default::default())","cfg":"unix"},{"name":"crates/viola-pty/src/lib.rs:323:64: replace != with == in HostTerminal::enter","cfg":"unix"},{"name":"crates/viola-pty/src/lib.rs:328:71: replace == with != in HostTerminal::enter","cfg":"unix"},{"name":"crates/viola-pty/src/lib.rs:487:76: replace != with == in host_size","cfg":"unix"}]},"archived":"target/run-archive/1"}
```

`mutants (viola-channel)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":127,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":148,"verdict":"package","package":"viola-channel","files":["crates/viola-channel/src/client.rs","crates/viola-channel/src/endpoint.rs","crates/viola-channel/src/server.rs","crates/viola-channel/src/server/win.rs","crates/viola-channel/src/test_support.rs"],"scratch_bytes":0,"host_excluded":[{"name":"crates/viola-channel/src/client.rs:272:5: replace open_by -> io::Result<Stream> with Ok(Default::default())","cfg":"unix"},{"name":"crates/viola-channel/src/endpoint.rs:76:5: replace host_socket_dir -> PathBuf with Default::default()","cfg":"unix"},{"name":"crates/viola-channel/src/server.rs:91:9: replace <impl Drop for Guard>::drop with ()","cfg":"unix"},{"name":"crates/viola-channel/src/server.rs:124:5: replace listen -> Result<(Listener, Guard), ChannelError> with Ok((Default::default(), Default::default()))","cfg":"unix"}]},"archived":"target/run-archive/1"}
```

`mutants (viola-state)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":138,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":156,"verdict":"package","package":"viola-state","files":["crates/viola-state/src/pin.rs","crates/viola-state/src/fs.rs","crates/viola-state/src/strict.rs"],"scratch_bytes":0,"host_excluded":[{"name":"crates/viola-state/src/fs.rs:19:5: replace restrict -> io::Result<()> with Ok(())","cfg":"unix"},{"name":"crates/viola-state/src/strict.rs:128:9: replace unix::reading -> io::Result<(u32, u32)> with Ok((0, 0))","cfg":"unix"},{"name":"crates/viola-state/src/strict.rs:128:9: replace unix::reading -> io::Result<(u32, u32)> with Ok((0, 1))","cfg":"unix"},{"name":"crates/viola-state/src/strict.rs:128:9: replace unix::reading -> io::Result<(u32, u32)> with Ok((1, 0))","cfg":"unix"},{"name":"crates/viola-state/src/strict.rs:128:9: replace unix::reading -> io::Result<(u32, u32)> with Ok((1, 1))","cfg":"unix"},{"name":"crates/viola-state/src/strict.rs:134:9: replace unix::euid -> u32 with 0","cfg":"unix"},{"name":"crates/viola-state/src/strict.rs:134:9: replace unix::euid -> u32 with 1","cfg":"unix"}]},"archived":"target/run-archive/1"}
```

`mutants (viola-agent-claude)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":36,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":40,"verdict":"package","package":"viola-agent-claude","files":["crates/viola-agent-claude/src/lib.rs"],"scratch_bytes":0},"archived":"target/run-archive/1"}
```

`mutants (viola-cmd-run)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":32,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":50,"verdict":"package","package":"viola","files":["src/cmd/run.rs"],"scratch_bytes":0,"host_excluded":[{"name":"src/cmd/run.rs:385:5: replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with (\"\", None)","cfg":"all(windows, not(target_arch = \"x86_64\"))"},{"name":"src/cmd/run.rs:385:5: replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with (\"\", Some(Default::default()))","cfg":"all(windows, not(target_arch = \"x86_64\"))"},{"name":"src/cmd/run.rs:385:5: replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with (\"xyzzy\", None)","cfg":"all(windows, not(target_arch = \"x86_64\"))"},{"name":"src/cmd/run.rs:385:5: replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with (\"xyzzy\", Some(Default::default()))","cfg":"all(windows, not(target_arch = \"x86_64\"))"}]},"archived":"target/run-archive/1"}
```

`mutants (viola-main)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":33,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":34,"verdict":"package","package":"viola","files":["src/main.rs","src/conpty.rs"],"scratch_bytes":0},"archived":"target/run-archive/1"}
```

`mutants (viola-run-env)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":24,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":33,"verdict":"package","package":"viola","files":["src/run/env.rs"],"scratch_bytes":0},"archived":"target/run-archive/1"}
```

`mutants (viola-panic-frames)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":14,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":23,"verdict":"package","package":"viola","files":["src/panic_frames.rs"],"scratch_bytes":0,"host_excluded":[{"name":"src/panic_frames.rs:69:5: replace raw_frames -> Vec<usize> with vec![]","cfg":"unix"},{"name":"src/panic_frames.rs:69:5: replace raw_frames -> Vec<usize> with vec![0]","cfg":"unix"},{"name":"src/panic_frames.rs:69:5: replace raw_frames -> Vec<usize> with vec![1]","cfg":"unix"},{"name":"src/panic_frames.rs:80:5: replace module_of -> Option<(String, usize)> with None","cfg":"unix"},{"name":"src/panic_frames.rs:80:5: replace module_of -> Option<(String, usize)> with Some((String::new(), 0))","cfg":"unix"},{"name":"src/panic_frames.rs:80:5: replace module_of -> Option<(String, usize)> with Some((String::new(), 1))","cfg":"unix"},{"name":"src/panic_frames.rs:80:5: replace module_of -> Option<(String, usize)> with Some((\"xyzzy\".into(), 0))","cfg":"unix"},{"name":"src/panic_frames.rs:80:5: replace module_of -> Option<(String, usize)> with Some((\"xyzzy\".into(), 1))","cfg":"unix"},{"name":"src/panic_frames.rs:82:77: replace == with != in module_of","cfg":"unix"}]},"archived":"target/run-archive/1"}
```

`mutants (viola-e2e)`:

```json
{"v":1,"cmd":"run","ok":true,"suites":[{"suite":"mutants","passed":60,"failed":0,"skipped":0,"survived":0,"artifact":"mutants.out/outcomes.json","failures":[]}],"mutants":{"tested":69,"verdict":"package","package":"viola-e2e","files":["crates/viola-e2e/src/harness/run/mutants/scratch.rs","crates/viola-e2e/src/harness/cleanup.rs","crates/viola-e2e/src/harness/run/browser.rs"],"scratch_bytes":0,"host_excluded":[{"name":"crates/viola-e2e/src/harness/cleanup.rs:145:9: delete ! in unconnectable","cfg":"unix"}]},"archived":"target/run-archive/1"}
```

## Not measured by this dispatch

- The four `src/cmd/run.rs:385:5` mutants: left out on this host as on Linux, graded on no host
  (`host-excluded.md`).
- Any mutant of a file outside the workflow's nine items.
- A second run of the same sha: one run, one attempt per job.
