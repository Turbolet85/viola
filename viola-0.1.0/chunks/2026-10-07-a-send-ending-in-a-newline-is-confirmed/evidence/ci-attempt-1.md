# ci#37627485806 attempt 1 — the folded red, as the run records it

Read at 2026-10-07T14:31Z to 14:34Z from the run itself, while its artifacts stand (they expire 2026-10-14).
Sha `9f2bebe5102b8df1053b4b5c26225d1f6352f0e4`, the last wrap's commit. Attempt 2 of the same run, on the same
sha, is green 15/15; this file is about attempt 1 only.

**The limit of this closure, in one sentence:** the mechanism is measured at HEAD, and pid 10799's identity is
not provable from the run.

## How it was read
Each record was fetched once into a scratch directory outside the repository and read there:

| record | command |
|---|---|
| the attempt | `gh api "repos/{owner}/{repo}/actions/runs/37627485806/attempts/1"` |
| its jobs and steps | `gh api "repos/{owner}/{repo}/actions/runs/37627485806/attempts/1/jobs?per_page=100"` |
| the artifact list | `gh api "repos/{owner}/{repo}/actions/runs/37627485806/artifacts?per_page=100"` |
| the failed job's log | `gh api --allow-escape-sequences "repos/{owner}/{repo}/actions/jobs/112812904580/logs"` |
| the three artifacts | `gh api "repos/{owner}/{repo}/actions/artifacts/<id>/zip"` for ids 11485600497, 11484568161, 11484488518 |

`evidence/guards/read-attempt1.py <dir>` reads that directory and prints every figure below. Two figures were
read a second way: the failed steps with `gh run view 37627485806 --attempt 1 --json jobs`, and the harness
artifact's members with `unzip -l`.

## The attempt
- `run_attempt` 1, `completed`, `failure`. Fifteen jobs: fourteen `success`, one `failure`,
  `test (ubuntu-latest)` (job 112812904580).

## The failed log: the two `llvm-profdata` lines and the harness's two verdict lines
The log prints the profile's path under the runner's checkout; it is given here from `target/` on.

- `2026-10-07T13:22:24.0998920Z Summary [ 144.959s] 1695 tests run: 1695 passed (5 slow), 0 skipped`
- `2026-10-07T13:22:25.3491326Z warning: target/llvm-cov-target/viola-10799-3177174534174374433_3.profraw:
  invalid instrumentation profile data (file header is corrupt)`
- `2026-10-07T13:22:25.3492410Z error: no profile can be merged`
- the same two lines once more at 13:22:26.659Z (what printed them twice was not read);
- the harness's verdict, 13:22:28.907Z: `{"v":1,"cmd":"run","ok":false,"suites":[{"suite":"coverage",
  "passed":1695,"failed":2,…,"failures":["llvm-cov-exit-1","llvm-cov-summary-missing"]},{"suite":"doctest",…}],
  "archived":"target/run-archive/30"}`;
- the gate's verdict, 13:23:08.099Z: `{"v":1,"cmd":"gate","ok":false,"breaches":[{"gate":"suite-failed",
  "suite":"coverage","detail":"failed 2"},{"gate":"artifact-missing","suite":"coverage",
  "detail":"llvm-cov-summary.json"},{"gate":"suite-missing","suite":"playwright","detail":"absent"},
  {"gate":"artifact-missing","suite":"playwright","detail":"junit-playwright.xml"}]}`.

Every test passed. The red is the merge of the raw profiles, and what follows from it.

## The per-step conclusions of `test (ubuntu-latest)`
| steps | conclusion |
|---|---|
| 1 to 6 (set up, checkout, toolchain, llvm-tools, rust-cache, install-action) | success |
| 7 ConPTY vendor verification, 8 Coverage and doctest (pwsh shim) | skipped (not this OS) |
| **9 Coverage and doctest (sh shim)** | **failure** |
| 10 to 18 (harness lifecycle, Node, browser dependencies, Chromium, Browser suite, Gate tools present) | skipped |
| 19 G2 zero panics, 20 G4 schema conformance, 21 Harness capture, 22 Secret scan | success |
| 23 Upload diagnostics, 24 Upload harness capture, 25 Upload JUnit | success |
| 26 Upload secret-scan hit report | skipped |
| **27 Gate verdict** | **failure** |

The browser suite never ran (steps 12 to 17 skipped after step 9), which is why the gate also lists
`suite-missing` and `artifact-missing` for `playwright`.

## `harness-ubuntu-latest` (id 11485600497, 31 348 310 B): ten members, no profile
`artifacts/run-summary.json` (534 B), `artifacts/junit-coverage.xml` (309 582 B), `status.json` (61 B),
`logs.ndjson` (59 B) and six fake-agent copies of 18 847 320 B each: `cli-drive-14794/bin/claude`,
`cli-keep-14755/bin/claude`, `harness_lifecycle-dead-15136/bin/claude`, `harness_lifecycle-main-15008/bin/claude`,
`harness_lifecycle-unstamped-14881/bin/claude`, `harness_lifecycle-orphan-14970/bin/claude`.

`.profraw` among the members: 0. The upload takes `target/agent-run/`; the refused profile sat in
`target/llvm-cov-target/`. The file itself is therefore not on the run, and the session-learnings method, which
reads the refused profile's own counters, cannot name its writer on this attempt.

## `diag-ubuntu-latest` (id 11484568161, 139 780 B): the two rows that bracket pid 10799
217 role files of 127 test homes, 2 040 lines. 222 lines carry a `pid`; every one is a `process-start` of
process `run` or `cli`. No line carries pid 10799.

| | pid | timestamp | event | process |
|---|---|---|---|---|
| nearest below | 10401 | 2026-10-07T13:21:39.511Z | `process-start` | `cli` |
| nearest above | 11040 | 2026-10-07T13:21:41.038Z | `process-start` | `run` |

Eight lines of any kind fall after the lower row and before the upper one; the last of them is at
13:21:39.514Z. From 13:21:39.514Z to 13:21:41.038Z no test home wrote a line.

## `junit-ubuntu-latest` (id 11484488518, 37 181 B): the named test's window
`nextest/ci/junit.xml`, suite `viola::bin/viola`:
`cmd::run::tests::start_opens_the_scenario_one_spans_under_run_start` started 2026-10-07T13:21:39.913Z and took
0.793 s, so it ended 13:21:40.706Z: inside the span above. Its home is a temp dir outside `target/e2e-home`
(`src/cmd/run.rs`, the test's own comment), so it leaves no line in the diagnostics artifact.

279 cases overlap the window between the two pid rows: 277 of suite `viola::bin/viola` and 2 of
`viola::tui_wheel`.

## What this does and does not establish
- Established on the run: a profile written by pid 10799 was refused by `llvm-profdata`; pid 10799 was created
  in a span of about 1.5 s in which no test home logged anything; the named test ran inside that span.
- Established at HEAD, not on the run (`profraw-red-green.md`): that test killed an instrumented child which
  exited by itself, and a kill that landed in that child's exit-time profile write left a profile of the same
  refused kind.
- Not established: that pid 10799 was that child. The file is gone, and 278 other cases overlap the span. Two
  earlier runs with the same message (ci#36529038462, ci#36481260151) are not claimed closed by this chunk.
