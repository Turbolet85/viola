# P5 controls, revision 1 — entry 17 (the runtime-dir read)

Read at 2026-11-20T12:51:44Z on the tree (HEAD `1e9f1ea3`, the stopped run's edits uncommitted; entry 17 reads no
file of the tree). Entry 17 is a `leg = 'round'` entry: it is baseline-exempt (`gate.py baseline` reads `leg round —
exempt`) and `gate.py run --entry` does not fire a round entry, so each control TYPES the entry's own `run` text,
unchanged, as the tool types an entry — `bash -o pipefail -c '<run>'` from the root — the input operand alone
differing: the `XDG_RUNTIME_DIR` value it reads (validation.md check 4 (9), the typed form). The inputs were made in
the session scratchpad, `$TMPDIR/{scratchpad}/xdg17/`, never in the tree or a run dir.

A first form, a script in this run dir with a `mktemp -d` and an `rm -rf` trap, was refused by the permission
classifier and never ran; it was removed and the same cases were fired one call, visibly.

| input (`XDG_RUNTIME_DIR`) | reading | side |
|---|---|---|
| the host value as inherited (`/run/user/1000`, uid 1000, 0700) | exit 0 · `runtime-dir: owner-only` | must-pass green |
| a fresh dir of the user at 0700 | exit 0 · `runtime-dir: owner-only` | must-pass green |
| unset (`env -u`) | exit 1 · no output | planted red |
| empty | exit 1 · no output | planted red |
| an absent path | exit 1 · no output | planted red |
| a regular file at 0700 | exit 1 · no output | planted red |
| a symlink to a 0700 dir of the user | exit 1 · no output | planted red |
| a dir of the user at 0755 | exit 1 · no output | planted red |
| a dir of the user at 0770 | exit 1 · no output | planted red |
| another owner's dir at 0700 (`/var/lib/private`, root) | exit 1 · no output | planted red |

Every red side reads `exit 0` ✗ and `last line` ✗ against `expect = ['exit 0', 'last line runtime-dir: owner-only']`.
