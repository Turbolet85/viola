@history_item n=1 marker=amd-i7hubkaz
**amd-i7hubkaz** — (2026-11-24-viola-adopt, 1 of 2) put `adopt` in the help `setup:` row, ~ contract/viola-run-output/child-started (`run`, `revive` and `adopt` print nothing once the child starts), ~ decision/start-refusals-show-no-path-or-pid and ~ contract/cli-refusal-line/fixed-unable-form. Why it matters: adopt hands its child to `run`'s start and pump as a silent role. Any new early-exit line has to fit that silence and must not show the recorded directory, a path, a pid or an id.

@history_item n=2 marker=amd-go4loibi
**amd-go4loibi** — (2026-11-24-viola-adopt, 2 of 2) ~ contract/cli-start-refusal/fixed-message (adopt's causes are read in a fixed order that stops at the first), ~ decision/silent-catch-sites (adopt is silent at the catch site), + contract/cli-exit-codes/1-adopt-start (a row of its own, because `cli-exit-codes/1` has no room left under its bound) and + contract/viola-adopt-refusals/session-live (PROVISIONAL wording, the delegate's W4). This is the register a new adopt closed word, exit and hint would extend, and it sets where a new cause sits in the fixed order.

@history_item n=3 marker=amd-btv6wzpv
**amd-btv6wzpv** — (2026-11-24-viola-adopt) twin rows: ~ contract/cli-stderr/fixed-message-refusal and ~ contract/cli-catch-site/silent-verbs name `adopt`, and ~ decision/no-json-after-child-held-terminal says an error while pumping the child under `viola adopt --json` prints no document. Trap: a child that ends at once after the start is already past `pump_child`, so this rule limits any `--json` document for the early exit.

@history_item n=4 marker=amd-kurg3anp
**amd-kurg3anp** — (2026-11-24-viola-adopt, Brand Identity) ~ decision/run-prints-nothing-while-wrapped now covers a `viola adopt` that passed its preflight. Any line about an early child exit has to be reconciled with this rule, for example printed only after the child has ended or before it holds the terminal.

@history_item n=5 marker=amd-gtarjlax
**amd-gtarjlax** — (2026-10-10-viola-revive) landed revive's fixed-order exit-1 refusals, including `cwd-missing` (`<name>'s recorded directory is missing` -> `resume it by hand from a directory you choose: viola run <name> -- claude --resume <id>, ids from viola revive <name> --list`), no recorded directory, pid or id shown, no hint naming `viola release`, and nothing printed once the child starts. It is the precedent for a resume folder taken from the recorded cwd, which revive has and adopt lacks (scope measurement 2), and for wording a directory-related refusal.

@history_item n=6 marker=amd-5gcswmwj
**amd-5gcswmwj** — (2026-10-16-unwrapped-sessions-on-the-board) added revive's `session-live` pair, read after the logged session and before the recorded directory (skipped under `--fork`; a failed read passes). It is the origin of adopt's session-live pair and of the fixed slot order (session-live before cwd-missing) that a new adopt cause would follow.

@history_item n=7 marker=amd-jfw4i3y5
**amd-jfw4i3y5** — (2026-10-17-cli-machine-contract) revive's `--json` exit-1 document `{"v":1,"error":"<code>","detail":null}` with closed codes (`no-session`, `session-live`, `cwd-missing` …), no hint, stderr empty. This is the shape adopt's `--json` start refusals reuse, and any new adopt code would join it.

@history_item n=8 marker=amd-4rauf2gc
**amd-4rauf2gc** — (2026-10-31-exit-cause-code-catalogue) revive `--json` prints one document for every refusal of the start it shares with `run`; the closed codes became ten; a program that is not found reads `internal-error`; a `pump_child` error prints no document, because the child has held the terminal (a document printed from `main` was weighed and rejected). This is a direct constraint on how an adopted child that exits at once is reported under `--json`.

@history_item n=9 marker=amd-umh66io5
**amd-umh66io5** — (2026-10-31-exit-cause-code-catalogue) D-20's log detail codes became the closed list (`stale-heartbeat`, `squatted-name`, `already-live`, … and, for revive, `no-session` and `cwd-missing`). revive's strict-modes refusal writes no log line. This is the register that a recorded event or log code for an adopt early exit would extend.

@history_item n=10 marker=amd-db4gplsy
**amd-db4gplsy** — (2026-10-31-exit-cause-code-catalogue) the catch site's `internal-error` document under `--json`; `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too. adopt later joined this list (decision/silent-catch-sites). An early-exit report must not go through the catch site.

@history_item n=11 marker=amd-naaf5x6y
**amd-naaf5x6y** — (2026-10-31-exit-cause-code-catalogue) a stale holder prints and logs its own code `stale-heartbeat`, and a live holder `already-live`; a frame verb against a stale instance reads `instance-dead`. It is the precedent for giving a start outcome its own printed and logged code. It also covers the picked rows viola-run-start-refusals/stale-heartbeat and revive-json-refusal-code/stale-heartbeat.

@history_item n=12 marker=amd-nhh7jcfw
**amd-nhh7jcfw** — (2026-10-31-exit-cause-code-catalogue) a frame verb with no live endpoint reads its instance directory: a directory is `instance-dead`, absent is `unwrapped-name`; a failed connect is `instance-dead`. This bears on what `viola list` and the frame verbs read if the adopt instance directory is kept after the child ends at once (the scope asks whether the directory and the name are released).

@history_item n=13 marker=amd-qvu57cdd
**amd-qvu57cdd** — (2026-10-31-exit-cause-code-catalogue) exit 21's `--json` document names its cause in `detail` from a closed set. Its precedent: a code that is shared needs a per-cause word so a driver can tell causes apart. That is the same need as telling «no such session here» from another early exit.

@history_item n=14 marker=amd-4tbohrca
**amd-4tbohrca** — (2026-11-01-t02-46-44-wrap) the founder ratified reading an unwrapped name by its instance directory (and the `endpoint-lost` wording). The picked row contract/exit-21-json-cause/unwrapped-name now stands on his word.

@history_item n=15 marker=amd-w5cbpn6n
**amd-w5cbpn6n** — (2026-10-26-home-and-code-bearing-file-integrity) `run`'s and `revive`'s start check the home tree before any log: a home-level refusal gets the home pair and an instance-level refusal the instance pair, exit 1, no log and no child. adopt starts through the same start, so a new early-exit step comes after these checks.

@history_item n=16 marker=amd-3pr4zv7x
**amd-3pr4zv7x** — (2026-11-01-unix-endpoint-and-home-hardening) added the socket-directory pair at `run`'s and `revive`'s start (before any snapshot, child or bind). Row 1's revive list was folded because the row stood 9 characters under the 2 000-character line limit. Trap: the exit-1 rows are at their length bound, which is why adopt took its own row.

@history_item n=17 marker=amd-io7txdcs
**amd-io7txdcs** — (2026-11-01-unix-endpoint-and-home-hardening) revive `--json` gained an eleventh code, `socket-dir-refused`, after `squatted-name`, with `internal-error` last. It is the precedent for adding a code to a closed start-refusal code list: `internal-error` stays last.

@history_item n=18 marker=amd-luiowdl3
**amd-luiowdl3** — (2026-11-06-version-done-check) the founder confirmed the socket-directory pair's wording and its OPEN item was retired. It is the precedent that a planning-form start-refusal wording stays open until the founder's own word.

@history_item n=19 marker=amd-pa2lbwql
**amd-pa2lbwql** — (2026-11-05-linux-only-code-and-ci) `batch-script-child` was retired: revive's closed `--json` codes went back to ten, pinned in `tests/cross_exit_causes.rs`. Any new start code has to update that pin.

@history_item n=20 marker=amd-qbriq52u
**amd-qbriq52u** — (2026-10-14-resume-and-compact-ledger-rows) added the ledger row `resume-session` («--resume <id> reopens the session under that id»), relied on every OS. This is the row scope measurement 3 must read: whether the ledger says anything about resuming from another folder. It states no folder, so that case is UNMEASURED.

@history_item n=21 marker=amd-c43esfh6
**amd-c43esfh6** — (2026-11-05-linux-only-code-and-ci) `LedgerRow::relied()` is 24 on every build, and the resume row is relied with no OS qualifier. It confirms the resume row's standing when the ledger is read for the resume-folder question.

@history_item n=22 marker=amd-qik42dm3
**amd-qik42dm3** — (2026-11-24-switching-a-session-s-model-and-effort, 1 of 2) ~ contract/cli-exit-codes/13 gained `dialog-pending`, `no-session`, `child-not-ended` and `resume-failed` (PROVISIONAL, fork F3). These are the nearest closed words for a resumed child that fails or ends. Reuse versus a new adopt word has to be weighed against them and the exit-13 register.

@history_item n=23 marker=amd-7l7w76nd
**amd-7l7w76nd** — (2026-11-24-switching-a-session-s-model-and-effort) ~ contract/cli-refusal-line/fixed-unable-form, the current form of the picked row, with `switch` in its exit-21 list. A new adopt refusal is already covered by the row's exit-1 list.

@history_item n=24 marker=amd-m37devjk
**amd-m37devjk** — (2026-11-22-paste-write-bounded) exit 13's closed detail set gained `paste-cut`, with + contract/refusal-hints/not-delivered-paste-cut. Precedent: each new closed detail word gets its own hint row and an edit to the exit-13 enumeration. Its PROVISIONAL marks were ratified in amd-makocpx5.

@history_item n=25 marker=amd-ybx4pz4y
**amd-ybx4pz4y** — (2026-11-24-viola-up, 1 of 2) added `up`'s output, the outcome-word register and the help row (PROVISIONAL, inputs#I8). `up` resumes from the recorded cwd as revive does, which is the comparison scope measurement 2 asks for.

@history_item n=26 marker=amd-c77zejnh
**amd-c77zejnh** — (2026-11-24-viola-up, 2 of 2) + contract/viola-up-outcome/no-session, cwd-missing, session-live, not-live …, one row per closed outcome word (PROVISIONAL). These are the picked rows up-outcome/cwd-missing and no-session: a parallel vocabulary for resume outcomes that a new adopt word should stay consistent with.

@history_item n=27 marker=amd-veadijwm
**amd-veadijwm** — (2026-11-24-viola-up) ~ contract/cli-stderr/fixed-message-refusal names `up`, and + contract/fixed-refusal-pairs/boot-time-unread (a new refusal word with its own fixed pair, PROVISIONAL). It is the precedent for registering a new fixed pair as a row of its own.

@history_item n=28 marker=amd-rdpw7edc
**amd-rdpw7edc** — (2026-11-24-viola-up) + contract/viola-up-output/not-live-hint: one fixed stderr `hint:` line, none under `--json` (PROVISIONAL). It is a precedent for an after-the-fact output line or hint of the kind adopt's early-exit report would add.

@history_item n=29 marker=amd-u2kuu7lv
**amd-u2kuu7lv** — (2026-11-23-viola-open-2) ~ contract/cli-exit-codes/1 and ~ contract/cli-refusal-line/fixed-unable-form grew to name `open`, with its `--json` refusal documents. It shows how the exit-1 row's verb list is extended, and why adopt later needed a row of its own.

@history_item n=30 marker=amd-sskvyypn
**amd-sskvyypn** — (2026-11-23-viola-open-2) the twin ~ contract/cli-stderr/fixed-message-refusal names `open`. Every exit-1 verb change lands on both twin rows (cli-refusal-line and cli-stderr).

@history_item n=31 marker=amd-nodoguut
**amd-nodoguut** — (2026-11-24-viola-focus) ~ contract/cli-exit-codes/1, the current wording of the picked row (`run`, `revive`, `open`, `focus` or `verify`; `--json` for revive's shared-start, open's and focus's refusals). That is the row whose bound forced adopt's separate 1-adopt-start row.

@history_item n=32 marker=amd-wfszjzeo
**amd-wfszjzeo** — (2026-11-24-viola-focus) the operator ratified the focus forks: contract/cli-exit-codes/1, cli-refusal-line/fixed-unable-form and cli-stderr/fixed-message-refusal stand as written. These are the picked rows' current standing.

@history_item n=33 marker=amd-tw4doisi
**amd-tw4doisi** — (2026-11-24-viola-focus) ~ contract/cli-stderr/fixed-message-refusal, adding focus to its exit-21 list. It is part of the chain that set the picked row's present text.

@history_item n=34 marker=amd-xmchmhkf
**amd-xmchmhkf** — (2026-11-24-viola-focus) ~ contract/cli-refusal-line/fixed-unable-form, adding focus to its exit-21 list. It is part of the chain that set the picked row's present text.

@history_item n=35 marker=amd-qcifhjq7
**amd-qcifhjq7** — (2026-11-20-a-new-cli-version-stamped-automatically) + contract/auto-verify-line/*: fixed stderr lines that `run` and `revive` print at start, silent under `--json` (ratified in amd-o6leil5z). It is the precedent for a start printing a fixed stderr line before the child holds the terminal, and it fixes the line form a new adopt line would match.

@history_item n=36 marker=amd-ngs74kgp
**amd-ngs74kgp** — (2026-10-31-sanitised-error-surfaces) `unknown`'s detail is a code word or nothing; exit 14 prints no hint because a hint would have to guess. This underlies the picked rows decision/no-hint-for-unknown and no-hint-for-faults. Trap: an early exit whose cause cannot be told apart should not get a guessing hint.

@history_item n=37 marker=amd-eohruztr
**amd-eohruztr** — (2026-10-31-sanitised-error-surfaces) `error: wrapper fault` and `error: internal error` print no hint; the human line holds the code only and detail goes to `diagnostics/`. This underlies decision/no-hint-for-faults: an early exit reported as a fault gets no hint, while one reported as a refusal gets one.

@history_item n=38 marker=amd-owfr5fvz
**amd-owfr5fvz** — (2026-10-31-sanitised-error-surfaces) a code word is 1 to 64 bytes of lower-case letters, digits and hyphens, with a letter first (`viola_core::is_code_word`). A new closed word for the early exit has to pass it.

@history_item n=39 marker=amd-skxu24vm
**amd-skxu24vm** — (2026-10-31-sanitised-error-surfaces) every `--json` failure driven by `tests/cli_error_bodies.rs` is one document, exits its literal code, leaves stderr empty, and holds no ESC, path or upstream text. This is the guard of decision/json-failure-documents-clean, which a new adopt `--json` failure must pass. No child stderr or exit text may be echoed.

@history_item n=40 marker=amd-k65k5y2q
**amd-k65k5y2q** — (2026-10-31-sanitised-error-surfaces) exit 2's fixed stderr lines carry no `--json` document. This is the origin of the exceptions on decision/json-one-document-on-stdout, the picked row a new adopt exit path must respect or extend.

@history_item n=41 marker=amd-baejdken
**amd-baejdken** — (2026-11-23-model-and-effort-at-launch) ~ contract/cli-exit-codes/2 adds `PAIR_AFTER_DASHES` for `run`/`revive`. It is a recent precedent for a start-path exception to one-document `--json`.

@history_item n=42 marker=amd-ye7lvnrp
**amd-ye7lvnrp** — (2026-11-13-rows-proposal-rule-work) ~ contract/refusal-hints/refusal-line (one hint per reason or detail; an opaque detail or none prints no hint) and ~ contract/exit-21-json-cause/unwrapped-name (a symlinked instance directory reads `strict-modes-failed`). Both are picked rows, and they set the hint-keying rule for a new adopt reason or detail.

@history_item n=43 marker=amd-ocscbfpx
**amd-ocscbfpx** — (2026-11-14-budget-gate-removed) ~ decision/fixed-phraseology-vocabulary dropped `budget-paused`. A new closed adopt word enters this fixed vocabulary, which takes no filler and no guessing.

@history_item n=44 marker=amd-orppoko6
**amd-orppoko6** — (2026-11-14-budget-gate-removed) ~ contract/cli-exit-codes/10-14: 11 is retired and never reused. Trap: a new exit for an adopt early exit must not take 11.

@history_item n=45 marker=amd-cwwvu75z
**amd-cwwvu75z** — (2026-11-14-budget-gate-removed) ~ decision/no-hint-names-viola-release (why narrowed to human-typing) and ~ contract/cli-exit-codes/11 (retired; an older `budget-paused` reads `unknown`, exit 14). Both are picked rows: an early-exit hint must not name `viola release`.

@history_item n=46 marker=amd-iq2x6bhq
**amd-iq2x6bhq** — (2026-11-08T20-32-52-wrap, census) scoped the spinner, progress-bar and live-redraw ban to a verb's output. This is decision/no-spinners-in-verb-output: an adopt early-exit report is one static line, not a wait indicator.

@history_item n=47 marker=amd-afdghrws
**amd-afdghrws** — (2026-09-28-cli-output-tokens) clap is built without `color`, so every styled byte is viola's own SGR module's. This underlies decision/hand-written-sgr-module and the plain-output modes that adopt's output inherits.

@history_item n=48 marker=amd-dw5o7rze
**amd-dw5o7rze** — (2026-10-17-cli-output-discipline) the colour layer was built and the `NO_COLOR` rung reads present, an empty value included. This is the origin of the picked colour decision-order rows: a new adopt line stays uncoloured, with amber for `DIALOG` only.

@history_item n=49 marker=amd-q5giruei
**amd-q5giruei** — (2026-11-05-linux-only-code-and-ci) the Windows VT enable and rung 7 were retired from the colour decision order. This leaves the picked order rows (json, NO_COLOR, TERM=dumb, non-terminal) as they now stand.

@history_item n=50 marker=amd-ulngzfa3
**amd-ulngzfa3** — (2026-11-12T03-23-49-rows) the masters became rows, each checked against the code. An amendment for this chunk names its changes by row handle (`+`, `~`, `−`): new rows go into the adopt registers (viola-adopt-refusals/*, cli-exit-codes/1-adopt-start).
