# Outcome — 2026-11-17-first-readings-on-the-standing-rig

**Run:** 2026-11-17T17-01-20Z-implement · re-entry on plan rev 8 (revision 1) at step 12 · **Outcome: green** —
the block green on its first firing (9 non-leg entries), the `user-config-mark` and `ready-deadline` controls hold
(both through `gate.py control`), the `R3D` round COMPLETE (1/1 legs, no spare). Next → the OPERATOR PASS: entries
21 (`bash scripts/agent-run.sh pre-push`), 22 (`gate.py hygiene`), 23 (the push, the operator's), 24 (`ci.py
conclusion`), then `/andromeda-wrap-session`.

## Files changed this run (5, all in research's lists)
- `crates/viola-e2e/src/rig/isolate.rs` — step 12: `witness` (the closed word over the two stamps, the read and the
  needles), `holds` (the byte search), `read_moved` (one read through `take(MAX_FRAME + 1)`, a link followed only
  into the home); 7 unit cases.
- `crates/viola-e2e/src/rig/mod.rs` — step 12: the path needle (the canonical scratch root) made in `start`, the id
  needle from `run_child`'s return; `finish` takes the needles, prints `user-config: {word}` and `needles: {n}`, and a
  `changed-by-rig` makes the exit 1; `Session::session_id` and `Session::learn_id` (kept only when
  `viola_agent_claude::is_session_id` accepts it), learned by R2's first `session-start`; step 15: `R3D` → `Arm::R3`,
  the label case gains it.
- `crates/viola-e2e/src/rig/wake.rs` — step 12: the wake arms learn the id from their first `session-start`;
  step 14: R3 keeps the `left-idle` frame at the first reading change and puts `send-1-wakes`.
- `crates/viola-e2e/src/bin/viola-live-rig.rs` — the `live` doc line names `R3D`.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — step 13: the no-mark rewrite reads `changed-by-others` and
  `needles: 1`; new `rig_reads_its_own_scratch_path_in_the_user_config_as_changed_by_rig` (`changed-by-rig`,
  `needles: 1`, exit 1, no path in stdout); `r3b` and `r3` (unfired) read `needles: 2`, and `r3` `send-1-wakes: 0`.
- Evidence: `evidence/control-4.txt`, `control-5.txt`, `round-170712Z.txt`, `live-starts.md` (R3D section).
- Scope read: `scope: clean — changed 11 · listed 11 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 ·
  widening 0) · absorbed 0 · excluded 243` (trail `scope-2026-11-17-first-readings-on-the-standing-rig.json`).

## Gates (trail `gate-2026-11-17-first-readings-on-the-standing-rig.json`)
- Block: `entries 24 · green 9 · red 0 · recorded 0 · timeout 0 · not-run 15` (entries 1–8 and 20 green; legs
  not run by plain `run`). Entry 3: 48 tests run, 48 passed (the 7 witness cases among them). Entry 4:
  `nextest-integration` 14 passed, 0 failed (the new stand-in mark case among them). Entry 5: 26 passed, no TIMEOUT;
  the never-ready case **2.487 s**, the slowest case 6.168 s. Entry 6: 2.65 s. Entry 7: last line `3`. Entry 8:
  `release-check: viola only`. Entry 20: green, 4.01 s.
- Fix-loop: 0 iterations (first firing green).
- Controls: `user-config-mark` · entry 3 · **holds** (in place green, removed red exit 100; `evidence/control-4.txt`);
  `ready-deadline` · entry 20 · **holds** (in place green 28.68 s, removed red exit 100 23.79 s; the removed log
  holds 0 TIMEOUT lines and the case `FAIL [  20.332s]`; `evidence/control-5.txt`). The stopped run's hand-held
  `evidence/control-2.txt` stands as that firing's record.
- Live round (`--live-legs --entry 19`): `round: COMPLETE · legs fired 1/1 · entries 24 · green 3 · red 0 …`:
  17 `census: 0` · 18 key `d1fb7a4551aecb00` · 19 `R3D` green 111.94 s (`evidence/round-170712Z.txt`).
- No `gate.py guard` entry in the block. No deferral (Rust delta; every changed-surface gate ran).
- Entries 13–16 not re-fired (inputs#I6 item 4); entry 14's recorded red stands as its note disposes it.

## R3D readings (`evidence/live-starts.md`)
`ledgered: 2026-11-17T17:07:15Z` · `cli: 2.1.325` · `viola-build: d1fb7a4551aecb00` · `wake-form: dynamic` ·
`send-1-wakes: 1` · `wake: fired` · `wake-sign: capture` · `wake-origin: human` · `capture-source: absent` ·
`latency-ms: 1` · `screen-ms: unmeasured` · `user-config: changed-by-others` · `needles: 2` · census 0,
credentials removed, scratch absent · `keys: none`. The dynamic form set one wake (scheduled_wakes 1) and it fired
inside the stand; its UserPromptSubmit again carries no `source` key.

## Deviations from plan
- none in this run's steps. Steps 12–15 as written; the control file entries as the plan names them.
- `read_moved` takes the home as an argument (the caller passes `HOME`, an absent one read as an empty path that
  canonicalises to nothing → `unread` for a link), so the link case is unit-tested over a temp home.

## Process hygiene
| process | started by | final state |
|---|---|---|
| viola-live-rig `R3D` + its `viola run` / `claude` children | a leg (entry 19) | terminated — the start's teardown read `census: 0`; the round printed no survivors |
| rig census / plain build (entries 17, 18) | a leg (round steps) | terminated |
| rehearsal rigs, fake agents, `viola run` wrappers, nextest | the gate entries (this run) | terminated |
| control clones' builds and tests | `gate.py control` (this run) | terminated, both clones removed |
`gate.py census --comm viola --comm viola-live-rig --comm viola-fake-agent --comm claude --args viola-rig- --args
projects/viola/target` (trail `census-…json`): 15 running, none this run's — the host's overseer and builder
`claude` sessions, other trees' `viola` builds (`viola-lab/prototype`, `~/.local/viola-product`), and this
session's own `~/.viola/bin/0.1.0-…/viola mcp`; no row under `projects/viola/target` and none with `viola-rig-`.

## Decisions & corrections
- The operator's directive for this run (the invocation): re-enter at step 12; R3D alone, one start on 2.1.325
  with `--wake dynamic`, no spare, no key; the four spent starts not re-fired; a red that moves a plan entry stops
  before the operator pass — none occurred.
- Guard refusal: a `cd` into the skill's references dir and one into the evidence dir refused by the session-cwd
  guard; each re-issued with absolute paths.
- Guard refusal: a `cat` heredoc writing a scratch script refused (bash-guard, file-target heredoc); re-issued as a
  Write-tool script run by path.
- Reading for v2-36's owner: the fired dynamic wake's wrapper line read `origin: human` (as R3 and R3-326 did).
