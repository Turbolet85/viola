# Operator pass — 2026-10-10-statusline-pass-through

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual", `inputs#I4`). Times are `date -u` or the gate
trail's own `ts`; the pass ran on 2026-10-10. No mutation run and no workflow dispatch is part of it
(`inputs#I1`). The three live `claude` starts of plan step 12 were made before it and are recorded in
`live-statusline.md`; the pass itself starts none.

## Before it — the block, read green

The whole block read green in one call of the gate tool, on HEAD `4d77eacd14d6` plus the chunk's uncommitted
edits: 17 entries, 14 green, 0 red, 3 not run. Entries 15 to 17 are the operator's and are this pass. It was
started at 19:15:48Z. One firing of entries 1 and 2 came before it and read entry 2 red: `clippy::
large_enum_variant` on `SnapshotRead` and `Recovered`, once the snapshot gained its field (216 bytes). The
snapshot is boxed in both enums; the whole block ran after that fix. No push went out before every entry read
green.

The block's lines, from the trail:

```
  1 lint        green · exit 0 · 0.73s · 0 B → 1.2.log · cargo fmt --all --check
  2 lint        green · exit 0 · 0.16s · 72 B → 2.2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 18.17s · 229385 B → 3.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 1.06s · 22515 B → 4.log · bash scripts/agent-run.sh run --unit --filter 'test(/statu… (135 chars)
  5 integration green · exit 0 · 4.73s · 14185 B → 5.log · bash scripts/agent-run.sh run --integration --filter 'bina… (206 chars)
  6 probe       green · exit 0 · 0.01s · 0 B → 6.log · git diff --quiet 4d77eacd14d6 -- crates/viola-agent-claude… (380 chars)
  7 integration green · exit 0 · 7.08s · 5494 B → 7.log · bash scripts/agent-run.sh run --fuzz-replay
  8 smoke       green · exit 0 · 0.12s · 47 B → 8.log · bash scripts/agent-run.sh cleanup --session p-sl-smoke
  9 smoke       green · exit 0 · 6.19s · 795 B → 9.log · bash scripts/agent-run.sh boot --session p-sl-smoke --inst… (70 chars)
 10 smoke       green · exit 0 · 0.09s · 216 B → 10.log · bash scripts/agent-run.sh status --session p-sl-smoke
 11 probe       green · exit 0 · 0.03s · 10 B → 11.log · bash scripts/g2-zero-panics.sh
 12 probe       green · exit 0 · 0.32s · 108 B → 12.log · bash scripts/agent-run.sh schema-check
 13 smoke       green · exit 0 · 0.3s · 175 B → 13.log · bash scripts/agent-run.sh cleanup --session p-sl-smoke
 14 probe       green · exit 0 · 62.48s · 279329 B → 14.log · bash scripts/agent-run.sh pre-push
 15 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 16 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 17 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 17 · green 14 · red 0 · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold:

- entry 3: unit 1765 of 1765; entry 4: the 140 selected inline cases, 140 passed (`viola-agent-claude` 93,
  `viola-core` 5, `viola-state` 9, the root bin 29, the fake agent's own 4); entry 5: the six named binaries, 107
  of 107 (`hook_statusline` 11, `cli_instance_state` 20, `tui_passthrough` 6, `hook_fail_open` 24,
  `cli_fake_agent` 41, `state_replay` 5);
- entry 6, the preservation guard against `4d77eacd14d6`: exit 0, no output;
- entry 7: the five fuzz targets' corpora replayed, 5 passed (the `hook_stdin` corpus holds 12 seeds);
- entries 8 to 10 and 13, the smoke session `p-sl-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 11: `g2: clean`; entry 12: schema-check 154 files, 2142 lines, 0 torn, no failure;
- entry 14, `pre-push`: `"ok":true`, `"stage":"linux-tests"`; coverage 2163 of 2163, doctest 0 of 0, playwright
  1 of 1, `gate` no breaches. The coverage summary it left reads lines 97.69, functions 97.63, regions 97.46
  (the thresholds are 85, 95 and 80).

One limit of entries 11 and 12 on this host: a local run removes each test home when its test ends, so neither
entry read the lines the new cases wrote here. Case 1, case 4, case 5 and case 6 of `binary(hook_statusline)`
hold their own lines to the schema themselves; CI keeps its homes, and its G2 and G4 steps read them (the CI
read below).

Entries 11 and 12 were fired once more at 19:29:08Z, after step 12's live sessions, so that they read the rig
home's role files, the real CLI's statusline lines among them:

```
 11 probe       green · exit 0 · 0.03s · 10 B → 11.2.log · bash scripts/g2-zero-panics.sh
 12 probe       green · exit 0 · 0.34s · 108 B → 12.2.log · bash scripts/agent-run.sh schema-check
entries 17 · green 2 · red 0 · recorded 0 · timeout 0 · not-run 15
```

`g2: clean`; schema-check 157 files, 2198 lines, 0 torn, no failure.

The process list read at 19:27:35Z held no process whose executable lies under this repository's `target/`, no
rig host and no `claude` 2.1.287 by path.

The scope read (`gate.py scope`, 19:27:28Z): `scope: clean — changed 36 · listed 35 · recorded 1 (companion 0 ·
mechanical 1 · in-intent 0 · widening 0) · absorbed 0 · excluded 66`, base HEAD. The one recorded file is
`fuzz/Cargo.lock`.

## Step 1 — `pre-push` (entry 14) on the uncommitted tree, 19:27:57Z to 19:28:58Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 14`):

  ```
   14 probe       green · exit 0 · 60.7s · 279328 B → 14.2.log · bash scripts/agent-run.sh pre-push
  entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2163 of 2163, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓.
- No source or test file changed between the block's firing and this one; the files written between them are
  the friction ledger, the gate trail and this chunk's evidence (the live record, its ledger and its rig
  scripts).
- That call's printed lines were read through a filter that kept the entry's line and the summary line; the two
  lines above are those, and the atoms are the trail's own (`gate.py show … --n 14`).

## Step 2 — entry 15, hygiene (fired as written)

- Read at 19:29:08Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 57 (runs 42 · evidence 9 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this file was added, right before the commit: the verdict is in the next section, which
  was written after the commit and rides the next one.
