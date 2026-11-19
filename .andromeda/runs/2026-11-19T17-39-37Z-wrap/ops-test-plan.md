@row_set n=1 row=decision/hook-payloads-recorded-by-verify cell=except seen=3 fact=F7 detector=none
value: StopFailure has no recorded payload, ledger row or verify probe, settled on one live StopFailure reading that claims nothing for other error words or versions; its tests use synthetic payloads.
basis: fct-vj6tsu22, fct-xq3ufz23, fct-wbyo6j4o
because: The StopFailure exception is now settled as outcome B on the live reading of one connection-lost payload, no longer provisional, and the row's except must say what it rests on and what it does not claim.
@row_set n=2 row=decision/standing-live-rig cell=except seen=1 detector=none
value: The SF arm's rehearsals run unwrapped in CI; its contained rehearsal is a local entry that skips by its printed sandbox probe where the sandbox or the runtime dir cannot serve it.
basis: fct-ooesrp6l, fct-h5srvyh4, fct-zrtvvb6q
because: The rig gained the SF arm whose contained rehearsal is local only, an exception to its rehearsals running in CI, which the plan's expected amendment owes to this row.
@row_set n=3 row=decision/no-child-screen-assertions cell=except seen=2 detector=none
value: The forced vt100 panic case byte-searches the outer terminal; the test-only live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never text.
basis: fct-wlojdipp, fct-ooesrp6l
because: The live rig now also reads the box row's cell attributes as closed words, never text, so an except naming compiled literals alone is incomplete.
@row_set n=4 row=decision/never-parse-rendered-child-screen cell=except seen=2 detector=none
value: The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
basis: fct-wlojdipp, fct-ooesrp6l
because: The live rig now also reads the box row's cell attributes as closed words, never text, so an except naming compiled literals alone is incomplete.
@row_set n=5 row=decision/no-screen-parsing cell=except seen=2 detector=none
value: The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
basis: fct-wlojdipp, fct-ooesrp6l
because: The live rig now also reads the box row's cell attributes as closed words, never text, so an except naming compiled literals alone is incomplete.
@row_set n=6 row=decision/discipline-at-every-layer cell=except seen=2 detector=none
value: The test-only live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never text, and passes a read trust dialog under its key grant.
basis: fct-wlojdipp, fct-ooesrp6l
because: The live rig now also reads the box row's cell attributes as closed words, never text, so an except naming compiled literals alone is incomplete.
@row_add n=1 row=decision/live-rig-sf-arm-sandboxed area=6-e2e-test-strategy fact=F3 detector=none
name: Live rig StopFailure arm sandboxed
rule: The live rig's SF arm runs only inside a network-closed bwrap sandbox, a test-only host dependency of the rig alone; without the sandbox the rig refuses sandbox, exit 3, uncounted, before any credentials read or ledger line.
guard: rig_rehearsal_sf_refuses_contain_without_the_sandbox_before_its_ledger_line
basis: fct-wlojdipp, fct-icy56rsj, fct-ooesrp6l
because: The rig gained a sandboxed arm and its refusal before the ledger line, a decision no test-plan row states.
@row_add n=2 row=decision/live-rig-stop-failure-capture-closed area=6-e2e-test-strategy fact=F5 detector=none
name: Live rig StopFailure capture closed
rule: The live rig's StopFailure capture keeps key names, the error word only from the CLI's closed list of 13 (any other value as other) and the byte lengths of two text fields, never a text.
why: Ratified as the one widening of the rig's capture, in exactly that width.
guard: a_stop_failure_line_keeps_a_known_word_and_lengths_only
basis: fct-wlojdipp, fct-edja7n2n, fct-usi3wfxj
because: The rig's capture now keeps a StopFailure line in a ratified closed width, a decision no test-plan row states.
@row_add n=3 row=contract/fake-agent-cli/turn-fail area=7-test-data-fixtures fact=F1 detector=none
boundary: viola-fake-agent command line
item: --turn-fail
promise: Beside --turn-stop, --turn-fail <variant> ends the turn with the recorded set's StopFailure.<variant> payload in place of Stop.default.
basis: fct-wlojdipp
because: The fake agent's command line gained a new option, an item of its contract no row names.
@row_add n=4 row=contract/fake-agent-cli/settings-hooks area=7-test-data-fixtures fact=F2 detector=none
boundary: viola-fake-agent command line
item: --settings-hooks
promise: --settings-hooks <file> names a Claude settings file whose hooks run beside the plugin's; a command string with no args is split on spaces and never run through a shell.
basis: fct-wlojdipp, fct-nlima2au
because: The fake agent's command line gained a new option, an item of its contract no row names.
@row_none n=1 fact=F4 type=4
relied: The contained SF rehearsal skips by its printed sandbox probe where the runtime dir cannot cross the sandbox; the code holds the no-runtime-dir refusal.
@row_none n=2 fact=F6 type=4
relied: A CLI version's kept record set is never overwritten and later versions are kept beside it; the code and its keep-version control hold it.
