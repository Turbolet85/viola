@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=13 class=stale
leaf-text: and by the `viola run` and `viola revive` start, only when
source: dec-wjypwipg «viola verify, and the run, revive and adopt start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.»
replace-with: and by the `viola run`, `viola revive` and `viola adopt` start, only when
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=33 class=stale
leaf-text: `run`, `revive`'s start, `mcp` and `hook` silent at that site
source: ctr-y4fupb3w «without `--json` the human line `error: internal error` stands, and `run`, `revive`'s start, `adopt`'s start, `mcp` and `hook` stay silent.»
replace-with: `run`, `revive`'s start, `adopt`'s start, `mcp` and `hook` silent at that site
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: also `viola run` and `viola revive`'s start refusing the socket directory
source: ctr-uwztwbnt «Exit 21 with during connect, at WARN, no channel-request line; run's, revive's and adopt's start write it with exit_code 1.»
replace-with: also `viola run`'s, `viola revive`'s and `viola adopt`'s start refusing the socket directory
@leaf_edit n=4 leaf=.claude/docs/conventions.md line=33 class=omission
leaf-text: also `viola focus`'s and `viola up`'s refusals
source: ctr-yzm3hmbz «`<code>` is one of `strict-modes-failed` · `session-live` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
replace-with: also `viola adopt`'s refusals, under `--json` one refusal document whose `<code>` is one of `strict-modes-failed` · `session-live` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error` (`session-live` when a `claude agents --json` row carries the adopted id); also `viola focus`'s and `viola up`'s refusals
@leaf_edit n=5 leaf=.claude/docs/conventions.md line=40 class=omission
leaf-text: `viola focus <name> [--json]` takes one ViolaName and `--json` only;
source: ctr-asykvpei «`viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json]` takes the id in the closed session-id shape, hex of either case, and no words after `--`»
replace-with: `viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json]` takes the id in the closed session-id shape, hex of either case, and no words after `--` (a `--parent` equal to `--name` a usage error, exit 2); `viola focus <name> [--json]` takes one ViolaName and `--json` only;
@leaf_edit n=6 leaf=.claude/docs/conventions.md line=45 class=stale
leaf-text: makes `viola verify`, `viola run` and `viola revive` read its value
source: dec-wjypwipg «viola verify, and the run, revive and adopt start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.»
replace-with: makes `viola verify`, `viola run`, `viola revive` and `viola adopt` read its value
@leaf_edit n=7 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: It also holds the resume shapes `viola revive` launches with
source: ctr-xtt3xpx7 «A revived or adopted start launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.»
replace-with: It also holds the resume shapes `viola revive` and `viola adopt` launch with
@leaf_edit n=8 leaf=.claude/docs/services/viola-agent-claude.md line=14 class=omission
leaf-text: `viola revive`'s preflight and `viola verify`'s Run B
source: dec-3tojobmm «viola adopt resumes under viola, with an operator-given name, a plain claude session named by its session id, and refuses with session-live, exit 1, when a claude agents --json row carries that id»
replace-with: `viola revive`'s preflight, `viola adopt`'s `session-live` check and `viola verify`'s Run B
@leaf_edit n=9 leaf=.claude/docs/services/viola-agent-claude.md line=28 class=stale
leaf-text: and the `viola run` and `viola revive` start, read the value
source: dec-wjypwipg «viola verify, and the run, revive and adopt start, read the value of CLAUDE_CONFIG_DIR, only when claude_env_keep keeps that name, to locate the Claude config file.»
replace-with: and the `viola run`, `viola revive` and `viola adopt` start, read the value
