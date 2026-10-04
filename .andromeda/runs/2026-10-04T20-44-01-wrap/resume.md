# Wrap resume point — 2026-10-04-the-wheel

**Stopped at:** P7.1, the light gate, RED on entry 16. Halted on the overseer's direction (founder-delegated, through
the operator's AskUserQuestion, 2026-10-04): "halt per P7.1, keep the P2-P6 work in the tree, and record tests-failing
with entry 16."
**Next:** P7.1, re-run in THIS run dir, after the operator pass's fix. The gate is the full block
(`gate.py run --plan viola-0.1.0/chunks/2026-10-04-the-wheel/plan.md --run-dir .andromeda/runs/2026-10-04T20-44-01-wrap
--marker 2026-10-04-the-wheel`; no `--only`). Then P7.2 → P7.3 → P7.3c → P7.4 → P7.5 → P7.6 and the gates evolve checkpoint.
(A first gates checkpoint already fired at this halt; the resumed exit fires its own.)

## The red
- Entry 16: `! (git diff eb53a582c8dc -- '*.rs' .config/nextest.toml | grep -E '^\+.*(#\[ignore|retries *=|test\.skip|std::env::var)' | grep -v 'VIOLA_NAME')`
  → exit 1. It hit `+        if let Ok(mode @ ("reads" | "reads-win32")) = std::env::var(CHILD_MODE).as_deref() {` in
  `crates/viola-pty/src/lib.rs` (the `#[cfg(test)] mod tests` self-exec child; `CHILD_MODE` = `PTY_SEAM_TEST_MODE`,
  read twice already at the base `eb53a58`), added by the operator pass's `849588b`.
- Remedy (overseer): move the new test-child modes off the env read into argv, so there is no new env read and the
  guard stays as written and uncorrected. Commit it as a `fix(2026-10-04-the-wheel): …` operator commit, run CI on all
  three OSes, and record the run in `evidence/operator-pass.md`. The report's Outcome / CI lines then name the new
  final HEAD's run; append that line to report.md before the re-gate.
- The other 19 entries are green as of this halt (the gate tool's log dir `wrap-2026-10-04T20-44-01/` under its own log root; the summary capture moved to the
  session scratchpad, its figures here: entries 20 · green 16 · red 1 (16) · not-run 3;
  pre-push green). Entries 18–20 are `leg = 'operator'`: re-verify them from the NEW final HEAD's operator-pass evidence.

## Done in this run dir (uncommitted; rides the wrap commit)
- P2: `fanout-results.md` (7 verdicts, 34 proposals + 3 orchestrator raises, dispositions; E1 ratified), raw twins for
  design-system / layout-templates, `cascade-patterns.toml` + `sweep-out.txt` (re-run after the late test-plan
  amendment), `cascade-dispositions.md`, the `*-entry*.md` sidecar payloads (all appended, read back).
  37 amendments · 1 escalation resolved — drift = 0 as of the halt.
- P3: `curation.md` (T2 testing.md 1 · T3 1 · the `--e2e` correction at its source).
- P4: code-graph `rust ok 3784/18485 · ts ok 7/1` (`.refresh-done`, 20:44:24Z).
- P5: `route-new-entry.txt` (Running-turn refusal minted at `working-route.md:82`, the founder's live ruling). CARRY
  pins `:84` (F-W3) and `:111` / `:113` (F-W1); `:82` carries a WATCH (the `.profraw` red, 0/3) and a CARRY (the two
  test comments). `renumber-manifest.md`: 71 route citations, old +2 for old ≥ `:82`, on the operator's word with the
  overseer's two guards; 227 sidecar / archive / source files hash-unchanged.
- P6: `state.yaml` (last_wrap 21:12Z, session 40, tree_db 20:44:24Z) and the handoff (Status tests-failing).
- P7.3 payloads staged, NOT applied: `refine-v1-32.md` + `acceptance-v1-32.md` (`matrix.py refine --id v1-32 --chunk
  2026-10-04-the-wheel --text-file … --acceptance-file …`, the F-W3 Windows clause), `note-v1-31.md`, `note-v1-40.md`
  (`matrix.py note`), then `matrix.py flip`.
- Note for the fix: the operator pass's argv change touches `crates/viola-pty/src/lib.rs` only. It adds no route
  coordinate, but `tests/tui_wheel.rs:267` / `tests/cli_answer.rs:9` still say `:82` (CARRY on the new `:82`). The fix
  MAY correct them to `:84` in the same diff; if it does, the CARRY is spent — strip that one block at resume.

## Commit, when it lands
- Reconcile: 37 amendments (architecture 10 · security-plan 8 · obs-plan 9 · a11y-plan 5 · test-plan 4 · layout-templates 1) · 1 escalation resolved
- Curation: T1 0 · T2 1 · T3 1 (filtered 3 at exactly 0.6) · correction: testing.md --e2e at its source
- Route: +1 entry (Running-turn refusal, founder live ruling) · 3 CARRY + 1 WATCH + 1 CARRY · 71 citations renumbered · master → complete
