@row_add n=1 row=contract/cli-frame-verbs/strict-modes-failed area=test-scope-summary rec=1,2
boundary: CLI frame verbs
item: strict-modes-failed
promise: A CLI frame verb refuses a widened instance before any role file or log opens: exit 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, with no frame and no line written.
@row_add n=2 row=decision/local-command-exact-match area=test-scope-summary rec=3,4
name: Local commands matched by exact equality
rule: Send recognises a local command only by exact equality of the typed text (without its trailing CR and LF characters) against the compiled list: nothing else trimmed, no case folding, never a leading-slash test.
@row_add n=3 row=decision/budget-governor-thresholds area=unit-test-strategy rec=4,5
name: Budget governor thresholds
rule: Automated send is refused with budget-paused once five_hour reaches 90 % or seven_day reaches 85 % (89 and 84 stay open), and release --budget overrides the pause until its override instant has passed.
guard: gate_decide_an_expired_override_is_dropped
