# Operator pass — 2026-10-31-sanitised-error-surfaces

Run by the session on the operator's word (the /andromeda-implement invocation of 2026-10-31: "Run the operator pass with the ci.py conclusion read (leg=operator) as usual; its first push carries the three records-door commits."). Base: HEAD `7bf5c32add0e` before the pre-CI commit. `origin/build/viola-0.1.0` read `5273ab8c68e8`, so the first push also carries `8ebed5bc`, `2cedb584` and `7bf5c32a`.

## Entry 22 re-run — `bash scripts/agent-run.sh pre-push` (the uncommitted tree)

### First re-run: red, 2026-10-31T09:19:30Z to about 09:21:20Z
`3347 tests run: 3345 passed (22 slow), 2 failed`. The failures are `viola::cli_verify verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` `case_1_sigint` (7.535 s) and `case_2_sigterm` (7.188 s), each `timed out waiting for Run E's start` at the 7 s `WITHIN` of `tests/support/fake.rs`'s `wait_for`, its watch report showing the receipt still growing.

Host record for that window: `CONTENDED for viola - 20 s stalled on IO: 09:20:14Z..09:20:34Z 20 s (rule: >= 8 s with io some >= 50%); build tools alive in them: escher 20 s, viola 20 s`, load peak 90.7, 22 slow tests against 5 in the green firing before it.

Reading: the red is the host's contention, not this chunk's change. What holds it: the same tree read green on this case in the gate block's second firing and in the re-run below; five rounds alone read green at about 4.3 s, four of them under `QUIET` and one under `CONTENDED`; and this chunk's only edit on the case's path is the `AgentError` rename in `src/cmd/verify.rs`, ahead of any run. What is not measured: the nextest log carries no wall clock per test, so the two cases' position inside the 20 s window is inferred from their place in the run (391 and 400 of 3347), not read. The case's 4.3 s against its 7 s wait leaves a thin margin on a contended host; this pass changes nothing there, and no bound moves.

### Five rounds of the two cases alone, nextest `ci` profile
| round | summary | exit | window | host verdict |
|---|---|---|---|---|
| 1 | `[4.356s] 2 tests run: 2 passed, 35 skipped` | 0 | 09:21:54Z..09:23:16Z | CONTENDED, 65 s stalled on IO (the round's own build beside `escher`) |
| 2 | `[4.322s] 2 tests run: 2 passed, 35 skipped` | 0 | 09:23:16Z..09:23:21Z | QUIET |
| 3 | `[4.313s] 2 tests run: 2 passed, 35 skipped` | 0 | 09:23:21Z..09:23:25Z | QUIET |
| 4 | `[4.304s] 2 tests run: 2 passed, 35 skipped` | 0 | 09:23:25Z..09:23:30Z | QUIET |
| 5 | `[4.312s] 2 tests run: 2 passed, 35 skipped` | 0 | 09:23:30Z..09:23:34Z | QUIET |

### Second re-run: green, 2026-10-31T09:23:34Z to 09:25:12Z
Exit 0. `{"v":1,"cmd":"pre-push","ok":true,"stage":"linux-tests","linux":{"run":{"ok":true,"suites":[{"suite":"coverage","passed":3347,"failed":0,"skipped":0,"failures":[]},{"suite":"doctest","passed":0,"failed":0,"skipped":0,"failures":[]}]},"gate":{"ok":true,"breaches":[]}}}`

Host record: `QUIET for viola - 5 s stalled on IO`.

## Entry 23 — fired as written
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`
```
gate v1.22 · 20a62737
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 573 (runs 567 · evidence 2 · inputs 4) · trails 14 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
This record is written after that read, as the plan template says.
