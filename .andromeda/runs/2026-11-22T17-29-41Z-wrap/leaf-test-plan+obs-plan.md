@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=omission
leaf-text: whose one fixed line on a met prompt is the only record that the prompt stood at start.
source: dec-ha4p5d3m «decision/standing-modal-reading-uninstrumented · Standing-modal reading uninstrumented — The readiness gate's standing-modal reading adds no log line, span field, event or detail code of its own»
replace-with: whose one fixed line on a met prompt is the only record that the prompt stood at start. The readiness gate's standing-modal reading adds no log line, span field, event or detail code of its own: a modal label quoted above a ready input box lets the send proceed with nothing written, and a standing modal refuses with the existing detail `input-not-ready`.
