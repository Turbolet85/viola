@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=41 class=omission
leaf-text: Human text by default, `--json` for agents; the CLI adds
source: dec-ziwtdizz «decision/cli-output-conventions · Human text by default, --json for agents — The CLI prints human text by default and `--json` for agents, with a typed exit code per refusal. · why: Agents need»
replace-with: Human text by default, `--json` for agents, except `viola console`, the terminal fleet console's own surface, which takes no `--json` and draws its own screen; the CLI adds
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: also a `viola up` whose lines are not all successes)
source: ctr-tfxx4gxd «contract/cli-exit-codes/1 · CLI exit codes · 1 — Internal error; the start, revive-preflight, open, focus, up, verify and console refusals, a refused home-tree check or socket directory of the start»
replace-with: also `viola console`'s refusals, each an `unable:` / `hint:` pair on stderr with no path; also a `viola up` whose lines are not all successes)
