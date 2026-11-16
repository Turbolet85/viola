@row_set n=1 row=decision/hook-payloads-recorded-by-verify cell=except seen=2 detector=none
value: StopFailure is registered with no recorded payload, no ledger row and no verify probe; its tests use a synthetic StopFailure payload.
basis: adaptation:3
because: The founder ratified StopFailure's registration with no ledger row, so the exception no longer stands provisionally.
@row_set n=2 row=decision/send-confirmation-trailing-trim-match cell=state seen=1 detector=none
value: standing
basis: adaptation:7
because: The founder ratified the trailing-only tolerance, so the decision no longer stands provisionally.
