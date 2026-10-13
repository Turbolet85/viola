# Operator pass — 2026-10-13-paste-newline-ledger-row

Run on the operator's word, given in this session's `/andromeda-implement` arguments: "Run the operator pass with
the ci.py conclusion read (leg=operator) as usual." Lines are copied verbatim; the tools' headers are left out.

## Entry 21 (pre-push) on the final uncommitted source tree

`gate.py run … --only 1,2,5,10,11,12,13,21` (firing 2 of entry 21), after the last source edit:

```
 21 probe       green · exit 0 · 69.04s · 321656 B → 21.2.log · bash scripts/agent-run.sh pre-push
```

The document read `"ok":true,"stage":"linux-tests"`: coverage 2475 passed / 0 failed, playwright 1 passed, gate
breaches `[]`. After it only evidence and run-dir records were written; no source changed.

The same firing read entries 12 and 13 red, both surfaced in the implement outcome
(`.andromeda/runs/2026-10-13T21-22-15-implement/outcome-2026-10-13-paste-newline-ledger-row.md`):

```
 12 probe       red · last line '0' ✗ (last: '1') · exit 0 · 0.02s · 2 B → 12.3.log · python3 -c 'import re,sys; print(sum(len(re.findall(r"/17\… (311 chars)
 13 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 13.3.log · git diff --quiet 19483111dbe9 -- . ':!crates/viola-agent-c… (539 chars)
```

## Entry 22 — hygiene, fired as written before the pre-CI commit

First firing:

```
P1 viola-0.1.0/chunks/2026-10-13-paste-newline-ledger-row/evidence/live-round.md:59 ×2 · tmp
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 51 (runs 42 · evidence 4 · inputs 5) · trails 13 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The census paragraph spelled Run A's temp-dir pattern with the temp dir's root. It was rewritten as "the OS temp
dir's `viola-verify-*`" and the entry fired again:

```
hygiene: clean — read 51 (runs 42 · evidence 4 · inputs 5) · trails 13 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`chore(2026-10-13-paste-newline-ledger-row): operator pre-CI commit, for the run this chunk's verdict reads`, made
of the whole tree (`git add -A`) on `build/viola-0.1.0`. The record above rides this commit. The sections below
are written after it and ride the wrap commit.
