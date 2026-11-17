@history_item n=1 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — set verify's interrupted pair (`unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, no step line, no summary, stdout empty, exit 1), now the rows contract/viola-verify-refusals/interrupted and contract/viola-verify-output/stopped-or-refused. Part B's test kills the tied stamping `viola verify`. A SIGKILL is not the Ctrl-C/TERM this pair covers, so the test must read "nothing left" and must not expect this pair. A TERM-based kill would bind the test to the exact pair.

@history_item n=2 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted pair as his own wording, so the provisional mark is gone. The pair is fixed text this chunk must not reword or loosen while killing a stamping verify.

@history_item n=3 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals) — before its log, `verify` checks its home tree. A refusal there, or a refused `stamps.json` update, prints only the home-state pair and exits 1 (rows contract/viola-verify-output/stamps-refused, contract/viola-verify-refusals/stamps-update, the refused arm of stopped-or-refused). A killed stamping verify must leave no half-written `stamps.json` and no stray process, and its test home must pass the strict-modes check so the run gets as far as the kill.

@history_item n=4 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check) — `run`'s start checks its home tree before any log: home pair or instance pair, exit 1, no child. This is the start path a booted wrapper takes in the e2e boots. A tied `BootOptions` boot must keep this order and these refusals unchanged.

@history_item n=5 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's own code) — a stale holder prints and logs `stale-heartbeat`, and a live holder or fresh replayed beat logs `already-live` (row contract/viola-run-start-refusals/already-live). A frame verb against a stale instance reads `instance-dead`. Part B's proof that a killed supervisor leaves nothing behind must not leave a beat or endpoint that a later boot of the same name reads as `already-live` or `squatted-name`.

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the pending arch block spent, the log's codes and the two carve-outs) — fixed the D-20 log detail codes, `already-live` and `squatted-name` among them, and left the socket-directory cause without a code (owner: "Unix endpoint and home hardening", later closed). This is the code list that rows contract/viola-run-start-refusals/squatted-name and log-detail-codes rest on. No test of this chunk may add a code.

@history_item n=7 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (how a frame verb reads no endpoint and a lost call) — with no live endpoint, a frame verb reads `instance-dead` for an instance directory and `endpoint-lost` for a wrapper lost with the call open. After a test kills a booted wrapper, a verb against it reads one of these causes. A census or assertion that checks "ended with its test" through a verb must expect these words.

@history_item n=8 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive) — added `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`, refused before any snapshot, child or bind (rows contract/viola-run-start-refusals/socket-dir, contract/refusal-hints/socket-dir). The scope puts test sockets under the short `/tmp` root. That directory must be owner-only, or every tied boot is refused at start.

@history_item n=9 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the eight frame verbs; `socket-dir-refused`, exit 21's sixth cause and a log code) — introduced `socket-dir-refused` as exit 21's sixth cause, written to the log by the eight frame verbs and at `run`'s and `revive`'s start (rows contract/exit-21-cause/socket-dir-refused, contract/log-detail-codes/socket-dir-refused). Socket placement in the e2e boots must not trip this cause.

@history_item n=10 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the wrapper's silent drop of a refused peer) — a peer the wrapper refuses is dropped before any frame: no line, nothing on the child's terminal, no wheel moved, and the client reads EOF (row contract/wrapper-socket/refused-peer). This precedent matters for a census or a kill probe that touches a wrapper socket it does not own: no test may send at a process it does not own.

@history_item n=11 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (`revive --json`'s eleventh code) — `socket-dir-refused` joined revive's closed codes, and one code covers a failed create and a refused check. Booted tests that exercise revive's start read this code list, and this chunk adds no code to it.

@history_item n=12 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — the founder confirmed the socket-directory pair's wording and retired the OPEN item. The pair under row contract/viola-run-start-refusals/socket-dir is fixed text.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli Platform narrowed to Linux terminals) — narrowed the Platform to Linux terminals, CI on `ubuntu-latest` alone (row decision/linux-terminal-platform). The tie (`HARNESS_TIED_TEST_CHILD`, `ROOT_TIED_TEST_CHILD`), the census and the kill test need to hold on Linux only. The `supply-chain` job also runs on `ubuntu-latest`.

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal and batch-script-child retired; verify's relied count on every build) — removed `batch-script-child` from `run`'s and `verify`'s refusals and from revive's codes. It also made `LedgerRow::relied()` 24 on every build, with `shim-resolution` passing when `claude --version` answers. These set what a stamping `viola verify` run against the fake agent goes through before the kill. revive's ten codes and verify's five are pinned in `tests/cross_exit_causes.rs`.

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the stale-row colour witness and where it now stands) — the matrix is `[ubuntu-latest]` alone, and the outer-PTY witness `tests/cross_list_rows.rs` is ungated. This is a precedent for booted integration tests that hold a PTY child on the one CI runner. Part B's tied boots run in the same matrix.

@history_item n=16 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — on an unverified CLI a `/compact` stays `unconfirmable`, which ties readback to the stamp (row contract/cli-verification/unverified-cli). The last chunk's harness victim booted unstamped, so it ran transport only as `unverified-cli`. That is why only a stamping `viola verify` kill closes v2-37's narrowing.

@history_item n=17 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows, each verified against the code. Every handle in this chunk's picked rows dates from it. Part A (`deny.toml`) touches no design row, and part B must leave every picked row's promise true, so this chunk lists no design amendment.
