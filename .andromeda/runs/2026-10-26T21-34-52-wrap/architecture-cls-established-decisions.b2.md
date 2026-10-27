# Classified — architecture · established-decisions.b2

## Answers
established-decisions#17.1 · no · the wheel rule lives in src/run/wheel.rs, which the change does not reach, and the reached frame verbs only gain a strict-modes refusal ahead of their frame, so the wheel contract the report re-reads as true still holds
established-decisions#18.1 · no · the link rule (implicit record, log replay, the link and unlink channel methods) lives in src/run/links.rs, which the change does not reach, and link.rs only moves its check into the shared refuse_untrusted ahead of the frame, so a refused call records no link
established-decisions#18.2 · yes · add-to · live_endpoint, the liveness-only pre-check the explicit verbs run, now proceeds only on a real snapshot and gives no endpoint on a replayed one (F11)
established-decisions#18.3 · no · the wrapper's no-span and no-line rule for link and unlink is unchanged, a refused strict-modes check writes no line anywhere, and the reached obs.rs change concerns the diagnostics opens (F15), not the wrapper's channel pair
established-decisions#18.4 · no · LinkSet::line and its Utc::now() stamp sit in src/run/links.rs at line 92, which the change does not reach
established-decisions#19.1 · no · the budget gate, its budget.json read and the send refusal order are wrapper-side in src/run/budget.rs, which the change does not reach, and the report lists the budget governor and read_budget as unchanged
established-decisions#20.1 · no · the ledger rows, LedgerRow::relied() and the stamped CLI versions are untouched, and verify's new home-tree refusal sits before its cli log without changing the probe runs or the single stamp writer
