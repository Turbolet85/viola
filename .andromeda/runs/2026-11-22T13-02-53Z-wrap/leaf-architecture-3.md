@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=59
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=26 class=stale
leaf-text: human bytes are never blocked beyond the current atomic paste (`ESC[200~…ESC[201~` + CR, one write)
source: dec-zzgfrbyq «Human keystrokes are never blocked, refused or delayed past the current paste's in-flight chunk and closing marker»
replace-with: human bytes are never blocked, refused or delayed past the current paste's in-flight chunk and closing marker (a send is one bracketed-paste write `ESC[200~…ESC[201~` + CR, human bytes arriving during it held and passed on straight after; a paste not finished within the compiled 5 s paste bound is cut: the writer stops after its in-flight chunk of at most 4096 bytes and writes the closing marker with no CR)
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=27 class=omission
leaf-text: `pause` → `manual-pause`; only CLI `release` returns the wheel;
source: dec-bfwj243p «A cut sets an in-memory mark at the wheel's human-key count; while the count is unchanged the next send is refused not-delivered paste-cut with no cursor»
replace-with: `pause` → `manual-pause`; a paste cut at its bound sets an in-memory mark at the wheel's human-key count, and while the count is unchanged the next send is refused `not-delivered` / `paste-cut` with no cursor, the mark cleared by a human key read, a `prompt-submitted` of any origin, a `session-start` or a `session-end`, and left by a key-less `release` or a `turn-ended`; only CLI `release` returns the wheel;
