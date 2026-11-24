@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=132
last: - 2026-11-22: A rename of a product constant or a move of a
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: refused under `CI` (`live-in-ci`), and the live starts
source: dec-e5w63374 «reached only locally, through viola verify, the standing live rig's live starts and the gated live-reading scripts, never in CI.»
replace-with: refused under `CI` (`live-in-ci`), the gated live-reading scripts, and the live starts
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=51 class=omission
leaf-text: and is never a live reading).
source: dec-zqz4a5ld «Real adopt of a plain claude session, the resume of a real ended session and the refusal of a real live one, is proven only by the gated live reading R5; the fake-agent rehearsal of its script is never a live reading.»
replace-with: and is never a live reading), nor of a real adopt of a plain claude session, the resume of a real ended session or the refusal of a real live one, which only the gated live reading R5 proves (the fake-agent rehearsal of its script is never a live reading).
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=50 class=omission
leaf-text: and the fake-agent receipt (the one exception
source: dec-d5iw22pb «A test claim about the child's argv is read from the live child's /proc cmdline, never from the fake agent's receipt.»
replace-with: and the fake-agent receipt (a claim about the child's argv read from the live child's `/proc` cmdline, never from the receipt, which records no argv) (the one exception
@leaf_edit n=4 leaf=.claude/rules/testing.md line=40 class=omission
leaf-text: the fake-agent receipt and exit codes
source: dec-d5iw22pb «A test claim about the child's argv is read from the live child's /proc cmdline, never from the fake agent's receipt.»
replace-with: the fake-agent receipt and exit codes (a claim about the child's argv read from the live child's `/proc` cmdline, never from the fake agent's receipt, which records no argv)
