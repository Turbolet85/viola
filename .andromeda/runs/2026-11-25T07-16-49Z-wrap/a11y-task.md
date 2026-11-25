@task_prose part=session
The window step of a placing open now reads the compositor's monitors before and after its one move, and when the focused monitor's active workspace moved it sends one workspace request back to the earlier workspace, so the human's view stays where it was; the placement word is still set by the move alone, and a failed read or restore changes no word, field or line.
The Hyprland seam's request set stays eight, the placement words stay eight, no argv, flag, env var, socket, IPC method or crate was added; viola open, viola up, viola revive --origin and viola adopt under --origin all take the changed step.
viola verify gains a refusal: when any of Runs B to E first read the trust or imports screen, verify refuses with the fixed pair unable: claude did not start ready in this folder: it is not trusted / hint: run viola verify from a folder you trust in Claude Code, exits 1, prints no step lines and no stamped summary, writes no stamp, and logs process-exit detail untrusted-folder at ERROR; the pair's wording stands PROVISIONAL on a delegate's word.
The automatic verify at start then prints its existing no-stamp unstamped line and leaves the version un-attempted.
The process-exit detail enum of the diagnostic line schema gains untrusted-folder; verify's closed refusal codes gain untrusted-folder beside program-not-found, pinned-hash-mismatch, version-unreadable, strict-modes-failed, unclean-fixture and interrupted.
A new test-only live-check script (R7) reads whether a placed window leaves the active workspace unchanged; it ran only as a rehearsal over fakes, and the capability stays gated on R7 with the view, keyboard focus after the restore, and a failed restore live all unproven.
@task_file n=1 path=src/run/window.rs
what: the window step of viola open / up / revive / adopt — the placing move over the Hyprland seam, each compositor read bounded, now with the monitors reads before and after and the conditional restore of the active workspace
@task_file n=2 path=src/cmd/verify.rs
what: viola verify — its measured runs, step lines, stamped summary, stamps and its closed set of refusals with their unable/hint pairs and process-exit detail
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostic log line schema — the process-exit detail enum of code words
@task_file n=4 path=scripts/r7-place.sh
what: the test-only live check R7 — opens one terminal window on the real compositor and prints a green or red verdict on placement and the active workspace; under --rehearse a rehearsal over fakes, never a live reading
@task_file n=5 path=tests/cli_open.rs
what: the viola open integration suite — the placing open's dispatch list held to the move alone
@task_file n=6 path=tests/cli_verify.rs
what: the viola verify integration suite — the untrusted-folder refusal's exit, pair, no stamped line and no stamp
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the viola verify stamp tests — a failing relied row still stamps, a verify under an instance logs its start and exit
@task_file n=8 path=tests/cli_verify/runs.rs
what: the viola verify run-dir tests — a failing verify leaves neither run dir behind
@task_file n=9 path=tests/cli_version_gate.rs
what: the version gate suite — the automatic verify at start, the no-stamp unstamped line, a stamp holding a failing row read as unverified
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-verb exit-cause table — every refusal code word with its exit code and hint
@task_file n=11 path=tests/contract_diag_schema.rs
what: the diagnostic line schema contract test — each process-exit detail code word in the enum
@task_file n=12 path=tests/support/verify.rs
what: test support for viola verify — a stamping verify typed in a given folder
@task_file n=13 path=tests/support/home.rs
what: test support for the test home — starting a wrapped session as the user in a given folder
@task_item n=1 part=touches
viola verify's closed refusal set gains untrusted-folder; rows holding the verify refusal pairs (the interrupted pair and its siblings) take a sibling for the new unable/hint pair, its wording PROVISIONAL.
@task_item n=2 part=touches
CLI refusal output (unable line, hint line, exit code, stderr layout): the new verify refusal follows the unable/hint form, exit 1, one hint for its one code.
@task_item n=3 part=touches
Reason codes always beside unable, every refusal ends with a hint, one hint per code, no hint names release: the new untrusted-folder code and its hint fall under these rules.
@task_item n=4 part=touches
Fixed CLI strings pinned by literal and refusal layout held by literal pins: the new pair is pinned by literal in the verify integration test; fixed CLI copy comes from design, and this pair's words are not yet his — PROVISIONAL for the epoch batch.
@task_item n=5 part=touches
Code word format and unlisted code words stay visible: untrusted-folder is a new kebab-case code word on the process-exit detail and in the exit-cause table.
@task_item n=6 part=touches
viola verify output step line and the critical path over verify output: a confounded run now prints no step lines and no stamped summary — a refusal before any row is read, while a failing row stays no refusal.
@task_item n=7 part=touches
viola verify interrupted exit 1: the new refusal shares exit 1 and the same no-stamp end; rows naming verify's exit-1 refusal codes take its sibling.
@task_item n=8 part=touches
Each compositor read bounded at 5 s and the window step's wait bound: the window step now makes up to two more monitors reads and one restore request, each through the same bounded seam; a failed one changes no word or line.
@task_item n=9 part=touches
Time bounds fall on the driver only: the added compositor reads and restore lengthen the driver's placing step, never a wait on the human.
@task_item n=10 part=touches
Screen-reader target for the tui (Orca with foot on Hyprland) and human-takes-the-wheel: a placing open no longer moves the human's view to the placed window's workspace; where keyboard focus lands after the restore is unproven until R7.
@task_item n=11 part=touches
Live readings never gate: R7 is a live reading that stays unfired; the capability is gated on it, with only its rehearsal over fakes run in the light gate.
@task_item n=12 part=touches
Run diagnostics never reach the terminal and paths not carried: the new process-exit detail goes to the log only, and the refusal pair carries no path.
@task_fact n=1 slug=cognitive-accessibility
fact: viola verify refuses with untrusted-folder when any of Runs B to E first read the trust or imports screen — the unable/hint pair, exit 1, no step lines, no stamped summary, no stamp, process-exit detail untrusted-folder; its pair wording PROVISIONAL on a delegate's word until the epoch batch.
@task_fact n=2 slug=keyboard-test-harness
fact: a placing open leaves the human's active workspace as it was — the window step restores it when the move took the view — and the live proof of the view, and of where keyboard focus lands after the restore, is R7, a live reading not yet fired.
@task_item n=1 part=untouched
The TUI passthrough rows (entry, human keystrokes, focus/mouse/resize sequences, vendor content, viola terminal bytes, vt100 feed panic) — no byte path of the wrapped terminal changed.
@task_item n=2 part=untouched
viola list board, its caption, colour, stale row, NAME truncation and every SGR rule — no board or colour output changed, and the new pair carries no SGR.
@task_item n=3 part=untouched
viola revive --json refusals, run/revive start refusals under --json and the --json failure documents — verify takes no --json, so the new refusal writes no error body.
@task_item n=4 part=untouched
viola link / unlink, viola send refusals, viola wait, viola switch, plugin install and viola --help rows — no output or exit code of these changed; the help text is unchanged.
@task_item n=5 part=untouched
Frame verb, hook call, MCP frame tool and instance-unreachable detail rows — no socket, strict-modes or server-verify refusal changed.
@task_item n=6 part=untouched
CI integration, SLO invariants and the a11y gate decisions — no CI step, harness command or gate threshold changed.
@task_item n=7 part=untouched
The absences (web or graphical frontend, touch surface, rendered page, internationalization, user time limit) — no frontend, locale or user-facing time limit added.
@task_item n=8 part=untouched
The a11y plan's own manual screen-reader and keyboard pass rows of another meaning — the R7 here is the placing live check, not an a11y reading.
