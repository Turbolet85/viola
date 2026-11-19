@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc
@leaf_read leaf=CLAUDE.md lines=135
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=84 class=omission
leaf-text: Skills are not local commands.
source: dec-lbocs2om «The local-command-context relied ledger row holds that /context writes its stdout tag to the transcript and submits no prompt within the confirmation window; verify probes it by Run B's eighth paste, /context»
replace-with: `/context` is confirmed the same way on a verified version, by a new `<local-command-stdout>` tag past the transcript's pre-paste end with no prompt submitted (row `local-command-context`, probed by Run B's eighth paste `/context`); on an unverified CLI or before its transcript exists it is `unconfirmable`. Skills are not local commands.
@leaf_edit n=2 leaf=CLAUDE.md line=40 class=omission
leaf-text: only `viola verify` writes `ledger/stamps.json`;
source: dec-nyf6efjo «except: The test-only live rig seeds a stamp kept from its own verify scratch home into its own CC scratch home only.»
replace-with: only `viola verify` writes `ledger/stamps.json` (save the test-only live rig, which seeds a stamp kept from its own verify scratch home into its own CC scratch home only);
