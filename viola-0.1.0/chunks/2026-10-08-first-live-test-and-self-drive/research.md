# Codebase Research — 2026-10-08-first-live-test-and-self-drive

## Scope
- **Depth:** moderate · **Reads:** 14 direct, plus two read-only explorers (77 tool uses, every answer with its
  `file:line`) · **Globs/Greps:** 12
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, its thirteen Session Additions
  included; four apply here (never pipe `boot`; the tmpfs backing behind `target/e2e-home`; a stalled-start red is a
  finding about the backing; the fake agent receipts the submitting Enter as its own `key` line).
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:**
  - `inputs#I1` — the take-up directive: 2.1.287 by path, live cap 8, the chunk stamps its own home, the hint
    wording to P4 as a founder question, the two audit CARRYs out of scope.
  - `inputs#I2` — founder rulings R-L1 and R-L3 (the Linux dev host; `v1-33` re-worded to it; cap 8).
  - `inputs#I3` — the founder's ruling of 2026-10-08T05:02Z: he is not at the keyboard; the takeover is a
    compositor-typed key; `v1-33` is re-worded on the typing.
  - `inputs#I4` — the operator's guard for the key probe: type only into the probe window, never focus or type into
    the `viola.viola-builder` or `overseer.viola-overseer` windows.
  - `inputs#I5` — the overseer's driving guide for the prototype (M9's gap list is read against it).
  - `inputs#I6` — the operator's answer on the blocked probe: both monitors read `dpmsStatus` false; test with DPMS
    on; if focus still does not move, the probe becomes implement's step 0 with the DPMS-on precondition in the
    plan.
  - `inputs#I7`, `inputs#I8`, `inputs#I9` — the P4 answers, the operator's P5 review and the approval of the first
    plan.
  - `inputs#I10` (the revision) — the overseer's focus measurements: the dispatcher moves the workspace and the
    cursor, never the focused window.
  - `inputs#I11` (the revision) — the operator's answer on step 0: steps 1 to 3, stop before step 4, no lever past
    the lock; the relay's "no locker" line cannot see this lock.
  - `inputs#I12` (the revision) — the founder's ruling of 2026-10-08T06:43Z: the key is typed in a compositor of
    the chunk's own; the desktop lock stays untouched.
  - `inputs#I13` (the revision) — the founder's answer: the nested form, its price accepted, one compositor start
    for all live work; the desktop lock read after the start and at the end, an unlocked reading stops the chunk.
  - `inputs#I15` (the second revision) — the operator's answer on the stop of the live work: close the session,
    measure which terminal reply is read as typing, keep the own compositor up with no window, a 3 hour bound.
  - `inputs#I16` (the second revision) — the operator's word: the seven measured reply shapes join the closed
    list, each by its exact grammar, with a red-green case and a negative control; research reads the CLI's
    start-up queries from the fixtures if they hold them; the live run is retried on the standing compositor.
  - Not snapshotted: the installed `claude` 2.1.287 binary, read at M14. `inputs.py snap` refuses a binary source
    over 1 MiB; its size, sha256 and what was read are in
    `.andromeda/runs/2026-10-08T07-43-15-phase/cli-reply-parser.md`.

## Files inspected
- `src/run/wheel.rs` (284-380, and the explorer's read of 195-233) — the stdin observer and the classifier; the
  `release` handler.
- `src/cmd/mod.rs` (18-148) — the verbs, `resolve_home`, where `VIOLA_NAME` is read.
- `src/cmd/client.rs` (96-108) — `own_name()`: the caller's `from`.
- `src/human.rs` (196-232, and the pins at 362-428, 548) — `send_hint` and its tests.
- `src/run/wait.rs` (128-172) — where a cursor-less `wait` starts.
- `src/run/send.rs`, `src/run/gate.rs`, `crates/viola-agent-claude/src/screen.rs` (the explorer's read) — every
  cause of `input-not-ready`.
- `crates/viola-e2e/src/harness/run.rs` (112-160, 326-412) and `src/bin/viola-harness.rs` (the explorer's read) —
  `Selection`, `local_live`, `LEDGER_ROWS`.
- `crates/viola-e2e/src/harness/supervise.rs` (40-79) — the fake agent's command line under the harness.
- `src/cmd/verify.rs`, `src/cmd/verify/typed.rs` (outline, and the explorer's read) — verify's six children and its
  probe dirs.
- `src/cmd/run.rs`, `src/run/mod.rs`, `src/run/version_gate.rs` (the explorer's read) — `run`'s arguments, what it
  writes at start, the `cli_verified` verdict.
- `src/cmd/answer.rs`, `crates/viola-agent-claude/src/dialog.rs` (the explorer's read) — the response shapes.
- `scripts/release-check.sh`, `.claude/docs/commands.md:79` (the explorer's read) — the product build.
- `refs/viola-brief.md` (109-110, 288-295) and `viola-0.1.0/intent.md` (340-346) — what the first live test was.
- `.andromeda/runs/2026-10-07T05-47-07-wrap/adaptation-record.md` (53-56) — the retired "Linux live confirmation"
  entry, verbatim.
- chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host: `scope.md`, `evidence/hint-window.md` (12, 80-99,
  120-123), `evidence/hint-card-answer.md`.

## Measurements and readings (M1 to M12)

### M1 — the compositor key: NOT measured; what was tried (inputs#I3, inputs#I4, inputs#I6)
The question the founder's ruling left to this research, "whether such a key moves the wheel", is still open. Four
guarded runs of one script against the harness `viola` over the fake agent (no live start); a key was typed in none.

| run (UTC, 2026-10-08) | what happened |
|---|---|
| 05:15:06 | `hyprctl dispatch exec "[workspace 9 silent] foot …"` opened nothing: this Hyprland (0.56.2, Lua config) reads `dispatch` as Lua and refused the classic form with a syntax error. No window, no home. |
| 05:18:21 | the script's own emptiness check misread the state (it asked `activewindow`, which kept naming the overseer's window on an empty workspace) and stopped before launching. |
| 05:18:41 | workspace 9 on the second monitor, `hl.dsp.exec_cmd("foot …")`: the wrapper started (three start records, one `wheel` record), the window was listed on workspace 9. It did not take focus. The guard stopped the script: active window `overseer.viola-overseer`. |
| 05:23:16 | the operator's test (inputs#I6): DPMS turned on first (`hl.dsp.dpms("on")`, both monitors `True`), the window read `mapped: True` before any focus. It did not take focus by itself, and `hl.dsp.focus({ window = "address:…" })` answered `ok` with the active window unchanged. The guard stopped the script. DPMS off and workspace 7 restored at the end. |

- Read from the code, not run: on Linux the pump's stdin is `std::io::stdin()`; `Observed::read` hands every read to
  the classifier and calls `human_input` when it held an editing byte (`src/run/wheel.rs:306-313`); every byte is
  editing except the closed list of focus reports, mouse reports and terminal replies (`:337-339`, `:360-369`).
  Graph: `human_input` is called from `read @ src/run/wheel.rs:310` and `append_hook_event @ src/run/send.rs:242`.
  So a key that reaches the terminal's PTY moves the wheel. What is unmeasured is that a compositor key reaches the
  window.
- The DPMS hypothesis of inputs#I6 is falsified as the cause: with the outputs on and the window mapped, focus
  still did not move.
- Hypothesis, not measured: something holds keyboard focus on the overseer's window (a rule set at runtime, or a
  grab). `~/.config/hypr` names no such rule (`grep -i` for `stay_focused`, `no_focus`, `overseer`, `viola.`: 0
  hits). The installed Lua API stub file was not read: the permission layer refused that read.
- Dispatcher forms this compositor accepts (each returned `ok`): `hl.dsp.focus({ workspace = <n> })`,
  `hl.dsp.exec_cmd("<command>")`, `hl.dsp.dpms("on")` / `("off")`, `hl.dsp.focus({ window = "address:<addr>" })`.
  `hl.dsp.focus` names its own keys in its error text: `direction, monitor, window, urgent_or_last, last`.
- Host facts read on the way: two monitors, every session window on `DP-2`, the second monitor (`HDMI-A-1`) on an
  empty workspace 7; `input:follow_mouse` is 1; no locker process; DPMS went off again by itself within about a
  minute of being turned on.
- Left behind: four scratch homes `target/e2e-home/viola-keyprobe-20261008T05*Z/` on the tmpfs (three empty or
  near-empty, one with a `home-diag` from a no-window start check). No process and no window is left (read back).
- On the operator's word (inputs#I6) the probe is implement's step 0, before any live start, with DPMS on as a
  precondition of the probe and of the live run.

### M1r — the revision (2026-10-08): why focus never moved, and the compositor of the chunk's own
The stopped implement run and this revision closed M1. Records: `evidence/key-probe.md`, `evidence/comp-probe.md`;
the script, the two configs and the three logs in `.andromeda/runs/2026-10-08T06-46-31-phase/`.

- The cause M1 did not name: the desktop session is locked. `hyprctl locked` reads `true`; the lock is held by the
  desktop shell (`quickshell`, its `lock` plugin), taken 2026-10-07T19:39:15Z by the shell's own journal, before
  every run of M1. A search for a locker process and logind's `LockedHint` both read "not locked" (re-derived:
  `hyprctl locked`, `omarchy-shell lock isLocked`, `journalctl --user` lines `qml: omarchy lock`).
- M1's hypothesis "something holds keyboard focus on the overseer's window" is corrected to that. Its DPMS finding
  stands (DPMS was not the cause).
- The founder ruled that nobody unlocks the session and the key is typed in a compositor this chunk starts
  (inputs#I12). Measured, one script, three runs over the harness `viola` and the fake agent, no live start:

| run (UTC) | form | reading |
|---|---|---|
| 06:50:57 | headless (no parent compositor, the seat backend forced to an absent `seatd`) | aborts 0.2 s after start: `CBackend::create() failed!` |
| 06:51:50 | nested (one client window of the desktop compositor) | the key probe passes |
| 06:53:02 | headless, the own log on | the same abort, the log empty |

- The nested pass, as equalities the plan leans on (the frame is `Observed::read` → `human_input`,
  `src/run/wheel.rs:306-313`; the control is this run): a window of class `viola.keyprobe` on the own instance
  takes focus by itself (active address equal to the probe's, 1.5 s after it mapped); the focus writes 0 `wheel`
  records; `wtype x` on the own socket writes one `wheel` record `{"cause":"human-input","holder":"human"}` within
  4 ms; the fake agent's receipt holds `key` `78`; a driver `send` exits 10 with `human-typing` and the hint `the
  human has the wheel; send again after the human hands it back`; 0 `send-issued` records.
- The form that passed: a private runtime dir `$XDG_RUNTIME_DIR/vcomp` (0700) holding the own instance's control
  and Wayland sockets; the compositor started under `env -i` with no session bus address,
  `LIBSEAT_BACKEND=seatd` (no `seatd` socket exists on this host, so no seat is opened and logind is never asked),
  `HYPRLAND_NO_SD_VARS=1`, `HYPRLAND_NO_SD_NOTIFY=1`, `HYPRLAND_NO_RT=1`, `HYPRLAND_NO_CRASHREPORTER=1`, and
  `WAYLAND_DISPLAY` naming the desktop socket by absolute path; `Hyprland --config <a Lua file>`, which
  `Hyprland --verify-config` reads `config ok`. foot started by the script under `env -i` with the desktop's
  runtime dir and `WAYLAND_DISPLAY` naming the own socket by absolute path.
- Why foot keeps the desktop's runtime dir: viola resolves its endpoint dir from `XDG_RUNTIME_DIR`
  (`crates/viola-channel/src/endpoint.rs:78`), so the wrapper in the window and the driver verbs of the
  implementing session must share it.
- Every `hyprctl` of the script goes through one function that sets the private runtime dir and
  `--instance <own signature>`; the desktop instance was never asked. `hyprctl keyword` is refused on this build
  under the Lua parser; options go through `hyprctl eval 'hl.config({...})'`.
- What the nested start did to the desktop (the shell's journal): `idle-monitor: active` 0.2 s after the start,
  the wake script, and 4 s later the shell lost its Wayland connection, exited 255, was relaunched, logged
  `lock-stranded: recovering` and `secure=true` 2.2 s after the exit. The same exit and relaunch stand at 05:22:54Z
  and 06:48:11Z, not attributed. The founder accepted the price for one start (inputs#I13).
- Unchanged across the runs: the user manager's `WAYLAND_DISPLAY`, `HYPRLAND_INSTANCE_SIGNATURE` and
  `XDG_CURRENT_DESKTOP` (one hash before and after), one desktop instance dir, one desktop Wayland socket.
- The terminal the nested window's child saw: 26 columns by 14 rows (the receipt's `size` line), on an output of
  621x688 at the compositor's automatic scale, foot at its configured font (size 11, pad 14x14;
  `~/.config/foot/foot.ini:4-5`). Not measured: the size with the output at scale 1, no gaps, no border and a
  smaller font (`comp-live.lua` reads `config ok`; the plan reads the size under the fake agent first).
- Not measured: a live `claude` under the declared environment (`HOME`, `USER`, `LOGNAME`, `SHELL`, `PATH`,
  `LANG`, the two display variables); how long the own compositor stays up; whether the desktop shell exits again
  at the next start.
- Left by the revision: `$XDG_RUNTIME_DIR/vcomp/` (three instance dirs, a `dconf/` dir), three scratch homes
  `target/e2e-home/viola-keyprobe-20261008T0650*Z` to `T0653*Z`, two `Hyprland` crash records in the coredump
  store. Left by the stopped implement run: `target/e2e-home.disk/viola-none-3881459`.

### M2 — who the driving session is to the product
- This session's environment carries `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (names read with `env | cut -d= -f1`),
  set by the prototype's wrapper. `CI` is not set.
- A product verb run from here reports `from: viola-builder`: `own_name()` reads `VIOLA_NAME`
  (`src/cmd/client.rs:105-108`).
- The product's CLI verbs take the home from `--home`, else `~/.viola` (`resolve_home`, `src/cmd/mod.rs:140-148`).
  They do not read `VIOLA_DIR`; only `hook` does (`:71`). The architecture extract states a `VIOLA_DIR` step for
  every verb. Every verb of the run names `--home`, so the run does not turn on it.

### M3 — the wheel's return
- A `release` carrying a string `from` is refused `-32602` `release-from-driver`, exit 20, the wheel unmoved, one
  INFO line (`src/run/wheel.rs:197-200`, `:223-233`). From this session every `viola release` carries `from` (M2).
- A `release` without `from` moves the wheel back and records `{"holder":"driver","cause":"release"}`
  (`src/run/wheel.rs:195-210`). Run by an agent with `VIOLA_NAME` dropped, that is the step-around the security
  extract names: a founder ruling, never a default.
- `v1-33`'s acceptance asks for the takeover, not for the return. The prototype's pass record did include the
  return (`refs/viola-brief.md:290-295`: "`release` returned it").

### M4 — "answers a review"
- The first live test, as the brief records it (`refs/viola-brief.md:288-295`): the overseer sends
  `/andromeda-new-session`, waits for `Stop`, reads the dashboard; "the answer to *Ready to continue?* arrived; the
  founder typing into the window took the wheel, `send` was refused, `release` returned it."
- R7 (`refs/viola-brief.md:109-110`): "Keystrokes carry user turns alone — skill starts, relays, review answers. The
  dialogs are answered through the hook contract".
- So the review answer is a typed reply, a `send`. The dialog answered by `dialog_id` is W3's reading, not this step.

### M5 — the driving verbs at HEAD
- `send <name>` takes the text from stdin or `--file`, never an argument (`src/cmd/send.rs:24-48`). Its own result
  line is the delivery proof (`[RB] read back … cursor <n>`).
- `wait <name> --after <cursor> --timeout-ms <n>` wakes on `turn-ended`, `question`, `permission`, `plan`,
  `session-end` (`src/cmd/wait.rs:19-35`). With no `--after` it starts at the log's end at the call
  (`src/run/wait.rs:139-147`).
- `last <name>` prints the newest turn's message.
- `answer <name> <dialog_id>` takes one JSON object on stdin or `--file`: a question is
  `{"answers": {"<question>": "<answer>"}}`, a permission `{"behavior": "allow"|"deny"}`, a plan
  `{"behavior": "approve"|"revise"}` (`src/cmd/answer.rs:24-54`; `crates/viola-agent-claude/src/dialog.rs:199-230`).
- `/clear` is sent as a listed local command and, on a verified CLI, is confirmed by a `session-start` with cause
  `clear` and a new session id.
- No MCP server ships: `plugin/.mcp.json` has an empty `mcpServers` and the `Command` enum has no `mcp`
  (`src/cmd/mod.rs:36-61`).
- `run <name> -- <program> [args]` has no other flag (`src/cmd/run.rs:35-42`). It does not require a terminal on
  stdin (`crates/viola-pty/src/lib.rs:319-331`), so only the window decides whether a typed key can arrive.

### M6 — `run --local-live` at HEAD
- It names no program: `viola --home <home> verify`, cwd the workspace root
  (`crates/viola-e2e/src/harness/run.rs:379-385`); the harness `run` has no pass-through field
  (`crates/viola-e2e/src/bin/viola-harness.rs:77-106`). `verify` resolves `claude` on its own `PATH`
  (`src/cmd/verify.rs:103-115`; `src/run/mod.rs:134-143`), which is 2.1.289 on this host.
- Passed alone it runs no other suite: `Selection::from_flags` turns `unit` and `integration` on only when no
  selector is named (`run.rs:58-65`). This corrects the tests extract's "after the selected suites (the default
  selection is `--all`)".
- It first builds `cargo build --workspace --features viola/fake-agent` into `target/harness` (`run.rs:359-364`), so
  the `viola` that stamps is the harness build.
- Its home is `target/e2e-home/viola-live-<pid>/home` (`run.rs:375-378`). Nothing removes it (`grep viola-live` over
  the harness: the creation site and one test of its absence after the CI refusal).
- Graph: `local_live` has one caller, `run_with @ crates/viola-e2e/src/harness/run.rs:151`.

### M7 — the price of one verify, and the cap
- One `viola verify` starts six children: the `--version` read (`src/cmd/verify.rs:149`), the print probe (`:171`)
  and four PTY runs (`src/cmd/verify/typed.rs:69`, `:98`, `:135`, `:178`). Five are `claude` sessions.
- A failing row still writes the stamp and exits 1 (`verify.rs:214-232`); a start that shows a modal is killed with
  no key and its rows fail.
- Against the cap of 8: a by-path verify for the chunk's home (5) plus the builder (1) is 6. A separate
  `--local-live` firing is 5 more, 11. One red verify round is 5 more on any path.

### M8 — the product build and the verified home
- Product build: `bash scripts/release-check.sh` runs `cargo build --release --locked --bin viola` and judges the
  build's own artifact records; its last line is `release-check: viola only`; locally with
  `CARGO_TARGET_DIR=target/release-check` (`.claude/docs/commands.md:79`; `scripts/release-check.sh:19-37`, `:89`).
- `cli_verified` is decided once, at `run`'s start: the child's `--version`, then `ledger/stamps.json` through the
  strict read; false on a missing, unreadable, malformed or loose-moded stamps file, or when not all seventeen rows
  read `pass` for that version (`src/run/version_gate.rs:115-172`). A strict-modes failure logs one
  `parse-rejected{parser:"ledger-stamps", detail:"strict-modes-failed"}` and prints nothing.
- A no-window start of the harness `viola` in a home it created itself came up with `bin`, `diagnostics`,
  `instances` and `plugin` and logged its `version-probe` pair and the `claude-child` start (the probe's start
  check, 05:16Z).

### M9 — the prototype against the product (inputs#I5)
Measured at this take-up (`ps -eo pid,comm,args`): every builder on this host, this session's own included, runs
`../additional/viola-lab/prototype/target/debug/viola run <name> --dialog-timeout 3500 -- claude --plugin-dir
../additional/viola-lab/prototype/plugin …`. The explorer read the guide and both trees: 25 items, of which 5 stop a
switch as the guide stands.

| the guide relies on | the product at HEAD |
|---|---|
| `run --dialog-timeout 3500`, and an unanswered dialog HELD until the overseer answers it | no flag; `DIALOG_DEADLINE` is a compiled 60 s (`crates/viola-core/src/lib.rs:16-19`), after which the dialog is the human's |
| `list` before every send (state `idle`, the wheel) | no `list` verb; it is route entry `working-route.md:115` |
| `out.raw` and `scr.sh` to read the screen (CLI modals, `ctx NN%`, the screen before a dialog) | no counterpart; `run` never reads or keeps screen content |
| `allow <n>` to pick a permission suggestion | not carried, a deliberate v1 limit (`crates/viola-agent-claude/src/dialog.rs:172-173`) |
| the start line and three scripts outside this repository (`vstart.sh`, `waitm.sh`, `clear-worker.sh`) | they name the prototype's path, flags, event names and `sessions/` folder; each needs a rewrite |

Workable differently: text on stdin, `--timeout-ms`, `wait --after`, `last` for the text, `answer` by id with JSON,
`/clear` confirmed by the send itself, `release` only from a shell without `VIOLA_NAME`.

### M10 — the `input-not-ready` refusal at HEAD
- A `send` ends `not-delivered` / `input-not-ready` when: no child is attached yet (`src/run/send.rs:343-345`); the
  screen model is poisoned (`crates/viola-agent-claude/src/screen.rs:131-133`); the screen was never quiet for
  300 ms within 8.5 s (`:134-137`); on a verified CLI a modal stands (`:145-147`); on a verified CLI the screen is
  quiet with no input box at 8.5 s (`:150-156`).
- In none of them is a turn running (that is the `turn-running` refusal), so a cursor-less `viola wait` has nothing
  to wake on. Measured on live 2.1.287 before the gate waited (`evidence/hint-window.md:83`, `:98`): such a wait
  returned `timed_out` at its own 20 s deadline.
- The line and its pins: `src/human.rs:216` (producer), `:371` and `:406` (unit pins), `.andromeda/design-system.md:764`
  and `.andromeda/layout-templates.md:415` (spec text). No integration test, snapshot or trycmd file pins it
  (`grep -rn "was not ready for input"` over the tree outside `target/`, run dirs and chunk folders: 5 hits, those
  five).

### M11 — "never renders"
- `run` never reads screen content, and `Screen::rows()` is `verify`'s alone (the arch extract, [Screen Model]); the
  tests extract bars a verdict from parsed screen content.
- On the records, a decided dialog reads: `dialog-raised`, `dialog-answered`, then `hook-decision` with
  `decision_emitted:true` and `deadline_hit:false`, one `dialog_id` in `corr` (the obs extract).
- With nobody at the desk, a dialog that did render would hold the turn until a key. So a `turn-ended` after the
  decided dialog, with no `wheel` record and no key typed, shows the turn went on without a human.
- That is a reading of the effect, not of the pixels. A late answer (past the 60 s deadline) does render, by
  contract, and is a timing result.

### M12 — the Linux live confirmation
- The retired entry is not in `route-archive.md` (127 lines, 0 hits for the title). It is kept verbatim in
  `.andromeda/runs/2026-10-07T05-47-07-wrap/adaptation-record.md:53-56`: "Linux live confirmation — founder-attended
  on the Linux laptop, one live viola run checks the fake agent's Unix fidelity; reorderable, never blocks CI-run
  Unix chunks". It names no comparison.
- The fake agent's Unix-only code is two sites: the `fds` receipt (`src/bin/viola-fake-agent.rs:554-564`) and the
  held grandchild's own process group (`:648-649`).
- Comparisons the live run can take from its own records, at no further start: the first three event lines
  (`wheel{start}`, `budget-gate`, `session-start{source:hook}`); the `claude-child` start line (`pty_backend`
  `openpty`, the stripped and kept `CLAUDE*` names against the recorded 12-name reading); the wrapper's
  `endpoint_kind`; the live child's open fd numbers against the fake agent's measured `[0,1,2,3,4]`; whether
  `session-end` lands before the child is gone at the session's close.

### M13 — the second revision (2026-10-08): the live session lost the wheel at its start
Records: `evidence/terminal-replies.md`, `evidence/live-run-start6.ndjson`, `evidence/reply-probe.ndjson`.

- Start 6 (the product build over `claude` 2.1.287, `cli_verified` true, in a foot window on the own compositor):
  `wheel {driver, start}` and `budget-gate` at 07:28:40.939Z, `wheel {human, human-input}` at 07:28:41.176Z,
  `session-start` at 07:28:41.536Z. No `key` call was made while the window was open. The first `send` exited 10,
  `human-typing`, with one `send-refused` and no `send-issued`.
- The frame: `Observed::read` hands every stdin read to `Classifier::feed` and calls `human_input` when it returns
  true (`src/run/wheel.rs:306-313`). `feed` is true for every byte except a completed sequence `is_reply` accepts
  (`:429-433`, `:475-504`) and an OSC or DCS string of at most 64 payload bytes (`:444-459`). `is_reply`'s match
  (`:496-503`) accepts, after `CSI`: `I` / `O` with no field; `< n;n;n M|m` and `n;n;n M`; `? … c` and `> … c`;
  `n;n R`; `? n;n $ y`; `? n u`. Every other final byte is typing.
- The control that decides which replies are typing on this host, measured 07:33Z to 07:34Z on foot 1.28.0, the
  product build over a script child, one query per window, no key (23 windows): seven replies wrote a `wheel`
  record of cause `human-input`:

| the reply as foot wrote it | the query | on the list at HEAD |
|---|---|---|
| `CSI 0 n` | DSR `CSI 5 n` | no: no arm takes final `n` |
| `CSI ? 997;1 n` | colour scheme `CSI ? 996 n` | no |
| `CSI 4;675;1260 t` | window size `CSI 14 t` | no: no arm takes final `t` |
| `CSI 6;15;6 t` | cell size `CSI 16 t` | no |
| `CSI 8;45;210 t` | text area `CSI 18 t` | no |
| `CSI 48;45;210;675;1260 t` | in-band resize, mode 2048 set | no |
| `CSI > 4;1 m` | modifyOtherKeys query `CSI ? 4 m` | no: the `m` arm takes `<` only |

- Both controls of that probe held: no query, no `wheel` record; DA1, on the list, no `wheel` record.
- Not typing on foot, measured in the same probe: DA1, DA2, XTVERSION (a DCS string), the kitty flags reply,
  DECRPM, CPR, OSC 10 / 11 / 4 (25 to 26 bytes), XTGETTCAP and DECRQSS (DCS strings), the focus report. No answer
  at all: mode 2031 set, the kitty graphics query.

### M14 — which queries the CLI sends at its start (inputs#I16's hypothesis)
Record: `.andromeda/runs/2026-10-08T07-43-15-phase/cli-reply-parser.md`.

- The hypothesis is falsified: the fixtures hold no query bytes. `grep -r -l -a` for an ESC byte or an escaped
  ESC over `fixtures/`: 0 files; the 2.1.287 set's three `Screen.<phase>.json` files hold `cols`, `rows` and
  `screen_phase` only.
- Read instead, with no live start: the CLI's own reply parser in its binary (sha256 `3920489a…18f0`). It returns
  ten response types: `decrpm`, `da1`, `da2`, `kittyKeyboard`, `cursorPosition` (`CSI ? r;c R`), `themeNotify`
  (`CSI ? 997;1|2 n`), `cellSize` (`CSI 6;h;w t`), `osc`, `xtversion`, `kittyGraphics`.
- Against the list at HEAD and foot: `themeNotify` and `cellSize` are read as typing today and are among the
  seven. `cursorPosition` is not on the list either, but foot gives no answer to `CSI ? 6 n` (measured in this
  revision, 07:47Z, `replies-r.ndjson`: 0 reply bytes, 0 `wheel` records). `kittyGraphics` draws no answer from
  foot. The other six types are on the list or inside the 64-byte string bound, and read as not typing.
- So on this terminal, every reply this CLI has a parser for is either not typing today or among the seven.
- Not measured: the queries the CLI sends in its first 237 ms, as bytes (a live start); a reply no parser names;
  any other terminal. A terminal that answers `CSI ? 6 n` or the kitty graphics query would write a reply that is
  typing at HEAD and is not among the seven.

### M15 — the list's tests, and the grammar each shape takes
- The list is pinned in `src/run/wheel.rs` alone (`grep -rn -E 'is_reply|REPLIES|Classifier' --include=*.rs` over
  `src crates tests fuzz`, outside that file: 0 hits). Its tests: `REPLIES`, an array of 15 byte strings (`:518`),
  each asserted not typing whole and split at every byte from the third on, then all in one read
  (`classifier_every_listed_reply_is_not_editing_whole_and_split`, `:564`); and 22 rstest cases of
  `classifier_typing_is_editing` (`:537-558`), among them the near-misses of the listed shapes
  (`cpr_shape_with_one_field`, `sgr_mouse_with_two_fields`, `decrpm_without_question`, `kitty_key_event`).
- `is_reply` already splits a prefix (`?`, `>`, `<`), the numeric fields and the intermediates, and counts the
  fields (`:476-495`). The seven shapes need the field VALUES too (`0`; `997` then `1` or `2`; a first field of
  `4`, `6`, `8` or `48`; `4` after `>`), which no arm reads at HEAD.
- No human key encoding ends in `n`, `t` or `m`: the keys this classifier's own cases name end in a letter of
  `A` to `Z`, `~` or `u` (`:541-549`), and `m` is the SGR mouse release, on the list with `<`. The nearest human
  keys to the new shapes are therefore the plain letters `n`, `t`, `m`, their Alt forms (`ESC n`, `ESC t`,
  `ESC m`), and their kitty key events (`CSI 110 u`, `CSI 116 u`, `CSI 109 u`, with or without a modifier
  field). Each is typing at HEAD (ground bytes; `ESC` then a letter, `:415-418`; `u` without `?`, `:501`).
- A limit the list already has and this change does not move: the classifier reads bytes, not time. A human who
  types the bytes of a reply one key at a time (Alt+`[`, then `0`, then `n`) is read as that reply, as Alt+`[`
  then `I` is read as a focus report today.

## Graph impact (from the code-graph query; rust plane, regenerated: 4346 nodes / 22497 edges)
- **is_reply** (the second revision's query; trace
  `.andromeda/runs/2026-10-08T07-43-15-phase/tree-query-2026-10-08-first-live-test-and-self-drive.json`) — one
  caller, `step @ src/run/wheel.rs:430`. **feed** — `read @ src/run/wheel.rs:308` and the two classifier tests
  (`:559`, `:565-574`). No signature moves: the change is inside `is_reply`'s match and the test tables.
- **send_hint** — 3 product callers: `unable_unreachable @ src/cmd/client.rs:154`, `unable @ src/cmd/send.rs:149`,
  `unreachable @ src/cmd/send.rs:157`; 6 call sites in its own unit tests (`src/human.rs:362`, `:422`, `:427`,
  `:428`, `:448`, `:548`). A re-worded `input-not-ready` line changes one match arm and two pins; no caller's
  signature moves.
- **local_live** — 1 caller, `run_with @ crates/viola-e2e/src/harness/run.rs:151`. It changes only if the founder
  picks a by-path form for the harness.
- **human_input** — `read @ src/run/wheel.rs:310` (the stdin observer) and `append_hook_event @ src/run/send.rs:242`
  (a human-filed prompt), plus tests. Not changed by this chunk; it is the mechanism the takeover reads.

## Patterns detected
- **Evidence on the product's own records** (chunk 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host,
  `evidence/hint-window.md`): each live step is a row of verb, exit, the record it produced and the time; no
  session text is quoted.
- **A live reading enters as a literal oracle** (the tests history, 2026-10-07): what was measured is pinned by a
  unit case, and a failing reading is kept in evidence.
- **One fixed hint per cause, pinned by literal** (`src/human.rs:209-225`, `:406-428`): a wording change moves the
  producer and its pins in one commit.

## Conventions to follow
- **Prompt text from stdin or `--file`**: the skill and `/clear` both start with a slash (CLAUDE.md, Session
  Learnings).
- **Never pipe a harness step that leaves a process**: write each step's output to a file
  (`.claude/rules/verification-harness.md`, 2026-09-25).
- **A count or `find` over a home behind the link takes the trailing slash** (`target/e2e-home/`), or it reads an
  empty scope (the obs history, 2026-10-07).
- **No absolute path, token or `CLAUDE*` value in an evidence file**; names and codes only.

## New files to create
- `viola-0.1.0/chunks/2026-10-08-first-live-test-and-self-drive/evidence/` — the live run's records, the key probe's records, the own compositor's config and its scripts

## Files to modify
- `src/human.rs` — the `input-not-ready` hint line and its two pins; done by the stopped implement run, in the tree uncommitted
- `src/run/wheel.rs` — the seven reply shapes in `is_reply`, its doc comment, and the classifier's two test tables

## Open questions
- none — the three questions of the take-up were answered at its P4 (inputs#I7); the revision's one fork, the
  compositor form, was answered by the founder (inputs#I13); the second revision's direction is the operator's
  word (inputs#I16), and M14 found no reply shape on this terminal outside its seven.
