@row_set n=1 row=decision/adopt-proven-live cell=rule seen=1 fact=F3 detector=none
value: Real adopt of a plain claude session, the resume of a real ended session, the refusal of a real live one and the real CLI's end for an id it cannot resume are proven only by the gated live reading R5; the fake-agent rehearsal of its script is never a live reading.
basis: fct-b3bx62ut
because: Adopt's not-resumed end is proven over the fake agent alone, so the real CLI's end for an id it cannot resume joins what only R5 reads.
@row_set n=2 row=contract/fake-agent-events/session-start cell=promise seen=2 detector=none
value: After start_receipts the fake agent fires SessionStart (variant default) once, silent without a registered hook or a fixture, like every fire; under --screens with an untrusted cwd only after Enter on the trust row, which draws the ready screen; under --resume-absent with --resume it fires none.
basis: fct-ftx443bh
because: Under the new --resume-absent option given with --resume the fake fires no SessionStart, so the row's «fires SessionStart once» no longer holds for every start.
@row_add n=1 row=contract/fake-agent-cli/resume-absent area=7-test-data-fixtures fact=F1 detector=none
boundary: viola-fake-agent command line
item: --resume-absent
promise: The argv-only --resume-absent, given with --resume, makes the fake agent fire SessionEnd with no SessionStart, close its hooks, ignore SIGHUP and exit 1, at both launch sites: a trusted start and the trust dialog's accept.
basis: fct-p367ntgk
because: The fake agent's argv-only option list gains --resume-absent, and the command-line contract holds one row an option, none for it.
@row_add n=2 row=contract/adopt-exit-1/not-resumed area=6-e2e-test-strategy fact=F2 detector=none
boundary: adopt exit 1 causes
item: not-resumed
promise: A child that ended without its session starting makes viola adopt write its unable line naming the instance and its own hint on stderr, or under --json {"v":1,"error":"not-resumed","detail":null}, once the child released the terminal, and log a self process-exit detail not-resumed after the child's.
basis: fct-x4brbvla
because: viola adopt gains a third own exit-1 refusal, not-resumed, with its own pair, document and process-exit detail, and no row of test-plan names it.
@row_add n=3 row=decision/sessionless-start-runs-piped area=integration-test-strategy fact=F5 detector=none
name: Sessionless-ending starts run piped
rule: A test whose start must end before a session starts runs the verb piped outside a terminal, never through the readiness-waiting test wrapper, and reads stdout and stderr apart.
why: The readiness-waiting test wrapper waits for a readiness such a start never reaches.
basis: fct-cgxaz3tn
because: The not-resumed cases set how a start that ends before its session starts is tested, a convention later cases of the same kind must follow.
@row_none n=1 fact=F4 type=4
what: How the R5 rig's script is built (its r5c rehearse-only arm and r5a's start= field); the script holds it, and adopt-proven-live already rules that its rehearsal is never a live reading.
