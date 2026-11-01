## 2026-11-01-unix-endpoint-and-home-hardening — the board's one rule on the home
**Section:** Surface: cli › Component Patterns (design-system-detail/surface.md: the cli-list record of the board's strict-modes scan)
**Change:**
- The scan: was holding the home to two rules, the write rule and then, through each instance's set, the read rule; now the home to the read rule on every OS and `instances/` to the write rule, then each instance's set.
- A home another user may read: was passing as the empty board with no instance and refusing with the instance-state pair with one, the pair depending on whether an instance exists; now refuses the whole board with the home-state pair, with or without an instance.
- The order and the no-open rule hold.
- The record's pending clause, a CARRY on the working-route entry «Unix endpoint and home hardening», leaves.
**Why:** `scan` now takes the home through the read check and `instances/` through the write check on every OS, and the refusal is witnessed with and without an instance on all three OSes. The rule is the overseer's technical answer as the founder's delegate (`inputs#I2` Q3), PROVISIONAL, a refusal tightened and no widening; the founder's own word is owed at the batch at Epoch 7's boundary.
**Kept:** this item carries no PROVISIONAL mark: one proposed here was rejected, a status standing once, at its owner.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a refused socket directory at the eight frame verbs
**Section:** Surface: cli › Component Patterns (design-system-detail/surface.md: the `instance-unreachable` hint list, the two cli-send items on the hint cause and the refusal, exit-code row 21's human cell)
**Change:**
- The `instance-unreachable` hint list gains: the socket directory is refused: the hint of `run`'s socket-directory pair.
- The hint causes beside `server-verify`: was `not-running`, `unwrapped-name` and `endpoint-lost`; now those and `socket-dir`.
- A new item: a refused socket directory at `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` or `unlink` exits 21 before any frame: `unable  <name>  instance-unreachable` (`send` in its mirror's form), then the hint of cause `socket-dir`, no strict-modes pair; under `--json` the `instance-unreachable` document with `detail` `socket-dir-refused`, stderr empty.
- Row 21's human cell: was the hints of not running, unwrapped, endpoint lost and server verification; now those and socket directory.
**Why:** the client's `connect`, which all eight verbs call, checks the recorded endpoint's directory before it connects, and a refusal reads hint cause `socket-dir`. The closed word is the founder's own answer (`inputs#I2` Q1), live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the overseer, as the founder's delegate, PROVISIONAL, confirmed this item as lying inside that answer, until the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket-directory pair at the start of run and revive
**Section:** Surface: cli › Component Patterns (design-system-detail/surface.md: `viola run`'s and `viola revive`'s exit-1 refusals, the cli-send OPEN marker, exit-code row 1's human cell)
**Change:**
- `viola run`'s start refusals gain `unable: the socket directory for <name> cannot be trusted` (before any snapshot, child or bind) → `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`.
- `viola revive`'s refusals gain: a refused socket directory at its start: `run`'s socket-directory pair.
- Row 1, `run`'s list gains the new `unable:` line.
- Row 1, `revive`'s list: was naming `<name>'s state files can be written by another user` and `the viola home is not private to you` itself, beside `run`'s `already live` pair; now its four own lines (no logged session, no logged session with that id, session open in another claude, recorded directory missing) and `run`'s `already live`, home, instance and socket-directory pairs.
- A new marker: `OPEN — the socket-directory pair's wording is a planning form · owner: the Epoch 7 boundary batch`.
**Why:** `run` and `revive`'s shared start refuse the directory before any snapshot, child or `Server::bind`. The wording landed as a planning form on the overseer's P5 word as the founder's delegate (`inputs#I3`), PROVISIONAL; the founder's own wording is owed at the batch at Epoch 7's boundary. `revive`'s shared pairs fold because row 1 stood 9 characters under the 2 000-character line limit; each literal stays on the row in `run`'s list.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `socket-dir-refused`, exit 21's sixth cause and a log code
**Section:** Surface: cli › Component Patterns (design-system-detail/surface.md: the hints paragraph with the D-20 sentence, exit-code row 21's `--json` cell; design-system.md: pattern 2, Exit-code phraseology)
**Change:**
- Exit 21's document in surface.md and pattern 2: was one of five closed words; now one of six.
- The cause list in row 21 and Exit-code phraseology: was `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; now those and `socket-dir-refused`.
- D-20's log detail codes gain `socket-dir-refused` after `server-verify-failed`, written at the eight frame verbs' exit 21 and at `run`'s and `revive`'s start.
- The socket-directory half of the D-20 sentence's last clause: was the cause standing without these codes, owner the route entry "Unix endpoint and home hardening"; now gone, the code built.
**Why:** `viola_core::UnreachableCause::SocketDir` reads `socket-dir-refused`, the sixth cause. The word is the founder's own answer (`inputs#I2` Q1), live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the overseer, as the founder's delegate, PROVISIONAL, confirmed the D-20 sentence and the two cause lists as lying inside that answer, until the founder's own word in the batch at Epoch 7's boundary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `server-verify-failed` on macOS and a refused peer euid
**Section:** Surface: cli › Component Patterns (design-system-detail/surface.md: the D-20 sentence, the cli-send macOS item, the cli-exit-codes server-verification item)
**Change:**
- The macOS item: was the server check passing with no comparison, the hint unable to print there, a recorded gap owned by the route entry "Unix endpoint and home hardening"; now the check compares no pid (a named gap, owned by no route entry), compares the peer's euid, and a mismatch alone prints the server-verification hint there.
- The D-20 sentence: was macOS's server check standing without these codes under that owner; now on macOS `server-verify-failed` means an euid mismatch alone: no peer pid is compared, a named gap no route entry owns.
- Exit 21's server-verification cause: a process that is not the recorded wrapper was one whose pid or start time differs, or the check cannot be made; now also, on Linux and macOS, a peer whose euid is not this user's or cannot be read, on macOS the euid alone.
**Why:** the client euid check is built, and a refused euid reads `server-verify-failed`. That word on Linux and macOS is the founder's own answer (`inputs#I2` Q2), live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; on macOS it means an euid mismatch alone, the pid and start-time comparison staying absent, the named carve-out.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `revive --json`'s eleventh code
**Section:** Surface: cli › Component Patterns (design-system-detail/surface.md: the cli-verify-run `revive --json` items; design-system.md: pattern 5, the `revive` paragraph)
**Change:**
- `revive`'s closed codes in both files: was ten, `squatted-name` then `internal-error` last; now eleven, `socket-dir-refused` after `squatted-name`, `internal-error` last.
- A new surface.md item: `revive --json`'s socket-directory refusal is one stdout document with the code `socket-dir-refused`, no hint, stderr empty; a failed create of the directory and a refused check read that one code.
**Why:** `revive`'s start shares `run`'s socket-directory refusal. The eleventh code is the founder's own answer (`inputs#I2` Q1), live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the overseer, as the founder's delegate, PROVISIONAL, confirmed these three edits as lying inside that answer, until the founder's own word in the batch at Epoch 7's boundary. One code for a failed create and a refused check is a lean the overseer's P5 word as the founder's delegate took (`inputs#I3`), PROVISIONAL.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the wrapper's silent drop of a refused peer
**Section:** Surface: cli › Component Patterns (design-system-detail/surface.md: the cli-verify-run items)
**Change:**
- A new item: a peer the wrapper refuses is dropped before any frame: no line written, nothing on the terminal the child holds, no wheel moved; the client reads end-of-file.
**Why:** the wrapper's accept loop filters each stream by its peer's euid and drops a refused one before its worker thread and before any frame. That the drop writes no line is a lean the overseer's P5 word as the founder's delegate took (`inputs#I3`), PROVISIONAL.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/
