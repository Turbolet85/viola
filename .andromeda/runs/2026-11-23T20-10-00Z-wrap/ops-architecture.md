@row_set n=1 row=decision/hyprctl-direct-spawn cell=rule seen=1 detector=D-arch-decisions
value: hyprctl is spawned directly from a caller-resolved path, with no shell and no PATH lookup, stdin null and a closed argv; a dispatch is one Lua element, hl.dsp.focus or hl.dsp.window.move, whose only inserted values are a closed window address or an i32 workspace id, never reply text.
basis: fct-y4jmpjio · fct-xaiu57oy
because: The seam's closed set gains a move-to-workspace dispatch built as the one-element Lua form hl.dsp.window.move on a closed address and an i32, measured live on Hyprland 0.56.2, so a rule naming hl.dsp.focus as the only dispatch form no longer holds.
@row_add n=1 row=contract/hyprland-seam-replies/monitors area=standard-contracts fact=F1 detector=D-arch-resources
boundary: Hyprland seam replies
item: monitors
promise: The compositor's monitors reply is read into a closed Monitor record of numbers and booleans only: a required id, and optional width, height, x, y, focused and active workspace; every text of the reply is dropped at the parse.
basis: fct-y4jmpjio
because: The chunk lands a monitors query whose reply the seam hands outward only as a closed record with no text field, a promise at the seam's boundary that no standing row states.
