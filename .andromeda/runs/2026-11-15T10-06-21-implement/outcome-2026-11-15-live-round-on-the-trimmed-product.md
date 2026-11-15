# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (run 2026-11-15T10-06-21)

**Outcome: stuck, and surfaced. Entry 12 (entry 10's re-fire, start 9 of 9, the last) ran its whole course and reads
RED as fired. It is never re-fired, and no start remains.**
- **What read as planned:**
  - the near-ceiling hold: `dialog-expired` from the wrapper, `deadline_hit=True`, `duration_ms=3600013`;
  - the send confirmed into an erased box;
  - the status row present, at the same place as the unwrapped side;
  - the name refusals and the killed wrapper.
- **What went red:**
  - the rig's `start` check: the instance's log carries entry 10's lines ahead of this start's, and the trust keys put
    a `wheel{human}` line before `session-start`;
  - the `x`'s `human=False`: the wheel was already `human` from the trust keys.
- **Surfaced:** two clauses of v2-03's acceptance cannot read as written on a start that types the trust keys (below).

Setup's approval line: `2026-11-15T09-56-08-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json` (plan rev
14). The operator's word is snapped as inputs#I31. The run-dir journal was written before the fire (`hold-journal.md`
in this run dir).

## Entry 12 — the readings (`evidence/operator-12.txt` holds the closing line whole; `evidence/live-round.md` § Entry 12)
- **Gate line:** `12 e2e red · exit 0 ✗ (exit 1) · exit 1 · 3650.0s · history: unmoved`.
- **Start readings:**
  - green: `session_start=True` · `child_cli_verified:true` · `child_pty_backend:"openpty"` ·
    `snapshot_cli_verified:true` · `budget_kind_lines:0` · `plugin_written_at_start` · `hooks_run_pinned_copy` ·
    socket dir 0700 / socket 0600 · child fds 0–2 on the pty, none on the endpoint or under the home ·
    `unable_bytes=0 · hint_bytes=0`;
  - red: `first_two_wheel_then_session_start:false` and `child_process_start:2`.
  - The instance's log, kinds only: lines 1–4 are entry 10's (the reused name `round`); this start's are 5
    `wheel{start, driver}` · 6 `wheel{human-input, human}` (1.3 s later: the trust keys) · 7 `session-start` ·
    8 `wheel{release, driver}`.
- **Trust keys typed:** `trust=passed · arrows=1 · enters=1`.
- **Name refusals:** `second_run=1/already-live` · `third_run=1/stale-heartbeat` · `heartbeat_live_again=True` ·
  `refusal_children=0`.
- **The box reading and the send's outcome, side by side:**

  | box reading (screen model) | send's outcome |
  |---|---|
  | `echo=True · human=False · box=erased · release=0 · driver=True · keys_after_del=0` | `send=0` · `send-issued / prompt-submitted / send-confirmed` · `box_witness=empty` |

- **The status row (CARRY 3, the wrapped side):** `status_row present · status_below_rule 1 · status_sha256 70f400ce…`.
  The unwrapped side (entry 25) reads `present · below-rule 1`, with the same hash. `hits: 0` stays a column.
  `settings_unchanged:true`, no `--settings`, no instance `settings.json`.
- **The hold:**
  - `question=True` (hook, 10:07:38Z), `keys_during_hold=0`;
  - `dialog_expired=True` (source `wrapper`, 11:07:38Z), `hold_s=3600`;
  - `deadline_hit=True · duration_ms=3600013`.
- **Killed wrapper:** `child_gone_after_kill=True`, `census_held=0`; `drained_bytes=1534698`.
- **Frames:** 11 in `target/live-round-screens/screens-live-wrapped-20261115T110739Z.json` (local). The summary is
  closed; the trust frames read `selected` 0 → 1. `screens-check: clean — 87 lines · 87 frames local`.

## Surfaced — a spec↔reality gap in matrix#v2-03's acceptance (nothing authored)
Two clauses meet this start's reality:
- **«`events.ndjson` lines 1-2 read `wheel{cause:start}` then `session-start`».** On 2.1.325 in a fresh cwd the
  folder-trust dialog stands before any hook. The founder's granted keys (`keys: trust yes`) pass through the wrapper,
  which writes `wheel{human-input}` before `session-start`. So a start that types the trust keys cannot read lines
  1–2 as written. Read on this start's own lines, the order holds: `wheel{start}` comes first, `session-start` follows
  after the trust keys' wheel line, and there is no budget step. A reused instance name also keeps the earlier start's
  lines at the head of the file: the rig reads that file from line 1, while this start's lines are 5–7.
- **«one human key `x` … with a `wheel` line holder `human`».** The wheel was already `human` from the trust keys, so
  the `x` raised no new line. `echo=True` and `box=erased` were read off the screen, and the release returned the
  wheel to `driver`. The clause's `wheel human` line exists, but from the trust Enter, not from the `x`.

Whether v2-03's acceptance is read on this start's own lines, with the trust keys' wheel line admitted, is the
operator's and the wrap's call. No ref is written.

## Matrix
No ref.
- **v2-02:** its live reading stands on record — `question`, no key during the hold, `dialog-expired` from the wrapper,
  `deadline_hit:true`, `duration_ms` 3600013. Its acceptance also asks the ceiling settled on the founder's `hold:`
  line, which is still `unanswered`, so it stays gated.
- **v2-03:** gated and surfaced as above.

## Files changed (work dir only)
- `evidence/operator-12.txt` — new.
- `evidence/live-round.md` — § Entry 12.
- `evidence/live-sessions.ndjson` — the `start` and `reading` rows, appended by the rig.
- `evidence/screens-summary.ndjson` — 11 lines.
- `evidence/hold-journal.md` — the rig's own journal at the send.
- `target/live-round-screens/` — the frame file, local only.

## Deviations
None. No code changed this run. Scope read: `scope: clean — changed 0 · listed 0 · recorded 0`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entry 12's `viola run round` wrapper and its `claude` 2.1.325 child | this run (operator entry 12) | terminated — the wrapper SIGKILLed by the rig, the child gone within 10 s; `census_held=0`; no process holds `viola-live-20261115T083805Z` (`gate.py census --comm claude` / `--comm viola`) |
| the second and third `viola run round`, `viola release`, `viola send` | this run | exited on their own |
| 8 `claude` and 6 `viola` processes on the host (other sessions' builders, overseers and their wrappers) | not this run (NOT DERIVED) | left running — not this run's |

## Decisions & corrections
- **The operator's word:** entry 12 only (start 9 of 9, the last, never re-fired), with the journal written before
  the wait (inputs#I31). Fired once, red, not re-fired.
- **The rig's journal at the send** still names «the gate log of entry 10» in its text: this firing is entry 12's. The
  run-dir journal names the right log. The rig was not edited after entry 24 read green.
