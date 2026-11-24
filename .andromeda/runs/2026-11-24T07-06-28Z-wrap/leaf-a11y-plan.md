@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pres
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=53 class=stale
leaf-text: Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
source: dec-3cppjeep «Eight time bounds viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) fall on»
replace-with: Eight time bounds viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) fall on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by them (provisional).
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=53 class=omission
leaf-text: the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched.
source: dec-eepbe3vk «A start's window step waits at most 5 s for its own terminal's window to appear in the compositor's client list; the wait falls on viola's own start, never on a key typed into a session, and never moves»
replace-with: the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched. A start's window step waits at most 5 s for its own terminal's window to appear in the compositor's client list, each compositor read through the Hyprland seam is bounded at 5 s a reply (the window step, viola open, viola focus), and viola open waits at most 30 s for the instance it started to come live, then reads not-live; each falls on viola's own start or verb, never on a key typed into a session, and never moves the wheel.
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=22 class=omission
leaf-text: `viola link` / `viola unlink` print one plain-ASCII result line in one write with no SGR in any mode.
source: ctr-rkdfpfga «`viola up` prints one line per instance, `<name>  <word>` or `<name>  live  <placement>`, plain ASCII, with no SGR, each line in one write.»
replace-with: `viola link` / `viola unlink` print one plain-ASCII result line in one write with no SGR in any mode, and `viola up` prints one line per instance, `<name>  <word>` or `<name>  live  <placement>`, plain ASCII, with no SGR, each line in one write.
