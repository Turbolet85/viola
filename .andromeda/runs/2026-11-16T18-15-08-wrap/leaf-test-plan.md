@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=119
last: - 2026-11-16: A CI red read at the operator pass that is not
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=27 class=omission
leaf-text: an API-error turn end's carries `cause` too, absent on a Stop
source: ctr-gyhkfi7n «cause is present on an API-error turn end only, absent on a Stop, and is one of nine closed words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error.»
replace-with: an API-error turn end's carries `cause` too, absent on a Stop and one of nine closed words (`rate-limited`, `overloaded`, `auth-failed`, `billing`, `invalid-request`, `model-not-found`, `server-error`, `output-limit`, `api-error`)
