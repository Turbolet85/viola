@row_set n=1 row=decision/planted-hygiene-violations cell=rule seen=1 fact=F3 detector=none
value: Every hygiene violation class is proven red by a planted input; a planted unknown-word case names a word outside the closed set, and when the set widens every planted case naming a new word moves to a word still outside it.
basis: fct-iodqdgsl, fct-g5icygyl
because: Widening the Hyprland request enum turned the planted unknown-request case valid until it was re-pointed to a word still outside the set, so the planted-case rule must say that a planted unknown word follows the set it tests.
@row_add n=1 row=decision/hypr-tap-moves-only-its-own-window area=test-data-fixtures fact=F1 detector=none
name: Recorded window move only on the tap's own terminal
rule: A recorded dispatch that moves a window acts only on a terminal the recording tap itself started, ends that terminal and confirms its window is gone; it never places a host window it did not open.
guard: hypr_tap_record_move_moves_the_window_of_its_own_terminal_and_ends_it
state: provisional
basis: fct-y4jmpjio, fct-xiw6gmo2, fct-qrunt2s3
because: The chunk adds the first recorded Hyprland dispatch that moves a window, recorded only against the tap's own terminal under the directive's host rule, and no test-plan row says what a moving recording may act on.
@row_add n=2 row=decision/hyprland-reply-keep-list-per-request area=7-test-data-fixtures fact=F2 detector=none
name: Hyprland monitors and dispatch recordings keep no host text
rule: A recorded Hyprland monitors reply keeps numbers and booleans alone and no string, and a recorded dispatch reply is kept only as its success text; the fixture hygiene walk rejects a planted monitors string and a planted non-success dispatch reply.
guard: planted_hyprland_monitors_name_is_rejected, planted_hyprland_move_reply_other_than_success_is_rejected
basis: fct-ul56qpfc, fct-y4jmpjio, fct-mfyq2wsc
because: The chunk adds the monitors query and the move dispatch to the recorded set with their keep-list entries and planted hygiene cases, and the standing walk row names the per-request keep-list only in general and has no room left under its bound.
