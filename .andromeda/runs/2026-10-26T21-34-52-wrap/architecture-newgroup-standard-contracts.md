# New groups — architecture · standard-contracts

## Verdicts
- fanout-architecture--cascade-g1.md 25 · distinct · the ledger stamps group (lines 29–37) names `update_stamps` only as the writer and `read_stamps_strict` as `run`'s strict read; no standing line states the writer's strict-modes check before `ledger/` or under its lock, nor `StampsError`
