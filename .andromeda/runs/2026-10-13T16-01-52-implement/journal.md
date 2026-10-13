# Implement journal — 2026-10-13-epoch-4-survivors

One line per long run, written before the wait.

- 16:02Z · step 2a · `agent-run.sh run --unit --filter` over viola-pty's three resize tests, the `resize → Ok(())`
  mutant hand-applied at `crates/viola-pty/src/lib.rs:200`; output to the scratchpad `step2a-mutant.json`.
- 2026-10-13 16:09Z · steps 2b–11 · the hand-applied mutant script over the 28 remaining specs (m31 read first, alone, at
  16:09Z), backgrounded: per mutant apply + rustfmt, red run, byte-exact restore, green run; results to the
  scratchpad `mutant-results.ndjson`.
- 2026-10-13 16:14Z · the 19 specs whose filter was `test(=tests::name)` re-run with `test(/(^|::)tests::name$/)`:
  the `=` form names only a crate-root module's tests, so for `cmd::hook::tests::…` and the like it selected nothing
  and both readings were `nextest-exit-4`, not the case (m00 m07 m13 m20–m23 m33–m44). Backgrounded.
- 2026-10-13 16:18Z · P2 · the whole gate block, `gate.py run --plan … --run-dir … --marker …`, backgrounded (19
  entries, three operator legs not fired; `pre-push` bounded at 7200 s).
