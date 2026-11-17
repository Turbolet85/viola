@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — landed `viola verify`'s output structure: one step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` last stdout line, a failing-row run that exits 1 and still writes the stamp, the usage `viola verify [--record <DIR>] [-- <program> [args…]]`, and verify's fixed `unable:`/`hint:` pairs, among them the dirty-recording refusal. This is the base of R4's live `viola verify --record` reading (contract/viola-verify-output/usage, /failing-row, /summary, /record).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added the `viola verify --help` paragraph: run it from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe. Also named the fixture refusal `a recorded fixture is not clean: <file> <code>`, with codes `home-path` · `absolute-path` · `username` · `email`, the founder's live ruling. Bears directly on the external-imports modal reading (contract/viola-verify-output/help-paragraph) and on R4's record-home rule, since a home with a `home`-named component trips `home-path` (contract/viola-verify-refusals/fixture-not-clean).

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** (its Kept clause) — the `--help` sentence "verify never answers it" is about the CLI-native external-import dialog, which verify never answers; the probe's own dialogs are answered by its hook. A precedent for the imports-modal reading: no key is typed into that modal, which fits the scope's rule that any key into a CLI-native modal is the founder's.

@history_item n=4 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped before its rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, with no summary and no stamp, exit 1. The summary closes only a completed run (contract/viola-verify/summary, contract/viola-verify-output/exit-1). A live `--record` run the rig ends early must read this form, not a summary.

@history_item n=5 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified verify's interrupted pair, as worded, along with other delegate marks. A live verify reading on the rig meets settled wording, not a planning form.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count (`LedgerRow::relied()`): one step line per relied row, exit 0 when every relied row passes (contract/viola-verify-output/step-line, /exit-0). R4's live stamping of 2.1.326 is read against this rule.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the verify wireframe went to 24 relied rows (`[NN/24]`, ending `local-command-compact`), and `/compact` is read back by its compacted-session post-condition. This is the last row-count change, so a live `--record` run should print 24 step lines. The scope builds no ledger row (that is v2-36).

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS; the .cmd/.bat refusal leaves revive and verify) — `MM` is all 24 rows on every build, and the stop is `SIGINT` / `SIGTERM` only. Verify's exit-1 pairs are now not-found, version, fixture, stop and home, with no `.cmd`/`.bat` pair. This sets which refusals a live verify can print (contract/viola-verify-refusals/cli-not-found, /version-unreadable). The same entry dropped the Windows VT enable, so the plain-output conditions are `--json`, piped, `NO_COLOR` and `TERM=dumb` (decision/plain-output-modes).

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — verify, and the run and revive starts, refuse a widened home, `ledger/`, `bin/`, `plugin/` or pinned copy before the log opens. They print the home pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions`, exit 1, no step line, no summary, no child (contract/viola-verify-refusals/home-not-private, contract/run-revive-start/home-tree-refused). R4's record home and the rig's private viola home must be owner-only, or the reading hits this refusal. A trap: opening a role file narrows a home to 0700, so where the check sits matters.

@history_item n=10 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — landed the `viola wait` lines: every non-dialog kind takes the `turn-ended` form, the exit-21 pair `unable  <name>  instance-unreachable` with its `hint:`, and `waiting:` only on a terminal stderr. R3b reads `turn-ended`'s `scheduled_wakes` live through `viola wait` (contract/viola-wait-output/turn-ended, contract/cli-verbs/wait).

@history_item n=11 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `session-end` annotation now reads "every kind without a dialog_id takes the turn-ended form" (was "every non-dialog kind"), and the `dialog-expired` line was added. This is the current wording of contract/viola-wait-output/turn-ended that R3b's live wake reading relies on.

@history_item n=12 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — a precedent for live readings: the `input-not-ready` hint now follows the shipped text, `hint: builder was not ready for input; send again, and if it repeats a human must look at the session` (was `viola wait builder, then send again`). R3c's hook-latency window is the race behind this outcome (contract/viola-send-output/input-not-ready-hint).

@history_item n=13 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses two texts itself before any frame (`control-character`, `empty-text`), each `[/ ] unable`, exit 13. Its Kept clause: whether the issue line prints on a client-side refusal was never measured. This is the base for contract/viola-send-output/unable and a live reading the rig could take.

@history_item n=14 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the read-back line got a second trigger, confirmation by a post-condition for `/clear` on a verified CLI. This is the history of contract/viola-send-output/read-back, which the rig's sends read.

@history_item n=15 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/viola-send-output/read-back`: the confirmation widened to also accept the typed text less a trailing run of space, TAB, CR and LF, marked PROVISIONAL. Live sends on the rig are confirmed under this rule.

@history_item n=16 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance (`~ contract/viola-send-output/read-back`, mark removed). A leading trim ("either end") waits for the live reading carried on «Standing live rig with a screen model». This chunk's live readings are on that rig, and any measured leading difference feeds that owed decision. Do not widen the row here.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli, 1 of 2) — `~ contract/cli-exit-codes/14`: an older wrapper's `budget-paused` reply reads as `unknown`. `~ contract/viola-send-output/unable`: exit 13 (or 10/12/14), with 11 retired and never reused. This fixes the exit set a live send on the rig can return.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the run/revive start socket-directory refusal; exit 21's sixth cause `socket-dir-refused`) — a `viola run` start refuses an untrusted socket directory before any snapshot, child or bind, exit 1, the directory left as found. Frame verbs meeting it exit 21 with the socket-dir hint, or `detail:"socket-dir-refused"` under `--json` (contract/run-revive-start/socket-dir-refused, contract/frame-verb-hints/socket-dir-refused, contract/instance-unreachable-detail/socket-dir-refused). The rig's scratch root and private dirs must pass this check when they start a wrapper.

@history_item n=19 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census: there is no web surface (absence/web-surface), and the "on Unix" qualifiers on both socket-directory refusals were dropped. The CLI ban on interactive verbs is kept, scoped to verbs. This is the current form of the socket-dir rows and of the web absence.

@history_item n=20 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — Surface: web-spa was cut to a stub and the cli surface stands alone. This precedes absence/web-surface.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`; the unwrapped-name and endpoint-lost hints) — exit 21's `--json` document now carries `detail:"<cause>"`. The endpoint-lost hint `hint: <name>'s wrapper went away during the call; viola list shows whether it is still running` is the last stderr line, and every exit-21 cause has its own hint (contract/frame-verb-hints/wrapper-went-away, contract/cli-exit-codes/21, decision/hint-for-every-cause). A wrapper the rig stops mid-call reads this cause.

@history_item n=22 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink` join the exit-21 `unable` lines) — the not-running hint `hint: <name> is not running; viola list shows the live instances` covers `wait`, `last`, `link` and `unlink`, plus `send` in its mirror (contract/frame-verb-hints/instance-not-running, contract/cli-refusals/instance-unreachable). A `viola wait` taken after the rig's wrapper stops (R3b stops the wrapper before the wake is due) prints this pair.

@history_item n=23 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — the not-running hint stopped being exit 21's only hint. A failed server check writes no frame and exits 21 with its own hint. This explains why each exit-21 cause has its own hint (decision/hint-for-every-cause).

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing, and `unknown`, `wrapper fault` and `internal error` still take no hint. This is the except clause of decision/hint-for-every-cause.

@history_item n=25 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a start that passes its preflight prints nothing of its own while the child holds the terminal, and the plain-output list names the `viola run` / `viola revive` passthrough. This is the base for contract/viola-run/start-output, contract/cli-verbs/run and decision/plain-output-modes, which the rig's `viola run` starts rely on.

@history_item n=26 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (226), each verified against the code. Every picked row's handle dates from this conversion, and later changes to it are the `~` entries above.
