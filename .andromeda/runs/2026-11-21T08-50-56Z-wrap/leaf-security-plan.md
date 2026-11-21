@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAM
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=73 class=stale
leaf-text: The live rig keeps a stamp from its CS, WS or PQ scratch home owner-only and no-clobber
source: dec-cgqgi7dg «The live rig keeps a stamp from its CS, WS, PQ, HF or HG scratch home owner-only and no-clobber, and seeds it only into its own CC or R3W scratch home»
replace-with: The live rig keeps a stamp from its CS, WS, PQ, HF or HG scratch home owner-only and no-clobber
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=71 class=retired
leaf-text: (12 names, four outside the 11-name floor, the floor unchanged)
source: dec-7bf5jzrq «CLAUDE_PLUGIN_ROOT, CLAUDE_PROJECT_DIR, CLAUDE_ENV_FILE and CLAUDE_PLUGIN_DATA, names the CLI adds to a hook, are known and not identity names: they stand off the identity floor and are read only by verify's env reading.»
replace-with: (12 names; the hook-added `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`, `CLAUDE_ENV_FILE` and `CLAUDE_PLUGIN_DATA` are known, not identity names, stand off the identity floor and are read only by verify's env reading)
@leaf_edit n=3 leaf=.claude/rules/security.md line=13 class=omission
leaf-text: no `VIOLA_*` read, no obs init, no channel, exit 0 always.
source: dec-sxexddjn «The hidden capture arm reads no VIOLA_* variable, opens no channel, adds no env var and always exits 0; on SessionStart alone it writes an owner-only file of its own process's env names, CLAUDE names kept and every other name only counted, never reading a value.»
replace-with: no `VIOLA_*` read, no obs init, no channel, no env var added, exit 0 always; on SessionStart alone it also writes an owner-only file of its own process's env names, `CLAUDE*` names kept and every other name only counted, never reading a value.
@leaf_edit n=4 leaf=.claude/rules/security.md line=6 class=omission
leaf-text: None may reach a log, diagnostic, event, snapshot, fixture or error body.
source: dec-iqzweh6b «The verify stamp's env probe holds names and counts only (read, added, unknown, other count, unshaped), never a value, and names kept by claude_env_keep are removed before it is written.»
replace-with: None may reach a log, diagnostic, event, snapshot, stamp, fixture or error body; the verify stamp's env probe holds names and counts only (names kept by `claude_env_keep` removed before it is written), and the recorded env fixture only its version and the hook-added `CLAUDE*` names.
