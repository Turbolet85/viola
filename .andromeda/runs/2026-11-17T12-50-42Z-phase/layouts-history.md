@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — landed `viola verify`'s usage `viola verify [--record <DIR>] [-- <program> [args…]]` and its fixed `unable:`/`hint:` exit-1 pairs, the unreadable-version pair included. This is where contract/viola-verify-output/usage and contract/viola-verify-refusals/version-unreadable come from. A test that kills the tied stamping verify must leave these refusal and usage forms as they are.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — gave `viola verify --help` its static paragraph and added the named fixture refusal (the `--record` path). It is a precedent: verify's output forms and the recorded-fixture hygiene are pinned byte for byte, and a new kill test of a recording verify must not produce an unclean fixture.

@history_item n=3 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C or TERM prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1, with no summary and no stamp. It bears on part B: killing the tied stamping verify is a different path (SIGKILL, which no handler sees). The proof should read "nothing left running" and must not assume the interrupted pair or a stamp is written.

@history_item n=4 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted pair as worded (provisional mark removed). Its wording is now fixed, so a test in this chunk must not re-word or rely on changing it.

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — added verify's home pair: a refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives exit 1 with stdout empty, no step line and no summary. This is the source of contract/viola-verify-refusals/home-not-private. Trap: opening a role file narrows the home to 0700, so where a check sits matters. Any test verify booted under a test home has to pass this check, which means an owner-only temp home.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS; the .cmd/.bat refusal leaves revive and verify) — verify's stop is now plain `SIGINT` / `SIGTERM`, with the Windows console-control arm removed, and its exit-1 pairs are now not-found, version, fixture, stop and home, with no `.cmd`/`.bat` pair. This is the current list of the verify-refusals rows, and the Linux-only signal model that a kill test of verify works within.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the latest change to verify's Usage: the row list ends at `local-command-compact`, `MM` is 24, and the example is `[NN/24]`. contract/viola-verify-output/usage rests on it. A test reading verify's step lines must count them by `LedgerRow::relied()`, not by a literal.

@history_item n=8 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the `run` and `revive` starts; exit 21's sixth cause, `socket-dir-refused`, at the frame verbs) — added the hint `viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` and the `--json` detail `socket-dir-refused`. These are the sources of contract/frame-verb-hints/socket-dir-refused and contract/instance-unreachable-detail/socket-dir-refused. The scope puts a test socket under the short `/tmp` root, and that directory must be owner-only or the booted wrapper refuses it.

@history_item n=9 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`) — set the `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` form and its closed cause list, which `socket-dir-refused` later extended. It is the register that contract/instance-unreachable-detail/socket-dir-refused belongs to. This chunk adds no cause.

@history_item n=10 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — set the pattern for frame-verb exit-21 hints: no frame is written, the cause's hint is the last stderr line, and under `--json` the document goes out with stderr empty. The socket-dir-refused hint row follows this pattern. It also explains why booted test wrappers must stay the recorded wrapper processes.

@history_item n=11 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — dropped the «on Unix» qualifiers on the socket-directory refusals, so both stand unscoped, and recorded absence/web-surface (no web surface). This is the current wording of the socket-dir rows this chunk rests on.

@history_item n=12 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut the web surface and re-stated the cli surface standing alone, board vs strip included. It explains decision/board-vs-strip and the web absence among the picked rows.

@history_item n=13 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — served `viola plugin install`, a verb that names no strip. It is a precedent behind decision/board-vs-strip, where `verify` and `plugin install` are the board-level setup verbs. A tied verify needs no `ViolaName`.

@history_item n=14 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master into rows, each verified against the code. Every picked row (board-vs-strip, the socket-dir hint and detail, verify usage, the home-not-private and version-unreadable refusals) dates from this conversion. Changing one is the wrap's job as an amendment: this chunk edits no master.
