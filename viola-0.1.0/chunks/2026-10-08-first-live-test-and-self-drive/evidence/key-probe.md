# Key probe — step 0 (2026-10-08, 06:29Z to 06:34Z)

**Outcome: not passed. No key was typed, into any window. No live `claude` start was made (0 of 8).**

The probe window cannot take keyboard focus because the desktop session is locked. This is measured below. It
is the cause the four runs of the phase (research M1) and the overseer's measurements (inputs#I10) did not name.

## What was measured

| when (UTC) | reading | result |
|---|---|---|
| 06:29 | `hyprctl locked` | `true` (`-j`: `{"locked": true}`) |
| 06:29 | the shell's own answer, `omarchy-shell lock isLocked` | `true` |
| 06:29 | the shell's journal, lock lines of the last 36 h | last `unlocked` 2026-10-07T19:08:23Z; `lock-requested` then `secure=true` 2026-10-07T19:39:15Z (the idle lock timeout); the shell restarted 2026-10-08T05:22:54Z, logged `lock-stranded: recovering` and `secure=true` at 05:22:57Z. No `unlocked` line since 19:08:23Z |
| 06:29 | locker processes | no separate locker: the lock is held by the desktop shell (`quickshell`, the `lock` plugin) |
| 06:31:33 | `evidence/focus-diag.sh`, one plain foot window of this chunk's own (class `viola.focusprobe`, running `sleep`), DPMS turned on first | both monitors `dpmsStatus` true; workspace 9 on the second monitor, empty; the window listed `mapped: True`, `focusHistoryID` 15 |
| 06:31:37 | the window 1.2 s after it mapped | did not take focus by itself: active window class `overseer.viola-overseer` |
| 06:31:38 | `hl.dsp.focus({ window = "address:<probe>" })` | answered `ok`; the cursor moved to the probe window's centre and the second monitor became the focused monitor; the active window stayed `overseer.viola-overseer`; the probe's `focusHistoryID` stayed 15 |
| 06:31 | the compositor's own log for the refusal | not readable: `debug:disable_logs` is true on this desktop; setting it false through `hyprctl eval` changed the option's value and the compositor still wrote none of its own lines (only its DRM backend's). The option was put back to true |

So every focus reading of this chunk, from 05:18Z on, was taken on a locked session: the lock was taken at
2026-10-07T19:39:15Z and has not been released since.

## What follows

- Read from the compositor's behaviour, not from its source or log here: while a session lock is held the
  compositor gives keyboard focus to no window, and a key typed through the compositor goes to the lock screen,
  not to a terminal. The correlation is measured (locked, and no focus change by any form); that an unlocked
  session lets the same dispatch move focus is not measured here.
- The guard (the active window's address read before `wtype`) is what kept a key out of the lock screen's
  password field in every run. It stays as written.
- No lever was taken past the lock. Unlocking the session takes the founder's password or fingerprint; a lock
  screen is an access control and nothing in this chunk steps around it.
- Step 0 cannot pass while the session is locked: stop rule S1. The fake-agent key probe (`wtype x`, the `wheel`
  record, the receipt's `78`, the refused `send`) was not reached.

## Left as found

- The probe window's own foot process was ended by pid and read gone; 0 windows of class `viola.focusprobe` or
  `viola.keyprobe` are listed.
- The second monitor is back on workspace 7, the first on workspace 4; DPMS reads false on both again.
- `debug:disable_logs` reads true again (it now reads as explicitly set; its value is the one found).
- Two things differ from the state found, neither undone because undoing them needs a focus dispatch at a session
  window: the focused monitor is the second one (it was the first), and the cursor stands at the second monitor's
  centre.
- The DPMS-on dispatch counted as activity for the shell's idle monitor (`idle-monitor: active` at 06:31:35Z, its
  wake script ran). The session stayed locked throughout (`hyprctl locked` true after the restore).
