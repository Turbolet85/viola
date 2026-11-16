@history_item n=1 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the read-back line gained its second trigger: a listed local command (`/clear` on a verified CLI) is read back by its post-condition instead of a `prompt-submitted`. This is the direct precedent for a local command's echo confirming its send. It also kept the refusal rungs, such as `input-not-ready`, applying to a listed command as to any text. A new echo trigger extends this same sentence (contract/viola-send-output/read-back, read-back-clear).

@history_item n=2 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` was read back by its compacted-session post-condition inside its own 40 s window, else `not-delivered` / `no-prompt-submitted`. `[  ] unconfirmable …` was fixed for `/remote-control` and for `/compact` / `/clear` on an unverified CLI. Verify went to 24 rows, ending `local-command-compact`. These are exactly the readings this chunk re-measures and may change (contract/viola-send-output/read-back-compact, unconfirmable). A new echo row repeats this pattern of a verify step line plus the count bump.

@history_item n=3 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — added the `local-command-clear` ledger row and verify step line (`/clear starts a new session and submits no prompt`). This is the precedent that a relied local-command behaviour is a ledger row with a verify probe, which is what this chunk must do if it relies on an echo.

@history_item n=4 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify went to 22 rows. The Kept note says Signature placement's generic local-command wording covers `/compact` unchanged. Precedent: a new local-command confirmation may fit the existing generic wording instead of needing a new layout sentence.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS) — `MM` is `LedgerRow::relied()`'s length, all 24 rows on every build, ending at `local-command-compact` and growing as owning chunks land rows. A new echo row moves the verify wireframe to `[NN/25]` and its summary and failing example with it (contract/viola-verify-output/step-line, summary).

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — set the rule that `MM` is the relied count, with one step line per relied row and exit 0 when every relied row passes. A new ledger row must decide whether it is relied, which sets the count verify prints (contract/viola-verify-output/exit-0, step-line).

@history_item n=7 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the count now carries its rule instead of a literal (`MM` from the ledger), and the ledger-order list grows with each row. A new row is appended in ledger order.

@history_item n=8 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — origin of verify's output structure: step lines, the `stamped <ver>  <n> pass  <m> fail` summary, a failing row exiting 1 with the stamp still written, and the `--record` usage. These are the rows a new probe extends (contract/viola-verify-output/*, contract/viola-verify/summary).

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added screen and timing ledger rows and the named `a recorded fixture is not clean: <file> <code>` refusal (a screen fixture is reported as `row <n>[ seam]`). This chunk measures the echo from recorded fixtures and may add a screen-read probe, so the fixture-cleanliness refusal applies to any new recording (contract/viola-verify-refusals/fixture-not-clean).

@history_item n=10 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — the summary closes only a completed run, and a stopped run prints the fixed pair with no summary and no stamp. This bounds the verify output that a new probe joins (contract/viola-verify/summary).

@history_item n=11 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — the `[/ ] unable` line also prints with exit 13 when the client refuses the text before any frame (`control-character`, `empty-text`). Whether the issue line prints on a client-side refusal was left unstated because nothing measured it. This shaped the contract/viola-send-output/unable row that a local command's not-delivered outcome reads through.

@history_item n=12 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli, 1 of 2) — `~ contract/viola-send-output/unable`: the exit set became 13 (or 10/12/14), with exit 11 retired and never reused, and an older wrapper's `budget-paused` reads `unknown` (14) (`~ contract/cli-exit-codes/11`, `/14`). Any send outcome this chunk adds must map into this exit set without reviving 11.

@history_item n=13 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/viola-send-output/read-back`: confirmation widened from exact equality to a `prompt-submitted` whose prompt equals the typed text less a trailing run of space, TAB, CR and LF. The scope keeps this match untouched for ordinary sends. The echo trigger must be added beside it in the same row without disturbing it.

@history_item n=14 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (Surface: cli: 1 row(s); and the tolerance ratified by the founder) — removed the provisional mark from `contract/viola-send-output/read-back` because the founder ratified the trailing-only tolerance, while a leading trim waits on the live reading carried on «Standing live rig with a screen model». The row now stands as founder-ratified, so any edit this chunk makes to it must keep that wording intact.

@history_item n=15 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `viola send` specimen's `input-not-ready` hint became the founder's wording `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session` (contract/viola-send-output/input-not-ready-hint). That hint stays fixed if a local command's send is refused at the readiness gate.

@history_item n=16 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `+ contract/viola-answer-output/not-delivered-hints`: the answer hint table carries the send hints for five more not-delivered details. Precedent: a new or re-worded send not-delivered detail (for example the end of `no-prompt-submitted` for a local command) touches the shared hint table and needs its own hint (decision/hint-for-every-cause).

@history_item n=17 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` tools are the driver's machine view: each result is the CLI verb's `--json` document less `v`, a refusal is `isError: true` with `refused: <code>` and codes only, and no tool names release, pause, link or unlink (contract/mcp-tool-results/*, contract/mcp-tools/send, decision/mcp-hides-wheel-and-handoff-verbs). The MCP `send` outcome of a local command follows the CLI `--json` document unchanged in shape.

@history_item n=18 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `--json` became the machine view of every data verb, `viola revive` included (decision/json-on-every-data-verb). A local command's `ok` / `confirmed:false` outcome stays one typed document under `--json` (contract/send-json/output).

@history_item n=19 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `wait` / `last` lines as landed: every non-dialog kind takes the `turn-ended` form, and `send` keeps its `[/ ] unable` mirror while wait/last share the `unable` word column. This underlies decision/turn-ended-not-send-outcome and contract/viola-wait-output/turn-ended, so a local command's completion must not be reported as a send outcome through `turn-ended`.

@history_item n=20 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `turn-ended` annotation was re-worded to «every kind without a dialog_id takes the turn-ended form» (contract/viola-wait-output/turn-ended). Any new event kind this chunk records for an echo falls under this rule if it wakes `wait`.

@history_item n=21 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing and takes no hint (decision/hint-for-every-cause except clause). Any echo-derived detail must be a closed code word and never upstream text (decision/hints-never-quote-or-locate).

@history_item n=22 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows verified against the code. The picked send, verify and readback rows were set by this conversion, and later changes go through `+ / ~ / −` row amendments such as the send-confirmation `~` entries.
