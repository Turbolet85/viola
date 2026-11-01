# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the session adds no styling; the new refusal is plain ASCII with no SGR and no token value moved
surface#1.2 · no · the stale row and its dim SGR are unchanged; the new lines carry no styling
surface#1.3 · no · the callsign token is unchanged; the new lines carry no styling
surface#1.4 · no · the reset sequence is unchanged; the new lines carry no styling
surface#1.5 · no · the handoff blue is not used and the new lines carry no styling
surface#1.6 · no · backgrounds are never set, and the session sets none
surface#2.1 · yes · add-to · F1 and F2 join the fixed-message exit-1 refusals of `viola run` and `revive` (`unable: the socket directory for <name> cannot be trusted`, planning wording owed at Epoch 7); F3's frame-verb refusal prints `unable  <name>  instance-unreachable` in the mirror form, not the fixed form
surface#2.2 · no · `socket-dir-refused` is lower-case with hyphens, so it passes `is_code_word`, and `every_code_is_a_code_word` covers it; the screening rule is unchanged
surface#2.3 · no · the screening rule is unchanged, and `socket-dir-refused` is a code word, so it reads as itself and not as `unknown`
surface#2.4 · no · the new word joins the closed list, and the task names no change to the rule for words outside it or to `reason_of`'s exit-14 path
surface#3.1 · no · the session adds no table or width rule; NAME truncation and `viola_pty::host_size()` are not touched
surface#4.1 · no · `viola list`'s layout, captions, BAY line and row order are unchanged (Untouched), and the home rule sits in the refusal and scan entities, not this rendering rule
surface#4.2 · yes · stale · the home's strict-modes check is now the read rule alone on every OS, so a home another user may read refuses the board with or without an instance on Linux, macOS and Windows; the row's home reading is the old check
surface#5.1 · yes · contradict · `viola list` no longer applies two rules to the home, it holds the home to the read rule alone on every OS (F6, provisional on the overseer's word, the founder's own word owed at Epoch 7), and the owner «Unix endpoint and home hardening» is retired, so its CARRY is discharged
surface#6.1 · yes · contradict · the D-20 sentence's owner for the socket-directory cause and macOS's server check is the retired route entry; exit 21's closed causes move from five to six (F3's `socket-dir` hint cause joins the per-cause list), and F1's run start line joins the start refusals
surface#7.1 · yes · add-to · F3 adds `socket-dir` beside `not-running`, `unwrapped-name` and `endpoint-lost` as a hint cause for the eight frame verbs, and the server-verification line also reaches a peer euid mismatch
surface#8.1 · yes · contradict · macOS's server check now compares euid, so the server-verification hint can print there, and the gap's owner (the route entry) is retired, leaving it owned by no route entry
surface#9.1 · no · the session changes no success line; `release --budget`'s output and time form are as built, and F3's refusal prints no line
surface#10.1 · no · this rule states the success line and exit 0; F3's link and unlink refusal prints no result line and is a refusal path, not a change to this line
surface#10.2 · no · the `since` form of the link line is the reply's and is untouched by the session
surface#10.3 · no · the already-linked and unlink-not-held forms are success outputs, unchanged by the session
surface#11.1 · no · verify's step counter and its refusals moved in no check or line (Untouched), and the socket check reaches no verify step
surface#12.1 · no · verify's home-tree check and its pair are unchanged (Untouched: no verify check or line moved)
surface#13.1 · yes · stale · the frame verbs now run the socket-directory check before any frame, so the refusal a home under a regular file gets on Unix (the instance-state pair) can move, and the report does not show that it holds
surface#14.1 · yes · add-to · F1 and F4 join revive's refusals: an untrusted socket directory gets the fixed-message pair and its hint at exit 1, and under `--json` one document with the code `socket-dir-refused`, no hint, stderr empty
