@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=129
last: - 2026-11-21: A test helper that rebuilds a `Command` from `
@leaf_edit n=1 leaf=.claude/rules/testing.md line=27 class=omission
leaf-text: wait on an `events.ndjson` byte offset, `viola wait --after` or a status field.
source: dec-h43uw4nb «decision/key-after-resize-waits-for-size · Key after a resize waits for the new size — A test that writes a key after a resize sends it only once the child reports the new size.»
replace-with: wait on an `events.ndjson` byte offset, `viola wait --after` or a status field; a test that writes a key after a resize sends it only once the child reports the new size (its `size` receipt line), the live rig's KR arm alone typing its key 0, 20 or 100 ms after a resize of its own pty, since that is what it measures.
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: or `/compact` and `/clear` on an unverified CLI)
source: ctr-fguybmtp «contract/send-result/unconfirmable · the send result · unconfirmable — A listed local command with no measured post-condition, any listed command on an unverified CLI, or an echo-confirmed command with no»
replace-with: or any listed command, `/compact`, `/clear`, `/mcp` and `/context` among them, on an unverified CLI)
