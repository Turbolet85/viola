@row_add n=1 row=decision/move-focus-effect-not-asserted-on-fakes area=e2e-test-strategy fact=F1 detector=none
name: No fake-backed assertion of a move's focus effect
rule: No fake-backed test may assert that an opened window's move leaves focus or the active workspace unchanged: on Hyprland the non-silent move takes the active workspace with the moved window, and the recorded move fixture holds no active-workspace effect.
why: The live reading saw the active workspace follow the moved window, while the fake replays only the move's arguments and its ok reply.
basis: fct-s4kb5bjz
because: The gated wrap read that the built move takes the active workspace with it and that its fixture records nothing of that, so no fake-backed test may promise the opposite.
@row_add n=2 row=decision/gated-live-reading-operator-away area=e2e-test-strategy fact=F2 detector=none
name: Gated live reading with the operator away
rule: A gated live reading may run with the operator away only on his own typed word, at his host unlocked, on the workspaces he names, fired once; the reading clears its gated block, but its criterion reaches verified only once his answers land.
basis: fct-gmjojd6b
because: R3 ran with the operator away on his typed word, on a named workspace, once, clearing its block while the criterion waited on his answers, and no row states when a gated reading may run unattended.
