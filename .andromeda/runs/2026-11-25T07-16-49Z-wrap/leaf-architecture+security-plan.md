@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=42
last: - 2026-11-24: A verb that opens its log on a home it resolve
@leaf_edit n=1 leaf=.claude/rules/api.md line=20 class=omission
leaf-text: answered `{model, effort}`; a bad or missing half `-32602` with `data: null`)
source: ctr-qd4d52i7 «A request: `switch` `{model?, effort?}`, at least one, reads no `from` and answers `{model, effort}` or a refusal human-typing, unverified-cli, or not-delivered with turn-running, dialog-pending»
replace-with: answered `{model, effort}` or a refusal `human-typing`, `unverified-cli`, or `not-delivered` with `turn-running`, `dialog-pending`, `no-session`, `child-not-ended` or `resume-failed`; a bad or missing half `-32602` with `data: null`)
