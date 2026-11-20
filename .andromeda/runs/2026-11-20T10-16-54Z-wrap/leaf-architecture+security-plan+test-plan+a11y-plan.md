@leaf_group group=architecture+security-plan+test-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=150
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=135
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=79 class=stale
leaf-text: quiet period + no modal signature + the input box
source: dec-ujwoyzmr «vt100 0.16.2 serves only as a pre-send readiness gate: after `turn-ended` it waits for a quiet screen, requires no standing modal and waits, up to its maximum wait, for the version-stamped input-box signature»
replace-with: quiet period + no standing modal + the input box
@leaf_edit n=2 leaf=.claude/docs/gotchas.md line=79 class=stale
leaf-text: (a modal refuses at once;
source: dec-xsqsrc2s «A modal literal's row is a quote only when it is not option-shaped and the box's own rows stand below it; a quote alone on its own line, or a code-block line holding only the label, still refuses.»
replace-with: (a standing modal refuses at once, while a modal literal's row that is not option-shaped with the box's own rows below it is a quote and does not;
