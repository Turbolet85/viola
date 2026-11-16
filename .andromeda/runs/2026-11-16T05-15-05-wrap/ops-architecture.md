@row_set n=1 row=decision/capability-ledger cell=except seen=2 detector=D-arch-decisions
value: StopFailure is registered, provisionally, with no ledger row and no verify probe; delivery confirmation relies, provisionally, on the CLI's trailing-whitespace trim at submit with no ledger row.
basis: fct-imkps2py
because: The chunk's confirmation claim relies on an assumed CLI trim set with no ledger row added, so the ledger rule's named exceptions must carry it.
@row_set n=2 row=decision/cli-capability-ledger cell=except seen=2 detector=D-arch-decisions
value: StopFailure is registered, provisionally, with no ledger row and no verify probe; delivery confirmation relies, provisionally, on the CLI's trailing-whitespace trim at submit with no ledger row.
basis: fct-imkps2py
because: The chunk's confirmation claim relies on an assumed CLI trim set with no ledger row added, so the ledger rule's named exceptions must carry it.
@row_set n=3 row=decision/capability-ledger-gate cell=except seen=2 detector=D-arch-decisions
value: A CLI shape no send relies on needs no probe; StopFailure and the CLI's trailing-whitespace trim at submit are relied on, provisionally, with no ledger row and no verify probe.
basis: fct-imkps2py
because: The chunk's confirmation claim relies on an assumed CLI trim set with no ledger row or probe added, so the gate's named exceptions must carry it.
@row_set n=4 row=contract/hook-event/driver-relabel cell=promise seen=1 detector=none
value: A prompt-submitted whose text is the in-flight send's typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with origin driver, its text kept as submitted, and only then settles that send.
basis: fct-ynhvzkwk
because: The claim now compares through the submitted-form predicate instead of plain equality, so the relabel's promise of «equals» is false.
@row_set n=5 row=contract/channel-methods/hook-event-driver-origin cell=promise seen=1 detector=none
value: A `prompt-submitted` that is the in-flight `send`'s typed text, or a non-empty text equal to it less its trailing run of ASCII space, TAB, CR and LF, is appended with `origin:"driver"`, its text kept as submitted, and only then settles that send.
basis: fct-ynhvzkwk
because: The claim now compares through the submitted-form predicate instead of plain equality, so the promise of «equal to» is false.
@row_retire n=1 row=decision/exact-prompt-match seen=1 heir=decision/submitted-form-prompt-match detector=D-arch-decisions
why: The delivery match is no longer exact: a submitted text equal to the typed text less its trailing whitespace run is also taken.
basis: fct-ynhvzkwk
because: The claim compares through the new submitted-form predicate, so a row named and keyed for an exact match no longer states the decision.
@row_add n=1 row=decision/submitted-form-prompt-match area=established-decisions fact=F3 detector=D-arch-decisions
name: Prompt match in the submitted form after agent normalisation
rule: The typed text matches prompt-submitted's text, whatever its origin, after the agent crate removes the long-paste wrapper and frame and reverses tag escaping, when equal or when non-empty and equal to the typed text less its trailing run of ASCII space, TAB, CR and LF.
why: The CLI trims whitespace at submit; the trim set is an assumption not yet read live, and trimming at either end is held as the candidate.
state: provisional
basis: fct-ynhvzkwk
because: The confirmation match gains a provisional trailing-whitespace tolerance, compared inside the agent crate, which replaces the exact-match decision.
@row_add n=2 row=decision/send-flight-lock area=established-decisions fact=F1 detector=none
name: Send post-gate section under the wrapper flight lock
rule: A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel moves also take; a human key never takes the lock.
why: Closes the race between the readiness gate and a turn the wrapper learns of, while the human's keys never wait.
basis: fct-ynhvzkwk
because: The chunk puts the post-gate reads, link, send-issued and paste under one flight guard shared with the claim and the turn and wheel moves, a decision no row states.
@row_add n=3 row=decision/send-race-residual area=established-decisions fact=F2 detector=none
name: Send race residual before the wrapper learns of a turn
rule: The send race is closed only for turns the wrapper has learned of before the paste; a turn the CLI started whose UserPromptSubmit has not yet reached the wrapper is an accepted residual.
basis: fct-izz7vw4n
because: The flight lock reaches only turns already known to the wrapper, and the remaining window is accepted, a decision no row states.
