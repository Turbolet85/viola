# Classified — architecture · established-decisions.b2

## Answers
established-decisions#20.1 · no · the hook path (hook.rs, the replay read, verify_server) is not in the reached files and the session leaves dec-hook-transport untouched, so its fail-open and server-verify-failed claims hold
established-decisions#21.1 · no · hooks fail open and exit 0, hook.rs is not in the diff and the session changed no hook exit
established-decisions#22.1 · no · the dialog-expired append and WaitFeed live in the wrapper, outside the reached files, and the hook dialog path is unchanged
established-decisions#22.2 · no · the measured 671 s dialog hold is a live measurement the session does not touch
established-decisions#22.3 · no · a failed server check on a dialog hook still sends no hook.dialog and exits 0, the hook path is not reached
established-decisions#22.4 · no · the instance check ahead of diagnostics in the hook is unchanged, and the hook path and strict.rs are not reached
established-decisions#23.1 · no · the pending hold-ceiling reading is a later live measurement that the session does not touch
established-decisions#24.1 · no · the wheel refusals and the pause and release moves hold, the reached send, answer, wait, last and pause arms change only their no-endpoint document and not the wheel
established-decisions#25.1 · no · link recording and unlink events are unchanged, the reached link verb changes only its no-endpoint document, and -32603 keeps its fixed body
established-decisions#25.2 · yes · add-to · F3 joins the explicit verbs' liveness-only pre-check, whose no-endpoint case now reads through without_endpoint as InstanceDead for a directory and UnwrappedName otherwise, behind the strict-modes check
established-decisions#25.3 · no · the cli.link and cli.unlink span and the channel-request method pair for link and unlink are unchanged, the session moves no span or channel frame
established-decisions#25.4 · no · the Utc::now stamp in LinkSet::line lives in src/run/links.rs, which the session does not reach
established-decisions#26.1 · no · the gate's read points and its order before turn-running stand, and answer, wait, last and pause still read no gate with closed-gate documents matching open-gate ones
