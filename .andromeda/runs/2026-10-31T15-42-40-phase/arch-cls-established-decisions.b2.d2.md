# Classified — arch · established-decisions.b2

## Answers
established-decisions#20.1 · yes · the shared connect and `verify_server` check the chunk edits (server check, viola-channel client) must keep writing no frame on a pid or start-time mismatch
established-decisions#21.1 · yes · the scope's `viola hook` bound: exit 0, no body, no stderr, and no chunk code reaches the hook; its codes-only log clause is standing S4 and not carried
established-decisions#22.1 · no · the wrapper's `dialog-expired` line on the dialog path is not reached, and the chunk adds no event kind
established-decisions#22.2 · no · a live measurement of a 671 s dialog hold; no chunk code or check relies on it
established-decisions#22.3 · yes · the server check the chunk edits must send no frame when identity fails, and the hook's exit 0 with no body on that failure is held by the same check
established-decisions#22.4 · no · the hook's own check order ahead of diagnostics and obs; the chunk adds no code to the hook path and the scope names no such order
established-decisions#23.1 · no · pending measurement of a dialog hold near the 3600 s ceiling; the chunk does not touch the hold
established-decisions#24.1 · no · the human-takeover wheel and its `human-typing` / `turn-running` refusals are exits 10–13 and not re-opened by this chunk (Not needed list)
established-decisions#25.1 · yes · the explicit `viola link` / `viola unlink` verbs (reached) and the rule that a refused `send` or `answer` records no link, which the chunk's new refusals must keep
established-decisions#25.2 · yes · the explicit verbs' order (strict check via `refuse_untrusted` before `client::start`, then the liveness pre-check, the server identity check, one request) is where the chunk's strict-modes refusals and edited checks sit
established-decisions#25.3 · no · the wrapper's own `link` / `unlink` channel lines and span; the chunk adds no wrapper line or span for them
established-decisions#25.4 · no · the `link` line's `ts` stamped in `LinkSet::line`, a file the chunk does not reach
established-decisions#26.1 · yes · `answer`, `wait`, `last` and `pause` (reached whole) must not read the budget gate, so the chunk's new codes in their documents come out the same under a closed gate, and `send`'s refusal order sits in the reached `send.rs`
