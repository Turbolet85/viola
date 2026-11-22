@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=59
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rul
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=26 class=omission
leaf-text: and a resize is no byte
source: dec-zyofwgok «viola never holds, queues or re-orders a human key behind a resize.»
replace-with: and a resize is no byte, and viola never holds, queues or re-orders a human key behind a resize
@leaf_edit n=2 leaf=.claude/docs/services/viola.md line=6 class=omission
leaf-text: else failing, never on the sender's words
source: dec-rkrohwfd «arrives opening with the CLI's cross-session harness prefix, else fail, never on the sender's words; no runtime dir runs no turn.»
replace-with: else failing, never on the sender's words, and with no runtime dir running no turn
@leaf_edit n=3 leaf=.claude/docs/services/viola.md line=6 class=omission
leaf-text: with one print-mode sender, `verify-sender`, inside Run B's cross-session turn
source: dec-wgz7t5no «A local claude session is addressed for a cross-session send by its inbox socket address, uds:<runtime>/cc-socks/<pid>.sock, composed from the runtime dir and the session's pid, never by a bare session id.»
replace-with: with one print-mode sender, `verify-sender`, inside Run B's cross-session turn (Run B addressed by its inbox socket `uds:<runtime>/cc-socks/<pid>.sock`, from the runtime dir and its pid, never by a bare session id)
