@row_set n=1 row=decision/key-after-resize-waits-for-size cell=except seen=1 detector=D-tests-coverage
value: The live rig's KR arm types its key 0, 20 or 100 ms after a resize of its own pty, in its rehearsals and its live starts, since a key typed right after a resize is what it measures.
basis: fct-ynvxadkv (the KR arm, kr_delay and KR_DELAYS_MS 0, 20, 100); fct-qulhmgcs (test-plan decision/key-after-resize-waits-for-size: the except names the KR rehearsal); fct-st4ypkii
because: The KR rehearsals and live starts now write a key right after a resize without waiting for the child's size report, so the rule needs that named exception to stay true.
@row_add n=1 row=decision/kr-after-key-settle area=6-e2e-test-strategy fact=F2 detector=D-tests-coverage
name: Live rig KR arm settles only after its key
rule: The live rig's KR arm counts its after-key quiet window only from a byte drained after the key, never from a byte before it; every other arm keeps the shared settle unchanged.
why: The mend is held to the KR arm alone so the HB arm and every earlier arm keep their readings.
guard: a_byte_drained_before_the_key_never_settles_it
basis: fct-nlkllz3t (the operator's word: mend the rig's settle, a byte drained after the key; HB and the earlier arms must not change their readings; a KR-only settle_since, Session::settle untouched); fct-ynvxadkv; fct-kbiwymov
because: The chunk adds a KR-only after-key settle beside the unchanged shared settle, a decision later rig arms must respect and no row states.
@row_add n=2 row=decision/kr-late-redraw-not-dirty-box area=6-e2e-test-strategy fact=F3 detector=D-tests-coverage
name: Live rig KR arm tells a late redraw from a dirty box
rule: The live rig's KR arm re-reads a box that is not clean: a box clean only on its re-read keeps the round and is counted late-redraw, and a box still dirty on its re-read stops the start, recorded unverified.
guard: the_kr_box_word_tells_a_late_redraw_from_a_dirty_box
basis: fct-nlkllz3t (tell a dirty box from a late redraw); fct-ynvxadkv (kr_box_word clean · late-redraw · dirty, KrTally.late_redraws); fct-kk7cxiqt (the late rehearsal late-redraw: 1, kept; the withheld dirty-box: 3, unverified, exit 1)
because: The chunk makes the KR arm's round verdict tell a late redraw from a dirty box and stop a start on a still-dirty box, a reading rule no row states.
@row_add n=3 row=decision/refusal-rehearsal-reads-own-effects area=6-e2e-test-strategy fact=F4 detector=D-tests-coverage
name: Refusal rehearsal reads the refusal's own effects
rule: A rig rehearsal of a refusal that prints none of the shared teardown words reads the refusal's own effects (no ledger, an empty receipt, no process the rig started) instead of the shared teardown check.
why: The shared teardown check reads summary words a usage refusal never prints, so it would read red on a correct refusal.
guard: rig_rehearsal_kr_seams_refuse_a_live_start_and_another_arm
basis: fct-pxmu6daf (the seam case reads nothing spawned by its own checks, since assert_torn_down would read red on a correct refusal); fct-m3iencpr
because: The chunk sets a convention for rehearsing a refusal that the shared teardown check cannot read, and no row of the rig states it.
@row_none n=1 fact=F1 type=0
what: A live measurement reading of the real CLI on two versions (12 of 12 keys kept after a resize); the evidence file and the gotchas and viola-pty leaves hold it, not a decision of the test plan.
