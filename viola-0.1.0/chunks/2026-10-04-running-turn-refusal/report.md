# Report — 2026-10-04-running-turn-refusal

**Chunk:** Running-turn refusal — a prompt-submitted of any origin marks a turn running, turn-ended / session-start / session-end end it, an automated send refused turn-running meanwhile, viola release clears it
**Date:** 2026-10-04T22:29Z
**Commits:** since `last_wrap` 2026-10-04T21:12:00Z, base `eb37914`:
- `93a5cbf chore(2026-10-04-running-turn-refusal): operator pre-CI commit, for the run this chunk's verdict reads`
- `334ee7f fix(2026-10-04-running-turn-refusal): operator fix after the wrap's light gate — the two new turn witnesses wait for the prompt receipt before counting it`

## Changes (structured — detectors read this)
- **Files:** `src/run/wheel.rs` · `src/run/send.rs` · `src/bin/viola-fake-agent.rs` (test-only bin, feature `fake-agent`; a recorded widening, see Deviations) · `tests/tui_wheel.rs` · `tests/cli_wheel.rs` · `tests/cli_send.rs` · `tests/cli_controls_not_disableable.rs` · `tests/cli_answer.rs` (basis: `git diff --name-only eb37914` over `src tests`, 8 files; `gate.py scope` clean, changed 8 · listed 7 · recorded 1).
- **Symbols / APIs:**
  - `src/run/wheel.rs`: private `enum Turn { Idle, Running }`, a third field `turn` in `Held` beside `holder` and `cause` (starts `Idle`). New crate-private `WheelSlot::turn_started` / `turn_ended` / `turn_running`. None records anything: no event, snapshot field, span or obs line. `apply` sets `turn = Idle` in the same `held` hold when an `Input::Release` move is made (the `next()` arm for a human-held wheel only). A `release` the wheel ignores (driver-held), `release {budget:true}` and a refused `release` (`-32602 release-from-driver`, invalid params) never reach that line. `pause` and `Input::Human` never touch it.
  - `src/run/send.rs` `append_hook_event` (one production caller, `Methods::dispatch` @ `src/cmd/run.rs:117`, unchanged): before `feed.appending`, keyed on `line.kind` only, `PromptSubmitted` (any origin, the claimed driver prompt included) → `turn_started`; `TurnEnded` · `SessionStart` · `SessionEnd` → `turn_ended`; anything else (`activity`) → nothing. The turn is marked AHEAD of the existing human-wheel move, so a `release` landing between the two still clears it.
  - `src/run/send.rs` `send` (caller `dispatch_call` @ `src/cmd/run.rs:125`, unchanged): the existing rung now reads `flight.is_some() || slot.wheel.turn_running()` under the flight lock → `ctx.not_delivered(None, NotDelivered::TurnRunning)`, the one existing emitter (`send-refused` without `cursor`, no `send-issued`, nothing pasted). Lock order flight → held; `WheelSlot` holds no `SendSlot`, so there is no inversion. The order is unchanged: `control-character` → `human-typing` → `turn-running` → readiness gate (`input-not-ready`) → confirmation (`no-prompt-submitted`).
  - `src/bin/viola-fake-agent.rs` (test-only): `Agent` gains `hooks: Mutex<bool>`, held across each `run_hook` spawn+wait. New `Agent::new` and `Agent::close_hooks`, called on `\x03` and at stdin EOF: the agent waits for a hook still running and starts no other before it exits (a closed agent's `run_hook` returns without spawning).
  - No new IPC method, param, event kind, refusal detail, snapshot field, env var, CLI flag or test seam.
- **Crates / modules:** none added or removed; changed `viola` (root bin) only.
- **Dependencies:** none (probe: `git diff eb37914 -- Cargo.toml Cargo.lock deny.toml deny-sync.toml` adds no `skip`/`ignore`/`exceptions`/`allow`, exit 1 / count 0).
- **Schema / config:** none (`crates/viola-core/src/lib.rs`, `src/run/snapshot.rs`, `src/cmd/hook/seam.rs`, `scripts/g2-zero-panics.sh` byte-identical to `eb37914`: the gate's `git diff --quiet` probe, exit 0).
- **Spec-master edits:** none during the chunk.
- **Counts / qualifiers moved:**
  - The architecture's `send`-refusal rationale "as built, no running-turn state exists beyond the in-flight `send` slot … so `release` returns the wheel alone" (`architecture.md:70`, grep `no running-turn state`: 1 hit) is now false. A running-turn state exists, and a wheel-returning `release` clears it.
  - `turn-running` covers two causes: a running turn, or another `send` in flight. Sites that enumerate the detail without its cause stay true: arch `:133`, `:136` (it already reads "a turn running, or another `send` in flight"), obs-plan `:314` · `:644` · `:836`, design-system `:163` · `:572` · `:762` · `:820` (grep `turn-running`: arch 3 · design-system 4 · test-plan 1 · obs-plan 3 hits).
  - test-plan `:660` / `:661` (the controls table). `:660`'s "as landed" list gains the turn-running row (`send` during a running harness turn → exit 13 under every setting). `:661`'s four-negative list is the planned shape and does not name it.
- **Dev-tool versions:** none — cargo-nextest, cargo-llvm-cov, llvm-profdata (toolchain 1.98.1) re-read unchanged.
- **Harness / gate surface:** the fake agent's exit (test-plan §3 fake agent, `test-plan.md:1077` "It exits on `\x03`." — grep `exits on`: 1 hit in test-plan, 0 in `.claude/rules/verification-harness.md`). Now it exits on `\x03` or at stdin EOF, and before exiting it waits for a hook still running and starts no other. Why: a hook is in the agent's PTY foreground process group, and the agent's exit as session leader hangs that group up, cutting a hook off mid-exit (measured, the `.profraw` WATCH, `evidence/watch-profraw.md`). No harness verb, status shape or CI step changed.
- **Cross-project / external claims:**
  - CI ci#37239689446 on `93a5cbf4f13e` (the pre-CI commit): `verdict: green · checks 15/15 · wall 287 s`.
  - CI ci#37241137053 on `334ee7f3d0ef` (the operator fix after the light gate = the chunk's final code): `verdict: green · checks 15/15 · wall 290 s`. This wrap's commit adds only bookkeeping and the plan's entry-8 correction on top.
  - The overseer (founder-delegated) gave the fold-now word for the fake-agent widening, relayed by the operator in-session.
  - `inputs: absent — no external input snapshotted` (`inputs.py verify`).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):**
  - The wheel chunk's exit-flush reorder (`src/cmd/run.rs`, the flush before the raw-mode restore) was written for the `.profraw` red under the "wrapper killed mid-write" hypothesis. It is correct and kept: the wheel records still flush before the terminal is restored.
  - It was not the remedy. The truncated profile is a `viola hook`'s, cut by the fake agent's session-leader exit (`evidence/watch-profraw.md`). The remainder is owned and closed by this chunk's fake-agent fix.
- **Spec claims disproved by measurement:**
  1. `architecture.md:70`: "as built, no running-turn state exists beyond the in-flight `send` slot … so `release` returns the wheel alone". False after this chunk (the plan's expected amendment).
  2. This chunk's `plan.md`:
     - **The claim:** Acceptance Criteria "The turn's life" and Implementation Step 5 state that a `prompt-submitted` of origin `human` makes the next `send` refuse `turn-running`.
     - **Measured:** the human prompt does mark the turn (`send_a_prompt_of_any_origin_starts_a_turn::case_2_human` asserts `turn_running()`). But an unsent human prompt moves the wheel to the human before its line is appended, so the next `send` reads `human-typing` (the earlier rung). The only wheel return is a `release`, which clears the turn. The `turn-running` reading is therefore unreachable for a human-origin turn.
     - **Evidence:** the unit case asserts reply `{"refusal":"human-typing","detail":null}` with zero pastes.
     - **Disposition:** the operator's directive (2) to amend that acceptance line as measured.
  3. scope.md's folded watch hypothesis: "a harness `cleanup` Ctrl-C reached a wrapper mid profile write".
     - **Measured false:** the truncated profile decodes (raw profile v10, counters intact) to a `viola hook` handling `UserPromptSubmit`, not a wrapper.
     - **The kill:** the fake agent's exit as PTY session leader (SIGHUP to its foreground group), triggered by `hook_events::hook_prompts_arrive_normalised_with_their_origin` stopping right after its last line (`evidence/watch-profraw.md`).
- **Expected amendments (from plan):**
  - architecture §Established Decisions [Human Takeover / Wheel], retire the "as built" clause and state the running-turn state:
    - **Status:** carried (Symbols / APIs + Spec claims disproved 1).
    - **Site:** grep `no running-turn state` → architecture.md 1 hit (`:70`), 0 in the other six.
    - **Residuals** (named in that entry):
      - The readiness-gate window: a turn starting during the gate's wait (≤ `GATE_MAX_WAIT`, 5 s provisional) is not refused. This is lean 2, the operator's directive (4), recorded at P5.
      - The SessionEnd direct append (`src/cmd/hook.rs:174-175`) leaves the turn marked.
  - test-plan §6 Path 5 (harness-turn verification gains the `turn-running` refusal and acceptance after `turn-ended`; `tests/cli_wheel.rs` adds the `pause` / `release` recovery):
    - **Status:** carried.
    - **Sites:** grep `inject-harness-turn` → test-plan 2 hits (`:817` Path 5 step 8, `:1076` the fake agent's modes); Path 5 "As landed" `:808` and the signal `:826`.
  - test-plan §6 Path 2 (its two sends span a scripted turn end):
    - **Status:** carried.
    - **Site:** the Path 2 "As landed" sentence `:734` (heading `:732`). `path2_send_confirms_with_cl1_events` now boots over `fixtures/fake-scripts/path3.json` and releases its gated `PostToolUse` + `Stop` before its second send.
  - test-plan §5 CLI controls table (the turn-running row):
    - **Status:** carried (Counts / qualifiers moved).
    - **Site:** grep `human wheel` → test-plan 2 hits (`:660`, `:661`).
  - obs-plan §4 Scenario: Confirmed `send` (`turn-running` now also covers a running turn, same closed detail, no new field):
    - **Status:** carried.
    - **Sites:** grep `turn-running` → obs-plan 3 hits: `:644` (in the Scenario at `:629`), `:314`, `:836` (both detail lists, still true).
  - `matrix#v1-32 notes` (`path5_harness_turns_never_take_the_wheel` now also asserts the `turn-running` refusal; `viola release`'s `send` exit 0 rests on the running-turn clear; status / ref / acceptance untouched): ledger-note — owner P7.3.
  - `matrix#v1-29 notes` (the "Matrix notes owed at P5" entry): carried, already written at phase P5 (`matrix.py show --id v1-29` reads its `2026-10-04 (2026-10-04-running-turn-refusal, phase P5)` note).
- **Coverage of new surfaces:**
  - `send` rung `turn-running` (running-turn cause) → validation n/a (no new input; the turn keys on the closed `EventKind`, never on text) · instrumentation `send-refused` log✓ (the existing emitter: `side`, `wheel`, `corr`, `conn`, `rpc_id`, `detail`; asserted in `cli_wheel::path5_a_turn_left_running_is_cleared_by_pause_then_release`) · PII n/a (no text logged; the canary-clean check `assert_logs_clean` runs in that test) · tests unit (`send_refusal_order` ×4 new arms, `send_a_prompt_of_any_origin_starts_a_turn` ×2, `send_a_turn_end_lets_the_next_send_past_the_rung` ×4, `send_after_a_confirmed_send_is_turn_running_until_turn_ended`) / integ (`cli_send`, `cli_wheel`, `cli_controls_not_disableable`) / e2e-class tui (`tui_wheel::path5_harness_turns_never_take_the_wheel`) · a11y n/a (CLI text refusal unchanged: `unable … not-delivered  turn-running` + one `hint:` line, no SGR) · tokens n/a
  - `release` running-turn clear → validation n/a (params checked before any move, unchanged) · instrumentation n/a (in-memory; the existing `wheel` record and span) · PII n/a · tests unit (`wheel_only_a_release_returning_the_wheel_clears_the_turn` ×4, `wheel_turn_is_marked_and_ended_and_a_pause_or_a_key_leaves_it`) / integ (`cli_wheel` recovery) / tui (`path5_human_takes_the_wheel_and_release_returns_it`, its post-release send) · a11y n/a · tokens n/a
  - fake agent hook quiescence at exit (test-only) → validation n/a · instrumentation n/a · PII n/a · tests: the coverage merge (pre-push `linux-tests`) 4 consecutive green after the fix · a11y n/a · tokens n/a

## Deviations from intent
1. **Human-origin turn case (plan Step 5 / Acceptance "The turn's life").** The case asserts the turn state read (`turn_running()`) and the `human-typing` refusal with zero pastes, not `turn-running`. Justification: Spec claims disproved 2; the operator's directive (2) amends the acceptance line.
2. **Gate "no `:82` citation"** (`git grep -F -e '`:82`' -- tests src | wc -l`, expect `exit 0` + `last line 0`) reads `red · exit 0 ✗ (exit 1)` with `last line 0 ✓`.
   - **The subject holds:** no `:82` citation remains.
   - **The atom cannot hold on a green subject:** the gate shell runs `pipefail`, and `git grep` exits 1 on zero matches. Its P5 baseline was only ever read on the red (matches) side.
   - **Status:** a plan defect, not edited at /implement. Directive (1) brings it to the operator as a card at P7.1.
3. **The `.profraw` WATCH folded as a fix, not only an observation** (scope.md: "rides as an observation only"). The WATCH recurred in this chunk's first pre-push, so per the project rule a red met during a chunk folds into it. The cause was measured and fixed in `src/bin/viola-fake-agent.rs` on the overseer's founder-delegated word.
4. **A new unit test reworked under its own red-before-green control.** With the rung neutralised, `send_after_a_confirmed_send_is_turn_running_until_turn_ended` hit a 120 s nextest TIMEOUT: under a fixed clock, the wrongly pasted second send parked on its confirm window, which a mutant would grade timeout, not caught. It now uses a clock that jumps only during the refused send, and the control reads FAIL in 0.186 s (`evidence/controls.md`).

5. **Light-gate fold, a receipt-count race in the two new turn witnesses** (`light-gate-fix.md`).
   - **The red:** at this wrap's light gate, `cli_send::send_after_a_confirmed_send_is_turn_running_until_turn_ended`
     read 0 prompt receipts where it expected 1. `tui_wheel::path5_harness_turns_never_take_the_wheel` counts the same
     way.
   - **Why:** the fake agent receipts a prompt only after its hook returns, which can be after the send's reply or
     the line.
   - **Witness:** forced open with a temporary receipt hold, red on both, then green with the fix (each count now
     waits for at least one prompt receipt). Operator fix commit `334ee7f`, on the overseer's founder-delegated word.
6. **Entry 8 corrected in the plan** (dated in its `note`, on the overseer's founder-delegated word, operator
   directive 1): `{ git grep -F -e '`:82`' -- tests src || true; } | wc -l`, both atoms kept.
   - **Discrimination** (run under `bash -o pipefail -c`): a scratch repo with one `` `:82` `` citation reads
     `last line 1` (red); the tree reads exit 0 · `last line 0` (green); the old form reads exit 1 on the same tree.

Scope record (`scope-record.md`; `gate.py scope` at P1: `scope: clean — changed 8 · listed 7 · recorded 1 (companion 0 · mechanical 0 · in-intent 0 · widening 1) · absorbed 0 · excluded 59`):
- widening — `src/bin/viola-fake-agent.rs`, serves the folded .profraw WATCH (scope.md), pre-push gate 17 — word: "overseer (founder-delegated, fold-now): the cause is measured and the fix is test infrastructure only (no product code, seam or env read), so record src/bin/viola-fake-agent.rs as a scope-record widening on the operator word" — delegate overseer (founder-delegated), relayed by the operator, 2026-10-04.

## Decisions & corrections
- **Operator directives for this wrap:**
  1. Gate entry 8 (the `:82` probe) is a plan defect. Bring it as a priced card at P7.1 and never halt silently.
  2. Amend the step-5 acceptance line as measured (a human prompt reads `human-typing` first).
  3. Record the `.profraw` WATCH as CLOSED (cause measured, fake-agent fix, 4 green in a row).
  4. Record lean 2 (the up-to-5-s readiness window) as the residual.
- **The overseer's ruling on the WATCH** (founder-delegated, fold-now, relayed): fold the fake-agent fix as a recorded widening. The evidence names the failing pre-push and the green re-runs, and the WATCH closes only on 3 consecutive green pre-push runs after the fix. Tally: run 1 red, runs 2–5 green, 4/4 consecutive after the fix (`evidence/watch-profraw.md`).
- **Leans carried from the plan unchanged:**
  - Only a wheel-returning `release` clears the turn; recovery under a driver-held wheel is `pause` then `release`, witnessed.
  - The rung decides at arrival.
  - The turn is marked before the line is appended.
  - The state lives in `WheelSlot`'s `Held`.
- **The light-gate card (operator, overseer founder-delegated word, at this wrap):**
  - Entry 7's red is closed by the operator fix `334ee7f`, with red-before-green on the forced race.
  - Entry 8's run is rewritten as a dated plan correction, with both atoms kept and discrimination shown.
- **Test-authoring hazard:** a fake-agent `prompt` receipt is written only after its UserPromptSubmit hook returns. A
  count taken right after the send's reply, or right after the `prompt-submitted` line, races it. Wait for the
  receipt first (`prompts_at_least` / `fake::wait_for`).
- **Sweep hazard (gate authoring):** a zero-is-healthy count probe written as `git grep … | wc -l` with an `exit 0` atom can never pass on its green state under a pipefail shell (`git grep` exits 1 on no match). The gate tool runs `bash -c` with pipefail, so `last line 0` alone, or `{ git grep … || true; } | wc -l`, is the green-reachable form. The P5 baseline read only the red side, so nothing exercised the green.
- **Test-authoring lesson:** a remove-the-guard control can turn a unit test whose guard is a refusal into a hang. A wrongly accepted send under a fixed test clock parks on its confirmation window forever. Give the refused call a clock that can expire, so the control reads FAIL, not TIMEOUT.
- **Diagnosis technique:** a truncated LLVM raw profile whose cut falls in its trailing names section keeps every counter. Match the record name hashes (MD5 lower 64 bits of the mangled names, taken from a complete sibling of the same binary signature) to read which functions the dead process ran, and so name the process with no surviving home or log.
- **Fake-agent fact:** `--script` / `--inject-harness-turn` steps fire on a detached thread, and the agent is its PTY's session leader. Any test that stops the wrapper right after a scripted hook's event line lands could, before this fix, SIGHUP that hook mid-exit. A hook killed before its profile dump left no profile at all, a silent coverage loss.

## Outcome
**Acceptance criteria (re-asserted against the diff):**
- (arch) **The hypothesis, closed:** MET. `tui_wheel::path5_harness_turns_never_take_the_wheel`, green on 3 OSes in ci#37239689446:
  - `send --json` exits 13 with `{"v":1,"refusal":"not-delivered","detail":"turn-running"}`;
  - the receipt holds one `prompt` (the harness one);
  - the records after the pre-send line count are one `send-refused` `{refusal, detail}` with no `cursor`;
  - after the scripted Stop's `turn-ended`, `send` exits 0;
  - the harness prompt logs `origin:"harness"`, with no `wheel` record beyond start.
- (arch) **The turn's life:**
  - MET for harness, claimed-driver, the three end kinds, `activity`, and the `release` cases (driver-held, `budget:true`, string `from`, `pause`), via the unit tables.
  - For the human origin, AS AMENDED per directive (2): the turn is marked, and the next `send` reads `human-typing`. The original text is UNMET by construction (Spec claims disproved 2), and the amendment is its disposition.
- (arch) **`send` order:** MET (`send_refusal_order`: `control-character` → `human-typing`/`manual-pause` → `turn-running` → `input-not-ready` with a running turn). `answer` is never refused `turn-running`: `cli_answer` green unchanged.
- (security) **A refused `release` leaves the turn:** MET. A string `from` returns `-32602 release-from-driver` and the turn stays running (`release_from_a_driver` case). Human keys still reach the child (`path5_human_takes_the_wheel_and_release_returns_it` green).
- (security) **Nothing added:** MET. No event kind, detail, snapshot field, method, param, seam, env read, `#[ignore]` or deny exception (the three `git diff eb37914c0c59` probes green).
- (tests) **Release and recovery:** MET (`cli_wheel::path5_a_turn_left_running_is_cleared_by_pause_then_release`, 3 OSes).
- (tests) **The driver's own turn:** MET (`cli_send::send_after_a_confirmed_send_is_turn_running_until_turn_ended`, the exact human-mode line + hint). `path2_send_confirms_with_cl1_events` passes with its first turn ended.
- (tests) **Controls row:** MET (`setting_does_not_disable_the_send_turn_running_control`).
- (tests) **Renumber CARRY:** subject MET. No `` `:82` `` remains under `tests`/`src`, and both files cite `` `:84` ``. The `:82` probe's `exit 0` atom was a plan defect (Deviation 2); its run was corrected at this wrap (Deviation 6) and reads green.
- (obs) **The refusal line:** MET. `send-refused` carries `detail:"turn-running"`, `side:"wrapper"`, `wheel:"driver"`, `corr`, `conn`, `rpc_id`; asserted in `cli_wheel`, and CI G4 schema conformance is green on 3 OSes.
- (obs) **No prompt text in home lines:** MET (CI secret scan green; `assert_logs_clean` in `cli_wheel`).
- (design) / (a11y) **Text refusal:** MET. Human-mode refusal + one `hint:`, `--json` with no hint and no SGR (`cli_send`, `cli_wheel`).
- (tests) **Pre-push and CI:** MET. Pre-push `"ok":true` at `linux-tests`; ci#37241137053 `verdict: green` on the final HEAD `334ee7f`.

**Gates** (this wrap's light gate after its fold, run dir `2026-10-04T22-27-20-wrap`: `entries 20 · green 17 · red 0 · not-run 3`, the three operator legs re-verified from `evidence/operator-pass.md`):
- `cargo fmt --all --check`: green.
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: green.
- `run --unit --filter 'test(/run::wheel::tests::|run::send::tests::|cmd::run::tests::|human::tests::/)'`: green, 150 passed.
- `run --unit`: green, 1195.
- `run --integration --filter 'binary(cli_send) | … | binary(channel_endpoint)'`: green, 45.
- `run --integration --filter 'binary(tui_wheel) | … | binary(contract_fixture_hygiene)'`: green, 69.
- `run`: green (unit 1195 + integration 277). The first light-gate run read it red (the receipt-count race, Deviation 5), folded by `334ee7f`.
- `{ git grep -F -e '`:82`' -- tests src || true; } | wc -l` (the corrected run, Deviation 6): green, exit 0 · last line 0. The first light-gate run read the plan's original form `red · exit 0 ✗ (exit 1)` with `last line 0` ✓.
- `git grep -l -F -e '`:84`' -- tests/tui_wheel.rs tests/cli_answer.rs | wc -l`: green.
- The four smoke entries (cleanup, boot `--session p-turn-smoke --instance builder`, status, cleanup): green, ending `processes_gone` / `endpoint_gone` true.
- The three `git diff eb37914c0c59` probes: green.
- `bash scripts/agent-run.sh pre-push`: green, 0 corrupt-profile lines.
- `gate.py hygiene` (leg operator): the operator pass recorded `hygiene: clean` after removing phase P4's three raw gate listings.
- `git … push origin HEAD` (leg operator): pushed `93a5cbf`, then `334ee7f` (the latter with plain `git push origin HEAD`, the wrap's artifacts uncommitted and the source clean — `operator-pass.md`).
- `ci.py conclusion --sha HEAD --wait 1800` (leg operator): `verdict: green · checks 15/15` on `93a5cbf` (ci#37239689446) and on the final HEAD `334ee7f` (ci#37241137053).
- **Smoke:** cleanup → boot → status `ready` → cleanup, by hand at /implement P3, exact teardown.

**Watches:**
- `.profraw` merge refusal: RECURRED in implement pre-push run 1. First diagnostic: `warning: …/viola-673807-6045272833459289092_15.profraw: invalid instrumentation profile data (file header is corrupt)` → `error: no profile can be merged` (coverage 1472 passed / 2 failures `llvm-cov-exit-1`, `llvm-cov-summary-missing`).
- Cause measured: the fake agent's session-leader exit hung up an in-flight `viola hook`. Fixed in `src/bin/viola-fake-agent.rs`.
- After the fix: 4 green in a row (implement runs 2, 3, 4; operator pass run 5), each with 0 corrupt-profile lines. Two further green runs at this wrap (the operator pass before `334ee7f`, and the light gate), each with 0 corrupt-profile lines.
- Directive (3): CLOSED.

**Outcome basis:**
- The operator pass ran (Setup 4's pre-CI commit `93a5cbf`), then a second time after this wrap's light gate (the fix commit `334ee7f`). Gate verdicts rest on its final state: pre-push green on the fixed tree, ci#37241137053 green 15/15 on `334ee7f`, and the re-run light gate 17/17 (`evidence/operator-pass.md`, `evidence/light-gate-fix.md`).
- Implement's report, from this conversation, is the basis for the deviations, the controls (`evidence/controls.md`) and the WATCH record (`evidence/watch-profraw.md`).

**Process hygiene** (implement P4's census, re-measured here with `ps`):
- The harness session `p-turn-smoke` (wrapper + fake agent): started by this run, terminated.
- Gate, nextest and llvm-cov children: started by this run, terminated.
- Two `viola wait` processes and one `sh waitm.sh` belong to the `viola-lab` prototype. They were not started by this chunk, are not its own, and were left running.
