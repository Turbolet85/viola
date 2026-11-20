@row_set n=1 row=decision/no-screen-content-parsing cell=except seen=5 fact=F3 detector=none
value: the test-only rig reads closed words, never text: trust selector place, box as empty, holds-text or absent, modal dialogs, box-row cell attributes by count before any send or after a turn end or send
basis: fct-o2xu5jdq
because: The live rig's new hb arm reads the dim hint cells against the plain typed cells of a fresh box before any send, by attribute and count only, so the exception must name that reading too.
@row_add n=1 row=decision/told-wake-keeps-the-wheel area=keyboard-test-harness fact=F1 detector=none
name: Told wake keeps the driver wheel
rule: A keyless prompt-submitted the driver did not claim is a told wake, relabelled origin harness, no wheel move, only when no human key was typed since the last human-filed prompt, the last turn end counted at least one scheduled wake, and the CLI is verified; otherwise it takes the wheel.
why: so a CLI-scheduled wake's prompt is told from a typed one and the driver keeps its sends and dialogs
guard: path5_a_fired_wake_keeps_the_driver_wheel_its_next_send_and_its_dialog
basis: fct-yqdgc3b5
because: The wheel no longer gives every unclaimed keyless prompt to the human: a told wake under three conditions keeps the driver wheel, a rule the human-takes-the-wheel path now depends on.
@row_add n=2 row=decision/typed-mark-clears-at-human-prompt area=keyboard-test-harness fact=F2 detector=none
name: Typed mark lasts until a human-filed prompt
rule: The typed mark a human key sets clears only at the next human-filed prompt, never at a turn end or a release, so a message queued during a turn reads human and a draft typed then erased makes the next wake take the wheel once.
except: a prompt submitted by the mouse alone, with no key, is the one named false positive, unmeasured
guard: tui_a_message_queued_during_a_turn_reads_human_after_that_turn_ends
basis: fct-wncy4gmx
because: The wrap ruled the mark's life narrower than first shown, with one named unmeasured false positive, and keyboard boundary work must respect that life.
