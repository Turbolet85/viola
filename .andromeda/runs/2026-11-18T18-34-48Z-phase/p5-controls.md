# P5 controls — 2026-11-18-the-2-1-325-recorded-fixture-set

Check 4 (9)'s must-pass reading of entry 15, the one new entry whose verdict turns on a literal (`last line 0` over the
searched text `and the usage budget`). It was fired AS THE ENTRY through `gate.py run --chunk
2026-11-18-the-2-1-325-recorded-fixture-set --docs-root . --entry 15 --project-root
$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/ba40e976-9e88-4122-9f48-a7856532e302/scratchpad/ctl-clone`, a
`git clone --shared` of the project at HEAD `bb204f147f8fc7116e0c774b92b768502bddcdb2`.

- **Must fail.** Input: `fixtures/claude/2.1.325/Mcp.tools-list-1.json` minted in the clone as a copy of
  `fixtures/claude/2.1.302/Mcp.tools-list-1.json`, which holds the clause.
  Line: ` 15 probe       red · exit 1 ✗ (exit 0) · exit 0 · 0.0s · 2 B → 15.log` (run-20261118T192930Z).
- **Must pass.** Input: the same file with `, and the usage budget` removed from the `list` description by the Edit
  tool.
  Line: ` 15 probe       green · exit 1 · 0.0s · 2 B → 15.log` (run-20261118T192941Z).

Entries 13, 14 and 16 assert no literal text (`no output` / an exit only); their baseline reds on the absent input
stand as "can read red", and their own subject is built by the round and the landing.

A first attempt, a hand-written scratchpad script covering entries 13–16 together, was refused by the permission
classifier (verdict: undermining oversight). It was not re-tried in another form; the controls above use the letter's
own clone form instead.
