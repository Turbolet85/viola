@task_prose part=session
The relied trust-record ledger row was read live on claude 2.1.325 in the live rig and passed 23/23 at start S0B: the row is verified on 2.1.325 in the rig's private home only, nothing claimed for 2.1.326 or 2.1.327.
The cause that stopped runs B to F short of the input box was measured: 2.1.325's browser-tools prompt (ChromeAutoEnableDialog); only the rig was mended, by seeding claudeInChromeDefaultEnabled false; viola verify and viola run are unchanged for that prompt, which becomes its own route entry.
viola verify gained a HIDDEN flag --record-first-screens, absent from --help and passed only by the rig: on a FAILED --record verify it writes one scrubbed first-screen file per run that did not reach the input box; a plain --record and a passing verify write no such file; an unclean set of rows is refused whole.
The ledger stamp's measured block gained each interactive run's first-screen word, settle wall and closed facts; the rig gained a VC arm, new summary lines, labels S0R-3, S0C, S0B, S0B-2, and a five-key seed (two markers retired).
The verify window nextest kill moved 60 s to 80 s; the relied ledger row count stays 23; no verb, exit code, refusal wording, --json document or human output line of a public verb changed.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the claude screen reader: the compiled signature and literal set that classifies a screen as trust prompt, modal, input box, none or exited
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger and its stamp: the relied rows, the typed runs A to F, and the measured block of the verify stamp
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the ledger's unit cases
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamp's unit cases (the measured block a verify writes)
@task_file n=5 path=src/cmd/verify.rs
what: the viola verify verb: its flags (--record, the hidden --record-first-screens), its recorded-fixture write, scrub and fixture-not-clean refusal
@task_file n=6 path=src/cmd/verify/typed.rs
what: viola verify's typed interactive runs A to F against the real claude CLI
@task_file n=7 path=src/cmd/verify/typed/run.rs
what: one interactive verify run: its settle, its ending by kill or Ctrl-C, and its first-screen reading
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: the dialog, plan and resume interactive verify runs and how each run's reading is kept
@task_file n=9 path=tests/cli_verify.rs
what: the viola verify integration tests over the fake agent (--record, --help, stopped verify)
@task_file n=10 path=tests/cli_verify/runs.rs
what: the viola verify integration tests of the interactive runs' first screens
@task_file n=11 path=tests/support/verify.rs
what: the shared test support that starts a viola verify over the fake agent
@task_file n=12 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated child scratch: its private config seed, git root layout and kept first-screen copy
@task_file n=13 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its arms (V, VC), labels, summary lines and the viola verify argv it passes
@task_file n=14 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's terminal screen model
@task_file n=15 path=crates/viola-e2e/src/rig/ledger.rs
what: the live rig's start ledger and its spare labels
@task_file n=16 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary and its live verb
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsal tests over the fake agent
@task_file n=18 path=.config/nextest.toml
what: the test runner's profiles and per-class kill timeouts (the verify window kill)
@task_file n=19 path=.gitignore
what: the repository's ignore list (now the local first-screen record files under fixtures)
@task_item n=1 part=touches
contract/viola-verify-output/help-paragraph and contract/viola-verify-output/usage: viola verify gained a hidden flag --record-first-screens that is deliberately absent from --help and the usage; a picker checks the rows still state the visible flags only and whether they need to say a hidden rig-only flag exists
@task_item n=2 part=touches
contract/viola-verify-output/record: --record keeps its behaviour unchanged, but with the hidden flag a FAILED --record verify additionally writes one first-screen file per run that did not reach the input box (Run A's trust excepted); a passing verify writes none
@task_item n=3 part=touches
contract/viola-verify-refusals/fixture-not-clean: the first-screen rows go through the same scrub and screen-fault check as the recorded fixture, and an unclean set is refused whole by the same refusal path
@task_item n=4 part=touches
contract/viola-verify-output/summary, contract/viola-verify/summary, contract/viola-verify-output/exit-0, exit-1 and failing-row: the relied row count stays 23 and the trust-record row now reads pass on 2.1.325; any count or row wording held there is checked against 23, not moved
@task_item n=5 part=touches
contract/viola-verify-output/stop and contract/viola-verify-refusals/stopped: a stopped verify with --record writes no first-screen file (new case verify_stopped_with_record_writes_no_first_screen); the stop wording is unchanged
@task_item n=6 part=touches
contract/cli-verbs/help and contract/viola-verb-help/usage: a hidden flag on a verb, like decision/mcp-verb-hidden's hidden verb, is a precedent for help listing only what is shown
@task_item n=7 part=touches
decision/mcp-verb-hidden: indirectly, the hidden-flag pattern now also exists on viola verify (a flag, not a verb), passed only by the live rig
@task_item n=8 part=touches
contract/cli-verbs/run and contract/viola-send-output/input-not-ready-hint: indirectly, the measured 2.1.325 browser-tools prompt is not read as a modal by viola run's pre-send gate yet; run is unchanged and the reading is deferred to its own route entry, so these rows stand
@task_item n=9 part=touches
decision/no-prompt-or-pager: indirectly, the browser-tools prompt is claude's own screen, not a viola prompt; viola still asks nothing interactively
@task_item n=10 part=touches
contract/viola-verify-output/plain-ascii and contract/viola-verify-output/step-line: the first-screen record is a JSON file, not printed output; verify's printed lines are unchanged
@task_fact n=1 slug=surface-cli
fact: viola verify carries one hidden flag, --record-first-screens: absent from --help and the usage, passed only by the live rig, and acting only on a failed --record verify, where it writes one scrubbed first-screen file per interactive run that did not reach the input box (Run A's trust excepted) and refuses an unclean set whole; a plain --record and a passing verify write no such file
@task_item n=1 part=untouched
The board and viola list rows (BAY line, columns, colour, truncation, stale rows): no list rendering changed
@task_item n=2 part=untouched
The CLI token rendering rows, SGR styling, colour and motion decisions: no styling changed
@task_item n=3 part=untouched
CLI exit codes, the --json mode and internal-error rows: no exit code or document shape changed
@task_item n=4 part=untouched
viola send, wait, last, answer, link, unlink, pause, release and their output and refusal rows, and the readback mirror: none of these verbs changed
@task_item n=5 part=untouched
viola revive and viola run start refusals and start output, the frame-verb hints and the exit-21 detail rows: unchanged
@task_item n=6 part=untouched
viola mcp tools and results, plugin install rows, and the web-spa surface and deep-links absences: unreached
