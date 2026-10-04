# Codebase Research — 2026-10-04-running-turn-refusal

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 8 Session Additions applied (the `test(=tests::name)` / `binary(<stem>)` selector rule; never pipe `boot`); `.claude/rules/testing.md` — read in full, 27 Session Additions applied (`--e2e` is a usage error until the first E2E binary, so root tests ride `run --integration`; a chunk `[[gate]]` block holds no mutation entry; `Wrapper::stop`'s Ctrl-C adds a `wheel` record; wait on the exact line a test asserts)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a read `eb37914` green) and no CI-reading entry outside the operator leg

## Files inspected
- `src/run/send.rs` (1-260, 470-600) — `SendSlot` (`:49-55`: clock, wheel, io, `in_flight: Mutex<Option<InFlight>>`, `settled`), `append_hook_event` (`:139-161`), the `send` method and its rungs (`:213-265`), the `send_refusal_order` rstest table with its `Setup` enum (`:509-600`).
- `src/cmd/run.rs` (70-135) — `HOOK_KINDS` (`:78-84`: session-start, prompt-submitted, turn-ended, session-end, activity), `hook_event_line` re-validation (`:89-108`), `Dispatch::dispatch` → `send::append_hook_event` (`:111-121`), `dispatch_call` routing `send` / `pause` / `release` (`:123-135`).
- `src/run/wheel.rs` (20-200) — `next()` (`:39-47`: `Release` moves only a human-held wheel, else `None`), `WheelSlot` `held: Mutex<Held>` (`:73-76`), `apply` (`:123-143`), `release` (`:167-182`: the string-`from` refusal and `parse_from` run first, then `budget`, then `apply(Input::Release, true)` only when `budget` is false).
- `src/run/wait.rs` (55-125) — `WaitFeed::appending` (`:71-84`): the append runs inside the feed lock, and `turn-ended` updates `newest` under that lock. `rebuild` (`:110-125`) reads the log once at start.
- `src/cmd/hook.rs` (160-180, 395-425) — `deliver` sends `hook.event` as a notification (`:396-410`); a SessionEnd the channel did not take goes through `append_session_end` → `try_append_event`, a direct log append (`:174-175`, `:412-423`).
- `src/bin/viola-fake-agent.rs` (285-345, 466-520, 531-550) — `submit` fires only `UserPromptSubmit` (`:288-307`). `run_steps` fires each scripted step after its control gate, and the `--inject-harness-turn` step submits `HARNESS_TURN` with origin `harness` (`:328-342`, `:466-476`). Interactive stdin submits are `human` (`:539`). Only `print_turn` (`-p`) fires an unscripted Stop (`:513-519`).
- `tests/cli_send.rs` (20-40, 150-300) — `boot` is an unscripted fake agent (`:23-37`). `path2_send_confirms_with_cl1_events` sends twice (`:158`, `:216`) with no Stop between. `send_window_second_concurrent_send_is_refused_turn_running` (`:263-305`) pins the in-flight cause and the human-mode refusal text.
- `tests/tui_wheel.rs` (120-265) — `path5_human_takes_the_wheel_and_release_returns_it` (`:158-237`: a human prompt `hello`, `release`, then a driver `send` exits 0). `path5_harness_turns_never_take_the_wheel` (`:240-258`: an injected harness prompt, no Stop, then a driver `send` asserted exit 0). The stale citation is at `:267`.
- `tests/cli_wheel.rs` (120-210) — `path5_pause_refuses_send_and_answer_and_release_returns_the_wheel`: no prompt is typed before its sends, and its one accepted `send` (`:203`) is the last send.
- `tests/cli_wait_last.rs` (335-360, 500-515), `tests/cli_controls_not_disableable.rs` (115-132, 228-245), `tests/channel_endpoint.rs` (295-312), `tests/cli_fake_agent.rs` (590-612) — single-send or refused-before-the-rung sites.
- `tests/cli_answer.rs:9` — the stale `:82` citation (re-verified).
- `src/human.rs:214` — the `turn-running` hint `a turn is running; viola wait {name} first` already exists.
- `fixtures/fake-scripts/{gated-turn,path3,path4}.json` and `tests/contract_fixture_hygiene.rs:142` (`#[files("fixtures/fake-scripts/*.json")]`).
- `.andromeda/test-plan.md:806-830` — §6 Path 5's steps and verification signals (the harness-turn signal names no `send` outcome).

## Graph impact (from the code-graph query; trace `tree-query-2026-10-04-running-turn-refusal.json`, rust plane)
- **append_hook_event** — 1 production caller: `Methods::dispatch` @ `src/cmd/run.rs:117`. The other 10 calls are `src/run/send.rs` unit tests (`:640`–`:879`). Every hook kind the wrapper takes enters through this one function. That is the single place to mark and clear the running turn.
- **release** (`WheelSlot::release`) — production caller `dispatch_call` @ `src/cmd/run.rs:131`. The same name also covers the CLI verb `cmd::dispatch` @ `src/cmd/mod.rs:109` and the test-support `Wrapper::release` (the fake agent's control-line writer, `tests/support/home.rs:397`) with its many test callers. Those are name collisions, not callers of the wheel method.
- **send** (`send::send`) — production caller `dispatch_call` @ `src/cmd/run.rs:125`; 11 unit callers in `src/run/send.rs`. The CLI-side `send` hits are other functions sharing the name.
- **human_input** — production callers `append_hook_event` @ `src/run/send.rs:155` and `Observed::read` @ `src/run/wheel.rs:282`. This is the precedent: a state move made before the hook line is appended.
- **try_append_event** — `append_session_end` @ `src/cmd/hook.rs:422`, the only product writer that bypasses the wrapper.
- No signature changes. The new state is internal to `src/run/`, so no caller threading is owed.

## Patterns detected
- **State moved before the line is readable** (`src/run/send.rs:152-156`): an unsent human prompt moves the wheel before `feed.appending`, so anyone who sees the line on disk already sees the move. The running-turn mark and clear follow the same rule. A driver that read `turn-ended` through `wait` is then never refused by that turn, and one that read a `prompt-submitted` is.
- **One lock per related state; records off the hot path** (`src/run/wheel.rs:73-76`, `:123-143`): holder and cause under one `Mutex`, the record queued under the same hold. The running-turn state carries no record (scope, the obs extract), so it is a plain in-memory flag on a std lock.
- **Refusal through the existing emitter** (`src/run/send.rs:242`): `ctx.not_delivered(None, NotDelivered::TurnRunning)` already writes the `send-refused` record and line, with no `cursor`, before `send-issued`. The new cause reuses it unchanged.
- **Parameter checks before any state change** (`src/run/wheel.rs:170-178`): `release` refuses a string `from` and bad params before `apply`, so a clear placed with the move inherits "a rejected release changes nothing".
- **Gated fake-agent steps as the turn driver** (`src/bin/viola-fake-agent.rs:328-342`): a turn's end in a test is a scripted `Stop` released through the control file, never a timer.

## Conventions to follow
- **Refusal oracle as literals in an rstest table** (`src/run/send.rs:525-571`): new `Setup` arms (a turn started by each origin, a turn ended by each end kind, a turn cleared by `release`) join `send_refusal_order` or a sibling table, with wire values (`{"refusal":"not-delivered","detail":"turn-running"}`) as the oracle.
- **Test selection**: root tests ride `scripts/agent-run.sh run --integration`, a whole file by `binary(<stem>)`. `--e2e` is exit 2 at HEAD (`.claude/rules/testing.md:46`).
- **Hook kinds by `EventKind`, never by text** (`src/cmd/run.rs:78-84`): the turn reads `line.kind`. Origin comes from the hook's compiled classifier, already in `line.data["origin"]`, and is never re-parsed.
- **Waits on exact lines** (`.claude/rules/testing.md:53`): an E2E step that needs "turn running" waits for the `prompt-submitted` line, and one that needs "turn ended" waits for `turn-ended` (or a `wait --after` return).

## New files to create
- none

## Files to modify
- `src/run/send.rs` — the running-turn state is marked by any `prompt-submitted` and cleared by `turn-ended` / `session-start` / `session-end` inside `append_hook_event`, before the append. The `send` method's `turn-running` rung reads it beside the in-flight slot. New unit cases join the refusal-order table.
- `src/run/wheel.rs` — a wheel-returning `release` clears the running-turn state; unit cases.
- `tests/cli_send.rs` — `path2_send_confirms_with_cl1_events` ends the first turn (a released scripted Stop, its `turn-ended` awaited) before its second send; a new integration case for a driver send refused during an unended turn.
- `tests/tui_wheel.rs` — `path5_harness_turns_never_take_the_wheel` becomes the hypothesis witness (refused `turn-running`, nothing typed, then accepted once the turn ends); the `:267` citation reads `:84`.
- `tests/cli_answer.rs` — the `:9` citation reads `:84`.
- `tests/cli_wheel.rs` — a turn left running under a driver-held wheel: a bare `release` leaves it, and `pause` then `release` clears it (added at P4).
- `tests/cli_controls_not_disableable.rs` — the `turn-running` refusal holds under every setting, beside the human-wheel row (added at P4).

## Mechanism claims re-derived at HEAD
- **The CARRY's hypothesis**, "a driver `send` during a turn the driver did not start … is typed into that turn": VERIFIED. At HEAD the gate's only `turn-running` condition is an occupied flight slot (`src/run/send.rs:239-243`). `path5_harness_turns_never_take_the_wheel` (`tests/tui_wheel.rs:240-258`) drives exactly that case and asserts `send` exit 0 (confirmed, so pasted) after a harness `prompt-submitted` with no `turn-ended`. The suite has measured it green on all three CI OSes since the wheel's wrap.
- **"A turn can end with no `turn-ended`"** (architecture): the fake agent's interactive submit fires no Stop (`src/bin/viola-fake-agent.rs:288-307`). Every unscripted turn in the suite is such a turn. The loss of the SessionEnd channel delivery (`src/cmd/hook.rs:174-175`) is a second, product-side way to miss an end.
- **The equality the design needs**: for the inputs (a `prompt-submitted` line, then a `send` with no `send` in flight and the wheel at the driver), `send` returns `{"refusal":"not-delivered","detail":"turn-running"}` and writes no paste byte. For (a `turn-ended` | `session-start` | `session-end` line, then that `send`), it passes the rung. The deciding control is `append_hook_event` (`src/run/send.rs:139-161`), the one entry for every hook line (graph: 1 production caller). The deciding frame is the `send` rung at `:239-243`.

## Companion sweep
- `turn-running|TurnRunning|turn_running` (re-derived: `grep -rn 'turn-running\|TurnRunning\|turn_running' --include=*.rs src crates`): 17 hits · 1 file changed (`src/run/send.rs`) · 16 no-change. `src/human.rs` (the hint exists), `src/cmd/client.rs` (reply parsing), and `crates/viola-core/src/lib.rs` (the closed detail exists, no new value) need nothing.
- Root tests that make a driver `send` after an unscripted prompt (re-derived: the `"send"` arg sites in `tests/*.rs` plus the `send_*` helpers, each read): `tests/cli_send.rs:216` and `tests/tui_wheel.rs:254` change. The no-change sites are `tests/tui_wheel.rs:199` (after `release`, which clears), `tests/cli_wheel.rs:203` (no prompt before it), `tests/cli_wait_last.rs:345` (the first send), `tests/channel_endpoint.rs:309` (one send), `tests/chaos_feed_panic.rs:74` (input-not-ready), and `tests/cli_controls_not_disableable.rs:128,240` (refused before the rung).

## Scope premise closure
- `answer` / `hook.dialog` ungated by the running turn: VERIFIED. Architecture §Conventions' `answer` order has no `turn-running` rung, and `DialogSlot::answer` (`src/run/dialog.rs`) is routed separately (`src/cmd/run.rs:128-129`). The tag is dropped.
- Whether `turn-ended` / `session-start` / `session-end` reach the wrapper by the same path as `prompt-submitted`: VERIFIED, with a residual. All four ride `hook.event` → `append_hook_event` (`src/cmd/run.rs:78-84`, `:111-121`). The exception: a SessionEnd the channel did not take is appended directly (`src/cmd/hook.rs:174-175`), unseen by the wrapper. That loses nothing a driver can act on, because the child is ending.
- Whether `release` clears the turn on a driver-held wheel: the code's `next()` makes such a release a no-op (`src/run/wheel.rs:44-45`), matching architecture's "changes nothing and appends nothing". A turn stuck under a driver-held wheel is still recoverable without a keystroke: `viola pause` then `viola release`. Left to P4 as a lean.
- Snapshot / event carriage: VERIFIED none. The state is not among the snapshot's closed fields or replay's log-derived fields, so it is in memory only, and a restarted wrapper starts with none (its child's `session-start` ends any turn anyway).
- The CARRY hypothesis mechanism: VERIFIED (above).

## Open questions
- Which slot holds the running-turn flag (`SendSlot` beside `in_flight`, or `WheelSlot` beside `held`). `release` lives on `WheelSlot` and `send` holds an `Arc<WheelSlot>` (`src/run/send.rs:52`), so either reaches both without a signature change → blocks: implementation-scope.
- A turn that starts between the rung and the paste (during the readiness gate's wait, up to `GATE_MAX_WAIT`) is not refused by the rung. The architecture's rule is about a send that arrives while a turn runs → blocks: plan-decision (P4 leans: the rung decides at arrival; the window is recorded, not closed).
