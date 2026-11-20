@task_prose part=session
The session changed how the pre-send readiness gate reads a dialog: a compiled modal literal quoted inside a transcript row no longer refuses a send on an idle session, while a modal that truly stands still refuses with input-not-ready.
The input-box readiness predicate now refuses only on a standing modal: some row holds a modal literal AND that row is option-shaped (any indent, an optional selector glyph, optional numbering, the literal, only spaces after) OR no input-box row stands below it.
The same predicate serves every caller unchanged in signature: run's pre-send gate verdict, verify's paste guard, the ledger's input-box post-condition and the e2e rig's box read; the gate's quiet period, wait bound, poisoned arm and unverified arm are unchanged.
Run F's first-screen read, the raw modal-presence checks and verify's settle end stay on raw literal presence by design; no refusal word, detail, exit code, hint, event kind, config key, crate, dependency or count moved.
Tests added: unit cases for the option shape and the box-below clause, gate cases over a quoted and an option-shaped label above the box, a synthetic quoting screen in the fake-agent test support, and an integration send past a quoted modal label; CI green on the pushed head.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures of the claude agent crate and the input-box readiness predicate over vt100 rows (modal, footer and input-box literals), with the pre-send gate's screen verdict; edited
@task_file n=2 path=src/run/gate.rs
what: run's pre-send readiness gate (quiet period, wait bound, verified and unverified arms) and its unit tests; edited, tests only
@task_file n=3 path=tests/support/fake.rs
what: integration-test support for driving the fake agent, including its synthetic screens; edited, a synthetic quoting screen added
@task_file n=4 path=tests/cli_send.rs
what: integration tests of the send verb against the fake agent on a verified CLI; edited, a send past a quoted modal label added
@task_item n=1 part=touches
decision/screen-signatures-compiled goes stale: a modal literal no longer refuses on mere presence in any row; readiness refuses only on a standing modal, a literal on an option-shaped row or with no input-box row below it.
@task_item n=2 part=touches
decision/vt100-readiness-gate goes stale: the pre-send gate's at-once refusal now reads a standing modal, not any row holding a modal label; it stays a pre-send gate only.
@task_item n=3 part=touches
contract/send-results/input-not-ready: the condition that yields input-not-ready moved; a quoted modal label above the input box now reads ready, a standing modal still yields input-not-ready.
@task_item n=4 part=touches
contract/human-hint-lines/input-not-ready: the hint text is unchanged, but the screen condition behind it moved to the standing-modal rule.
@task_item n=5 part=touches
contract/not-delivered-details/input-not-ready: the detail is unchanged; the readiness condition that produces it moved.
@task_item n=6 part=touches
decision/partial-gate-on-unverified-cli and decision/poisoned-screen-model, indirect: rules over the readiness gate whose modal refusal moved; the unverified and poisoned arms themselves are unchanged.
@task_item n=7 part=touches
decision/run-never-reads-screen-content and decision/screen-rows-read-only-by-verify, indirect: the predicate now reads a row's shape around a compiled literal (selector glyph, numbering, trailing spaces) and the rows below it; it keeps no row text and returns only a boolean.
@task_item n=8 part=touches
decision/verify-box-wait-for-literal, indirect: verify's paste guard reads the moved predicate, while verify's settle end stays on raw literal presence by design.
@task_item n=9 part=touches
decision/trust-record-ledger-row, indirect: Run F's first-screen read keeps raw modal presence by design and does not take the standing-modal rule.
@task_item n=10 part=touches
decision/browser-tools-prompt-ledger-row and decision/no-typing-into-trust-dialogs, indirect: the trust, imports and browser-tools labels refuse a send only when standing; a shaped or box-less label of each still refuses.
@task_item n=11 part=touches
decision/local-command-post-conditions, indirect: the ledger's input-box-ready post-condition calls the moved predicate without edit.
@task_item n=12 part=touches
decision/send-confirmed-after-the-fact and decision/every-send-confirmed, indirect: their claim that a turn boundary does not prove the input box ready stays true; checked, unchanged.
@task_item n=13 part=touches
decision/screen-fixtures-signature-rows-only, indirect: the new test screen is synthetic and named so; no recorded screen fixture changed.
@task_item n=14 part=touches
decision/agent-claude-pure and decision/claude-shapes-isolated, indirect: the new predicate is pure over compiled literals and stays inside the claude agent crate.
@task_fact n=1 slug=established-decisions
fact: The input-box readiness predicate refuses on a standing modal only (a modal literal on an option-shaped row, or with no input-box row below it), so a modal label quoted in a transcript above the box reads ready; Run F's first-screen read and verify's settle end keep raw literal presence by design.
@task_item n=1 part=untouched
Channel methods, frames, fault codes and the MCP tools: no method, endpoint, socket or tool result shape added or changed.
@task_item n=2 part=untouched
Event kinds and event data, refusal reasons and the closed set of refusal details: no kind, word or detail added; a standing modal keeps input-not-ready.
@task_item n=3 part=untouched
CLI exit codes, verbs, arguments and the --json documents of every verb: none changed.
@task_item n=4 part=untouched
Names: no crate, binary, command, environment variable or endpoint name taken or freed.
@task_item n=5 part=untouched
The capability ledger's rows, stamps envelope and relied-row counts: no row added, no stamp reading or count moved.
@task_item n=6 part=untouched
Build, CI, dependency policy, toolchain and coverage floors: no dependency, lint, workflow or floor changed.
@task_item n=7 part=untouched
Hooks, dialog hold, the wheel, links, revive, list, liveness, config and diagnostics logs: not reached.
@task_item n=8 part=untouched
Occupied resources on disk and sockets, test-home handling and the fake agent binary's own seams: not reached.
