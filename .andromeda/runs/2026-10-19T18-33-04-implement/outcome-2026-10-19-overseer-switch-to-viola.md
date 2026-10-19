# Implement outcome: 2026-10-19-overseer-switch-to-viola (steps 12–14 under the product; the block cleared)

**Outcome word:** green + smoke. Steps 1–14 are done. Readers 23, 24 and 25 are green, 25 through its void. v1-33
is `implemented`, with its gated clearance.

## Update after the operator pass (2026-10-19T19:16Z)
- Operator pass (`evidence/operator-pass.md`):
  - Entry 20 read `hygiene: clean`.
  - The pre-CI commit is `aa965127`. Entry 21 pushed it (`f23586fb..aa965127`).
  - Entry 22 read CI green, 15/15, run `ci#39305131020`.
- **Step 14** (`inputs#I18`, the overseer's relay `relay-switch-step14.md`):
  - The founder's own key wrote `wheel{human,human-input}` @26032.
  - The overseer's send was refused `human-typing` @26181. It exited 10 (the CLI log's `process-exit`), and no
    `send-issued` followed.
  - The founder's own `release` exited 0 and wrote `wheel{driver,release}` @26337.
  - The `takeover` line is in `evidence/switch-readings.ndjson`, and a takeover section is in
    `evidence/switch-round.md`.
- **Entry 25:** green, run as `--only 25 --void '25:block cleared — evidence/switch-readings.ndjson takeover line'`.
- **The matrix:** v1-33 went `planned` → `implemented` through `matrix.py implement --cleared-file
  .andromeda/runs/2026-10-19T18-33-04-implement/cleared-v1-33.md`.
  - The `from` gap and the `confirmed:true` reading below are carried into its clearance note. They are surfaced,
    not resolved.

The text below is the step-13 report, kept as written then.

Earlier runs of this chunk:
- `.andromeda/runs/2026-10-19T10-56-42-implement/outcome-2026-10-19-overseer-switch-to-viola.md`: Parts A and B,
  gate entries 1–19, and the code and fix-loop evolve checkpoints.
- `.andromeda/runs/2026-10-19T13-27-50-implement/outcome-2026-10-19-overseer-switch-to-viola.md`: steps 10–11
  under the product.

This run is the re-entry after the `/clear`. The re-entry send is `send-issued` @10260 → `prompt-submitted{driver}`
@10391 → `send-confirmed` @11069, 68 ms.

## Inputs snapped this run
- `inputs#I16`: the overseer's relay `~/dev/projects/additional/viola-overseer/relay-switch-step12.md`. It holds
  turn-waited, final-read and clear-confirmed, the host restart, the founder's 2.1.302 verify (20/24), his pin to
  2.1.296, his start cap of 15, and his own budget release.
- `inputs#I17`: the re-entry directive (`.andromeda/runs/2026-10-19T18-33-04-implement/relay-1.md`): record step 12,
  then go on to step 13.

## Files changed this run (evidence only; no source, test, fixture or manifest file)
- `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-readings.ndjson`: four lines appended.
  - `turn-waited`: `turn-ended` @5489, ending @7895, `background_tasks` 0. The overseer's `wait --after 4981`
    (timeout 3 000 000 ms) woke on it after 74 499 ms with outcome `turn-ended`.
  - `final-read`: two `last` request/response pairs, at 13:31:37.756Z and 13:31:52.368Z. Both read `result_class`
    ok in 0 ms.
  - `clear-confirmed`, in order:
    - @8046: the first try, `send-refused{budget-paused, seven-day}` at 99 %.
    - @9519: the founder's `budget-gate{paused:false, override_until 2026-10-23T14:00Z}`.
    - @9694: `send-issued` (6 B).
    - @9824: `session-end`.
    - @9938: `session-start{clear}` with a new id (`ba538116-…`).
    - @10127: `send-confirmed`. The diag line reads `confirmed:true`, 123 ms.
  - `restarts`: an extra line, outside the seven acts. It holds the host restart, start 8 on 2.1.302
    (`cli_verified` false), the 2.1.302 verify (20/4 and its four rows), start 8's end (child 143, run exit 0) and
    start 15 on 2.1.296 (`cli_verified` true).
- `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/evidence/switch-round.md` gains these sections:
  - O1–O6.
  - The act table.
  - The restart account.
  - The closed live-start ledger: **15 of 15**.
  - The CLI residual: `~/.claude/projects` 120 entries, 78 `*viola-verify*` files.
  - The panic count: **0** in each of the four role files.
- `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/inputs/`: I16 and I17, written by the inputs tool.

Earlier runs changed the six Part A files (research's Files to modify) and nothing else in source.

## Gates (this run dir, trail `gate-2026-10-19-overseer-switch-to-viola.json`)
- Entry 23 is green, last line 7: the seven acts. Entry 24 is green, last line 1: the 2.1.296 stamp line.
- Entries 1–19 were not re-run. They went green in the first run, and nothing under `src/`, `crates/`, `tests/`,
  `fixtures/`, `plugin/` or `Cargo.toml` / `Cargo.lock` changed after its gate trail (`find -newer` read empty).
  - The four walk-class tests (`contract_ledger_probes`, `contract_fixture_hygiene`, `contract_fake_agent_drift`,
    `cli_controls_not_disableable`) walk only `fixtures/`, `src/` and `crates/*/src`. This run touched none of them.
  - No gate deferral is opened.
- Entries 20–22 are the operator pass's: the hygiene check, the push and the CI read. This run fired none of them.
- Entry 25 is gated on step 14 and was not run.
- WATCH `send_budget_paused…` case_4: no nextest ran this run. The first run's three nextest runs read PASS.

## Matrix
v1-33 is the chunk's only capability. It is gated, so it gets no ref and stays `planned` until step 14 clears it
through `--cleared-file`.

## Deviations, with justifications
- **The live-start cap: 15, not the plan's 8.** The host restart spent start 8 on claude 2.1.302. The founder raised
  the cap twice, both relayed in `inputs#I16`: «Verify 2.1.302, +6 стартов» took it to 14, and «Закрепить 2.1.296,
  +1 старт» to 15. The builder made none of starts 7–15. It read them from the run log's `process-start` lines.
- **Step 12's clear needed a second try.** The product's budget gate refused the first `/clear`, as designed, at
  seven-day 99 %. The founder lifted the gate himself (`release --budget`). The confirmed `/clear` came after that.
- **`send-confirmed{confirmed:true}`:** this narrows the earlier gap. The `events.ndjson` line holds `{cursor}` only.
  The run diagnostics line holds `confirmed:true` for @465 and @9694. The acceptance's form is read there.
- **`final-read`'s exit code** is not in `inputs#I16`. Both channel responses read `result_class` ok.
- **`dialog-answered` with `from`:** carried from step 11. No `from` was sent, and none is logged.
- **The dialog-hold CARRY** stays owed: the hold was 27.3 s.
- **A correction to step 8's residual:** the fourth `~/.claude/projects` entry it could not attribute is
  `*-viola-ledger-probes-635187`, the 2.1.296 verify's own (by pid).

## Surfaced (author nothing; for the wrap)
- **The obs acceptance and the inputs rule conflict.** The obs criterion says "no prompt, skill or dialog text …
  anywhere under the chunk folder". The skill's inputs rule writes a relayed directive verbatim into
  `chunks/{marker}/inputs/`. `inputs#I17` is the copy of this re-entry's own send text (the 532 B prompt @10391), as
  are `I1` and `I15` from the earlier sends.
- **2.1.302 fails four ledger rows:** `tag-escaping`, `local-command-clear`, `paste-newline` and
  `local-command-compact`. This is a product finding with no owner yet (`inputs#I16`). The CLI pin to 2.1.296 is the
  overseer's own driving fact.
- **No `process-exit` line for the killed run:** the first product run (pid 767453) has no
  `process-exit{subject:"self"}` line. The host restart killed it.

## Process hygiene
`gate.py census` (comm `viola` / `viola-fake-agent`; args `projects/viola/target/{debug,release}/viola`,
`viola-product/bin/viola`): 10 running. None is under this repository's `target/`.

| process | started by | final state |
|---|---|---|
| (none) | this run | none started |
| `~/.viola/bin/0.1.0-231c0225dac912d9/viola mcp` (3105384) | the claude child of start 15 (the overseer's start) | left running: it is the builder session's plugin server, stopped with the session (the overseer / the founder) |
| `viola-product/bin/viola wait viola-builder --after 10260` (3116975) | the overseer | left running: it waits on this turn's `turn-ended` (the overseer's) |
| prototype `viola run` / `wait` for pulse, conductor, andromeda-worker and escher | NOT DERIVED (other trees' builders) | left running: not this repository's (the overseer pair's) |

## Next
1. The OPERATOR PASS: entry 20 (hygiene), entry 21 (the push) and entry 22 (the CI read).
2. Step 14: the founder's own key and his own `release`, at his keyboard; then entry 25 with `--void 25:block
   cleared — …`. If he is not back first, the wrap flips this record `gated`.
3. `/andromeda-wrap-session`.

## Decisions & corrections
- The founder's own words, relayed by the overseer (`inputs#I16`):
  - «Держать до смены аккаунта»: hold the `/clear` until the account changes.
  - «Verify 2.1.302, +6 стартов».
  - «Закрепить 2.1.296, +1 старт».
  - His own `viola release --budget viola-builder` at 18:31:54Z.
- The operator's directive (`inputs#I17`): re-enter at step 12, snap the relay, record step 12, go on to step 13.
- Guard refusal: the Bash guard refused one `cd` into the chunk folder ("moves the session cwd"). It was re-issued
  with absolute paths. This is a recurrence of the standing learning host-linux.md §Paths.
