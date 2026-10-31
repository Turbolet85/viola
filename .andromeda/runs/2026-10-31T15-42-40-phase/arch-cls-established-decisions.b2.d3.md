# Classified — arch · established-decisions.b2

## Answers
established-decisions#20.1 · yes · the shared connect's server check: a peer whose pid or start time differs from the snapshot's gets no frame, which the chunk's edited server check must keep; the hook's own record is not reached
established-decisions#21.1 · yes · the hook bound: exit 0, no body, empty stderr, exit 2 never; no chunk code may reach the hook path
established-decisions#22.1 · no · the wrapper's dialog-expired line at a hold's end; the chunk adds no event kind and does not touch the dialog hold or the wait feed
established-decisions#22.2 · no · the measured live dialog hold of a past chunk; the chunk does not touch the hold
established-decisions#22.3 · yes · a dialog hook whose server check fails sends no frame and exits 0 with no body, which the edited server check must keep
established-decisions#22.4 · yes · the hook's refused-instance path writes no line and exits 0; refuse_untrusted runs the same check_instance, and no chunk code may add a line to the hook
established-decisions#23.1 · no · the pending read of the dialog hold's ceiling, owed to a later chunk
established-decisions#24.1 · no · the wheel's byte classification and its human-typing and turn-running refusals; Not needed (exits 10–13 not re-opened), and the chunk's send edit reads no wheel state
established-decisions#25.1 · yes · a refused send or answer records no link and the link record sits directly before send-issued, so the new wrapped:false refusal on send must come ahead of that record
established-decisions#25.2 · yes · cmd/link.rs and cmd/unlink.rs: check_instance through refuse_untrusted before client::start, refusal exit 21, then the liveness pre-check and server check, all edited units
established-decisions#25.3 · no · the wrapper's silence on link and unlink (no span, no process-log line, no names in the channel pair); the chunk adds no wrapper line and its codes sit CLI-side
established-decisions#25.4 · no · LinkSet::line's Utc::now timestamp; the chunk touches no link line
established-decisions#26.1 · yes · send's refusal order (control-character, empty-text, human-typing, budget-paused, turn-running) where the wrapped:false refusal joins send; answer, wait, last and pause must not read the gate, and last.rs, pause.rs, answer.rs and wait.rs are reached whole
