@history_item n=1 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — §8 Timeout extensions gained the confirm windows: a typed send waits `CONFIRM_WINDOW_FALLBACK` (10 s), a `/compact` send on a verified CLI its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL); unconfirmed it is `not-delivered` / `no-prompt-submitted`, the bound on the driver only. This is the origin of contract/send-refusal/no-prompt-submitted and the precedent for a local command's post-condition ledger row; a new local-command echo confirmation must keep its wait a driver-only bound and keep that row true (or amend it) for the unlisted local commands that today end `no-prompt-submitted`.

@history_item n=2 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — §8 / §3 / §4 P4: a send on a verified CLI waits in the readiness gate at most `GATE_MAX_WAIT` (8.5 s), then `not-delivered` / `input-not-ready` with nothing typed; a human key during the wait goes to the child, takes the wheel, and the waiting send is refused `human-typing`. Origin of contract/send-refusal/input-not-ready, decision/send-refused-while-human-typing and the readiness-gate bound in the time-bounds rows; the local-command send path rests on this gate and must not change who a wait falls on.

@history_item n=3 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing`: the vendor screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char, never the text after it; ratified as a boundary widening (founder's answer, card confirm PROVISIONAL). If the measured echo of a local command is a screen line, reading it widens this row again and needs the same ratification; a command's output text stays vendor content.

@history_item n=4 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility: 6 rows) — `~ decision/time-bounds-fall-on-the-driver` dropped the budget pause from the list of bounds; `~ contract/cli-exit-codes/11` retired exit 11 never to be reused (an old `budget-paused` reads `unknown`, exit 14). Precedent: every bound a chunk adds or removes is reflected in the time-bounds rows' enumerated list, and a retired exit code is not reused for a new send outcome.

@history_item n=5 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility: 1 row) — `~ decision/time-bounds-never-on-human` dropped the budget pause from its enumerated bounds. A new or changed confirm window for a local command's echo must land in this row's list.

@history_item n=6 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (8. Cognitive Accessibility) — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` dropped the status-line deadline, leaving wait timeout and cap, dialog hold, readiness gate, confirmation windows and agents read. The current wording of the two picked time-bound rows this chunk's echo wait must fit.

@history_item n=7 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `--json` one-document rule gained its exception (the `viola answer` / `viola send` exit-2 paths write a fixed stderr line, no document); peer replies screened to code words (`is_code_word`, a non-code `refusal` reads `unknown`, a non-code `detail` reads nothing); `thiserror_messages_interpolate_nothing` and `cli_error_documents_hold_no_path_or_upstream_text` added. Origin of contract/cli-refusal/json, decision/json-one-document-no-hint, decision/peer-replies-screened-to-code-words, decision/thiserror-messages-interpolate-nothing, decision/json-failure-documents-are-clean; any new send detail word must be a code word, and a local command's echoed output must never reach a document, error or hint.

@history_item n=8 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the exit-cause table (`tests/cross_exit_causes.rs`) with `no_two_codes_share_a_hint` and `no_hint_names_release`; new hints pinned by `send_hint_is_the_design_string`. Origin of decision/one-hint-per-code and decision/no-hint-names-release: any new send refusal detail or hint this chunk adds must take its own hint and its row in the table.

@history_item n=9 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins; the §8 verification lines pinned by literal; the cli tooling in the a11y testing contracts) — trycmd / assert_cmd retired: every fixed CLI string, hint and result line is pinned by literal over `std::process::Command`, the `unable <reason> <detail>` + `hint:` layout, typed exit codes (10–14, 20, 21) and hint-free `--json` held by literal asserts on ubuntu-latest. Origin of decision/fixed-cli-strings-pinned, decision/fixed-cli-strings-pinned-by-literal, decision/refusal-layout-literal-pins, decision/cli-a11y-by-literal-tests; a changed `send` mirror line (e.g. `unconfirmable`'s fixed note in `src/human/mirror.rs`) must be updated in its literal pin on purpose.

@history_item n=10 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — the TUI passthrough entity after a contained vt100 feed panic: the driver's `send` refused `not-delivered` / `input-not-ready` (exit 13) while a human key still reaches the child, no viola literal on the outer terminal (`chaos_feed_panic_refuses_send_and_keeps_passthrough`). Origin of contract/tui-passthrough/after-feed-panic; an echo read through the vt100 model must keep that send refusal and the outer-terminal silence intact.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone; the a11y cases' CI home, the one ubuntu-latest leg) — the TUI hosted in openpty only, the three boundary clauses each a nextest case on `ubuntu-latest`, the cli / tui cases in the `coverage` suite gated by `gate --require coverage,doctest`; the feed-panic boundary reads the child's own bytes with no Windows arm. Where this chunk's send and boundary cases run and are judged.

@history_item n=12 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — the closed non-editing list (focus, mouse reports, terminal replies), every other byte editing; the wheel probed with `exit 13 driver, 10 human`. Underlies decision/human-keystrokes-never-blocked, contract/tui-passthrough/human-keystrokes and contract/cli-exit-codes/10; any echo signal must not be a byte that moves or is mistaken for a wheel move.

@history_item n=13 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading (founder's own key on live `claude` 2.1.296), supplemental and never gating. Precedent for decision/live-readings-never-gate: any live start this chunk asks for to read a local command's echo is a supplemental reading, never a gate.

@history_item n=14 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the first live takeover reading on Linux recorded as supplemental and never gating, and the non-editing list widened with founder ratification. Origin of decision/live-readings-never-gate, bearing on the scope's no-live-start boundary.

@history_item n=15 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants (P4, P6) held in crate tests meet mutation only at the epoch-boundary audit (`run --mutants`). Origin of decision/mutation-audit-covers-crate-invariants; consistent with the scope's no-mutation-run boundary.

@history_item n=16 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's interrupted pair: no step line, no summary, `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last, exit 1. Origin of contract/interrupted-verify/exit and /stderr; a new `viola verify` probe for the echo ledger row must keep the `[NN/NN] … pass|fail` step line count and this interrupted form.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities) — `viola run`, a revive start and `viola verify` over a failing home tree refuse before any log, exit 1, the home-state pair, `verify` with no step line or summary. Origin of contract/start-refusal/home-strict-modes, which a new verify probe must not run ahead of.

@history_item n=18 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the same TUI passthrough boundary as `viola run`, the zero-SGR list and keystroke ban covering it. Origin of decision/single-vendor-content-boundary's revive clause; a local-command send into a revived start reads the same rules.

@history_item n=19 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision · contract · absence), each verified against the code; the picked rows' current form comes from this conversion, and any change this chunk makes lands as a `+` / `~` / `−` row entry.
