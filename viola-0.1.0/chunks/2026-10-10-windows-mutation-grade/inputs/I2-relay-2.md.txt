The operator's answers in the P4 fork dialog, 2026-10-10, verbatim (each question as asked, then the option chosen and any note):

1. Question: "How should the `mutants (viola)` job fit? At the last dispatch it was cancelled at 120 min with 125 of 140 graded (about 54 s a mutant; the 15 never graded are all in src/run/env.rs)."
   Answer: "Per-file split (Recommended)"
   The option as shown: "Four jobs under package viola: run.rs (50 mutants), main.rs + conpty.rs (34 + 0), env.rs (33), panic_frames.rs (23). Longest is estimated at 52 min, so one dispatch's wall is about an hour, and so is every re-dispatch in the proof loop. The contract test already accepts several items of one package; job names must be made distinct. At the wrap the masters' 'one job per package' wording moves. conpty.rs has no mutants, so it rides with another file."

2. Question: "`fs.rs:18:5`: on Windows `restrict → Ok(())` is the function itself, and the run's verdict has no arm for an argued mutant, so it must leave the generated set. Which restatement? Both keep behaviour and both reverse one line of testing.md (2026-09-24: a trivial other-OS stub sits as a cfg block inside one shared function)."
   Answer: "Two cfg-gated functions (Recommended)"
   The option as shown: "A `cfg(unix)` `restrict` that sets the mode and a `cfg(not(unix))` one whose body is a bare `Ok(())`. Measured at P3 on cargo-mutants 27.1.0: the stub gets no mutant. On Windows the Unix function is left out by the host exclusion, on Linux it is still killed by the mode tests. The five call sites stay as they are."
   The operator's note: "The reversed testing.md line is amended at the wrap with this case named as its exception and the measured reason (the stub gets no mutant on cargo-mutants 27.1.0)."

3. Question: "Which reader backs the host exclusion in `run --mutants`? A syn-based one (`harness::cfg_legs`, 410 lines with tests) was retired on 2026-09-28 with the CI mutation legs and is in git history."
   Answer: "Revive the retired syn reader (Recommended)"
   The option as shown: "About 200 lines come back from git with their tests, adapted from 'legs covering a line' to 'this host, the whole span'. Real Rust parsing, so comments, strings and macros cannot fool it. It returns two direct dependencies to viola-e2e (syn 2.0.119, proc-macro2 1.0.107 with span-locations; both still in Cargo.lock), and the architecture's dependency key records them at the wrap. Smaller for one builder window."
