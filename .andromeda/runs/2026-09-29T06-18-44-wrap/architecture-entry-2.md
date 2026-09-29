
## 2026-09-29-h2-conpty-resize-probe — the viola-pty-watch dir holds the test's own report beside the child's
**Section:** Occupied Resources → Filesystem (`<temp dir>/viola-pty-watch/`)
**Change:** the test-only row now registers two files per real-PTY test: `<test name>.report` (the child's lines — `start`, the key-free size watcher's `size`, `byte`, `restored`) and the sibling `<test name>.test.report` (the test's steps `resize-returned` · `key-written` · `key-flushed` · `dsr-cpr {n}`, the count of `ESC [ 6 n` the rig's output drain saw). Both kept on a panic or kill, both removed on a pass; `viola` never reads or writes either (was: the child's `.report` only).
**Why:** the H2 recorder needed the test side of the sequence to localise a lost key (resize never applied · resize applied and key lost · key before the size); codes and counts only, so no user content reaches either file.
**Ref:** .andromeda/runs/2026-09-29T06-18-44-wrap/
