# New groups — architecture · established-decisions

## Verdicts
- fanout-architecture--cascade.md 2 · contradiction · lines 130–130 · «`ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes» · the standing line gives exit 11 for a cut before `initialize`, the proposal exit 1 with 11 only the unit child's own code; the new bullet would sit after that line and leave it standing
- fanout-architecture--cascade.md 4 · contradiction · lines 267–267 · «the crate also holds the thiserror enum `Refusal` (`BatchScriptChild`, `NotFound`, the `resolve_program` refusal), which predates that fold; the rule stands, and its fold or ratification is open» · the standing line holds `Refusal` as a live enum whose fold is open, the proposal says it was folded into `AgentError`; the new bullet leaves that line unchanged
- fanout-architecture--cascade.md 5 · contradiction · lines 267–267 · «the crate also holds the thiserror enum `Refusal`» · the standing line names `Refusal` as a thiserror enum, the census of 9 derives omits it; no standing line states the count of 9 or its rule
