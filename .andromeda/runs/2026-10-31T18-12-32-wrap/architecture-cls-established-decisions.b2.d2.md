# Classified — architecture · established-decisions.b2

## Answers
established-decisions#20.1 · no · the hook path (src/cmd/hook.rs) is not in the diff; the hook still records server-verify-failed and exits 0 with no body
established-decisions#21.1 · no · hook is not reached and the catch site keeps hook silent (F12 excludes it), so stderr stays empty and the exit stays 0
established-decisions#22.1 · no · the wrapper's dialog-expired append lives in src/run, which is not reached, and nothing in the session touches it
established-decisions#22.2 · no · a measured 671 s live round on the hook's reply path; no reached file or change touches that path
established-decisions#22.3 · no · the hook's server-check failure path (hook.rs, not reached) still sends no hook.dialog and exits 0 with no body
established-decisions#22.4 · no · the hook's instance check precedes all its reads; hook.rs and strict.rs are not in the diff
established-decisions#23.1 · no · a pending reading about the hold ceiling; the session changes no hold or dialog path
established-decisions#24.1 · no · the wheel lives in src/run/wheel.rs (not reached); the reached pause, send and answer verbs change only their no-endpoint arm, so the human-typing and manual-pause refusals still hold
established-decisions#25.1 · no · link recording and the from-passing are unchanged; the reached link, send, answer and tools.rs changes sit in error arms only
established-decisions#25.2 · yes · add-to · F3: the explicit verbs' pre-check now reads a cause for no endpoint through without_endpoint, still behind check_instance and opening nothing
established-decisions#25.3 · no · the wrapper's channel pair and the cli.link / cli.unlink dispatch span are unchanged; the session adds no span and no wrapper line
established-decisions#25.4 · no · cites src/run/links.rs:92, which is not reached, and no reached file named in the citation rule is cited
established-decisions#26.1 · no · the gate's read points, check order and evaluation (src/run/budget.rs, not reached) are unchanged; the reached verbs change only error arms, revive changes only its documents, and list changes only its board refusal document
