@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set up `viola verify`'s output: one `[NN/MM]` step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` summary, a failing-row run that exits 1 and still writes the stamp, the usage `viola verify [--record <DIR>] [-- <program> [args…]]`, and verify's fixed `unable:`/`hint:` pairs, the dirty-recording pair among them. This is the surface a clean `viola verify --record` on 2.1.328 prints (contract/viola-verify-output/usage, failing-row, summary).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added verify's `--help` paragraph (run from a trusted folder; an unapproved external CLAUDE.md import blocks the probe) and turned the dirty-recording refusal into `a recorded fixture is not clean: <file> <code>` (`home-path` · `absolute-path` · `username` · `email`, never the content). This is the recorder hygiene the new set must pass: a refused payload is a red, never a scrub by hand (contract/viola-verify-refusals/fixture-not-clean, contract/viola-verify-output/help-paragraph).

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — took verify to 14 rows with the dialog rows. Its Kept note says verify never answers the CLI-native external-import dialog, while the probe's own dialogs are answered by its hook. Re-recording on 2.1.328, which has a new trust-dialog decline text, rests on that split.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — took verify to 17 rows, and made the examples name the one stamped set (2.1.287). This is the precedent that the wireframe's stamped version follows the recorded sets, which the 2.1.328 landing may raise.

@history_item n=5 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — made `/clear` read back by its new-session post-condition on a verified CLI version (contract/viola-send-output/read-back-clear). Whether a version counts as verified depends on the stamp this chunk's recording backs.

@history_item n=6 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added the client-side `control-character` / `empty-text` refusals to `[/ ] unable` with exit 13. These are the client-side causes in contract/viola-send-output/unable, which the recorded send exchanges replay.

@history_item n=7 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added verify's stopped form: Ctrl-C or TERM gives stdout empty, the `verify was stopped before it finished` pair, no summary, no stamp and exit 1 (contract/viola-verify-output/stop, contract/viola-verify-refusals/stopped). This governs how a live `--record` start ends if it is interrupted.

@history_item n=8 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — took verify to 18 rows and made `MM` a rule (`LedgerRow::ALL.len()`) instead of a fixed number. The step counter grows with the ledger, so a new recorded set must cover every relied row.

@history_item n=9 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — took verify to 21 rows and made `MM` the relied count (`LedgerRow::relied()`), with exit 0 when every relied row passes. This explains why the step line counts relied rows (contract/viola-verify-output/step-line).

@history_item n=10 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — took verify to 22 rows (`resume-session`). Its note that the generic local-command wording covers `/compact` unchanged comes before the compact post-condition row.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — took verify to 23 rows with `agents-join` (`claude agents --json`). A 2.1.328 recording must capture that read too.

@history_item n=12 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — set the uncoloured conditions (`--json`, pipe, `NO_COLOR`, `TERM=dumb`), now decision/plain-output-modes. Recorded screens and verify's plain ASCII output follow them.

@history_item n=13 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — defined `viola mcp` as a hidden plugin-run verb whose five tools return the CLI verb's `--json` document less `v`, with `refused: <code>` refusals, `invalid-params`, and no wheel or handoff verbs named. This is the surface the `viola-mcp-tap` re-record (`R4M`) captures (contract/mcp-tools/*, contract/mcp-tool-results/*).

@history_item n=14 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — took verify to 24 rows (`local-command-compact`) and made `/compact` read back by a `session-start` of cause `compact` inside its 40 s window, with `[  ] unconfirmable` on an unverified CLI. This is the current 24-row shape the 2.1.328 stamp holds (contract/viola-send-output/read-back-compact).

@history_item n=15 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted pair as worded, among other marks. The stop wording a recording run can hit is therefore no longer provisional.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — `viola verify` checks its home tree before its log opens and maps a refused stamps write to the home pair (stdout empty, no step line, no summary). `run`/`revive` starts refuse a widened home tree before the log opens. These constrain the rig's private config home used for `R4` / `R4M` (contract/viola-verify-refusals/home-not-private, contract/run-revive-start/home-tree-refused).

@history_item n=17 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS; the .cmd/.bat refusal leaves revive and verify) — made `MM` all 24 rows on every build, ending at `local-command-compact`, with the stop on `SIGINT` / `SIGTERM` only, and dropped verify's `.cmd`/`.bat` pair. The verify refusal set the recording can meet is not-found, version, fixture, stop and home.

@history_item n=18 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master into rows, each verified against the code. Every picked row's handle dates from this conversion, so later row edits are the history this chunk extends.

@history_item n=19 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — changed contract/cli-exit-codes/14 (an older wrapper's `budget-paused` reads `unknown`) and contract/viola-send-output/unable (exit 13 or 10/12/14; 11 retired). These are picked rows the recorded send exchanges must agree with.

@history_item n=20 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — widened contract/viola-send-output/read-back from an exact match to the typed text less a trailing run of space, TAB, CR and LF. The recorded prompt-submitted payloads replay against this match rule.

@history_item n=21 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance on contract/viola-send-output/read-back, with a leading trim waiting on a live reading. The read-back row the recorded set exercises is final.

@history_item n=22 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/viola-send-output/unconfirmable` (`/cost`, `/usage`; `/mcp` with no readable transcript) and `+ contract/viola-send-output/read-back-mcp` (a new stdout tag in the session transcript). This is the transcript stdout-tag reading that C3's `/context` post-condition, half UNREAD on 2.1.328 with the transcript absent, must follow from a readable transcript.

@history_item n=23 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run` and `~ contract/viola-run/start-output`: on a verified CLI the start writes the trust record for an untrusted folder, so no trust dialog stands and no key is typed, adding no refusal, exit, hint or byte. On 2.1.328 the trust prompt shows only with `--add-dir` and gains a decline text, so the recording and the fake agent must keep this row true.

@history_item n=24 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ contract/viola-verify-output/summary` and `~ contract/viola-verify-output/exit-0`: the browser-tools-prompt row may read `absent` and still count. The 2.1.328 recording's step lines and stamp (24/24 on the live home) rest on this; the overseer's confirm of these rows is still PROVISIONAL.

@history_item n=25 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa to a stub, leaving the cli surface alone. This begins the line that ends in absence/web-surface.

@history_item n=26 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census: no web surface (absence/web-surface), Linux the one target, flat verbs with no interactive prompt. The fixture set records only the CLI surface.
