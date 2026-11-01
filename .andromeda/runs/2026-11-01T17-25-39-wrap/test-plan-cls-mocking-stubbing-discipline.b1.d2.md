# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the run.rs edit only moves the socket-directory check ahead of the snapshot, so the budget gate's Utc::now reads and the clock seam are unchanged
mocking-stubbing-discipline#2.1 · no · the session adds no outbound HTTP, so the HTTP row stays N/A
mocking-stubbing-discipline#2.2 · no · Cargo.toml gains only a [[test]] table and every dependency version is unmoved, so mock_instant and tokio test-util stay in no manifest
mocking-stubbing-discipline#2.3 · no · the new root binary reuses the existing fake-agent resolution with no new double or held child, and handlers.rs's Held child is unchanged
mocking-stubbing-discipline#2.4 · no · the kill helpers and cross_exit_causes.rs's lost row are unchanged, and the two new cases are refusals with no kill
mocking-stubbing-discipline#2.5 · no · impostor and squatter listeners stay as they were and the server-identity negatives stay Linux and Windows only; the only change in those files is the temp directory's mode
mocking-stubbing-discipline#2.6 · no · widened home trees are still refused by the product's check on windows-2025, the new board and list cases use real homes, and home.rs gains only socket roots
mocking-stubbing-discipline#2.7 · yes · add-to · F15: the peer euid decisions are injected as function parameters (peer_euid_admitted, the Admission function) with no env, flag or feature seam, beside this row's no-seam parameter pattern
