# Operator pass — 2026-10-10-session-links (Part A)

The implementer drove this pass on the operator's word, given with the implement invocation ("Then run the
operator pass for Part A with the ci.py conclusion read (leg=operator) as usual; fold a red into a fix commit
and read CI again; report the run ids", `inputs#I4`). It is plan step 16, which step 11 does not hold. The tree
it takes to CI is Part A alone: steps 1 to 10. Steps 12 to 15 are held by step 11 and nothing of them is built:
no `src/cmd/link.rs`, no `src/cmd/unlink.rs`, no `tests/channel_links.rs`, no dispatch arm for `link` or
`unlink`. Times are `date -u` readings or the tools' own; the pass ran on 2026-10-11. No mutation run, no
workflow dispatch and no start of the real `claude` is part of it (`inputs#I1`, `inputs#I4`).

## Before it — the block, fired once by this implement run

One whole firing (`gate.py run --plan … --run-dir .andromeda/runs/2026-10-11T00-04-43-implement --marker
2026-10-10-session-links`, backgrounded), on HEAD `f02d54865815` plus the chunk's uncommitted edits. Its entry
lines and summary line, as printed:

```
  1 lint        green · exit 0 · 0.77s · 0 B → 1.log · cargo fmt --all --check
  2 lint        green · exit 0 · 0.13s · 72 B → 2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 8.31s · 247395 B → 3.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 1.06s · 5930 B → 4.log · bash scripts/agent-run.sh run --unit --filter 'test(/link_… (101 chars)
  5 integration green · exit 0 · 5.12s · 4799 B → 5.log · bash scripts/agent-run.sh run --integration --filter 'bina… (139 chars)
  6 integration red · exit 0 ✗ (exit 1) · exit 1 · 0.57s · 1577 B → 6.log · bash scripts/agent-run.sh run --integration --filter 'bina… (76 chars)
  7 probe       green · exit 0 · 0.01s · 0 B → 7.log · git diff --quiet f02d54865815 -- crates/viola-agent-claude… (673 chars)
  8 smoke       green · exit 0 · 0.08s · 47 B → 8.log · bash scripts/agent-run.sh cleanup --session p-links-smoke
  9 smoke       green · exit 0 · 5.45s · 798 B → 9.log · bash scripts/agent-run.sh boot --session p-links-smoke --i… (73 chars)
 10 smoke       green · exit 0 · 0.09s · 216 B → 10.log · bash scripts/agent-run.sh status --session p-links-smoke
 11 probe       green · exit 0 · 0.02s · 10 B → 11.log · bash scripts/g2-zero-panics.sh
 12 probe       green · exit 0 · 0.31s · 108 B → 12.log · bash scripts/agent-run.sh schema-check
 13 smoke       green · exit 0 · 0.3s · 178 B → 13.log · bash scripts/agent-run.sh cleanup --session p-links-smoke
 14 probe       green · exit 0 · 63.12s · 300467 B → 14.log · bash scripts/agent-run.sh pre-push
 15 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 16 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 17 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 17 · green 13 · red 1 (6) · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold:

- entry 3: unit 1899 of 1899; entry 4: the 34 inline cases the four prefixes select, 34 passed; entry 5: the
  four named binaries, 28 of 28;
- **entry 6 is red, and that red is the hold.** It selects `binary(channel_links)`, a file of step 15. Its log
  reads nextest's own refusal, `operator didn't match any binary names` under `binary(channel_links)`, then
  `failed to parse filterset`: the text of the entry's baseline. The plan's Test Commands prose says the entry
  reads red for as long as step 11 holds, and that this red is the hold and nothing else. No test ran in it;
- entry 7, the preservation guard against `f02d54865815`: exit 0, no output;
- entries 8 to 10 and 13, the smoke session `p-links-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 11: `g2: clean`; entry 12: schema-check 160 files, 2220 lines, 0 torn, no failure. On this host that
  read does not reach the new cases' lines: a root test's home is removed when the test ends, so the entry read
  the booted smoke home and the kept live homes (the same two figures the previous chunk's entry read). The new
  root cases validate their own role lines against `schemas/diag-line.v1.json` in the test
  (`tests/cli_links.rs` `assert_logs_clean`); CI keeps its test homes and its G4 step reads them.

Before the block, two hand probes: `cargo fmt --all --check` exit 0, and `cargo clippy --workspace --all-targets
--features fake-agent -- -D warnings`, which read one finding in a new test
(`clippy::cloned_ref_to_slice_refs`, `crates/viola-state/tests/state_replay.rs`), fixed before the block; the
re-run exited 0.

The Windows lint, on this host, after that fix: `cargo clippy --workspace --all-targets --features fake-agent
--target x86_64-pc-windows-msvc -- -D warnings` with `CARGO_TARGET_DIR=target/wincheck` (check only, nothing
linked or run) exited 0. It runs no test.

The scope read (`gate.py scope`, 00:20:41Z): `scope: clean — changed 13 · listed 13 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 59`, base HEAD. No `scope-record.md` exists and
none is needed.

## Step 1 — `pre-push` (entry 14) on the uncommitted tree, 00:21:09Z to 00:22:12Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 14`, this run's dir), its second firing
  on this tree; no source, test or config file changed between the two:

  ```
   14 probe       green · exit 0 · 63.1s · 300467 B → 14.2.log · bash scripts/agent-run.sh pre-push
  entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2321 of 2321, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches.
- The coverage summary it left (`target/agent-run/artifacts/llvm-cov-summary.json`, written 00:22:10Z, inside
  the run's window) reads lines 98.08 (10218 of 10418), functions 97.99 (1221 of 1246), regions 97.89 (19922 of
  20351). The thresholds are 85, 95 and 80.
- The process list read at 00:22:17Z, every process by its command line: none under this repository's
  `target/` and none under the test-home base.

## Step 2 — entry 15, hygiene (fired as written)

- Read at 00:22:17Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 51 (runs 44 · evidence 1 · inputs 6) · trails 15 not read · copies 4 not read by P1 —
  0 host paths kept · binary 0 not read by P1`, base HEAD. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No
  row to rewrite.
- Read once more after this file was added, right before the commit: the verdict is in the next section, which
  was written after the commit and rides the next one.
