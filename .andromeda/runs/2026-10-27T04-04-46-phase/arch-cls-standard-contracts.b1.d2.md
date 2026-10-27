# Classified — arch · standard-contracts.b1

## Answers
standard-contracts#1.1 · yes · the agents read reads a row's status as the closed set idle, busy, unknown, and the budget reading keeps its per-field unknown rule
standard-contracts#2.0 · yes · snapshot.json is read through the MAX_FRAME cap, and a higher v or a parse failure is ignored and replayed from events.ndjson
standard-contracts#3.0 · yes · the stamps verdict: a non-v:1 or wrong-shaped ledger/stamps.json is malformed and records cli_verified:false with one parse-rejected line
standard-contracts#4.1 · yes · update_stamps' capped read under its lock is an external reader the proof enumerates with its cap (its strict-modes ordering is owned by home hardening)
standard-contracts#5.1 · no · the link set's replay, restore at start and write-back are Not-needed link-set semantics, beyond the driver / driven validation
standard-contracts#5.2 · no · the set's change on an implicit link and on served link / unlink is Not-needed implicit-link timing, not a validation
standard-contracts#5.3 · no · the already_linked / since answer of a served link is link-set semantics the chunk does not change (Not needed)
standard-contracts#5.4 · no · the unlink answer {} and the no-append on an absent pair are link-set semantics the chunk does not change (Not needed)
standard-contracts#5.5 · yes · a link or unlink refused -32602 writes no line and a failed record or remove is -32603, the order the driver / driven validation must keep
standard-contracts#6.1 · no · the peer pid and start-time compare before a client's first frame is the server identity check, owned by Unix endpoint and home hardening (Not needed)
standard-contracts#7.1 · no · the strict-modes check each frame client runs before that compare is the home strict-modes check, owned by Unix endpoint and home hardening (Not needed)
standard-contracts#8.1 · yes · the send {text, from?} shape and its refusal ladder bind: the paste refusal sits before the first wheel read, and an earlier-rung refusal records no link line and no send-issued
standard-contracts#9.1 · yes · the wait params rule binds: after and timeout_ms are non-negative u64 else -32602 with data null, from follows the from rule, and the checked_add deadline bounds timeout_ms
standard-contracts#10.1 · yes · link and unlink read no from of any shape, so the chunk's from check (absent, null or ViolaName, else -32602) must not reach those two methods
standard-contracts#11.1 · yes · the wrapper's hook.event re-validation (five closed kinds, object data, string text and closed origin on prompt-submitted, no append when invalid, -32603 on append failure) is the closed event decode the chunk keeps
standard-contracts#12.1 · yes · a failed, oversize or malformed claude agents --json read leaves the board as board::read gave it (unknown status, no unwrapped row, never an error), the failure rule the chunk bounds
standard-contracts#13.1 · yes · prompt-submitted text is the hook's decoded prompt, with the pasted_content unwrap and tag reversal in hook.rs, compared with the typed text, and origin is a closed three-value set
standard-contracts#14.1 · no · the link and unlink --json documents live in link.rs, which the chunk does not reach
standard-contracts#15.1 · yes · send and answer are reached, and their capped reads and paste pre-check sit on the path that prints this refused-check document, which the chunk must keep
standard-contracts#16.1 · yes · the statusline arm's stdin through take(MAX_FRAME + 1), its command stdout through take(MAX_FRAME), the statusLine reader rule and the closed ShellVerdict are the chunk's bounds, with exit 0 and empty stderr
standard-contracts#17.1 · no · live_endpoint and server_verdict are the liveness and server identity check owned by Unix endpoint and home hardening (Not needed)
standard-contracts#18.1 · yes · the bounded claude agents --json read (R8-stripped direct spawn, stdout through take(MAX_FRAME + 1), killed at 5 s, parsed to rows or a closed AgentsFailure) and its unknown-on-failure rule bind
