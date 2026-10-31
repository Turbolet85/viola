# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · yes · stale · the change reached `src/cmd/mod.rs`, which holds `resolve_home`, `canonical_home` and `dispatch`'s per-arm `resolved_home` read, and no reading shows the home rule still holds on that changed file
cross-cutting-patterns#2.1 · no · the ledger rows live in `viola-agent-claude` and the stamps in `viola_state::stamps`, neither in the diff, and `verify`'s change is log codes on its refusals only, so the probes and stamps stand
cross-cutting-patterns#3.1 · no · the session added no reader of external bytes (`without_endpoint` opens nothing), so the bounded-reader table and census still hold
cross-cutting-patterns#4.1 · yes · add-to · F17: every new cause and exit-1 word passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, carrying no path, pid or upstream text
