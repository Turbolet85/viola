@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s step counter as one static stdout line per ledger row plus the `stamped <ver>  <n> pass  <m> fail` summary, the count growing "as owning chunks land rows", and a failing row printing no stderr word; this chunk's survey-grammar ledger row adds one step line under exactly this pattern (`contract/viola-verify-output/step-line`, `summary`, `summary-line`).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the counter at ten rows and the named fixture refusal `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`, closed codes, never the content) replacing the fixed payload line; any recorded survey screen under `fixtures/claude/<version>/` must pass this screen check (`contract/viola-verify-refusals/unclean-fixture`).

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the dialog rows landed as ledger rows named with their words, step-line pattern unchanged (static appended lines, uncoloured `fail`); the nearest precedent for a screen-signature ledger row like the survey's, and for the `fail` word staying uncoloured (`contract/viola-verify-output/fail-word`).

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — three rows added, each one static ASCII step line named with its words, the summary example following the one stamped CLI version; the form the survey row's step line and the summary example update follow.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the step counter derives from `LedgerRow::ALL.len()` and the master's row count "carries its rule"; adding the survey row moves the counter and the summary sample, which the master must then restate.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the step lines became one per RELIED row, `MM` = `LedgerRow::relied()`'s count, n + m = `MM`; the survey row must decide whether it is relied, which decides whether `MM` moves (`contract/viola-verify-output/step-line`, `summary-line`).

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22nd row appended at the end of the ledger order with its words, `MM` and the summary sample moved; precedent for where a new row sits and what the master restates.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (the `verify` bullet) the 23rd row `agents-join` appended last, `[01/23]`, the summary sample moved; same row-count precedent.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the 24th row `local-command-compact` with `[NN/24]` and `stamped 2.1.287  24 pass  0 fail`, and a measured post-condition read back only on a verified CLI, `unconfirmable` on an unverified one; the latest counter value this chunk's row increments, and the precedent that a measured behaviour is relied only on a verified CLI (`contract/cli-verification/verified`, `unverified-cli`).

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count on every build) `LedgerRow::relied()` reads 24 on every build, no OS qualifier, `shim-resolution` passing when `claude --version` answers; the survey row lands on a Linux-only, single-relied-set ledger, so the new count has no per-OS split.

@history_item n=11 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1; a new probe (a survey probe that may wait on a live screen) must keep this stopped-run shape (`contract/viola-verify-output/stopped-or-refused`).

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the interrupted-verify pair's wording among his Epoch 5 batch; the pair stands as his word, not a planning form, so a new probe does not reword it.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree and stamps refusals) `verify` checks its home tree before its log and a refused `stamps.json` update prints the home-state `unable:` / `hint:` pair, exit 1, no step line; the survey probe's stamp runs through this path (`contract/viola-verify-output/stamps-refused`, `stopped-or-refused`).

@history_item n=14 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `run` and `revive` print nothing once the child starts (a passed `revive` is the same passthrough as `run` under the no-colour ban), and no hint names `viola release`; the wheel classifier runs inside this silent pump, so the survey path may print nothing to the child's terminal (`contract/viola-run-output/child-started`, `decision/run-prints-nothing-while-wrapped`, `decision/no-hint-names-release`).

@history_item n=15 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive --json`'s refusals became one stdout document `{"v":1,"error":"<code>","detail":null}`, no hint, stderr empty; background for `contract/viola-revive-output/json-refusal`, which a wheel change inside the pump must leave unchanged.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (`revive --json`'s documents; the catch site's `internal-error` document) a `pump_child` error prints no document because the child has held the terminal, and `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too; any new failure in the classifier's screen read inside the pump falls under these rules (`decision/no-json-after-child-held-terminal`, `decision/silent-catch-sites`, `contract/cli-catch-site/silent-verbs`).

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the `unknown` detail; the `--json` failures the witness drives) exit 14's detail is a code word or nothing, and each `--json` failure prints one document, its literal exit code, empty stderr, no ESC, path or upstream text (guard `tests/cli_error_bodies.rs`); bounds any refusal this chunk touches in the 10–14 range and forbids survey screen text reaching output (`decision/json-failure-documents-clean`, `contract/cli-exit-codes/10-14`, `decision/hint-never-quotes-text`).

@history_item n=18 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's word discharged the PROVISIONAL marks of the sanitised-error-surfaces and exit-cause-code-catalogue entries; the catch-site silence and clean-document rules this chunk rests on stand as his word.

@history_item n=19 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the board's WHEEL padding of 23 confirmed by the founder; the WHEEL field prints words (`human` / `driver`), the field the survey key must leave with its holder (`contract/flight-progress-strip/wheel`, `decision/wheel-as-words`).

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — dropped `budget-paused` from the fixed vocabulary (`decision/fixed-phraseology-vocabulary`), retired exit 11 never to be reused (`contract/cli-exit-codes/10-14`), narrowed `decision/no-hint-names-viola-release`'s why to the human-typing refusal reaching drivers, and kept WHEEL at 23 with no `- budget-paused` form; the human-typing refusal (exit 10) is the one a misread survey key wrongly provokes, and its hint must still not name `viola release`.

@history_item n=21 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `contract/refusal-hints/refusal-line` now says `send` and `answer` print the same hint for the same reason or detail (answer's `human-typing` and `unverified-cli` gained send's hints), an opaque or absent `not-delivered` detail printing none; the human-typing and unverified-cli hints this chunk's wheel and ledger work reach are shared across both verbs.

@history_item n=22 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on a measured post-condition as well as on `prompt-submitted`, no new word; precedent that only a measured, ledgered CLI behaviour may stand in for confirmation (`decision/no-optimistic-send-confirmation`, `decision/no-success-before-confirmation`).

@history_item n=23 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (the trailing-only tolerance ratified) the founder kept a CLI behaviour (leading trim) out of the contract until a live reading on «Standing live rig with a screen model» measures it; the same "nothing joins unmeasured" rule this chunk applies to the survey grammar before it joins the closed non-editing list.

@history_item n=24 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — the stdout echo added as a further confirmer, which confirms nothing without a verified CLI (and a declared transcript); the most recent precedent for a newly measured CLI behaviour that counts only on a verified CLI, as the survey grammar must (`contract/cli-verification/verified`, `unverified-cli`).

@history_item n=25 marker=2026-10-01-t12-19-55-wrap
**2026-10-01-t12-19-55-wrap** — irrelevant to the web surface now cut, but it set the rule that relaxing a ban is a boundary widening the founder rules live; this chunk's "no env var, flag or config key widens the list" boundary follows the same rule.

@history_item n=26 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (brand, surface and bans scoped to the CLI) the Brand Identity direction is the one the CLI carries, and the spinner / live-redraw ban reads «in a verb's output»; the brand rows this chunk is bound by (pause/release as control transfer, the transfer marker, WHEEL) are scoped to the CLI (`overview/brand-identity`, `decision/plain-output-modes`).

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the Windows VT enable retired from the colour rules) the colour decision order lost rung 7; the `viola run` rung (no colour, glyph or SGR, nothing printed while the child runs) stands, and the wheel path inside `run` keeps it (`contract/cli-colour-decision-order/viola-run`).

@history_item n=28 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap built without `color`, so every styled byte is viola's own SGR module's; supports `decision/plain-output-modes` under `viola run`, which a new survey path must not break.

@history_item n=29 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code, the code holding the rest; the picked rows (WHEEL, pause/release, verify output, refusal hints) are this conversion's rows, and any change this chunk makes is amended row by row (`+` / `~` / `−` by handle).
