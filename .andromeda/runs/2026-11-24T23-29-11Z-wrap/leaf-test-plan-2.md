@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=81
last: - 2026-11-24: A live-reading script prints verdict words fro
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: `--fork-session` (beside `--resume`, the compiled fork id; alone it changes nothing);
source: ctr-7t7jbcb4 «contract/fake-agent-cli/resume-absent · viola-fake-agent command line · --resume-absent — The argv-only --resume-absent, given with --resume, makes the fake agent fire SessionEnd with no SessionStart»
replace-with: `--fork-session` (beside `--resume`, the compiled fork id; alone it changes nothing), and the argv-only `--resume-absent` (beside `--resume`: fires SessionEnd with no SessionStart, closes its hooks, ignores SIGHUP and exits 1, at both launch sites: a trusted start and the trust dialog's accept);
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=38 class=omission
leaf-text: under `--resume <id>` the launch SessionStart is
source: ctr-j3iinkni «under --resume-absent with --resume it fires none.»
replace-with: under `--resume <id>` (save under `--resume-absent`, which fires no SessionStart) the launch SessionStart is
