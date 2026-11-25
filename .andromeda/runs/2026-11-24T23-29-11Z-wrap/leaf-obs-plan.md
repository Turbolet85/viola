@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/ci.md lines=36
last: - 2026-11-20: A WATCH's green run read from CI counts only o
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: and from `viola adopt`'s preflight `session-live` (one `process-exit` line, no child started)
source: ctr-2k6eabzj «a resumed child ending before its session starts gives exit 1 detail not-resumed after the child's process-exit.»
replace-with: and from `viola adopt`'s preflight `session-live` (one `process-exit` line, no child started), and from `viola adopt`'s resumed child ending before its session starts `not-resumed` (after the child's `process-exit`)
@leaf_edit n=2 leaf=.claude/docs/obs-summary.md line=70 class=stale
leaf-text: adding no event, detail word, field, subject or span of its own.
source: dec-eyetd4mi «except: The detail word not-resumed, on adopt's own self process-exit exit 1 when the resumed child ends before its session starts.»
replace-with: adding no event, detail word, field, subject or span of its own, save the detail word `not-resumed` on its own self `process-exit` exit 1 when the resumed child ends before its session starts.
