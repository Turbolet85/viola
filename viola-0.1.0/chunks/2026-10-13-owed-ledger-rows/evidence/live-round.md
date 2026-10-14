# Live round — 2026-10-13-owed-ledger-rows

Written 2026-10-13T23:24:11Z (`date -u`) from the gate tool's round logs (`entry 7`–`12` of the run dir's trail,
`.andromeda/runs/2026-10-13T23-05-48-implement/`; the logs under `$TMPDIR/andromeda-gate/2026-10-13-owed-ledger-rows/`)
and this record's own reads of the two copied fixtures. No prompt text, payload value or path beyond the repo-relative
ones is in this file.

The cap: ten live `claude` starts, the founder's own answer (`inputs#I1`, fork 4). The ten were ledgered in the run dir's
`journal.md` at 2026-10-13T23:21:12Z, before the one `gate.py run --live-legs` call that made them. The round was fired
once and neither leg was re-fired: `round: COMPLETE · legs fired 2/2` (`evidence/round-232112Z.txt`).

## Round 1 — the record round (entry 8)

- Start: 2026-10-13T23:21:22Z (the round home's own stamp, `target/e2e-home/viola-record-20261013T232122Z/`); ended
  2026-10-13T23:22:16Z; 53.68 s.
- Starts: five, `claude` 2.1.287 by path — the print probe, Run A, Run B, Run C, Run D.
- Its summary line: `stamped 2.1.287  21 pass  0 fail` (rows 19–21 `statusline-override`, `statusline-shell`,
  `statusline-rate-limits`, each `pass`).
- The `cp` took the two status line variants alone into `fixtures/claude/2.1.287/`.
- Census (entry 9): green, no output — no `claude` process whose cwd is one of this round's probe dirs.

## Round 2 — the live-suite round (entry 11)

- Start: 2026-10-13T23:22:16Z (entry 10's end; the suite home `target/e2e-home/viola-live-2266945/`); ended
  2026-10-13T23:23:12Z; 55.69 s, the harness build included.
- Starts: five, `claude` 2.1.287 first on `PATH` — the print probe, Run A, Run B, Run C, Run D.
- Its verify summary: `stamped 2.1.287  21 pass  0 fail`; its suite line: `"suite":"local-live","passed":1,"failed":0`,
  `"ok":true`.
- Census (entry 12): green, no output.

## The two variants (key names and sizes only)

`StatusLine.default-1.json` — 1 081 bytes, one line ending LF. No `rate_limits`. Top-level keys (13): `session_id`,
`transcript_path`, `cwd`, `scratchpad_dir`, `model` (`id`, `display_name`), `workspace` (`current_dir`, `project_dir`,
`repo` with `host`, `owner`, `name`), `version`, `output_style` (`name`), `cost` (5 keys), `context_window` (6 keys),
`exceeds_200k_tokens`, `fast_mode`, `thinking` (`enabled`).

`StatusLine.rate-limits-1.json` — 1 724 bytes, one line ending LF. Top-level keys (17): the 13 above, `prompt_id`,
`session_name`, `prompt_cache` (11 keys), `rate_limits`. `rate_limits` holds `five_hour` and `seven_day` and no other
window; each holds `used_percentage` and `resets_at`, both JSON integers.

Hygiene read of both: no `/home/` text, no whole-word OS user name, no email-shaped token. **Surfaced:** both carry
`workspace.repo.owner`, whose value is the repository's GitHub owner handle; the record scrub rewrites only the OS user
as a whole word, so the handle stands in both files (no committed fixture held it before this round). Raised for the
operator's word before the pre-CI commit (the run dir's `outcome-2026-10-13-owed-ledger-rows.md`).

## The round homes on the tmpfs backing

Read 2026-10-13T23:24:11Z: `target/e2e-home` is the link to the owner-only tmpfs directory; both round homes stand there,
`viola-record-20261013T232122Z/` and `viola-live-2266945/`, beside the previous chunk's record home. They are left for
the operator's desk and are gone at the next reboot.
