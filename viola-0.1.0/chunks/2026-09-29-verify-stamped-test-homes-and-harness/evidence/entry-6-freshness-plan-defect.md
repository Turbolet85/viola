# Entry 6's freshness check targets a directory — a plan defect, for the wrap to retarget

**Operator's word (2026-09-29, at the operator pass):** "The entry-6 freshness red is a plan-target defect:
record it for the wrap to retarget the freshness check to a file the run writes, not a directory, rather than
accept a standing red."

## The defect
Plan gate 6 (`bash scripts/agent-run.sh run`) carries `artifact = 'target/agent-run/artifacts/'`. `gate.py`
judges an `artifact` by that path's own mtime. The path is a directory, and on this host (NTFS) overwriting a
file inside a directory does not move the directory's mtime. `run` overwrites `junit-nextest-unit.xml` and
`junit-nextest-integration.xml` in place, so the atom read `STALE` on every run of gate 6 in this chunk, three
of them, while the run's own files were fresh. The measured readings are in `item8-witness-guard.md` §Gate 6.

It is not a red of this chunk's code:
- gate 6's `exit 0` and `contains "ok":true` atoms were green on the final tree;
- its files were written inside the entry's window (09:38:13 and 09:38:40 +0200, archived as
  `target/run-archive/359/`);
- the directory mtime moved only when a later stage created a new file inside it.

## What the wrap retargets
The check must name a FILE that the run writes on every default run. The candidate is
`target/agent-run/artifacts/junit-nextest-integration.xml`: the integration layer's JUnit, which is also the
report that names `contract_ledger_probes` (plan acceptance "named in the archived JUnit").

- Retarget the chunk's `plan.md` gate 6 to that file before the wrap's light gate (P7.1) re-runs the block, so
  the light gate does not read a standing red.
- Record the rule so later plans do not repeat it: an `artifact` key names a file the entry writes, never a
  directory.

/implement touched neither `plan.md` nor any spec (both are read-only to it). No standing red is accepted.
