@leaf_group group=architecture+security-plan
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/api.md line=23 class=retired
leaf-text: compares, on Linux and Windows, the connected stream's peer pid
source: fct-ju3c6nzj «then on Linux checks the serving process's identity before the frame»
replace-with: compares, on Linux, the connected stream's peer pid
@leaf_edit n=2 leaf=.claude/rules/api.md line=23 class=retired
leaf-text: one it cannot read reading `server-verify-failed`; Windows reads no euid, its DACL and the pid comparison the controls)
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: one it cannot read reading `server-verify-failed`)
@leaf_edit n=3 leaf=.claude/rules/api.md line=23 class=retired
leaf-text: `hook-decision` line carrying `server-verify-failed`); macOS reports no peer pid and compares no pid or start time, the named carve-out, owned by no route entry, holding the directory and euid checks, so `server-verify-failed` there means an euid mismatch alone.
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: `hook-decision` line carrying `server-verify-failed`).
@leaf_edit n=4 leaf=.claude/rules/api.md line=28 class=retired
leaf-text: `NO_COLOR` present, `TERM` exactly `dumb` or a failed Windows VT enable
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: `NO_COLOR` present or `TERM` exactly `dumb`
@leaf_edit n=5 leaf=.claude/rules/api.md line=28 class=retired
leaf-text: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
source: fct-54jxzo44 «`<code>` one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` ·»
replace-with: `stale-heartbeat` · `pinned-hash-mismatch`
@leaf_edit n=6 leaf=.claude/rules/api.md line=28 class=retired
leaf-text: in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`
source: fct-zxepdepr «`refused_with` (`src/cmd/run.rs`) prints `refused_document` in place of the human pair for `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`»
replace-with: in place of the human pair for `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`
@leaf_edit n=7 leaf=.claude/rules/api.md line=29 class=retired
leaf-text: `batch-script-child` · `program-not-found` · `pinned-hash-mismatch`
source: fct-tg3zks37 «(`src/cmd/verify.rs` `Ended::Refused`): `program-not-found` · `pinned-hash-mismatch` · `version-unreadable` · `strict-modes-failed` · `unclean-fixture`, beside `interrupted`.»
replace-with: `program-not-found` · `pinned-hash-mismatch`
@leaf_edit n=8 leaf=.claude/rules/api.md line=29 class=retired
leaf-text: on Linux and Windows the connected peer's pid or OS start time not the snapshot's
source: fct-ju3c6nzj «then on Linux checks the serving process's identity before the frame»
replace-with: on Linux the connected peer's pid or OS start time not the snapshot's
@leaf_edit n=9 leaf=.claude/rules/api.md line=30 class=retired
leaf-text: on Linux and Windows, the server check
source: fct-ju3c6nzj «then on Linux checks the serving process's identity before the frame»
replace-with: on Linux, the server check
@leaf_edit n=10 leaf=.claude/rules/api.md line=30 class=retired
leaf-text: and, on Linux and Windows, verify the serving process
source: fct-ju3c6nzj «then on Linux checks the serving process's identity before the frame»
replace-with: and, on Linux, verify the serving process
@leaf_edit n=11 leaf=.claude/rules/api.md line=30 class=retired
leaf-text: with no dated gap, macOS's pid and start-time check alone absent, the named carve-out, owned by no route entry, and so do
source: fct-hss5qy6y «No Windows or macOS code, test, fixture or CI job stands in the tree; that code is recoverable at the tag `multi-os-final` (`ecbe7ab3`).»
replace-with: with no dated gap, and so do
