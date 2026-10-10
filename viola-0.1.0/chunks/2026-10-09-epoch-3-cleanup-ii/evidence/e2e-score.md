# The `viola-e2e` whole-unit score on the dev host — 2026-10-09-epoch-3-cleanup-ii

A one-shot measurement (plan steps 2 and 13), the boundary tier's form run once over the whole unit. It is not a
gate entry. Its records beside this file: `e2e-score.json` (the harness's one document, whole) and
`e2e-score-outcomes.txt` (cargo-mutants' outcome lines alone, 718 lines).

## The run

- **Command:** `TMPDIR="$(dirname "$PWD")/viola-mutants-scratch" bash scripts/agent-run.sh run --mutants --package
  viola-e2e`, from the repository root, stdout and stderr each to a file, never through a pipe.
- **Tree:** HEAD `14f1fb5`, as taken up: no chunk edit in the tree when the tool copied it (the copy stood in the
  scratch at 22:05Z; the first source edit of this chunk was made after the unmutated baseline had started in
  that copy).
- **Host:** `x86_64-unknown-linux-gnu`, the Linux dev host, cargo-mutants 27.1.0, jobs 1.
- **Start:** 2026-10-09T22:03:20Z. **End:** 2026-10-09T23:32:36Z. **Wall:** 5356 s (89 min 16 s).
- cargo-mutants' own lines: `Found 718 mutants to test` · `ok Unmutated baseline in 12s build + 35s test` ·
  `Auto-set test timeout to 178s` · `718 mutants tested in 89m: 2 missed, 656 caught, 60 unviable`.
- **Harness exit:** 1. Its document reads `"ok":false`.

## The four counts (the document's `mutants` suite, and the outcome lines counted)

| caught | missed | timeout | unviable | tested |
|---|---|---|---|---|
| 656 | 2 | 0 | 60 | 718 |

The document: suite `mutants`, `passed` 656, `failed` 2, `survived` 2; `mutants.tested` 718, `mutants.verdict`
`package`, `mutants.package` `viola-e2e`. The outcome lines counted by their first word read the same four numbers.

## The verdict rule's reading

- `missed == 0`: **not met**, 2 missed.
- `timeout == 0`: met.
- `unviable <= caught`: met, 60 against 656.

So the score reads red by the rule. Both missed mutants are ones this host cannot reach (below); no reachable
mutant of the unit was read missed or timed out on this host.

## Every missed or timed-out mutant, by name

Both are in `crates/viola-e2e/src/harness/run/mutants/scratch.rs`, function `prepare`:

1. `crates/viola-e2e/src/harness/run/mutants/scratch.rs:48:5: replace prepare -> Result<Option<(PathBuf, u64)>,
   String> with Ok(None)` — **not measured here; owed to Windows mutation grade.**
2. `crates/viola-e2e/src/harness/run/mutants/scratch.rs:54:8: delete ! in prepare` — **not measured here; owed to
   Windows mutation grade.**

Basis, read at the source: `prepare` opens with `if !HOST_SCRATCH { return Ok(None); }`, and `HOST_SCRATCH` is the
const `cfg!(windows)`. On this host that const is false, so the function's own value is `Ok(None)` (the first
mutant returns what the code returns) and line 54 stands after the return (the second mutant is never executed).
A Windows host runs the rest of the body. Neither is called equivalent, and neither is counted caught.

No mutant timed out.

This chunk kills neither of them (the plan's lean: the entry asks for the score).

## What the run left

- 17 `.tmp*` directories in the scratch, and nothing else there: the tool removed its copy of the tree
  (`cargo-mutants-viola-*.tmp`). The scratch read empty before the run.
- The harness archived the tool's `outcomes.json` under `target/run-archive/862` (ignored by git). It is not copied
  here: it holds absolute argv paths.
- 26 `viola-session-*` directories under the test-home base (`target/e2e-home/`, the tmpfs behind the link), dated
  inside the run's window (22:05Z to 22:59Z), read after the operator pass. The tool's copy of the tree carries the
  same link, so a harness test running under a mutated `boot` or `cleanup` leaves its session home on the shared
  base. No process of this repository was alive when they were read (no supervisor, wrapper or child). They were
  not removed: they are gone at a reboot, and their deletion before that is the operator's.

## What else ran in its window (from the run journal)

- This chunk's own work on the working tree, none of it in the tool's copy: source edits; `cargo check` three
  times and `cargo fmt`; one `cargo clippy` over the workspace; three filtered `cargo nextest` runs of the new
  cases (27, 12 and 164 tests, build jobs limited to 8); three `cargo mutants --list` calls; the instrument's
  `selftest` once and its two graded verbs twice (`jscpd`, `rust-code-analysis-cli`, source text only).
- No full suite, no `pre-push` and no second mutation run.
- Other builders share the host. The host record over the window (`hostwatch.py read --from 2026-10-09T22:03:20Z
  --to 2026-10-09T23:32:36Z --for viola`): verdict `QUIET for viola`, 0 s stalled on IO; io some peak 5 %, mean
  0 %; load peak 18.7, mean 9.7 on 32 cores; another tree's build files were running through the whole window.
- The test homes' backing read `tmpfs` behind the `target/e2e-home` link, before and after.

## Not measured

- The unit's Windows grade: owed to "Windows mutation grade".
- Whether the 60 unviable mutants include ones a Windows host compiles: not read.
- One earlier whole-unit wall is on record, 78 min for 711 mutants (2026-10-04). This run's 89 min was taken with
  the work above beside it; the two walls are not compared.
