@row_set n=1 row=decision/ci-stamped-home-kill-line cell=rule seen=1 detector=D-tests-count-literals
value: On the ci nextest profile the stamped-home test binaries, viola-e2e's booting harness test binaries among them, are killed after 20 seconds.
basis: fct-wy25afcs, fct-swiaphrz
because: The ci profile's 20 s override now also covers viola-e2e's harness_lifecycle, cli and harness_tied binaries, so the row must name the harness binaries it now kills.
@row_add n=1 row=decision/rig-user-config-mark-witness area=6-e2e-test-strategy fact=F1 detector=none
name: Live rig witnesses the user config by marks
rule: The live rig witnesses its isolation from the user's claude config by marks: on a moved stamp it searches the file for two rig-made needles, its scratch path and session id, and keeps only needles and a word unchanged, absent, changed-by-others, changed-by-rig or unread, never content.
why: A whole-file stamp of a shared user file cannot attribute a change on a host with other writers.
guard: rig_rehearsal
basis: fct-qf2qhfdi, fct-dahwatls, fct-wy25afcs, fct-vqnif4am
because: The stamp-only witness read changed whatever the rig did and was replaced by the ratified mark witness, a decision no row of test-plan holds.
@row_add n=2 row=decision/rig-prompt-capture-value-free area=6-e2e-test-strategy fact=F2 detector=none
name: Live rig captures its prompt hook value-free
rule: The live rig captures its own UserPromptSubmit payload by a hidden capture verb into one private line per payload: key names matching a closed pattern up to 40 chars, a count of other keys, a source word schedule_wakeup, loop_wakeup, other or absent, and ms, or unread words; no value.
guard: rig_rehearsal
basis: fct-wy25afcs, fct-pmfixyq6, fct-rfaae7sn
because: The chunk added the rig's own hook capture as a ratified widening with a closed line shape, and no row of test-plan holds it.
@row_add n=3 row=decision/rig-keys-never-the-stop area=6-e2e-test-strategy fact=F3 detector=none
name: Live rig key grant never names its stop
rule: The standing live rig's keys none names its argv grant into dialogs and box text, never its stop: wake arms stop by Ctrl-C into the rig's own pty, and the imports arm stops by a process-group kill with no key typed.
why: A no-key group-kill stop types less than the key rule allows, never more.
basis: fct-rvgjiddx, fct-rfaae7sn
because: The overseer's directive stated what keys none means and the chunk added the no-key group-kill stop, a rule no row of test-plan holds.
@row_none n=1 fact=F4 type=4
what: How the rig's private settings register a hook path, with no reason stated; the code holds it.
@row_none n=2 fact=F5 type=9
what: A measured reading of upstream claude carried by the route on the entry «A fired scheduled wake keeps the wheel» (v2-36), whose fate is asked at route-resolve.
