# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · the chunk drives no timer: heartbeat, send-window, budget and link clocks are untouched, and wait and send are named only for their error bodies, not their deadlines
mocking-stubbing-discipline#2.1 · no · HTTP mocking is N/A and no outbound HTTP lies on any of the chunk's three surfaces, so no HTTP double applies
mocking-stubbing-discipline#2.2 · no · the Clock injection and advance pattern serves the heartbeat, send-window and budget tests, none of which the chunk changes or drives
mocking-stubbing-discipline#2.3 · yes · the -32603 fault is forced only by process and file faults or existing mockall seams, so any new mock stays on seam traits only, and viola-fake-agent runs as the claude stub in root tests
mocking-stubbing-discipline#2.4 · yes · viola-fake-agent as the fake CLI binds, since the child pipe drain witness runs it as claude on a test-scoped PATH; the squatter and impostor server rules are not reached
mocking-stubbing-discipline#2.5 · yes · the -32603 fault may be forced by a file fault, which must be a real mode on a real tempfile home and never stubbed; the 0600 diagnostics/ leg of v1-47 reads real modes too
mocking-stubbing-discipline#2.6 · no · the placed hook in viola-state fs.rs is a private placement parameter the chunk never reaches, and no placement window is in scope
