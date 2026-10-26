# Classified — architecture · established-decisions.b2

## Answers
established-decisions#13.1 · yes · add-to · a SessionEnd whose server check fails now takes the direct append too (F7), a second cause of the turn this record leaves marked
established-decisions#14.1 · yes · stale · the held link and unlink verbs' widening reads as a frame written after the liveness-only pre-check, which every existing frame writer now follows with the server check before its first frame
established-decisions#15.1 · no · the server check reads only the snapshot's pid and start time and adds no budget.json read, so the gate's points and the verbs that never read it (answer, wait, last, pause, hook.dialog) still hold on the changed files
established-decisions#16.1 · no · no ledger row, probe, stamp, relied set or fake-agent mode changes, the reached hints file gains a server-verify hint beside send_hint whose wording stands, and the drift test's ETXTBSY fix is test-only
