@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: the rest blanked, and another box row is refused
source: dec-pvviak6k «A screen fixture keeps only signature rows (a row holding a compiled footer or modal literal whole, a box rule row as its rule chars alone, a prompt row as the prompt char alone), the rest blanked; another box row, a kept row the scrub would rewrite or a username split across a row seam is refused.»
replace-with: the rest blanked, and another box row, a kept row the scrub would rewrite or a username split across a row seam is refused
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: after the same scrub (its `record` capture 0600 in a private directory, never committed);
source: dec-w6trh2g6 «viola-hypr-tap keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag); every other string becomes the placeholder, a non-JSON reply stays only as ok, then the shared scrub runs and an unclean set is refused whole.»
replace-with: after the same scrub (its `record` capture 0600 in a private directory, never committed); the test-only `viola-hypr-tap` keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag), every other string the placeholder and a non-JSON reply only `ok`, then the shared scrub runs and an unclean set is refused whole;
@leaf_edit n=3 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: its `viola mcp` a direct spawn with no shell and no PATH lookup);
source: dec-w6trh2g6 «viola-hypr-tap keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag); every other string becomes the placeholder, a non-JSON reply stays only as ok, then the shared scrub runs and an unclean set is refused whole.»
replace-with: its `viola mcp` a direct spawn with no shell and no PATH lookup); the test-only `viola-hypr-tap` keeps a reply string only under a closed per-request key allowlist in the form the seam parses (window addresses, the v-digit version tag), every other string the placeholder and a non-JSON reply only `ok`, then the shared scrub runs and an unclean set is refused whole;
@leaf_edit n=4 leaf=.claude/rules/security.md line=22 class=omission
leaf-text: the Hyprland seam's window Address validator lives in the seam crate); the wrapper re-runs them
source: dec-o6xntrql «The hyprctl child runs with null stdin and the named environment variables removed, under a 5 s deadline at which it is killed and reaped; a deadline kill or a non-zero exit reads as unreachable.»
replace-with: the Hyprland seam's window Address validator lives in the seam crate, whose `hyprctl` child runs with null stdin and the named environment variables removed, killed and reaped at a 5 s PROVISIONAL deadline, a deadline kill or a non-zero exit reading as unreachable); the wrapper re-runs them
@leaf_edit n=5 leaf=.claude/rules/security.md line=41 class=omission
leaf-text: Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves.
source: dec-fv2lrqu3 «Hyprland reply types hold no text field: window titles, classes and every other reply string are dropped at the parse and reach no log line and no error.»
replace-with: Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves. Hyprland reply types hold no text field: window titles, classes and every other reply string are dropped at the parse and reach no log line and no error.
