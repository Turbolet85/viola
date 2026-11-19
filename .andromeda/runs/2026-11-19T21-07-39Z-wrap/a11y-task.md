@task_prose part=session
The session made /context a listed local command of the claude agent, confirmed by the stdout tag it writes to the transcript, and it submits no prompt.
viola send of /context reads back confirmed on a verified CLI, and before its transcript exists it reads ok with confirmed false and detail unconfirmable, exit 0.
viola verify's Run B pastes /context as its eighth paste, and a new relied ledger row local-command-context brings the relied set from 24 to 25 rows. Verify's step lines now read [n/25] and the all-pass summary 25 pass 0 fail. A stamp holding only the 24 earlier rows now reads unverified.
The test-only live rig gained the CS and CC arms. CS kept a scratch-home stamp and read the private config's top-level names and file counts. CC sent a prompt and then /context, typed the two keys x and DEL, and read the attribute words, never the text, of every drawn hint cell past the cursor's cell. The live reading was unread (no-cell), and the hint carry stays open.
One live claude version, 2.1.328, was measured in a scratch home only. The live home is not re-stamped, and no capability is claimed beyond that.
The wrap rules on the a11y except clause on no screen-content parsing (its raw frames kept local and gitignored), which the last wrap left owed: it either returns or stays homed in the security plan's kept-local decision.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: the claude agent's verify ledger: the relied ledger rows and their checks, the listed local commands with their post-conditions, the probe literals and the stamp merge
@task_file n=2 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit cases of the verify ledger's relied rows
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of each ledger row's check
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: unit cases pinning the verify probe literals and the local-command list
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_runs.rs
what: unit cases of the typed verify runs' row checks, including the new local-command-context check
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit cases of the stamps file: which stamped row sets read verified or unverified
@task_file n=7 path=src/cmd/verify.rs
what: the viola verify command: its step lines [n/MM] and its all-pass summary over the relied row count
@task_file n=8 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed probe runs (Run A and Run B), threading the measured latencies
@task_file n=9 path=src/cmd/verify/typed/turns.rs
what: viola verify's Run B framing turns: the compiled literal pastes, now with /context as the eighth
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the fake agent used by the test suites; under --framing it echoes the exact verify probe commands, now the /context probe literal too
@task_file n=11 path=src/run/send/classify.rs
what: viola send's classification and confirmation of a sent text: a listed local command confirmed by a new stdout tag, else unconfirmable
@task_file n=12 path=src/run/version_gate.rs
what: the version gate: a claude version is verified only when its stamp holds every relied row passing
@task_file n=13 path=tests/cli_send.rs
what: integration cases of viola send output, including /context read back on a verified CLI and unconfirmable before its transcript exists
@task_file n=14 path=tests/cli_verify.rs
what: integration cases of viola verify output: its step lines and summary literal pins
@task_file n=15 path=tests/cli_verify/runs.rs
what: integration cases of viola verify's typed runs and their step lines
@task_file n=16 path=tests/cli_verify/stamps.rs
what: integration cases of viola verify's stamp writing and its pass/fail summary
@task_file n=17 path=tests/contract_ledger_probes.rs
what: contract cases stamping every recorded fixture set over the fake agent with every relied row
@task_file n=18 path=crates/viola-e2e/src/harness/run.rs
what: the E2E harness's boot and its pinned list of relied ledger rows and verify step-line literal
@task_file n=19 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig: its arm labels, the CS and CC arms, the kept scratch stamp, the config top-level reading, the hint-cell attribute reading and its summary words
@task_file n=20 path=crates/viola-e2e/src/rig/screen.rs
what: the rig's terminal screen model read at the TUI boundary, now reading the attribute words of drawn hint cells past the cursor's cell
@task_file n=21 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary's command line: its rehearse and live words, now rehearse cs and cc
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig's rehearsal cases over the fake agent, including the cs, cc and cc-refusal rehearsals
@task_item n=1 part=touches
contract/viola-verify/step-line: the step line's total is the relied row count, which moved from 24 to 25. If the row's cell holds a figure (/24, 24 pass), that figure is now stale. A row that says only the relied row count stays true.
@task_item n=2 part=touches
decision/no-screen-content-parsing: the except clause about raw frames kept local and gitignored is ruled on in this wrap. The rig also now reads attribute words of drawn hint cells past the cursor's cell, and it still reads no text. Check that the row's exception covers that reading.
@task_item n=3 part=touches
decision/boundary-check-never-inspects-claude-content: the test-only rig now reads the hint cells' attribute words after the second send, beyond the box row's cell attributes after a turn end. This is the hint-cell width that the founder's answer allowed (2 keys, x and DEL).
@task_item n=4 part=touches
decision/live-readings-never-gate: the new CS and CC live readings (local-command-context pass, context-tags, hint-told-apart unread) are recorded readings that gate nothing. Check that the row still holds.
@task_item n=5 part=touches
decision/critical-path-send-readback: the confirmed send and readback path now includes /context, confirmed by its stdout tag. Before its transcript exists, the read back is ok with confirmed false and detail unconfirmable.
@task_item n=6 part=touches
contract/send-refusal/no-prompt-submitted: /context submits no prompt, but as a listed local command it is confirmed by its tag rather than refused. Check whether the row's wording excepts the listed local commands.
@task_item n=7 part=touches
decision/fixed-cli-strings-pinned and decision/fixed-cli-strings-pinned-by-literal: the verify step-line and summary literals were re-pinned at 25 rows in the integration and E2E pins.
@task_item n=8 part=touches
decision/refusal-layout-literal-pins: in the rig, a CC label with no kept stamp refuses before its ledger line. This is a test-only refusal, not a CLI refusal layout. Flag the row only if it covers the rig.
@task_item n=9 part=touches
decision/tui-boundary-assertions-only and decision/tui-boundary-driver-pin: the rig's screen model at the TUI boundary gained a hint-cell reading. Check that it stays a boundary reading of attributes.
@task_item n=10 part=touches
decision/human-key-takes-the-wheel and decision/stdin-classifier-non-editing-list: the CC arm types x and DEL into the wrapped TUI after the second send. These are keys from the rig in the human's place, and no product stdin rule changed.
@task_item n=11 part=touches
decision/a11y-inside-existing-suites: the new send, verify and rig cases landed inside the existing suites (cli_send, cli_verify, contract_ledger_probes, rig_rehearsal). No separate a11y suite was added.
@task_item n=12 part=touches
decision/zero-flakiness: watch C4 on the rig rehearsal that reads the box empty after x and DEL stayed green over 7 runs, the pre-push and CI. It was not acted on.
@task_fact n=1 slug=keyboard-test-harness
fact: The test-only live rig may read the attribute words, never the text, of every drawn hint cell past the cursor's cell after a second send, and type exactly the two keys x and DEL. This is the founder's width; the reading is live and gates nothing.
@task_item n=1 part=untouched
The viola list board rows (caption, order, colour, stale-row dim, unwrapped separator, NAME truncation): no list output changed.
@task_item n=2 part=untouched
The SGR and colour decisions (word beside every SGR span, no SGR off a terminal, amber DIALOG, bold NAME, failure words uncoloured): no colour or escape output changed.
@task_item n=3 part=untouched
The revive, link and unlink, plugin install, hook call, MCP frame tool and frame verb refusal contracts, and their exit codes: none moved.
@task_item n=4 part=untouched
The absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit): none gained or lost.
@task_item n=5 part=untouched
The CI integration decisions (GitHub Actions workflow, coverage suite, no own a11y artifact, gate breach fails the build): no CI step, gate or status shape changed.
@task_item n=6 part=untouched
The screen-reader targets and manual Orca and keyboard passes: no manual pass was run or changed.
@task_item n=7 part=untouched
The cognitive refusal-text rules (one hint per code, faults carry no hint, unknown refusal has no hint, reason code beside unable): no refusal or fault wording was added.
@task_item n=8 part=untouched
The config.json dialog_hold_s, wait timeouts and dialog expiry rows: no time bound on the human or the driver moved.
