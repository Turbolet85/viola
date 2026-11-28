@history_item n=1 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: the «deferred to 0.2.0; technology not chosen» frontend stub in §1, §5, §6, §7 and the keys wcag-criteria-mapping, structured-violation-json-schema, focus-management-test-harness, contrast-verification-harness became N/A (no web or graphical frontend), with the terminal fleet console (matrix#v2-14) taking its a11y scope at «Fleet a11y verification» and §5 naming the console's keyboard reach as that entry's; the P6 cli mirror says the console reuses `viola list`'s caption order. This is the stub this chunk's «surface section» replaces: the N/A overviews in the rows file come from it, and the console's surface must keep the board's captions and slot order.

@history_item n=2 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the web entities, browser harness, ARIA and contrast left 0.1.0; the frontend was deferred to 0.2.0 with its technology not chosen, and only the terminal surfaces stand (CLI output discipline, the TUI boundary, the wheel case, the manual SR passes). This is where the stub came from. The tier `Standard (1)` was left for the founder to re-decide (kept at 2026-11-01-t02-46-44-wrap). A new interactive console surface may reopen that tier question.

@history_item n=3 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the browser a11y leg left. The cli and tui cases became nextest cases in the `test` job's `coverage` suite under `gate --require coverage,doctest`, with no a11y artifact, stage, budget, second driver or `suite` value of their own. The obs `ui` process was removed, leaving the `process` enum `run · hook · mcp · cli`. This is the origin of decision/a11y-runs-in-existing-test-suites, decision/no-separate-a11y-driver, decision/no-own-a11y-artifact and decision/no-a11y-performance-budget. Two things follow for this chunk: its keystroke driver must be an ordinary nextest case in `coverage`, never a new driver or suite value; and the «own diagnostics role» must reckon with `ui` having been removed from the process enum, not with `ui` as a live role.

@history_item n=4 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — the `a11y-violation` row was ruled not a product `event` value: a harness-only row, with the shipped `ObsEvent` and `diag-line.v1.json` enum carrying none. Precedent for this chunk's obs event catalog rows for the console role: a11y adds no product event of its own.

@history_item n=5 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — the CLI output-discipline tests report under the CI `test` job's single instrumented `coverage` suite, gated by `gate --require coverage,doctest` (locally under `nextest-integration` / `nextest-e2e`), with failures only in `suites[].failures[]`. This is the source of decision/a11y-cases-inside-coverage-suite and decision/a11y-inside-existing-suites, and the console's keystroke cases land there.

@history_item n=6 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow and the scheduled `nightly.yml` carries no a11y step (decision/a11y-ci-github-actions). This chunk's carry `cry-h6vzhlce` (the RUSTSEC chrono ignore in `deny.toml`) touches the nightly advisories run that this entry keeps a11y out of.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — this marker has several entries. They put the a11y cases on the one `ubuntu-latest` leg inside `coverage`, with `scripts/agent-run.sh run` the only runner. The TUI boundary is hosted on openpty alone, each of the three boundary clauses its own nextest case. They added the Linux-only platform line (decision/linux-only: «the 0.2.0 window features on Hyprland»). They made the manual SR pass Orca alone, the tui pairing Orca + foot on Hyprland and founder-owned. The cli SGR checks lost their Windows arms. The console's PTY driver, platform and SR pairing build on these.

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — the cli checks are now `std::process::Command` with literal pins (trycmd and assert_cmd retired), and §8 says every fixed CLI string is pinned by a literal. The tui boundary driver moved to portable-pty `=0.9.0`, which a11y inherits and never re-pins. These are the sources of decision/cli-a11y-by-literal-tests, decision/tui-boundary-driver-pin and decision/fixed-cli-strings-pinned. The keystroke driver's PTY dependency must sit on that workspace pin, any new terminal-drawing or terminal-reading dependency is a fork, and the console's fixed strings need literal pins.

@history_item n=9 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer: bold NAME, amber only on `DIALOG`, dim on stale rows, each state carried by its word. The decision reads `NO_COLOR` (empty included), `TERM`, `COLORTERM` and whether stdout is a TTY, applies none under `--json` or a passthrough role, and picks a truecolor / 256 / 16 depth. A narrow terminal cuts an unwrapped NAME with `...`, the width from `viola_pty::host_size()`. Built the `--help` group labels over the served verbs. These are the sources of decision/colour-decision-inputs, decision/sgr-beside-its-word, decision/no-sgr-off-terminal, decision/unwrapped-name-truncation and contract/cli-help/*. A colour-drawing console frame must follow the same inputs and the word-beside-SGR rule, and a console start verb or flag joins the `--help` groups.

@history_item n=10 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified the `--help` served-verbs-only rows and `viola_pty::host_size()` as the board's cut-width source. A width source that already stands, relevant to the quarter-screen frame's size reading and its degrade rule.

@history_item n=11 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board's structure as built is BAY line → caption row → wrapped rows, with the `CAPTION` literal pinned by `list_human_board_is_the_literal_rows`, and there is no SGR on the board. This is the origin of the structural-equivalents rows the console's surface section extends.

@history_item n=12 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the `-- UNWRAPPED - READ-ONLY --` separator and unwrapped rows (NAME through `escape_row`), and the one 5 s `claude agents --json` read, which fails open and whose bound falls on viola's child. These are the structural carrier and the «agents read» bound that the time-bound rows list.

@history_item n=13 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — changed three rows: decision/terminal-structural-equivalents and decision/cli-structural-landmark-equivalents now add the `-- LINKS --` separator and the plain-space NAME indent of tree depth, and contract/viola-list-board/order became a tree plus the LINKS block. These are the exact rows the console's cli / tui landmarks entry extends. Each structural-carrier addition was a founder fork (F4, F5).

@history_item n=14 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code, with twin areas (for example `1-a11y-scope-summary` / `a11y-scope-summary`). This explains the twin and duplicate rows in the rows file (for example decision/static-linear-output, decision/linear-static-output, decision/linear-static-cli-output and decision/no-spinners). A redraw exception has to be reconciled across every twin.

@history_item n=15 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — this wrap named exceptions on zero-bytes and boundary rows rather than generalising them: `~ decision/portable-pty-outer-pty-driver` (except: the checks hold from the child's start), `~ decision/zero-viola-bytes-on-the-terminal`, `~ decision/wheel-ignores-terminal-events` and `~ contract/tui-passthrough/own-bytes` (the automatic-verify lines before the child). It added `+ decision/compiled-shapes-over-signatures` and paste write to both time-bound rows. This is the closest precedent for the redraw-ban reconciliation: each exception is narrow, named on the row's `except`, and decided by the operator.

@history_item n=16 marker=2026-11-24-adopt-s-resume-folder
**2026-11-24-adopt-s-resume-folder** — `~ decision/wheel-ignores-terminal-events` gained a second named exception (adopt's not-resumed pair after the child ends), and `~ contract/cli-refusal/unable-line` and `~ contract/cli-fixed-message/form` moved to «three own refusals». More precedent that an exception is named per surface on the row and that each new fixed-message refusal is added to the closed lists.

@history_item n=17 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/tui-passthrough/human-keystrokes`, `~ contract/tui-passthrough/viola-terminal-bytes` and `~ decision/human-keystrokes-never-blocked` gained the one named exception (the start's verify window, before the child). It was ratified at 2026-11-22T17-29-41Z-wrap. Another precedent for a narrow, named exception to a terminal-bytes promise.

@history_item n=18 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `~ decision/no-screen-content-parsing` gained its first `except`: a test-only rig that reads the vendor screen only as closed words, its raw frames kept local and gitignored. Precedent and constraint for this chunk's keystroke driver, which must read state and never screen bytes.

@history_item n=19 marker=2026-11-21-key-after-a-resize
**2026-11-21-key-after-a-resize** — the latest form of `~ decision/no-screen-content-parsing` except: «the test-only rig reads closed words, never text: trust selector's place, input box empty, holds-text or absent, modals, box-row cell attributes before any send or after a turn end, send or KR resize». It was ratified at 2026-11-22T17-29-41Z-wrap. The row's except is a closed list that grows a member at a time; a console test driver reading anything screen-derived would be another named member, which is a fork.

@history_item n=20 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing` rule: parsing only for compiled signatures, the prompt row only for its opening char. It was ratified at 2026-11-27T19-07-11Z-wrap. With 2026-11-20-a-quoted-modal-never-blocks-a-send (the compiled shapes added), this is the rule's current text that the console driver's state-not-screen design must respect.

@history_item n=21 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the new outer-PTY case is sequenced on the wrapper's lines and the `wheel` record, never a timer, and its limit of proof is stated. This is the origin of decision/boundary-tests-sequenced-on-records: the console's keystroke driver must sequence on log records or the state read, never on sleeps, inside the 20 s lint.

@history_item n=22 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — set the closed list of non-editing terminal input (focus, mouse, terminal replies), with the boundary case run step-wise behind receipts. The console's own keys and the resize to a quarter screen meet this boundary's notion of what counts as editing.

@history_item n=23 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ decision/run-diagnostics-off-terminal`: `run`'s diagnostics go to `diagnostics/`, never the terminal, its trust word one `trust-record` line there. The same marker added the start's Claude config lock wait to `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver`. This is a precedent for the console's own diagnostics role: diagnostics go to the log, never the console's terminal.

@history_item n=24 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — `~ contract/log-identity/process` (`process:"run"` for a `viola adopt` start), `~ contract/cli-refusal/unable-line`, `~ decision/strict-modes-refusal-before-any-write`, `~ contract/cli-fixed-message/form` and `~ contract/tui-passthrough/entry`. The same marker's later wrap set both time-bound rows' «Eight» lead to standing. Precedent for a new process or verb: each closed list (process tags, strict-modes verbs, fixed-message refusals) names the newcomer. The console's diagnostics role extends contract/log-identity/process.

@history_item n=25 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — added `+ contract/viola-up-output/instance-line`, a new human output surface given its own row. The operator's fork F1 settled that the time-bound rows hold «Eight», with `+ decision/open-live-wait-bound`, `+ decision/window-wait-bound` and `+ decision/hyprctl-read-deadline` as rows of their own. `~ contract/cli-exit-codes/21` gained `viola up`. A console bound (a redraw tick, a key read, a state poll) would take its own row, not join the lists.

@history_item n=26 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — `~ decision/strict-modes-refusal-before-any-write` and `~ contract/cli-exit-codes/21` gained `viola focus`, a human-only Hyprland command. The ratification at 2026-11-24T17-14-08Z-wrap fixed the F1 rule: «a later bound a row only where it is new on a new bearer». This is the nearest precedent for a human-only window-side verb such as the console's start verb, and the rule for whether its bounds take rows.

@history_item n=27 marker=2026-11-22-epoch-4-cleanup
**2026-11-22-epoch-4-cleanup** — `+ decision/start-verify-stop-grace-off-the-human`: the two time-bound lists are full at their bound, so the operator's ruling lands a new bound as a row of its own. Decides where any console time bound goes.

@history_item n=28 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** — added `+ contract/viola-switch-output/result-line` (a new verb's output-discipline row) and `+ decision/respawn-reap-bound` (its bound as a row of its own), and changed exit 10 and decision/send-refused-while-human-typing to name `switch`. Another precedent for how a new human surface enters the plan row by row.

@history_item n=29 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/time-bounds-fall-on-the-driver`, `~ decision/time-bounds-never-on-human` and `~ decision/state-carried-by-printed-word` (budget-paused dropped from the board's state words). These rows carry the board's state words, which a console view would reuse. Not later reversed.

@history_item n=30 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` joined the passthrough and the zero-SGR list, and its exit-1 refusals took the fixed `unable:` / `hint:` form. This is the source of the «passed `viola revive`» member of decision/no-sgr-in-plain-modes and decision/no-sgr-off-terminal. Precedent for how a new start path enters the no-SGR and refusal-form lists.

@history_item n=31 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — built the `handoff:` help row (`handoff:  link, unlink` between `wheel:` and `setup:`, contract/cli-help/handoff-row) and added the link verbs' result lines to the literal-pinned strings. A console start verb in `--help` would change contract/cli-help/group-labels and usage-line, which are held by `help_is_the_grouped_verb_table` and `help_lists_only_served_verbs`.

@history_item n=32 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — `plugin install` built as one entry in `setup:`, with a bare `plugin` unserved (contract/cli-help/served-verbs-only). The served-verbs-only rule any console verb must follow in `--help`.

@history_item n=33 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `unknown`'s detail is a code word or nothing (contract/cli-refusal/unknown-refusal, decision/refusal-ends-with-hint except). The census `error_bodies_each_name_a_writer_and_a_witness_that_stand` was added (decision/error-bodies-have-writer-and-witness). Any console refusal or error body must have a named writer and a witness in `tests/contract_error_bodies.rs`.

@history_item n=34 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — added `no_two_codes_share_a_hint`, `no_hint_names_release` and the exit-cause table `tests/cross_exit_causes.rs` (guards of decision/one-hint-per-code and decision/no-hint-names-release). New console refusal hints fall under these catalogue checks.

@history_item n=35 marker=2026-11-25-placing-without-moving-the-view
**2026-11-25-placing-without-moving-the-view** — `~ contract/cli-fixed-message/form` gained `viola verify`'s untrusted-folder refusal. This is the row's current closed list, which a console start's fixed-message refusal would extend.

@history_item n=36 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants are covered by the epoch-boundary mutation audit (`run --mutants`), not by a per-chunk gate. The scaffold's invariants meet mutation only at the Epoch 6 boundary, and this phase runs no mutation.
