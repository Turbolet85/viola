@task_prose part=session
The session made the CLI's browser-tools prompt (literal Yes, use my browser) a third compiled modal literal beside the trust and imports literals, and added the first-screen closed word browser-tools.
It added a 24th relied ledger row, browser-tools-prompt, read in three words: pass when some first screen is the prompt, absent when no run met it, fail when a first screen reads none; absent counts toward verified on that row alone.
The verified rule moved from every relied row passing to every relied row counting; verify's step line can now end absent and its summary counts an absent row in n pass; relied rows 23 to 24.
Run's pre-send gate and verify both read the prompt as a modal and type nothing into it; the fake agent gained an argv-only start-screen option and the live rig gained VP and VK arms plus a cache-a line that reads only the presence of one key name in its own private config.
Measured live on 2.1.325: with the prompt unanswered, verify settles at once, pastes nothing, and the version stays unverified (8 of 24); Run E spawns nothing when Run B stands on the prompt.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: edited — the compiled screen signatures of the Claude agent crate: the modal literals, the input-box literals and the first-screen classifier with its closed words
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the per-behaviour CLI capability ledger: its relied rows, each row's check and word, the stamp merge and the verified-version rule
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited — the ledger's unit tests of a row's readings from verify's typed runs
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited — the ledger's unit tests of row checks, the modal-signature and trust-record checks
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — the ledger's unit tests of the stamps envelope, the stamp merge and the verified rule
@task_file n=6 path=src/cmd/verify.rs
what: edited — the viola verify command: its probe steps, the step line, the summary line and its exit
@task_file n=7 path=src/cmd/verify/typed/pty.rs
what: edited — verify's typed-run PTY settle, waiting on the screen until a modal or the input box stands
@task_file n=8 path=src/cmd/verify/typed/turns.rs
what: edited — verify's typed-run turns, what is pasted and typed in each run
@task_file n=9 path=src/run/gate.rs
what: edited — run's pre-send readiness gate over the vt100 screen model, refusing a send while a modal stands
@task_file n=10 path=src/run/version_gate.rs
what: edited — the version gate that reads the ledger stamps strictly and decides verified or unverified for the running CLI
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: edited — the test-only fake agent binary, its argv-only options and drawn screens
@task_file n=12 path=tests/cli_send.rs
what: edited — integration tests of viola send, its refusals and the pre-send gate
@task_file n=13 path=tests/cli_verify.rs
what: edited — integration tests of viola verify, its step lines, summary and stamps
@task_file n=14 path=tests/cli_verify/runs.rs
what: edited — integration tests of verify's runs A to F against the fake agent
@task_file n=15 path=tests/cli_verify/stamps.rs
what: edited — integration tests of the stamps verify writes
@task_file n=16 path=tests/contract_ledger_probes.rs
what: edited — the contract test holding the ledger rows to their probes
@task_file n=17 path=crates/viola-e2e/src/harness/run.rs
what: edited — the viola-harness run, its local-live relied-row list and the live verify pass it accepts
@task_file n=18 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the viola-live-rig standing live rig: its arms, labels, stops and summary lines
@task_file n=19 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's isolated scratch home: the seeded CLI markers and the reads of its own private config
@task_file n=20 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the live rig's rehearsals against the fake agent
@task_item n=1 part=touches
decision/capability-ledger and decision/cli-capability-ledger: the ledger gains a 24th relied row, browser-tools-prompt, the only row read in three words pass, absent, fail.
@task_item n=2 part=touches
decision/version-verified-all-relied-rows and contract/ledger-stamps/verified-version: a version is verified when every relied row counts, pass, or absent on browser-tools-prompt alone; absent on any other row stays unverified, and a stamp of the 23 pre-chunk ids reads unverified.
@task_item n=3 part=touches
contract/ledger-stamps/rows: the stamp gains the row key browser-tools-prompt with the words pass, absent, fail.
@task_item n=4 part=touches
contract/ledger-stamps/first-screen: the closed word set of measured.first_screen gains browser-tools, five words to six.
@task_item n=5 part=touches
decision/screen-signatures-compiled: the modal literals go from two to three (trust, imports, browser-tools), and the first-screen order reads trust, imports, browser-tools, box, exited, none.
@task_item n=6 part=touches
contract/viola-verify-output/step-line: a step line's verdict can now be absent besides pass and fail; the step counter reads NN/24.
@task_item n=7 part=touches
contract/viola-verify-output/summary-line: n pass now counts the rows that count, an absent row inside n; the all-pass summary reads 24 pass 0 fail.
@task_item n=8 part=touches
contract/viola-verify-exit-codes/1-relied-row-failed and contract/viola-verify-exit-codes/1: exit 1 follows a relied row that does not count; absent on browser-tools-prompt is no failure.
@task_item n=9 part=touches
decision/claude-first-no-adapter-trait: its except widens, by the founder's own word, to the live rig reading whether the key name cachedChromeExtensionInstalled stands in its own private scratch config, never a value.
@task_item n=10 part=touches
decision/verify-never-types-into-dialog, decision/verify-types-nothing-into-dialogs and decision/no-typing-into-trust-dialogs: the browser-tools prompt joins the dialogs verify reads as a modal and types nothing into.
@task_item n=11 part=touches
decision/vt100-readiness-gate and decision/partial-gate-on-unverified-cli: run's pre-send gate refuses at once while the browser-tools prompt stands, on a verified CLI as on a partial gate (send exits 13, no key typed).
@task_item n=12 part=touches
decision/trust-record-ledger-row: Run A's modal-signature clause reads the untrusted-start modals (trust, imports) alone, while Run F's no-modal clause keeps the whole set of three.
@task_item n=13 part=touches
decision/verify-run-e-resumes-run-b-first-session: measured live, Run E spawns nothing when Run B stands on the browser-tools prompt, so resume-session fails.
@task_item n=14 part=touches
decision/fake-agent-test-seams and decision/fake-agent-after-double-dash: the fake agent gains the argv-only option start-screen, drawn only under screens; no env var added.
@task_item n=15 part=touches
decision/capability-ledger-gate: indirect, through the ledger's new counts rule that every reader of a relied row now goes through.
@task_item n=16 part=touches
decision/screen-fixtures-signature-rows-only: indirect, new browser-tools screen fixtures in the tests and rehearsals keep the signature rows only.
@task_item n=17 part=touches
decision/no-probe-for-unrelied-shapes: indirect, the new relied row comes with its own verify probe and live proof.
@task_item n=18 part=touches
decision/malformed-stamps-unverified and decision/version-gate-strict-stamps-read: indirect, the version gate's verdict now reads the 24th row and its absent word.
@task_fact n=1 slug=established-decisions
fact: On a host where the CLI's browser-tools prompt stands unanswered, viola neither answers nor seeds it: verify types nothing, the paste rows and trust-record fail, and the version stays unverified until a person answers the prompt once.
@task_fact n=2 slug=established-decisions
fact: The browser-tools-prompt ledger row reads pass when some run's first screen is the prompt, absent when no run met it, fail when any first screen reads none; it is the one relied row whose absent counts toward verified.
@task_fact n=3 slug=established-decisions
fact: The modal-signature check at an untrusted start reads only the untrusted-start modals (trust and imports); the browser-tools literal is a modal for the settle, the pre-send gate and Run F, not for that check.
@task_item n=1 part=untouched
The wrapper channel, its JSON-RPC methods, fault codes and frames: no method, param or code moved.
@task_item n=2 part=untouched
The MCP server, its tools and results: nothing in MCP changed.
@task_item n=3 part=untouched
Hooks, the hook tiers, hook exit codes and the dialog hold: no hook behaviour moved.
@task_item n=4 part=untouched
The wheel, links, pause and release: no wheel or link rule touched.
@task_item n=5 part=untouched
Normalised event kinds, event data, the events log and instance snapshots: no event kind or field added.
@task_item n=6 part=untouched
Revive, plugin install, list and the board: none of these verbs changed.
@task_item n=7 part=untouched
Stack, dependencies, cargo-deny, toolchain, MSRV and CI workflows: no dependency, crate, workflow or tool version moved.
@task_item n=8 part=untouched
Occupied names: no binary, crate, command, env var, socket or port taken or freed.
@task_item n=9 part=untouched
Configuration, config.json keys and the diagnostics files: no config key or log line added.
@task_item n=10 part=untouched
The trust writer and its code words: the trust record writer is unchanged.
