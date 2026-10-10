# Operator pass — 2026-10-10-viola-revive

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual", `inputs#I5`). Times are `date -u` or the gate
trail's own `ts`; the pass ran on 2026-10-10. No mutation run and no workflow dispatch is part of it
(`inputs#I1`). The three live `claude` starts of plan step 12 were made before it and are recorded in
`live-revive.md`; the pass itself starts none.

## Before it — the block, read green twice

The whole block read green in one call of the gate tool, on HEAD `00c73fda87ef` plus the chunk's uncommitted
edits: 16 entries, 13 green, 0 red, 3 not run. Entries 14 to 16 are the operator's and are this pass. It was
fired twice. The first firing ended 14:42Z. One test edit followed it: in `tests/cli_revive.rs` the removal of
the recorded directory became a bounded wait on the removal itself (a directory a process still holds cannot be
removed on Windows; that hold was not measured here, and on this host the first try removes it). The second
firing, ended 14:55Z, is the reading on the final tree. No push went out before every entry read green.

The second firing's lines, from the trail:

```
  1 lint        green · exit 0 · 0.7s · 0 B → 1.2.log · cargo fmt --all --check
  2 lint        green · exit 0 · 0.34s · 134 B → 2.2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 6.83s · 207662 B → 3.2.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 0.75s · 12026 B → 4.2.log · bash scripts/agent-run.sh run --unit --filter 'test(/snaps… (155 chars)
  5 integration green · exit 0 · 4.53s · 7744 B → 5.2.log · bash scripts/agent-run.sh run --integration --filter 'bina… (144 chars)
  6 probe       green · exit 0 · 0.01s · 0 B → 6.2.log · git diff --quiet 00c73fda87ef -- crates/viola-channel crat… (503 chars)
  7 smoke       green · exit 0 · 0.09s · 47 B → 7.2.log · bash scripts/agent-run.sh cleanup --session p-rev-smoke
  8 smoke       green · exit 0 · 3.79s · 285 B → 8.2.log · bash scripts/agent-run.sh boot --session p-rev-smoke --ins… (71 chars)
  9 smoke       green · exit 0 · 0.1s · 215 B → 9.2.log · bash scripts/agent-run.sh status --session p-rev-smoke
 10 probe       green · exit 0 · 0.02s · 10 B → 10.2.log · bash scripts/g2-zero-panics.sh
 11 probe       green · exit 0 · 0.32s · 108 B → 11.2.log · bash scripts/agent-run.sh schema-check
 12 smoke       green · exit 0 · 0.31s · 176 B → 12.2.log · bash scripts/agent-run.sh cleanup --session p-rev-smoke
 13 probe       green · exit 0 · 58.35s · 255563 B → 13.2.log · bash scripts/agent-run.sh pre-push
 14 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 15 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 16 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold:

- entry 3: unit 1624 of 1624; entry 4: the 70 selected inline cases, 70 passed; entry 5: the four named
  binaries, 55 of 55 (`cli_revive` 11, `chaos_revive` 1, `state_replay` 5, `cli_fake_agent` 38);
- entry 6, the preservation guard against `00c73fda87ef`: exit 0, no output;
- entries 7 to 9 and 12, the smoke session `p-rev-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 10: `g2: clean`; entry 11: schema-check 154 files, 2142 lines, 0 torn, no failure;
- entry 13, `pre-push`: `"ok":true`, `"stage":"linux-tests"`; coverage 2002 of 2002, doctest 0 of 0, playwright
  1 of 1, `gate` no breaches. The kill of the instrumented wrapper in `tests/chaos_revive.rs` did not break the
  coverage merge: the stop rule of `inputs#I4` did not fire.

One limit of entries 10 and 11 on this host: a local run removes each test home when its test ends, so neither
entry read the refusal cases' `cwd-missing` and `no-session` lines or the cut-short snapshot's `state-recovered`
line here. The cut-short case holds its lines to the schema itself; CI keeps its homes, and its G2 and G4 steps
read them (the CI read below). What entry 11 did read beside the smoke home is the stamped live home, with the
three live sessions' role files of step 12.

The process list read after that firing (14:55:38Z) held no process whose command line lies under this
repository's `target/`.

The scope read (`gate.py scope`, 14:52Z and again 14:56:56Z): `scope: UNPARSED 4 · changed 21 · listed 21 ·
recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 86`, base HEAD. Every
edited file is in research's two lists, so no scope record was needed. The four `UNPARSED` rows are
`research.md` lines 171 to 174, the sweep record's four column-0 lines inside the `## Files to modify` section;
research is not edited here, and the rows are the wrap's to settle with the operator.

## Step 1 — `pre-push` (entry 13) on the uncommitted tree, 14:55:49Z to 14:56:47Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 13`):

  ```
   13 probe       green · exit 0 · 58.25s · 255563 B → 13.3.log · bash scripts/agent-run.sh pre-push
  entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2002 of 2002, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓.
- No source or test file changed between the block's second firing and this one; the files written between them
  are the friction ledger, the gate trail and this chunk's evidence.

## Step 2 — entry 14, hygiene (fired as written)

- Read at 14:56:51Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 78 (runs 62 · evidence 9 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this file was added, right before the commit: the verdict is in the next section, which
  was written after the commit and rides the next one.
