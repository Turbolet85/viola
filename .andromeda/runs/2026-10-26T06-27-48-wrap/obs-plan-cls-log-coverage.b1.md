# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the session moved only the Cargo.toml tokio pin at line 242 and its lock entry, so the process-start fields and their values stand
log-coverage#1.2 · no · the session changed no spawn or exit path, so the process-exit fields and their enum values stand
log-coverage#1.3 · no · the session changed no channel source, so the channel-request fields and values stand
log-coverage#1.4 · no · the session changed no channel source, so the JSON-RPC result classes and error codes stand
log-coverage#1.5 · no · the session changed no send path, so the send-issued, send-confirmed and send-refused fields stand
log-coverage#1.6 · no · the session changed no dialog path, so the dialog-raised and dialog-answered fields and dialog_kind values stand
log-coverage#1.7 · no · the session changed no hook code, so the hook_event values and the hook-invoked and hook-decision fields stand
log-coverage#1.8 · no · the panic_location shape `<crate>-<ver>/src/...` names no version, so it holds on the changed Cargo.lock
log-coverage#1.9 · no · the session changed no wrapper release path, so the release-from-driver fields stand
log-coverage#1.10 · no · the session changed no viola-state code, so the state-recovered detail enum and its fields stand
log-coverage#1.11 · no · the session changed no parser code, so the parse-rejected parser enum stands
log-coverage#2.1 · no · the session touched neither the refusal code nor schemas/diag-line.v1.json, so the exit-1 details and the 13-value closure stand
log-coverage#2.2 · no · the session changed no refusal code, so the exit-21 details stand
log-coverage#2.3 · no · the session changed no wrapper-fault code, so the exit-20 path stands under the §7 panic containment the session re-read as unmoved
log-coverage#2.4 · no · the session changed no hook code, so the seven fail-open hook-decision codes stand
log-coverage#2.5 · no · the session changed no parser code, so the per-parser parse-rejected details stand
log-coverage#2.6 · no · the session changed no viola-core code, so the RefusalReason details stand
log-coverage#2.7 · no · the session changed no state or file-check code, so the strict-modes-failed and symlink-ignored sites stand
log-coverage#3.1 · no · the session added no spawn, child or shell call and changed no subject, so the spawn pairs and the closed subject enum stand
