@task_prose part=session
The window step that places a terminal window on a workspace now reads the Hyprland monitors before and after the one move, and asks the seam to show the previous workspace again, once, only when the active workspace moved; a failed read or a failed restore changes no placement word, field or line, and the eight placement words and the eight seam requests stay as they were.
viola verify gains a refusal: when any of Runs B to E first read the trust screen or the imports screen, it refuses with the code untrusted-folder (exit 1, a fixed unable and hint pair, no step lines, no stamped summary, no stamp written, process-exit detail untrusted-folder); a failing relied row with no confounded run still stamps; the automatic verify at a start then prints its no-stamp unstamped line and leaves the version un-attempted.
The diagnostics line schema's process-exit detail enum gains untrusted-folder, additively; the cross-surface exit-cause table gains its row with its own hint.
A new test-only live rig script for the placing live reading R7 was added; it was fired only in its rehearse form over fake foot and hyprctl and a stand-in claude, which read green placement=placed active=unchanged; R7 itself was NOT fired, so real placement without moving the view stays unproven and the capability stays gated on R7.
Three existing verify tests that failed a relied row by moving the fake agent's trusted root off verify's folder were re-based onto a spine fixture set with no SessionEnd payload in the trusted folder (one renamed); new shared test helpers run a stamping verify and a wrapper start from a given folder.
@task_file n=1 path=src/run/window.rs
what: the window step that places viola's own terminal window on a workspace through the Hyprland seam and reports the placement word; edited (monitors read before and after the move, one conditional restore of the previous workspace, four new unit cases)
@task_file n=2 path=src/cmd/verify.rs
what: the viola verify command, its probe runs, relied-row checks, stamp writing and closed refusal codes; edited (new untrusted-folder refusal before rows are checked or stamps written)
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostics line schema that every process log line is validated against, holding the process-exit detail enum; edited (one enum value added)
@task_file n=4 path=scripts/r7-place.sh
what: the live rig for the gated live reading R7 of real window placement, with its rehearse-only form over stand-ins; new file
@task_file n=5 path=tests/cli_open.rs
what: the viola open CLI integration suite, run as a subprocess over the fake terminal and fake Hyprland; edited (a placing open dispatches only the move)
@task_file n=6 path=tests/cli_verify.rs
what: the viola verify CLI integration suite (the verify_window_ class); edited (an untrusted folder refuses and stamps nothing)
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the viola verify stamp-writing integration cases (failing rows still stamp, verify under an instance logs its start and exit); edited (failing row re-based, one test renamed)
@task_file n=8 path=tests/cli_verify/runs.rs
what: the viola verify probe run-directory cases (no run dir left behind); edited (a doc comment only)
@task_file n=9 path=tests/cli_version_gate.rs
what: the unverified CLI version gate integration suite and its stamp helper; edited (failing-row stamp re-based, a start from an untrusted folder leaves the version un-attempted)
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-surface exit-cause table, each cause with its exit code and its own hint; edited (untrusted-folder row added)
@task_file n=11 path=tests/contract_diag_schema.rs
what: the diagnostics schema conformance contract suite; edited (untrusted-folder enum row added)
@task_file n=12 path=tests/support/verify.rs
what: the shared test helper that runs stamping verifies under a test-owned home; edited (a stamping verify typed in a given folder with no trusted root)
@task_file n=13 path=tests/support/home.rs
what: the shared test home and wrapper-start helper (wrappers started under a test-owned user home, held in a drop guard); edited (a wrapper start from a given folder)
@task_item n=1 part=touches
decision/window-placement-proven-live: the live reading R7 now also reads that the active workspace is unchanged after a placing open; still not proven, R7 unfired, the capability stays gated.
@task_item n=2 part=touches
decision/verify-failing-row-no-refusal: a failing relied row stays no refusal, but a run whose first screen read trust or imports on Runs B to E is now a refusal before any row is read, writing no stamp; the row gains that exception.
@task_item n=3 part=touches
The viola verify process-exit detail family (contract/verify-exit-detail/interrupted, pinned-hash-mismatch, program-not-found, strict-modes-failed, unclean-fixture, version-unreadable) gains a sibling untrusted-folder.
@task_item n=4 part=touches
The diagnostics schema rows (decision/diagnostics-schema-conformance, decision/jsonschema-validator, decision/diag-line-schema-requires-corr): the process-exit detail enum gains one value; conformance suite gains its row.
@task_item n=5 part=touches
decision/hint-names-its-cause and decision/each-exit-cause-own-hint: a new exit cause untrusted-folder with its own hint in the cross-surface cause table.
@task_item n=6 part=touches
contract/verify-output/summary-line, contract/verify-output/last-line, contract/verify-output/step-line, contract/viola-verify/stamped-line and contract/cli-human-lines/unable: the refusal prints no step lines and no stamped summary, only a fixed unable and hint pair whose wording is PROVISIONAL.
@task_item n=7 part=touches
decision/unverified-cli-gate and decision/version-gate-needs-all-relied-rows: an automatic verify refused as untrusted-folder prints its no-stamp unstamped line and leaves the version un-attempted, cli_verified false.
@task_item n=8 part=touches
decision/stamps-written-only-by-verify, decision/verify-sole-stamp-writer and decision/stamps-only-from-verify: verify now has a path that writes no stamp at all (stamps.json untouched).
@task_item n=9 part=touches
decision/wrapper-boots-pass-trusted-screens, decision/trust-start-folders-outside-repository and contract/fake-agent-cli/trust-config: a test that moves the fake agent's trusted root off verify's folder now meets the untrusted-folder refusal; failing rows are taken from a spine set with no SessionEnd payload instead.
@task_item n=10 part=touches
decision/verify-window-80s-kill: the verify_window_ test class gains two cases (refuses an untrusted folder; start from an untrusted folder leaves the version un-attempted).
@task_item n=11 part=touches
decision/verify-under-test-owned-user-home, decision/run-under-test-owned-user-home, decision/no-verify-start-outside-test-home and decision/stamping-verify-passes-transcript: new shared helpers run a stamping verify and a wrapper start from a given folder, still under a test-owned home.
@task_item n=12 part=touches
decision/one-shared-test-helper and decision/shared-e2e-helpers: the new folder-taking verify and wrapper-start helpers sit beside the existing ones in the shared support.
@task_item n=13 part=touches
decision/standing-live-rig, decision/live-rig-root-outside-repositories, decision/live-rig-clean-start and decision/kr-seams-rehearse-only: a new live rig for R7 that unsets every inherited VIOLA_ name, opens one terminal window, starts no claude, and has a rehearse-only form over stand-ins.
@task_item n=14 part=touches
decision/live-failing-reading-not-verification and decision/fake-agent-not-proof-of-real-cli: the rehearsal prints that it is not a live reading; over the fakes no restore is ever dispatched, so the restore is proven at the unit tier only.
@task_item n=15 part=touches
decision/window-raise-proven-live and decision/window-reopen-proven-live: sibling gated live readings; the R7 rig copies the R3 rig's shape including its jq and fake-agent preconditions.
@task_item n=16 part=touches
decision/mocks-only-at-product-seams and decision/external-processes-real-stubs: the window step's new reads and restore go through the Hyprland seam and are tested over its recorded replies; the rig uses stub foot, hyprctl and a stand-in claude.
@task_item n=17 part=touches
contract/fake-foot/spawn-record and decision/stand-in-terminal-tree-teardown: the R7 rig reads the terminal's process-start record and tears the opened foot down by pid.
@task_item n=18 part=touches
decision/test-function-naming: new tests named window_step_restores_the_active_workspace_the_move_took, open_placing_keeps_the_dispatch_list_to_the_move, confounded_reads_trust_or_imports_on_runs_b_to_e_only and a rename to verify_a_set_without_session_end_fails_its_rows_and_still_stamps.
@task_item n=19 part=touches
contract/strict-modes-refusal/verify and contract/process-exit-detail/unclean-fixture: neighbouring verify refusals that state other causes; they stay true but the verify refusal set they sit in grows by one.
@task_fact n=1 slug=integration-test-strategy
fact: viola verify refuses with untrusted-folder when any of Runs B to E first read the trust screen or the imports screen (Runs A and F never read): exit 1, the fixed unable and hint pair, no step lines, no stamped summary, stamps untouched, probe dirs removed, process-exit detail untrusted-folder at ERROR.
@task_fact n=2 slug=integration-test-strategy
fact: contract verify-exit-detail untrusted-folder is a member of viola verify's closed process-exit detail set, beside interrupted and the five other refusal codes.
@task_fact n=3 slug=integration-test-strategy
fact: an automatic verify at a start refused as untrusted-folder prints the start's no-stamp unstamped line and leaves the version un-attempted (cli_verified false, no stamps entry); a later start from a trusted folder verifies.
@task_fact n=4 slug=test-data-fixtures
fact: a verify test that needs a failing relied row takes it from a spine fixture set with no SessionEnd payload in the trusted folder, never by moving the fake agent's trusted root off verify's folder (that path is now the untrusted-folder refusal).
@task_fact n=5 slug=e2e-test-strategy
fact: real window placement without moving the view is proven only by the gated live reading R7, which reads green only on placement=placed with the active workspace unchanged; its rehearse form over stand-ins is no live reading, and over the fakes the restore is never dispatched.
@task_fact n=6 slug=unit-test-strategy
fact: the window step keeps its placement word when the monitors read or the restore fails; a restore of the previous workspace is sent at most once, only when the active workspace moved.
@task_item n=1 part=untouched
MCP tools, the wrapper channel and JSON-RPC error rows: no MCP method, channel method or error body changed; verify takes no --json and writes no error body.
@task_item n=2 part=untouched
send, wait, answer, release, last, dialogs, hooks and the wheel: none of their contracts or decisions moved.
@task_item n=3 part=untouched
CI workflows, coverage floors, mutation gate, perf budgets, fuzz and supply-chain gates: no CI step, threshold, floor or kill line changed.
@task_item n=4 part=untouched
The agent-run harness commands (boot, status, cleanup, run, logs, gate, pre-push) and their documents and exit codes: unchanged.
@task_item n=5 part=untouched
revive and run start refusal codes and exit causes, list board and list --json, instance snapshot and heartbeat rows: none touched.
@task_item n=6 part=untouched
Log line format, corr rules and the event log record kinds: no event kind, field or log key added; only one enum value of the process-exit detail.
@task_item n=7 part=untouched
Fake agent command line and receipt contracts and the recorded fixtures: no fake-agent option added and no fixture re-recorded.
@task_item n=8 part=untouched
Absences (database, outbound HTTP, browser suite, Windows and macOS, deploy stage): nothing added that touches them.
