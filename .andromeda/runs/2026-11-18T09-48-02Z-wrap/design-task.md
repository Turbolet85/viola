@task_prose part=session
The session read the relied trust-record row live on Claude Code 2.1.325 through the standing live rig, and the row is now VERIFIED on 2.1.325 under the rig's private config (start S0B, 23/23, trust-record pass); nothing is claimed for 2.1.326 or 2.1.327.
It named the cause that stopped interactive verify runs B to F short of the input box: the CLI's browser-tools prompt, raised after an earlier start under the same config; only the rig was mended for it (one config key), viola verify and viola run are unchanged for the prompt, and the prompt becomes its own route entry.
viola verify now names each interactive run's first screen in closed words (trust, imports, box, none, exited) with its settle wall and closed row facts in the stamp's measured block; no screen text enters the stamp.
viola verify gains a HIDDEN option, absent from --help and passed only by the rig, that on a FAILED record verify writes each non-box run's first screen as whole scrubbed rows to a local file; an unclean set is refused whole; a plain record verify and a passing verify are unchanged.
The rig gained a first-frame arm, new labels and summary lines, a five-key private config seed (two markers retired), and the verify-window test kill moved 60 s to 80 s; the relied ledger row count stays 23.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the agent screen reader: the compiled screen signatures and literals, the classifier that names a run's first screen (modal before box) and its closed row facts
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger and its stamp writer: the typed runs' first readings and the stamp's measured first-screen words, settle walls and facts
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: the capability ledger's unit cases
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: the ledger stamp's unit cases, including the six first screens with their settles and facts
@task_file n=5 path=src/cmd/verify.rs
what: the viola verify verb: its options, the record mode, the hidden rig-only first-screen record of a failed verify with its scrub and whole-set refusal
@task_file n=6 path=src/cmd/verify/typed.rs
what: viola verify's typed interactive runs
@task_file n=7 path=src/cmd/verify/typed/run.rs
what: one interactive verify run: keeping its first screen at the settle, its reading and exit
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: viola verify's dialog, plan and resume runs, each returning its first screen
@task_file n=9 path=tests/cli_verify.rs
what: the viola verify integration tests: record mode, the failed-verify first-screen record, its absence without the rig option and from --help
@task_file n=10 path=tests/cli_verify/runs.rs
what: the viola verify interactive-run integration tests, the first screens read over the fake agent
@task_file n=11 path=tests/support/verify.rs
what: the shared test support for starting a viola verify run
@task_file n=12 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolated private scratch: its git-root layout, its private config seed of five keys, the kept first screens
@task_file n=13 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig: its arms, start labels, the V arm's verify argv and its summary lines
@task_file n=14 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's terminal screen model, sized by rows and columns
@task_file n=15 path=crates/viola-e2e/src/rig/ledger.rs
what: the live rig's start ledger and its spare-label count
@task_file n=16 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's command-line entry point and its live verb
@task_file n=17 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent, including the first-frame arm and the kept first screen
@task_file n=18 path=.config/nextest.toml
what: the test runner's profiles and the verify-window class kill timeout
@task_file n=19 path=.gitignore
what: the repository's ignore list, now ignoring the local first-screen record files
@task_item n=1 part=touches
contract/viola-verify-output/trust-record-names-nothing: the trust-record row's verify output; the row now reads pass on 2.1.325 in the rig, and the session changed none of that output's words.
@task_item n=2 part=touches
contract/cli-verification/verified and contract/cli-verification/unverified-cli: the trust-record row is verified on 2.1.325 under the rig only; a live home still needs its own live verify, and nothing is claimed for 2.1.326 or 2.1.327.
@task_item n=3 part=touches
contract/viola-verify-output/help and contract/viola-help/verb-groups and contract/help-output/verb-groups: viola verify gained a hidden option that --help never lists; the help output is unchanged.
@task_item n=4 part=touches
contract/viola-verify-output/summary, contract/viola-verify-output/summary-line, contract/viola-verify-output/step-line and contract/viola-verify-output/fail-word: a failed or passing verify prints the same lines as before; the first-screen words go to the stamp, not to verify output.
@task_item n=5 part=touches
contract/viola-verify-output/stopped-or-refused: a stopped record verify writes no first-screen file; its output is unchanged.
@task_item n=6 part=touches
contract/viola-verify-refusals/unclean-fixture: the hidden first-screen record is checked row and seam for faults and an unclean set is refused whole, like an unclean fixture.
@task_item n=7 part=touches
name/command/verify: the verify verb gains a hidden rig-only option; no new verb and no visible option.
@task_item n=8 part=touches
contract/refusal-hints/not-delivered-unknown-dialog and contract/refusal-hints/not-delivered-input-not-ready: the CLI's browser-tools prompt is today an unknown screen to viola run's pre-send gate; unchanged this session, owed to the new route entry that reads it as a modal.
@task_item n=9 part=touches
decision/no-internals-in-errors and decision/start-refusals-show-no-path-or-pid: the first-screen record rewrites the home path and user out of its rows and prints nothing new to the human.
@task_item n=10 part=touches
decision/stdout-data-stderr-messages and contract/cli-streams/stdout: the hidden record writes local files only; neither stream carries anything new.
@task_item n=11 part=touches
decision/no-interactive-prompt-or-pager: the browser-tools prompt is the agent CLI's own prompt, not viola's; viola still asks nothing interactively.
@task_fact n=1 slug=surface-cli
fact: viola verify carries one hidden option, never listed in --help and passed only by the live rig, that on a failed record verify writes each run's first screen not reaching the input box as whole scrubbed rows to a local file; a plain record verify and a passing verify write no such file.
@task_item n=1 part=untouched
The brand-identity rows: the flight-progress strip, readback box, send readback, handoff and dialog-pending strip are not reached.
@task_item n=2 part=untouched
The color-palette, typography, spacing and iconography rows: no colour, glyph, separator or column changed.
@task_item n=3 part=untouched
viola list, the board and its table, skip counts and strict-modes scan: not reached.
@task_item n=4 part=untouched
viola revive, viola run start refusals, viola link and unlink, viola wait, viola last and the send mirror outputs: not reached.
@task_item n=5 part=untouched
The CLI exit codes, the exit-21 causes and their json causes, wrapper fault details and peer reply screening: unchanged.
@task_item n=6 part=untouched
The CLI colour decision order, colour depth and colour tokens, and the hint rules: no output styling changed.
@task_item n=7 part=untouched
The self-validation protocol and the web surface absence: not reached.
