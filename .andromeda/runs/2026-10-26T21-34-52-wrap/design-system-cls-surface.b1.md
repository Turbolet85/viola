# Classified — design-system · surface.b1

## Answers
surface#1.1 · no · the attention token and the SGR module are unchanged, and the new refusals print unstyled stderr
surface#1.2 · no · the stale dim token is unchanged, since list's table and the SGR module were not edited
surface#1.3 · no · the callsign bold token is unchanged, since the NAME column and the SGR module were not edited
surface#1.4 · no · the reset token is unchanged, since the new refusals add no styled span
surface#1.5 · no · the handoff blue stays unused, since the session adds no colour
surface#1.6 · no · no background is set, since the session adds no colour
surface#2.1 · yes · add-to · F1: the six frame verbs' exit-21 strict-modes refusal joins the fixed-form list, so its "only" list grows
surface#3.1 · no · the width rule and the NAME-only truncation are unchanged, since list's table was not edited and no new output changes them
surface#4.1 · no · the board's columns, BAY line, ordering, escaping and empty-board line are unchanged, since list's table was not edited
surface#4.2 · yes · add-to · F1 and F4: the six frame verbs join the verbs that print the instance pair, and list's refusal pair now depends on whether an instance exists
surface#5.1 · yes · add-to · F1 and F3: send's new instance-pair refusal covers the strict-modes home cause its hint list names, run's and revive's start refusals now cover the whole home tree, and a refused diagnostics open drops its detail line
surface#6.1 · no · the server-verify hint still prints after the new instance check passes, since the new refusal prints the instance pair and not this line
surface#7.1 · no · the macOS server check still passes with no comparison, since the change adds only a pre-frame refusal
surface#8.1 · yes · add-to · F1: release refuses a widened instance first with the instance pair at exit 21 and writes no line, which this entry must carry
surface#9.1 · no · link's behaviour is unchanged on the shared helper, so its one plain result line and exit 0 stand
surface#9.2 · no · link's behaviour is unchanged on the shared helper, so its since format stands
surface#9.3 · no · link's behaviour is unchanged on the shared helper, so the already-linked and unlink-not-held lines stand
surface#10.1 · yes · add-to · F2: verify's home-tree refusal and its stamps-write refusal print the home pair with no step line and no summary, at exit 1
surface#11.1 · yes · add-to · F3: revive's start now checks the whole home tree, so its home-level and instance-level refusals print their pairs at exit 1 with no child
surface#12.1 · no · exit 0 is unchanged, since the session added only refusals at exit 21 and exit 1 and left success results alone
surface#12.2 · yes · add-to · F2, F3 and F5: verify's home-tree refusal, the whole-tree start refusals of run and revive, and a Unix regular-file home join the exit-1 fixed-message list
surface#12.3 · no · exit 2 stays clap-only and hook's refusals still exit 0
surface#12.4 · no · the human-typing refusal is the wrapper's reply and keeps its form, since the new check is a separate exit 21 before any frame
surface#12.5 · no · the budget-paused refusal is the wrapper's reply and keeps its form, since the new check is a separate exit 21 before any frame
surface#12.6 · no · the unverified-cli refusal keeps its form, since the session changed verify's stamps write and not the version gate that send reads
