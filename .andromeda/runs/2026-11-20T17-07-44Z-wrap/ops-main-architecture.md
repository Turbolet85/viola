@row_set n=1 row=decision/run-start-order cell=rule seen=3 detector=none
value: A start resolves the program, checks collision, pins the copy and plugin, gates the version, binds the endpoint, runs any automatic verify, on a verified verdict writes an untrusted spawn folder's trust record, writes the first snapshot, heartbeat and start events, and only then spawns the child.
basis: fct-zfntgo5a
because: The start now runs its automatic verify after the endpoint bind and writes the trust record after it on the final verdict, so the order the row states no longer holds; the set keeps the row's «only then».
@row_add n=1 row=decision/stop-during-start-verify area=established-decisions detector=none
name: A stop during the start's verify ends the start
rule: The start's stop handler stands before its automatic verify's start line; a SIGINT or SIGTERM in that window TERMs the verify child, KILLs it after a 30 s grace, writes its exit line and re-raises the signal: no stamp, no snapshot, no session. The child dies with the start by a parent-death SIGTERM.
basis: fct-gookuxzl
because: The chunk fixes how a stop ends a start whose verify runs, with the handler placed before the start line as built (report Deviations 2), a bound later start and cleanup work relies on.
@row_add n=2 row=decision/automatic-verify-attempt-lock area=occupied-resources detector=D-arch-resources
name: Automatic verify under its own attempt lock
rule: A start verifies only holding ledger/verify.lock, owner-only, try-locked after the stamps strict-modes check, never the stamps lock; a held lock means no verify and an unverified start. Under it the start takes another start's verdict only when the entry changed after its own gate read it.
guard: verify_window_two_starts_at_once_run_one_verify
basis: fct-to36e532
because: The chunk lands the attempt lock that keeps two starts from both verifying, with the under-lock re-read narrowed as built (report Deviations 1), a resource and rule the registry must hold.
