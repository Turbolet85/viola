# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · stale · the tokio pin moved from =1.53.1 to =1.53.3, so this line's tokio 1.53.1 test-util reading now reads 1.53.3
mocking-stubbing-discipline#2.1 · no · tokio 1.53.3 has the same dependency set and every other crate is unmoved, so the N/A (no outbound HTTP) claim holds on the changed Cargo.lock
mocking-stubbing-discipline#2.2 · yes · stale · the tokio pin moved from =1.53.1 to =1.53.3, so this row's tokio 1.53.1 test-util reading now reads 1.53.3
mocking-stubbing-discipline#2.3 · no · mockall 0.15.0, the seam traits and the stub binaries did not move and the lock delta is tokio alone, so the process-mocking claim holds on the changed Cargo.lock
mocking-stubbing-discipline#2.4 · no · interprocess 2.4.4 and windows-sys 0.61.2 did not move, the lock delta is tokio alone, and the row names no tokio version, so the fake-server claim holds
