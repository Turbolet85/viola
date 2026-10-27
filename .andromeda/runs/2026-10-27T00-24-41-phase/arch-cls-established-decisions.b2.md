# Classified — arch · established-decisions.b2

## Answers
established-decisions#17.3 · no · the dialog hook's server-check refusal (src/cmd/hook.rs) is not reached; the chunk's concern is a spawn of the pinned path failing during a write, not a dialog refusal
established-decisions#17.4 · no · the event and dialog instance check in src/cmd/hook.rs is not reached; the chunk moves no hook code and reads no refused-instance exit
established-decisions#18.1 · no · a dialog hold near its 3600 s ceiling is not reached; the chunk reads and measures no hold
established-decisions#19.1 · no · the human-takeover wheel and its byte classifier are untouched (Not needed); the run start's pin and companion edits sit before the pump and change no wheel reading, the Windows ConPTY one included
established-decisions#20.1 · no · link recording and the link set's restore from the log (src/run/links.rs) are untouched (Not needed); the start's pin edits do not move that restore
established-decisions#20.2 · no · the explicit link and unlink verbs' check_instance through client::refuse_untrusted is not reached; the client verbs are untouched
established-decisions#20.3 · no · the link and unlink wrapper's silence (no span, no process-log line) belongs to the link verbs, which the chunk does not reach
established-decisions#20.4 · no · LinkSet::line's Utc::now() stamp at src/run/links.rs:92 is not reached; the chunk adds no clock use there
established-decisions#21.1 · yes · a record that cannot land is an error and the recorded state stays as it was, and the snapshot's budget fields are rewritten through replace_private, so the fix must keep its failure-leaves-target guarantee (the gate itself is untouched)
