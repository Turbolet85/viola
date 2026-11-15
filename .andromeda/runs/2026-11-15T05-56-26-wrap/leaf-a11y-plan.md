@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pres
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=53 class=retired
leaf-text: The statusline-bearing start (as landed, chunk 2026-10-10-statusline-pass-through): `passthrough_with_a_statusline_source_adds_no_viola_bytes` (`tests/tui_passthrough.rs`) is its reading of the zero-viola-bytes clause, green in CI (run 38080855246). `viola hook statusline`'s 5 s PROVISIONAL bound falls on the user's own command, never on a keystroke or the human.
source: dec-3cppjeep «Time bounds never fall on the human — Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read) falls on the driver or viola's child; no key is blocked, refused or delayed by it.»
replace-with: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read) falls on the driver or viola's child; no key is blocked, refused or delayed by it.
