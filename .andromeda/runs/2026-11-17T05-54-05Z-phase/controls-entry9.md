# Entry 9 — known-positive controls (validation.md check 4 (9))

Fired as the entry (`gate.py run --chunk 2026-11-17-standing-live-rig --docs-root {root} --entry 9 --project-root
{clone}`), plan rev 5, over a scratch clone (`git clone --shared`) at HEAD bd95971c65cf05f2555fed3e4fa6ddcdaf0a1ad2,
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/0a31e5f3-4d4a-43d2-8781-5b6f43ede5cb/scratchpad/clone9`;
script `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/0a31e5f3-4d4a-43d2-8781-5b6f43ede5cb/scratchpad/ctl9.sh`.

- clone, unmodified (rev 1 of the entry): `9 probe green · exit 1 · 2 B` — as on the tree.
- input (a), a tracked `canary-probe.txt` holding the token: `9 probe red · exit 1 ✗ (exit 0) · exit 0` — must fail: fails.
- input (b), only `crates/viola-e2e/tests/rig_rehearsal.rs` holding it: `9 probe green · exit 1` — must pass: passes.
- input (c), a tracked `.andromeda/canary-probe.txt` holding it: `9 probe green · exit 1` — must pass: passes.
