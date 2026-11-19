# P5 card 1 — «A live StopFailure payload» (2026-11-19-a-live-stopfailure-payload), plan rev 4

The plan to open in an editor: `.andromeda/cache/docs/2026-11-19-a-live-stopfailure-payload/plan.md` (its render).
No plan stands in a chunk's folder, and writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-19-a-live-stopfailure-payload`.

## Goal
Read one live StopFailure payload from claude 2.1.328 inside an unprivileged network-closed `bwrap` sandbox, through
the standing rig's new `SF` arm, without the start reaching Anthropic. The exception is settled at the wrap on that
reading (`inputs#I3` item 3): green on predicates 1-7 keeps it on the founder's word (B); red keeps it PROVISIONAL
with the red named. C5's kept-dir mend lands first.

## Top acceptance criteria
- C5: `keep_record` keeps a later version's set beside an earlier one's and refuses `exists` writing nothing for a
  standing version; the `keep-version` control holds; this lands before the live round.
- CI (unwrapped `SF` over the fake agent):
  - `server_error` reads `turn-end: stop-failure`, `cause: server-error`, `captured-word: server_error`,
    `cause-matches: yes`, `unknown-value-lines: 0`;
  - a needle word reads `api-error`, `other` and `1`, with the needles absent from everything kept (F1, `inputs#I4`).
- A contained start is refused `refused: sandbox`, exit 3, before its ledger line, where `bwrap` is missing.
- The wrapped local rehearsal reads `sandbox-probe: own` and `net-ns: own` on this host before the live start.
- Live `SF`, within the 1 + 1 cap: predicates 1-7 recorded as read; `turn-end: stop` is reported as «not contained»
  and the round stops; C3's three words are read after the turn end (F2, `inputs#I4`) as readings.
- Nothing is written under the real `~/.claude*` or `~/.viola` by the rig; the census after the last leg reads 0;
  unit, integration and E2E layers, fmt and clippy are green.

## Touchpoints (all test-only, no product crate)
- `crates/viola-e2e/src/rig/isolate.rs`, `mod.rs`, `capture.rs` and `screen.rs`
- `crates/viola-e2e/src/bin/viola-live-rig.rs`
- `crates/viola-e2e/tests/rig_rehearsal.rs`
- `src/bin/viola-fake-agent.rs` (`--turn-fail <variant>`)

## Test commands
22 entries:
- fmt and clippy;
- the rig's unit cases (baseline green, 318/318);
- `run --unit`;
- the rig rehearsals (baseline green, 27);
- the round: the wrapped local rehearsal, the install read, census, the build plus hash, the sandbox probe, the host
  read, the scratch base read, `SF`, census, `SF-2` (spare only), the operator census;
- `run --integration` and `run --e2e`;
- pre-push, hygiene, push and the CI read.

Two remove-the-guard controls at implement: `keep-version` and `sandbox-refusal`.

## Concretization preview
None. No capability is claimed (none pooled names StopFailure or the rig, and no `CARRY: proves`).

## The directive's item 12 (verbatim where a tool printed it)
The union's per-master lines and its `rows-union` line:
```
architecture: rows 967 · packs 10 · picked 162 (102 by every draw) · standing 17 · 50303 B
security-plan: rows 617 · packs 7 · picked 154 (102 by every draw) · standing 6 · 45414 B
design-system: rows 317 · packs 4 · picked 23 (16 by every draw) · standing 24 · 7804 B
layout-templates: rows 222 · packs 3 · picked 35 (23 by every draw) · standing 17 · 8066 B
test-plan: rows 1347 · packs 14 · picked 283 (164 by every draw) · standing 32 · 72694 B
obs-plan: rows 1028 · packs 11 · picked 314 (193 by every draw) · standing 21 · 73763 B
a11y-plan: rows 235 · packs 3 · picked 48 (23 by every draw) · standing 15 · 14650 B
rows-union: 7 master(s) · picked 1019 of 4733 · rule any
```

Not every cited row is in the union. 19 of 21 cited handles are in a rows file or a standing list. Two are not:
- `architecture · decision/byte-source-wheel-claim`;
- `security-plan · decision/live-rig-prompt-capture-closed`.
Both were looked up small by `rec.py find` and are cited by handle. The rows files were read by search (272 694 B,
past the 250 000 B bound).

`[RETYPED]` lines (no re-spawn ran; every batch passed first try):
```
tests-pick-p6 · [RETYPED] line 49: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 49
tests-pick-p6.d3 · [RETYPED] line 81: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 81
```

Non-zero exits, refusals and parse lines:
- `rec.py query` on `input_row` with a guessed column: `query: Binder Error: Referenced column "chunk" not found in
  FROM clause!`, exit 2; re-read by `describe`.
- The Bash guard refused a `cd` twice (re-issued once each) and one pipe into `tail` (re-issued bare).
- A bounded-repetition grep over the 373 MB CLI bundle: `timeout: failed to run command 'command': No such file or
  directory`, then `Terminated` (exit 143); read by a scratchpad byte-window script instead.
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed by any tool this phase.

Where a letter was wrong or silent:
- The handoff's Position Next said the kept-dir keying had landed; it had not (the operator's correction, inputs#I2).
- My P3 block shaped two things it did not flag:
  - C3's keys before the send, which the wheel row refuses;
  - the error-word capture, a value reading.
  Both cost the founder's P4 round (F1, F2, inputs#I4).
- The directive and the playbook do not say how a wrapped rehearsal's printed skip is read by a gate entry; the plan
  reads it by `--no-capture` and a `lacks sandbox-probe: unavailable` atom.
- The bundle's retry delay could not be read from the minified code: the turn-end delay is a reading `SF` records,
  bounded by `SF_TURN_BOUND` 480 s under `--deadline 900`.

## Beyond the plan
none
