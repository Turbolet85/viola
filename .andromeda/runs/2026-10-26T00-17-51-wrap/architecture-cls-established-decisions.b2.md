# Classified — architecture · established-decisions.b2

## Answers
established-decisions#13.1 · no · its rule lives in src/run/wheel.rs, which the session does not reach, and the report shows no product behaviour moved
established-decisions#14.1 · no · the route-entry order it cites still holds after the tokio entry is inserted right after Windows endpoint admission, and links.rs is unreached
established-decisions#15.1 · yes · stale · its gate lives in reached src/run/budget.rs, which the session changed only in test code, and the report names no re-read of the gate
established-decisions#16.1 · no · the report's Untouched item shows no ledger row, probe, stamp or relied-set count moved and viola-agent-claude unedited
