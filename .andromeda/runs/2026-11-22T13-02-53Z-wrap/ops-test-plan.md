@row_set n=1 row=decision/send-refusal-order cell=rule seen=2 detector=D-tests-count-literals
value: send refuses in the order control-character, empty-text, human-typing, turn-running, input-not-ready; after the gate is ready, human-typing, turn-running and a standing cut mark's paste-cut, before anything is issued or typed; then a paste cut at its bound's paste-cut, then no-prompt-submitted.
basis: fct-vl24ioge (the send refusal order: + two paste-cut rungs); fct-z6owtgsx (the mark's rung after the second reads and before the link; the cut after the paste, with cursor)
because: The chunk adds two paste-cut rungs to the send refusal order, so the row's enumerated order must name them where they stand.
@row_set n=2 row=contract/driver-send/typed-text cell=promise seen=1 detector=none
value: The typed text is written as one bracketed paste plus Enter, with CR LF and CR turned to LF and trailing CR and LF dropped; a paste cut at its bound closes the bracket with no Enter.
basis: fct-z6owtgsx (a cut writer writes the end marker with no CR; the CR only on Committed); fct-shrbvdzl (test-plan contract/driver-send/typed-text except a cut)
because: A paste cut at the compiled paste bound now closes the bracket and types no Enter, an exception the typed-text promise must carry.
@row_set n=3 row=contract/viola-send/paste-framing cell=promise seen=1 detector=none
value: A sent text is typed as one bracketed paste followed by one CR, its newlines kept inside; a paste cut at its bound closes the bracket and types no CR.
basis: fct-z6owtgsx (the cut closes the bracket, no CR); fct-shrbvdzl (test-plan contract/viola-send/paste-framing except a cut)
because: The framing promise of one paste plus one CR no longer holds for a paste cut at its bound, which ends with the closing marker and no CR.
@row_set n=4 row=contract/send-event-log/send-issued cell=promise seen=1 detector=none
value: An accepted driver send logs send-issued with its cursor and from before one bracketed paste plus Enter is written; a paste cut at its bound writes no Enter, and the send is refused paste-cut with that cursor.
basis: fct-z6owtgsx (send refuses not-delivered / paste-cut WITH the send's cursor; no CR on a cut); fct-shrbvdzl (test-plan contract/send-event-log/send-issued except a cut)
because: After send-issued a cut paste types no Enter and ends the send refused paste-cut with its cursor, so the row's paste-plus-Enter promise takes that exception.
@row_set n=5 row=decision/human-never-blocked cell=rule seen=1 fact=F4 detector=none
value: The human is never blocked: human typing still reaches the agent while automation is refused, and during a driver paste a human key is never refused or dropped, waiting behind at most the in-flight paste chunk of 4096 bytes and the closing marker.
basis: fct-z6owtgsx (HumanInput jobs in arrival order, keys never dropped; CHUNK = 4096; a cut writer stops after its in-flight chunk); fct-shrbvdzl (a human key waits behind at most the in-flight chunk and the closing marker)
because: The paste now runs on a writer thread shared with human input, and the row must state the bound on how long a human key may wait behind a driver paste.
@row_add n=1 row=decision/send-refused-after-a-paste-cut area=unit-test-strategy fact=F1 detector=none
name: Sends refused after a paste cut until a human looks
rule: After a paste cut, every next send is refused not-delivered / paste-cut with no cursor until a human key read is counted or a prompt-submitted of any origin, a session-start or a session-end clears the mark; a release with no key and a turn end leave it.
why: A cut paste leaves the agent without the whole text, so a human must look at the session before the next send.
guard: send_after_a_cut_is_refused_paste_cut_with_nothing_issued
basis: fct-z6owtgsx (Flight::cut_at set to the wheel's key count, its clears, the mark's rung; send_hint paste-cut wording); fct-p2qhyrcq (a release with no key and a turn-ended leave the mark); fct-7lkp72ab
because: The chunk adds an in-memory cut mark that refuses every later send until a human key or a clearing record, a new standing rule future send work must respect.
@row_add n=2 row=contract/fake-agent-cli/stop-reading area=7-test-data-fixtures fact=F2 detector=D-tests-count-literals
boundary: viola-fake-agent command line
item: --stop-reading
promise: The argv-only --stop-reading makes the fake agent never read stdin, parked forever with its terminal left raw, so a paste into it blocks.
basis: fct-z6owtgsx (the fake agent's --stop-reading, argv only, read_nothing parks forever, the terminal stays raw); fct-vegw4vbz; fct-vl24ioge (the fake agent's argv options: + --stop-reading)
because: The fake agent gains an argv option the cross-process suite relies on, and each fake-agent option stands as its own contract row.
@row_add n=3 row=contract/viola-send/paste-cut area=6-e2e-test-strategy fact=F3 detector=none
boundary: viola send
item: paste-cut
promise: A send into a child that stops reading exits 13 not-delivered / paste-cut within the compiled 5 s paste bound, with the send's cursor and no Enter typed, and the next send is refused paste-cut with nothing issued.
basis: fct-h72wgfzz (cut 5 293 ms, next send paste-cut with no send-issued); fct-h5c7drpa (PASTE_BOUND 5 s compiled); fct-z6owtgsx
because: viola send gains a new not-delivered outcome at its CLI boundary, and the e2e area holds one row a viola send outcome.
@row_add n=4 row=contract/viola-send/during-a-blocked-paste area=6-e2e-test-strategy fact=F5 detector=none
boundary: viola send
item: during a blocked paste
promise: A send during a blocked paste is refused turn-running after waiting at most the paste bound, and a hook record during the blocked paste lands within the bound.
basis: fct-h72wgfzz (during-block send 4 001 ms turn-running, hook line 4 000 ms); fct-uidyvbfk (a second send waits on the flight guard for up to PASTE_BOUND, then reads turn-running)
because: A blocked paste now holds a second send and a hook record for at most the compiled bound, a promise at the send boundary no standing row states.
@row_none n=1 fact=F6 type=4
relied: A paste still queued at its deadline is a cut with nothing written, refused paste-cut with the send's cursor, and it sets the cut mark like a mid-write cut.
