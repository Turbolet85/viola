@row_set n=1 row=decision/hyprland-recorder-string-allowlist cell=rule seen=1 detector=D-security-input
value: viola-hypr-tap keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag; none of monitors); any other string becomes the placeholder, a non-JSON reply (the move's) only ok; the shared scrub refuses an unclean set whole.
basis: fct-y4jmpjio · fct-v4hfzas6
because: The recorder now records the monitors query, which keeps no string, and the move dispatch, kept only as its success text, so the allowlist row must name both requests.
@row_add n=1 row=decision/hyprland-recorder-moves-own-window area=authentication-authorization detector=none
name: Hyprland recorder moves only its own terminal's window
rule: The test-only Hyprland recorder moves only the window of a terminal it started itself, then ends and reaps that terminal and waits until its window is gone; it never moves a window it did not open.
guard: hypr_tap_record_move_moves_the_window_of_its_own_terminal_and_ends_it
state: provisional
basis: fct-y4jmpjio · fct-xiw6gmo2
because: The recorder gains a verb that acts on a live desktop window, and its bound to a window of its own making is what keeps it off the user's windows; it rests on the overseer's delegated host rule, so it stands provisional.
