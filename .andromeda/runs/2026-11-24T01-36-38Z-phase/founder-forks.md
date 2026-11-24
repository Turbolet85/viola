# Founder forks — 2026-11-24-viola-focus («`viola focus`», chk-fw7hhw4r)

## P3 block — written 2026-11-24T02:11:07Z, before P4

**Boundary widening, first:** `viola focus` would be the FIRST product caller of the seam's two focus dispatches
(`hl.dsp.focus` on a window and on a workspace). Today the product dispatches only the wrapper's move of its own
terminal's window (`decision/window-step-moves-own-terminal-window`). Focus moves the operator's keyboard focus to
another session's window, and for a hand-started session that is a window viola did not place. That is a
subprocess crossing that gains a new act (playbook `rul-r74hshif`, `rul-p2zl2kgl`), and it is his to ratify. Its
conservative arm, taken PROVISIONAL here: CLI-only, a human verb with no MCP tool, raising only a LIVE wrapper's own
terminal window, and printing no window address or workspace. The entry can be built whole on that arm.

### The window
- This phase, run dir `.andromeda/runs/2026-11-24T01-36-38Z-phase/`: no live start, no window, no dispatch, no key,
  no read of the real `~/.claude*` (inputs#I2).
- `hyprctl locked` read `true` at 01:35:50Z (the directive's fact, inputs#I1); nothing in this phase needed the
  desktop.

### Measurements (research rev 1, `doc:2026-11-24-viola-focus/research`)
- The two dispatches at HEAD: `FocusWindow(a)` → `["dispatch", "hl.dsp.focus({ window = \"address:<a>\" })"]`,
  `Workspace(w)` → `["dispatch", "hl.dsp.focus({ workspace = <w> })"]` (crates/viola-hyprland/src/lib.rs:68-78).
  Both are recorded for 0.56.2, each `ok\n`, exit 0. Both stay within `decision/hyprctl-direct-spawn`'s measured
  forms, so no new form is needed.
- No product caller issues either dispatch at HEAD. The graph's `viola_hyprland::read` callers are open's `clients`
  read (src/cmd/open.rs:139) and the wrapper's window step (src/cmd/run.rs:549).
- The fake replays a dispatch only for the exact recorded argv (address `0x555a9c521790`, workspace 6). Any other
  argv exits 1, which reads as `unreachable`. Its `terminal` mode lists the stand-in foot at `0x555aa16bde50` on
  workspace 2, and it keeps no receipt of what it was asked. An E2E focus over the stand-in therefore needs a
  test-only fake change, and the order and argv of the dispatches are asserted at the verb's closure seam.
- Identity, opened session: the snapshot's `window.address`, re-parsed through `Address`, matched in a fresh
  `clients` read.
- Identity, hand-started `viola run`: no window record; `ancestors(snapshot.pid, 8)` (crates/viola-state/src/
  liveness.rs:61), the same function open walks. Open's matcher `target_of` returns a workspace only, so focus needs
  the client itself.
- A replayed snapshot has no pid, so it has no ancestry and no liveness.
- For an opened session whose terminal is still its parent, the recorded address and the first ancestry hit name
  the same client (E3, read from the code, not live).
- Outside Hyprland: with no `hyprctl` on PATH the seam reads `unsupported` and spawns nothing. With `hyprctl` and
  no compositor, it reads `unreachable` (security-plan `decision/hyprctl-read-deadline`: «a deadline kill or a
  non-zero exit reads as unreachable»).
- Order and what each failure leaves: lean workspace, then window. A failed first dispatch changes nothing viola
  did. A failed second leaves the workspace shown and the window not raised. The earlier focus is never restored:
  viola keeps no record of it, and a restore is a third dispatch on a window it did not open.
- Unmeasured live, the fakes' only: that the raise happens on screen; whether `hl.dsp.focus({window})` alone also
  switches workspace on 0.56.2; a special (negative-id) workspace; a footclient window, whose pid is the foot server
  and so not an ancestor; a host with `hyprctl` installed outside a Hyprland session.
- No no-focus probe exists: viola has no dry-run, and adding one is a new surface. The resolution can run without
  a dispatch only over the fake, as a rehearsal.

### Every reached row (searched in all seven masters, by the rows files and `rec.py find`)
Literals searched: «product caller» — 0 hits in 7 masters. «replays no» — 1 hit, architecture
`decision/revive-resumes-in-place`, read, not reached. `focus-window` / `FocusWindow` / `HYPRCTL_DEADLINE` — 0 hits in
7. «only/never … dispatch» — architecture `decision/hyprland-seam-crate`, reached and holding.
- architecture:
  - reached:
    - `decision/hyprland-seam-crate` (holds)
    - `decision/hyprctl-direct-spawn` (holds, no new form)
    - `decision/human-controls-cli-only` (amend: names `focus`)
    - `contract/cli-exit-codes/1` and `/21` (amend: name focus's refusals)
    - a new `name/command/focus`
  - read, not reached: `decision/window-features-target-hyprland`, `decision/lineage-label-only` (focus resolves by
    process ancestry, never by the declared parent).
- security-plan:
  - reached:
    - `decision/hyprctl-read-deadline` (binds the `unsupported` fork)
    - `decision/open-ancestor-walk-in-memory` (focus walks the same 8, in memory: a sibling row)
    - `decision/window-record-kept-to-snapshot-and-event-log` (focus prints no address or workspace, so it opens no
      new leave path)
    - `decision/state-tree-strict-modes-before-use` (amend: add `focus`)
    - a NEW row for the act itself: «focus raises only a live wrapper's own terminal window», no row covering it —
      the widening above
  - read, not reached: `decision/window-step-moves-own-terminal-window`, `decision/hyprland-recorder-moves-own-window`,
    `decision/window-origin-authenticates-nothing`.
- design-system / layout-templates:
  - reached:
    - `contract/help-output/verb-groups`, `contract/viola-help/verb-groups`, `contract/cli-verbs/help` (the help row,
      fork F-B)
    - `contract/cli-exit-codes/1` and `/21`
    - `contract/cli-refusal-line/fixed-unable-form`, `contract/cli-stderr/fixed-message-refusal` (the verb lists)
  - read, not reached: the eight placement words (focus coins no duplicate of them), `decision/no-hint-names-viola-release`.
- obs-plan:
  - reached:
    - `contract/process-exit-detail/exit-1` (new words)
    - `contract/cli-log-file/*` (a focus row, as open's)
    - `decision/distinct-detail-code-per-exit-cause` (binds)
  - read, not reached: `decision/span-naming` (`cli.focus` fits `cli.<verb>`), `contract/diag-subject/hyprctl-probe`
    (the seam's pair, unchanged).
- a11y-plan:
  - reached: `decision/time-bounds-never-on-human` and `decision/time-bounds-fall-on-the-driver` (a human verb now
    bears `HYPRCTL_DEADLINE`, up to three reads; the cells are full, so the case is NOT grown here and waits on F1 of
    run `2026-11-24T00-26-24Z-wrap`)
  - read, not reached: `contract/tui-passthrough/own-bytes` (focus writes no byte to the focused terminal),
    focus-report non-editing (a raised window's focus reports never move the wheel).
- test-plan:
  - reached:
    - `decision/window-placement-proven-live` (the gated clause)
    - `decision/live-rig-clean-start` (binds any R3 script)
    - `decision/fakes-for-external-services` (the fake change stays inside it)
  - read, not reached: `decision/standing-live-rig` (see F-D).

### His forks (each with the arm taken PROVISIONAL meanwhile)
**F-A — the widening: what focus may do.**
1. **(Recommended)** CLI-only human verb, raising only a LIVE wrapper's own terminal window, with no MCP tool and
   no window address or workspace printed. Cost: drivers cannot focus a window (no tool), and a driver can still
   type the CLI in its own shell, as with `release`, which no hint or doc offers it.
2. CLI plus an MCP `focus` tool. Cost: a driver session moves the operator's keyboard focus, a second widening of
   the MCP surface (`decision/human-controls-cli-only`).
3. Do not build focus. Cost: v2-11 stays unbuilt.
Arm taken: 1, PROVISIONAL.

**F-B — the help row.**
1. **(Recommended)** `board:    list, focus`. Cost: none to the five-row form; focus sits with the verb that shows
   the sessions it finds.
2. A sixth row `window:   focus`. Cost: the «five rows» cells grow.
3. `setup:    open, focus, …`. Cost: focus starts nothing, so it reads oddly there.
Arm taken: 1, PROVISIONAL.

**F-C — refusal words and wording.** New codes: `window-not-found`, and the seam's own words as refusal codes,
`unsupported` · `unreachable` · `oversize` · `malformed` · `refused`, each exit 1 with one hint. The plan's lines
are planning forms with his wording owed (the precedent of `2026-10-13-interrupted-verify-cleanup`). Arm taken: the
plan's words, PROVISIONAL.

**F-D — the live reading (R3), his to grant at the host.**
1. **(Recommended)** Two readings in one script with a `--rehearse` form that cannot reach a dispatch. Each closes
   what it opened on every exit path, strips every inherited `VIOLA_*`, reads the board with no `claude` on PATH,
   and uses a stand-in that takes the wrapper's leading arguments.
   - R3a: one hand-started `viola run` over the fake agent in a real foot window — no claude start — then
     `viola focus` once from another window. It raises that foot window once and switches to its workspace.
   - R3b: one `viola open` — ONE claude start under the cap — then `viola focus` once.
   Neither restores the earlier focus; the operator's screen is left on the focused window, then each opened window
   is closed. Cost: one claude start, two focus changes on his screen.
2. R3a only; the opened arm rests on the fakes and the E3 equality. Cost: the opened arm is never seen live, so
   v2-11's «for a session viola opened» clause stays unproven live.
3. No live reading. Cost: v2-11 cannot be claimed; it stays pooled.
Arm taken: claim v2-11 GATED on arm 1. The script is built and rehearsed now; nothing live fires without his word.

**F1 (standing, run `2026-11-24T00-26-24Z-wrap`):** `HYPRCTL_DEADLINE` now falls on a human verb. It is listed in
Expected amendments as waiting on F1; no cell grows.

### Technical forks (the overseer's relay answers; the lean the plan takes meanwhile)
- T1 strict-modes refusal: exit 21 with the instance pair (link's precedent for a verb on a named instance) —
  lean — or exit 1 (open's).
- T2 recorded address: trusted only when its client's pid is in the live wrapper's ancestry, else the ancestry arm
  (a gone or reused address falls back, never raises another window) — lean — or recorded first unconditionally
  (open's `target_of`).
- T3 (moved to F-E at the P5 review, inputs#I3) `unsupported`: only no `hyprctl` on PATH; `hyprctl` present with no compositor refuses `unreachable` (the
  seam's row stands) — lean — or a quick non-zero exit remapped to `unsupported` (amends
  `decision/hyprctl-read-deadline`).
- T4 seam failures: refuse with the seam's own word as the code, exit 1, a hint each — lean — or one folded word.
- T5 order: workspace dispatch then window dispatch; the workspace dispatch is skipped when the client's workspace
  is unread or negative — lean.
- T6 output: `<name>  focused` / `{"v":1,"ok":{"name":"<name>"}}`, no address or workspace — lean.
- T7 liveness: a not-live instance refuses exit 21 `instance-dead` / `unwrapped-name` (by `without_endpoint`)
  before any hyprctl read — lean.

### P5 review fold (inputs#I3, the overseer's review of card 1, 2026-11-24T02:22Z)
- Technical answers, a delegate's (the overseer): T1, T2, T4, T5, T6 and T7 each take their lean. T3 is moved to
  the operator as F-E, below; it is no longer a technical fork.
- **F-E — what «outside Hyprland → unsupported» means for v2-11** (moved from T3: it changes what v2-11's
  acceptance says, so what «verified» rests on is his).
  1. **(Recommended)** `unsupported` only with no `hyprctl` reachable. A host with `hyprctl` and no compositor
     answering refuses `unreachable`, as security-plan `decision/hyprctl-read-deadline` reads it today («a deadline
     kill or a non-zero exit reads as unreachable»). Cost: the acceptance's «outside Hyprland» narrows to «no hyprctl
     reachable»; the other case is still a named refusal, under another word. Changes no ratified row.
  2. A quick non-zero `hyprctl` exit remapped to `unsupported` at the verb. Cost: amends
     `decision/hyprctl-read-deadline` (a row of the seam) and splits one seam word into two by exit timing — a
     reading the seam does not make today.
  Arm taken: 1, PROVISIONAL. The claim is GATED, so his answer lands before any `verified`.
- The plan's four changes from the review are folded in plan rev 4: the control ids `focus-reuse` and
  `focus-no-client`; R3b fires only after a green R3a; R3a's stand-in is a script R3 writes for itself, as R2's
  does; R3b may be the first live `open`, and its verdict word says which.

### Disk, SIGKILL, cross-session (directive item 6)
- Disk: focus writes no snapshot, event line or record. It writes only lines into `<home>/diagnostics/
  cli-<name>.ndjson` (0600): its `process-start`/`process-exit` pair, the seam's `hyprctl-probe` pairs (up to three),
  and `state-recovered` on a replayed read — codes only, no address, pid or workspace.
- SIGKILL: the lines already written stay. A dispatch already made stays: focus has moved. Nothing else outlives
  it, because no file is opened for writing beyond the append log.
- Cross-session: a caller learns nothing new from focus's output (no address or workspace); the window record
  still leaves only through `list --json` and the MCP list item. A caller can change which window has the
  operator's keys: the widening above. It reads another session's snapshot under the strict-modes and liveness
  checks only.

### Can the entry be built whole on these arms?
Yes. Every fork of his has a conservative arm that builds; the live clause is GATED on F-D arm 1. Going on to P4
without waiting.
