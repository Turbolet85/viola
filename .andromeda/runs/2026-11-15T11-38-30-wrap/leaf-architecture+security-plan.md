@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=17 class=omission
leaf-text: Adding a field/kind is additive; removing or re-meaning bumps `v`.
source: dec-zejotuie «decision/protocol-version-bump-rule · When the protocol version bumps — Adding a field or event kind is additive and needs no bump of `v`; removing a field or changing its meaning bumps `v`.»
replace-with: Adding a field/kind is additive; removing or re-meaning bumps `v` (save the removed budget-gate fields and event kind, the snapshot's `statusline_command` and the stamps' two status-line row ids, which kept every own-format `v` at 1; readers skip what was removed).
@leaf_edit n=2 leaf=.claude/rules/api.md line=29 class=omission
leaf-text: 2 usage (clap, a second positional to `viola send` included;
source: ctr-zfrhupy4 «contract/cli-verbs/release · CLI verbs · release — `viola release <target>`; a `--budget` flag is a usage error (exit 2).»
replace-with: 2 usage (clap, a second positional to `viola send` and a `--budget` flag on `viola release` included;
