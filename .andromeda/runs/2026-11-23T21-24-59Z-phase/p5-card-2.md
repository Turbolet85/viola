# P5 card 2 — «`viola open`» (2026-11-23-viola-open-2)

**The plan to open in an editor:** `.andromeda/cache/docs/2026-11-23-viola-open-2/plan.md` (the render of
`doc:2026-11-23-viola-open-2/plan` rev 7; card 1 `.andromeda/runs/2026-11-23T21-24-59Z-phase/p5-card-1.md` answered «review»).
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-23-viola-open-2` is a diff of
  the chunk's records.

## Goal
`viola open <name> … -- claude` starts a `foot` window running the wrapped session, spawned directly with no shell,
and returns once the instance is live (at most 30 s).

The new wrapper finds its own window by its parent pid through the Hyprland seam (T1). It moves the window to the
target the open handed it on two hidden checked flags (W1 a): the parent's workspace (its recorded window, else an
ancestry walk of at most 8, W2 a) or `--workspace N`. It records `window {address, workspace, placement, origin}` in
its first snapshot and in one log-only line after `launch`. `list --json` and MCP `list` gain `window?` / `origin?`
(W3 a).

The allowlist is closed: the words after `--` are exactly `claude`.

## Top acceptance criteria
- **matrix#v2-08**, claimed. Verified on the fake.
- **matrix#v2-07**, claimed GATED on R2. The fakes prove every clause short of the real window.
- The window record replays (the last line after the last `launch`); help, exit codes, documents and diag lines are
  pinned.
- Carry `cry-pu3g6nb7`: the tap's three codes are tested (step 21).
- Carry `cry-2oejf2md`: `HYPRCTL_DEADLINE` becomes an a11y time-bound row of its own (Expected amendments, wrap).

## Touchpoints
- **New:**
  - `src/cmd/open.rs`
  - `src/run/window.rs`
  - `src/bin/viola-fake-agent/foot.rs`
  - `tests/cli_open.rs`
  - `scripts/r2-open.sh`
- **Modified (20):**
  - `Cargo.toml` and `crates/viola-state/Cargo.toml`
  - `src/cmd/{mod,run}.rs`, `src/run/mod.rs`
  - `crates/viola-core/src/{lib,obs}.rs`
  - `crates/viola-state/src/{snapshot,replay,board,liveness}.rs`
  - `src/human/hints.rs`
  - the fake agent's root file and its `hyprctl.rs`
  - `crates/viola-e2e/tests/hypr_tap.rs`
  - `schemas/diag-line.v1.json`
  - `tests/{cli_output_plain,cross_exit_causes,contract_error_bodies,contract_diag_schema}.rs`

## Test commands (28 entries)
- **Lint:** fmt and clippy (1–2), and `bash -n` on the R2 script (10).
- **Units:** viola-core (3), viola-state (4), and the root bins (5); the new-case filters 6–7 (vacuous red at
  baseline).
- **Integration:** the new cases (8, vacuous red) and the readers of the moved literals (9, green, 188 tests).
- **Probes:** `cargo deny check bans licenses sources` (11) and the host read (12).
- **Harness layers:** 13–15.
- **Gated:** the reader of `evidence/r2.md` (16).
- **Operator pass:** 17–23 (smoke, pre-push, hygiene, push, CI), then 24–28 (build, lock read, probe, preconditions,
  R2), in plan step 28's order.
- **Remove-the-guard one-shots:** `open-allowlist`, `window-launch` and `place-only-seen`.

**Baselines** (the host read CONTENDED: escher was building, 430 s stalled on IO):

| entry | reading |
|---|---|
| 3 | green, 40/40 |
| 4 | green, 298/298 |
| 5 | green, 1260/1260 |
| 6 | red, 0 of 298 selected |
| 7 | red, 0 of 1260 selected |
| 8 | red, 0 of 4161 selected |
| 9 | green, 188/188 |
| 10 | red, exit 127: the script is absent until step 27 |
| 11 | green: bans, licences and sources ok |
| 12 | recorded |
| 16 | red, exit 1: no record; control typed and recorded in `control-entry-16.md` |

## Concretization preview (the never-weaken checkpoint)
**v2-08 · The launch authority bounded** (method `unit`).

*Now:* «An open carrying a flag outside the closed allowlist is refused before anything starts; no shell line is ever
built from the given arguments, whatever characters they hold; a security decision entry names the argument surface,
the allowlist and the agent path.»

*Proposed:*
- An `open` carrying a flag outside the allowlist, or any word after `--` other than `claude` alone, exits 2 before
  any home read, log, pin or spawn (`open_args_refuse_a_word_after_dashes_other_than_claude`,
  `open_refuses_a_word_after_dashes_and_reads_nothing`).
- The one spawn hands foot the closed argv, one element per value whatever characters a name, cwd or role holds, with
  no shell (`open_argv_is_the_closed_argv_one_element_each`, `open_spawns_the_terminal_with_the_closed_argv`).
- `evidence/v2-08-by-construction.md` enumerates the surface, the allowlist, the one spawn site and the agent path.
  The wrap writes the security entry from it; the claim stands on implement's enumeration, never on the wrap's
  amendment.

**v2-07 · Launch a session into the fleet** (method `e2e`), claimed **GATED**.

*The gate:* «R2, ONE viola open end to end beside a live parent under a scratch home, at the operator host, unlocked,
its moment his to name (carry cry-fhabylf6, F9)». The operator has since answered (`inputs#I3`): under the lock if the
probe reads green, else his morning window.

*Now:* «From the CLI and from an agent's own shell, an open brings up a new terminal window running the named wrapped
session, placed beside its parent's window on Hyprland and left to the window manager elsewhere; it returns only once
the session is live, records the window's identity and the open's origin, and names the calling agent as the parent
when none is given.»

*Proposed:* the same outcome, proven in two halves.
- **Over the fakes, this chunk:** the closed argv; live within the bound; the `window` record and line; `placed` by
  the one move; `unplaced` · `window-unseen` · `unsupported` where it cannot place; the caller named as the parent;
  origin `agent`.
- **By R2 alone** («a new terminal window … placed beside its parent's window on Hyprland»), its record
  `r2: green placement=placed …`:
  - the recorded address stands in `clients` with the foot's pid;
  - its workspace equals the parent window's.
- A stand-in never verifies the gated cap.

## The review folded (`inputs#I5`, the overseer's, read 2026-11-23T23:01Z) — plan rev 6 → 7, steps 28–30 and their entries only
1. **The probe is a READING.**
   - Its entry carries `expect = []`. The tool reads it `recorded · exit {rc} · …` whatever the exit
     (gate-contract.md §Operator entry, *The line*: «with `expect = []` it asserts nothing and reads
     `recorded · exit {rc} · …` whatever the exit. The exit is on the line either way»).
   - So the tool records a non-zero exit as a reading with no wrapper; the command stands unwrapped.
   - The tap's verdict word stands in the entry's log. Both readings go into `evidence/probe-under-lock.md`, and the
     branch line reads them.
2. **The morning order:** the lock read (either reading) → the probe AGAIN → the preconditions → R2 only when THAT probe
   read exit 0.
   - A probe that is not exit 0 on the UNLOCKED desktop is a finding: no start; the pass stops and reports.
   - The night branch is unchanged.
3. **Who fires what, in one order (plan step 28):**
   - Implement greens every non-leg entry and reports; it fires no operator entry.
   - The operator pass fires through the CI read, then the build entry. It makes the scratch home with its
     `config.json` and the private claude-config dir, then STOPS and reports. The R2 ledger line is not yet written.
   - **First go:** the lock read and the probe, reported in one line, `lock <true|false> · probe exit <rc> <word>`.
   - **Second go**, only on the branch line: the R2 ledger line from `date -u`, the preconditions, R2, its verdict
     line copied into `evidence/r2.md`, and a report.
4. **The preconditions entry** now runs `test -f …/config.json` before its `! grep -q verify_new_cli`.

Unchanged, as the review confirmed: the three controls, entry 16 and its typed control, the amendments, the cap of ONE
start with no spare, and workspace 1.

**When a window ends with R2 unfired:**
- The record, under `.andromeda/cache/chunks/2026-11-23-viola-open-2/evidence/`, holds each fired operator entry's
  line, `operator-pass.md`, `probe-under-lock.md` (once the probe has fired), and no `r2.md`.
- Entry 16 stays deferred, and v2-07 stays claimed gated.

**The morning re-entry:**
- It is the same entries fired by number, `gate.py run --chunk 2026-11-23-viola-open-2 --operator {n}`, from whatever
  window the operator opens.
- **The letters name no door for an operator pass split across two windows.** They name only the gated door, once the
  chunk is wrapped `gated`: `/andromeda-implement` in its gated mode, its `--void 16:block cleared — {evidence}`, then
  `/andromeda-wrap-session`.

## Beside the directive's item 12 — the probe and R2 (the W relay's step 3), as rev 7 holds them
The entries, verbatim, each keyed `leg = 'operator'`, fired in the operator pass in the order of plan step 28:
```toml
[[gate]]
run = 'cargo build -p viola && cargo build -p viola-e2e --bin viola-hypr-tap'
role = 'build'
leg = 'operator'

[[gate]]
run = 'hyprctl locked'
role = 'probe'
leg = 'operator'
expect = []

[[gate]]
run = '''mkdir -p "$XDG_RUNTIME_DIR/viola-r2/probe" && target/debug/viola-hypr-tap record-move --program /usr/bin/hyprctl --terminal /usr/bin/foot --workspace 1 --out "$XDG_RUNTIME_DIR/viola-r2/probe"'''
role = 'probe'
leg = 'operator'
env = ['XDG_RUNTIME_DIR']
expect = []

[[gate]]
run = '''test -d "$XDG_RUNTIME_DIR/viola-r2/home" && test -d "$XDG_RUNTIME_DIR/viola-r2/claude-config" && test -f "$XDG_RUNTIME_DIR/viola-r2/home/config.json" && ! grep -q verify_new_cli "$XDG_RUNTIME_DIR/viola-r2/home/config.json" && test ! -e "$XDG_RUNTIME_DIR/viola-r2/home/instances/parent" && test ! -e "$XDG_RUNTIME_DIR/viola-r2/home/instances/child" && command -v foot hyprctl && test -x target/debug/viola && df -m /tmp'''
role = 'probe'
leg = 'operator'
env = ['XDG_RUNTIME_DIR']

[[gate]]
run = '''bash scripts/r2-open.sh "$XDG_RUNTIME_DIR/viola-r2/home" target/debug/viola'''
role = 'e2e'
leg = 'operator'
env = ['XDG_RUNTIME_DIR']
expect = ['exit 0', 'contains r2: green placement=placed ']
```
(Each entry's `timeout` and `note` are in the plan.)

**The probe** spends nothing of the cap.
- Absent: no claude.
- Present: `/usr/bin/foot`, `/usr/bin/hyprctl`, the built tap (the build entry).
- What it reads: `recorded · exit 0` and the one fixture file, or `recorded · exit 1` with `no-terminal-window` ·
  `move-refused` · `move-unseen` · `window-left` in its log.
- What it leaves: that file under `$XDG_RUNTIME_DIR/viola-r2/probe`. Its window is killed and waited gone by the tap
  itself.
- It sends no key and dispatches no focus.

**R2:**
- Absent: `verify_new_cli` in the scratch `config.json`, and instances `parent` and `child`.
- Present: `$XDG_RUNTIME_DIR/viola-r2/home` and its `config.json` `{"v":1,"claude_env_keep":["CLAUDE_CONFIG_DIR"]}`,
  `$XDG_RUNTIME_DIR/viola-r2/claude-config` (0700, empty), `foot`, `hyprctl`, `target/debug/viola`.
- The no-start entry reading them is the preconditions entry above. It also prints `df -m /tmp`, and it fires at the
  second go after the ledger line.
- The cap: ONE live `claude` start, no spare. A red R2 is final, and v2-07 stays claimed GATED.
- The parent is a `viola run` over `/usr/bin/sleep 900` (no claude), started by the script with `--origin cli --place 1`.
- The script closes both foot windows on every exit path. It leaves the scratch dirs and two sockets, each named for
  removal by name at the operator's desk.

**The branch line (plan step 30):**
- **Night:** R2 fires only when the probe read `recorded · exit 0` AND the lock read just before it read `true`, at the
  overseer's second go, once.
- **Morning (anything else):** the lock read (either reading) → the probe AGAIN → the preconditions → R2 only when THAT
  probe read exit 0.
- A morning probe that is not exit 0 on the unlocked desktop is a finding: no start; stop and report.

## The directive's item 12 — in one block, verbatim where a tool printed it
**The union's per-master lines and the `rows-union` line** (the row read re-run whole with `--repin` on scope rev 4):
```
architecture: rows 1008 · packs 11 · picked 266 (153 by every draw) · standing 24 · 73209 B
security-plan: rows 645 · packs 7 · picked 158 (100 by every draw) · standing 16 · 45304 B
design-system: rows 326 · packs 4 · picked 113 (80 by every draw) · standing 11 · 26773 B
layout-templates: rows 228 · packs 3 · picked 59 (38 by every draw) · standing 14 · 12494 B
test-plan: rows 1374 · packs 14 · picked 341 (217 by every draw) · standing 68 · 84559 B
obs-plan: rows 1041 · packs 11 · picked 301 (191 by every draw) · standing 88 · 73506 B
a11y-plan: rows 238 · packs 3 · picked 90 (58 by every draw) · standing 12 · 23288 B
rows-union: 7 master(s) · picked 1328 of 4860 · rule any
```
**Every cited row in the union or a standing list?**
- No: two are not, and every other row the plan cites is (a search of each cited handle over the 7 rows and 7
  standing files).
- `decision/parent-identity-env-stripped` was found by `rec.py find --doc security-plan` (`dec-okhdupwc`) and is
  cited by its handle.
- `name/command/open` is a new row the wrap mints.
- The rows files sum to 339,133 B, past 250,000 B, so P4 read them by search, never whole.

**The round pre-flight:** no `round` or `live` entry, so there is no pre-flight.

**Every `[RETYPED]` line:**
- First read:
  - `tests-pick-p9.d3 · [RETYPED] line 12: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 12`
  - `tests-pick-p10.d2 · [RETYPED] line 76: decision/external-processes-real-stub-executables is no row of the pack — read as decision/external-processes-real-stubs, the pack's row 76`
- Re-run:
  - `arch-pick-p1.d2 · [RETYPED] line 13: decision/flat-cargo-workspace-with-bin-at-root is no row of the pack — read as decision/flat-cargo-workspace, the pack's row 13`
  - `security-pick-p3.d3 · [RETYPED] line 53: decision/unwrapped-agent-name-never-target is no row of the pack — read as decision/unwrapped-name-never-target, the pack's row 53`
  - `security-pick-p4 · [RETYPED] line 84: decision/plugin-files-rewritten-every-start is no row of the pack — read as decision/plugin-files-rewritten-each-start, the pack's row 84`
  - `security-pick-p7 · [RETYPED] line 77: decision/server-verified-before-any-channel-frame is no row of the pack — read as decision/server-verified-before-any-frame, the pack's row 77`
  - `tests-pick-p6 · [RETYPED] line 90: decision/unwrapped-name-never-target is no row of the pack — read as decision/unwrapped-name-never-a-target, the pack's row 90`
  - `tests-pick-p6.d3 · [RETYPED] line 91: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 91`
  - `tests-pick-p9.d2 · [RETYPED] line 67: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 67`
  - `obs-pick-p8.d3 · [RETYPED] line 54: decision/schema-top-level-unevaluatedProperties is no row of the pack — read as decision/schema-top-level-unevaluated-properties, the pack's row 54`

**Re-spawns:** none. Every agent of both row reads passed first try, with no dead draw.

The re-run's history check printed `not a marker` rows. They are information, not failures:
- `layouts-history · not a marker: **2026-10-20T06-58-31-wrap** — history:28`
- tests-history cited amendment ids (`amd-…`) in place of markers: 14 printed, `… +67 rows` in
  `.andromeda/runs/2026-11-23T21-24-59Z-phase/fanout-2026-11-23-viola-open-2.json`.

**Every non-zero exit, refusal, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED` line** (card 2's checks re-run whole after the fold: 28 entries parse, planlint `hits: 0`, `scope lists: parsed 25 items`, `inputs: 5 entries — unchanged 5`, no new non-zero exit):
- **Two Bash-guard refusals at P3:** a `cd` that moved the session's cwd, and a pipe of a tool's output into a filter.
  Each was re-issued once in the passing form.
- **The baseline reds:** entries 6, 7 and 8 (exit 4, no tests to run), entry 10 (exit 127) and entry 16 (exit 1),
  each read above.
- **The typed control of entry 16:** exit 1 on the tree and on the red input, exit 0 on the green one.
- **One read of mine exited 1:** a `grep` over a background output, whose last `tail` named a log entry 16 never
  wrote.
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed.

**Every place a letter was wrong or silent:**
- **The P3 block came before the escalate read.** The directive orders the operator-fork block at P3's end, while the
  skill reads the playbook's escalate rows only at P4. The block first read «no fork of his» and was corrected at P4
  (W1–W3). Telemetry `2026-11-23T22:07:22Z-a` records it.
- **No per-master row-read re-run.** The W relay asked for a re-run «on the masters `scripts/r2-open.sh` reaches»;
  `fanout.py render --repin` re-runs all seven, which it did.
- **No form for a gated entry's baseline.** The letter gives none, and the tool fires no deferred entry, so its
  control was typed beside it in the tool's shell form (validation check 4 (9): the clone pair cannot hold).
- **The scope correction came at P4.** It named `scripts/r2-open.sh` there (P4 folded `inputs#I3`), not at P3; the row
  read was re-run whole for it, as the letter orders.
- **Silent, at the review:** the letters name no door for an operator pass split across two windows (above).
- **Size:** the plan's prose is about 445 lines (the fence and the R2 block excluded), over the ~400 guide. A warning,
  not a fail.

## Beyond the plan
none

Apply? (yes / review / cancel)
