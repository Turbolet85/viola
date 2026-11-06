@account n=1 entity=span-trace-coverage#2.2 v=proposed
why: the viola-pty row names portable-pty 0.8.1, and the pin moved to =0.9.0
@account n=2 entity=span-trace-coverage#5.1 v=not_touched
why: the report names no symbol added or removed in crates/viola-state/src/pin.rs, so the place_absent rule and its outcome set still hold
@account n=3 entity=span-trace-coverage#6.1 v=not_touched
why: the retired Git Bash warning was a stderr line, never a span or log line, and the send path's spans and checks are unchanged
@account n=4 entity=span-trace-coverage#6.2 v=not_touched
why: the report adds or removes no span attribute or log field on the send path
@account n=5 entity=span-trace-coverage#6.3 v=not_touched
why: no log field or event kind changed, and src/cmd/hook.rs still logs hook-invoked per invocation
@account n=6 entity=span-trace-coverage#6.4 v=not_touched
why: send's exit codes and details are unchanged; only its stderr warning left
@account n=7 entity=span-trace-coverage#7.1 v=not_touched
why: wait and last spans and check order are not moved by the remnant cleanup
@account n=8 entity=span-trace-coverage#7.2 v=not_touched
why: no span attribute or wake kind changed
@account n=9 entity=span-trace-coverage#7.3 v=not_touched
why: no log field, detail code or refusal changed on wait or last
@account n=10 entity=span-trace-coverage#7.4 v=not_touched
why: the wait.block cleanup rule is unchanged by the session
@account n=11 entity=span-trace-coverage#8.1 v=not_touched
why: the dialog spans and fail-open order are unchanged, and the entity claims no v1-27 proof
@account n=12 entity=span-trace-coverage#8.2 v=not_touched
why: the hold range and default stand, and the entity carries no PROVISIONAL ceiling owner to re-point
@account n=13 entity=span-trace-coverage#8.3 v=not_touched
why: no dialog log field or refusal detail changed
@account n=14 entity=span-trace-coverage#8.4 v=not_touched
why: the expiry and backstop bound are unchanged, and no ceiling owner is named here
@account n=15 entity=span-trace-coverage#9.1 v=not_touched
why: the wheel and release spans are unchanged by the session
@account n=16 entity=span-trace-coverage#9.3 v=not_touched
why: the release and pause log fields are unchanged
@account n=17 entity=span-trace-coverage#10.2 v=not_touched
why: the budget span attributes are unchanged; F1 is placed beside the capability ledger entity
@account n=18 entity=span-trace-coverage#10.3 v=not_touched
why: the budget log fields are unchanged; F1 is placed beside the capability ledger entity
@account n=19 entity=span-trace-coverage#10.4 v=not_touched
why: STATUSLINE_DEADLINE and its cleanup are unchanged by the session
@account n=20 entity=span-trace-coverage#11.1 v=not_touched
why: the report moves nothing in the R8 strip or IDENTITY_FLOOR, and the measured-origin wording still holds
@account n=21 entity=span-trace-coverage#14.1 v=not_touched
why: no snapshot-replay caller or line changed in list.rs or events.rs
@account n=22 entity=span-trace-coverage#15.1 v=not_touched
why: the torn-line heal in events.rs is unchanged; its one added line moves no behaviour the report states
@account n=23 entity=span-trace-coverage#16.1 v=proposed
why: F1, the statusline-rate-limits ledger row reading the account's plan, belongs beside the capability ledger entity
@account n=24 entity=span-trace-coverage#17.1 v=not_touched
why: the report re-reads it on 0.9.0 with no value moved, and verify's step and summary format still hold
@account n=25 entity=log-coverage#1.1 v=proposed
why: the process-start row still lists conpty pty_backend values and the Windows sideload_fallback field, which F434 drops
