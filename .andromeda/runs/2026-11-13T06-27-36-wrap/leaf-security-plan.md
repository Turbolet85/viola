@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/ci.md lines=35
last: - 2026-10-13: A CI job that reads `cancelled` with no steps 
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/security.md line=31 class=stale
leaf-text: no advisory is ignored (`ignore = []`; portable-pty pinned `=0.9.0`)
source: dec-wfyawxog «cargo-deny treats unmaintained and unsound advisories as failures, denies yanked crates and ignores exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, each in an rmcp feature banned or off»
replace-with: the advisory ignore list holds exactly RUSTSEC-2026-0457, RUSTSEC-2026-0461 and RUSTSEC-2026-0462, each in an rmcp feature banned or off in the release graph, its reason naming it, standing until rmcp leaves its below-3.5 range; portable-pty pinned `=0.9.0`)
@leaf_edit n=2 leaf=.claude/rules/security.md line=17 class=omission
leaf-text: each added one only into a settled input box, waited for up to the probe deadline
source: dec-fn2oz73t «The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a modal literal stands»
replace-with: each added one only into a settled input box (read up only on compiled literals — a footer literal, or a rule row of at least 40 rule chars directly above a row opening the prompt char — and never while a modal literal stands; the one predicate that also decides `viola run`'s pre-send gate), waited for up to the probe deadline
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: each added one only into a settled input box, Ctrl-C ×2 then kill
source: dec-fn2oz73t «The input box reads up only on compiled literals, a footer literal or a rule row of at least 40 rule chars directly above a row opening the prompt char, and never while a modal literal stands»
replace-with: each added one only into a settled input box (read up only on compiled literals — a footer literal, or a rule row of at least 40 rule chars directly above a row opening the prompt char — never while a modal literal stands; one predicate for verify and `viola run`'s pre-send gate), Ctrl-C ×2 then kill
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=72 class=omission
leaf-text: Screen fixtures are signature-rows-only;
source: dec-pvviak6k «A screen fixture keeps only signature rows (a row holding a compiled footer or modal literal whole, a box rule row as its rule chars alone, a prompt row as the prompt char alone), the rest blanked; another box row»
replace-with: Screen fixtures keep only signature rows (a compiled footer or modal literal row whole, a box rule row as its rule chars alone, a prompt row as the prompt char alone), the rest blanked, and another box row is refused;
