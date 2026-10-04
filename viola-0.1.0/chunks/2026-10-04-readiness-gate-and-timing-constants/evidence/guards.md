# Guard pairs — 2026-10-04-readiness-gate-and-timing-constants

Measured 2026-10-04 at /implement on the Linux dev host (testing.md 2026-09-25: every new guard test carries its
remove-the-guard run). Each neutralising edit was re-read (`grep -n`) before its run, and the restore was re-read
before the next one.

| guard | neutralised as | command | guard off | guard on |
|---|---|---|---|---|
| the feed poisons on a caught vt100 panic (`src/run/gate.rs` `guarded`, `screen.poison()`) | the call commented out | `bash scripts/agent-run.sh run --unit --filter 'test(/run::gate::tests::/)'` | exit 1 · `feed_panic_writes_one_parse_rejected_line_until_the_size_changes` FAIL at `src/run/gate.rs:226` — `left: 4` / `right: 3` (the poisoned model re-panicked on the same-size bytes and wrote a second line) | exit 0 · 3/3 PASS |
| the `vt100_feed` target's silent panic hook | see `fuzz-override.md` | — | crash, `deadly signal` | 0 |

The screen model's own guards (poisoned → `input-not-ready`, the quiet and maximum-wait boundaries) are pinned by
literal-oracle boundary cases in `crates/viola-agent-claude/src/screen.rs` (`verdict_at_each_boundary` at 299/300 ms and
4 999/5 000 ms; `verdict_poisoned_is_input_not_ready`); their mutants are graded at the epoch-boundary audit
(testing.md 2026-10-04 — no mutation entry in a chunk's block).

Red-before-green for the two new unit entries is the P5 baseline (`nextest-exit-4`, 0 selected, the files absent);
at implement entry 3 selected 21 tests and entry 4 selected 3, all PASS (gate trail
`.andromeda/runs/2026-10-04T04-59-18-implement/`).
