# Operator pass — 2026-10-10-budget-governor

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass: pre-push, the hygiene entry, the pre-CI commit, the push, and the ci.py conclusion read (leg=operator).
Fold a red into a fix commit and read CI again; report the run ids", `inputs#I8`). It is plan step 16; step 15
as revised lifted the hold (`inputs#I6`, `inputs#I7`). Times are `date -u` or the gate trail's own `ts`; the
pass ran on 2026-10-10. No mutation run, no workflow dispatch and no start of the real `claude` is part of it
(`inputs#I1`).

## Before it — the block, read green by the run that built steps 1 to 14

This session re-entered at step 16 and did not fire the block again. Its green reading is the earlier implement
run's (`.andromeda/runs/2026-10-10T21-29-10-implement/`, the trail's call stamped 22:01:10Z), on HEAD
`396c108ba64c` plus the chunk's uncommitted edits: 16 entries, 13 green, 0 red, 3 not run. Entries 14 to 16 are
the operator's and are this pass. The same trail holds one earlier whole firing, stamped 21:58:51Z, with the
same summary line.

The block's lines, from that trail:

```
  1 lint        green · exit 0 · 0.76s · 0 B → 1.2.log · cargo fmt --all --check
  2 lint        green · exit 0 · 1.22s · 134 B → 2.2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 8.13s · 242871 B → 3.2.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 0.93s · 10991 B → 4.2.log · bash scripts/agent-run.sh run --unit --filter 'test(/perce… (147 chars)
  5 integration green · exit 0 · 4.74s · 9514 B → 5.2.log · bash scripts/agent-run.sh run --integration --filter 'bina… (205 chars)
  6 probe       green · exit 0 · 0.01s · 0 B → 6.2.log · git diff --quiet 396c108ba64c -- crates/viola-agent-claude… (424 chars)
  7 smoke       green · exit 0 · 0.09s · 47 B → 7.2.log · bash scripts/agent-run.sh cleanup --session p-bg-smoke
  8 smoke       green · exit 0 · 4.13s · 348 B → 8.2.log · bash scripts/agent-run.sh boot --session p-bg-smoke --inst… (70 chars)
  9 smoke       green · exit 0 · 0.09s · 216 B → 9.2.log · bash scripts/agent-run.sh status --session p-bg-smoke
 10 probe       green · exit 0 · 0.02s · 10 B → 10.2.log · bash scripts/g2-zero-panics.sh
 11 probe       green · exit 0 · 0.32s · 108 B → 11.2.log · bash scripts/agent-run.sh schema-check
 12 smoke       green · exit 0 · 0.3s · 175 B → 12.2.log · bash scripts/agent-run.sh cleanup --session p-bg-smoke
 13 probe       green · exit 0 · 62.43s · 295146 B → 13.2.log · bash scripts/agent-run.sh pre-push
 14 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 15 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 16 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold, read in this session while they still stand:

- entry 3: unit 1865 of 1865; entry 4: the 72 inline cases the seven prefixes select, 72 passed; entry 5: the
  six named binaries, 62 of 62;
- entry 6, the preservation guard against `396c108ba64c`: exit 0, no output;
- entries 7 to 9 and 12, the smoke session `p-bg-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 10: `g2: clean`; entry 11: schema-check 160 files, 2220 lines, 0 torn, no failure.

What changed between that firing and this pass: the plan's revision of step 15 (`plan.md`, `research.md`,
`scope.md`, the chunk's inputs I6 and I7, the revision's phase run dir), the friction ledger and the handoff's
session-end stamp. No file under `src`, `crates`, `tests`, `.config`, `scripts`, `fixtures`, `plugin` or
`schemas`, and neither `Cargo.toml` nor `Cargo.lock`, is newer than that trail (a `find -newer` read at 22:18Z,
empty).

The Windows lint, on this host, 22:18:30Z: `cargo clippy --workspace --all-targets --features fake-agent --target
x86_64-pc-windows-msvc -- -D warnings` with `CARGO_TARGET_DIR=target/wincheck` (check only, nothing linked or
run) exited 0 in 0.12 s, its build already current for this tree. It runs no test.

The scope read (`gate.py scope`, 22:20:01Z): `scope: clean — changed 31 · listed 31 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 75`, base HEAD. No `scope-record.md` exists
and none is needed.

## Step 1 — `pre-push` (entry 13) on the uncommitted tree, 22:18:34Z to 22:19:37Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 13`, this run's dir):

  ```
   13 probe       green · exit 0 · 62.8s · 295147 B → 13.log · bash scripts/agent-run.sh pre-push
  entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2279 of 2279, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches. Atoms, from the trail (`gate.py show … --n 13`): `exit 0` ✓, `contains "ok":true` ✓,
  `contains "stage":"linux-tests"` ✓.
- The coverage summary it left (`target/agent-run/artifacts/llvm-cov-summary.json`, written 22:19:35Z, inside
  the run's window) reads lines 97.98 (9739 of 9940), functions 97.90 (1166 of 1191), regions 97.76 (18694 of
  19122). The thresholds are 85, 95 and 80.
- The process list read at 22:20:02Z, every process by its executable path: none under this repository's
  `target/` and none under the test-home base.

## Step 2 — entry 14, hygiene (fired as written)

- Read at 22:19:55Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 67 (runs 56 · evidence 1 · inputs 10) · trails 21 not read · copies 8 not read by P1 —
  0 host paths kept · binary 0 not read by P1`, base HEAD. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No
  row to rewrite.
- Read once more after this file was added, right before the commit: the verdict is in the next section, which
  was written after the commit and rides the next one.
