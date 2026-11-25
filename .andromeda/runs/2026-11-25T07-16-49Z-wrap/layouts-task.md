@task_prose part=session
The window step that places a new terminal window on a workspace now reads the focused monitor's active workspace before the move and, when the move switched it, sends the view back once; the placement word, its fields and every printed line stay as they were, and the eight placement words and the seam's eight requests are unchanged.
Every placing caller takes it: viola open, viola up, viola revive --origin and viola adopt under --origin; over the fakes it is proven, on the real compositor it stays unproven and the capability stays GATED on R7.
viola verify gains a refusal: when any of Runs B to E met the trust form or the imports form at its first screen, verify refuses before any row is read with the pair unable: claude did not start ready in this folder: it is not trusted / hint: run viola verify from a folder you trust in Claude Code, exits 1, prints no step lines and no stamped summary, and writes no stamp.
The new refusal code untrusted-folder joins verify's closed refusal codes and the process-exit detail enum of the diag line schema; the automatic verify at start then reads no stamp and leaves the version un-attempted.
The refusal pair's wording stands PROVISIONAL on a delegate's word, for the epoch batch; a test-only R7 script to read placement and view live was added, and nothing was retired.
@task_file n=1 path=src/run/window.rs
what: the window step of a start that places the terminal window on a workspace, its placement word and the view it leaves
@task_file n=2 path=src/cmd/verify.rs
what: the viola verify command, its runs, rows, stamps, refusals and their unable and hint lines
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostic line schema, its process-exit detail enum
@task_file n=4 path=scripts/r7-place.sh
what: a test-only script that opens one placed window and reads its placement and whether the active workspace stayed, the R7 live check
@task_file n=5 path=tests/cli_open.rs
what: the viola open integration tests, its placing case and dispatch list
@task_file n=6 path=tests/cli_verify.rs
what: the viola verify integration tests, its window and refusal cases
@task_file n=7 path=tests/cli_verify/stamps.rs
what: the viola verify stamp tests, a failing row that still stamps and the logged start and exit
@task_file n=8 path=tests/cli_verify/runs.rs
what: the viola verify run directory tests, a failing verify leaving no run dir behind
@task_file n=9 path=tests/cli_version_gate.rs
what: the version gate tests, the automatic verify at start and the stamp it reads
@task_file n=10 path=tests/cross_exit_causes.rs
what: the cross-verb table of refusal causes with their exit codes and hints
@task_file n=11 path=tests/contract_diag_schema.rs
what: the contract check of the diagnostic line schema
@task_file n=12 path=tests/support/verify.rs
what: test support for running viola verify, a stamping verify typed in a given folder
@task_file n=13 path=tests/support/home.rs
what: test support for a viola home and its wrapper, a start as the user in a given folder
@task_item n=1 part=touches
contract/viola-verify-refusals gains a sibling of stopped, cli-not-found, fixture-not-clean, home-not-private and version-unreadable: the untrusted folder refusal, its unable and hint pair, exit 1, no stamp; its wording PROVISIONAL.
@task_item n=2 part=touches
contract/viola-verify-output/failing-row: a run whose first screen read the trust or imports form is no longer a failing row that still stamps but a refusal before any row is read; a failing relied row still stamps.
@task_item n=3 part=touches
contract/viola-verify-output/exit-1: exit 1 now also covers the untrusted folder refusal, with no step lines and no stamped summary.
@task_item n=4 part=touches
contract/viola-verify-output/summary and contract/viola-verify/summary: the untrusted folder refusal prints no stamped summary line.
@task_item n=5 part=touches
contract/viola-verify-output/step-line and contract/viola-verify-output/stop: the new refusal comes after the stop check and before any step line is printed.
@task_item n=6 part=touches
contract/viola-verify-output/help-paragraph: the help text is unchanged; the new hint echoes its advice to run verify from a trusted folder.
@task_item n=7 part=touches
contract/viola-verify-output/plain-ascii: the new unable and hint pair is plain ASCII, a fixed text with no path.
@task_item n=8 part=touches
decision/hint-for-every-cause, decision/hint-last-on-stderr and decision/hints-never-quote-or-locate: the new cause untrusted-folder carries its hint, last on stderr, naming no path.
@task_item n=9 part=touches
decision/shared-refusal-word-column and contract/cli-refusal-line/form: the new refusal uses the shared unable: line form.
@task_item n=10 part=touches
decision/hints-name-next-verb: the new hint names viola verify as the next verb.
@task_item n=11 part=touches
contract/viola-revive/no-window: the window placement of a revive with --origin now keeps the active workspace where it was; the placement word and printed output are unchanged.
@task_item n=12 part=touches
contract/cli-verbs/open, contract/cli-verbs/up, contract/cli-verbs/revive and contract/cli-verbs/adopt: a placing start under each leaves the view on the workspace it was on; no flag, argument or output changed.
@task_item n=13 part=touches
contract/viola-open/parent-default and contract/viola-run/start-output and contract/viola-revive/start-output: the window step under them gained the view restore, their output and placement words unchanged.
@task_item n=14 part=touches
decision/human-output-is-contract and decision/words-carry-every-state: a failed monitors read or a failed restore changes no placement word, field or line; the eight placement words stand.
@task_fact n=1 slug=surface-cli
fact: viola verify refuses an untrusted folder: when Claude met the trust or imports form at the first screen of Runs B to E, verify prints unable: claude did not start ready in this folder: it is not trusted then hint: run viola verify from a folder you trust in Claude Code, exits 1 with code untrusted-folder, prints no step line and no stamped summary and writes no stamp; the wording is PROVISIONAL on a delegate's word for the epoch batch.
@task_fact n=2 slug=surface-cli
fact: a confounded verify is a refusal, not a failing row: a run whose first screen read trust or imports refuses before any row is read and stamps nothing, while a failing relied row still stamps as before.
@task_fact n=3 slug=surface-cli
fact: when the automatic verify at a start meets the untrusted folder refusal it prints the start's no-stamp unstamped line and leaves the version un-attempted.
@task_fact n=4 slug=surface-cli
fact: a placing start (viola open, viola up, viola revive --origin, viola adopt --origin) leaves the active workspace unchanged, sending the view back once when the move switched it, with no new placement word, field or line; proven over the fakes, GATED on R7 on the real compositor.
@task_item n=1 part=untouched
The viola list board, its BAY line, columns, truncation, tree, links block and --json document: no row of the board moved.
@task_item n=2 part=untouched
viola send, its readback mirror, read back lines, refusals and --json form: untouched.
@task_item n=3 part=untouched
viola wait, viola last and viola answer output: untouched.
@task_item n=4 part=untouched
The viola mcp tools and their results, and the decisions hiding verbs from mcp: untouched.
@task_item n=5 part=untouched
CLI token rendering, colour, spacing and styling decisions, banner, spinner and progress decisions: no styling changed.
@task_item n=6 part=untouched
The frame verbs' exit-21 hints and instance-unreachable details, and CLI exit codes 10 to 21: no code or detail of them moved.
@task_item n=7 part=untouched
viola revive --list, its --json error codes and its refusals: none moved; only the placement under --origin changed.
@task_item n=8 part=untouched
viola link and unlink, viola switch, viola plugin install, the wheel verbs and viola --help verb groups: untouched.
@task_item n=9 part=untouched
The web-spa surface and its absence, and the deep-links absence: untouched.
