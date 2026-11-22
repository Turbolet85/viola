@leaf_group group=architecture+design-system+layout-templates+test-plan+obs-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=141 class=omission
leaf-text: and the `verify_window_` override stands ahead of `package(viola-e2e)`
source: dec-5eznx6xr «Tests named verify_window_, and the fake agent dialog and framing replay tests over every recorded set, take an 80 s per-test kill, in both the ci and mutants profiles, through an override that stands first.»
replace-with: the fake agent dialog and framing replay tests over every recorded set take the same 80 s kill (in both the ci and mutants profiles), and the `verify_window_` override stands ahead of `package(viola-e2e)`
