@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=99 class=stale
leaf-text: (StopFailure excepted: registered, provisionally, with no ledger row and no verify probe)
source: dec-zzhuidvp «except: StopFailure is registered with no ledger row and no verify probe.»
replace-with: (StopFailure excepted: registered with no ledger row and no verify probe)
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=stale
leaf-text: StopFailure is registered, provisionally, with no ledger row and no verify probe.
source: dec-udlphze4 «except: A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered with no ledger row and no verify probe.»
replace-with: StopFailure is registered with no ledger row and no verify probe.
@leaf_edit n=3 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=stale
leaf-text: less its trailing run of ASCII space, TAB, CR and LF (provisional), the text kept as submitted
source: dec-sdymze5l «decision/submitted-form-prompt-match · Prompt match in the submitted form after agent normalisation — The typed text matches prompt-submitted's text, whatever its origin, after the agent crate»
replace-with: less its trailing run of ASCII space, TAB, CR and LF, the text kept as submitted
