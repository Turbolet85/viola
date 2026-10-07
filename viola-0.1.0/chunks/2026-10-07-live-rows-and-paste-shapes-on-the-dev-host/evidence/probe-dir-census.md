# Probe-dir census

Every live round of this chunk starts a real CLI in a `<repo root>/.viola-verify-<pid>/` dir that its scratch driver
makes and removes. This file names each count in words; the same readings are the `census` lines of
`live-sessions.ndjson`.

What is counted:
- the repository root's `.viola-verify-*` dirs, by name;
- the OS temp dir's `viola-verify-*` dirs, by count (Run A's class; no round of this chunk makes one);
- `own_left`: the dirs a round of this chunk made that still stand after it.

## Before anything moved (2026-10-07T09:58:40Z, `ls -ld .viola-verify-*`)
Eleven dirs at the root, none made by this chunk:
- `.viola-verify-2095228/`, dated 2026-10-05, empty: the operator desk's (the founder's word: leave it);
- ten dated 2026-10-06T22:12Z (local 2026-10-07 00:12): four `-dialogs` (`965892`, `966253`, `966441`, `966492`,
  each empty) and six `-plan` (`966289`, `966381`, `966390`, `966540`, `966569`, `966809`, one entry each).

The OS temp dir holds 0 `viola-verify-*` dirs.

None of the eleven is touched by this chunk: their disposition is the operator's (scope W5).

## After every live round
Each round's driver made exactly one dir, `.viola-verify-<its own pid>/` at 0700, and removed it on its exit path.
Each reading is `ls -d .viola-verify-*`, the OS temp dir's count, and every `claude` process's cwd read for a
probe dir.

| round | read at | made by the round | left by the round | at the root | in the OS temp dir | `claude` processes with a probe-dir cwd |
|---|---|---|---|---|---|---|
| start 1, the scratch session | 2026-10-07T10:06:34Z | 1 | **0** | 11, the same eleven | 0 | 0 |
| start 2, the hint on a verified home | 2026-10-07T10:14:52Z | 1 | **0** | 11, the same eleven | 0 | 0 |
| start 3, the hint on an unstamped home | 2026-10-07T10:15:49Z | 1 | **0** | 11, the same eleven | 0 | 0 |

The two rehearsals made dirs too and left none: the scratch driver's dry run against its stand-in (one dir), and
the hint driver's run against the fake agent (one dir; the `viola verify` that stamped its home made and removed
its own, by its drop guard).

No session of this chunk was killed, so no round met the kill-leftover case below.

## The kill-leftover finding (research M8, carried to the wrap)
A killed `viola verify` leaves the probe dir of the run it was in: the dirs are held by a drop guard and verify
handles no signal. A leftover's name carries a pid and no start time, so the repository's owner-record sweep (pid and
start time) cannot be applied to it as it is; a remover needs an owner record first. Nothing is built for it here.
