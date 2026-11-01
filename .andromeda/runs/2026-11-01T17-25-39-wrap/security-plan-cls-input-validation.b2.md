# Classified — security-plan · input-validation.b2

## Answers
input-validation#1.19 · yes · stale · run's start check runs before any snapshot (F4), so a refused start on Unix reads no statusline source, against the row's "at every start" (src/cmd/run.rs reached)
input-validation#1.20 · no · the new directory and euid checks sit on the hook's dialog path (F18), not the statusline arm, whose order, STATUSLINE_DEADLINE and single shell-out the session leaves as they were
input-validation#1.21 · no · the run.rs change is a start-side socket-directory check, which is Ok(()) off Unix, so the Windows conpty_sideload step and its order are unchanged
input-validation#1.22 · no · N/A entity: no upload, webhook or database surface, and the session adds none
input-validation#2.0 · no · the session adds no content read and no cap change, so MAX_FRAME and its listed bounded readers hold
