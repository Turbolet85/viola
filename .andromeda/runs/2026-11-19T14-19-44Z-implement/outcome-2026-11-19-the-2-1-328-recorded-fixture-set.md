# Outcome — 2026-11-19-the-2-1-328-recorded-fixture-set

**Outcome: green.** This is the re-entry on plan rev 5 (`inputs#I8`, `inputs#I9`). Every gate entry implement fires
read green:
- step 8's rename, whole;
- `R4-2`, the one spare (24/24, record 27 kept);
- `R4M` (the tap fixture written);
- `L2` (`context-transcript-after: read`);
- the landed set `fixtures/claude/2.1.328/` (28 files), with `STAMPED` naming it;
- the drift, hygiene, ledger-probe and recorded-exchange suites over it;
- the whole integration and E2E layers.

Next → the OPERATOR PASS, entries 24 (pre-push), 25 (`gate.py hygiene`, as written), 26 (the push) and 27
(`ci.py conclusion`, as written). Then the wrap.

## Files changed
- `crates/viola-e2e/src/rig/mod.rs` and `crates/viola-e2e/tests/rig_rehearsal.rs` (steps 1–5). These are the stopped
  run's edits, standing unchanged (§R).
- `fixtures/claude/2.1.328/` — new: 28 files, a plain copy of `target/live-rig/R4-2/record/2.1.328/` (27) and
  `target/live-rig/R4M/record/2.1.328/Mcp.tools-list-1.json` (`evidence/landing.md`).
- `tests/contract_ledger_probes.rs:44` — `STAMPED: [&str; 4]` now names `"2.1.328"`.
- Outside the tree (gitignored, plan step 8): `target/live-rig/R4/record` → `record-2.1.325` (27 files, 27 after), and
  `target/live-rig/R4M/record` → `record-2.1.325` (1 file, 1 after).

## Gates (this run; trail `.andromeda/runs/2026-11-19T14-19-44Z-implement/gate-2026-11-19-the-2-1-328-recorded-fixture-set.json`)
- Entries 3–5 were not re-fired (§R): the `git status`/`stat` precondition held (`evidence/fixture-round.md`).
- `--live-legs --entry 10` (14:20:34Z): entries 6–10 green · `round: COMPLETE · legs fired 1/1 · … green 5 · red 0`.
- `--live-legs --entry 13` (14:21:29Z): entries 11–13 green · `round: COMPLETE · legs fired 1/1 · … green 3 · red 0`.
- `--live-legs --entry 15` (14:21:47Z): entries 14–15 green · `round: COMPLETE · legs fired 1/1 · … green 2 · red 0`.
- `--operator 16`: `green · exit 0 · … history: unmoved` (`census: 0`).
- `--only 1,2,17,18,19`: all green. `diff -r` and `cmp` printed nothing; the resume variant stands.
- `--only 20,21`: green. `cli_verify` 56 passed; the five contract/MCP binaries 127 passed, 0 failed.
- `--only 22,23`: green. Integration 558 passed, 0 failed; E2E 321 passed, 0 failed.

## L2's readings (C3) — recorded as read, no verification claimed
`context-transcript: absent` · `send-1-reply: not-delivered:no-prompt-submitted` · `context-transcript-after: read` ·
`context-submitted: no` · `context-tags: 1` · `context-post: tag`.

## Deviations
- Entries 1–2 (fmt, clippy) were re-fired, though §R says entries 1–5 are not re-fired. Step 14 edits a Rust test file
  after they last ran (12:00), so the changed-surface lint was read on the final tree. Both are green.
- Post-landing entries were fired grouped by `--only` (1,2,17,18,19 · 20,21 · 22,23), where step 15 names
  `--entry {n}` one by one. Every entry read green, so no red was carried past.
- Scope: `gate.py scope` clean — changed 31 · listed 31 · recorded 0 · absorbed 28. Scope record: none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| viola-live-rig (entries 7, 10, 11, 13, 14, 15, 16) | a leg / round step | terminated — each round `COMPLETE` with no survivors; `census: 0` |
| claude 2.1.328 (R4-2: up to 9 one at a time; R4M: 1; L2: 1) | a leg | terminated — each rig summary `census: 0`; tool census below lists none |
| viola-mcp-tap + viola mcp (R4M) | a leg | terminated — `mcp-child: exit 0`, `census: 0` |
| viola run (L2) | a leg | terminated — `child: stopped`, `census: 0` |
`gate.py census --comm claude --comm viola-live-rig --comm viola-mcp-tap --args projects/viola/target/debug/viola`
(14:26:52Z) reads `census: 8 running`. All 8 are the host's own `installs/claude/latest/claude` sessions (other
projects' overseers and builders), none started by this run; `NOT DERIVED` beyond that name read. There is no
`viola-live-rig`, `viola-mcp-tap` or `target/debug/viola` row.

## Decisions & corrections
- Directive (`inputs#I8`): hostwatch was read before each base start. Three QUIET readings, no neighbour named
  (14:20:28Z, 14:21:26Z, 14:21:43Z).
- The cap is spent whole: `R4` (red, the earlier run), `R4-2` (the spare), `R4M` and `L2`. No further start is
  admissible (`inputs#I2` item 1).
- `user-config: changed-by-others` on `L2` is the host's own sessions' rewrite of `~/.claude.json` (host-linux.md
  2026-11-17). It is not `changed-by-rig`, and both stopping atoms held.
