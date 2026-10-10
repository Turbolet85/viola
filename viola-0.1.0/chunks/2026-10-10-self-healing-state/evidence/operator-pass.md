# Operator pass — 2026-10-10-self-healing-state

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual", `inputs#I3`). Times are `date -u` or the gate
trail's own `ts`; the pass ran on 2026-10-10. No mutation run, no workflow dispatch and no live `claude` start
is part of it (`inputs#I1`, `inputs#I3`).

## Before it — the block, read green twice

The whole block read green in one call of the gate tool, on HEAD `2f1efe3a8962` plus the chunk's uncommitted
edits: 16 entries, 13 green, 0 red, 3 not run. Entries 14 to 16 are the operator's and are this pass. It was
fired twice. The first firing ended 10:43:46Z; one assertion was then added to the chaos case (its
`state-recovered` line against the diag-line schema, `red-green.md` section 3), and the second firing, ended
10:46:27Z, is the reading on the final tree. No push went out before every entry read green.

The second firing's lines, from the trail:

```
  1 lint        green · exit 0 · 0.69s · 0 B → 1.2.log · cargo fmt --all --check
  2 lint        green · exit 0 · 1.32s · 221 B → 2.2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 10.61s · 195244 B → 3.2.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 0.7s · 7197 B → 4.6.log · bash scripts/agent-run.sh run --unit --filter 'test(/torn_… (120 chars)
  5 integration green · exit 0 · 11.49s · 6893 B → 5.6.log · bash scripts/agent-run.sh run --integration --filter 'bina… (166 chars)
  6 probe       green · exit 0 · 0.01s · 0 B → 6.2.log · git diff --quiet 2f1efe3a8962 -- src crates/viola-channel … (248 chars)
  7 smoke       green · exit 0 · 0.1s · 47 B → 7.2.log · bash scripts/agent-run.sh cleanup --session p-shs-smoke
  8 smoke       green · exit 0 · 4.46s · 436 B → 8.2.log · bash scripts/agent-run.sh boot --session p-shs-smoke --ins… (71 chars)
  9 smoke       green · exit 0 · 0.1s · 216 B → 9.2.log · bash scripts/agent-run.sh status --session p-shs-smoke
 10 probe       green · exit 0 · 0.02s · 10 B → 10.2.log · bash scripts/g2-zero-panics.sh
 11 probe       green · exit 0 · 0.32s · 108 B → 11.2.log · bash scripts/agent-run.sh schema-check
 12 smoke       green · exit 0 · 0.31s · 176 B → 12.2.log · bash scripts/agent-run.sh cleanup --session p-shs-smoke
 13 probe       green · exit 0 · 56.83s · 241263 B → 13.2.log · bash scripts/agent-run.sh pre-push
 14 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 15 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 16 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold:

- entry 3: unit 1540 of 1540; entry 4: the 43 selected inline cases, 43 passed; entry 5: the five named
  binaries, 43 of 43;
- entry 6, the preservation guard against `2f1efe3a8962`: exit 0, no output;
- entries 7 to 9 and 12, the smoke session `p-shs-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 10: `g2: clean`; entry 11: schema-check 145 files, 1952 lines, 0 torn, no failure;
- entry 13, `pre-push`: `"ok":true`, `"stage":"linux-tests"`; coverage 1902 of 1902, doctest 0 of 0, playwright
  1 of 1, `gate` no breaches.

One limit of entries 10 and 11 on this host, measured after the second firing: no `.ndjson` file under
`target/e2e-home` holds a `state-recovered` line (`grep -rl`, 0 files). A local run removes each test home when
its test ends, so neither entry read the chaos case's line here. The case holds that line to the schema itself;
CI keeps its homes, and its G2 and G4 steps read them (the CI read below).

The process list read after that firing (10:46:36Z) held no process whose executable lies under this
repository. The host's pressure record was not read for these windows.

The scope read (`gate.py scope`, 10:47:35Z): `scope: clean — changed 12 · listed 12 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 74`, base HEAD. No scope record was needed:
every edited file is in research's two lists.

## Step 1 — `pre-push` (entry 13) on the uncommitted tree, 10:48:26Z to 10:49:23Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 13`):

  ```
   13 probe       green · exit 0 · 56.37s · 241263 B → 13.3.log · bash scripts/agent-run.sh pre-push
  entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 1902 of 1902, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓.
- No source or test file changed between the block's second firing and this one; the files written between them
  are the friction ledger, the gate trail and this chunk's evidence.

## Step 2 — entry 14, hygiene (fired as written)

- Read at 10:49:50Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 67 (runs 60 · evidence 2 · inputs 5) · trails 16 not read · copies 3 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this section was added, right before the commit: the verdict is in the next section,
  which was written after the commit and rides the next one.
