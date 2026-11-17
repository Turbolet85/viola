# Outcome — 2026-11-17-first-readings-on-the-standing-rig

**Run:** 2026-11-17T16-26-19Z-implement · **Outcome: surfaced** — every fake-agent entry green, all four controls
hold, the founder's round fired whole (4 base starts, 0 spares); entry 14 (`R3`) read red on its
`contains 'user-config: unchanged'` atom, whose cause is a host writer outside the rig. Fixing it moves the plan's
live entries and the rig's isolation witness, so per the operator's directive this run stops BEFORE the operator
pass (entries 17–20 not fired). No chunk report is written (a surfaced run's report is the wrap's).

## Files changed (11, all in research's lists)
- New: `crates/viola-e2e/src/rig/capture.rs` (step 4), `crates/viola-e2e/src/rig/wake.rs` (step 6)
- Modified: `crates/viola-e2e/src/harness/boot.rs` (step 1: `ready_deadline` on `BootOptions`, `INSTANCE_DEADLINE`
  made `pub`), `crates/viola-e2e/src/harness/run/perf.rs`, `crates/viola-e2e/tests/cli.rs` (the field at their
  literals), `crates/viola-e2e/tests/harness_lifecycle.rs` (step 2), `.config/nextest.toml` (step 3),
  `crates/viola-e2e/src/rig/isolate.rs` (step 5), `crates/viola-e2e/src/rig/mod.rs` (steps 6–7),
  `crates/viola-e2e/src/bin/viola-live-rig.rs` (step 8), `crates/viola-e2e/tests/rig_rehearsal.rs` (step 9)
- Evidence: `evidence/holds-text.md` (step 10), `live-starts.ndjson` / `live-starts.md` (step 11), `round-*.txt`,
  `control-1..3.txt`, `user-config-writer.md`
- Scope read: `scope: clean — changed 11 · listed 11 · recorded 0` (trail `scope-…json`).

## Gates (trail `gate-2026-11-17-first-readings-on-the-standing-rig.json`)
- Block, final run: entries 1–8 green (`entries 20 · green 8 · red 0 · … · not-run 12`). Entry 5 (the three
  viola-e2e binaries under `ci`, 20 s line): 26 passed, no TIMEOUT; the never-ready case **2.430 s / 2.415 s**
  (was 20.644 s), the slowest case 5.674 s. Entry 6: 2.55 s. Entry 7: `3`.
- Fix-loop: 1 iteration — entry 4's `r3b` rehearsal asserted `send-n-wakes: null` after the plan's prose («the fake
  records no `session_crons`»); the fake replays `Stop.default` with `session_crons: []`, a count of 0. Oracle
  corrected to `0` (`wake-request: none` holds as planned).
- Controls: `capture-keys` holds · `capture-source` holds · `wake-capture` holds (tool clone); `ready-deadline`
  read UNREPRODUCED in the tool's clone (entry 6 is package-scoped: no `viola-fake-agent` built — testing.md
  2026-11-13) and **holds** fired by hand over a scratch clone that built the workspace first (in place PASS
  2.303 s; removed FAIL 20.340 s on the wall assertion, no TIMEOUT). `evidence/control-1..3.txt`.
- Live round: R3B 13 green · R3 14 **red** · R3-326 15 green · IM 16 green (`evidence/live-starts.md`).
- No `gate.py guard` entry in the block. No deferral (Rust delta; every changed-surface gate ran).

## Readings (closed words; `evidence/live-starts.md`)
- **R3b («a real wake reads 1»):** `send-1-wakes: 1`, `send-2-wakes: 1`, `wake-request: interval` on 2.1.325.
- **R3a (a fired wake's `source`):** a fired `/loop` wake (scheduled through CronCreate; the CLI's scheduled-task
  marker counted on the kept frame) reaches UserPromptSubmit with **no `source` key** — `capture-source: absent` —
  on 2.1.325 and on 2.1.326. Per the scope, this reading bears on «A fired scheduled wake keeps the wheel» (v2-36);
  its withdrawal is the wrap's route adaptation on the founder's word.
- **R3c (hook latency):** the wrapper's `prompt-submitted` stood 3 ms after the capture hook's stamp, on both CLIs.
- **IM:** first frame `modal: imports`, `box: down`; closed by a group kill, `keys: none`.
- **R2's `holds-text`:** the CLI's placeholder hint (`placeholder-hint`), from the kept frames, no start.
- 2.1.326 runs the wrapper's `--plugin-dir` hooks (`session-start: seen`).

## Surfaced — spec ↔ reality gap
- **Value/contract:** every live entry (13–16) asserts `contains user-config: unchanged`; the rig's witness is a
  whole-file stamp (sha + mtime) of `~/.claude.json` (security-plan · decision/live-rig-isolated-child; test-plan ·
  decision/standing-live-rig).
- **Reality:** the host's own Claude sessions rewrite `~/.claude.json` about once a minute — measured with no rig
  alive (16:40:20.72Z, 16:41:20.70Z; `evidence/user-config-writer.md`). A start that stands past one write reads
  `changed` whatever the rig does: R3 (39 s) did; R3B, R3-326 and IM happened to fall between writes.
- **Impl reality:** the rig cannot tell its own write from the host's with this witness. Every other isolation
  witness read clean on R3 (census 0, credentials removed, scratch absent; the child on the private config).
- **Needs:** the operator's / founder's ruling on the witness (e.g. a quiet host for the round, or a witness that
  separates the rig's write) and the plan entries' atom — a plan entry moves, so this run stops before the
  operator pass. R3's readings stand; no spare was spent (R3's shape was right).

## Deviations from plan
- `crates/viola-e2e/src/harness/session.rs` and `crates/viola-e2e/src/bin/viola-harness.rs` (listed to modify)
  needed no edit: they build `BootOptions` through `for_session` / `for_cli`, which set the field.
- `INSTANCE_DEADLINE` made `pub` so the literals outside `boot.rs` name it (no environment read, flag or seam).
- The rehearsals keep the file's 18 s `WITHIN` (step 9 names 12 s; the file's constant is 18 s).
- The `r3` capture rehearsal runs under `--deadline 10` and ends by the watchdog (its fired turn never ends on the
  fake), so it asserts the closed words and the teardown, not exit 0.
- `capture --out` writes `{"unread":true,"oversize":true,"ms":n}` for an over-long input (step 4 names
  `oversize` without its line form).
- The `ready-deadline` control's `holds` comes from a hand-fired scratch clone, not `gate.py control` (above).

## Process hygiene
| process | started by | final state |
|---|---|---|
| viola-live-rig + its `viola run` / `claude` children (R3B, R3, R3-326, IM) | a leg | terminated — each start's teardown read `census: 0`; `viola-live-rig census` reads 0 |
| rehearsal rigs, fake agents, `viola run` wrappers | the gate entries (this run) | terminated — `gate.py census` (trail `census-…json`): none running |
| control clones' builds/tests | `gate.py control` (this run) | terminated, clones removed |
| hand control clone `~/.cache/viola-ctl-ready-deadline-20261117T163750Z` | this run | **left**: a directory, no process; recursive rm refused by the host's permission rule — the operator removes it at the desk |

## Decisions & corrections
- The operator's directive for this run: «A red that moves a plan entry stops you BEFORE the operator pass» —
  applied: the round finished (no spare spent), the operator pass not run.
- Guard refusal: `rm -rf` of the hand clone refused by the host's deny rule (bash-guard.py); left, named above.
- Guard refusal: a `cd` into the skill's references dir refused by the session-cwd guard; re-issued with absolute
  paths.
- testing.md 2026-11-13 (a package-scoped control entry builds no root binary) recurred: the plan named entry 6.
