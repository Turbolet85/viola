# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the agents row status is the closed set idle, busy or unknown, and an unwrapped row's sessionId is never a ViolaName
standard-contracts#2.0 · yes · snapshot.json is read through the MAX_FRAME bound, and a higher v or a parse failure is ignored and replayed from events.ndjson
standard-contracts#3.0 · yes · a non-v:1 or wrong-shaped ledger/stamps.json is cli_verified:false in run::version_gate, which reads no measured field and applies the relied-row verdict
standard-contracts#4.1 · yes · update_stamps's capped read under its lock is an external reader the chunk must bound and enumerate; its strict-modes ordering is Unix endpoint and home hardening's
standard-contracts#5.1 · no · the link-set restore at start is Not needed beyond driver and driven validation; its events.ndjson pass is events::read_from's cap, not this rule
standard-contracts#5.2 · no · the set's changes through record and remove are link-set semantics, Not needed beyond driver and driven validation
standard-contracts#5.3 · no · the already_linked and since answers are link-set semantics, Not needed beyond driver and driven validation
standard-contracts#5.4 · no · unlink's {} answer and its absent-pair no-op are link-set semantics, Not needed beyond driver and driven validation
standard-contracts#5.5 · yes · a link or unlink answered -32602 on its driver or driven checks writes no line, so those checks run before any record
standard-contracts#6.1 · no · the per-frame peer pid and start-time comparison is the server identity check, owned by Unix endpoint and home hardening
standard-contracts#7.1 · no · the strict-modes check before each frame's comparison is owned by Unix endpoint and home hardening
standard-contracts#8.1 · yes · a send refused at an earlier rung records nothing and types nothing, which the wrapper's paste refusal (control-character first, before the first wheel read) must meet
standard-contracts#9.1 · yes · wait's after and timeout_ms type rule (non-negative u64, else -32602 with data null) and its checked_add deadline are channel input bounds
standard-contracts#10.1 · yes · link and unlink read no from of any shape, so the from type check must not apply to them
standard-contracts#11.1 · yes · the wrapper re-validates hook.event: one of the five closed kinds, an object data, prompt-submitted text a string with origin harness or human, and an invalid event is never appended
standard-contracts#12.1 · yes · a failed, oversize or malformed claude agents read leaves every status unknown and no unwrapped item, never an error, which is the bounded read the chunk owes
standard-contracts#13.1 · no · prompt-submitted read-back and the pasted-wrapper unwrap are delivery matching, Not needed beyond typed_text; its text-is-a-string and origin rules bind through 11.1
standard-contracts#14.1 · no · link and unlink --json output shapes: their CLI modules are not reached, and the strict-modes refusal they print is owned by Unix endpoint and home hardening
standard-contracts#15.1 · no · the frame verbs' strict-modes refusal document is owned by Unix endpoint and home hardening (Not needed)
standard-contracts#16.1 · yes · the statusline arm's stdin through take(MAX_FRAME + 1), the user command's stdout through take(MAX_FRAME), no print on oversize or malformed stdin, and the statusLine reader's type, non-empty and no-NUL rule
standard-contracts#17.1 · no · the live_endpoint and server_verdict identity read is owned by Unix endpoint and home hardening, and session liveness timing is Not needed
standard-contracts#18.1 · yes · the claude agents --json read is stdout through take(MAX_FRAME + 1) in run::agents, parsed into rows or the closed AgentsFailure, and any failure leaves every status unknown and is never an error
