# Codebase Research — 2026-10-07-send-waits-out-the-paste-hint

## Scope
- **Depth:** deep · **Reads:** 27 · **Globs/Greps:** 19
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, 8 additions; applied: the
  fake agent's capped argv hold `--paste-hint-ms` (8 s at most today), the root fixture chain `home →
  fake_agent_path → stamped_home → booted_wrapper`, the 2026-10-07 stalled-start rule (a stalled start is a
  finding about the backing: stop and report, never re-run for green), the 2026-10-04 `binary(<file stem>)` form
  for a whole root test file. `.claude/rules/testing.md` — read whole, 32 additions; applied: 2026-09-25 "every
  new guard test carries its remove-the-guard run" with its 2026-10-04 extension (run the control under the
  nextest `mutants` profile when the neutralised guard can park on a fixed clock), 2026-09-27 "force a timing
  window open with a hold, red before and green after", 2026-09-28 with its 2026-10-05 extension (a kill line
  moves only for a by-design floor, with a planted-hang control recorded both ways), 2026-10-06 "size a new
  stamped-home case against the 10 s mutants kill, the stamp alone is about 4 s", 2026-10-04 "no mutation entry in
  a `[[gate]]` block", 2026-10-05 `vhome`, 2026-10-07 "a product command a live round starts runs with
  `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed and `--home` given".
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the take-up directive: the founder is away, a technical fork goes to the
  overseer, a new boundary widening is held, a red re-verify round is not retried, one builder window.
  `inputs#I2` — the overseer's answers to the four P4 forks.

## Files inspected
- `crates/viola-agent-claude/src/screen.rs` (1-160, 180-345, 386-447, 486-507) — `QUIET_PERIOD` `:10`,
  `GATE_MAX_WAIT` `:14` (5 s), their pin `:183-184`; `Screen::verdict` `:122-152`: poisoned refuses first
  (`:128-130`), not quiet waits to the bound (`:131-136`), no signatures is `Ready` (`:137-139`), a modal row
  refuses inside the row loop (`:141-144`), and the last arm (`:147-151`) is `Ready` with an input-box row and
  `InputNotReady` without one. The function is pure: `waiting_since` and `now` are parameters.
- `src/run/gate.rs` (full) — `Gate::wait_ready` `:158-180` re-reads the verdict every 20 ms, takes the model's
  lock inside the loop body and sleeps outside it; the span `run.readiness_gate` encloses the whole loop and
  records `outcome` and `vt100_panicked` once. Its cases `:520-629`.
- `src/run/send.rs` (150-480, 482-560, 696-736, 960-1016) — the `send` method `:296-353`: the paste check, then
  `human_typing()` at `:314`, then the in-flight and running-turn rung `:322-333`, then the gate at `:338` with
  `slot.clock.now()` as `waiting_since`, then `send-issued`, the paste and the confirmation window. Nothing
  re-reads the wheel or the turn after the gate returns. `Ctx::refuse` reads the end offset at the refusal
  (`:459-462`). `started` is taken before the gate (`:309`), so `send-confirmed.duration_ms` spans the gate's wait.
- `src/run/wheel.rs` (60-160) — `human_typing()` `:110-114` reads the holder under the wheel's own lock.
- `src/cmd/verify/typed.rs` (440-625, 728-800) — `settled` `:478-498` falls back at `from + GATE_MAX_WAIT` when no
  literal stands; `box_wait` `:505-523` has no such fallback and is bounded only by `PROBE_DEADLINE`
  (`src/cmd/verify.rs:37`, 120 s). Their cases `:737-800` carry the 5 s readings as literals.
- `crates/viola-agent-claude/src/ledger.rs` (725-772, 919-927, 2004-2030) — `check` for `QuietPeriod` `:759-762`;
  `verified` `:919` reads `data.versions[<version>].rows` and nothing else; the row's boundary cases `:2011-2016`
  carry 5000 and 5001 as literals.
- `src/bin/viola-fake-agent.rs` (30-34, 100-110, 436-493, 860-900) — `PASTE_HINT_CAP_MS` `:33`, read once at
  `:108`; the hold `:472-476` draws an empty screen and sleeps after the long text's Stop, then draws the `turn`
  screen. The agent reads its stdin again only after the hold. The `capped` case is at `:869`.
- `src/human.rs` (205-222, 361-427, 547) — `send_hint` `:209`; the `input-not-ready` string `:216`; pinned at
  `:371` and `:406`; `no_hint_names_release` at `:547`.
- `src/cmd/send.rs` (195-228) — the client calls `Client::request`, which has no deadline;
  `crates/viola-channel/src/client.rs` has a bounded form `request_until` (`:105`) that `send` does not use.
- `tests/cli_send.rs` (28-72, 118-150, 530-640, 677-700) — `boot_on`, `spawn_send` / `finish` (no test-side bound
  on the child), the forced-window pair `:551-554` with its 3 000 ms hold, and the trust-dialog refusal `:677`.
- `tests/cli_verify.rs` (827-829, 1040-1075) — the two `verify_window_` cases; the hint case holds 6 000 ms.
- `tests/tui_wheel.rs` (heads) and `tests/support/home.rs` (`Wrapper::send`, `:473`) — a key is written into the
  outer PTY with `wrapper.send(bytes)`; the wheel is read from `wheel` records.
- `tests/support/verify.rs` (300-350, 444), `tests/support/watch.rs:14` — `WITHIN` is 7 s; a verify-driven test
  has no test-side bound.
- `tests/contract_lints.rs` (205-262) — reads the `mutants` profile's own kill and its first override
  (`package(viola-e2e)`); it reads no `verify_window_` override.
- `.config/nextest.toml` (full) — the kill lines below.
- `crates/viola-e2e/src/harness/run.rs` (353-420) — `--local-live` runs PATH `claude` with no `--`; it has no
  by-path form.
- `viola-0.1.0/chunks/2026-10-06-local-command-and-paste-framing-rows/plan.md` (the gate fence) and its
  `evidence/record-round-green.md` — the by-path round's firing form and its recorded verdict line.
- `.andromeda/architecture.md:70` — [Human Takeover / Wheel], read whole.
- `.andromeda/playbook.md:36-50` — the two `verdict: escalate` patterns, both "Boundary widening".

## Graph impact (from the code-graph query; plane `rust`, trace `tree-query-2026-10-07-send-waits-out-the-paste-hint.json`)
- **`GATE_MAX_WAIT`** — 7 references outside its definition: `ledger.rs:16` (import), `ledger.rs:760`,
  `ledger.rs:761`, `screen.rs:132`, `screen.rs:184`, `typed.rs:28` (import), `typed.rs:487`. No other crate and no
  test target reads it by name; every other "5 s" is a literal in a case or a comment (the sweep below).
- **`Screen::verdict`** — one product caller, `Gate::wait_ready @ src/run/gate.rs:169`; eleven call sites in ten
  test functions of `screen.rs` (`:224`, `:241`, `:250`, `:263`, `:285`, `:295`, `:299`, `:325`, `:337`, `:393`,
  `:501`).
- **`Gate::wait_ready`** — one product caller, `send @ src/run/send.rs:338`; nine call sites in eight cases of
  `gate.rs`. The
  `wait_ready` rows under `crates/viola-e2e` and `tests/support/home.rs` are another function of the same name
  (the harness's boot readiness), not this one.
- **`settled`** (verify) — one product caller, `Run::settle @ typed.rs:599`; `box_wait` — `framing_turns @
  typed.rs:276`.
- **`PASTE_HINT_CAP_MS`** — one reference, `viola-fake-agent.rs:108`.
- **`send_hint`** — product callers `src/cmd/send.rs:149`, `:157`, `src/cmd/client.rs:154`; five cases in
  `human.rs`.
- **`CONFIRM_WINDOW_FALLBACK`** — unchanged by this chunk; `send.rs:178` and `:254`, `ledger.rs:763`.

## Patterns detected
- **The equality the remedy needs** (`screen.rs:122-152`): for a verified screen (`sigs` is `Some`), quiet for
  `QUIET_PERIOD`, with no modal row and no input-box row, the verdict must be `Wait` while `now - waiting_since <
  GATE_MAX_WAIT` and `Done(InputNotReady)` from the bound on. Today it is `Done(InputNotReady)` at once. The frame
  that produces the asserted quantity is the last arm; `waiting_since` already reaches it as a parameter, so
  `Gate::wait_ready` keeps its shape and its span.
- **The lock is per read** (`gate.rs:163-179`): the model's guard is dropped before the 20 ms sleep, so a wait of
  seconds stalls neither the feed thread nor the human's passthrough. Nothing here changes.
- **The wheel is read once** (`send.rs:314`, then `:338-345`): a human key during the gate's wait moves the wheel
  at once (architecture.md:70, "a read holding an editing key moves the wheel before it returns"), and the send,
  already past that rung, pastes when the gate reads `Ready`. Architecture records the same shape for a turn that
  starts during the wait as an accepted residual, "up to `GATE_MAX_WAIT`, 5 s". Before this build a verified
  literal-less screen ended the wait 0.63 s in; after it the wait can run to 8.5 s on every send that follows a
  long paste, so the same race has a window an order of magnitude longer.
- **A verified case with a fixed clock loops** (`gate.rs:562-567`): `wait_ready_verified_over_the_input_box_is_ready`
  reads a literal-less verified screen through `FixedClock(base + 300 ms)` and expects `InputNotReady`. Under the
  new arm that verdict is `Wait` forever on a fixed clock: the case must read at or past the bound.
- **Cases that turn by the arm** (verified, quiet, no literal, read at 300 ms): `screen.rs:271-272`
  (`input_box_absent`, `empty_screen`), `screen.rs:330-339` (`resize_starts_a_blank_screen`), `gate.rs:562-567`.
  The proptest at `screen.rs:499-502` reads a poisoned screen and stands.
- **Cases that turn by the constant**: `screen.rs:184` (the pin), `screen.rs:230-232` (5000 / 4999 / 6000),
  `gate.rs:579-587` (`from_secs(5)`), `typed.rs:748-756` (4999 / 5000 / 5100 / 5200), `typed.rs:776-785` (the
  `settled(.., 5000)` control), `ledger.rs:2012-2014` (5000 / 5001).
- **`send`'s unit cases are on the partial gate** (`send.rs:538-553`): `quiet_gate` and `poisoned_gate` start the
  gate with no signatures, whatever `cli_verified` the slot carries, so none of them turns by the arm.
- **The by-path round's firing form** (the 2026-10-06 plan's fence; its record `record-round-green.md`): a
  `role = 'probe'`, `leg = 'round'` entry running the 2.1.287 binary's `--version` by path, then a `leg = 'live'`
  entry `cargo build -q && target/debug/viola --home "$h/vhome" verify -- "$HOME/.local/share/mise/installs/claude/2.1.287/claude"`
  with a `stop` keyed on the probe dirs' cwd. The binary is present and executable at that path (read at P3). The
  2026-10-07 rule adds `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed. That round took 47.8 s and printed
  `stamped 2.1.287  17 pass  0 fail` as its last line.
- **A failing row still stamps** (arch history, 2026-09-28-capability-ledger-and-viola-verify): a red round writes
  a stamp with a `fail` row and exits 1. The round's home is its own, so a red round degrades no standing home.

## Conventions to follow
- **Boundary cases as labelled literals** (`screen.rs:228-232`, `ledger.rs:2011-2016`): the bound's cases name
  8500 and 8499 as literals; the pin at `screen.rs:184` reads `from_millis(8500)`.
- **Clock-driven waits at the unit layer** (`gate.rs:232-249`, `send.rs:496-530`): `FixedClock`, `SteppingClock`
  and `JumpClock` are in the tree; no unit case sleeps.
- **A forced window, never a sampled race** (`tests/cli_send.rs:544-554`): the fake agent's hold opens the window;
  the verdict is the exit code, the `--json` document, the receipt and `events.ndjson`. A stamped-home case with a
  3 000 ms hold read 7.4 s, under the `mutants` profile's 10 s kill.
- **A key into the outer PTY** (`tests/tui_wheel.rs:167`, `tests/support/home.rs:473`): `wrapper.send(b"h")`, the
  wheel read from the `wheel{holder:"human",cause:"human-input"}` record.
- **One human-stderr writer** (`src/human.rs`): a changed hint string changes at `:216` and its two pins, nowhere
  else.
- **The hint line, the lean P4 brings**: after this build the `input-not-ready` line is no longer printed for the
  paste-hint cause; the previous chunk's card said so under options 1 with 2. No amendment has ever changed a hint
  string of cli pattern 2 (design history), and a split by cause would be a new pattern beyond T4.

## The kill lines the moved bound meets (`.config/nextest.toml`, read whole)
- `verify_window_` on CI: 15 s × 3 = 45 s (`:25-27`); under `mutants`: 15 s × 2 = 30 s (`:56-58`). Both comments
  state the floor as "the gate's 5 s maximum four times, about 21 s". At 8.5 s the same four waits are 34 s, so
  `verify_window_without_screens_fails_every_interactive_row` (`tests/cli_verify.rs:829`) lands near 35 s: over the
  30 s `mutants` kill, and 10 s under the CI kill where it had 24 s.
- `binary(cli_send)` and the other verify-driven binaries on CI: 10 s × 2 = 20 s (`:31-33`); under `mutants` a
  case with no `send_window_` or `verify_window_` prefix runs under 5 s × 2 = 10 s (`:41`).
- `tests/contract_lints.rs:223` reads the profile's own kill and the `package(viola-e2e)` override only, so a
  moved `verify_window_` override does not reach it.
- testing.md 2026-09-28 (extended 2026-10-05) lets a kill line move only for a floor that is by design, with a
  planted-hang control recorded both ways. The four waits are by design (the comment at `:54-55`).

## Sweeps
- `GATE_MAX_WAIT` over `*.rs` outside `target/` (`grep -rn`): 10 hits · 2 changed (`screen.rs:14`, `:184`) · 8
  no-change (two imports, four reads by name, and the doc comments at `typed.rs:477` and `:502`, which name the
  constant and stay true).
- `gate.s 5 s` / `5 s maximum` / `21 s` over `tests/`, `src/`, `crates/`, `.config/` (`grep -rn`): 7 hits · 7
  changed (comments at `nextest.toml:23`, `:24`, `:54`, `:55`, `tests/support/verify.rs:444`,
  `tests/cli_verify.rs:827`, `:1052`). The doc comment at `tests/cli_send.rs:544-550` states the old reading in
  other words and changes with its case.
- `input-not-ready` over `tests/` and `crates/viola-e2e/` (`grep -rn`): 9 hits in 2 files · 3 changed (the `hint`
  case, `cli_send.rs:547`, `:591`, `:598`) · 6 no-change (`chaos_feed_panic.rs:5`, `:90`, `:101` read a poisoned
  screen; `cli_send.rs:677`, `:688`, `:700` read the trust dialog, a modal: both still refuse at once).
- `FAKE_AGENT_PUMP_DELAY_MS` over `*.rs`: 3 hits · 0 changed (`tests/tui_passthrough.rs:185` holds 1 000 ms; no
  case leans on its 5 000 ms cap equalling the gate's bound).
- `paste-hint-ms` / `PASTE_HINT_CAP_MS` over `*.rs`: 10 hits · 3 changed (`viola-fake-agent.rs:32`, `:33`,
  `tests/cli_verify.rs:1061`) · 7 no-change; the capped case's expected value at `viola-fake-agent.rs:869` changes
  with the cap. The leaves that name the cap (`.claude/rules/verification-harness.md`,
  `.claude/docs/tests-summary.md`, `.claude/docs/services/viola-agent-claude.md`) are the wrap's.

## New files to create
- `viola-0.1.0/chunks/2026-10-07-send-waits-out-the-paste-hint/evidence/` — the red-before and green-after readings, the remove-the-guard controls, the live-start ledger and the round's record

## Files to modify
- `crates/viola-agent-claude/src/screen.rs` — the verdict's last arm, the constant, its pin and the cases that turn
- `crates/viola-agent-claude/src/ledger.rs` — the quiet-period row's boundary cases
- `src/run/gate.rs` — the verified literal-less case and the never-quiet case
- `src/run/send.rs` — the wheel and the running turn read again after the gate's wait, with their cases
- `src/cmd/verify/typed.rs` — the two settle cases' readings
- `src/bin/viola-fake-agent.rs` — the hold's cap, its doc comment and its capped case
- `tests/cli_send.rs` — the hint case turned to delivered, and the keystroke case
- `tests/cli_verify.rs` — the hint case's hold past the new bound, and two comments
- `tests/support/verify.rs` — one comment
- `.config/nextest.toml` — the two verify-window overrides and their comments

## Open questions
- none. Three plan-decision questions stood at the end of research and were answered by the overseer in the P4
  round (`inputs#I2`): after the gate's wait `send` reads the wheel and the running turn again; both
  `verify_window_` overrides move in proportion (20 s × 3 on CI, 15 s × 3 under `mutants`) with a planted-hang
  control; the refusal at the bound is proved at the unit layer on the injected clock, and the fake agent's cap
  becomes 10 000 ms. The hint line is left as it is, so `src/human.rs` is not on the list above.
