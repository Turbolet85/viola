# Codebase Research — 2026-10-04-the-wheel

## Scope
- **Depth:** deep · **Reads:** 27 · **Globs/Greps:** 24 · **Graph queries:** 1 (rust plane, `db_state: fresh`, 136 rows — trace `.andromeda/runs/2026-10-04T17-30-10-phase/tree-query-2026-10-04-the-wheel.json`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 8 Session Additions applied (the `test(=tests::name)` / `binary(<stem>)` filter forms; never pipe `boot`; a selector not yet built is a plan defect). No live leg against a real CLI in this chunk: every leg is nextest through `scripts/agent-run.sh` over the fake agent.
- **Platform issues consulted:** none — no runner-only CI-verdict bullet (Setup 5a read `eb53a58` green 15/15). The CARRY §8 mechanism was read from the pinned toolchain's own source instead: `~/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/stdio/windows.rs:353-392`, whose comment cites rust-lang/rust#38274.

## Files inspected
- `src/cmd/run.rs` (full) — `Methods` dispatch (`:106-129`): `pause` / `release` fall to `dispatch` → `-32601`, pinned by `methods_answer_method_not_found_for_every_other_method` (`:611-620`). `start_state` writes `wheel: Wheel::Driver` (`:392`) and the one start `wheel{holder:driver,cause:start}` event (`:401-410`). `pump_child` (`:447-491`) hands `Box::new(io::stdin())` to `viola_pty::pump_with_paste` (`:470-477`) and attaches the paste to `SendSlot` (`:459-461`).
- `src/run/mod.rs` (full) — module list (`dialog`, `env`, `gate`, `send`, `version_gate`, `wait`): no wheel module yet.
- `crates/viola-pty/src/pump.rs` (`:1-200`, test names `:805-861`) — the human copy is `copy(input, HumanInput)` on its own thread (`:164-165`); `copy` ends on `Ok(0) | Err(_)` (`:46`). `PasteHandle` (`:58-89`) is one `Mutex` around the child's writer: a paste is one `write_all` + flush under the lock, and a human chunk waits for that lock only. Resize never travels as stdin bytes: the pump polls `host_size()` every 250 ms and calls `pty.resize` (`:186-193`).
- `crates/viola-pty/src/lib.rs` (`:254-340`) — `HostTerminal::enter` on Windows clears `ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT` and sets `ENABLE_VIRTUAL_TERMINAL_INPUT` (`:254-264`, `:295-316`).
- `src/run/send.rs` (`:1-360`) — the refusal order as landed: params → `validate_paste_text` (`:214`) → in-flight `turn-running` (`:217-227`) → `input-not-ready` (no io `:229`, gate `:232-235`, paste `:237`) → `no-prompt-submitted` (`:240-243`). `Ctx::refuse` (`:319-344`) takes only a `NotDelivered` detail; its `send-refused` obs line carries `side:"wrapper"` and no `wheel`.
- `src/run/dialog.rs` (`:1-380`) — `hook_dialog` holds a dialog only when `cli_verified && pending.is_none() && !unanswerable` (`:178`); `await_answer` (`:254-279`) loops on an answer or the deadline; `write_pending` (`:283-288`) is a read-modify-write of `snapshot.json`; `answer` (`:292-338`) orders params → free text → `unverified-cli` → `unknown-dialog`; `refusal()` (`:128-130`) takes `Option<NotDelivered>`.
- `crates/viola-core/src/lib.rs` (`:40-170`) — `EventKind::Wheel` exists; `RefusalReason::HumanTyping` exists; `NotDelivered` is the only closed detail set — no `human-typing` detail type (`manual-pause`) yet.
- `crates/viola-state/src/snapshot.rs` (`:1-80`) — `Wheel { Driver, Human }` (kebab serde) and `InstanceSnapshot.wheel`, `pending_dialog`.
- `crates/viola-channel/src/lib.rs` (`:54-92`) — `ProtocolError` has no variant whose `body()` carries a `data` other than `UnsupportedVersion`'s.
- `crates/viola-channel/src/server.rs` (`:355-366`, `:509`, `:535`, `:622`) — `METHODS` already lists `pause` and `release`; the `-32602` classification at `:509` matches on variants.
- `schemas/diag-line.v1.json` (`:59-68`, `:105-114`, `:125`) — `wheel` is already an admitted field on `send-issued` / `send-confirmed` / `send-refused`; `release-from-driver` is in the event enum with `corr`, `conn`, `from`, `from_trust:"self-reported"`, `corr` required.
- `crates/viola-core/src/obs.rs` (`:22`, `:45`, `:68`, `:185`) — `ObsEvent::ReleaseFromDriver` exists.
- `src/cmd/mod.rs` (full) — clap `Command` enum: no `Pause` / `Release`; `cli_sink` gives every cli verb its detail sink.
- `src/main.rs` (`:90-104`) — `role_of` files every first word but `run`, `hook` and a `-` flag under `cli`: `pause` / `release` need no edit.
- `src/cmd/send.rs` (full) — `exit_of` (`:85-93`) already maps `human-typing` → 10; `Out::unable` (`:127-141`) asks `human::send_hint` only for a `not-delivered` detail.
- `src/cmd/answer.rs` (full) — the same shape; `human::answer_hint` gives `human-typing` no hint.
- `src/cmd/client.rs` (full) — `live_endpoint` (`:95-102`) is the liveness-only pre-check of the dated gaps; `own_name` (`:105-108`) is the `from` every verb adds from `VIOLA_NAME`; `fault` (`:121-130`) is exit 20.
- `src/human.rs` (`:165-202`, `:505-522`) — `answer_hint` / `send_hint` keyed tables; the test `answer_hint_is_the_fixed_table` pins `human-typing` → no hint (`:515`).
- `src/run/gate.rs` (`:119-180`) — `Tee` writes the human's output first, then offers a copy to the feed thread: the pattern an input-side observer follows.
- `src/bin/viola-fake-agent.rs` (`:22`, `:466-485`, `:527-547`) — `--inject-harness-turn` submits one `<task-notification>` prompt, filed `origin:"harness"`; stdin is read byte by byte and every byte outside a submit is receipted `key {hex}`.
- `crates/viola-agent-claude/src/hook.rs` (`:105-109`, `:445-460`) — the four `HARNESS_PREFIXES`, each pinned by a unit case.
- `tests/cli_controls_not_disableable.rs` (full) — interim shape; its header names "the human wheel" as a row still to join.
- `tests/chaos_feed_panic.rs` (full) — the refused `send` (`:71-102`) runs before the typed `k` (`:104`): the wheel cannot change its verdict on Linux/macOS (see the DA1 pattern below for Windows).
- `tests/channel_endpoint.rs` (`:205-260`) — `channel_wrapper_logs_each_call_with_corr_and_conn` sends `pause` to a real wrapper and asserts `error_code: -32601` (`:223`, `:249-250`).
- `tests/tui_passthrough.rs` (outline, `:98-110`) — the outer-PTY rig; its comment records that the sideloaded ConPTY holds the child's start for a DA1 answer "that travels through the pump".
- `tests/support/outer_pty.rs` (outline) — `OuterPty::{spawn, write, resize, wait_exit, finish}`.
- `tests/contract_fixture_hygiene.rs`, `tests/contract_ledger_probes.rs`, `tests/contract_fake_agent_drift.rs` (heads `:1-12`, walk sites) — each enumerates `fixtures/claude/*` with a run-time `read_dir` (`:28`/`:34`, `:25`, `:22`).
- `tests/contract_windows_mutation_scope.rs` (`:1-60`) and `.github/workflows/windows-mutants.yml` (`:25-35`) — the workflow's file lists must equal the Windows-gated product sources; `crates/viola-pty/src/lib.rs` is already listed.
- `~/.claude/skills/andromeda-tools/scripts/gate.py` (`:347-402`) — `WALK_MARK = "andromeda:walks-tree"`; `walk_line` prints `walk-class {lang} {k} ({path}, …) · uncommitted {u}`.
- `~/.claude/skills/andromeda-setup-project/references/migrations.toml` (`:474-483`) — U40 as the directive relayed it.
- rust-src 1.98.1 `library/std/src/sys/stdio/windows.rs` (`:353-392`) — `read_u16s`.

## Graph impact (from the code-graph query)
One query on the rust plane (the trace above, `rows: 136`, `db_state: fresh`), keyed on `callee_name` + `callee_file`:
- **`DialogSlot::new` / `SendSlot::new`** (`new`, 22 sites) — production callers `src/cmd/run.rs:204`, `:209`; the rest are each file's own tests (`src/run/dialog.rs:427`, `:807`; `src/run/send.rs:500-755`) and `src/cmd/run.rs:586`, `:596` (its `methods()` test helper). A signature change to either constructor threads through exactly these sites.
- **`run::send::send`** (11) — production caller `src/cmd/run.rs:121` only; the rest are `src/run/send.rs` tests.
- **`DialogSlot::hook_dialog`** (18) / **`DialogSlot::answer`** (9) — production callers `src/cmd/run.rs:124` / `:125`; the rest are `src/run/dialog.rs` tests.
- **`DialogSlot::write_pending`** (2) — `src/run/dialog.rs:239`, `:272`: the one in-process snapshot read-modify-write besides `start_state` / `spawn_child`.
- **`write_snapshot`** (13) — production writers `src/cmd/run.rs:398` (`start_state`), `:436` (`spawn_child`), `src/run/dialog.rs:286` (`write_pending`); `src/cmd/hook.rs:659` is a test. **`read_snapshot`** (19) — `src/cmd/client.rs:96` (`live_endpoint`), `src/cmd/hook.rs:246`, `:397`, `src/cmd/run.rs:264`, `src/run/dialog.rs:284`.
- **`human::send_hint`** (8) — `src/cmd/send.rs:138`, `:146`, `src/cmd/client.rs:154` and `src/human.rs` tests. **`human::answer_hint`** (2) — `src/cmd/answer.rs:78`, test `src/human.rs:521`. **`write_unable`** (5) / **`write_send_unable`** (6) — the verbs' `Out` and the human tests.
- **`ProtocolError::body`** (2) — `crates/viola-channel/src/frame.rs:94`, `crates/viola-channel/src/lib.rs:242`: a new variant changes no caller.
- **`pump_with_paste`** (4) — `src/cmd/run.rs:470` and viola-pty's own; **`pump`** (12) — viola-pty only. The stdin source is the `input` argument, so wrapping stdin changes `src/cmd/run.rs` alone.
Crate edges: the chunk adds no dependency edge (the wheel lives in the root bin, which already depends on every crate it calls).

## Patterns detected
- **One-slot wrapper modules** (`src/run/send.rs:44-60`, `src/run/dialog.rs:59-68`): a struct behind `Arc`, built in `start` and shared with `Methods`; state under one `Mutex`, a `Condvar` for waiters, the clock injected.
- **Output tee, human first** (`src/run/gate.rs:130-140`): the human's bytes are written before any side processing, which only gets a copy. The input side has the same shape available: `pump_with_paste` takes any `Box<dyn Read + Send>`, so a `Read` wrapper around stdin sees every human byte before the child does, with no viola-pty change (`src/cmd/run.rs:470-472`).
- **Refusal records** (`src/run/send.rs:319-344`): one `send-refused` event (`refusal`, `detail`, `cursor?`) plus one codes-only `send-refused` obs line (`side:"wrapper"`, `corr`, `rpc_id`, `conn`, `srv_conn`, `from`, `from_trust`).
- **Per-method params + `from`** (`src/run/send.rs:180-197`): `parse_from` — absent, `null` or a valid `ViolaName`, else `-32602`.
- **CLI verb shape** (`src/cmd/answer.rs:56-140`): `Out { name, json }`; `client::start` → `live_endpoint` → one request → `Reply::{Ok, Refused, Fault}` → a typed exit and `run::log_self_exit`.
- **Fixed hint tables** (`src/human.rs:170-202`): one keyed `match`, literal strings, a case table test.

## Conventions to follow
- **Closed details live in `viola-core`** (`crates/viola-core/src/lib.rs:119-140`): a `human-typing` detail type (`manual-pause`) joins `NotDelivered` there, kebab `as_str`, no free string.
- **One snapshot writer per process** (`crates/viola-state/src/snapshot.rs:1-3`, `src/run/dialog.rs:283-288`): the wrapper is the only writer, but within it two threads (dialogs, and now the wheel) each read-modify-write the file; the plan must serialise them (one in-process holder of the snapshot under one lock), or a wheel write can be lost behind a pending-dialog write.
- **Logs through `obs_event!` only** (`.claude/rules/observability.md`): no per-byte line or span in the stdin path; one line or span per wheel transition.
- **Rendered refusal strings are literals in tests** (`tests/cli_controls_not_disableable.rs:146-150`).

## New files to create
- `src/run/wheel.rs` — the wheel: holder + cause under one lock, the stdin observer (a `Read` wrapper that classifies bytes and never holds one), the transitions (human key, `pause`, `release`), the `wheel` event and the snapshot update
- `src/run/snapshot.rs` — the wrapper's one in-process snapshot holder: every snapshot change (start, child pid, pending dialog, wheel) is a read-modify-write under its lock
- `src/cmd/pause.rs` — `viola pause <name> [--json]`
- `src/cmd/release.rs` — `viola release <name> [--json]`
- `tests/tui_wheel.rs` — Path 5 over the outer PTY, the a11y-plan §3 case (3) focus / mouse / resize case, and the Windows `^Z` and focus-report measurements
- `tests/cli_wheel.rs` — `pause` / `release` human and `--json` output, `release-from-driver`, `answer` under a human wheel, the dialog `null` on a wheel move

## Files to modify
- `src/run/mod.rs` — declare `wheel` and `snapshot`
- `src/cmd/mod.rs` — the `Pause` and `Release` subcommands and their dispatch arms
- `src/cmd/run.rs` — `pause` / `release` methods, the wheel and snapshot holder built in `start`, stdin wrapped before `pump_with_paste`, its `-32601` pin test retargeted
- `src/run/send.rs` — the `human-typing` step ahead of `turn-running`; `wheel` on every wrapper `send-refused` line; `refuse` taking a reason and a detail
- `src/run/dialog.rs` — `null` at once under a human wheel; a pending dialog answered `null` on a wheel move; `answer`'s `human-typing` slot; `write_pending` through the snapshot holder
- `src/cmd/send.rs` — the `human-typing` hint
- `src/cmd/answer.rs` — the `human-typing` hint
- `src/human.rs` — the `human-typing` hint rows, the `pause` / `release` result lines, and their tests
- `crates/viola-core/src/lib.rs` — the closed `human-typing` detail type and the closed wheel cause
- `crates/viola-channel/src/lib.rs` — a `-32602` variant whose `data` is `{"reason":"release-from-driver"}`
- `crates/viola-channel/src/server.rs` — the new variant in the `-32602` classification
- `crates/viola-pty/src/lib.rs` — the Windows console stdin reader that keeps `0x1A` and never reads an empty console read as end of input
- `tests/channel_endpoint.rs` — the `pause` call retargeted to the still-unbuilt `unlink`
- `tests/cli_controls_not_disableable.rs` — the human-wheel `send` exit-10 negative on every row, and its header
- `tests/contract_fixture_hygiene.rs` — the `andromeda:walks-tree` comment line
- `tests/contract_ledger_probes.rs` — the `andromeda:walks-tree` comment line
- `tests/contract_fake_agent_drift.rs` — the `andromeda:walks-tree` comment line

## Sweeps
- `"pause"|"release"|human-typing|manual-pause|HumanTyping` over `src crates tests` (`grep -rn … --include=*.rs`): 19 hits · 4 changed (`src/cmd/run.rs:613`, `tests/channel_endpoint.rs:223/:249-250`, `src/human.rs:515`) · 15 no-change (`crates/viola-channel/src/{client,server}.rs` unit stand-ins answering a canned `pause`; `crates/viola-core` serde pins; `src/cmd/send.rs:87/:260` already map exit 10; `src/cmd/client.rs:178` already parses a `human-typing` reply; `tests/contract_diag_schema.rs:154` already validates a `human-typing` wrapper line; two `release` hits in `crates/viola-state/src/snapshot.rs` and `crates/viola-e2e/src/harness/run/perf.rs` are unrelated words).
- Outer-PTY writers before a driver `send` (`grep -rn '\.send(b\|pty\.write(\|\.write(b' tests crates/viola-e2e`): 25 hits · 0 changed · 25 no-change (every one is a Ctrl-C at stop, a key into a bare fake agent, or `chaos_feed_panic.rs:104`'s key after its refused send).

## Mechanisms re-derived at HEAD
- **The held paste window** — the equality: a human chunk read while a paste holds `PasteHandle`'s lock is written whole after the paste's one `write_all`, never inside it. Produced by `pump.rs:82-87` (paste under the lock) and `:96-101` (each human chunk under the same lock); witnessed by `paste_human_bytes_during_a_paste_land_wholly_after_it` (`pump.rs:814`). Exists at HEAD; this chunk adds no hold.
- **Resize never takes the wheel** — the equality: a host resize yields `pty.resize(size)` and no stdin byte. Produced by `pump.rs:186-193`; on Unix the size arrives by `TIOCGWINSZ` polling, on Windows by the console buffer query, neither through `io::stdin()`. An observer on stdin never sees a resize.
- **Terminal replies arrive on stdin** — the equality: on Windows x64 the sideloaded ConPTY writes `ESC[c` at spawn and the outer terminal's DA1 answer reaches the child through `viola run`'s stdin → pump (`tests/tui_passthrough.rs:101-104`; chunk 2026-09-29-sideloaded-conpty `evidence/da1-stall.md`, measured). A classifier that counts every byte but focus and mouse would therefore move the wheel to `human` at every Windows x64 start with a terminal. This decides that terminal replies (DA1/DA2 `CSI ?/> … c`, CPR `CSI … R`, DECRPM `CSI ? … $ y`, OSC and DCS replies) need a ruling (P4).
- **`^Z` on Windows console stdin (CARRY §8)** — source-read at 1.98.1, runtime unmeasured: `read_u16s` (`windows.rs:353-392`) drops a trailing `0x1A` from every `ReadConsoleW` result. A read of `^Z` alone returns `Ok(0)`, which `pump.rs:46`'s `copy` reads as end of input, so the human→child copy thread ends and no later key reaches the child. The mask (`dwCtrlWakeupMask`) matters only under line input, which `HostTerminal` clears, but the strip at `:389-391` runs in every mode. The carried hypothesis ("may treat a leading `^Z` as end of input") holds by the source and is wider than stated: any read ending in `^Z` loses that byte. To measure: a `windows-2025` outer-PTY case writing `\x1a` then `k`.
- **ConPTY focus reports (CARRY §9)** — not re-derivable on this host (no Windows): the outer PTY on `windows-2025` is the inbox ConPTY, and whether it re-emits `ESC[I` / `ESC[O` into `viola run`'s VT-input console is the measurement. Unmeasured; carried as the hypothesis to measure.
- **`release` clears the running-turn state** — at HEAD there is none to clear: `turn-running` is raised only by a second send while one is in flight (`src/run/send.rs:217-222`); nothing tracks `prompt-submitted` → `turn-ended`. The in-flight slot belongs to a live `send` and `release` must not empty it. The `prompt-submitted` → `turn-ended` half of architecture [Human Takeover / Wheel]'s `turn-running` has no owner on the route (`grep -n 'turn.running' viola-0.1.0/working-route.md`: only `:80`'s CARRY, which names the refusal order, not the state).
- **Release's `from`** — the CLI verbs add `from` from `VIOLA_NAME` (`src/cmd/client.rs:105-108`, `src/cmd/send.rs:188-190`), so `viola release` run inside a wrapped session (Path 5 step 7, `VIOLA_NAME=overseer`) carries `from` and is refused `-32602` `release-from-driver` → exit 20 through `client::fault`.

## Open questions
- Do terminal replies (DA1/DA2, CPR, DECRPM, OSC/DCS replies) count as human editing keys? The architecture names only focus, mouse and resize; the measured DA1 reply on Windows x64 decides the cost of "yes" → blocks: plan-decision
- Do the CLI `pause` / `release` frames ride the liveness-only pre-check as a seventh dated gap, or land full server verification here? → blocks: plan-decision
- The `prompt-submitted` → `turn-ended` running-turn state has no route owner → blocks: plan-decision (surfaced at P5, not folded)
