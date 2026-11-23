@row_set n=1 row=decision/hyprctl-read-deadline cell=state seen=1 detector=D-security-input
value: standing
basis: fct-tbmaklu4, fct-vqtx5iug, fct-v4hfzas6
because: The hyprctl read deadline moved from provisional to settled at its unchanged 5 s on the operator's own word, so the row's provisional mark ends.
@row_add n=1 row=decision/hyprland-move-one-argv-element area=input-validation fact=F1 detector=D-security-input
name: Hyprland move dispatch as one closed argv element
rule: The Hyprland seam's move-to-workspace dispatch crosses to the directly spawned hyprctl child as one argv element into which only a checked window address and an i32 workspace enter, never reply or user text.
state: provisional
basis: fct-y4jmpjio, fct-xaiu57oy, fct-v4hfzas6
because: The chunk adds a dispatch whose argv carries values to hyprctl, and the closed shape of that crossing, resting on the overseer's delegated word, binds every later caller of the seam.
@row_add n=2 row=decision/hyprland-fixture-numbers-kept area=data-protection fact=F2 detector=none
name: Recorded Hyprland replies keep numbers as host facts
rule: A recorded Hyprland reply keeps its numbers and booleans (monitor ids, sizes, positions, scale, focus, active workspace) and no string; the kept numbers are accepted as host facts, the recorder's allowlist governing strings only.
state: provisional
basis: fct-wixyuc6s, fct-fgfd3vxe, fct-v4hfzas6
because: The new recorded monitors reply commits host numbers that no standing row allows or forbids, and their acceptance, on the delegate's R3 allowance, binds later recordings.
@row_add n=3 row=decision/hyprland-recorder-moves-own-window area=authentication-authorization fact=F3 detector=none
name: Hyprland recorder moves only its own terminal's window
rule: The test-only Hyprland recorder's move acts only on the window of a terminal it spawned itself, ends and reaps that terminal after the move, and refuses with no-terminal-window when no window of its own terminal appears.
basis: fct-y4jmpjio, fct-xiw6gmo2, fct-l4tic6ef
because: The recorder gains a verb that acts on a live desktop window, and its bound to a window of its own making is the rule that keeps it off the user's windows.
