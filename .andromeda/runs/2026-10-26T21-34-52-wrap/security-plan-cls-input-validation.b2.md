# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.20 · yes · contradict · the conpty/ owner/DACL gap is retired, as pin_companions runs check_present on conpty/ and each companion before the held open (F9), and the closed sideload_fallback list gains strict-modes-failed
input-validation#1.21 · no · the session adds no upload, webhook or database surface, so the N/A claim holds on the changed files
input-validation#2.0 · yes · stale · update_stamps now runs check_stamps before creating ledger/ and before reading ledger/stamps.json, returning StampsError (F8), so this read clause's reading has moved
