CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + testing.md: "A test that forces an I/O error must fail at the same call on every CI OS — cfg-scope the case per OS or pick an input whose failing call matches everywhere" (confidence 0.8)
      Proof: CI run 35990393334 (sha 809456e) job mutants MISSED src/obs.rs:193:19 — the directory-based Unreadable test reached the read arm (:197) on ubuntu and the open arm (:194) on Windows; measured this session on Linux 6.6 (busybox cat: dir → "read error: Is a directory"; file/config.json → "can't open …: Not a directory") and Windows 11 (python open: dir → errno 13; file-parent → errno 2); Rust std ErrorKind docs (NotADirectory, stable 1.83). Signals: verified by a real gate failure +0.4 · specific technical detail +0.2 · no-other-home +0.2 (no master, route annotation or ledger note carries it).
    + verification-harness.md: "run --mutants mutates only lines the chunk diff touches (--in-diff) — witness an earlier chunk's missed-mutant kill another way, never by a green mutants job" (confidence 0.8)
      Proof: crates/viola-e2e/src/harness/run.rs:426-440 (`cargo mutants … --in-diff <chunk.diff>`); it changed the chunk's design (the operator's P4 decision to witness the kill through the ubuntu/macOS test job logs). Signals: verified by measurement, changed the design +0.4 · specific technical detail +0.2 · no-other-home +0.2.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters:
    - 1 rejected at 0.6 (Filter 4): "obs-plan §1 is a verbatim obs-scope copy; a cascade hit there stays by rule" — obs :485 already states it in the master body (no-other-home cannot fire); the recurrence is recorded as friction.
    - 1 rejected (Filter 1 / pipeline-owned): "a CI red on the last wrap's sha folds into the next chunk through the loop" — the phase skill's promotion contract (CI verdict fold) already owns it.
    - 1 duplicate (Filter 1): the one-test-per-OnceLock rule (testing.md Session Addition, applied this chunk, not re-added).
  No-other-home: "A test that forces an I/O error must fail at the same call on every CI OS" · "run --mutants mutates only lines the chunk diff touches"
  CLAUDE.md size: 121/200 · T1 1.3 KB, 0 over 600 B
