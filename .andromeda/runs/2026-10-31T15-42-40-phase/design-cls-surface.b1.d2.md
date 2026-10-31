# Classified — design · surface.b1

## Answers
surface#1.1 · no · the DIALOG amber token sits on the board's wheel column; the chunk edits only viola list's refusal, and its refusal lines carry no SGR
surface#1.2 · no · the stale dim row is board rendering; the chunk touches no board row
surface#1.3 · no · the bold NAME callsign is a board column value; the board table is not touched
surface#1.4 · no · the reset byte follows styled spans, and the chunk adds no styled span
surface#1.5 · no · standing S2 · each new refusal or hint line carries no colour, glyph or SGR
surface#1.6 · no · standing S2 · each new refusal or hint line carries no colour, glyph or SGR
surface#2.1 · yes · the fixed `unable: <text>` form and one `hint:` line directly after each refusal bind the moved exit-1 and exit-21 refusals of run, revive, verify, list, link, unlink, send, wait, last, answer, pause and release
surface#2.2 · yes · new codes (per-cause detail codes, process-exit detail values) must be code words, since reply_of and of_reply screen every peer refusal and detail through is_code_word
surface#2.3 · yes · a new refusal or detail code that fails is_code_word reads unknown or no detail on the peer read, so the chunk's new codes must pass it
surface#2.4 · yes · each new refusal code must sit in the closed refusal list, or it stays visible but exits 14 as unknown does
surface#3.1 · no · board table truncation; the task's Not-needed names the board's width truncation and the chunk touches only list's refusal
surface#4.1 · no · the board's rows, WHEEL width and unwrapped-session rows; the task's Not-needed excludes the board's columns and ordering, and the chunk touches only list's refusal
surface#4.2 · yes · list's whole-board exit-21 refusal in the fixed unable/hint pair, and the pair that link, unlink, send, wait, last, answer, pause, release, run and revive print naming their instance or home
surface#5.1 · yes · the strict-modes scan's rule decides which pair list's exit-21 refusal prints (home pair when no instance exists, instance pair when one does), so the new list case must assert it
surface#6.1 · yes · the per-cause instance-unreachable hints (not-running, unwrapped name, server-verify), the run and revive start refusal lines and hints, and the no-`viola release` rule bind the hint table and hints.rs
surface#7.1 · yes · the server-verify hint is an instance-unreachable cause for send, wait, last, answer, pause, release, link and unlink; its table case asserts the byte-exact line with no pid, path or endpoint
surface#8.1 · yes · on macOS the server-verify hint cannot print, so the server-verify table case must carry this recorded gap rather than assert the line there
surface#9.1 · no · release's --budget success line; the chunk moves only release's exit-21 refusal, and the task's Not-needed excludes success lines
surface#10.1 · no · link and unlink success line on stdout; the task's Not-needed excludes success lines, and the chunk moves only their refusals
surface#10.2 · no · the since of link's success line; success output, excluded by the task's Not-needed
surface#10.3 · no · the already-linked success form exits 0 and is not a cause of exit 1 or 21
surface#11.1 · no · verify's step counter, step lines and summary are excluded by the task's Not-needed; its Ctrl-C stop pair is another unit of verify.rs that the change does not edit, so it is a re-check
surface#12.1 · yes · verify's refused home tree prints no step line and no summary, only the home-state pair, exit 1; a refused stamps.json update prints the same pair, which the verify.rs edit and its tests assert
surface#13.1 · yes · on Unix a home under a regular file is a strict-modes refusal, so verify's home-state pair (exit 1) and the frame verbs' instance-state pair (exit 21) are table cases; only the Windows NotFound path stays absent
surface#14.1 · yes · revive's refusal lines and hints, its --json strict-modes-failed, already-live, no-session, session-live and cwd-missing documents with no hint, and the post-preflight start refusal's document, which this chunk closes
