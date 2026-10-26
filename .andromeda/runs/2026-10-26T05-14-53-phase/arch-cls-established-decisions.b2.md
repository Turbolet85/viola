# Classified — arch · established-decisions.b2

## Answers
established-decisions#13.1 · no · the wheel's byte classifier, holder and running-turn state live in src/run/wheel.rs and the std worker thread, none of them a reached file, and the chunk changes no wheel logic
established-decisions#14.1 · no · the link set and its implicit record, fed by the from param of CLI and MCP calls, sit in src/run/links.rs and the request path, none of them a reached file, and the chunk changes no link logic
established-decisions#15.1 · no · the gate, its thresholds and the budget.json read sit in src/run/budget.rs and viola-state, none of them a reached file, and the closed-gate test named here runs only in the generic workspace gate every change owes
established-decisions#16.1 · no · the ledger rows, stamps, verify probes and the R8 spawn strip sit in viola-agent-claude and the run and verify spawn paths, none of them a reached file, and the chunk runs no verify, re-stamps nothing and its named tests run only in the generic workspace gate
