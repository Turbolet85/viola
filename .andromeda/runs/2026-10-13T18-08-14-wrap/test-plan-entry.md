
## 2026-10-13-epoch-4-survivors-ii — the 16 typed.rs survivors disposed
**Section:** §10 Quality Gates → Mutation gate
**Change:** the 16 in `src/cmd/verify/typed.rs` (`framing_turns` 5, `SharedPty` 5, `Run`'s ending sequence 6) were disposed by this chunk in its ledger `evidence/survivors.ndjson`. Each was disposed after a behaviour-preserving restatement inside `typed.rs`:
- the two framing waits became the named predicates `added_turn_ended` and `local_command_answered`;
- every `clock.now() + PROBE_DEADLINE` became the one function `probe_until`;
- `SharedPty` became generic over its `Pty`, reached through a test-only recording double;
- the Ctrl-C ending and the pump wait became the plain functions `ctrl_c_ending` and `ended_by`.

Each was then killed by a case in `typed.rs`'s `mod tests`, shown red under its hand-applied mutant and green restored, with no mutation run made. None is owed. The Epoch 5 boundary audit's own run confirms the 45 kills: the 29 of chunk 2026-10-13-epoch-4-survivors and the 16 of this chunk. This replaces "owed to the route entry "Epoch 4 survivors II"" and "confirms the 29 kills", per the heading "2026-10-13-epoch-4-survivors — the Epoch 4 survivors disposed, the resize miss recorded", whose other claims stand.
**Why:** the founder's split moved the Run B driver's survivors to their own entry. His standing direction holds that the mutation workflow runs at the epoch boundary only, so each kill is shown by hand and read by the next boundary audit.
**Ref:** .andromeda/runs/2026-10-13T18-08-14-wrap/
