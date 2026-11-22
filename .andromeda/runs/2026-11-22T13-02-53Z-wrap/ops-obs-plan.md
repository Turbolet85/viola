@row_set n=1 row=decision/run-threads-inside-catch-unwind cell=rule seen=1 detector=none
value: The three run PTY threads, the two pump copy threads and the paste writer thread, each run inside catch_unwind, and the child's exit is polled inline on run's main thread under main's catch_unwind; a worker panic that leaves main running with no process-exit is forbidden.
basis: fct-vl24ioge
because: The paste writer thread joins the two copy threads under catch_unwind, so the run PTY threads under catch_unwind move from two to three and the count the rule holds must follow.
@row_set n=2 row=decision/run-worker-panic-ends-run cell=rule seen=1 detector=none
value: A panic in any of the three PTY threads (two copy threads, the paste writer) is sent to main, which kills the child and takes run's exit path, exit 1 internal-error; the child's exit is polled on main, not a handle-wait thread; a worker panic leaving main running with no process-exit is forbidden.
basis: fct-z6owtgsx
because: The paste writer thread is started through spawn_worker inside catch_unwind and its panic is Worker::Panicked with the child killed, so the row's two-thread wording no longer covers every PTY worker.
@row_set n=3 row=decision/panic-caught-at-every-role cell=rule seen=1 detector=none
value: A hand-written panic hook plus catch_unwind at every role's main-thread catch site (covering hook dispatch and run's inline handle-wait), the three run PTY threads (the two pump copy threads and the paste writer thread) and the vt100 feed turn a panic into one record.
basis: fct-vl24ioge
because: The rule names the count of run PTY threads under catch_unwind, which moved from two to three with the paste writer thread.
@row_set n=4 row=contract/send-refused-detail/refusal-reason-words cell=promise seen=2 detector=none
value: Refusal details are the arch RefusalReason details verbatim: manual-pause, input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut.
basis: fct-vl24ioge
because: The closed not-delivered detail set gains paste-cut, so the enumerated list of refusal detail words must carry it.
@row_set n=5 row=contract/send-log-line/detail cell=promise seen=1 detector=none
value: The send detail is one of input-not-ready, no-prompt-submitted, control-character, empty-text, turn-running or paste-cut.
basis: fct-z6owtgsx
because: The wrapper's send now refuses not-delivered with detail paste-cut, so the closed list of send details must name it.
@row_set n=6 row=contract/send-path-log-events/event-order cell=promise seen=2 detector=none
value: A send logs channel-request, then send-issued, then hook-invoked (UserPromptSubmit, or SessionStart for a confirmed /clear or /compact; none for an unconfirmable send, a cut paste, or a local command confirmed by its transcript echo), then send-confirmed or send-refused, then channel-response.
basis: fct-z6owtgsx
because: A paste cut at its bound refuses paste-cut after send-issued with no hook awaited, a path the stated order did not cover.
@row_add n=1 row=decision/send-cut-mark-uninstrumented area=span-trace-coverage fact=F1 detector=D-obs-instrumentation
name: Send cut mark uninstrumented
rule: The cut mark a cut paste leaves is in-memory wrapper state that opens no span and writes no line of its own; its set shows as the cut send's send-refused paste-cut record and its clears as their own prompt-submitted, session-start and session-end records.
basis: fct-shrbvdzl
because: The chunk adds the cut mark as a new wrapper state, and the plan carried a new row saying it opens no span and writes no line of its own.
@row_add n=2 row=contract/send-refused-detail/paste-cut area=span-trace-coverage fact=F2 detector=none
boundary: wrapper send-refused detail
item: paste-cut
promise: A paste cut at its bound refuses not-delivered paste-cut after send-issued, with the send's cursor; a send refused on a standing cut mark refuses paste-cut with no cursor and nothing issued.
basis: fct-z6owtgsx
because: The wrapper's send-refused gains the closed detail paste-cut with two placements, a contract item its sibling detail rows do not hold.
