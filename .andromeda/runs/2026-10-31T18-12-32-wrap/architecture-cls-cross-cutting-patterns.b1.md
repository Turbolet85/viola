# Classified — architecture · cross-cutting-patterns.b1

## Answers
cross-cutting-patterns#1.1 · no · the session adds no home, config key or environment read; src/cmd/mod.rs and src/cmd/run.rs change only refusal and json paths, so the home-resolution and canonicalisation claims hold
cross-cutting-patterns#2.1 · no · the capability-ledger rows and stamps are not in the diff, verify changes only its log codes, and the session adds no claude dependence, so the agents-join and resume-session claims hold
cross-cutting-patterns#3.1 · no · no reader of external bytes was added and without_endpoint opens nothing, so the bounded-reader census claim holds on the changed files
cross-cutting-patterns#4.1 · yes · add-to · the new cause and exit-1 words (F17) become rows of tests/contract_error_bodies.rs, each carrying no path, pid or upstream text
