@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=36 class=stale
leaf-text: so a decision flows only when every relied row passes (a stamp missing a relied row or holding a failing one
source: dec-t4wfjgzz «No non-null answer decision is sent without a viola verify stamp for the child's CLI version on which every relied ledger row counts: it reads pass, or absent on the browser-tools-prompt row alone»
replace-with: so a decision flows only when every relied row counts: it reads pass, or absent on the browser-tools-prompt row alone (a stamp missing any other relied row or holding a failing one
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=73 class=stale
leaf-text: (used-config markers and the browser-tools prompt turned off)
source: dec-4yctyxtx «its used-config markers and, on every arm but VP, the browser-tools prompt turned off come from the CLI bundle's own logic; VP leaves the browser-tools key out.»
replace-with: (used-config markers and, on every arm but VP, the browser-tools prompt turned off; VP leaves the browser-tools key out)
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=73 class=omission
leaf-text: dropping the bytes and keeping one closed word.
source: dec-m33lb4da «The live rig may read whether the key name cachedChromeExtensionInstalled stands in its own private scratch config, printing written or absent, and never reads a value.»
replace-with: dropping the bytes and keeping one closed word. The live rig may read whether the key name `cachedChromeExtensionInstalled` stands in its own private scratch config, printing written or absent, and never reads a value (a boundary widening, answered for the rig alone).
@leaf_edit n=4 leaf=.claude/rules/security.md line=19 class=omission
leaf-text: A non-`null` dialog decision requires a `viola verify` stamp for the child's CLI version (exception:
source: dec-t4wfjgzz «No non-null answer decision is sent without a viola verify stamp for the child's CLI version on which every relied ledger row counts: it reads pass, or absent on the browser-tools-prompt row alone»
replace-with: A non-`null` dialog decision requires a `viola verify` stamp for the child's CLI version on which every relied ledger row counts: it reads pass, or absent on the browser-tools-prompt row alone; absent on any other row leaves the version unverified (exception:
