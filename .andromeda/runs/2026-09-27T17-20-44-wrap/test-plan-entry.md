
## 2026-09-27-epoch-2-cleanup — Windows host mutation scratch, scoped inner loop, run archive, keep-failed lifecycle guard, probe 5/5
**Section:** §2 (machine-parseable output) · §3 (Exit codes; `run` Command body, step 4 Command / Classification / Verdict, Output format, Test selection; `gate` Inputs; `pre-push` stages and document; Closed enums; Test data bootstrap Cleanup) · §4 (Test grouping) · §9 (Release build row; release-build errors) · §10 (Mutation gate) · §12 (new `2026-09-27` entry)
**Change:**
- `run --mutants` on a Windows host: the host mutation scratch `<repo parent>/viola-mutants-scratch` (guard → `scratch-refused`; wipe → `scratch-wipe-failed`; `TMP`/`TEMP` + `--output`); `mutants.out/outcomes.json` read from the run's own output dir; the counted/scoped object gains `scratch_bytes`.
- `--file <path>` (repeatable): `verdict:"scoped"` with `files`, never a leg verdict; with `--leg` usage `scoped-leg`. The one-file selector was raw `cargo mutants --file`; now `run --mutants --file`.
- The run document gains `archived` (`target/run-archive/<n>`, newest 10). `pre-push` `cache` gains `windows_scratch_bytes` / `windows_scratch_bytes_after`; `windows-leg` runs in the host scratch.
- viola-e2e's booted lifecycle tests: a `Booted` drop guard keeps a failing test's home under `AGENT_RUN_KEEP_FAILED=1`. §4: viola-e2e (no dev-dependencies) uses a labelled case table instead of rstest `#[case]`.
- §9: the release-check probe reads `5/5 refused, control clean` (was `3/3`), and the check also fails an artifact built with a test-only feature.
**Why:** the Epoch 2 cleanup chunk's measured facts (the temp copy follows `TMP`/`TEMP`: seen in the scratch mid-run, none in `%TEMP%`).
**Kept:** the §12 history entries that list the earlier verdict set stand (true when written).
**Ref:** .andromeda/runs/2026-09-27T17-20-44-wrap/
