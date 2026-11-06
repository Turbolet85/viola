@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=31 class=retired
leaf-text: then, on Unix, refuses a connected peer of another euid
source: fct-wular7qm «both sides admit only a peer of the owning user's euid, the wrapper dropping another before any frame»
replace-with: then refuses a connected peer of another euid
@leaf_edit n=2 leaf=.claude/rules/events.md line=31 class=retired
leaf-text: then, on Linux and Windows, compares the connected peer's pid
source: fct-y7p4rnnj «`viola_state::liveness::server_verdict` and `verify_server` always compare the connected peer's pid and OS start time with the snapshot's: no verdict reads `Matches` without that comparison.»
replace-with: then compares the connected peer's pid
@leaf_edit n=3 leaf=.claude/rules/events.md line=31 class=retired
leaf-text: macOS reports no peer pid, so `server_verdict` returns `Matches` with no comparison: the recorded gap, owned by no route entry; the socket-directory and euid checks stand built there, so on macOS `server-verify-failed` means an euid mismatch alone.
source: fct-y7p4rnnj «`viola_state::liveness::server_verdict` and `verify_server` always compare the connected peer's pid and OS start time with the snapshot's: no verdict reads `Matches` without that comparison.»
replace-with: `server_verdict` and `verify_server` always make that comparison: no verdict reads `Matches` without it.
@leaf_edit n=4 leaf=.claude/rules/events.md line=35 class=stale
leaf-text: Lock files are separate siblings (`<name>.lock`): append + exclusive lock fails on Windows.
source: fct-73drmylz «lock files are separate, a layout chosen while Windows (where append + exclusive lock fails) was a target, and kept.»
replace-with: Lock files are separate siblings (`<name>.lock`): a layout chosen while Windows (where append + exclusive lock fails) was a target, and kept.
