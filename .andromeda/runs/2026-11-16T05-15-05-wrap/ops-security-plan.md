@row_set n=1 row=decision/typed-text-line-ending-normalisation cell=why seen=1 detector=none
value: Ratified; only CR and LF are rewritten, so the set of accepted texts is unchanged.
basis: fct-ynhvzkwk, fct-hlqa3jtq
because: The delivery-confirmation match is no longer an exact equality but takes a trailing whitespace run as well, so the why's claim that the match stays an exact equality is false and leaves the row.
@row_retire n=1 row=decision/delivery-confirmation-exact-match seen=1 heir=decision/delivery-confirmation-trailing-tolerance detector=none
why: Delivery confirmation no longer compares by exact equality alone; a submitted text equal to the typed text with its trailing ASCII space, TAB, CR and LF run removed is confirmed too.
basis: fct-ynhvzkwk, fct-hlqa3jtq
because: is_submitted_form replaces the exact comparison in the claim, so the row's name, rule and key, all made from exact equality, are false and the decision returns under a new key.
@row_add n=1 row=decision/delivery-confirmation-trailing-tolerance area=threat-model-summary fact=F3 detector=none
name: Delivery confirmed with a trailing whitespace tolerance
rule: Delivery confirmation takes the submitted prompt text as the driver's when it equals the typed text exactly, or is non-empty and equals the typed text with its trailing run of ASCII space, TAB, CR and LF removed; no other difference, leading trimming included, is tolerated.
why: The claude CLI trims whitespace at submit, so a driver's send ending in that whitespace otherwise read back as not its own.
guard: submitted_form_takes_a_trailing_run_the_cli_trims
state: provisional
basis: fct-ynhvzkwk, fct-hlqa3jtq, fct-kbepn7pe
because: The chunk widens the confirmation match to a trailing whitespace run, held provisional pending the founder's word, and the threat model must say the boundary as it now stands.
@row_add n=2 row=decision/send-under-flight-lock area=threat-model-summary fact=F1 detector=none
name: Send's post-gate checks and paste under one flight lock
rule: After the readiness gate, a send's human-typing and turn-running reads, link record, send-issued and paste run under one flight lock hook events also take, so a turn or human prompt the wrapper learned of before the paste refuses the send with nothing typed; a human key never takes the lock.
why: Closes the race between the readiness gate and the paste without ever making a human key wait.
basis: fct-ynhvzkwk, fct-gpeny4p4, fct-72hewwbd
because: The chunk closes the readiness-gate turn race by a lock shared with hook events, a control no standing row of the area states.
@row_none n=1 fact=F2 type=9
what: An accepted residual owned by the standing live rig carry, pending a hook-latency measurement; the route holds it.
