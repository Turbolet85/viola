@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=129
last: - 2026-11-21: A test helper that rebuilds a `Command` from `ge
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: a slash text off the list → exit 13 `no-prompt-submitted`;
source: ctr-e7wtr26y «contract/viola-send/paste-cut · viola send · paste-cut — A send into a child that stops reading exits 13 not-delivered / paste-cut within the compiled 5 s paste bound, with the send's cursor and no Enter typed, and t»
replace-with: a slash text off the list → exit 13 `no-prompt-submitted`; a send into a child that stops reading exits 13 `not-delivered`/`paste-cut` within the compiled 5 s paste bound, with the send's cursor and no Enter typed, and the next send is refused `paste-cut` with nothing issued;
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: a long text the CLI wraps is confirmed (`send_long_text_wrapped_by_the_cli_is_confirmed`);
source: ctr-3b4jrhoy «contract/viola-send/during-a-blocked-paste · viola send · during a blocked paste — A send during a blocked paste is refused turn-running after waiting at most the paste bound, and a hook record during the blocked p»
replace-with: a long text the CLI wraps is confirmed (`send_long_text_wrapped_by_the_cli_is_confirmed`); a send during a blocked paste is refused `turn-running` after waiting at most the paste bound, and a hook record during the blocked paste lands within the bound;
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: `--turn-fail <variant>`, beside `--turn-stop`,
source: ctr-l5pnzopa «contract/fake-agent-cli/stop-reading · viola-fake-agent command line · --stop-reading — The argv-only --stop-reading makes the fake agent never read stdin, parked forever with its terminal left raw, so a paste into it blo»
replace-with: the argv-only `--stop-reading` makes it never read stdin, parked forever with its terminal left raw, so a paste into it blocks, `--turn-fail <variant>`, beside `--turn-stop`,
