@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters as rows: the layout text became the rows this chunk is bound by (every picked handle dates from it), each verified against the code; a planner respects the rows and the code holds the rest, so a fixture-only chunk must leave every picked row (run passthrough, start refusals, verify output) byte-identical.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s output structure: the summary `stamped <ver>  <n> pass  <m> fail` as the last stdout line, a failing-row run that exits 1 and still writes the stamp, and the usage `viola verify [--record <DIR>] [-- <program> [args…]]`. These are the rows `contract/viola-verify-output/*`; the shared stamped test home (`stamp_dir` / `stamped_home`) is built by running verify against the fake agent, so its rebuild must still read these lines.

@history_item n=3 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C or TERM before its rows are checked prints only the stop pair, no summary and no stamp, and exits 1; the summary closes only a completed run (row `contract/viola-verify/summary`, `contract/viola-verify-output/exit-1`). This matters for the fixture rebuild: a verify killed or stopped mid-build leaves no stamp, and the fixture must not trust a partial home.

@history_item n=4 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify exits 0 when every relied row passes, `MM` being `LedgerRow::relied()`'s length (row `contract/viola-verify-output/exit-0`). The stamped fixture's validity rests on this exit and count.

@history_item n=5 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the verify wireframe at 24 relied rows (`stamped 2.1.287  24 pass  0 fail`), the current count a rebuilt stamped home must reach; with 2026-10-13-paste-newline-ledger-row's rule that the count follows the ledger, not a fixed number.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop on one OS) all 24 rows relied on every build and the stop is `SIGINT` / `SIGTERM` unconditional; (the .cmd/.bat refusal leaves revive and verify) the start-refusal codes shrink to `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error`; (the Windows VT enable leaves) the uncoloured conditions are `--json`, a pipe, `NO_COLOR`, `TERM=dumb`. Linux-only: the kill/census proof needs no other-OS arm.

@history_item n=7 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `viola run` / `viola revive` start passthrough prints nothing of its own while the child holds the terminal (rows `contract/cli-verbs/run`, `contract/viola-run/start-output`), and the collision pair `unable: <name> is already live` / `hint: viola list` (row `contract/viola-run/already-live`). A wrapper that outlives its test holds its name live, so the next boot of that name meets this refusal: the fix must not change the pair.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (a stale holder's code) a stale holder is refused `stale-heartbeat`, a live one or a fresh beat `already-live`, the collision check a leftover wrapper trips; (the post-preflight gap closed) a start refusal shared with `run` prints one exit-1 document (`pinned-hash-mismatch`, `internal-error` for a program not found or a start error), while a `pump_child` error prints none, the child having held the terminal (rows `contract/run-revive-start-json/internal-error`, `/pinned-hash-mismatch`). Any change to the wrapper's loop once its PTY or parent is gone sits on that pump path and must keep it.

@history_item n=9 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `viola revive` took `--json` and `run` has none; refusal documents `{"v":1,"error":"<code>","detail":null}` exit 1 with stderr empty, the base the start-refusal JSON rows extend; `revive` start stays outside the cli catch site.

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree refusals of verify and the run and revive starts) a refused home, `ledger/`, `bin/`, `plugin/` or pinned copy gives the home pair before the log opens, exit 1, no log, no child; verify's home pair also covers a refused stamps write (rows `contract/run-revive-start/home-tree-refused`, `contract/viola-verify-refusals/home-not-private`). A rebuilt shared stamped home must be owner-only, or every booted test reads this refusal instead of its subject.

@history_item n=11 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the strict-modes refusal of the driven instance) records the trap that opening a role file narrows an existing home to 0700, so a check placed after start passes a widened home; a fixture home's mode read after a refusal is the order guard. Bears on rebuilding the stamped home and asserting its mode.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory refusal of the run and revive starts) exit 1 before any snapshot, child or bind, stdout empty, the directory left as found, `socket-dir-refused` under revive `--json`; (exit 21's sixth cause) a frame verb on a refused socket directory exits 21 with the same hint (rows `contract/run-revive-start/socket-dir-refused`, `contract/run-revive-start-json/socket-dir-refused`, `contract/frame-verb-hints/socket-dir-refused`). A test socket under the short `/tmp` root must sit in a directory these checks accept.

@history_item n=13 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census: drops the «on Unix» qualifiers on the run/revive socket-directory refusal and the frame verbs' refused socket directory (both standing as written), scopes the no-TUI ban to verbs, and states no web surface (`absence/web-surface`).

@history_item n=14 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the uncoloured conditions (`--json`, a pipe, `NO_COLOR`, `TERM=dumb`) and the help usage `viola [--home <dir>] <verb> [args]`, the ground of `decision/plain-output-modes` and `contract/cli-global-flags/home`: the booted fixtures run the wrapper on a PTY under `--home`, and its passthrough prints no colour or cursor control of viola's own.
