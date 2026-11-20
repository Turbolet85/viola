@task_prose part=session
The session changed when viola send gets refused input-not-ready. Before, a modal label anywhere on the child's screen refused the send. Now only a standing modal does.
A modal stands when a row holding a modal label reads as a dialog option (any indent, an optional selector glyph, an optional number, the label, then only spaces), or when no input-box row stands below that row.
A modal label quoted inside a transcript row, with the input box drawn below it, no longer refuses a send on an idle verified session. The send now goes through: issued, prompt submitted, confirmed.
The same readiness predicate is shared by viola run's pre-send gate, viola verify's paste guard and the rig's input-box read. All three moved together. No caller, signature, refusal word, detail, exit code, hint text, log line or schema changed.
Nothing was retired, no count moved and no capability was claimed. The work was proved over the fake agent and synthetic screens only, with no live start. CI is green on the pushed HEAD.
@task_file n=1 path=crates/viola-agent-claude/src/screen.rs
what: the compiled screen signatures of the claude CLI. It holds the readiness read of the input box and the modal-label read that decides the input-not-ready refusal, now refusing only on a standing modal (option-shaped, or with no input box below it).
@task_file n=2 path=src/run/gate.rs
what: viola run's pre-send gate. It holds the at-once input-not-ready refusal, the quiet period and the bounded wait before a send is issued, now refusing at once only on a standing modal.
@task_file n=3 path=tests/support/fake.rs
what: the fake agent's test-support screens, now including a synthetic idle screen that quotes a modal label mid-sentence above the input box.
@task_file n=4 path=tests/cli_send.rs
what: the viola send integration tests over the fake agent, now including a send on a verified CLI that goes past a quoted modal label.
@task_item n=1 part=touches
The input-not-ready refusal of viola send, the cause behind contract/viola-send-output/input-not-ready-hint. Its trigger narrowed from any modal label on screen to a standing modal. The hint text and the refusal word are unchanged, so any row that describes when the refusal fires (for example as "any dialog label on the screen") is now stale.
@task_item n=2 part=touches
The send refusal in viola send --json (contract/send-json/output). The input-not-ready document still reads the same, but it no longer appears for an idle session whose transcript quotes a dialog label.
@task_item n=3 part=touches
The viola send verb row (contract/cli-verbs/send) and the send outcome lines (issue line, read back, unable, unconfirmable). These are reached indirectly: a send that used to end in input-not-ready on a quoted label now gives the issue line, a read back and a confirmed outcome.
@task_item n=4 part=touches
The viola mcp send tool and its refusal result (contract/mcp-tools/send, contract/mcp-tool-results/refusal) are reached through the same pre-send gate. The input-not-ready refusal now fires only on a standing modal.
@task_item n=5 part=touches
decision/wrapper-repeats-send-refusals and decision/hint-for-every-cause, reached indirectly. The input-not-ready cause and its hint still stand, but the screens that raise that cause narrowed.
@task_item n=6 part=touches
viola verify output and its stop and failing-row contracts, reached indirectly. Verify's paste guard reads the same moved readiness predicate, so a quoted modal label above the input box no longer holds verify's paste back. Verify's settle end still reads raw label presence.
@task_item n=7 part=touches
decision/amber-means-dialog and decision/dialog-attention-only-on-board, only where a row ties the DIALOG state to the send gate's refusal. The DIALOG state itself and the dialog detection that feeds the board did not change.
@task_item n=1 part=untouched
The viola list board and its BAY line, columns, widths, truncation, stale rows, colour depth and --json document. No board read or rendering changed.
@task_item n=2 part=untouched
viola revive, viola run start output and their start refusals and --json error codes. No start path or preflight changed.
@task_item n=3 part=untouched
viola link and unlink output, viola plugin install, viola --help and per-verb usage.
@task_item n=4 part=untouched
The CLI token rendering rows (colour, spacing, radius, motion, roles) and the styling and TTY decisions. No colour, glyph or layout changed.
@task_item n=5 part=untouched
viola wait output, viola last output, viola answer output and the wheel verb output (pause, release). None of them reads the readiness predicate.
@task_item n=6 part=untouched
The CLI exit codes, the exit-21 instance-unreachable details and their hints, and the internal-error contracts. No code, detail or hint was added or renumbered.
@task_item n=7 part=untouched
The web surface absence and the CLI deep-links absence. No surface was added.
