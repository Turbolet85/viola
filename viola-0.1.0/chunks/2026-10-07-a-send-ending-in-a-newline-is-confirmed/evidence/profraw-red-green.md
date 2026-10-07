# The profile census — red on the untouched test, green after the fix (steps 6 to 8)

Times are `date -u`, 2026-10-07, on the Linux dev host (32 cores). The census is `scripts/profraw-census.sh
<runs> <workers>`: it runs `cmd::run::tests::start_opens_the_scenario_one_spans_under_run_start` on the
instrumented root-bin test binary, each run with its own profile prefix, and counts and sizes what each run
left. On the untouched test a whole profile of this binary is 128 264 B and a clean run leaves two: the test
process and the version probe's child. On the fixed test a whole profile is 129 512 B and a clean run leaves
one, the test process's own (the sections from "The must-pass control after the fix" on).

Before every census run below, one line was written to the operator saying that it starts and for how long
(inputs#I3, inputs#I4, inputs#I9).

## Why the old child could leave a corrupt profile
Read at implement, and it corrects the plan's picture of the child. The test started its wrapper with the test
binary itself and the word `--list` as the program. The wrapper puts its plugin flag first in a child's
arguments (`src/run/mod.rs`, `child_launch`), so the child ran as `<test binary> --plugin-dir <dir> --list`.
libtest refuses that flag: measured on the unit-test binary, `<binary> --plugin-dir /nonexistent --list` prints
`error: Unrecognized option: 'plugin-dir'` on stderr, 0 bytes on stdout, and exits 101.

So the old child never listed anything. It exited 101 by itself, at once, and under coverage an instrumented
process writes its profile as it exits. The test called `kill` straight after the start, which is why a kill
could land in that exit-time profile write. When the kill came first the child left no profile; when the child
finished first it left a whole third profile; when the kill landed in the write it left a short one, which
`llvm-profdata` refuses.

The same reading makes step 7's planned design unbuildable: a child-entry test in the test binary can never be
entered through `start`, because libtest exits before any test body runs.

## Research's three scratch batches (before this chunk's script, on the binary of `9f2bebe`)
1 720 loaded runs in three batches by a scratch loop (research.md): 5 runs with a third profile, 1 profile
short (69 632 B against 128 264 B).

## Step 6 — the census on the untouched test: RED, as the must-fail control requires
- Script check first, 14:20:54Z: `bash scripts/profraw-census.sh 4 2` → exit 0,
  `profraw-census: runs 4 · passed 4 · profiles 8 · third 0 · short 0`; each run left two profiles of
  128 264 B. (An earlier 4-run check at 14:20:34Z exited 2, `no instrumented binary`: the script read the
  binary's test list through a pipe to a reader that stops at the first match, which broke the lister's pipe.
  The listing is now read whole from a file.)
- 14:21:01Z to 14:23:18Z, `bash scripts/profraw-census.sh 4800 48` on
  `target/llvm-cov-target/debug/deps/viola-f49f369ad1a28f06`, the instrumented binary the last pre-push built
  from `9f2bebe` (the test untouched): **exit 1**, wall 136.6 s, load average 1.93 at the start and 55.84 at
  the end.
  - first line: `profraw-census: runs 4800 · workers 48`
  - last line: `profraw-census: runs 4800 · passed 4800 · profiles 9603 · third 3 · short 3`
  - the tally: 4 797 runs left two profiles of 128 264 B each; 3 runs left a third, of 0 B, 73 728 B and
    124 288 B. All three third profiles are short. Every run's test passed.

The predicted reading was `third` above 0. Measured: 3 in 4 800, and each of the three is a truncated file.

## The fix (step 7, as decided on the step 7 card — inputs#I5)
The test's child is now `whoami`, a host program, not the test binary. It refuses the wrapper's plugin flag and
exits by itself too (measured here: `whoami --plugin-dir /x` prints `unrecognized option`, exit 1), but it is
not instrumented, so it writes no profile whenever the kill lands. This departs from step 7's wording (a child
that cannot exit by itself); the deviation is in `scope-record.md` with the overseer's word. `grep -c
'"--list"' src/cmd/run.rs` reads 0.

Unmeasured before CI: the Windows and macOS halves. `whoami` is resolved through the wrapper's own program
lookup on each OS; on Windows it is `whoami.exe`. A red there is read and fixed by cause in its own commit
(inputs#I5).

## The must-pass control after the fix: a FINDING, recorded as read, not re-run
`pre-push` (gate entry 15, 14:34Z, green: coverage 1712/1712, playwright 1/1, no breach) rebuilt the
instrumented binary at 14:34:23Z with the changed test.

- 14:35:36Z to 14:35:47Z, `bash scripts/profraw-census.sh 48 1`: **exit 1**, wall 10.3 s, load average 5.52 at
  the start.
  - first line: `profraw-census: runs 48 · workers 1`
  - last line: `profraw-census: runs 48 · passed 48 · profiles 48 · third 0 · short 0`
  - the tally: all 48 runs left exactly one profile, of 129 512 B (the rebuilt binary's whole profile).

The plan predicted `profiles 96`, two a run. The red is not of the `third` or `short` shape: every run passed,
no run left an extra profile and no profile is short. The count is one a run because the second profile of
the old picture was the version probe's child, and the probe runs the same program as the wrapper's child
(`<program> --version`). With `whoami` as the program neither child is instrumented, so only the test process
writes a profile.

Consequences, as read:
- the script's rule (two whole profiles a run, a third when a run left more than two) and gate entry 16's atom
  (`profiles 9600`) describe the test binary as the program. No design that can be built meets them: two
  profiles a run needs an instrumented probe child, which is the test binary, whose wrapper child then exits
  101 by itself and writes the third;
- what I told the operator on the step 7 card, that the census gate would read as planned with the host
  child, was wrong. It was corrected on a second card (inputs#I6).

Not run: gate entry 16 (`bash scripts/profraw-census.sh 4800 48` on the fixed test) and step 8's reading. The
script still holds the two-a-run rule. On the overseer's word (inputs#I6) implement stopped here with the tree
preserved and nothing pushed: the plan's census numbers are revised through `/andromeda-phase` first (one whole
profile a run; steps 6 to 8 and entry 16 reworded to what was measured), and implement then finishes the census
and the operator pass against the revised entry.

## The plan's revision, and its baseline read of the census entry
The plan was revised through `/andromeda-phase` (inputs#I7, reviewed with inputs#I8): one whole profile a run,
the red side above standing as recorded, and a planted control for the revised rule's failing side. The
revision's own baseline of the census entry, on the tree as the stopped run left it and with the script still
under the two-a-run rule, 14:45:59Z to 14:48:13Z: `bash scripts/profraw-census.sh 4800 48` → exit 1, wall
134.0 s, `runs 4800 · passed 4800 · profiles 4800 · third 0 · short 0`; all 4 800 runs left one profile of
129 512 B. The line already read as the revised entry asks; the exit was 1 only by the old rule. Its record is
`p5-baseline-census.out` in the revision's run dir.

## Step 6, the remaining part — the script under the one-a-run rule (the re-entry, inputs#I9)
`scripts/profraw-census.sh`, 96 lines:
- a run counts under `third` when it left more than one profile; exit 0 only when every run passed, the
  profiles equal the runs, and `third` and `short` are 0; any other count is exit 1;
- the header comment says a clean run leaves one profile, the test process's own, because neither the wrapper's
  child nor the version probe's child is instrumented;
- an optional third word names the census directory, `target/profraw-census/<name>/`, which may already exist;
  without it the directory is the stamped one as before.

Usage refusals, read at 15:12Z with no census run (each prints the usage line and exits 2): one word; four
words; `0 1`; a name `a/b`; a name with a space; an empty name; the names `.` and `..`. The last two are not in
step 6's letter: its character class admits them, and they would aim the census directory at
`target/profraw-census/` itself or at `target/`. The script refuses them beside the class.

## Step 8 — the census after the fix, on the instrumented binary of the final tree
`pre-push` (gate entry 15, 15:14Z, green: coverage 1712/1712, playwright 1/1, no breach) left the instrumented
binary `target/llvm-cov-target/debug/deps/viola-f49f369ad1a28f06`, written 15:14:21Z. Entries 1 to 15 read
green in that one run of the gate tool (the unit filter 16 passed, the integration filter 2 passed).

### The must-pass control: GREEN
- 15:15:24Z to 15:15:34Z, `bash scripts/profraw-census.sh 48 1`: **exit 0**, wall 10.3 s, load average 4.35 at
  the start and 3.98 at the end.
  - first line: `profraw-census: runs 48 · workers 1`
  - last line: `profraw-census: runs 48 · passed 48 · profiles 48 · third 0 · short 0`
  - the tally: all 48 runs left exactly one profile, of 129 512 B.

### The planted control: the revised rule's own failing side, RED twice as required (inputs#I8)
Two one-run censuses with no rebuild, each in a census directory under a name not used before, with one file
of zeros planted at `<name>/w0/r0-plant.profraw` before the script ran (`truncate -s`). Both started at
15:15:42Z; the names end `T1516Z` and are names only.
- The extra plant, 129 512 B (a whole profile's size, from the must-pass tally), name
  `plant-extra-20261007T1516Z`: `bash scripts/profraw-census.sh 1 1 plant-extra-20261007T1516Z` → **exit 1**,
  wall 0.2 s.
  - last line: `profraw-census: runs 1 · passed 1 · profiles 2 · third 1 · short 0`
  - the tally line: `1 2 129512 129512`
- The short plant, 1 000 B, name `plant-short-20261007T1516Z`: `bash scripts/profraw-census.sh 1 1
  plant-short-20261007T1516Z` → **exit 1**, wall 0.2 s.
  - last line: `profraw-census: runs 1 · passed 1 · profiles 2 · third 1 · short 1`
  - the tally line: `1 2 1000 129512`
- The removal, checked after each run: `find target/profraw-census/<name> -name '*.profraw' | wc -l` reads 0
  under both directories, and each holds one file, `w0/tally`. Before each run the same count read 1 (the
  plant). The two directories stay under `target/` with the other census directories.

Both readings are the predicted ones. The size rule's own reading is `short 1` against the extra plant's
`short 0`; `third` reads 1 in both because a short plant is also one file too many.

### The census entry (gate entry 16): GREEN
- 15:15:50Z to 15:18:03Z, `bash scripts/profraw-census.sh 4800 48`, fired through the gate tool (`--entry
  16`): **green, exit 0**, 132.76 s by the tool's own clock, load average 3.10 at the start and 49.10 at the
  end.
  - first line: `profraw-census: runs 4800 · workers 48`
  - last line: `profraw-census: runs 4800 · passed 4800 · profiles 4800 · third 0 · short 0`
  - the tally (`sort | uniq -c` over the per-worker tallies of `20261007T151551Z-…`): 4 800 lines `1 1
    129512`. Every run passed and left one whole profile; no `.profraw` and no run log is left in the directory.

The predicted reading was that line. Measured: that line, with exit 0.

## What the readings establish
- On the untouched test, 4 800 loaded runs: 3 truncated third profiles (step 6). The mechanism is measured.
- On the fixed test, 48 runs one at a time and 4 800 runs at 48 workers, the second read twice (the revision's
  baseline and gate entry 16): no child profile at all, one whole profile of 129 512 B a run.
- The revised rule fails when it should: one extra whole file reads `third 1`, one short file reads `short 1`,
  each exit 1.
- Not measured: the fixed test on Windows and macOS, before CI.
- The limit of the closure stands: the mechanism is measured at HEAD, and pid 10799's identity is not provable
  from the run (`ci-attempt-1.md`).
