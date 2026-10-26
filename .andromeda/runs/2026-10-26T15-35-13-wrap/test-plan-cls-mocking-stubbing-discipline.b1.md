# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · add-to · F8 adds LinkSet::line's direct Utc::now() read for the link since in src/run/links.rs, a second read outside the clock seam that this one-exception rule does not name
mocking-stubbing-discipline#2.1 · no · the session adds no outbound HTTP, so the N/A row still holds
mocking-stubbing-discipline#2.2 · no · the session mocks no time and adds no manifest entry, so the mock_instant and test-util rows and the clock-injection pattern still hold
mocking-stubbing-discipline#2.3 · no · its new root tests run the real viola binary and the fake agent and add no mockall trait or process double, so the process claim still holds
mocking-stubbing-discipline#2.4 · no · the link and unlink rows added to tests/security_negatives_server_identity.rs are cases of its same server-identity table, which keeps the test-owned impostor listener the claim names
