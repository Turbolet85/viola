@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s step counter (one static stdout line per ledger row, then `stamped <ver>  <n> pass  <m> fail`) and its exit-1 refusals (`the claude CLI was not found`, `the CLI version could not be read`, …), each with its own hint. The round stamps the ledger for the CLI it runs with `viola verify` on a test home, so its step lines, summary and refusals are this output (rows contract/viola-verify-output/*, contract/viola-verify-refusals/cli-not-found, version-unreadable).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — replaced the fixed `a recorded payload still holds a path or a username` with the named `unable: a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; closed codes; never the content), on the founder's live ruling, and gave `verify --help` a static paragraph about a trusted folder. CARRY 1 and CARRY 2 record fixtures through `viola verify --record` and its scrub. A record that is not clean refuses in this form (row contract/viola-verify-refusals/unclean-fixture), and a trust dialog bears on the rig-keys fork (2).

@history_item n=3 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — made the step count `MM` the relied count (`LedgerRow::relied()`) and added three statusline rows (`statusline-override` · `statusline-shell` · `statusline-rate-limits`), relied only where the status line was wrapped. Trap: chunk 2026-11-14-status-line-interception-removed retired the status-line wrapping, and no design amendment records the statusline rows leaving the step list or `MM` changing. The plan must read the relied count and step list from the code (`LedgerRow::relied()`), not from this history, before it asserts any verify output or stamp on the test home.

@history_item n=4 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — took verify's step lines to `[NN/24]` with `local-command-compact` last, and made `/compact` read back by its same-session post-condition. This is the last recorded count of verify's rows. The hour-long hold rests on the CLI's ≈55-minute compact (scope boundary), and the round's verify stamps include the compact row.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — made `relied()` 24 on every build, with the statusline rows relied and no OS qualifier, and made `shim-resolution` pass when `claude --version` answers. This is the standing design text the status-line removal leaves stale (see n=3). `shim-resolution` bears on fork (3) (shim 2.1.296 vs PATH `claude` vs the 2.1.302 fixture dir).

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli Platform narrowed to Linux terminals; strict-modes readings without an OS arm) — the Platform became Linux terminals, CI on `ubuntu-latest`, and a home under a regular file became a strict-modes refusal with no OS lead. Row decision/linux-terminal-platform is the basis of v2-03's «wraps the unmodified CLI on Linux». Row contract/strict-modes/home-under-regular-file binds how a tmpfs test home (`target/e2e-home/viola-live-*`) must be laid out.

@history_item n=7 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — gave `revive` its exit-1 fixed-message refusals (one hint per cause, read in a fixed order that stops at the first, none showing a path, pid, recorded directory or logged id, no hint naming `viola release`). It made `revive` print nothing once the child starts, and named a passed `viola revive` as the same plain passthrough as `viola run`. v2-03's «a live or stale name refused» and «no byte of viola's own on the rig's pty» are read against these rules (rows decision/start-refusals-show-no-path-or-pid, decision/no-hint-names-release, contract/viola-revive-output/after-child-start, decision/plain-output-modes).

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added `revive`'s `session-live` pair (`<name>'s session is open in another claude`), read after the logged session and before the recorded directory, in the founder's own wording. It belongs to the fixed refusal order a live wrapped start or revive can meet when a real `claude` already holds the session.

@history_item n=9 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's own code) — gave a stale holder its own printed and logged code `stale-heartbeat`, and kept `already-live` for a live holder and a fresh replayed beat. This is the source of rows contract/viola-run-start-refusals/already-live and stale-heartbeat. v2-03's «a live or stale name refused» proves both pairs and their logged codes.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`revive --json`'s documents; the catch site's `internal-error` document) — every refusal of the start that `revive` shares with `run` prints one exit-1 document `{"v":1,"error":"<code>","detail":null}`. A `pump_child` error prints no document, because the child held the terminal and viola writes nothing past it. `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too. These are rows decision/no-json-after-child-held-terminal, contract/cli-catch-site/silent-verbs and contract/revive-json-refusal-code/*. A live wrapped start must show no viola byte even on an error path.

@history_item n=11 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — first gave `revive` its `--json` refusal documents (`already-live` among the codes, no hint, stderr empty). This is the origin of the revive-json-refusal-code rows the live refusal reading may touch.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal and batch-script-child retired) — `revive --json`'s closed codes became ten (`batch-script-child` gone), and `run`'s and `verify`'s `.cmd`/`.bat` refusals left. This is the current closed code list a live refusal must land in, pinned in `tests/cross_exit_causes.rs`.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check; verify's home-tree and stamps refusals; a home under a regular file) — `run` and `revive`'s start check the home tree (home, `ledger/`, `bin/`, `plugin/`, the pinned copy and its `plugin/<key>/`) before any log. `verify` does the same before its log and on a `stamps.json` update, refusing with the home-state pair and no step line. The rigs' test homes and freshly written plugin must be owner-only, or the live start and verify refuse before they run (rows contract/fixed-refusal-pairs/home-state, contract/viola-verify-refusals/home-tree-check, stamps-update).

@history_item n=14 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive) — `run`'s and `revive`'s start refuse an untrusted socket directory with `unable: the socket directory for <name> cannot be trusted`, before any snapshot, child or bind. A rig's test home on tmpfs must give a socket directory the user owns, readable only by them, or the wrapped start refuses.

@history_item n=15 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — the founder confirmed the socket-directory pair's wording as his own, so the OPEN mark is retired. A live start refusal reads against settled wording.

@history_item n=16 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the wrapper's silent drop of a refused peer) — a peer the wrapper refuses is dropped before any frame, with no line written and nothing on the terminal the child holds. This sits beside v2-03's «no byte of viola's own on the rig's pty» and the `viola-mcp-tap` / harness peers the rigs connect.

@history_item n=17 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `viola verify` stopped by Ctrl-C or TERM before its rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1. The worded pair was ratified by the founder at 2026-10-20-t06-58-31-wrap. Every spawn in the round has a kill deadline, so a verify the rig kills must read this pair (row contract/viola-verify-refusals/interrupted).

@history_item n=18 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — an expired dialog hold prints in `viola wait`'s dialog form, `dialog-expired  <name>  dialog <id>  cursor <n>`, and the time form is now for every other kind without a `dialog_id`. v2-02's live hold near the 3600 s `DIALOG_HOLD_MAX` ends in this line if it expires (row contract/viola-wait-output/dialog-form).

@history_item n=19 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch: `dialog-expired`, `viola plugin install`'s four lines and verify's interrupted pair stand as worded, and served-verbs-only `--help` and the `host_size()` width stand as built. This is a precedent for founder-owned PROVISIONAL marks settled at a batch, as fork (4) settles the 3600 s ceiling's PROVISIONAL mark.

@history_item n=20 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `viola plugin install` prints four stdout lines in one write (`viola@viola  written  <dir>` …) and refuses at exit 21 with the home-state pair. v2-03 runs the pinned copy through a freshly written plugin by absolute path, so a rig that writes it meets these lines and this refusal.

@history_item n=21 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `mcp` is a hidden verb, run by the plugin and never by a person, and in no `--help` row. Its stdout carries MCP frames only. CARRY 1 re-records `Mcp.tools-list-1.json` through `viola-mcp-tap` on a live `claude` start, and those frames are what it captures (rows decision/mcp-hidden-verb, contract/mcp-verb/stdout).

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail; exit 20's fixed `wrapper fault`) — `error: wrapper fault` and `error: internal error` carry no hint, and their human line holds the code only. `unknown` (exit 14) has no hint, its detail a code word or nothing. Row decision/no-hint-for-faults binds what a live round may see on a fault, and it is how an older wrapper's `budget-paused` reads (n=25).

@history_item n=23 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the spinner / progress-bar / live-redraw ban became «in a verb's output», with no web surface, the design direction the CLI carries. This scopes rows decision/no-spinners-in-verb-output and decision/cli-has-no-motion, which the round's rigs and evidence records read against.

@history_item n=24 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint. This is the current form of a picked row. The `unverified-cli` hint (`run viola verify for this CLI version`) is shared, which matters if the round's CLI is unstamped (row contract/cli-verification/unverified-cli).

@history_item n=25 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity; Surface: cli 4 and 10 rows; Anti-Patterns) — `~ decision/fixed-phraseology-vocabulary` drops `budget-paused`. `~ contract/cli-exit-codes/10-14` and `~ contract/cli-exit-codes/11` retire 11, never reused, so an older wrapper's `budget-paused` reads `unknown` (exit 14). `~ decision/no-hint-names-viola-release` keeps only the human-typing reason. `− decision/budget-as-ruled-figures` and the `release --budget` rows are gone. This chunk is the first live round on the trimmed product: v2-03's start order has no budget step, and the stale `~/.viola/budget.json` on the operator's live home is out of scope. A live reading must find no budget word in any output.
