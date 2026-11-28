# Outcome — 2026-11-28-console-scaffold (implement run 2026-11-28T07-53-14Z-implement)

**Outcome: green + smoke.** Next: the OPERATOR PASS — entries 21 (`bash scripts/agent-run.sh pre-push`), 22 (the
seeded hygiene, fired as written), the land and the pre-CI commit, 23 (the clean-tree push), 24 (the CI read, fired as
written) — then `/andromeda-wrap-session`.

## Files changed
- New: `src/cmd/console.rs` · `src/console/mod.rs` · `src/console/frame.rs` · `src/console/keys.rs` ·
  `src/human/console.rs` · `tests/tui_console.rs`
- Modified: `crates/viola-core/src/obs.rs` · `schemas/diag-line.v1.json` · `schemas/diag-detail.v1.json` ·
  `src/obs.rs` · `src/main.rs` · `src/cmd/mod.rs` · `src/human.rs` · `tests/cli_output_plain.rs` ·
  `tests/contract_error_bodies.rs` · `tests/cross_exit_causes.rs` · `tests/contract_diag_schema.rs` · `Cargo.toml` ·
  `crates/viola-agent-claude/src/ledger.rs` · `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` ·
  `src/cmd/verify.rs` · `src/cmd/verify/typed/turns.rs`

## Gates (block of 24, final firing 2026-11-28T08:15:07Z)
- 1–17 green but 14 `recorded` (hostwatch: `CONTENDED - 10 s stalled on IO … build tools alive in them: viola 10 s`,
  the window 08:00:19Z–08:15:19Z, this run's own builds); summary `entries 24 · green 16 · red 0 · recorded 1 ·
  timeout 0 · not-run 7`.
- The first firing (08:03:44Z) read entry 17 (`bash scripts/agent-run.sh run --e2e`) `red · exit 0 ✗ (exit 1)`: one
  failed assertion, `viola::contract_diag_schema diag_line_schema_event_enum_equals_the_log_format_literals` (its
  pinned 16-word event list against the schemas' 18). Fixed (the pin and diag-detail's event enum gain
  `console-frame`, `console-key`); `--only 17` green (17.2.log), then the whole block green.
- Probes as measured: board row `src/cmd/mod.rs:1` · `tests/cli_output_plain.rs:1`; `\b29\b` 27 → 3 (predicted 3);
  `run.paste_text(` 6 → 2 (predicted 2); `advisories ok`.
- Controls (`evidence/control-1.txt`): `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone
  removed` — quit-key and floor-cmp on entry 8, restore on entry 9.
- Smoke (`evidence/smoke.txt`): entries 18–20 green through `--operator` — boot `"ok":true`, status `state ready`,
  cleanup `processes_gone true · home_removed true`.
- No deferral: rust delta 20 files, every gate ran. No `gate.py guard` entry in the block.
- Verification matrix: no `chunk == 2026-11-28-console-scaffold` capability (coverage query, 0 rows): no ref written,
  as the plan states (no capability claimed).

## Deviations (with justifications)
1. `console_restores_the_terminal_on_quit` reads `ICANON` on the console's own terminal, opened through
   `/proc/<pid>/fd/0` with `O_NOCTTY`, not on the master's fd: `viola_pty`'s `PortablePty` / `Pty` expose no master fd
   (`crates/viola-pty/src/lib.rs:113-128`), and a seam change is out of the plan. The plan's «before the start» read
   is dropped: the console enters raw mode as it starts, so a read right after the spawn races it; the case reads raw
   while the frame stands and cooked after the quit, and the `restore` control holds on it.
2. `schemas/diag-detail.v1.json` gains the two events in its event enum as well as `console` in its process enum (the
   plan named the process word only): `tests/contract_diag_schema.rs` pins one list for both schemas' event enums.
3. The two refusal writers are named in `tests/contract_error_bodies.rs`'s `NOT_ERROR_BODIES` (stderr pairs, no
   document): its `CLI_DOCUMENTS` table holds machine bodies only, and the console takes no `--json`.
4. `tests/cross_exit_causes.rs`: the console's two causes enter `CAUSES` as `no-terminal` and `strict-modes-failed`
   (table labels; the console prints no code), with one piped case `console_off_a_terminal_is_refused_with_its_hint`;
   the widened-home cause is named in the header as held by `tests/tui_console.rs` (it needs a terminal to be reached).
5. `tests/contract_bounded_readers.rs` unchanged: its census reads the key reader (`input.read(&mut buf)` into a
   `[0u8; READ_CAP]`) as bounded — `run --e2e` green with it.
6. The terminal writes go through a new `src/human/console.rs` (enter, frame, floor line, restore), declared in
   `src/human.rs`: the plan's «the human module's writer» named no such writer.
7. `tests/cli_output_plain.rs`: `help_lists_only_served_verbs` gains `console` beside the `HELP` literal
   (testing.md 2026-11-22, extended 2026-11-24).
8. M1: the probed local command's paste goes through `paste_turn` too, its rows from `run.settle`; `address` is
   computed before the cross-session guard and a guard miss on the local paste reads `Kill` when an address stood,
   `CtrlC` otherwise — the original's two guards and their ends, unchanged. The echo and context latencies read the
   tag-only answer at the poll that answered the wait (`echo_answered`), where the original re-read both after it.
9. The console's `process-start` is its own (`subject`, `service_name`, `version`, `os`, `pid`), without
   `run::log_self_start`'s `endpoint_kind`: the console binds no endpoint.
10. M3: `crates/viola-e2e/src/harness/run.rs:1055`'s doc «all twenty-nine» left as written (outside the plan's sites).
11. Unit cases beyond the plan's two: the frame floor table, the key decoder table, the reader thread, the loop's quit,
    interrupt, unbound, below-floor, no-size, redraw and one-draw cases, the human writer's three, `home_refused`.
- Scope record (`gate.py scope`: `scope: clean — changed 22 · listed 18 · recorded 4 (companion 2 · mechanical 0 ·
  in-intent 2 · widening 0) · absorbed 3 · excluded 234`):
  - in-intent: `src/human.rs` (step 7, self) · `src/human/console.rs` (step 7, self)
  - companion: `crates/viola-agent-claude/src/ledger.rs` (step 10, self) · `tests/contract_diag_schema.rs` (serves
    `schemas/diag-line.v1.json`, self)

## Symbols / APIs
- `viola console` (`Command::Console`, `src/cmd/console.rs` `console`, `NO_TERMINAL`, `HOME_REFUSED`); `console::run`,
  `console::End`, `Screen` (guard), `keys::{Key, READ_CAP, spawn}`, `frame::{FLOOR, fits}`;
  `human::{write_console_enter, write_console_frame, write_console_too_small, write_console_restore}`;
  `ObsProcess::Console`; `ObsEvent::{ConsoleFrame, ConsoleKey}`; `ledger::RELIED_COUNT`; turns.rs private
  `paste_turn`, `Missed`, `Answered`, `echo_answered`.

## Schema / config
- `diag-line.v1.json`: `process` + `console`; events `console-frame` (`cols`, `rows`, `fits` required) and
  `console-key` (`key` ∈ `quit` · `interrupt` · `unbound`, required); description names `console.ndjson`.
  `diag-detail.v1.json`: `process` + `console`; event enum + the two words. No config key.

## Surfaced / stuck
- none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| the block's cargo / nextest / viola / fake-agent children | this run (gate.py) | terminated (no survivor line) |
| the control clone's builds and firings | this run (gate.py control) | terminated · clone removed |
| smoke session `console-smoke` (two wrappers, two fake children) | this run (entry 18) | terminated (entry 20: `processes_gone true`) |
| 3 `viola` processes the census lists (viola-lab prototype `run`, the pinned bridge's `mcp`, the product's `wait`) | NOT DERIVED — not this run's (none from this tree's `target/debug`) | left running: outside this repository — their owners' |

## Decisions & corrections
- The directive of this run (inputs#I6, the overseer, the operator away): no dialog; console tests under their own
  PTY and test home; no window, no real claude; a red with no failed assertion read against hostwatch first; the
  operator pass through the push and its CI read; `evidence/operator-pass.md`; end the turn with the pre-CI sha and
  CI verdict. Entry 17's first red HAD a failed assertion, so it was fixed, not read against hostwatch.
- Bash guard refusals, each re-issued once: a `cd` into the skill's references dir (the Read tool used); a `cat`
  heredoc to a scratch script (the Write tool used).
- Sweep hazard: a sweep of the role / event lists by their words (`"run", "hook", "mcp", "cli"`) does not reach the
  pinned event list in `tests/contract_diag_schema.rs` (`LOG_FORMAT_EVENTS: [&str; 16]`), which binds both schemas'
  event enums: found only by `run --e2e`.
