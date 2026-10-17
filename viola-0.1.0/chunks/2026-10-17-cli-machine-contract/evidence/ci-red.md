# The last wrap's red CI — `ci#38994749835`, `test (ubuntu-latest)`

Part A of the plan (steps 1-4). Readings in the order they were taken.

## 1. The recorded run's artifacts (step 1)

Downloaded 2026-10-17T02:52Z with `gh run download 38994749835 -n <name> -D target/ci-38994749835/<name>` for
`harness-ubuntu-latest` (211 MB on disk), `diag-ubuntu-latest` (2.0 MB) and `junit-ubuntu-latest` (492 KB).

- `diag-ubuntu-latest` keeps `target/e2e-home/**/diagnostics/*.ndjson` only (ci.yml's upload path): 216 homes, 466
  files — `run-*` 183, `hook-*` 138, `cli-*` 89, `detail-*` 12. `viola verify` called without a `VIOLA_NAME` (the
  `stamped_home` fixture's call) opens no process log, so no line of the two failing verify runs is in it; and the
  probe dir `<home>/ledger/probes/<pid>/` with its Run E captures is removed when verify returns, on every path.
- `harness-ubuntu-latest` keeps the harness sessions' captures and homes (`cli-drive-*`, `harness_lifecycle-*`,
  `path6_budget_harness-*`), none of them a `cli_controls_not_disableable` home.
- `junit-ubuntu-latest` `junit.xml`, the three cases of `setting_does_not_disable_the_budget_gate`:
  - `case_1_over_one_hundred` — pass, 4.340 s, started 02:25:19.564;
  - `case_2_negative` — fail, 4.464 s, started 02:25:20.385;
  - `case_3_a_string` — fail, 4.439 s, started 02:25:20.405 (20 ms after case 2);
  - each failure's only text: `panicked at tests/support/home.rs:368:5: viola verify did not stamp the home: exit
    Some(1), summary line matched false, failing rows ["resume-session"]`. The verify run's own stdout (its step
    lines) is not kept: the fixture prints only the failing ids.

**What the artifacts show:** which row failed (`resume-session`, alone), that the two failing runs started 20 ms apart
and ran as long as the passing one (so Run E did not wait out `PROBE_DEADLINE` for a capture that never came), and
nothing more. They do not show which of the check's arms failed (no Run E capture, another id, another source): no
artifact keeps a Run E capture or a verify step line.

## 2. The mechanism read from the code

The `resume-session` check (`crates/viola-agent-claude/src/ledger.rs:872-878` `session_resumed`) passes only when
Run E's first SessionStart capture parses and carries `source` `resume` and the target id. The fake agent sets both
from its own arguments (`src/bin/viola-fake-agent.rs:175-184` `resume_fields`), and the target is the one Run B's
first SessionStart named, which the `agents-join` row read in the same runs (it passed). So a failing run read a
capture whose payload did not parse: an EMPTY one.

The capture arm files a payload in two steps (`src/cmd/hook.rs:384-397` `file_from`):
1. it claims the name `SessionStart.<k>.json` EMPTY with `create_private_new` (`src/cmd/hook.rs:390`);
2. it then writes the bytes to a temp file, `sync_all`s it and renames it over the claim through `replace_private`
   (`src/cmd/hook.rs:396` → `crates/viola-state/src/fs.rs:289-297`, the fsync at `fs.rs:292`).

Between the two the name exists with no bytes. Run E (`src/cmd/verify/typed.rs:251-276` `resume_run`) waits for the
capture with `wait_capture`, whose test is a NAME count:

    fn captured(dir: &Path, event: HookEvent) -> bool {
        count_of(dir, event) > 0                        // src/cmd/verify/typed.rs:467-468
    }

`count_of` (`typed.rs:316-329`) counts every `<Event>.<k>.json` name, the empty claim included. Run E then settles
(the quiet period after the claim was seen), kills the fake agent with no key, and verify reads the captures
(`src/cmd/verify.rs:253` `read_captures(&resume.1)`). A hook still inside step 2 when the kill lands is hung up with
its terminal (it runs in the fake agent's foreground group) or finishes after the read: either way the read takes the
empty claim, `Capture::read` gives `payload: None`, and `session_resumed` is false.

**The equality that decides the case:** `count_of(dir, HookEvent::SessionStart) > 0` at `src/cmd/verify/typed.rs:468`
reads TRUE on the empty claim `src/cmd/hook.rs:390` made, before the bytes land at `src/cmd/hook.rs:396`. The window
it opens is the hook's write, fsync and rename. The input the simultaneous runs share is not a file: it is the
runner's disk, whose `sync_all` (`crates/viola-state/src/fs.rs:292`) the other verify runs and the coverage build's
profile writes load. No other run is ended on a name count alone: Runs B, C and D read their captures' content only
after later steps, so Run E, the last run, ended by a kill right after its one capture, is the one row exposed.

(The plan's step 3 asked for "which shared input two simultaneous verify runs read". Read as measured, no shared
input decides it; the shared resource is the disk's write latency. Recorded here as a premise correction.)

## 3. The forced concurrent start on this host (step 2)

`tests/cli_verify.rs` `stamped_home_concurrent_verify_runs_each_stamp_the_home`: three `viola verify` runs over three
fresh homes, released by one `std::sync::Barrier`, each the call `stamped_home` makes. Fired through entry 8 (`bash
scripts/agent-run.sh run --integration --filter 'test(/stamped_home_concurrent_/)'`) on the unchanged product tree
(the only edits: this case and the unit case of section 4, both test code), 2026-10-17T02:54:47Z-02:57:10Z:

- 20 firings (`8.log`, `8.2.log` … `8.20.log`), **20 green, 0 red**. The case's own wall (JUnit): 4.269-4.285 s,
  under the root tests' 20 s CI kill and the `mutants` profile's 10 s kill.
- No red, so no failing rows and no Run E capture to record. This host's home base (the `target/e2e-home` link to
  an owner-only tmpfs directory) is memory-backed, where a sync costs next to nothing, and three concurrent runs take the time one takes:
  the concurrent start does not open the window here. A green re-run proves no cause (testing.md 2026-09-27).

## 4. The window forced open at the decision itself

The window is the instant in which a capture name exists with no bytes. testing.md 2026-09-27 asks for a timing window
to be forced, never sampled; the state the window exposes is planted directly where the decision reads it, in
`src/cmd/verify/typed.rs`'s test module:

`captured_reads_no_capture_while_the_claim_is_empty` — an EMPTY `SessionStart.1.json` must not read as a capture
(`captured` false, `count_of` 0); the same name holding bytes must (`captured` true, `count_of` 1).

## 5. The fix, and the witness both ways (steps 3-4)

**The fix** (`src/cmd/verify/typed.rs` `count_of`): a capture counts only once its bytes landed — the name parses as
`<Event>.<k>.json` AND the file is non-empty. The capture arm renames the payload over its claim, so a non-empty file
is the whole payload. Run E's wait (`captured`) now holds until the hook's rename lands, within the same
`PROBE_DEADLINE`; Runs B, C and D's waits take the same rule. No bound was raised, no retry added, no test
serialised: the waiter's condition was the wrong equality, and it is the one changed.

Readings (unit witness via `bash scripts/agent-run.sh run --unit --filter
'test(/captured_reads_no_capture_while_the_claim_is_empty/)'`):

| tree | reading | first diagnostic line |
|---|---|---|
| before the fix (02:57Z) | **red** · `failed 1` | `panicked at src/cmd/verify/typed.rs:1103:9: assertion failed: !captured(tmp.path(), HookEvent::SessionStart)` |
| the fix (02:57:36Z, with all 58 `cmd::verify::` unit cases) | green · `passed 58` | — |
| the fix reverted in place, `meta.len() > 0` → `>= 0` (remove-the-guard, 02:59Z) | **red** · `failed 1` | `panicked at src/cmd/verify/typed.rs:1106:9: assertion failed: !captured(tmp.path(), HookEvent::SessionStart)` |
| the guard restored (re-read at typed.rs:329 before the run, 03:00:16Z) | green · `passed 1` | — |

The concurrent case over the fix, entry 8 firings 21-40 (`8.21.log` … `8.40.log`, 02:57:49Z-02:59:34Z): **20 green,
0 red**, the case 4.274-4.285 s. The binary they drove was rebuilt from the fixed source
(`target/harness/debug/viola` 02:57:36Z, the fix 02:57:30Z).

**Reading against step 4 as written:** "with the fix reverted in place, the same case reads red" holds for the forced
unit case; for the concurrent root case it does NOT — that case reads green on this host with or without the fix (its
40 firings above), so it guards the property only where a runner's disk opens the window. Its red-on-revert reading
is unobtainable on this host by measurement; the runner leg that failed (`test (ubuntu-latest)`) is the acceptance's
last half, read by the operator pass's `ci.py conclusion` of the pushed HEAD.
