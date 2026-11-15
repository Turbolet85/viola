@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=74
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=42 class=omission
leaf-text: `StampedHome::unstamped` for the unverified path;
source: dec-ioj2wnmy «A home a test gives to viola verify --record comes from the shared record-home helper and fixture, and no component of its path is named home.»
replace-with: `StampedHome::unstamped` for the unverified path; a home a test gives to `viola verify --record` comes from the shared record-home helper and fixture, and no component of its path is named `home` (`record_home_names_no_path_component_home`);
