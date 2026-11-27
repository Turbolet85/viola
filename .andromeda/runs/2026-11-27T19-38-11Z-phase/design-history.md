@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — landed `viola revive`'s exit-1 refusals as fixed `unable:` / `hint:` pairs read in a fixed order that stops at the first (instance-state pair, `run`'s already-live / stale pairs unchanged, no-session, cwd-missing), none showing a path, pid or logged id, and no hint naming `viola release`. Item 3's shared revive/adopt prologue is a pure move: it must keep this order and these pairs byte for byte (contract/viola-revive-refusals/order, contract/viola-revive-refusals/cwd-missing, decision/start-refusals-show-no-path-or-pid).

@history_item n=2 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive --json` its one stdout exit-1 document `{"v":1,"error":"<code>","detail":null}`, no hint, stderr empty, for a refused strict-modes check or a preflight refusal. The shape the moved prologue (item 3) produces under `--json` must stay exactly this (contract/viola-revive-output/json-refusal).

@history_item n=3 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (revive's session-live pair) — added `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`, read after the logged session and before the recorded directory, skipped under `--fork`, a failed read passing. Its place in revive's cause order is part of what item 3's prologue move must preserve (contract/viola-revive-refusals/session-live).

@history_item n=4 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (run's and revive's start refusals at the home-tree check; a home under a regular file) — put the home-tree strict-modes check into `run`'s and `revive`'s start before any log and with no child: home-level → home-state pair, instance-level → instance-state pair, exit 1, `revive --json` → `strict-modes-failed`; a home under a regular file is a strict-modes refusal, not the catch-site line. This is the "strict-modes check that runs before the log opens" item 3 moves into one function shared by `revive` and `adopt`: its position before the log open is the ordering the next audit reads in one place (contract/viola-revive-refusals/home-tree-check, contract/viola-run-start-refusals/home-tree-check, contract/strict-modes/home-under-regular-file, contract/fixed-refusal-pairs/home-state, contract/fixed-refusal-pairs/instance-state).

@history_item n=5 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (revive --json's documents and codes; the catch site's internal-error document; the log's codes and carve-outs; how a frame verb reads no endpoint) — every refusal of the start `revive` shares with `run` prints one exit-1 document with a closed code (`internal-error` for a program not found or an anyhow error of the start), `run` itself printing none; a `pump_child` error prints no document, a document from `main` after the start returned having been weighed and rejected; `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site; `revive`'s strict-modes refusal writes no log line and carries no logged detail code; a frame verb with no live endpoint reads its instance directory after the strict-modes check. Traps for item 3 (the prologue's strict-modes refusal must stay unlogged; anyhow errors must still reach the same document) and item 6 (the pump restart-path move must keep the pump error silent) (contract/viola-revive-json-codes/internal-error, decision/no-json-after-child-held-terminal, contract/cli-catch-site/silent-verbs, contract/viola-revive-refusals/strict-modes-no-log, contract/exit-21-json-cause/instance-dead).

@history_item n=6 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failures the witness drives) — fixed that each CLI `--json` failure prints one document, exits its literal code, leaves stderr empty and holds no ESC, path or upstream text, guarded by `tests/cli_error_bodies.rs`; also the exit-2 fixed-line exception to one-document-on-stdout. Item 7's single `refused()` helper for `open` and `up` must leave these documents unchanged, the guard green unedited (decision/json-failure-documents-clean, decision/json-one-document-on-stdout).

@history_item n=7 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory pair at the start of run and revive; `socket-dir-refused` as a log code; revive --json's eleventh code) — `run`'s and `revive`'s shared start refuse an untrusted socket directory before any snapshot, child or bind with a fixed pair, logged as `socket-dir-refused`, and `revive --json` prints that code for a failed create and a refused check alike. Part of the start ordering item 3's prologue move and item 6's `run.rs` split must keep (contract/viola-run-start-refusals/socket-dir, contract/viola-revive-refusals/socket-dir, contract/viola-revive-json-codes/socket-dir-refused, contract/log-detail-codes/socket-dir-refused).

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal and batch-script-child retired; strict-modes readings without an OS arm; verify's relied count on every build) — `revive`'s closed `--json` codes became ten, pinned in `tests/cross_exit_causes.rs`; strict-modes readings lost their OS arms; `LedgerRow::relied()` reads 24 on every build. The ten-code set is what the moved prologue must still produce (the pinning test green unedited), and the relied count is what the whole `viola verify` inside `path4_…` (item 1) runs through (contract/revive-json-refusal-code/*, contract/viola-verify-output/step-line).

@history_item n=9 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — `adopt` joined `run`'s start and pump as a silent role: its start refusals in the fixed pair form read in a fixed order, its own exit-1 `--json` row, the `session-live` pair, silent at the catch site, no document from a pump error under `--json`, nothing printed while wrapped. These rows came in when `adopt.rs` copied `revive.rs`'s prologue; item 3 makes that copy one function and must keep every one of them (contract/cli-start-refusal/fixed-message, contract/cli-exit-codes/1-adopt-start, contract/viola-adopt-refusals/session-live, decision/silent-catch-sites, decision/no-json-after-child-held-terminal, decision/run-prints-nothing-while-wrapped, decision/start-refusals-show-no-path-or-pid).

@history_item n=10 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** — named `adopt`'s not-resumed end as the one exception to "nothing past the child": it writes its fixed pair or `--json` document after the child released the terminal, exit 1, a resumed child exiting 0; `run` and `revive` still write nothing past the child. A trap for item 3's shared prologue: the exception is `adopt`'s alone and must not leak into `revive` (contract/viola-run-output/child-started, contract/cli-exit-codes/1-adopt-start, decision/no-json-after-child-held-terminal).

@history_item n=11 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — landed `viola open`'s exit-1 refusals (fixed pairs, one `{"v":1,"error":"<code>","detail":null}` document each under `--json`) and the eight closed placement words the window record carries. Item 7's `refused()` (`src/cmd/open.rs:195`) is the printer of these refusals, and item 6's window-step move out of `run.rs` carries the placement words (contract/cli-exit-codes/1, contract/cli-refusal-line/fixed-unable-form, contract/placement-word/*).

@history_item n=12 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — landed `viola up`: its exit-1 fixed pairs and `boot-time-unread` pair, its nine closed outcome words, line order, `--dry-run`, the not-live hint, the `<name>  live  <placement>` line and `--json` document, exit-21 `strict-modes-failed`, and `up` acting on the whole fleet. Item 6 splits `upped` out of `src/cmd/up.rs` and item 7 folds its byte-identical `refused()` into `open`'s: all these wordings and documents must stay as they are (contract/cli-verb-output/up, contract/viola-up-outcome/*, contract/viola-up-output/*, contract/fixed-refusal-pairs/boot-time-unread, decision/list-is-the-board, contract/exit-21-json-cause/strict-modes-failed).

@history_item n=13 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** (ratifications) — the operator's own word ratified every PROVISIONAL mark standing at `a80df0e5`, the `viola up` rows and the `adopt` not-resumed rows among them. The rows items 3, 6 and 7 touch are now standing, not provisional: a pure move must not reopen them.

@history_item n=14 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — added `focus` to the closed exit-1 and exit-21 verb lists of the fixed `unable:` form and of `cli-exit-codes/1` (open's and focus's `--json` refusals), later ratified. It shows the closed verb lists item 7's shared helper sits under; no list changes, since the chunk adds no refusal (contract/cli-exit-codes/1, contract/cli-refusal-line/fixed-unable-form, contract/cli-stderr/fixed-message-refusal).

@history_item n=15 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — brought the fixed-form and exit-21 strict-modes lists to their current text (`switch` added, seven frame verbs). This is the latest wording of rows the chunk rests on and must not change (contract/cli-refusal-line/fixed-unable-form, contract/cli-stderr/fixed-message-refusal, contract/exit-21-json-cause/strict-modes-failed).

@history_item n=16 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — added the five fixed automatic-verify stderr lines a `run` / `revive` start prints (verifying, did not verify, earlier red, verifying elsewhere, no stamp) and their line form (one ASCII write, no SGR, none under `--json`). They live in `run.rs`'s start; item 6's move of the window and trust steps out of `run.rs` must keep each line and its silence under `--json` (contract/auto-verify-line/*).

@history_item n=17 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — an automatic verify refused for an untrusted folder leaves no stamp, prints the no-stamp `unstamped` line and leaves the version unattempted; did-not-verify belongs only to a run that left a non-green stamp. This is the last change to the trust step item 6 moves out of `run.rs` (the commit this chunk follows), so the move must keep this outcome (contract/auto-verify-line/no-stamp, contract/auto-verify-line/did-not-verify).

@history_item n=18 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — ratified the auto-verify lines and verify's `absent` step-line / summary rows, and added decision/permission-request-question-held (a human key, an expired hold or an unverified CLI leave the question to the human). That decision is what `tests/cli_answer.rs`'s `path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human` (item 1) witnesses: re-pointing its nextest kill must leave the test and its assertion unedited.

@history_item n=19 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** (cli-verification rows) — `verified` now needs a stamp reading every relied row `pass`; a stamp lacking a relied row or reading one `fail` reads `unverified-cli`. `path4_…` (item 1) builds exactly such a custom red stamp and runs a whole verify to reach this state, which is why it outruns the `mutants` profile's 15 s kill (contract/cli-verification/verified, contract/cli-verification/unverified-cli).

@history_item n=20 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line gained the `absent` verdict on the `browser-tools-prompt` row, and the summary's `n` counts it as a pass. This is the output of the whole verify that item 1's tests run, and the verify-class kill (`verify_window_`, 20 s × 4) those tests move into (contract/viola-verify-output/step-line, contract/viola-verify-output/summary).

@history_item n=21 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added the trust-record row: verify writes and removes a trust record in the Claude config file, and no output names that file, key or value. This bears on item 6's trust-step move out of `run.rs` and on item 1's verify-running tests, which must use a scratch home and never the operator's real `~/.claude*` (contract/viola-verify-output/trust-record-names-nothing).

@history_item n=22 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** (contract/exit-21-json-cause/unwrapped-name) — a symbolic link in place of an instance directory is refused by the strict-modes check (`strict-modes-failed`) before the unwrapped-name read. The strict-modes-first ordering is what the shared prologue (item 3) moves and must keep.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (decision/no-hint-names-viola-release) — narrowed the rule's why to the human-typing refusal once `budget-paused` was retired. The rule still binds every hint the moved prologue and the shared `refused()` print: none may name `viola release`.
