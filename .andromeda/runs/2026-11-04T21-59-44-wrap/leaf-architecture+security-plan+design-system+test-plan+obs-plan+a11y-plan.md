@leaf_group group=architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
@leaf_read leaf=CLAUDE.md lines=131
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=86 class=omission
leaf-text: built and CI-tested on Windows, macOS and Linux from the first commit
source: fct-xxlqtlbc «A new OS gate on a test therefore lands with its `GATES` row and reason, or the census goes red.»
replace-with: built and CI-tested on Windows, macOS and Linux from the first commit (every OS-gated test a `GATES` row with its reason in the census binary `tests/contract_os_gated_tests.rs`, or the census goes red)
