# P3 block — 2026-11-23-seam-requests-for-viola-open (directive item 6)

**This block holds NO fork of the operator's: the phase goes on to P4 without waiting.**

## Window
- Run `.andromeda/runs/2026-11-23T13-35-35Z-phase/`, take-up tx 4279, scope rev 3 (closed), research rev 1 (`scope lists: parsed 15 items`). HEAD `c3403fa6`, CI green 7/7 on it and on `ddf3b411`. No live start, no window opened, moved, focused or closed, no key, no mutation run, no timer tool. Compositor reads made: `hyprctl locked` (true), `hyprctl version`, `hyprctl -j monitors` ×6, `hyprctl -j clients` ×1, all read-only.

## Measurements (research §Measurements; re-read today, or carried with its date)
- M1–M4 re-read at HEAD, each agreeing with split-carry measurement 1 (2026-11-23): the seam's 6 requests, 6 words, 5 errors; the tap's two verbs (5 + 2 files); the fake's exact-argv replay in 4 modes; `HYPRCTL_DEADLINE` 5 s never timed, its only test a 200 ms `read_within`.
- M5 re-read today: `monitors` 4–5 ms over 5 runs (carried: 4 ms × 10), 2 monitors, 37 keys each, its strings host text.
- M6 **corrected**: the carried «stubs name `hl.dsp.window.move_to_workspace`» reads an `HL.EventName` (`/usr/share/hypr/stubs/hl.meta.lua:25`); the dispatcher is `hl.dsp.window.move` (`HL.DspWindowNamespace`), and the packaged `/usr/share/hypr/hyprland.lua:278` binds `hl.dsp.window.move({ workspace = i })`. Its form with a window named by address is unmeasured: R1 measures it.
- M7 from code + no-window probes: foot 1.28.0's `--app-id`, `--title`, `--working-directory` and its trailing-argv command (carried 2026-11-23); `clients`' `pid` is the foot pid (carried 9 of 9; 9 clients today).

## The design each item leans to (decisive, each on an artifact)
- `Request::Monitors` — argv `-j monitors`, word `monitors`, a query; `Reply::Monitors(Vec<Monitor>)`, a `Monitor` of `id`, `width`, `height`, `x`, `y`, `focused`, `active_workspace` (numbers and bools only, every string dropped at the parse: security `decision/hyprland-reply-text-dropped`); unparseable → `malformed`, past the cap → `oversize` (arch `decision/unparseable-external-values-unknown` except). Its keep-list keeps **no** string: the ratified allowlist does not widen (security `decision/hyprland-recorder-string-allowlist`).
- `Request::MoveToWorkspace(Address, WorkspaceId)` — word `move-to-workspace`, a dispatch, its success the existing `ok` text (`Reply::Dispatched`, `refused` otherwise). Its argv is built now in the candidate form `dispatch` + `hl.dsp.window.move({ workspace = N, window = "address:0x…" })`, only a closed `Address` and an i32 entering it (arch `decision/hyprctl-direct-spawn`); R1 confirms the form or gives the one to put in its place.
- No new `HyprlandError` word, no new log subject or parser (the new spawn rides `hyprctl-probe` and `hyprctl-json`), no new env read, no fake-agent change (exact-argv replay serves both).
- `HYPRCTL_DEADLINE` stays 5 s, settled (F8, the operator's own answer); a test times `read` against it with a child that never answers: the read returns `unreachable` no sooner than 5 s and well under the 12 s `WITHIN`.
- The drain witness: a new `read.rs` case whose stub hyprctl writes without end — the read keeps exactly `MAX_FRAME + 1` bytes, reads `oversize`, and the child is killed and reaped; the `hyprctl reply` Boundary row is re-pointed to it.
- R3 in implement: a tap verb `record-monitors --program <abs> --out <dir>`, writing `Monitors.recorded-1.json` alone (the other 7 files untouched: `write_set` writes only what it is handed).
- R1's verb, built and rehearsed over planted stubs in implement, run live only at R1: `record-move --program <abs hyprctl> --terminal <abs foot> --workspace <N> --out <dir>` — spawns foot directly as `--app-id=viola.hypr-tap --title=viola-hypr-tap /usr/bin/sleep 30` (no claude, no shell), polls `clients` every 100 ms up to 5 s for the window whose `pid` is its child's, issues ONE `MoveToWorkspace` to `<N>`, reads `clients` once to see the window on `<N>`, kills and reaps its own child, reads `clients` again until no window holds that pid (5 s), and writes `MoveToWorkspace.recorded-1.json` alone. `<N>` is the operator's to name at R1.

## Every master row the change reaches (from the carried sweep, re-searched by the counted literals `hl.dsp`, «closed request set», `hyprctl-read-deadline`, `viola-hypr-tap`, `monitors` over every master)
- **Reached:** architecture `decision/hyprctl-direct-spawn` (names `hl.dsp.focus` as the only dispatch form: gains the move form), `decision/hyprland-seam-crate` (the closed set grows by two; words unchanged otherwise); security-plan `decision/hyprctl-read-deadline` (provisional → settled), `decision/hyprland-recorder-string-allowlist` (two words added, `monitors` keeping none, the move keeping `ok` only), `decision/parser-fuzz-scope` (stands: the new parser joins the hyprctl reply parsers); test-plan `decision/bounded-readers-keep-the-cap` (its witness moves; text stands), `decision/fixture-hygiene-walk` (stands, two words walked), `decision/fixtures-recorded-by-live-verify` (stands: both files from the tap).
- **Read, not reached:** obs `contract/diag-subject/hyprctl-probe`, `contract/parse-rejected/hyprctl-json` (reused as they stand); arch `contract/hyprland-seam-errors/*` (no new word); a11y `decision/time-bounds-fall-on-the-driver` / `-never-on-human` (the seam has no product caller: a bound with no product caller is not listed, a11y history n=5; «`viola open`» lands its bounds); every `viola open` row of the carried sweep (§5): its verb, flags, exits, window kind, snapshot field and help belong to «`viola open`».

## The forks that are the operator's
- None. The two boundary crossings this chunk adds — the `monitors` reply into the seam and the move dispatch into the compositor — are the ones the operator's own F9 answer named («the `monitors` query, the one move-to-workspace dispatch in the closed set»), so they reach the wrap as ONE answer card citing F9 (playbook `rul-bznresq5`), not a new question. R1 is his to run at his host, unlocked, at a moment he names; `<N>` is his to name then.

## Disk, SIGKILL, identity (directive item 7)
- **Disk:** the two recorded files, 0600 in the 0700 `fixtures/hyprland/0.56.2/`, committed: `Monitors.recorded-1.json` (numbers, bools and the placeholder; geometry 2560×1440 and monitor ids are kept), `MoveToWorkspace.recorded-1.json` (`args` holding one window address and one integer, `stdout` `ok`). No product file, no events line, no snapshot field.
- **What outlives a SIGKILL:** of the tap at R1, the foot window it opened, which closes by itself when its `sleep 30` ends (no orphan past 30 s); no fixture is written before the end (`write_set` runs last). Of a test: nothing (planted stubs in a tempdir, killed at `WITHIN`).
- **A parent's identity:** none: no path, session id or socket of any session enters a new record. The tap's app-id `viola.hypr-tap` is visible to any same-user `hyprctl clients` for at most 30 s.

## Test children (directive item 8)
- A planted `#!/bin/sh` stub hyprctl that writes without end (the drain case) and one that never answers (the timed case), each in a test tempdir, under `read_within`'s bound / the 5 s deadline, both under the 12 s `WITHIN` and the 20 s lint.
- In `crates/viola-e2e/tests/hypr_tap.rs`: a stub terminal (`#!/bin/sh`, writes its `$$` and sleeps) and a stub hyprctl whose `clients` names that pid, for `record-move`'s rehearsal; each child killed at the file's `WITHIN` 10 s.

## Does one implement window hold it?
- Yes: 12 modified files and 3 new, one crate's arms plus their readers. Implement stops once, before R1: R1 is the plan's one `leg = 'operator'` reading entry, fired in the operator pass; the entries that read its fixture follow it there. A refused R1 stops the pass and comes back as a phase revision with an implement re-entry (the letters' door).
