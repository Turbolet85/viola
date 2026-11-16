# Outcome — 2026-11-16-send-confirmation (implement run 2026-11-16T04-56-13-implement)

**Outcome: green.** Smoke skipped: no boot-path change (`src/cmd/run.rs`, the plugin files and the start path
untouched); the booted wrapper cases in the block ran green. Next → the OPERATOR PASS (entries 14 `gate.py hygiene`,
15 the push, 16 `ci.py conclusion`), then `/andromeda-wrap-session`.

## Files changed (8, all on research's Files to modify)
- `crates/viola-agent-claude/src/hook.rs` — `SUBMIT_TRIM`, `is_submitted_form`, two rstest tables, one 512-case property
- `src/run/send/slot.rs` — `Flight::claim` (through `is_submitted_form`) and `Flight::session_started`; `append_hook_event`'s claim, turn and wheel moves under one flight guard
- `src/run/send.rs` — `send` → `send_through(…, window)`; post-gate reads, link, `send-issued` and paste under the flight guard; two race cases
- `src/run/send/relabel.rs` — the trimmed-prompt table and the leading-trim case; two doc comments reworded
- `src/run/send/test_support.rs` — `sent_through_the_post_gate_window`, `sent_then_submitted`
- `src/bin/viola-fake-agent.rs` — `--trim-prompt-end`
- `src/bin/viola-fake-agent/test_cases_opts.rs` — `opts_parse_reads_the_trim_prompt_end_flag`
- `tests/cli_send.rs` — `send_ending_in_whitespace_the_cli_trims_is_read_back_and_keeps_the_wheel`

## Gates (trail `.andromeda/runs/2026-11-16T04-56-13-implement/gate-2026-11-16-send-confirmation.json`)
`entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3` — entries 1-13 green on the first firing
(13 `bash scripts/agent-run.sh pre-push` 86.02 s); 14-16 `not run — leg operator`.
No `gate.py guard` entry in the block.

## Controls (`evidence/control-1.txt`, `--timeout 120`)
`control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` — trail-trim (entry 7),
locked-turn-read (entry 6), fake-trim (entry 9).

## Matrix
v2-18 and v2-19: `planned → implemented`, refs written through `matrix.py implement`.

## Deviations (with justifications)
- `sent_through_the_post_gate_window`'s window closure receives the slot and the instance dir (plan step 5: "receives
  the slot"): the race case appends through `append_hook_event`, which needs the instance dir.
- `relabel.rs`: two existing doc comments said "The claim has no tolerance"; reworded to "takes no longer text" so
  they stay true beside the trailing-run tolerance. The cases themselves are unchanged (plan step 6).
- Scope record: none — `gate.py scope` `clean — changed 8 · listed 8 · recorded 0`.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo / nextest / viola / viola-fake-agent children of entries 1-13 and the three controls | this run (gate tool) | terminated — no survivors line; `gate.py census --args projects/viola/target` lists none of this tree's binaries |
| the 15 processes `census` lists (overseer pair sessions, prototype/product `viola run`/`wait`/`mcp`) | NOT DERIVED — not this run's; other trees' binaries | left running: not this run's — the operator's |

## Decisions & corrections
- The operator's direction at invocation: no live `claude` start (cap 0), no mutation run, controls under
  `--timeout 120` on filtered entries, each long run journaled before its wait (`evidence/journal.md`), stop before the
  operator pass on a plan-moving red, else run the pass in this window and land `evidence/operator-pass.md`.
- Guard refusals: two Bash-guard refusals met, each cost its one re-issue (a `cd` into the skill's references dir →
  absolute paths; a `rec.py` call piped into `head` → `rec.py files --out` and a direct read).
