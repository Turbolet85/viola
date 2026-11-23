# Founder forks — «`viola open`» (2026-11-23-viola-open-2) · the P3 block

**This block holds THREE forks of the operator's (W1–W3, §0). The phase STOPS here for his word.** It first read
«no fork of his» at 22:00:59Z. P4's read of the playbook's escalate rows (`rul-r74hshif` and `rul-p2zl2kgl`, both
«Boundary widening», `verdict: escalate`) then found three ★ leans that ship a widening no answer of his covers. F3, F5
and F7 are a delegate's, and a delegate's direction never ratifies a widening. The plan was drafted on the ★ leans
before that read (`doc:2026-11-23-viola-open-2/plan` rev 1). It folds his answers before P5, and no P5 runs until
then. The six technical forks (T1–T6, §3) stay the overseer's; T2 and T3 meet W1 and W2.

## 0 · The operator's forks (each a Boundary widening; recommendation first)
**W1 · origin and target workspace on the started `run`'s argv.**
*The pattern:* a subprocess boundary gains a new crossing. Foot's argv, like `viola run`'s today, is readable by every
local user. F2 (his) put the name, the home and the cwd there. F7 (a delegate's) added the origin.
- **a ★** Both cross as hidden checked flags: `--origin cli|agent` and `--place <i32>`.
  - *Cost:* a closed word and an integer, no content, readable by every local user beside the name.
  - *Gain:* `--workspace N` (F1) and the parent's workspace reach the wrapper, which moves the window and records
    `placement` (F7).
- **b** Only the origin crosses; the open moves the window itself.
  - *Cost:* the wrapper's record keeps no `placement`. Placement lives only in the open's `--json` and its own `cli`
    log line.
- **c** Nothing crosses; the open moves the window and logs the origin in its own `cli` log.
  - *Cost:* the window record keeps neither origin nor placement. v2-07's «logs each open's origin» then stands on the
    open's own log line alone.

**W2 · the parent's window found through process ancestry (T3).**
*The pattern:* a new crossing. viola would read the parent pids of processes it did not start, up to 8 steps up from the
parent wrapper's pid, through sysinfo. The pids are held in memory and written nowhere.
- **a ★** First the parent's recorded window, else the ancestry walk.
  - *Gain:* the children of a hand-started root session (one not opened by viola) are placed beside it.
- **b** The recorded window only.
  - *Cost:* every child of a session not itself opened by `viola open` reads `unplaced`. Today's fleet roots are
    hand-started, so their children would always be `unplaced` until the roots are themselves opened.

**W3 · the window address and workspace on the board (`viola list --json` and MCP `list`, which share one document).**
*The pattern:* compositor data crosses onto the driver surface. Every driver session's MCP `list` returns it.
- **a ★** Items gain `window?` `{address, workspace}` (the address re-checked by `is_address`) and `origin?`.
  - *Cost:* a window address and a workspace integer reach every driver.
  - *Gain:* a driver sees where a session's window stands.
- **b** Neither on the board. The snapshot alone keeps them, for `viola focus` to read later.
  - *Cost:* F7's `list --json` members leave; a driver cannot see placement.

Each answer is folded into the plan at P4, and the wrap writes each into the owning sidecar as his.

Written 2026-11-23T22:00:59Z at P3's end, at HEAD `d7b65a6d`. The directive is `inputs#I1`. The answers F1 · F2 · F4 ·
F6 · F8 · F9 (the operator's) and F3 · F5 · F7 (the overseer's) are cited from `split-carry.md` §4 and re-asked of
nobody.

## 1 · The window
- **Run dir:** `.andromeda/runs/2026-11-23T21-24-59Z-phase/`.
- **Promotion:** tx 4332. The marker takes `-2` because the cancelled take-up's journal file holds the bare marker.
- **Row read:** 7 tasks, 159 picker draws and 7 history agents, all first try. Two pickers carried `[RETYPED]` notes
  (tests p9.d3 and p10.d2), each read as the pack's row.
- **Union:** picked 1279 of 4860 rows. No master reads `picked 0`. The rows files total 331,587 B, so P4 reads them by
  search.
- **Nothing live:** no live start, no window, no key, no dispatch, and nothing of the real `~/.claude*`. The host was
  read with `hyprctl -j version`, `command -v foot` and `foot --version` only.

## 2 · Measurements at HEAD (directive item 3; research M1–M11)
| # | what | at HEAD | split-carry §1 |
|---|---|---|---|
| M1 | requests · words · error words | 8 (5 queries, 3 dispatches) · 8 · 5 | 6 · 6 · 5 |
| M2 | `HYPRCTL_DEADLINE` | 5 s, settled; its security row carries no provisional tag | 5 s PROVISIONAL |
| M3 | 0.56.2 fixture files | 9 (+ `Monitors`, `MoveToWorkspace`) | 7 |
| M4 | the recorded move | argv `dispatch` + `hl.dsp.window.move({ workspace = 1, window = "address:0x555aa16bde50" })`; exit 0; stdout `ok\n`. The fake replays only that exact argv. | unmeasured |
| M5 | product caller of the seam | none: `Cargo.toml:174` under `[dev-dependencies]` | none |
| M6 | host (read-only) | Hyprland v0.56.2 `efb50993` · foot 1.28.0 at `/usr/bin/foot` | the same |
| M7 | the tap's `move-refused` · `move-unseen` · `window-left` | each reachable over a `stub_move` variant (`hypr_tap.rs:95-118`); `WINDOW_WAIT` 5 s at `WINDOW_POLL` 100 ms | — |
| M9 | the automatic verify | runs only with `config.json` `verify_new_cli` true (absent = false) | stated as «may» |

**Stays the fake's, unmeasured live until R2:**
- the window's map time after the spawn;
- whether foot maps its window before its child runs;
- a locked desktop's effect on a new window;
- a move of a window the product opened (R1 moved the tap's own).

`exec_cmd`'s shell question is moot, because no exec is issued.

## 3 · The technical forks (the overseer's; the plan stands on ★)
**T1 · Who finds and moves the window.**
- *The conflict:* F3 says open reads `clients` and moves the window. F7 says the wrapper records
  `window {address?, workspace?, placement, origin}`. Only the mover knows the placement, and only the wrapper writes
  the record (test-plan decision/single-writer-state-files). So HEAD cannot honour both as written.
- ★ The wrapper finds its own window and moves it. Its window is the one whose `pid` is its parent pid
  (`std::os::unix::process::parent_id`, the foot that F2 spawns it under), read through the seam. It records the
  window itself.
- Option b: open moves the window, and the wrapper records the address and origin with no placement. Cost: F7's
  `placement` field leaves the record.
- Option c: open moves the window and tells the wrapper. Cost: a new channel method, which needs a security entry.

**T2 · How the target reaches the wrapper.**
- ★ Two hidden checked `run` flags: `--origin cli|agent` and `--place <i32>`.
- Option: one hidden flag carrying both values.
- F7 named one flag.

**T3 · The parent's window.**
- ★ First the parent snapshot's recorded `window.address`, with its current workspace read from `clients`.
- With no recorded window, the parent wrapper's pid and up to 8 of its ancestors (sysinfo, already a `viola-state`
  dependency) are matched against `clients`' `pid`.
- With neither, there is no target, and the record reads `unplaced`.
- Option: the recorded window only. Cost: a hand-started root session's children always read `unplaced`.

**T4 · Where the window step stands.**
- ★ In the wrapper's start, under `--origin` only: after the automatic verify and the trust step, before the first
  snapshot.
- The first snapshot then holds `window`, and the `window` line follows `launch`. So «live» implies «window recorded».
- Cost: the claude spawn waits up to F8's 5 s window bound.

**T5 · The pinned viola on foot's argv.**
- ★ Open pins its own bytes into the home through the start's `pin_bytes` (placed new, re-hashed on reuse) and hands
  that absolute path.

**T6 · The replay's window rule.**
- ★ The last `window` line after the last `launch` line wins. A later start's `launch` line clears it, because a
  revive or a plain start records no window.

## 4 · New beyond `split-carry.md` §4 (directive item 6)
**Closed words:**
- placement: `placed` · `unplaced` · `window-unseen` · `unsupported` · `unreachable` · `oversize` · `malformed` ·
  `refused`. The last five are the seam's own words. F4 and F5 named `unplaced` and «the record says why».
- origin `cli` · `agent` (F7).

**Exits:** none beyond F6.

**Bounds:**
- the live poll at 100 ms inside F8's 30 s;
- the parent-ancestry walk of at most 8 steps (T3) — a count bound, no time.

**Record fields:**
- the `window` line `{address, workspace, placement, origin}`, with `address` and `workspace` null when no window was
  seen;
- the snapshot's `window?` with the same four;
- the hidden `run` flags `--origin` and `--place` (T2).

**`--json` members:**
- open's `ok.window` is `{address, workspace}` or null, and `ok.placement` is a placement word;
- `list --json` items gain `window?` (`{address, workspace}`) and `origin?`.

**Log:**
- the spawn subject `terminal` (one `process-start` / `process-exit` pair around foot's spawn);
- the exit-1 details `terminal-not-found` and `not-live`;
- the span `cli.open`;
- the open logs as role `cli` into `cli-<name>.ndjson` of the name it opens, as `link` does for the driven.

## 5 · Every reached row (directive item 4)
Searched again at P3 by handle over the seven rows files, and by `rec.py find` over all seven masters with
`replays? no|recover(s|ing) only|only links|launch line|time bounds? (fall|never)|lineage line` (the order: those
re-runs before the whole layers).

**architecture**
- decision/run-start-order (a window step, T4)
- decision/snapshot-replay-log-fields-only («recovers only links, agent_session_id, wheel, lineage and launch» + window)
- contract/snapshot-envelope/unreadable-snapshot (the same list)
- contract/instance-snapshot/data (+ `window?`)
- contract/normalised-event-kinds/* (a new `window` kind)
- contract/wrapper-event-appends (a `window` row after `launch`)
- decision/event-kinds-agent-neutral (a reading: `window` is agent-neutral)
- decision/hyprland-seam-crate (its first product caller)
- name/crate/viola-hyprland
- decision/window-features-target-hyprland
- decision/no-shell-out (a reading: foot spawned directly)
- decision/env-var-viola-prefix (a reading: no env read added)
- decision/no-environment-configuration
- contract/cli-exit-codes/1 and /2
- decision/retired-exit-codes-never-reused (no exit 11)
- contract/child-environment/* (a reading: foot inherits the open's env; the wrapper sets its child's)
- a new name/command/open
- decision/lineage-label-only (placement reads the parent's window, not its lineage: a reading)

**security-plan**
- the new v2-08 decision entry (the argument surface, the allowlist, the agent path)
- decision/run-lineage-flags-checked and decision/launch-flags-checked (open's own flags)
- decision/launch-pair-argv-crossing (the pair crosses onto foot's argv)
- decision/lineage-kept-to-snapshot-and-event-log and decision/launch-pair-kept-to-snapshot-and-event-log (the template
  for the window record)
- decision/lineage-parent-authenticates-nothing (`--origin` is self-reported too)
- decision/parent-identity-env-stripped (the parent's `CLAUDE*` ride foot's env into the wrapper and are stripped from
  its child: a reading)
- decision/hyprland-reply-text-dropped (the address and the workspace kept, nothing else)
- decision/new-env-read-needs-log-entry and decision/controls-not-switchable (readings: none added)
- decision/hyprland-recorder-moves-own-window (the product now moves a window it started: a sibling)

**design-system**
- contract/help-output/verb-groups and contract/viola-help/verb-groups (`setup:` gains `open`)
- contract/cli-exit-codes/1 and /2
- decision/list-six-fixed-columns (unchanged: a reading)
- decision/no-spinners-in-verb-output (the live wait prints no progress)
- contract/viola-run-start-refusals/* (open's `already-live` / `stale-heartbeat`)

**layout-templates**
- contract/cli-verbs/help
- contract/viola-help/verb-groups
- contract/viola-list-json/document (`window?` / `origin?`)
- decision/json-on-every-data-verb
- decision/hints-name-next-verb (two new pairs)

**test-plan**
- contract/viola-help/verb-table
- contract/viola-list-json/items
- decision/snapshot-replay-fields
- contract/boot-readiness/events and contract/run-start-events/order (an opened start has a `window` line after
  `launch`)
- contract/snapshot/run-fields
- decision/fakes-for-external-services (a stand-in foot)
- decision/fake-agent-argv-only and decision/fake-agent-on-test-path
- decision/fake-agent-not-proof-of-real-cli (R2)
- decision/no-untied-viola-spawn and decision/wrapper-spawned-with-guard (the test's foot child)
- decision/single-writer-state-files

**obs-plan**
- contract/run-start-events/order
- decision/launch-line-untraced and decision/lineage-line-untraced (the template for a window line with no trace of its
  own)
- contract/diag-subject/hyprctl-probe (a sibling subject `terminal`)
- contract/exit-1-detail/* and decision/distinct-detail-code-per-exit-cause (two codes)
- decision/span-naming (`cli.open`)
- contract/run-start-spans/span-chain (a window step span inside `run.start`)

**a11y-plan**
- decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human. Their list cells are full, so three
  rows of their own land: open's 30 s live wait, the 5 s window wait, and `HYPRCTL_DEADLINE` (carry `cry-2oejf2md`).
  The reading for the live wait: it falls on the open verb's own run, and no key is blocked.
- contract/cli-help/served-verbs-only
- contract/cli-exit-codes/2

**Read, not reached:**
- decision/revive-resumes-in-place («replays no launch spec but the recorded model and effort»): revive hands no origin
  and no place, and T6 clears a stale window at its start.
- decision/hyprctl-read-deadline and decision/hyprctl-direct-spawn: moved by the seam chunk's wrap (settled; the window
  move added), and not moved again here.
- decision/hyprland-recorder-string-allowlist (the tap only).
- the per-verb exit tables of `list`, `link`, `mcp`, `verify`, `plugin install` and `revive`.
- the exit-21 causes (open sends no frame).
- the channel methods (none added under ★T1).
- the board's six human columns and captions.
- the MCP tool set (`open` stays CLI-only).
- the paste, wheel, dialog and hook rows.

## 6 · Disk, SIGKILL, identity (directive item 7, re-read against §6)
**What reaches disk:**
- The open writes its pinned copy only when absent (T5), plus its own `cli` log lines.
- The wrapper writes one `window` line and the snapshot's `window` (both 0600).
- Moved from §6: «open itself writes no file» now reads «open places the pinned copy when absent».

**What outlives a SIGKILL:**
- of open after the spawn: the terminal and the session, re-parented;
- of the wrapper: its `window` line, which the replay recovers until the next start's `launch` (T6).

**Identity:**
- The parent's name, the home, the cwd, the origin word and the target workspace stand on foot's argv. Every local user
  can read them, as `viola run`'s argv is today. New since §6: the origin and the workspace integer.
- `hyprctl clients` shows the app-id `viola.<name>` to any same-user process.
- No new file holds a pid, session id or socket. The parent's pid stays in the open's memory (T3).

## 7 · Test children (directive item 8)
- The fake agent standing as `foot` by its own file name, in a test-scoped bin dir. It runs its trailing argv as its
  direct child and writes an argv receipt beside its exe. Case: `open_spawns_the_terminal_with_the_closed_argv`.
- A fake-hyprctl mode `terminal`: its `clients` reply names the stand-in foot's pid at the recorded address
  `0x555aa16bde50`, so the recorded move's exact argv replays. Case: `open_records_the_window_found_by_the_terminal_pid`.
- Three `hypr_tap.rs` stub variants for the carried codes.
- Every wait is bounded by `WITHIN` 12 s under the 20 s lint, with open's live bound injected below it. A test's
  `window-left` waits the tap's 5 s `WINDOW_WAIT` once.
- Every test start runs under a test-owned user home.

## 8 · Window
One implement window holds the entry, tightly: about 23 files, a new verb, a wrapper step, two fakes and the tests. The
plan orders its steps by layer (core and state → the wrapper's step → open → fakes and tests). A stop mid-way leaves
green layers, and implement STOPS before R2 with the pre-CI commit made.

## Answer — W1–W3 (the operator's own, `inputs#I4`; relayed verbatim by overseer1 from his dialog answered 2026-11-23T22:20:06Z, read by the overseer 22:20:20Z)
Each question was put to him whole, every option shown with its cost.
- **W1** «Что ещё попадает в командную строку …» → «Происхождение и воркспейс (Recommended)». Arm (a): the origin and
  the target workspace cross as two hidden checked flags.
- **W2** «Как viola находит окно родительской сессии …» → «Запись, иначе обход предков (Recommended)». Arm (a): the
  parent's recorded window first, else the ancestry walk of at most 8 steps, held in memory and written nowhere.
- **W3** «Показывать ли адрес окна и номер воркспейса …» → «Да, окно и происхождение (Recommended)». Arm (a):
  `list --json` and MCP `list` items gain `window?` and `origin?`; the six human columns stay unchanged.

So T2 and T3 stand on their ★ arms. R2 stands as `inputs#I3` says: ONE start, no spare, the probe first, workspace 1.

## Answer — the technical forks T1–T6 (the overseer's, a delegate's; `inputs#I2`, read by him 2026-11-23T22:07:45Z)
None of these answers W1, W2 or W3.
- **T1 ★.** The wrapper finds its own window (the one whose `pid` is its parent pid) through the seam, moves it and
  records it. This REPLACES the delegate's F3 of `split-carry.md` §4: F3 and F7 cannot both hold at HEAD, and the
  single-writer row decides. It stands only as far as W1 lets the target reach the wrapper. The plan writes W1's arms
  b and c as the fallback shape in one paragraph.
- **T2 ★, pending W1.** Two hidden checked flags if W1 reads (a).
- **T3, pending W2.** The recorded window comes first in every arm.
- **T4 ★.** The window step stands in the wrapper's start, under `--origin` only, before the first snapshot. When the
  5 s bound runs out, the start goes on and records `window-unseen`. A start with no `--origin` makes no seam read at
  all.
- **T5 ★.** The open pins its own bytes through the start's `pin_bytes`.
- **T6 ★.** The last `window` line after the last `launch` line wins.

## Answer — R2 (`inputs#I3`, read by the overseer 2026-11-23T21:47:15Z; it binds the plan beside `split-carry.md` §4)
**The operator's own words, relayed verbatim by overseer1** (his dialog, answered 2026-11-23T21:46:52Z):
- **On when:** «Давай так поступим пусть планирует делает, если получится проверить на заблокированном ок а если нет
  то пусть вче приготовит я утром включу на 10 мин пока буду собираться чтоб он проверил».
- **On a spare start:** «Один старт, без запасного (Recommended)». The cap for R2 is ONE live `claude` start, with no
  spare. A red R2 stops there: no second start, and v2-07 stays claimed GATED.
- **On where** (typed 2026-11-23T21:49:42Z): «Ок если что я освободил первый воркспейс может там проверять если что».
  Workspace 1 is free for the probe and for R2. It is an offer, not an order.

**The overseer's technical orders:**
1. Expect the desktop LOCKED at the R2 stop.
2. A no-start probe decides the branch. It is a plan step of its own, before R2, and spends nothing of the cap.
3. Both branches are written in the plan:
   - the probe reads green under the lock, and R2 needs no key and no eye → R2 runs once on the locked desktop, at the
     overseer's go;
   - otherwise → no start, and R2 waits for the operator's morning window.
4. R2 fits about ten minutes, the automatic verify included.
5. R2 is ONE command, written verbatim, with its preconditions read by a no-start entry first, its ledger line written
   before the start, and everything it opens closed on every exit path.
6. A key, an eye or a focus change moves R2 to the morning branch, and he hears it.
