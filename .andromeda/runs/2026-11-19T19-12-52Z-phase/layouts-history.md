@history_item n=1 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/viola-send-output/unconfirmable` (the local-command list grew to six by name: /remote-control, /cost, /usage with no post-condition; /compact, /clear, /mcp unconfirmable on an unverified CLI; /mcp unconfirmable when the session declared no readable transcript) and `+ contract/viola-send-output/read-back-mcp` (/mcp read back when a new stdout tag lands in the transcript read from its pre-send end). The direct precedent: /context takes the same `CommandStdout` post-condition, so both rows must be extended to name /context (read back on a verified CLI; unconfirmable on an unverified one and with no readable transcript).

@history_item n=2 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the read-back line's second trigger: `[RB] read back` also prints when a listed local command is confirmed by its post-condition on a verified CLI (/clear then). Kept: the refusal rungs apply to a listed command as to any text. Origin of the rule /context joins; the outcome row wording must keep the "verified CLI version" gate.

@history_item n=3 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — /compact entered the readback outcome line by its post-condition, with unconfirmable on an unverified CLI, and verify grew to 24 rows with `[24/24] local-command-compact …`, the summary and failing example re-counted, `MM` re-stated. The pattern for a local-command post-condition plus its ledger row landing together, as this chunk's /context row does.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — the first local-command ledger row (`local-command-clear`) joined the verify wireframe with its step-line words, counts and usage list. Precedent for naming a `local-command-*` row and its step-line words for /context.

@history_item n=5 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify at 22 rows; Kept: Signature placement's generic local-command wording covered /compact unchanged. A new listed command may need no new wording where a generic clause already covers it; check before amending.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count (`LedgerRow::relied()`), exit 0 when every relied row passes; a row's relied status changes what verify counts. Bears on the open question of whether the /context row is RELIED (every binary after it reading stamped versions unverified until re-stamped).

@history_item n=7 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `MM` stopped being a literal and took its rule (then `LedgerRow::ALL.len()`); one step line per row in ledger order. A new row extends the order without re-literalising counts.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS) — `MM` is all relied rows on every build, "growing as owning chunks land their rows"; the stop is `SIGINT`/`SIGTERM` with the plain ASCII pair and no stamp. The /context row grows this count on the one Linux target.

@history_item n=9 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line` (verdict `pass|absent|fail`, `absent` the browser-tools-prompt row's alone), `~ contract/viola-verify-output/summary` (`n` counts rows that count), `~ contract/viola-verify-output/exit-0` (every relied row counts), PROVISIONAL delegate overseer. The current verify output rows the /context row's step line and stamp must fit; `absent` is not available to the new row.

@history_item n=10 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/viola-send-output/read-back`: confirmation by a prompt-submitted equal to the typed text or less a trailing whitespace run. A local command submits no prompt; /context confirms by the post-condition path, not this match, so the row stays untouched.

@history_item n=11 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance of `contract/viola-send-output/read-back`; a leading trim waits on a live reading. Current standing of the read-back row beside the post-condition read-backs.

@history_item n=12 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — `viola verify`'s output structure: one step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` summary, a failing run still writing the stamp, `--record`, and its `unable:`/`hint:` pairs. The base of the verify rows the /context probe and live stamp ride.

@history_item n=13 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the named fixture refusal `a recorded fixture is not clean: <file> <code>` (codes `home-path`, `absolute-path`, `username`, `email`, never the content). A trap for recording the /context probe's fixture live: the recording must be clean or verify refuses.

@history_item n=14 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's stopped run: only the `unable: verify was stopped …` / `hint: run viola verify again …` pair, stdout empty, no summary, no stamp, exit 1. Unchanged by a new row, but binds the live stamp run.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — verify's home pair before its log opens or on a refused stamps write; the run/revive start's home-tree refusal. Binds the stamp write and the rig start of this chunk.

@history_item n=16 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses `control-character` and `empty-text` itself before any frame (exit 13, `[/ ] unable`, hint last; one document under `--json`), the wrapper repeating them. These rungs apply to a `/context` text as to any text.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1 of 2) — `~ contract/cli-exit-codes/11` retired, `~ contract/cli-exit-codes/14` (an older wrapper's `budget-paused` reads `unknown`), `~ contract/viola-send-output/unable` (exit 13 or 10/12/14). The current exit set of a /context send's refusals.

@history_item n=18 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp`'s tools are the driver's machine view of the verbs: success the `--json` ok payload, refusal `isError: true` with `refused: <code>`, no tool naming release/pause/link/unlink. The MCP send tool inherits /context's `ok` with `confirmed:false` / `unconfirmable` unchanged.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`; the unwrapped-name and endpoint-lost hints) — exit 21's `--json` `detail` causes and each cause's own hint, `send` in its `[/ ] unable` mirror. Standing refusal surface a /context send passes through unchanged.

@history_item n=20 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows verified against the code; wireframe counts and examples (the verify `[NN/24]` block) are no longer master text, the code holding the rest. Amend the rows (`contract/viola-send-output/*`, `contract/viola-verify-output/*`), not a wireframe count.
