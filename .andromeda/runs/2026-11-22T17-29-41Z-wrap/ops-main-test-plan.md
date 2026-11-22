@row_set n=1 row=decision/human-never-blocked cell=except seen=1 detector=none
value: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.
basis: adaptation:4
because: A start with verify_new_cli on runs viola verify on null stdin before any agent child exists, so the human-never-blocked decision takes that window as its named exception, in the words the a11y row carries.
@row_set n=2 row=contract/fake-agent-cli/argv-only-options cell=promise seen=2 detector=none
value: The fake agent's modes and options are taken from argv and from a fake-args file beside its exe, one argument per line within MAX_FRAME, put ahead of argv so argv's own value wins, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
basis: adaptation:4
because: The built fake agent reads a fake-args file beside its executable ahead of argv, argv's own value winning, so options no longer come from argv alone.
@row_set n=3 row=decision/fake-agent-argv-only cell=except seen=2 detector=none
value: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe; a fake-args file beside its exe goes ahead of argv, argv winning.
basis: adaptation:4
because: The fake-args file the built fake agent reads ahead of argv is a new named exception to argv-only configuration, added beside the hyprctl files with their words kept whole.
@row_set n=4 row=decision/readiness-gate-tested-by-refusals cell=rule seen=1 detector=none
value: The readiness gate of viola run is tested through its refusal outputs and by one delivered case: a send on a verified CLI past a modal label quoted in the transcript over an idle input box.
basis: adaptation:5
because: The gate's test strategy holds a delivered case past a quoted modal label beside the refusals, so the row stating refusals alone is incomplete.
@row_set n=5 row=decision/screen-signatures-only cell=rule seen=2 detector=none
value: The screen is evaluated only for compiled signatures as the readiness gate's verdict: the input-box and modal literals, and the compiled shapes over them (a modal label row's option shape; the box's rows below it); content beyond the compiled literals is never asserted.
basis: adaptation:5
because: The gate's verdict reads the standing-modal predicate, whose compiled shapes are a label row's option shape and the box's rows below it, and the row now carries that parenthetical in full.
@row_set n=6 row=decision/no-child-screen-assertions cell=except seen=4 detector=none
value: The forced vt100 panic case byte-searches the outer terminal; the test-only live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never text.
basis: adaptation:5
because: The except's members were shortened to fit the rig's compiled-shapes reading; the bound cannot hold that reading with every member whole, so the except returns to its full members and the reading stands in its own row.
@row_set n=7 row=decision/never-parse-rendered-child-screen cell=except seen=4 detector=none
value: The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
basis: adaptation:5
because: The except's members were shortened to fit the rig's compiled-shapes reading; the bound cannot hold that reading with every member whole, so the except returns to its full members and the reading stands in its own row.
@row_set n=8 row=decision/discipline-at-every-layer cell=except seen=4 detector=none
value: The test-only live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never text, and passes a read trust dialog under its key grant.
basis: adaptation:5
because: The except's members were shortened to fit the rig's compiled-shapes reading; the bound cannot hold that reading with every member whole, so the except returns to its full members and the reading stands in its own row.
@row_set n=9 row=decision/no-screen-parsing cell=except seen=4 detector=none
value: The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
basis: adaptation:5
because: The except's members were shortened to fit the rig's compiled-shapes reading; the bound cannot hold that reading with every member whole, so the except returns to its full members and the reading stands in its own row.
@row_add n=1 row=decision/run-start-verify-and-trust-after-bind area=test-scope-summary detector=none
name: viola run's verify and trust steps after the bind
rule: Between its exclusive endpoint bind and its first snapshot, a viola run start runs any automatic verify and then, on a verified final verdict, writes an untrusted spawn folder's trust record.
basis: adaptation:4
because: The built start order runs the automatic verify after the bind and the trust record after it on the final verdict, a reading the start-order row's bound cannot hold with its words whole, so it stands in its own row.
@row_add n=2 row=decision/live-rig-reads-compiled-shapes area=1-test-scope-summary detector=none
name: Live rig's screen read takes the compiled shapes
rule: The test-only live rig's closed-word read of its child's screen also reads the compiled shapes over the compiled literals, a modal label row's option shape and the box's rows below it, never text.
basis: adaptation:5
because: The rig's box read runs the standing-modal predicate, a reading the four screen-parsing excepts cannot hold with their members whole, so it stands in its own row.
