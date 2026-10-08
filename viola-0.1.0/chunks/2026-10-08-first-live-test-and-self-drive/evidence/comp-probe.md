# A compositor of the chunk's own — the revision's measurement (2026-10-08, 06:50Z to 06:53Z)

The founder's ruling of 2026-10-08T06:43Z (inputs#I12): the takeover key is typed in a compositor this chunk
starts itself, a nested or headless Hyprland with a minimal config and a socket of its own, foot and wtype inside
it; the desktop lock stays untouched. Whether it runs here was unmeasured. Three runs of one script over the
harness `viola` and the fake agent, no live `claude` start (0 of 8). Every `hyprctl` call named the own instance
(a private runtime dir and `--instance <its signature>`); the desktop compositor was never asked and never
dispatched to. Script, config and logs: `.andromeda/runs/2026-10-08T06-46-31-phase/` (`comp-probe.sh`,
`comp-min.lua`, `comp-probe-1.log` to `-3.log`).

## The form

- Private runtime dir `$XDG_RUNTIME_DIR/vcomp` (0700): the own instance's control socket and Wayland socket live
  there and nowhere else.
- The own compositor is started with an emptied environment (`env -i`): no session bus address, the seat backend
  forced to an absent `seatd` (it opens no seat and never asks logind), `HYPRLAND_NO_SD_VARS=1`. Config
  `comp-min.lua`: no autostart, no bind, no XWayland, no shell, no locker.
- foot is started by the script with a declared environment (no `CLAUDE*`, no `VIOLA_*` name), the desktop's
  runtime dir (viola's endpoint dir is resolved from it) and `WAYLAND_DISPLAY` naming the own socket by absolute
  path. `wtype` is pointed at the same socket.

## Readings

| run (UTC) | form | result |
|---|---|---|
| 06:50:57 | headless: no parent compositor | the compositor aborted 0.2 s after start, `CBackend::create() failed!`; no socket, no window, no key. It left one crash record with a core file in the system's coredump store |
| 06:51:50 | nested: the own compositor is one client window of the desktop compositor | **the key probe passed**, see below |
| 06:53:02 | headless again, the own instance's log on | the same abort; its log is empty, so the reason is not named. One more crash record (no core file) |

The nested run, in order: own instance up in 0.2 s, its signature not the desktop's, one output `WAYLAND-1`
621x688, `locked` false, 0 windows; foot started; the wrapper's three start records (`wheel`, `budget-gate`,
`session-start`); the probe window listed `mapped: True`; it took focus by itself (active class
`viola.keyprobe`, the address equal); `wheel` records 1 before and 1 after the focus (delta 0); the guard passed
(the own instance's active window is the probe, it holds 1 window, it is not locked); `wtype x` exit 0; a second
`wheel` record `{"cause": "human-input", "holder": "human"}` within 4 ms; the fake agent's receipt holds `key`
`78`; a driver `send` exit 10, stderr `[/ ] unable  keyprobe  human-typing` then
`hint: the human has the wheel; send again after the human hands it back`; 0 `send-issued` records. foot, the
wrapper and the own compositor were ended by pid and read gone.

## What the nested form did to the desktop

Read from the desktop shell's journal and its own lock answer, never from the desktop compositor:

- The own compositor's window appearing on the desktop counted as activity for the desktop's idle monitor
  (`idle-monitor: active` at 06:51:50.951Z, 0.2 s after the start), and the shell ran its wake script.
- 4 s later the desktop shell, the process that holds the lock, lost its Wayland connection
  (`Got removal for monitor "FALLBACK"`, `The Wayland connection experienced a fatal error: Invalid argument`),
  exited 255 and was relaunched by its supervisor (06:51:55Z).
- The relaunched shell logged `lock-stranded: recovering` and `secure=true` at 06:51:57.254Z, 2.2 s after the
  exit. Its answer `lock isLocked` reads `true` since. The wording `lock-stranded` says the compositor still
  reported the session locked while no shell held it; the compositor's own lock state was not read here.
- The same exit and relaunch stand in the journal three more times today: 05:22:54Z, 06:48:11Z and 06:55:07Z,
  each within seconds of `idle-monitor: active`. None of the three is attributed here: the first falls in the
  phase's DPMS test window; nothing of this run was touching the desktop at the second; the third came two minutes
  after this run's last compositor had ended (06:53:03Z), 4 s after the shell's own wake script started, and the
  shell logged `lock-stranded: recovering` and `secure=true` 2.2 s after its exit again. One wake, at 06:31:35Z,
  did not end the shell. So the shell's exit after a wake on this locked desktop also happens with no compositor of
  this chunk running.
- Unchanged before and after every run: the user manager's `WAYLAND_DISPLAY`, `HYPRLAND_INSTANCE_SIGNATURE` and
  `XDG_CURRENT_DESKTOP` (one hash), 1 desktop instance dir, 1 desktop Wayland socket, 1 `Hyprland` process once
  the own one is gone.

So the nested form proves the key and is not free: each start of the own compositor wakes the screens of the
locked desktop and, in the one nested run made, was followed by the lock holder's exit and relaunch. The headless
form, which would open no window on the desktop, does not start on this build as tried.

## Left

- `$XDG_RUNTIME_DIR/vcomp/` with three instance dirs under `hypr/` and a `dconf/` dir (a tmpfs, gone at logout).
- Three scratch homes on the tmpfs behind the link: `target/e2e-home/viola-keyprobe-20261008T065057Z/`,
  `-065150Z/`, `-065302Z/`.
- Two crash records of `Hyprland` in the coredump store (08:50:57 and 08:53:02 local), one with a core file.
- No process and no window of this chunk (read by pid).
