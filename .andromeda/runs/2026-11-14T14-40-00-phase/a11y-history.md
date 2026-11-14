@history_item n=1 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** — the budget pause as a driver-only bound: added to §8 Timeout extensions (CLI) that an automated `viola send` is refused `budget-paused` / `five-hour`|`seven-day` (exit 11), nothing typed, until the reset or `viola release <name> --budget`; `answer`/`wait`/`last`/`pause` do not read the gate; the keyboard-test-harness Tooling label gained the closed gate's reading `path6_a_reading_over_the_threshold_refuses_the_send` (human key reaches the child, no `unable`/`hint:`/`budget-paused` on the outer PTY). This is the origin of decision/budget-gate, decision/budget-gate-only-on-send, contract/cli-exit-codes/11 and the budget-pause clause of the time-bound rows; every one of those clauses leaves with this chunk, and the guard it names leaves with `path6_budget_harness.rs`.

@history_item n=2 marker=2026-10-10-session-links
**2026-10-10-session-links** — held the "four verbs don't read the gate" clause by `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`, exit 11 `budget-paused` / `five-hour` as control) and named it in keyboard-test-harness Tooling as the driver-side reading. It is decision/budget-gate-only-on-send's guard; `tests/cli_budget.rs` leaves whole here, so the row and its Tooling mention must retire together, not left with a dangling guard.

@history_item n=3 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the three-runner greens re-worded as dated readings) — re-worded `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` as green at CI run 38098238731 and keyboard-test-harness's revive, statusline and budget-gate readings as green in CI. These dated budget readings in cognitive-accessibility.md (closed-gate case) and keyboard-test-harness.md (Tooling) are the text that retires with the budget passages; precedent: a dated measurement whose ground moved is re-worded by its owner, never left as current.

@history_item n=4 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — set the statusline-bearing start's zero-viola-bytes reading (`passthrough_with_a_statusline_source_adds_no_viola_bytes`, guard of decision/statusline-output-passed-byte-for-byte) and `viola hook statusline`'s 5 s `STATUSLINE_DEADLINE` (contract/viola-hook-statusline/exit, exit 0, empty stderr). This chunk removes only the arm's `rate_limits` parse, `budget.json` write and `hook-decision{budget_written}`; the pass-through, its guard and its time bound stay for the next entry («Status-line interception removed»).

@history_item n=5 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — first entry listing a time bound that falls on the driver, never the human (dialog deadline, `not-delivered` / `unknown-dialog` exit 13). Start of the enumerated list in decision/time-bounds-never-on-human and decision/time-bounds-fall-on-the-driver from which this chunk drops the budget pause; the other bounds stand.

@history_item n=6 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — added the readiness gate (`GATE_MAX_WAIT` 8.5 s, `input-not-ready`) to the driver-only bounds and the case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` (guard of decision/human-key-takes-the-wheel). Its human-typing refusal shares `send`'s refusal order with the `budget-paused` rung being removed; the remaining rungs, exit 10 and this guard must stay intact.

@history_item n=7 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — fixed the board's structure (BAY line, caption row, wrapped rows, then separator + unwrapped rows; contract/viola-list-board/order and /unwrapped-separator) and added the 5 s `claude agents --json` read to the driver-only bounds. The budget header in the BAY line and budget cells leave from that structure; the order, separator and agents-read bound stay.

@history_item n=8 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — added the MCP `wait` cap (`WAIT_CAP` 50 s) to the driver-only bounds; a sibling clause in the time-bound rows that stays when the budget pause leaves them.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — added the confirm windows (10 s; 40 s for `/compact`) to the driver-only bounds (contract/send-refusal/no-prompt-submitted); a sibling clause that stays in the time-bound rows.

@history_item n=10 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — made the dialog hold configurable via `config.json` `dialog_hold_s` (60–3600, default 60; contract/config-json/dialog-hold-s). That key and the file stay when `five_hour` / `seven_day` (contract/config-json/five-hour, /seven-day) leave; how the config reader treats a leftover unknown key is this chunk's P3 measurement.

@history_item n=11 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — pinned the board caption by the `CAPTION` literal and set "each state carried by its word alone". decision/state-carried-by-printed-word enumerates `budget-paused` among the board's words; that word leaves the board, the caption set (decision/list-captions-fixed) does not change.

@history_item n=12 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour spans (amber only on `DIALOG`, dim on stale, bold NAME; none under `--json`, pipe, `NO_COLOR`, `TERM=dumb`) and the `--help` group labels, pinned by `help_is_the_grouped_verb_table` / `help_lists_only_served_verbs`. Removing the board's budget cells and the `list` help text naming the budget must keep contract/viola-list-board/colour and contract/cli-help/group-labels true and their literal pins updated on purpose.

@history_item n=13 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `handoff:` help row built) — set the help groups to board / traffic / wheel / handoff / setup (contract/cli-help/group-labels); `viola release` stays in `wheel:` after `--budget` leaves it.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's live reading: a human key wrote `wheel{human, human-input}`, the next `send` exited 10 `human-typing`, and the founder's own `viola release` returned the wheel. Confirms that plain `release` (the wheel handed back; critical path "human takes the wheel") stays while only `release --budget` leaves.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail, a code word or nothing) — `unknown` (exit 14)'s detail became a code word or nothing, still with no hint (decision/unknown-refusal-has-no-hint, contract/cli-refusal/unknown-refusal). Relevant because a `budget-paused` reaching a new client from an older wrapper falls outside the closed refusal list once it is removed.

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (a peer's refusal screened to a code word) — a code word outside the closed refusal list stays visible in the document and in `refused: <word>` and takes `unknown`'s exit 14 (decision/unlisted-code-words-stay-visible, decision/peer-replies-screened-to-code-words). Trap for exit 11's retirement: after `RefusalReason::BudgetPaused` leaves, a `budget-paused` from a mixed-version peer reads exit 14, not 11; exit 11 must not be reused.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the census of error writers and the body check) — `error_bodies_each_name_a_writer_and_a_witness_that_stand` (guard of decision/error-bodies-have-writer-and-witness) and `product_error_writers_are_each_named` in `tests/contract_error_bodies.rs`. The budget writers and witnesses (`tests/cli_budget.rs`, budget hints) leave, so this census must drop them or it goes red on witnesses that no longer stand.

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness) — `cli_error_documents_hold_no_path_or_upstream_text` (guard of decision/json-failure-documents-are-clean, contract/cli-json-failure/document); any budget-paused rows it drives leave with the refusal, the rule and its witness stay.

@history_item n=19 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`--json`'s one document and its exit-2 exception) — the one-document-no-hint rule with its `answer` / `send` exit-2 exception (decision/json-one-document-no-hint, contract/cli-json-failure/send-answer-exit-2, contract/cli-refusal/json); unchanged by the removal, it constrains `send`'s remaining refusal paths.

@history_item n=20 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the exit-cause table and one hint per code) — `no_two_codes_share_a_hint` and `no_hint_names_release` in `tests/cross_exit_causes.rs` (guards of decision/one-hint-per-code, decision/no-hint-names-release), the exit-cause table checking each cause's exit, document and hint. The budget-paused hint and any table row for exit 11 leave; both guards must stay green over the shrunk catalogue.

@history_item n=21 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — every refusal of the start `revive` shares with `run` gives one exit-1 document (contract/viola-revive-json/start-refusal, contract/start-json-refusal/document, the revive refusal codes). The budget step of the start order (the `budget-gate` start record) leaves from that shared start in `src/cmd/run.rs` / `src/cmd/revive.rs`; the closed code set must not change.

@history_item n=22 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (revive's start refusals without batch-script-child) — precedent: a closed word (`batch-script-child`) left the code list together with its emitter, a narrowing recorded as an amendment. The direct model for retiring `budget-paused` and exit 11 from the refusal list and exit-code rows while the remaining codes keep their numbers.

@history_item n=23 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the §8 verification lines pinned by literal) — Error recovery verification: literal asserts over `std::process::Command` hold the typed exit codes (`10`–`14`, `20`, `21`) and a hint-free `--json`; every fixed CLI string pinned by literal. Exit 11 sits inside that enumerated range; with it retired the range text and decision/refusal-layout-literal-pins / decision/fixed-cli-strings-pinned must not claim an exit-11 pin, and removed budget strings need their pins removed on purpose.

@history_item n=24 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins) — the cli checks are `std::process::Command` with literal pins (decision/cli-a11y-by-literal-tests, P4's `human-typing` refusal); the tooling the replacement or removed budget cases must follow.

@history_item n=25 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0) — the tui boundary driver is portable-pty `=0.9.0`, inherited, never re-pinned (decision/tui-boundary-driver-pin); the closed-gate outer-PTY reading leaving does not touch the driver.

@history_item n=26 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli / tui cases run as nextest cases in the `coverage` suite under `gate --require coverage,doctest`; a breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) fails the build; `#[ignore]` banned (decision/a11y-gate-breach-fails-build, decision/zero-flakiness, decision/ci-test-gate-no-skips). Removed budget tests must be deleted, not skipped or ignored, and the `.config/nextest.toml` Path 6 entry must leave so no suite reads missing.

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg) — the a11y cases' CI home is the single `ubuntu-latest` leg's `coverage` suite, none skipped; the verdict this chunk's removal is read against.

@history_item n=28 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants in crate tests are covered by the epoch-boundary mutation audit only (decision/mutation-audit-covers-crate-invariants); consistent with this chunk's directive of no mutation run, guard controls going through `gate.py control`.

@history_item n=29 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (hook and MCP on a refused socket directory: not-assertable) — the `hook-decision` record shape for hook calls (contract/hook-call/socket-dir-refused: fail open, exit 0, empty streams, one `hook-decision` at WARN). The statusline arm's `hook-decision{budget_written}` field leaves; the record and its fail-open contract stay.

@history_item n=30 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start: exit 1, revive's eleventh code) — the start's socket-directory refusal before any snapshot, bind or child (contract/start-refusal/socket-dir, contract/revive-json-refusal/socket-dir-refused, contract/cli-fixed-message/form); a start-order refusal that stays when the start's budget step is removed.

@history_item n=31 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities) — `run`, revive's start and `verify` refuse a failing home tree before any log opens (contract/start-refusal/home-strict-modes); the first rung of the start order, unchanged when the budget step leaves it.

@history_item n=32 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — the after-feed-panic boundary (contract/tui-passthrough/after-feed-panic: `send` refused `input-not-ready` exit 13, human key reaches the child, no viola literal on the outer terminal); a sibling outer-PTY reading to the removed closed-gate one, which stays.

@history_item n=33 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0 retired the frontend stubs to N/A (the absences and §5/§6/§7 N/A overviews in the rows file); the same census (`doc:viola-0.2.0/census`, F-08) routes the budget rows to v2-04, which retire at this chunk's wrap.

@history_item n=34 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code, with guards re-pointed; the budget rows (decision/budget-gate, /budget-gate-only-on-send, exit 11, config thresholds) and their guards now live as rows, which this removal retires by handle (`− {handle}`) or edits (`~ {handle}`) at wrap, the phase amending no master.

@history_item n=35 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — precedent for a row-level amendment (`~ decision/no-screen-content-parsing`, was/now with a resolved-by line); the form in which the time-bound and state-word rows naming the budget pause are edited at wrap.
