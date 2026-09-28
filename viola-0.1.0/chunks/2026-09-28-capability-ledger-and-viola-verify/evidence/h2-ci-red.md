# H2 CI red met in this chunk's operator pass — recorded, not folded

On the overseer's word (2026-09-28): "do not fold it. Record it with the run and job, and quote the kill-proof
recorder report from that job log (CHILD_WITHIN, streamed reports). I take H2 to the founder now."

- **Run / job:** ci#36436266196 (head `8cc9f14`, a measurement-only commit of this chunk's operator pass), job
  `test (windows-2025)` 108974874287 — `gh api repos/Turbolet85/viola/actions/jobs/108974874287/logs`
- **Test:** `viola-pty tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code`, in a crate
  this chunk does not touch; the rest of that run's 956 Windows tests passed (955 passed, 1 failed).
- **The class:** H2, open with the founder — a key lost within ~50 ms of a ConPTY resize (rstudio/rstudio#18884).
  The child took its first key before the resize; the key written right after `resize(120x40)` never reached it.
- **The kill-proof recorder:** the child streams one report line per step to a known file outside the test's
  tempdir (`crates/viola-pty/src/lib.rs` `report_path`), and `lines()` waits on it below `CHILD_WITHIN` (7 s, under
  the nextest mutants profile's 10 s kill) and fails the test with the report so far. The job log, verbatim:

  ```
  2026-09-28T14:31:30.5281487Z         FAIL [   7.042s] (955/956) viola-pty tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code
  2026-09-28T14:31:30.5288004Z     thread 'tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code' (4680) panicked at crates\viola-pty\src\lib.rs:582:13:
  2026-09-28T14:31:30.5289180Z     child report stopped at ["start pid=6880 raw=true size=100x30", "byte 78"]
  ```

  The report holds the start line (raw, 100x30) and `byte 78` (the `x` sent before the resize); the next expected
  line, `byte 79 size=120x40` (the `y` sent right after the resize), never arrived within `CHILD_WITHIN`.
- **Owner:** the founder (H2). The wrap carries this record onto the route; this chunk does not fold it.
