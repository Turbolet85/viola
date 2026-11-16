@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/r
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=omission
leaf-text: (Stop's `turn-ended` data is `{last_assistant_message, background_tasks}`:
source: ctr-nyxx7utw «StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null and `cause` `api-error`.»
replace-with: (StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null and `cause` `api-error`; Stop's `turn-ended` data is `{last_assistant_message, background_tasks}`, with no `cause`:
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=omission
leaf-text: a shape no `send` relies on needs none (the founder's ruling, 2026-10-07).
source: dec-udlphze4 «A CLI shape no send relies on needs no probe, such as the long-paste frame beside typed text and between two pairs; StopFailure is registered, provisionally, with no ledger row and no verify probe.»
replace-with: a shape no `send` relies on needs none (the founder's ruling, 2026-10-07). StopFailure is registered, provisionally, with no ledger row and no verify probe.
@leaf_edit n=3 leaf=.claude/docs/gotchas.md line=99 class=omission
leaf-text: every relied-on behaviour is a ledger row with a `viola verify` probe.
source: dec-zzhuidvp «StopFailure is registered, provisionally, with no ledger row and no verify probe.»
replace-with: every relied-on behaviour is a ledger row with a `viola verify` probe (StopFailure excepted: registered, provisionally, with no ledger row and no verify probe).
