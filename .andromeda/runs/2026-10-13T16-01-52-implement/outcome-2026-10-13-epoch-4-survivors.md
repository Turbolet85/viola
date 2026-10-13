# Implement outcome — 2026-10-13-epoch-4-survivors

**Outcome: green, and the smoke ran as P2 gates.** The gate block went green in 1 iteration: 17 green, 0 red, and
the 3 `operator` legs not fired by this skill. The scope read is `clean`: 10 files changed, all 10 listed, 0 recorded.
No verification-matrix capability is claimed by this chunk (`matrix.py show`: 0), so there was no matrix write.

Next → the OPERATOR PASS: entry 18 (`gate.py hygiene`) before the pre-CI commit, entry 19 (the push), entry 20 (the
`ci.py conclusion` read). Then `/andromeda-wrap-session`.

## Files changed (10, every one in research's Files to modify)
- `crates/viola-pty/src/lib.rs` — step 2, branch b: `report_paths` names both files `<test>.<pid>.report` /
  `<test>.<pid>.test.report`.
- `crates/viola-agent-claude/src/statusline.rs` — step 3: the test reads the whole `shell_argv` value.
- `crates/viola-state/src/fs.rs` — step 4: `replace_private_retrying(…, retry)` holds the loop. `replace_private_with`
  keeps its signature and calls it with `retry_replace(_, HOST_IS_WINDOWS)`. Two new cases.
- `crates/viola-state/src/board.rs` — step 5: the skip-sum case plants an unlisted key in both logs
  (`unknown_fields: 2`), plus two `cfg(unix)` mode-000 cases.
- `src/run/wheel.rs` — step 6: 4 typing rows, 4 reply rows, one split-read table (2 cases), one recorder case.
  `REPLIES` is unchanged.
- `src/run/dialog.rs` — step 7: the arm-retain case and the `millis` case.
- `src/cmd/hook.rs` — step 8: the relative-answers case that resolves.
- `src/cmd/send.rs` — step 9: `issue_line_shows(json, terminal)` and its 4-case table.
- `src/run/send/slot.rs` — step 10: the expired-window case.
- `src/cmd/verify/typed.rs` — step 11: two cases, inside the test module only (gate entry 8: last line 0).

Evidence: `viola-0.1.0/chunks/2026-10-13-epoch-4-survivors/evidence/survivors.ndjson` and `survivors.md`.
- The ledger has 47 rows: 22 `test`, 7 `restatement`, 16 `owed` to "Epoch 4 survivors II", and 2 `not-measured-here`.
- Gate entry 7 reads `last line 1`.
- `survivors.md` holds all 29 red/green pairs and the step-2 measurement.

## Gate readings (this run's trail: `gate-2026-10-13-epoch-4-survivors.json` in this run dir)
- **Unit:** `run --unit` passed 1997, failed 0.
- **pre-push:** `ok:true`, stage `linux-tests` (coverage suite 2431 passed, Playwright 1 passed, gate no breaches,
  67.97 s).
- **cargo deny:** green.
- **Windows-target clippy:** green.
- **G1, lint probes, G2, G4:** all green.
- **Harness smoke `p-e4s-smoke`:** boot, status and cleanup green (`processes_gone` and `endpoint_gone` true).

## Deviations from the plan, with justifications
1. **Step 8's relative path.** The answers dir is a tempdir under the current dir's `target/`, reached as
   `../<current dir's name>/target/<tmp>/answers`, not a tempdir under the system temp dir. A system temp dir on
   another Windows drive (`windows-2025`: repo on `D:`, temp on `C:`) has no relative path from the current dir. The
   path is still built from `..` components and the tempdir's path, as the step says.
2. **Two reds are not a plain assertion.**
   - `board.rs:164:29` `+=`→`-=` reads red as `attempt to subtract with overflow` at the mutated statement. The sum
     starts at 0, so no case can reach an assertion under that mutant. Its `*=` twin reads red at the assertion.
   - `board.rs:45:19` first read red at the case's own `.expect("read")`. The case was changed to assert the whole
     value (`matches!(…, Ok(Some(ScanRefused::Home)))`) and re-read: red at that assertion.
3. **Step 5's mode-000 cases** assert on `scan`'s value, never a CLI line, and assert that no line is written
   (`capture` reads none), as the step requires.
4. **The kill filter.**
   - The research summary names `test(=tests::name)`. That form selects only a crate-root module's tests, so 19 of
     the 29 pairs first ran no test (`nextest-exit-4`).
   - Those readings were discarded and re-run under `test(/(^|::)tests::name$/)` (journal.md). `survivors.md` holds
     only the valid readings.
5. **Step 2 measured branch b.** All three resize tests read red alone under the mutant at 7.0–7.06 s, with no
   `size 120x40` in any child report. The audit's miss is attributed to two jobs sharing one report file — a
   HYPOTHESIS, not re-run (`inputs#I1`). The rest of the step-2 entry in `survivors.md` gives the detail.

## Surfaced (for the wrap — nothing authored)
- The `viola-pty-watch` file names changed. The plan's expected amendment covers architecture §Occupied Resources.
  `.claude/docs/services/viola-pty.md:33` and `.claude/docs/gotchas.md:111` also still name `<test>.report` /
  `<test>.test.report`, without the pid.
- `.claude/rules/verification-harness.md` Session Additions 2026-09-27 says an inline unit test's full path is
  `tests::name`. That holds only for a crate-root module; in a submodule the path is `<module>::tests::name`
  (measured: 19 filters selected nothing).

## Process hygiene
| process | started by | final state |
|---|---|---|
| harness session `p-e4s-smoke` (supervisor, wrapper, fake agent) | gate entry 12 (this run) | terminated — entry 16 read `processes_gone:true`, `endpoint_gone:true` |
| nextest runs of the hand-applied mutants (98 harness calls: m31 2, the batch 56, the re-run 38, m35 2), the batch green and step 2a (with their PTY children) | this run | terminated — each harness call returned; a `/proc/<pid>/exe` census at 2026-10-13 16:20:58Z found 0 processes whose binary is under this repository |
| `pre-push`'s children (`run --coverage`, `run --browser`, `gate`) | gate entry 17 (this run) | terminated — the entry returned; the same census |
| the `viola` processes on the host (`viola-lab/prototype`) | the operator's bridge | not this run's — another tree's build, left running |

Left on disk, outside the tree: the deliberate mutant reds' kept reports under `$TMPDIR/viola-pty-watch/` (kept on
a failure by design).

## Decisions & corrections
- The operator's direction for this run (the invocation): no live `claude` start, no mutation run, witness run or
  workflow dispatch; a kill shown by applying the mutant by hand; pipeline-tool output to a file, read whole; a
  scripted `.rs` write runs rustfmt in the same call; each long gate run journaled before the wait
  (`journal.md`, four lines); the operator pass with the `ci.py conclusion` read.
- A recurrence of the standing learning "a time written into a record ahead of the clock": the stamp-ahead hook
  refused the journal's first line (16:10Z written at 16:02:54Z) and it was corrected to the read clock.
- The harness filter form: `test(=tests::name)` does not select a test inside a submodule — see Surfaced.
