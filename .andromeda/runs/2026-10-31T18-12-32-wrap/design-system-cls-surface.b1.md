# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the session added no colour and the colour tokens sit in the styling module, which the change does not reach
surface#1.2 · no · the stale row's dim token is colour in the styling module, which the change does not reach; the session added no colour
surface#1.3 · no · the callsign bold token is colour in the styling module, which the change does not reach
surface#1.4 · no · the reset token is colour in the styling module, which the change does not reach
surface#1.5 · no · the handoff blue is unused colour in the styling module, which the change does not reach
surface#1.6 · no · backgrounds are never set, and the styling module is not in the change
surface#2.1 · yes · add-to · F11 and F12: under --json the catch site prints the internal-error document on stdout with stderr empty, and it counts --json only before `--`, so the stderr error line stands only without --json
surface#2.2 · yes · stale · reply_of, of_reply and is_code_word sit in the reached files client.rs, result.rs and viola-core lib.rs, and the report does not clear the code-word screen for the new cause words
surface#2.3 · yes · stale · the peer-reply reads in the reached files client.rs and result.rs now meet the new detail code words, and the report does not clear the null and unknown readings
surface#2.4 · no · the report says no RefusalReason and no exit code moved, so the closed refusal list and reason_of still hold
surface#3.1 · yes · stale · the board's human table in the reached file src/cmd/list.rs is reached whole, and the report does not clear its NAME truncation and width rule
surface#4.1 · yes · add-to · F2: the As-built paragraph's board refusal now reads strict-modes-failed under --json, and the human pair stays unmoved
surface#4.2 · yes · add-to · F2: list's exit-21 strict-modes refusal reads strict-modes-failed in the --json document, and the fixed unable and hint pair stays unmoved
surface#5.1 · no · the report keeps the human pair unmoved and the scan's two rules and instance-dependent pair choice unchanged; the session adds only the --json code at 4.2
surface#6.1 · yes · contradict · the exit-21 document now carries its cause in detail (F1, F3, F4, F7), so the detail-null line and the no-code-yet line are false; the endpoint-lost hint is new (F5) and a stale holder logs stale-heartbeat (F6)
surface#7.1 · yes · add-to · F4: a failed server verification now reads server-verify-failed in the --json document beside the server-verify cause; the hint line is unmoved
surface#8.1 · no · the report keeps macOS's server check a carve-out still unmet, so macOS still has no peer pid and no server-verification hint
surface#9.1 · no · the report keeps every line of the release paragraph unmoved; the session changes only its strict-modes document
surface#10.1 · no · the report keeps no line of the link and unlink paragraph unmoved, the one-line success form included; the session changes only its strict-modes document
surface#10.2 · no · the report keeps the link since form unmoved, the same paragraph carrying the same single change
surface#10.3 · no · the already-linked and unlink-of-unheld lines are unmoved, per the report's no-line statement for the link and unlink paragraph
surface#11.1 · no · the report keeps verify's human output unmoved and verify takes no --json; only its log gains refusal codes
surface#12.1 · no · the report keeps a refused home tree lineless, with the home-state pair unmoved
surface#13.1 · no · the catch-site human line and the instance-state pair stand byte for byte without --json, and the report keeps them unmoved
surface#14.1 · yes · contradict · revive --json now prints one exit-1 document for every start refusal with closed codes including stale-heartbeat (F8, F9), so the prints-no-document line and the five-code list are false; pump_child prints none (F10) and the catch site stays silent for revive's start (F13)
