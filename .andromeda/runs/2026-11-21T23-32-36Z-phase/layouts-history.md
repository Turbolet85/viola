@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (226 rows, every row checked against the code); every handle in this chunk's picked rows dates from this pass. Why it matters: a promise the code holds but no row states belongs to the code. So a resize fix changes a row only where a cli promise actually moves (send outcome, refusal, hint, verify step line).

@history_item n=2 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `viola send` specimen's `input-not-ready` hint became «send again, and if it repeats a human must look at the session» (the founder's wording). Why it matters: the «a refusal» option for a lost key would most likely be an unable not-delivered outcome. contract/viola-send-output/input-not-ready-hint already holds this wording, and any reuse or new detail must keep it byte for byte.

@history_item n=3 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses two texts itself before any frame (control-character, empty-text): `[/ ] unable` with exit 13 and a hint, and the wrapper repeats both refusals. Why it matters: a resize-time refusal or re-type would extend the not-delivered set behind contract/viola-send-output/unable and decision/wrapper-repeats-send-refusals. Its Kept note matters too: whether the issue line prints on a client-side refusal was never measured.

@history_item n=4 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/viola-send-output/read-back`: the read-back match widened from exact equality to the typed text less a trailing run of space/TAB/CR/LF. Any leading or inner difference stays unconfirmed. Why it matters: a dropped first key is a leading difference, so it already leaves the send unconfirmed. Under the «read-back and one re-type» option, this row's match rule is what detects the loss.

@history_item n=5 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** (two entries: the read-back row's PROVISIONAL mark dropped; the trailing-only tolerance ratified) — the founder ratified trailing-only tolerance. A leading trim waits for a live reading on «Standing live rig with a screen model». Why it matters: a leading-edge key loss after a resize must not be hidden by widening the match. Changing leading tolerance needs a live measurement first, which is exactly the kind of reading this chunk prices.

@history_item n=6 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1 of 2) — `~ contract/cli-exit-codes/11` retired and never reused; `~ contract/cli-exit-codes/14` reads an old `budget-paused` as `unknown`; `~ contract/viola-send-output/unable` exits 13 (or 10/12/14), no longer 11. Why it matters: a new resize refusal must map onto the existing exit set and must never reuse 11. The send's unable row lists its exits exhaustively.

@history_item n=7 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`; the unwrapped-name and endpoint-lost hints; the internal-error document; revive's post-preflight codes) — each exit-21 cause got its own `detail` word and its own hint, and run/revive start refusals got closed codes (contract/run-revive-start-json/internal-error). Why it matters: this is the precedent for decision/hint-for-every-cause. Any new refusal cause or detail the fix adds needs its own hint and its own closed `--json` word, and `error`/`v` stay fixed.

@history_item n=8 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing, and `unknown`, `wrapper fault` and `internal error` still take no hint. Why it matters: this is the except clause of decision/hint-for-every-cause. A new resize detail must be a closed code word with a hint, never a free-text `unknown`.

@history_item n=9 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `+ contract/viola-answer-output/not-delivered-hints`: `viola answer` reuses `viola send`'s hint for five not-delivered details (input-not-ready included). Why it matters: a new or reworded not-delivered detail or hint for a lost key also reaches `viola answer`'s hint table and this row.

@history_item n=10 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — the server-verify cause of exit 21 got its own hint, and `send` keeps its `[/ ] unable` mirror for it. Why it matters: this is the precedent for adding a refusal cause to `send` inside its mirror (contract/cli-refusal-line/form, decision/shared-refusal-word-column) without a new surface or wireframe line.

@history_item n=11 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify gained the screen and timing ledger rows (modal-signature … confirm-window) and the static `--help` paragraph (contract/viola-verify-output/help-paragraph). Why it matters: if the resize answer becomes a relied row, it is a timing/screen probe like these, and verify's help paragraph stays static and path-free.

@history_item n=12 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the verify wireframe's `MM` became the relied count (`LedgerRow::relied()`), with one step line per relied row and exit 0 when every relied row passes. Why it matters: a new relied «key after a resize» row grows `MM` by this rule (contract/viola-verify-output/step-line, summary), and no wireframe number is hand-edited.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS; the VT enable leaving the colour surface) — `MM` covers every relied row on every build, the stop is SIGINT/SIGTERM, and the uncoloured conditions are `--json`, a pipe, NO_COLOR and TERM=dumb. Why it matters: Linux is the only target, so the resize measurement and any new ledger row are Linux-only, with no per-OS relied set. decision/plain-output-modes and contract/viola-verify-output/exit-1 stand as reworded here.

@history_item n=14 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ …/summary` and `~ …/exit-0`: `absent` was added as a verdict for the browser-tools-prompt row alone, and exit 0 means every relied row counts (PROVISIONAL, delegate the overseer). Why it matters: a resize row whose live reading may be mixed or unmet must not borrow `absent` silently. Any new verdict word changes these three rows.

@history_item n=15 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's exit 1 covers a failing row, a refusal or a stop, and a stopped run prints no summary (contract/viola-verify-output/exit-1, contract/viola-verify/summary). Why it matters: a new resize probe that waits on a settle bound must keep the stop path (no summary, no stamp) intact.

@history_item n=16 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run`: on a verified CLI the start writes the trust record so that no trust dialog stands and «no key is typed». Why it matters: this row states which keys viola writes at start. A resize fix or measuring arm must keep the start key-free and type only into the empty input box, never a dialog.

@history_item n=17 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/viola-run/start-output`: a key-on start runs `viola verify` before the child, and its output reaches the terminal (PROVISIONAL, delegate the overseer). Why it matters: a new relied resize row would run inside every key-on start's automatic verify, so its cost and bound land on the `run` start path.

@history_item n=18 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the board's width comes from `viola_pty::host_size()`, never `COLUMNS`, and the TTY form is styled while the plain modes stay plain (later ratified by the founder, 2026-10-20). Why it matters: `viola_pty::host_size()` is the established host-size seam in viola-pty, which is the crate this chunk reads for the resize path. It is a precedent for where size is read.

@history_item n=19 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` is read back by a session-start inside its own 40 s PROVISIONAL window, else not-delivered / no-prompt-submitted (contract/viola-send-output/read-back-compact). Why it matters: this is the precedent for a bounded post-condition wait on `send` that ends in a named not-delivered detail, the shape a «settle wait» or «re-type» option would take.

@history_item n=20 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the read-back line's second trigger: `/clear` is confirmed by its new-session post-condition (contract/viola-send-output/read-back-clear). Kept: the refusal rungs apply to a listed command as to any text. Why it matters: a resize-time refusal or re-type applies equally to local commands, and their read-back rows must stay true.

@history_item n=21 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/viola-send-output/unconfirmable` and `+ contract/viola-send-output/read-back-mcp`: `/mcp` is confirmed by a transcript echo, and `/cost` and `/usage` are unconfirmable. Why it matters: these sends have no prompt-submitted to compare against, so a read-back-and-re-type fix cannot detect a lost key on them. The unconfirmable list bounds what such a fix covers.

@history_item n=22 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/viola-send-output/unconfirmable` and `+ contract/viola-send-output/read-back-context`: `/context` is confirmed by a stdout tag and is unconfirmable before its transcript exists. Why it matters: like n=21, this transcript-confirmed case has no prompt echo, which limits a re-type option for a key lost after a resize.

@history_item n=23 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp`'s tools are a driver's machine view of the verbs (contract/mcp-tools/send): each result is the CLI `--json` document less `v`, and a refusal is `refused: <code>`. Why it matters: any new send outcome or refusal word for a lost key also reaches the MCP send tool, with codes only and no `release`.

@history_item n=24 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `viola send` keeps its `[/ ] unable` mirror while other verbs share the `unable` word column, and every verb prints `error: internal error` from one catch site. Why it matters: this is the source of decision/shared-refusal-word-column and the mirror form a new resize refusal must take.

@history_item n=25 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface (absence/web-surface), Linux the one target, and no interactive prompt, TUI or pager in a verb. Why it matters: a resize fix may not add an interactive prompt or redraw to `send`, consistent with decision/send-lines-appended-never-redrawn and decision/no-spinner.
