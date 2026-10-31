# Classified — design · surface.b1

## Answers
surface#1.1 · no · attention token for the board's DIALOG word; board row colour, which the chunk does not touch (the palette is a 0.2.0 input)
surface#1.2 · no · stale row dim on the board; board row colour the chunk does not touch
surface#1.3 · no · callsign bold on the board's NAME column; board row colour the chunk does not touch
surface#1.4 · no · reset SGR after styled spans; the chunk adds no styled span
surface#1.5 · no · standing S2 · each new refusal or hint line carries no colour, glyph or SGR
surface#1.6 · no · standing S2 · each new refusal or hint line carries no colour, glyph or SGR
surface#2.1 · yes · the fixed `unable: <text>` form with its `hint:` line directly after, for run, revive, verify, list, link/unlink and the instance-state verbs' refusals the chunk moves
surface#2.2 · yes · every new closed cause or detail code must be a code word (1 to 64 lower-case, digits, hyphens, lower-case first) to survive client screening
surface#2.3 · no · the peer-reply reading in reply_of and of_reply is a unit the scope does not name; the CLI's own causes need only 2.2
surface#2.4 · yes · a new code in refusal position outside the closed refusal list exits 14 as unknown, which would break the table's exit-code assertion
surface#3.1 · no · width truncation of unwrapped NAME in board tables; board output the chunk does not touch
surface#4.1 · yes · the join by child pid or session id that defines an unwrapped session, which the unwrapped-name cause and "an unwrapped session never a target" must test; the board's columns, width and padding are not needed
surface#4.2 · yes · list's whole-board strict-modes refusal in the fixed home or instance pair, naming the instance, exit 21
surface#5.1 · yes · which pair list's whole-board refusal prints depends on whether an instance exists; the scope names that refusal
surface#6.1 · yes · the send-pattern hints per reason and cause, including instance-unreachable's per-cause hints, the unwrapped-name hint, and the run and revive exit-1 hints; no hint names viola release
surface#7.1 · yes · the server-verify hint for send, wait, last, answer, pause, release, link and unlink, its exact line and no pid, path or endpoint
surface#8.1 · yes · on macOS the server check passes with no comparison, so the server-verify case of the table cannot print its hint there
surface#9.1 · no · release --budget success line; success lines are not needed
surface#10.1 · no · link and unlink success line; success lines are not needed
surface#10.2 · no · the since field of the link success line; success output the chunk does not change
surface#10.3 · no · the already-linked success form of link; success output the chunk does not change
surface#11.1 · yes · verify's stopped-run exit-1 refusal (fixed pair, exit 1) is an exit-1 cause the chunk gives a code; its step lines and summary are not needed
surface#12.1 · yes · verify's refused home tree: exit 1, no step line or summary, the home-state pair, and a refused stamps.json update printing that pair
surface#13.1 · yes · on Unix a home under a regular file is a strict-modes refusal: the home-state pair for verify, the instance-state pair for the frame verbs (exit 21)
surface#14.1 · yes · revive's refusals after a passed preflight: fixed line and hint per cause, and the one stdout document under --json with stderr empty and no hint
