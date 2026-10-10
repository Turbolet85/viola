# Operator pass — 2026-10-10-windows-mutation-grade

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass up to the ordinary CI read (ci.py conclusion, leg=operator), reading the attempt number", `inputs#I4`). Times
are `date -u`; the pass ran on 2026-10-10.

**The pass ends at the ordinary CI read. It fires no dispatch.** `windows-mutants.yml` was not dispatched, and the
block's last entry (`ci.py conclusion --sha HEAD --name mutants --wait 5400`) was not driven: step 11 holds both
until the founder's own word, given after this dispatch was shown to him, reaches the implementer through the
operator (`inputs#I3`, `inputs#I4`). No such word is recorded here, because none was given.

## Before it — the block, 03:28:58Z to 03:31:09Z

The whole block read green in one call of the gate tool, on the tree the three witness runs had measured (HEAD
`781563cd59a6` plus the chunk's uncommitted edits): 26 entries, 22 green, 0 red, 4 not run. Entries 23 to 26 are
the operator's; 23 to 25 are this pass and 26 is held. Fourteen of the 22 had also read green in a first firing at
03:02Z, ahead of the witness runs (entries 1 to 13 and 15). No push went out before every entry read green.

The readings of the whole-block firing:

- entries 1 to 3: `cargo fmt --all --check`; clippy with `-D warnings`; clippy of viola-state, viola-pty and
  viola-e2e with their tests for `x86_64-pc-windows-msvc` in `target/wincheck`; each exit 0. Entry 3 shows the
  Windows-gated test code compiles and lints. It runs no Windows test;
- entry 4: unit 1502 of 1502; entry 5: unit 1502, integration 355 of 355;
- entry 6, the workflow contract test: 4 of 4, the label case among them;
- entry 7: `zizmor .github/workflows/`, exit 0, no findings; entry 8, the forbidden-key probe: exit 0, no output;
- entries 9 to 11, the counts: four `viola` items; one `timeout-minutes: 120`; one "dispatched only during the";
- entry 12, the preservation guard against `781563cd59a6`: exit 0; entry 13, no `#[ignore]`, retry or skip
  added: exit 0, no output;
- entry 14, the reader of `linux-witness.json`: printed `true`, exit 0;
- entry 15: `cargo deny check`, exit 0, with syn and proc-macro2 as direct viola-e2e dependencies and no new
  ignore;
- entries 16 to 18 and 21, the smoke session `p-wmg-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` in 10.2 s;
  status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`, `home_removed:true`, `killed:[]`;
- entry 19: `g2: clean`; entry 20: schema-check 145 files, 1952 lines, 0 torn, no failure;
- entry 22, `pre-push`: `"ok":true`, `"stage":"linux-tests"`; coverage 1857 of 1857, doctest 0 of 0, playwright
  1 of 1, `gate` no breaches; 62.57 s.

The host over the block's window (`hostwatch.py read --from 03:28:58Z --to 03:31:09Z --for viola`): verdict
`QUIET`, 0 s stalled on IO. The process list read after the block held no process of this repository: seven
processes named `viola` were another tree's build, decided by `/proc/<pid>/exe`.

The scope read after the block (`gate.py scope`): `scope: clean — changed 11 · listed 11 · recorded 0`, base HEAD.
No scope record was needed: every edited file is in research's two lists.

## Step 1 — `pre-push` (entry 22) on the uncommitted tree, 03:31:59Z to 03:32:55Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 22`): green, exit 0, 56.06 s. Its document:
  `"ok":true`, `"stage":"linux-tests"`; coverage 1857 of 1857, doctest 0 of 0, playwright 1 of 1, `gate` no
  breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓.
- No source or test file changed between the block's firing and this one (the last source edit is 03:00:46Z); the
  files written between them are the friction ledger and the gate trail.

## Step 2 — entry 23, hygiene (by hand)

- Read at 03:33:00Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 76 (runs 61 · evidence 9 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- The three mutation runs' stderr files were never filed under the run dir or `evidence/`: each opens with
  cargo's own build lines, which carry the repository's absolute path. They stay under the ignored
  `target/witness-wmg/`; their outcome lines are the three `*-outcomes.txt` files here.
- Read once more after this file was added, right before the commit: the verdict is in the next section.
