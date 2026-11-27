@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (amd-xyx6zwjj) created the `viola revive` output structure: a passed start prints nothing of its own, each refusal is exit 1 with one `unable:` and one `hint:` line last on stderr and stdout empty, the named pairs (state-files, run's collision pair, the two no-session pairs, recorded-directory). This is the refusal shape (contract/viola-revive/refusal-shape, the viola-revive-refusals/* rows) item 3's shared revive/adopt prologue must keep byte-identical: a pure move, no pair or ordering changed.

@history_item n=2 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (amd-olqxjncd) added revive's fifth pair (session open in another claude, `--fork` hint) and fixed the preflight order: holder, logged session, `session-live` (skipped under `--fork`, a failed read passes), then the recorded directory. This is contract/viola-revive/preflight-order and session-live-check: item 3's single prologue function must keep this exact order for both revive and adopt.

@history_item n=3 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (amd-x7tvhxi5) gave `viola revive` `--json`: each refusal is one stdout document `{"v":1,"error":"<code>","detail":null}`, exit 1, empty stderr, the pairs byte-identical without `--json`; `--json` became the machine view of every data verb (decision/json-on-every-data-verb). The prologue move (item 3) must keep both forms; tests reading them must stay green unedited.

@history_item n=4 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (amd-kqw36xvm) put the home-tree check (`check_home_tree`, `TreeRefused`) into the `run` and `revive` starts after the state-files check and before the wrapper log opens: the home pair or the instance pair, exit 1, no log, no line, no child, revive `--json` keeping `strict-modes-failed` (contract/run-revive-start/home-tree-refused). This is the "strict-modes check that runs before the log opens" that the scope says adopt.rs copies; the shared prologue (item 3) must keep it ahead of the log open. (amd-gaqvh6mn, same chunk) moved the frame verbs onto `client::refuse_untrusted`: the precedent for merging copied strict-modes prologues into one call without changing behaviour.

@history_item n=5 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (amd-sgeusjmd) the recorded TRAP for item 3: opening a role file narrows an existing home to 0700, so a strict-modes check placed after `client::start` passes a 0770 home; `link_verbs_refuse_a_widened_instance_before_any_log` holds the order by reading the mode after the refusal. A shared prologue that moves the check even one step later would silently pass widened homes. (amd-o2qcxh5r) also extended the shared `unable` word column and the exit-21 hints (decision/shared-refusal-word-column).

@history_item n=6 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — four entries. (amd-7aelkpd3) exit 21's `--json` document carries its cause in `detail` (`strict-modes-failed`, `server-verify-failed`, …); this is the `--json` refusal shape of `viola open` / `viola up` that item 7's one `refused()` helper must keep unchanged (contract/frame-verb-hints/server-verify-failed, cli-exit-codes/json-form). (amd-ya3sjrab) under `--json` the catch site prints `{"v":1,"error":"internal-error","detail":null}`, and a `revive` start stays outside the catch site. (amd-k3l2xgun) `stale-heartbeat` beside `already-live`. (amd-zap6ccjp) revive's post-preflight start refusals shared with `run` print one exit-1 document (`pinned-hash-mismatch`, `squatted-name`, `internal-error`), and a `pump_child` error prints no document because the child has held the terminal: a constraint on where the prologue and run.rs's split (items 3, 6) may put the document emission. (amd-gfaq5lrd) gave every exit-21 cause its own hint (decision/hint-for-every-cause).

@history_item n=7 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (amd-2xh3er6v) added the socket-directory refusal of the `run` / `revive` starts, before any snapshot, child or bind, and its `socket-dir-refused` code (contract/run-revive-start/socket-dir-refused, run-revive-start-json/socket-dir-refused). `refuse_socket_dir` is shared start code that items 3 and 6 (the run.rs split) must keep in place and in order. (amd-lnl4f7mx) made `socket-dir-refused` exit 21's sixth cause at the frame verbs.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (amd-5fsmeey3) retired `batch-script-child`, so revive's start-refusal codes are `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error` (the run-revive-start-json/* rows). This is the closed set the moved start code must keep. (amd-aucyyl2q) verify's relied count is the same on every build, and the stop is `SIGINT` / `SIGTERM` with no stamp (contract/viola-verify/summary).

@history_item n=9 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — (amd-u7fc44ei) the `run` / `revive` start writes the claude CLI's trust record for an untrusted session folder (revive keyed by the recorded directory). The trust step adds no refusal, exit code, hint, `--json` code or terminal byte (contract/cli-verbs/run, cli-verbs/revive, viola-run/start-output). This is the trust step item 6 moves out of `src/cmd/run.rs`, and the move must keep those zero-output promises.

@history_item n=10 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — (amd-4cufnzt5, ratified by amd-qfze2g56 at 2026-11-22T17-29-41Z-wrap) before the child, only a key-on start's automatic verify line and verify's own output reach the terminal, discarded under `revive --json` (contract/viola-run/start-output). The run.rs split (item 6) is on the boot path and must keep this start sequence. A start that runs a whole verify is also the kind of test that item 1's census sorts by what it runs.

@history_item n=11 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — (amd-365ithl5) added `viola open` (contract/cli-verbs/open) and contract/viola-revive/no-window: the window step reaches only a start that asks for it. This is the window step in `src/cmd/run.rs` that item 6 moves out, and the origin of the `refused()` copy in `src/cmd/open.rs` that item 7 merges.

@history_item n=12 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — three entries, ratified at 2026-11-27T19-07-11Z-wrap (amd-j5lgq34y, amd-eontpln3). (amd-knq2hlrh) revive's hidden `--origin <cli|agent>` places the revived window (contract/viola-revive/no-window, as picked). (amd-lnsh5nlv) added `viola up` (contract/cli-verbs/up), its board refusal printed as the fixed state pair under exit 21 (contract/cli-exit-codes/21), and `viola up` in decision/board-vs-strip. `src/cmd/up.rs` (the `upped` split, item 6) and its `refused()` copy (item 7) come from here, and the exit-21 and `--json` refusal shapes they must keep are stated here. (amd-5j7uefdh) the help row.

@history_item n=13 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — (amd-3cnosfdq) added `viola adopt` (contract/cli-verbs/adopt): it resumes a plain session under viola with no window, logging as process run. (amd-5xd4auob) removed contract/cli-catch-site/internal-error because adopt's start is silent at the catch site, as revive's start is. This is the verb whose copied prologue item 3 merges with revive's: both starts stay outside the catch site, and the merged function must not route either through it.

@history_item n=14 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — (amd-jvczcqdf, ratified by amd-enexpeem) contract/cli-verbs/run now says the start prints nothing at a switch's respawn of the child. The restart path in `crates/viola-pty/src/pump.rs` (item 2's survivors, item 6's restart split) serves that respawn, so the split and the tests must keep it byte-silent. (amd-ontj7dmw, ratified by amd-cfh5u6ry) added the switch verb rows.

@history_item n=15 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — (amd-4bd6dbuu, ratified by amd-kxo2a4wf) revive gained `--model` / `--effort`, conflicting with `--list`, plus contract/run-revive-usage/pair-after-dashes, a run/revive start refusal. This is start-argument handling the shared revive/adopt prologue (item 3) and the run.rs split (item 6) sit next to.

@history_item n=16 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — (amd-vke7brhd, ratified by amd-7audx42w) contract/viola-verify-output/failing-row: a run whose first screen read is trust or imports is a refusal before any row is read and writes no stamp, while a failing row still stamps. It also added the untrusted-folder verify refusal. This is the latest change to the window step and the verify output. Item 1's `path4_…` test runs a whole verify on a custom-stamped home, and the `verify_window_` nextest override class comes from this area's tests.

@history_item n=17 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — (amd-zvyctfje, ratified by amd-w4ehwvp5) verify's step word gained `absent`, the summary's `n pass` counts an absent browser-tools-prompt row, and exit 0 means every relied row counts (contract/viola-verify-output/summary). This is the output a whole-verify test (item 1's `path4_…` and its census) asserts on.

@history_item n=18 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — (amd-aoiaah7h) the summary closes only a completed run; a stopped run prints the stop pair, no summary and no stamp, exit 1 (contract/viola-verify/summary, viola-verify-output/exit-1). Item 1's kill-deadline move must not let a killed whole-verify test be confused with this stop path.

@history_item n=19 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (amd-dw2whv7w) the origin of the verify wireframe: the failing-row run exits 1 with the stamp still written, plus verify's exit-1 refusal pairs (contract/viola-verify-output/failing-row, exit-1).

@history_item n=20 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — (amd-cvedhza4, ratified by amd-rc3osznz) contract/cli-exit-codes/21 names the strict-modes refusal of `viola focus` beside the frame verbs, printed as the fixed state pair. This is the exit-21 row that the shared `refused()` helper (item 7) must leave as it reads.

@history_item n=21 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (amd-36smu37h) the server-verification cause of exit 21 and its hint, which writes no frame (contract/frame-verb-hints/server-verify-failed). It is a frame-verb refusal and hint form that a shared refusal helper must not change.

@history_item n=22 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — (amd-kg4e7ksk) the exit-21 pair `unable  <name>  instance-unreachable` with its `hint:`, one `--json` document, and wait/last joining the shared `unable` word column (decision/shared-refusal-word-column, decision/hint-last-on-stderr, contract/cli-json-mode/one-document).

@history_item n=23 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (amd-gtjpdkbm) the `unknown` refusal's detail is a code word or nothing. `unknown`, `wrapper fault` and `internal error` still take no hint (the except clause of decision/hint-for-every-cause).

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (amd-z6yaakhq) reworded decision/board-planning-forms-wording to bare skip counts and the empty board's three lines (the budget forms retired). This is the current text of a picked row. (amd-qpv65sfz at 2026-10-13T06-38-35-wrap made the board's planning forms the founder's wording.)

@history_item n=25 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — (amd-qeiqybgw) styled output only on a terminal; uncoloured under `--json`, a pipe, `NO_COLOR` or `TERM=dumb` (decision/plain-output-modes), and the width read from `viola_pty::host_size()`. Its provisional marks were superseded by amd-464iju6y (2026-10-20T06-58-31-wrap). The moved run/up code must keep the passthrough and `--json` plain.

@history_item n=26 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — (amd-ehm3ecu6) added contract/viola-answer-output/not-delivered-hints beside the picked answer rows (answered, unknown-dialog). (amd-5h2dvmh2 at 2026-10-04-dialog-answers-by-dialog-id put `answer` on the shared catch site.) Item 1's `path4_…` test lives in `tests/cli_answer.rs`.

@history_item n=27 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — (amd-udm5ikh3) turned the master into rows, verified against the code. Every row this chunk is bound by dates from it, and later entries change rows only by handle (`+` / `~` / `−`). The phase writes no row, since the chunk adds no master rule.

@history_item n=28 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — (amd-464iju6y; likewise amd-k3slopis at 2026-11-01-t02-46-44-wrap and amd-wfhmwnjm at 2026-11-16T06-05-33-wrap) the precedent for item 8: a provisional or unresolved mark in this history is superseded by a new dated entry naming the old heading, "none pruned", never edited out by hand. Item 8 says the layouts history holds one mis-dated entry, but this read did not identify which one, and the history shows no `UNRESOLVED` marker text. Headings whose Ref date differs from their marker date (for example 2026-10-14-resume-and-compact-ledger-rows, Ref 2026-10-16T14-05-30-wrap) are candidates only. Any fix is a wrap amendment, not a plan step.

@history_item n=29 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (amd-p7aoh5og; earlier amd-7icpjupk at 2026-10-16T14-58-10-wrap) no web surface; viola renders no page (absence/web-surface). Flat verbs have no interactive prompt and no pager. The chunk's moves add no surface.
