@task_prose part=session
The window step that places a terminal window on a workspace now keeps the view: it reads the compositor's monitors once before the one move-to-workspace (the focused monitor's active workspace), and after a dispatched move reads them once more and sends one switch back to that workspace only when the active workspace moved.
The placement word is still set by the move alone; a failed before-read, after-read or restore changes no word, field or line; the Hyprland seam's request set stays eight and the placement words stay eight; no argv changed. Every caller of the window step (open, up, revive and adopt under --origin) takes it.
viola verify now refuses, exit 1, with code word untrusted-folder, when any of Runs B to E read the trust or imports form on its first screen; it writes no stamp, prints no step line or summary, removes its probe dirs, prints a fixed unable/hint pair, and logs one process-exit with detail untrusted-folder.
The automatic verify of a start from an untrusted folder therefore records no stamp and leaves the version un-attempted (the start prints its existing no-stamp unstamped line).
The process-exit detail enum of the diagnostics line schema gains untrusted-folder, additively; a test-only live-host proof script for the placing (R7) was added; v2-07 stays GATED on R7, which has not run, so the restore is unproven on the real compositor.
@task_file n=1 path=src/run/window.rs
what: the window step of a start: finds the terminal window it opened and moves it to its workspace through the Hyprland seam, setting the placement word; now also reads monitors before and after and restores the active workspace
@task_file n=2 path=src/cmd/verify.rs
what: viola verify, the one stamps writer: runs the probe runs A to F, checks the relied ledger rows and stamps the CLI version; now refuses a run whose first screen read trust or imports
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostics line schema, holding the process-exit detail enum
@task_file n=4 path=scripts/r7-place.sh
what: a new test-only live-host proof script for window placement (R7): opens one terminal window under a stand-in claude and reads placement and whether the active workspace stayed; its rehearse form runs over the fakes
@task_file n=5 path=tests/cli_open.rs
what: the integration tests of viola open, including the placing open's dispatch list
@task_file n=6 path=tests/cli_verify.rs
what: the integration tests of viola verify, including the untrusted-folder refusal that stamps nothing
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the verify tests of stamping, including a failing relied row that still stamps and a verify under an instance logging its start and exit
@task_file n=8 path=tests/cli_verify/runs.rs
what: the verify tests of the probe run dirs, that a failing verify leaves neither run dir behind
@task_file n=9 path=tests/cli_version_gate.rs
what: the tests of the start's version gate and the automatic verify of a new CLI version
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-verb census of exit causes, each code word with its exit code and hint
@task_file n=11 path=tests/contract_diag_schema.rs
what: the contract test holding the diagnostics line schema's enums to the product's words
@task_file n=12 path=tests/support/verify.rs
what: test support for running verify, now with a stamping verify typed in a given folder without a trusted root
@task_file n=13 path=tests/support/home.rs
what: test support for an isolated viola home and its wrapper starts, now with a start as the user in a given folder
@task_item n=1 part=touches
The viola verify exit code 1 rows (the relied-row-failed item and the exit-1 item): exit 1 now also covers the untrusted-folder refusal, which writes no stamp, unlike a failing relied row that still stamps.
@task_item n=2 part=touches
The viola verify process-exit refusal code set (interrupted, pinned-hash-mismatch, program-not-found, strict-modes-failed, unclean-fixture, version-unreadable) gains untrusted-folder; a closed list a row family holds.
@task_item n=3 part=touches
The viola verify process-exit detail rows (both families, under conventions and under occupied resources) take a sibling untrusted-folder at ERROR, under a parsed VIOLA_NAME.
@task_item n=4 part=touches
The process-exit detail enum rows: the enum gains one value, untrusted-folder, additively; the rows naming other verbs' details stand.
@task_item n=5 part=touches
The decision that a CLI version is verified only when every relied row counts, and the failing-row-still-stamps reading: a confounded run is now a refusal before any row is read, so no row result and no stamp exist for it.
@task_item n=6 part=touches
The verify trust decisions (verify types into no trust dialog, writes and removes its own trust record, refuses an untrusted home tree or stamps): verify still types nothing, but now a trust or imports first screen on Runs B to E is a refusal, not rows failed.
@task_item n=7 part=touches
The ledger stamps envelope, the one stamps writer and stamps-merge rules, and measured.first_screen: the refusal leaves stamps untouched; the first-screen words trust and imports are now what decides the refusal.
@task_item n=8 part=touches
The automatic verify of a new CLI version at start, its attempt lock and the no-automatic-reverify-of-red rule: a start from an untrusted folder leaves the version un-attempted with no stamp entry, so a later start from a trusted folder verifies it.
@task_item n=9 part=touches
The verify probe dir rules (removed by drop guard, removes only its own dirs): the refusal path ends with the probe dirs removed by their guards.
@task_item n=10 part=touches
The closed refusal-detail and code-word conventions: untrusted-folder is a new code word in a closed set.
@task_item n=11 part=touches
The human stderr refusal line and verify's no --json contract: the refusal prints a fixed unable/hint pair with no path, and writes no error body since verify takes no --json; the pair's wording is provisional.
@task_item n=12 part=touches
The window-features-target-Hyprland decision, the one compositor seam crate and hyprctl spawned directly with a closed argv: the window step now asks the seam for monitors twice and may send one workspace switch; the request set stays eight, no argv changed.
@task_item n=13 part=touches
The window event, its event data, the wrapper's window append and the instance snapshot's window field: the placement word is set by the move alone and the eight words are unchanged; no workspace is put on any line.
@task_item n=14 part=touches
The Hyprland seam error rows (malformed, oversize, refused, unreachable, unsupported): a failed monitors read or a failed restore now leaves the placement placed rather than changing the outcome.
@task_item n=15 part=touches
The open, up, revive --origin and adopt verbs and the viola open exit codes, through their one window step caller: a placing start now leaves the active workspace where it was.
@task_item n=16 part=touches
The focus and human-wheel-adjacent rules about the user's view: the window step now restores the active workspace after a move; where keyboard focus lands after the restore is unproven until R7.
@task_item n=17 part=touches
The real-claude-local-only and live-probe rules: the new R7 proof script starts no claude (a stand-in), runs live only on the host by the operator, and runs over the fakes only in its rehearse form; no CI step or harness command changed.
@task_item n=18 part=touches
The VIOLA_ environment namespace rule: the R7 script unsets every inherited VIOLA_ name by pattern before it runs.
@task_item n=19 part=touches
The test sockets under a short /tmp root rule: a rehearsal rooted under a long path overflowed sun_path; the rehearse form must take a short mktemp root.
@task_item n=20 part=touches
The each-test-in-its-own-home and fake-agent test seam rules: tests now take a verify and a wrapper start run from a given folder with the fake's trust root moved off it, to reach the trust form.
@task_fact n=1 slug=established-decisions
fact: viola verify refuses with untrusted-folder, exit 1, when any of Runs B to E read the trust or imports form on its first screen (Runs A and F never read): no step line, no summary, no stamp, probe dirs removed; an automatic verify at start then records nothing and leaves the version un-attempted.
@task_fact n=2 slug=established-decisions
fact: The viola verify process-exit refusal codes include untrusted-folder, beside program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture and interrupted.
@task_fact n=3 slug=conventions
fact: viola verify's process-exit detail includes untrusted-folder, logged at ERROR under a parsed VIOLA_NAME.
@task_fact n=4 slug=occupied-resources
fact: The process-exit detail enum of the diagnostics line schema includes untrusted-folder, viola verify's untrusted-folder refusal.
@task_fact n=5 slug=established-decisions
fact: The window step keeps the view: it reads the focused monitor's active workspace before the one move and, after a dispatched move, sends one switch back to it only when the active workspace changed; the placement word is set by the move alone, and a failed read or restore changes no word, field or line.
@task_item n=1 part=untouched
The wrapper channel, its methods, frames, fault codes and protocol version: no method, endpoint or frame added or changed.
@task_item n=2 part=untouched
The MCP server, its tools and results, and the hook tiers, hook contract and hook-to-kind map: untouched.
@task_item n=3 part=untouched
Send, wheel, dialog hold, link and pause rules: the session reaches none of them.
@task_item n=4 part=untouched
The build system, dependency policy, crate graph, toolchain pin, CI workflows, coverage floors and the pre-push gate: no crate, dependency, gate or CI step changed.
@task_item n=5 part=untouched
Names: no binary, command, crate, env var, config key or CLI flag added or freed; the names sheet is untouched.
@task_item n=6 part=untouched
The revive --json and run start refusal codes, the unclean-fixture refusal rows and the trust writer code words: they state other refusals and stay as they are.
@task_item n=7 part=untouched
The events log, snapshot envelope, list payload and board: no event kind, field or snapshot shape changed.
