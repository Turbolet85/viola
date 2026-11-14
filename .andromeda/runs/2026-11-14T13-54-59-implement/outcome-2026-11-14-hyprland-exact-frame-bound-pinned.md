# Implement outcome — 2026-11-14-hyprland-exact-frame-bound-pinned

**Outcome: green** (smoke skipped: no boot-path / UI-surface change; the plan lists no `smoke` or `self-verify` entry)

## Files changed
- `crates/viola-hyprland/src/read.rs` — test `read_takes_a_reply_of_max_frame_bytes_and_refuses_one_more_as_oversize` (plan step 1); no product line
- `crates/viola-hyprland/src/reply.rs` — test `parse_dispatch_takes_max_frame_bytes_as_refused_and_refuses_one_more_as_oversize` (plan step 2); no product line

## Gates (trail `.andromeda/runs/2026-11-14T13-54-59-implement/gate-2026-11-14-hyprland-exact-frame-bound-pinned.json`)
`entries 15 · green 12 · red 0 · recorded 0 · timeout 0 · not-run 3` — one iteration. Entries 13–15 are `leg = 'operator'`
(hygiene, push, CI read), left for the operator pass. No deferral: every entry ran (`delta docs 221 · rust 2`).
- Entry 5 (the two tests by name): `2 tests run: 2 passed, 82 skipped`. Measured: the read test 0.185 s (forecast about 0.1 s),
  the dispatch test 0.005 s, both far below the `mutants` profile's 10 s kill.
- Entry 12 (`agent-run.sh pre-push`) green in 104.51 s.

## Controls (plan step 4; `gate.py control`, `--timeout 600`)
Controls file `.andromeda/cache/chunks/2026-11-14-hyprland-exact-frame-bound-pinned/evidence/controls.toml`, lines in
`…/evidence/control-1.txt`:
```
entry 4 in place green · 6.0s → 4.2.log
control read-77 · entry 4 · holds — in place green, removed red · exit 0 ✗ (exit 100) · 6.0s · 0.7s → 4.without-read-77.log
control reply-151 · entry 4 · holds — in place green, removed red · exit 0 ✗ (exit 100) · 6.0s · 0.7s → 4.without-reply-151.log
control: 2 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed
```

## Matrix
No capability carries `chunk == 2026-11-14-hyprland-exact-frame-bound-pinned` (0 rows): no ref written, as the plan states.

## Deviations from plan
- none. Scope: `scope: clean — changed 2 · listed 2 · recorded 0 … · excluded 222`; no scope-record line.
- The controls file sits at the path plan step 4 names (`{work_dir}/evidence/controls.toml`), not at `{run_dir}/control-{k}.toml`.

## Guard entries
- none in the plan.

## Surfaced / stuck
- none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started outside the gate tool's own trees | — | `census: none running` (cargo, cargo-nextest, hyprctl, viola-fake-agent, projects/viola/target/debug/viola) |

## Decisions & corrections
- The operator's invocation word: no live `claude` start and no live desktop step. The controls file carries a `case`
  per control, the test NAME alone. `gate.py run --only` takes `N,N` alone. Run the operator pass with the `ci.py`
  conclusion read (leg=operator). Followed: no live step ran, each `case` is the bare test name, and no `--only` was needed.

## Next
The OPERATOR PASS: entry 13 (`gate.py hygiene`, as written), the pre-CI commit, entry 14 (the push, `--operator 14`) and entry 15
(`ci.py conclusion --sha HEAD --wait 1200`, as written). Then `/andromeda-wrap-session`.
