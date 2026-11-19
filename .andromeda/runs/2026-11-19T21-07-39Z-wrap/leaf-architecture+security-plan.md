@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=22 class=omission
leaf-text: `unconfirmable` only ever appears inside an `ok` payload.
source: ctr-qrekz4gq «Types a prompt into the driven Claude session; a listed local command sent before its transcript exists, or on an unverified CLI, answers ok with confirmed false (unconfirmable), never a confirmation.»
replace-with: `unconfirmable` only ever appears inside an `ok` payload, as `confirmed: false` with detail `unconfirmable`: a ledger-listed local command whose post-condition is none or not yet measured, a stdout echo with no readable session transcript, a listed local command sent before its transcript exists, or any listed local command on an unverified CLI (never refused as `unverified-cli` on `send`).
