@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added `+ contract/viola-verify-output/trust-record-names-nothing`: the 23rd relied row writes and removes a trust record in the Claude config file, and no step line, summary or refusal names that file, key or value. This chunk re-reads that very row live and adds per-run first-screen words to the stamp's `measured` and a `--record` witness; neither may name the config file, the record's key or its value in any verify output.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `verify`'s step counter (one static stdout line per ledger row, ending `pass`/`fail`, then the `stamped <ver>  <n> pass  <m> fail` summary) and its exit-1 refusals with their hints; a failing row prints no stderr word. This is the origin of the step-line / summary rows this chunk's S0′ reading is judged by (`contract/viola-verify-output/step-line`, `summary`, `summary-line`, `fail-word`); new instruments must not add lines to this pattern.

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the counter at ten rows; `viola verify --help` gained its one static ASCII paragraph naming no path (run from a folder you trust; an unapproved external CLAUDE.md import blocks the probe); the unclean-fixture refusal became `a recorded fixture is not clean: <file> <code>` with closed codes, never the content. Bears directly: the help paragraph's trust and imports blockers are exactly what the new `trust` / `imports` first-screen words name, and the closed-code, no-content rule is the precedent for keeping closed words, no screen text, in `measured` (`contract/viola-verify-output/help`, `contract/viola-verify-refusals/unclean-fixture`).

@history_item n=4 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the counter at fourteen rows with the four dialog rows. It confirms that row growth leaves the pattern alone (static appended lines, uncoloured `fail`), the same pattern that applies to the floor rows S0′ reads.

@history_item n=5 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the counter at seventeen rows, adding the framing rows and `local-command-clear`. These are among the interactive rows (from `input-box-signature` on) that failed in S0 and that the floor predicate reads in S0′.

@history_item n=6 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1. This matters because the rig's standing stop is Ctrl-C into its own pty or a no-key group kill. A start cut that way yields this pair, not a stamp, and the `V` arm's printing of the six words before teardown has to account for it (`contract/viola-verify-refusals/interrupted`, `contract/viola-verify-output/stopped-or-refused`).

@history_item n=7 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the counter at eighteen rows, derived from `LedgerRow::ALL.len()`. This is where the rule "the count carries its rule" begins, and the later relied count builds on it.

@history_item n=8 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the step line became one per relied row, `MM` being `LedgerRow::relied()`'s count, and the summary's n + m equals `MM`. The floor this chunk measures ("21 rows the 2.1.325 stamp passes") is counted against the relied set, and so is `trust-record` as the 23rd relied row.

@history_item n=9 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the counter at `[01/22]` with `resume-session` relied on every OS. This is a step in the relied-row register that `trust-record` extends.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (the `verify` item) — the counter at `[01/23]` with `agents-join` last in the member list. This is part of the relied register whose ordering the step lines follow.

@history_item n=11 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — 24 rows, the list ending `local-command-compact`. A `/compact` send is read back only on a verified CLI and stays `unconfirmable` on an unverified one. This ties the `verified` / `unverified-cli` state, which a stamp grants, to what sends confirm (`contract/cli-verification/verified`, `unverified-cli`). This chunk does not re-stamp the live home.

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted `unable:` / `hint:` pair as worded, removing its planning-form mark. The pair is fixed wording, and the rig's stop must not reword it.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals; a home under a regular file) — before its log, `verify` checks the home, `ledger/`, `bin/` and `plugin/`. A refusal prints only the home-state pair with no step line or summary, exit 1, and so does a `stamps.json` update refused by its strict-modes check. A home under a regular file is a strict-modes refusal. These matter here because the chunk writes new `measured` data into the stamp and runs verify under the rig's private config/home: a stamps write still passes through the strict-modes check and its fixed pair (`contract/viola-verify-refusals/home-tree-check`, `stamps-update`, `contract/strict-modes/home-under-regular-file`, `contract/fixed-refusal-pairs/home-state`).

@history_item n=14 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the stderr rule) — the fixed `unable: <text>` form is used by the exit-1 refusals of `run`, `revive` and `verify`, each followed by one hint. This is the origin of `contract/cli-stderr/fixed-message-refusal` and `contract/cli-refusal-line/fixed-unable-form`, which any new verify refusal must take.

@history_item n=15 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal) — extended the closed list of fixed-form users (stderr bullet, Streams line) that `contract/cli-stderr/fixed-message-refusal` now holds. It is a precedent: widening that list is an amendment in its own right.

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines; the `--json` failures the witness drives) — `--json` is one stdout document except an oversize `answer` and an unusable `send`. Each `--json` failure prints one document, exits its literal code, leaves stderr empty, and holds no ESC, path or upstream text (guard `tests/cli_error_bodies.rs`). Bears on `decision/json-one-document-on-stdout` and `decision/json-failure-documents-clean`. The new `measured` words and the record witness must hold no path or upstream (screen) text.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document; `revive --json`'s documents) — under `--json`, the catch site prints `{"v":1,"error":"internal-error","detail":null}` with stderr empty, and without `--json` it prints `error: internal error` byte for byte. A `pump_child` error under `revive --json` prints no document. Explains `contract/cli-catch-site/json-internal-error`, `contract/cli-exit-codes/1` and the exception in `decision/json-one-document-on-stdout`. A verify failure reaching the catch site (for example a failed record-witness write) prints this fixed line and nothing more.

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build; the cli Platform narrowed to Linux terminals; the .cmd/.bat refusal retired; strict-modes readings without an OS arm) — `LedgerRow::relied()` became one count on every build, and `shim-resolution` passes when `claude --version` answers. The platform is Linux terminals with CI on `ubuntu-latest`. Verify's `.cmd`/`.bat` refusal is gone, its refusal codes pinned in `tests/cross_exit_causes.rs`. A home under a regular file has no OS lead. These set the relied count and platform this chunk's live start and fake-agent gate stand on (`decision/linux-terminal-platform`).

@history_item n=19 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the spinner / progress-bar / live-redrawing ban now reads "in a verb's output", and "no interactive prompt, no TUI mode and no pager" is scoped to a verb. This explains `decision/no-spinners-in-verb-output`, `decision/no-interactive-prompt-or-pager` and `decision/no-spinner`. Verify's new instruments may add no live progress, and the 52 s case's bound may not show an elapsed counter.

@history_item n=20 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision / contract / absence / name / overview). Every picked row's current wording, its guards (for example `tests/cli_error_bodies.rs`) and its relied marks descend from this conversion, each verified against the code at the time.

@history_item n=21 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (`~ contract/refusal-hints/refusal-line`) — `send` and `answer` print the same hint for the same reason, and `send`'s `unverified-cli` refusal now prints its hint. This is where the `unverified-cli` hint (`run viola verify for this CLI version`, `contract/refusal-hints/unverified-cli`, exit 12) gets its reach. Lifting "unverified" for `trust-record` here changes no CLI's stamp: the live home is not re-stamped.

@history_item n=22 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity; Surface: cli 4 rows; Surface: cli 10 rows) — `~ decision/fixed-phraseology-vocabulary` dropped `budget-paused`. `~ contract/cli-exit-codes/10-14` and `contract/cli-exit-codes/11` mark 11 retired and never reused. `~ decision/no-hint-names-viola-release` keeps only the human-typing reason. These are the current forms of three picked rows: any new word or exit stays inside this closed vocabulary and does not reuse 11.

@history_item n=23 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/send-readback/unconfirmable`, `+ contract/send-readback/mcp`: a `/mcp` send is confirmed by the stdout echo only on a verified CLI with a declared transcript. This is the echo the `local-command-echo` ledger row covers, a row the scope reads in S0′ but excludes from the floor predicate.

@history_item n=24 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without `color`, so `--help` and usage are plain in every mode, and every styled byte is viola's own. This bears on `contract/viola-verify-output/help` (one static ASCII paragraph) and `decision/plain-output-modes`: any help text this chunk touches stays plain.
