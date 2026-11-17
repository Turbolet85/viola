@row_set n=1 row=decision/claude-shapes-in-one-crate cell=except seen=1 detector=D-arch-decisions
value: The test-only live rig in viola-e2e reads the key names and the source word of its own captured UserPromptSubmit payload, never a value.
basis: fct-wy25afcs
because: The rig's hidden capture verb now reads a Claude hook payload's key names and source in viola-e2e, so the one-crate rule needs its test-only exception named.
@row_set n=2 row=decision/claude-first-no-adapter-trait cell=except seen=1 detector=D-arch-decisions
value: The test-only live rig in viola-e2e reads the key names and the source word of its own captured UserPromptSubmit payload, never a value.
basis: fct-wy25afcs
because: Hook JSON is now parsed outside viola-agent-claude by the test-only rig's capture verb, so the rule's exception must be named.
@row_add n=1 row=decision/live-rig-capture-key-names-only area=occupied-resources fact=F1 detector=none
name: Live rig hook capture keeps key names only
rule: The test-only live rig's hidden capture verb reads one UserPromptSubmit payload bounded by MAX_FRAME and appends one private line of patterned key names, a count of the others, a closed source word and a time, never a value; an unreadable or over-bound payload is written as unread.
why: A ratified widening: the rig reads its own hook payload while no value of it is kept.
basis: fct-wy25afcs
because: The chunk lands the rig's own UserPromptSubmit capture as a ratified widening with a closed, value-free line shape, and no row holds it.
@row_add n=2 row=decision/live-rig-user-config-mark-witness area=occupied-resources fact=F2 detector=none
name: Live rig user-config mark witness
rule: The test-only live rig witnesses its isolation from the user's ~/.claude.json by marks: on a moved stamp it reads the file bounded by MAX_FRAME, follows no link out of the home, seeks two rig-made needles and keeps one closed word: unchanged, absent, changed-by-others, changed-by-rig or unread.
why: The host's own Claude sessions rewrite that file about once a minute, so a whole-file stamp cannot witness one process's isolation.
basis: fct-qf2qhfdi
because: The stamp-only witness was disproved by measurement and replaced by the ratified mark witness, and no architecture row holds it.
@row_add n=3 row=name/other/viola-live-rig-capture area=occupied-resources fact=F3 detector=D-arch-resources
class: other
name: viola-live-rig capture
basis: fct-sqi7nisv
because: The test-only viola-live-rig binary now occupies a hidden capture verb, an occupied name the register does not yet hold.
