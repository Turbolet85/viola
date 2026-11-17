@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (243 records to 344 rows), each verified against the code. Every picked row (the run-silent decision, the start refusals, the exit codes, the verify refusals) is a row of this migration. A planner must respect the rows and treat the code as holding the rest, so the plan must not change any of these outputs while it adds fixture and process hygiene.

@history_item n=2 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive refusals, the stderr rule, pattern 5) — established that `run` and `revive` print nothing once the child starts, that a passed `revive` is the same passthrough as `run` under the no-colour ban, and that the start refusals use the fixed `unable:` form (none shows a path, a pid or a logged id, and no hint names `viola release`). These are the sources of decision/run-prints-nothing-while-wrapped, contract/viola-run-output/child-started and decision/start-refusals-show-no-path-or-pid. A wrapper end path the chunk adds for when its test is gone (P3) must write nothing to the child's terminal.

@history_item n=3 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check; verify's home-tree and stamps refusals; a home under a regular file; the six frame verbs' instance-state pair) — `run`'s start checks the home, `ledger/`, `bin/`, `plugin/`, the pinned copy and `plugin/<key>/` (each that exists) before any log, with no child. `verify` checks its home tree before its log and refuses a `stamps.json` update with the home-state pair, exit 1. A home that cannot be stat'ed is refused under strict modes. This is the ground the shared stamped home (`tests/support/home.rs`, `harness/boot.rs`) must satisfy when it is rebuilt: a rebuilt home with the wrong modes, or a home path under a dangling link, meets these refusals and not the `unverified-cli` / `NotFound` failures of the CI red.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli Platform narrowed to Linux terminals; strict-modes readings without an OS arm; the .cmd/.bat refusal retired; verify's relied count on every build) — the platform is Linux terminals with CI on `ubuntu-latest` alone (decision/linux-terminal-platform). A home under a regular file is a strict-modes refusal with no OS lead. `LedgerRow::relied()` reads 24 on every build, and `shim-resolution` passes when `claude --version` answers. Process-tree reaping (for example PR_SET_PDEATHSIG, process groups or a `/proc` census) can be Linux-only without a cross-OS arm. The stamped home's verify summary counts every relied row.

@history_item n=5 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's own code; how a frame verb reads no endpoint and a lost call; exit 21's document names its cause; the catch site's `internal-error` document; `revive --json`'s documents; the log's codes) — a stale holder prints and logs `stale-heartbeat`, while a live holder and a fresh replayed beat give `already-live`. With no live endpoint a frame verb reads `instance-dead` or `unwrapped-name`, and a wrapper lost mid-call reads `endpoint-lost`. `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, and a `pump_child` error prints no document. This bears on the chunk because a leftover or spinning wrapper is exactly what yields `already-live` or `stale-heartbeat` in a later test, and the killed-test proof's census reads these causes. A new wrapper end path when the PTY or parent is gone must stay inside the silent catch-site rule.

@history_item n=6 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive; `socket-dir-refused`, exit 21's sixth cause; `revive --json`'s eleventh code; the board's one rule on the home; the wrapper's silent drop of a refused peer) — `run` and `revive` refuse an untrusted socket directory before any snapshot, child or bind, and the log code is `socket-dir-refused`. The board holds the home to the read rule. A refused peer is dropped with no line and nothing on the child's terminal. This constrains the scope's boundary that test sockets go under the short `/tmp` root: that root must pass the socket-directory check (owned, owner-only), and the stamped home must be private to its user.

@history_item n=7 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — the founder confirmed the wording of the socket-directory `unable:` / `hint:` pair (contract/viola-run-start-refusals/socket-dir, contract/refusal-hints/socket-dir). It is fixed text, so tests the chunk adds must assert it byte for byte and must not reword it.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `viola verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only its fixed `unable:` / `hint:` pair, and exits 1 (contract/viola-verify-output/stopped-or-refused). This is the precedent for how a killed process behaves in this product. A fixture that stamps its home by running `verify` and is killed mid-run leaves no stamp, so the chunk's rebuild must not trust a partial marker.

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** (the named unclean-fixture refusal) — `unable: a recorded fixture is not clean: <file> <code>`, with the codes `home-path` · `absolute-path` · `username` · `email` (contract/viola-verify-refusals/unclean-fixture). A rebuilt stamped home, or fake-agent fixtures written under `/tmp` or the home, must not leave home paths or usernames in recorded fixtures, or verify refuses to stamp.

@history_item n=10 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — the origin of `verify`'s step lines, its `stamped <ver>  <n> pass  <m> fail` summary and its exit-1 refusals (`the claude CLI was not found`, `the CLI version could not be read`, each with its own hint). The shared stamped home is produced by this verb. The rows contract/viola-verify-output/summary, cli-not-found and version-unreadable stem from it.

@history_item n=11 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the step counter derives from the relied count (`LedgerRow::relied()`), and the summary's n + m equals it (contract/viola-verify-output/summary). A stamped-home validity check must not hard-code the row count.

@history_item n=12 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** (the exit-21 refusal) — introduced the home-state pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, which the stamped home's rebuild must not trigger (contract/fixed-refusal-pairs/home-state).

@history_item n=13 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ratified the list refusal's fixed `unable:` / `hint:` pairs (the home pair included) as his own wording. Fixed text: tests assert it and do not reword it.

@history_item n=14 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal takes the fixed instance-state pair) — extended the closed list of fixed `unable: <text>` form users and the instance-state pair's printers (contract/cli-refusal-line/fixed-unable-form, contract/cli-stderr/fixed-message-refusal, contract/fixed-refusal-pairs/instance-state). The chunk must not add to these lists.

@history_item n=15 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive --json` start refusals print one exit-1 stdout document `{"v":1,"error":"<code>","detail":null}` with no hint and stderr empty, and `run` prints none (contract/viola-revive-output/json-refusal). This is the shape the revive refusal-code rows rest on.

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail, a code word or nothing; exit 20's closed `detail`; exit 2's fixed stderr lines; the `--json` failures the witness drives) — the sources of the exit-2, exit-14 and exit-20 rows and of decision/no-hint-for-unknown. They are untouched by fixture work but stand as constraints: no new output word may come in through a changed fake agent.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli, 4 rows and 10 rows) — exit 11 is retired and never reused, and a `budget-paused` refusal from an older wrapper reads `unknown`, exit 14 (contract/cli-exit-codes/10-14, contract/cli-exit-codes/11). decision/no-hint-names-viola-release's why now names only the human-typing refusal. The chunk must not reuse exit 11.

@history_item n=18 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (Surface: cli, 2 rows) — the strict-modes check now refuses a symbolic link in place of an instance directory, which reads `strict-modes-failed`. This is a trap for the stamped home, which reaches the runner's `/tmp` through the `target/e2e-home` link: what the strict checks see through that link decides whether a rebuilt home is accepted.

@history_item n=19 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without `color`, so every styled byte is viola's own (decision/plain-output-modes, contract/cli-colour-decision-order/viola-run). Under `viola run` nothing is printed while the child runs, and any change to the wrapper loop must keep it so.
