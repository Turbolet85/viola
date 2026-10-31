# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the CLI colour tokens and the colour decision order are untouched: no styling rule moved and the session adds no SGR (Untouched)
surface#1.2 · no · the stale (lamp-off) dim token is untouched: no colour rule moved; only the RefusalReason::unknown palette row moves, and that row is not in this batch
surface#1.3 · no · the callsign bold token is untouched: no colour rule moved (Untouched)
surface#1.4 · no · the reset token is untouched: no colour rule moved (Untouched)
surface#1.5 · no · the handoff blue is not used and no colour rule moved (Untouched)
surface#1.6 · no · the backgrounds rule (never set) is untouched: no colour rule moved (Untouched)
surface#2.1 · yes · add-to · the stderr refusal form `unable  <name>  <reason>  <detail>` now receives a peer's refusal and detail only through the code-word screen (F1): a non-word refusal reads `unknown` and a non-word string detail reads as no detail (F2)
surface#3.1 · no · the human table's NAME truncation reads stdout's own width, a rule the session does not change; no column or row form moved (Untouched)
surface#4.1 · no · the board's rows, BAY line, escaping and failed-read outcome are untouched (Untouched: strip table, BAY line, escaping), and its `unknown` words are STATUS and WHEEL values, not a peer's refusal reading
surface#4.2 · no · the exit-21 strict-modes pair and its fixed `unable:` / `hint:` forms are unchanged; `Refusal` here is the pattern's label, not the retired viola_agent_claude::Refusal
surface#5.1 · no · the board's strict-modes scan, its two rules and its pending CARRY are unchanged: the session moves no home or instance check
surface#6.1 · yes · contradict · `unknown`'s detail is now a code word or nothing, not opaque free text (F2), so the reason "its detail is opaque" no longer holds; the no-hint line itself stands, and a code word outside the closed list still exits 14 with no hint (F3)
surface#7.1 · no · the server-verification line is the client's own fixed text, not a peer's reply, and the no-pid, path or endpoint rule still holds on the changed client.rs
surface#8.1 · no · the macOS server check (no peer pid, passes with no comparison) is unchanged; the session names only channel error data, not the check
surface#9.1 · no · the release budget line prints the reply's `override_until` and the wheel holder, neither of which the screen touches; its claim still holds on the changed client.rs
surface#10.1 · no · link and unlink print the verb's parsed arguments, never reply fields, and the screen takes only a peer's refusal and detail
surface#10.2 · no · the `since` is not a screened field and the message-mode escaper is not named as changed, so the link line's claim still holds
surface#10.3 · no · the already-linked form and the unlink line are exit 0 success lines that print no screened field
surface#11.1 · no · on the changed verify.rs the step lines, ledger rows, counts and summary still hold: callers moved to AgentError, printed lines unchanged (pattern 5)
surface#12.1 · no · on the changed verify.rs the home-tree pair and its exit 1 still hold: callers moved, printed lines unchanged (pattern 5)
surface#13.1 · no · the Unix pair and the Windows NotFound catch-site behaviour still hold; this NotFound is the stat's error kind, not the retired Refusal::NotFound
surface#14.1 · no · revive's list and its --json refusal documents still hold (no hint, stderr empty, re-read green); its private Refusal stays and src/cmd/revive.rs is not reached
surface#15.1 · no · the exit 0 result line and the {"v":1,"ok":…} document are success output the session does not touch
surface#15.2 · no · on the changed run, verify and agent files the exit 1 printed lines still hold (callers moved, lines unchanged, pattern 5), and revive's --json codes stand
surface#15.3 · yes · add-to · the oversize `viola answer` and the unusable `viola send` also exit 2, writing the fixed UNUSABLE_RESPONSE or UNUSABLE_TEXT stderr line whatever --json says, with no document (F6)
