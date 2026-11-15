# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (plan rev 18, step 3g)

**Outcome: green, stopped before entry 27.** The operator's word for this run is inputs#I43. Entry 27 is a live
operator entry with no word yet, so it was not fired. No live start was made, and `live-sessions.ndjson` gained no
`start` row.

## Files changed
All in `.andromeda/cache/chunks/2026-11-15-live-round-on-the-trimmed-product/evidence/`, inside research's list:
- `founder-words.md`: `cap: 9 starts` became `cap: 10 starts` (the founder's own word, inputs#I39).
- `live-hold.py` gains the following:
  - `live-trusted` (entry 27);
  - `rehearse-trusted` (entry 26);
  - `trust-record`, a no-start read of the CLI's trust record for entry 27's cwd;
  - the helpers `trust_record`, `trusted_preflight`, `names_only`, `trusted_session` and `bounded`;
  - an optional `name` argument on `events` and `await_kind`. It defaults to `round`, so the earlier subcommands are
    unchanged.
- `live-read.py`: `SUBS` and `FRAME_FILE` now admit the `live-trusted` and `rehearse-trusted` frames.
- `rehearsal.md`: a «Revision 8» section with the readings below.

## Gates
- **Entries 1–8, 21–25:** `entries 30 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 17`.
- **Entry 26:** `26 e2e green · exit 0 · 11.49s`, on its first firing. Its closing line, whole:
  `rehearse-trusted: green — viola_sha=f525a258df9c38b5 · absent_arm=trust record absent · false_arm=trust record absent · refused_made_nothing=True · granted_preflight=go · granted={"box":"stands","budget_kind_lines":0,"child_gone_after_kill":true,"child_process_start":1,"driver":true,"echo":true,"found":"input-box","hint_bytes":0,"human_before_x":false,"keys":"input-box+input-box","keys_before_box":0,"keys_before_x":0,"lines_1_2":true,"release":0,"screens":"frames=6,clean=True,summary=6,closed=True","session_start":true,"strip_names_only":true,"unable_bytes":0,"x_wheel_human":true} · probe_name_logged=True · probe_value_logged=False · names_only_controls=True · fresh_arm=name not fresh · modal_preflight=go · modal=trust-dialog/modal-standing/keys=none/green=False · census_held=0`
- **Entries 22 and 8, after 26:**
  - entry 22 read `screens-check: clean — 138 lines · 138 frames local · 0 deleted`;
  - entry 8 was re-fired last, so the plain build `f525a258df9c38b5` is in place for entry 27.
- **The trust record for `proj-20261115T095240Z`** (`live-hold.py trust-record`): `trust-record: true — cwd proj-20261115T095240Z`.
  It was one read-only read of `~/.claude.json`, capped at 16 MiB, and nothing was written there.
- **Matrix:** no write. v2-03 is gated and its ref waits on entry 27's reading. The v2-02 ref stands from run
  `2026-11-15T11-19-59-implement`.
- **Smoke:** skipped, because no boot path or UI surface changed (the edits are rigs in the evidence dir).

## Deviations
1. **`trust-record` subcommand:** the plan doesn't name it. I added it to report the record for entry 27's cwd without a
   start, as the invocation asks (inputs#I43). It reuses the same read as `live-trusted`'s preflight.
2. **A `CLAUDE*` probe in `rehearse-trusted`'s granted arm:** a name in its environment (`CLAUDE_CODE_REHEARSAL_PROBE`)
   makes the strip-field reading non-vacuous. Its name is logged and its value is not
   (`probe_name_logged=True · probe_value_logged=False`). The live arm runs on the declared environment, which has no
   `CLAUDE*` name.
3. **The DEL's box reading** is recorded, not gated. The fake agent keeps a DEL in its prompt (`box: stands`,
   inputs#I4); the live arm records the box beside the other codes.

The scope read is clean: `scope: clean — changed 1 · listed 1 · recorded 0`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| rehearsal wrappers, fake agents, the trust-form stub (entries 5, 6, 23, 24, 26) | this run, through a gate entry | terminated: each rig's `census_held=0`, and the census lists none of them |
| host sessions: 4 overseers, 4 prototype builder wrappers with their `claude` children, the product's `viola mcp` and `viola wait` | NOT DERIVED, not this run | left running, not this run's |

`gate.py census` (comm viola · claude · viola-fake-agent, args `projects/viola/target/debug/viola`) reads `census: 14
running`, and none of those processes is this repository's build or a rehearsal base.

## Decisions & corrections
- The P5 approval of plan rev 18 is the delegate overseer's technical approval (inputs#I42). It rests on the founder's
  one-time override and his word for the tenth start (inputs#I39).

## Next
Entry 27 (`live-hold.py live-trusted`, LIVE, start 10 of 10) runs on its own word only. After it, the operator pass runs
entries 28–30 (hygiene, the second pre-CI commit and push, the CI read), then the halted wrap
`2026-11-15T11-38-30-wrap` resumes.
