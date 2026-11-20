@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: `driver`, `driven` and `since` reach no home-level line.
source: dec-njuxvfi2 «decision/wheel-typed-mark-uninstrumented · Wheel typed mark and wakes count uninstrumented — The stdin typed mark and the held scheduled-wakes count are in-memory wrapper state that open no span»
replace-with: `driver`, `driven` and `since` reach no home-level line. The wrapper's stdin typed mark and its held scheduled-wakes count are in-memory state that open no span and write no line of their own, and carry no key and no prompt text.
