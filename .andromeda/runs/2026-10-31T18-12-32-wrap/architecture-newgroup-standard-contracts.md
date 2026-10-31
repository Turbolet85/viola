# New groups — architecture · standard-contracts

## Verdicts
- fanout-architecture--cascade-g2.md 6 · contradiction · lines 111–117 · «prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21)» · lines 111 and 117 give the exit-21 document's `detail` as null, the proposal as one of five closed causes
- fanout-architecture--cascade-g2.md 8 · contradiction · lines 117–117 · «print the same `{"v":1,"error":"instance-unreachable","detail":null}` on stdout under `--json` (exit 21) for their refused strict-modes check of the home and the target instance, made by `client::refuse_untrusted`» · line 117 has `refuse_untrusted`'s document read `detail` null, the proposal has it write `StrictModes` (`strict-modes-failed`)
- fanout-architecture--cascade-g2.md 9 · distinct · no group names `Launch.json`, `refused_with` or the start's refusal documents
- fanout-architecture--cascade-g2.md 10 · distinct · no group states `viola revive --json`'s documents or the `pump_child` exception
- cascade-compare-architecture.md 6 · contradiction · lines 111–117 · «prints exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout (exit 21)» · lines 111 and 117 give `detail` as null, the proposal as `"<cause>"` of five closed causes; it also restates fanout-architecture--cascade-g2.md 6 and 8
- cascade-compare-architecture.md 7 · distinct · no group states `client::Unreachable`'s variants or how a CLI frame verb's failure maps to a cause
- cascade-compare-architecture.md 8 · distinct · no group states the MCP unreachable result's body or the path of each of its causes; lines 63 and 99 name only the refused check and the board's refusal, consistent with it
- cascade-compare-architecture.md 9 · twin · fanout-architecture--cascade-g2.md 10 · both state `viola revive --json`'s exit-1 documents and the `pump_child` exception; its `Launch` and `refused_with` part is also fanout-architecture--cascade-g2.md 9, and only its list of codes is new
- cascade-compare-architecture.md 10 · distinct · no group names `stale-heartbeat`; line 24's `already-live` covers a fresh replayed beat and frees a stale replayed one, which the proposal does not restate otherwise
- cascade-compare-architecture.md 11 · distinct · no group states the catch site's internal-error document, `json_asked` or `internal_error_document`
- cascade-compare-architecture.md 12 · distinct · no group names `without_endpoint`; line 135 lists `live_endpoint`'s `None` for a replayed, absent or unread snapshot and says nothing of a stale wrapper either way
