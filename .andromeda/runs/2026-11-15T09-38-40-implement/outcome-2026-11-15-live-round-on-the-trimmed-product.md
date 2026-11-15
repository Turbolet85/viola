# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (run 2026-11-15T09-38-40)

**Outcome: stuck. Entry 11 fired once and read RED in 0.11 s, before any start; it is not re-fired. No start was
spent: starts stay 7 of 9. No screen was kept, because no session started.** Step 3c is built and green. Entry 12 has
no word.

Setup's approval line, quoted: `2026-11-15T09-30-45-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json · open · 5057 B`
(plan rev 10). The operator's word for this run is snapped as inputs#I24.

## Step 3c — built and green (`evidence/rehearsal.md` § Step 3c)
- **`live-hold.py`:** `save_screens` writes each session's frames, scrubbed, only under `target/live-round-screens/`
  (dir 0700, files 0600, gitignored). It appends one closed-word line per frame to `evidence/screens-summary.ndjson`.
  The modal arm goes through it too.
- **`live-read.py`:** gains `found` · `selected` · `summarise` · `line_fault` and `screens-check [<summary> [<work dir>]]`.
- **Block 1–8:** `green 8 · red 0`.
  - Entry 5: `screens=frames=9,clean=True,summary=9,closed=True · modal_arm=modal-refused-no/13/unread/frames=3,clean=True,summary=3,closed=True`,
    hold `hold_s=60 · deadline_hit=True · duration_ms=60000`.
  - Entry 6: each arm `…,closed=True`.
- **Entry 21:** `stamp-check: pass — stamped 2.1.325 · 21 of 21 relied pass · stamps sha256 86f763d8f43f5281 · start rows 5 · latest 2.1.325`.
- **Entry 22:** `screens-check: clean — 20 lines · 20 frames local · 0 deleted`.
- **One-shot controls:**
  - must-pass: `clean`, exit 0;
  - one `found` set to a screen-text row: `red — open-found`, exit 1;
  - a planted `screens-*.json`: `red — frame-file-landed`, exit 1.

## Entry 11 — RED as fired, no start spent (`evidence/operator-11.txt`, `evidence/live-round.md`)
- **Pre-fire:** `latest: 2.1.325 · cli 2.1.325 · match` · `ledger: starts 7` · state stamp `pass` for 2.1.325 · `/tmp`
  21 G free · plain build `a0e247fd3a77f744` · the host QUIET (the operator's word).
- **Gate line:** `11 e2e red · exit 0 ✗ (exit 1) · exit 1 · 0.11s · 1842 B → 11.log · history: unmoved`.
- **Cause, read from the log:** `FileExistsError` in `live-hold.py` `make_cwd`. On `trust yes` it makes `<base>/proj`
  with `exist_ok=False`, and the kept home's base `target/e2e-home/viola-live-20261115T083805Z/` still holds entry 10's
  `proj/`. It raised before the ledger `start` row and before any spawn. A rig fault, not the CLI's. The rehearsals
  miss it by construction: each makes a fresh base, and none runs a second live use of a kept home.
- **Its reading:** none. No session started, so:
  - no frame, so no summary line for `live-unwrapped` (the summary holds only the 20 rehearsal lines);
  - no diagnosis of the 2.1.325 fresh-cwd dialog;
  - no unwrapped status-line reading.
- **Starts:** 7 of 9, unchanged. `live-sessions.ndjson` holds no row of this firing.

## The frames' local paths (for the overseer)
No live frame exists. `target/live-round-screens/` holds only the five rehearsal files (the fake agent's screens):
`screens-rehearse-wrapped-20261115T094221Z.json` · `screens-rehearse-wrapped-modal-20261115T094222Z.json` ·
`screens-rehearse-unwrapped-main-20261115T094225Z.json` · `screens-rehearse-unwrapped-trust-granted-20261115T094229Z.json` ·
`screens-rehearse-unwrapped-trust-refused-20261115T094229Z.json`. They are the operator's to read and to delete.

## What a re-fire of entry 11 needs (the operator's word; nothing changed after the red)
1. **A rig fix for the kept home's cwd:** each live start makes its own fresh `proj-{stamp}/` under the kept base, never
   the fixed `proj/`, so entry 10's `proj/` stays as found and no second use collides. Entry 12's `live-wrapped`
   calls the same `make_cwd` on the same base, so as the rig stands it would raise the same way.
2. **A rehearsal that reaches the fault:** a rehearsed second live-form start on a base that already holds a `proj/`
   (and, for entry 12, the instance `round` entry 10's SIGKILL left), reading green before any live word. Step 3d
   already names the instance half for entry 12; the `proj/` half is new.
3. **The word to re-fire entry 11.** It read red as fired and «a red live step is never re-fired» binds it; the one
   bend the founder gave is entry 10's (inputs#I21). Its re-fire is a new word, and so is how it fits the cap: 7 spent,
   so entries 11 and 12 still fit within 9 only if this firing counts no start, which the ledger shows.

## What step 3d's rehearsal needs (unchanged by this red, all after entry 11's screen)
- Entry 11's kept frames: the summary lines (`found`, `selected`, `rows`) and the local frame files, which the overseer
  reads. None exist yet.
- That screen's form drawn under the fake agent or as a synthetic stream: the dialog rows entry 11 shows. `pass_trust`
  must then read `passed` within ≤ 2 down-arrows and 1 Enter, only on the trust literal and only on a `selected` read.
  If `found` reads `unknown-modal` at `trust-seen`, start 9 is not spent.
- The kept home's re-fire readiness: entry 10's `proj/` (item 1 above) and its SIGKILLed instance `round`.

## Files changed (work dir only)
- `evidence/live-hold.py`, `evidence/live-read.py` — step 3c.
- `evidence/screens-summary.ndjson` — new, 20 rehearsal lines in closed words.
- `evidence/rehearsal.md` — § Step 3c.
- `evidence/live-round.md` — § Entry 11.
- `evidence/operator-11.txt` — new.
- `target/live-round-screens/` — 5 rehearsal frame files, local only.

## Deviations
- None from the plan's steps. Scope read: `scope: clean — changed 0 · listed 0 · recorded 0`.
- One choice the plan left open: `found` reads a numbered selector dialog as `question` at a post-send label and as
  `unknown-modal` before a send.
- An addition beyond the plan: the rehearsal's summary lines land in `evidence/screens-summary.ndjson` too, marked
  `"rehearse": true`. They are closed words like the live ones, and entry 22 reads them.

## Matrix
No ref. v2-02 and v2-03 stay gated and unread.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entries 4–7's rehearsal sessions (fake agent, wrappers) | this run (gate entries) | terminated — `census_held=0` in each closing line |
| entry 11's `live-unwrapped` | this run (operator entry 11) | none started — raised before any spawn |
| `claude` processes on the host (8: the overseers' and builders' own sessions) | not this run (NOT DERIVED) | left running — not this run's; none holds `viola-live-20261115T083805Z` (`gate.py census`) |

## Decisions & corrections
- The operator's word: step 3c, then entry 11 only, no re-fire of a red step, stop after entry 11; entry 12 has no
  word (inputs#I24). Entry 11 fired once, red, not re-fired.
- Hygiene's first read refused 2 files (P2, binary): the `evidence/__pycache__/` bytecode that this run's own
  `python3 -m py_compile` check wrote at 09:40Z. Removed (this run's own files), hygiene re-read `clean`, then landed
  (`2 new · 5 changed · 0 gone · 35 kept`, tx 1422).
- A rig fault the fake rehearsals cannot reach by construction (a fresh base each), met on the first second use of
  the kept home.
