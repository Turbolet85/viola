@row_set n=1 row=contract/viola-wait/turn-ended cell=promise seen=1 fact=F1 detector=none
value: Exits 0 with event.kind "turn-ended" and a cursor at or past that record's offset, whatever the record's background_tasks (zero, any count or null); telling a finished session from a paused one is the driver's.
basis: fct-xueqgrc6, fct-j7joxnr6
because: The chunk pinned that every turn-ended wakes a parked wait whatever its background_tasks, and the row names the wake without that clause.
@row_set n=2 row=contract/viola-wait/non-waking-records cell=promise seen=1 detector=none
value: Activity and wheel records, and a harness-origin prompt-submitted, never wake a parked wait.
basis: fct-xueqgrc6, fct-j7joxnr6
because: The report states a harness-origin prompt-submitted never wakes a parked wait, and the row names activity and wheel records alone.
@row_set n=3 row=decision/test-home-owner-record-sweep cell=except seen=1 fact=F3 detector=none
value: A build's verify stamp dir carries no owner record; another build's stamp is swept only when its lock is taken without waiting.
basis: fct-xueqgrc6
because: The stamp dir is a test-side dir the owner-record sweep never takes, and its own sweep rule is the exception the row does not state.
@row_set n=4 row=decision/fresh-home-per-test cell=rule seen=1 fact=F4 detector=none
value: Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests.
basis: fct-j7joxnr6, fct-777k7lv6
because: A stamped home is now a copy of one shared verify stamp, and the row's no-sharing rule must say the copy shares no file with the stamp or another home.
@row_set n=5 row=decision/verify-driven-tests-20s-kill cell=why seen=1 detector=none
value: Ratified; sized from the measured verify floor plus a 3x runner tail; one verify stamp taken per build keeps each stamped-home test under it.
basis: fct-j7joxnr6, fct-tqy4gmxm
because: The report moves the kill's rationale to the stamp taken once per build, the 20 s kill itself unchanged.
@row_set n=6 row=contract/fake-agent-cli/script cell=promise seen=1 detector=none
value: The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse, Stop, Notification and UserPromptSubmit steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
basis: fct-tsb4lz7x
because: The chunk's fake-script drives Notification and UserPromptSubmit steps, which the row's list of step kinds leaves out.
@row_add n=1 row=decision/one-verify-stamp-per-build area=test-data-bootstrap fact=F2 detector=none
name: One verify stamp per build
rule: Stamped test homes copy one viola verify stamp taken per pair of built binaries, kept under target/e2e-home keyed by the binaries' hashes and taken under an exclusive lock.
why: Taking the stamp once keeps every stamped-home test under its unchanged per-test kill.
guard: stamped_home_reuses_one_stamp_for_every_home
basis: fct-xueqgrc6, fct-tqy4gmxm
because: The chunk made stamped_home copy one stamp per build under a lock, a convention no row of test-data-bootstrap states.
