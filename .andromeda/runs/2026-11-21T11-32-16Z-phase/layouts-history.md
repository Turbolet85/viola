@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set up `viola verify`'s output structure: one `[NN/MM]` step line per ledger row, `stamped <ver>  <n> pass  <m> fail` last, a failing-row run exits 1 and still writes the stamp, and verify's fixed `unable:`/`hint:` pairs. `MM` grows as owning chunks land rows. This is the register the 29th row extends (contract/viola-verify-output/step-line, summary, failing-row, exit-1; the verify refusals).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify went to ten rows. The `--help` paragraph says to run verify from a folder you trust in Claude Code. The dirty-recording refusal became `a recorded fixture is not clean: <file> <code>` (codes `home-path` · `absolute-path` · `username` · `email`, never the content), on the founder's live ruling. This matters because the cross-session payload recorded for the fake agent's replay must pass this refusal (contract/viola-verify-refusals/fixture-not-clean), and the probe's sender and Run B start under the trusted-folder constraint.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify went to fourteen rows. Kept: verify never answers the CLI-native external-import dialog, and the probe's own dialogs are answered by its hook, never by verify typing. This is the constraint for a probe that must read "no dialog stood in Run B" without verify typing a key.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — verify went to seventeen rows. The wireframe examples name the one stamped set's CLI version, and the failing example is the chunk's own first record round. This is the precedent for how a landed row updates the counters, the summary and the ledger-order list.

@history_item n=5 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added verify's stopped form. Ctrl-C or TERM before the rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, stdout empty, no summary, no stamp, exit 1. A completed run ends on the summary. The new sender probe must keep this stop behaviour (contract/viola-verify-output/stop, contract/viola-verify-refusals/stopped).

@history_item n=6 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — verify went to eighteen rows. `MM` stopped being a literal and became `LedgerRow::ALL.len()`, so the count carries its rule. This is the precedent for the 28 → 29 count change.

@history_item n=7 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count, `LedgerRow::relied()`'s length, with exit 0 when every relied row passes. The statusline rows were relied only where the status line is wrapped (the founder's cut). The new row is a *relied* row, so `MM` is the count it changes.

@history_item n=8 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify went to 22 relied rows (`resume-session` last), and the failing example was kept summing to the total. This is a precedent for appending a row at the end of ledger order.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify went to 23 relied rows (`agents-join` last), and the Usage row list grew with it. It also shows that a row's step-line words are printed verbatim as `<row id> <row words>`, so the new row needs its own fixed words.

@history_item n=10 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify went to 24 relied rows (`local-command-compact`), with `MM` 24 / 21. This was the last wireframe-era count change before the masters became rows.

@history_item n=11 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified verify's interrupted pair as worded (it had been a planning form). The stop pair's wording is founder-owned and must not move.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — verify checks its home, `ledger/`, `bin/` and `plugin/` before its log opens. A refused stamps write maps to the home pair: stdout empty, no step line, no summary. The `run` / `revive` starts refuse a widened home tree before the wrapper log opens, exit 1, no child. These are contract/viola-verify-refusals/home-not-private and contract/run-revive-start/home-tree-refused, which the sender spawn and Run B's start must keep. Trap: opening a role file narrows an existing home to 0700, so the check order matters.

@history_item n=13 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the no-hint exceptions are the `unknown` refusal (detail a code word or nothing), `wrapper fault` and `internal error`. This is the standing exception set of decision/hint-for-every-cause: any new verify refusal cause takes its own hint.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document under `--json`) — the one catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout under `--json` instead of `error: internal error`. This is the origin of contract/cli-catch-site/internal-error, the path a failed verify or sender spawn takes.

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS; the .cmd/.bat refusal leaves verify) — the relied set became one Linux set, all rows on every build, with no Windows count. The stop is `SIGINT` / `SIGTERM` unconditional. Verify's exit-1 pairs are now not-found, version, fixture, stop and home, with no `.cmd`/`.bat` pair. The 29th row adds to a single relied count, and the verify refusal set it must respect is that list.

@history_item n=16 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows VT enable leaves the cli colour surface) — the uncoloured conditions became `--json`, a pipe, `NO_COLOR` or `TERM=dumb`. This is the current wording behind decision/plain-output-modes. Verify stays plain ASCII.

@history_item n=17 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — defined the styled vs plain conditions (`--json`, a pipe, `NO_COLOR`, `TERM=dumb`) behind decision/plain-output-modes. It is background to the run / revive passthrough and verify's plain ASCII rule (contract/viola-verify-output/plain-ascii).

@history_item n=18 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa to a stub, leaving the cli surface to stand alone. This is the root of absence/web-surface: the chunk adds no page.

@history_item n=19 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census: no web surface (absence/web-surface), no interactive prompt or pager in a verb, Linux the one target. These constraints hold for verify's new probe output.

@history_item n=20 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (226). The wireframe-era verify counters and examples were dropped for code-backed rows (`relied: 45 kept · 12 cut`). This is why the 29th row changes no `[NN/MM]` literal in the master: the step-line, summary and exit rows state the rule, and the count lives in code.

@history_item n=21 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli: 6 row(s), 1 of 2) — `~ contract/cli-exit-codes/11` changed to retired and never reused. A picked row: no new refusal may map to exit 11.

@history_item n=22 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run` and `~ contract/viola-run/start-output` changed: on a verified CLI the start writes the claude CLI's trust record for an untrusted session folder (flag only, no key typed), and the trust step adds no refusal, exit code, hint, `--json` code or terminal byte. Run B's verify start, with the compiled `crossSessionInbound` accept literal, rests on this.

@history_item n=23 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ …/summary` and `~ …/exit-0` changed: `absent` became a verdict word for the browser-tools-prompt row alone, and exit 0 now means every relied row counts. This was delegate-PROVISIONAL. Trap: the new row cannot borrow `absent` (for example, for an unreached prompt like `no-prompt`); it reads pass or fail.

@history_item n=24 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/viola-run/start-output` changed: before the child, only a key-on start's automatic verify line and verify's own output reach the terminal (discarded under `revive --json`). This is PROVISIONAL (delegate the overseer) until the founder's own word. The new sender would also run in this automatic verify behind `verify_new_cli`, so its output must fit this promise.
