@history_item n=1 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's step counter (pattern 5) became one line per relied ledger row, `MM` = `LedgerRow::relied()`'s count, and the rows after `paste-newline` were named `statusline-override` · `statusline-shell` · `statusline-rate-limits`. This is where the two status-line rows this chunk removes entered the verify register, so the counter and summary rows (contract/viola-verify-output/step-line, summary, summary-line) drop from 23 to 21 under the same relied() rule, and the named member list loses the two rows.

@history_item n=2 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — `WRAPPED_HERE` was folded and `relied()` became all rows on every build, with the statusline rows relied with no OS qualifier. This is the last design statement of the statusline rows' relied status, and the chunk retires it. The same version's other entries set the removal precedent: the `.cmd`/`.bat` refusal and the closed word `batch-script-child` left the schema with their code, and revive's code count was re-pinned. Use that as the pattern for the `statusline-shell` diag subject and the area words leaving.

@history_item n=3 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the step counter reached `[01/22]` with `resume-session` relied on every OS. It is one of the verify count register's history steps. The chunk's new count of 21 and its member list come after it, and `resume-session` stays.

@history_item n=4 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `agents-join` became the 23rd row, and the `viola list` As built note set the unwrapped separator `-- UNWRAPPED - READ-ONLY --`, the six columns, name order and the `claude agents --json` join. These underlie the picked rows contract/viola-list-table/columns and row-order and decision/bare-strips-never-target. The `agents --json` read is Run B's surviving read that the scope keeps.

@history_item n=5 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify's lines read `[NN/24]`, ending `local-command-compact`, and the summary was `stamped 2.1.287  24 pass  0 fail`. This is the count before budget-gate-removed (23) and this chunk (21). The summary sample and the `[NN/MM]` example in the verify rows must follow.

@history_item n=6 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the step counter was derived from the ledger's member list (`LedgerRow::ALL.len()`, later `relied()`), so "the count carries its rule". The chunk's change from 23 to 21 needs no new design rule, only the samples and member list updated.

@history_item n=7 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — origin of verify's static appended step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` summary, and "a failing row prints no stderr word". These are the picked rows decision/step-counter-only-progress-pattern, one-progress-pattern, contract/viola-verify-output/*, and they stay unchanged in form while two rows leave.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the fixed refusal became the named `a recorded fixture is not clean: <file> <code>` with closed codes and never the content. This is the picked row contract/viola-verify-refusals/unclean-fixture. It applies to the box-only fixture CARRY recorded by `viola verify --record` and to the StatusLine fixtures that leave.

@history_item n=9 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity 3 · Iconography 1 · Surface: cli 4 · Surface: cli 10 · Anti-Patterns 1) — the immediate precedent for a whole-feature removal at the row level: `~` rows reworded (contract/cli-separators/space-hyphen-space's example became `skipped 2 - 0 - 1`, contract/viola-list-table/bay-line now skip counts only, decision/wheel-column-width-23 kept its width without its widest word), `−` rows removed with "none: …" reasons, and exit 11 retired and never reused. It also took the `statusline-rate-limits` row out first, leaving the two this chunk removes. Its row form (`~`/`−` with promise was/now and why) is the shape this chunk's expected amendments take.

@history_item n=10 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision/contract/absence/name/overview), with relied rows kept or cut and each verified against the code. That is the register the picked rows live in and the form the wrap's amendment of the 160+ census rows mapped to v2-05 takes. The name rows (name/command/hook, mcp, run …) come from it.

@history_item n=11 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document · `revive --json`'s documents) — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too (contract/cli-catch-site/silent-verbs, decision/silent-catch-sites). A `pump_child` error prints no document because the child holds the terminal. The removal of the `hook statusline` arm and of run's start-time status-line override must keep `hook`'s and `run`'s silence and exit behaviour intact.

@history_item n=12 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive` prints nothing once the child starts, and a passed `viola revive` is the same no-colour passthrough as `viola run` (contract/viola-revive-output/after-child-start, decision/run-prints-nothing-while-wrapped, decision/plain-output-modes). The run/revive start that loses the settings override and snapshot recording keeps this silence.

@history_item n=13 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `mcp` is a hidden verb that appears in no `--help` row (decision/mcp-hidden-verb, which also says `hook` has no human design surface). Removing the `hook statusline` arm and its hidden `--capture`/`--shell` probe form touches no help row.

@history_item n=14 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour layer (DIALOG amber, stale dim only) and `--help` listing served verbs only. These are behind decision/cli-colour-amber-and-dim-only, plain-output-modes and the help verb-groups rows, which stay unchanged: the status line's removal must not touch the help groups or SGR rules.

@history_item n=15 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:` help row built) — `--help` reached its five groups `board` / `traffic` / `wheel` / `handoff` / `setup`, which are the picked rows contract/help-output/verb-groups and contract/viola-help/verb-groups. `hook` (and its statusline arm) is absent from them, so nothing in the help text moves.

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines) — exit 2 is clap usage text "never from `viola hook`" or the fixed UNUSABLE lines (contract/cli-exit-codes/2). With contract/hook-verb/exit-code (always 0) and stderr (nothing), this is the hook verb's output contract that the remaining hook arms keep after `hook statusline` leaves.

@history_item n=17 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap was built without `color`, so every styled byte is viola's own SGR. This underlies decision/plain-output-modes and the colour decision order rows for `viola hook` and `viola run` (no colour, glyph or SGR). The removal must not reintroduce output on those paths.

@history_item n=18 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: WHEEL padded to 23, bare nonzero skip counts in key order, row escaping, and `list`'s exit-21 refusal. This explains the picked rows decision/wheel-column-width-23, skipped-count-stays-visible and contract/viola-list-board/skip-counts. It matters for the board.rs `"statusline"` literal the scope sends to P3 and for the board rows that must stay unchanged.

@history_item n=19 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder ratified WHEEL 23 and the board's five forms, the bare skip counts among them. The picked board rows stand on his word, so changing them is out of this removal's scope.
