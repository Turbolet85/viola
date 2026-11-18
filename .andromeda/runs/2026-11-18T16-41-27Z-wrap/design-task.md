@task_prose part=session
The session made the CLI's browser-tools prompt (Yes, use my browser) a third compiled modal literal beside the trust and external-imports modals, read as the new first-screen word browser-tools.
It added a 24th relied ledger row, browser-tools-prompt, whose word is pass, absent or fail; absent counts toward a verified CLI on this row alone.
viola verify now prints [NN/24] step lines whose verdict word is pass, absent or fail, and its summary stamped <ver>  n pass  m fail counts an absent row inside n (all-pass reads 24 pass  0 fail).
The run gate refuses a send at once (exit 13, no key typed) while the browser-tools prompt stands; viola never types into that prompt.
Measured live on 2.1.325: with the prompt unanswered verify types nothing, the paste rows and trust-record fail, and the version stays unverified until a person answers the prompt once (8/24, exit 1).
The test-only fake agent gained --start-screen and the live rig gained the VP and VK arms with cache-a and browser-tools-prompt summary lines; nothing was retired.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: edited — the compiled screen signatures: the modal literals, the first-screen classification and its closed words
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: edited — the CLI capability ledger: the relied rows, each row's word, and the verified-version rule
@task_file n=3 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited — the ledger's unit checks of each row's reading
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited — the ledger's unit checks of the modal-signature and trust-record rows
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited — the ledger's unit checks of the stamps.json record and the verified rule
@task_file n=6 path=src/cmd/verify.rs
what: edited — the viola verify verb: its step line, step counter, verdict word and summary line
@task_file n=7 path=src/cmd/verify/typed/pty.rs
what: edited — verify's settle of an interactive start's first screen
@task_file n=8 path=src/cmd/verify/typed/turns.rs
what: edited — verify's typed runs and the first screen each run meets
@task_file n=9 path=src/run/gate.rs
what: edited — the pre-send input-box gate that refuses a send while a modal stands
@task_file n=10 path=src/run/version_gate.rs
what: edited — the version gate: the verified or unverified CLI state read from the stamp
@task_file n=11 path=src/bin/viola-fake-agent.rs
what: edited — the test-only fake agent and its argv-only options
@task_file n=12 path=tests/cli_send.rs
what: edited — the viola send integration checks, its refusals included
@task_file n=13 path=tests/cli_verify.rs
what: edited — the viola verify output integration checks: step lines and summary
@task_file n=14 path=tests/cli_verify/runs.rs
what: edited — the viola verify integration checks of its typed runs
@task_file n=15 path=tests/cli_verify/stamps.rs
what: edited — the viola verify integration checks of the stamps.json record
@task_file n=16 path=tests/contract_ledger_probes.rs
what: edited — the contract check of the ledger's relied row list
@task_file n=17 path=crates/viola-e2e/src/harness/run.rs
what: edited — the e2e harness: its live row list and live-pass reading
@task_file n=18 path=crates/viola-e2e/src/rig/mod.rs
what: edited — the standing live rig: its arms, labels and summary lines
@task_file n=19 path=crates/viola-e2e/src/rig/isolate.rs
what: edited — the live rig's private scratch home and its seeded markers
@task_file n=20 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited — the live rig's rehearsal checks
@task_item n=1 part=touches
contract/viola-verify-output/step-line: the step counter is now [NN/24] and the verdict word is pass, absent or fail (absent only on the browser-tools-prompt row).
@task_item n=2 part=touches
contract/viola-verify-output/summary and contract/viola-verify-output/summary-line: n in n pass  m fail now counts the rows that count, an absent browser-tools-prompt row among them; the all-pass line reads 24 pass  0 fail.
@task_item n=3 part=touches
contract/viola-verify-output/trust-record-names-nothing: its like every relied row passes wording moves to every relied row counting (pass, or absent on browser-tools-prompt alone).
@task_item n=4 part=touches
contract/viola-verify-output/fail-word: the fail word now stands beside a third verdict word absent; any row cell listing the verdict words as pass and fail only is stale.
@task_item n=5 part=touches
contract/cli-verification/verified and contract/cli-verification/unverified-cli: the verified rule changed from every relied row pass to every relied row counting, absent counting on browser-tools-prompt alone; a stamp of the 23 pre-chunk ids reads unverified.
@task_item n=6 part=touches
decision/step-counter-only-progress-pattern and decision/one-progress-pattern: the step counter's total moved 23 to 24 wherever a row holds it.
@task_item n=7 part=touches
decision/no-success-before-confirmation and decision/fixed-phraseology-vocabulary: a new fixed word absent enters verify's output, and an absent row is counted inside n pass while its own line says absent.
@task_item n=8 part=touches
contract/cli-exit-codes/13 and contract/refusal-hints/not-delivered-input-not-ready (or not-delivered-unknown-dialog): a send on a verified CLI is now refused at once while the browser-tools prompt stands, as for the other modals.
@task_item n=9 part=touches
contract/refusal-hints/unverified-cli: an unanswered browser-tools prompt now leaves the version unverified after verify, so the unverified-cli refusal and hint are reached on such hosts until a person answers the prompt once.
@task_item n=10 part=touches
decision/no-interactive-prompt-or-pager: viola reads the CLI's browser-tools prompt as a modal and never types into it; whether it ever answers or seeds the prompt is left open.
@task_item n=11 part=touches
contract/viola-verify-output/stopped-or-refused: verify with the prompt unanswered settles at once on runs B to F, pastes nothing and exits 1 with paste rows failing; check its wording still holds.
@task_item n=12 part=touches
name/command/verify: the verb's output gained the browser-tools-prompt row id and the first-screen word browser-tools in its stamp.
@task_fact n=1 slug=surface-cli
fact: viola verify's step-line verdict word is one of pass, absent, fail; absent is written only on the browser-tools-prompt row (no run met the prompt) and counts toward a verified CLI and toward n pass, on that row alone.
@task_fact n=2 slug=surface-cli
fact: the CLI's browser-tools prompt (Yes, use my browser) is a modal viola never answers: verify pastes nothing past it and send is refused while it stands, so the version stays unverified until a person answers it once.
@task_item n=1 part=untouched
brand-identity strips, the flight-progress strip contracts, the readback box and send readback rows: no strip field, readback form or handoff marker moved.
@task_item n=2 part=untouched
color-palette, typography, spacing and the CLI colour tokens, colour depth and colour decision order: no colour, SGR span or font rule changed; the new word absent is plain text.
@task_item n=3 part=untouched
the viola list board and list table contracts, the six fixed columns and the WHEEL column width: the board is unchanged.
@task_item n=4 part=untouched
viola revive, run start refusals, link and unlink, wait and last output contracts and their --json codes: none moved.
@task_item n=5 part=untouched
exit 21 causes, strict-modes refusals, socket-dir and wrapper fault detail rows: no endpoint, IPC method, env var or refusal cause was added.
@task_item n=6 part=untouched
the self-validation protocol, anti-patterns and the web-spa and rendered-page absences: no page or web surface was added.
@task_item n=7 part=untouched
the viola verify refusals (CLI not found, version unreadable, unclean fixture, stamps.json update, interrupted, home-tree check) and --help: their words and order did not move.
