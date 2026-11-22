@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pres
@leaf_edit n=1 leaf=.claude/rules/api.md line=22 class=omission
leaf-text: `send`: human-typing → turn-running → input-not-ready → no-prompt-submitted
source: ctr-xguevze4 «control-character, empty-text, human-typing, turn-running, the readiness gate (input-not-ready), human-typing and turn-running read again after the gate, paste-cut of a standing cut mark, paste-cut of a paste cut at its bound, then no-prompt-submitted.»
replace-with: `send`: human-typing → turn-running → input-not-ready → paste-cut of a standing cut mark → paste-cut of a paste cut at its bound → no-prompt-submitted
