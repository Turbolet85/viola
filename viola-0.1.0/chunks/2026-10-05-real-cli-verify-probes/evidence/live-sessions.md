# Live `claude` sessions — 2026-10-05-real-cli-verify-probes

The founder's cap is 18 sessions (12 from ~08:50Z, `relay-6.md`; raised to 15 live on 2026-10-05 after the first entry-7
STOP, then to 18 live after the second, each via the overseer's AskUserQuestion: 10 used, 6 needed, 2 spare). Each line is written before its session starts,
and its outcome is filled in when the session ends.

| # | UTC start | CLI (`--version`) | step / entry | outcome |
|---|---|---|---|---|
| 1 | 2026-10-05T06:28:57Z | 2.1.288 (Claude Code) | step 0 — scratch PTY probe outside the repository | STOP 2: Enter on the trust dialog chose "No, exit"; child exit 1 at ~2.6 s; no hook fired (`screen-probe-2.1.288.md`) |
| 2 | 2026-10-05T08:37:44Z | 2.1.288 (Claude Code) | step 0 (plan revision 2) — Run A shape, scratch probe `probe1` outside the repository | the trust dialog settled at 847 ms (300 ms quiet), `Yes, I trust this folder` on it, no other modal; no byte written; ended by kill (exit 1, ~0.2 s); STOP 2 clean (no `projects` key, no projects dir) |
| 3 | 2026-10-05T08:38:15Z | 2.1.288 (Claude Code) | step 0 (plan revision 2) — Run B shape, scratch probe `probe1`, cwd `<repo root>/.viola-verify-<pid>/` | STOP 3: the start showed a modal, "Allow external CLAUDE.md file imports?" (focus "No, disable external imports"), settled at 563 ms; no byte written, no paste; ended by kill (exit 1, ~0.2 s); no hook fired; no `projects` key or projects dir for the dir, and the root project's two external-include flags unchanged |
| 4 | 2026-10-05T09:20:15Z (row written ~09:20:00Z with a planned 09:20:30Z; the start is the probe's own `date -u`) | 2.1.288 (Claude Code) | step 0 (plan revision 3, the plan's "session 5") — Run B shape only, scratch probe `probe2` outside the repository, cwd `<repo root>/.viola-verify-<pid>/` | clear, no STOP: no modal at start; `ready` settled at 1 084 ms; paste → UserPromptSubmit 31 ms; Stop at 4 379 ms, `turn` settled 314 ms after it; max turn gap 224 ms; Ctrl-C ×2, child exit 0, about 1.4 s after the first Ctrl-C; all four spine hooks fired; input-box literal `for agents` chosen (`screen-probe-2.1.288.md`) |
| 5 | 2026-10-05T09:42:11Z (the record home's stamp; row written 09:42:03Z, before the round) | 2.1.288 (Claude Code) | step 10, gate entry 7 (the 2.1.288 record run) — `verify`'s print-mode probe | the six print rows pass |
| 6 | 2026-10-05T09:42:11Z (same run) | 2.1.288 (Claude Code) | step 10, gate entry 7 — Run A, untrusted, under `/tmp` | `modal-signature` pass; ended by kill; STOP 2 clear (no `projects` key, no `/tmp` transcript dir) |
| 7 | 2026-10-05T09:42:11Z (same run) | 2.1.288 (Claude Code) | step 10, gate entry 7 — Run B, trusted, `<repo root>/.viola-verify-<pid>/` | the four typed rows pass (ready 1 084 ms, latency 50 ms, turn settle 601 ms, max gap 310 ms); the run printed `stamped 2.1.288  10 pass  0 fail`, then `--record` refused the recording (`unable: a recorded payload still holds a path or a username`), verify exit 1, nothing written or copied: **gate entry 7 red, the round STOPPED** (`round-094211Z.txt`) |

| 8 | 2026-10-05T10:00:46Z (record home `viola-record-20261005T100046Z`; row written 10:00:40Z) | 2.1.288 (Claude Code) | step 10, gate entry 7 re-run (the refusal now names its file and check) — print-mode probe | the six print rows pass |
| 9 | 2026-10-05T10:00:46Z (same run) | 2.1.288 (Claude Code) | step 10, gate entry 7 re-run — Run A, untrusted, under `/tmp` | `modal-signature` pass; ended by kill; STOP 2 clear |
| 10 | 2026-10-05T10:00:46Z (same run) | 2.1.288 (Claude Code) | step 10, gate entry 7 re-run — Run B, trusted, `<repo root>/.viola-verify-<pid>/` | the four typed rows pass (ready 1 057 ms, latency 45 ms, turn settle 597 ms, max gap 310 ms); `stamped 2.1.288  10 pass  0 fail`; `--record` refused, now named: `SessionStart.default.json absolute-path`; **entry 7 red again, the round STOPPED** (`round-100046Z.txt`); entry 8 not fired |

Used: 10 of 15 at the **STOP after row 10**, reported before any further round (the overseer's direction).

**Plan correction, 2026-10-05 (the overseer, founder-delegated):** entries 7 and 8 are driven by hand with the plan's exact
`run` except `--home "$h/home"` → `--home "$h/vhome"`: a `/home/` component in the record home survives the scrub and is
refused as an absolute path (the product works as designed; `screen-probe-2.1.288.md`).

| 11 | 2026-10-05T10:20:55Z | 2.1.288 (Claude Code) | step 10, entry 7 by hand (`vhome`) — print-mode probe | the six print rows pass |
| 12 | 2026-10-05T10:20:55Z (same run) | 2.1.288 (Claude Code) | step 10, entry 7 by hand — Run A, untrusted, under `/tmp` | `modal-signature` pass; STOP 1 and STOP 2 clear |
| 13 | 2026-10-05T10:20:55Z (same run) | 2.1.288 (Claude Code) | step 10, entry 7 by hand — Run B, trusted, `<repo root>/.viola-verify-<pid>/` | **`stamped 2.1.288  10 pass  0 fail` — the W5 stamp**; the recording written and copied, exit 0 (`hand-entries-7-8.md`) |
| 14 | 2026-10-05T10:21:40Z (row written 10:21:33Z) | 2.1.287 (Claude Code) | step 10, entry 8 by hand (`vhome`) — print-mode probe | the six print rows pass |
| 15 | 2026-10-05T10:21:40Z (same run) | 2.1.287 (Claude Code) | step 10, entry 8 by hand — Run A, untrusted, under `/tmp` | `modal-signature` pass; STOP 1 and STOP 2 clear |
| 16 | 2026-10-05T10:21:40Z (same run) | 2.1.287 (Claude Code) | step 10, entry 8 by hand — Run B, trusted, `<repo root>/.viola-verify-<pid>/` | `stamped 2.1.287  10 pass  0 fail`; the three screens copied, exit 0 (`hand-entries-7-8.md`) |

Used: **16 of 18**. Step 10 complete: both record entries green by hand under the dated plan correction. 2 spare, unspent. Earlier: 7 of 12 at the **STOP after row 7:** entry 7 is red and there is no retry (a re-run needs 3 sessions, and with entry 8's 3 that is 13 of 12). Entry 8 (2.1.287) was not fired. Revision 1 stopped at step 0 (STOP 2), and revision 2 stopped at step 0 (STOP 3), with no code written either time.

Plan revision 3 (2026-10-05) counts 7 sessions after rows 1-3: step 0 = 1 (row 4), step 10 = 3 + 3. That is 10 of 12, with 2 spare and no retry.
