# Curation — 2026-09-24-fake-agent-and-test-data-fixtures

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (none)
  Tier 2 (.claude/rules/*):                   + testing.md: "Wait on the exact line a test asserts … never on an earlier sibling" (confidence 0.8)
                                               + testing.md: "Inside a `proptest!` body build strings outside the format macro …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "Reproduce a folded CI red on the host before planning its fix" (confidence 0.8)
  Filters: 1 dup ("consumer-first, no shapes invented before a recorded fixture": now in test-plan §7/§12 and the verification-harness rule body) · 0 task-specific · 0 conflict · 0 deferred
  No-other-home: "Wait on the exact line a test asserts"; "proptest! inline format captures"; "Reproduce a folded CI red on the host"
  CLAUDE.md size: 120/200 · T1 0.9 KB, 0 over 600 B

## Proofs
- **Wait on the asserted line.** Verified by a real gate failure (+0.4), specific technical detail (+0.2), no-other-home (+0.2).
  - Proof: implement gate `run --integration --filter 'binary(cli_fake_agent)'` went red on `booted_wrapper_fixture_is_ready_and_receipting` (assertion `!of_kind(&lines, "env").is_empty()` after waiting on `start`). It went green after waiting on `env` (run dir `.andromeda/runs/2026-09-24T08-27-34-implement`, gate show --n 8).
  - A wording of this rule briefly written into the verification-harness rule body during the cascade was removed: no master carries it, so the leaf could not hold it.
- **proptest! inline captures.** Verified by a compile failure (+0.4), specific technical detail (+0.2), no-other-home (+0.2).
  - Proof: `cargo clippy --workspace --all-targets --features fake-agent` exit 101, "there is no argument named `stem`" / "`bad`" at `crates/viola-core/src/lib.rs` inside `proptest!`. It was fixed by building the string outside the macro.
- **Reproduce a folded CI red.** Verified by measurement that changed the chunk's design (+0.4), specific technical detail (+0.2), no-other-home (+0.2).
  - Proof: phase P3 local run of `cargo mutants --in-diff <Rust-free diff>` exited 0 and left `mutants.out/outcomes.json` at its earlier mtime. The phase P5 baseline then read `ok:true "tested":8` from that stale file. This added the stale-removal requirement to scope item 7.
