@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/r
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_edit n=1 leaf=.claude/docs/services/viola-core.md line=20 class=stale
leaf-text: `EventKind` holds 15 kinds (the length of `EventKind::ALL`; `budget-gate` is no longer one)
source: ctr-zkohdqkb «contract/normalised-event-kinds/lineage · normalised event kinds · lineage — `lineage` is a normalised event kind.»
replace-with: `EventKind` holds 16 kinds (the length of `EventKind::ALL`; `budget-gate` is no longer one), `lineage` among them (`data` `{parent, role}`, each a string or `null`, `source: wrapper`, log-only, never waking `wait`)
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=40 class=omission
leaf-text: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` send no channel frame.
source: ctr-jtlmr6fa «contract/viola-cli-arguments/run-parent · viola CLI arguments · run --parent — viola run --parent <NAME> declares the instance's parent; a parent equal to the instance's own name is a usage error, exit 2,»
replace-with: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `viola run --parent <NAME>` declares the instance's parent (one equal to its own name a usage error, exit 2, before anything is read; with no `--parent` an inherited `VIOLA_NAME` is taken as parent only, dropped when it equals the own name) and `viola run --role <WORD>` its role, a code word.
@leaf_stands n=1 leaf=.claude/docs/services/viola-agent-claude.md line=39
because: The line's `v1` sits only in schema file names (`claude-agents-fixture.v1.json`, `claude-mcp-fixture.v1.json`, `claude-fixture.v1.json`), not in the retired `in v1` qualifier of ctr-q5dspexy's link-column promise, which the line never states.
@leaf_stands n=2 leaf=.claude/docs/services/viola-core.md line=24
because: The line's `v1` is only the schema file name `schemas/diag-line.v1.json`, unrelated to the `in v1` qualifier ctr-q5dspexy dropped from its `viola list` column promise, which the line never states.
