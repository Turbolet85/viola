## After implement
- **Report:** 2026-11-17T17-01-20Z-implement — the newest implement run of 2026-11-17-first-readings-on-the-standing-rig
  (`rec.py runs … --file '*-2026-11-17-first-readings-on-the-standing-rig.json'`, its last run line), its outcome `green`
- **Commits:** since last_wrap 2026-11-17T14:02:45Z: `88e310ba feat(2026-11-17-supply-chain-green-and-booted-tests-end): …`
  (the prior chunk's wrap commit) and `0e1373d7 chore(2026-11-17-first-readings-on-the-standing-rig): operator pre-CI
  commit, for the run this chunk's verdict reads`. Base: 88e310ba, the parent of the oldest pre-CI commit 0e1373d7.
- **Operator pass:** run by the implement session on the operator's word (`evidence/operator-pass.md`). Entry 21
  (`bash scripts/agent-run.sh pre-push`) green · exit 0 · 79.95 s · `history: unmoved` · coverage 3486 passed, 0
  failed, `breaches: []` (`evidence/operator-21.txt`); entry 22 (`gate.py hygiene`) `hygiene: clean — read 262 …`;
  `rec.py land` tx 2417 and the pre-CI commit 0e1373d7; entry 23 (the push) green · `history moved:
  refs/remotes/origin/build/viola-0.2.0 88e310ba→0e1373d7` (`evidence/operator-22-23.txt`); entry 24 (`ci.py
  conclusion --sha HEAD --wait 1800`): `0e1373d73fe8 verdict: green · checks 7/7 · wall 520 s · runs ci#44134805079
  completed/success`. The pass's end: as-left — no commit after the pre-CI one, no correction or ruling during it.
- **Changes after implement:** none — no commit after 0e1373d7 and no directed edit of the wrap's.
- **Inputs:** `inputs.py verify`: `inputs: 8 entries — unchanged 5 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 3 · uncited 1 · unparsed 0`. I1 · ../additional/viola-overseer/phase-first-readings-directive.md ·
  no-repo · unchanged; I2 · …/relay-first-readings-p3.md · no-repo · unchanged; I3 · …/relay-first-readings-p5-review.md
  · no-repo · unchanged; I4 · message (operator, own word) · n/a; I5 · message (the operator, /andromeda-phase
  invocation) · n/a; I6 · …/relay-first-readings-revision1.md · no-repo · unchanged; I7 · message (the operator, own
  word) · n/a; **inputs#I8** · ../additional/viola-overseer/wrap-first-readings-directive.md · no-repo · unchanged —
  the overseer's wrap directive, snapped `--step wrap`, cited here (printed `UNCITED` before this close existed). What
  it changes: the two widenings (the rig's own UserPromptSubmit capture, inputs#I2 item 3; the user-config mark
  witness, inputs#I6 item 1) land as standing rows on ONE answer card on the founder's words already given, asking
  nothing new; the stop's key is restated (below); three ledger notes (v2-35, v2-19, v2-32) land at P7.3; route-resolve
  asks the overseer in one dialog (R4's re-pin, v2-36's fate, the carries discharged).
- **Scope:** none — `scope: clean — changed 11 · listed 11 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 ·
  widening 0) · absorbed 0 · excluded 247` (trail `scope-2026-11-17-first-readings-on-the-standing-rig.json`).
- **Sweep reach:** `cascade.py sweep --pre-pass` over `report-patterns.toml`, baseline 0e1373d7 (trail
  `cascade-2026-11-17-first-readings-on-the-standing-rig-pre-pass.json`). First firing refused, exit 3: `never-ready`
  and `rig-units` controls UNFIRED over the seven masters — that refusal is the basis of their master zero; re-fired
  with the other eight.
  - Counts: `ci` filter binaries 25 → 28 · `ci-line` · standing 5 (test-plan 5): dec-ic4quyea
    (decision/ci-stamped-home-kill-line) and dec-enalpnf3 (decision/verify-driven-tests-20s-kill) stale — the 20 s
    line now holds three viola-e2e binaries; dec-fvoytlxf, dec-gk7uv4y5, dec-emfw6k27 a true claim sharing the token.
    Owner master: test-plan.
  - Counts: the never-ready case 20.644 s → 2.487 s · `never-ready` · master 0 (refused control, the basis above).
  - Counts: viola-e2e `rig::` unit cases 29 → 48 · `rig-units` · master 0 (refused control); a test count is no
    master's (CLAUDE.md, 2026-10-16).
  - Counts: `rig_rehearsal` 9 → 14 · `rehearsal` · standing 1 (test-plan 1): dec-k4rtddnt
    (decision/standing-live-rig) names the binary as its guard — no count stated, true.
  - Spec claim disproved: a fired wake's `source` · `wake-source` · standing 2 (architecture 2): ctr-z5xuu6kt,
    ctr-z4sahxy3 — the event line's own `source` field, a true claim sharing the token. No master states the bundle's
    `loop_wakeup` / `schedule_wakeup` words: the disproved claim stood in the plan alone; its owner is the route
    (v2-36's entry, P5).
  - Spec claim disproved: the stamp-only user-config witness · `user-config` · standing 2 (security-plan 2):
    dec-75ttngyv (decision/live-rig-isolated-child) stale — the witness is now the mark witness; dec-q2hyz4q4 (verify
    writes nothing into ~/.claude.json) true. Owner master: security-plan.
  - Expected: test-plan §harness ~ contract/harness-boot/timeout · `boot-deadline` · standing 14 (test-plan 14):
    ctr-eo3m22cw (contract/harness-boot/timeout) add-to — `BootOptions::ready_deadline`; ctr-byjdhveq
    (readiness-failure) and the other 12 harness-boot rows a true claim sharing the token. Owner: test-plan.
  - Expected: standing-live-rig (wake and imports arms, capture verb, mark witness, `needles`, `R3D`) · `live-rig` ·
    standing 3 (architecture 2 · test-plan 1): dec-k4rtddnt stale (add-to); arch dec-ue4ubtxp, nam-dxznt5js true.
    Owner: test-plan.
  - Expected: the rig's own UserPromptSubmit capture · `rig-capture` · standing 12 (architecture 6 · test-plan 5 ·
    obs-plan 1): every row the product hook's or the fake agent's — a true claim sharing the token; no row holds the
    rig's capture: a new fact. Owner: security-plan (rul-bznresq5, the answer card).
  - Expected: live-round-rig-keys-own-pty (a no-key group-kill close) · `rig-keys` · standing 11 (architecture 2 ·
    security-plan 5 · test-plan 3 · a11y-plan 1): dec-wpnve6qm (decision/live-round-rig-keys-own-pty) add-to; the
    other 10 (verify's and the product's no-key rules) true. Owner: security-plan.
  - Expected: matrix#v2-35 · matrix#v2-19 · matrix#v2-32 notes — no master site: ledger notes, owner P7.3.
  - Expected: route adaptation on «A fired scheduled wake keeps the wheel» — `wake-source` above; owner P5.
  - Sweep reach: 8 patterns · 17 moved facts, 14 with a pattern (the report's own count: 4 Counts, 2 Spec claims
    disproved, 11 Expected amendments; the three matrix notes carry none) · by line 0 lines · 0 offsets
- **Outcome, final:**
  - supersedes «(live) `R3D` fired once, ledgered 17:07:15Z, no key: …»: `R3D` fired once, ledgered 17:07:15Z, with
    no key into any dialog or box text (`keys: none` — the argv grant: arrows, Enter, `x`, DEL); its stop was the
    rig's standard stop, Ctrl-C into its OWN pty at an idle box (`stop: ctrl-c 4`), under security-plan ·
    decision/live-round-rig-keys-own-pty («Ctrl-C is its stop's alone») — the rest of the criterion as written. MET.
  - The stop of each start, as recorded in `evidence/live-starts.ndjson`: `R3B` `stop: ctrl-c 5` · `R3` `stop: ctrl-c
    5` · `R3-326` `stop: ctrl-c 5` · `R3D` `stop: ctrl-c 4` · `IM` alone `stop: killed-no-key` (a group kill). Wherever
    implement's sections say «no key» of a wake start (`R3B`, `R3`, `R3-326`, `R3D`), read it as: no key into any
    dialog or box text; the stop was Ctrl-C (inputs#I8 item 2).
  - Live starts: FIVE base starts (`R3B`, `R3`, `R3-326`, `IM`, then `R3D` on the founder's word of 16:50:18Z,
    inputs#I6 item 2), ZERO spares. `R3`'s `user-config: changed` stands as read, with its measured cause
    (`evidence/user-config-writer.md`), not re-worded.
  - The operator pass's gates: `bash scripts/agent-run.sh pre-push` green · exit 0; `gate.py hygiene` clean; the push
    green; `ci.py conclusion` — ci#44134805079 on 0e1373d7 · green · 7/7 (`evidence/operator-pass.md`).
  - Watches: none folded. Outcome basis: Setup 4's commit list (0e1373d7 alone past the base), the final HEAD's CI run
    ci#44134805079 recorded in `evidence/operator-pass.md`, and implement's report for what only it holds;
    implement's Outcome otherwise stands.
- **Decisions & corrections after implement:**
  - The overseer's wrap directive (inputs#I8): the stop's key said exactly — `keys: none` names the argv grant, never
    the stop; whether security-plan · decision/live-round-rig-keys-own-pty's «anew for each chunk» binds the stop is
    asked of the founder by the overseer, the row changed only on the answer relayed at the card.
  - inputs#I8: the two widenings are RATIFIED by the founder's own words (inputs#I2 item 3, read 16:05:02Z; inputs#I6
    item 1, read 16:50:18Z) and land on ONE answer card that cites them and asks nothing new; the no-key group-kill
    stop of the imports arm is a change inside the key rule's bound (it types less, never more).
  - inputs#I8: v2-36's entry is not withdrawn or re-worded by the wrap; its fate is asked of the overseer at
    route-resolve.
  - inputs#I8, curation candidates (technical, judged by the filters): a witness that compares a whole shared file
    cannot attribute a change on a host with other writers; a one-shot control is fired on an entry whose build
    reaches every binary its case needs.
- **Process hygiene, re-measured:** `gate.py census --comm viola --comm viola-live-rig --comm viola-fake-agent --args
  viola-rig- --args projects/viola/target` (this wrap's run dir): `census: 7 running` — none this chunk's: the host's
  `viola-lab/prototype` builds (`wait`, four `run` wrappers), `~/.local/viola-product/bin/viola wait`, and this
  session's `~/.viola/bin/0.1.0-…/viola mcp`; no row under `projects/viola/target` and none with `viola-rig-`. Left (a
  directory, no process): `~/.cache/viola-ctl-ready-deadline-20261117T163750Z`, queued for the founder's desk.
