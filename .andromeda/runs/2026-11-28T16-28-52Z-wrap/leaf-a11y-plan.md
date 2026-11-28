@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project prese
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=19 class=stale
leaf-text: and redraws on a resize or a change in the fleet it shows;
source: dec-3qin7hbf «the terminal fleet console (`viola console`), its own surface, which redraws its frame when the terminal's size changes, the fleet it shows changes or a key moves its selection or opens a prompt»
replace-with: and redraws on a resize, a change in the fleet it shows or a key that moves its selection or opens a prompt;
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=19 class=omission
leaf-text: which reads keys from stdin in raw mode
source: dec-2jfyrkg6 «The fleet console's one editing line, for send, switch and open, takes one line, echoed as typed; Enter hands it to the verb and Escape cancels and runs nothing.»
replace-with: which reads keys from stdin in raw mode, its one editing line, for send, switch and open, taking one line echoed as typed (Enter hands it to the verb, Escape cancels and runs nothing),
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=19 class=omission
leaf-text: the terminal fleet console's keyboard reach is the route entry «Fleet a11y verification»'s.
source: dec-7gd7u3fi «The fleet console has no key trap: n or Escape leaves a confirm, Escape leaves the editing line and the answer prompt running nothing, and Ctrl-C ends the console from every mode with the terminal restored.»
replace-with: the terminal fleet console's keyboard reach is the route entry «Fleet a11y verification»'s, and it has no key trap: n or Escape leaves a confirm, Escape leaves the editing line and the answer prompt running nothing, and Ctrl-C ends the console from every mode with the terminal restored.
@leaf_edit n=4 leaf=.claude/rules/a11y.md line=22 class=omission
leaf-text: viola has no web or graphical frontend, so no page token pair or layout to verify.
source: dec-2ejrxhtb «The fleet console's selection is carried by a gutter mark on the selected row alone, never by colour.»
replace-with: The fleet console's selection is carried by a gutter mark on the selected row alone, never by colour. viola has no web or graphical frontend, so no page token pair or layout to verify.
