# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (plan rev 19, step 3g (b) and entry 27)

**Outcome: green.** Entry 27, the TENTH and last live start, read green on its one firing. It is never re-fired.
The operator's word is inputs#I47 and the founder's start is inputs#I39. No commit was made.

## Files changed
In `.andromeda/cache/chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/`, inside research's list:
- `live-hold.py`:
  - `live-trusted` now carries the one dummy probe `CLAUDE_CODE_REHEARSAL_PROBE` (step 3, revision 9).
  - The probe reading (name logged, value not) now lives in `trusted_session`, so the live and rehearsed arms read it
    alike and both gate green on it.
- `operator-27.txt` (new), `live-round.md` (§ Entry 27), `rehearsal.md` (the revision 9 lines).
- `live-sessions.ndjson`: the rig's `start` and `reading` rows for entry 27.

## Entry 26 — its line, re-fired on the revised environment line
`26 e2e green · exit 0 · 11.52s`:
`rehearse-trusted: green — viola_sha=f525a258df9c38b5 · absent_arm=trust record absent · false_arm=trust record absent · refused_made_nothing=True · granted_preflight=go · granted={"box":"stands","budget_kind_lines":0,"child_gone_after_kill":true,"child_process_start":1,"driver":true,"echo":true,"found":"input-box","hint_bytes":0,"human_before_x":false,"keys":"input-box+input-box","keys_before_box":0,"keys_before_x":0,"lines_1_2":true,"probe_name_logged":true,"probe_value_logged":false,"release":0,"screens":"frames=6,clean=True,summary=6,closed=True","session_start":true,"strip_names_only":true,"unable_bytes":0,"x_wheel_human":true} · probe_name_logged=True · probe_value_logged=False · names_only_controls=True · fresh_arm=name not fresh · modal_preflight=go · modal=trust-dialog/modal-standing/keys=none/green=False · census_held=0`

Entry 26's first green (run `2026-11-15T12-19-22-implement`) stood on a deviation: its probe was set against step 3's
old environment line (inputs#I45). This firing is on the revised line.

After it, entry 8 read `8 build green · exit 0 · 0.19s`, so the plain build `f525a258df9c38b5` was in place.

## Entry 27 — the tenth start, clause by clause
`27 e2e green · exit 0 · 7.0s · history: unmoved`

| clause | reading |
|---|---|
| trust record (read only, before the start) | `trust_record=true` |
| CLI | `latest=2.1.325` |
| no key before the input box | `found=input-box` · `keys_before_box=0`; no dialog or modal stood |
| `events.ndjson` lines 1–2 | `session_start=True` · `lines_1_2=True` (`wheel{cause:start}` then `session-start`) · `budget_kind_lines=0` |
| child start line | `child_process_start=1` · `strip_names_only=True` |
| the probe | `probe=CLAUDE_CODE_REHEARSAL_PROBE` · `probe_name_logged=True` · `probe_value_logged=False` |
| the `x`'s own wheel line | `human_before_x=False` · `keys_before_x=0` · `echo=True` · `x_wheel_human=True` |
| the DEL | `box=erased` (recorded) |
| release | `release=0` · `driver=True`; no send, no hold |
| keys typed in all | `input-box+input-box` |
| killed wrapper | `child_gone_after_kill=True` · `census_held=0` |
| terminal literals | `unable_bytes=0` · `hint_bytes=0` |
| frames | `frames=6,clean=True,summary=6,closed=True`, local only |

After the start:
- entry 22 read `screens-check: clean — 153 lines · 153 frames local · 0 deleted`;
- `live-read.py ledger` read `ledger: starts 10 · reds 4 · unread 0`.

## v2-03, by the plan's split
- **On entry 27, all green:** lines 1–2, the `x`'s own `human` wheel line, and names only in the strip fields (the
  probe's name is logged, its value is not).
- **On entry 12:** every other clause, read green there.

## Not done this run
- **v2-03's ref:** not written, because the word was "stop and report". v2-03 is gated, so its ref takes `matrix.py
  implement … --cleared-file` with the founder's words as the clearance, as v2-02's did.
- **Entries 28–30:** not fired. They are the operator pass: hygiene, the second pre-CI commit and push, the CI read.

## Deviations
Entry 26's first green stood on a deviation (above); the plan's ninth revision made the probe lawful.

`scope: clean — changed 1 · listed 1 · recorded 0`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entry 26's rehearsal wrappers, fake agent and stub | this run, through a gate entry | terminated: `census_held=0` |
| entry 27's `viola run trusted` and its `claude` child | this run, through `--operator 27` | terminated: SIGKILL of the wrapper, child gone by pid and start time, `census_held=0` |

`gate.py census` (comm viola-fake-agent · args `projects/viola/target/debug/viola`, `viola-live-20261115T083805Z`):
`census: none running`.

## Decisions & corrections
- The revision 9 probe, the founder's second override (inputs#I44): entry 26 is re-fired on the revised line, and its
  earlier green is named a deviation.

## Next
1. The operator's word on writing v2-03's gated ref.
2. Entries 28–30.
3. The halted wrap `2026-11-15T11-38-30-wrap` resumes.
