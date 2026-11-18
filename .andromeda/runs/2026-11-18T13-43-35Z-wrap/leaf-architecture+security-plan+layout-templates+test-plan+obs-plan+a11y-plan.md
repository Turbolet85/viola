@leaf_group group=architecture+security-plan+layout-templates+test-plan+obs-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=134
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=38 class=stale
leaf-text: only `NO_COLOR` / `TERM` / `COLORTERM` for `list`'s colour, `XDG_RUNTIME_DIR`, `PATH`, two `fake-agent` test seams and `CLAUDE_CONFIG_DIR`'s value, read by `viola verify` only when `claude_env_keep` keeps that name, are read
source: dec-u3r3pgro «NO_COLOR, TERM, COLORTERM for colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent-only test seams; CLAUDE_CONFIG_DIR's value, read when kept by verify, run and revive.»
replace-with: only `NO_COLOR` / `TERM` / `COLORTERM` for colour, `XDG_RUNTIME_DIR`, `PATH`, two `fake-agent` test seams and `CLAUDE_CONFIG_DIR`'s value, read by `viola verify` and by the `viola run` / `viola revive` start only when `claude_env_keep` keeps that name, are read
