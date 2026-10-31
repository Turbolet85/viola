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

## The pre-CI commit
`git add -A` once entry 23 read `clean`, then `fa7be1f3 chore(2026-10-31-sanitised-error-surfaces): operator pre-CI commit, for the run this chunk's verdict reads`; the tree clean after it. The master record stays `pending`, `v1-47` `implemented`.

## Entry 24 — `python -X utf8 {tools_dir}/gate.py run --plan … --operator 24` (no run dir), read 2026-10-31T09:26:06Z
```
operator entry 24 · history tripwire: git
 24 probe       green · exit 0 · 3.33s · 92 B → 24.log · history moved: refs/remotes/origin/HEAD 5273ab8c→fa7be1f3; refs/remotes/origin/build/viola-0.1.0 5273ab8c→fa7be1f3 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
not run — excluded · 24: 1-23,25
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```
The history move is the push this entry exists for. It carried the three records-door commits and the pre-CI commit, as the operator directed.

## Entry 25 — fired as written
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`
```
ci v1.3 · 7961bdce
repo Turbolet85/viola (the push remote `origin`) · polled 29× over 869 s
fa7be1f37746 verdict: green · checks 15/15 · wall 838 s · runs ci#41188681876 completed/success
runs: ci#41188681876 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

### The OS-sensitive cases, read from each `test` job's nextest lines (`gh run view 41188681876 --job {id} --log`), read 2026-10-31T09:41:09Z
| case | ubuntu-latest | macos-latest | windows-2025 |
|---|---|---|---|
| `channel_error_bodies_hold_no_path_or_upstream_text` | PASS 6.981 s | PASS 6.545 s | PASS 6.722 s |
| `cli_error_documents_hold_no_path_or_upstream_text` | PASS 5.563 s | PASS 5.869 s | PASS 6.080 s |
| `error_body_check_reports_no_byte_of_what_it_read` | PASS 0.004 s | PASS 0.010 s | PASS 0.015 s |
| `mcp_tools_error_results_hold_no_path_or_upstream_text` | PASS 0.836 s | PASS 0.586 s | PASS 0.724 s |
| `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` | PASS 0.348 s | PASS 0.321 s | PASS 0.414 s |
| `mcp_tools_a_wrong_shaped_call_answers_no_caller_text` | PASS 0.020 s | PASS 0.090 s | PASS 0.049 s |
| `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` | PASS 0.278 s | PASS 0.217 s | PASS 0.351 s |
| `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap` | PASS 0.033 s | PASS 0.038 s | `cfg(unix)`, not run |
| `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command` | PASS 0.061 s | PASS 0.066 s | `cfg(unix)`, not run |
| `drain_keeps_cap_bytes_of_an_endless_pipe` | PASS 0.088 s | PASS 0.061 s | PASS 0.110 s |
| `read_thresholds_keeps_no_key_past_the_frame_cap` | PASS 0.385 s | PASS 0.327 s | PASS 0.522 s |
| `read_hold_keeps_no_key_past_the_frame_cap` | PASS 0.287 s | PASS 0.208 s | PASS 0.331 s |
| `env_keep_keeps_no_key_past_the_frame_cap` | PASS 0.312 s | PASS 0.244 s | PASS 0.328 s |
| `product_error_writers_are_each_named` | PASS 0.507 s | PASS 0.229 s | PASS 0.513 s |
| `thiserror_messages_interpolate_nothing` | PASS 0.224 s | PASS 0.102 s | PASS 0.238 s |
| `path1_run_start_one_home_in_order` (the WATCH) | PASS 9.067 s | PASS 6.587 s | PASS 6.943 s |

Jobs: test (ubuntu-latest) 124500759324 (`3347 tests run: 3347 passed (5 slow)`) · test (macos-latest) 124500759418 (`3301 … 3301 passed (5 slow)`) · test (windows-2025) 124500759467 (`3300 … 3300 passed (5 slow)`).

The fold's `.cmd` refusal on `windows-2025`, `tests/cli_program_resolution.rs`: `run_refuses_a_batch_script_child` `case_1_cmd`, `case_2_bat` and `case_3_upp…` PASS, `run_refuses_a_batch_script_found_on_path` PASS, `run_resolves_the_claude_npm_shim_to_its_exe` PASS.

### `v1-47`'s scan clause: the `Secret scan` step of each `test` job, `success`, with its one document
- ubuntu-latest: `{"v":1,"cmd":"secret-scan","ok":true,"files":716,"skipped_non_file":2,"mode_check":"unix","hits":[]}`
- macos-latest: `{"v":1,"cmd":"secret-scan","ok":true,"files":689,"skipped_non_file":6,"mode_check":"unix","hits":[]}`
- windows-2025: `{"v":1,"cmd":"secret-scan","ok":true,"files":695,"skipped_non_file":8,"mode_check":"skipped-windows","hits":[]}`

No line of the three job logs names `claude-stripped`.

### The WATCH on Path 1's CI timeout
Green under the ci profile's 20 s line: 9.067 s on `test (ubuntu-latest)`, no `SLOW` line for it. By the tally `scope.md` carries (2/3 with Setup 5a's reading of `ci#40467224735`) this run makes 3/3, the WATCH's retire condition; retiring it is the wrap's. No cause is claimed for the faster time. The causal reading stays PROVISIONAL as written.
