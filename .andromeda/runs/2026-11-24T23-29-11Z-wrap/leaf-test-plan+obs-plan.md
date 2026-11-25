@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=40 class=stale
leaf-text: adding no event, detail word, field, subject or span of its own.
source: dec-eyetd4mi «except: The detail word not-resumed, on adopt's own self process-exit exit 1 when the resumed child ends before its session starts.»
replace-with: adding no event, field, subject or span of its own and one detail word only, `not-resumed`, on its own self `process-exit` exit 1.
@leaf_edit n=2 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: `viola adopt`'s preflight likewise ends one `process-exit{exit_code:1, detail:"session-live"}` with no child started;
source: ctr-2k6eabzj «a resumed child ending before its session starts gives exit 1 detail not-resumed after the child's process-exit.»
replace-with: `viola adopt`'s preflight likewise ends one `process-exit{exit_code:1, detail:"session-live"}` with no child started, and a resumed child ending before its session starts gives one `process-exit{exit_code:1, detail:"not-resumed"}` after the child's `process-exit`;
