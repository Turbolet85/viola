@task_prose part=session
The session landed the recorded fixture set for claude 2.1.328, taken by the test-only live rig under its private config, and stamped it: the stamped recorded sets go from 3 to 4 (2.1.287, 2.1.302, 2.1.325, 2.1.328), and the recorded CLI fixture sets go from 5 to 6.
The drift, ledger-probe, hygiene and recorded-exchange suites now walk the 2.1.328 set, and all are green. The default CLI version, the harness default and the stamped-home version did not move.
The live rig's context arm (CX) now looks the session transcript up once more after the send. When the transcript is first found after the send, it counts tags from that transcript's start. The arm prints a new closed summary word, context-transcript-after: read|absent|unread. It still keeps no byte.
One live reading was recorded, and no verification is claimed for it: on 2.1.328, the output of the /context command reaches the session transcript as one local-command-stdout tag. The send reply was not-delivered:no-prompt-submitted, which was expected.
No product crate, CLI output, refusal, exit code, colour rule, config key or CI workflow changed. No capability is claimed.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig (the live readings that never gate): its context arm's after-send transcript lookup, the shared read/absent/unread word, and two unit cases
@task_file n=2 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal over the fake agent: the cx rehearsal now asserts context-transcript-after: absent
@task_file n=3 path=fixtures/claude/2.1.328/
what: a new recorded fixture set of claude's hook payloads, MCP tools list and screen signatures (ready, turn, modal) for CLI 2.1.328, which the boundary and drift tests are sequenced on
@task_file n=4 path=tests/contract_ledger_probes.rs
what: the ledger-probe contract suite's list of stamped CLI versions, now four, adding 2.1.328
@task_item n=1 part=touches
Indirect: the rule that live readings never gate covers the live rig, which changed. The 2.1.328 /context reading was recorded as a reading, with no verification claimed.
@task_item n=2 part=touches
Indirect: the rule that boundary tests are sequenced on records. A new recorded set (2.1.328) now feeds the recorded-exchange and drift tests.
@task_item n=3 part=touches
Indirect: the rule that the boundary check never inspects claude content, and that no screen content is parsed beyond signatures. The rig's context arm now counts tags in a transcript it first finds after the send, still as a count only, keeping no byte. The new set also brings fresh screen signatures (ready, turn, modal).
@task_item n=4 part=touches
Indirect: the send refusal no-prompt-submitted. It is the expected reply to the rig's /context send on 2.1.328, and it was read live there.
@task_item n=5 part=touches
Indirect: the send refusal input-not-ready, and the rule that a send is refused while the screen is not ready. A transcript row that quotes a modal literal is read by the pre-send gate as a standing modal and refuses the driver's send. This was measured, not mended, and it is a carry proposal for the wrap.
@task_item n=6 part=touches
Indirect: the single vendor-content boundary and the passthrough of vendor content. The vendor's hook payloads and screens for a new CLI version (2.1.328) are now recorded and walked by the hygiene check with no hand edit.
@task_item n=1 part=untouched
CLI output contracts: refusal format and layout, the unable and hint lines, exit codes, --json failure documents, fault bodies and code words. No product output changed.
@task_item n=2 part=untouched
Colour and SGR rules, the list board (caption, order, stale row, the unwrapped separator), NAME truncation and the help output. None moved.
@task_item n=3 part=untouched
CI integration and the a11y gate rules: no workflow, coverage gate, flakiness rule or suite driver changed. Only the test counts grew, and no row holds those counts.
@task_item n=4 part=untouched
The manual Orca and keyboard passes, the screen-reader targets, and the cognitive-accessibility rows: nothing in the session reaches them.
@task_item n=5 part=untouched
The absences (web or graphical frontend, touch surface, internationalization, rendered page, user time limit) and the TUI passthrough's own terminal bytes, keystrokes and diagnostics. No product crate changed.
