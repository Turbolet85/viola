@history_item n=1 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — run's pre-spawn stderr carries only the start refusals, two fixed lines through `src/human.rs` `refuse` → `write_refusal`; any verify run by `viola run` before the session must keep run's terminal to that rule (decision/run-silent-terminal, decision/run-silent-on-terminal), since verify itself prints step and summary lines on stdout.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (verify's outcome on stdout) — fixed verify's agent-readable outcome: exit 0/1, one stdout step line per row and the last line `stamped <version>  N pass  N fail`, refusals as the `unable:`/`hint:` pair; an automatic verify inherits this outcome shape (contract/verify-stdout/*, contract/verify-exit-codes/*) and must say where that stdout goes when run starts it.

@history_item n=3 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — verify with `VIOLA_NAME` writes its self pair plus version-probe and verify-probe pairs, while `run`'s version gate keeps one `version-probe` pair; CI verify only against the fake agent, the real-`claude` verify only through the local live run, refused under `CI` (decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci) — a run-triggered verify must not reach a real CLI in CI.

@history_item n=4 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — `parse-rejected{parser:"ledger-stamps"}` gained `strict-modes-failed`: run's version gate reads the stamps through `read_stamps_strict` and a refused, unreadable or malformed file counts as unverified (`cli_verified:false`) at warn (contract/run-log-parse-rejected/ledger-stamps, decision/unreadable-ledger-means-unverified); the trigger for an automatic verify sits on this read and must not treat a refused ledger as a missing stamp.

@history_item n=5 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added `verify-pty-probe` to the closed `subject` enum, each verify spawn logged at the call site; precedent that a new spawn kind (e.g. run spawning verify) takes a closed subject value and a schema change, not an ad-hoc line.

@history_item n=6 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's start arm is role `run`, sharing `run`'s setup and logging into `run-<name>.ndjson` (contract/cli-log-file/revive-start); where the version gate is shared, an automatic verify on revive's start logs as `run`, not as `cli`.

@history_item n=7 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (run.budget_eval in the start chain) — the start's span chain gained a step between bind and the first snapshot, and the start re-read `config.json` for its keys with one `parse-rejected{config-json, malformed}` per bad key; precedent for adding a start step and a config read, and for the must-trace chain (contract/run-start-spans/span-chain) moving only by amendment.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by SIGINT/SIGTERM ends `process-exit{detail:"interrupted"}` after closing every spawn pair, no stamp written; "run's version gate observes no stop" — a verify started by run must say how a Ctrl-C on run's terminal reaches it (contract/verify-process-exit/interrupted, contract/verify-stop-stderr/unable-hint-pair).

@history_item n=9 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify's stdout is one `[NN/MM]` step line per relied row, MM the relied count, the summary summing to it (contract/verify-stdout/step-line); a RED automatic verify that "says which rows failed" reuses these lines.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify gained Run B's `agents-probe` pair through `run::agents::read_logged` (the eighth pair), the same writer `revive` and `list` use; the spawn-pair census an automatic verify's lines would add to the run log (contract/verify-spawn-subject/agents-probe, contract/diag-subject/agents-probe).

@history_item n=11 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the start reads `config.json` a third time for `dialog_hold_s`, one `parse-rejected{config-json, malformed}` on a bad value, `CONFIG_KEYS` widened; that boundary widening was the founder's own answer to a P4 fork — the precedent if a consent/opt-out config key is the founder's form.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree check; run's home-tree check; pre-init window; refusal-telemetry record) — verify and run/revive's start check the home tree before their log opens, a refusal writing no line; `update_stamps` runs `check_stamps` before and under its lock, its refusal leaving only verify's own `process-exit`; a strict-modes failure has a line only on the `ledger-stamps` read and verify's refused stamp write (decision/strict-modes-failure-lines, decision/home-tree-check-before-start-log) — an automatic verify must not add a third site silently.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`verify` writes its refusal's code; the `process-exit` detail enum; Founder Direction 4's codes) — verify's `process-exit` under a parsed `VIOLA_NAME` carries its refusal code (program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture, interrupted), a failing row carrying none; each exit cause has a distinct detail code with no path or pid (contract/process-exit-detail/verify-refusal, decision/distinct-detail-code-per-exit-cause) — a recorded failed attempt or a new start outcome needs a closed word, each added through the schema enum.

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left `diag-line.v1.json`; the `.cmd`/`.bat` refusal retired; the Windows host arms) — `process-exit` `detail` closes at 19 and a closed word no build emits leaves the schema with its emitter; verify's refusal codes lost `batch-script-child`, the `human::refuse(` census reads `run` 7 · `verify` 5; a verify stop reads SIGINT/SIGTERM and the relied count all rows of `ALL` — counts and rules a new verify trigger must re-measure, not copy.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (config keys; exit 11 retired; start chain; additive-only rules narrowed) — `config.json` has four known keys, any other counted in one `parse-rejected{config-json, unknown-keys}` line (contract/config-json/known-keys); exit 11 is retired and never reused (decision/exit-code-log-field); the start chain lost `run.budget_eval`; the additive-only schema rules were narrowed to "never remove a field the product still emits" (decision/diag-schema-additive-only, decision/log-fields-additive-only) — a consent key extends the closed key set and any new outcome must not take exit 11.

@history_item n=16 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — a new spawn (`hyprctl-probe`) joined the closed subject enum with its emitter and a new span area (`hyprland`) joined the closed span-area list (decision/span-naming, contract/diag-line-fields/process-start); the template if run spawning verify takes its own subject or span.

@history_item n=17 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — the span-area list dropped `statusline`; verify's lines carry no row text, prompt, session id, probe command or path (decision/verify-lines-carry-no-content); the hook keeps one uninstrumented capture arm used by verify's probes — rows an automatic verify's lines must keep.

@history_item n=18 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — the start's product order is `wheel{start}` before `session-start` (with `wheel{human-input}` where trust keys pass), read from that start's own lines (contract/run-start-events/order); a verify run before the session must not insert product events into that order.

@history_item n=19 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — first named exceptions to decision/claude-internals-not-a-contract and decision/refusal-telemetry (the transcript echo read); the start of the exception lists any new reach into claude's state by an automatic verify would extend.

@history_item n=20 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify gained Run F (six PTY runs, nine spawn pairs), writes and removes its temp folder's trust record with no line (decision/trust-record-write-uninstrumented), and reads the kept `CLAUDE_CONFIG_DIR` value (decision/r8-strip-logged-as-names-only, decision/env-vars-not-configuration); an automatic verify writes into the user's real claude config through that path — the spend/consent question at P3 rests on it.

@history_item n=21 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — a verified start writes one INFO `trust-record` line from 16 start words and sets its folder's trust flag only; `CLAUDE_CONFIG_DIR` is read by verify and the run and revive start (contract/start-trust-log/trust-record, decision/claude-boundary-only-instrumentation); a start turning verified mid-session or after an automatic verify must say when its trust step and line run.

@history_item n=22 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line reads `pass|absent|fail` and exit 0 holds when the browser-tools row reads absent (contract/verify-exit-codes/0, contract/verify-stdout/summary-line); the green/red verdict an automatic verify reads to go on verified.

@history_item n=23 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — verify's stdout gained a fixed browser-tools line as the only record of a met prompt (decision/browser-tools-prompt-reading-uninstrumented); if run starts verify and discards or redirects its stdout, that record is lost.
