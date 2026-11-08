@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=7 class=stale
leaf-text: with no WCAG conformance claim; the frontend is deferred to 0.2.0 and its technology is not chosen.
source: fct-tzchk23h «No web or graphical frontend: the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».»
replace-with: with no WCAG conformance claim; viola has no web or graphical frontend: the terminal fleet console (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=9 class=stale
leaf-text: none in 0.1.0: the frontend is deferred to 0.2.0 (its technology is not chosen), and
source: fct-5k3yneta «N/A — viola has no web or graphical frontend.»
replace-with: none: viola has no web or graphical frontend, and
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=13 class=stale
leaf-text: the frontend's harness is deferred to 0.2.0.
source: fct-pjrikcxj «N/A — viola has no web or graphical frontend.»
replace-with: viola has no web or graphical frontend, so no frontend harness.
@leaf_edit n=4 leaf=.claude/docs/a11y-summary.md line=16 class=retired
leaf-text: **web-spa:** none in 0.1.0: the frontend is deferred to 0.2.0 and its technology is not chosen
source: fct-tzchk23h «No web or graphical frontend: the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».»
replace-with: **web-spa:** N/A: viola has no web or graphical frontend
@leaf_edit n=5 leaf=.claude/docs/a11y-summary.md line=20 class=stale
leaf-text: none in 0.1.0 (the frontend is deferred to 0.2.0)
source: fct-5k3yneta «N/A — viola has no web or graphical frontend.»
replace-with: none (viola has no web or graphical frontend)
@leaf_edit n=6 leaf=.claude/docs/a11y-summary.md line=22 class=stale
leaf-text: the frontend is deferred to 0.2.0; cli / tui write no a11y rows
source: fct-hn7hmb2g «N/A — viola has no web or graphical frontend.»
replace-with: viola has no web or graphical frontend; cli / tui write no a11y rows
@leaf_edit n=7 leaf=.claude/docs/a11y-summary.md line=29 class=stale
leaf-text: pins the caption order the deferred 0.2.0 page must share
source: fct-uok334yy «a literal pins the caption order (NAME · LIVE · STATUS · WHEEL · DIALOG · CLI), which the terminal fleet console (matrix#v2-14) reuses»
replace-with: pins the caption order, which the terminal fleet console (matrix#v2-14) reuses
@leaf_edit n=8 leaf=.claude/docs/a11y-summary.md line=33 class=stale
leaf-text: none in 0.1.0: the cli cases drive the `viola` binary through `std::process::Command`, and the tui cases use the tests' own portable-pty pin; the browser a11y tooling left with the browser leg, and the frontend's is a 0.2.0 input, its technology not chosen
source: fct-trh3q5nm «none. The cli cases drive the `viola` binary through `std::process::Command`, and the tui cases use the tests' own portable-pty pin; viola renders no page, so no browser a11y tooling.»
replace-with: none: the cli cases drive the `viola` binary through `std::process::Command`, and the tui cases use the tests' own portable-pty pin; viola renders no page, so no browser a11y tooling
@leaf_edit n=9 leaf=.claude/docs/a11y-summary.md line=35 class=stale
leaf-text: none; the frontend is deferred to 0.2.0 and its technology is not chosen
source: fct-lfnx2oea «**aria-component-library-install:** none: viola renders no page.»
replace-with: none: viola renders no page
@leaf_edit n=10 leaf=.claude/docs/a11y-summary.md line=36 class=stale
leaf-text: none in 0.1.0: no page renders a token pair
source: fct-d5bgiald «**contrast-verification-harness-setup:** none: no page renders a token pair.»
replace-with: none: no page renders a token pair
@leaf_edit n=11 leaf=.claude/docs/a11y-summary.md line=51 class=stale
leaf-text: The frontend is deferred to 0.2.0 and its technology is not chosen, so no page structure, live-region, announcer or focus decision binds in 0.1.0.
source: fct-tzchk23h «No web or graphical frontend: the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».»
replace-with: viola has no web or graphical frontend, so no page structure, live-region, announcer or focus decision binds; the terminal fleet console (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».
@leaf_edit n=12 leaf=.claude/rules/a11y.md line=13 class=stale
leaf-text: No WCAG label in 0.1.0: the frontend is deferred to 0.2.0 and its technology is not chosen;
source: fct-5k3yneta «N/A — viola has no web or graphical frontend.»
replace-with: No WCAG label: viola has no web or graphical frontend;
@leaf_edit n=13 leaf=.claude/rules/a11y.md line=15 class=stale
leaf-text: ## The frontend (deferred to 0.2.0)
source: fct-tzchk23h «No web or graphical frontend: the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».»
replace-with: ## The frontend (none)
@leaf_edit n=14 leaf=.claude/rules/a11y.md line=16 class=stale
leaf-text: The frontend is deferred to 0.2.0 and its technology is not chosen.
source: fct-tzchk23h «No web or graphical frontend: the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».»
replace-with: viola has no web or graphical frontend: the terminal fleet console (matrix#v2-14) takes its a11y scope at the route entry «Fleet a11y verification».
@leaf_edit n=15 leaf=.claude/rules/a11y.md line=19 class=retired
leaf-text: The page's tab order is deferred with the frontend to 0.2.0.
source: fct-5lohrrsl «N/A — viola has no web or graphical frontend; the terminal fleet console's keyboard reach is the route entry «Fleet a11y verification»'s.»
replace-with: viola has no web or graphical frontend; the terminal fleet console's keyboard reach is the route entry «Fleet a11y verification»'s.
@leaf_edit n=16 leaf=.claude/rules/a11y.md line=22 class=retired
leaf-text: The page's contrast pairs are deferred with the frontend to 0.2.0.
source: fct-qmgxjgnl «N/A — viola has no web or graphical frontend, so no page token pair or layout to verify.»
replace-with: viola has no web or graphical frontend, so no page token pair or layout to verify.
@leaf_edit n=17 leaf=.claude/rules/a11y.md line=25 class=stale
leaf-text: 0.1.0 runs no browser a11y leg.
source: fct-ktqfqjb4 «No browser a11y leg: viola renders no page.»
replace-with: no browser a11y leg: viola renders no page.
@leaf_edit n=18 leaf=.claude/rules/a11y.md line=26 class=retired
leaf-text: the web row shape is deferred with the frontend to 0.2.0.
source: fct-hn7hmb2g «N/A — viola has no web or graphical frontend.»
replace-with: there is no web row shape: viola has no web or graphical frontend.
