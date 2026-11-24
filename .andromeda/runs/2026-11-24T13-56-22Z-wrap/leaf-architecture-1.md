@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=40 class=omission
leaf-text: `viola focus <name> [--json]` takes one ViolaName and `--json` only;
source: ctr-d6isq7rt «`viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` takes at least one of `--model` and `--effort`, else a usage error, exit 2; prints `<name>  switched  <model> <effort>` on stdout»
replace-with: `viola focus <name> [--json]` takes one ViolaName and `--json` only; `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` takes at least one of `--model` and `--effort` (neither a usage error, exit 2), prints `<name>  switched  <model> <effort>` and exits 0, 10, 12, 13, 14, 20 or 21 (CLI-only, no MCP tool);
@leaf_edit n=2 leaf=.claude/docs/services/viola-channel.md line=21 class=omission
leaf-text: `send · wait · last · answer · pause · release · link · unlink · hook.dialog`
source: ctr-qd4d52i7 «A request: `switch` `{model?, effort?}`, at least one, reads no `from` and answers `{model, effort}` or a refusal human-typing, unverified-cli, or not-delivered with turn-running, dialog-pending»
replace-with: `send · wait · last · answer · pause · release · link · unlink · switch · hook.dialog`
@leaf_edit n=3 leaf=.claude/docs/services/viola-channel.md line=26 class=omission
leaf-text: `link` (→ `{since, already_linked}`) and `unlink` (→ `{}`), the last two sent to the driven instance's wrapper.
source: ctr-qd4d52i7 «A request: `switch` `{model?, effort?}`, at least one, reads no `from` and answers `{model, effort}` or a refusal human-typing, unverified-cli, or not-delivered with turn-running, dialog-pending»
replace-with: `link` (→ `{since, already_linked}`) and `unlink` (→ `{}`), the last two sent to the driven instance's wrapper, and `switch` (`{model?, effort?}`, at least one, no `from`, → `{model, effort}` or a refusal `human-typing`, `unverified-cli` or `not-delivered` with `turn-running`, `dialog-pending`, `no-session`, `child-not-ended` or `resume-failed`; a bad or missing half `-32602`, `data` null).
@leaf_edit n=4 leaf=.claude/docs/services/viola-channel.md line=26 class=omission
leaf-text: re-validated, appended with `source:"hook"`;
source: ctr-u5chf3br «A valid event is appended as {v, ts:<append time>, instance, kind, source:"hook", data}; after a switch's respawn the old child's late `session-end` of the switched id is dropped once while the new child's pid is alive.»
replace-with: re-validated, appended with `source:"hook"`, except that after a switch's respawn the old child's late `session-end` of the switched id is dropped once while the new child's pid is alive;
@leaf_edit n=5 leaf=.claude/docs/services/viola-core.md line=21 class=stale
leaf-text: `NotDelivered` (seven values, `empty-text` an empty typed text on `send`, `paste-cut` the seventh: a paste cut at its bound or a standing cut mark)
source: ctr-thxblc25 «Serialised kebab-case; its detail is one of input-not-ready, no-prompt-submitted, turn-running, unknown-dialog, control-character, empty-text, paste-cut, dialog-pending, no-session, child-not-ended, resume-failed.»
replace-with: `NotDelivered` (eleven values, `empty-text` an empty typed text on `send`, `paste-cut` the seventh: a paste cut at its bound or a standing cut mark, then `dialog-pending`, `no-session`, `child-not-ended` and `resume-failed`)
@leaf_edit n=6 leaf=.claude/docs/services/viola-core.md line=20 class=omission
leaf-text: one line at every start after `lineage`, log-only)
source: ctr-yemjpoxs «The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`, and once at every switch, after the old child is reaped and before the respawn; it is log-only and never wakes `wait`.»
replace-with: one line at every start after `lineage` and one at every switch after the old child is reaped and before the respawn, log-only)
@leaf_edit n=7 leaf=.claude/docs/services/viola-core.md line=20 class=omission
leaf-text: one line after `launch` only when started under `--origin`, log-only, never waking `wait`)
source: ctr-t3m4inxx «The wrapper appends one `window` line with `source: wrapper` after the `launch` line, only when started under --origin, and again after a switch's `launch` line when the snapshot holds a window»
replace-with: one line after `launch` only when started under `--origin`, and again after a switch's `launch` line when the snapshot holds a window, log-only, never waking `wait`)
@leaf_edit n=8 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=omission
leaf-text: It also holds the resume shapes `viola revive` and `viola adopt` launch with
source: ctr-xtt3xpx7 «A revived or adopted start, and a switch's respawn, launches `claude` and adds `--resume <id>`, the id one argv element of a direct spawn.»
replace-with: It also holds the resume shapes `viola revive`, `viola adopt` and a `viola switch` respawn launch with
