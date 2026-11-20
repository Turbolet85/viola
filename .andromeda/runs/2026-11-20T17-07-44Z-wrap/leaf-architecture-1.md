@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=9 class=omission
leaf-text: one `<name>.lock` sibling per guarded file.
source: dec-z7zvfnpe «except: ledger/verify.lock, the automatic verify's attempt lock, guards no file of its own name and stands apart from stamps.json.lock.»
replace-with: one `<name>.lock` sibling per guarded file, except `ledger/verify.lock`, the automatic verify's attempt lock, which guards no file of its own name and stands apart from `stamps.json.lock`.
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=45 class=omission
leaf-text: so no value makes the hold zero or unbounded).
source: ctr-zle6a63y «contract/config-json/verify-new-cli · config.json · verify_new_cli — A JSON boolean, default false, turning on the start's automatic verify; any other value reads false with one parse-rejected config-json malf»
replace-with: so no value makes the hold zero or unbounded), `verify_new_cli` (a JSON boolean, default false, turning on the `viola run` / `viola revive` start's automatic verify of a claude version with no attempted stamps entry; any other value reads false with one `parse-rejected` config-json `malformed` line).
@leaf_edit n=3 leaf=.claude/docs/services/viola-agent-claude.md line=27 class=omission
leaf-text: An unlisted CLI version → transport-only: types, runs the wheel, emits events, withholds dialog answers (`unverified-cli`).
source: dec-zrumetff «decision/start-verifies-new-cli-version · Start verifies a new CLI version when keyed — With config.json verify_new_cli on, a viola run or revive start meeting a claude version with no attempted stamps entry»
replace-with: An unlisted CLI version → transport-only: types, runs the wheel, emits events, withholds dialog answers (`unverified-cli`); with `config.json` `verify_new_cli` on (off by default), a `viola run` / `viola revive` start meeting a version with no attempted stamps entry first runs `viola verify` once as a child after the endpoint bind and gates on the final verdict (an entry that held every relied row and did not verify is never verified again automatically).
