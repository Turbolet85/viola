# Outcome — 2026-11-19-the-2-1-328-recorded-fixture-set

**Outcome: stuck** — a RED live entry. `R4` (entry 9) read red on `lacks 'record: exists'`. The directive and plan step
10 stop implement before the landing and the operator pass; `R4M` and `L2` were not fired. The working tree is
preserved.

## Files changed
- `crates/viola-e2e/src/rig/mod.rs`:
  - `transcript_after_send` and `transcript_word` beside `session_transcript`;
  - `arm_cx` calls the helper once after the submit wait and writes `context-transcript-after`;
  - two inline cases.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — the cx rehearsal asserts one `context-transcript-after: absent` line.
- Not made: `fixtures/claude/2.1.328/` and `STAMPED` (steps 12–13 come after a green `R4` and `R4M`).

## Gates
- Entries 1–5 green (evidence/gates-pre-round.txt). Entry 3: 318/318.
- Live round, firing 1 at 13:41:25Z: `R4` was refused `token-window`. Uncounted, no ledger file.
- Live round, firing 2 at 14:06:59Z: entries 6–8 green; entry 9 red ·
  `lacks 'record: exists' ✗ · exit 0 · 45.18s → 9.2.log`;
  `round: STOPPED at 9 (red · lacks 'record: exists' ✗) · legs fired 1/1`.

## What stuck — the entry and what the start itself showed
- `R4` itself was clean on 2.1.328: `verify-exit: 0`, `pass-count: 24/24`, `failing: none`,
  `browser-tools-prompt: absent`, `screen-b`…`screen-f: box`, `stop: none`, `census: 0`, `credentials: removed`,
  `scratch: absent`, `record-left: absent`, `user-config: unchanged`.
- **`record: exists`**: `keep_record` (`crates/viola-e2e/src/rig/isolate.rs:753`) writes nothing when the kept dir
  stands. `target/live-rig/R4/record/` still holds the 2.1.325 chunk's kept `2.1.325/` (2026-11-18 20:40). The 2.1.328
  record was not kept, and the scratch was removed at teardown, so the recording is lost.
- `target/live-rig/R4M/record/` also holds `2.1.325/`, so `R4M` as planned (no spare) would meet the same rule.
- Cap: base starts spent 1 of 3 (`R4`, ledgered 2026-11-19T14:07:01Z); spare `R4-2` unspent. The spare's condition is
  met in kind: a cause `R4` itself showed, by a closed word.
- Its mend goes through a plan revision (directive, plan step 10). The candidates are for the phase, not taken here:
  - move the standing `target/live-rig/R4/` and `R4M/` kept dirs aside (gitignored residue of the 2.1.325 chunk,
    whose set already stands as `fixtures/claude/2.1.325/`);
  - or re-point entries 9 and 15 at `R4-2`'s kept dir, as step 10 names, together with `R4M`'s kept-dir collision.
  This run moves and deletes nothing.

## Deviations
- `cx-relook` control: on entry 3 as the plan names it, it read UNREPRODUCED. The package-scoped clone builds no
  `viola-fake-agent`, and four existing harness cases fail on ENOENT (testing.md 2026-11-13). Re-fired with the same
  edit and case on entry 4 (`run --unit`), it reads «holds»: the named case failed, 2787/2788 passed, no TIMEOUT
  (evidence/control-1.txt, control-2.txt).
- `transcript_after_send` carries the held transcript as `(PathBuf, Option<u64>)`, so a held transcript whose
  metadata failed stays held (as `unread`) and is never looked up again. The word mapping is shared through
  `transcript_word`.
- The round was fired leg by leg (`--entry 9`, plan step 11's form), so the host can be read before each base start.

## Process hygiene
| process | started by | final state |
|---|---|---|
| viola-live-rig (entries 7, 9) | a leg / round step | terminated — `census: none running` after `R4` |
| claude 2.1.328 (up to 9, one at a time, under R4's verify) | a leg | terminated — rig `census: 0`, `child: exit 0`; tool census none running |
| viola-mcp-tap | none started | — |
Control clones were removed by the tool. Trail: `.andromeda/runs/2026-11-19T10-57-42Z-implement/gate-2026-11-19-the-2-1-328-recorded-fixture-set.json`.

## Decisions & corrections
- Directive (inputs#I4): CONTENDED with a neighbour named → no start. Applied at 11:02:28Z, 13:33:01Z and 13:52:43Z.
  Started on QUIET at 13:41:15Z and 14:06:50Z.
- Directive and plan step 8: a token-window refusal is uncounted → stop and say so (13:41:25Z).
- Directive and plan step 10: a RED `R4` stops implement before the landing and the operator pass (14:07:46Z).
