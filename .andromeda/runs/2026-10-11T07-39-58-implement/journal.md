# Implement journal — 2026-10-11-the-board-viola-list

One line per long gate run, written before the wait on it.

- 2026-10-11T07:49Z · firing 1 · `gate.py run --plan … ` the whole block (entries 1–13; the three `operator`
  entries print not run), backgrounded. Its longest entry is 13, `pre-push`, timeout 7200 s. The verdict is
  the SUMMARY line, never the notification's exit.
- 2026-10-11T07:52Z · firing 2 · `--only 3` (the full unit suite) after the red of firing 1 (a dangling e2e-home link after the 07:18Z reboot), foreground, bound 600 s.
- 2026-10-11T07:55Z · firing 3 · the whole block again on the final tree (tests/cli_list.rs tree() now hashes; the six remove-the-guard runs are restored, SHA-256 read back), backgrounded; entry 13 pre-push bounded 7200 s.
- 2026-10-11T07:57Z · firing 4 · `--only 1,2,5` after tests/cli_list.rs's UNREACHABLE took cfg(unix) (the Windows-target clippy read it dead), foreground.
- 2026-10-11T07:59Z · firing 5 · operator pass: `--only 13` (pre-push) on the final uncommitted tree, before the pre-CI commit, foreground, bound 600 s (the last run read 65 s).
