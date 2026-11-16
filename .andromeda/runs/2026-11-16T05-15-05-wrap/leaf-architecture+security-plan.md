@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=22 class=omission
leaf-text: with human-typing and turn-running read a second time once the readiness gate's wait has returned ready, before anything is issued or typed;
source: dec-5khspss2 «A send's post-gate section, the human-typing and turn-running reads, the link record, send-issued and the paste, runs under one wrapper flight lock that the hook.event claim, turn start and end and wheel»
replace-with: with human-typing and turn-running read a second time once the readiness gate's wait has returned ready, before anything is issued or typed, that post-gate section (the second reads, the link record, `send-issued` and the paste) running under the one wrapper flight lock the `hook.event` claim, turn start and end and wheel moves also take, a human key never taking it;
