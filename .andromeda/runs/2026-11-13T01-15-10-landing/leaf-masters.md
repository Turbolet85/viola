@leaf_group group=masters
@leaf_read leaf=CLAUDE.md lines=131
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=24 class=stale
leaf-text: shim resolution
source: dec-zypgxbrh «All Claude-specific logic (hook JSON parsing, dialog answer mapping, the identity strip, child program resolution, the Claude half of the CLI version gate) lives only in `viola-agent-claude`»
replace-with: child program resolution
@leaf_edit n=2 leaf=CLAUDE.md line=37 class=omission
leaf-text: outside `VIOLA_*` plumbing a build reads only the `fake-agent` test seams and, in every build,
source: dec-u3r3pgro «except: NO_COLOR, TERM and COLORTERM for list's human-mode colour; XDG_RUNTIME_DIR for the socket dir; PATH for program lookup; two fake-agent test seams compiled only under the fake-agent feature.»
replace-with: outside `VIOLA_*` plumbing a build reads only the `fake-agent` test seams, `XDG_RUNTIME_DIR` (the socket directory) and `PATH` (program lookup) and, in every build,
@leaf_edit n=3 leaf=CLAUDE.md line=39 class=omission
leaf-text: set 0700 dirs / 0600 files explicitly (never the umask)
source: dec-qghazdrh «except: the pinned bin/<version>-<hash>/viola is created 0700, because at 0600 every hook and MCP exec would fail»
replace-with: set 0700 dirs / 0600 files explicitly (never the umask; the pinned `bin/<version>-<hash>/viola` copy is 0700)
@leaf_edit n=4 leaf=CLAUDE.md line=84 class=omission
leaf-text: whose post-condition is none or not yet measured on this CLI version
source: dec-zztbfb2d «except: A ledger-listed local command whose post-condition is none, not yet measured, or whose CLI is unverified returns `ok` with `confirmed: false` and detail `unconfirmable`.»
replace-with: whose post-condition is none or not yet measured, or whose CLI is unverified
