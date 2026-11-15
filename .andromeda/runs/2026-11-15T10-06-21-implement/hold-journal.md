# Hold journal — entry 12 (entry 10's re-fire, start 9 of 9, the LAST) — run 2026-11-15T10-06-21-implement

Written BEFORE the fire. The operator's word is this run's `relay-1.md` (snapped as inputs#I31).

## State at the fire
- Chunk `2026-11-15-live-round-on-the-trimmed-product`, plan rev 14, approved in
  `.andromeda/runs/2026-11-15T09-56-08-phase` (its `matrix-…json`).
- Pre-fire reads (2026-11-15T10:06:22Z):
  - `latest: 2.1.325 · cli 2.1.325 · match`; `claude --version` by path: `2.1.325 (Claude Code)`;
  - `ledger: starts 8 · reds 3`;
  - `stamp-check: pass — stamped 2.1.325 · 21 of 21 relied pass · stamps sha256 86f763d8f43f5281`;
  - `/tmp` 21 G free; `target/debug/viola` sha256 `a0e247fd3a77f744…` (the plain build);
  - entries 1–8 and 21–25 last read `green 13 · red 0` (run `2026-11-15T10-00-02-implement`), the rigs unedited since.
- **The kept home:** `target/e2e-home/viola-live-20261115T083805Z/`. It holds `home/`, `priv/`, entry 10's `proj/`
  and entry 11's `proj-20261115T095240Z/`, all left as found; this start makes its own fresh `proj-{stamp}/`.
- **The fire:** `gate.py run --chunk 2026-11-15-live-round-on-the-trimmed-product --run-dir
  .andromeda/runs/2026-11-15T10-06-21-implement --operator 12`, backgrounded. It runs `live-hold.py live-wrapped`
  (timeout 4500 s), whose hold is the `config.json` `dialog_hold_s` 3600.
- **Expected duration:** about 3700 s from the fire, plus the stop. No clock time is predicted here; the stamp hook
  refuses predicted times. The fire's own UTC time is the gate line's and the rig's `start` row's `written_at`.

## What a resumed window re-reads (in this order)
1. This journal, and this run's `relay-1.md`.
2. The rig's own journal at the send: `.andromeda/cache/chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/hold-journal.md`
   (the home and instance, the send's UTC time, the expected expiry). The rig rewrites it at this firing's send.
3. The gate's output and log: `$TMPDIR/andromeda-gate/2026-11-15-live-round-on-the-trimmed-product/implement-2026-11-15T10-06-21/12.log`,
   and the trail `.andromeda/runs/2026-11-15T10-06-21-implement/gate-2026-11-15-live-round-on-the-trimmed-product.json`.
4. `live-sessions.ndjson` (the `start` and `reading` rows of `live-wrapped`, `written_at` after 10:06Z) and
   `python3 -I {evidence}/live-read.py ledger`, which should read `starts 9`.
5. The instance's `events.ndjson`: `target/e2e-home/viola-live-20261115T083805Z/home/instances/round/events.ndjson`.
   Read its kinds only: `question`, `dialog-expired`, `dialog-answered`, `wheel`, `send-*`.
6. `round-state.json`, and the frames at `target/live-round-screens/screens-live-wrapped-*.json` (local; summary
   lines in `evidence/screens-summary.ndjson`).
7. The census: `gate.py census --comm claude --args viola-live-20261115T083805Z` and `--comm viola`. If the gate call
   died with the window, the rig's wrapper and child may still run: stop them by pid (pid + start time against the
   snapshot), never by name.

Entry 12 is NEVER re-fired, whatever it reads. No start remains after it (9 of 9).
