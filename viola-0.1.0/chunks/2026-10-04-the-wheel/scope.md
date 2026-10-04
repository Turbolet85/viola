# Scope — The wheel

**Marker:** `2026-10-04-the-wheel` · **Version:** viola-0.1.0 · **Epoch:** Epoch 3 — Windows slice II: driving verbs and live proof
**Working entry:** `viola-0.1.0/working-route.md:80`: "The wheel — human editing key takes it, focus/mouse/resize never do, held bytes during paste, harness turns ignored, pause and release, human-typing/manual-pause refusals, release-from-driver", with five CARRY blocks. All five are folded below (§8–§12). `route.py pins` indexed 5 freight blocks on :80, of 292 · 357 · 191 · 324 · 272 chars.
**Host:** Linux (Omarchy, btrfs). Windows / ConPTY behaviour is witnessed only by the `windows-2025` CI runner.
**CLI:** installed `claude` 2.1.288 (mise); running sessions and the stamped fixtures are 2.1.287 (handoff Notes). Stamping 2.1.288 is the operator's, not this chunk's.
**CI read at take-up (Setup 5a):** the last wrap's flip `eb53a58` through HEAD is one sha. `eb53a58` → green · checks 15/15 · wall 265 s · ci#37220349220 completed/success. Nothing to fold.
**Operator directives at take-up (2026-10-04):**
- fold ONE `[inferred]` item, founder-ruled D-3 (a) via the setup registry's U40 (class `operator`, relayed by the overseer): the `andromeda:walks-tree` mark on the three tests that enumerate `fixtures/claude/*` (§13);
- Epoch 3 stays unsplit (founder ruling 2026-09-29); no boundary proposal is made.
**Held widening:** the PTY typed-input `viola verify` probe, the live recording, and the signature / quiet-period / max-wait ledger rows stay HELD for the founder's ruling (handoff Notes; owned by `:82`). Nothing in this chunk builds toward them.
**P3 closure:** every `[inferred]` bullet below is closed against `research.md`. A closed bullet is either verified (tag dropped, evidence named) or marked `[premise-corrected: …]`.

## What this chunk builds

The wheel: one holder at a time, `driver` or `human`, starting `driver` when `viola run` starts (`architecture.md:70`, [Human Takeover / Wheel]). Until now the wheel cannot move: `run` writes `wheel{holder:driver, cause:start}` once and the snapshot's `wheel` stays `Driver` (`src/cmd/run.rs:392`, `:403`). This chunk lets the human take it by typing, lets the CLI take it by `viola pause`, returns it only by `viola release`, and puts the wheel ahead of every automated input path (`send`, `answer`, `hook.dialog`). viola stays mechanism: the wheel is byte provenance plus an explicit CLI verb, never a policy decision (architecture's first principle).

### 1. A human editing key takes the wheel (working entry)
- `viola run` owns the real stdin, so it knows byte provenance exactly. Any human editing key since the last turn boundary moves the wheel to `human` (`architecture.md:70`).
- The move appends one `wheel` event `{holder:"human", cause:"human-input"}` (`source: wrapper`, log-only, never wakes `wait`) and rewrites the snapshot's `wheel` (`architecture.md:255`, `:299`, `:305`). Only a CHANGE appends — a second key while `human` adds no line.
- An automated `send` while the wheel is `human` is refused `human-typing` (exit 10, `src/cmd/send.rs:87`).
- A human interrupt is human input: it moves the wheel to the human (`architecture.md:70`).
- The human always wins: no human byte is blocked, refused or delayed past the current atomic paste (CLAUDE.md universal invariant 1).
- What an "editing key" is is a classifier over the stdin byte stream, agent-neutral (verified, research.md §Patterns: `pump_with_paste` takes any `Box<dyn Read + Send>` at `src/cmd/run.rs:470-472`, so a `Read` wrapper in the root bin's `src/run/` sees every human byte before the child, mirroring the output `Tee` at `src/run/gate.rs:130-140`; viola-pty is not changed for it).
- [premise-corrected: the stdin stream carries terminal replies as well as keys — on Windows x64 the outer terminal's DA1 answer to the sideloaded ConPTY's spawn-time `ESC[c` travels through `viola run`'s stdin (`tests/tui_passthrough.rs:101-104`; chunk 2026-09-29-sideloaded-conpty `evidence/da1-stall.md`, measured). The architecture names only focus, mouse and resize as non-editing, so whether DA/CPR/DECRPM/OSC/DCS replies count is a P4 ruling, not a code detail.]

### 2. Focus, mouse and resize never take it (working entry)
- Focus reports (`ESC[I` / `ESC[O`), mouse reports and resize events do not count as editing keys and never move the wheel (`architecture.md:70`; a11y-plan §3 tui clause, `a11y-plan.md:321`, and its three outer-PTY nextest cases, `a11y-plan.md:573`).
- These bytes still pass through to the child unchanged — "never move the wheel" is not "swallowed".

### 3. Held bytes during the paste (working entry)
- A bridge send is ONE `ESC[200~…ESC[201~` + CR write. Human bytes arriving during that write are held in the pump and passed on straight after it (`architecture.md:70`; `viola-pty`'s `PasteHandle` is the shared child input, CLAUDE.md Modules).
- The hold exists at HEAD and this chunk adds none (verified: `PasteHandle` is one `Mutex` around the child's writer, the paste one `write_all` under it and each human chunk written under the same lock, `crates/viola-pty/src/pump.rs:58-108`; witnessed by `paste_human_bytes_during_a_paste_land_wholly_after_it`, `pump.rs:814`). What this chunk owes is that those held bytes still move the wheel.

### 4. Harness turns never take it (working entry)
- An unsent `prompt-submitted` (one no `send` issued) confirms and logs a human take. A prompt whose raw start (no trim) is one of `viola-agent-claude`'s four `HARNESS_PREFIXES` is a harness turn, never a human take (`architecture.md:70`; `crates/viola-agent-claude/src/hook.rs:105`, `:177`).
- The founder-accepted side effect stands: a human who types the cross-session tag at a prompt's start is filed `harness` (2026-09-29 15:21:44).
- The fake agent's `--inject-harness-turn` submits one `<task-notification>` prompt filed `origin:"harness"` (verified, `src/bin/viola-fake-agent.rs:22`, `:466-485`). The wheel reads only the normalised `origin`; each of the four prefixes is pinned at the unit layer (`crates/viola-agent-claude/src/hook.rs:445-460`), so the existing mode witnesses "harness turn ignored" end-to-end without a fake-agent change.

### 5. `viola pause` and `viola release` (working entry)
- CLI `viola pause` moves the wheel to `human` without a keystroke, cause `manual-pause`; channel `pause` `{}` → `{wheel:"human"}` (`architecture.md:282`). After it, automated `send` is refused `human-typing` with detail `manual-pause`. It is the CLI twin of the reserved v1.x GUI brake route (no GUI route lands here).
- The wheel returns ONLY through CLI `viola release`: channel `release` `{budget?:bool}` → `{wheel:<holder>, budget_paused:bool}` (`architecture.md:283`); cause `release`.
- [premise-corrected: "`release` also clears the running-turn state" has nothing to clear at HEAD — `turn-running` is raised only by a second send while one is in flight (`src/run/send.rs:217-222`), and that slot belongs to a live `send`, so `release` must not empty it. No code tracks `prompt-submitted` → `turn-ended`, and no route entry owns that half of the architecture's `turn-running` (research.md §Mechanisms). Surfaced at P5, not folded.]
- `viola release` is a human verb: no driver-facing hint, MCP tool or doc suggests it to a driver (CLAUDE.md Session Learnings).
- At HEAD `pause` / `release` sit in the channel's `METHODS` (`crates/viola-channel/src/server.rs:360-361`) but `run` answers them `-32601` (verified: `src/cmd/run.rs:611-620`, and on a real wrapper `tests/channel_endpoint.rs:223`, `:249-250`). This chunk replaces both pins; the endpoint test retargets to the still-unbuilt `unlink`.
- `release`'s `budget?` half is wire shape only here (verified against architecture [Budget Governor]: "`release --budget` leaves the wheel where it is"). A `budget:true` must not move the wheel; lifting a pause is `:91`'s.

### 6. Refusal order: `human-typing` / `manual-pause` (working entry + CARRY §11)
- The wheel refusals come AHEAD of `turn-running` in `send`'s refusal order (`src/run/send.rs`); `send-refused` lines carry the `wheel` field obs-plan §4 Scenario CL-1 requires (CARRY §11).

### 7. `release-from-driver` (working entry)
- A `release` carrying `from` is refused `-32602` `release-from-driver`; `from` is self-reported, never identity (security.md §Local trust boundary, last bullet; `crates/viola-core/src/obs.rs:22` already names `ReleaseFromDriver`).

### 8. CARRY — `^Z` on Windows console stdin (chunk 2026-09-25-pty-wrapper-on-windows)
- HYPOTHESIS, unmeasured, verbatim as carried: "Rust std's Windows console stdin may treat a leading `^Z` as end of input, which would stop that key reaching the child through `viola run`'s stdin → PTY pump — measure it with the human-keystroke path this entry builds".
- [premise-corrected: source-read at the pinned 1.98.1 (`library/std/src/sys/stdio/windows.rs:353-392`, runtime still unmeasured), the effect is wider than carried — `read_u16s` drops a TRAILING `0x1A` from every `ReadConsoleW` result in every console mode, so a read of `^Z` alone returns `Ok(0)`, which the pump's `copy` (`crates/viola-pty/src/pump.rs:46`) takes as end of input: the human→child copy thread ends and no later key reaches the child. The witness is a `windows-2025` outer-PTY case (the outer PTY presents a console stdin to `viola run`) writing `\x1a` then `k`.]

### 9. CARRY — ConPTY and focus reports (chunk 2026-09-25-pty-wrapper-on-windows)
- [runner-only — not re-derivable on this Linux host; its witness is this chunk's `windows-2025` outer-PTY case, against the inbox ConPTY the outer PTY runs on] HYPOTHESIS, unmeasured — relayed without an artifact, verbatim: "ConPTY may swallow focus reports, so `\x1b[I` / `\x1b[O` injected through the Windows outer PTY might never reach the child — measure it before a11y-plan §3's \"focus/mouse/resize never move the wheel\" case relies on it on windows-2025".

### 10. CARRY — the human-wheel negative in `cli_controls_not_disableable` (chunk 2026-09-28-hook-perf-gate)
- `tests/cli_controls_not_disableable.rs` landed in its interim shape; this chunk adds its human-wheel negative (exit 10) to that table (test-plan §5 CLI).

### 11. CARRY — `send` without a wheel (chunk 2026-10-04-confirmed-send-with-cl-1-records)
- Verbatim: "landed the wrapper `send` without a wheel: its `send-refused` lines ship without the `wheel` field obs-plan §4 Scenario CL-1 requires, and its refusal order (`src/run/send.rs`) has no `human-typing` / `manual-pause` step ahead of `turn-running` — this entry adds both" (§6).

### 12. CARRY — `answer` and `hook.dialog` under a human wheel (chunk 2026-10-04-dialog-answers-by-dialog-id)
- `answer`'s `human-typing` refusal slot (at HEAD the wheel cannot move, so `answer` checks none) lands here.
- While the wheel is `human`, the wrapper answers `hook.dialog` with `null` at once, so the dialog renders for the human without waiting out the dialog deadline (`architecture.md:70`).
- If the wheel moves to the human while a `hook.dialog` is pending (a human editing key or `viola pause`), the wrapper answers that pending request `null` at once and clears its `dialog_id`; `pending_dialog` leaves the snapshot (`architecture.md:259`).
- `budget-paused` gates only `send`, never `hook.dialog` or `answer` (`architecture.md:70`) — unchanged here.

### 13. Operator fold — the `andromeda:walks-tree` mark (U40, founder-ruled D-3 (a))
- One comment line holding the token `andromeda:walks-tree` in each of `tests/contract_fixture_hygiene.rs`, `tests/contract_ledger_probes.rs` and `tests/contract_fake_agent_drift.rs` — the three tests that enumerate `fixtures/claude/*` (each opens a `read_dir` over it: `:28`/`:34`, `:25`, `:22`). Registry: `andromeda-setup-project/references/migrations.toml:474` `id = "U40"`, class `operator`, owner "implement P1/P2 (the chunk that writes such a test writes the mark)", act "the overseer's instance relay names each project's walk-class tests … and the project's next chunk writes one comment line each" — verified at take-up.
- Witness: the gate header's walk-class line. `[premise-corrected at take-up: the directive's "walk-class 3 marked" is a paraphrase — gate.py `walk_line` prints `walk-class {lang} {k} ({path},…) · uncommitted {u}` once any mark exists, and `walk-class 0 marked (…)` only at zero; the witness is `walk-class rust 3 (tests/contract_fake_agent_drift.rs, tests/contract_fixture_hygiene.rs, tests/contract_ledger_probes.rs) · uncommitted {u}` — the exact path order is gate.py's (`git grep` order), read off the run]`.
- Other tree walkers exist in `tests/` (verified: `contract_lints.rs:40`, `:240` and `contract_windows_mutation_scope.rs:22`, `:42` walk `crates/`). The relay named the measured list of three; marking any other is outside the ruling, so this chunk marks exactly the three.

## Boundaries (out of scope)
- The budget governor and `budget-paused` (`:91`); `release`'s `budget` flag beyond its wire shape (§5).
- The first live test, the founder taking the wheel live, and every held-widening item (`:82`).
- The GUI brake route (reserved v1.x) and any web rendering of the wheel (Epoch 8).
- MCP: no `release` or `pause` tool; the MCP server itself is `:102`.
- Links (`:93`) — a `pause` / `release` records no link.
- Server verification before the `pause` / `release` frames (`:109` / `:111`) — whether these two frames ride the existing liveness-only pre-check is a dated-gap question P4 must surface (verified: security.md lists six dated gaps and `pause` / `release` are in none; a widening needs the founder's live answer, playbook "Boundary widening — what ratifies it"), never a silent borrow.
- `--help` grouping (`wheel: pause, release`) and the a11y-plan §3 outer-PTY cases (1) zero wrapper bytes and (2) unblocked human keys during a send belong to `:100` "CLI output discipline" (its entry names "grouped --help … zero wrapper bytes, unblocked human keys during sends"); this chunk owns case (3).

## Expected touchpoints
As closed by research.md §New files / §Files to modify: `src/run/wheel.rs` and `src/run/snapshot.rs` (new; the latter serialises the wrapper's snapshot writes, which `DialogSlot::write_pending` and the wheel would otherwise race), `src/cmd/{pause,release}.rs` (new), `src/cmd/{mod,run,send,answer}.rs`, `src/run/{mod,send,dialog}.rs`, `src/human.rs`, `crates/viola-core/src/lib.rs`, `crates/viola-channel/src/{lib,server}.rs`, `crates/viola-pty/src/lib.rs` (the Windows console stdin reader, §8 — not `PasteHandle`, whose hold already exists), tests `tui_wheel.rs` / `cli_wheel.rs` (new), `channel_endpoint.rs`, `cli_controls_not_disableable.rs`, the three U40 files. [premise-corrected: `viola-state::snapshot` needs no change; `viola-pty`'s change is the `^Z` reader, not the paste hold]
