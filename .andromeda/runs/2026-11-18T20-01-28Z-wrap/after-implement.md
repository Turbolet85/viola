## After implement
- **Report:** 2026-11-18T19-33-40Z-implement — the newest implement run of 2026-11-18-the-2-1-325-recorded-fixture-set
  (`rec.py runs --kind '*-implement' --file '*-{marker}.json'`, its last run line), its outcome `green`
  (`outcome-2026-11-18-the-2-1-325-recorded-fixture-set.md`: «Outcome: green (no fix iteration; smoke skipped — no
  boot-path change)»).
- **Commits:** since `last_wrap` 2026-11-18T16:41:27Z (the `state` view): `bb204f14 feat(2026-11-18-browser-tools-prompt-modal):
  the CLI's browser-tools prompt read as a compiled modal, the 24th relied ledger row` (the prior chunk's wrap commit) ·
  `06f5f6e5 chore(2026-11-18-the-2-1-325-recorded-fixture-set): operator pre-CI commit, for the run this chunk's verdict
  reads`. Basis: `git log --since=2026-11-18T16:41:27Z`; the pass's commits from the parent `bb204f14`: `06f5f6e5` alone.
- **Operator pass:** run in the implement window on the user's own instruction (evidence/operator-pass.md). Entry 21
  `bash scripts/agent-run.sh pre-push` green · exit 0 · 139.2 s (coverage 3649 passed, 0 failed; breaches none) ·
  entry 22 `gate.py hygiene` `clean — read 232 … 0 host paths kept`, then `rec.py land` (tx 2869) · the pre-CI commit
  `06f5f6e5` · entry 23 the push `bb204f14..06f5f6e5 build/viola-0.2.0 -> build/viola-0.2.0` green · entry 24
  `ci.py conclusion --sha HEAD --wait 1800`: `06f5f6e5cca4 verdict: green · checks 7/7 · wall 555 s · runs
  ci#44384569763 completed/success` (fuzz-replay · lint · msrv · perf · release · supply-chain · test). The pass's end:
  as-left — no commit after the pre-CI one; the final HEAD is `06f5f6e5`.
- **Changes after implement:** none — no commit after the pre-CI one; no directed edit of the wrap's.
- **Inputs:** `inputs.py verify` (this run dir): `I1 · ../additional/viola-overseer/phase-fixture-set-directive.md · copy ·
  unchanged` · `I2 · ../additional/viola-overseer/relay-fixture-set-p3.md · copy · unchanged` · `I3 ·
  ../additional/viola-overseer/implement-fixture-set-directive.md · copy · unchanged` · `I4 ·
  ../additional/viola-overseer/wrap-fixture-set-directive.md · copy · unchanged` — snapped at this wrap (`--step wrap`),
  cited here, `inputs#I4`: the overseer's wrap directive (the operator away; forks to the overseer as a dialog; the
  answer card citing the founder's own answers of 2026-11-18T19:16:46Z, `inputs#I2` answers 1–4; the two new route
  entries written on his placement, `inputs#I2` answer 1; the two departures below kept in the report's own words).
  Summary: `inputs: 4 entries — unchanged 4 · drifted 0 · vanished 0 · broken 0 · … · uncited 1` (I4, before this
  close cited it). No DRIFT.
- **Scope:** none appended — `gate.py scope`: `clean — changed 33 · listed 32 · recorded 1 (companion 0 · mechanical 0
  · in-intent 1 · widening 0) · absorbed 28 · excluded 229`; implement's one in-intent line
  (`crates/viola-e2e/tests/rig_rehearsal.rs`, self) stands.
- **Sweep reach:** the site sweep `cascade.py sweep --pre-pass` (trail `cascade-2026-11-18-the-2-1-325-recorded-fixture-set-pre-pass.json`,
  baseline `06f5f6e5`), patterns `{run_dir}/report-patterns.toml`:
  - `stamped` (STAMPED `[&str; 2]` → `[&str; 3]`; the stamped sets every 24-row probe reads 2 → 3): standing 1 —
    test-plan 1 (`decision/ledger-probe-contract-suite`: no count in its rule, a true claim sharing the token —
    no-change; its `verified` evidence quotes the retired `[&str; 2]` line, a record of a past read, not a row claim);
    leaf 1 (`.claude/docs/tests-summary.md:21` «2.1.287 and 2.1.302 stamped» — stale, the cascade's).
  - `recsets` (the recorded sets under `fixtures/claude/` 4 → 5): standing 7 — security-plan 1 · test-plan 6. test-plan's
    6 (`decision/every-recorded-set-replayed`-class rows dec-li2rn6b7, dec-kj6lptge, dec-hc4i3vva, dec-gc4pdlwl,
    dec-ngqgaouf, `contract` ctr-gpfaitx3) state no count — no-change. security-plan 1
    (`decision/unverified-cli-gate`, its `except:` «StopFailure … owned by the 2.1.325 recorded fixture set») —
    STALE: the founder moved StopFailure to its own entry «A live StopFailure payload» (`inputs#I2` answer 3, «Своя
    запись»); the exception stays PROVISIONAL as it stands, only its owner moves — a P2 fact. Leaves 7, the cascade's
    (`.claude/rules/security.md:19` the same owner; `.claude/docs/services/viola-mcp.md:27,36` naming 2.1.302's exchange
    alone).
  - `riglabels` (the rig's live labels gain `R4` / `R4-2` / `R4M`, arm words `vr` / `mt`, the kept record dir): standing
    3 — architecture 2 (dec-ue4ubtxp, a true list of test-only binaries — no-change; `name/binary/viola-live-rig`
    nam-dxznt5js — the Expected amendment's site, add-to) · test-plan 1 (`decision/standing-live-rig`, true —
    no-change). Owner of the Expected amendment: architecture (a hit in it).
  - `recordhome` (the rig's half of the record-home rule): standing 1 — test-plan 1
    (`decision/record-home-names-no-home` dec-ioj2wnmy — the Expected amendment's site, add-to). Owner: test-plan.
  - `mcptap` (the tap's live start as the rig's `R4M` under its private config): standing 7 — architecture 2
    (dec-ue4ubtxp, nam-mlsyxxct — true, no-change) · security-plan 1 (`decision/mcp-fixtures-via-test-only-tap`
    dec-pvrejrf6 — the Expected amendment's site, add-to) · test-plan 4 (dec-d5bfbqzd, dec-dpnz2git, dec-k7mvueq4,
    dec-kaqjszck — true, no-change). Owner: security-plan.
  - Spec claims disproved: none (implement's) — no pattern.
  Sweep reach: 5 patterns · 6 moved facts, 6 with a pattern (the report's own count: the two `STAMPED` deltas share
  `stamped`) · by line 0 lines · 0 offsets.
- **Outcome, final:** supersedes «`ci.py conclusion` on the pushed HEAD — the operator pass's (entry 24), not yet read»:
  read — `06f5f6e5` green 7/7, ci#44384569763. supersedes «operator entries (pre-push · hygiene · push · `ci.py
  conclusion`): `not run — leg operator` here»: entries 21–24 green (Operator pass, above). Every other line of
  implement's Outcome stands. Watches: none folded. Outcome basis: Setup 4's commit list (`06f5f6e5` the only pass
  commit), the final HEAD's CI run ci#44384569763 in evidence/operator-pass.md, implement's report and outcome file.
  Live work, said exactly (`inputs#I4` item 1): TWO starts in one ledger, evidence/fixture-starts.ndjson, the spare
  `R4-2` unspent, on 2.1.325 by path under the rig's private config — `R4` 2026-11-18T19:40:12Z (`24/24`, `verify-exit:
  0`, `browser-tools-prompt: absent`, Run E `box`, `record: 27`, `keys: none`, `stop: none`, the teardown whole) ·
  `R4M` 19:40:47Z (`mcp-child: exit 0`, `capture-lines: 4`, `tap-fixture: written`, the teardown whole). The set
  `fixtures/claude/2.1.325/` (28 files) is a plain copy of the two recorders' output (entries 13 and 14), recorded
  under the rig's PRIVATE config with the browser-tools key seeded off: it is not a reading of a real home, and
  nothing is claimed for 2.1.326 / 2.1.327.
- **Decisions & corrections after implement:**
  - The builder's own read of `expiresAt` from the real `~/.claude/.credentials.json` before the round is a SECOND
    read of that file, outside the letter of the founder's cap («READ once», by the rig — `inputs#I2` answer 2);
    nothing kept or written. Named plainly so the founder is told (the wrap's last card carries it, `inputs#I4` item
    2). Curation candidate: the token-window check before a live round is the rig's own (it refuses under 20
    minutes), never a hand read.
  - The overseer's wrap directive (`inputs#I4`) judges the other departures — the 5 s `MCP_LEAVE` wait in `R4M`, the
    0644 modes set on the landed fixtures, the one in-intent scope-record line — as the reconcile's to judge; no
    ruling beyond it.
  - The pass record: «Corrections or rulings given during the pass: none.»
- **Process hygiene, re-measured:** `gate.py census` (comm `claude`, `viola`, `viola-mcp-tap`, `viola-live-rig`,
  `viola-fake-agent`; args `viola-rig-1000`, `projects/viola/target/debug/viola`; this run dir): `census: 16 running`,
  none this chunk's — the host's overseer sessions, the prototype's `viola run` wrappers and their `claude` children,
  `~/.viola/bin/0.1.0-…/viola mcp`, `~/.local/viola-product/bin/viola wait` and one prototype `viola wait`; no
  `viola-live-rig`, `viola-mcp-tap` or `target/debug/viola` process.
