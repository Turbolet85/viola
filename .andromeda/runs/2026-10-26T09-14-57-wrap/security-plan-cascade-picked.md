# security-plan · picked accounting

## Picked
threat-model-summary#2.1 · proposed · its line on clients acting on whatever server answers the recorded endpoint is now enforced on Linux and Windows at every frame-writing client
authentication-authorization#1.6 · proposed · the Windows check names `server_process_id()` while the build reads `peer_creds().pid()`, the dated gaps lose their server-identity half, the mismatch outcome moved, and the collision check verifies no server
authentication-authorization#1.8 · not touched · its dated gaps are the strict-modes half, which the report leaves standing, and its reference-values rule still holds for `live_endpoint`'s one read
input-validation#1.4 · not touched · the report's `src/cmd/hook.rs` change adds a pre-frame server check after the stdin cap, event, one-object, dialog-classify and capture rules, none of which moved
input-validation#1.5 · not touched · `tools.rs` gains a server check after the argument validation, which the report leaves as it was; its refusal is an Error Handling fact
input-validation#1.6 · proposed · its fourth gap waits on "Server verification before any frame", which landed; only the strict-modes half stands
input-validation#1.12 · proposed · the snapshot readers are three, not five, and fork 1's PROVISIONAL answer re-pins the readers' move (F8)
input-validation#2.0 · not touched · no bounded reader was added: the peer pid and start time come from the OS, and every `MAX_FRAME` site is unchanged
bootstrap-phases#1.1 · proposed · `viola-channel` now supplies only the peer pid, the verdict lives in `viola-state` liveness, and the snapshot is not yet read after strict-modes
error-handling#1.1 · proposed · server verification has joined the hook with the fail-open detail `server-verify-failed` (F4); strict-modes is still to join
security-anti-patterns#1.1 · proposed · it names Windows `server_process_id()` and lists exceptions whose server-identity half is closed on Linux and Windows
security-anti-patterns#4.1 · not touched · the session moved no ledger row, stamp read or dialog-decision gate
