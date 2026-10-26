# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the claim still holds on the reached src/cmd/run.rs: the session changed only its unit-test log capture, no product behaviour moved and no seam was added, so the budget gate's direct Utc::now() reads and the clock-seam rule stand
mocking-stubbing-discipline#2.1 · no · the claim still holds on the reached Cargo.toml: its only new entry is the Windows-only windows-sys dev-dependency, Cargo.lock and the release graph are unchanged, so no outbound HTTP enters
mocking-stubbing-discipline#2.2 · no · the claim still holds on the reached Cargo.toml: its only new entry is the Windows-only windows-sys dev-dependency, so mock_instant and tokio test-util stay in no manifest
mocking-stubbing-discipline#2.3 · no · the claim still holds: the new cases spawn no external process (real pipes on one spawned thread) and add no mockall seam, and the reached Cargo.toml adds only windows-sys
mocking-stubbing-discipline#2.4 · no · the squatter and fake CLI rows stand, since the squatted-endpoint case was re-asserted green unedited and the new listener cases bind the real viola_channel server; the two test-owned CreateNamedPipeW controls are an e2e fact left for a later pass
