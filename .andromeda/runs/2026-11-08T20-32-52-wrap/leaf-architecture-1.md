@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/gotchas.md lines=147
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/r
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fra
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=29 class=retired
leaf-text: GUI contracts: none in v1; the frontend is deferred to 0.2.0 and its technology is not chosen.
source: fct-ygzlcqsl «**GUI contracts.** None: viola has no web or graphical surface; the terminal fleet console's contracts (matrix#v2-14) are owed by the route entry «Console scaffold».»
replace-with: GUI contracts: none; viola has no web or graphical surface, and the terminal fleet console's contracts (matrix#v2-14) are owed by the route entry «Console scaffold».
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=35 class=retired
leaf-text: GUI: no HTTP error contract in v1; the frontend is deferred to 0.2.0.
source: fct-jn2hd4ac «viola has no web or graphical frontend. Its view beyond `viola list` is the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14)»
replace-with: GUI: no HTTP error contract; viola has no web or graphical frontend (its view beyond `viola list` is the terminal fleet console, matrix#v2-14).
@leaf_edit n=3 leaf=.claude/docs/gotchas.md line=109 class=stale
leaf-text: Owner: the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`).
source: fct-bq65okf4 «OPEN — whether the real `claude` on Linux loses a key typed right after a resize · owner: the measurement of the route entry «Key after a resize» (0.2.0, Epoch 4)»
replace-with: The open question is now whether the real `claude` on Linux loses a key typed right after a resize; owner: the measurement of the route entry «Key after a resize» (0.2.0, Epoch 4).
