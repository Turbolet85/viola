@task_prose part=session
The window step that places a session's own terminal window on its asked workspace now keeps the view: it reads the focused monitor's active workspace before the one move and again after it, and sends the one return to the earlier workspace only when the move changed it; the placement word is still set by the move alone, and a failed read or failed return changes no word, field or line.
Every placing caller takes it: viola open, viola up, viola revive --origin and viola adopt under --origin. The Hyprland seam's request set stays eight and the placement words stay eight.
viola verify gains a refusal, code untrusted-folder: when any of Runs B to E first read the trust form or the imports form, verify refuses with a fixed unable/hint pair, exit 1, no step lines, no stamped summary, stamps.json untouched, and a process-exit line with detail untrusted-folder; the automatic verify then prints its no-stamp line and leaves the version unattempted.
The diag line schema's process-exit detail enum gains untrusted-folder; a test-only live-reading script for the placing check (R7) was added and only rehearsed over fakes. R7 has not run; the capability stays gated, unproven on the real compositor.
@task_file n=1 path=src/run/window.rs
what: the window step — placing a wrapped session's own terminal window on its workspace through the Hyprland seam, setting the placement word; now also reading the active workspace before and after the move and returning the view to it
@task_file n=2 path=src/cmd/verify.rs
what: viola verify — the probe runs, the step lines, the summary, the stamps.json update and verify's refusals with their unable/hint pairs and refusal codes
@task_file n=3 path=schemas/diag-line.v1.json
what: the diag line schema — the closed log line shape, including the process-exit detail codes
@task_file n=4 path=scripts/r7-place.sh
what: the live check that a placed window reaches its workspace with the active workspace unchanged (R7), test-only, with its rehearse form over fakes
@task_file n=5 path=tests/cli_open.rs
what: the viola open integration tests — the placing open and the dispatch list it sends to the compositor
@task_file n=6 path=tests/cli_verify.rs
what: the viola verify integration tests — refusals, stamping and output of verify
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the viola verify stamping tests — a failing relied row still stamps, and a verify under an instance logs its start and exit
@task_file n=8 path=tests/cli_verify/runs.rs
what: the viola verify probe-run tests — no run folder left behind by a passing or failing verify
@task_file n=9 path=tests/cli_version_gate.rs
what: the version gate tests — the automatic verify at a start, the verified and unverified CLI states and the no-stamp line
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-verb table of exit causes — every refusal code, its exit code and its hint
@task_file n=11 path=tests/contract_diag_schema.rs
what: the diag line schema contract check — every process-exit detail code present in the schema
@task_file n=12 path=tests/support/verify.rs
what: test support for viola verify — spawning a stamping verify, now in a given folder
@task_file n=13 path=tests/support/home.rs
what: test support for a test viola home — starting a wrapped session as the user, now in a given folder
@task_item n=1 part=touches
The viola verify refusals family (contract/viola-verify-refusals/*: cli-not-found, home-tree-check, interrupted, stamps-update, unclean-fixture, version-unreadable) gains a sibling: the untrusted-folder refusal with its fixed unable/hint pair; its wording is PROVISIONAL on a delegate's word, for the epoch batch.
@task_item n=2 part=touches
contract/viola-verify-output/stopped-or-refused — a run whose first screen read the trust or imports form is now a refusal before any row is read: no step lines and no stamped summary.
@task_item n=3 part=touches
contract/viola-verify-output/step-line, fail-word, summary and summary-line — a confounded run no longer prints failing rows or a stamped summary; a failing relied row still stamps as before, but a trust or imports first screen no longer yields one.
@task_item n=4 part=touches
contract/viola-verify-output/stamps-refused and the stamps rule — the untrusted-folder refusal writes no stamp and leaves stamps.json untouched.
@task_item n=5 part=touches
contract/viola-verify-output/trust-record-names-nothing — verify's relation to folder trust changed: an untrusted folder now refuses the verify instead of failing rows.
@task_item n=6 part=touches
contract/auto-verify-line/no-stamp and did-not-verify — the automatic verify at a run, revive or open start, meeting the untrusted-folder refusal, prints the no-stamp unstamped line and leaves the version unattempted.
@task_item n=7 part=touches
contract/cli-verification/unverified-cli and contract/refusal-hints/unverified-cli — after an untrusted-folder refusal the CLI version stays unverified (cli_verified false) until a verify from a trusted folder.
@task_item n=8 part=touches
contract/cli-exit-codes/1 — verify's exit 1 gains one more refusal cause, untrusted-folder.
@task_item n=9 part=touches
contract/log-detail-codes/* — the process-exit detail codes gain untrusted-folder for verify's refusal; the family may take a sibling.
@task_item n=10 part=touches
contract/cli-refusal-line/fixed-unable-form and hint, contract/cli-stderr/refusal-line and hint-line, contract/fixed-refusal-pairs/* — the new fixed pair is one more refusal in that form; it names no path or pid.
@task_item n=11 part=touches
decision/hint-never-quotes-text, decision/no-internals-in-errors and decision/start-refusals-show-no-path-or-pid — rules the new pair must hold: its hint names the action (run verify from a folder you trust in Claude Code) and quotes no text, path or internal.
@task_item n=12 part=touches
contract/viola-verify-output/help — the new hint echoes the help text; the help text itself is unchanged.
@task_item n=13 part=touches
contract/placement-word/placed, refused, unreachable and the other placement words — still set by the move alone; a failed before-read, after-read or return leaves placed; the count stays eight, no ninth word.
@task_item n=14 part=touches
name/command/open, name/command/up, name/command/revive and viola adopt under --origin — every placing verb now keeps the active workspace where it was after it places its window; no output line, word or field of those verbs changed.
@task_item n=15 part=touches
contract/cli-verb-output/up and contract/viola-up-output/line-order — viola up places through the same window step; its lines and words are unchanged.
@task_fact n=1 slug=surface-cli
fact: viola verify refuses, code untrusted-folder, when any of Runs B to E first read Claude Code's trust form or imports form: the fixed pair unable: claude did not start ready in this folder: it is not trusted / hint: run viola verify from a folder you trust in Claude Code, exit 1, no step lines, no stamped summary, stamps.json untouched; the wording is PROVISIONAL, delegate overseer, for the epoch batch.
@task_fact n=2 slug=surface-cli
fact: After an untrusted-folder refusal the automatic verify at a start prints its no-stamp unstamped line and leaves the version unattempted; the CLI stays unverified.
@task_fact n=3 slug=surface-cli
fact: The process-exit log detail codes include untrusted-folder, viola verify's refusal for a folder Claude Code does not trust.
@task_fact n=4 slug=surface-cli
fact: A placing verb (open, up, revive --origin, adopt under --origin) keeps the view: when the move of its own window changed the focused monitor's active workspace, it returns to the earlier workspace once; the placement word is set by the move alone and a failed read or return changes no word, field or line.
@task_item n=1 part=untouched
Brand identity, the flight-progress strip, the readback box and the send readback rows — no strip, readback or send changed.
@task_item n=2 part=untouched
Colour palette, typography, spacing, iconography and the colour tokens and colour decision order — no colour, glyph, column or SGR changed; the new pair is plain ASCII.
@task_item n=3 part=untouched
The viola list board and table rows — no board column, row, cut or skip count changed.
@task_item n=4 part=untouched
viola revive --json codes and refusals, viola run start refusals, exit 21 causes and the strict-modes refusals — none gained or lost a code.
@task_item n=5 part=untouched
viola link, unlink, wait, last, send, answer, pause and release output, the wrapper fault detail and peer reply screening — none changed.
@task_item n=6 part=untouched
viola focus exit-1 codes and refusals — no focus code or refusal changed.
@task_item n=7 part=untouched
The hook and mcp verbs, the help verb groups, the self-validation protocol, the anti-patterns and the web page absence — none reached.
