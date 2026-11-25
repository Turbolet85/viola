@task_prose part=session
The wrapper's window step, after moving its own terminal's window to its workspace, now keeps the user's view: it reads the compositor's monitors once before and once after the one move, and sends one switch back to the previously active workspace only when the move changed it.
A failed monitors read or a failed restore changes no placement word, field or line; the Hyprland seam's request set stays eight, the placement words stay eight, no argv, env var, flag, crate or IPC method was added.
viola verify gains a refusal: when any of Runs B to E first read a trust or imports dialog screen, verify refuses with a fixed unable/hint pair, exit 1, writes no stamp, removes its probe dirs, and logs process-exit detail untrusted-folder.
The process-exit detail enum of the diagnostics line schema gains untrusted-folder additively; the automatic verify at start then leaves the version unverified and un-attempted.
A new test-only R7 script places one terminal window live (rehearsed over fakes only); R7 itself is unfired, so the restore stands unproven on the real compositor and the capability stays gated.
The refusal pair's wording is provisional, on a delegate's word, owed to the operator in the epoch batch.
@task_file n=1 path=src/run/window.rs
what: the wrapper's window step that moves only its own terminal's window to its workspace through the Hyprland seam, now with the before/after monitors read and the conditional restore of the active workspace
@task_file n=2 path=src/cmd/verify.rs
what: viola verify, the probe runs against the real or fake claude and the stamps writer, with its closed refusal codes and the new untrusted-folder refusal before any row is checked or stamp written
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostics log line schema, its process-exit detail enum (the closed code words a process-exit line may carry)
@task_file n=4 path=scripts/r7-place.sh
what: a test-only live rig script that opens one terminal window through viola open on the real compositor, starts no claude (an owner-only stand-in), unsets every inherited VIOLA_ name, and closes only its own terminal
@task_file n=5 path=tests/cli_open.rs
what: integration tests of viola open, its window placement and dispatch list
@task_file n=6 path=tests/cli_verify.rs
what: integration tests of viola verify, its refusals and stamps
@task_file n=7 path=tests/cli_verify/stamps.rs
what: integration tests of verify's stamp writing and its start and exit log lines
@task_file n=8 path=tests/cli_verify/runs.rs
what: integration tests that verify's probe run dirs are removed after a run
@task_file n=9 path=tests/cli_version_gate.rs
what: integration tests of viola run's version gate reading the stamps file
@task_file n=10 path=tests/cross_exit_causes.rs
what: the test table of every refusal code word, its exit code and its hint
@task_file n=11 path=tests/contract_diag_schema.rs
what: contract test of the diagnostics line schema's enums
@task_file n=12 path=tests/support/verify.rs
what: test support that runs a stamping verify, now also typed from a given folder without a trusted root
@task_file n=13 path=tests/support/home.rs
what: test support for a test's own viola home and a wrapper started as the user, now also from a given folder
@task_item n=1 part=touches
decision/window-step-moves-own-terminal-window: widened, the operator's own answer M1 — the window step still moves only its own terminal's window, but now also reads the monitors before and after and may send one workspace switch back to the previously active workspace; the rule's bound must say so, named, not generalised.
@task_item n=2 part=touches
Indirect over the window step: the Hyprland seam rows (hyprctl child bounded by a deadline, reply text dropped at the parse) now cover two more monitors reads and a possible workspace request per placing open; the seam's request set stays eight, no new seam request.
@task_item n=3 part=touches
Indirect over the window step: its callers viola open, viola up, viola revive under origin and viola adopt under origin all take the restore, so any row bounding what those verbs do to windows or workspaces is reached.
@task_item n=4 part=touches
decision/window-record-kept-to-snapshot-and-event-log and decision/focus-raises-only-ancestry-windows: no workspace id, address or pid reaches a log line from the new reads; the restore switches a workspace, it raises or focuses no window by itself (where focus lands after it is unproven until R7).
@task_item n=5 part=touches
contract/verify-refusal-codes/*: verify's closed refusal code set gains untrusted-folder (was program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture, beside interrupted); a sibling row is owed.
@task_item n=6 part=touches
decision/verify-never-answers-import and decision/verify-never-answers-import-dialog: a verify run from a folder that is not trusted now refuses whole before any row is read and writes no stamp, instead of failing rows and stamping.
@task_item n=7 part=touches
decision/verify-never-answers-dialogs, decision/verify-writes-no-dialog-byte and decision/verify-stamp-first-screen-closed-words: the new refusal reads only the closed first-screen words trust and imports of Runs B to E and types nothing into the dialog.
@task_item n=8 part=touches
decision/stamps-written-only-by-verify and decision/stamps-checked-under-lock: on the untrusted-folder refusal the stamps file is untouched; the refusal fires before rows are checked and stamps updated.
@task_item n=9 part=touches
decision/verify-probe-dirs-cleanup and decision/verify-probe-artifacts-cleanup family: on the new refusal the probe dirs are removed by their drop guards.
@task_item n=10 part=touches
decision/unverified-cli-gate and decision/version-gate-strict-stamps-read: an automatic verify at start from an untrusted folder prints the no-stamp unstamped line and leaves the version un-attempted, cli_verified false.
@task_item n=11 part=touches
decision/refusals-hold-no-paths-pids-ids, decision/fixed-error-display-messages and decision/error-messages-interpolate-nothing: the new refusal pair is fixed text with no path; its wording is provisional, delegate overseer.
@task_item n=12 part=touches
decision/every-error-writer-named: the new refusal goes through verify's existing human refusal writer; no new error writer, no error body (verify takes no json).
@task_item n=13 part=touches
Diagnostics schema guard rows: the process-exit detail enum gains one value additively; the rows naming that schema as a guard still hold, and any row listing verify's process-exit details gains untrusted-folder.
@task_item n=14 part=touches
decision/verify-trusted-runs-risk and decision/start-trust-skips-trusted-folders: verify now requires its folder trusted in Claude Code by refusing otherwise; the hint names running from a trusted folder.
@task_item n=15 part=touches
decision/hyprland-recorder-moves-own-window, decision/r5-adopt-script-ends-own-processes and decision/real-cli-local-only: a new live rig script of the same kind opens and closes only its own terminal window and starts no real claude.
@task_item n=16 part=touches
decision/test-seams-fake-agent-only, decision/tests-use-own-home and decision/test-removal-own-dirs-only: new test support starts verify and a wrapper from a given temporary folder; test-only, in the test's own home.
@task_item n=17 part=touches
decision/new-env-read-needs-log-entry and decision/new-listener-needs-log-entry: no new env read, listener, channel method or seam request was added.
@task_item n=18 part=touches
decision/fixture-hygiene and decision/fixture-recording-scrubbed: no fixture was re-recorded; over the fakes the monitors reply reads the same workspace before and after, so the restore is proven at the unit tier only.
@task_fact n=1 slug=data-protection
fact: viola verify refuses with untrusted-folder, exit 1, when any of Runs B to E first read the trust or imports dialog: no step lines, no stamp written, probe dirs removed, process-exit detail untrusted-folder logged.
@task_fact n=2 slug=data-protection
fact: verify's refusal codes include untrusted-folder, a fixed unable/hint pair holding no path; its wording stands provisional on a delegate's word until the operator's epoch batch.
@task_fact n=3 slug=authentication-authorization
fact: the window step may send one workspace switch back to the active workspace it read before its move, only when the move changed it; a failed read or restore changes no placement word.
@task_fact n=4 slug=authentication-authorization
fact: the R7 placing script opens one terminal window of its own through viola open, starts no claude (an owner-only stand-in answering its version), unsets every inherited VIOLA_ name, refuses on a locked desktop, and closes only its own terminal.
@task_item n=1 part=untouched
Dependency security: no crate, dependency, toolchain, pin, audit tool or CI workflow changed.
@task_item n=2 part=untouched
API security and the rmcp feature set: no MCP tool, MCP input or MCP error changed.
@task_item n=3 part=untouched
The wrapper channel: no channel method, frame, peer-euid check, socket directory or socket mode changed.
@task_item n=4 part=untouched
Hook payloads, hook decisions and hook refusals are unchanged.
@task_item n=5 part=untouched
Send, answer and paste text validation and their refusal words are unchanged.
@task_item n=6 part=untouched
Secret management: no credential strip, child environment, canary scan or stored secret changed.
@task_item n=7 part=untouched
The revive json codes, instance-unreachable causes and the CLI exit codes other than verify's refusal set are unchanged.
@task_item n=8 part=untouched
Verify's unclean-fixture refusal codes and line form state another refusal and are unchanged.
@task_item n=9 part=untouched
Config keys, home resolution, strict-modes checks and owner-only file modes are unchanged.
