# Operator pass — 2026-10-27-bounded-inputs-at-every-boundary

Run by the session on the operator's word (the /andromeda-implement invocation of 2026-10-27: "please run
implement per the skill on 2026-10-27-bounded-inputs-at-every-boundary, through its operator pass and the CI
read"). Base: HEAD `4b0bf776d33f` before the pre-CI commit.

## Entry 24 re-run — `bash scripts/agent-run.sh pre-push` (2026-10-27T06:01Z–06:03Z, the uncommitted tree)
Exit 0. Its last two documents:
`{"v":1,"cmd":"gate","ok":true,"breaches":[]}`
`{"v":1,"cmd":"pre-push","ok":true,"stage":"linux-tests","linux":{"run":{"ok":true,"suites":[{"suite":"coverage","passed":3251,"failed":0,"skipped":0,"failures":[]},{"suite":"doctest","passed":0,"failed":0,"skipped":0,"failures":[]}]},"gate":{"ok":true,"breaches":[]}}}`

## Entry 25 — fired as written (2026-10-27T06:03Z)
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`
```
gate v1.22 · 6739ca7e
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 529 (runs 523 · evidence 1 · inputs 5) · trails 16 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
This record is written after that read, as the plan template says.

## The pre-CI commit (2026-10-27T06:04Z)
`git add -A` once entry 25 read `clean`, then
`607792cb chore(2026-10-27-bounded-inputs-at-every-boundary): operator pre-CI commit, for the run this chunk's verdict reads`;
the tree clean after it. The master record stays `pending`, v1-46 `implemented`.

## Entry 26 — `python -X utf8 {tools_dir}/gate.py run --plan … --operator 26` (no run dir)
```
operator entry 26 · history tripwire: git
 26 probe       green · exit 0 · 3.36s · 92 B → 26.log · history moved: refs/remotes/origin/HEAD 4b0bf776→607792cb; refs/remotes/origin/build/viola-0.1.0 4b0bf776→607792cb · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
not run — excluded · 26: 1-25,27
entries 27 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 26
```
The history move is the push this entry exists for: the remote branch moved to the pre-CI commit, as intended.

## Entry 27 — fired as written (2026-10-27T06:04Z → 06:16Z)
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`
```
ci v1.3 · 9058737c
repo Turbolet85/viola (the push remote `origin`) · polled 24× over 714 s
607792cb8178 verdict: green · checks 15/15 · wall 719 s · runs ci#40458923095 completed/success
runs: ci#40458923095 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

### The OS-sensitive cases, read from each `test` job's nextest lines (`gh run view 40458923095 --job {id} --log`)
| case | ubuntu-latest | macos-latest | windows-2025 |
|---|---|---|---|
| `chaos_feed_panic_refuses_send_and_keeps_passthrough` (bytes on Unix, literal absence everywhere) | PASS 4.015 s | PASS 0.289 s | PASS 0.512 s |
| `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` | PASS 0.233 s | PASS 0.364 s | PASS 0.504 s |
| `tests::serve_ends_the_session_at_a_line_past_max_frame` (viola-mcp) | PASS 0.177 s | PASS 0.233 s | PASS 0.339 s |
| `product_readers_are_each_bounded_or_named` | PASS 0.452 s | PASS 0.326 s | PASS 0.994 s |
| `path1_run_start_one_home_in_order` (the WATCH) | `SLOW [> 10.000s]`, then PASS 15.010 s | PASS 6.927 s | PASS 6.418 s |

Jobs: test (ubuntu-latest) 122038651827 (3251 tests) · test (macos-latest) 122038651789 (3205) ·
test (windows-2025) 122038652098 (3206). The `lint (windows-2025)` job compiled step 7's `cfg(unix)` gate (green).

### The WATCH on Path 1's CI timeout
This run is green under the ci profile's 20 s line: 15.010 s on `test (ubuntu-latest)`. The tally moves 0/3 → 1/3.
This reading is recorded without a cause: it is slower than the earlier greens (13.468 s, 8.219 s) and below the
red's 20.089 s, and no case of this chunk touches Path 1's start sequence.
