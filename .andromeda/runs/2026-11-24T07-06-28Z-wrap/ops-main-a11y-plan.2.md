@row_add n=1 row=decision/open-live-wait-bound area=cognitive-accessibility detector=none
name: viola open's live wait is bounded at 30 s
rule: viola open waits at most 30 s for the instance it started to come live, then reads not-live; the wait holds only the verb's own return, never a key typed into a session, and never moves the wheel.
basis: adaptation:1
because: The operator answered fork F1 with option 1: open's 30 s live wait takes a row of its own; up's reuse of the same wait is no new bound and adds nothing.
@row_add n=2 row=decision/window-wait-bound area=cognitive-accessibility detector=none
name: The window step's wait is bounded at 5 s
rule: A start's window step waits at most 5 s for its own terminal's window to appear in the compositor's client list; the wait falls on viola's own start, never on a key typed into a session, and never moves the wheel.
basis: adaptation:1
because: The operator answered fork F1 with option 1: the wrapper's 5 s window wait takes a row of its own.
@row_add n=3 row=decision/hyprctl-read-deadline area=cognitive-accessibility detector=none
name: Each compositor read is bounded at 5 s
rule: Each compositor read through the Hyprland seam is bounded at 5 s a reply; it falls on viola's own start or verb (the window step, viola open, viola focus), never on a key typed into a session, and never moves the wheel.
basis: adaptation:1
because: The operator answered fork F1 with option 1: the seam's 5 s read deadline takes a row of its own, the human verb viola focus among its bearers.
