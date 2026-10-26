# Classified — tests · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · the link's since stamp is a new wrapper clock read, which must go through the viola_core::Clock seam (the budget gate is the one exception as landed)
mocking-stubbing-discipline#2.1 · no · the row names no HTTP double because viola makes no outbound HTTP, and the chunk adds no HTTP call, client or fake server
mocking-stubbing-discipline#2.2 · yes · the verbs' liveness-only pre-check reads heartbeat live against the clock, so the stale-heartbeat case of their refusal-ordering unit layer takes the injected Clock (MockClock::advance) or an instant argument, since a spawned wrapper cannot take a mock clock
mocking-stubbing-discipline#2.3 · yes · the CLI frame-verb case of tests/security_negatives_server_identity.rs needs a held live process recorded by an impostor snapshot, which is the fake agent's --hold-stdout-internal holder, a real process and never a mockall double
mocking-stubbing-discipline#2.4 · yes · the same CLI frame-verb case needs a test-owned viola_channel::Server::bind impostor on the recorded endpoint, which the two new verbs must refuse with no frame and exit 21 instance-unreachable
