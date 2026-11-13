@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s output structure: one `[NN/MM] <row id> <row words>  pass|fail` step line per ledger row, the last stdout line `stamped <ver>  <n> pass  <m> fail`, a failing-row run exiting 1 with the stamp still written, the usage `viola verify [--record <DIR>] [-- <program> [args…]]`, and `MM` growing as owning chunks land rows. This is the base of contract/viola-verify-output/step-line · summary · failing-row · usage. If this chunk adds or reshapes an input-box ledger row, the count and the wireframe follow this rule, and the proof run on the newest CLI writes its stamp through it.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — landed the screen and timing ledger rows (`modal-signature` … `confirm-window`) that read the screen through screen signatures, the static ASCII `--help` paragraph (run from a trusted folder; an unapproved external CLAUDE.md import blocks the probe), and the named `a recorded fixture is not clean: <file> <code>` refusal (`<file> row <n>[ seam] <code>` for a screen fixture). Precedent for this chunk: a readiness literal in `SIGNATURES` is a ledger row with a verify probe, and new recorded footer-notice screens under `fixtures/claude/<cli-version>/` must pass that cleanliness check (contract/viola-verify-refusals/fixture-not-clean, contract/viola-verify-output/help-paragraph).

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — took verify to 14 rows with the dialog rows. Its Kept says verify never answers the CLI-native external-import dialog: the probe's dialogs are answered by its hook, not by verify typing. That constraint still holds when this chunk changes how the typed run decides the box is up. A modal literal must still block a paste.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — added the typed-run rows `long-paste-wrapper`, `tag-escaping` and `local-command-clear` (17 rows). These are the Run B pastes whose readiness gate this chunk re-keys from a footer literal to the input box. Every one of them has to read pass in the proof. The examples name the one stamped CLI version, so a new stamp updates the examples by the same rule.

@history_item n=5 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the read-back line's second trigger: on a verified CLI version `/clear` is read back by its new-session post-condition. Its Kept: the `viola send` specimen ending `not-delivered  input-not-ready` stays, because the refusal rungs apply to listed commands too. This bears on contract/viola-send-output/read-back-clear: "verified" means a stamp verify writes, and that is the stamp this chunk's proof produces for the newest CLI.

@history_item n=6 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `viola send` specimen's `input-not-ready` hint became `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session` (the founder's wording). This is the user-facing face of a readiness gate (contract/viola-send-output/input-not-ready-hint). The scope keeps the `viola run` pump's readiness gate out unless research shows it shares the footer literal. If it is widened, this hint text must not move.

@history_item n=7 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — set verify's stopped form: Ctrl-C or TERM before the rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, with stdout empty, no summary and no stamp, and exits 1. The summary closes only a completed run. This is a constraint on any new waiting or settle loop in the typed run (`box_wait`, `turn`'s settle): it must stay interruptible with this exact pair (contract/viola-verify-output/stop, contract/viola-verify-refusals/stopped).

@history_item n=8 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — added the `paste-newline` typed-run row (18 rows) and made `MM` carry its rule instead of a literal count (`LedgerRow::ALL.len()`). It is one of the Run B pastes this chunk re-gates. If this chunk adds a row, the count follows from the rule, not from an edited number.

@history_item n=9 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count (`LedgerRow::relied()`). It added the three statusline rows, `statusline-rate-limits` among them, and set exit 0 when every relied row passes. This is the source of contract/viola-verify-output/exit-0 and of the scope's proof exception: every row reads pass except the account-dependent `statusline-rate-limits`.

@history_item n=10 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the verify example block at 22 rows (`resume-session` last). Signature placement's generic local-command wording covers `/compact` unchanged. This is part of the row-count lineage that a new or changed input-box row would extend.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (verify at 23 rows) — added the `agents-join` relied row, `[23/23]`. It is another row the proof run on the newest CLI must read pass, and another step in the verify wireframe count lineage.

@history_item n=12 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added the 24th row, `local-command-compact`, and the `/compact` read-back by a `session-start` of cause `compact` with the session's own id inside its 40 s window. Under it, `[  ] unconfirmable` covers `/compact` and `/clear` on an unverified CLI. The `/compact` paste is in this chunk's Run B. The stamp the proof writes is what moves the newest CLI from `unconfirmable` to read back for `/clear` and `/compact` (contract/viola-send-output/read-back-compact · unconfirmable).

@history_item n=13 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted pair as worded, along with other marks. The stop pair is his wording now, so this chunk must not reword it.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — verify checks its home tree (home, `ledger/`, `bin/`, `plugin/`) before its log opens, and a refused stamps write maps to the home pair with stdout empty and no step line or summary. This is a constraint on the proof: `ledger/stamps.json` is written only by verify, under these checks.

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS; the .cmd/.bat refusal leaves revive and verify) — all 24 rows are relied on every build, ending at `local-command-compact` and growing as owning chunks land rows. The stop is `SIGINT` / `SIGTERM` only. Verify's exit-1 pairs are now not-found, version, fixture, stop and home, with no `.cmd`/`.bat` pair. This is the current shape of the relied set and refusal pairs that the proof run and any new row work within (contract/viola-verify-refusals/*).

@history_item n=16 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — set `viola mcp` as a hidden, plugin-run verb with no human surface. Its tools are the drivers' machine view: each success or refusal is the CLI verb's `--json` document less `v`, with `refused: <code>` and `invalid-params` with a closed field word, and nothing names release, pause, link or unlink. It explains the picked contract/mcp-tools/* and contract/mcp-tool-results/* rows and decision/mcp-server-without-human-surface. It bears on this chunk only if the supply-chain item bumps `rmcp`, the crate this server is built on: a bump past the ratified range must leave these tool results byte-identical.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the no-hint rule names the `unknown` refusal by its detail (a code word or nothing). `unknown`, `wrapper fault` and `internal error` still take no hint. This explains the except clause of the picked decision/hint-for-every-cause: any new refusal cause this chunk adds to verify needs its own hint, as the last stderr line.

@history_item n=18 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (226 rows), verified against the code at 8834195b0683, with the fates by the founder. Every picked row's handle dates from this entry. The prose wireframes, for example the verify example block and its stamped version, are no longer master text, and their divergences were settled "as the code shows". This chunk's wrap amends rows by handle, not prose.
