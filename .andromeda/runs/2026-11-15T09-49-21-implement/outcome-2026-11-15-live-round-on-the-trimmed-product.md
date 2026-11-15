# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (run 2026-11-15T09-49-21)

**Outcome: stuck. Entry 11's SECOND firing ran (start 8 of 9 spent) and reads RED as fired, on the status-line
reading alone. Its diagnosis stands: 2.1.325 raises the folder-trust dialog in a fresh cwd, and the rig passed it
(`main_trust=passed/1/1`) to the input box.** Stopped on the operator's word; entry 12 has no word.

Setup's approval line: `2026-11-15T09-46-06-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json` (plan rev
12). The operator's word for this run is snapped as inputs#I27.

## Step 3e — built and green
- `live-hold.py`: `fresh_proj` and `make_cwd` now make a fresh `proj-{stamp}/` per start and leave earlier ones as found.
  The new `rehearse-reuse` subcommand is entry 23.
- Block 1–8, 21, 22, 23: `green 11 · red 0`.
- Entry 23 `rehearse-reuse: green` reads `first_session_start=True · first_killed=True · wrapped_cwd_fresh=True ·
  wrapped_session_start=True · unwrapped_ready=True · proj_left_as_found=True · instance_dir_stands=True · census_held=0`
  (`evidence/rehearsal.md` § Step 3e).
- Entry 21 `stamp-check: pass`; entry 22 `screens-check: clean — 40 lines · 40 frames local · 0 deleted`.

## Entry 11, second firing — its reading (`evidence/operator-11-second.txt`, `evidence/live-round.md`)
- **Gate line:** `11 e2e red · exit 0 ✗ (exit 1) · exit 1 · 40.15s · history: unmoved`.
- **Closing line:** `live-unwrapped: red — viola_sha=a0e247fd3a77f744 · latest=2.1.325 · main_trust=passed/1/1 ·
  statusline=red · statusline_detail={"child_argv_read":true,"hits":0,"instance_settings_json":false,
  "settings_flag":false,"settings_unchanged":true,"statusline_read":"yes"} · main_stop=exit-0 ·
  main_screens=frames=5,clean=True,summary=5,closed=True,file=screens-live-unwrapped-main-20261115T095321Z.json ·
  census_held=0`.
- **The dialog (the diagnosis):** the folder-trust dialog. Its options are unnumbered rows opened by `❯`. The default
  `❯` stands on a `No, …` row; one down-arrow puts it on the trust literal's row, and the Enter there passes it to the
  input box. This is within the founder's pty word (≤ 2 down-arrows, 1 Enter, only on the trust literal).
- **Why red:** the status line read `hits: 0`. No descendant ran the user's `statusLine.command` in the 30 s poll after
  the input box stood. `~/.claude/settings.json` was unchanged. CARRY 3's unwrapped side thus reads 0 hits; whether
  2.1.325 runs the status-line command before a first turn is unread.
- **Starts:** 8 of 9 (`ledger: starts 8 · reds 3`).

## The frames' summary lines (closed words) and local paths
Local file (0600, the operator's to read and delete):
`target/live-round-screens/screens-live-unwrapped-main-20261115T095321Z.json`

| label | found | selected | rows | sha256 (16) |
|---|---|---|---|---|
| trust-seen | trust-dialog | null | 16 | 2ee46be915dad8ab |
| trust-selected | trust-dialog | null | 16 | 592f1a27ee71dde4 |
| trust-after-enter | input-box | null | 10 | b36bade43ddfe0ef |
| start | input-box | null | 10 | b36bade43ddfe0ef |
| final | input-box | null | 10 | e1ef636440c3eaee |

## Defect found, not fixed (the operator's word was to stop)
`live-read.py`'s `selected` counts only NUMBERED option rows (`OPTION`). 2.1.325's trust options are unnumbered, so
`selected` reads `null` where a `❯` stood. The fix is to count the rows of the dialog's option block that open with
`❯` or with the same indent, and to rehearse it against this form. `found` and the trust step itself are right.

## What step 3d needs, now that entry 11's screen stands
- The overseer's read of the local frame above.
- The trust step rehearsed against THIS form: unnumbered `❯` option rows with the default on a non-trust row, one
  down-arrow to the trust literal, Enter on a `selected` read. The fake agent's modal has no selector row, so this is a
  synthetic drawn stream or a fake-agent screen carrying such rows. The `selected` fix above rides the same rehearsal.
- The kept home's readiness: done (entry 23 green).
- The status-line reading: entry 12's wrapped side reads the same poll. With the unwrapped side at 0 hits, CARRY 3's
  «hits > 0 on each side» cannot read green as the criterion stands, whatever entry 12 reads. That is a criterion for
  the operator's word, not the rig's.

## Files changed (work dir only)
- `evidence/live-hold.py` — step 3e.
- `evidence/rehearsal.md` — § Step 3e.
- `evidence/live-round.md` — § Entry 11, second firing.
- `evidence/operator-11-second.txt` — new.
- `evidence/screens-summary.ndjson` — +20 rehearsal lines, +5 live.
- `evidence/live-sessions.ndjson` — rows appended by the rig.
- `target/live-round-screens/` — local frames.

## Deviations
None from the plan's steps. Scope read: `scope: clean — changed 0 · listed 0 · recorded 0`.

## Matrix
No ref. v2-02 and v2-03 stay gated and unread.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entries 4–7 and 23's rehearsal sessions | this run (gate entries) | terminated — `census_held=0` in each closing line |
| entry 11's unwrapped `claude` 2.1.325 | this run (operator entry 11) | terminated — Ctrl-C stop, `main_stop=exit-0`, `census_held=0` |
| 8 `claude` processes on the host (the overseers' and builders' own sessions) | not this run (NOT DERIVED) | left running — not this run's (`gate.py census`) |

## Decisions & corrections
- The operator's word: step 3e and its gates, then entry 11's second firing only, read in the foreground; entry 12 has
  no word (inputs#I27). Fired once, red, not re-fired.
- The parse check this time used `ast.parse`, not `py_compile`, so no bytecode lands (last run's hygiene refusal).
