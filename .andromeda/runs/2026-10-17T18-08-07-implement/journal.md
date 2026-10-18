# Journal — 2026-10-17-optional-plugin-install · implement

Long runs, one line each at the start and at the verdict (UTC, `date -u`).

- 2026-10-17T18:15Z · start · `gate.py run --only 1,2,3,4,5,6,7,8,9,10,11` (lints, deny, unit, the new cases, integration, E2E), backgrounded
- 2026-10-17T18:17Z · verdict · `entries 21 · green 10 · red 1 (11)`: `contract_windows_mutation_scope` read `unlisted: src/human/verbs.rs` (the `cfg!(windows)` of `command_word`); fixed by the companion line in `windows-mutants.yml` (scope record), `--only 11` green
- 2026-10-17T18:17:55Z / 18:18:04Z · the remove-the-guard control on `plugin.rs:52`, red then green (`evidence/guard-controls.md`)
- 2026-10-17T18:18Z · start · `gate.py run --only 12,13,14,15,16,17,18` (the booted smoke, G2, G4, cleanup, pre-push), backgrounded
- 2026-10-17T18:20Z · verdict · `entries 21 · green 7 · red 0`: entries 12-18 green, pre-push 87.62 s
- 2026-10-17T18:20:41Z · `cargo build --bin viola` for the live round (`target/debug/viola` read stale, the earlier chunk's 16:15 build)
- 2026-10-17T18:21:29Z – 18:22:44Z · the live round: STOPPED at the (a) guard after command 1 (the local scope landed in the repository root's `.claude/settings.local.json`, not `proj/.claude/`); commands 3 and 4 run at once; 0 starts; ledger row `round-stopped` (`evidence/live-round.md`, `evidence/live-sessions.ndjson`)
- 2026-10-17T18:25Z – 2026-10-18T01:42Z · idle: the founder's question held by the driver's hook (no answer in 58 min), then the answer relayed (`inputs#I5`)
- 2026-10-18T01:44:07Z · the run dir and the tree's end state re-read from disk (the guard at `plugin.rs:52`, no source newer than the 18:20:41Z build)
- 2026-10-18T01:44:31Z · rig 1's residual `.claude/settings.local.json` removed (the founder's Q3)
- 2026-10-18T01:44:42Z – 01:45:13Z · rig 2 (`proj/` its own git repo): install, command 1, the guard passed, command 2
- 2026-10-18T01:45:33Z · start 1 ledgered; 01:45:41Z started; no `session-start` in 60 s, no `mcp.ndjson` in either home; 01:47:19Z stopped (3 presses, child exit 1); RED, not re-fired; census 0 (ledger 01:47:41Z)
- 2026-10-18T01:48Z · start 2 HELD for the founder (the same modal would meet it); the plugin left installed at `proj/`'s local scope, commands 3-4 unrun; the operator pass not started (it waits for the round, `inputs#I5` Q2)
- 2026-10-18T02:14:55Z · `inputs#I6` snapped (the founder: one Enter on a read trust dialog; the cap raised to 3)
- 2026-10-18T02:15Z – 02:17Z · `evidence/live-pty-trust.py` written (live-pty.py + an in-memory screen + at most one Enter); 8 synthetic screen controls green
- 2026-10-18T02:17:32Z · start 2 ledgered (`hyprctl locked` read `true`; no desktop key in this start); 02:17:51Z started; the rig read `unselected` to the window's close (02:18:36Z), no key; no `session-start` in 75 s, no `mcp.ndjson`; 02:19:29Z stopped (child exit 1); RED, not re-fired; census 0 (ledger 02:19:59Z)
- 2026-10-18T02:20Z · start 3 HELD for the founder (the same dialog would meet it); the plugin left installed at `proj/`'s local scope, commands 3-4 unrun; the operator pass not started
- 2026-10-18T02:21:49Z · `inputs#I7` snapped (the overseer: close the round as it stands)
- 2026-10-18T02:22:00Z – 02:22:46Z · commands 3-4 from rig 2's `proj/`, both exit 0 `ok`; the plugins diff read (the orphaned `cache/viola/viola/0.1.0/`, 3 files); census 0; ledger row `round-closed`
- 2026-10-18T02:23:04Z · the operator pass: `--only 3,18` green (pre-push 82.87 s, breaches []); hygiene clean ×2; pre-CI commit `43c64109`; push (entry 20) green, origin 9afbff44→43c64109
- 2026-10-18T02:25:11Z – 02:35Z · `ci.py conclusion`: verdict green, checks 15/15, wall 586 s, `ci#39090008812` attempt 1 (`evidence/operator-pass.md`)
